use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, Sprite},
    window::{mouse::Button, Event, Key, Style},
};

use rand::Rng;

const MAX_ENEMIES: usize = 10;
const SPAVN_ENEMY_DELAY: f32 = 10.;

use crate::player::Player;
use crate::{assets::AssetManager, game_obj::GameObj};

#[derive(Clone, Copy, Debug)]
pub enum GameObjKind {
    Default,
    Plasma,
    Iron,
    Hidrogen,
}

pub struct Game<'a> {
    win: RenderWindow,
    player: Player<'a>,
    enemies: Vec<GameObj<'a>>,
    spawn_enemy_timer: f32,
    bg_obj: Sprite<'a>,
    asset_manager: &'a AssetManager,
}

impl<'a> Game<'a> {
    pub fn new(asset_manager: &'a AssetManager) -> Self {
        let mut game = Game {
            win: RenderWindow::new((800, 600), "sft", Style::CLOSE, &Default::default()),
            player: Player::new(),
            enemies: vec![],
            spawn_enemy_timer: 0.,
            bg_obj: Sprite::new(),
            asset_manager,
        };
        game.bg_obj
            .set_texture(asset_manager.get_texture("bg"), true);
        game.win.set_framerate_limit(60);
        game.win.set_vertical_sync_enabled(false);
        game.player.set_texture(asset_manager.get_texture("ship"));
        game
    }

    pub fn run(mut self) {
        'main: loop {
            while let Some(ev) = self.win.poll_event() {
                match ev {
                    Event::Closed => {
                        self.win.close();
                        break 'main;
                    }
                    Event::KeyPressed {
                        code: Key::ESCAPE, ..
                    } => {
                        self.win.close();
                        break 'main;
                    }
                    _ => {}
                }
            }

            self.update();
            self.render();
        }
    }

    fn spawn_enemies(&mut self) {
        if self.spawn_enemy_timer > SPAVN_ENEMY_DELAY {
            if self.enemies.len() < MAX_ENEMIES {
                let mut en = match Game::dice(0, 100) as i32 {
                    40..=60 => {
                        GameObj::new(self.asset_manager.get_texture("go1"), GameObjKind::Iron)
                    }
                    61..=80 => {
                        GameObj::new(self.asset_manager.get_texture("go1"), GameObjKind::Hidrogen)
                    }
                    81..=100 => {
                        GameObj::new(self.asset_manager.get_texture("go1"), GameObjKind::Plasma)
                    }
                    _ => GameObj::new(self.asset_manager.get_texture("go1"), GameObjKind::Default),
                };

                en.set_position((
                    Game::dice(0, self.win.size().x - en.get_bounds().width as u32),
                    0.,
                ));
                self.enemies.push(en);
                self.spawn_enemy_timer = 0.;
            }
        }
    }

    fn update(&mut self) {
        self.player.update(&self.win);
        self.spawn_enemy_timer += 0.1;
        self.spawn_enemies();

        for en in self.enemies.iter_mut() {
            en.update();
        }

        // fire
        if Button::LEFT.is_pressed() {
            self.player.fire(self.asset_manager.get_texture("bang1"));
        }

        // remove object if it intersection with bullet
        for b in self.player.get_bullets() {
            self.enemies
                .retain(|e| e.get_bounds().intersection(&b.get_bounds()).is_none());
        }

        // itneraction object with player
        for e in self.enemies.iter() {
            if e.get_bounds()
                .intersection(&self.player.get_bounds())
                .is_some()
            {
                match e.get_kind() {
                    GameObjKind::Default => {
                        self.player.take_dmg((Game::dice(1, 5) + 5.) as i32);
                    }
                    GameObjKind::Iron => {
                        self.player.repear_hull((Game::dice(1, 8) + 3.) as i32);
                    }
                    GameObjKind::Plasma => {
                        self.player.restore_weapon((Game::dice(1, 10) + 5.) as i32);
                    }
                    GameObjKind::Hidrogen => {
                        self.player.refill_fuel(Game::dice(1, 8) + 2.);
                    }
                }
            }
        }

        // remove objects if they insersect the player
        let player_border = self.player.get_bounds();
        self.enemies
            .retain(|e| e.get_bounds().intersection(&player_border).is_none());

        // remove objects at the bottom of window
        let bottom_border = self.win.size().y as f32;
        self.enemies
            .retain(|e| e.get_bounds().top + e.get_bounds().height < bottom_border);
    }

    fn render(&mut self) {
        self.win.clear(Color::BLACK);
        self.win.draw(&self.bg_obj);
        self.player.render(&mut self.win);

        for en in self.enemies.iter_mut() {
            en.render(&mut self.win);
        }

        self.win.display();
    }

    fn dice(from: u32, to: u32) -> f32 {
        rand::thread_rng().gen_range(from..to) as f32
    }
}
