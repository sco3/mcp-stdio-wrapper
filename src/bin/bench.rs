use serde::Deserialize;
use serde_json::Value;
use std::time::Instant;
use tokio::process::Command;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use std::process::Stdio;

// For CPU time measurement
fn get_cpu_time_ns() -> i64 {
    unsafe {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut ts);
        ts.tv_sec * 1_000_000_000 + ts.tv_nsec
    }
}

// Format number with thousand separators
fn format_with_separators(num: u128) -> String {
    let s = num.to_string();
    let mut result = String::new();
    let mut count = 0;
    
    for c in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push('_');
        }
        result.push(c);
        count += 1;
    }
    
    result.chars().rev().collect()
}

#[derive(Deserialize)]
struct BenchConfig {
    steps: Vec<Step>,
}

#[derive(Deserialize)]
struct Step {
    name: String,
    payload: Value,
}

#[derive(Deserialize, Clone)]
struct AppCommand {
    bin: String,
    args: Vec<String>,
}

async fn run_benchmark(target: AppCommand, bench_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load the benchmark sequence
    let bench_str = std::fs::read_to_string(bench_path)?;
    let config: BenchConfig = toml::from_str(&bench_str)?;

    // 2. Spawn the MCP process
    let mut child = Command::new(&target.bin)
        .args(&target.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())  // IMPROVEMENT: Capture stderr for error diagnostics
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    let mut reader = BufReader::new(child.stdout.take().unwrap()).lines();

    println!("{:<25} | {:<10} | {:<15} | {:<15}", "Step Name", "Status", "Wall (μs)", "CPU (μs)");
    println!("{}", "-".repeat(80));

    for step in config.steps {
        let req_id = step.payload.get("id").and_then(|v: &Value| v.as_i64());
        let start_wall = Instant::now();
        let start_cpu = get_cpu_time_ns();

        // Send the JSON payload exactly as defined in TOML
        stdin.write_all(format!("{}\n", step.payload).as_bytes()).await?;

        // Handle Notifications (No response expected)
        if req_id.is_none() {
            println!("{:<25} | {:<10} | {:<15} | {:<15}", step.name, "SENT", "N/A", "N/A");
            continue;
        }

        // Wait for Response
        while let Some(line) = reader.next_line().await? {
            let resp: Value = serde_json::from_str(&line)?;
            
            // Basic check: did we get the right ID back?
            if resp.get("id").and_then(|v| v.as_i64()) == req_id {
                let wall_duration_us = start_wall.elapsed().as_micros();
                let cpu_duration_ns = get_cpu_time_ns() - start_cpu;
                let cpu_duration_us = (cpu_duration_ns / 1_000) as u128;
                let status = if resp.get("error").is_some() { "ERROR" } else { "OK" };
                
                let wall_str = format_with_separators(wall_duration_us);
                let cpu_str = format_with_separators(cpu_duration_us);
                
                println!("{:<25} | {:<10} | {:<15} | {:<15}", step.name, status, wall_str, cpu_str);
                break;
            }
        }
    }

    child.kill().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <bench_config.toml> <bin_path> [--iterations N] [args...]", args[0]);
        eprintln!("  --iterations N: Run the benchmark N times (default: 1)");
        std::process::exit(1);
    }
    
    let bench_path = &args[1];
    
    // Parse iterations parameter
    let mut iterations = 1;
    let mut target_args_start = 3;
    
    if args.len() > 3 && (args[3] == "--iterations" || args[3] == "-n") {
        if args.len() < 5 {
            eprintln!("Error: --iterations requires a number argument");
            std::process::exit(1);
        }
        iterations = args[4].parse::<usize>().unwrap_or_else(|_| {
            eprintln!("Error: Invalid number for iterations: {}", args[4]);
            std::process::exit(1);
        });
        target_args_start = 5;
    }
    
    let target = AppCommand {
        bin: args[2].clone(),
        args: args[target_args_start..].to_vec(),
    };
    
    // Run benchmark multiple times
    for iteration in 1..=iterations {
        if iterations > 1 {
            println!("\n{}", "=".repeat(80));
            println!("Iteration {}/{}", iteration, iterations);
            println!("{}\n", "=".repeat(80));
        }
        
        run_benchmark(target.clone(), bench_path).await?;
    }
    
    if iterations > 1 {
        println!("\n{}", "=".repeat(80));
        println!("Completed {} iterations", iterations);
        println!("{}", "=".repeat(80));
    }
    
    Ok(())
}