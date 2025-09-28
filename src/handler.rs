use std::{collections::HashMap, env::args, hash::Hash, net::TcpStream, sync::{Arc, RwLock}};

use crate::{types::Value, writer::RespWriter};

lazy_static::lazy_static!{
    static ref KV_Store:Arc<RwLock<HashMap<String,String>>>=Arc::new(RwLock::new(HashMap::new()));
    static ref HashStore:Arc<RwLock<HashMap<String,HashMap<String,String>>>>=Arc::new(RwLock::new(HashMap::new()));
}

fn ping(args:&[Value])->Value{
    if args.is_empty(){
        Value::str("PONG".to_string())
    }else {
        match &args[0]{
            Value::bulk(Some(s))=>Value::str(s.clone()),
            _=>Value::str("PONG".to_string())
        }
    }
}

fn set(args:&[Value])->Value{
    if args.len()!=2{
        return Value::err("ERR wrong number of arguments".to_string());
    };
    let key=match &args[0]{
        Value::bulk(Some(s))=>s.clone(),
        _=>return Value::err("Err invalid key".to_string())
    };
    let value=match &args[1]{
        Value::bulk(Some(s))=>s.clone(),
        _=>return Value::err("Err invalid key".to_string())
    };
    let mut store=KV_Store.write().unwrap();
    store.insert(key, value);
    Value::str("OK".to_string())
}

fn get(args:&[Value])->Value{
    if args.len()!=1{
        return Value::err("ERR wrong number of args".to_string());
    };
    let key=match &args[0]{
        Value::bulk(Some(s))=>s.clone(),
        _=>return Value::err("Err invalid key".to_string())
    };
    let store=KV_Store.read().unwrap();
    match store.get(&key){
        Some(val)=>Value::bulk(Some(val.clone())),
        None=>Value::bulk(None)
    }
}


type Handler = fn(&[Value]) -> Value;

lazy_static::lazy_static!{
    static ref Handlers:HashMap<String,Handler>={
        let mut m=HashMap::new();
        m.insert("PING".to_string(), ping as Handler);
        m.insert("SET".to_string(),set as Handler);
        m.insert("GET".to_string(),get as Handler);
        m
    };
}


pub fn handleCommand(value:Value,writer:&mut RespWriter<&mut TcpStream>){
    if let Value::array(arr)=value{
        if arr.is_empty(){
            let _=writer.write_values(&Value::err("ERR empty cmd".into()));
            return;
        }
        let cmd=match &arr[0]{
            Value::bulk(Some(s))=>s.to_uppercase(),
            _=>{
                let _=writer.write_values(&Value::err("ERR invalid cmds".into()));
                return;
            }
        };
        let args=&arr[1..];
        match Handlers.get(&cmd){
            Some(data)=>{
                let resp=data(args);
                let _ = writer.write_values(&resp);
            }
            None=>{
            let _ = writer.write_values(&&Value::err("ERR unknown command".into()));

            }
        }
    }
    else{
        let _ = writer.write_values(&Value::err("ERR expected array".into()));

    }
}