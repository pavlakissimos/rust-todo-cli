pub mod todo;
pub mod utils;

use todo::{add_todo, Todo};

fn main() {
    let mut todos: Vec<Todo> = vec![];

    loop {
        if todos.len() == 0 {
            println!("Yoy haven't any todos yet...\n");
        } else {
            let mut index: usize = 0;

            for todo in todos.iter() {
                index += 1;
                println!("{}) {}", index, todo.name);
            }
        }

        match utils::choose_command() {
            None => (),
            Some(utils::Command::Add) => {
                add_todo(&mut todos);
            }
            Some(utils::Command::Delete) => println!("Not implemented yet."),
        };
    }
}
