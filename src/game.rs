use sfml::{
    graphics::{Color, RenderTarget, RenderWindow, Sprite, Texture},
    window::{Event, Key, Style},
};

use rand::Rng;

const MAX_ENEMIES: usize = 10;
const SPAVN_ENEMY_DELAY: f32 = 10.;

use crate::game_obj::GameObj;
use crate::player::Player;

pub struct Game<'a> {
    win: RenderWindow,
    player: Player<'a>,
    enemy_tex: &'a Texture,
    enemies: Vec<GameObj<'a>>,
    spawn_enemy_timer: f32,
    bg: &'a Texture,
    bg_obj: Sprite<'a>,
}

impl<'a> Game<'a> {
    pub fn new(tex: &'a Texture, go_tex: &'a Texture, bg: &'a Texture) -> Self {
        let mut game = Game {
            win: RenderWindow::new((800, 600), "sft", Style::CLOSE, &Default::default()),
            player: Player::new(),
            enemies: vec![],
            spawn_enemy_timer: 0.,
            enemy_tex: go_tex,
            bg_obj: Sprite::new(),
            bg,
        };
        game.bg_obj.set_texture(&bg, true);
        game.win.set_framerate_limit(60);
        game.win.set_vertical_sync_enabled(false);
        game.player.set_texture(&tex);
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
                let mut en = GameObj::new(&self.enemy_tex);
                en.set_position((self.dice(0, self.win.size().x), 0.));
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

        // remove objects they insersect the player
        let player_border = self.player.get_bounds();
        self.enemies
            .retain(|e| e.get_bounds().intersection(&player_border).is_none());

        // remove objects at the bottom of wndow
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

    fn dice(&self, from: u32, to: u32) -> f32 {
        rand::thread_rng().gen_range(from..to) as f32
    }
}
