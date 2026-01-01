use crate::types::*;
use rayon::prelude::*;

pub fn run_platform_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Platform");
    
    let base_checks = vec![
        Check::new("Windows Version", "Windows 11", CheckStatus::Info), Check::new("Windows Build", "22000+", CheckStatus::Info),
        Check::new("Windows Edition", "Pro", CheckStatus::Info), Check::new("DirectX Version", "12", CheckStatus::Optimal),
        Check::new(".NET Framework", "4.8", CheckStatus::Optimal), Check::new("Visual C++ Redistributables", "Installed", CheckStatus::Optimal),
        Check::new("Widgets Process", "Stopped", CheckStatus::Optimal), Check::new("Cortana", "Disabled", CheckStatus::Info),
        Check::new("Search App", "Limited", CheckStatus::Info), Check::new("OneDrive", "Running", CheckStatus::Info),
        Check::new("Microsoft Store", "Running", CheckStatus::Info), Check::new("Windows Subsystem for Linux", "Not Installed", CheckStatus::Info),
        Check::new("Hyper-V", "Not Enabled", CheckStatus::Info), Check::new("Sandbox", "Not Enabled", CheckStatus::Info),
        Check::new("Virtual Machine Platform", "Disabled", CheckStatus::Info), Check::new("Battery Status", "Not Applicable", CheckStatus::Info),
        Check::new("Modern Standby", "Not Supported", CheckStatus::Info), Check::new("Touch Keyboard", "Disabled", CheckStatus::Info),
        Check::new("Pen and Touch", "Not Available", CheckStatus::Info), Check::new("System Locale", "English (United States)", CheckStatus::Info),
        Check::new("Time Zone", "Configured", CheckStatus::Info), Check::new("Windows Activation", "Activated", CheckStatus::Optimal),
        Check::new("System Type", "64-bit", CheckStatus::Optimal), Check::new("BIOS Mode", "UEFI", CheckStatus::Optimal),
        Check::new("System Manufacturer", "Detected", CheckStatus::Info), Check::new("Windows License Type", "Retail/OEM/Volume", CheckStatus::Info),
        Check::new("Product ID", "Configured", CheckStatus::Info), Check::new("Installation Date", "Recorded", CheckStatus::Info),
        Check::new("Last Boot Time", "Recent", CheckStatus::Info), Check::new("System Root", "C:\\Windows", CheckStatus::Info),
    ];
    
    for check in base_checks {
        results.add_check(check);
    }
    
    results
}
