use sfml::graphics::{FloatRect, RenderTarget, Sprite, Texture, Transformable};

pub struct Bullet<'a> {
    sprite: Sprite<'a>,
    speed: f32,
}

impl<'a> Bullet<'a> {
    pub fn new(texture: &Texture, pos: (f32, f32)) -> Bullet {
        let mut bullet = Bullet {
            sprite: Sprite::new(),
            speed: 5.,
        };

        bullet.sprite.set_texture(texture, true);
        bullet.sprite.set_scale((0.4, 0.4));
        bullet.sprite.set_position(pos);
        bullet
    }

    pub fn get_bounds(&self) -> FloatRect {
        self.sprite.global_bounds()
    }

    pub fn update(&mut self) {
        self.sprite.move_((0., -self.speed));
    }

    pub fn render(&self, target: &mut dyn RenderTarget) {
        target.draw(&self.sprite);
    }
}
