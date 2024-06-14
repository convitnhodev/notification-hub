use std::{env, io}; 
use std::net::TcpListener;



#[actix_web::main]
async fn main() -> io::Result<()> {
    let enviroment_file; 
    if let Ok(e) = env::var("ENV") {
        enviroment_file = format!(".env.{}", e);
    } else {
        enviroment_file = String::from(".env");
    }


    dotenv::from_filename(enviroment_file).ok();


    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind radom port");
    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be seted");

    run(listener, &database_name)?.await
}