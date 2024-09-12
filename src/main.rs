use clap::{Arg, Command};
use task::{StatusTask, Task};
pub mod task;

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

    let matches = Command::new("tdo")
        .version("0.1.0")
        .author("Valb")
        .about("Simple script to manage tasks")
        .arg(Arg::new("add").short('a').long("add"))
        .get_matches();
}
