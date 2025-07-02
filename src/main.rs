use std::{io::{BufReader, Read, Write}, net::{TcpListener, TcpStream}};

enum HandleClientError {
  Io(std::io::Error),
  Utf8(core::str::Utf8Error)
}

impl From<std::io::Error> for HandleClientError {
  fn from(err: std::io::Error) -> Self {
    return HandleClientError::Io(err);
  }
}

impl From<core::str::Utf8Error> for HandleClientError {
  fn from(err: core::str::Utf8Error) -> Self {
    return HandleClientError::Utf8(err);
  }
}

impl std::fmt::Display for HandleClientError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      HandleClientError::Io(e) => write!(f, "{}", e),
      HandleClientError::Utf8(e) => write!(f, "{}", e)
    }      
  }
}

fn handle_client(mut stream: TcpStream) -> Result<(), HandleClientError> {
  let mut request = String::new();
  
  let mut reader = BufReader::new(&stream);
  let mut buffer = [0; 255];

  loop {
    let n = reader.read(&mut buffer)?;
    let s = str::from_utf8_mut(&mut buffer)?; 

    request.push_str(s);

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


  stream.write(response)?;
  stream.shutdown(std::net::Shutdown::Both)?;

  Ok(())
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
          Err(e) => eprintln!("Error while handling client : {e}")
        };
      },

      Err(e) =>  {
        eprintln!("Failed to accept stream : {e}");
      }
    }
  }
}
