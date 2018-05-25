use std::collections::HashMap;

mod continue_command;
mod entry_command;
mod regs_command;
mod start_command;
mod step_command;

use api::continue_command::ContinueCommand;
use api::entry_command::EntryCommand;
use api::regs_command::RegsCommand;
use api::start_command::StartCommand;
use api::step_command::StepCommand;
use core::debugger::Debugger;
use util::errors::*;

pub trait Command {
    fn execute(&self, &[&str], debugger: &mut Debugger) -> RdbgResult<()>;
    fn usage(&self);
}

pub struct RdbgApi {
    commands: HashMap<&'static str, Box<Command>>,
    debugger: Debugger,
}

impl RdbgApi {
    pub fn new(program_path: String) -> RdbgApi {
        let mut commands: HashMap<&str, Box<Command>> = HashMap::new();

        commands.insert("entry", Box::new(EntryCommand));
        commands.insert("start", Box::new(StartCommand));
        commands.insert("continue", Box::new(ContinueCommand));
        commands.insert("regs", Box::new(RegsCommand));
        commands.insert("step", Box::new(StepCommand));

        RdbgApi {
            commands: commands,
            debugger: Debugger::new(program_path),
        }
    }

    pub fn run(&mut self, command: &str) -> RdbgResult<()> {
        // Split the input by spaces
        let v: Vec<&str> = command.split(' ').collect();

        match self.commands.get(v[0]) {
            Some(cmd) => {
                let mut args = v.as_slice();

                if 0 < args.len() {
                    args = &args[1..];
                }

                debug!("Calling \'{}\' command", v[0]);
                cmd.execute(args, &mut self.debugger)
            }
            None => RdbgApi::handle_unknown_command(v[0]),
        }
    }

    fn handle_unknown_command(cmd: &str) -> RdbgResult<()> {
        println!("Undefined command: \"{}\".  Try \"help\"", cmd);
        Ok(())
    }
}
