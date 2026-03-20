/// Znaki wiedźmińskie - 5 znaków Gerarda z Rumii
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SignType {
    Aard,
    Igni,
    Quen,
    Yrden,
    Axii,
}

impl fmt::Display for SignType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignType::Aard => write!(f, "Aard"),
            SignType::Igni => write!(f, "Igni"),
            SignType::Quen => write!(f, "Quen"),
            SignType::Yrden => write!(f, "Yrden"),
            SignType::Axii => write!(f, "Axii"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sign {
    pub sign_type: SignType,
    pub name: String,
    pub description: String,
    pub stamina_cost: i32,
    pub base_power: i32,
    pub level: i32,
}

impl Sign {
    pub fn all_signs() -> Vec<Sign> {
        vec![
            Sign {
                sign_type: SignType::Aard,
                name: "Aard".to_string(),
                description: "Telekinetyczny podmuch odrzucający wrogów".to_string(),
                stamina_cost: 15,
                base_power: 20,
                level: 1,
            },
            Sign {
                sign_type: SignType::Igni,
                name: "Igni".to_string(),
                description: "Strumień płomieni podpalający przeciwników".to_string(),
                stamina_cost: 20,
                base_power: 35,
                level: 1,
            },
            Sign {
                sign_type: SignType::Quen,
                name: "Quen".to_string(),
                description: "Ochronna bariera pochłaniająca obrażenia".to_string(),
                stamina_cost: 25,
                base_power: 40,
                level: 1,
            },
            Sign {
                sign_type: SignType::Yrden,
                name: "Yrden".to_string(),
                description: "Magiczny krąg spowalniający wrogów".to_string(),
                stamina_cost: 15,
                base_power: 15,
                level: 1,
            },
            Sign {
                sign_type: SignType::Axii,
                name: "Axii".to_string(),
                description: "Kontrola umysłu - oszałamia przeciwnika".to_string(),
                stamina_cost: 20,
                base_power: 10,
                level: 1,
            },
        ]
    }

    pub fn effective_power(&self) -> i32 {
        self.base_power + (self.level - 1) * 5
    }
}
