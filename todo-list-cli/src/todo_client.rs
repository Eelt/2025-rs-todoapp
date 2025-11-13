use std::collections::BTreeMap;
use reqwest::blocking::{Client, Response};
use reqwest::header::CONTENT_TYPE;
use todo_list_common::TodoItem;

const BASE_URL: &str = "http://127.0.0.1:8081";

fn get_client() -> Client {
    Client::new()
}

pub fn insert_todo(item: &TodoItem) -> Result<Response, reqwest::Error> {
    get_client()
        .post(&format!("{}/insert", BASE_URL))
        .header(CONTENT_TYPE, "application/json")
        .json(item)
        .send()
}

pub fn update_todo(id: &str, item: &TodoItem) -> Result<Response, reqwest::Error> {
    get_client()
        .put(&format!("{}/update/{}", BASE_URL, id))
        .header(CONTENT_TYPE, "application/json")
        .json(item)
        .send()
}

pub fn delete_todo(id: &str) -> Result<Response, reqwest::Error> {
    get_client()
        .delete(&format!("{}/delete/{}", BASE_URL, id))
        .send()
}

pub fn view_todo(id: &str) -> Result<Option<TodoItem>, reqwest::Error> {
    let resp = get_client()
        .get(&format!("{}/view/{}", BASE_URL, id))
        .send()?;

    if resp.status().is_success() {
        Ok(Some(resp.json::<TodoItem>()?))
    } else {
        Ok(None)
    }
}

pub fn list_todos() -> Result<BTreeMap<String, TodoItem>, reqwest::Error> {
    let resp = get_client().get(&format!("{}/list", BASE_URL)).send()?;
    resp.json::<BTreeMap<String, TodoItem>>()
}
