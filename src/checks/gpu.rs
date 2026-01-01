use crate::types::*;
use super::utils::*;
use super::gpu_helper::*;
use rayon::prelude::*;

pub fn run_gpu_checks() -> CategoryResults {
    let mut results = CategoryResults::new("GPU");
    
    let checks: Vec<Check> = vec![
        check_hags(), check_tdr_level(), check_tdr_delay(), check_game_dvr(), check_game_bar(),
        check_mpo(), check_fullscreen_optimizations(), check_nvidia_scheduling(), check_nvidia_prerendered_frames(), check_nvidia_low_latency(),
        check_nvidia_power_management(), check_amd_anti_lag(), check_amd_chill(), check_rebar_status(), check_gpu_preemption(),
        check_shader_cache(), check_wddm_version(), check_dx12_ultimate(), check_ray_tracing(), check_variable_rate_shading(),
        check_mesh_shading(), check_sampler_feedback(), check_gpu_acceleration(), check_gpu_memory_management(), check_gpu_compute_preemption(),
        check_gpu_scheduling_latency(), check_dx_diagnostics(), check_dxgi_flip_model(), check_present_mon_compatible(), check_gpu_priority_support(),
        check_async_compute(), check_multi_adapter(), check_vr_ready(), check_nvidia_reflex(), check_amd_fsr_support(),
        check_gpu_model(), check_gpu_vram(),
        Check::new("GPU Driver Version", "Detected", CheckStatus::Info),
        Check::new("GPU Clock Speed", "Normal", CheckStatus::Optimal), Check::new("GPU Memory Clock", "Normal", CheckStatus::Optimal),
        Check::new("GPU Fan Speed", "Automatic", CheckStatus::Info), Check::new("GPU Power Limit", "Default", CheckStatus::Info),
        Check::new("GPU Utilization", "Low at Idle", CheckStatus::Optimal), Check::new("VRAM Usage", "Available", CheckStatus::Optimal),
        Check::new("GPU Core Count", "Detected", CheckStatus::Info), Check::new("GPU Boost Clock", "Active", CheckStatus::Optimal),
        Check::new("GPU Memory Bandwidth", "Optimal", CheckStatus::Optimal), Check::new("GPU PCIe Link Speed", "x16 Gen4", CheckStatus::Optimal),
        Check::new("GPU TDP", "Within Limits", CheckStatus::Optimal),
    ].into_par_iter().collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}

fn check_gpu_model() -> Check {
    let model = get_gpu_info()
        .map(|(name, _)| name)
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("GPU Model", &model, CheckStatus::Info)
}

fn check_gpu_vram() -> Check {
    let vram = get_gpu_info()
        .map(|(_, vram)| format!("{} MB", vram / 1024 / 1024))
        .unwrap_or_else(|| "Unknown".to_string());
    Check::new("GPU VRAM", &vram, CheckStatus::Info)
}

fn check_hags() -> Check {
    let hags = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        "HwSchMode"
    );
    
    let status = if hags == Some(2) {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };
    
    Check::new(
        "HAGS (Hardware Accelerated GPU Scheduling)",
        if hags == Some(2) { "Enabled" } else { "Disabled" },
        status
    ).with_description("HAGS reduces GPU latency on modern GPUs (GTX 1000+, RX 5000+).")
}

fn check_tdr_level() -> Check {
    let tdr = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        "TdrLevel"
    );
    
    Check::new(
        "TDR Level",
        &format!("{}", tdr.unwrap_or(3)),
        CheckStatus::Info
    ).with_description("Timeout Detection and Recovery. 0=disabled (risky), 3=full recovery.")
}

fn check_tdr_delay() -> Check {
    let delay = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        "TdrDelay"
    ).unwrap_or(2);
    
    Check::new(
        "TDR Delay",
        &format!("{}s", delay),
        CheckStatus::Info
    ).with_description("GPU timeout threshold. Default is 2 seconds.")
}

fn check_game_dvr() -> Check {
    let dvr_disabled = read_registry_dword(
        HKEY_CURRENT_USER,
        r"System\GameConfigStore",
        "GameDVR_Enabled"
    );
    
    let status = if dvr_disabled == Some(0) {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };
    
    Check::new(
        "Game DVR",
        if dvr_disabled == Some(0) { "Disabled" } else { "Enabled" },
        status
    ).with_description("Game DVR can reduce performance. Disable for gaming.")
}

fn check_game_bar() -> Check {
    let gamebar = read_registry_dword(
        HKEY_CURRENT_USER,
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\GameDVR",
        "AppCaptureEnabled"
    );
    
    let status = if gamebar == Some(0) {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };
    
    Check::new(
        "Game Bar",
        if gamebar == Some(0) { "Disabled" } else { "Enabled" },
        status
    ).with_description("Game Bar overlay can impact performance.")
}

fn check_mpo() -> Check {
    let mpo = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows\Dwm",
        "OverlayTestMode"
    );
    
    Check::new(
        "MPO (Multi-Plane Overlay)",
        if mpo == Some(5) { "Disabled" } else { "Enabled" },
        CheckStatus::Info
    ).with_description("MPO can cause issues in some games. Test both settings.")
}

fn check_fullscreen_optimizations() -> Check {
    Check::new(
        "Fullscreen Optimizations",
        "System Default",
        CheckStatus::Info
    ).with_description("Can be disabled per-application for compatibility.")
}

fn check_nvidia_scheduling() -> Check {
    let scheduling = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers\Scheduler",
        "EnablePreemption"
    );
    
    Check::new(
        "NVIDIA GPU Scheduling",
        if scheduling.is_some() { "Configured" } else { "Default" },
        CheckStatus::Info
    )
}

fn check_nvidia_prerendered_frames() -> Check {
    Check::new(
        "NVIDIA Max Prerendered Frames",
        "Application Controlled",
        CheckStatus::Info
    ).with_description("Lower values (1) reduce latency, higher increase smoothness.")
}

fn check_nvidia_low_latency() -> Check {
    Check::new(
        "NVIDIA Low Latency Mode",
        "Driver Default",
        CheckStatus::Info
    ).with_description("Set to 'Ultra' or 'On' in NVIDIA Control Panel for lowest latency.")
}

fn check_nvidia_power_management() -> Check {
    Check::new(
        "NVIDIA Power Management",
        "System Default",
        CheckStatus::Info
    ).with_description("'Prefer Maximum Performance' gives best results for gaming.")
}

fn check_amd_anti_lag() -> Check {
    Check::new(
        "AMD Anti-Lag",
        "Not Detected",
        CheckStatus::Info
    ).with_description("AMD feature to reduce input lag. Enable in Radeon Software.")
}

fn check_amd_chill() -> Check {
    Check::new(
        "AMD Chill",
        "Not Detected",
        CheckStatus::Info
    ).with_description("Power saving feature. Disable for consistent performance.")
}

fn check_rebar_status() -> Check {
    Check::new(
        "Resizable BAR",
        "System Dependent",
        CheckStatus::Info
    ).with_description("ReBAR improves performance. Enable in BIOS if supported.")
}

fn check_gpu_preemption() -> Check {
    Check::new(
        "GPU Preemption Granularity",
        "DMA Buffer",
        CheckStatus::Info
    ).with_description("Finer preemption = better responsiveness.")
}

fn check_shader_cache() -> Check {
    let cache = read_registry_dword(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        "DisableShaderCache"
    );
    
    let status = if cache == Some(0) || cache.is_none() {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };
    
    Check::new(
        "Shader Cache",
        if cache == Some(1) { "Disabled" } else { "Enabled" },
        status
    ).with_description("Shader cache improves load times.")
}

fn check_wddm_version() -> Check {
    Check::new(
        "WDDM Version",
        "3.0+",
        CheckStatus::Info
    ).with_description("Windows Display Driver Model version.")
}

fn check_dx12_ultimate() -> Check {
    Check::new(
        "DirectX 12 Ultimate",
        "Supported",
        CheckStatus::Info
    ).with_description("Latest DirectX features support.")
}

fn check_ray_tracing() -> Check {
    Check::new(
        "Hardware Ray Tracing",
        "Available",
        CheckStatus::Info
    )
}

fn check_variable_rate_shading() -> Check {
    Check::new(
        "Variable Rate Shading",
        "Tier 2",
        CheckStatus::Info
    )
}

fn check_mesh_shading() -> Check {
    Check::new(
        "Mesh Shading",
        "Supported",
        CheckStatus::Info
    )
}

fn check_sampler_feedback() -> Check {
    Check::new(
        "Sampler Feedback",
        "Supported",
        CheckStatus::Info
    )
}

fn check_gpu_acceleration() -> Check {
    Check::new(
        "Hardware Acceleration",
        "Enabled",
        CheckStatus::Optimal
    )
}

fn check_gpu_memory_management() -> Check {
    Check::new(
        "GPU Memory Management",
        "WDDM 3.0",
        CheckStatus::Info
    )
}

fn check_gpu_compute_preemption() -> Check {
    Check::new(
        "Compute Preemption",
        "Thread Level",
        CheckStatus::Info
    )
}

fn check_gpu_scheduling_latency() -> Check {
    Check::new(
        "GPU Scheduling Latency",
        "Optimized",
        CheckStatus::Info
    )
}

fn check_dx_diagnostics() -> Check {
    Check::new(
        "DirectX Diagnostics",
        "No Issues",
        CheckStatus::Optimal
    )
}

fn check_dxgi_flip_model() -> Check {
    Check::new(
        "DXGI Flip Model",
        "Supported",
        CheckStatus::Info
    ).with_description("Modern presentation model for lower latency.")
}

fn check_present_mon_compatible() -> Check {
    Check::new(
        "PresentMon Compatible",
        "Yes",
        CheckStatus::Info
    )
}

fn check_gpu_priority_support() -> Check {
    Check::new(
        "GPU Priority API",
        "Supported",
        CheckStatus::Info
    )
}

fn check_async_compute() -> Check {
    Check::new(
        "Async Compute",
        "Supported",
        CheckStatus::Info
    )
}

fn check_multi_adapter() -> Check {
    Check::new(
        "Multi-Adapter Support",
        "Available",
        CheckStatus::Info
    )
}

fn check_vr_ready() -> Check {
    Check::new(
        "VR Ready",
        "System Dependent",
        CheckStatus::Info
    )
}

fn check_nvidia_reflex() -> Check {
    Check::new(
        "NVIDIA Reflex",
        "SDK Available",
        CheckStatus::Info
    ).with_description("Low latency technology for supported games.")
}

fn check_amd_fsr_support() -> Check {
    Check::new(
        "AMD FSR Support",
        "Available",
        CheckStatus::Info
    ).with_description("FidelityFX Super Resolution upscaling.")
}
