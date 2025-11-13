mod todo_client;

use chrono::{DateTime, Utc};
use todo_list_common::TodoItem;
use std::env;
use todo_client::*;

/// CLI app to test Todo backend
///
/// Example usage:
/// ```bash
/// cargo run -- list
/// cargo run -- view 1
/// cargo run -- insert "Buy groceries" "Milk, eggs, bread" "2025-11-20T23:59:59Z"
/// cargo run -- update 1 "Do laundry" "Fold clothes" "2025-11-21T23:59:59Z" true
/// cargo run -- complete 1
/// cargo run -- incomplete 1
/// cargo run -- delete 1
/// ```

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "list" => {
            match list_todos() {
                Ok(map) => {
                    println!("=== All Todos ===");
                    for (id, item) in map {
                        println!(
                            "[{}] {} - {} | Due: {} | Completed: {}",
                            id,
                            item.title,
                            item.description,
                            item.due_date.to_rfc3339(),
                            item.completed
                        );
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        "view" => {
            if args.len() < 3 {
                eprintln!("Usage: view <id>");
                return;
            }
            let id = &args[2];
            match view_todo(id) {
                Ok(Some(todo)) => println!("{:#?}", todo),
                Ok(None) => println!("No todo found with id {}", id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        "insert" => {
            if args.len() < 4 {
                eprintln!("Usage: insert <title> <description> [due_date]");
                eprintln!("Example: insert \"Buy groceries\" \"Milk, eggs\" \"2025-11-20T23:59:59Z\"");
                return;
            }
            let title = &args[2];
            let description = &args[3];
            let now = Utc::now();

            // Optional due date
            let due_date: DateTime<Utc> = if args.len() > 4 {
                match args[4].parse() {
                    Ok(dt) => dt,
                    Err(_) => {
                        eprintln!("Invalid due_date format. Example: 2025-11-20T23:59:59Z");
                        return;
                    }
                }
            } else {
                now + chrono::Duration::days(3)
            };

            let new_todo = TodoItem {
                title: title.to_string(),
                description: description.to_string(),
                due_date,
                created_at: now,
                completed: false,
            };

            match insert_todo(&new_todo) {
                Ok(resp) => println!("Inserted successfully: {:?}", resp.status()),
                Err(e) => eprintln!("Error inserting: {}", e),
            }
        }

        "update" => {
            if args.len() < 6 {
                eprintln!("Usage: update <id> <title> <description> <due_date> <completed>");
                eprintln!("Example: update 1 \"Do laundry\" \"Fold clothes\" \"2025-11-21T23:59:59Z\" false");
                return;
            }
            let id = &args[2];
            let title = &args[3];
            let description = &args[4];
            let due_date = match args[5].parse::<DateTime<Utc>>() {
                Ok(dt) => dt,
                Err(_) => {
                    eprintln!("Invalid due_date format. Example: 2025-11-21T23:59:59Z");
                    return;
                }
            };
            let completed: bool = args[6].parse().unwrap_or(false);

            let now = Utc::now();
            let updated = TodoItem {
                title: title.to_string(),
                description: description.to_string(),
                due_date,
                created_at: now,
                completed,
            };

            match update_todo(id, &updated) {
                Ok(resp) => println!("Updated successfully: {:?}", resp.status()),
                Err(e) => eprintln!("Error updating: {}", e),
            }
        }

        "complete" | "incomplete" => {
            if args.len() < 3 {
                eprintln!("Usage: {} <id>", args[1]);
                return;
            }
            let id = &args[2];
            match view_todo(id) {
                Ok(Some(mut todo)) => {
                    todo.completed = args[1] == "complete";
                    match update_todo(id, &todo) {
                        Ok(resp) => println!(
                            "{} successfully: {:?}",
                            args[1].to_uppercase(),
                            resp.status()
                        ),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Ok(None) => println!("No todo found with id {}", id),
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        "delete" => {
            if args.len() < 3 {
                eprintln!("Usage: delete <id>");
                return;
            }
            let id = &args[2];
            match delete_todo(id) {
                Ok(resp) => println!("Deleted successfully: {:?}", resp.status()),
                Err(e) => eprintln!("Error deleting: {}", e),
            }
        }

        _ => print_usage(),
    }
}

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  list");
    eprintln!("  view <id>");
    eprintln!("  insert <title> <description> [due_date]");
    eprintln!("  update <id> <title> <description> <due_date> <completed>");
    eprintln!("  complete <id>");
    eprintln!("  incomplete <id>");
    eprintln!("  delete <id>");
    eprintln!();
    eprintln!("Example ISO 8601 datetime: 2025-11-20T23:59:59Z");
}
