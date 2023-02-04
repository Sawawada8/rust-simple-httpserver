use std::net::{TcpStream};

const SERVER: &str = "0.0.0.0:8082";

fn main() -> std::io::Result<()> {
    println!("===============");
    println!("client start");
    println!("===============\n");

    // 接続する
    println!("connect serve !");
    // 成功すればそのまま処理が続く、失敗すれば Err返す
    let stream = TcpStream::connect(SERVER)?;
    println!("Stream: {:?}", stream);

    // こんな感じにもかける
    // match TcpStream::connect(SERVER) {
    //     Ok(result) => {println!("Ok: {:?}", result);},
    //     Err(err) => {println!("Err: {:?}", err)}
    // };


    println!("\n===============");
    println!("client end");
    println!("===============");
    Ok(())
}
