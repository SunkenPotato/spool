use serde::Deserialize;

use crate::{AppState, ASSET_PATH};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
    sync::LazyLock,
};

pub static DOC_PATH: LazyLock<String> = LazyLock::new(|| format!("{}/commands.json", &*ASSET_PATH));

pub struct CommandRegistry {
    commands: Vec<Command>,
    command_meta: HashMap<String, CommandMeta>,
}

#[derive(Deserialize)]
pub struct CommandMeta {
    doc: String,
}

impl CommandRegistry {
    pub fn new() -> std::io::Result<Self> {
        let input = || {
            let mut file = match File::open(Path::new(&*DOC_PATH)) {
                Ok(v) => v,
                Err(_) => {
                    return String::new();
                }
            };

            let mut buf = String::new();
            let _ = file.read_to_string(&mut buf);
            buf
        };
        let command_meta: HashMap<String, CommandMeta> =
            serde_json::from_str(&input()).unwrap_or(HashMap::new());

        Ok(Self {
            commands: Vec::new(),
            command_meta,
        })
    }

    pub fn register(&mut self, c: Command) -> &mut Self {
        self.commands.push(c);
        self
    }

    pub fn execute(&self, id: &String, state: &mut AppState) -> Result<CommandReturns, ()> {
        let command = self.commands.iter().find(|c| &c.id == id).ok_or(())?;
        Ok(command.env_modifier.as_ref()(state, self))
    }
}

#[allow(unused)]
pub enum CommandReturns {
    Exit,
    None,
}

pub struct Command {
    id: String,
    env_modifier: Box<dyn Fn(&mut AppState, &CommandRegistry) -> CommandReturns>,
}

impl Command {
    pub fn new<F>(id: &'static str, f: F) -> Self
    where
        F: Fn(&mut AppState, &CommandRegistry) -> CommandReturns + 'static,
    {
        Self {
            id: format!("/{id}"),
            env_modifier: Box::new(f),
        }
    }

    pub fn get_doc<'s, 'r>(&'s self, reg: &'r CommandRegistry) -> Option<&'r String> {
        reg.command_meta.get(&self.id).map(|meta| &meta.doc)
    }
}

pub(super) fn register_default_commands(reg: &mut CommandRegistry) {
    let exit_command = Command::new("exit", |_, _| CommandReturns::Exit);
    let clr_env_cmd = Command::new("clr-env", |state, _| {
        state.env.store.clear();
        CommandReturns::None
    });

    let help_command = Command::new("help", |state, reg| {
        for command in &reg.commands {
            let _ = write!(
                state.stdout,
                "{}: {}\n",
                command.id,
                command.get_doc(reg).unwrap_or(&"N/A".into())
            );
        }
        CommandReturns::None
    });

    reg.register(exit_command)
        .register(clr_env_cmd)
        .register(help_command);
}
