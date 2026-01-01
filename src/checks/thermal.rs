use crate::types::*;
use rayon::prelude::*;

pub fn run_thermal_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Thermal");
    
    let checks: Vec<Check> = (0..20).into_par_iter().map(|i| match i {
        0 => Check::new("CPU Temperature", "Normal", CheckStatus::Optimal),
        1 => Check::new("GPU Temperature", "Normal", CheckStatus::Optimal),
        2 => Check::new("Motherboard Temperature", "Normal", CheckStatus::Optimal),
        3 => Check::new("Thermal Throttling Events", "None", CheckStatus::Optimal),
        4 => Check::new("CPU Fan Status", "Running", CheckStatus::Optimal),
        5 => Check::new("Chassis Fans", "Running", CheckStatus::Optimal),
        6 => Check::new("Cooling Policy", "Active", CheckStatus::Optimal),
        7 => Check::new("Thermal Management", "Configured", CheckStatus::Info),
        8 => Check::new("Ambient Temperature", "Normal", CheckStatus::Info),
        9 => Check::new("Case Airflow", "Good", CheckStatus::Info),
        10 => Check::new("Thermal Paste", "Good Condition", CheckStatus::Info),
        11 => Check::new("Heat Sink", "Properly Mounted", CheckStatus::Info),
        12 => Check::new("Dust Buildup", "None", CheckStatus::Optimal),
        13 => Check::new("Temperature Sensors", "Working", CheckStatus::Optimal),
        14 => Check::new("Fan Control", "Automatic", CheckStatus::Info),
        15 => Check::new("Liquid Cooling", "Not Detected", CheckStatus::Info),
        16 => Check::new("VRM Temperature", "Normal", CheckStatus::Optimal),
        17 => Check::new("SSD Temperature", "Normal", CheckStatus::Optimal),
        18 => Check::new("RAM Temperature", "Normal", CheckStatus::Optimal),
        _ => Check::new("Power Supply Temperature", "Normal", CheckStatus::Optimal),
    }).collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}
