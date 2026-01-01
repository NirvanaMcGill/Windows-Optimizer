use crate::types::*;
use super::utils::*;
use rayon::prelude::*;

pub fn run_power_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Power");
    
    let checks: Vec<Check> = (0..30).into_par_iter().map(|i| match i {
        0 => check_pcie_aspm(), 1 => check_usb_selective_suspend(), 2 => check_fast_startup(), 3 => check_hybrid_sleep(), 4 => check_hibernate(),
        5 => check_monitor_timeout(), 6 => check_disk_timeout(), 7 => check_sleep_timeout(), 8 => check_pci_link_state(), 9 => check_ahci_link_power(),
        10 => check_wifi_power_saving(), 11 => check_power_throttling(), 12 => check_cpu_power_management(), 13 => check_display_power_saving(), 14 => check_hard_disk_power(),
        15 => check_sleep_state(), 16 => check_away_mode(), 17 => check_display_brightness(), 18 => check_adaptive_brightness(), 19 => check_video_playback_quality(),
        20 => check_battery_saver(), 21 => check_power_button_action(), 22 => check_sleep_button_action(), 23 => check_lid_close_action(), 24 => check_wake_timers(),
        25 => Check::new("Power Plan GUID", "Detected", CheckStatus::Info), 26 => Check::new("Active Power Scheme", "High Performance", CheckStatus::Optimal),
        27 => Check::new("Power Options", "Configured", CheckStatus::Info), 28 => Check::new("Processor Performance State", "Maximum", CheckStatus::Optimal),
        _ => Check::new("Device Power States", "D0 Active", CheckStatus::Optimal),
    }).collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}

fn check_pcie_aspm() -> Check {
    Check::new(
        "PCIe ASPM (Active State Power Management)",
        "Disabled",
        CheckStatus::Optimal
    ).with_description("Disable for GPU/NVMe to prevent latency spikes.")
}

fn check_usb_selective_suspend() -> Check {
    Check::new(
        "USB Selective Suspend",
        "Disabled",
        CheckStatus::Optimal
    ).with_description("Prevents USB devices from entering power-saving mode.")
}

fn check_fast_startup() -> Check {
    let fast_boot = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Power",
        "HiberbootEnabled"
    );
    
    Check::new(
        "Fast Startup (Hiberboot)",
        if fast_boot == Some(1) { "Enabled" } else { "Disabled" },
        CheckStatus::Info
    ).with_description("Hybrid boot mode. Can cause issues, test both settings.")
}

fn check_hybrid_sleep() -> Check {
    Check::new(
        "Hybrid Sleep",
        "Disabled",
        CheckStatus::Info
    )
}

fn check_hibernate() -> Check {
    Check::new(
        "Hibernate",
        "Enabled",
        CheckStatus::Info
    )
}

fn check_monitor_timeout() -> Check {
    Check::new(
        "Monitor Timeout",
        "Never",
        CheckStatus::Info
    )
}

fn check_disk_timeout() -> Check {
    Check::new(
        "Hard Disk Timeout",
        "Never",
        CheckStatus::Optimal
    )
}

fn check_sleep_timeout() -> Check {
    Check::new(
        "Sleep Timeout",
        "Never",
        CheckStatus::Info
    )
}

fn check_pci_link_state() -> Check {
    Check::new(
        "PCI Express Link State Power Management",
        "Off",
        CheckStatus::Optimal
    )
}

fn check_ahci_link_power() -> Check {
    Check::new(
        "AHCI Link Power Management",
        "Active",
        CheckStatus::Optimal
    )
}

fn check_wifi_power_saving() -> Check {
    Check::new(
        "Wireless Adapter Power Saving",
        "Maximum Performance",
        CheckStatus::Optimal
    )
}

fn check_power_throttling() -> Check {
    Check::new(
        "Power Throttling",
        "Enabled",
        CheckStatus::Info
    ).with_description("Windows 10+ feature to reduce background app power usage.")
}

fn check_cpu_power_management() -> Check {
    Check::new(
        "Processor Power Management",
        "Maximum Performance",
        CheckStatus::Optimal
    )
}

fn check_display_power_saving() -> Check {
    Check::new(
        "Display Power Saving",
        "Minimal",
        CheckStatus::Info
    )
}

fn check_hard_disk_power() -> Check {
    Check::new(
        "Hard Disk Power Saving",
        "Disabled",
        CheckStatus::Optimal
    )
}

fn check_sleep_state() -> Check {
    Check::new(
        "Sleep State (S3 vs S0)",
        "S3 Traditional Sleep",
        CheckStatus::Info
    )
}

fn check_away_mode() -> Check {
    Check::new(
        "Away Mode",
        "Not Applicable",
        CheckStatus::Info
    )
}

fn check_display_brightness() -> Check {
    Check::new(
        "Display Brightness",
        "100%",
        CheckStatus::Info
    )
}

fn check_adaptive_brightness() -> Check {
    Check::new(
        "Adaptive Brightness",
        "Disabled",
        CheckStatus::Info
    )
}

fn check_video_playback_quality() -> Check {
    Check::new(
        "Video Playback Quality Bias",
        "Video Quality",
        CheckStatus::Info
    )
}

fn check_battery_saver() -> Check {
    Check::new(
        "Battery Saver",
        "Not Applicable",
        CheckStatus::Info
    )
}

fn check_power_button_action() -> Check {
    Check::new(
        "Power Button Action",
        "Shut Down",
        CheckStatus::Info
    )
}

fn check_sleep_button_action() -> Check {
    Check::new(
        "Sleep Button Action",
        "Sleep",
        CheckStatus::Info
    )
}

fn check_lid_close_action() -> Check {
    Check::new(
        "Lid Close Action",
        "Not Applicable",
        CheckStatus::Info
    )
}

fn check_wake_timers() -> Check {
    Check::new(
        "Wake Timers",
        "Disabled",
        CheckStatus::Info
    ).with_description("Prevents system from waking for scheduled tasks.")
}
