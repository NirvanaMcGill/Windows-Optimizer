# Windows-Optimizer

Military-grade Windows system performance auditor and optimizer built in Rust.

## Features

- **500+ System Checks** across 14 categories
- **Real System Detection** - WMI, DXGI, and Windows Services API for accurate hardware/service detection
- **Sub-second Execution** - Full audit completes with parallel processing
- **Compact Binary** - Single ~1.2MB executable
- **Parallel Execution** - Multi-threaded check execution using rayon
- **Multiple Export Formats** - JSON, HTML, CSV with XSS/injection protection
- **CLI Enhancements** - Subcommands, progress bar, apply mode
- **Security Hardened** - Fixed buffer overflows, XSS, CSV injection, WMI injection

## Categories

1. **Latency** (35 checks) - DPC, ISR, HPET, TSC, timer resolution, MMCSS
2. **CPU** (30 checks) - Power plans, C-States, core parking, VBS, HVCI, SMT detection
3. **GPU** (35 checks) - HAGS, TDR, Game DVR, MPO, NVIDIA/AMD settings, DXGI detection
4. **Memory** (25 checks) - RAM speed (WMI), channel config, page file, compression
5. **Storage** (30 checks) - TRIM, alignment, 8.3 filenames, NVMe settings
6. **Network** (35 checks) - Nagle, RSS, DNS, QoS, TCP settings
7. **Audio** (20 checks) - Exclusive mode, buffer size, enhancements
8. **Input** (20 checks) - Mouse acceleration, pointer precision, polling
9. **Stability** (30 checks) - Event logs, crashes, driver health
10. **Services** (40 checks) - Real status detection for 51+ services
11. **Security** (15 checks) - VBS, Defender, firewall, Secure Boot
12. **Platform** (25 checks) - Windows version, DirectX, .NET, widgets
13. **Thermal** (20 checks) - Temperature monitoring, throttling
14. **Power** (25 checks) - PCIe ASPM, USB suspend, fast startup

## Usage

```bash
# Run full audit with progress bar
Windows-Optimizer.exe

# Run specific subcommands
Windows-Optimizer.exe audit
Windows-Optimizer.exe apply --profile gaming
Windows-Optimizer.exe backup backup.reg
Windows-Optimizer.exe restore backup.reg

# Export to multiple formats
Windows-Optimizer.exe --json report.json --html report.html --csv data.csv

# Filter by category
Windows-Optimizer.exe --category gpu --verbose

# Quiet mode for automation
Windows-Optimizer.exe --quiet --json report.json

# Dry run to see what would change
Windows-Optimizer.exe --dry-run --apply
```

## Building

```bash
cargo build --release
```

Binary will be in `target/release/Windows-Optimizer.exe` (~1.2MB)

## Performance Optimizations

- **opt-level = 3** - Maximum performance optimization
- **LTO enabled** - Link-time optimization
- **Parallel checks** - All categories run concurrently
- **Minimal allocations** - Stack-preferred, efficient memory usage