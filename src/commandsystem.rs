pub use loging::*;


pub mod commandbase {
    use std::{fmt, io::Error, iter::FromFn, process::CommandArgs, thread};
    use tokio;
    use lazy_static::*;

    use crate::commandsystem::loging::*;


    pub struct Command {
        command_name: String,
        command_description: String,
        command_action: Box<dyn Fn() -> () + Send + 'static + Sync>
    }

    pub trait CommandExecuter { 
        fn execute(self);
        fn execute_static(&'static self);
    }

    impl CommandExecuter for Command{
        fn execute(self) // Neuen Thread mit der Funktion erstellen
        {                         
            let action = self.command_action;                            
            let thread = thread::spawn(action);  

            match thread.join() {   // Check if thread can be created
                Ok(thread) => return thread,
                Err(e) => {log(LogLevel::EXCEPTION, format!("Failed to execute Command. {:#?}",&e));},
            };
        }

        fn execute_static(&'static self) // Neuen Thread mit der Funktion erstellen
        {                         
            let action = &self.command_action;                            
            let thread = thread::spawn(action);  

            match thread.join() {   // Check if thread can be created
                Ok(thread) => return thread,
                Err(e) => {log(LogLevel::EXCEPTION, format!("Failed to execute Command. {:#?}",&e));},
            };
        }
    }
    
    impl fmt::Display for Command {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
            write!(f,"Command Name:     {}\nDescription:    '{}' ",&self.command_name,&self.command_description)
        }
    }

    impl Command {
        pub fn new(name: &'static str, desc: &'static str,action: impl Fn() -> () + Send + 'static + Sync) {
            let cmd = Command {
                command_name: name.into(),
                command_description: desc.into(),
                command_action: Box::new(action)
            };
            manager::init(cmd)
        }
    }

    mod manager {
        use crate::commandsystem::commandbase::Command;
        use std::{collections::{self, HashMap}, hash::Hash, io::read_to_string, sync::{Mutex, MutexGuard}};
        use lazy_static::*;


        lazy_static! {
            pub static ref COMMANDS: Mutex<HashMap<String,Command>> = Mutex::new({
                HashMap::new()
            });
        }
        
        pub fn init(cmd: Command)
        {
            lazy_static::initialize(&COMMANDS);
            let name = cmd.command_name.clone();
            let command = cmd;

            &COMMANDS.lock().unwrap().insert(name, command);
        }

        pub fn get(name: String) -> Box<T> {
            lazy_static::initialize(&COMMANDS);

            let c = match &COMMANDS.lock().unwrap().get(&name) {
                Some(k) => k.to_owned(),
                None => todo!(),
            };

            return c;
        }
    }
}

pub mod loging {
    use termcolor::{self, Color};
    use std::io::{Write};
    use chrono::{Datelike, Timelike, Utc};

    pub enum LogLevel {
        NORMAL,
        INFORMATION,
        WARNING,
        EXCEPTION
    }

    pub struct Log {
        prefix: String,
        message: String,
        color: Color
    }

    fn create_log_prefix(label: &'static str) -> String {        // Macht ein prefix das so aussehen soll: [ZEIT UND DATUM - CommandName]
        let utc = Utc::now();
        let time = format!("{}:{}:{}",utc.hour(),utc.minute(),utc.second());
        let date =format!("{}.{}.{}",utc.date(),utc.month(),utc.year());

        return format!("[{} - {}] ({}):",time,date,label);
    }

    pub fn log(level: LogLevel,message: String) -> Log {  // IMPORTANT: Include fmt::Display for Log to make it better and simpler
        let logtolog = match level {
            LogLevel::NORMAL => {
                Log {
                    prefix: create_log_prefix(""),
                    message: message,
                    color: termcolor::Color::White,
                }
            },
            LogLevel::INFORMATION => {
                Log {
                    prefix: create_log_prefix("INFORMATION"),
                    message: message,
                    color: termcolor::Color::Magenta,
                }
            },
            LogLevel::WARNING => {
                Log {
                    prefix: create_log_prefix("WARN"),
                    message: message,
                    color: termcolor::Color::Yellow,
                }
            },
            LogLevel::EXCEPTION => {
                Log {
                    prefix: create_log_prefix("EXCEPTION"),
                    message: message,
                    color: termcolor::Color::Red,
                }
            },
        };

        return logtolog;
    }
}
