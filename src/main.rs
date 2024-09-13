use task::{StatusTask, Task};
pub mod task;

use clap::{Arg, Command, Parser};

struct Args {
    command: Command,
}

enum Comands {
    Add,
}

fn main() {
    // [INFO] Option que vai vir do usuário
    //
    // tdo add -n "Lavar o carro" -d "Lavar o carro, pr muié n brigar"
    // tdo add -n "Lavar o carro"

    // let task = Task::new(
    //     String::from("Lavar o carro"),
    //     Some(String::from("Lavar o carro, pr muié n brigar")),
    //     StatusTask::Pendente,
    // );

    // task.add();

    // [FIX!]

    let args = Args::parse();
}
