/// System ekwipunku i przedmiotów
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    SteelSword,
    SilverSword,
    Armor,
    Potion,
    Oil,
    Bomb,
    AlchemyIngredient,
    QuestItem,
    Junk,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WeaponMaterial {
    Steel,
    Silver,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArmorWeight {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub item_type: ItemType,
    pub attack_bonus: i32,
    pub defense_bonus: i32,
    pub health_restore: i32,
    pub stamina_restore: i32,
    pub value: i32,
    pub quantity: i32,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Item {
    pub fn steel_sword(name: &str, attack: i32, value: i32) -> Self {
        Item {
            name: name.to_string(),
            description: format!("Stalowy miecz - {} pkt ataku", attack),
            item_type: ItemType::SteelSword,
            attack_bonus: attack,
            defense_bonus: 0,
            health_restore: 0,
            stamina_restore: 0,
            value,
            quantity: 1,
        }
    }

    pub fn silver_sword(name: &str, attack: i32, value: i32) -> Self {
        Item {
            name: name.to_string(),
            description: format!("Srebrny miecz - {} pkt ataku na potwory", attack),
            item_type: ItemType::SilverSword,
            attack_bonus: attack,
            defense_bonus: 0,
            health_restore: 0,
            stamina_restore: 0,
            value,
            quantity: 1,
        }
    }

    pub fn armor(name: &str, defense: i32, value: i32) -> Self {
        Item {
            name: name.to_string(),
            description: format!("Zbroja - {} pkt obrony", defense),
            item_type: ItemType::Armor,
            attack_bonus: 0,
            defense_bonus: defense,
            health_restore: 0,
            stamina_restore: 0,
            value,
            quantity: 1,
        }
    }

    pub fn potion(name: &str, health: i32, stamina: i32, value: i32) -> Self {
        Item {
            name: name.to_string(),
            description: format!("Eliksir - przywraca {} zdrowia, {} wytrzymałości", health, stamina),
            item_type: ItemType::Potion,
            attack_bonus: 0,
            defense_bonus: 0,
            health_restore: health,
            stamina_restore: stamina,
            value,
            quantity: 1,
        }
    }

    pub fn oil(name: &str, attack: i32, value: i32) -> Self {
        Item {
            name: name.to_string(),
            description: format!("Olej na broń - +{} do ataku", attack),
            item_type: ItemType::Oil,
            attack_bonus: attack,
            defense_bonus: 0,
            health_restore: 0,
            stamina_restore: 0,
            value,
            quantity: 1,
        }
    }

    pub fn bomb(name: &str, power: i32, value: i32) -> Self {
        Item {
            name: name.to_string(),
            description: format!("Bomba - {} pkt obrażeń obszarowych", power),
            item_type: ItemType::Bomb,
            attack_bonus: power,
            defense_bonus: 0,
            health_restore: 0,
            stamina_restore: 0,
            value,
            quantity: 1,
        }
    }

    pub fn ingredient(name: &str, value: i32) -> Self {
        Item {
            name: name.to_string(),
            description: "Składnik alchemiczny".to_string(),
            item_type: ItemType::AlchemyIngredient,
            attack_bonus: 0,
            defense_bonus: 0,
            health_restore: 0,
            stamina_restore: 0,
            value,
            quantity: 1,
        }
    }

    pub fn quest_item(name: &str, desc: &str) -> Self {
        Item {
            name: name.to_string(),
            description: desc.to_string(),
            item_type: ItemType::QuestItem,
            attack_bonus: 0,
            defense_bonus: 0,
            health_restore: 0,
            stamina_restore: 0,
            value: 0,
            quantity: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub gold: i32,
    pub equipped_steel_sword: Option<usize>,
    pub equipped_silver_sword: Option<usize>,
    pub equipped_armor: Option<usize>,
    pub active_oil: Option<usize>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: Vec::new(),
            gold: 100,
            equipped_steel_sword: None,
            equipped_silver_sword: None,
            equipped_armor: None,
            active_oil: None,
        }
    }

    pub fn add_item(&mut self, item: Item) {
        // Sprawdź czy taki przedmiot stackowalny już istnieje
        if matches!(item.item_type, ItemType::AlchemyIngredient | ItemType::Potion | ItemType::Bomb) {
            for existing in &mut self.items {
                if existing.name == item.name {
                    existing.quantity += item.quantity;
                    return;
                }
            }
        }
        self.items.push(item);
    }

    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.items.len() {
            let item = &mut self.items[index];
            if item.quantity > 1 {
                item.quantity -= 1;
                return Some(item.clone());
            }
            // Aktualizuj indeksy wyposażenia
            if self.equipped_steel_sword == Some(index) { self.equipped_steel_sword = None; }
            if self.equipped_silver_sword == Some(index) { self.equipped_silver_sword = None; }
            if self.equipped_armor == Some(index) { self.equipped_armor = None; }
            if self.active_oil == Some(index) { self.active_oil = None; }
            // Przesuń indeksy
            if let Some(ref mut idx) = self.equipped_steel_sword { if *idx > index { *idx -= 1; } }
            if let Some(ref mut idx) = self.equipped_silver_sword { if *idx > index { *idx -= 1; } }
            if let Some(ref mut idx) = self.equipped_armor { if *idx > index { *idx -= 1; } }
            if let Some(ref mut idx) = self.active_oil { if *idx > index { *idx -= 1; } }
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn equip_item(&mut self, index: usize) {
        if index >= self.items.len() { return; }
        match self.items[index].item_type {
            ItemType::SteelSword => self.equipped_steel_sword = Some(index),
            ItemType::SilverSword => self.equipped_silver_sword = Some(index),
            ItemType::Armor => self.equipped_armor = Some(index),
            ItemType::Oil => self.active_oil = Some(index),
            _ => {}
        }
    }

    pub fn get_attack_bonus(&self, use_silver: bool) -> i32 {
        let sword_idx = if use_silver {
            self.equipped_silver_sword
        } else {
            self.equipped_steel_sword
        };
        let sword_bonus = sword_idx.map_or(0, |i| self.items[i].attack_bonus);
        let oil_bonus = self.active_oil.map_or(0, |i| self.items[i].attack_bonus);
        sword_bonus + oil_bonus
    }

    pub fn get_defense_bonus(&self) -> i32 {
        self.equipped_armor.map_or(0, |i| self.items[i].defense_bonus)
    }

    pub fn starter_equipment() -> Self {
        let mut inv = Inventory::new();
        inv.add_item(Item::steel_sword("Stalowy miecz szkoły Dzika", 15, 200));
        inv.add_item(Item::silver_sword("Srebrny miecz szkoły Dzika", 20, 300));
        inv.add_item(Item::armor("Zbroja szkoły Dzika", 12, 250));
        inv.add_item(Item::potion("Jaskółka", 50, 0, 30));
        inv.add_item(Item::potion("Jaskółka", 50, 0, 30));
        inv.add_item(Item::potion("Piołun", 0, 40, 25));
        inv.add_item(Item::bomb("Samum", 30, 20));
        inv.add_item(Item::bomb("Samum", 30, 20));
        inv.equipped_steel_sword = Some(0);
        inv.equipped_silver_sword = Some(1);
        inv.equipped_armor = Some(2);
        inv
    }
}
