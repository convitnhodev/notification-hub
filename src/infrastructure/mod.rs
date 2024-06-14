use std::{env, net::TcpListener};


use actix_web::{dev::Server, middleware::Logger}; 
use actix_web::{web, App, HttpServer};


use crate::adapters::{
    self, 
};


pub fn server(listener: TcpListener, db_name: &str) -> Result<Server, std::io::Error> {
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");


    env_logger::try_init();

    let db_connection = DbConnection{db_name: db_name.to_string()};
    let http_connection = HttpConnection{};



    let port = listener.local_addr().unwrap().port();


    let server = HttpServer::new(move || App::new().app_data().wrap(Logger::default())).configure().listen(listener)?.run();

    println!("Server running on port {}, db_name {}", port, db_name);

    Ok(server);

}