/// System dialogów z wyborami

#[derive(Debug, Clone)]
pub struct DialogueChoice {
    pub text: String,
    pub next_node: Option<String>,
    pub action: DialogueAction,
}

#[derive(Debug, Clone)]
pub enum DialogueAction {
    None,
    StartQuest(String),
    GiveItem(String),
    GiveGold(i32),
    GiveExp(i32),
    Trade,
    Heal,
    EndDialogue,
    Axii(String), // next_node jeśli Axii się uda
}

#[derive(Debug, Clone)]
pub struct DialogueNode {
    pub id: String,
    pub speaker: String,
    pub text: String,
    pub choices: Vec<DialogueChoice>,
}

#[derive(Debug, Clone)]
pub struct Dialogue {
    pub nodes: Vec<DialogueNode>,
    pub current_node: Option<String>,
}

impl Dialogue {
    pub fn get_current_node(&self) -> Option<&DialogueNode> {
        if let Some(ref id) = self.current_node {
            self.nodes.iter().find(|n| n.id == *id)
        } else {
            None
        }
    }

    pub fn start(&mut self) {
        if let Some(first) = self.nodes.first() {
            self.current_node = Some(first.id.clone());
        }
    }

    pub fn select_choice(&mut self, choice_index: usize) -> Option<DialogueAction> {
        let node = self.get_current_node()?.clone();
        if choice_index >= node.choices.len() { return None; }
        let choice = &node.choices[choice_index];
        let action = choice.action.clone();
        self.current_node = choice.next_node.clone();
        Some(action)
    }
}

pub fn get_dialogue(npc_id: &str) -> Dialogue {
    match npc_id {
        "soltys" => Dialogue {
            current_node: None,
            nodes: vec![
                DialogueNode {
                    id: "start".into(), speaker: "Sołtys Bogdan".into(),
                    text: "Witaj, wiedźminie! Gerard z Rumii, prawda? Szkoła Dzika... dawno nie widziałem jednego z was.\nMamy tu problemy z potworami. Utopce na bagnach i coś gorszego w lesie...".into(),
                    choices: vec![
                        DialogueChoice { text: "Opowiedz mi o problemach z potworami.".into(), next_node: Some("quests".into()), action: DialogueAction::None },
                        DialogueChoice { text: "Szukam informacji o Szkole Dzika.".into(), next_node: Some("school".into()), action: DialogueAction::StartQuest("main_school".into()) },
                        DialogueChoice { text: "Do widzenia.".into(), next_node: None, action: DialogueAction::EndDialogue },
                    ],
                },
                DialogueNode {
                    id: "quests".into(), speaker: "Sołtys Bogdan".into(),
                    text: "Utopce nękają ludzi nad bagnem - nikt nie może iść po wodę. Dam ci 100 koron za oczyszczenie bagien.\nA w lesie endriagi zaatakowały drwali - porozmawiaj z Zielarką Bożeną o tym.".into(),
                    choices: vec![
                        DialogueChoice { text: "Zajmę się utopocami. (Przyjmij kontrakt)".into(), next_node: Some("accepted".into()), action: DialogueAction::StartQuest("contract_drowners".into()) },
                        DialogueChoice { text: "A co ze Szkołą Dzika?".into(), next_node: Some("school".into()), action: DialogueAction::None },
                        DialogueChoice { text: "Może później.".into(), next_node: None, action: DialogueAction::EndDialogue },
                    ],
                },
                DialogueNode {
                    id: "school".into(), speaker: "Sołtys Bogdan".into(),
                    text: "Szkoła Dzika? Słyszałem, że stary wiedźmin Vesimir przebywa w zamku na wschodzie.\nOn może wiedzieć coś o zaginionych sekretach waszej szkoły.\nAle droga jest niebezpieczna - las pełen potworów.".into(),
                    choices: vec![
                        DialogueChoice { text: "Dziękuję za informację. Wyruszę do zamku.".into(), next_node: None, action: DialogueAction::EndDialogue },
                        DialogueChoice { text: "A co z kontraktem na utopce?".into(), next_node: Some("quests".into()), action: DialogueAction::None },
                    ],
                },
                DialogueNode {
                    id: "accepted".into(), speaker: "Sołtys Bogdan".into(),
                    text: "Doskonale! Bagno jest na południe od wioski. Uważaj na siebie, wiedźminie.\nOlej na nieumarłych może się przydać - Zielarka Bożena pewnie go ma.".into(),
                    choices: vec![
                        DialogueChoice { text: "Ruszam natychmiast.".into(), next_node: None, action: DialogueAction::EndDialogue },
                    ],
                },
            ],
        },
        "handlarz" => Dialogue {
            current_node: None,
            nodes: vec![
                DialogueNode {
                    id: "start".into(), speaker: "Handlarz Mirek".into(),
                    text: "Witam, witam! Mirek, handlarz z Rumii. Co mogę zaoferować wiedźminowi?\nMam miecze, zbroje, eliksiry - wszystko czego dusza zapragnie!".into(),
                    choices: vec![
                        DialogueChoice { text: "Pokaż mi swój towar. (Handel)".into(), next_node: None, action: DialogueAction::Trade },
                        DialogueChoice { text: "Może później.".into(), next_node: None, action: DialogueAction::EndDialogue },
                    ],
                },
            ],
        },
        "zielarka" => Dialogue {
            current_node: None,
            nodes: vec![
                DialogueNode {
                    id: "start".into(), speaker: "Zielarka Bożena".into(),
                    text: "A, wiedźmin! Dobrze, że jesteś. Endriagi rozpleniły się w lesie.\nDrwale boją się tam wchodzić. Pomożesz?".into(),
                    choices: vec![
                        DialogueChoice { text: "Zajmę się endriagami. (Przyjmij kontrakt)".into(), next_node: Some("accepted".into()), action: DialogueAction::StartQuest("contract_endriaga".into()) },
                        DialogueChoice { text: "Masz jakieś eliksiry na sprzedaż?".into(), next_node: Some("trade".into()), action: DialogueAction::None },
                        DialogueChoice { text: "Mogę cię uleczyć za darmo. [Axii]".into(), next_node: Some("axii_success".into()), action: DialogueAction::Axii("axii_success".into()) },
                        DialogueChoice { text: "Do widzenia.".into(), next_node: None, action: DialogueAction::EndDialogue },
                    ],
                },
                DialogueNode {
                    id: "accepted".into(), speaker: "Zielarka Bożena".into(),
                    text: "Dzięki ci, wiedźminie! Weź ten eliksir na drogę. Endriagi są wrażliwe na ogień - Igni będzie skuteczne.\nOlej na insektoidy też pomoże.".into(),
                    choices: vec![
                        DialogueChoice { text: "Dziękuję. (Otrzymujesz eliksir Jaskółki)".into(), next_node: None, action: DialogueAction::GiveItem("Jaskółka".into()) },
                    ],
                },
                DialogueNode {
                    id: "trade".into(), speaker: "Zielarka Bożena".into(),
                    text: "Mam sporo ziół i eliksirów. Mogę też kupić składniki alchemiczne.".into(),
                    choices: vec![
                        DialogueChoice { text: "Pokaż towar. (Handel)".into(), next_node: None, action: DialogueAction::Trade },
                        DialogueChoice { text: "Wracam do innych spraw.".into(), next_node: Some("start".into()), action: DialogueAction::None },
                    ],
                },
                DialogueNode {
                    id: "axii_success".into(), speaker: "Zielarka Bożena".into(),
                    text: "Tak... masz rację... weź ten eliksir za darmo... *oszołomiona*".into(),
                    choices: vec![
                        DialogueChoice { text: "Dziękuję. (Otrzymujesz eliksir Kota)".into(), next_node: None, action: DialogueAction::GiveItem("Kot".into()) },
                    ],
                },
            ],
        },
        "vesimir" => Dialogue {
            current_node: None,
            nodes: vec![
                DialogueNode {
                    id: "start".into(), speaker: "Stary Wiedźmin Vesimir".into(),
                    text: "Gerard... z Rumii? Ze Szkoły Dzika? Dawno nie słyszałem tej nazwy.\nSzkoła Dzika zaginęła wieki temu. Ale zachowały się zapiski...".into(),
                    choices: vec![
                        DialogueChoice { text: "Opowiedz mi o Szkole Dzika.".into(), next_node: Some("school_info".into()), action: DialogueAction::None },
                        DialogueChoice { text: "Potrzebuję pomocy w walce z potworami.".into(), next_node: Some("help".into()), action: DialogueAction::None },
                        DialogueChoice { text: "Do widzenia, Vesimir.".into(), next_node: None, action: DialogueAction::EndDialogue },
                    ],
                },
                DialogueNode {
                    id: "school_info".into(), speaker: "Stary Wiedźmin Vesimir".into(),
                    text: "Szkoła Dzika była znana z niekonwencjonalnych metod. Łączyła techniki wielu szkół.\nIch siedziba była gdzieś na północy. Ostatni mistrz zostawił legendarne ostrze...\nAle to wiedza, którą musisz zdobyć sam, wiedźminie. Zasłużyłeś na nią.".into(),
                    choices: vec![
                        DialogueChoice { text: "Dziękuję za wiedzę, Vesimir. (Quest główny ukończony!)".into(), next_node: None, action: DialogueAction::GiveExp(500) },
                    ],
                },
                DialogueNode {
                    id: "help".into(), speaker: "Stary Wiedźmin Vesimir".into(),
                    text: "Odpocznij tu, wiedźminie. Wyleczę twoje rany i naostrzę miecze.\nPamiętaj - każdy potwór ma słabość. Oleje, znaki, bomby - to twoje narzędzia.".into(),
                    choices: vec![
                        DialogueChoice { text: "Dziękuję. (Pełne leczenie)".into(), next_node: None, action: DialogueAction::Heal },
                    ],
                },
            ],
        },
        "elf" => Dialogue {
            current_node: None,
            nodes: vec![
                DialogueNode {
                    id: "start".into(), speaker: "Tajemniczy Elf".into(),
                    text: "Dh'oine... wiedźmin. Rzadki gość w tych lasach.\nWiem, czego szukasz. Szkoła Dzika... tak, pamiętam ją z dawnych czasów.".into(),
                    choices: vec![
                        DialogueChoice { text: "Co wiesz o Szkole Dzika?".into(), next_node: Some("info".into()), action: DialogueAction::None },
                        DialogueChoice { text: "Leszy grasuje w lesie. Wiesz coś o nim?".into(), next_node: Some("leshen".into()), action: DialogueAction::StartQuest("contract_leshen".into()) },
                        DialogueChoice { text: "Do widzenia.".into(), next_node: None, action: DialogueAction::EndDialogue },
                    ],
                },
                DialogueNode {
                    id: "info".into(), speaker: "Tajemniczy Elf".into(),
                    text: "Szkoła Dzika była bliska elfom. Uczyli się od nas magii natury.\nMusisz iść do zamku na wschodzie. Vesimir posiada klucz do ich tajemnic.\nAle uważaj - w jaskini czai się wilkołak. Pokonaj go, a udowodnisz swoją wartość.".into(),
                    choices: vec![
                        DialogueChoice { text: "Dziękuję za pomoc.".into(), next_node: None, action: DialogueAction::GiveExp(50) },
                    ],
                },
                DialogueNode {
                    id: "leshen".into(), speaker: "Tajemniczy Elf".into(),
                    text: "Leszy... starożytna istota. Strzeże tego lasu od wieków.\nJest potężny, ale wrażliwy na ogień. Igni i olej na relikty - to twoja szansa.\nUważaj na jego korzenie i wilki, które przywołuje.".into(),
                    choices: vec![
                        DialogueChoice { text: "Przyjmę wyzwanie.".into(), next_node: None, action: DialogueAction::EndDialogue },
                    ],
                },
            ],
        },
        _ => Dialogue {
            current_node: None,
            nodes: vec![
                DialogueNode {
                    id: "start".into(), speaker: "Nieznajomy".into(),
                    text: "Nie mam nic do powiedzenia.".into(),
                    choices: vec![
                        DialogueChoice { text: "Do widzenia.".into(), next_node: None, action: DialogueAction::EndDialogue },
                    ],
                },
            ],
        },
    }
}
