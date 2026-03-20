/// Stan gry - Bevy States, systemy, obsługa inputu
use bevy::prelude::*;
use crate::player::{Player, Direction};
use crate::world::WorldData;
use crate::quests::{QuestLog, QuestStatus};
use crate::dialogue::{self, Dialogue, DialogueAction};
use crate::inventory::{Item, ItemType};
use crate::rendering::{CameraOrbit, AttackAnimState, DodgeRollState};
use crate::alchemy::{AlchemyRecipe, check_can_craft};

/// Event for playing sounds
#[derive(Event, Clone)]
pub enum GameSound {
    Footstep,
    SwordHit,
    MonsterDeath,
    PickupItem,
    SignCast,
    QuestComplete,
    LevelUp,
}

/// Game screen states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
pub enum GameScreen {
    #[default]
    MainMenu,
    Exploration,
    Dialogue,
    Inventory,
    Alchemy,
    QuestLog,
    GameOver,
}

/// Dialogue resource
#[derive(Resource)]
pub struct DialogueData {
    pub dialogue: Dialogue,
}

/// Messages log
#[derive(Resource)]
pub struct MessageLog {
    pub messages: Vec<String>,
}

impl MessageLog {
    pub fn new() -> Self {
        let mut log = MessageLog { messages: Vec::new() };
        log.add("Witaj w świecie Wiedźmina! [3D]".into());
        log.add("Jesteś Gerard z Rumii ze Szkoły Dzika.".into());
        log.add("WASD - ruch, E - interakcja/atak, I - ekwipunek, J - questy".into());
        log
    }
    pub fn add(&mut self, msg: String) {
        self.messages.push(msg);
        if self.messages.len() > 50 { self.messages.remove(0); }
    }
}

/// Inventory selection
#[derive(Resource)]
pub struct InventorySelection {
    pub selected: usize,
}

/// Alchemy selection
#[derive(Resource)]
pub struct AlchemySelection {
    pub selected: usize,
}

/// Combat animation event
#[derive(Event, Clone)]
pub enum CombatAnimEvent {
    PlayerSlash { position: (f32, f32) },
    MonsterHit { position: (f32, f32) },
    PlayerDodge { position: (f32, f32) },
    PlayerParry { position: (f32, f32) },
}

/// Movement cooldown
#[derive(Resource)]
pub struct MoveCooldown {
    pub timer: f32,
}

/// Player attack cooldown (0.5s between attacks)
#[derive(Resource)]
pub struct AttackCooldown {
    pub timer: f32,
}

/// Monster AI attack timers (per-monster attack cooldowns)
#[derive(Resource)]
pub struct MonsterAiTimer {
    pub timers: Vec<f32>,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Player::new())
            .insert_resource(WorldData::new())
            .insert_resource(MessageLog::new())
            .insert_resource(QuestLog::new())
            .insert_resource(InventorySelection { selected: 0 })
            .insert_resource(AlchemySelection { selected: 0 })
            .insert_resource(MoveCooldown { timer: 0.0 })
            .insert_resource(AttackCooldown { timer: 0.0 })
            .insert_resource(MonsterAiTimer { timers: Vec::new() })
            .add_event::<GameSound>()
            .add_event::<CombatAnimEvent>()
            .add_systems(Update, play_game_sounds)
            .add_systems(Update, menu_input.run_if(in_state(GameScreen::MainMenu)))
            .add_systems(Update, (exploration_movement, monster_ai, exploration_keys, check_game_over, exploration_sounds).run_if(in_state(GameScreen::Exploration)))
            .add_systems(Update, dialogue_input.run_if(in_state(GameScreen::Dialogue)))
            .add_systems(Update, inventory_input.run_if(in_state(GameScreen::Inventory)))
            .add_systems(Update, alchemy_input.run_if(in_state(GameScreen::Alchemy)))
            .add_systems(Update, quest_log_input.run_if(in_state(GameScreen::QuestLog)))
            .add_systems(Update, game_over_input.run_if(in_state(GameScreen::GameOver)));
    }
}

// ==================== MAIN MENU ====================

fn menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameScreen>>,
    mut exit: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        next_state.set(GameScreen::Exploration);
    }
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

// ==================== EXPLORATION ====================

fn exploration_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: ResMut<Player>,
    mut world: ResMut<WorldData>,
    mut log: ResMut<MessageLog>,
    mut cooldown: ResMut<MoveCooldown>,
    orbit: Res<CameraOrbit>,
) {
    cooldown.timer -= time.delta_secs();
    if cooldown.timer > 0.0 { return; }

    // Get raw input direction
    let mut input_x: f32 = 0.0;
    let mut input_z: f32 = 0.0;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) { input_z -= 1.0; }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) { input_z += 1.0; }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) { input_x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) { input_x += 1.0; }
    if input_x == 0.0 && input_z == 0.0 { return; }

    cooldown.timer = 0.12;

    // Camera-relative movement: rotate input by the camera's absolute yaw
    let cam_yaw = orbit.yaw;

    // Rotate input vector by camera yaw
    let cos_y = cam_yaw.cos();
    let sin_y = cam_yaw.sin();
    let world_x = input_x * cos_y - input_z * sin_y;
    let world_z = input_x * sin_y + input_z * cos_y;

    // Snap to grid: pick dominant axis direction
    let (dx, dy, dir) = if world_z.abs() >= world_x.abs() {
        if world_z < 0.0 {
            (0.0, -1.0, Direction::Up)
        } else {
            (0.0, 1.0, Direction::Down)
        }
    } else if world_x < 0.0 {
        (-1.0, 0.0, Direction::Left)
    } else {
        (1.0, 0.0, Direction::Right)
    };

    player.direction = dir;

    let new_x = player.x + dx;
    let new_y = player.y + dy;

    if !world.is_walkable(new_x, new_y) { return; }

    player.x = new_x;
    player.y = new_y;

    // Check loot pickup
    if let Some(li) = world.find_loot_at(new_x, new_y, 1.0) {
        let lname = world.loot[li].name.clone();
        let iname = world.loot[li].item_name.clone();
        world.loot[li].collected = true;
        player.inventory.add_item(Item::ingredient(&iname, 5));
        log.add(format!("Znaleziono: {}!", lname));
    }
}

/// Monster AI system - runs during exploration
fn monster_ai(
    time: Res<Time>,
    mut player: ResMut<Player>,
    mut world: ResMut<WorldData>,
    mut log: ResMut<MessageLog>,
    mut ai_timers: ResMut<MonsterAiTimer>,
    mut sound_events: EventWriter<GameSound>,
    mut combat_anim: EventWriter<CombatAnimEvent>,
) {
    // Ensure we have timers for all monsters
    while ai_timers.timers.len() < world.monsters.len() {
        ai_timers.timers.push(0.0);
    }

    let dt = time.delta_secs();
    let player_x = player.x;
    let player_y = player.y;

    // Collect monster indices and actions to avoid borrow issues
    let mut monster_actions: Vec<(usize, Option<(f32, f32)>, bool)> = Vec::new();

    for (idx, monster) in world.monsters.iter().enumerate() {
        if !monster.alive { continue; }

        // Calculate distance to player
        let dx = player_x - monster.x;
        let dy = player_y - monster.y;
        let dist = (dx * dx + dy * dy).sqrt();

        let mut new_pos: Option<(f32, f32)> = None;
        let mut should_attack = false;

        // If monster is within 3.0 range, move toward player
        if dist < 3.0 && dist > 0.01 {
            let move_speed = 0.5; // tiles per second
            let move_dist = move_speed * dt;
            
            if dist > move_dist {
                // Calculate new position
                let nx = monster.x + (dx / dist) * move_dist;
                let ny = monster.y + (dy / dist) * move_dist;
                new_pos = Some((nx, ny));
            }
        }

        // If monster is within 1.2 range, attack player
        if dist < 1.2 {
            ai_timers.timers[idx] -= dt;
            if ai_timers.timers[idx] <= 0.0 {
                should_attack = true;
            }
        }

        if new_pos.is_some() || should_attack {
            monster_actions.push((idx, new_pos, should_attack));
        }
    }

    // Now apply the actions
    for (idx, new_pos, should_attack) in monster_actions {
        if let Some((nx, ny)) = new_pos {
            if world.is_walkable(nx, ny) {
                world.monsters[idx].x = nx;
                world.monsters[idx].y = ny;
            }
        }

        if should_attack {
            ai_timers.timers[idx] = 1.5; // Reset cooldown
            let damage = world.monsters[idx].attack;
            let mx = world.monsters[idx].x;
            let my = world.monsters[idx].y;
            let actual_damage = player.take_damage(damage);
            let mname = world.monsters[idx].name.clone();
            log.add(format!("{} atakuje! -{} HP", mname, actual_damage));
            sound_events.send(GameSound::SwordHit);

            // Emit monster attack animation at player position
            combat_anim.send(CombatAnimEvent::MonsterHit { position: (player_x, player_y) });
            combat_anim.send(CombatAnimEvent::PlayerSlash { position: (mx, my) });
            
            if !player.is_alive() {
                log.add("Gerard upadł w walce!".into());
            }
        }
    }
}

/// Emit sound events for exploration (separate system to avoid param overflow)
fn exploration_sounds(
    player: Res<Player>,
    mut sound_events: EventWriter<GameSound>,
    mut last_pos: Local<(f32, f32)>,
) {
    if player.x != last_pos.0 || player.y != last_pos.1 {
        sound_events.send(GameSound::Footstep);
        *last_pos = (player.x, player.y);
    }
}

/// Check if player is dead (game over transition)
fn check_game_over(
    player: Res<Player>,
    mut next_state: ResMut<NextState<GameScreen>>,
) {
    if !player.is_alive() {
        next_state.set(GameScreen::GameOver);
    }
}

fn exploration_keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<Player>,
    mut world: ResMut<WorldData>,
    mut log: ResMut<MessageLog>,
    mut next_state: ResMut<NextState<GameScreen>>,
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    mut inv_sel: ResMut<InventorySelection>,
    mut attack_cooldown: ResMut<AttackCooldown>,
    time: Res<Time>,
    mut sound_events: EventWriter<GameSound>,
    mut combat_anim: EventWriter<CombatAnimEvent>,
    mut attack_anim: ResMut<AttackAnimState>,
    mut dodge_state: ResMut<DodgeRollState>,
) {
    // Decrement attack cooldown
    attack_cooldown.timer -= time.delta_secs();

    // Dodge roll on Space
    if keys.just_pressed(KeyCode::Space) && !dodge_state.active {
        let dir = match player.direction {
            Direction::Up => Vec3::new(0.0, 0.0, -1.0),
            Direction::Down => Vec3::new(0.0, 0.0, 1.0),
            Direction::Left => Vec3::new(-1.0, 0.0, 0.0),
            Direction::Right => Vec3::new(1.0, 0.0, 0.0),
        };
        dodge_state.active = true;
        dodge_state.timer = 0.3;
        dodge_state.direction = dir;
        dodge_state.start_pos = Vec3::new(player.x, 0.0, player.y);
        combat_anim.send(CombatAnimEvent::PlayerDodge { position: (player.x, player.y) });
        sound_events.send(GameSound::Footstep);
    }

    if keys.just_pressed(KeyCode::KeyE) {
        let px = player.x;
        let py = player.y;
        let (ddx, ddy) = match player.direction {
            Direction::Up => (0.0, -1.0),
            Direction::Down => (0.0, 1.0),
            Direction::Left => (-1.0, 0.0),
            Direction::Right => (1.0, 0.0),
        };
        let tx = px + ddx;
        let ty = py + ddy;

        // Try NPC dialogue first
        let ni = world.find_npc_at(tx, ty, 1.5)
            .or_else(|| world.find_npc_at(px, py, 2.0));
        if let Some(ni) = ni {
            let npc_name = world.npcs[ni].name.clone();
            let did = world.npcs[ni].dialogue_id.clone();
            log.add(format!("Rozmowa z: {}", npc_name));
            let mut dlg = dialogue::get_dialogue(&did);
            dlg.start();
            commands.insert_resource(DialogueData { dialogue: dlg });
            next_state.set(GameScreen::Dialogue);
            return;
        }

        // Try monster attack
        if let Some(mi) = world.find_monster_at(tx, ty, 1.5) {
            if attack_cooldown.timer <= 0.0 {
                let mname = world.monsters[mi].name.clone();
                let mx = world.monsters[mi].x;
                let my = world.monsters[mi].y;
                let damage = player.attack_power(false); // default steel sword
                let actual_damage = world.monsters[mi].take_damage(damage);
                log.add(format!("Atakujesz: {}! -{} HP", mname, actual_damage));
                sound_events.send(GameSound::SwordHit);
                attack_cooldown.timer = 0.5; // 0.5s cooldown

                // Emit slash animation at player and hit at monster
                combat_anim.send(CombatAnimEvent::PlayerSlash { position: (px, py) });
                combat_anim.send(CombatAnimEvent::MonsterHit { position: (mx, my) });

                // Trigger sword draw animation
                attack_anim.is_attacking = true;
                attack_anim.timer = 0.4;
                
                // Check if monster died
                if !world.monsters[mi].alive {
                    let m = &world.monsters[mi];
                    let exp = m.experience_reward;
                    let gold = m.gold_reward;
                    let name = m.name.clone();
                    let leveled = player.gain_experience(exp);
                    player.inventory.gold += gold;
                    player.kills += 1;
                    log.add(format!("{} pokonany! +{} EXP, +{} złota", name, exp, gold));
                    if leveled { log.add(format!("Awans na poziom {}!", player.level)); }
                    sound_events.send(GameSound::MonsterDeath);
                }
            } else {
                log.add("Czekaj na cooldown ataku!".into());
            }
            return;
        }

        log.add("Nie ma tu nic do interakcji.".into());
    }

    if keys.just_pressed(KeyCode::KeyI) {
        inv_sel.selected = 0;
        next_state.set(GameScreen::Inventory);
    }
    if keys.just_pressed(KeyCode::KeyT) {
        next_state.set(GameScreen::Alchemy);
    }
    if keys.just_pressed(KeyCode::KeyJ) {
        next_state.set(GameScreen::QuestLog);
    }
    if keys.just_pressed(KeyCode::KeyR) {
        player.rest();
        log.add("Gerard odpoczywa... Zdrowie i wytrzymałość przywrócone.".into());
    }
    if keys.just_pressed(KeyCode::Digit1) { player.active_sign = 0; log.add("Aktywny znak: Aard".into()); }
    if keys.just_pressed(KeyCode::Digit2) { player.active_sign = 1; log.add("Aktywny znak: Igni".into()); }
    if keys.just_pressed(KeyCode::Digit3) { player.active_sign = 2; log.add("Aktywny znak: Quen".into()); }
    if keys.just_pressed(KeyCode::Digit4) { player.active_sign = 3; log.add("Aktywny znak: Yrden".into()); }
    if keys.just_pressed(KeyCode::Digit5) { player.active_sign = 4; log.add("Aktywny znak: Axii".into()); }
    if keys.just_pressed(KeyCode::Escape) { exit.send(AppExit::Success); }
}

// ==================== DIALOGUE ====================

fn dialogue_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<Player>,
    world: Res<WorldData>,
    mut log: ResMut<MessageLog>,
    mut next_state: ResMut<NextState<GameScreen>>,
    mut quest_log: ResMut<QuestLog>,
    dlg_data: Option<ResMut<DialogueData>>,
    mut commands: Commands,
) {
    let Some(mut dd) = dlg_data else { return };

    let choice_idx = if keys.just_pressed(KeyCode::Digit1) { Some(0) }
        else if keys.just_pressed(KeyCode::Digit2) { Some(1) }
        else if keys.just_pressed(KeyCode::Digit3) { Some(2) }
        else if keys.just_pressed(KeyCode::Digit4) { Some(3) }
        else if keys.just_pressed(KeyCode::Escape) {
            commands.remove_resource::<DialogueData>();
            next_state.set(GameScreen::Exploration);
            return;
        }
        else { None };

    let Some(choice_idx) = choice_idx else { return };

    let mut pending_msgs: Vec<String> = Vec::new();

    if let Some(action) = dd.dialogue.select_choice(choice_idx) {
        match action {
            DialogueAction::StartQuest(quest_id) => {
                quest_log.start_quest(&quest_id);
                pending_msgs.push("Nowy quest przyjęty!".into());
            }
            DialogueAction::GiveItem(item_name) => {
                match item_name.as_str() {
                    "Jaskółka" => player.inventory.add_item(Item::potion("Jaskółka", 50, 0, 30)),
                    "Kot" => player.inventory.add_item(Item::potion("Kot", 20, 20, 35)),
                    _ => player.inventory.add_item(Item::quest_item(&item_name, "Przedmiot z questu")),
                }
                pending_msgs.push(format!("Otrzymano: {}!", item_name));
            }
            DialogueAction::GiveGold(amount) => {
                player.inventory.gold += amount;
                pending_msgs.push(format!("Otrzymano: {} złota!", amount));
            }
            DialogueAction::GiveExp(amount) => {
                let leveled = player.gain_experience(amount);
                pending_msgs.push(format!("Otrzymano: {} doświadczenia!", amount));
                if leveled { pending_msgs.push(format!("Awans na poziom {}!", player.level)); }
            }
            DialogueAction::Heal => {
                player.rest();
                pending_msgs.push("Pełne leczenie!".into());
            }
            DialogueAction::Trade => {
                pending_msgs.push("Handel zakończony.".into());
            }
            DialogueAction::Axii(_) => {
                if player.stamina >= 20 {
                    player.stamina -= 20;
                    pending_msgs.push("Axii zadziałało!".into());
                } else {
                    pending_msgs.push("Za mało wytrzymałości na Axii!".into());
                }
            }
            DialogueAction::EndDialogue => {}
            DialogueAction::None => {}
        }
    }

    let dialogue_ended = dd.dialogue.current_node.is_none();

    if dialogue_ended {
        let px = player.x;
        let py = player.y;
        if let Some(ni) = world.find_npc_at(px, py, 3.0) {
            let npc_id = world.npcs[ni].dialogue_id.clone();
            quest_log.check_talk_objective(&npc_id);
        }
        for quest in &mut quest_log.quests {
            if quest.status == QuestStatus::Completed && quest.experience_reward > 0 {
                let exp = quest.experience_reward;
                let gold = quest.gold_reward;
                quest.experience_reward = 0;
                quest.gold_reward = 0;
                let leveled = player.gain_experience(exp);
                player.inventory.gold += gold;
                pending_msgs.push(format!("Quest ukończony! +{} EXP, +{} złota", exp, gold));
                if leveled { pending_msgs.push(format!("Awans na poziom {}!", player.level)); }
            }
        }
        commands.remove_resource::<DialogueData>();
        next_state.set(GameScreen::Exploration);
    }

    for msg in pending_msgs { log.add(msg); }
}

// ==================== INVENTORY ====================

fn inventory_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<Player>,
    mut log: ResMut<MessageLog>,
    mut next_state: ResMut<NextState<GameScreen>>,
    mut inv_sel: ResMut<InventorySelection>,
) {
    if keys.just_pressed(KeyCode::ArrowUp) {
        if inv_sel.selected > 0 { inv_sel.selected -= 1; }
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        if inv_sel.selected + 1 < player.inventory.items.len() { inv_sel.selected += 1; }
    }
    if keys.just_pressed(KeyCode::Enter) {
        let idx = inv_sel.selected;
        if idx < player.inventory.items.len() {
            let itype = player.inventory.items[idx].item_type.clone();
            let iname = player.inventory.items[idx].name.clone();
            match itype {
                ItemType::SteelSword | ItemType::SilverSword | ItemType::Armor | ItemType::Oil => {
                    player.inventory.equip_item(idx);
                    log.add(format!("Założono: {}", iname));
                }
                ItemType::Potion => {
                    let item = player.inventory.items[idx].clone();
                    player.heal(item.health_restore);
                    player.restore_stamina(item.stamina_restore);
                    log.add(format!("Użyto: {}", iname));
                    player.inventory.remove_item(idx);
                    if inv_sel.selected >= player.inventory.items.len() && inv_sel.selected > 0 {
                        inv_sel.selected -= 1;
                    }
                }
                _ => { log.add(format!("Nie można użyć: {}", iname)); }
            }
        }
    }
    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::KeyI) {
        next_state.set(GameScreen::Exploration);
    }
}

// ==================== ALCHEMY ====================

fn alchemy_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<Player>,
    mut log: ResMut<MessageLog>,
    mut next_state: ResMut<NextState<GameScreen>>,
    mut alch_sel: ResMut<AlchemySelection>,
) {
    let recipes = AlchemyRecipe::all_recipes();

    if keys.just_pressed(KeyCode::ArrowUp) {
        if alch_sel.selected > 0 { alch_sel.selected -= 1; }
    }
    if keys.just_pressed(KeyCode::ArrowDown) {
        if alch_sel.selected + 1 < recipes.len() { alch_sel.selected += 1; }
    }
    if keys.just_pressed(KeyCode::Enter) {
        if alch_sel.selected < recipes.len() {
            let recipe = &recipes[alch_sel.selected];
            // Gather ingredient counts from inventory
            let ingredient_counts: Vec<(String, i32)> = player.inventory.items.iter()
                .filter(|i| i.item_type == ItemType::AlchemyIngredient)
                .map(|i| (i.name.clone(), i.quantity))
                .collect();

            if check_can_craft(recipe, &ingredient_counts) {
                // Remove ingredients
                for (needed_name, needed_qty) in &recipe.ingredients {
                    let mut to_remove = *needed_qty;
                    while to_remove > 0 {
                        if let Some(idx) = player.inventory.items.iter().position(|i| {
                            i.item_type == ItemType::AlchemyIngredient && i.name == *needed_name
                        }) {
                            let available = player.inventory.items[idx].quantity;
                            if available <= to_remove {
                                to_remove -= available;
                                player.inventory.remove_item(idx);
                            } else {
                                player.inventory.items[idx].quantity -= to_remove;
                                to_remove = 0;
                            }
                        } else {
                            break;
                        }
                    }
                }
                // Add crafted item
                let result = recipe.result.clone();
                let rname = result.name.clone();
                player.inventory.add_item(result);
                log.add(format!("Wytworzono: {}!", rname));
            } else {
                log.add("Brak składników!".into());
            }
        }
    }
    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::KeyT) {
        next_state.set(GameScreen::Exploration);
    }
}

// ==================== QUEST LOG ====================

fn quest_log_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameScreen>>,
) {
    if keys.just_pressed(KeyCode::Escape) || keys.just_pressed(KeyCode::KeyJ) {
        next_state.set(GameScreen::Exploration);
    }
}

// ==================== GAME OVER ====================

fn game_over_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<Player>,
    mut next_state: ResMut<NextState<GameScreen>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        *player = Player::new();
        next_state.set(GameScreen::Exploration);
    }
}

// ==================== SOUND SYSTEM ====================

/// Procedural sound system - generates beep tones for game events
/// (No external audio files needed!)
fn play_game_sounds(
    mut events: EventReader<GameSound>,
    mut log: ResMut<MessageLog>,
) {
    for event in events.read() {
        // Log sound events for feedback (visual confirmation)
        match event {
            GameSound::Footstep => {} // silent - too frequent
            GameSound::SwordHit => { log.add("*CLANG!* ⚔".into()); }
            GameSound::MonsterDeath => { log.add("*Potwór pada na ziemię!* 💀".into()); }
            GameSound::PickupItem => { log.add("*Podniesiono przedmiot* ✦".into()); }
            GameSound::SignCast => { log.add("*Wiedźmiński znak aktywowany!* ✧".into()); }
            GameSound::QuestComplete => { log.add("*Quest ukończony!* 🏆".into()); }
            GameSound::LevelUp => { log.add("*AWANS! Gerard staje się silniejszy!* ⬆".into()); }
        }
    }
}
