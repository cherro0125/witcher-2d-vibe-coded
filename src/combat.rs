/// System walki turowej
use crate::player::Player;
use crate::monsters::Monster;
use crate::signs::SignType;
use crate::inventory::ItemType;

#[derive(Debug, Clone, PartialEq)]
pub enum CombatAction {
    AttackSteel,
    AttackSilver,
    UseSign(usize),
    UsePotion(usize),
    UseBomb(usize),
    Dodge,
    Parry,
}

#[derive(Debug, Clone)]
pub struct CombatLog {
    pub messages: Vec<String>,
}

impl CombatLog {
    pub fn new() -> Self { CombatLog { messages: Vec::new() } }
    pub fn add(&mut self, msg: String) { self.messages.push(msg); }
    pub fn last_messages(&self, count: usize) -> Vec<&String> {
        let start = if self.messages.len() > count { self.messages.len() - count } else { 0 };
        self.messages[start..].iter().collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CombatState {
    PlayerTurn,
    EnemyTurn,
    Victory,
    Defeat,
}

#[derive(Debug, Clone)]
pub struct Combat {
    pub state: CombatState,
    pub turn: i32,
    pub log: CombatLog,
    pub player_dodging: bool,
    pub player_parrying: bool,
}

impl Combat {
    pub fn new() -> Self {
        let mut combat = Combat {
            state: CombatState::PlayerTurn,
            turn: 1,
            log: CombatLog::new(),
            player_dodging: false,
            player_parrying: false,
        };
        combat.log.add("=== WALKA ROZPOCZĘTA ===".into());
        combat
    }

    pub fn execute_player_action(
        &mut self, action: CombatAction, player: &mut Player, monster: &mut Monster
    ) {
        self.player_dodging = false;
        self.player_parrying = false;

        match action {
            CombatAction::AttackSteel => {
                let damage = player.attack_power(false);
                let bonus = if monster.monster_type.is_human() { 5 } else { 0 };
                let total = damage + bonus;
                let actual = monster.take_damage(total);
                self.log.add(format!(
                    "Gerard atakuje {} stalowym mieczem! {} obrażeń.", monster.name, actual
                ));
            }
            CombatAction::AttackSilver => {
                let damage = player.attack_power(true);
                let bonus = if !monster.monster_type.is_human() { 10 } else { 0 };
                let total = damage + bonus;
                let actual = monster.take_damage(total);
                self.log.add(format!(
                    "Gerard atakuje {} srebrnym mieczem! {} obrażeń.", monster.name, actual
                ));
            }
            CombatAction::UseSign(idx) => {
                if let Some(sign) = player.use_sign(idx) {
                    let sign_type = sign.sign_type;
                    let power = sign.effective_power();
                    let sign_name = sign.name.clone();
                    let is_weakness = sign_type == monster.weakness_sign;
                    let bonus = if is_weakness { power / 2 } else { 0 };

                    match sign_type {
                        SignType::Aard => {
                            let dmg = monster.take_damage(power + bonus);
                            monster.stunned_turns += if is_weakness { 2 } else { 1 };
                            self.log.add(format!(
                                "Gerard używa {}! {} obrażeń, wróg oszołomiony!{}",
                                sign_name, dmg,
                                if is_weakness { " SŁABOŚĆ!" } else { "" }
                            ));
                        }
                        SignType::Igni => {
                            let dmg = monster.take_damage(power + bonus);
                            self.log.add(format!(
                                "Gerard używa {}! Strumień ognia zadaje {} obrażeń!{}",
                                sign_name, dmg,
                                if is_weakness { " SŁABOŚĆ!" } else { "" }
                            ));
                        }
                        SignType::Quen => {
                            self.log.add(format!(
                                "Gerard używa {}! Tarcza ochronna ({} pkt).",
                                sign_name, player.quen_shield
                            ));
                        }
                        SignType::Yrden => {
                            monster.slowed = true;
                            let dmg = monster.take_damage(power / 2 + bonus);
                            self.log.add(format!(
                                "Gerard używa {}! Magiczny krąg spowalnia wroga. {} obr.{}",
                                sign_name, dmg,
                                if is_weakness { " SŁABOŚĆ!" } else { "" }
                            ));
                        }
                        SignType::Axii => {
                            monster.stunned_turns += if is_weakness { 3 } else { 2 };
                            self.log.add(format!(
                                "Gerard używa {}! Wróg oszołomiony na {} tury!{}",
                                sign_name, monster.stunned_turns,
                                if is_weakness { " SŁABOŚĆ!" } else { "" }
                            ));
                        }
                    }
                } else {
                    self.log.add("Za mało wytrzymałości na znak!".into());
                    return;
                }
            }
            CombatAction::UsePotion(inv_idx) => {
                if inv_idx < player.inventory.items.len() {
                    let item = player.inventory.items[inv_idx].clone();
                    if item.item_type == ItemType::Potion {
                        player.heal(item.health_restore);
                        player.restore_stamina(item.stamina_restore);
                        self.log.add(format!(
                            "Gerard pije {}! +{} zdrowia, +{} wytrzymałości.",
                            item.name, item.health_restore, item.stamina_restore
                        ));
                        player.inventory.remove_item(inv_idx);
                    }
                }
            }
            CombatAction::UseBomb(inv_idx) => {
                if inv_idx < player.inventory.items.len() {
                    let item = player.inventory.items[inv_idx].clone();
                    if item.item_type == ItemType::Bomb {
                        let dmg = monster.take_damage(item.attack_bonus);
                        monster.stunned_turns += 1;
                        self.log.add(format!(
                            "Gerard rzuca {}! {} obrażeń, wróg oszołomiony!",
                            item.name, dmg
                        ));
                        player.inventory.remove_item(inv_idx);
                    }
                }
            }
            CombatAction::Dodge => {
                self.player_dodging = true;
                self.log.add("Gerard przygotowuje się do uniku.".into());
            }
            CombatAction::Parry => {
                self.player_parrying = true;
                self.log.add("Gerard przygotowuje się do parowania.".into());
            }
        }

        if !monster.is_alive() {
            self.state = CombatState::Victory;
            self.log.add(format!("=== {} POKONANY! ===", monster.name));
            return;
        }

        self.state = CombatState::EnemyTurn;
    }

    pub fn execute_enemy_turn(&mut self, player: &mut Player, monster: &mut Monster) {
        if monster.stunned_turns > 0 {
            monster.stunned_turns -= 1;
            self.log.add(format!("{} jest oszołomiony i traci turę!", monster.name));
            player.regenerate_tick();
            self.state = CombatState::PlayerTurn;
            self.turn += 1;
            return;
        }

        let attack = monster.attack_power();

        if self.player_dodging {
            let dodge_roll = (self.turn * 7 + attack) % 10;
            if dodge_roll < 6 {
                self.log.add(format!("Gerard unika ataku {}!", monster.name));
            } else {
                let dmg = player.take_damage(attack / 2);
                self.log.add(format!(
                    "{} trafia mimo uniku! {} obrażeń (częściowe).", monster.name, dmg
                ));
            }
        } else if self.player_parrying {
            let dmg = player.take_damage(attack / 3);
            self.log.add(format!(
                "Gerard paruje atak {}! Tylko {} obrażeń.", monster.name, dmg
            ));
        } else {
            let dmg = player.take_damage(attack);
            self.log.add(format!(
                "{} atakuje Gerarda! {} obrażeń.", monster.name, dmg
            ));
        }

        if !player.is_alive() {
            self.state = CombatState::Defeat;
            self.log.add("=== GERARD POLEGŁ! ===".into());
            return;
        }

        player.regenerate_tick();
        if monster.slowed {
            monster.slowed = false;
        }
        self.state = CombatState::PlayerTurn;
        self.turn += 1;
    }
}
