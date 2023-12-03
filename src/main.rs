use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello_project::ThreadPool;
// use hello_project::ThreadPool;
fn main() {
    //This(Local ip address ) will listen incoming stream when it gets incoming stream, it will print `connection established!`.
    // HTTP accept this port 7878 and here bind function works like new function retun new tcp instance.
    // bind function return Result enum.
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    // Here listner.incoming() incoming method return a iterator-process sequence of element(stream)
    for stream in listner.incoming().take(4) {
        // if stream has an error unwrap method will terminate the program. unles print the Connection established! message.
        let stream = stream.unwrap();
        println!("Connection established!");
        // println!(" {:?} ", stream);
        // handle_connection(stream);

        // implementing threadpool for being single threaded server to Multithreaded server
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });


        pool.execute(|| {
            handle_connection(stream)
        });
    }
}
// handle incomming request-read stream and read from the browser.
fn handle_connection(mut stream: TcpStream) {
    // create a bufreader instace with fn new
    let buf_reader = BufReader::new(&mut stream);
    // here we will read buf_reade line by line then unwrap if error in stream then filter after that collect
    // let http_request :Vec<_>= buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
    // collect the line of http requests browser send to server. BufReader is trait provide line method return an iterator
    // println!("Reqeusts buf_reader line: {:#?} line", http_request); // Browser requests headers
    let request_line = buf_reader.lines().next().unwrap().unwrap(); // next return Option<Result<String, Error>>, first Unwrap return Result and second unwrapr reurn String
    println!("Request line is {}", request_line); // GET / HTTP/1.1

    // request_line contains html request headers
    // In this sectio a lot of repetition
    // if request_line == "GET / HTTP/1.1" {
    //     let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let length = contents.len();
    //     let response =
    //         format!("{status_line}\r\nContent-lenght:{length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();

    // }else {
    //     let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();
    //     let response =
    //         format!("{status_line}\r\nContent-lenght:{length}\r\n\r\n{contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // }

    // remove repetition using if let statement

    // let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // }else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };
    // let contents = fs::read_to_string(filename).unwrap();
    // let length  = contents.len();

    // let response =
    //     format!("{status_line}\r\nContent-lenght: {length}\r\n\r\n{contents}");

    // stream.write_all(response.as_bytes()).unwrap();

    /*
    * here this is a single thread server when we /sleep request it will sleep for a provided time
    * duration after that it will response another request sequentially.
    * Connection established!
       Request line is GET /ffgff HTTP/1.1
       Connection established!
       Request line is GET /sleep HTTP/1.1
       Connection established!
       Request line is GET /ffgff HTTP/1.1
       Connection established!
       Request line is GET / HTTP/1.1
    */
    // Single threaded Server into a Multithreaded server
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(50));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
