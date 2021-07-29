use sfml::{
    graphics::{FloatRect, RenderTarget, Sprite, Texture, Transformable},
    window::Key,
};

use crate::bullet::Bullet;

const MAX_BULLETS: usize = 20;
const FIRE_KD: f32 = 30.;

pub struct Player<'a> {
    sprite: Sprite<'a>,
    speed: f32,
    bullets: Vec<Bullet<'a>>,
    fire_kd: f32,
    bullet_count: usize,
}

impl<'a> Player<'a> {
    pub fn new() -> Self {
        let mut player = Player {
            sprite: Sprite::new(),
            speed: 4.,
            bullets: vec![],
            fire_kd: 0.,
            bullet_count: MAX_BULLETS,
        };
        player.sprite.set_position((100., 100.));
        player.sprite.set_scale((0.1, 0.1));
        player
    }

    pub fn fire(&mut self, texture: &'a Texture) {
        if self.fire_kd > FIRE_KD {
            if self.bullet_count > 0 {
                self.bullets.push(Bullet::new(
                    texture,
                    (
                        (self.get_bounds().left + self.get_bounds().width / 2.) - 7.,
                        self.get_bounds().top,
                    ),
                ));
                self.fire_kd = 0.;
            }
        }
    }

    pub fn set_texture(&mut self, texture: &'a Texture) {
        self.sprite.set_texture(&texture, true);
    }

    pub fn update(&mut self, target: &dyn RenderTarget) {
        self.move_handler();
        self.check_win_bounds(target);
        self.fire_kd += 1.;

        for b in self.bullets.iter_mut() {
            b.update();
        }

        // remove if out of window
        self.bullets.retain(|b| b.get_bounds().top > 0.);
    }

    pub fn render(&mut self, target: &mut dyn RenderTarget) {
        target.draw(&self.sprite);
        for b in self.bullets.iter_mut() {
            b.render(target);
        }
    }

    pub fn get_bounds(&self) -> FloatRect {
        return self.sprite.global_bounds();
    }

    fn move_handler(&mut self) {
        if Key::D.is_pressed() {
            self.sprite.move_((self.speed, 0.));
        }

        if Key::A.is_pressed() {
            self.sprite.move_((-self.speed, 0.));
        }

        if Key::W.is_pressed() {
            self.sprite.move_((0., -self.speed));
        }

        if Key::S.is_pressed() {
            self.sprite.move_((0., self.speed));
        }
    }

    fn check_win_bounds(&mut self, target: &dyn RenderTarget) {
        if self.sprite.global_bounds().left < 0.0 {
            self.sprite
                .set_position((0., self.sprite.global_bounds().top));
        }

        if self.sprite.global_bounds().top < 0.0 {
            self.sprite
                .set_position((self.sprite.global_bounds().left, 0.));
        }

        if self.sprite.global_bounds().top + self.sprite.global_bounds().height
            > target.size().y as f32
        {
            self.sprite.set_position((
                self.sprite.global_bounds().left,
                target.size().y as f32 - self.sprite.global_bounds().height,
            ));
        }

        if self.sprite.global_bounds().left + self.sprite.global_bounds().width
            > target.size().x as f32
        {
            self.sprite.set_position((
                target.size().x as f32 - self.sprite.global_bounds().width,
                self.sprite.global_bounds().top,
            ));
        }
    }

    pub fn get_bullets(&self) -> &Vec<Bullet<'a>> {
        &self.bullets
    }
}
