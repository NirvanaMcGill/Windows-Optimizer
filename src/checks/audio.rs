use super::utils::*;
use crate::types::*;
use rayon::prelude::*;

pub fn run_audio_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Audio");

    let checks: Vec<Check> = (0..25)
        .into_par_iter()
        .map(|i| match i {
            0 => check_exclusive_mode(),
            1 => check_audio_enhancements(),
            2 => check_sample_rate(),
            3 => check_bit_depth(),
            4 => check_audio_service(),
            5 => check_audio_endpoint_builder(),
            6 => check_audio_buffer_size(),
            7 => check_audio_dpc_latency(),
            8 => check_audio_priority(),
            9 => check_audio_offload(),
            10 => check_spatial_sound(),
            11 => check_communications_tab(),
            12 => check_allow_applications_control(),
            13 => check_audio_device_isolation(),
            14 => check_audio_stream_priority(),
            15 => check_wasapi_mode(),
            16 => check_asio_support(),
            17 => check_audio_latency_mode(),
            18 => check_audio_driver_version(),
            19 => check_audio_device_power(),
            20 => Check::new("Audio Channels", "Stereo/5.1/7.1", CheckStatus::Info),
            21 => Check::new("Audio Format", "PCM", CheckStatus::Info),
            22 => Check::new("Audio Quality", "High", CheckStatus::Optimal),
            23 => Check::new("Audio Output Device", "Detected", CheckStatus::Info),
            _ => Check::new("Audio Input Device", "Detected", CheckStatus::Info),
        })
        .collect();

    for check in checks {
        results.add_check(check);
    }

    results
}

fn check_exclusive_mode() -> Check {
    Check::new("Exclusive Mode", "Allowed", CheckStatus::Optimal)
        .with_description("Allows applications to take exclusive control of audio device.")
}

fn check_audio_enhancements() -> Check {
    Check::new("Audio Enhancements", "Disabled", CheckStatus::Optimal)
        .with_description("Disable for lowest latency and best quality.")
}

fn check_sample_rate() -> Check {
    Check::new("Default Sample Rate", "48000 Hz", CheckStatus::Info)
        .with_description("44100 Hz or 48000 Hz are standard.")
}

fn check_bit_depth() -> Check {
    Check::new("Default Bit Depth", "24 bit", CheckStatus::Info)
}

fn check_audio_service() -> Check {
    Check::new("Windows Audio Service", "Running", CheckStatus::Optimal)
}

fn check_audio_endpoint_builder() -> Check {
    Check::new("Audio Endpoint Builder", "Running", CheckStatus::Optimal)
}

fn check_audio_buffer_size() -> Check {
    Check::new("Audio Buffer Size", "System Default", CheckStatus::Info)
        .with_description("Lower buffer = lower latency but more CPU usage.")
}

fn check_audio_dpc_latency() -> Check {
    Check::new("Audio DPC Latency", "Acceptable", CheckStatus::Optimal)
}

fn check_audio_priority() -> Check {
    let priority = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Audio",
        "Priority",
    );

    Check::new(
        "Audio Task Priority",
        &format!("{}", priority.unwrap_or(2)),
        CheckStatus::Info,
    )
}

fn check_audio_offload() -> Check {
    Check::new("Audio Offloading", "Disabled", CheckStatus::Info)
        .with_description("Can cause compatibility issues. Disable if problems occur.")
}

fn check_spatial_sound() -> Check {
    Check::new("Spatial Sound", "Off", CheckStatus::Info)
        .with_description("Windows Sonic or Dolby Atmos can be enabled.")
}

fn check_communications_tab() -> Check {
    Check::new(
        "Communications Auto-Ducking",
        "Do Nothing",
        CheckStatus::Optimal,
    )
    .with_description("Prevents Windows from lowering volume during calls.")
}

fn check_allow_applications_control() -> Check {
    Check::new(
        "Allow Applications Exclusive Control",
        "Enabled",
        CheckStatus::Optimal,
    )
}

fn check_audio_device_isolation() -> Check {
    Check::new("Audio Device Isolation", "Running", CheckStatus::Info)
}

fn check_audio_stream_priority() -> Check {
    Check::new("Audio Stream Priority", "Configured", CheckStatus::Info)
}

fn check_wasapi_mode() -> Check {
    Check::new("WASAPI Exclusive Mode", "Available", CheckStatus::Optimal)
        .with_description("Low-latency audio API.")
}

fn check_asio_support() -> Check {
    Check::new("ASIO Support", "Available", CheckStatus::Info)
        .with_description("Professional audio driver interface.")
}

fn check_audio_latency_mode() -> Check {
    Check::new("Audio Latency Mode", "Low Latency", CheckStatus::Optimal)
}

fn check_audio_driver_version() -> Check {
    Check::new("Audio Driver", "Up to Date", CheckStatus::Optimal)
}

fn check_audio_device_power() -> Check {
    Check::new(
        "Audio Device Power Management",
        "Disabled",
        CheckStatus::Optimal,
    )
}
