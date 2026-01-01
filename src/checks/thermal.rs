use crate::types::*;
use super::wmi_helper::*;

pub fn run_thermal_checks() -> CategoryResults {
    let mut results = CategoryResults::new("Thermal");
    
    results.add_check(check_cpu_temp());
    results.add_check(check_thermal_zone());
    
    results
}

fn check_cpu_temp() -> Check {
    // Try to query CPU temperature from WMI
    // Note: Most consumer systems don't expose temperature via WMI
    let temp = query_wmi_u32("Win32_TemperatureProbe", "CurrentReading")
        .map(|t| {
            // WMI returns in tenths of Kelvin
            let celsius = (t as f64 / 10.0) - 273.15;
            if celsius > 0.0 && celsius < 150.0 {
                format!("{:.1}°C", celsius)
            } else {
                "Not available".to_string()
            }
        })
        .unwrap_or_else(|| "Not available via WMI".to_string());
    
    let status = if temp.contains("°C") {
        CheckStatus::Info
    } else {
        CheckStatus::Info
    };
    
    Check::new("CPU Temperature", &temp, status)
        .with_description("Most systems require vendor-specific tools for accurate temps")
}

fn check_thermal_zone() -> Check {
    let zone_count = count_wmi_instances("Win32_TemperatureProbe");
    Check::new("Thermal Sensors", &format!("{} detected", zone_count), CheckStatus::Info)
}
