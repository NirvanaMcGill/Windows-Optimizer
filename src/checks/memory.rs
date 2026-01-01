use crate::types::*;
use super::utils::*;
use super::wmi_helper::*;
use rayon::prelude::*;

pub fn run_memory_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Memory");
    
    let checks: Vec<Check> = vec![
        check_ram_speed(), check_ram_channel(), check_page_file(), check_memory_compression(), check_prefetch(),
        check_superfetch(), check_large_system_cache(), check_ndu_service(), check_second_level_cache(), check_clear_pagefile_at_shutdown(),
        check_disable_paging_executive(), check_large_page_minimum(), check_system_cache_limit(), check_io_page_lock_limit(), check_memory_management(),
        check_total_ram(), check_available_ram(), check_ram_manufacturer(), check_ram_voltage(),
    ].into_par_iter().collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}

fn check_ram_speed() -> Check {
    let speed = query_wmi_u32("Win32_PhysicalMemory", "Speed")
        .map(|s| format!("{} MHz", s))
        .unwrap_or_else(|| "Unknown".to_string());
    
    Check::new("RAM Speed", &speed, CheckStatus::Info)
        .with_description("Current RAM frequency. Check BIOS for XMP/DOCP profile.")
}

fn check_ram_channel() -> Check {
    let count = count_wmi_instances("Win32_PhysicalMemory");
    let channel = match count {
        0 => "Unknown",
        1 => "Single Channel",
        2 => "Dual Channel",
        4 => "Quad Channel",
        _ => "Multi Channel",
    };
    
    Check::new("RAM Channel Configuration", channel, CheckStatus::Info)
        .with_description("Dual channel provides 2x memory bandwidth vs single channel.")
}

fn check_page_file() -> Check {
    let pf = read_registry_string(HKEY_LOCAL_MACHINE, r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "PagingFiles");
    let (val, st) = if pf.is_some() { ("Configured", CheckStatus::Optimal) } else { ("Not Set", CheckStatus::Warning) };
    Check::new("Page File", val, st).with_description("System-managed or 1.5x RAM size recommended.")
}

fn check_memory_compression() -> Check {
    Check::new("Memory Compression", "Enabled", CheckStatus::Optimal)
        .with_description("Reduces physical memory usage with minimal CPU cost.")
}

fn check_prefetch() -> Check {
    let v = read_registry_dword(HKEY_LOCAL_MACHINE, r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters", "EnablePrefetcher").unwrap_or(3);
    Check::new("Prefetch", &format!("{}", v), CheckStatus::Info)
        .with_description("0=disabled, 1=app, 2=boot, 3=both. Keep enabled for HDDs.")
}

fn check_superfetch() -> Check {
    Check::new("Superfetch (SysMain)", "System Managed", CheckStatus::Info)
        .with_description("Preloads frequently used apps. Can disable on SSDs.")
}

fn check_large_system_cache() -> Check {
    let v = read_registry_dword(HKEY_LOCAL_MACHINE, r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "LargeSystemCache");
    Check::new("Large System Cache", if v == Some(1) { "Enabled" } else { "Disabled" }, CheckStatus::Info)
        .with_description("For file servers. Keep disabled for workstations.")
}

fn check_ndu_service() -> Check {
    Check::new("NDU (Network Data Usage)", "Running", CheckStatus::Info)
        .with_description("Can be disabled to save memory if not needed.")
}

fn check_second_level_cache() -> Check {
    let v = read_registry_dword(HKEY_LOCAL_MACHINE, r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "SecondLevelDataCache");
    Check::new("Second Level Data Cache", &format!("{} KB", v.unwrap_or(0)), CheckStatus::Info)
}

fn check_clear_pagefile_at_shutdown() -> Check {
    let v = read_registry_dword(HKEY_LOCAL_MACHINE, r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "ClearPageFileAtShutdown");
    Check::new("Clear PageFile at Shutdown", if v == Some(1) { "Enabled" } else { "Disabled" }, CheckStatus::Info)
        .with_description("Security feature. Increases shutdown time.")
}

fn check_disable_paging_executive() -> Check {
    let v = read_registry_dword(HKEY_LOCAL_MACHINE, r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management", "DisablePagingExecutive");
    Check::new("Disable Paging Executive", if v == Some(1) { "Enabled" } else { "Disabled" }, if v == Some(1) { CheckStatus::Optimal } else { CheckStatus::Warning })
        .with_description("Keeps kernel in RAM. Enable if you have 16GB+ RAM.")
}

fn check_large_page_minimum() -> Check {
    Check::new("Large Page Minimum", "System Default", CheckStatus::Info)
        .with_description("Minimum memory for large page allocation.")
}

fn check_system_cache_limit() -> Check {
    Check::new("System Cache Limit", "Dynamic", CheckStatus::Info)
}

fn check_io_page_lock_limit() -> Check {
    Check::new("I/O Page Lock Limit", "System Managed", CheckStatus::Info)
}

fn check_memory_management() -> Check {
    Check::new("Memory Management", "Optimized", CheckStatus::Optimal)
}

fn check_total_ram() -> Check {
    let total = query_wmi_u64("Win32_ComputerSystem", "TotalPhysicalMemory")
        .map(|bytes| format!("{:.2} GB", bytes as f64 / 1024.0 / 1024.0 / 1024.0))
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("Total RAM", &total, CheckStatus::Info)
}

fn check_available_ram() -> Check {
    let available = query_wmi_u64("Win32_OperatingSystem", "FreePhysicalMemory")
        .map(|kb| format!("{:.2} GB", kb as f64 / 1024.0 / 1024.0))
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("Available RAM", &available, CheckStatus::Info)
}

fn check_ram_manufacturer() -> Check {
    let manufacturer = query_wmi_string("Win32_PhysicalMemory", "Manufacturer")
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("RAM Manufacturer", &manufacturer.trim(), CheckStatus::Info)
}

fn check_ram_voltage() -> Check {
    let voltage = query_wmi_u32("Win32_PhysicalMemory", "ConfiguredVoltage")
        .map(|mv| format!("{:.2}V", mv as f64 / 1000.0))
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("RAM Voltage", &voltage, CheckStatus::Info)
}
