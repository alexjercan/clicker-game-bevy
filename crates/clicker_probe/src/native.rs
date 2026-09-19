use std::{path::PathBuf, process::ExitCode};

use clap::{Args, Parser, Subcommand};

use crate::{
    capabilities::timing::{aggregate_timing, TIMING_WARMUP_ENV},
    html::{render_all_report, render_run_report},
};

const TIMING_TARGET_UPDATES: usize = 600;
const MAX_TIMING_RUNS: usize = 32;

const EXAMPLES: [&str; 4] = [
    "autopilot_tiles",
    "autopilot_expectation",
    "autopilot_deadline",
    "autopilot_performance",
];

#[derive(Parser)]
#[command(name = "clicker probe")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Args)]
struct RunOptions {
    #[arg(long)]
    norender: bool,
    #[arg(long)]
    seed: Option<u64>,
    #[arg(long, default_value_t = 120.0)]
    deadline: f32,
}

#[derive(Subcommand)]
enum Command {
    Run {
        example: String,
        #[command(flatten)]
        options: RunOptions,
        #[arg(long, default_value = "target/probe")]
        out: PathBuf,
    },
    All {
        #[command(flatten)]
        options: RunOptions,
        #[arg(long, default_value = "target/probe-all")]
        out: PathBuf,
    },
}

pub fn main(args: &[String]) -> ExitCode {
    let parsed = Cli::try_parse_from(
        std::iter::once("clicker probe").chain(args.iter().map(String::as_str)),
    );
    let cli = match parsed {
        Ok(cli) => cli,
        Err(error) => {
            let _ = error.print();
            return ExitCode::from(2);
        }
    };
    match cli.command {
        Command::Run {
            example,
            options,
            out,
        } => exit(run(&example, &options, &out)),
        Command::All { options, out } => all(&options, &out),
    }
}

fn all(options: &RunOptions, out: &std::path::Path) -> ExitCode {
    if let Err(error) = std::fs::create_dir_all(out) {
        eprintln!("clicker probe: cannot create {out:?}: {error}");
        return ExitCode::from(2);
    }
    let rows = EXAMPLES
        .iter()
        .map(|example| {
            let success = run(example, options, &out.join(example));
            serde_json::json!({"example": example, "success": success})
        })
        .collect::<Vec<_>>();
    let manifest = serde_json::json!({
        "norender": options.norender,
        "seed": options.seed,
        "deadline_secs": options.deadline,
        "runs": rows,
    });
    if let Err(error) = std::fs::write(out.join("probe-all.json"), manifest.to_string()) {
        eprintln!("clicker probe: cannot write aggregate metadata: {error}");
        return ExitCode::from(2);
    }
    if let Err(error) = std::fs::write(out.join("report.html"), render_all_report(&rows)) {
        eprintln!("clicker probe: cannot write aggregate report.html: {error}");
        return ExitCode::from(2);
    }
    let success = rows
        .iter()
        .all(|row| row["success"].as_bool().unwrap_or(false));
    eprintln!(
        "clicker probe: {} all",
        if success { "PASS" } else { "FAIL" }
    );
    eprintln!("{}", out.join("report.html").display());
    exit(success)
}

fn run(example: &str, options: &RunOptions, out: &std::path::Path) -> bool {
    if let Err(error) = std::fs::create_dir_all(out) {
        eprintln!("clicker probe: cannot create {out:?}: {error}");
        return false;
    }
    let paths = [
        out.join("timeline.jsonl"),
        out.join("snapshot.jsonl"),
        out.join("timing.json"),
        out.join("stdout.log"),
        out.join("stderr.log"),
        out.join("probe-run.json"),
        out.join("report.html"),
        out.join("trace.json"),
    ];
    for path in &paths {
        let _ = std::fs::remove_file(path);
    }

    let mut command = example_command(example, options);
    command
        .env("CLICKER_PROBE_TIMELINE", &paths[0])
        .env("CLICKER_PROBE_SNAPSHOT", &paths[1])
        .env("CLICKER_PROBE_TIMING", &paths[2])
        .env("TRACE_CHROME", &paths[7]);

    let output = match command.output() {
        Ok(output) => output,
        Err(error) => {
            eprintln!("clicker probe: cannot launch example {example:?}: {error}");
            return false;
        }
    };
    let _ = std::fs::write(&paths[3], &output.stdout);
    let _ = std::fs::write(&paths[4], &output.stderr);
    if output.status.success() {
        collect_timing_runs(example, options, out, &paths[2]);
    }
    let artifacts_exist = paths[..3].iter().all(|path| path.exists()) && paths[7].exists();
    let timing_complete = serde_json::from_str::<serde_json::Value>(&read(&paths[2]))
        .ok()
        .and_then(|timing| timing["complete"].as_bool())
        .unwrap_or(false);
    let success = output.status.success() && artifacts_exist;
    let metadata = serde_json::json!({
        "example": example,
        "norender": options.norender,
        "seed": options.seed,
        "deadline_secs": options.deadline,
        "success": success,
        "exit_code": output.status.code(),
        "artifacts_complete": artifacts_exist,
        "timing_complete": timing_complete,
    });
    if let Err(error) = std::fs::write(&paths[5], metadata.to_string()) {
        eprintln!("clicker probe: cannot write run metadata: {error}");
        return false;
    }
    let report = render_run_report(
        &metadata,
        &read(&paths[1]),
        &read(&paths[0]),
        &read(&paths[2]),
        &read(&paths[3]),
        &read(&paths[4]),
        paths[7].exists(),
    );
    if let Err(error) = std::fs::write(&paths[6], report) {
        eprintln!("clicker probe: cannot write report.html: {error}");
        return false;
    }
    if output.status.success() && !artifacts_exist {
        eprintln!("clicker probe: successful run did not produce all probe artifacts");
    }
    eprintln!(
        "clicker probe: {} {example}",
        if success { "PASS" } else { "FAIL" }
    );
    eprintln!("{}", paths[6].display());
    success
}

fn example_command(example: &str, options: &RunOptions) -> std::process::Command {
    let mut command = std::process::Command::new(env!("CARGO"));
    command
        .args(["run", "--quiet", "--features", "dev", "--example", example])
        .env("CLICKER_AUTOPILOT", "1")
        .env("CLICKER_AUTOPILOT_DEADLINE", options.deadline.to_string());
    if options.norender {
        command.env("CLICKER_NORENDER", "1");
    } else {
        command.env_remove("CLICKER_NORENDER");
    }
    if let Some(seed) = options.seed {
        command.env("CLICKER_SEED", seed.to_string());
    } else {
        command.env_remove("CLICKER_SEED");
    }
    command
}

fn collect_timing_runs(
    example: &str,
    options: &RunOptions,
    out: &std::path::Path,
    timing_path: &std::path::Path,
) {
    let Some(initial) = parse_json(timing_path) else {
        return;
    };
    if initial["complete"].as_bool().unwrap_or(false) {
        return;
    }
    let runs_dir = out.join("timing-runs");
    if std::fs::create_dir_all(&runs_dir).is_err() {
        return;
    }
    let _ = std::fs::write(runs_dir.join("0000.json"), initial.to_string());
    let mut reports = vec![initial];
    for run in 1..MAX_TIMING_RUNS {
        let path = runs_dir.join(format!("{run:04}.json"));
        let mut command = example_command(example, options);
        command
            .env("CLICKER_PROBE_TIMING", &path)
            .env(TIMING_WARMUP_ENV, "0")
            .env_remove("CLICKER_PROBE_TIMELINE")
            .env_remove("CLICKER_PROBE_SNAPSHOT")
            .env_remove("TRACE_CHROME");
        let Ok(output) = command.output() else {
            break;
        };
        if !output.status.success() {
            break;
        }
        let Some(report) = parse_json(&path) else {
            break;
        };
        let usable = report["frame_ms"]
            .as_array()
            .map_or(0, |frames| frames.len().saturating_sub(1));
        reports.push(report);
        let aggregate = aggregate_timing(&reports, TIMING_TARGET_UPDATES);
        if usable == 0 || aggregate["complete"].as_bool().unwrap_or(false) {
            let _ = std::fs::write(timing_path, aggregate.to_string());
            return;
        }
    }
    let aggregate = aggregate_timing(&reports, TIMING_TARGET_UPDATES);
    let _ = std::fs::write(timing_path, aggregate.to_string());
}

fn parse_json(path: &std::path::Path) -> Option<serde_json::Value> {
    serde_json::from_str(&read(path)).ok()
}

fn read(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

fn exit(success: bool) -> ExitCode {
    if success {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
