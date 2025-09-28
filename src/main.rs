use std::{io::{BufReader, Read, Write}, net::{TcpListener, TcpStream}};
use std::error::Error;
mod writer;
use writer::*;
mod types;
use types::*;
mod reader;
use reader::*;
mod handler;
use handler::*;

fn accept_connection(mut stream:TcpStream)->Result<(),Box<dyn Error>>{
    println!("Connection triggered");

    let mut reader = BufReader::new(stream.try_clone()?);

    loop {
        match read_resp(&mut reader) {
            Ok(value) => {
                println!("Parsed: {:?}", value);
                let mut writer=RespWriter::new(&mut stream);
                let response=handleCommand(value, &mut writer);
                // stream.write_all(b"+OK\r\n")?;
            },
            Err(err) => {
                eprintln!("Error parsing reader: {}", err);
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
    // parser::parsing("*3\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$7\r\nmyvalue\r\n".to_string());

    println!("this is the redis server");
    let listener=TcpListener::bind("127.0.0.1:6379".to_string()).unwrap();
    for   stream in listener.incoming(){
        match stream{
            Ok(mut stream)=>{
                accept_connection(stream)?
            },
            Err(err)=>{
                println!("An error has occured while accepting the data :{}",err);
            }
        }
    };
    Ok(())
}
