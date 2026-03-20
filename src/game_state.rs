use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color, PxScale};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};

use crate::player::{Player, Direction};
use crate::world::WorldMap;
use crate::combat::{Combat, CombatAction, CombatState};
use crate::quests::{QuestLog, QuestStatus};
use crate::dialogue::{self, Dialogue, DialogueAction};
use crate::inventory::{Item, ItemType};
use crate::rendering::{self, Camera, SCREEN_WIDTH};
use crate::ui;

#[derive(Debug, Clone, PartialEq)]
pub enum GameScreen {
    Exploration,
    Combat(usize),
    Dialogue,
    Inventory,
    QuestLog,
    GameOver,
    MainMenu,
}

pub struct GameState {
    pub player: Player,
    pub world: WorldMap,
    pub camera: Camera,
    pub screen: GameScreen,
    pub combat: Option<Combat>,
    pub quest_log: QuestLog,
    pub dialogue: Option<Dialogue>,
    pub messages: Vec<String>,
    pub inventory_selected: usize,
    pub move_cooldown: f32,
    pub started: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut gs = GameState {
            player: Player::new(),
            world: WorldMap::new(),
            camera: Camera::new(),
            screen: GameScreen::MainMenu,
            combat: None,
            quest_log: QuestLog::new(),
            dialogue: None,
            messages: Vec::new(),
            inventory_selected: 0,
            move_cooldown: 0.0,
            started: false,
        };
        gs.messages.push("Witaj w świecie Wiedźmina!".into());
        gs.messages.push("Jesteś Gerard z Rumii ze Szkoły Dzika.".into());
        gs.messages.push("WASD - ruch, E - interakcja, I - ekwipunek, J - questy".into());
        gs
    }

    fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
        if self.messages.len() > 50 { self.messages.remove(0); }
    }

    fn try_move(&mut self, dx: f32, dy: f32) {
        let new_x = self.player.x + dx;
        let new_y = self.player.y + dy;
        if dx < 0.0 { self.player.direction = Direction::Left; }
        if dx > 0.0 { self.player.direction = Direction::Right; }
        if dy < 0.0 { self.player.direction = Direction::Up; }
        if dy > 0.0 { self.player.direction = Direction::Down; }
        if !self.world.is_walkable(new_x, new_y) { return; }
        self.player.x = new_x;
        self.player.y = new_y;
        if let Some(mi) = self.world.find_monster_at(new_x, new_y, 1.2) {
            let mname = self.world.monsters[mi].name.clone();
            self.add_message(format!("Spotkanie z: {}!", mname));
            self.combat = Some(Combat::new());
            self.screen = GameScreen::Combat(mi);
        }
        if let Some(li) = self.world.find_loot_at(new_x, new_y, 1.0) {
            let lname = self.world.loot[li].name.clone();
            let iname = self.world.loot[li].item_name.clone();
            self.world.loot[li].collected = true;
            self.player.inventory.add_item(Item::ingredient(&iname, 5));
            self.add_message(format!("Znaleziono: {}!", lname));
        }
    }

    fn interact(&mut self) {
        let px = self.player.x;
        let py = self.player.y;
        let (dx, dy) = match self.player.direction {
            Direction::Up => (0.0, -1.0),
            Direction::Down => (0.0, 1.0),
            Direction::Left => (-1.0, 0.0),
            Direction::Right => (1.0, 0.0),
        };
        let tx = px + dx;
        let ty = py + dy;
        // NPC in facing direction or nearby
        let ni = self.world.find_npc_at(tx, ty, 1.5)
            .or_else(|| self.world.find_npc_at(px, py, 2.0));
        if let Some(ni) = ni {
            let npc_name = self.world.npcs[ni].name.clone();
            let did = self.world.npcs[ni].dialogue_id.clone();
            self.add_message(format!("Rozmowa z: {}", npc_name));
            let mut dlg = dialogue::get_dialogue(&did);
            dlg.start();
            self.dialogue = Some(dlg);
            self.screen = GameScreen::Dialogue;
            return;
        }
        if let Some(mi) = self.world.find_monster_at(tx, ty, 1.5) {
            let mname = self.world.monsters[mi].name.clone();
            self.add_message(format!("Atakujesz: {}!", mname));
            self.combat = Some(Combat::new());
            self.screen = GameScreen::Combat(mi);
            return;
        }
        self.add_message("Nie ma tu nic do interakcji.".into());
    }

    fn handle_dialogue_choice(&mut self, choice_idx: usize) {
        // Take dialogue out to avoid borrow issues
        let mut dlg = match self.dialogue.take() {
            Some(d) => d,
            None => return,
        };

        let mut pending_msgs: Vec<String> = Vec::new();

        if let Some(action) = dlg.select_choice(choice_idx) {
            match action {
                DialogueAction::StartQuest(quest_id) => {
                    self.quest_log.start_quest(&quest_id);
                    pending_msgs.push("Nowy quest przyjęty!".into());
                }
                DialogueAction::GiveItem(item_name) => {
                    match item_name.as_str() {
                        "Jaskółka" => self.player.inventory.add_item(Item::potion("Jaskółka", 50, 0, 30)),
                        "Kot" => self.player.inventory.add_item(Item::potion("Kot", 20, 20, 35)),
                        _ => self.player.inventory.add_item(Item::quest_item(&item_name, "Przedmiot z questu")),
                    }
                    pending_msgs.push(format!("Otrzymano: {}!", item_name));
                }
                DialogueAction::GiveGold(amount) => {
                    self.player.inventory.gold += amount;
                    pending_msgs.push(format!("Otrzymano: {} złota!", amount));
                }
                DialogueAction::GiveExp(amount) => {
                    let leveled = self.player.gain_experience(amount);
                    pending_msgs.push(format!("Otrzymano: {} doświadczenia!", amount));
                    if leveled { pending_msgs.push(format!("Awans na poziom {}!", self.player.level)); }
                }
                DialogueAction::Heal => {
                    self.player.rest();
                    pending_msgs.push("Pełne leczenie!".into());
                }
                DialogueAction::Trade => {
                    pending_msgs.push("Handel zakończony.".into());
                }
                DialogueAction::Axii(_) => {
                    if self.player.stamina >= 20 {
                        self.player.stamina -= 20;
                        pending_msgs.push("Axii zadziałało!".into());
                    } else {
                        pending_msgs.push("Za mało wytrzymałości na Axii!".into());
                    }
                }
                DialogueAction::EndDialogue => {}
                DialogueAction::None => {}
            }
        }

        let dialogue_ended = dlg.current_node.is_none();

        if dialogue_ended {
            let px = self.player.x;
            let py = self.player.y;
            if let Some(ni) = self.world.find_npc_at(px, py, 3.0) {
                let npc_id = self.world.npcs[ni].dialogue_id.clone();
                self.quest_log.check_talk_objective(&npc_id);
            }
            // Check completed quests
            for quest in &mut self.quest_log.quests {
                if quest.status == QuestStatus::Completed && quest.experience_reward > 0 {
                    let exp = quest.experience_reward;
                    let gold = quest.gold_reward;
                    quest.experience_reward = 0;
                    quest.gold_reward = 0;
                    let leveled = self.player.gain_experience(exp);
                    self.player.inventory.gold += gold;
                    pending_msgs.push(format!("Quest ukończony! +{} EXP, +{} złota", exp, gold));
                    if leveled { pending_msgs.push(format!("Awans na poziom {}!", self.player.level)); }
                }
            }
            self.dialogue = None;
            self.screen = GameScreen::Exploration;
        } else {
            self.dialogue = Some(dlg);
        }

        for msg in pending_msgs { self.add_message(msg); }
    }

    fn handle_combat_key(&mut self, keycode: KeyCode) {
        let mi = if let GameScreen::Combat(mi) = self.screen { mi } else { return; };
        let combat = match self.combat.as_mut() {
            Some(c) => c,
            None => return,
        };

        match combat.state {
            CombatState::PlayerTurn => {
                let action = match keycode {
                    KeyCode::Q => Some(CombatAction::AttackSteel),
                    KeyCode::W => Some(CombatAction::AttackSilver),
                    KeyCode::E => Some(CombatAction::UseSign(self.player.active_sign)),
                    KeyCode::A => Some(CombatAction::Dodge),
                    KeyCode::S => Some(CombatAction::Parry),
                    KeyCode::D => {
                        self.player.inventory.items.iter().position(|i| i.item_type == ItemType::Potion)
                            .map(CombatAction::UsePotion)
                    }
                    KeyCode::F => {
                        self.player.inventory.items.iter().position(|i| i.item_type == ItemType::Bomb)
                            .map(CombatAction::UseBomb)
                    }
                    KeyCode::Key1 => { self.player.active_sign = 0; None }
                    KeyCode::Key2 => { self.player.active_sign = 1; None }
                    KeyCode::Key3 => { self.player.active_sign = 2; None }
                    KeyCode::Key4 => { self.player.active_sign = 3; None }
                    KeyCode::Key5 => { self.player.active_sign = 4; None }
                    _ => None,
                };
                if let Some(act) = action {
                    let monster = &mut self.world.monsters[mi];
                    combat.execute_player_action(act, &mut self.player, monster);
                    if combat.state == CombatState::EnemyTurn {
                        let monster = &mut self.world.monsters[mi];
                        combat.execute_enemy_turn(&mut self.player, monster);
                    }
                }
            }
            CombatState::Victory => {
                if keycode == KeyCode::Return {
                    let m = &self.world.monsters[mi];
                    let exp = m.experience_reward;
                    let gold = m.gold_reward;
                    let name = m.name.clone();
                    let leveled = self.player.gain_experience(exp);
                    self.player.inventory.gold += gold;
                    self.player.kills += 1;
                    self.add_message(format!("{} pokonany! +{} EXP, +{} złota", name, exp, gold));
                    if leveled { self.add_message(format!("Awans na poziom {}!", self.player.level)); }
                    self.quest_log.check_kill_objective(&name);
                    self.combat = None;
                    self.screen = GameScreen::Exploration;
                }
            }
            CombatState::Defeat => {
                if keycode == KeyCode::Return {
                    self.player.rest();
                    self.player.x = 25.0;
                    self.player.y = 18.0;
                    self.add_message("Gerard odzyskuje przytomność...".into());
                    self.combat = None;
                    self.screen = GameScreen::Exploration;
                }
            }
            _ => {}
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.screen == GameScreen::Exploration {
            self.camera.follow(self.player.x, self.player.y);
            if self.move_cooldown > 0.0 {
                self.move_cooldown -= ctx.time.delta().as_secs_f32();
            }
            if self.move_cooldown <= 0.0 {
                let mut moved = false;
                if ctx.keyboard.is_key_pressed(KeyCode::W) || ctx.keyboard.is_key_pressed(KeyCode::Up) {
                    self.try_move(0.0, -1.0); moved = true;
                } else if ctx.keyboard.is_key_pressed(KeyCode::S) || ctx.keyboard.is_key_pressed(KeyCode::Down) {
                    self.try_move(0.0, 1.0); moved = true;
                } else if ctx.keyboard.is_key_pressed(KeyCode::A) || ctx.keyboard.is_key_pressed(KeyCode::Left) {
                    self.try_move(-1.0, 0.0); moved = true;
                } else if ctx.keyboard.is_key_pressed(KeyCode::D) || ctx.keyboard.is_key_pressed(KeyCode::Right) {
                    self.try_move(1.0, 0.0); moved = true;
                }
                if moved { self.move_cooldown = 0.12; }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::new(0.05, 0.03, 0.08, 1.0));

        match &self.screen {
            GameScreen::MainMenu => {
                let title = graphics::Text::new(
                    graphics::TextFragment::new("WIEDŹMIN: SZKOŁA DZIKA")
                        .color(Color::new(1.0, 0.85, 0.0, 1.0))
                        .scale(PxScale { x: 36.0, y: 36.0 })
                );
                let tw = title.measure(ctx)?.x;
                canvas.draw(&title, graphics::DrawParam::default()
                    .dest(ggez::glam::Vec2::new(SCREEN_WIDTH / 2.0 - tw / 2.0, 150.0)));

                let subtitle = graphics::Text::new(
                    graphics::TextFragment::new("Gerard z Rumii")
                        .color(Color::new(0.8, 0.8, 0.8, 1.0))
                        .scale(PxScale { x: 24.0, y: 24.0 })
                );
                let sw = subtitle.measure(ctx)?.x;
                canvas.draw(&subtitle, graphics::DrawParam::default()
                    .dest(ggez::glam::Vec2::new(SCREEN_WIDTH / 2.0 - sw / 2.0, 210.0)));

                let start = graphics::Text::new(
                    graphics::TextFragment::new("Naciśnij ENTER aby rozpocząć")
                        .color(Color::new(0.6, 0.9, 0.6, 1.0))
                        .scale(PxScale { x: 20.0, y: 20.0 })
                );
                let stw = start.measure(ctx)?.x;
                canvas.draw(&start, graphics::DrawParam::default()
                    .dest(ggez::glam::Vec2::new(SCREEN_WIDTH / 2.0 - stw / 2.0, 350.0)));

                let info_lines = [
                    "Pełne RPG w świecie mrocznej fantasy",
                    "Walka turowa, questy, alchemia, znaki wiedźmińskie",
                    "WASD - ruch | E - interakcja | I - ekwipunek | J - questy",
                    "1-5 - znaki | R - odpoczynek | ESC - wyjście",
                ];
                for (i, line) in info_lines.iter().enumerate() {
                    let t = graphics::Text::new(
                        graphics::TextFragment::new(*line)
                            .color(Color::new(0.5, 0.5, 0.5, 1.0))
                            .scale(PxScale { x: 14.0, y: 14.0 })
                    );
                    canvas.draw(&t, graphics::DrawParam::default()
                        .dest(ggez::glam::Vec2::new(SCREEN_WIDTH / 2.0 - 200.0, 430.0 + i as f32 * 18.0)));
                }
            }
            GameScreen::Exploration => {
                rendering::draw_map(ctx, &mut canvas, &self.world, &self.camera)?;
                rendering::draw_loot(ctx, &mut canvas, &self.world, &self.camera)?;
                rendering::draw_npcs(ctx, &mut canvas, &self.world, &self.camera)?;
                rendering::draw_monsters(ctx, &mut canvas, &self.world, &self.camera)?;
                rendering::draw_player(ctx, &mut canvas, &self.player, &self.camera)?;
                let location = self.world.location_name(self.player.x, self.player.y);
                ui::draw_hud(ctx, &mut canvas, &self.player, location)?;
                ui::draw_message_log(ctx, &mut canvas, &self.messages)?;
            }
            GameScreen::Combat(mi) => {
                let mi = *mi;
                if let Some(ref combat) = self.combat {
                    let m = &self.world.monsters[mi];
                    ui::draw_combat_ui(ctx, &mut canvas, combat, &self.player, &m.name, m.health, m.max_health)?;
                }
                let location = self.world.location_name(self.player.x, self.player.y);
                ui::draw_hud(ctx, &mut canvas, &self.player, location)?;
            }
            GameScreen::Dialogue => {
                rendering::draw_map(ctx, &mut canvas, &self.world, &self.camera)?;
                rendering::draw_npcs(ctx, &mut canvas, &self.world, &self.camera)?;
                rendering::draw_player(ctx, &mut canvas, &self.player, &self.camera)?;
                if let Some(ref dlg) = self.dialogue {
                    if let Some(node) = dlg.get_current_node() {
                        let choices: Vec<(usize, String)> = node.choices.iter().enumerate()
                            .map(|(i, c)| (i, c.text.clone())).collect();
                        ui::draw_dialogue_ui(ctx, &mut canvas, &node.speaker, &node.text, &choices)?;
                    }
                }
                let location = self.world.location_name(self.player.x, self.player.y);
                ui::draw_hud(ctx, &mut canvas, &self.player, location)?;
            }
            GameScreen::Inventory => {
                ui::draw_inventory_ui(ctx, &mut canvas, &self.player, self.inventory_selected)?;
                let location = self.world.location_name(self.player.x, self.player.y);
                ui::draw_hud(ctx, &mut canvas, &self.player, location)?;
            }
            GameScreen::QuestLog => {
                ui::draw_quest_log_ui(ctx, &mut canvas, &self.quest_log)?;
                let location = self.world.location_name(self.player.x, self.player.y);
                ui::draw_hud(ctx, &mut canvas, &self.player, location)?;
            }
            GameScreen::GameOver => {
                let go = graphics::Text::new(
                    graphics::TextFragment::new("KONIEC GRY\nNaciśnij ENTER aby zacząć od nowa")
                        .color(Color::new(1.0, 0.2, 0.2, 1.0))
                        .scale(PxScale { x: 28.0, y: 28.0 })
                );
                canvas.draw(&go, graphics::DrawParam::default()
                    .dest(ggez::glam::Vec2::new(300.0, 300.0)));
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        let keycode = match input.keycode {
            Some(k) => k,
            None => return Ok(()),
        };

        match &self.screen {
            GameScreen::MainMenu => {
                if keycode == KeyCode::Return { self.screen = GameScreen::Exploration; self.started = true; }
                if keycode == KeyCode::Escape { _ctx.request_quit(); }
            }
            GameScreen::Exploration => {
                match keycode {
                    KeyCode::E => self.interact(),
                    KeyCode::I => { self.inventory_selected = 0; self.screen = GameScreen::Inventory; }
                    KeyCode::J => { self.screen = GameScreen::QuestLog; }
                    KeyCode::R => { self.player.rest(); self.add_message("Gerard odpoczywa... Zdrowie i wytrzymałość przywrócone.".into()); }
                    KeyCode::Key1 => { self.player.active_sign = 0; self.add_message("Aktywny znak: Aard".into()); }
                    KeyCode::Key2 => { self.player.active_sign = 1; self.add_message("Aktywny znak: Igni".into()); }
                    KeyCode::Key3 => { self.player.active_sign = 2; self.add_message("Aktywny znak: Quen".into()); }
                    KeyCode::Key4 => { self.player.active_sign = 3; self.add_message("Aktywny znak: Yrden".into()); }
                    KeyCode::Key5 => { self.player.active_sign = 4; self.add_message("Aktywny znak: Axii".into()); }
                    KeyCode::Escape => { _ctx.request_quit(); }
                    _ => {}
                }
            }
            GameScreen::Combat(_) => { self.handle_combat_key(keycode); }
            GameScreen::Dialogue => {
                match keycode {
                    KeyCode::Key1 => self.handle_dialogue_choice(0),
                    KeyCode::Key2 => self.handle_dialogue_choice(1),
                    KeyCode::Key3 => self.handle_dialogue_choice(2),
                    KeyCode::Key4 => self.handle_dialogue_choice(3),
                    KeyCode::Escape => { self.dialogue = None; self.screen = GameScreen::Exploration; }
                    _ => {}
                }
            }
            GameScreen::Inventory => {
                match keycode {
                    KeyCode::Up => { if self.inventory_selected > 0 { self.inventory_selected -= 1; } }
                    KeyCode::Down => { if self.inventory_selected + 1 < self.player.inventory.items.len() { self.inventory_selected += 1; } }
                    KeyCode::Return => {
                        let idx = self.inventory_selected;
                        if idx < self.player.inventory.items.len() {
                            let itype = self.player.inventory.items[idx].item_type.clone();
                            let iname = self.player.inventory.items[idx].name.clone();
                            match itype {
                                ItemType::SteelSword | ItemType::SilverSword | ItemType::Armor | ItemType::Oil => {
                                    self.player.inventory.equip_item(idx);
                                    self.add_message(format!("Założono: {}", iname));
                                }
                                ItemType::Potion => {
                                    let item = self.player.inventory.items[idx].clone();
                                    self.player.heal(item.health_restore);
                                    self.player.restore_stamina(item.stamina_restore);
                                    self.add_message(format!("Użyto: {}", iname));
                                    self.player.inventory.remove_item(idx);
                                    if self.inventory_selected >= self.player.inventory.items.len() && self.inventory_selected > 0 {
                                        self.inventory_selected -= 1;
                                    }
                                }
                                _ => { self.add_message(format!("Nie można użyć: {}", iname)); }
                            }
                        }
                    }
                    KeyCode::Escape | KeyCode::I => { self.screen = GameScreen::Exploration; }
                    _ => {}
                }
            }
            GameScreen::QuestLog => {
                if keycode == KeyCode::Escape || keycode == KeyCode::J { self.screen = GameScreen::Exploration; }
            }
            GameScreen::GameOver => {
                if keycode == KeyCode::Return {
                    *self = GameState::new();
                    self.screen = GameScreen::Exploration;
                    self.started = true;
                }
            }
        }
        Ok(())
    }
}
