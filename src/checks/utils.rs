pub type HKEY = u32;
pub const HKEY_LOCAL_MACHINE: HKEY = 1;
pub const HKEY_CURRENT_USER: HKEY = 2;

#[cfg(windows)]
pub fn read_registry_string(_hkey: HKEY, subkey: &str, value_name: &str) -> Option<String> {
    use std::ptr;
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    
    unsafe {
        let hkey_root: isize = if _hkey == HKEY_LOCAL_MACHINE {
            0x80000002_u32 as isize
        } else {
            0x80000001_u32 as isize
        };
        
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        let value_wide: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
        
        let mut hkey_result: isize = 0;
        if winapi::um::winreg::RegOpenKeyExW(
            hkey_root,
            subkey_wide.as_ptr(),
            0,
            winapi::um::winnt::KEY_READ,
            &mut hkey_result,
        ) != 0 {
            return None;
        }

        // Query size first
        let mut size = 0u32;
        let mut data_type = 0u32;
        if winapi::um::winreg::RegQueryValueExW(
            hkey_result,
            value_wide.as_ptr(),
            ptr::null_mut(),
            &mut data_type,
            ptr::null_mut(),
            &mut size,
        ) != 0 {
            winapi::um::winreg::RegCloseKey(hkey_result);
            return None;
        }

        // Allocate buffer based on actual size
        let mut buffer = vec![0u16; (size as usize / 2) + 1];
        let mut buffer_size = size;

        if winapi::um::winreg::RegQueryValueExW(
            hkey_result,
            value_wide.as_ptr(),
            ptr::null_mut(),
            &mut data_type,
            buffer.as_mut_ptr() as *mut u8,
            &mut buffer_size,
        ) == 0 {
            winapi::um::winreg::RegCloseKey(hkey_result);
            let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
            Some(OsString::from_wide(&buffer[..len]).to_string_lossy().to_string())
        } else {
            winapi::um::winreg::RegCloseKey(hkey_result);
            None
        }
    }
}

#[cfg(not(windows))]
pub fn read_registry_string(_hkey: HKEY, _subkey: &str, _value_name: &str) -> Option<String> {
    None
}

#[cfg(windows)]
pub fn read_registry_dword(_hkey: HKEY, subkey: &str, value_name: &str) -> Option<u32> {
    use std::ptr;
    
    unsafe {
        let hkey_root: isize = if _hkey == HKEY_LOCAL_MACHINE {
            0x80000002_u32 as isize
        } else {
            0x80000001_u32 as isize
        };
        
        let subkey_wide: Vec<u16> = subkey.encode_utf16().chain(std::iter::once(0)).collect();
        let value_wide: Vec<u16> = value_name.encode_utf16().chain(std::iter::once(0)).collect();
        
        let mut hkey_result: isize = 0;
        if winapi::um::winreg::RegOpenKeyExW(
            hkey_root,
            subkey_wide.as_ptr(),
            0,
            winapi::um::winnt::KEY_READ,
            &mut hkey_result,
        ) != 0 {
            return None;
        }

        let mut value: u32 = 0;
        let mut buffer_size = 4u32;
        let mut data_type = 0u32;

        if winapi::um::winreg::RegQueryValueExW(
            hkey_result,
            value_wide.as_ptr(),
            ptr::null_mut(),
            &mut data_type,
            &mut value as *mut u32 as *mut u8,
            &mut buffer_size,
        ) == 0 && data_type == 4 {
            winapi::um::winreg::RegCloseKey(hkey_result);
            Some(value)
        } else {
            winapi::um::winreg::RegCloseKey(hkey_result);
            None
        }
    }
}

#[cfg(not(windows))]
pub fn read_registry_dword(_hkey: HKEY, _subkey: &str, _value_name: &str) -> Option<u32> {
    None
}

pub fn registry_key_exists(hkey: HKEY, subkey: &str) -> bool {
    read_registry_string(hkey, subkey, "").is_some() ||
        read_registry_dword(hkey, subkey, "").is_some()
}

pub fn registry_value_exists(hkey: HKEY, subkey: &str, value_name: &str) -> bool {
    read_registry_string(hkey, subkey, value_name).is_some() ||
    read_registry_dword(hkey, subkey, value_name).is_some()
}
