mod economy;
mod strategicmap;

use strategicmap::map as mapmod;
use economy::economy as economymod;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


fn main() {
    economymod::main();
    mapmod::main();
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup, mapmod::setup)
    .add_systems(Update, mapmod::movement)
    .run();
}