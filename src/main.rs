#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket_contrib::{Json, Value};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

type ID = String;

type PointOfSaleMap = Mutex<HashMap<ID, PointOfSale>>;

#[derive(Serialize, Deserialize, Clone)]
struct PointOfSale {
    id: ID,
    owner_name: String,
}

#[derive(Serialize, Deserialize)]
struct PointsOfSaleWrapper {
    #[serde(rename = "pdvs")]
    points_of_sale: Vec<PointOfSale>,
}

#[post("/", format = "application/json", data = "<point_of_sale>")]
fn new(point_of_sale: Json<PointOfSale>, map: State<PointOfSaleMap>) -> Json<Value> {
    let mut hashmap = map.lock().expect("map lock.");
    let cl = point_of_sale.0.clone();
    let id = cl.id;
    if hashmap.contains_key(&id) {
        Json(json!({
            "status": "error",
            "reason": "ID exists. Try put."
        }))
    } else {
        hashmap.insert(id, point_of_sale.0);
        Json(json!({ "status": "ok" }))
    }
}

#[get("/<id>", format = "application/json")]
fn get(id: ID, map: State<PointOfSaleMap>) -> Option<Json<PointOfSale>> {
    let hashmap = map.lock().unwrap();
    hashmap.get(&id).map(|pos| {
        Json(PointOfSale {
            id: id,
            owner_name: pos.owner_name.clone(),
        })
    })
}

#[get("/all", format = "application/json")]
fn list(map: State<PointOfSaleMap>) -> Json<PointsOfSaleWrapper> {
    let hashmap = map.lock().unwrap();
    let points = hashmap
        .values()
        .map(|p| p.clone())
        .collect::<Vec<PointOfSale>>();
    let wrapper = PointsOfSaleWrapper { points_of_sale: points };
    Json(wrapper)
}

#[error(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Resource was not found."
    }))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/point_of_sale", routes![new, get, list])
        .catch(errors![not_found])
        .manage(Mutex::new(HashMap::<ID, PointOfSale>::new()))
}

fn main() {
    rocket().launch();
}
