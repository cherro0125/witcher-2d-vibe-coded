/// UI - interfejs użytkownika (HUD, panele, menu)
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment, PxScale};
use ggez::Context;
use ggez::GameResult;
use ggez::glam::Vec2;

use crate::rendering::{MAP_VIEW_WIDTH, MAP_VIEW_HEIGHT, SCREEN_HEIGHT, UI_PANEL_WIDTH};
use crate::player::Player;
use crate::combat::{Combat, CombatState};
use crate::quests::QuestLog;

pub fn draw_hud(ctx: &mut Context, canvas: &mut Canvas, player: &Player, location: &str) -> GameResult {
    let panel_x = MAP_VIEW_WIDTH;

    // Tło panelu
    let bg = Mesh::new_rectangle(ctx, DrawMode::fill(),
        Rect::new(0.0, 0.0, UI_PANEL_WIDTH, SCREEN_HEIGHT),
        Color::new(0.12, 0.1, 0.15, 1.0))?;
    canvas.draw(&bg, DrawParam::default().dest(Vec2::new(panel_x, 0.0)));

    // Separator
    let sep = Mesh::new_rectangle(ctx, DrawMode::fill(),
        Rect::new(0.0, 0.0, 2.0, SCREEN_HEIGHT),
        Color::new(0.5, 0.4, 0.2, 1.0))?;
    canvas.draw(&sep, DrawParam::default().dest(Vec2::new(panel_x, 0.0)));

    let x = panel_x + 10.0;
    let mut y = 10.0;

    // Nazwa postaci
    let title = Text::new(TextFragment::new(&player.name)
        .color(Color::new(1.0, 0.85, 0.0, 1.0)).scale(PxScale { x: 20.0, y: 20.0 }));
    canvas.draw(&title, DrawParam::default().dest(Vec2::new(x, y)));
    y += 22.0;

    let school = Text::new(TextFragment::new(&player.school)
        .color(Color::new(0.7, 0.7, 0.7, 1.0)).scale(PxScale { x: 14.0, y: 14.0 }));
    canvas.draw(&school, DrawParam::default().dest(Vec2::new(x, y)));
    y += 20.0;

    let lvl = Text::new(TextFragment::new(format!("Poziom: {}  EXP: {}/{}", player.level, player.experience, player.exp_to_next_level))
        .color(Color::new(0.8, 0.8, 0.3, 1.0)).scale(PxScale { x: 14.0, y: 14.0 }));
    canvas.draw(&lvl, DrawParam::default().dest(Vec2::new(x, y)));
    y += 22.0;

    // Pasek zdrowia
    draw_bar(ctx, canvas, x, y, UI_PANEL_WIDTH - 20.0, 18.0,
        player.health as f32 / player.max_health as f32,
        Color::new(0.8, 0.1, 0.1, 1.0), Color::new(0.3, 0.05, 0.05, 1.0),
        &format!("HP: {}/{}", player.health, player.max_health))?;
    y += 22.0;

    // Pasek wytrzymałości
    draw_bar(ctx, canvas, x, y, UI_PANEL_WIDTH - 20.0, 18.0,
        player.stamina as f32 / player.max_stamina as f32,
        Color::new(0.2, 0.6, 0.8, 1.0), Color::new(0.05, 0.15, 0.25, 1.0),
        &format!("Wytrzymałość: {}/{}", player.stamina, player.max_stamina))?;
    y += 22.0;

    // Quen
    if player.quen_shield > 0 {
        draw_bar(ctx, canvas, x, y, UI_PANEL_WIDTH - 20.0, 14.0,
            1.0, Color::new(0.3, 0.8, 1.0, 0.8), Color::new(0.1, 0.2, 0.3, 1.0),
            &format!("Quen: {}", player.quen_shield))?;
        y += 18.0;
    }

    y += 5.0;
    let sep2 = Mesh::new_rectangle(ctx, DrawMode::fill(),
        Rect::new(0.0, 0.0, UI_PANEL_WIDTH - 20.0, 1.0),
        Color::new(0.4, 0.3, 0.2, 0.8))?;
    canvas.draw(&sep2, DrawParam::default().dest(Vec2::new(x, y)));
    y += 8.0;

    // Statystyki
    let stats = format!("SIŁ: {}  ZRC: {}  INT: {}", player.strength, player.dexterity, player.intelligence);
    let stat_text = Text::new(TextFragment::new(stats)
        .color(Color::new(0.7, 0.7, 0.7, 1.0)).scale(PxScale { x: 13.0, y: 13.0 }));
    canvas.draw(&stat_text, DrawParam::default().dest(Vec2::new(x, y)));
    y += 18.0;

    let atk = format!("Atak(S/Sr): {}/{}  Obr: {}", player.attack_power(false), player.attack_power(true), player.defense_power());
    let atk_text = Text::new(TextFragment::new(atk)
        .color(Color::new(0.7, 0.7, 0.7, 1.0)).scale(PxScale { x: 13.0, y: 13.0 }));
    canvas.draw(&atk_text, DrawParam::default().dest(Vec2::new(x, y)));
    y += 18.0;

    let gold = format!("Złoto: {} koron", player.inventory.gold);
    let gold_text = Text::new(TextFragment::new(gold)
        .color(Color::new(1.0, 0.85, 0.0, 1.0)).scale(PxScale { x: 14.0, y: 14.0 }));
    canvas.draw(&gold_text, DrawParam::default().dest(Vec2::new(x, y)));
    y += 22.0;

    // Aktywny znak
    let sign = &player.signs[player.active_sign];
    let sign_text = Text::new(TextFragment::new(format!("Znak: {} (koszt: {})", sign.name, sign.stamina_cost))
        .color(Color::new(0.5, 0.8, 1.0, 1.0)).scale(PxScale { x: 14.0, y: 14.0 }));
    canvas.draw(&sign_text, DrawParam::default().dest(Vec2::new(x, y)));
    y += 20.0;

    y += 5.0;
    canvas.draw(&sep2, DrawParam::default().dest(Vec2::new(x, y)));
    y += 8.0;

    // Lokalizacja
    let loc = Text::new(TextFragment::new(format!("Lokacja: {}", location))
        .color(Color::new(0.6, 0.9, 0.6, 1.0)).scale(PxScale { x: 14.0, y: 14.0 }));
    canvas.draw(&loc, DrawParam::default().dest(Vec2::new(x, y)));
    y += 22.0;

    // Sterowanie
    y += 5.0;
    canvas.draw(&sep2, DrawParam::default().dest(Vec2::new(x, y)));
    y += 8.0;

    let controls = vec![
        "WASD - Ruch",
        "E - Interakcja",
        "I - Ekwipunek",
        "J - Dziennik",
        "1-5 - Zmień znak",
        "R - Odpoczynek",
        "ESC - Menu",
    ];
    let ctrl_title = Text::new(TextFragment::new("Sterowanie:")
        .color(Color::new(0.9, 0.8, 0.4, 1.0)).scale(PxScale { x: 14.0, y: 14.0 }));
    canvas.draw(&ctrl_title, DrawParam::default().dest(Vec2::new(x, y)));
    y += 16.0;

    for c in &controls {
        let ct = Text::new(TextFragment::new(*c)
            .color(Color::new(0.6, 0.6, 0.6, 1.0)).scale(PxScale { x: 12.0, y: 12.0 }));
        canvas.draw(&ct, DrawParam::default().dest(Vec2::new(x + 5.0, y)));
        y += 14.0;
    }

    Ok(())
}

pub fn draw_bar(ctx: &mut Context, canvas: &mut Canvas, x: f32, y: f32, w: f32, h: f32,
    ratio: f32, fg: Color, bg: Color, label: &str) -> GameResult {
    let bg_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0.0, 0.0, w, h), bg)?;
    canvas.draw(&bg_mesh, DrawParam::default().dest(Vec2::new(x, y)));
    if ratio > 0.0 {
        let fg_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), Rect::new(0.0, 0.0, w * ratio.min(1.0), h), fg)?;
        canvas.draw(&fg_mesh, DrawParam::default().dest(Vec2::new(x, y)));
    }
    let text = Text::new(TextFragment::new(label).color(Color::WHITE).scale(PxScale { x: 12.0, y: 12.0 }));
    canvas.draw(&text, DrawParam::default().dest(Vec2::new(x + 4.0, y + 2.0)));
    Ok(())
}

pub fn draw_combat_ui(ctx: &mut Context, canvas: &mut Canvas, combat: &Combat, player: &Player, monster_name: &str, monster_hp: i32, monster_max_hp: i32) -> GameResult {
    // Tło walki
    let bg = Mesh::new_rectangle(ctx, DrawMode::fill(),
        Rect::new(0.0, 0.0, MAP_VIEW_WIDTH, MAP_VIEW_HEIGHT),
        Color::new(0.05, 0.02, 0.08, 0.95))?;
    canvas.draw(&bg, DrawParam::default().dest(Vec2::new(0.0, 0.0)));

    let cx = MAP_VIEW_WIDTH / 2.0;

    // Tytuł
    let title = Text::new(TextFragment::new(format!("⚔ WALKA: {} vs {} ⚔", player.name, monster_name))
        .color(Color::new(1.0, 0.3, 0.2, 1.0)).scale(PxScale { x: 24.0, y: 24.0 }));
    let tw = title.measure(ctx).unwrap().x;
    canvas.draw(&title, DrawParam::default().dest(Vec2::new(cx - tw/2.0, 15.0)));

    // Pasek HP gracza
    draw_bar(ctx, canvas, 30.0, 50.0, 400.0, 22.0,
        player.health as f32 / player.max_health as f32,
        Color::new(0.1, 0.8, 0.1, 1.0), Color::new(0.2, 0.1, 0.1, 1.0),
        &format!("Gerard HP: {}/{}", player.health, player.max_health))?;

    // Pasek wytrzymałości
    draw_bar(ctx, canvas, 30.0, 75.0, 400.0, 16.0,
        player.stamina as f32 / player.max_stamina as f32,
        Color::new(0.2, 0.5, 0.8, 1.0), Color::new(0.1, 0.1, 0.2, 1.0),
        &format!("Wytrzymałość: {}/{}", player.stamina, player.max_stamina))?;

    // Pasek HP potwora
    draw_bar(ctx, canvas, MAP_VIEW_WIDTH - 430.0, 50.0, 400.0, 22.0,
        monster_hp as f32 / monster_max_hp as f32,
        Color::new(0.8, 0.1, 0.1, 1.0), Color::new(0.2, 0.1, 0.1, 1.0),
        &format!("{} HP: {}/{}", monster_name, monster_hp, monster_max_hp))?;

    // Log walki
    let log_y = 110.0;
    let msgs = combat.log.last_messages(8);
    for (i, msg) in msgs.iter().enumerate() {
        let color = if msg.contains("POKONANY") || msg.contains("ZWYCIĘSTWO") {
            Color::new(0.2, 1.0, 0.2, 1.0)
        } else if msg.contains("POLEGŁ") {
            Color::new(1.0, 0.2, 0.2, 1.0)
        } else if msg.contains("Gerard") {
            Color::new(0.9, 0.9, 0.5, 1.0)
        } else {
            Color::new(0.7, 0.7, 0.7, 1.0)
        };
        let t = Text::new(TextFragment::new(msg.as_str()).color(color).scale(PxScale { x: 14.0, y: 14.0 }));
        canvas.draw(&t, DrawParam::default().dest(Vec2::new(30.0, log_y + i as f32 * 18.0)));
    }

    // Akcje (tylko w turze gracza)
    if combat.state == CombatState::PlayerTurn {
        let actions_y = 360.0;
        let act_title = Text::new(TextFragment::new("Wybierz akcję:")
            .color(Color::new(1.0, 0.85, 0.0, 1.0)).scale(PxScale { x: 18.0, y: 18.0 }));
        canvas.draw(&act_title, DrawParam::default().dest(Vec2::new(30.0, actions_y)));

        let sign = &player.signs[player.active_sign];
        let actions = vec![
            format!("[Q] Atak stalowy ({})", player.attack_power(false)),
            format!("[W] Atak srebrny ({})", player.attack_power(true)),
            format!("[E] Znak: {} (koszt: {})", sign.name, sign.stamina_cost),
            "[A] Unik".to_string(),
            "[S] Parowanie".to_string(),
            "[D] Użyj eliksiru".to_string(),
            "[F] Rzuć bombę".to_string(),
            "[1-5] Zmień aktywny znak".to_string(),
        ];
        for (i, a) in actions.iter().enumerate() {
            let t = Text::new(TextFragment::new(a.as_str())
                .color(Color::new(0.8, 0.8, 0.8, 1.0)).scale(PxScale { x: 15.0, y: 15.0 }));
            canvas.draw(&t, DrawParam::default().dest(Vec2::new(40.0, actions_y + 25.0 + i as f32 * 20.0)));
        }
    } else if combat.state == CombatState::Victory {
        let v = Text::new(TextFragment::new("ZWYCIĘSTWO! Naciśnij ENTER aby kontynuować.")
            .color(Color::new(0.2, 1.0, 0.2, 1.0)).scale(PxScale { x: 22.0, y: 22.0 }));
        let vw = v.measure(ctx).unwrap().x;
        canvas.draw(&v, DrawParam::default().dest(Vec2::new(cx - vw/2.0, 400.0)));
    } else if combat.state == CombatState::Defeat {
        let d = Text::new(TextFragment::new("PORAŻKA! Naciśnij ENTER aby wczytać ostatni zapis.")
            .color(Color::new(1.0, 0.2, 0.2, 1.0)).scale(PxScale { x: 22.0, y: 22.0 }));
        let dw = d.measure(ctx).unwrap().x;
        canvas.draw(&d, DrawParam::default().dest(Vec2::new(cx - dw/2.0, 400.0)));
    }

    Ok(())
}

pub fn draw_dialogue_ui(ctx: &mut Context, canvas: &mut Canvas, speaker: &str, text: &str, choices: &[(usize, String)]) -> GameResult {
    let bg = Mesh::new_rectangle(ctx, DrawMode::fill(),
        Rect::new(0.0, 0.0, MAP_VIEW_WIDTH, MAP_VIEW_HEIGHT),
        Color::new(0.05, 0.05, 0.1, 0.92))?;
    canvas.draw(&bg, DrawParam::default().dest(Vec2::new(0.0, 0.0)));

    // Speaker name
    let name = Text::new(TextFragment::new(speaker)
        .color(Color::new(1.0, 0.85, 0.0, 1.0)).scale(PxScale { x: 22.0, y: 22.0 }));
    canvas.draw(&name, DrawParam::default().dest(Vec2::new(40.0, 30.0)));

    // Dialogue text - split into lines
    let max_chars = 80;
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in words {
        if current_line.len() + word.len() + 1 > max_chars {
            lines.push(current_line.clone());
            current_line = word.to_string();
        } else {
            if !current_line.is_empty() { current_line.push(' '); }
            current_line.push_str(word);
        }
    }
    if !current_line.is_empty() { lines.push(current_line); }

    for (i, line) in lines.iter().enumerate() {
        let t = Text::new(TextFragment::new(line.as_str())
            .color(Color::new(0.9, 0.9, 0.9, 1.0)).scale(PxScale { x: 16.0, y: 16.0 }));
        canvas.draw(&t, DrawParam::default().dest(Vec2::new(50.0, 70.0 + i as f32 * 20.0)));
    }

    // Choices
    let choices_y = 250.0;
    let ch_title = Text::new(TextFragment::new("Odpowiedzi:")
        .color(Color::new(0.8, 0.8, 0.3, 1.0)).scale(PxScale { x: 16.0, y: 16.0 }));
    canvas.draw(&ch_title, DrawParam::default().dest(Vec2::new(40.0, choices_y)));

    for (i, (_, choice_text)) in choices.iter().enumerate() {
        let label = format!("[{}] {}", i + 1, choice_text);
        let t = Text::new(TextFragment::new(label)
            .color(Color::new(0.7, 0.9, 0.7, 1.0)).scale(PxScale { x: 15.0, y: 15.0 }));
        canvas.draw(&t, DrawParam::default().dest(Vec2::new(50.0, choices_y + 25.0 + i as f32 * 22.0)));
    }

    Ok(())
}

pub fn draw_inventory_ui(ctx: &mut Context, canvas: &mut Canvas, player: &Player, selected: usize) -> GameResult {
    let bg = Mesh::new_rectangle(ctx, DrawMode::fill(),
        Rect::new(0.0, 0.0, MAP_VIEW_WIDTH, MAP_VIEW_HEIGHT),
        Color::new(0.08, 0.06, 0.12, 0.95))?;
    canvas.draw(&bg, DrawParam::default().dest(Vec2::new(0.0, 0.0)));

    let title = Text::new(TextFragment::new("⚔ EKWIPUNEK ⚔")
        .color(Color::new(1.0, 0.85, 0.0, 1.0)).scale(PxScale { x: 22.0, y: 22.0 }));
    canvas.draw(&title, DrawParam::default().dest(Vec2::new(30.0, 15.0)));

    let gold = Text::new(TextFragment::new(format!("Złoto: {} koron", player.inventory.gold))
        .color(Color::new(1.0, 0.85, 0.0, 1.0)).scale(PxScale { x: 14.0, y: 14.0 }));
    canvas.draw(&gold, DrawParam::default().dest(Vec2::new(30.0, 42.0)));

    let mut y = 65.0;
    for (i, item) in player.inventory.items.iter().enumerate() {
        let is_equipped = player.inventory.equipped_steel_sword == Some(i)
            || player.inventory.equipped_silver_sword == Some(i)
            || player.inventory.equipped_armor == Some(i)
            || player.inventory.active_oil == Some(i);

        let prefix = if is_equipped { "[E] " } else { "    " };
        let sel = if i == selected { "► " } else { "  " };
        let qty = if item.quantity > 1 { format!(" x{}", item.quantity) } else { String::new() };
        let label = format!("{}{}{}{}", sel, prefix, item.name, qty);

        let color = if i == selected {
            Color::new(1.0, 1.0, 0.5, 1.0)
        } else if is_equipped {
            Color::new(0.5, 1.0, 0.5, 1.0)
        } else {
            Color::new(0.7, 0.7, 0.7, 1.0)
        };

        let t = Text::new(TextFragment::new(label).color(color).scale(PxScale { x: 14.0, y: 14.0 }));
        canvas.draw(&t, DrawParam::default().dest(Vec2::new(30.0, y)));
        y += 18.0;
    }

    if player.inventory.items.is_empty() {
        let t = Text::new(TextFragment::new("Ekwipunek pusty")
            .color(Color::new(0.5, 0.5, 0.5, 1.0)).scale(PxScale { x: 14.0, y: 14.0 }));
        canvas.draw(&t, DrawParam::default().dest(Vec2::new(30.0, y)));
    }

    // Opis wybranego przedmiotu
    if selected < player.inventory.items.len() {
        let item = &player.inventory.items[selected];
        let desc_y = MAP_VIEW_HEIGHT - 100.0;
        let sep = Mesh::new_rectangle(ctx, DrawMode::fill(),
            Rect::new(0.0, 0.0, MAP_VIEW_WIDTH - 60.0, 1.0),
            Color::new(0.4, 0.3, 0.2, 0.8))?;
        canvas.draw(&sep, DrawParam::default().dest(Vec2::new(30.0, desc_y)));

        let desc = Text::new(TextFragment::new(&item.description)
            .color(Color::new(0.8, 0.8, 0.6, 1.0)).scale(PxScale { x: 14.0, y: 14.0 }));
        canvas.draw(&desc, DrawParam::default().dest(Vec2::new(30.0, desc_y + 10.0)));

        let controls = Text::new(TextFragment::new("[ENTER] Załóż/Użyj  [↑↓] Nawiguj  [ESC] Zamknij")
            .color(Color::new(0.6, 0.6, 0.6, 1.0)).scale(PxScale { x: 13.0, y: 13.0 }));
        canvas.draw(&controls, DrawParam::default().dest(Vec2::new(30.0, desc_y + 30.0)));
    }

    Ok(())
}

pub fn draw_quest_log_ui(ctx: &mut Context, canvas: &mut Canvas, quest_log: &QuestLog) -> GameResult {
    let bg = Mesh::new_rectangle(ctx, DrawMode::fill(),
        Rect::new(0.0, 0.0, MAP_VIEW_WIDTH, MAP_VIEW_HEIGHT),
        Color::new(0.06, 0.05, 0.1, 0.95))?;
    canvas.draw(&bg, DrawParam::default().dest(Vec2::new(0.0, 0.0)));

    let title = Text::new(TextFragment::new("📜 DZIENNIK QUESTÓW 📜")
        .color(Color::new(1.0, 0.85, 0.0, 1.0)).scale(PxScale { x: 22.0, y: 22.0 }));
    canvas.draw(&title, DrawParam::default().dest(Vec2::new(30.0, 15.0)));

    let mut y = 50.0;

    // Aktywne questy
    let active = quest_log.active_quests();
    if !active.is_empty() {
        let header = Text::new(TextFragment::new("--- Aktywne ---")
            .color(Color::new(0.3, 1.0, 0.3, 1.0)).scale(PxScale { x: 16.0, y: 16.0 }));
        canvas.draw(&header, DrawParam::default().dest(Vec2::new(30.0, y)));
        y += 22.0;

        for quest in active {
            let marker = if quest.is_main_quest { "★ " } else { "• " };
            let qt = Text::new(TextFragment::new(format!("{}{}", marker, quest.title))
                .color(Color::new(0.9, 0.9, 0.5, 1.0)).scale(PxScale { x: 15.0, y: 15.0 }));
            canvas.draw(&qt, DrawParam::default().dest(Vec2::new(35.0, y)));
            y += 18.0;

            if let Some(step) = quest.current_objective() {
                let st = Text::new(TextFragment::new(format!("  → {}", step.description))
                    .color(Color::new(0.7, 0.7, 0.7, 1.0)).scale(PxScale { x: 13.0, y: 13.0 }));
                canvas.draw(&st, DrawParam::default().dest(Vec2::new(40.0, y)));
                y += 18.0;
            }
        }
    } else {
        let no_q = Text::new(TextFragment::new("Brak aktywnych questów. Porozmawiaj z NPC!")
            .color(Color::new(0.6, 0.6, 0.6, 1.0)).scale(PxScale { x: 14.0, y: 14.0 }));
        canvas.draw(&no_q, DrawParam::default().dest(Vec2::new(30.0, y)));
        y += 22.0;
    }

    y += 10.0;

    // Ukończone questy
    let completed = quest_log.completed_quests();
    if !completed.is_empty() {
        let header = Text::new(TextFragment::new("--- Ukończone ---")
            .color(Color::new(0.5, 0.5, 0.5, 1.0)).scale(PxScale { x: 16.0, y: 16.0 }));
        canvas.draw(&header, DrawParam::default().dest(Vec2::new(30.0, y)));
        y += 22.0;

        for quest in completed {
            let qt = Text::new(TextFragment::new(format!("✓ {}", quest.title))
                .color(Color::new(0.4, 0.7, 0.4, 1.0)).scale(PxScale { x: 14.0, y: 14.0 }));
            canvas.draw(&qt, DrawParam::default().dest(Vec2::new(35.0, y)));
            y += 18.0;
        }
    }

    let esc = Text::new(TextFragment::new("[ESC] Zamknij dziennik")
        .color(Color::new(0.6, 0.6, 0.6, 1.0)).scale(PxScale { x: 13.0, y: 13.0 }));
    canvas.draw(&esc, DrawParam::default().dest(Vec2::new(30.0, MAP_VIEW_HEIGHT - 25.0)));

    Ok(())
}

pub fn draw_message_log(ctx: &mut Context, canvas: &mut Canvas, messages: &[String]) -> GameResult {
    let log_y = MAP_VIEW_HEIGHT;
    let bg = Mesh::new_rectangle(ctx, DrawMode::fill(),
        Rect::new(0.0, 0.0, MAP_VIEW_WIDTH, SCREEN_HEIGHT - MAP_VIEW_HEIGHT),
        Color::new(0.08, 0.06, 0.1, 1.0))?;
    canvas.draw(&bg, DrawParam::default().dest(Vec2::new(0.0, log_y)));

    let sep = Mesh::new_rectangle(ctx, DrawMode::fill(),
        Rect::new(0.0, 0.0, MAP_VIEW_WIDTH, 1.0),
        Color::new(0.5, 0.4, 0.2, 1.0))?;
    canvas.draw(&sep, DrawParam::default().dest(Vec2::new(0.0, log_y)));

    let max_msgs = 6;
    let start = if messages.len() > max_msgs { messages.len() - max_msgs } else { 0 };
    for (i, msg) in messages[start..].iter().enumerate() {
        let t = Text::new(TextFragment::new(msg.as_str())
            .color(Color::new(0.7, 0.7, 0.6, 1.0)).scale(PxScale { x: 13.0, y: 13.0 }));
        canvas.draw(&t, DrawParam::default().dest(Vec2::new(10.0, log_y + 5.0 + i as f32 * 16.0)));
    }
    Ok(())
}
