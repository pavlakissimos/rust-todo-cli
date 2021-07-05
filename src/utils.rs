use std::io;

pub enum Command {
    Add,
    Delete,
    Edit,
}

/// Get input from terminal.
pub fn get_user_input(prompt: &str) -> Option<String> {
    let mut input = String::new();

    println!("{}", prompt);

    match io::stdin().read_line(&mut input) {
        Ok(_) => Some(input.trim().to_string()),
        Err(_) => None,
    }
}

pub fn choose_command() -> Option<Command> {
    match get_user_input(
        "\n\n## Available commands:\n + -> Add todo | - -> Delete todo | E -> Edit todo",
    )
    .unwrap()
    .trim()
    {
        "+" => Some(Command::Add),
        "-" => Some(Command::Delete),
        "E" => Some(Command::Edit),
        _ => {
            println!("Not an available command. Try again...\n");
            None
        }
    }
}
