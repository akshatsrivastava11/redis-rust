use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::error::Error;
fn accept_connection(mut stream:TcpStream)->Result<(),Box<dyn Error>>{
    println!("Connection triggered");
    let buf=&mut String::new();
    match stream.read_to_string(buf){
        Ok(data)=>println!("Everything good {}",data),
        Err(error)=>println!("an error have occured {}",error)
    };
    println!("Connection reaced here");
    stream.write("This is me nigga".as_bytes());
    println!("no problme occured");
    Ok(())
}

#[tokio::main]
async fn main()->Result<(),Box<dyn Error>>{
    println!("this is the redis server");
    let listener=TcpListener::bind("127.0.0.1:8080".to_string()).unwrap();
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
