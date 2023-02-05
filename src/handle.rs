use std::io::{Write, BufRead, BufReader, Read};
use std::fmt::format;
use std::fs::File;
use std::net::TcpStream;
use std::thread;
use std::time::{Duration};


pub fn handle(mut socket: TcpStream) {
        // stream から request を読み込む
        let mut b = [0; 1024];

        socket.read(&mut b).unwrap();

        let sleep = b"GET /sleep HTTP/1.1\r\n";
        if b.starts_with(sleep) {
            println!("call sleep");
            thread::sleep(
                Duration::from_secs(5)
            );
        }
        println!("REQUEST: {:?}", String::from_utf8_lossy(&b[..]));


        // 受け付けた socket に書き込む
        let mut file = File::open("i.html").unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let response_text = format!(
            "HTTP/1.1 200 OK\r\n\r\n{}",
            contents
        );
        // let mut res_socket = stream;
        socket.write(response_text.as_bytes()).unwrap();
        socket.flush().unwrap();
}


#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn demo_test() {
        assert_eq!(2, 2);
    }
}
