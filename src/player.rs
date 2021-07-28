use sfml::{
    graphics::{FloatRect, RenderTarget, Sprite, Texture, Transformable},
    window::Key,
};

pub struct Player<'a> {
    sprite: Sprite<'a>,
    speed: f32,
}

impl<'a> Player<'a> {
    pub fn new() -> Self {
        let mut player = Player {
            sprite: Sprite::new(),
            speed: 4.,
        };
        player.sprite.set_position((100., 100.));
        player.sprite.set_scale((0.1, 0.1));
        player
    }

    pub fn set_texture(&mut self, texture: &'a Texture) {
        self.sprite.set_texture(&texture, true);
    }

    pub fn update(&mut self, target: &dyn RenderTarget) {
        self.move_handler();
        self.check_win_bounds(target);
    }

    pub fn render(&self, target: &mut dyn RenderTarget) {
        target.draw(&self.sprite);
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
}
