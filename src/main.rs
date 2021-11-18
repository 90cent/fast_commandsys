use std::{thread};

mod commandsystem;
use commandsystem::commandbase::*;


fn main() {
    let mut cmd = Command::new("test", "test",testo);
    println!("{}",&cmd);
    cmd.execute();
}

fn testo() {
    println!("kekw");
}

    