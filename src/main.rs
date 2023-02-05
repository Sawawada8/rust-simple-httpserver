use std::{net::{TcpListener}, io::{Write, BufRead, BufReader, Read}, fmt::format, fs::File, time::Duration, thread};

mod handle;


const HOST: &str = "0.0.0.0:8082";

fn main() -> std::io::Result<()> {
    println!("=====================");
    println!("SERVER start");
    println!("=====================");

    let listener = TcpListener::bind(HOST)?;

    // 接続を受け入れ、処理していく
    for stream in listener.incoming() {
        println!("-----------------");
        println!("accept !!!");
        println!("-----------------");

        // let mut socket = stream?.try_clone().unwrap();
        let socket = stream.unwrap();

        // socket は今後使わないので、そのまま渡して大丈夫
        // multi thred で処理
        thread::spawn(|| {
            handle::handle(socket);
        });
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
