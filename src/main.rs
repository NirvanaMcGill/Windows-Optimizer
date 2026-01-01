use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::time::Instant;
use std::path::PathBuf;

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
    let cli = Cli::parse();
    
    // Handle subcommands
    match cli.cmd {
        Some(Cmd::Audit) => run_audit(&cli)?,
        Some(Cmd::Apply { profile }) => run_apply(profile)?,
        Some(Cmd::Backup { path }) => run_backup(&path)?,
        Some(Cmd::Restore { path }) => run_restore(&path)?,
        None => {
            // Default to audit if no subcommand specified
            run_audit(&cli)?;
        }
    }

    Ok(())
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
        println!("\n{} {} checks completed in {:.2}s", 
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
    println!("{} Apply mode not yet implemented", "⚠".yellow());
    if let Some(p) = profile {
        println!("Profile: {}", p);
    }
    Ok(())
}

fn run_backup(path: &PathBuf) -> Result<()> {
    println!("{} Backing up to {:?}", "→".blue(), path);
    // TODO: Implement backup_registry
    println!("{} Backup not yet implemented", "⚠".yellow());
    Ok(())
}

fn run_restore(path: &PathBuf) -> Result<()> {
    println!("{} Restoring from {:?}", "→".blue(), path);
    // TODO: Implement restore_registry
    println!("{} Restore not yet implemented", "⚠".yellow());
    Ok(())
}

fn print_banner() {
    println!("{}", "╔════════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║        Windows Optimizer - System Performance Audit       ║".cyan());
    println!("{}", "╚════════════════════════════════════════════════════════════╝".cyan());
    println!();
}

fn run_all_checks(cli: &Cli) -> Result<AuditResults> {
    let categories = if let Some(ref cat) = cli.category {
        vec![cat.as_str()]
    } else {
        vec![
            "latency", "cpu", "gpu", "memory", "storage", "network",
            "audio", "input", "stability", "services", "security",
            "platform", "thermal", "power"
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
                .progress_chars("#>-")
        );
        Some(p)
    } else {
        None
    };

    // Run checks in parallel by category
    let category_results: Vec<_> = categories.par_iter()
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
    println!("{}", format!(
        "Summary: {} optimal, {} warnings, {} issues, {} info",
        results.count_status(CheckStatus::Optimal).to_string().green(),
        results.count_status(CheckStatus::Warning).to_string().yellow(),
        results.count_status(CheckStatus::Issue).to_string().red(),
        results.count_status(CheckStatus::Info).to_string().blue(),
    ));
}
