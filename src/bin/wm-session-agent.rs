extern crate wm_daemons;
use wm_daemons::config::{load_config, load_config_path};
use wm_daemons::dbus_listen::{CallbackMap, SignalInfo, match_signal};
use wm_daemons::exec::run_config_program;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

extern crate config;
use self::config::types::Config;

extern crate dbus;
use self::dbus::{Connection, BusType};

use std::error::Error;
use std::path::Path;

fn run_program(action: &str, conf: &Config) -> () {
    let path = format!("actions.{}", action);
    let res = run_config_program(conf, &path[..]);
    if res.is_err() {
        println!("failed to handle '{}' action: {}", action, res.err().unwrap());
    }
}

fn handle_signal(info: &SignalInfo, conf: &Config) -> () {
    if info.member == Some("Lock".to_string()) {
        run_program("lock", conf);
    } else if info.member == Some("Unlock".to_string()) {
        run_program("unlock", conf);
    }
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
                .required(true)
                .help("Session ID to listen on")
                .takes_value(true))
        .get_matches();

    let conf = try!(if matches.is_present("CONFIG") {
            load_config_path(Path::new(matches.value_of("CONFIG").unwrap()))
        } else {
            load_config("wm-session-agent", "config")
        });

    let sid = matches.value_of("SESSION").unwrap();
    let spath = format!("/org/freedesktop/login1/session/_{}", sid);

    let conn = try!(Connection::get_private(BusType::System));

    let cbs: CallbackMap<Config> = vec![
        (SignalInfo {
            path: None,
            object: None,
            member: Some("Lock".to_string()),
        }, handle_signal),
        (SignalInfo {
            path: None,
            object: None,
            member: Some("Unlock".to_string()),
        }, handle_signal),
    ];

    let match_str = format!("type='signal',interface='org.freedesktop.login1.Session',path='{}'", spath);
    try!(conn.add_match(&match_str[..]));

    for items in conn.iter(100) {
        match_signal(items, &cbs, &conf);
    }

    Ok(())
}

fn main() -> () {
    if let Err(err) = try_main() {
        panic!("{}", err.description());
    }
}
