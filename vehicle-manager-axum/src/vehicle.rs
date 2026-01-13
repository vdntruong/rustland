use std::collections::HashMap;
use axum::extract::Query;
use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: Option<String>,
    pub manufacturer: String,
    pub model_name: String,
    pub year: u32,
}

#[axum::debug_handler]
pub async fn handle_get_vehicle(Query(params): Query<HashMap<String, String>>) -> Json<Vehicle> {
    println!("getting vehicle");

    if params.get("year").is_some() && params.get("year").unwrap() == "2025" {
        return Json(
            Vehicle{
                id: Some(uuid::Uuid::new_v4().to_string()),
                manufacturer: "Dodge".to_string(),
                model_name: "RAM".to_string(),
                year: 2025,
            }
        )
    }

    Json(
        Vehicle{
            id: Some(uuid::Uuid::new_v4().to_string()),
            manufacturer: "Dodge".to_string(),
            model_name: "RAM".to_string(),
            year: 2024,
        }
    )
}

pub async fn handle_post_vehicle(Json(mut vehicle): Json<Vehicle>) -> Json<Vehicle> {
    println!("posting vehicle");
    vehicle.id = Some(uuid::Uuid::new_v4().to_string());
    Json(vehicle)
}
