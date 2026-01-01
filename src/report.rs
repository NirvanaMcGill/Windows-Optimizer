use crate::types::*;
use anyhow::Result;
use std::fs;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

fn esc_csv(s: &str) -> String {
    let s_clean = s.replace('"', "\"\"");
    if s.starts_with(|c: char| "=+-@\t\r".contains(c)) {
        format!("'{}", s_clean)
    } else {
        s_clean
    }
}

pub fn export_json(results: &AuditResults, path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(results)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn export_csv(results: &AuditResults, path: &str) -> Result<()> {
    let mut csv = String::from("Category,Check,Value,Status,Description\n");

    for category in results.categories.values() {
        for check in &category.checks {
            let status_str = match check.status {
                CheckStatus::Optimal => "Optimal",
                CheckStatus::Warning => "Warning",
                CheckStatus::Issue => "Issue",
                CheckStatus::Info => "Info",
            };
            csv.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                esc_csv(&category.name),
                esc_csv(&check.name),
                esc_csv(&check.value),
                status_str,
                esc_csv(&check.description)
            ));
        }
    }

    fs::write(path, csv)?;
    Ok(())
}

pub fn export_html(results: &AuditResults, path: &str) -> Result<()> {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Windows Optimizer Report</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: #1e1e1e;
            color: #d4d4d4;
        }
        .header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            padding: 30px;
            border-radius: 10px;
            margin-bottom: 30px;
            text-align: center;
        }
        h1 { margin: 0; color: white; }
        .timestamp { color: rgba(255,255,255,0.8); margin-top: 10px; }
        .summary {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        .summary-card {
            background: #2d2d30;
            padding: 20px;
            border-radius: 8px;
            border-left: 4px solid;
        }
        .optimal { border-color: #4ec9b0; }
        .warning { border-color: #ce9178; }
        .issue { border-color: #f48771; }
        .info { border-color: #4fc1ff; }
        .category {
            background: #2d2d30;
            margin-bottom: 20px;
            border-radius: 8px;
            overflow: hidden;
        }
        .category-header {
            background: #37373d;
            padding: 15px;
            font-weight: bold;
            font-size: 1.2em;
        }
        .check {
            padding: 12px 15px;
            border-bottom: 1px solid #3e3e42;
            display: grid;
            grid-template-columns: 30px 1fr 1fr;
            gap: 15px;
            align-items: center;
        }
        .check:last-child { border-bottom: none; }
        .check-icon { font-size: 1.2em; }
        .check-name { font-weight: 500; }
        .check-value { color: #9cdcfe; }
        .check-description { 
            grid-column: 2 / -1; 
            font-size: 0.9em; 
            color: #858585;
            margin-top: 5px;
        }
    </style>
</head>
<body>
    <div class="header">
        <h1>🚀 Windows Optimizer Report</h1>
        <div class="timestamp">"#,
    );

    html.push_str(&format!(
        "Generated: {}</div>\n    </div>\n",
        results.timestamp
    ));

    // Summary cards
    html.push_str("    <div class=\"summary\">\n");
    html.push_str(&format!(
        "        <div class=\"summary-card optimal\"><h2>{}</h2><p>Optimal</p></div>\n",
        results.count_status(CheckStatus::Optimal)
    ));
    html.push_str(&format!(
        "        <div class=\"summary-card warning\"><h2>{}</h2><p>Warnings</p></div>\n",
        results.count_status(CheckStatus::Warning)
    ));
    html.push_str(&format!(
        "        <div class=\"summary-card issue\"><h2>{}</h2><p>Issues</p></div>\n",
        results.count_status(CheckStatus::Issue)
    ));
    html.push_str(&format!(
        "        <div class=\"summary-card info\"><h2>{}</h2><p>Info</p></div>\n",
        results.count_status(CheckStatus::Info)
    ));
    html.push_str("    </div>\n\n");

    // Categories
    for category in results.categories.values() {
        html.push_str(&format!(
            "    <div class=\"category\">\n        <div class=\"category-header\">{}</div>\n",
            esc(&category.name)
        ));

        for check in &category.checks {
            let icon = match check.status {
                CheckStatus::Optimal => "✓",
                CheckStatus::Warning => "⚠",
                CheckStatus::Issue => "✗",
                CheckStatus::Info => "ℹ",
            };

            let class = match check.status {
                CheckStatus::Optimal => "optimal",
                CheckStatus::Warning => "warning",
                CheckStatus::Issue => "issue",
                CheckStatus::Info => "info",
            };

            html.push_str(&format!(
                "        <div class=\"check\">\n            <div class=\"check-icon {}\">{}</div>\n            <div class=\"check-name\">{}</div>\n            <div class=\"check-value\">{}</div>\n",
                class, icon, esc(&check.name), esc(&check.value)
            ));

            if !check.description.is_empty() {
                html.push_str(&format!(
                    "            <div class=\"check-description\">{}</div>\n",
                    esc(&check.description)
                ));
            }

            html.push_str("        </div>\n");
        }

        html.push_str("    </div>\n");
    }

    html.push_str("</body>\n</html>");

    fs::write(path, html)?;
    Ok(())
}
