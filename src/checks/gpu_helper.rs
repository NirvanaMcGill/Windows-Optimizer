#[cfg(windows)]
pub fn get_gpu_info() -> Option<(String, u64)> {
    use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIFactory1};

    unsafe {
        let factory: Result<IDXGIFactory1, _> = CreateDXGIFactory1();
        if let Ok(factory) = factory {
            if let Ok(adapter) = factory.EnumAdapters1(0) {
                if let Ok(desc) = adapter.GetDesc1() {
                    let name = String::from_utf16_lossy(&desc.Description)
                        .trim_end_matches('\0')
                        .to_string();
                    let vram = desc.DedicatedVideoMemory as u64;
                    return Some((name, vram));
                }
            }
        }
        None
    }
}

#[cfg(not(windows))]
pub fn get_gpu_info() -> Option<(String, u64)> {
    None
}
