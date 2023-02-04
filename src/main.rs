use std::{net::{TcpListener}, io::{Write, BufRead, BufReader, Read}};

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

        let mut socket = stream?.try_clone().unwrap();

        // stream から request を読み込む
        let mut request = String::new();
        let mut reader = BufReader::new(&socket);

        if let Ok(v) = reader.read_line(&mut request) {
            println!("v = {}", v);
            if v > 0 {
                // リクエストを表示
                println!("Request: {}",request);
            }
        } else {
            println!("no response");
        }

        // 受け付けた socket に書き込む
        let response_text = String::from("hello world").into_bytes();
        // let mut res_socket = stream;
        socket.write(&response_text)?;
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
