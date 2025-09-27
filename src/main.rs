use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::error::Error;
mod parser;
use parser::*;
mod types;
use types::*;


fn accept_connection(mut stream:TcpStream)->Result<(),Box<dyn Error>>{
    println!("Connection triggered");
    let buf=&mut [0u8;1024];
    match stream.read(buf){
        Ok(data)=>{
            if data>0{
                println!("Message recieved");
                println!("Raw bytes : {:?}",&buf[..data]);
                match std::str::from_utf8(&buf[..data]){
                    Ok(test)=>println!("Printing as utf-8 {}",test),
                    Err(err)=>println!("not a valid utf-8")
                };

            }
        },
        Ok(_)=>{
            println!("No data recieved")
        },
        Err(err)=>{
            println!("An error has occured {}",err)
        }
    }
    Ok(())
}

#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
    parser::parsing("*3\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$7\r\nmyvalue\r\n".to_string());

    println!("this is the redis server");
    let listener=TcpListener::bind("127.0.0.1:6379".to_string()).unwrap();
    for   stream in listener.incoming(){
        match stream{
            Ok(stream)=>accept_connection(stream)?,
            Err(err)=>{
                println!("An error has occured while accepting the data :{}",err);
            }
        }
    };
    Ok(())
}
