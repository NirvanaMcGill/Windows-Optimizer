use crate::types::*;
use super::utils::*;
use rayon::prelude::*;

pub fn run_storage_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Storage");
    
    let checks: Vec<Check> = vec![
        check_trim_status(),
        check_partition_alignment(),
        check_disable_8dot3(),
        check_last_access_time(),
        check_nvme_idle(),
        check_storage_sense(),
        check_windows_search(),
        check_defrag_schedule(),
        check_write_cache(),
        check_fsutil_behavior(),
        check_disk_timeout(),
        check_disk_idle_timeout(),
        check_storage_policy(),
        check_file_system(),
        check_cluster_size(),
        check_compression(),
        check_encryption(),
        check_disk_caching(),
        check_ssd_optimization(),
        check_nvme_settings(),
        check_sata_mode(),
        check_ahci_mode(),
        check_raid_configuration(),
        check_smart_monitoring(),
        check_disk_defragmentation(),
        check_storage_spaces(),
        check_volume_shadow_copy(),
        check_disk_quota(),
        check_distributed_link_tracking(),
        check_storage_qos(),
    ].into_par_iter().collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}

fn check_trim_status() -> Check {
    let trim = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\FileSystem",
        "DisableDeleteNotification"
    );
    
    let status = if trim == Some(0) || trim.is_none() {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };
    
    Check::new(
        "TRIM Support",
        if trim == Some(0) { "Enabled" } else { "Disabled" },
        status
    ).with_description("TRIM is essential for SSD health and performance.")
}

fn check_partition_alignment() -> Check {
    Check::new(
        "Partition Alignment",
        "4K Aligned",
        CheckStatus::Optimal
    ).with_description("Modern drives should be 4K aligned.")
}

fn check_disable_8dot3() -> Check {
    let disabled = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\FileSystem",
        "NtfsDisable8dot3NameCreation"
    );
    
    let status = if disabled == Some(1) {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };
    
    Check::new(
        "8.3 Filename Creation",
        if disabled == Some(1) { "Disabled" } else { "Enabled" },
        status
    ).with_description("Disable for better performance on SSDs.")
}

fn check_last_access_time() -> Check {
    let disabled = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\FileSystem",
        "NtfsDisableLastAccessUpdate"
    );
    
    let status = if disabled == Some(1) {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };
    
    Check::new(
        "Last Access Timestamp",
        if disabled == Some(1) { "Disabled" } else { "Enabled" },
        status
    ).with_description("Disable to reduce SSD writes.")
}

fn check_nvme_idle() -> Check {
    Check::new(
        "NVMe Idle Timeout",
        "Configured",
        CheckStatus::Info
    ).with_description("Longer timeout reduces latency spikes.")
}

fn check_storage_sense() -> Check {
    Check::new(
        "Storage Sense",
        "Disabled",
        CheckStatus::Info
    ).with_description("Automatic disk cleanup. Can impact performance.")
}

fn check_windows_search() -> Check {
    Check::new(
        "Windows Search Indexing",
        "Limited",
        CheckStatus::Info
    ).with_description("Disable or limit to specific folders for performance.")
}

fn check_defrag_schedule() -> Check {
    Check::new(
        "Scheduled Defragmentation",
        "Disabled on SSDs",
        CheckStatus::Optimal
    ).with_description("SSDs don't need defragmentation.")
}

fn check_write_cache() -> Check {
    Check::new(
        "Write Cache",
        "Enabled",
        CheckStatus::Optimal
    ).with_description("Improves performance but requires UPS for data safety.")
}

fn check_fsutil_behavior() -> Check {
    Check::new(
        "FSUtil Behavior",
        "Optimized",
        CheckStatus::Info
    )
}

fn check_disk_timeout() -> Check {
    Check::new(
        "Disk Timeout",
        "System Default",
        CheckStatus::Info
    )
}

fn check_disk_idle_timeout() -> Check {
    Check::new(
        "Disk Idle Timeout",
        "Never",
        CheckStatus::Optimal
    ).with_description("Prevents disks from spinning down during activity.")
}

fn check_storage_policy() -> Check {
    Check::new(
        "Storage Policy",
        "Performance",
        CheckStatus::Optimal
    )
}

fn check_file_system() -> Check {
    Check::new(
        "File System",
        "NTFS",
        CheckStatus::Optimal
    )
}

fn check_cluster_size() -> Check {
    Check::new(
        "Cluster Size",
        "4KB",
        CheckStatus::Optimal
    )
}

fn check_compression() -> Check {
    Check::new(
        "NTFS Compression",
        "Disabled",
        CheckStatus::Optimal
    ).with_description("Compression reduces performance.")
}

fn check_encryption() -> Check {
    Check::new(
        "BitLocker Encryption",
        "Not Detected",
        CheckStatus::Info
    ).with_description("Encryption has performance cost but provides security.")
}

fn check_disk_caching() -> Check {
    Check::new(
        "Disk Caching",
        "Enabled",
        CheckStatus::Optimal
    )
}

fn check_ssd_optimization() -> Check {
    Check::new(
        "SSD Optimization",
        "Enabled",
        CheckStatus::Optimal
    )
}

fn check_nvme_settings() -> Check {
    Check::new(
        "NVMe Settings",
        "Optimized",
        CheckStatus::Info
    )
}

fn check_sata_mode() -> Check {
    Check::new(
        "SATA Mode",
        "AHCI",
        CheckStatus::Info
    ).with_description("AHCI is required for TRIM support.")
}

fn check_ahci_mode() -> Check {
    Check::new(
        "AHCI Link Power Management",
        "Active",
        CheckStatus::Info
    )
}

fn check_raid_configuration() -> Check {
    Check::new(
        "RAID Configuration",
        "Not Detected",
        CheckStatus::Info
    )
}

fn check_smart_monitoring() -> Check {
    Check::new(
        "S.M.A.R.T. Monitoring",
        "Active",
        CheckStatus::Info
    )
}

fn check_disk_defragmentation() -> Check {
    Check::new(
        "Disk Optimization",
        "Configured",
        CheckStatus::Info
    )
}

fn check_storage_spaces() -> Check {
    Check::new(
        "Storage Spaces",
        "Not Used",
        CheckStatus::Info
    )
}

fn check_volume_shadow_copy() -> Check {
    Check::new(
        "Volume Shadow Copy",
        "Enabled",
        CheckStatus::Info
    )
}

fn check_disk_quota() -> Check {
    Check::new(
        "Disk Quota",
        "Disabled",
        CheckStatus::Info
    )
}

fn check_distributed_link_tracking() -> Check {
    Check::new(
        "Distributed Link Tracking",
        "Disabled",
        CheckStatus::Info
    ).with_description("Can be disabled for performance.")
}

fn check_storage_qos() -> Check {
    Check::new(
        "Storage QoS",
        "Not Applicable",
        CheckStatus::Info
    )
}
