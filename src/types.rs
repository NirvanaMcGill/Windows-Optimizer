use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CheckStatus {
    Optimal,
    Warning,
    Issue,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Check {
    pub name: String,
    pub value: String,
    pub status: CheckStatus,
    pub description: String,
}

impl Check {
    pub fn new(name: &str, value: &str, status: CheckStatus) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            status,
            description: String::new(),
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResults {
    pub name: String,
    pub checks: Vec<Check>,
}

impl CategoryResults {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            checks: Vec::new(),
        }
    }

    pub fn add_check(&mut self, check: Check) {
        self.checks.push(check);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditResults {
    pub categories: HashMap<String, CategoryResults>,
    pub timestamp: String,
}

impl AuditResults {
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
            timestamp: chrono::Local::now().to_rfc3339(),
        }
    }

    pub fn add_category(&mut self, category: CategoryResults) {
        self.categories.insert(category.name.clone(), category);
    }

    pub fn total_checks(&self) -> usize {
        self.categories.values().map(|c| c.checks.len()).sum()
    }

    pub fn count_status(&self, status: CheckStatus) -> usize {
        self.categories.values()
            .flat_map(|c| &c.checks)
            .filter(|check| check.status == status)
            .count()
    }
}
