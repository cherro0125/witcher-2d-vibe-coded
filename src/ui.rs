/// UI - interfejs użytkownika Bevy (HUD, panele, menu)
use bevy::prelude::*;
use crate::player::Player;
use crate::world::WorldData;
use crate::game_state::*;
use crate::rendering::DayNightCycle;
use crate::alchemy::AlchemyRecipe;
use crate::inventory::ItemType;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameScreen::MainMenu), spawn_main_menu)
            .add_systems(OnExit(GameScreen::MainMenu), despawn_ui::<MainMenuUi>)
            .add_systems(OnEnter(GameScreen::Exploration), (spawn_hud, spawn_minimap))
            .add_systems(Update, (update_hud, update_minimap).run_if(in_state(GameScreen::Exploration)))
            .add_systems(OnExit(GameScreen::Exploration), (despawn_ui::<HudUi>, despawn_ui::<MinimapUi>))
            .add_systems(OnEnter(GameScreen::Dialogue), spawn_dialogue_ui)
            .add_systems(Update, update_dialogue_ui.run_if(in_state(GameScreen::Dialogue)))
            .add_systems(OnExit(GameScreen::Dialogue), despawn_ui::<DialogueUi>)
            .add_systems(OnEnter(GameScreen::Inventory), spawn_inventory_ui)
            .add_systems(Update, update_inventory_ui.run_if(in_state(GameScreen::Inventory)))
            .add_systems(OnExit(GameScreen::Inventory), despawn_ui::<InventoryUi>)
            .add_systems(OnEnter(GameScreen::Alchemy), spawn_alchemy_ui)
            .add_systems(Update, update_alchemy_ui.run_if(in_state(GameScreen::Alchemy)))
            .add_systems(OnExit(GameScreen::Alchemy), despawn_ui::<AlchemyUi>)
            .add_systems(OnEnter(GameScreen::QuestLog), spawn_quest_ui)
            .add_systems(OnExit(GameScreen::QuestLog), despawn_ui::<QuestUi>)
            .add_systems(OnEnter(GameScreen::GameOver), spawn_game_over_ui)
            .add_systems(OnExit(GameScreen::GameOver), despawn_ui::<GameOverUi>);
    }
}

// ===== Marker components =====
#[derive(Component)] struct MainMenuUi;
#[derive(Component)] struct HudUi;
#[derive(Component)] struct HudHealthText;
#[derive(Component)] struct HudStaminaText;
#[derive(Component)] struct HudInfoText;
#[derive(Component)] struct HudLogText;
#[derive(Component)] struct MinimapUi;
#[derive(Component)] struct MinimapPixel(usize, usize); // (map_x, map_y)
#[derive(Component)] struct MinimapPlayerDot;
#[derive(Component)] struct DayNightIndicator;
#[derive(Component)] struct DialogueUi;
#[derive(Component)] struct DialogueContentText;
#[derive(Component)] struct InventoryUi;
#[derive(Component)] struct InventoryContentText;
#[derive(Component)] struct AlchemyUi;
#[derive(Component)] struct AlchemyContentText;
#[derive(Component)] struct QuestUi;
#[derive(Component)] struct GameOverUi;

fn despawn_ui<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in &q { commands.entity(e).despawn_recursive(); }
}

// ==================== MAIN MENU ====================

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.03, 0.08, 1.0)),
        MainMenuUi,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("WIEDŹMIN: SZKOŁA DZIKA"),
            TextFont { font_size: 48.0, ..default() },
            TextColor(Color::srgba(1.0, 0.85, 0.0, 1.0)),
        ));
        parent.spawn((
            Text::new("Gerard z Rumii"),
            TextFont { font_size: 28.0, ..default() },
            TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
            Node { margin: UiRect::top(Val::Px(10.0)), ..default() },
        ));
        parent.spawn((
            Text::new("~ Edycja 3D ~"),
            TextFont { font_size: 22.0, ..default() },
            TextColor(Color::srgba(0.5, 0.8, 1.0, 1.0)),
            Node { margin: UiRect::top(Val::Px(5.0)), ..default() },
        ));
        parent.spawn((
            Text::new("Naciśnij ENTER aby rozpocząć"),
            TextFont { font_size: 22.0, ..default() },
            TextColor(Color::srgba(0.6, 0.9, 0.6, 1.0)),
            Node { margin: UiRect::top(Val::Px(60.0)), ..default() },
        ));
        parent.spawn((
            Text::new("WASD - ruch | E - interakcja | I - ekwipunek | J - questy\n1-5 - znaki | R - odpoczynek | ESC - wyjście"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgba(0.5, 0.5, 0.5, 1.0)),
            Node { margin: UiRect::top(Val::Px(40.0)), ..default() },
        ));
    });
}

// ==================== HUD (Exploration) ====================

fn spawn_hud(mut commands: Commands) {
    // Right panel
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Px(300.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.12, 0.1, 0.15, 0.92)),
        HudUi,
    )).with_children(|parent| {
        // Health
        parent.spawn((
            Text::new("HP: 150/150"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgba(1.0, 0.3, 0.3, 1.0)),
            HudHealthText,
        ));
        // Stamina
        parent.spawn((
            Text::new("Wytrzymałość: 100/100"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgba(0.3, 0.7, 1.0, 1.0)),
            HudStaminaText,
            Node { margin: UiRect::top(Val::Px(4.0)), ..default() },
        ));
        // Info
        parent.spawn((
            Text::new(""),
            TextFont { font_size: 13.0, ..default() },
            TextColor(Color::srgba(0.8, 0.8, 0.6, 1.0)),
            HudInfoText,
            Node { margin: UiRect::top(Val::Px(8.0)), ..default() },
        ));
    });

    // Bottom message log
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            bottom: Val::Px(0.0),
            width: Val::Px(980.0),
            height: Val::Px(110.0),
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.08, 0.06, 0.1, 0.9)),
        HudUi,
    )).with_children(|parent| {
        parent.spawn((
            Text::new(""),
            TextFont { font_size: 13.0, ..default() },
            TextColor(Color::srgba(0.7, 0.7, 0.6, 1.0)),
            HudLogText,
        ));
    });

    // === Crosshair / Celownik (center of screen) ===
    // Horizontal line
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Px(20.0),
            height: Val::Px(2.0),
            margin: UiRect {
                left: Val::Px(-10.0),
                top: Val::Px(-1.0),
                ..default()
            },
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
        HudUi,
    ));
    // Vertical line
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Px(2.0),
            height: Val::Px(20.0),
            margin: UiRect {
                left: Val::Px(-1.0),
                top: Val::Px(-10.0),
                ..default()
            },
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.5)),
        HudUi,
    ));
    // Center dot
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Px(4.0),
            height: Val::Px(4.0),
            margin: UiRect {
                left: Val::Px(-2.0),
                top: Val::Px(-2.0),
                ..default()
            },
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 0.85, 0.0, 0.8)),
        HudUi,
    ));

    // Camera control hint
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
        HudUi,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("PPM + mysz = obrót kamery | Scroll = zoom"),
            TextFont { font_size: 12.0, ..default() },
            TextColor(Color::srgba(0.6, 0.6, 0.6, 0.7)),
        ));
    });
}

fn update_hud(
    player: Res<Player>,
    world: Res<WorldData>,
    log: Res<MessageLog>,
    mut hp_q: Query<&mut Text, (With<HudHealthText>, Without<HudStaminaText>, Without<HudInfoText>, Without<HudLogText>)>,
    mut sta_q: Query<&mut Text, (With<HudStaminaText>, Without<HudHealthText>, Without<HudInfoText>, Without<HudLogText>)>,
    mut info_q: Query<&mut Text, (With<HudInfoText>, Without<HudHealthText>, Without<HudStaminaText>, Without<HudLogText>)>,
    mut log_q: Query<&mut Text, (With<HudLogText>, Without<HudHealthText>, Without<HudStaminaText>, Without<HudInfoText>)>,
) {
    for mut t in &mut hp_q {
        let quen = if player.quen_shield > 0 { format!("  Quen: {}", player.quen_shield) } else { String::new() };
        **t = format!("HP: {}/{}{}",  player.health, player.max_health, quen);
    }
    for mut t in &mut sta_q {
        **t = format!("Wytrzymałość: {}/{}", player.stamina, player.max_stamina);
    }
    for mut t in &mut info_q {
        let loc = world.location_name(player.x, player.y);
        let sign = &player.signs[player.active_sign];
        **t = format!(
            "{} | {}\nPoziom: {} | EXP: {}/{}\nSIŁ: {} ZRC: {} INT: {}\nAtak S/Sr: {}/{} | Obr: {}\nZłoto: {} | Znak: {} ({})\n\nWASD-ruch E-atak/rozmowa I-ekwip\nJ-questy T-alchemia 1-5-znaki R-odp\nPPM+mysz=kamera Scroll=zoom",
            player.name, loc,
            player.level, player.experience, player.exp_to_next_level,
            player.strength, player.dexterity, player.intelligence,
            player.attack_power(false), player.attack_power(true), player.defense_power(),
            player.inventory.gold, sign.name, sign.stamina_cost,
        );
    }
    for mut t in &mut log_q {
        let max_msgs = 6;
        let start = if log.messages.len() > max_msgs { log.messages.len() - max_msgs } else { 0 };
        **t = log.messages[start..].join("\n");
    }
}

// ==================== MINIMAP ====================

const MINIMAP_SIZE: usize = 30; // tiles visible on minimap
const MINIMAP_PIXEL: f32 = 4.0; // pixel size per tile

fn spawn_minimap(mut commands: Commands) {
    // Container
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(30.0),
            width: Val::Px(MINIMAP_SIZE as f32 * MINIMAP_PIXEL + 4.0),
            height: Val::Px(MINIMAP_SIZE as f32 * MINIMAP_PIXEL + 24.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        MinimapUi,
    )).with_children(|parent| {
        // Day/night time label
        parent.spawn((
            Text::new("☀ Poranek"),
            TextFont { font_size: 11.0, ..default() },
            TextColor(Color::srgba(1.0, 0.9, 0.5, 1.0)),
            DayNightIndicator,
            Node { margin: UiRect::bottom(Val::Px(2.0)), ..default() },
        ));

        // Grid container
        parent.spawn((
            Node {
                width: Val::Px(MINIMAP_SIZE as f32 * MINIMAP_PIXEL),
                height: Val::Px(MINIMAP_SIZE as f32 * MINIMAP_PIXEL),
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
        )).with_children(|grid| {
            // Spawn pixels
            for dy in 0..MINIMAP_SIZE {
                for dx in 0..MINIMAP_SIZE {
                    grid.spawn((
                        Node {
                            width: Val::Px(MINIMAP_PIXEL),
                            height: Val::Px(MINIMAP_PIXEL),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 1.0)),
                        MinimapPixel(dx, dy),
                    ));
                }
            }
        });

        // Player dot (overlaid on center)
        parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(2.0 + (MINIMAP_SIZE as f32 / 2.0) * MINIMAP_PIXEL - 1.0),
                top: Val::Px(20.0 + (MINIMAP_SIZE as f32 / 2.0) * MINIMAP_PIXEL - 1.0),
                width: Val::Px(MINIMAP_PIXEL + 2.0),
                height: Val::Px(MINIMAP_PIXEL + 2.0),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 0.85, 0.0, 1.0)),
            MinimapPlayerDot,
        ));
    });
}

fn update_minimap(
    player: Res<Player>,
    world: Res<WorldData>,
    cycle: Res<DayNightCycle>,
    mut pixels: Query<(&MinimapPixel, &mut BackgroundColor)>,
    mut day_text: Query<(&mut Text, &mut TextColor), With<DayNightIndicator>>,
) {
    let px = player.x as i32;
    let py = player.y as i32;
    let half = MINIMAP_SIZE as i32 / 2;

    for (mp, mut bg) in &mut pixels {
        let world_x = px - half + mp.0 as i32;
        let world_y = py - half + mp.1 as i32;

        if world_x < 0 || world_y < 0 || world_x >= world.width as i32 || world_y >= world.height as i32 {
            *bg = BackgroundColor(Color::srgba(0.05, 0.05, 0.08, 1.0));
            continue;
        }

        let tile = world.get_tile(world_x as usize, world_y as usize);
        let c = tile.color_array();

        // Check for monsters/NPCs at this position
        let has_monster = world.monsters.iter().any(|m| m.alive && m.x as i32 == world_x && m.y as i32 == world_y);
        let has_npc = world.npcs.iter().any(|n| n.x as i32 == world_x && n.y as i32 == world_y);

        let color = if has_npc {
            Color::srgba(0.0, 0.8, 1.0, 1.0) // cyan for NPCs
        } else if has_monster {
            Color::srgba(1.0, 0.2, 0.2, 1.0) // red for monsters
        } else {
            Color::srgba(c[0], c[1], c[2], 1.0)
        };

        *bg = BackgroundColor(color);
    }

    // Update day/night indicator
    let tod = cycle.time_of_day;
    for (mut text, mut color) in &mut day_text {
        let (icon, name, tc) = if tod < 0.15 || tod > 0.85 {
            ("🌙", "Noc", Color::srgba(0.5, 0.5, 0.9, 1.0))
        } else if tod < 0.3 {
            ("🌅", "Świt", Color::srgba(1.0, 0.7, 0.4, 1.0))
        } else if tod < 0.45 {
            ("☀", "Poranek", Color::srgba(1.0, 0.9, 0.5, 1.0))
        } else if tod < 0.55 {
            ("☀", "Południe", Color::srgba(1.0, 1.0, 0.8, 1.0))
        } else if tod < 0.7 {
            ("☀", "Popołudnie", Color::srgba(1.0, 0.85, 0.5, 1.0))
        } else {
            ("🌅", "Zmierzch", Color::srgba(0.9, 0.5, 0.3, 1.0))
        };
        **text = format!("{} {}", icon, name);
        *color = TextColor(tc);
    }
}

// ==================== COMBAT ====================

// ==================== DIALOGUE ====================

fn spawn_dialogue_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(30.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 0.92)),
        DialogueUi,
    )).with_children(|parent| {
        parent.spawn((
            Text::new(""),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgba(0.9, 0.9, 0.9, 1.0)),
            DialogueContentText,
        ));
    });
}

fn update_dialogue_ui(
    dlg_data: Option<Res<DialogueData>>,
    mut q: Query<&mut Text, With<DialogueContentText>>,
) {
    let Some(dd) = dlg_data else { return };
    let Some(node) = dd.dialogue.get_current_node() else { return };

    for mut t in &mut q {
        let mut s = format!("{}\n\n{}\n\n--- Odpowiedzi ---", node.speaker, node.text);
        for (i, choice) in node.choices.iter().enumerate() {
            s.push_str(&format!("\n[{}] {}", i + 1, choice.text));
        }
        s.push_str("\n\n[ESC] Zakończ rozmowę");
        **t = s;
    }
}

// ==================== INVENTORY ====================

fn spawn_inventory_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        InventoryUi,
    )).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(500.0),
                max_height: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.06, 0.12, 0.98)),
        )).with_children(|panel| {
            panel.spawn((
                Text::new("⚔ EKWIPUNEK ⚔"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgba(1.0, 0.85, 0.0, 1.0)),
                Node { margin: UiRect::bottom(Val::Px(10.0)), ..default() },
            ));
            panel.spawn((
                Text::new(""),
                TextFont { font_size: 14.0, ..default() },
                TextColor(Color::srgba(0.8, 0.8, 0.7, 1.0)),
                InventoryContentText,
            ));
        });
    });
}

fn update_inventory_ui(
    player: Res<Player>,
    inv_sel: Res<InventorySelection>,
    mut q: Query<&mut Text, With<InventoryContentText>>,
) {
    for mut t in &mut q {
        let mut s = format!("Złoto: {} koron\n\n", player.inventory.gold);

        if player.inventory.items.is_empty() {
            s.push_str("Ekwipunek pusty");
        } else {
            for (i, item) in player.inventory.items.iter().enumerate() {
                let is_equipped = player.inventory.equipped_steel_sword == Some(i)
                    || player.inventory.equipped_silver_sword == Some(i)
                    || player.inventory.equipped_armor == Some(i)
                    || player.inventory.active_oil == Some(i);

                let prefix = if is_equipped { "[E] " } else { "    " };
                let sel = if i == inv_sel.selected { "► " } else { "  " };
                let qty = if item.quantity > 1 { format!(" x{}", item.quantity) } else { String::new() };
                s.push_str(&format!("{}{}{}{}\n", sel, prefix, item.name, qty));
            }

            if inv_sel.selected < player.inventory.items.len() {
                let item = &player.inventory.items[inv_sel.selected];
                s.push_str(&format!("\n--- {} ---\n{}", item.name, item.description));
            }
        }

        s.push_str("\n\n[ENTER] Załóż/Użyj  [↑↓] Nawiguj  [ESC] Zamknij");
        **t = s;
    }
}

// ==================== ALCHEMY ====================

fn spawn_alchemy_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.06, 0.08, 0.05, 0.95)),
        AlchemyUi,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("⚗ ALCHEMIA ⚗"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::srgba(0.4, 1.0, 0.5, 1.0)),
        ));
        parent.spawn((
            Text::new(""),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgba(0.8, 0.8, 0.7, 1.0)),
            AlchemyContentText,
            Node { margin: UiRect::top(Val::Px(10.0)), ..default() },
        ));
    });
}

fn update_alchemy_ui(
    player: Res<Player>,
    alch_sel: Res<AlchemySelection>,
    mut q: Query<&mut Text, With<AlchemyContentText>>,
) {
    let recipes = AlchemyRecipe::all_recipes();

    // Gather ingredients
    let ingredients: Vec<(String, i32)> = player.inventory.items.iter()
        .filter(|i| i.item_type == ItemType::AlchemyIngredient)
        .map(|i| (i.name.clone(), i.quantity))
        .collect();

    for mut t in &mut q {
        let mut s = String::from("Składniki w ekwipunku: ");
        if ingredients.is_empty() {
            s.push_str("brak\n");
        } else {
            let ing_str: Vec<String> = ingredients.iter()
                .map(|(n, q)| format!("{} x{}", n, q))
                .collect();
            s.push_str(&ing_str.join(", "));
            s.push('\n');
        }
        s.push_str("\n--- Receptury ---\n\n");

        for (i, recipe) in recipes.iter().enumerate() {
            let can_craft = crate::alchemy::check_can_craft(recipe, &ingredients);
            let sel = if i == alch_sel.selected { "► " } else { "  " };
            let status = if can_craft { "✓" } else { "✗" };
            let color_hint = if can_craft { "" } else { " (brak składników)" };

            s.push_str(&format!("{}[{}] {} - {}{}\n", sel, status, recipe.name, recipe.description, color_hint));

            if i == alch_sel.selected {
                s.push_str("     Potrzebne: ");
                let reqs: Vec<String> = recipe.ingredients.iter()
                    .map(|(name, qty)| {
                        let have = ingredients.iter()
                            .filter(|(n, _)| n == name)
                            .map(|(_, q)| *q)
                            .sum::<i32>();
                        format!("{} {}/{}", name, have, qty)
                    })
                    .collect();
                s.push_str(&reqs.join(", "));
                s.push_str(&format!("\n     Wynik: {}\n", recipe.result.name));
            }
            s.push('\n');
        }

        s.push_str("\n[ENTER] Wytworz  [↑↓] Wybierz  [ESC/T] Zamknij");
        **t = s;
    }
}

// ==================== QUEST LOG ====================

fn spawn_quest_ui(
    mut commands: Commands,
    quest_log: Res<crate::quests::QuestLog>,
) {
    let mut content = String::from("📜 DZIENNIK QUESTÓW 📜\n\n");

    let active = quest_log.active_quests();
    if !active.is_empty() {
        content.push_str("--- Aktywne ---\n");
        for quest in &active {
            let marker = if quest.is_main_quest { "★ " } else { "• " };
            content.push_str(&format!("{}{}\n", marker, quest.title));
            if let Some(step) = quest.current_objective() {
                content.push_str(&format!("  → {}\n", step.description));
            }
        }
    } else {
        content.push_str("Brak aktywnych questów. Porozmawiaj z NPC!\n");
    }

    let completed = quest_log.completed_quests();
    if !completed.is_empty() {
        content.push_str("\n--- Ukończone ---\n");
        for quest in &completed {
            content.push_str(&format!("✓ {}\n", quest.title));
        }
    }

    content.push_str("\n[ESC] Zamknij dziennik");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.06, 0.05, 0.1, 0.95)),
        QuestUi,
    )).with_children(|parent| {
        parent.spawn((
            Text::new(content),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgba(0.8, 0.8, 0.7, 1.0)),
        ));
    });
}

// ==================== GAME OVER ====================

fn spawn_game_over_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.1, 0.0, 0.0, 0.95)),
        GameOverUi,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("KONIEC GRY\nNaciśnij ENTER aby zacząć od nowa"),
            TextFont { font_size: 32.0, ..default() },
            TextColor(Color::srgba(1.0, 0.2, 0.2, 1.0)),
        ));
    });
}
