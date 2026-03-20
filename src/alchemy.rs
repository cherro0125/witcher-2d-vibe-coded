/// System alchemii - eliksiry, oleje, bomby
use crate::inventory::Item;

#[derive(Debug, Clone)]
pub struct AlchemyRecipe {
    pub name: String,
    pub description: String,
    pub ingredients: Vec<(String, i32)>, // (nazwa składnika, ilość)
    pub result: Item,
}

impl AlchemyRecipe {
    pub fn all_recipes() -> Vec<AlchemyRecipe> {
        vec![
            AlchemyRecipe {
                name: "Jaskółka".into(),
                description: "Eliksir regenerujący zdrowie".into(),
                ingredients: vec![("Jaskier".into(), 2), ("Berberysy".into(), 1)],
                result: Item::potion("Jaskółka", 50, 0, 30),
            },
            AlchemyRecipe {
                name: "Piołun".into(),
                description: "Eliksir przywracający wytrzymałość".into(),
                ingredients: vec![("Piołun ziołowy".into(), 2), ("Trujący bluszcz".into(), 1)],
                result: Item::potion("Piołun", 0, 40, 25),
            },
            AlchemyRecipe {
                name: "Kot".into(),
                description: "Eliksir poprawiający widzenie w ciemności".into(),
                ingredients: vec![("Oczy endriagi".into(), 1), ("Berberysy".into(), 2)],
                result: Item::potion("Kot", 20, 20, 35),
            },
            AlchemyRecipe {
                name: "Petri".into(),
                description: "Eliksir wzmacniający intensywność znaków".into(),
                ingredients: vec![("Rdest".into(), 2), ("Piołun ziołowy".into(), 1)],
                result: Item::potion("Eliksir Petriego", 0, 30, 40),
            },
            AlchemyRecipe {
                name: "Olej na nieumarłych".into(),
                description: "Olej skuteczny przeciw ghulom i utopcom".into(),
                ingredients: vec![("Trujący bluszcz".into(), 2), ("Tłuszcz niedźwiedzi".into(), 1)],
                result: Item::oil("Olej na nieumarłych", 15, 25),
            },
            AlchemyRecipe {
                name: "Olej na bestie".into(),
                description: "Olej skuteczny przeciw gryfom i wilkołakom".into(),
                ingredients: vec![("Jaskier".into(), 1), ("Tłuszcz niedźwiedzi".into(), 2)],
                result: Item::oil("Olej na bestie", 15, 25),
            },
            AlchemyRecipe {
                name: "Olej na insektoidy".into(),
                description: "Olej skuteczny przeciw endriagom i kikimorom".into(),
                ingredients: vec![("Oczy endriagi".into(), 2), ("Rdest".into(), 1)],
                result: Item::oil("Olej na insektoidy", 15, 25),
            },
            AlchemyRecipe {
                name: "Samum".into(),
                description: "Bomba oślepiająca i ogłuszająca".into(),
                ingredients: vec![("Saletra".into(), 2), ("Rdest".into(), 1)],
                result: Item::bomb("Samum", 30, 20),
            },
            AlchemyRecipe {
                name: "Księżycowy Pył".into(),
                description: "Bomba blokująca zdolności potworów".into(),
                ingredients: vec![("Saletra".into(), 1), ("Berberysy".into(), 2)],
                result: Item::bomb("Księżycowy Pył", 25, 25),
            },
        ]
    }
}

pub fn check_can_craft(recipe: &AlchemyRecipe, ingredients: &[(String, i32)]) -> bool {
    for (needed_name, needed_qty) in &recipe.ingredients {
        let have = ingredients.iter()
            .filter(|(name, _)| name == needed_name)
            .map(|(_, qty)| qty)
            .sum::<i32>();
        if have < *needed_qty {
            return false;
        }
    }
    true
}
