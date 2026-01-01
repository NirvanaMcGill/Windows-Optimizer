use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Category {
    Latency,
    Cpu,
    Gpu,
    Memory,
    Storage,
    Network,
    Audio,
    Input,
    Stability,
    Services,
    Security,
    Platform,
    Thermal,
    Power,
}

impl Category {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Latency => "latency",
            Category::Cpu => "cpu",
            Category::Gpu => "gpu",
            Category::Memory => "memory",
            Category::Storage => "storage",
            Category::Network => "network",
            Category::Audio => "audio",
            Category::Input => "input",
            Category::Stability => "stability",
            Category::Services => "services",
            Category::Security => "security",
            Category::Platform => "platform",
            Category::Thermal => "thermal",
            Category::Power => "power",
        }
    }

    #[allow(dead_code)]
    pub fn display_name(&self) -> &'static str {
        match self {
            Category::Latency => "Latency",
            Category::Cpu => "CPU",
            Category::Gpu => "GPU",
            Category::Memory => "Memory",
            Category::Storage => "Storage",
            Category::Network => "Network",
            Category::Audio => "Audio",
            Category::Input => "Input",
            Category::Stability => "Stability",
            Category::Services => "Services",
            Category::Security => "Security",
            Category::Platform => "Platform",
            Category::Thermal => "Thermal",
            Category::Power => "Power",
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[allow(dead_code)]
pub enum CheckError {
    #[error("Registry: {0}")]
    Registry(String),
    #[error("WMI: {0}")]
    Wmi(String),
    #[error("Privilege: {0}")]
    Privilege(String),
    #[error("Timeout")]
    Timeout,
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),
}

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
    pub expected: Option<String>,
    pub status: CheckStatus,
    pub description: String,
    pub severity: u8,
    pub fix_cmd: Option<String>,
}

impl Check {
    pub fn new(name: &str, value: &str, status: CheckStatus) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            expected: None,
            status,
            description: String::new(),
            severity: 5,
            fix_cmd: None,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    #[allow(dead_code)]
    pub fn with_expected(mut self, expected: &str) -> Self {
        self.expected = Some(expected.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_severity(mut self, severity: u8) -> Self {
        self.severity = severity.min(10);
        self
    }

    #[allow(dead_code)]
    pub fn with_fix_cmd(mut self, cmd: &str) -> Self {
        self.fix_cmd = Some(cmd.to_string());
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

impl Default for AuditResults {
    fn default() -> Self {
        Self::new()
    }
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
        self.categories
            .values()
            .flat_map(|c| &c.checks)
            .filter(|check| check.status == status)
            .count()
    }
}
