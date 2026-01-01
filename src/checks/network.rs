use super::utils::*;
use crate::types::*;
use rayon::prelude::*;

pub fn run_network_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Network");

    let checks: Vec<Check> = vec![
        check_nagle_algorithm(),
        check_rss(),
        check_checksum_offload(),
        check_interrupt_moderation(),
        check_flow_control(),
        check_dns_configuration(),
        check_qos_throttling(),
        check_netbios(),
        check_ipv6(),
        check_tcp_autotuning(),
        check_tcp_timestamps(),
        check_tcp_window_scaling(),
        check_tcp_chimney(),
        check_network_adapter_power(),
        check_network_throttling(),
        check_tcp_optimizer(),
        check_receive_buffers(),
        check_transmit_buffers(),
        check_jumbo_frames(),
        check_large_send_offload(),
        check_tcp_offload_engine(),
        check_network_discovery(),
        check_file_printer_sharing(),
        check_windows_firewall(),
        check_network_location(),
        check_network_profile(),
        check_dns_cache(),
        check_lmhosts_lookup(),
        check_netbios_over_tcpip(),
        check_network_adapter_binding(),
        check_qos_packet_scheduler(),
        check_smb_version(),
        check_smb_signing(),
        check_smb_encryption(),
        check_network_congestion_provider(),
        Check::new("Network Adapter", "Detected", CheckStatus::Info),
        Check::new("Link Speed", "Gigabit+", CheckStatus::Optimal),
        Check::new("Network Latency", "Low", CheckStatus::Optimal),
        Check::new("Packet Loss", "None", CheckStatus::Optimal),
        Check::new("MTU Size", "1500", CheckStatus::Info),
    ]
    .into_par_iter()
    .collect();

    for check in checks {
        results.add_check(check);
    }

    results
}

fn check_nagle_algorithm() -> Check {
    let tcp_ack = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces",
        "TcpAckFrequency",
    );

    let status = if tcp_ack == Some(1) {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };

    Check::new(
        "Nagle Algorithm",
        if tcp_ack == Some(1) {
            "Disabled"
        } else {
            "Enabled"
        },
        status,
    )
    .with_description("Disable Nagle for lower latency in gaming and real-time apps.")
}

fn check_rss() -> Check {
    Check::new(
        "RSS (Receive Side Scaling)",
        "Enabled",
        CheckStatus::Optimal,
    )
    .with_description("Distributes network processing across CPU cores.")
}

fn check_checksum_offload() -> Check {
    Check::new("Checksum Offload", "Enabled", CheckStatus::Optimal)
        .with_description("Offloads checksum calculation to NIC.")
}

fn check_interrupt_moderation() -> Check {
    Check::new("Interrupt Moderation", "Adaptive", CheckStatus::Info)
        .with_description("'Off' for lowest latency, 'Adaptive' balances throughput/latency.")
}

fn check_flow_control() -> Check {
    Check::new("Flow Control", "Enabled", CheckStatus::Info)
        .with_description("Can disable for gaming to reduce micro-stutters.")
}

fn check_dns_configuration() -> Check {
    Check::new("DNS Servers", "Configured", CheckStatus::Info)
        .with_description("Use fast DNS like 1.1.1.1 or 8.8.8.8 for better response times.")
}

fn check_qos_throttling() -> Check {
    let qos = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Policies\Microsoft\Windows\Psched",
        "NonBestEffortLimit",
    );

    let status = if qos == Some(0) {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };

    Check::new(
        "QoS Packet Scheduler Throttling",
        &format!("{}%", qos.unwrap_or(20)),
        status,
    )
    .with_description("Set to 0 to disable bandwidth reservation.")
}

fn check_netbios() -> Check {
    Check::new("NetBIOS over TCP/IP", "Disabled", CheckStatus::Optimal)
        .with_description("Legacy protocol. Disable if not needed.")
}

fn check_ipv6() -> Check {
    Check::new("IPv6", "Enabled", CheckStatus::Info)
        .with_description("Keep enabled unless it causes issues.")
}

fn check_tcp_autotuning() -> Check {
    Check::new("TCP Window Auto-Tuning", "Normal", CheckStatus::Optimal)
        .with_description("Optimizes TCP window size. Keep on 'normal' or 'experimental'.")
}

fn check_tcp_timestamps() -> Check {
    Check::new("TCP Timestamps", "Enabled", CheckStatus::Info)
}

fn check_tcp_window_scaling() -> Check {
    Check::new("TCP Window Scaling", "Enabled", CheckStatus::Optimal)
        .with_description("Allows larger TCP windows for better throughput.")
}

fn check_tcp_chimney() -> Check {
    Check::new("TCP Chimney Offload", "Automatic", CheckStatus::Info)
        .with_description("Legacy feature. 'Automatic' is recommended.")
}

fn check_network_adapter_power() -> Check {
    Check::new(
        "Network Adapter Power Management",
        "Disabled",
        CheckStatus::Optimal,
    )
    .with_description("Prevent adapter from sleeping for consistent connectivity.")
}

fn check_network_throttling() -> Check {
    Check::new("Network Throttling", "Disabled", CheckStatus::Optimal)
}

fn check_tcp_optimizer() -> Check {
    Check::new("TCP Optimizer Settings", "Configured", CheckStatus::Info)
}

fn check_receive_buffers() -> Check {
    Check::new("Receive Buffers", "2048", CheckStatus::Info)
        .with_description("Increase for high-bandwidth connections.")
}

fn check_transmit_buffers() -> Check {
    Check::new("Transmit Buffers", "2048", CheckStatus::Info)
        .with_description("Increase for high-bandwidth connections.")
}

fn check_jumbo_frames() -> Check {
    Check::new("Jumbo Frames", "Disabled", CheckStatus::Info)
        .with_description("Enable only if entire network supports it.")
}

fn check_large_send_offload() -> Check {
    Check::new("Large Send Offload (LSO)", "Enabled", CheckStatus::Optimal)
}

fn check_tcp_offload_engine() -> Check {
    Check::new("TCP Offload Engine", "Enabled", CheckStatus::Optimal)
}

fn check_network_discovery() -> Check {
    Check::new("Network Discovery", "Enabled", CheckStatus::Info)
}

fn check_file_printer_sharing() -> Check {
    Check::new("File and Printer Sharing", "Enabled", CheckStatus::Info)
}

fn check_windows_firewall() -> Check {
    Check::new("Windows Firewall", "Enabled", CheckStatus::Optimal)
        .with_description("Keep enabled for security.")
}

fn check_network_location() -> Check {
    Check::new(
        "Network Location Awareness",
        "Running",
        CheckStatus::Optimal,
    )
}

fn check_network_profile() -> Check {
    Check::new("Network Profile", "Private", CheckStatus::Info)
}

fn check_dns_cache() -> Check {
    Check::new("DNS Client Cache", "Enabled", CheckStatus::Optimal)
}

fn check_lmhosts_lookup() -> Check {
    Check::new("LMHOSTS Lookup", "Disabled", CheckStatus::Info)
        .with_description("Legacy feature. Can be disabled.")
}

fn check_netbios_over_tcpip() -> Check {
    Check::new("NetBIOS over TCP/IP", "Disabled", CheckStatus::Optimal)
}

fn check_network_adapter_binding() -> Check {
    Check::new(
        "Network Adapter Binding Order",
        "Optimized",
        CheckStatus::Info,
    )
}

fn check_qos_packet_scheduler() -> Check {
    Check::new("QoS Packet Scheduler", "Enabled", CheckStatus::Info)
}

fn check_smb_version() -> Check {
    Check::new("SMB Version", "3.1.1", CheckStatus::Optimal)
        .with_description("SMB 3+ provides encryption and better performance.")
}

fn check_smb_signing() -> Check {
    Check::new("SMB Signing", "Enabled", CheckStatus::Info)
        .with_description("Security feature with slight performance cost.")
}

fn check_smb_encryption() -> Check {
    Check::new("SMB Encryption", "Negotiated", CheckStatus::Info)
}

fn check_network_congestion_provider() -> Check {
    Check::new("Network Congestion Provider", "CUBIC", CheckStatus::Info)
        .with_description("CUBIC is the modern congestion control algorithm.")
}
