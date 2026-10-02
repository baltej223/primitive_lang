use std::env;

pub mod args_handler;
pub mod backend;
pub mod parser;
pub mod reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let params = args_handler::handle_args(&args)?;
    // print!("filename={}", params.filename());

    let contents = reader::read_file(params.filename())?;
    let frontend = parser::cparser::parse(contents);

    backend::generate::generate_backend(frontend);

    Ok(())
}
