use crate::types::Value;
use std::io::{self, Write};

impl Value {
    pub fn marshall<'b>(self)->Vec<u8>{
        match self{
            Value::str(s)=>{
                let mut out:Vec<u8> =Vec::new();
                out.push(b"+"[0]);
                let byte = s.as_bytes()[0]; // get the single byte
                // out.extend_from_slice(&[&[byte]]); // wrap in 1-element array
                out.extend_from_slice(&[byte]);
                // let cr: [u8; 1] = [b'\r'];
                // let lf: [u8; 1] = [b'\n'];
                //             `
                // let slice_of_refs: &[&[u8; 1]] = &[&cr`, &lf];
                out.extend_from_slice(b"\r\n");
                out
            },
            Value::err(err)=>{
                let mut out=Vec::new();
                out.push(b'-');
                out.extend_from_slice(err.as_bytes());
                out.extend_from_slice(b"\r\n");
                out
            },
            Value::num(n)=>{
                let mut out:Vec<u8>=Vec::new();
                out.push(b":"[0]);
                let bytes=n.to_string().as_bytes()[0];
                // out.extend_from_slice(&[&[bytes]]);
                out.extend_from_slice(&[bytes]);
                // let cr: [u8; 1] = [b'\r'];
                // let lf: [u8; 1] = [b'\n'];
                            
                // let slice_of_refs: &[&[u8; 1]] = &[&cr, &lf];
                // out.extend_from_slice(slice_of_refs);
                out.extend_from_slice(b"\r\n");
                out
            },
            Value::bulk(Some(s))=>{
                let mut out = Vec::new();
                out.push(b'$');
                out.extend_from_slice(s.len().to_string().as_bytes());
                out.extend_from_slice(b"\r\n");
                out.extend_from_slice(s.as_bytes());
                out.extend_from_slice(b"\r\n");
                out

            },
            Value::bulk(None)=>{
                b"$-1\r\n".to_vec()
            },
            Value::array(arr) => {
                let mut out = Vec::new();
                out.push(b'*');
                out.extend_from_slice(arr.len().to_string().as_bytes());
                out.extend_from_slice(b"\r\n");
                for v in arr {
                    out.extend_from_slice(&v.marshall()); // recursion
                }
                out
            },
            Value::typ(data)=>{
                println!("It is type typ {}",data);
                let mut out:Vec<u8>=Vec::new();
                out
            }

        }

    }
}



pub struct RespWriter<W:Write>{
    writer:W
}
impl<W:Write> RespWriter<W>{
    pub fn new(write:W)->Self{
        RespWriter{writer:write}
    }
    pub fn write_values(&mut self,v:&Value)->io::Result<()>{
        let bytes=v.clone().marshall();
        self.writer.write_all(&bytes);
        self.writer.flush()
    }
}