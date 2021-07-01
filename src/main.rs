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
                let name = utils::get_user_input("\nAdd a new todo.\n").unwrap();

                if name.len() != 0 {
                    todos.push(Todo::new(&name));
                } else {
                    println!("No empty todos allowed\n");
                }
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
            Some(utils::Command::Edit) => {
                let name =
                    utils::get_user_input("\nEnter the todo's name you want to edit\n").unwrap();

                if todos.iter().any(|todo| todo.name == name) {
                    for todo in todos.iter_mut() {
                        if todo.name == name {
                            let new_name = utils::get_user_input("\nEnter new name").unwrap();
                            todo.name = new_name;
                        }
                    }
                } else {
                    println!("\nThere is no todo with that name.\n")
                }
            }
        };
    }
}
