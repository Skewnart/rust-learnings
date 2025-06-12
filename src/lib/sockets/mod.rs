// Exemple d'un serveur web reposant sur des sockets TCP entrants

use std::{
    fs, io::{prelude::*, BufReader}, net::{Shutdown, TcpListener, TcpStream}, thread, time::Duration
};

pub fn using_sockets() {
    launch_server();
}

fn launch_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server is running on localhost:7878");
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream : TcpStream) {
let buf_reader = BufReader::new(&stream);

// ## Pour lire toutes les lignes de la requête HTTP entrante :

    // let http_request : Vec<_> = buf_reader
    //     .lines()
    //     .map(|line| line.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("Request: {http_request:#?}");

// ## Ou juste la première ligne pour déterminer le type de requête :

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (_response_line, _html_file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/lib/sockets/views/hello.html"),
        "GET /sleep HTTP/1.1" => { 
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "src/lib/sockets/views/duration.html") }
        _ => ("HTTP/1.1 404 NOT FOUND", "src/lib/sockets/views/404.html")
    };

    let status_line = _response_line;
    let contents = fs::read_to_string(_html_file).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    
    stream.flush().unwrap();
    print!("Response sent. ");
    stream.shutdown(Shutdown::Both).unwrap();
    println!("Connection closed");
}