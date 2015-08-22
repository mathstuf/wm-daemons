extern crate dbus;
use self::dbus::{ConnectionItem, Message};

use std::vec::Vec;

pub struct DBusInfo {
    pub path: Option<String>,
    pub object: Option<String>,
    pub member: Option<String>,
}

pub type CallbackMap<Ctx> = Vec<(DBusInfo, fn(Ctx, &Message) -> Ctx)>;

fn cmp_option<T: Eq>(a: &Option<T>, b: &Option<T>) -> bool {
    a.is_none() || a == b
}

fn match_info(info: &DBusInfo, expect: &DBusInfo) -> bool {
    cmp_option(&expect.path, &info.path) &&
    cmp_option(&expect.object, &info.object) &&
    cmp_option(&expect.member, &info.member)
}

fn handle_message<Ctx>(ctx: Ctx, map: &CallbackMap<Ctx>, msg: Message) -> Ctx {
    let (_, p, o, m) = msg.headers();
    let info = DBusInfo { path: p, object: o, member: m };

    (*map).iter().fold(ctx, |old, ref item| {
        let (ref expect, ref cb) = **item;

        if match_info(&info, expect) {
            cb(old, &msg)
        } else {
            old
        }
    })
}

pub fn match_method<Ctx>(ctx: Ctx, map: &CallbackMap<Ctx>, item: ConnectionItem) -> Ctx {
    match item {
        ConnectionItem::MethodCall(m) => handle_message(ctx, map, m),
        _ => ctx,
    }
}

pub fn match_signal<Ctx>(ctx: Ctx, map: &CallbackMap<Ctx>, item: ConnectionItem) -> Ctx {
    match item {
        ConnectionItem::Signal(s) => handle_message(ctx, map, s),
        _ => ctx,
    }
}

pub fn make_full_dbus_info(path: &str, object: &str, member: &str) -> DBusInfo {
    DBusInfo {
        path: Some(path.to_string()),
        object: Some(object.to_string()),
        member: Some(member.to_string()),
    }
}
