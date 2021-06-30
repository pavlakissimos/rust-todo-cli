use chrono::{DateTime, Utc};
use nanoid::nanoid;

#[derive(Debug, Clone)]
pub struct Todo {
  id: String,
  pub name: String,
  pub creation_date: DateTime<Utc>,
}

impl Todo {
  /// Creates a new Todo instance.
  pub fn new(name: &str) -> Todo {
    Todo {
      id: nanoid!(),
      name: String::from(name),
      creation_date: DateTime::from(Utc::now()),
    }
  }
}

#[cfg(test)]
mod todo_tests {
  use super::*;

  #[test]
  fn creates_new_todo() {
    let todo = Todo::new("Test todo");

    assert!(todo.id.len() > 0);
    assert!(todo.creation_date.timestamp() > 0);
    assert_eq!(todo.name, String::from("Test todo"))
  }
}
