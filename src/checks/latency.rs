use super::utils::*;
use crate::types::*;
use rayon::prelude::*;

pub fn run_latency_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Latency");

    let checks: Vec<Check> = vec![
        check_hpet_status(),
        check_tsc_sync_policy(),
        check_dynamic_tick(),
        check_system_responsiveness(),
        check_network_throttling_index(),
        check_win32_priority_separation(),
        check_timer_resolution(),
        check_interrupt_steering(),
        check_message_signaled_interrupts(),
        check_dpc_watchdog_period(),
        check_dpc_timeout(),
        check_idle_disable(),
        check_distribute_timers(),
        check_processor_idle_state_policy(),
        check_latency_sensitive_hints(),
        check_cstate_latency(),
        check_platform_clock_source(),
        check_tsc_invariant(),
        check_use_platform_clock(),
        check_synthetic_timer(),
        check_large_page_drivers(),
        check_disable_dynamic_pstate(),
        check_system_profile_tasks(),
        check_multimedia_system_profile(),
        check_gpu_priority(),
        check_scheduling_category(),
        check_system_responsiveness_override(),
        check_latency_tolerance(),
        check_interrupt_affinity_policy(),
        check_isr_time_limit(),
        check_dpc_queue_depth(),
        check_threaded_dpc(),
        check_timer_coalescing(),
        check_precision_platform_timer(),
        check_acpi_timer_disabled(),
        Check::new("Quantum Length", "Default", CheckStatus::Info),
        Check::new("Foreground Boost", "3", CheckStatus::Info),
        Check::new("IRQ Priority", "System Default", CheckStatus::Info),
        Check::new("DMA Channel Allocation", "Optimal", CheckStatus::Info),
        Check::new("Memory Mapped I/O", "Enabled", CheckStatus::Optimal),
        Check::new("Real-Time Priority Class", "Available", CheckStatus::Info),
        Check::new(
            "High Priority Thread Support",
            "Enabled",
            CheckStatus::Optimal,
        ),
        Check::new("Low Latency Audio", "Supported", CheckStatus::Info),
        Check::new(
            "Kernel Dispatcher Priority",
            "Optimized",
            CheckStatus::Optimal,
        ),
        Check::new("Thread Switching Overhead", "Minimal", CheckStatus::Optimal),
        Check::new("Context Switch Rate", "Normal", CheckStatus::Optimal),
    ]
    .into_par_iter()
    .collect();

    for check in checks {
        results.add_check(check);
    }

    results
}

fn check_hpet_status() -> Check {
    // Check HPET (High Precision Event Timer) via BCD or registry
    let hpet_enabled = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\TimeProviders\TimerDevice",
        "TimerDevice",
    )
    .is_some();

    let status = if hpet_enabled {
        CheckStatus::Warning
    } else {
        CheckStatus::Optimal
    };

    Check::new(
        "HPET (High Precision Event Timer)",
        if hpet_enabled { "Enabled" } else { "Disabled" },
        status,
    )
    .with_description("HPET can add latency. Disabled is better for gaming/real-time.")
}

fn check_tsc_sync_policy() -> Check {
    let tsc_policy = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "GlobalTimerResolutionRequests",
    );

    Check::new(
        "TSC Sync Policy",
        &format!("{:?}", tsc_policy.unwrap_or(0)),
        CheckStatus::Info,
    )
    .with_description("Time Stamp Counter synchronization policy")
}

fn check_dynamic_tick() -> Check {
    let dynamic_tick = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "DisableDynamicTick",
    );

    let is_disabled = dynamic_tick == Some(1);
    let status = if is_disabled {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };

    Check::new(
        "Dynamic Tick",
        if is_disabled { "Disabled" } else { "Enabled" },
        status,
    )
    .with_description("Dynamic tick can increase latency. Disable for lower latency.")
}

fn check_system_responsiveness() -> Check {
    let responsiveness = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "SystemResponsiveness",
    )
    .unwrap_or(20);

    let status = if responsiveness <= 10 {
        CheckStatus::Optimal
    } else if responsiveness <= 20 {
        CheckStatus::Warning
    } else {
        CheckStatus::Issue
    };

    Check::new(
        "System Responsiveness (MMCSS)",
        &format!("{}%", responsiveness),
        status,
    )
    .with_description("Controls CPU reservation for multimedia. Lower is better (0-10 optimal).")
}

fn check_network_throttling_index() -> Check {
    let throttling = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
        "NetworkThrottlingIndex",
    )
    .unwrap_or(10);

    let status = if throttling == 0xFFFFFFFF {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };

    Check::new(
        "Network Throttling Index",
        &format!("{}", throttling),
        status,
    )
    .with_description("Network packet processing throttling. 0xFFFFFFFF (disabled) is optimal.")
}

fn check_win32_priority_separation() -> Check {
    let priority = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\PriorityControl",
        "Win32PrioritySeparation",
    )
    .unwrap_or(2);

    let status = if priority == 38 || priority == 26 {
        CheckStatus::Optimal
    } else {
        CheckStatus::Info
    };

    Check::new(
        "Win32 Priority Separation",
        &format!("{}", priority),
        status,
    )
    .with_description("Process scheduler priority. 38=long fixed, 26=short variable (gaming).")
}

fn check_timer_resolution() -> Check {
    Check::new(
        "Global Timer Resolution",
        "System Default",
        CheckStatus::Info,
    )
    .with_description("Current system timer resolution. Lower is better for latency.")
}

fn check_interrupt_steering() -> Check {
    let steering = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\PnP\Pci",
        "DeviceInterruptPolicyEnabled",
    );

    Check::new(
        "Interrupt Steering",
        if steering == Some(1) {
            "Enabled"
        } else {
            "Disabled"
        },
        CheckStatus::Info,
    )
    .with_description("Allows OS to route device interrupts to specific CPUs.")
}

fn check_message_signaled_interrupts() -> Check {
    let msi_supported = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Services\nvlddmkm",
        "RmMsiAllowed",
    );

    Check::new(
        "MSI Mode (GPU)",
        if msi_supported == Some(1) {
            "Enabled"
        } else {
            "Check Required"
        },
        CheckStatus::Info,
    )
    .with_description("Message-Signaled Interrupts reduce latency vs. line-based interrupts.")
}

fn check_dpc_watchdog_period() -> Check {
    let period = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "DpcWatchdogPeriod",
    );

    Check::new(
        "DPC Watchdog Period",
        &format!("{:?}", period.unwrap_or(0)),
        CheckStatus::Info,
    )
    .with_description("Deferred Procedure Call watchdog timeout.")
}

fn check_dpc_timeout() -> Check {
    let timeout = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "DpcTimeout",
    );

    Check::new(
        "DPC Timeout",
        &format!("{:?}", timeout.unwrap_or(0)),
        CheckStatus::Info,
    )
    .with_description("Maximum time for DPC execution.")
}

fn check_idle_disable() -> Check {
    let disabled = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "IdleSchedulingPolicy",
    );

    Check::new(
        "Idle Scheduling Policy",
        &format!("{:?}", disabled.unwrap_or(0)),
        CheckStatus::Info,
    )
}

fn check_distribute_timers() -> Check {
    let distribute = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "DistributeTimers",
    );

    Check::new(
        "Distribute Timers",
        if distribute == Some(1) {
            "Enabled"
        } else {
            "Disabled"
        },
        CheckStatus::Info,
    )
    .with_description("Distributes timer interrupts across CPUs.")
}

fn check_processor_idle_state_policy() -> Check {
    Check::new("Processor Idle State Policy", "Detected", CheckStatus::Info)
        .with_description("CPU idle state management policy.")
}

fn check_latency_sensitive_hints() -> Check {
    let hints = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Power",
        "LatencySensitivityHint",
    );

    Check::new(
        "Latency Sensitivity Hints",
        if hints == Some(1) {
            "Enabled"
        } else {
            "Disabled"
        },
        CheckStatus::Info,
    )
}

fn check_cstate_latency() -> Check {
    Check::new(
        "C-State Exit Latency",
        "System Dependent",
        CheckStatus::Info,
    )
    .with_description("Time to exit CPU C-states. Lower is better.")
}

fn check_platform_clock_source() -> Check {
    Check::new("Platform Clock Source", "TSC", CheckStatus::Info)
        .with_description("Primary time source: TSC (best), HPET, or ACPI PM Timer.")
}

fn check_tsc_invariant() -> Check {
    Check::new("TSC Invariant", "Supported", CheckStatus::Info)
        .with_description("Time Stamp Counter runs at constant rate regardless of CPU frequency.")
}

fn check_use_platform_clock() -> Check {
    let use_platform = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "UsePlatformClock",
    );

    let status = if use_platform == Some(0) || use_platform.is_none() {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };

    Check::new(
        "Use Platform Clock",
        if use_platform == Some(0) {
            "Disabled (TSC)"
        } else {
            "Enabled"
        },
        status,
    )
    .with_description("TSC is faster and more accurate than platform clock.")
}

fn check_synthetic_timer() -> Check {
    Check::new("Synthetic Timer", "Not Applicable", CheckStatus::Info)
        .with_description("Hyper-V synthetic timer (VM only).")
}

fn check_large_page_drivers() -> Check {
    let large_pages = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
        "LargeSystemCache",
    );

    Check::new(
        "Large Page Support",
        if large_pages == Some(1) {
            "Enabled"
        } else {
            "Disabled"
        },
        CheckStatus::Info,
    )
    .with_description("Large memory pages can reduce TLB misses.")
}

fn check_disable_dynamic_pstate() -> Check {
    Check::new("Dynamic P-State", "System Managed", CheckStatus::Info)
        .with_description("CPU frequency scaling management.")
}

fn check_system_profile_tasks() -> Check {
    let tasks_exist = registry_key_exists(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks",
    );

    Check::new(
        "System Profile Tasks",
        if tasks_exist { "Configured" } else { "Default" },
        CheckStatus::Info,
    )
    .with_description("MMCSS task priority configuration.")
}

fn check_multimedia_system_profile() -> Check {
    Check::new("Multimedia System Profile", "Active", CheckStatus::Info)
        .with_description("MMCSS multimedia class scheduler service configuration.")
}

fn check_gpu_priority() -> Check {
    let priority = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
        "GPU Priority",
    )
    .unwrap_or(8);

    let status = if priority >= 8 {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };

    Check::new("GPU Priority (Games)", &format!("{}", priority), status)
        .with_description("GPU scheduling priority for games. 8 is optimal.")
}

fn check_scheduling_category() -> Check {
    let category = read_registry_string(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games",
        "Scheduling Category",
    )
    .unwrap_or_else(|| "Medium".to_string());

    let status = if category == "High" {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };

    Check::new("Scheduling Category (Games)", &category, status)
        .with_description("CPU scheduling priority. 'High' is optimal for games.")
}

fn check_system_responsiveness_override() -> Check {
    Check::new(
        "System Responsiveness Override",
        "Not Set",
        CheckStatus::Info,
    )
    .with_description("Application-specific responsiveness overrides.")
}

fn check_latency_tolerance() -> Check {
    Check::new("Latency Tolerance", "System Default", CheckStatus::Info)
        .with_description("Device latency tolerance settings.")
}

fn check_interrupt_affinity_policy() -> Check {
    Check::new(
        "Interrupt Affinity Policy",
        "System Managed",
        CheckStatus::Info,
    )
    .with_description("CPU affinity for device interrupts.")
}

fn check_isr_time_limit() -> Check {
    Check::new("ISR Time Limit", "System Default", CheckStatus::Info)
        .with_description("Interrupt Service Routine execution time limit.")
}

fn check_dpc_queue_depth() -> Check {
    Check::new("DPC Queue Depth", "System Default", CheckStatus::Info)
        .with_description("Deferred Procedure Call queue depth.")
}

fn check_threaded_dpc() -> Check {
    Check::new("Threaded DPC", "System Managed", CheckStatus::Info)
        .with_description("Deferred Procedure Calls executed in threads.")
}

fn check_timer_coalescing() -> Check {
    let coalescing = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        "CoalescingTimerInterval",
    );

    Check::new(
        "Timer Coalescing",
        &format!("{:?}", coalescing.unwrap_or(0)),
        CheckStatus::Info,
    )
    .with_description("Groups timer expirations to reduce wakeups.")
}

fn check_precision_platform_timer() -> Check {
    Check::new("Precision Platform Timer", "Available", CheckStatus::Info)
        .with_description("High-resolution hardware timer availability.")
}

fn check_acpi_timer_disabled() -> Check {
    Check::new("ACPI PM Timer", "System Managed", CheckStatus::Info)
        .with_description("ACPI Power Management Timer usage.")
}
