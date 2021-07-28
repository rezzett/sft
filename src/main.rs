use game::Game;
use sfml::graphics::Texture;

mod game;
mod game_obj;
mod player;

fn main() {
    let tex = Texture::from_file("ship.png").unwrap();
    let go_tex = Texture::from_file("go1.png").unwrap();
    let bg = Texture::from_file("bg.jpg").unwrap();
    let new_game = Game::new(&tex, &go_tex, &bg);
    new_game.run();
}
