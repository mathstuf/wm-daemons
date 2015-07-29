extern crate dbus;
use self::dbus::{ConnectionItem, ConnectionItems};

pub struct SignalInfo {
    path: Option<String>,
    object: Option<String>,
    member: Option<String>,
}

pub type CallbackMap<Ctx> = [(SignalInfo, Box<fn(&SignalInfo, &Ctx) -> ()>)];

fn cmp_option<T: Eq>(a: &Option<T>, b: &Option<T>) -> bool {
    a.is_none() || a == b
}

fn match_info(info: &SignalInfo, expect: &SignalInfo) -> bool {
    cmp_option(&expect.path, &info.path) &&
    cmp_option(&expect.object, &info.object) &&
    cmp_option(&expect.member, &info.member)
}

pub fn match_signal<Ctx>(items: ConnectionItems, map: &CallbackMap<Ctx>, ctx: Ctx) -> () {
    for item in items {
        match item {
            ConnectionItem::Signal(s) => {
                let (_, p, o, m) = s.headers();
                let info = SignalInfo { path: p, object: o, member: m };

                for cb_item in (*map).iter() {
                    let (ref expect, ref cb) = *cb_item;

                    if match_info(&info, expect) {
                        cb(&info, &ctx);
                    }
                }
            },
            _ => ()
        }
    }
}
