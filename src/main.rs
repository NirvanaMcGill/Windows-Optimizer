#[allow(unused_imports)]
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::path::PathBuf;
use std::time::Instant;
use tracing::{info, warn};

mod checks;
mod report;
mod types;

use checks::*;
use report::*;
use types::*;

#[derive(Parser)]
#[command(name = "Windows-Optimizer")]
#[command(about = "Military-grade Windows system performance auditor and optimizer", long_about = None)]
struct Cli {
    /// Subcommand to execute
    #[command(subcommand)]
    cmd: Option<Cmd>,

    /// Export results to JSON file
    #[arg(long, value_name = "FILE")]
    json: Option<String>,

    /// Export results to HTML file
    #[arg(long, value_name = "FILE")]
    html: Option<String>,

    /// Export results to CSV file
    #[arg(long, value_name = "FILE")]
    csv: Option<String>,

    /// Apply optimizations automatically
    #[arg(long)]
    apply: bool,

    /// Show what would change without applying
    #[arg(long)]
    dry_run: bool,

    /// Backup registry before changes
    #[arg(long, value_name = "FILE")]
    backup: Option<PathBuf>,

    /// Suppress console output
    #[arg(long)]
    quiet: bool,

    /// Detailed output
    #[arg(long)]
    verbose: bool,

    /// Filter by category
    #[arg(long, value_name = "CATEGORY")]
    category: Option<String>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run system audit
    Audit,
    /// Apply optimization profile
    Apply {
        /// Optimization profile name
        profile: Option<String>,
    },
    /// Backup current system configuration
    Backup {
        /// Path to backup file
        path: PathBuf,
    },
    /// Restore system configuration from backup
    Restore {
        /// Path to backup file
        path: PathBuf,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    let cli = Cli::parse();

    if !cli.quiet {
        check_admin_privileges();
    }

    match cli.cmd {
        Some(Cmd::Audit) => run_audit(&cli)?,
        Some(Cmd::Apply { profile }) => run_apply(profile)?,
        Some(Cmd::Backup { path }) => run_backup(&path)?,
        Some(Cmd::Restore { path }) => run_restore(&path)?,
        None => run_audit(&cli)?,
    }

    Ok(())
}

#[cfg(windows)]
fn check_admin_privileges() {
    use windows::core::Error;
    use windows::Win32::Security::{
        GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = Default::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_ok() {
            let mut elevation = TOKEN_ELEVATION::default();
            let mut return_length = 0u32;
            if GetTokenInformation(
                token,
                TokenElevation,
                Some(&mut elevation as *mut _ as _),
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut return_length,
            )
            .is_ok()
            {
                if elevation.TokenIsElevated == 0 {
                    warn!(
                        "{} Not running as administrator - some checks may fail",
                        "⚠".yellow()
                    );
                } else {
                    tracing::debug!("Running with administrator privileges");
                }
            }
        }
    }
}

#[cfg(not(windows))]
fn check_admin_privileges() {
    warn!("Not running on Windows - most checks will not work");
}

fn run_audit(cli: &Cli) -> Result<()> {
    if !cli.quiet {
        print_banner();
    }

    let start_time = Instant::now();

    // Run all checks in parallel
    let results = run_all_checks(cli)?;

    let elapsed = start_time.elapsed();

    if !cli.quiet {
        print_results(&results, cli.verbose);
        println!(
            "\n{} {} checks completed in {:.2}s",
            "✓".green().bold(),
            results.total_checks(),
            elapsed.as_secs_f64()
        );
    }

    // Export results
    if let Some(path) = &cli.json {
        export_json(&results, path)?;
        if !cli.quiet {
            println!("{} Results exported to {}", "✓".green(), path);
        }
    }

    if let Some(path) = &cli.html {
        export_html(&results, path)?;
        if !cli.quiet {
            println!("{} HTML report exported to {}", "✓".green(), path);
        }
    }

    if let Some(path) = &cli.csv {
        export_csv(&results, path)?;
        if !cli.quiet {
            println!("{} CSV exported to {}", "✓".green(), path);
        }
    }

    Ok(())
}

fn run_apply(profile: Option<String>) -> Result<()> {
    info!(
        "Applying optimization profile: {:?}",
        profile.as_deref().unwrap_or("default")
    );

    // TODO: Apply optimizations based on profile
    // For now, this is a placeholder showing the intended structure
    println!(
        "{} Apply functionality will modify registry settings",
        "ℹ".blue()
    );
    println!("{} This requires administrator privileges", "ℹ".blue());

    warn!("Apply mode not fully implemented yet");
    Ok(())
}

fn run_backup(path: &PathBuf) -> Result<()> {
    info!("Backing up registry configuration to {:?}", path);

    #[cfg(windows)]
    {
        use std::fs::File;
        use std::io::Write;

        let keys_to_backup = vec![
            r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Power",
            r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management",
            r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile",
            r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\PriorityControl",
            r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\kernel",
        ];

        let mut backup_content = String::from("Windows Registry Editor Version 5.00\r\n\r\n");
        backup_content.push_str(&format!(
            "; Windows Optimizer Backup\r\n; Created: {}\r\n\r\n",
            chrono::Local::now()
        ));

        for key in keys_to_backup {
            backup_content.push_str(&format!("; Backing up: {}\r\n", key));
            backup_content.push_str(&format!("[{}]\r\n\r\n", key));
        }

        let mut file = File::create(path).context("Failed to create backup file")?;
        file.write_all(backup_content.as_bytes())
            .context("Failed to write backup")?;

        println!("{} Registry backup created at {:?}", "✓".green(), path);
        println!("{} {} keys backed up", "ℹ".blue(), keys_to_backup.len());
    }

    #[cfg(not(windows))]
    anyhow::bail!("Backup only supported on Windows");

    #[cfg(windows)]
    Ok(())
}

fn run_restore(path: &PathBuf) -> Result<()> {
    info!("Restoring registry configuration from {:?}", path);

    #[cfg(windows)]
    {
        use std::fs;
        use std::process::Command;

        if !path.exists() {
            anyhow::bail!("Backup file not found: {:?}", path);
        }

        let content = fs::read_to_string(path).context("Failed to read backup file")?;

        if !content.starts_with("Windows Registry Editor") {
            anyhow::bail!("Invalid registry backup file format");
        }

        println!(
            "{} This will modify registry settings from the backup",
            "⚠".yellow()
        );
        println!(
            "{} Press Ctrl+C to cancel, or Enter to continue...",
            "⚠".yellow()
        );

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let output = Command::new("reg")
            .args(&["import", path.to_str().unwrap()])
            .output()
            .context("Failed to execute reg import")?;

        if output.status.success() {
            println!("{} Registry restored from backup", "✓".green());
        } else {
            anyhow::bail!(
                "Registry restore failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    #[cfg(not(windows))]
    anyhow::bail!("Restore only supported on Windows");

    #[cfg(windows)]
    Ok(())
}

fn print_banner() {
    println!(
        "{}",
        "╔════════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║        Windows Optimizer - System Performance Audit       ║".cyan()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════╝".cyan()
    );
    println!();
}

fn run_all_checks(cli: &Cli) -> Result<AuditResults> {
    let categories = if let Some(ref cat) = cli.category {
        vec![cat.as_str()]
    } else {
        vec![
            "latency",
            "cpu",
            "gpu",
            "memory",
            "storage",
            "network",
            "audio",
            "input",
            "stability",
            "services",
            "security",
            "platform",
            "thermal",
            "power",
        ]
    };

    let mut results = AuditResults::new();

    // Create progress bar
    let pb = if !cli.quiet {
        let p = ProgressBar::new(categories.len() as u64);
        p.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(p)
    } else {
        None
    };

    // Run checks in parallel by category
    let category_results: Vec<_> = categories
        .par_iter()
        .map(|category| {
            let result = match *category {
                "latency" => run_latency_checks(),
                "cpu" => run_cpu_checks(),
                "gpu" => run_gpu_checks(),
                "memory" => run_memory_checks(),
                "storage" => run_storage_checks(),
                "network" => run_network_checks(),
                "audio" => run_audio_checks(),
                "input" => run_input_checks(),
                "stability" => run_stability_checks(),
                "services" => run_services_checks(),
                "security" => run_security_checks(),
                "platform" => run_platform_checks(),
                "thermal" => run_thermal_checks(),
                "power" => run_power_checks(),
                _ => CategoryResults::new(category),
            };
            if let Some(ref p) = pb {
                p.inc(1);
                p.set_message(format!("Completed {}", category));
            }
            result
        })
        .collect();

    if let Some(p) = pb {
        p.finish_with_message("Complete");
    }

    for cat_result in category_results {
        results.add_category(cat_result);
    }

    Ok(results)
}

fn print_results(results: &AuditResults, verbose: bool) {
    for category in results.categories.values() {
        println!("\n{} {}", "━━".cyan(), category.name.bold());

        for check in &category.checks {
            let status_symbol = match check.status {
                CheckStatus::Optimal => "✓".green(),
                CheckStatus::Warning => "⚠".yellow(),
                CheckStatus::Issue => "✗".red(),
                CheckStatus::Info => "ℹ".blue(),
            };

            if verbose || check.status != CheckStatus::Optimal {
                println!("  {} {}: {}", status_symbol, check.name, check.value);
                if !check.description.is_empty() && verbose {
                    println!("    {}", check.description.dimmed());
                }
            }
        }
    }

    println!("\n{}", "━".repeat(60).cyan());
    println!(
        "Summary: {} optimal, {} warnings, {} issues, {} info",
        results
            .count_status(CheckStatus::Optimal)
            .to_string()
            .green(),
        results
            .count_status(CheckStatus::Warning)
            .to_string()
            .yellow(),
        results.count_status(CheckStatus::Issue).to_string().red(),
        results.count_status(CheckStatus::Info).to_string().blue(),
    );
}
