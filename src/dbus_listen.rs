extern crate dbus;
use self::dbus::ConnectionItem;

use std::vec::Vec;

pub struct SignalInfo {
    pub path: Option<String>,
    pub object: Option<String>,
    pub member: Option<String>,
}

pub type CallbackMap<Ctx> = Vec<(SignalInfo, fn(&SignalInfo, &Ctx) -> ())>;

fn cmp_option<T: Eq>(a: &Option<T>, b: &Option<T>) -> bool {
    a.is_none() || a == b
}

fn match_info(info: &SignalInfo, expect: &SignalInfo) -> bool {
    cmp_option(&expect.path, &info.path) &&
    cmp_option(&expect.object, &info.object) &&
    cmp_option(&expect.member, &info.member)
}

pub fn match_signal<Ctx>(item: ConnectionItem, map: &CallbackMap<Ctx>, ctx: &Ctx) -> () {
    match item {
        ConnectionItem::Signal(s) => {
            let (_, p, o, m) = s.headers();
            let info = SignalInfo { path: p, object: o, member: m };

            for cb_item in (*map).iter() {
                let (ref expect, ref cb) = *cb_item;

                if match_info(&info, expect) {
                    cb(&info, ctx);
                }
            }
        },
        _ => ()
    }
}

pub fn make_signal_info(path: &str, object: &str, member: &str) -> SignalInfo {
    SignalInfo {
        path: Some(path.to_string()),
        object: Some(object.to_string()),
        member: Some(member.to_string()),
    }
}
