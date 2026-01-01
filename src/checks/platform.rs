use super::utils::*;
use super::wmi_helper::*;
use crate::types::*;

pub fn run_platform_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Platform");

    results.add_check(check_windows_version());
    results.add_check(check_windows_build());
    results.add_check(check_windows_edition());
    results.add_check(check_system_type());
    results.add_check(check_bios_mode());
    results.add_check(check_manufacturer());
    results.add_check(check_model());
    results.add_check(check_system_uptime());

    results
}

fn check_windows_version() -> Check {
    let version = query_wmi_string("Win32_OperatingSystem", "Caption")
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("Windows Version", &version, CheckStatus::Info)
}

fn check_windows_build() -> Check {
    let build = query_wmi_string("Win32_OperatingSystem", "BuildNumber")
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("Windows Build", &build, CheckStatus::Info)
}

fn check_windows_edition() -> Check {
    let edition = read_registry_string(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        "EditionID",
    )
    .unwrap_or_else(|| "Unknown".to_string());
    Check::new("Windows Edition", &edition, CheckStatus::Info)
}

fn check_system_type() -> Check {
    let arch = query_wmi_string("Win32_OperatingSystem", "OSArchitecture")
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("System Architecture", &arch, CheckStatus::Info)
}

fn check_bios_mode() -> Check {
    // Check if running in UEFI or Legacy BIOS mode
    let mode = if std::path::Path::new("C:\\Windows\\Panther\\setupact.log").exists() {
        // Try to detect from setupact.log or firmware environment variables
        "UEFI/Legacy"
    } else {
        "Unknown"
    };
    Check::new("BIOS Mode", mode, CheckStatus::Info)
}

fn check_manufacturer() -> Check {
    let manufacturer = query_wmi_string("Win32_ComputerSystem", "Manufacturer")
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("System Manufacturer", &manufacturer, CheckStatus::Info)
}

fn check_model() -> Check {
    let model =
        query_wmi_string("Win32_ComputerSystem", "Model").unwrap_or_else(|| "Unknown".to_string());
    Check::new("System Model", &model, CheckStatus::Info)
}

fn check_system_uptime() -> Check {
    let last_boot = query_wmi_string("Win32_OperatingSystem", "LastBootUpTime")
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("Last Boot Time", &last_boot, CheckStatus::Info)
}
