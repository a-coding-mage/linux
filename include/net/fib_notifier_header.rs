// Translated from net/fib_notifier.h.

use core::ffi::{c_int, c_uint, c_void};

// External types supplied by the included kernel headers.
#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fib_notifier_info {
    pub family: c_int,
    pub extack: *mut netlink_ext_ack,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fib_event_type {
    FIB_EVENT_ENTRY_REPLACE,
    FIB_EVENT_ENTRY_APPEND,
    FIB_EVENT_ENTRY_ADD,
    FIB_EVENT_ENTRY_DEL,
    FIB_EVENT_RULE_ADD,
    FIB_EVENT_RULE_DEL,
    FIB_EVENT_NH_ADD,
    FIB_EVENT_NH_DEL,
    FIB_EVENT_VIF_ADD,
    FIB_EVENT_VIF_DEL,
}

#[repr(C)]
pub struct fib_notifier_ops {
    pub family: c_int,
    pub list: list_head,
    pub fib_seq_read: Option<unsafe extern "C" fn(net: *const net) -> c_uint>,
    pub fib_dump: Option<
        unsafe extern "C" fn(
            net: *mut net,
            nb: *mut notifier_block,
            extack: *mut netlink_ext_ack,
        ) -> c_int,
    >,
    pub owner: *mut module,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn call_fib_notifier(
        nb: *mut notifier_block,
        event_type: fib_event_type,
        info: *mut fib_notifier_info,
    ) -> c_int;

    pub fn call_fib_notifiers(
        net: *mut net,
        event_type: fib_event_type,
        info: *mut fib_notifier_info,
    ) -> c_int;

    pub fn register_fib_notifier(
        net: *mut net,
        nb: *mut notifier_block,
        cb: Option<unsafe extern "C" fn(nb: *mut notifier_block)>,
        extack: *mut netlink_ext_ack,
    ) -> c_int;

    pub fn unregister_fib_notifier(net: *mut net, nb: *mut notifier_block) -> c_int;

    pub fn fib_notifier_ops_register(
        tmpl: *const fib_notifier_ops,
        net: *mut net,
    ) -> *mut fib_notifier_ops;

    pub fn fib_notifier_ops_unregister(ops: *mut fib_notifier_ops);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
