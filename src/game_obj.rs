use sfml::graphics::{FloatRect, RenderTarget, Sprite, Texture, Transformable};

#[derive(Clone)]
pub struct GameObj<'a> {
    sprite: Sprite<'a>,
    speed: f32,
}

impl<'a> GameObj<'a> {
    pub fn new(texture: &'a Texture) -> Self {
        let mut enemy = Self {
            sprite: Sprite::with_texture(&texture),
            speed: 4.,
        };
        enemy.sprite.set_scale((0.1, 0.1));
        enemy
    }

    pub fn get_bounds(&self) -> FloatRect {
        self.sprite.global_bounds()
    }

    pub fn set_position(&mut self, pos: (f32, f32)) {
        self.sprite.set_position(pos);
    }

    pub fn update(&mut self) {
        self.sprite.move_((0., self.speed));
    }

    pub fn render(&self, target: &mut dyn RenderTarget) {
        target.draw(&self.sprite);
    }
}
