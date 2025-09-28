#[derive(Debug,Clone)]
pub enum Value{
    typ(String),
    str(String),
    bulk(Option<String>),
    num(i64),
    err(String),
    array(Vec<Value>)
}