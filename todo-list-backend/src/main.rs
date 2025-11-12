use std::{collections::BTreeMap, fs::{self, File}, io::Write, path::Path, sync::Mutex};

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, http, post, put, web};
use todo_list_common::{TodoItem};

async fn initial_loading_from_disk() -> BTreeMap<u32, TodoItem> {
    let storage_path = "./.storage/todo_list.json";

    // Ensure the directory exists
    if let Some(parent) = Path::new(storage_path).parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // If file does not exist, create it with empty JSON object
    if !Path::new(storage_path).exists() {
        let mut file = File::create(storage_path).unwrap();
        file.write_all(b"{}").unwrap();
        println!("Created empty todo list at {}", storage_path);
    }

    // Open file and deserialize into BTreeMap
    let file = File::open(storage_path).unwrap();
    let todo_map: BTreeMap<String, TodoItem> =
        serde_json::from_reader(file).unwrap_or_default();

    // Convert string keys to u32
    let mut result = BTreeMap::new();
    for (key_str, value) in todo_map {
        if let Ok(key) = key_str.parse::<u32>() {
            result.insert(key, value);
        }
    }

    result
}

async fn update_on_disk(todo_map: &BTreeMap<u32, TodoItem>) -> Result<(), Box<dyn std::error::Error>> {
    let storage_path = "./.storage/todo_list.json";

    // Convert BTreeMap<u32, TodoItem> into BTreeMap<String, TodoItem>
    let todo_map_string_keys: BTreeMap<String, &TodoItem> = todo_map
        .iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

    // Serialize map to pretty JSON
    let json_data = serde_json::to_string_pretty(&todo_map_string_keys)?;

    // Overwrite the file
    let mut file = File::create(storage_path)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

// To minimize complexity, I opted not to include a new struct that would take in the field
// to update, and the data itself. This is mainly to avoid the complexity of decoding
// the type of data dynamically. Since I'm not concerned about performance in this case,
// I am not concerned about resending the entire todo entry back with updated data.
// In Rust, this function is technically redundant, as reinserting will override +
// pop the old hashmap or btreemap entry out.
#[put("/update/{id}")]
async fn update_value(
    todo_list: web::Data<Mutex<BTreeMap<u32, TodoItem>>>,
    id: web::Path<u32>,
    recieved_todo: web::Json<TodoItem>,
) -> impl Responder {
    let mut list = todo_list.lock().unwrap();

    let _original_data = list.insert(*id,recieved_todo.0);

    update_on_disk(&list).await.unwrap();

    HttpResponse::Ok()
}

#[delete("/delete/{id}")]
async fn delete(
    todo_list: web::Data<Mutex<BTreeMap<u32, TodoItem>>>,
    id: web::Path<u32>,
) -> impl Responder {
    let mut list = todo_list.lock().unwrap();

    let _removed_data = list.remove(&id);

    update_on_disk(&list).await.unwrap();

    HttpResponse::Ok()
}


#[post("/insert")]
async fn insert(
    todo_list: web::Data<Mutex<BTreeMap<u32, TodoItem>>>,
    recieved_todo: web::Json<TodoItem>,
) -> impl Responder {
    let mut list = todo_list.lock().unwrap();
    let id = list.len() as u32; // assume essentially appending similar to a SQL db

    list.insert(id,recieved_todo.0); // add the new item

    update_on_disk(&list).await.unwrap();

    HttpResponse::Ok()
}

#[get("/view/{id}")]
async fn view(
    todo_list: web::Data<Mutex<BTreeMap<u32, TodoItem>>>,
    id_to_view: web::Path<u32>,
) -> impl Responder {
    let list = todo_list.lock().unwrap();

    match list.get(&id_to_view) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().body(format!("Todo item with id {} not found", id_to_view)),
    }
}

#[get("/list")]
async fn list_all(
    todo_list: web::Data<Mutex<BTreeMap<u32, TodoItem>>>
) -> impl Responder {
    let list = todo_list.lock().unwrap();
    HttpResponse::Ok()
        .json(&*list)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Initially, load from disk!
    let todo_from_disk = initial_loading_from_disk().await;

    // Mutex is being used to avoid concurrency issues, if this was a regular eg db refrence, with no writes to the object itself,
    // the mutex lock would not be included, and would be more optimal for an application at scale.
    // BTreeMap is used here, because at lower n-counts, it's faster than a hashmap.
    let todo_data = web::Data::new(Mutex::new(todo_from_disk));

    HttpServer::new(move || App::new().wrap(
        Cors::default()
        .allowed_origin("http://localhost:8080") // Replace with your desired origin, specifically if served in the browser with this URL
        .allowed_methods(vec!["GET", "POST"]) // Specify the allowed HTTP methods
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT]) // Specify the allowed headers
        .max_age(usize::MAX) // Set the maximum age of the CORS options request
    )
        // Data
        .app_data(todo_data.clone()) // This acts similarly to a global variable
        // Services (API Endpoints)
        .service(list_all)
        .service(insert)
        .service(view)
        .service(update_value)
        .service(delete)
    )
        .bind("127.0.0.1:8081")?
        .run()
        .await
    
}