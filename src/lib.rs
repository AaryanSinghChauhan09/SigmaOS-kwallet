// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-kwallet: kwallet component model.
//!
//! Real implementation: kwallet_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KwalletItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KwalletError {
    EmptyKwalletItemName,
    DuplicateKwalletItem(String),
    EmptyEventSubject,
}

impl fmt::Display for KwalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyKwalletItemName => write!(f, "kwallet_item name must not be empty"),
            Self::DuplicateKwalletItem(id) => write!(f, "duplicate kwallet_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for KwalletError {}

pub struct Kwallet {
    pub name: String,
    kwallet_items: BTreeMap<String, KwalletItem>,
    events: BTreeMap<String, Event>,
}

impl Kwallet {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kwallet_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_kwallet_item(&mut self, item: KwalletItem) -> Result<(), KwalletError> {
        if item.name.trim().is_empty() {
            return Err(KwalletError::EmptyKwalletItemName);
        }
        let id = item.id.clone();
        if self.kwallet_items.contains_key(&id) {
            return Err(KwalletError::DuplicateKwalletItem(id.clone()));
        }
        self.kwallet_items.insert(id, item);
        Ok(())
    }

    pub fn kwallet_item_count(&self) -> usize {
        self.kwallet_items.len()
    }

    pub fn get_kwallet_item(&self, id: &str) -> Option<&KwalletItem> {
        self.kwallet_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), KwalletError> {
        if event.subject.trim().is_empty() {
            return Err(KwalletError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_kwallet_item(id: &str, name: &str) -> KwalletItem {
        KwalletItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Kwallet::new("Kwallet");
        assert_eq!(m.name, "Kwallet");
        assert_eq!(m.kwallet_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_kwallet_item() {
        let mut m = Kwallet::new("Kwallet");
        m.add_kwallet_item(sample_kwallet_item("a1", "Alpha")).unwrap();
        assert_eq!(m.kwallet_item_count(), 1);
        assert!(m.get_kwallet_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Kwallet::new("Kwallet");
        assert!(m.add_kwallet_item(sample_kwallet_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Kwallet::new("Kwallet");
        m.add_kwallet_item(sample_kwallet_item("x", "One")).unwrap();
        assert!(m.add_kwallet_item(sample_kwallet_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Kwallet::new("Kwallet");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
