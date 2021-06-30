use std::io;

pub enum Command {
    Add,
    Delete,
}

/// Get input from terminal.
pub fn get_user_input(prompt: &str) -> io::Result<String> {
    let mut input = String::new();

    println!("{}", prompt);

    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

pub fn choose_command() -> Option<Command> {
    match get_user_input("\n\n## Available commands:\n + -> Add todo | - -> Delete ")
        .unwrap()
        .trim()
    {
        "+" => Some(Command::Add),
        "-" => Some(Command::Delete),
        _ => {
            println!("Not an available command. Try again...\n");
            None
        }
    }
}
