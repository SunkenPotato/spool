use std::io::{self, Write};

const PROMPT: &str = "→ ";

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let mut input = String::new();

    let mut global_env = spool::Env::default();

    loop {
        write!(stdout, "{}", PROMPT)?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        match spool::parse(&input.trim()) {
            Ok(parsed) => match parsed.eval(&mut global_env) {
                Ok(val) => println!("{val}"),
                Err(e) => {
                    writeln!(stderr, "{e}")?;
                    stderr.flush()?;
                }
            },
            Err(e) => {
                writeln!(stderr, "{e}")?;
                stderr.flush()?;
            }
        }

        input.clear();
    }
}
