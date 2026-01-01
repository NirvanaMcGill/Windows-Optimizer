#[cfg(windows)]
pub fn query_service_status(service_name: &str) -> Option<String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows::Win32::System::Services::{
        CloseServiceHandle, OpenSCManagerW, OpenServiceW, QueryServiceStatus, SC_MANAGER_CONNECT,
        SERVICE_PAUSED, SERVICE_QUERY_STATUS, SERVICE_RUNNING, SERVICE_STATUS, SERVICE_STOPPED,
    };

    unsafe {
        let scm = OpenSCManagerW(None, None, SC_MANAGER_CONNECT).ok()?;

        let name: Vec<u16> = OsStr::new(service_name)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let service = OpenServiceW(
            scm,
            windows::core::PCWSTR(name.as_ptr()),
            SERVICE_QUERY_STATUS,
        )
        .ok()?;

        let mut status = SERVICE_STATUS::default();
        let result = QueryServiceStatus(service, &mut status);

        let _ = CloseServiceHandle(service);
        let _ = CloseServiceHandle(scm);

        if result.is_ok() {
            Some(match status.dwCurrentState {
                s if s == SERVICE_RUNNING => "Running".to_string(),
                s if s == SERVICE_STOPPED => "Stopped".to_string(),
                s if s == SERVICE_PAUSED => "Paused".to_string(),
                _ => "Unknown".to_string(),
            })
        } else {
            None
        }
    }
}

#[cfg(not(windows))]
pub fn query_service_status(_service_name: &str) -> Option<String> {
    None
}
