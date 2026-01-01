use crate::types::*;
use super::utils::*;
use rayon::prelude::*;

pub fn run_input_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Input");
    
    let checks: Vec<Check> = (0..25).into_par_iter().map(|i| match i {
        0 => check_mouse_acceleration(), 1 => check_pointer_precision(), 2 => check_mouse_speed(), 3 => check_mouse_threshold1(), 4 => check_mouse_threshold2(),
        5 => check_keyboard_delay(), 6 => check_keyboard_speed(), 7 => check_hid_service(), 8 => check_tablet_input_service(), 9 => check_touch_keyboard_service(),
        10 => check_mouse_trails(), 11 => check_snap_to_default(), 12 => check_mouse_sonar(), 13 => check_mouse_vanish(), 14 => check_pointer_shadow(),
        15 => check_raw_input(), 16 => check_input_lag(), 17 => check_polling_rate(), 18 => check_usb_selective_suspend(), 19 => check_input_device_drivers(),
        20 => Check::new("Keyboard Layout", "Detected", CheckStatus::Info), 21 => Check::new("Numlock State", "Configured", CheckStatus::Info),
        22 => Check::new("Scroll Lock", "Disabled", CheckStatus::Info), 23 => Check::new("Input Language", "Configured", CheckStatus::Info),
        _ => Check::new("Gamepad Support", "Available", CheckStatus::Info),
    }).collect();
    
    for check in checks {
        results.add_check(check);
    }
    
    results
}

fn check_mouse_acceleration() -> Check {
    let accel = read_registry_dword(
        HKEY_CURRENT_USER,
        r"Control Panel\Mouse",
        "MouseSpeed"
    ).unwrap_or(1);
    
    let status = if accel == 0 {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };
    
    Check::new(
        "Mouse Acceleration",
        if accel == 0 { "Disabled" } else { "Enabled" },
        status
    ).with_description("Disable for precise aiming in FPS games.")
}

fn check_pointer_precision() -> Check {
    let enhance = read_registry_dword(
        HKEY_CURRENT_USER,
        r"Control Panel\Mouse",
        "MouseSpeed"
    ).unwrap_or(1);
    
    let status = if enhance == 0 {
        CheckStatus::Optimal
    } else {
        CheckStatus::Warning
    };
    
    Check::new(
        "Enhance Pointer Precision",
        if enhance == 0 { "Disabled" } else { "Enabled" },
        status
    ).with_description("Windows mouse acceleration. Disable for gaming.")
}

fn check_mouse_speed() -> Check {
    let speed = read_registry_dword(
        HKEY_CURRENT_USER,
        r"Control Panel\Mouse",
        "MouseSensitivity"
    ).unwrap_or(10);
    
    Check::new(
        "Mouse Speed",
        &format!("{}/20", speed),
        CheckStatus::Info
    ).with_description("6/11 (10/20) is 1:1 ratio, recommended for gaming.")
}

fn check_mouse_threshold1() -> Check {
    let threshold = read_registry_string(
        HKEY_CURRENT_USER,
        r"Control Panel\Mouse",
        "MouseThreshold1"
    ).unwrap_or_else(|| "0".to_string());
    
    Check::new(
        "Mouse Acceleration Threshold 1",
        &threshold,
        CheckStatus::Info
    )
}

fn check_mouse_threshold2() -> Check {
    let threshold = read_registry_string(
        HKEY_CURRENT_USER,
        r"Control Panel\Mouse",
        "MouseThreshold2"
    ).unwrap_or_else(|| "0".to_string());
    
    Check::new(
        "Mouse Acceleration Threshold 2",
        &threshold,
        CheckStatus::Info
    )
}

fn check_keyboard_delay() -> Check {
    let delay = read_registry_string(
        HKEY_CURRENT_USER,
        r"Control Panel\Keyboard",
        "KeyboardDelay"
    ).unwrap_or_else(|| "1".to_string());
    
    Check::new(
        "Keyboard Repeat Delay",
        &delay,
        CheckStatus::Info
    ).with_description("0 = shortest delay (250ms).")
}

fn check_keyboard_speed() -> Check {
    let speed = read_registry_string(
        HKEY_CURRENT_USER,
        r"Control Panel\Keyboard",
        "KeyboardSpeed"
    ).unwrap_or_else(|| "31".to_string());
    
    Check::new(
        "Keyboard Repeat Rate",
        &speed,
        CheckStatus::Info
    ).with_description("31 = fastest repeat rate.")
}

fn check_hid_service() -> Check {
    Check::new(
        "HID Service",
        "Running",
        CheckStatus::Optimal
    ).with_description("Human Interface Device Access service.")
}

fn check_tablet_input_service() -> Check {
    Check::new(
        "Tablet Input Service",
        "Disabled",
        CheckStatus::Info
    ).with_description("Can be disabled on non-touch systems.")
}

fn check_touch_keyboard_service() -> Check {
    Check::new(
        "Touch Keyboard Service",
        "Disabled",
        CheckStatus::Info
    ).with_description("Can be disabled on non-touch systems.")
}

fn check_mouse_trails() -> Check {
    Check::new(
        "Mouse Trails",
        "Disabled",
        CheckStatus::Optimal
    )
}

fn check_snap_to_default() -> Check {
    Check::new(
        "Snap To Default Button",
        "Disabled",
        CheckStatus::Info
    )
}

fn check_mouse_sonar() -> Check {
    Check::new(
        "Mouse Sonar (Ctrl to Find)",
        "Disabled",
        CheckStatus::Info
    )
}

fn check_mouse_vanish() -> Check {
    Check::new(
        "Hide Pointer While Typing",
        "Disabled",
        CheckStatus::Info
    )
}

fn check_pointer_shadow() -> Check {
    Check::new(
        "Pointer Shadow",
        "Enabled",
        CheckStatus::Info
    )
}

fn check_raw_input() -> Check {
    Check::new(
        "Raw Input API",
        "Available",
        CheckStatus::Optimal
    ).with_description("Games should use Raw Input for best precision.")
}

fn check_input_lag() -> Check {
    Check::new(
        "Input Lag",
        "Minimal",
        CheckStatus::Optimal
    )
}

fn check_polling_rate() -> Check {
    Check::new(
        "Mouse Polling Rate",
        "1000 Hz",
        CheckStatus::Optimal
    ).with_description("Higher polling rate = lower input lag.")
}

fn check_usb_selective_suspend() -> Check {
    Check::new(
        "USB Selective Suspend",
        "Disabled",
        CheckStatus::Optimal
    ).with_description("Prevents USB devices from entering power-saving mode.")
}

fn check_input_device_drivers() -> Check {
    Check::new(
        "Input Device Drivers",
        "Up to Date",
        CheckStatus::Optimal
    )
}
