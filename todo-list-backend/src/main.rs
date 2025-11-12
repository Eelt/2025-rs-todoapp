use std::{collections::BTreeMap, sync::Mutex};

use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, http, post, web};
use todo_list_common::TodoItem;

#[post("/insert")]
async fn insert(
    todo_list: web::Data<Mutex<BTreeMap<u32, TodoItem>>>,
    recieved_todo: web::Json<TodoItem>,
) -> impl Responder {
    let mut list = todo_list.lock().unwrap();
    let id = list.len() as u32; // assume essentially appending similar to a SQL db

    list.insert(id,recieved_todo.0); // add the new item

    HttpResponse::Ok()
}

#[get("/view")]
async fn view(
    todo_list: web::Data<Mutex<BTreeMap<u32, TodoItem>>>,
    id_to_view: web::Json<u32>,
) -> impl Responder {
    let list = todo_list.lock().unwrap();
    
    match list.get(&id_to_view.0) {
        Some(item) => HttpResponse::Ok().json(item),
        None => HttpResponse::NotFound().body(format!("Todo item with id {} not found", id_to_view.0)),
    }
}

#[get("/all")]
async fn fetch_all(
    todo_list: web::Data<Mutex<Vec<TodoItem>>>
) -> impl Responder {
    let list = todo_list.lock().unwrap();
    HttpResponse::Ok()
        .json(&*list)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Mutex is being used to avoid concurrency issues, if this was a regular eg db refrence, with no writes to the object itself,
    // the mutex lock would not be included, and would be more optimal for an application at scale.
    // BTreeMap is used here, because at lower n-counts, it's faster than a hashmap.
    let todo_from_disk = web::Data::new(Mutex::new(BTreeMap::<u32,TodoItem>::new()));

    HttpServer::new(move || App::new().wrap(
        Cors::default()
        .allowed_origin("http://localhost:8080") // Replace with your desired origin, specifically if served in the browser with this URL
        .allowed_methods(vec!["GET", "POST"]) // Specify the allowed HTTP methods
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT]) // Specify the allowed headers
        .max_age(usize::MAX) // Set the maximum age of the CORS options request
    )
        // Data
        .app_data(todo_from_disk.clone())
        // Services
        .service(fetch_all)
        .service(insert)
    )
        .bind("127.0.0.1:8081")?
        .run()
        .await
    
}