/// Mapa świata - lokacje, kafelki, NPC
use crate::monsters::Monster;

pub const TILE_SIZE: f32 = 24.0;
pub const MAP_WIDTH: usize = 60;
pub const MAP_HEIGHT: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Grass,
    Forest,
    Water,
    Road,
    Building,
    Wall,
    Door,
    Bridge,
    Swamp,
    Mountain,
    Cave,
    Sand,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        !matches!(self, TileType::Water | TileType::Wall | TileType::Mountain)
    }

    pub fn color(&self) -> [f32; 4] {
        match self {
            TileType::Grass => [0.2, 0.7, 0.2, 1.0],
            TileType::Forest => [0.1, 0.4, 0.1, 1.0],
            TileType::Water => [0.1, 0.3, 0.8, 1.0],
            TileType::Road => [0.6, 0.55, 0.4, 1.0],
            TileType::Building => [0.55, 0.35, 0.2, 1.0],
            TileType::Wall => [0.4, 0.4, 0.4, 1.0],
            TileType::Door => [0.6, 0.4, 0.15, 1.0],
            TileType::Bridge => [0.5, 0.4, 0.25, 1.0],
            TileType::Swamp => [0.3, 0.45, 0.2, 1.0],
            TileType::Mountain => [0.5, 0.5, 0.5, 1.0],
            TileType::Cave => [0.25, 0.2, 0.2, 1.0],
            TileType::Sand => [0.85, 0.8, 0.5, 1.0],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Npc {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub dialogue_id: String,
    pub quest_giver: bool,
    pub merchant: bool,
    pub color: [f32; 4],
}

#[derive(Debug, Clone)]
pub struct Loot {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub item_name: String,
    pub collected: bool,
}

#[derive(Debug, Clone)]
pub struct WorldMap {
    pub tiles: Vec<Vec<TileType>>,
    pub npcs: Vec<Npc>,
    pub monsters: Vec<Monster>,
    pub loot: Vec<Loot>,
    pub width: usize,
    pub height: usize,
}

impl WorldMap {
    pub fn new() -> Self {
        let mut tiles = vec![vec![TileType::Grass; MAP_WIDTH]; MAP_HEIGHT];

        // === Drogi ===
        for x in 0..MAP_WIDTH {
            tiles[18][x] = TileType::Road; // główna droga pozioma
            tiles[19][x] = TileType::Road;
        }
        for y in 0..MAP_HEIGHT {
            tiles[y][30] = TileType::Road; // droga pionowa
        }

        // === Wioska Rumia (lewy górny) ===
        for y in 3..10 {
            for x in 3..14 {
                if y == 3 || y == 9 || x == 3 || x == 13 {
                    tiles[y][x] = TileType::Wall;
                } else {
                    tiles[y][x] = TileType::Building;
                }
            }
        }
        tiles[9][8] = TileType::Door; // wejście do wioski
        // Domki wewnątrz
        for y in 4..6 { for x in 5..8 { tiles[y][x] = TileType::Building; } }
        for y in 4..6 { for x in 10..13 { tiles[y][x] = TileType::Building; } }
        for y in 7..9 { for x in 5..8 { tiles[y][x] = TileType::Building; } }
        // Droga do wioski
        for y in 10..18 { tiles[y][8] = TileType::Road; }

        // === Las (prawy górny) ===
        for y in 1..16 {
            for x in 35..55 {
                if rand_simple(x, y) > 3 {
                    tiles[y][x] = TileType::Forest;
                }
            }
        }

        // === Bagno (lewy dolny) ===
        for y in 24..35 {
            for x in 2..20 {
                if rand_simple(x, y) > 2 {
                    tiles[y][x] = TileType::Swamp;
                } else {
                    tiles[y][x] = TileType::Water;
                }
            }
        }

        // === Jaskinia (środek-dolny) ===
        for y in 28..36 {
            for x in 28..38 {
                if y == 28 || y == 35 || x == 28 || x == 37 {
                    tiles[y][x] = TileType::Mountain;
                } else {
                    tiles[y][x] = TileType::Cave;
                }
            }
        }
        tiles[28][33] = TileType::Door; // wejście do jaskini

        // === Zamek (prawy dolny) ===
        for y in 26..38 {
            for x in 44..58 {
                if y == 26 || y == 37 || x == 44 || x == 57 {
                    tiles[y][x] = TileType::Wall;
                } else {
                    tiles[y][x] = TileType::Building;
                }
            }
        }
        tiles[26][50] = TileType::Door; // brama zamku
        // Wieże
        for y in 27..30 { for x in 45..48 { tiles[y][x] = TileType::Wall; } }
        for y in 27..30 { for x in 54..57 { tiles[y][x] = TileType::Wall; } }

        // === Rzeka ===
        for y in 0..MAP_HEIGHT {
            let x_off = (y as f32 * 0.3).sin() as usize;
            let rx = 22usize.wrapping_add(x_off);
            if rx < MAP_WIDTH {
                tiles[y][rx] = TileType::Water;
                if rx + 1 < MAP_WIDTH { tiles[y][rx + 1] = TileType::Water; }
            }
        }
        // Most
        tiles[18][22] = TileType::Bridge;
        tiles[18][23] = TileType::Bridge;
        tiles[19][22] = TileType::Bridge;
        tiles[19][23] = TileType::Bridge;

        // === NPC ===
        let npcs = vec![
            Npc {
                name: "Sołtys Bogdan".into(), x: 6.0, y: 5.0,
                dialogue_id: "soltys".into(), quest_giver: true, merchant: false,
                color: [0.9, 0.8, 0.3, 1.0],
            },
            Npc {
                name: "Handlarz Mirek".into(), x: 11.0, y: 5.0,
                dialogue_id: "handlarz".into(), quest_giver: false, merchant: true,
                color: [0.3, 0.7, 0.9, 1.0],
            },
            Npc {
                name: "Zielarka Bożena".into(), x: 6.0, y: 8.0,
                dialogue_id: "zielarka".into(), quest_giver: true, merchant: true,
                color: [0.4, 0.9, 0.4, 1.0],
            },
            Npc {
                name: "Stary Wiedźmin Vesimir".into(), x: 50.0, y: 30.0,
                dialogue_id: "vesimir".into(), quest_giver: true, merchant: false,
                color: [0.9, 0.9, 0.9, 1.0],
            },
            Npc {
                name: "Tajemniczy Elf".into(), x: 45.0, y: 8.0,
                dialogue_id: "elf".into(), quest_giver: true, merchant: false,
                color: [0.5, 0.9, 0.7, 1.0],
            },
        ];

        // === Potwory ===
        let monsters = vec![
            Monster::ghul(38.0, 10.0, 1),
            Monster::ghul(42.0, 12.0, 1),
            Monster::utopiec(8.0, 28.0, 2),
            Monster::utopiec(12.0, 30.0, 1),
            Monster::kikimora(15.0, 32.0, 2),
            Monster::endriaga(40.0, 5.0, 2),
            Monster::endriaga(48.0, 14.0, 3),
            Monster::gryf(50.0, 3.0, 3),
            Monster::wilkolak(33.0, 31.0, 4),
            Monster::bazyliszek(34.0, 33.0, 5),
            Monster::bandyta(25.0, 18.0, 1),
            Monster::bandyta(35.0, 19.0, 2),
            Monster::leszen(44.0, 6.0, 6),
        ];

        // === Loot (przedmioty do zebrania) ===
        let loot = vec![
            Loot { name: "Jaskier".into(), x: 36.0, y: 4.0, item_name: "Jaskier".into(), collected: false },
            Loot { name: "Jaskier".into(), x: 40.0, y: 7.0, item_name: "Jaskier".into(), collected: false },
            Loot { name: "Berberysy".into(), x: 38.0, y: 9.0, item_name: "Berberysy".into(), collected: false },
            Loot { name: "Rdest".into(), x: 43.0, y: 11.0, item_name: "Rdest".into(), collected: false },
            Loot { name: "Piołun ziołowy".into(), x: 10.0, y: 26.0, item_name: "Piołun ziołowy".into(), collected: false },
            Loot { name: "Piołun ziołowy".into(), x: 14.0, y: 29.0, item_name: "Piołun ziołowy".into(), collected: false },
            Loot { name: "Trujący bluszcz".into(), x: 6.0, y: 31.0, item_name: "Trujący bluszcz".into(), collected: false },
            Loot { name: "Saletra".into(), x: 32.0, y: 30.0, item_name: "Saletra".into(), collected: false },
            Loot { name: "Saletra".into(), x: 35.0, y: 32.0, item_name: "Saletra".into(), collected: false },
            Loot { name: "Tłuszcz niedźwiedzi".into(), x: 47.0, y: 10.0, item_name: "Tłuszcz niedźwiedzi".into(), collected: false },
            Loot { name: "Oczy endriagi".into(), x: 52.0, y: 6.0, item_name: "Oczy endriagi".into(), collected: false },
            Loot { name: "Rdest".into(), x: 16.0, y: 27.0, item_name: "Rdest".into(), collected: false },
        ];

        WorldMap { tiles, npcs, monsters, loot, width: MAP_WIDTH, height: MAP_HEIGHT }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> TileType {
        if x < self.width && y < self.height {
            self.tiles[y][x]
        } else {
            TileType::Wall
        }
    }

    pub fn is_walkable(&self, x: f32, y: f32) -> bool {
        let tx = x as usize;
        let ty = y as usize;
        if tx >= self.width || ty >= self.height { return false; }
        self.tiles[ty][tx].is_walkable()
    }

    pub fn find_monster_at(&self, x: f32, y: f32, radius: f32) -> Option<usize> {
        for (i, m) in self.monsters.iter().enumerate() {
            if m.alive && (m.x - x).abs() < radius && (m.y - y).abs() < radius {
                return Some(i);
            }
        }
        None
    }

    pub fn find_npc_at(&self, x: f32, y: f32, radius: f32) -> Option<usize> {
        for (i, npc) in self.npcs.iter().enumerate() {
            if (npc.x - x).abs() < radius && (npc.y - y).abs() < radius {
                return Some(i);
            }
        }
        None
    }

    pub fn find_loot_at(&self, x: f32, y: f32, radius: f32) -> Option<usize> {
        for (i, l) in self.loot.iter().enumerate() {
            if !l.collected && (l.x - x).abs() < radius && (l.y - y).abs() < radius {
                return Some(i);
            }
        }
        None
    }

    pub fn location_name(&self, x: f32, y: f32) -> &'static str {
        let tx = x as usize;
        let ty = y as usize;
        if tx >= 3 && tx <= 13 && ty >= 3 && ty <= 9 { return "Wioska Rumia"; }
        if tx >= 35 && tx <= 55 && ty >= 1 && ty <= 16 { return "Mroczny Las"; }
        if tx >= 2 && tx <= 20 && ty >= 24 && ty <= 35 { return "Bagno"; }
        if tx >= 28 && tx <= 38 && ty >= 28 && ty <= 36 { return "Jaskinia"; }
        if tx >= 44 && tx <= 58 && ty >= 26 && ty <= 38 { return "Zamek"; }
        "Dzicz"
    }
}

fn rand_simple(x: usize, y: usize) -> usize {
    (x * 7 + y * 13 + x * y) % 7
}
