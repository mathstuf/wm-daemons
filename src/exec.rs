extern crate config;
use self::config::types::{Config, ScalarValue, Value};

use std::env::home_dir;
use std::process::Command;

fn expand_home(path: &String) -> String {
    if path.starts_with("~/") {
        match home_dir() {
            Some(home) => format!("{}/{}", home.display(), &path[2..]),
            _ => format!("{}", path)
        }
    } else {
        format!("{}", path)
    }
}

fn make_command_vec(vec: &Vec<Value>) -> Result<Command, String> {
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
    let mut cmd = Command::new(expand_home(prog[0]));
    cmd.args(args);

    Ok(cmd)
}

fn make_command_str(string: &String) -> Result<Command, String> {
    if string.is_empty() {
        Err(format!("empty execution string"))
    } else {
        Ok(Command::new(expand_home(string)))
    }
}

pub fn run_config_program(conf: &Config, path: &str) -> Result<(), String> {
    let val = conf.lookup(path);
    if val.is_none() {
        return Ok(());
    }

    let cmd = try!(match val.unwrap() {
        &Value::Array(ref a) => make_command_vec(a),
        &Value::Svalue(ref sv) => {
            match sv {
                &ScalarValue::Str(ref s) => make_command_str(s),
                _ => Err(format!("unsupported type for {}", path))
            }
        },
        _ => Err(format!("unsupported type for {}", path))
    }).output();

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
