/// Renderowanie mapy i postaci
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect, Text, TextFragment, PxScale};
use ggez::Context;
use ggez::GameResult;
use ggez::glam::Vec2;

use crate::world::{WorldMap, TileType, TILE_SIZE};
use crate::player::Player;

pub const SCREEN_WIDTH: f32 = 1280.0;
pub const SCREEN_HEIGHT: f32 = 720.0;
pub const MAP_VIEW_WIDTH: f32 = 960.0;
pub const MAP_VIEW_HEIGHT: f32 = 600.0;
pub const UI_PANEL_WIDTH: f32 = SCREEN_WIDTH - MAP_VIEW_WIDTH;
pub const UI_PANEL_HEIGHT: f32 = SCREEN_HEIGHT;
pub const LOG_HEIGHT: f32 = SCREEN_HEIGHT - MAP_VIEW_HEIGHT;

pub struct Camera {
    pub x: f32,
    pub y: f32,
}

impl Camera {
    pub fn new() -> Self { Camera { x: 0.0, y: 0.0 } }

    pub fn follow(&mut self, player_x: f32, player_y: f32) {
        let target_x = player_x * TILE_SIZE - MAP_VIEW_WIDTH / 2.0;
        let target_y = player_y * TILE_SIZE - MAP_VIEW_HEIGHT / 2.0;
        self.x += (target_x - self.x) * 0.15;
        self.y += (target_y - self.y) * 0.15;
    }
}

pub fn draw_map(ctx: &mut Context, canvas: &mut Canvas, world: &WorldMap, camera: &Camera) -> GameResult {
    let start_tile_x = (camera.x / TILE_SIZE).floor().max(0.0) as usize;
    let start_tile_y = (camera.y / TILE_SIZE).floor().max(0.0) as usize;
    let end_tile_x = ((camera.x + MAP_VIEW_WIDTH) / TILE_SIZE).ceil().min(world.width as f32) as usize;
    let end_tile_y = ((camera.y + MAP_VIEW_HEIGHT) / TILE_SIZE).ceil().min(world.height as f32) as usize;

    for y in start_tile_y..end_tile_y {
        for x in start_tile_x..end_tile_x {
            let tile = world.get_tile(x, y);
            let c = tile.color();
            let screen_x = x as f32 * TILE_SIZE - camera.x;
            let screen_y = y as f32 * TILE_SIZE - camera.y;

            if screen_x + TILE_SIZE < 0.0 || screen_x > MAP_VIEW_WIDTH || 
               screen_y + TILE_SIZE < 0.0 || screen_y > MAP_VIEW_HEIGHT {
                continue;
            }

            let rect = Mesh::new_rectangle(
                ctx, DrawMode::fill(),
                Rect::new(0.0, 0.0, TILE_SIZE - 1.0, TILE_SIZE - 1.0),
                Color::new(c[0], c[1], c[2], c[3]),
            )?;
            canvas.draw(&rect, DrawParam::default().dest(Vec2::new(screen_x, screen_y)));

            // Dodaj oznaczenia specjalnych kafelków
            if matches!(tile, TileType::Door) {
                let door_text = Text::new(TextFragment::new("▢").color(Color::WHITE).scale(PxScale { x: 16.0, y: 16.0 }));
                canvas.draw(&door_text, DrawParam::default().dest(Vec2::new(screen_x + 4.0, screen_y + 4.0)));
            }
        }
    }
    Ok(())
}

pub fn draw_player(ctx: &mut Context, canvas: &mut Canvas, player: &Player, camera: &Camera) -> GameResult {
    let screen_x = player.x * TILE_SIZE - camera.x;
    let screen_y = player.y * TILE_SIZE - camera.y;

    if screen_x < -TILE_SIZE || screen_x > MAP_VIEW_WIDTH || 
       screen_y < -TILE_SIZE || screen_y > MAP_VIEW_HEIGHT {
        return Ok(());
    }

    // Ciało postaci
    let body = Mesh::new_rectangle(
        ctx, DrawMode::fill(),
        Rect::new(0.0, 0.0, TILE_SIZE - 2.0, TILE_SIZE - 2.0),
        Color::new(1.0, 0.85, 0.0, 1.0), // złoty
    )?;
    canvas.draw(&body, DrawParam::default().dest(Vec2::new(screen_x + 1.0, screen_y + 1.0)));

    // Litera G na postaci
    let label = Text::new(
        TextFragment::new("G").color(Color::new(0.1, 0.1, 0.1, 1.0)).scale(PxScale { x: 18.0, y: 18.0 })
    );
    canvas.draw(&label, DrawParam::default().dest(Vec2::new(screen_x + 5.0, screen_y + 3.0)));

    // Wskaźnik kierunku
    let (dx, dy) = match player.direction {
        crate::player::Direction::Up => (screen_x + 9.0, screen_y - 4.0),
        crate::player::Direction::Down => (screen_x + 9.0, screen_y + TILE_SIZE),
        crate::player::Direction::Left => (screen_x - 4.0, screen_y + 9.0),
        crate::player::Direction::Right => (screen_x + TILE_SIZE, screen_y + 9.0),
    };
    let indicator = Mesh::new_rectangle(
        ctx, DrawMode::fill(),
        Rect::new(0.0, 0.0, 4.0, 4.0),
        Color::new(1.0, 0.3, 0.0, 1.0),
    )?;
    canvas.draw(&indicator, DrawParam::default().dest(Vec2::new(dx, dy)));

    // Quen shield indicator
    if player.quen_shield > 0 {
        let shield = Mesh::new_rectangle(
            ctx, DrawMode::stroke(2.0),
            Rect::new(0.0, 0.0, TILE_SIZE + 2.0, TILE_SIZE + 2.0),
            Color::new(0.3, 0.8, 1.0, 0.8),
        )?;
        canvas.draw(&shield, DrawParam::default().dest(Vec2::new(screen_x - 1.0, screen_y - 1.0)));
    }

    Ok(())
}

pub fn draw_monsters(ctx: &mut Context, canvas: &mut Canvas, world: &WorldMap, camera: &Camera) -> GameResult {
    for monster in &world.monsters {
        if !monster.alive { continue; }
        let screen_x = monster.x * TILE_SIZE - camera.x;
        let screen_y = monster.y * TILE_SIZE - camera.y;

        if screen_x < -TILE_SIZE || screen_x > MAP_VIEW_WIDTH || 
           screen_y < -TILE_SIZE || screen_y > MAP_VIEW_HEIGHT {
            continue;
        }

        let c = monster.monster_type.color();
        let body = Mesh::new_rectangle(
            ctx, DrawMode::fill(),
            Rect::new(0.0, 0.0, TILE_SIZE - 2.0, TILE_SIZE - 2.0),
            Color::new(c[0], c[1], c[2], c[3]),
        )?;
        canvas.draw(&body, DrawParam::default().dest(Vec2::new(screen_x + 1.0, screen_y + 1.0)));

        let ch = monster.monster_type.display_char().to_string();
        let label = Text::new(
            TextFragment::new(ch).color(Color::WHITE).scale(PxScale { x: 16.0, y: 16.0 })
        );
        canvas.draw(&label, DrawParam::default().dest(Vec2::new(screen_x + 6.0, screen_y + 4.0)));

        // Pasek zdrowia potwora
        let hp_ratio = monster.health as f32 / monster.max_health as f32;
        let hp_bg = Mesh::new_rectangle(
            ctx, DrawMode::fill(),
            Rect::new(0.0, 0.0, TILE_SIZE - 2.0, 3.0),
            Color::new(0.3, 0.0, 0.0, 0.8),
        )?;
        canvas.draw(&hp_bg, DrawParam::default().dest(Vec2::new(screen_x + 1.0, screen_y - 5.0)));
        
        if hp_ratio > 0.0 {
            let hp_bar = Mesh::new_rectangle(
                ctx, DrawMode::fill(),
                Rect::new(0.0, 0.0, (TILE_SIZE - 2.0) * hp_ratio, 3.0),
                Color::new(0.8, 0.1, 0.1, 0.9),
            )?;
            canvas.draw(&hp_bar, DrawParam::default().dest(Vec2::new(screen_x + 1.0, screen_y - 5.0)));
        }
    }
    Ok(())
}

pub fn draw_npcs(ctx: &mut Context, canvas: &mut Canvas, world: &WorldMap, camera: &Camera) -> GameResult {
    for npc in &world.npcs {
        let screen_x = npc.x * TILE_SIZE - camera.x;
        let screen_y = npc.y * TILE_SIZE - camera.y;

        if screen_x < -TILE_SIZE || screen_x > MAP_VIEW_WIDTH || 
           screen_y < -TILE_SIZE || screen_y > MAP_VIEW_HEIGHT {
            continue;
        }

        let body = Mesh::new_rectangle(
            ctx, DrawMode::fill(),
            Rect::new(0.0, 0.0, TILE_SIZE - 2.0, TILE_SIZE - 2.0),
            Color::new(npc.color[0], npc.color[1], npc.color[2], npc.color[3]),
        )?;
        canvas.draw(&body, DrawParam::default().dest(Vec2::new(screen_x + 1.0, screen_y + 1.0)));

        let label = Text::new(
            TextFragment::new("N").color(Color::new(0.0, 0.0, 0.0, 1.0)).scale(PxScale { x: 16.0, y: 16.0 })
        );
        canvas.draw(&label, DrawParam::default().dest(Vec2::new(screen_x + 6.0, screen_y + 4.0)));

        // Ikona questu
        if npc.quest_giver {
            let quest_mark = Text::new(
                TextFragment::new("!").color(Color::new(1.0, 1.0, 0.0, 1.0)).scale(PxScale { x: 14.0, y: 14.0 })
            );
            canvas.draw(&quest_mark, DrawParam::default().dest(Vec2::new(screen_x + 8.0, screen_y - 12.0)));
        }
        if npc.merchant {
            let trade_mark = Text::new(
                TextFragment::new("$").color(Color::new(0.0, 1.0, 0.0, 1.0)).scale(PxScale { x: 12.0, y: 12.0 })
            );
            canvas.draw(&trade_mark, DrawParam::default().dest(Vec2::new(screen_x + 16.0, screen_y - 10.0)));
        }
    }
    Ok(())
}

pub fn draw_loot(ctx: &mut Context, canvas: &mut Canvas, world: &WorldMap, camera: &Camera) -> GameResult {
    for loot in &world.loot {
        if loot.collected { continue; }
        let screen_x = loot.x * TILE_SIZE - camera.x;
        let screen_y = loot.y * TILE_SIZE - camera.y;

        if screen_x < -TILE_SIZE || screen_x > MAP_VIEW_WIDTH || 
           screen_y < -TILE_SIZE || screen_y > MAP_VIEW_HEIGHT {
            continue;
        }

        let gem = Mesh::new_rectangle(
            ctx, DrawMode::fill(),
            Rect::new(0.0, 0.0, 8.0, 8.0),
            Color::new(0.0, 1.0, 0.5, 0.9),
        )?;
        canvas.draw(&gem, DrawParam::default().dest(Vec2::new(screen_x + 8.0, screen_y + 8.0)));
    }
    Ok(())
}
