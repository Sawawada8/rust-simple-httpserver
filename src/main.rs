use std::{net::{TcpListener}, io::Write};

mod handle;


const HOST: &str = "0.0.0.0:8082";

fn main() -> std::io::Result<()> {
    println!("=====================");
    println!("main start");
    println!("=====================");
    handle::handle();

    let listener = TcpListener::bind(HOST)?;

    // 接続を受け入れ、処理していく
    for stream in listener.incoming() {
        println!("-----------------");
        println!("accept !!!");
        println!("-----------------");

        let mut socket = stream?;
        // 受け付けた socket に書き込む
        let response_text = String::from("hello").into_bytes();
        socket.write(&response_text);
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
