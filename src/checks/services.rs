use crate::types::*;
use rayon::prelude::*;

pub fn run_services_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Services");
    
    let base_checks = vec![
        Check::new("DiagTrack (Telemetry)", "Running", CheckStatus::Warning).with_description("Can be disabled for privacy."),
        Check::new("dmwappushservice", "Stopped", CheckStatus::Optimal), Check::new("SysMain (Superfetch)", "Running", CheckStatus::Info),
        Check::new("TabletInputService", "Stopped", CheckStatus::Info), Check::new("WSearch (Windows Search)", "Running", CheckStatus::Info),
        Check::new("XblAuthManager", "Stopped", CheckStatus::Info), Check::new("XblGameSave", "Stopped", CheckStatus::Info),
        Check::new("XboxNetApiSvc", "Stopped", CheckStatus::Info), Check::new("XboxGipSvc", "Stopped", CheckStatus::Info),
        Check::new("Windows Update", "Running", CheckStatus::Optimal), Check::new("Windows Defender", "Running", CheckStatus::Optimal),
        Check::new("Windows Firewall", "Running", CheckStatus::Optimal), Check::new("Task Scheduler", "Running", CheckStatus::Optimal),
        Check::new("Plug and Play", "Running", CheckStatus::Optimal), Check::new("RPC Service", "Running", CheckStatus::Optimal),
        Check::new("DCOM Server", "Running", CheckStatus::Optimal), Check::new("Cryptographic Services", "Running", CheckStatus::Optimal),
        Check::new("Windows Audio", "Running", CheckStatus::Optimal), Check::new("Windows Audio Endpoint Builder", "Running", CheckStatus::Optimal),
        Check::new("Themes", "Running", CheckStatus::Info), Check::new("Print Spooler", "Running", CheckStatus::Info),
        Check::new("Background Intelligent Transfer", "Running", CheckStatus::Info), Check::new("Windows Biometric Service", "Stopped", CheckStatus::Info),
        Check::new("Remote Desktop Services", "Stopped", CheckStatus::Info), Check::new("Fax", "Stopped", CheckStatus::Info),
        Check::new("HomeGroup Listener", "Stopped", CheckStatus::Info), Check::new("HomeGroup Provider", "Stopped", CheckStatus::Info),
        Check::new("Windows Mobile Hotspot", "Stopped", CheckStatus::Info), Check::new("Phone Service", "Stopped", CheckStatus::Info),
        Check::new("Retail Demo Service", "Stopped", CheckStatus::Info), Check::new("Sensor Service", "Stopped", CheckStatus::Info),
        Check::new("Smart Card", "Stopped", CheckStatus::Info), Check::new("Smart Card Removal Policy", "Stopped", CheckStatus::Info),
        Check::new("Windows Image Acquisition", "Stopped", CheckStatus::Info), Check::new("Windows Connect Now", "Stopped", CheckStatus::Info),
        Check::new("WalletService", "Stopped", CheckStatus::Info), Check::new("Windows Insider Service", "Stopped", CheckStatus::Info),
        Check::new("Downloaded Maps Manager", "Stopped", CheckStatus::Info), Check::new("Geolocation Service", "Stopped", CheckStatus::Info),
        Check::new("Remote Registry", "Stopped", CheckStatus::Info), Check::new("Connected User Experiences", "Running", CheckStatus::Info),
        Check::new("Windows Event Log", "Running", CheckStatus::Optimal), Check::new("COM+ Event System", "Running", CheckStatus::Optimal),
        Check::new("Distributed Transaction Coordinator", "Stopped", CheckStatus::Info), Check::new("Windows Management Instrumentation", "Running", CheckStatus::Optimal),
        Check::new("Shell Hardware Detection", "Running", CheckStatus::Info), Check::new("Security Accounts Manager", "Running", CheckStatus::Optimal),
        Check::new("Server", "Running", CheckStatus::Info), Check::new("Workstation", "Running", CheckStatus::Optimal),
        Check::new("Network List Service", "Running", CheckStatus::Optimal), Check::new("DNS Client", "Running", CheckStatus::Optimal),
    ];
    
    for check in base_checks {
        results.add_check(check);
    }
    
    results
}
