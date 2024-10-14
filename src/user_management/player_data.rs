use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub level: u32,
    pub experience: u32,
}

impl User {
    pub fn new(username: &str) -> User {
        User {
            username: username.to_string(),
            level: 1,
            experience: 0,
        }
    }

    pub fn gain_experience(&mut self, amount: u32) {
        self.experience += amount;
        if self.experience >= 100 {
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.experience = 0;
    }
}
