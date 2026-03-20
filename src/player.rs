/// Postać gracza - Gerard z Rumii ze szkoły Dzika
use crate::signs::{Sign, SignType};
use crate::inventory::Inventory;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub school: String,
    pub level: i32,
    pub experience: i32,
    pub exp_to_next_level: i32,

    // Statystyki
    pub max_health: i32,
    pub health: i32,
    pub max_stamina: i32,
    pub stamina: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,

    // Pozycja na mapie
    pub x: f32,
    pub y: f32,
    pub direction: Direction,

    // Znaki wiedźmińskie
    pub signs: Vec<Sign>,
    pub active_sign: usize,

    // Ekwipunek
    pub inventory: Inventory,

    // Status
    pub quen_shield: i32,  // ilość obrażeń pochłoniętych przez Quen
    pub yrden_active: bool,
    pub kills: i32,
    pub quests_completed: i32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            name: "Gerard z Rumii".to_string(),
            school: "Szkoła Dzika".to_string(),
            level: 1,
            experience: 0,
            exp_to_next_level: 100,

            max_health: 150,
            health: 150,
            max_stamina: 100,
            stamina: 100,
            strength: 12,
            dexterity: 10,
            intelligence: 8,

            x: 25.0,
            y: 18.0,
            direction: Direction::Down,

            signs: Sign::all_signs(),
            active_sign: 0,

            inventory: Inventory::starter_equipment(),

            quen_shield: 0,
            yrden_active: false,
            kills: 0,
            quests_completed: 0,
        }
    }

    pub fn gain_experience(&mut self, amount: i32) -> bool {
        self.experience += amount;
        if self.experience >= self.exp_to_next_level {
            self.level_up();
            return true;
        }
        false
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.experience -= self.exp_to_next_level;
        self.exp_to_next_level = 100 + (self.level * 50);

        self.max_health += 20;
        self.health = self.max_health;
        self.max_stamina += 10;
        self.stamina = self.max_stamina;
        self.strength += 2;
        self.dexterity += 1;
        self.intelligence += 1;
    }

    pub fn attack_power(&self, use_silver: bool) -> i32 {
        let base = self.strength * 2 + self.dexterity;
        let weapon_bonus = self.inventory.get_attack_bonus(use_silver);
        base + weapon_bonus
    }

    pub fn defense_power(&self) -> i32 {
        let base = self.dexterity + self.strength / 2;
        let armor_bonus = self.inventory.get_defense_bonus();
        base + armor_bonus
    }

    pub fn use_sign(&mut self, sign_index: usize) -> Option<&Sign> {
        if sign_index >= self.signs.len() { return None; }
        let cost = self.signs[sign_index].stamina_cost;
        if self.stamina >= cost {
            self.stamina -= cost;
            if self.signs[sign_index].sign_type == SignType::Quen {
                self.quen_shield = self.signs[sign_index].effective_power();
            }
            if self.signs[sign_index].sign_type == SignType::Yrden {
                self.yrden_active = true;
            }
            Some(&self.signs[sign_index])
        } else {
            None
        }
    }

    pub fn take_damage(&mut self, damage: i32) -> i32 {
        let defense = self.defense_power();
        let mut actual_damage = (damage - defense / 2).max(1);

        // Quen pochłania obrażenia
        if self.quen_shield > 0 {
            if self.quen_shield >= actual_damage {
                self.quen_shield -= actual_damage;
                return 0;
            } else {
                actual_damage -= self.quen_shield;
                self.quen_shield = 0;
            }
        }

        self.health = (self.health - actual_damage).max(0);
        actual_damage
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn restore_stamina(&mut self, amount: i32) {
        self.stamina = (self.stamina + amount).min(self.max_stamina);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn rest(&mut self) {
        self.health = self.max_health;
        self.stamina = self.max_stamina;
        self.quen_shield = 0;
        self.yrden_active = false;
    }

    pub fn regenerate_tick(&mut self) {
        // Powolna regeneracja wytrzymałości
        self.stamina = (self.stamina + 1).min(self.max_stamina);
    }
}
