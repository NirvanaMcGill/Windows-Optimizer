use anyhow::Result;
use clap::Parser;
use colored::*;
use rayon::prelude::*;
use std::time::Instant;

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
    /// Export results to JSON file
    #[arg(long, value_name = "FILE")]
    json: Option<String>,

    /// Export results to HTML file
    #[arg(long, value_name = "FILE")]
    html: Option<String>,

    /// Export results to CSV file
    #[arg(long, value_name = "FILE")]
    csv: Option<String>,

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

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    if !cli.quiet {
        print_banner();
    }

    let start_time = Instant::now();
    
    // Run all checks in parallel
    let results = run_all_checks(&cli)?;
    
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
    if let Some(path) = cli.json {
        export_json(&results, &path)?;
        if !cli.quiet {
            println!("{} Results exported to {}", "✓".green(), path);
        }
    }

    if let Some(path) = cli.html {
        export_html(&results, &path)?;
        if !cli.quiet {
            println!("{} HTML report exported to {}", "✓".green(), path);
        }
    }

    if let Some(path) = cli.csv {
        export_csv(&results, &path)?;
        if !cli.quiet {
            println!("{} CSV exported to {}", "✓".green(), path);
        }
    }

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

    // Run checks in parallel by category
    let category_results: Vec<_> = categories.par_iter()
        .map(|category| match *category {
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
        })
        .collect();

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
