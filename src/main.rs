use std::{io::{BufReader, Error, Read, Write}, net::{TcpListener, TcpStream}};

fn handle_client(mut stream: TcpStream) -> Result<bool, Error> {
  let mut request = String::new();
  
  let mut reader = BufReader::new(&stream);
  let mut buffer = [0; 255];

  loop {
    let n = match reader.read(&mut buffer) {
      Ok(n) => n,

      Err(e) => {
        eprint!("Error while reading stream : {e}");
        return Err(e)
      }
    };

    let s = match str::from_utf8_mut(&mut buffer) {
      Ok(s) => s,
      
      Err(_) => {
        eprintln!("Error while converting buffer to string");
        ""
      }
    };

    request.push_str(s);
    println!("{}", n);
    if n < 255 {
      break;
    }
  }

  let response = "HTTP/1.1 200 OK\r\n\
    Server: WebServer\r\n\
    Content-Type: text/html\r\n\
    Content-Length: 12\r\n\
    Connection: close\r\n\
    \r\n\
    Hello world.".as_bytes();


  match stream.write(response) {
    Ok(_) => {},

    Err(e) => {
      eprintln!("Failed to write in stream : {e}");
      return Err(e)
    }
  }

  match stream.shutdown(std::net::Shutdown::Both) {
    Ok(_) => return Ok(true),

    Err(e ) => {
      eprintln!("Failed to shutdown stream : {e}");
      return Err(e);
    }
  }
}

fn main() {
  let listener = match TcpListener::bind("127.0.0.1:8080") {
    Ok(listener) => {
      print!("Server is running on port 8080.\n");
      listener
    },

    Err(e) => {
      eprintln!("Failed to bind : {e}");
      return
    }
  };

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        match handle_client(stream) {
          Ok(_) => {},
          Err(_) => {}
        };
      },

      Err(e) =>  {
        eprintln!("Failed to accept stream : {e}");
      }
    }
  }
}
