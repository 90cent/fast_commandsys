pub mod commandbase {
    use std::{fmt, io::Error, iter::FromFn, process::CommandArgs, thread};
    use tokio;

    pub struct Command {
        command_name: String,
        command_description: String,
        command_action: Box<dyn Fn() -> () + Send + 'static + Sync>
    }

    pub trait CommandExecuter { 
        fn execute(self);  // FUNC wird so irgendwie in der methode nochmal definiert. Rust halt 
    }

    impl CommandExecuter for Command{
        fn execute(self) // Neuen Thread mit der Funktion erstellen
        {                         
            let action = self.command_action;                            
            let thread = thread::spawn(action);  
            thread.join().unwrap();                                     
        }
    }
    
    impl fmt::Display for Command {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
            write!(f,"Command Name:     {}\nDescription:    '{}' ",&self.command_name,&self.command_description)
        }
    }

    impl Command {
        pub fn new(name: &'static str, desc: &'static str,action: impl Fn() -> () + Send + 'static + Sync) -> Self {
            return  Command {
                command_name: name.into(),
                command_description: desc.into(),
                command_action: Box::new(action)
            };
        }
    }
}

pub mod logging {
    use termcolor::{self, Color};
    use std::io::{Write};


    pub enum LogLevel {
        NORMAL,
        INFORMATION,
        WARNING,
        EXCEPTION
    }

    struct Log {
        prefix: String,
        message: String,
        color: Color
    }

    fn create_log_prefix() -> String {        // Macht ein prefix das so aussehen soll: [ZEIT UND DATUM - CommandName] 

    }

    pub fn log(level: LogLevel,message: String) {  // IMPORTANT: Include fmt::Display for Log to make it better and simpler
        let color = match level {
            LogLevel::NORMAL => {
                Log {
                    prefix: "[]".into(),
                    message: "".into(),
                    color: termcolor::Color::White,
                }
            },
            LogLevel::INFORMATION => termcolor::Color::Green,
            LogLevel::WARNING => termcolor::Color::Magenta,
            LogLevel::EXCEPTION => todo!(),
        };
    }
}