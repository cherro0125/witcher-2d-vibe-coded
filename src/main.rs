/// Wiedźmin: Szkoła Dzika - Gra RPG
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

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;
use ggez::ContextBuilder;

use game_state::GameState;
use rendering::{SCREEN_WIDTH, SCREEN_HEIGHT};

fn main() {
    let (ctx, event_loop) = ContextBuilder::new("witcher_game", "Szkoła Dzika")
        .window_setup(WindowSetup::default().title("Wiedźmin: Szkoła Dzika - Gerard z Rumii"))
        .window_mode(
            WindowMode::default()
                .dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
                .resizable(false),
        )
        .build()
        .expect("Nie udało się zainicjalizować ggez!");

    let state = GameState::new();
    event::run(ctx, event_loop, state);
}
