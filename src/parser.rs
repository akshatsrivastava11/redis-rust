use std::error::Error;



//RESP-parser
pub fn parsing(input:String)->Result<(),Box<dyn Error>>{
    for (index,chr) in input.chars().enumerate(){
        if index==0{
            if chr!='$'{
                eprintln!("Not a valid string to parse out");
                Err("An error occured during parsing")          
        }
    }
    
};
Ok(())
}