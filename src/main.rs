
mod get_opendatasoft_data;
mod count_activite_principale_etablissement;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Add /ape/{lat}/{long}/{distance} in url to get APE code counts")
}


#[get("/ape/{lat}/{lon}/{distance}")]
async fn ape(path: web::Path<(String, String, String)>) -> impl Responder {
    let (lat, long, distance) = path.into_inner();

    if let Ok(data) = get_opendatasoft_data::get_opendatasoft_data(lat.to_string(), long.to_string(), distance.to_string()).await {
        let count = count_activite_principale_etablissement::count(&data);
        let mut response = String::new();

        for (key, value) in count.iter() {
            response += &format!("{}: {}\n", key, value);
        }

        if response.is_empty() {
            response = "No data found".to_string();
        }

        HttpResponse::Ok().body(response)
    } else {
        HttpResponse::Ok().body("Error: unable to fetch data from opendatasoft")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello).service(ape)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
