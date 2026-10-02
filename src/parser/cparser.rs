use std::ffi::{CStr, CString, c_char, c_int};

use crate::parser::frontend_defi::defining::{self, Statement, Type, Value};

unsafe extern "C" {
    fn sscanf(input: *const c_char, format: *const c_char, ...) -> c_int;
}

pub fn parse(c: String) -> Vec<Statement> {
    let mut index = 0;
    let mut frontend_output: Vec<Statement> = Vec::new();
    while index < c.len() {
        let mut conti = true;
        let index_prev = index;
        while conti {
            if (c.chars().nth(index).unwrap() == '\n') || (c.chars().nth(index).unwrap() == ';') {
                conti = false;
            }
            index += 1;
        }
        let substring = &c[index_prev..index];

        if let Some(Statement::VariableDeclaration(var)) = check_for_var_dec(&substring) {
            if !var.name.is_empty() {
                frontend_output.push(Statement::VariableDeclaration(var));
            }
        } else if let Some(Statement::FunctionCall(fnc)) = check_for_function_call(&substring) {
            if !fnc.name.is_empty() {
                frontend_output.push(Statement::FunctionCall(fnc));
            }
        }
    }
    frontend_output
}

fn check_for_var_dec(c: &str) -> Option<defining::Statement> {
    let input = CString::new(c).unwrap();

    let mut dtype = [0i8; 100];
    let mut name = [0i8; 100];
    let mut value = [0i8; 100];

    let success;
    unsafe {
        success = sscanf(
            input.as_ptr(),
            c"var %99s %99s = %99s".as_ptr(),
            name.as_mut_ptr(),
            dtype.as_mut_ptr(),
            value.as_mut_ptr(),
        );
    }

    if success != 3 {
        return None;
    }

    let dtype = unsafe { CStr::from_ptr(dtype.as_ptr()).to_string_lossy() };
    let name = unsafe { CStr::from_ptr(name.as_ptr()).to_string_lossy() };
    let value = unsafe { CStr::from_ptr(value.as_ptr()).to_string_lossy() };

    let data_type = match dtype.as_ref() {
        "string" => Type::String,

        "int" => Type::Int,

        "float" => Type::Float,

        _ => return None,
    };

    Some(Statement::VariableDeclaration(
        defining::VariableDeclaration {
            dtype: data_type,
            name: name.to_string(),
            value: parse_value(&dtype, &value)?,
        },
    ))
}

fn parse_arguments(args: &str) -> Option<Vec<Value>> {
    let mut result = Vec::new();

    for arg in args.split(',') {
        let arg = arg.trim();

        if let Ok(v) = arg.parse::<i64>() {
            result.push(Value::Int(v));
        } else if let Ok(v) = arg.parse::<f64>() {
            result.push(Value::Float(v));
        } else {
            result.push(Value::String(arg.to_string()));
        }
    }

    Some(result)
}

fn check_for_function_call(c: &str) -> Option<defining::Statement> {
    let input = CString::new(c).unwrap();

    let mut name = [0i8; 100];
    let mut args = [0i8; 500];

    let success;

    unsafe {
        success = sscanf(
            input.as_ptr(),
            c"%99[^ (](%499[^)])".as_ptr(),
            name.as_mut_ptr(),
            args.as_mut_ptr(),
        );
    }

    if success != 2 {
        return None;
    }

    let name = unsafe { CStr::from_ptr(name.as_ptr()).to_string_lossy() };

    let args = unsafe { CStr::from_ptr(args.as_ptr()).to_string_lossy() };

    let parsed_args = parse_arguments(&args)?;

    Some(Statement::FunctionCall(defining::FunctionCall {
        name: name.to_string(),
        args: parsed_args,
    }))
}

// fn check_for_var_dec_with_arthematic_on_rhs(c: &str) -> Option<defining::Statement> {
//     let input = CString::new(c).unwrap();
//
//     let mut dtype = [0i8; 100];
//     let mut name = [0i8; 100];
//     let mut value = [0i8; 100];
//
//     let success;
//     unsafe {
//         success = sscanf(
//             input.as_ptr(),
//             c"var %99s %99s = %99[^;\n]".as_ptr(),
//             name.as_mut_ptr(),
//             dtype.as_mut_ptr(),
//             value.as_mut_ptr(),
//         );
//     }
//
//     if success != 3 {
//         return None;
//     }
//
//     let dtype = unsafe { CStr::from_ptr(dtype.as_ptr()).to_string_lossy() };
//     let name = unsafe { CStr::from_ptr(name.as_ptr()).to_string_lossy() };
//     let value = unsafe { CStr::from_ptr(value.as_ptr()).to_string_lossy() };
//
//     let data_type = match dtype.as_ref() {
//         "string" => Type::String,
//
//         "int" => Type::Int,
//
//         "float" => Type::Float,
//
//         _ => return None,
//     };
//
//     Some(Statement::VariableDeclaration(
//         defining::VariableDeclaration {
//             dtype: data_type,
//             name: name.to_string(),
//             value: parse_value(&dtype, &value)?,
//         },
//     ))
// }

fn parse_value(dtype: &str, value: &str) -> Option<Value> {
    match dtype {
        "string" => Some(Value::String(
            value
                .to_string()
                .trim_matches(';')
                .trim_matches('"')
                .to_string(),
        )),

        "int" => Some(Value::Int(value.parse().ok()?)),

        "float" => Some(Value::Float(value.parse().ok()?)),

        _ => None,
    }
}

// pub fn parse(c: String) {
//     let input = CString::new(c).unwrap();
//
//     let mut dtype = [0i8; 100];
//     let mut name = [0i8; 100];
//     let mut value = [0i8; 100];
//
//     unsafe {
//         sscanf(
//             input.as_ptr(),
//             c"var %99s %99s = %99s".as_ptr(),
//             name.as_mut_ptr(),
//             dtype.as_mut_ptr(),
//             value.as_mut_ptr(),
//         );
//     }
//
//     let dtype = unsafe { CStr::from_ptr(dtype.as_ptr()).to_string_lossy() };
//
//     let name = unsafe { CStr::from_ptr(name.as_ptr()).to_string_lossy() };
//
//     let value = unsafe { CStr::from_ptr(value.as_ptr()).to_string_lossy() };
//
//     println!("dtype: {dtype}");
//     println!("name: {name}");
//     println!("value: {value}");
//
//     // Continue parsing other constructs here...
// }
