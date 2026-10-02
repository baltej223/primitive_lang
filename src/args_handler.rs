use std::error::Error;

pub struct ArgsStruct {
    file_name: String,
}

pub fn handle_args(args: &Vec<String>) -> Result<ArgsStruct, Box<dyn Error>> {
    if args.len() < 2 {
        return Err("Wrong Params".into());
    }
    Ok(ArgsStruct {
        file_name: args[1].clone(),
    })
}

impl ArgsStruct {
    pub fn filename(&self) -> &String {
        &self.file_name
    }
}
