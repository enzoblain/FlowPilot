use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::{AddUserResult, RemoveUserResult};
use crate::telegram::ChatId;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    telegram_token: String,
    allowed_chat_ids: HashMap<String, ChatId>,
}

impl Config {
    pub fn telegram_token(&self) -> &str {
        &self.telegram_token
    }

    pub fn allowed_chat_ids(&self) -> Vec<ChatId> {
        self.allowed_chat_ids.values().cloned().collect()
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

        self.allowed_chat_ids.insert(name.to_owned(), id);

        AddUserResult::Added
    }

    pub fn remove_allowed_name(&mut self, name: &str) -> RemoveUserResult {
        if self.allowed_chat_ids.remove(name).is_some() {
            RemoveUserResult::Removed
        } else {
            RemoveUserResult::NotFound
        }
    }

    pub fn remove_allowed_chat_id(&mut self, id: ChatId) -> RemoveUserResult {
        let key = self
            .allowed_chat_ids
            .iter()
            .find(|(_, value)| **value == id)
            .map(|(key, _)| key.clone());

        match key {
            Some(key) => {
                self.allowed_chat_ids.remove(&key);
                RemoveUserResult::Removed
            }
            None => RemoveUserResult::NotFound,
        }
    }

    pub fn is_allowed_chat_id(&self, id: ChatId) -> bool {
        self.allowed_chat_ids().contains(&id)
    }

    pub fn is_allowed_name(&self, name: &str) -> bool {
        self.allowed_chat_ids.contains_key(name)
    }
}
