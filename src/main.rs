/// Wiedźmin: Szkoła Dzika - Gra RPG 3D (Bevy)
/// Główna postać: Gerard z Rumii ze szkoły Dzika

mod player;
mod signs;
mod inventory;
mod monsters;
mod world;
mod combat;
mod alchemy;
mod quests;
mod dialogue;
mod rendering;
mod ui;
mod game_state;

use bevy::prelude::*;
use game_state::{GameScreen, GamePlugin};
use rendering::RenderingPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Wiedźmin: Szkoła Dzika - Gerard z Rumii [3D]".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameScreen>()
        .add_plugins((GamePlugin, RenderingPlugin, UiPlugin))
        .run();
}
