use std::io::{self, BufRead};

use crate::types::Value;



pub fn read_line<R:BufRead>(reader:&mut R)->std::io::Result<String>{
    let mut line=String::new();
    reader.read_line(&mut line)?;
    if line.ends_with("\r\n"){
        line.truncate(line.len()-2);
    }
    Ok(line)
}

pub fn read_integer<R:BufRead>(reader:&mut R)->std::io::Result<i64>{
    let mut line=read_line(reader)?;
    line.parse::<i64>().map_err(|e|io::Error::new(io::ErrorKind::InvalidData,e))
}

pub fn read_bulk<R:BufRead>(reader:&mut R)->std::io::Result<Value>{
    let line =read_integer(reader)?;
    let mut buf=vec![0u8;line as usize];
    reader.read_exact(&mut buf)?;
    let mut crlf=[0u8;2];
    reader.read_exact(&mut crlf)?;
    let s=String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData,e))?;
    Ok(Value::bulk(Some(s)))
}

pub fn read_array<R:BufRead>(reader:&mut R)->std::io::Result<Value>{
    let len=read_integer(reader)?;
    println!("from the read array fn ,len is {}",len);
    let mut values=Vec::with_capacity(len as usize);
    for _ in 0..len{
        values.push(read_resp(reader)?);
    }
    println!("In the read  arraay fn {:?}",values);
    Ok(Value::array(values))
}

pub fn read_resp<R:BufRead>(reader:&mut R)->std::io::Result<Value>{
    let mut type_byte=[0u8;1];
    reader.read(&mut type_byte)?;
    println!("In the read resp :{:?}",std::str::from_utf8(&type_byte).unwrap());
    match type_byte[0] as char{
        '+' => Ok(Value::str(read_line(reader)?)),
        '-' => Ok(Value::err(read_line(reader)?)),
        ':' => Ok(Value::num(read_integer(reader)?)),
        '$' => read_bulk(reader),
        '*' => read_array(reader),
        other => Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unknown type: {}", other))),
    }
}