extern crate config;
use self::config::types::{Config, ScalarValue, Value};

use std::env::home_dir;
use std::process::Command;

pub struct CommandLine {
    program: String,
    args: Vec<String>,
}

fn expand_home(path: &String) -> String {
    if path.starts_with("~/") {
        match home_dir() {
            Some(home) => format!("{}/{}", home.display(), &path[2..]),
            _ => path.clone()
        }
    } else {
        path.clone()
    }
}

fn make_command_vec(vec: &Vec<Value>) -> Result<CommandLine, String> {
    if vec.is_empty() {
        return Err(format!("empty execution vector"));
    }

    let mut prog_args = vec![];
    for v in vec {
        match v {
            &Value::Svalue(ref sv) => {
                match sv {
                    &ScalarValue::Str(ref s) => prog_args.push(s),
                    _ => {
                        return Err(format!("non-string program value"));
                    }
                }
            },
            _ => {
                return Err(format!("non-string program value"));
            }
        }
    }

    let (prog, args) = prog_args.split_at(1);
    Ok(CommandLine {
        program: expand_home(prog[0]),
        // XXX: Why does .cloned() not work?
        args: args.iter().map(|&arg| (*arg).clone()).collect(),
    })
}

fn make_command_str(string: &String) -> Result<CommandLine, String> {
    if string.is_empty() {
        Err(format!("empty execution string"))
    } else {
        Ok(CommandLine {
            program: expand_home(string),
            args: vec![],
        })
    }
}

pub fn read_command_line_from_config(conf: &Config, path: &str) -> Option<CommandLine> {
    let val = conf.lookup(path);
    if val.is_none() {
        return None;
    }

    let cmd_line = match val.unwrap() {
        &Value::Array(ref a) => make_command_vec(a),
        &Value::Svalue(ref sv) => {
            match sv {
                &ScalarValue::Str(ref s) => make_command_str(s),
                _ => Err(format!("unsupported type for {}", path))
            }
        },
        _ => Err(format!("unsupported type for {}", path))
    };

    if cmd_line.is_err() {
        println!("Failed to make a command line for '{}': {:?}", path, cmd_line.as_ref().err());
    }
    cmd_line.ok()
}

pub fn run_command_line(cmd_line: &CommandLine) -> Result<(), String> {
    let cmd = Command::new(&cmd_line.program)
                .args(&cmd_line.args)
                .output();

    match cmd {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                Err(format!("failed to execute process ({:?}):\noutput:\n{}\nerror:{}",
                            output.status.code(),
                            String::from_utf8_lossy(&output.stdout),
                            String::from_utf8_lossy(&output.stderr)))
            }
        },
        _ => Err(format!("failed to execute process"))
    }
}
