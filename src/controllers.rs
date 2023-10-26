use std::sync::{Arc, Mutex};

use actix_web::{get, post, put, delete, Responder, HttpResponse, web};
use uuid::Uuid;

use crate::{models::{ReviewCreation, Stars, Review}, db::MyDatabase};

#[get("/")]
pub async fn get_all_reviews(data: web::Data<Arc<Mutex<MyDatabase>>>) -> impl Responder {
    if let Ok(database) = data.lock() {
        HttpResponse::Ok().json(database.get_all())
    } else {
        HttpResponse::InternalServerError().body("F")
    }
}

#[get("/{id}")]
pub async fn get_review_by_id(path: web::Path<Uuid>, data: web::Data<Arc<Mutex<MyDatabase>>>) -> impl Responder {
    let id = path.into_inner();
    if let Ok(database) = data.lock() {
        match database.get_by_id(id) {
            Some(review) => {
                HttpResponse::Ok().json(serde_json::json!({
                    "review": review,
                    "status": "success",
                }))
            }
            None => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                }))
            }
        }
    } else {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
        }))
    }
}

#[post("/")]
pub async fn create_review(body: web::Json<ReviewCreation>, data: web::Data<Arc<Mutex<MyDatabase>>>) -> impl Responder {
    let new_review = body.into_inner();
    if let Ok(mut database) = data.lock() {
        database.insert_review(&new_review);
        HttpResponse::Ok().json(serde_json::json!(new_review))
    } else {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
        }))
    }
}

#[put("/{id}")]
pub async fn modify_review(path: web::Path<Uuid>, body: web::Json<ReviewCreation>, data: web::Data<Arc<Mutex<MyDatabase>>>) -> impl Responder {
    let id = path.into_inner();
    let review_update_data = body.into_inner();
    if let Ok(mut database) = data.lock() {
        match database.modify_review(id, review_update_data) {
            Some(review) => {
                HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "review": review,
                }))
            }
            None => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                }))
            }
        }
    } else {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
        }))
    }
}

#[delete("/{id}")]
pub async fn delete_review(path: web::Path<Uuid>, data: web::Data<Arc<Mutex<MyDatabase>>>) -> impl Responder {
    let id = path.into_inner();
    if let Ok(mut database) = data.lock() {
        match database.delete_review(id) {
            Some(id) => {
                HttpResponse::Ok().json(serde_json::json!({
                    "status": "success",
                    "deleted_review_id": id,
                }))
            }
            None => {
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                }))
            }
        }
    } else {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
        }))
    }
}

#[get("/stars/{ammount_of_stars}")]
pub async fn get_with_stars_filter(path: web::Path<Stars>, data: web::Data<Arc<Mutex<MyDatabase>>>) -> impl Responder {
    if let Ok(database) = data.lock() {
        let stars = path.into_inner();
        let mut final_vec: Vec<Review> = vec![];
        let vec_de_reviews = database.get_all();
        for review in vec_de_reviews {
            if review.get_stars() == stars {
                final_vec.push(review.clone());
            }
        }
        HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "list_of_reviews": final_vec,
        }))
    } else {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
        }))
    }
} 
