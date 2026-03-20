/// System questów i fabuły

#[derive(Debug, Clone, PartialEq)]
pub enum QuestStatus {
    NotStarted,
    Active,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuestObjective {
    KillMonster(String),      // nazwa potwora
    TalkToNpc(String),        // id NPC
    CollectItem(String, i32), // nazwa przedmiotu, ilość
    GoToLocation(String),     // nazwa lokacji
}

#[derive(Debug, Clone)]
pub struct QuestStep {
    pub description: String,
    pub objective: QuestObjective,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: QuestStatus,
    pub steps: Vec<QuestStep>,
    pub current_step: usize,
    pub experience_reward: i32,
    pub gold_reward: i32,
    pub is_main_quest: bool,
}

impl Quest {
    pub fn current_objective(&self) -> Option<&QuestStep> {
        if self.current_step < self.steps.len() {
            Some(&self.steps[self.current_step])
        } else {
            None
        }
    }

    pub fn advance_step(&mut self) -> bool {
        if self.current_step < self.steps.len() {
            self.steps[self.current_step].completed = true;
            self.current_step += 1;
            if self.current_step >= self.steps.len() {
                self.status = QuestStatus::Completed;
                return true; // quest ukończony
            }
        }
        false
    }
}

pub struct QuestLog {
    pub quests: Vec<Quest>,
    pub active_quest_index: Option<usize>,
}

impl QuestLog {
    pub fn new() -> Self {
        let quests = vec![
            // Quest główny
            Quest {
                id: "main_school".into(),
                title: "Tajemnica Szkoły Dzika".into(),
                description: "Odkryj tajemnicę zaginionej wiedzy Szkoły Dzika. Vesimir w zamku może wiedzieć więcej.".into(),
                status: QuestStatus::NotStarted,
                steps: vec![
                    QuestStep {
                        description: "Porozmawiaj z Sołtysem Bogdanem w Wiosce Rumia".into(),
                        objective: QuestObjective::TalkToNpc("soltys".into()),
                        completed: false,
                    },
                    QuestStep {
                        description: "Zabij ghule w Mrocznym Lesie".into(),
                        objective: QuestObjective::KillMonster("Ghul".into()),
                        completed: false,
                    },
                    QuestStep {
                        description: "Porozmawiaj z Tajemniczym Elfem w lesie".into(),
                        objective: QuestObjective::TalkToNpc("elf".into()),
                        completed: false,
                    },
                    QuestStep {
                        description: "Pokonaj Wilkołaka w Jaskini".into(),
                        objective: QuestObjective::KillMonster("Wilkołak".into()),
                        completed: false,
                    },
                    QuestStep {
                        description: "Odwiedź Vesimira w Zamku".into(),
                        objective: QuestObjective::TalkToNpc("vesimir".into()),
                        completed: false,
                    },
                ],
                current_step: 0,
                experience_reward: 500,
                gold_reward: 200,
                is_main_quest: true,
            },
            // Kontrakt: Utopce na bagnach
            Quest {
                id: "contract_drowners".into(),
                title: "Kontrakt: Utopce na Bagnach".into(),
                description: "Sołtys prosi o oczyszczenie bagien z utopców nękających wieśniaków.".into(),
                status: QuestStatus::NotStarted,
                steps: vec![
                    QuestStep {
                        description: "Porozmawiaj z Sołtysem o kontrakcie".into(),
                        objective: QuestObjective::TalkToNpc("soltys".into()),
                        completed: false,
                    },
                    QuestStep {
                        description: "Zabij utopce na Bagnach".into(),
                        objective: QuestObjective::KillMonster("Utopiec".into()),
                        completed: false,
                    },
                    QuestStep {
                        description: "Wróć do Sołtysa po nagrodę".into(),
                        objective: QuestObjective::TalkToNpc("soltys".into()),
                        completed: false,
                    },
                ],
                current_step: 0,
                experience_reward: 150,
                gold_reward: 100,
                is_main_quest: false,
            },
            // Kontrakt: Endriagi
            Quest {
                id: "contract_endriaga".into(),
                title: "Kontrakt: Endriagi w Lesie".into(),
                description: "Endriagi zaatakowały drwali w lesie. Trzeba je wytępić.".into(),
                status: QuestStatus::NotStarted,
                steps: vec![
                    QuestStep {
                        description: "Porozmawiaj z Zielarką Bożeną".into(),
                        objective: QuestObjective::TalkToNpc("zielarka".into()),
                        completed: false,
                    },
                    QuestStep {
                        description: "Zabij endriagi w Mrocznym Lesie".into(),
                        objective: QuestObjective::KillMonster("Endriaga".into()),
                        completed: false,
                    },
                    QuestStep {
                        description: "Wróć do Zielarki po nagrodę".into(),
                        objective: QuestObjective::TalkToNpc("zielarka".into()),
                        completed: false,
                    },
                ],
                current_step: 0,
                experience_reward: 200,
                gold_reward: 120,
                is_main_quest: false,
            },
            // Kontrakt: Bazyliszek
            Quest {
                id: "contract_basilisk".into(),
                title: "Kontrakt: Bazyliszek w Jaskini".into(),
                description: "W jaskini zagnieździł się potężny bazyliszek. To zlecenie nie dla żółtodziobów.".into(),
                status: QuestStatus::NotStarted,
                steps: vec![
                    QuestStep {
                        description: "Znajdź i zabij Bazyliszka w Jaskini".into(),
                        objective: QuestObjective::KillMonster("Bazyliszek".into()),
                        completed: false,
                    },
                ],
                current_step: 0,
                experience_reward: 300,
                gold_reward: 200,
                is_main_quest: false,
            },
            // Kontrakt: Leszy
            Quest {
                id: "contract_leshen".into(),
                title: "Kontrakt: Leszy - Strażnik Lasu".into(),
                description: "Starożytny Leszy terroryzuje okolice. Tylko doświadczony wiedźmin da radę.".into(),
                status: QuestStatus::NotStarted,
                steps: vec![
                    QuestStep {
                        description: "Porozmawiaj z Elfem o Leszym".into(),
                        objective: QuestObjective::TalkToNpc("elf".into()),
                        completed: false,
                    },
                    QuestStep {
                        description: "Znajdź i pokonaj Leszego".into(),
                        objective: QuestObjective::KillMonster("Leszy".into()),
                        completed: false,
                    },
                ],
                current_step: 0,
                experience_reward: 400,
                gold_reward: 250,
                is_main_quest: false,
            },
        ];

        QuestLog {
            quests,
            active_quest_index: None,
        }
    }

    pub fn start_quest(&mut self, quest_id: &str) {
        for (i, q) in self.quests.iter_mut().enumerate() {
            if q.id == quest_id && q.status == QuestStatus::NotStarted {
                q.status = QuestStatus::Active;
                if self.active_quest_index.is_none() {
                    self.active_quest_index = Some(i);
                }
            }
        }
    }

    pub fn active_quests(&self) -> Vec<&Quest> {
        self.quests.iter().filter(|q| q.status == QuestStatus::Active).collect()
    }

    pub fn completed_quests(&self) -> Vec<&Quest> {
        self.quests.iter().filter(|q| q.status == QuestStatus::Completed).collect()
    }

    pub fn check_kill_objective(&mut self, monster_name: &str) {
        for quest in &mut self.quests {
            if quest.status != QuestStatus::Active { continue; }
            if let Some(step) = quest.steps.get(quest.current_step) {
                if let QuestObjective::KillMonster(ref name) = step.objective {
                    if name == monster_name {
                        quest.advance_step();
                    }
                }
            }
        }
    }

    pub fn check_talk_objective(&mut self, npc_id: &str) {
        for quest in &mut self.quests {
            if quest.status != QuestStatus::Active { continue; }
            if let Some(step) = quest.steps.get(quest.current_step) {
                if let QuestObjective::TalkToNpc(ref id) = step.objective {
                    if id == npc_id {
                        quest.advance_step();
                    }
                }
            }
        }
    }
}
