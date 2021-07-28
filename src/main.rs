use game::Game;
use sfml::graphics::Texture;

mod game;
mod game_obj;
mod player;

// TODO resource manger
// TODO bullet struct and logic
// TODO objects kind (meteor fuel ammo health ...?)
// TODO gui (hp bar, fuel bar ammo bar disnace to finish???)
// TODO main menu (replay quit setting)
// TODO audio for shoot and colision + bg sound
// TODO score (max distance?) and save score to file

fn main() {
    let tex = Texture::from_file("ship.png").unwrap();
    let go_tex = Texture::from_file("go1.png").unwrap();
    let bg = Texture::from_file("bg.jpg").unwrap();
    let new_game = Game::new(&tex, &go_tex, &bg);
    new_game.run();
}
