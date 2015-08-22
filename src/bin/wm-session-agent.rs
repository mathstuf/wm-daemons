extern crate wm_daemons;
use wm_daemons::config::{load_config, load_config_path};
use wm_daemons::dbus_listen::{CallbackMap, DBusInfo, match_signal};
use wm_daemons::exec::{CommandLine, read_command_line_from_config, run_command_line};

#[macro_use]
extern crate clap;
use clap::{Arg, App};

extern crate dbus;
use self::dbus::{Connection, BusType, Message};

use std::env::var;
use std::error::Error;
use std::path::Path;

struct Context {
    on_lock: Option<CommandLine>,
    on_unlock: Option<CommandLine>,
}

fn run_program(action: &str, cmd_line: &Option<CommandLine>) -> () {
    cmd_line.as_ref().map(|ref cmd| {
        let res = run_command_line(&cmd);
        if res.is_err() {
            println!("failed to handle '{}' action: {}", action, res.err().unwrap());
        }
    });
}

fn handle_signal(ctx: Context, info: &Message) -> Context {
    let (_, _, _, member) = info.headers();

    member.map(|member_name| {
        if member_name == "Lock" {
            run_program("lock", &ctx.on_lock);
        } else if member_name == "Unlock" {
            run_program("unlock", &ctx.on_unlock);
        }
    });

    ctx
}

fn try_main() -> Result<(), Box<Error>> {
    let matches = App::new("wm-session-agent")
        .version(&crate_version!()[..])
        .author("Ben Boeckel <mathstuf@gmail.com>")
        .about("Listens for logind's Lock and Unlock signals")
        .arg(Arg::with_name("CONFIG")
                .short("c")
                .long("config")
                .help("Path to the configuration file")
                .takes_value(true))
        .arg(Arg::with_name("SESSION")
                .short("s")
                .long("session")
                .help("Session ID to listen on")
                .takes_value(true))
        .get_matches();

    let conf = try!(if matches.is_present("CONFIG") {
            load_config_path(Path::new(matches.value_of("CONFIG").unwrap()))
        } else {
            load_config("wm-session-agent", "config")
        });
    let ctx = Context {
        on_lock: read_command_line_from_config(&conf, "actions.lock"),
        on_unlock: read_command_line_from_config(&conf, "actions.unlock"),
    };

    let sid = match matches.value_of("SESSION") {
        Some(sid) => sid.to_string(),
        None => match var("XDG_SESSION_ID") {
            Ok(sid) => sid,
            Err(err) => {
                println!("No session ID given or available: {}", err);
                return Err(Box::new(err));
            },
        },
    };
    let spath = format!("/org/freedesktop/login1/session/_{}", sid);

    let conn = try!(Connection::get_private(BusType::System));

    let cbs: CallbackMap<Context> = vec![
        (DBusInfo {
            path: None,
            object: None,
            member: Some("Lock".to_string()),
        }, handle_signal),
        (DBusInfo {
            path: None,
            object: None,
            member: Some("Unlock".to_string()),
        }, handle_signal),
    ];

    let match_str = format!("type='signal',interface='org.freedesktop.login1.Session',path='{}'", spath);
    try!(conn.add_match(&match_str[..]));

    conn.iter(100).fold(ctx, |inner_ctx, item| {
        match_signal(inner_ctx, &cbs, item)
    });

    Ok(())
}

fn main() -> () {
    if let Err(err) = try_main() {
        panic!("{}", err.description());
    }
}
