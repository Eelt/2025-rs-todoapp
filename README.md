

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
```
⚠️ The timestamps are client side authoritative. While this simplifies backend code, ideally this should perhaps be considered to be done on the backend.


## View a specific entry by id (u32):
Note: first entry will be of id 0
```
curl -X GET http://127.0.0.1:8081/view \
     -H "Content-Type: application/json" \
     -d '0'
```

## View all:
```
curl http://127.0.0.1:8081/all
```