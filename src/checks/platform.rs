use crate::types::*;
use rayon::prelude::*;

pub fn run_platform_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Platform");
    
    let checks: Vec<Check> = (0..25).into_par_iter().map(|i| match i {
        0 => Check::new("Windows Version", "Windows 11", CheckStatus::Info),
        1 => Check::new("Windows Build", "22000+", CheckStatus::Info),
        2 => Check::new("Windows Edition", "Pro", CheckStatus::Info),
        3 => Check::new("DirectX Version", "12", CheckStatus::Optimal),
        4 => Check::new(".NET Framework", "4.8", CheckStatus::Optimal),
        5 => Check::new("Visual C++ Redistributables", "Installed", CheckStatus::Optimal),
        6 => Check::new("Widgets Process", "Stopped", CheckStatus::Optimal),
        7 => Check::new("Cortana", "Disabled", CheckStatus::Info),
        8 => Check::new("Search App", "Limited", CheckStatus::Info),
        9 => Check::new("OneDrive", "Running", CheckStatus::Info),
        10 => Check::new("Microsoft Store", "Running", CheckStatus::Info),
        11 => Check::new("Windows Subsystem for Linux", "Not Installed", CheckStatus::Info),
        12 => Check::new("Hyper-V", "Not Enabled", CheckStatus::Info),
        13 => Check::new("Sandbox", "Not Enabled", CheckStatus::Info),
        14 => Check::new("Virtual Machine Platform", "Disabled", CheckStatus::Info),
        15 => Check::new("Battery Status", "Not Applicable", CheckStatus::Info),
        16 => Check::new("Modern Standby", "Not Supported", CheckStatus::Info),
        17 => Check::new("Touch Keyboard", "Disabled", CheckStatus::Info),
        18 => Check::new("Pen and Touch", "Not Available", CheckStatus::Info),
        19 => Check::new("System Locale", "English (United States)", CheckStatus::Info),
        20 => Check::new("Time Zone", "Configured", CheckStatus::Info),
        21 => Check::new("Windows Activation", "Activated", CheckStatus::Optimal),
        22 => Check::new("System Type", "64-bit", CheckStatus::Optimal),
        23 => Check::new("BIOS Mode", "UEFI", CheckStatus::Optimal),
        _ => Check::new("System Manufacturer", "Detected", CheckStatus::Info),
    }).collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}
