mod economy;
mod strategicmap;

use std::collections::HashMap;
use strategicmap::map as mapmod;
use economy::economy as economymod;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


fn main() {
    let companies = economymod::Companies{iteration:HashMap::new()};
    economymod::main();
    mapmod::main();
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(companies)
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup, mapmod::setup)
    .add_systems(Startup, economymod::trade)
    .add_systems(Update, mapmod::movement)
    .run();
}