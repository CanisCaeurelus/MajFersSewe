use actix_web::{get,post,put,delete,web,Responder,HttpResponse};
use crate::{AppState, TodolistEntry};
use super::models::{CreateEntryData,UpdateEntryData};

#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}

#[post("/todolist/entries")]
async fn create_entry(data: web::Data<AppState>, param_obj: web::Json<CreateEntryData>) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let mut idx:i32 = 0;
    for entry in todolist_entries.iter_mut()
    {
        if idx < entry.id
        {
            idx = entry.id;   
        }
    }
    idx += 1;
    todolist_entries.push(TodolistEntry{
        id: idx,
        title: param_obj.title.clone(),
        date: param_obj.date,
    });

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[put("/todolist/entries/{id}")]
async fn update_entry(data: web::Data<AppState>, path: web::Path<i32>, param_obj: web::Json<UpdateEntryData>) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let idx:i32 = path.into_inner();
    for entry in todolist_entries.iter_mut()
    {
        if idx == entry.id
        {
            entry.title = param_obj.title.clone();
            break;
        } 
    }

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[delete("/todolist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let idx:i32 = path.into_inner();
    *todolist_entries = todolist_entries.to_vec().into_iter().filter(|x| x.id != idx).collect();

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

pub fn config_services(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries)
    .service(create_entry)
    .service(update_entry)
    .service(delete_entry);
}