#[cfg(windows)]
fn sanitize_wmi_identifier(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

#[cfg(windows)]
pub fn query_wmi_u32(class: &str, property: &str) -> Option<u32> {
    use wmi::{COMLibrary, WMIConnection};
    
    if !sanitize_wmi_identifier(class) || !sanitize_wmi_identifier(property) {
        return None;
    }
    
    let com_con = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com_con).ok()?;
    
    let results: Vec<std::collections::HashMap<String, wmi::Variant>> = 
        wmi_con.raw_query(&format!("SELECT {} FROM {}", property, class)).ok()?;
    
    results.first().and_then(|r| {
        r.get(property).and_then(|v| match v {
            wmi::Variant::UI4(n) => Some(*n),
            wmi::Variant::I4(n) => Some(*n as u32),
            wmi::Variant::UI2(n) => Some(*n as u32),
            wmi::Variant::UI1(n) => Some(*n as u32),
            _ => None,
        })
    })
}

#[cfg(windows)]
pub fn query_wmi_u64(class: &str, property: &str) -> Option<u64> {
    use wmi::{COMLibrary, WMIConnection};
    
    if !sanitize_wmi_identifier(class) || !sanitize_wmi_identifier(property) {
        return None;
    }
    
    let com_con = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com_con).ok()?;
    
    let results: Vec<std::collections::HashMap<String, wmi::Variant>> = 
        wmi_con.raw_query(&format!("SELECT {} FROM {}", property, class)).ok()?;
    
    results.first().and_then(|r| {
        r.get(property).and_then(|v| match v {
            wmi::Variant::UI8(n) => Some(*n),
            wmi::Variant::I8(n) => Some(*n as u64),
            wmi::Variant::UI4(n) => Some(*n as u64),
            _ => None,
        })
    })
}

#[cfg(windows)]
pub fn query_wmi_string(class: &str, property: &str) -> Option<String> {
    use wmi::{COMLibrary, WMIConnection};
    
    if !sanitize_wmi_identifier(class) || !sanitize_wmi_identifier(property) {
        return None;
    }
    
    let com_con = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com_con).ok()?;
    
    let results: Vec<std::collections::HashMap<String, wmi::Variant>> = 
        wmi_con.raw_query(&format!("SELECT {} FROM {}", property, class)).ok()?;
    
    results.first().and_then(|r| {
        r.get(property).and_then(|v| match v {
            wmi::Variant::String(s) => Some(s.clone()),
            _ => None,
        })
    })
}

#[cfg(windows)]
pub fn count_wmi_instances(class: &str) -> usize {
    use wmi::{COMLibrary, WMIConnection};
    
    if !sanitize_wmi_identifier(class) {
        return 0;
    }
    
    let com_con = COMLibrary::new().ok();
    let wmi_con = com_con.and_then(|c| WMIConnection::new(c).ok());
    
    wmi_con.and_then(|w| {
        let results: Result<Vec<std::collections::HashMap<String, wmi::Variant>>, _> = 
            w.raw_query(&format!("SELECT * FROM {}", class));
        results.ok().map(|r| r.len())
    }).unwrap_or(0)
}

#[cfg(windows)]
pub fn query_cpu_info() -> Option<CpuInfo> {
    use wmi::{COMLibrary, WMIConnection};
    
    let com_con = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com_con).ok()?;
    
    let results: Vec<std::collections::HashMap<String, wmi::Variant>> = 
        wmi_con.raw_query("SELECT Name,NumberOfCores,NumberOfLogicalProcessors,MaxClockSpeed,CurrentClockSpeed,L2CacheSize,L3CacheSize,Architecture FROM Win32_Processor").ok()?;
    
    let first = results.first()?;
    
    Some(CpuInfo {
        name: first.get("Name").and_then(|v| if let wmi::Variant::String(s) = v { Some(s.clone()) } else { None }).unwrap_or_default(),
        cores: first.get("NumberOfCores").and_then(|v| if let wmi::Variant::UI4(n) = v { Some(*n) } else { None }).unwrap_or(0),
        logical: first.get("NumberOfLogicalProcessors").and_then(|v| if let wmi::Variant::UI4(n) = v { Some(*n) } else { None }).unwrap_or(0),
        max_clock: first.get("MaxClockSpeed").and_then(|v| if let wmi::Variant::UI4(n) = v { Some(*n) } else { None }).unwrap_or(0),
        current_clock: first.get("CurrentClockSpeed").and_then(|v| if let wmi::Variant::UI4(n) = v { Some(*n) } else { None }).unwrap_or(0),
        l2_cache: first.get("L2CacheSize").and_then(|v| if let wmi::Variant::UI4(n) = v { Some(*n) } else { None }).unwrap_or(0),
        l3_cache: first.get("L3CacheSize").and_then(|v| if let wmi::Variant::UI4(n) = v { Some(*n) } else { None }).unwrap_or(0),
        architecture: first.get("Architecture").and_then(|v| if let wmi::Variant::UI2(n) = v { Some(*n) } else { None }).unwrap_or(0),
    })
}

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub name: String,
    pub cores: u32,
    pub logical: u32,
    pub max_clock: u32,
    pub current_clock: u32,
    pub l2_cache: u32,
    pub l3_cache: u32,
    pub architecture: u16,
}

#[cfg(not(windows))]
pub fn query_wmi_u32(_class: &str, _property: &str) -> Option<u32> {
    None
}

#[cfg(not(windows))]
pub fn query_wmi_u64(_class: &str, _property: &str) -> Option<u64> {
    None
}

#[cfg(not(windows))]
pub fn query_wmi_string(_class: &str, _property: &str) -> Option<String> {
    None
}

#[cfg(not(windows))]
pub fn count_wmi_instances(_class: &str) -> usize {
    0
}

#[cfg(not(windows))]
pub fn query_cpu_info() -> Option<CpuInfo> {
    None
}
