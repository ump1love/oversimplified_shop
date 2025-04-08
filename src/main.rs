use std::{
    thread,
    io::{stdin, BufRead}
};
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_files::Files;
use console_output;

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!(".././static/index.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(Files::new("/static", "./static"))
    })
    .bind("0.0.0.0:8787")?
    .run();

    thread::spawn(|| {
        println!("Main menu.\nType \"help\" for more information");
        let mut buffer = String::new();
        let stdin = stdin();
        let mut handle = stdin.lock();
            loop {
                if let Ok(_) = handle.read_line(&mut buffer) {
                    let _ = buffer.trim();
                    console_output::args_handler(buffer.clone());
                    buffer = "".to_string();
                }
            }
    });

    server.await
}
