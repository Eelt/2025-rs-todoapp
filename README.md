# Initial Setup
Make sure you have `rustup` installed, you can follow instructions for your system here:
https://rustup.rs/

## Dependencies
While Rust dependencies are mostly pulled from crates.io (your initial compile time will be long due to the need to compile all the libraries in use), 
some special cases exist, such as for OpenSSL, the cc linker, etc.
### OpenSSL / libssl-devel
Because we are dealing with HTTP requests, you will need the OpenSSL dev package for your system installed to compile correctly.
On Debian/Ubuntu:
```
sudo apt install libssl-dev
```
On RHEL
```
sudo dnf install openssl-devel
```

Or equivalent for your system.

### cc / linker
On Debian/Ubuntu:
```
sudo apt install binutils
sudo apt install build-essential
```

On RHEL:
```
sudo dnf install binutils
sudo dnf install gcc gcc-c++ make
```

Or equivalent for your system.


# Run the backend:
```
$ cargo run --release -p todo-list-backend
```

The backend will run on localhost on port 8081. It can be changed to 8080 directly in the codebase, as while I could've (and probably should've) used dotenv and env vars, I did
not in this case for brevity/time constraints. Although that wouldn't be really that difficult to implement aside from some minor refactoring in the backend's main function.


### CLI Examples:

## Demo insertion:
```
curl -X POST http://127.0.0.1:8081/insert     -H "Content-Type: application/json"     -d '{ 
      "title": "Buy groceries",
      "description": "Milk, eggs, bread",
      "due_date": "2025-11-15T18:00:00Z",
      "created_at": "2025-11-12T14:30:00Z",
      "completed": false
    }'

curl -X POST http://127.0.0.1:8081/insert     -H "Content-Type: application/json"     -d '{ 
      "title": "Run errands",
      "description": "Shopping, clean kitchen, shovel snow",
      "due_date": "2025-11-19T23:59:59Z",
      "created_at": "2025-11-12T14:32:59Z",
      "completed": false
    }'
```
⚠️ The timestamps are client side authoritative. While this simplifies backend code, ideally this should perhaps be considered to be done on the backend.


## View a specific entry by id (u32):
Note: first entry will be of id 0
```
curl -X GET http://127.0.0.1:8081/view/0
```
Probably could be simpler for the end user if we changed this to decode the URL (eg /view/0, /view/1, etc). That's not hard to do in Actix, but to save time, I essentially am reusing the same style for decoding as with other functions that need more data from the user/client.

## List all:
```
curl -X GET http://127.0.0.1:8081/list
```

## Delete:
```
curl -X DELETE http://127.0.0.1:8081/delete/1
```

## Update

```
curl -X PUT http://127.0.0.1:8081/update/0 \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Run errands",
    "description": "Shopping, clean kitchen, shovel snow",
    "due_date": "2025-11-19T23:59:59Z",
    "created_at": "2025-11-12T14:32:59Z",
    "completed": true
  }'
```

# Run the CLI Application
Due to time constraints and bugs being encountered, for simplicity I decided to switch to a CLI app since it's a much easier client to implement. Here's how to get started:

```
cd todo-list-cli
cargo build --release
```

Or from the main workspace folder (root) it's recommended to use:
```
cargo run --release -p todo-list-cli
```

Then if you don't want to constantly run cargo commands, you will notice the path of the binary should look something like this:
```
target/release/todo-list-cli
```

And can be run like:
```
./target/release/todo-list-cli
```

## List
```
cargo run --release -p todo-list-cli list
```

Example output:
```
=== All Todos ===
[0] Run errands - Shopping, clean kitchen, shovel snow | Due: 2025-11-19T23:59:59Z | Completed: false
[1] Finish Rust project - Write Actix handler tests | Due: 2025-11-15T23:59:59Z | Completed: true
```

## View a specific TODO by ID:
```
cargo run --release -p todo-list-cli view 0
```

Example output:
```
TodoItem {
    title: "Run errands",
    description: "Shopping, clean kitchen, shovel snow",
    due_date: 2025-11-19T23:59:59Z,
    created_at: 2025-11-12T14:32:59Z,
    completed: false,
}
```

## Insert a todo:
```
cargo run --release -p todo-list-cli insert "Finish taxes" "Complete 2025 filing" "2025-11-25T23:59:59Z"

```

You should see 200 to indicate it's successful

## Update a todo:
```
cargo run --release -p todo-list-cli update <id> "<title>" "<description>" "<due_date>" <completed>

```

You should see 200 to indicate it's successful

## Mark a todo as completed:
```
cargo run --release -p todo-list-cli complete <id>
```

## Mark a todo as incomplete:
```
cargo run --release -p todo-list-cli incomplete <id>
```

## Delete a todo:
```
cargo run --release -p todo-list-cli delete <id>
```