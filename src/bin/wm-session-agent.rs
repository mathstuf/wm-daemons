extern crate wm_daemons;
use wm_daemons::config::load_config;
use wm_daemons::dbus_listen::{CallbackMap, SignalInfo, match_signal, make_signal_info};

extern crate config;
use self::config::types::Config;

extern crate dbus;
use self::dbus::{Connection, BusType};

use std::error::Error;

fn run_program(action: &str, conf: &Config) -> () {
    // TODO
}

fn handle_signal(info: &SignalInfo, conf: &Config) -> () {
    if info.member == Some("Lock".to_string()) {
        run_program("lock", conf);
    } else if info.member == Some("Unlock".to_string()) {
        run_program("unlock", conf);
    }
}

fn try_main() -> Result<(), Box<Error>> {
    let conf = try!(load_config("wm-session-agent", "config"));
    let conn = try!(Connection::get_private(BusType::System));

    let cbs: CallbackMap<Config> = vec![
        (make_signal_info(
            "org.freedesktop.login1.Session",
            "/org/freedesktop/login1/session/_33",
            "Lock",
        ), handle_signal),
        (make_signal_info(
            "org.freedesktop.login1.Session",
            "/org/freedesktop/login1/session/_33",
            "Unlock",
        ), handle_signal),
    ];

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
