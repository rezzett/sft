use assets::AssetManager;
use game::Game;

mod assets;
mod bullet;
mod game;
mod game_obj;
mod player;

// TODO add texture for different kins of obj
// TODO gui (hp bar, fuel bar ammo bar disnace to finish???)
// TODO main menu (replay quit setting)
// TODO audio for shoot and colision + bg sound
// TODO score (max distance?) and save score to file
// TODO load/save settings asset data from file

fn main() {
    let mut asset_manager = AssetManager::new();
    asset_manager.load_textures(assets::TEXTURES);

    let new_game = Game::new(&asset_manager);
    new_game.run();
}
