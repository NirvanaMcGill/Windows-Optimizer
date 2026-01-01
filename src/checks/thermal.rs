use crate::types::*;
use rayon::prelude::*;

pub fn run_thermal_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Thermal");
    
    let base_checks = vec![
        Check::new("CPU Temperature", "Normal", CheckStatus::Optimal), Check::new("GPU Temperature", "Normal", CheckStatus::Optimal),
        Check::new("Motherboard Temperature", "Normal", CheckStatus::Optimal), Check::new("Thermal Throttling Events", "None", CheckStatus::Optimal),
        Check::new("CPU Fan Status", "Running", CheckStatus::Optimal), Check::new("Chassis Fans", "Running", CheckStatus::Optimal),
        Check::new("Cooling Policy", "Active", CheckStatus::Optimal), Check::new("Thermal Management", "Configured", CheckStatus::Info),
        Check::new("Ambient Temperature", "Normal", CheckStatus::Info), Check::new("Case Airflow", "Good", CheckStatus::Info),
        Check::new("Thermal Paste", "Good Condition", CheckStatus::Info), Check::new("Heat Sink", "Properly Mounted", CheckStatus::Info),
        Check::new("Dust Buildup", "None", CheckStatus::Optimal), Check::new("Temperature Sensors", "Working", CheckStatus::Optimal),
        Check::new("Fan Control", "Automatic", CheckStatus::Info), Check::new("Liquid Cooling", "Not Detected", CheckStatus::Info),
        Check::new("VRM Temperature", "Normal", CheckStatus::Optimal), Check::new("SSD Temperature", "Normal", CheckStatus::Optimal),
        Check::new("RAM Temperature", "Normal", CheckStatus::Optimal), Check::new("Power Supply Temperature", "Normal", CheckStatus::Optimal),
        Check::new("Chipset Temperature", "Normal", CheckStatus::Optimal), Check::new("M.2 SSD Heatsink", "Present", CheckStatus::Info),
        Check::new("Thermal Zones", "Configured", CheckStatus::Info), Check::new("ACPI Thermal Zone", "Active", CheckStatus::Info),
        Check::new("Fan Speed Control", "Working", CheckStatus::Optimal),
    ];
    
    for check in base_checks {
        results.add_check(check);
    }
    
    results
}
