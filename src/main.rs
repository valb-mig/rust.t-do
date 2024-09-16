use clap::{Arg, ArgAction, Command};
use task::{StatusTask, Task};
pub mod task;


fn main() {

    let matches = Command::new("t-do")
        .version("0.1.0")
        .about("A simple task manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .short_flag('a')
                .long_flag("add")
                .about("Add a task")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                )
        )
        .get_matches();

    match matches.subcommand() {

        Some(( "add", add_match )) => {

            if add_match.contains_id("name") {
                
                let task_name: Vec<_> = add_match
                    .get_many::<String>("name")
                    .expect("Task Name")
                    .map(|s| s.as_str())
                    .collect();

                let values = task_name.join(", ");
                
                let task = Task::new(
                    values,
                    Some(String::from("Placeholder")),
                    StatusTask::Pendente
                );

                println!("{}", task.get_name());
            }
        }

        _ => unreachable!(),
    }
}
