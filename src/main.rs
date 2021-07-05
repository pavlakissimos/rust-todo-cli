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
                if let Some(name) = utils::get_user_input("\nAdd a new todo.\n") {
                    if name.len() != 0 {
                        todos.push(Todo::new(&name));
                    } else {
                        println!("No empty todos allowed\n");
                    }
                }
            }
            Some(utils::Command::Delete) => {
                if todos.len() == 0 {
                    println!("Yoy haven't any todos to delete.\n");
                } else {
                    let mut index: usize = 0;
                    for todo in todos.iter() {
                        index += 1;
                        println!("{}) {} - ID: {}", index, todo.name, todo.id);
                    }
                }

                if let Some(id) = utils::get_user_input("Type a todo's id to remove it\n") {
                    let todo_to_delete = todos
                        .clone()
                        .into_iter()
                        .filter(|todo| todo.id == id)
                        .next();

                    if let Some(todo) = todo_to_delete {
                        todos = todos.into_iter().filter(|todo| todo.id != id).collect();
                        println!("\nTodo with name '{}' deleted successfully\n", todo.name);
                    } else {
                        println!("\nYou have no todos with id: '{}'\n", id);
                    }
                }
            }
            Some(utils::Command::Edit) => {
                if todos.len() == 0 {
                    println!("Yoy haven't any todos to edit.\n");
                } else {
                    let mut index: usize = 0;
                    for todo in todos.iter() {
                        index += 1;
                        println!("{}) {} - ID: {}", index, todo.name, todo.id);
                    }
                }

                if let Some(todo_id) =
                    utils::get_user_input("\nEnter the todo's ID you want to edit.\n")
                {
                    if todos.iter().any(|todo| todo.id == todo_id) {
                        for todo in todos.iter_mut() {
                            if todo.id == todo_id {
                                let prompt = format!("Give '{}' a new name.", todo.name);

                                if let Some(new_name) = utils::get_user_input(prompt.as_str()) {
                                    todo.name = new_name;
                                }
                            } else {
                                println!("\nThere is no todo with that name.\n");
                            }
                        }
                    }
                }
            }
        };
    }
}
