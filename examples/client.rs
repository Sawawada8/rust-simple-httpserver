use std::io::prelude::*;

use std::net::{TcpStream};
use std::io::{BufReader,BufRead};

const SERVER: &str = "0.0.0.0:8082";

fn main() -> std::io::Result<()> {
    println!("===============");
    println!("CLIENT start");
    println!("===============\n");

    // 接続する
    println!("connect serve !");
    // 成功すればそのまま処理が続く、失敗すれば Err返す
    let mut stream = TcpStream::connect(SERVER)?;
    println!("Stream: {:?}", stream);

    // write
    write(&mut stream);


    // read
    // readが思ったようにできないが、serverから返していいないから？
    read(&mut stream);
    // read_byte(&mut stream);



    println!("\n===============");
    println!("client end");
    println!("===============");
    Ok(())
}

fn write(stream: &mut TcpStream) -> &mut TcpStream {
    let request_text = String::from("request text").into_bytes();
    if let Ok(result) = stream.write(&request_text) {
        println!("write OK : {:?}", result);
        stream
    } else {
        stream
    }
}

fn read(stream: &mut TcpStream) {
    // stream から response を読めるようにする
    let mut recv_response = String::new();
    let mut reader = BufReader::new(stream);

    // response 読んでいく
    if let Ok(v) = reader.read_line(&mut recv_response) {
        println!("v = {}", v);
        if v > 0 {
            //レスポンスを表示
            println!("response: {}",recv_response);
        }
    }
}

fn read_byte(stream: &mut TcpStream) {
    let reader = BufReader::new(stream);
    let bytes = reader.buffer();
    let converted: String = String::from_utf8(bytes.to_vec()).unwrap();
    println!("response: {:?}", converted);
}
