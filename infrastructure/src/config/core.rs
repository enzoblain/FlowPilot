use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::{AddUserResult, RemoveUserResult};
use crate::telegram::ChatId;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    telegram_token: String,
    allowed_users: HashMap<ChatId, String>,
}

impl Config {
    pub fn telegram_token(&self) -> &str {
        &self.telegram_token
    }

    pub fn allowed_chat_ids(&self) -> Vec<ChatId> {
        self.allowed_users.keys().cloned().collect()
    }

    pub fn allowed_users(&self) -> &HashMap<ChatId, String> {
        &self.allowed_users
    }

    pub fn set_telegram_token(&mut self, token: impl Into<String>) {
        self.telegram_token = token.into();
    }

    pub fn add_allowed_chat_id(&mut self, name: &str, id: ChatId) -> AddUserResult {
        if self.is_allowed_name(name) {
            return AddUserResult::NameAlreadyExists;
        }

        if self.is_allowed_chat_id(id) {
            return AddUserResult::ChatIdAlreadyExists;
        }

        self.allowed_users.insert(id, name.to_owned());

        AddUserResult::Added
    }

    pub fn remove_allowed_name(&mut self, name: &str) -> RemoveUserResult {
        let chat_id = self
            .allowed_users
            .iter()
            .find(|(_, value)| value.as_str() == name)
            .map(|(key, _)| *key);

        match chat_id {
            Some(chat_id) => {
                self.allowed_users.remove(&chat_id);
                RemoveUserResult::Removed
            }
            None => RemoveUserResult::NotFound,
        }
    }

    pub fn remove_allowed_chat_id(&mut self, id: ChatId) -> RemoveUserResult {
        if self.allowed_users.remove(&id).is_some() {
            RemoveUserResult::Removed
        } else {
            RemoveUserResult::NotFound
        }
    }

    pub fn is_allowed_chat_id(&self, id: ChatId) -> bool {
        self.allowed_users.contains_key(&id)
    }

    pub fn is_allowed_name(&self, name: &str) -> bool {
        self.allowed_users.values().any(|value| value == name)
    }
}
