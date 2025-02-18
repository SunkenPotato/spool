use spool::Env;

#[derive(Default)]
pub struct CommandRegistry {
    commands: Vec<Command>,
}

impl CommandRegistry {
    pub fn register(&mut self, c: Command) -> &mut Self {
        self.commands.push(c);
        self
    }

    pub fn execute(&self, id: &String, env: &mut Env) -> Result<CommandReturns, ()> {
        let command = self.commands.iter().find(|c| &c.id == id).ok_or(())?;
        Ok(command.env_modifier.as_ref()(env))
    }
}

#[allow(unused)]
pub enum CommandReturns {
    Exit,
    None,
}

pub struct Command {
    id: String,
    env_modifier: Box<dyn Fn(&mut Env) -> CommandReturns>,
}

impl Command {
    pub fn new<F>(id: &'static str, f: F) -> Self
    where
        F: Fn(&mut Env) -> CommandReturns + 'static,
    {
        Self {
            id: format!("/{id}"),
            env_modifier: Box::new(f),
        }
    }
}

pub(super) fn register_default_commands(reg: &mut CommandRegistry) {
    let exit_command = Command::new("exit", |_| CommandReturns::Exit);
    let clr_env_cmd = Command::new("clr-env", |e| {
        e.bindings.clear();
        CommandReturns::None
    });
    let help_command = Command::new("help", |_| {
        let _ = open::that("https://github.com/SunkenPotato/spool/wiki/CLI");
        CommandReturns::None
    });

    reg.register(exit_command)
        .register(clr_env_cmd)
        .register(help_command);
}
