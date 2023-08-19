use std::thread;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use tokio::net::TcpListener as TokioTcpListener;
use tokio::prelude::*;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};

// Concurrency and Parallelism
fn concurrency_example() {
    let data = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let mut data = data.lock().unwrap();
                *data += 1;
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

// Networking with Rust
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    stream.write(b"Hello from the server").unwrap();
}

fn networking_example() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

// Asynchronous programming with Rust
async fn async_example() {
    let listener = TokioTcpListener::bind("127.0.0.1:8080").await.unwrap();

    while let Ok((mut socket, _)) = listener.accept().await {
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            socket.read(&mut buffer).await.unwrap();
            socket.write_all(b"Hello from Tokio server").await.unwrap();
        });
    }
}

// Final Project: Create a complete REST API
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the REST API")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn main() {
    concurrency_example();
    networking_example();

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        async_example().await;
    });

    main().unwrap();
}
