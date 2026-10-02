use crate::parser::frontend_defi::defining::{Statement, Type, Value};
use std::fs::File;
use std::io::Write;
use std::process::Command;

use crate::parser::frontend_defi::defining::*;

pub fn generate_backend(v: Vec<Statement>) {
    let mut c_code = String::new();

    // basic C header
    c_code.push_str("#include <stdio.h>\n\n");
    c_code.push_str("int main(){\n");

    for statement in v {
        match statement {
            Statement::VariableDeclaration(var) => {
                c_code.push_str(&generate_variable(var));
            }

            Statement::FunctionCall(call) => {
                c_code.push_str(&generate_function_call(call));
            }
            Statement::ArthematicOperator(_) => todo!(),
        }
    }

    c_code.push_str("return 0;\n");
    c_code.push_str("}\n");

    // write C file
    let mut file = File::create("generated.c").unwrap();

    file.write_all(c_code.as_bytes()).unwrap();

    // compile using gcc
    let status = Command::new("gcc")
        .args(["generated.c", "-o", "generated"])
        .status()
        .unwrap();

    if status.success() {
        println!("Generated successfully!");
    } else {
        println!("gcc failed");
    }
}

fn generate_variable(var: VariableDeclaration) -> String {
    let dtype = match var.dtype {
        Type::Int => "int",
        Type::Float => "float",
        Type::String => "char*",
    };

    format!("{} {} = {};\n", dtype, var.name, generate_value(var.value))
}

fn generate_function_call(call: FunctionCall) -> String {
    let args = call
        .args
        .into_iter()
        .map(generate_value)
        .collect::<Vec<_>>()
        .join(", ");

    format!("{}({});\n", call.name, args)
}

fn generate_value(v: Value) -> String {
    match v {
        Value::Int(x) => x.to_string(),

        Value::Float(x) => x.to_string(),

        Value::String(x) => {
            format!("\"{}\"", x)
        }
    }
}
