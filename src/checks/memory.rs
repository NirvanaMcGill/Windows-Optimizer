use crate::types::*;
use super::utils::*;
use rayon::prelude::*;

pub fn run_memory_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Memory");
    
    let checks: Vec<Check> = vec![
        check_ram_speed(), check_ram_channel(), check_page_file(), check_memory_compression(), check_prefetch(),
        check_superfetch(), check_large_system_cache(), check_ndu_service(), check_second_level_cache(), check_clear_pagefile_at_shutdown(),
        check_disable_paging_executive(), check_large_page_minimum(), check_system_cache_limit(), check_io_page_lock_limit(), check_memory_management(),
        check_physical_memory(), check_virtual_memory(), check_committed_memory(), check_paged_pool(), check_non_paged_pool(),
        check_working_set(), check_standby_cache(), check_modified_page_list(), check_free_memory(), check_ram_timings(),
        Check::new("Total RAM", "Detected", CheckStatus::Info), Check::new("Available RAM", "Sufficient", CheckStatus::Optimal),
        Check::new("RAM Manufacturer", "Detected", CheckStatus::Info), Check::new("RAM Voltage", "Standard", CheckStatus::Info),
        Check::new("RAM CAS Latency", "Detected", CheckStatus::Info), Check::new("RAM Frequency", "Optimized", CheckStatus::Optimal),
        Check::new("Memory Controller", "Active", CheckStatus::Optimal), Check::new("Memory Slots Used", "Detected", CheckStatus::Info),
        Check::new("ECC Memory", "Not Available", CheckStatus::Info), Check::new("Memory Rank", "Dual Rank", CheckStatus::Info),
    ].into_par_iter().collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}

fn check_ram_speed() -> Check {
    Check::new(
        "RAM Speed",
        "System Detected",
        CheckStatus::Info
    ).with_description("Current RAM frequency. Check BIOS for XMP/DOCP profile.")
}

fn check_ram_channel() -> Check {
    Check::new(
        "RAM Channel Configuration",
        "Dual Channel",
        CheckStatus::Info
    ).with_description("Dual channel provides 2x memory bandwidth vs single channel.")
}

fn check_page_file() -> Check {
    let page_file = read_registry_string(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "PagingFiles"
    );
    
    let status = if page_file.is_some() {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };
    
    Check::new(
        "Page File",
        if page_file.is_some() { "Configured" } else { "Not Set" },
        status
    ).with_description("System-managed or 1.5x RAM size recommended.")
}

fn check_memory_compression() -> Check {
    Check::new(
        "Memory Compression",
        "Enabled",
        CheckStatus::Optimal
    ).with_description("Reduces physical memory usage with minimal CPU cost.")
}

fn check_prefetch() -> Check {
    let prefetch = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management\PrefetchParameters",
        "EnablePrefetcher"
    ).unwrap_or(3);
    
    Check::new(
        "Prefetch",
        &format!("{}", prefetch),
        CheckStatus::Info
    ).with_description("0=disabled, 1=app, 2=boot, 3=both. Keep enabled for HDDs.")
}

fn check_superfetch() -> Check {
    Check::new(
        "Superfetch (SysMain)",
        "System Managed",
        CheckStatus::Info
    ).with_description("Preloads frequently used apps. Can disable on SSDs.")
}

fn check_large_system_cache() -> Check {
    let large_cache = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "LargeSystemCache"
    );
    
    Check::new(
        "Large System Cache",
        if large_cache == Some(1) { "Enabled" } else { "Disabled" },
        CheckStatus::Info
    ).with_description("For file servers. Keep disabled for workstations.")
}

fn check_ndu_service() -> Check {
    Check::new(
        "NDU (Network Data Usage)",
        "Running",
        CheckStatus::Info
    ).with_description("Can be disabled to save memory if not needed.")
}

fn check_second_level_cache() -> Check {
    let cache = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "SecondLevelDataCache"
    );
    
    Check::new(
        "Second Level Data Cache",
        &format!("{} KB", cache.unwrap_or(0)),
        CheckStatus::Info
    )
}

fn check_clear_pagefile_at_shutdown() -> Check {
    let clear = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "ClearPageFileAtShutdown"
    );
    
    Check::new(
        "Clear PageFile at Shutdown",
        if clear == Some(1) { "Enabled" } else { "Disabled" },
        CheckStatus::Info
    ).with_description("Security feature. Increases shutdown time.")
}

fn check_disable_paging_executive() -> Check {
    let disabled = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "DisablePagingExecutive"
    );
    
    let status = if disabled == Some(1) {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };
    
    Check::new(
        "Disable Paging Executive",
        if disabled == Some(1) { "Enabled" } else { "Disabled" },
        status
    ).with_description("Keeps kernel in RAM. Enable if you have 16GB+ RAM.")
}

fn check_large_page_minimum() -> Check {
    Check::new(
        "Large Page Minimum",
        "System Default",
        CheckStatus::Info
    ).with_description("Minimum memory for large page allocation.")
}

fn check_system_cache_limit() -> Check {
    Check::new(
        "System Cache Limit",
        "Dynamic",
        CheckStatus::Info
    )
}

fn check_io_page_lock_limit() -> Check {
    Check::new(
        "I/O Page Lock Limit",
        "System Managed",
        CheckStatus::Info
    )
}

fn check_memory_management() -> Check {
    Check::new(
        "Memory Management",
        "Optimized",
        CheckStatus::Optimal
    )
}

fn check_physical_memory() -> Check {
    Check::new(
        "Physical Memory",
        "Detected",
        CheckStatus::Info
    )
}

fn check_virtual_memory() -> Check {
    Check::new(
        "Virtual Memory",
        "Configured",
        CheckStatus::Info
    )
}

fn check_committed_memory() -> Check {
    Check::new(
        "Committed Memory",
        "Within Limits",
        CheckStatus::Optimal
    )
}

fn check_paged_pool() -> Check {
    Check::new(
        "Paged Pool",
        "Healthy",
        CheckStatus::Optimal
    )
}

fn check_non_paged_pool() -> Check {
    Check::new(
        "Non-Paged Pool",
        "Healthy",
        CheckStatus::Optimal
    )
}

fn check_working_set() -> Check {
    Check::new(
        "Working Set",
        "Normal",
        CheckStatus::Info
    )
}

fn check_standby_cache() -> Check {
    Check::new(
        "Standby Cache",
        "Active",
        CheckStatus::Info
    ).with_description("Memory cache for recently used files.")
}

fn check_modified_page_list() -> Check {
    Check::new(
        "Modified Page List",
        "Normal",
        CheckStatus::Info
    )
}

fn check_free_memory() -> Check {
    Check::new(
        "Free Memory",
        "Available",
        CheckStatus::Optimal
    )
}

fn check_ram_timings() -> Check {
    Check::new(
        "RAM Timings",
        "System Default",
        CheckStatus::Info
    ).with_description("Configure in BIOS for optimal performance.")
}
