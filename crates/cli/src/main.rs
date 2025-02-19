mod command;

use std::io::{self, Stderr, Stdin, Stdout, Write};

use command::{register_default_commands, CommandRegistry};
use spool::{Env, Parsed};

const PROMPT: &str = "→ ";
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(all(any(target_os = "linux", target_os = "macos"), not(debug_assertions)))]
pub static ASSET_PATH: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    "~/.local/share/spool"
        .to_string()
        .replace("~", &std::env::var("HOME").unwrap())
});
#[cfg(all(any(target_os = "linux", target_os = "macos"), debug_assertions))]
pub const ASSET_PATH: &str = "../../assets/";
#[cfg(target_os = "windows")]
const ASSET_PATH: &str = compile_error!("Windows is not supported yet");

pub(crate) struct AppState<'e> {
    pub(crate) stdin: Stdin,
    pub(crate) stdout: Stdout,
    pub(crate) stderr: Stderr,
    pub(crate) env: Env<'e>,
}

fn main() -> io::Result<()> {
    let mut app_state = AppState {
        stdin: io::stdin(),
        stdout: io::stdout(),
        stderr: io::stderr(),
        env: Env::new(),
    };

    let mut command_registry = CommandRegistry::new()?;

    register_default_commands(&mut command_registry);
    greeting(&mut app_state.stdout)?;

    loop {
        let mut input = String::new();
        write!(app_state.stdout, "{}", PROMPT)?;
        app_state.stdout.flush()?;

        app_state.stdin.read_line(&mut input)?;
        input = input.trim().into();

        if input.is_empty() {
            continue;
        }

        if let Ok(returns) = command_registry.execute(&input, &mut app_state) {
            match returns {
                command::CommandReturns::Exit => return Ok(()),
                _ => continue,
            }
        }

        let (s, parsed) = match Parsed::parse(&input) {
            Ok(v) => v,
            Err(e) => {
                write!(app_state.stderr, "Parse error: {:?}\n", e)?;
                app_state.stderr.flush()?;
                continue;
            }
        };

        if !s.is_empty() {
            write!(
                app_state.stderr,
                "Input not fully consumed by parser: {s}\n"
            )?;
            app_state.stderr.flush()?;
            continue;
        }

        let eval = match parsed.eval(&mut app_state.env) {
            Ok(v) => v,
            Err(e) => {
                write!(app_state.stderr, "Evaluation error: {:?}\n", e)?;
                app_state.stderr.flush()?;
                continue;
            }
        };

        write!(app_state.stdout, "{:?}\n", eval)?;
        app_state.stdout.flush()?;
    }
}

fn greeting(stdout: &mut Stdout) -> io::Result<()> {
    write!(stdout, "Spool {} on {}\n", VERSION, std::env::consts::OS)?;
    write!(stdout, "Type '/help' for a list of commands\n")?;

    Ok(())
}
