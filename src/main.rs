use assets::AssetManager;
use game::Game;

mod assets;
mod game;
mod game_obj;
mod player;

// TODO bullet struct and logic
// TODO objects kind (meteor fuel ammo health ...?)
// TODO gui (hp bar, fuel bar ammo bar disnace to finish???)
// TODO main menu (replay quit setting)
// TODO audio for shoot and colision + bg sound
// TODO score (max distance?) and save score to file

fn main() {
    let mut asset_manager = AssetManager::new();
    asset_manager.load_textures(assets::TEXTURES);

    let new_game = Game::new(&asset_manager);
    new_game.run();
}
