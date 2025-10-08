use bevy::prelude::*;
use std::collections::HashMap;

/// Resource containing all loaded symbol textures
#[derive(Resource)]
pub struct SymbolAssets {
    pub cherry: Handle<Image>,
    pub lemon: Handle<Image>,
    pub bell: Handle<Image>,
    pub star: Handle<Image>,
    pub seven: Handle<Image>,
    // Map symbol names to texture handles for dynamic lookup
    pub symbol_map: HashMap<String, Handle<Image>>,
}

/// Resource containing UI textures
#[derive(Resource)]
pub struct UIAssets {
    pub button_normal: Handle<Image>,
    pub button_hovered: Handle<Image>,
    pub button_pressed: Handle<Image>,
    pub slot_frame: Handle<Image>,
    pub background: Handle<Image>,
    pub coin: Handle<Image>,
}

/// Resource containing sound effects
#[derive(Resource)]
pub struct SoundAssets {
    pub spin_sound: Handle<AudioSource>,
    pub win_sound: Handle<AudioSource>,
    pub coin_sound: Handle<AudioSource>,
    pub background_music: Handle<AudioSource>,
}

/// Resource containing fonts
#[derive(Resource)]
pub struct FontAssets {
    pub main_font: Handle<Font>,
}

/// Plugin for loading all game assets
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets);
    }
}

/// System to load all game assets at startup
fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load symbol textures
    let cherry = asset_server.load("textures/symbols/cherry.png");
    let lemon = asset_server.load("textures/symbols/lemon.png");
    let bell = asset_server.load("textures/symbols/bell.png");
    let star = asset_server.load("textures/symbols/star.png");
    let seven = asset_server.load("textures/symbols/seven.png");

    // Create symbol map for dynamic lookup
    let mut symbol_map = HashMap::new();
    symbol_map.insert("Cherry".to_string(), cherry.clone());
    symbol_map.insert("Lemon".to_string(), lemon.clone());
    symbol_map.insert("Bell".to_string(), bell.clone());
    symbol_map.insert("Star".to_string(), star.clone());
    symbol_map.insert("Seven".to_string(), seven.clone());

    let symbol_assets = SymbolAssets {
        cherry,
        lemon,
        bell,
        star,
        seven,
        symbol_map,
    };

    // Load UI textures
    let ui_assets = UIAssets {
        button_normal: asset_server.load("textures/ui/button_normal.png"),
        button_hovered: asset_server.load("textures/ui/button_hovered.png"),
        button_pressed: asset_server.load("textures/ui/button_pressed.png"),
        slot_frame: asset_server.load("textures/ui/slot_frame.png"),
        background: asset_server.load("textures/ui/background.png"),
        coin: asset_server.load("textures/ui/coin.png"),
    };

    // Load sound effects
    let sound_assets = SoundAssets {
        spin_sound: asset_server.load("sounds/spin.wav"),
        win_sound: asset_server.load("sounds/win.wav"),
        coin_sound: asset_server.load("sounds/coin.wav"),
        background_music: asset_server.load("sounds/background.ogg"),
    };

    // Load fonts
    let font_assets = FontAssets {
        main_font: asset_server.load("fonts/slot_font.ttf"),
    };

    // Insert resources
    commands.insert_resource(symbol_assets);
    commands.insert_resource(ui_assets);
    commands.insert_resource(sound_assets);
    commands.insert_resource(font_assets);

    info!("Assets loaded successfully!");
}

/// Helper function to get symbol texture by name
pub fn get_symbol_texture(symbol_assets: &SymbolAssets, symbol_name: &str) -> Option<Handle<Image>> {
    symbol_assets.symbol_map.get(symbol_name).cloned()
}

/// System to check if all assets are loaded
pub fn check_assets_loaded(
    symbol_assets: Option<Res<SymbolAssets>>,
    ui_assets: Option<Res<UIAssets>>,
    sound_assets: Option<Res<SoundAssets>>,
    font_assets: Option<Res<FontAssets>>,
) -> bool {
    symbol_assets.is_some() && ui_assets.is_some() && sound_assets.is_some() && font_assets.is_some()
}