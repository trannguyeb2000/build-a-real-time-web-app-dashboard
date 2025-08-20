use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::scope;
use serde::{Deserialize, Serialize};
use tokio_pg::NoTls;

// Define API models
#[derive(Debug, Serialize, Deserialize)]
struct DashboardData {
    cpu_usage: f64,
    memory_usage: f64,
    disk_usage: f64,
}

// Define API endpoints
async fn get_dashboard_data() -> impl Responder {
    // Database connection
    let (client, connection) = tokio_pg::connect("host=localhost user=postgres", NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("error: {}", e);
        }
    });

    // Mock data for demonstration purposes
    let data = DashboardData {
        cpu_usage: 0.5,
        memory_usage: 0.7,
        disk_usage: 0.3,
    };

    HttpResponse::Ok().json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                scope("/api")
                    .service(web::resource("dashboard").route(web::get().to(get_dashboard_data))),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}