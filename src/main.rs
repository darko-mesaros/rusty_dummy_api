#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    status: String,
    message: String,
}
// This spoofs weather for the demo
#[get("/weather/<city>")]
fn get_weather(city: &str) -> Json<Response> {
    let weather = match city.to_lowercase().as_str() {
        "subotica" => "4C - Flurries",
        "maple_valley" => "41F - Rain",
        "xemxija" => "23C - Clear",
        "brno" => "-1C - Snow",
        _ => "NaN",

    };

    Json(Response {
        status: "success".to_string(),
        message: format!("The current weather in {} is {}", city, weather),
    })
}
#[get("/")]
fn index() -> Json<Response> {
    Json(Response {
        status: "success".to_string(),
        message: "Welcome to the API".to_string(),
    })
}
#[catch(404)]
fn not_found() -> Json<Response> {
    Json(Response {
        status: "error".to_string(),
        message: "Route not found".to_string(),
    })
}
#[launch]
fn rocket() -> _ {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");
    let figment = rocket::Config::figment()
        .merge(("address", "0.0.0.0"))
        .merge(("port", port));

    rocket::custom(figment)
        .mount("/", routes![index, get_weather])
        .register("/", catchers![not_found])
}
