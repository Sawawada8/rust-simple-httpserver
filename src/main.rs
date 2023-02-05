use std::{net::{TcpListener}, io::{Write, BufRead, BufReader, Read}, fmt::format, fs::File};

mod handle;


const HOST: &str = "0.0.0.0:8082";

fn main() -> std::io::Result<()> {
    println!("=====================");
    println!("SERVER start");
    println!("=====================");
    handle::handle();

    let listener = TcpListener::bind(HOST)?;

    // 接続を受け入れ、処理していく
    for stream in listener.incoming() {
        println!("-----------------");
        println!("accept !!!");
        println!("-----------------");

        // let mut socket = stream?.try_clone().unwrap();
        let mut socket = stream.unwrap();

        // stream から request を読み込む
        let mut b = [0; 1024];
        socket.read(&mut b).unwrap();
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
        socket.write(response_text.as_bytes())?;
        socket.flush().unwrap();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn demo_test() {
        assert_eq!(4, 2 + 2);
    }
}
