#[cfg(not(target_arch = "wasm32"))]
use std::process::ExitCode;

#[cfg(not(target_arch = "wasm32"))]
use clap::Parser;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Parser)]
#[command(name = "clicker")]
struct Cli {
    #[arg(long)]
    norender: bool,
    #[cfg(feature = "dev")]
    #[command(subcommand)]
    command: Option<Command>,
}

#[cfg(all(not(target_arch = "wasm32"), feature = "dev"))]
#[derive(clap::Subcommand)]
enum Command {
    #[command(disable_help_flag = true)]
    Probe {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> ExitCode {
    let cli = Cli::parse();
    #[cfg(feature = "dev")]
    if let Some(Command::Probe { args }) = cli.command {
        return clicker_probe::native::main(&args);
    }
    let success = if cli.norender {
        game::headless_app().run().is_success()
    } else {
        game::app().run().is_success()
    };
    if success {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    game::app().run();
}
