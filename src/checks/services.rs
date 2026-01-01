use crate::types::*;
use super::service_helper::*;
use rayon::prelude::*;

pub fn run_services_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Services");
    
    let service_checks = vec![
        ("DiagTrack", "DiagTrack (Telemetry)", CheckStatus::Warning, "Can be disabled for privacy."),
        ("dmwappushservice", "dmwappushservice", CheckStatus::Info, "Push notification service."),
        ("SysMain", "SysMain (Superfetch)", CheckStatus::Info, "Preloads apps."),
        ("TabletInputService", "TabletInputService", CheckStatus::Info, "Tablet PC input."),
        ("WSearch", "WSearch (Windows Search)", CheckStatus::Info, "File indexing."),
        ("XblAuthManager", "XblAuthManager", CheckStatus::Info, "Xbox Live Auth."),
        ("XblGameSave", "XblGameSave", CheckStatus::Info, "Xbox Game Save."),
        ("XboxNetApiSvc", "XboxNetApiSvc", CheckStatus::Info, "Xbox Network."),
        ("XboxGipSvc", "XboxGipSvc", CheckStatus::Info, "Xbox Accessory Management."),
        ("wuauserv", "Windows Update", CheckStatus::Optimal, "System updates."),
        ("WinDefend", "Windows Defender", CheckStatus::Optimal, "Antivirus protection."),
        ("MpsSvc", "Windows Firewall", CheckStatus::Optimal, "Network security."),
        ("Schedule", "Task Scheduler", CheckStatus::Optimal, "Scheduled tasks."),
        ("PlugPlay", "Plug and Play", CheckStatus::Optimal, "Device detection."),
        ("RpcSs", "RPC Service", CheckStatus::Optimal, "Remote procedure calls."),
        ("DcomLaunch", "DCOM Server", CheckStatus::Optimal, "Component services."),
        ("CryptSvc", "Cryptographic Services", CheckStatus::Optimal, "Encryption services."),
        ("AudioSrv", "Windows Audio", CheckStatus::Optimal, "Audio management."),
        ("AudioEndpointBuilder", "Windows Audio Endpoint Builder", CheckStatus::Optimal, "Audio device management."),
        ("Themes", "Themes", CheckStatus::Info, "Visual themes."),
        ("Spooler", "Print Spooler", CheckStatus::Info, "Print management."),
        ("BITS", "Background Intelligent Transfer", CheckStatus::Info, "Background downloads."),
        ("WbioSrvc", "Windows Biometric Service", CheckStatus::Info, "Biometric devices."),
        ("TermService", "Remote Desktop Services", CheckStatus::Info, "Remote desktop."),
        ("Fax", "Fax", CheckStatus::Info, "Fax service."),
        ("HomeGroupListener", "HomeGroup Listener", CheckStatus::Info, "HomeGroup."),
        ("HomeGroupProvider", "HomeGroup Provider", CheckStatus::Info, "HomeGroup."),
        ("icssvc", "Windows Mobile Hotspot", CheckStatus::Info, "Mobile hotspot."),
        ("PhoneSvc", "Phone Service", CheckStatus::Info, "Phone functionality."),
        ("RetailDemo", "Retail Demo Service", CheckStatus::Info, "Demo mode."),
        ("SensorService", "Sensor Service", CheckStatus::Info, "Sensor management."),
        ("ScDeviceEnum", "Smart Card Device Enumeration", CheckStatus::Info, "Smart card."),
        ("SCPolicySvc", "Smart Card Removal Policy", CheckStatus::Info, "Smart card."),
        ("WiaRpc", "Windows Image Acquisition", CheckStatus::Info, "Scanner/camera."),
        ("wcncsvc", "Windows Connect Now", CheckStatus::Info, "Network setup."),
        ("WalletService", "WalletService", CheckStatus::Info, "Wallet service."),
        ("wisvc", "Windows Insider Service", CheckStatus::Info, "Insider builds."),
        ("MapsBroker", "Downloaded Maps Manager", CheckStatus::Info, "Offline maps."),
        ("lfsvc", "Geolocation Service", CheckStatus::Info, "Location services."),
        ("RemoteRegistry", "Remote Registry", CheckStatus::Info, "Remote registry access."),
        ("CDPUserSvc", "Connected User Experiences", CheckStatus::Info, "Connected experiences."),
        ("EventLog", "Windows Event Log", CheckStatus::Optimal, "Event logging."),
        ("EventSystem", "COM+ Event System", CheckStatus::Optimal, "COM+ events."),
        ("MSDTC", "Distributed Transaction Coordinator", CheckStatus::Info, "Distributed transactions."),
        ("Winmgmt", "Windows Management Instrumentation", CheckStatus::Optimal, "WMI service."),
        ("ShellHWDetection", "Shell Hardware Detection", CheckStatus::Info, "Hardware events."),
        ("SamSs", "Security Accounts Manager", CheckStatus::Optimal, "Account management."),
        ("LanmanServer", "Server", CheckStatus::Info, "File sharing."),
        ("LanmanWorkstation", "Workstation", CheckStatus::Optimal, "Network connections."),
        ("netprofm", "Network List Service", CheckStatus::Optimal, "Network identification."),
        ("Dnscache", "DNS Client", CheckStatus::Optimal, "DNS resolution."),
    ];
    
    let checks: Vec<Check> = service_checks.into_par_iter()
        .map(|(svc, name, default_status, desc)| {
            let status_str = query_service_status(svc).unwrap_or_else(|| "Unknown".to_string());
            Check::new(name, &status_str, default_status).with_description(desc)
        })
        .collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}
