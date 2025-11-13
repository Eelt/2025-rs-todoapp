mod todo_client;

use chrono::{DateTime, Utc};
use todo_list_common::TodoItem;
use std::env;
use todo_client::*;

/// CLI app to test Todo backend
///
/// Usage examples:
/// ```bash
/// cargo run -- list
/// cargo run -- view 1
/// cargo run -- insert "Buy groceries" "Milk, eggs, bread"
/// cargo run -- insert "Buy groceries" "Milk, eggs, bread" "2025-11-20T23:59:59Z"
/// cargo run -- update 1 "Buy groceries again" "Now with cheese" true
/// cargo run -- delete 1
/// ```
///
/// ISO 8601 datetime example:
/// `2025-11-20T23:59:59Z`

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  list");
        eprintln!("  view <id>");
        eprintln!("  insert <title> <description> [due_date]");
        eprintln!("  update <id> <title> <description> <completed>");
        eprintln!("  delete <id>");
        eprintln!();
        eprintln!("Example date format (ISO 8601): 2025-11-20T23:59:59Z");
        return;
    }

    match args[1].as_str() {
        // --- LIST ---
        "list" => {
            match list_todos() {
                Ok(map) => {
                    println!("=== All Todos ===");
                    for (id, item) in map {
                        println!(
                            "[{}] {} - {} (completed: {})",
                            id, item.title, item.description, item.completed
                        );
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        // --- VIEW ---
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

        // --- INSERT ---
        "insert" => {
            if args.len() < 4 {
                eprintln!("Usage: insert <title> <description> [due_date]");
                eprintln!("Example: insert \"Buy groceries\" \"Milk, eggs, bread\" \"2025-11-20T23:59:59Z\"");
                return;
            }

            let title = &args[2];
            let description = &args[3];
            let now = Utc::now();

            // Optional due date argument
            let due_date: DateTime<Utc> = if args.len() > 4 {
                match args[4].parse::<DateTime<Utc>>() {
                    Ok(parsed) => parsed,
                    Err(_) => {
                        eprintln!("Invalid due_date format.");
                        eprintln!("Expected ISO 8601 format, e.g. 2025-11-20T23:59:59Z");
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

        // --- UPDATE ---
        "update" => {
            if args.len() < 6 {
                eprintln!("Usage: update <id> <title> <description> <completed>");
                eprintln!("Example: update 1 \"Do laundry\" \"Fold clothes\" false");
                return;
            }

            let id = &args[2];
            let title = &args[3];
            let description = &args[4];
            let completed: bool = args[5].parse().unwrap_or(false);

            let now = Utc::now();
            let updated = TodoItem {
                title: title.to_string(),
                description: description.to_string(),
                due_date: now + chrono::Duration::days(1),
                created_at: now,
                completed,
            };

            match update_todo(id, &updated) {
                Ok(resp) => println!("Updated successfully: {:?}", resp.status()),
                Err(e) => eprintln!("Error updating: {}", e),
            }
        }

        // --- DELETE ---
        "delete" => {
            if args.len() < 3 {
                eprintln!("Usage: delete <id>");
                eprintln!("Example: delete 1");
                return;
            }
            let id = &args[2];

            match delete_todo(id) {
                Ok(resp) => println!("Deleted successfully: {:?}", resp.status()),
                Err(e) => eprintln!("Error deleting: {}", e),
            }
        }

        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}
