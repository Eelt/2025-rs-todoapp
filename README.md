

# Run the backend:
```
$ cargo run --release -p todo-list-backend
```

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
curl http://127.0.0.1:8081/list
```