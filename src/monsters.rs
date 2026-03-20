/// Bestiariusz - potwory świata wiedźmińskiego
use crate::signs::SignType;

#[derive(Debug, Clone, PartialEq)]
pub enum MonsterType {
    Ghul,
    Utopiec,
    Gryf,
    Wilkolak,
    Endriaga,
    Kikimora,
    Bazyliszek,
    Leszen,
    Bandyta,    // człowiek
    Dezercja,   // człowiek
}

impl MonsterType {
    pub fn is_human(&self) -> bool {
        matches!(self, MonsterType::Bandyta | MonsterType::Dezercja)
    }

    pub fn display_char(&self) -> char {
        match self {
            MonsterType::Ghul => 'G',
            MonsterType::Utopiec => 'U',
            MonsterType::Gryf => 'Y',
            MonsterType::Wilkolak => 'W',
            MonsterType::Endriaga => 'E',
            MonsterType::Kikimora => 'K',
            MonsterType::Bazyliszek => 'B',
            MonsterType::Leszen => 'L',
            MonsterType::Bandyta => 'b',
            MonsterType::Dezercja => 'd',
        }
    }

    pub fn color(&self) -> [f32; 4] {
        match self {
            MonsterType::Ghul => [0.6, 0.8, 0.2, 1.0],
            MonsterType::Utopiec => [0.2, 0.5, 0.8, 1.0],
            MonsterType::Gryf => [0.8, 0.6, 0.2, 1.0],
            MonsterType::Wilkolak => [0.5, 0.3, 0.1, 1.0],
            MonsterType::Endriaga => [0.7, 0.1, 0.1, 1.0],
            MonsterType::Kikimora => [0.3, 0.3, 0.3, 1.0],
            MonsterType::Bazyliszek => [0.8, 0.2, 0.8, 1.0],
            MonsterType::Leszen => [0.1, 0.4, 0.1, 1.0],
            MonsterType::Bandyta => [0.7, 0.5, 0.3, 1.0],
            MonsterType::Dezercja => [0.6, 0.6, 0.4, 1.0],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monster {
    pub name: String,
    pub monster_type: MonsterType,
    pub level: i32,
    pub max_health: i32,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub experience_reward: i32,
    pub gold_reward: i32,
    pub weakness_sign: SignType,
    pub weakness_oil: String,
    pub description: String,
    pub x: f32,
    pub y: f32,
    pub alive: bool,
    pub stunned_turns: i32,
    pub slowed: bool,
}

impl Monster {
    pub fn take_damage(&mut self, damage: i32) -> i32 {
        let actual = (damage - self.defense / 3).max(1);
        self.health = (self.health - actual).max(0);
        if self.health <= 0 { self.alive = false; }
        actual
    }

    pub fn attack_power(&self) -> i32 {
        if self.stunned_turns > 0 { return 0; }
        let mult = if self.slowed { 0.6 } else { 1.0 };
        (self.attack as f32 * mult) as i32
    }

    pub fn is_alive(&self) -> bool {
        self.alive && self.health > 0
    }

    pub fn ghul(x: f32, y: f32, level: i32) -> Self {
        Monster {
            name: "Ghul".into(), monster_type: MonsterType::Ghul, level,
            max_health: 60 + level * 15, health: 60 + level * 15,
            attack: 12 + level * 3, defense: 4 + level,
            experience_reward: 25 + level * 10, gold_reward: 5 + level * 3,
            weakness_sign: SignType::Igni, weakness_oil: "Olej na nieumarłych".into(),
            description: "Ohydny nieumarły żywiący się trupami".into(),
            x, y, alive: true, stunned_turns: 0, slowed: false,
        }
    }

    pub fn utopiec(x: f32, y: f32, level: i32) -> Self {
        Monster {
            name: "Utopiec".into(), monster_type: MonsterType::Utopiec, level,
            max_health: 70 + level * 15, health: 70 + level * 15,
            attack: 14 + level * 3, defense: 6 + level,
            experience_reward: 30 + level * 10, gold_reward: 8 + level * 3,
            weakness_sign: SignType::Yrden, weakness_oil: "Olej na nieumarłych".into(),
            description: "Wodny potwór czyhający w bagnach".into(),
            x, y, alive: true, stunned_turns: 0, slowed: false,
        }
    }

    pub fn gryf(x: f32, y: f32, level: i32) -> Self {
        Monster {
            name: "Gryf".into(), monster_type: MonsterType::Gryf, level,
            max_health: 120 + level * 20, health: 120 + level * 20,
            attack: 22 + level * 4, defense: 10 + level * 2,
            experience_reward: 60 + level * 15, gold_reward: 25 + level * 5,
            weakness_sign: SignType::Aard, weakness_oil: "Olej na bestie".into(),
            description: "Potężna latająca bestia z ostrymi szponami".into(),
            x, y, alive: true, stunned_turns: 0, slowed: false,
        }
    }

    pub fn wilkolak(x: f32, y: f32, level: i32) -> Self {
        Monster {
            name: "Wilkołak".into(), monster_type: MonsterType::Wilkolak, level,
            max_health: 150 + level * 25, health: 150 + level * 25,
            attack: 25 + level * 5, defense: 12 + level * 2,
            experience_reward: 80 + level * 20, gold_reward: 30 + level * 5,
            weakness_sign: SignType::Igni, weakness_oil: "Olej na bestie".into(),
            description: "Przeklęty człowiek-wilk o nadludzkiej sile".into(),
            x, y, alive: true, stunned_turns: 0, slowed: false,
        }
    }

    pub fn endriaga(x: f32, y: f32, level: i32) -> Self {
        Monster {
            name: "Endriaga".into(), monster_type: MonsterType::Endriaga, level,
            max_health: 80 + level * 15, health: 80 + level * 15,
            attack: 18 + level * 4, defense: 8 + level * 2,
            experience_reward: 35 + level * 10, gold_reward: 10 + level * 3,
            weakness_sign: SignType::Igni, weakness_oil: "Olej na insektoidy".into(),
            description: "Agresywny insektoid z twardym pancerzem".into(),
            x, y, alive: true, stunned_turns: 0, slowed: false,
        }
    }

    pub fn kikimora(x: f32, y: f32, level: i32) -> Self {
        Monster {
            name: "Kikimora".into(), monster_type: MonsterType::Kikimora, level,
            max_health: 90 + level * 18, health: 90 + level * 18,
            attack: 20 + level * 4, defense: 10 + level * 2,
            experience_reward: 40 + level * 12, gold_reward: 12 + level * 4,
            weakness_sign: SignType::Yrden, weakness_oil: "Olej na insektoidy".into(),
            description: "Wielki pająkowaty insektoid z bagien".into(),
            x, y, alive: true, stunned_turns: 0, slowed: false,
        }
    }

    pub fn bazyliszek(x: f32, y: f32, level: i32) -> Self {
        Monster {
            name: "Bazyliszek".into(), monster_type: MonsterType::Bazyliszek, level,
            max_health: 200 + level * 30, health: 200 + level * 30,
            attack: 30 + level * 6, defense: 15 + level * 3,
            experience_reward: 100 + level * 25, gold_reward: 50 + level * 10,
            weakness_sign: SignType::Aard, weakness_oil: "Olej na drakonidy".into(),
            description: "Przerażający gad zdolny zamienić w kamień spojrzeniem".into(),
            x, y, alive: true, stunned_turns: 0, slowed: false,
        }
    }

    pub fn leszen(x: f32, y: f32, level: i32) -> Self {
        Monster {
            name: "Leszy".into(), monster_type: MonsterType::Leszen, level,
            max_health: 250 + level * 35, health: 250 + level * 35,
            attack: 28 + level * 5, defense: 18 + level * 3,
            experience_reward: 120 + level * 30, gold_reward: 60 + level * 10,
            weakness_sign: SignType::Igni, weakness_oil: "Olej na relikty".into(),
            description: "Starożytny strażnik lasu o potwornej mocy".into(),
            x, y, alive: true, stunned_turns: 0, slowed: false,
        }
    }

    pub fn bandyta(x: f32, y: f32, level: i32) -> Self {
        Monster {
            name: "Bandyta".into(), monster_type: MonsterType::Bandyta, level,
            max_health: 50 + level * 10, health: 50 + level * 10,
            attack: 10 + level * 2, defense: 5 + level,
            experience_reward: 15 + level * 5, gold_reward: 15 + level * 5,
            weakness_sign: SignType::Axii, weakness_oil: String::new(),
            description: "Zwykły bandyta grasujący na drogach".into(),
            x, y, alive: true, stunned_turns: 0, slowed: false,
        }
    }
}
