use std::{
   fs,
   io::{prelude::*, BufReader},
   net::{TcpListener, TcpStream},
   thread, time::Duration,
};

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
   println!("Server running on http://127.0.0.1:7878");

   for stream in listener.incoming() {
       let stream = stream.unwrap();
       handle_connection(stream);
   }
}

fn handle_connection(mut stream: TcpStream) {
   let buf_reader = BufReader::new(&mut stream);
   let request_line = buf_reader.lines().next().unwrap().unwrap();

   let (status_line, filename) = match &request_line[..] {
       "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
       "GET /sleep HTTP/1.1" => {
           thread::sleep(Duration::from_secs(10));
           ("HTTP/1.1 200 OK", "hello.html")
       }
       _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
   };

   let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
       "<h1>404 - Page Not Found</h1><p>Sorry, the page you requested does not exist.</p>".to_string()
   });

   let length = contents.len();

   let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

   stream.write_all(response.as_bytes()).unwrap();
}
