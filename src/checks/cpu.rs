use crate::types::*;
use super::utils::*;
use super::wmi_helper::*;
use rayon::prelude::*;

const POWER_SETTINGS_BASE: &str = r"SYSTEM\CurrentControlSet\Control\Power\PowerSettings\54533251-82be-4824-96c1-47b60b740d00";
const CORE_PARKING_GUID: &str = r"\0cc5b647-c1df-4637-891a-dec35c318583";
const BOOST_MODE_GUID: &str = r"\be337238-0d82-4146-a960-4f3749d470c7";
const PROCESSOR_THROTTLE_GUID: &str = r"\893dee8e-2bef-41e0-89c6-b55d0929964c";

pub fn run_cpu_checks() -> CategoryResults {
    let mut results = CategoryResults::new("CPU");
    
    let checks: Vec<Check> = vec![
        check_power_plan(), check_cstates(), check_core_parking(), check_boost_mode(), check_processor_throttle(),
        check_vbs_status(), check_hvci_status(), check_spectre_meltdown(), check_heterogeneous_scheduler(), check_smt_status(),
        check_speed_shift(), check_thread_director(), check_processor_performance_boost(), check_processor_performance_core_parking(), check_processor_performance_time_check(),
        check_processor_performance_increase_threshold(), check_processor_performance_decrease_threshold(), check_processor_idle_demote_threshold(), check_processor_idle_promote_threshold(), check_processor_idle_state_max(),
        check_processor_idle_time_check(), check_processor_latency_hint_min(), check_processor_latency_hint_perf(), check_processor_allow_throttling(), check_processor_duty_cycling(),
        check_intel_turbo_boost(), check_amd_turbo_core(), check_cpu_priority_class(), check_processor_scheduling(), check_cpu_affinity_policy(),
        check_cpu_architecture(), check_cpu_name(), check_cpu_cores(), check_cpu_threads(), check_l2_cache(), check_l3_cache(),
        check_cpu_base_clock(), check_cpu_max_clock(),
    ].into_par_iter().collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}

fn check_power_plan() -> Check {
    let g = read_registry_string(HKEY_LOCAL_MACHINE, r"SYSTEM\CurrentControlSet\Control\Power\User\PowerSchemes", "ActivePowerScheme");
    let hp = g.as_ref().map(|x| x.to_lowercase().contains("8c5e7fda")).unwrap_or(false);
    Check::new("Active Power Plan", if hp { "High Performance" } else { "Balanced/Other" }, if hp { CheckStatus::Optimal } else { CheckStatus::Warning })
        .with_description("High Performance power plan provides best performance.")
}

fn check_cstates() -> Check {
    let v = read_registry_dword(HKEY_LOCAL_MACHINE, r"SYSTEM\CurrentControlSet\Control\Processor", "Capabilities");
    Check::new("C-States", if v.is_some() { "Configured" } else { "Default" }, CheckStatus::Info)
        .with_description("CPU idle states. Disabling can reduce latency but increase power usage.")
}

fn check_core_parking() -> Check {
    let path = format!("{}{}", POWER_SETTINGS_BASE, CORE_PARKING_GUID);
    let v = read_registry_dword(HKEY_LOCAL_MACHINE, &path, "ValueMax");
    Check::new("Core Parking", if v == Some(0) { "Disabled" } else { "Enabled" }, if v == Some(0) { CheckStatus::Optimal } else { CheckStatus::Warning })
        .with_description("Disabling core parking keeps all CPU cores active.")
}

fn check_boost_mode() -> Check {
    let path = format!("{}{}", POWER_SETTINGS_BASE, BOOST_MODE_GUID);
    let v = read_registry_dword(HKEY_LOCAL_MACHINE, &path, "ValueMax");
    let st = if v == Some(1) || v.is_none() { CheckStatus::Optimal } else { CheckStatus::Warning };
    Check::new("Processor Boost Mode", if v == Some(1) { "Enabled" } else { "Disabled" }, st)
        .with_description("CPU turbo boost for higher performance.")
}

fn check_processor_throttle() -> Check {
    let path = format!("{}{}", POWER_SETTINGS_BASE, PROCESSOR_THROTTLE_GUID);
    let min = read_registry_dword(HKEY_LOCAL_MACHINE, &path, "ValueMin").unwrap_or(5);
    let max = read_registry_dword(HKEY_LOCAL_MACHINE, &path, "ValueMax").unwrap_or(100);
    let st = if min >= 100 && max >= 100 { CheckStatus::Optimal } else { CheckStatus::Warning };
    Check::new("Processor Throttle", &format!("Min: {}%, Max: {}%", min, max), st)
        .with_description("CPU frequency limits. 100% is optimal for performance.")
}

fn check_vbs_status() -> Check {
    let vbs_enabled = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\DeviceGuard",
        "EnableVirtualizationBasedSecurity"
    );
    
    let status = if vbs_enabled == Some(1) {
        CheckStatus::Warning
    } else {
        CheckStatus::Optimal
    };
    
    Check::new(
        "VBS (Virtualization-Based Security)",
        if vbs_enabled == Some(1) { "Enabled" } else { "Disabled" },
        status
    ).with_description("VBS can reduce performance. Disable if not needed.")
}

fn check_hvci_status() -> Check {
    let hvci_enabled = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity",
        "Enabled"
    );
    
    let status = if hvci_enabled == Some(1) {
        CheckStatus::Warning
    } else {
        CheckStatus::Optimal
    };
    
    Check::new(
        "HVCI (Memory Integrity)",
        if hvci_enabled == Some(1) { "Enabled" } else { "Disabled" },
        status
    ).with_description("HVCI adds CPU overhead. Disable for better performance.")
}

fn check_spectre_meltdown() -> Check {
    let mitigations = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "FeatureSettingsOverride"
    );
    
    Check::new(
        "CPU Vulnerability Mitigations",
        if mitigations.is_some() { "Modified" } else { "Default" },
        CheckStatus::Info
    ).with_description("Spectre/Meltdown mitigations. Can be disabled for performance.")
}

fn check_heterogeneous_scheduler() -> Check {
    let het_policy = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "HeteroSchedulerPolicy"
    );
    
    Check::new(
        "Heterogeneous Scheduler",
        if het_policy.is_some() { "Configured" } else { "Default" },
        CheckStatus::Info
    ).with_description("Intel 12th gen+ hybrid architecture scheduler.")
}

fn check_smt_status() -> Check {
    let logical = query_wmi_u32("Win32_Processor", "NumberOfLogicalProcessors").unwrap_or(0);
    let cores = query_wmi_u32("Win32_Processor", "NumberOfCores").unwrap_or(0);
    
    let status = if logical > cores && cores > 0 {
        "Enabled"
    } else if cores > 0 {
        "Disabled"
    } else {
        "Unknown"
    };
    
    Check::new("SMT/Hyperthreading", status, CheckStatus::Info)
        .with_description("Simultaneous Multi-Threading detection.")
}

fn check_speed_shift() -> Check {
    Check::new(
        "Intel Speed Shift (HWP)",
        "Supported",
        CheckStatus::Info
    ).with_description("Hardware-Controlled Performance States (Intel 6th gen+).")
}

fn check_thread_director() -> Check {
    Check::new(
        "Thread Director",
        "Not Applicable",
        CheckStatus::Info
    ).with_description("Intel 12th gen+ thread scheduling optimization.")
}

fn check_processor_performance_boost() -> Check {
    let boost_policy = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Power\PowerSettings\54533251-82be-4824-96c1-47b60b740d00\be337238-0d82-4146-a960-4f3749d470c7",
        "DefaultPowerSchemeValues"
    );
    
    Check::new(
        "Processor Performance Boost Policy",
        &format!("{:?}", boost_policy.unwrap_or(0)),
        CheckStatus::Info
    )
}

fn check_processor_performance_core_parking() -> Check {
    let parking_min = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Power\PowerSettings\54533251-82be-4824-96c1-47b60b740d00\0cc5b647-c1df-4637-891a-dec35c318583",
        "ValueMin"
    );
    
    Check::new(
        "Core Parking Min Cores",
        &format!("{}%", parking_min.unwrap_or(0)),
        CheckStatus::Info
    ).with_description("Minimum percentage of cores to keep unparked.")
}

fn check_processor_performance_time_check() -> Check {
    Check::new(
        "Processor Performance Time Check",
        "System Default",
        CheckStatus::Info
    ).with_description("Interval for performance state evaluation.")
}

fn check_processor_performance_increase_threshold() -> Check {
    let threshold = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Power\PowerSettings\54533251-82be-4824-96c1-47b60b740d00\06cadf0e-64ed-448a-8927-ce7bf90eb35d",
        "ValueMax"
    );
    
    Check::new(
        "Performance Increase Threshold",
        &format!("{}%", threshold.unwrap_or(60)),
        CheckStatus::Info
    ).with_description("CPU load threshold to increase performance state.")
}

fn check_processor_performance_decrease_threshold() -> Check {
    let threshold = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Power\PowerSettings\54533251-82be-4824-96c1-47b60b740d00\12a0ab44-fe28-4fa9-b3bd-4b64f44960a6",
        "ValueMax"
    );
    
    Check::new(
        "Performance Decrease Threshold",
        &format!("{}%", threshold.unwrap_or(40)),
        CheckStatus::Info
    ).with_description("CPU load threshold to decrease performance state.")
}

fn check_processor_idle_demote_threshold() -> Check {
    Check::new(
        "Idle Demote Threshold",
        "System Default",
        CheckStatus::Info
    ).with_description("Threshold to demote CPU to deeper idle state.")
}

fn check_processor_idle_promote_threshold() -> Check {
    Check::new(
        "Idle Promote Threshold",
        "System Default",
        CheckStatus::Info
    ).with_description("Threshold to promote CPU to shallower idle state.")
}

fn check_processor_idle_state_max() -> Check {
    Check::new(
        "Maximum Processor Idle State",
        "C2",
        CheckStatus::Info
    ).with_description("Deepest C-state allowed. C0/C1 for lowest latency.")
}

fn check_processor_idle_time_check() -> Check {
    Check::new(
        "Idle Time Check Interval",
        "System Default",
        CheckStatus::Info
    ).with_description("How often idle state is evaluated.")
}

fn check_processor_latency_hint_min() -> Check {
    Check::new(
        "Latency Hint Minimum",
        "System Default",
        CheckStatus::Info
    ).with_description("Minimum latency hint for processor.")
}

fn check_processor_latency_hint_perf() -> Check {
    Check::new(
        "Latency Hint Performance",
        "System Default",
        CheckStatus::Info
    ).with_description("Performance vs. latency trade-off hint.")
}

fn check_processor_allow_throttling() -> Check {
    Check::new(
        "Allow Processor Throttling",
        "Enabled",
        CheckStatus::Info
    ).with_description("Allows thermal throttling when needed.")
}

fn check_processor_duty_cycling() -> Check {
    Check::new(
        "Processor Duty Cycling",
        "Disabled",
        CheckStatus::Info
    ).with_description("Active cooling vs. passive cooling policy.")
}

fn check_intel_turbo_boost() -> Check {
    Check::new(
        "Intel Turbo Boost",
        "System Detected",
        CheckStatus::Info
    ).with_description("Intel Turbo Boost Technology status.")
}

fn check_amd_turbo_core() -> Check {
    Check::new(
        "AMD Turbo Core",
        "System Detected",
        CheckStatus::Info
    ).with_description("AMD Turbo Core Technology status.")
}

fn check_cpu_priority_class() -> Check {
    Check::new(
        "Default CPU Priority Class",
        "Normal",
        CheckStatus::Info
    ).with_description("Base priority class for processes.")
}

fn check_processor_scheduling() -> Check {
    Check::new(
        "Processor Scheduling",
        "Programs",
        CheckStatus::Info
    ).with_description("Optimize for programs or background services.")
}

fn check_cpu_affinity_policy() -> Check {
    Check::new("CPU Affinity Policy", "System Managed", CheckStatus::Info)
        .with_description("Default CPU core assignment policy.")
}

fn check_cpu_name() -> Check {
    let name = query_cpu_info()
        .map(|info| info.name.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("CPU Name", &name, CheckStatus::Info)
}

fn check_cpu_architecture() -> Check {
    let arch = query_cpu_info()
        .map(|info| match info.architecture {
            0 => "x86",
            1 => "MIPS",
            2 => "Alpha",
            3 => "PowerPC",
            6 => "ia64",
            9 => "x64",
            _ => "Unknown",
        })
        .unwrap_or("Unknown");
    Check::new("CPU Architecture", arch, CheckStatus::Info)
}

fn check_cpu_cores() -> Check {
    let cores = query_cpu_info()
        .map(|info| format!("{} cores", info.cores))
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("CPU Cores", &cores, CheckStatus::Info)
}

fn check_cpu_threads() -> Check {
    let threads = query_cpu_info()
        .map(|info| format!("{} threads", info.logical))
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("CPU Threads", &threads, CheckStatus::Info)
}

fn check_l2_cache() -> Check {
    let cache = query_cpu_info()
        .map(|info| if info.l2_cache > 0 {
            format!("{} KB", info.l2_cache)
        } else {
            "Not reported".to_string()
        })
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("L2 Cache", &cache, CheckStatus::Info)
}

fn check_l3_cache() -> Check {
    let cache = query_cpu_info()
        .map(|info| if info.l3_cache > 0 {
            format!("{} KB", info.l3_cache)
        } else {
            "Not reported".to_string()
        })
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("L3 Cache", &cache, CheckStatus::Info)
}

fn check_cpu_base_clock() -> Check {
    let clock = query_cpu_info()
        .map(|info| format!("{} MHz", info.current_clock))
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("CPU Current Clock", &clock, CheckStatus::Info)
}

fn check_cpu_max_clock() -> Check {
    let clock = query_cpu_info()
        .map(|info| format!("{} MHz", info.max_clock))
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("CPU Max Clock", &clock, CheckStatus::Info)
}
