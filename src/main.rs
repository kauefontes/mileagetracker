use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Debug, Clone, Serialize)]
struct Vehicle {
    id: i32,
    created: String,
    modified: String,
    name: String,
}

#[get("/")]
async fn index(
    tera: web::Data<Tera>,
    pool: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> impl Responder {
    let conn = pool.get().expect("Failed to get connection from pool");
    let mut context = Context::new();

    let mut stmt = conn
        .prepare("SELECT id, name, created, modified FROM vehicles")
        .expect("Failed to prepare query");

    let vehicle_iter = stmt
        .query_map([], |row| {
            Ok(Vehicle {
                id: row.get(0)?,
                name: row.get(1)?,
                created: row.get(2)?,
                modified: row.get(3)?,
            })
        })
        .expect("Failed to map query");

    let vehicles = vehicle_iter
        .filter_map(|x| x.ok())
        .collect::<Vec<Vehicle>>();

    context.insert("vehicles", &vehicles);

    //Render the template with the context
    let rendered = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("I'm Alive!!")
}

#[derive(Deserialize, Debug)]
struct MileageFormData {
    vehicle_id: i32,
    total_mileage: i32,
    odometer_start: i32,
    odometer_end: i32,
    notes: String,
}

#[post("/submit-mileage")]
async fn submit_mileage(
    form: web::Form<MileageFormData>,
    tera: web::Data<Tera>,
    pool: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> impl Responder {
    let conn = pool.get().expect(" Failed to get connection from pool");
    let context = Context::new();

    println!("{:?}", form);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(" submit mileage")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting a server at http://localhost:42069");
    let manager = SqliteConnectionManager::file("vehicles.db");
    let pool = r2d2::Pool::new(manager).expect("Failed to create pool.");

    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .service(index)
            .service(health)
            .service(submit_mileage)
    })
    .bind(("0.0.0.0", 42069))?
    .run()
    .await
}
