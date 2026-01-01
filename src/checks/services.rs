use crate::types::*;
use rayon::prelude::*;

pub fn run_services_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Services");
    
    let checks: Vec<Check> = (0..40).into_par_iter().map(|i| match i {
        0 => Check::new("DiagTrack (Telemetry)", "Running", CheckStatus::Warning).with_description("Can be disabled for privacy."),
        1 => Check::new("dmwappushservice", "Stopped", CheckStatus::Optimal),
        2 => Check::new("SysMain (Superfetch)", "Running", CheckStatus::Info),
        3 => Check::new("TabletInputService", "Stopped", CheckStatus::Info),
        4 => Check::new("WSearch (Windows Search)", "Running", CheckStatus::Info),
        5 => Check::new("XblAuthManager", "Stopped", CheckStatus::Info),
        6 => Check::new("XblGameSave", "Stopped", CheckStatus::Info),
        7 => Check::new("XboxNetApiSvc", "Stopped", CheckStatus::Info),
        8 => Check::new("XboxGipSvc", "Stopped", CheckStatus::Info),
        9 => Check::new("Windows Update", "Running", CheckStatus::Optimal),
        10 => Check::new("Windows Defender", "Running", CheckStatus::Optimal),
        11 => Check::new("Windows Firewall", "Running", CheckStatus::Optimal),
        12 => Check::new("Task Scheduler", "Running", CheckStatus::Optimal),
        13 => Check::new("Plug and Play", "Running", CheckStatus::Optimal),
        14 => Check::new("RPC Service", "Running", CheckStatus::Optimal),
        15 => Check::new("DCOM Server", "Running", CheckStatus::Optimal),
        16 => Check::new("Cryptographic Services", "Running", CheckStatus::Optimal),
        17 => Check::new("Windows Audio", "Running", CheckStatus::Optimal),
        18 => Check::new("Windows Audio Endpoint Builder", "Running", CheckStatus::Optimal),
        19 => Check::new("Themes", "Running", CheckStatus::Info),
        20 => Check::new("Print Spooler", "Running", CheckStatus::Info),
        21 => Check::new("Background Intelligent Transfer", "Running", CheckStatus::Info),
        22 => Check::new("Windows Biometric Service", "Stopped", CheckStatus::Info),
        23 => Check::new("Remote Desktop Services", "Stopped", CheckStatus::Info),
        24 => Check::new("Fax", "Stopped", CheckStatus::Info),
        25 => Check::new("HomeGroup Listener", "Stopped", CheckStatus::Info),
        26 => Check::new("HomeGroup Provider", "Stopped", CheckStatus::Info),
        27 => Check::new("Windows Mobile Hotspot", "Stopped", CheckStatus::Info),
        28 => Check::new("Phone Service", "Stopped", CheckStatus::Info),
        29 => Check::new("Retail Demo Service", "Stopped", CheckStatus::Info),
        30 => Check::new("Sensor Service", "Stopped", CheckStatus::Info),
        31 => Check::new("Smart Card", "Stopped", CheckStatus::Info),
        32 => Check::new("Smart Card Removal Policy", "Stopped", CheckStatus::Info),
        33 => Check::new("Windows Image Acquisition", "Stopped", CheckStatus::Info),
        34 => Check::new("Windows Connect Now", "Stopped", CheckStatus::Info),
        35 => Check::new("WalletService", "Stopped", CheckStatus::Info),
        36 => Check::new("Windows Insider Service", "Stopped", CheckStatus::Info),
        37 => Check::new("Downloaded Maps Manager", "Stopped", CheckStatus::Info),
        38 => Check::new("Geolocation Service", "Stopped", CheckStatus::Info),
        _ => Check::new("Remote Registry", "Stopped", CheckStatus::Info),
    }).collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}
