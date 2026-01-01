use super::wmi_helper::*;
use crate::types::*;

pub fn run_thermal_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Thermal");

    results.add_check(check_cpu_temp());
    results.add_check(check_thermal_zone());

    results
}

fn check_cpu_temp() -> Check {
    let temp = query_wmi_u32("Win32_TemperatureProbe", "CurrentReading")
        .map(|t| {
            let celsius = (t as f64 / 10.0) - 273.15;
            if celsius > 0.0 && celsius < 150.0 {
                format!("{:.1}°C", celsius)
            } else {
                "Not available".to_string()
            }
        })
        .unwrap_or_else(|| "Not available via WMI".to_string());

    Check::new("CPU Temperature", &temp, CheckStatus::Info)
        .with_description("Most systems require vendor-specific tools for accurate temps")
}

fn check_thermal_zone() -> Check {
    let zone_count = count_wmi_instances("Win32_TemperatureProbe");
    Check::new(
        "Thermal Sensors",
        &format!("{} detected", zone_count),
        CheckStatus::Info,
    )
}
