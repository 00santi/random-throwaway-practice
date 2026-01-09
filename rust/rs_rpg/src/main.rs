struct Player {
    inventory: Vec<Item>,
    name: String,
    hp: u8,
}

enum Item {
    HealthPotion(u8),
    Sword(u8),
}

impl Player {
    fn create() -> Player {
        let name = String::from("Ashen One");
        let inventory = vec![Item::Sword(5), Item::HealthPotion(25), Item::HealthPotion(25)];
        let hp = 100;
        Player {
            name, inventory, hp
        }
    }

    fn create_with_name(name: &str) -> Player {
        let name = String::from(name);
        let inventory = vec![Item::Sword(5), Item::HealthPotion(25), Item::HealthPotion(25)];
        let hp = 100;
        Player {
            name, inventory, hp
        }
    }

    fn info_dump(&self) {
        println!("\nCurrent character: {}", self.name);
        println!("{} has {} HP", self.name, self.hp);
        println!("{} is holding {} items: ", self.name, self.inventory.len());
        for item in &self.inventory {
            println!("{}", item.name());
        }
    }

    fn take_damage(&mut self, damage: u8) {
        if self.hp > damage {
            self.hp -= damage;
        } else {
            self.hp = 0;
        }
    }

    fn use_healing_potion(&mut self) {
        let mut healing: u8 = 0;
        let mut idx_to_remove: Option<usize> = None;
        for (index, item) in self.inventory.iter().enumerate() {
            match item {
                Item::HealthPotion(h) => {
                    idx_to_remove = Some(index);
                    healing = *h;
                }
                _ => continue
            }
        };
        if let Some(i) = idx_to_remove {
            self.inventory.remove(i);
        }
        self.recover_hp(healing);
    }

    fn recover_hp(&mut self, healing: u8) {
        self.hp = self.hp.saturating_add(healing).min(100);
    }
}

impl Item {
    fn name(&self) -> String {
        match self {
            Item::Sword(d) => format!("Sword ({d} dmg)"),
            Item::HealthPotion(h) => format!("Potion ({h} hp)")
        }
    }
}

fn main() {
    println!("~~~~~Inventory System~~~~~");
    let mut player = Player::create();
    player.info_dump();
    player.take_damage(10);
    player.info_dump();
}
