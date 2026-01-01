#[cfg(windows)]
pub fn query_wmi_u32(class: &str, property: &str) -> Option<u32> {
    use wmi::{COMLibrary, WMIConnection};
    
    let com_con = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com_con).ok()?;
    
    let results: Vec<std::collections::HashMap<String, wmi::Variant>> = 
        wmi_con.raw_query(&format!("SELECT {} FROM {}", property, class)).ok()?;
    
    results.first().and_then(|r| {
        r.get(property).and_then(|v| match v {
            wmi::Variant::UI4(n) => Some(*n),
            wmi::Variant::I4(n) => Some(*n as u32),
            _ => None,
        })
    })
}

#[cfg(windows)]
pub fn query_wmi_u64(class: &str, property: &str) -> Option<u64> {
    use wmi::{COMLibrary, WMIConnection};
    
    let com_con = COMLibrary::new().ok()?;
    let wmi_con = WMIConnection::new(com_con).ok()?;
    
    let results: Vec<std::collections::HashMap<String, wmi::Variant>> = 
        wmi_con.raw_query(&format!("SELECT {} FROM {}", property, class)).ok()?;
    
    results.first().and_then(|r| {
        r.get(property).and_then(|v| match v {
            wmi::Variant::UI8(n) => Some(*n),
            wmi::Variant::I8(n) => Some(*n as u64),
            _ => None,
        })
    })
}

#[cfg(windows)]
pub fn query_wmi_string(class: &str, property: &str) -> Option<String> {
    use wmi::{COMLibrary, WMIConnection};
    
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
    
    let com_con = COMLibrary::new().ok();
    let wmi_con = com_con.and_then(|c| WMIConnection::new(c).ok());
    
    wmi_con.and_then(|w| {
        let results: Result<Vec<std::collections::HashMap<String, wmi::Variant>>, _> = 
            w.raw_query(&format!("SELECT * FROM {}", class));
        results.ok().map(|r| r.len())
    }).unwrap_or(0)
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
