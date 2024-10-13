use std::collections::HashMap;

pub struct AuthManager {
    users: HashMap<String, String>, // 存儲用戶名和密碼（示例中為明文，實際應加密）
}

impl AuthManager {
    pub fn new() -> AuthManager {
        AuthManager {
            users: HashMap::new(),
        }
    }

    pub fn register(&mut self, username: &str, password: &str) -> bool {
        if self.users.contains_key(username) {
            return false; // 用戶已存在
        }
        self.users.insert(username.to_string(), password.to_string());
        true
    }

    pub fn login(&self, username: &str, password: &str) -> bool {
        match self.users.get(username) {
            Some(stored_password) => stored_password == password,
            None => false,
        }
    }
}
