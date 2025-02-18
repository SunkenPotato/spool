mod command;

use std::io::{self, Write};

use command::{register_default_commands, CommandRegistry};
use spool::{Env, Parsed};

const PROMPT: &str = "→ ";

fn main() -> io::Result<()> {
    let mut command_registry = CommandRegistry::default();
    register_default_commands(&mut command_registry);

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut env = Env::new();

    loop {
        let mut input = String::new();
        write!(stdout, "{}", PROMPT)?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;
        input = input.trim().into();

        if input.is_empty() {
            continue;
        }

        if let Ok(returns) = command_registry.execute(&input, &mut env) {
            match returns {
                command::CommandReturns::Exit => return Ok(()),
                _ => (),
            }
        }

        let (s, parsed) = match Parsed::parse(&input) {
            Ok(v) => v,
            Err(e) => {
                write!(stderr, "Parse error: {:?}\n", e)?;
                stderr.flush()?;
                continue;
            }
        };

        if !s.is_empty() {
            write!(stderr, "Input not fully consumed by parser: {s}\n")?;
            stderr.flush()?;
            continue;
        }

        let eval = match parsed.eval(&mut env) {
            Ok(v) => v,
            Err(e) => {
                write!(stderr, "Evaluation error: {:?}\n", e)?;
                stderr.flush()?;
                continue;
            }
        };

        write!(stdout, "{:?}\n", eval)?;
        stdout.flush()?;
    }
}
