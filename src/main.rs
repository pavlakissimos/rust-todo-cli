pub mod todo;
pub mod utils;

use todo::Todo;

fn main() {
    let mut todos: Vec<Todo> = vec![];

    loop {
        if todos.len() == 0 {
            println!("Yoy haven't any todos...\n");
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
                match utils::get_user_input("\nAdd a new todo.\n") {
                    Ok(val) => {
                        if val.len() != 0 {
                            todos.push(Todo::new(&val));
                        } else {
                            println!("No empty todos allowed\n");
                        }
                    }
                    Err(error) => panic!("Panicked with error: {}", error),
                };
            }
            Some(utils::Command::Delete) => {
                let name = utils::get_user_input("Type a todo's name to remove it\n").unwrap();

                if todos.iter().any(|todo| todo.name == name) {
                    todos = todos.into_iter().filter(|todo| todo.name != name).collect();
                    println!("\nTodo with name '{}' removed successfully\n", name);
                } else {
                    println!("\nYou have no todos with name '{}'\n", name);
                }
            }
        };
    }
}
