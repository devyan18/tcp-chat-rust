use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

use serde::{Deserialize, Serialize};
use colored::*;

// ...

fn handle_client(mut stream: TcpStream, clients: &mut Vec<TcpStream>) {
    // ...

    let username = stream.peer_addr().unwrap().to_string();

    println!("{} has joined the chat!", username.green());

    // ...

    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[0..n]).to_string();
                let message_json = json!({"username": username, "message": message}).to_string();
                for mut client in clients.iter() {
                    if client.peer_addr().unwrap() != stream.peer_addr().unwrap() {
                        client.write_all(message_json.as_bytes()).unwrap();
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
    }

    println!("{} has left the chat!", username.red());

    // ...
}


#[derive(Serialize, Deserialize)]
struct Message {
    sender: String,
    content: String,
}

// fn handle_client(stream: TcpStream, clients: Arc<Mutex<Vec<TcpStream>>>) {
//     let client_address = stream.peer_addr().unwrap();

//     println!("{}: {}", client_address, "Client connected");

//     let mut reader = BufReader::new(&stream);
//     let mut line = String::new();


//     // Leer el nombre del cliente y mostrarlo en la consola
//     reader.read_line(&mut line).unwrap();
//     let client_name = line.trim().to_owned();


//     loop {
//         line.clear();
//         let bytes_read = reader.read_line(&mut line).unwrap();
//         if bytes_read == 0 {
//             println!("Client {} disconnected", client_name);
//             let mut clients = clients.lock().unwrap();
//             clients.retain(|c| c.peer_addr().unwrap() != stream.peer_addr().unwrap());

//             break;
//         }

//         let message = Message {
//             sender: client_name.clone(),
//             content: line.trim().to_owned(),
//         };

//         println!("{}: {}", client_name, message.content);

//         let clients = clients.lock().unwrap();
//         for client in clients.iter() {
//             let mut client = client.try_clone().expect("Failed to clone client.");

//             if client.peer_addr().unwrap() != stream.peer_addr().unwrap() {
//                 let formated_message = format!("{}: {}", message.sender, message.content);

//                 client.write_all(formated_message.as_bytes()).unwrap();

//                 client.write_all(b"\n").unwrap();

//                 client.flush().unwrap();
//             }
//         }
//     }
// }

fn main() {
    let listener = TcpListener::bind("localhost:8888").unwrap();
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let clients = clients.clone();
        let stream = stream.unwrap();
        clients.lock().unwrap().push(stream.try_clone().unwrap());
        thread::spawn(move || {
            handle_client(stream, clients);
        });
    }
}
