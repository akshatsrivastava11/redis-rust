pub enum Value{
    typ(String),
    str(String),
    bulk(String),
    num(u32),
    array(Vec<Value>)
}