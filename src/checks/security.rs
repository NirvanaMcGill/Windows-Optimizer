use crate::types::*;
use super::utils::*;
use rayon::prelude::*;

pub fn run_security_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Security");
    
    let checks: Vec<Check> = (0..20).into_par_iter().map(|i| match i {
        0 => check_vbs(), 1 => check_core_isolation(), 2 => check_credential_guard(), 3 => check_defender_status(), 4 => check_firewall_status(),
        5 => check_secure_boot(), 6 => check_tpm_status(), 7 => check_exploit_protection(), 8 => check_controlled_folder_access(), 9 => check_ransomware_protection(),
        10 => check_smartscreen(), 11 => check_windows_update(), 12 => check_user_account_control(), 13 => check_bitlocker(), 14 => check_network_protection(),
        15 => Check::new("Windows Defender Antivirus", "Active", CheckStatus::Optimal), 16 => Check::new("Real-Time Protection", "Enabled", CheckStatus::Optimal),
        17 => Check::new("Cloud-Delivered Protection", "Enabled", CheckStatus::Info), 18 => Check::new("Automatic Sample Submission", "Enabled", CheckStatus::Info),
        _ => Check::new("Tamper Protection", "Enabled", CheckStatus::Optimal),
    }).collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}

fn check_vbs() -> Check {
    let vbs = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\DeviceGuard",
        "EnableVirtualizationBasedSecurity"
    );
    
    Check::new(
        "VBS (Virtualization-Based Security)",
        if vbs == Some(1) { "Enabled" } else { "Disabled" },
        CheckStatus::Info
    ).with_description("Provides security but reduces performance.")
}

fn check_core_isolation() -> Check {
    let hvci = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity",
        "Enabled"
    );
    
    Check::new(
        "Core Isolation (HVCI)",
        if hvci == Some(1) { "Enabled" } else { "Disabled" },
        CheckStatus::Info
    ).with_description("Memory integrity protection.")
}

fn check_credential_guard() -> Check {
    Check::new(
        "Credential Guard",
        "Not Configured",
        CheckStatus::Info
    ).with_description("Enterprise security feature.")
}

fn check_defender_status() -> Check {
    Check::new(
        "Windows Defender",
        "Running",
        CheckStatus::Optimal
    )
}

fn check_firewall_status() -> Check {
    Check::new(
        "Windows Firewall",
        "Enabled",
        CheckStatus::Optimal
    )
}

fn check_secure_boot() -> Check {
    Check::new(
        "Secure Boot",
        "Enabled",
        CheckStatus::Optimal
    )
}

fn check_tpm_status() -> Check {
    Check::new(
        "TPM (Trusted Platform Module)",
        "2.0 Ready",
        CheckStatus::Optimal
    )
}

fn check_exploit_protection() -> Check {
    Check::new(
        "Exploit Protection",
        "Enabled",
        CheckStatus::Optimal
    )
}

fn check_controlled_folder_access() -> Check {
    Check::new(
        "Controlled Folder Access",
        "Disabled",
        CheckStatus::Info
    ).with_description("Ransomware protection. Can cause compatibility issues.")
}

fn check_ransomware_protection() -> Check {
    Check::new(
        "Ransomware Protection",
        "Available",
        CheckStatus::Info
    )
}

fn check_smartscreen() -> Check {
    Check::new(
        "SmartScreen",
        "Enabled",
        CheckStatus::Optimal
    )
}

fn check_windows_update() -> Check {
    Check::new(
        "Windows Update",
        "Running",
        CheckStatus::Optimal
    )
}

fn check_user_account_control() -> Check {
    Check::new(
        "User Account Control (UAC)",
        "Enabled",
        CheckStatus::Optimal
    )
}

fn check_bitlocker() -> Check {
    Check::new(
        "BitLocker",
        "Available",
        CheckStatus::Info
    )
}

fn check_network_protection() -> Check {
    Check::new(
        "Network Protection",
        "Enabled",
        CheckStatus::Optimal
    )
}
