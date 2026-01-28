use clap::Parser;
use serde::Deserialize;
use serde_json::Value;
use std::time::Instant;
use tokio::process::Command;
use tokio::io::{AsyncWriteExt, BufReader, AsyncBufReadExt};
use std::process::Stdio;
use indexmap::IndexMap;

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

// Convert nanoseconds to milliseconds with decimal precision
fn ns_to_ms_str(ns: u128) -> String {
    let ms = ns as f64 / 1_000_000.0;
    format!("{:.3}", ms)
}

// Calculate median from a sorted vector
fn calculate_median(sorted_values: &[u128]) -> f64 {
    if sorted_values.is_empty() {
        return 0.0;
    }
    let len = sorted_values.len();
    if len % 2 == 0 {
        (sorted_values[len / 2 - 1] + sorted_values[len / 2]) as f64 / 2.0
    } else {
        sorted_values[len / 2] as f64
    }
}

// Calculate 99th percentile from a sorted vector
fn calculate_p99(sorted_values: &[u128]) -> f64 {
    if sorted_values.is_empty() {
        return 0.0;
    }
    let len = sorted_values.len();
    let index = ((len as f64 * 0.99).ceil() as usize).saturating_sub(1).min(len - 1);
    sorted_values[index] as f64
}

// Structure to hold timing data for a step
#[derive(Default)]
struct StepTimings {
    wall_times: Vec<u128>,
    cpu_times: Vec<u128>,
}

#[derive(Deserialize)]
struct BenchConfig {
    steps: Vec<Step>,
}

#[derive(Deserialize)]
struct Step {
    name: String,
    payload: Value,
    #[serde(default = "default_bench")]
    bench: bool,
}

fn default_bench() -> bool {
    true
}

#[derive(Deserialize, Clone)]
struct AppCommand {
    bin: String,
    args: Vec<String>,
}

/// Benchmark tool for MCP stdio wrapper
#[derive(Parser, Debug)]
#[command(name = "bench")]
#[command(about = "Run benchmarks against an MCP server", long_about = None)]
struct Args {
    /// Path to the benchmark configuration file (TOML format)
    #[arg(value_name = "BENCH_CONFIG")]
    bench_config: String,

    /// Number of times to run the benchmark
    #[arg(short, long, default_value_t = 1)]
    iterations: usize,

    /// Reuse the same child process across all iterations instead of spawning a new one each time
    #[arg(short, long)]
    persistent: bool,

    /// Path to the binary to benchmark
    #[arg(value_name = "BIN_PATH")]
    bin_path: String,

    /// Arguments to pass to the binary
    #[arg(value_name = "ARGS", trailing_var_arg = true)]
    bin_args: Vec<String>,
}

async fn run_benchmark_steps(
    stdin: &mut tokio::process::ChildStdin,
    reader: &mut tokio::io::Lines<BufReader<tokio::process::ChildStdout>>,
    config: &BenchConfig,
    timings: &mut IndexMap<String, StepTimings>,
) -> Result<(), Box<dyn std::error::Error>> {
    for step in &config.steps {
        let req_id = step.payload.get("id").and_then(|v: &Value| v.as_i64());
        
        // Check if benchmarking is disabled for this step
        if !step.bench {
            // Send the JSON payload without timing
            stdin.write_all(format!("{}\n", step.payload).as_bytes()).await?;
            println!("{:<25} | {:<10} | {:<15} | {:<15}", step.name, "SENT", "N/A", "N/A");
            
            // If it's a request (has ID), still need to consume the response
            if req_id.is_some() {
                while let Some(line) = reader.next_line().await? {
                    let resp: Value = serde_json::from_str(&line)?;
                    if resp.get("id").and_then(|v| v.as_i64()) == req_id {
                        break;
                    }
                }
            }
            continue;
        }
        
        // Benchmarking enabled - measure timing
        let start_wall = Instant::now();
        let start_cpu = get_cpu_time_ns();

        // Send the JSON payload exactly as defined in TOML
        stdin.write_all(format!("{}\n", step.payload).as_bytes()).await?;

        if let Some(current_req_id) = req_id {
            // Wait for a response with a matching ID
            while let Some(line) = reader.next_line().await? {
                let resp: Value = serde_json::from_str(&line)?;
                if resp.get("id").and_then(|v| v.as_i64()) == Some(current_req_id) {
                    let wall_duration_ns = start_wall.elapsed().as_nanos();
                    let cpu_duration_ns = get_cpu_time_ns() - start_cpu;
                    let status = if resp.get("error").is_some() { "ERROR" } else { "OK" };

                    let wall_str = ns_to_ms_str(wall_duration_ns);
                    let cpu_str = ns_to_ms_str(cpu_duration_ns as u128);

                    println!("{:<25} | {:<10} | {:<15} | {:<15}", step.name, status, wall_str, cpu_str);
                    
                    // Collect timing data
                    let step_timing = timings.entry(step.name.clone()).or_insert_with(StepTimings::default);
                    step_timing.wall_times.push(wall_duration_ns);
                    step_timing.cpu_times.push(cpu_duration_ns as u128);
                    
                    break;
                }
            }
        } else {
            // No ID: this is a notification. It's fire and forget.
            let wall_duration_ns = start_wall.elapsed().as_nanos();
            let cpu_duration_ns = get_cpu_time_ns() - start_cpu;
            let wall_str = ns_to_ms_str(wall_duration_ns);
            let cpu_str = ns_to_ms_str(cpu_duration_ns as u128);
            println!("{:<25} | {:<10} | {:<15} | {:<15}", step.name, "SENT", wall_str, cpu_str);
            
            // Collect timing data
            let step_timing = timings.entry(step.name.clone()).or_insert_with(StepTimings::default);
            step_timing.wall_times.push(wall_duration_ns);
            step_timing.cpu_times.push(cpu_duration_ns as u128);
        }
    }

    Ok(())
}

async fn run_benchmark(target: AppCommand, bench_path: &str, timings: &mut IndexMap<String, StepTimings>) -> Result<(), Box<dyn std::error::Error>> {
    // Load the benchmark sequence
    let bench_str = std::fs::read_to_string(bench_path)?;
    let config: BenchConfig = toml::from_str(&bench_str)?;

    // Spawn the MCP process
    let mut child = Command::new(&target.bin)
        .args(&target.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    let mut reader = BufReader::new(child.stdout.take().unwrap()).lines();

    println!("{:<25} | {:<10} | {:<15} | {:<15}", "Step Name", "Status", "Wall (ms)", "CPU (ms)");
    println!("{}", "-".repeat(80));

    run_benchmark_steps(&mut stdin, &mut reader, &config, timings).await?;

    child.kill().await?;
    Ok(())
}

async fn run_benchmark_persistent(target: AppCommand, bench_path: &str, iterations: usize) -> Result<(), Box<dyn std::error::Error>> {
    // Load the benchmark sequence
    let bench_str = std::fs::read_to_string(bench_path)?;
    let config: BenchConfig = toml::from_str(&bench_str)?;

    // Spawn the MCP process once
    let mut child = Command::new(&target.bin)
        .args(&target.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    let mut reader = BufReader::new(child.stdout.take().unwrap()).lines();

    let mut timings: IndexMap<String, StepTimings> = IndexMap::new();

    // Run benchmark multiple times with the same process
    for iteration in 1..=iterations {
        if iterations > 1 {
            println!("\n{}", "=".repeat(80));
            println!("Iteration {}/{}", iteration, iterations);
            println!("{}\n", "=".repeat(80));
        }

        println!("{:<25} | {:<10} | {:<15} | {:<15}", "Step Name", "Status", "Wall (ms)", "CPU (ms)");
        println!("{}", "-".repeat(80));

        run_benchmark_steps(&mut stdin, &mut reader, &config, &mut timings).await?;
    }

    if iterations > 1 {
        println!("\n{}", "=".repeat(80));
        println!("Completed {} iterations", iterations);
        println!("{}", "=".repeat(80));
        
        // Print statistics
        print_statistics(&timings);
    }

    child.kill().await?;
    Ok(())
}

fn print_statistics(timings: &IndexMap<String, StepTimings>) {
    println!("\n{}", "=".repeat(100));
    println!("Statistics Summary");
    println!("{}", "=".repeat(100));
    println!("{:<25} | {:<15} | {:<15} | {:<15} | {:<15}",
             "Step Name", "Wall Med (ms)", "Wall P99 (ms)", "CPU Med (ms)", "CPU P99 (ms)");
    println!("{}", "-".repeat(100));
    
    for (step_name, step_timings) in timings {
        if step_timings.wall_times.is_empty() {
            continue;
        }
        
        let mut wall_sorted = step_timings.wall_times.clone();
        wall_sorted.sort_unstable();
        let wall_median = calculate_median(&wall_sorted) / 1_000_000.0;
        let wall_p99 = calculate_p99(&wall_sorted) / 1_000_000.0;
        
        let mut cpu_sorted = step_timings.cpu_times.clone();
        cpu_sorted.sort_unstable();
        let cpu_median = calculate_median(&cpu_sorted) / 1_000_000.0;
        let cpu_p99 = calculate_p99(&cpu_sorted) / 1_000_000.0;
        
        println!("{:<25} | {:<15.3} | {:<15.3} | {:<15.3} | {:<15.3}",
                 step_name, wall_median, wall_p99, cpu_median, cpu_p99);
    }
    println!("{}", "=".repeat(100));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let target = AppCommand {
        bin: args.bin_path,
        args: args.bin_args,
    };
    
    if args.persistent {
        // Persistent mode: reuse the same child process across all iterations
        run_benchmark_persistent(target, &args.bench_config, args.iterations).await?;
    } else {
        // Default mode: spawn a new child process for each iteration
        let mut timings: IndexMap<String, StepTimings> = IndexMap::new();
        
        for iteration in 1..=args.iterations {
            if args.iterations > 1 {
                println!("\n{}", "=".repeat(80));
                println!("Iteration {}/{}", iteration, args.iterations);
                println!("{}\n", "=".repeat(80));
            }
            
            run_benchmark(target.clone(), &args.bench_config, &mut timings).await?;
        }
        
        if args.iterations > 1 {
            println!("\n{}", "=".repeat(80));
            println!("Completed {} iterations", args.iterations);
            println!("{}", "=".repeat(80));
            
            // Print statistics
            print_statistics(&timings);
        }
    }
    
    Ok(())
}