//! SigmaOS-archlinux-vlc: VLC media config model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for VLC media config.
pub struct VlcPlaylist {
    pub id: String,
    pub name: String,
    pub url: String,
    pub enabled: bool,
}

/// Secondary domain entity for VLC media config.
pub struct VlcTrack {
    pub id: String,
    pub codec: String,
    pub description: String,
}

/// Errors for the VLC media config domain.
#[derive(Debug, PartialEq)]
pub enum VlcArchError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for VlcArchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for VlcArchError {}

/// Registry and manager for VLC media config.
pub struct VlcArchManager {
    pub name: String,
    items: BTreeMap<String, VlcPlaylist>,
    configs: BTreeMap<String, VlcTrack>,
}

impl VlcArchManager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: VlcPlaylist) -> Result<(), VlcArchError> {
        if item.name.is_empty() {
            return Err(VlcArchError::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(VlcArchError::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&VlcPlaylist> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: VlcTrack) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), VlcArchError> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| VlcArchError::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_VlcPlaylist(id: &str, name: &str) -> VlcPlaylist {
        VlcPlaylist {
            id: id.to_string(),
            name: name.to_string(),
            url: "archlinux-vlc-default".to_string(),
            enabled: true,
        }
    }

    fn make_VlcTrack(id: &str, val: &str) -> VlcTrack {
        VlcTrack {
            id: id.to_string(),
            codec: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = VlcArchManager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = VlcArchManager::new("test");
        let item = make_VlcPlaylist("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = VlcArchManager::new("test");
        let item = make_VlcPlaylist("item1", "");
        assert_eq!(mgr.add_item(item), Err(VlcArchError::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = VlcArchManager::new("test");
        let item1 = make_VlcPlaylist("item1", "First");
        let item2 = make_VlcPlaylist("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(VlcArchError::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = VlcArchManager::new("test");
        let cfg = make_VlcTrack("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = VlcArchManager::new("test");
        let item = make_VlcPlaylist("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = VlcArchManager::new("test");
        let item = make_VlcPlaylist("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(VlcArchError::NotFound("none".to_string())));
    }
}
