mod economy;
mod strategicmap;
mod userinterface;

use std::collections::HashMap;
use strategicmap::map as mapmod;
use economy::economy as economymod;
use userinterface::ui::{self as interface, Escape};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::window::{PresentMode, Window, WindowMode};


fn main() {
    let companies = economymod::Companies{iteration:HashMap::new()};
    economymod::main();
    mapmod::main();
    App::new()
    .insert_resource(companies)
    .insert_resource(Escape{isclicked: false})
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::Fullscreen, // Ustawienie pełnoekranowego trybu
            present_mode: PresentMode::AutoVsync, // Płynniejsze odświeżanie
            ..default()
        }),
        ..default()
    }))
    .add_plugins(WorldInspectorPlugin::new())
    .add_systems(Startup, (mapmod::setup, interface::ui_setup))
    .add_systems(Startup, economymod::trade)
    .add_systems(Update, (mapmod::movement, interface::esc, interface::escmenu, interface::exit_system))
    .run();
}