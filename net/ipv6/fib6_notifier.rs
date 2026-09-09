// SPDX-License-Identifier: GPL-2.0
//
// C dependencies are supplied by the surrounding kernel translation unit.

use core::ffi::c_int;

pub const AF_INET6: i32 = 10;

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fib_notifier_info {
    pub family: i32,
}

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_ipv6 {
    pub notifier_ops: *mut fib_notifier_ops,
}

#[repr(C)]
pub struct net {
    pub ipv6: net_ipv6,
}

#[repr(C)]
pub struct fib_notifier_ops {
    pub family: i32,
    pub fib_seq_read: Option<unsafe extern "C" fn(*const net) -> u32>,
    pub fib_dump: Option<
        unsafe extern "C" fn(
            *mut net,
            *mut notifier_block,
            *mut netlink_ext_ack,
        ) -> c_int,
    >,
    pub owner: *mut core::ffi::c_void,
}

#[repr(C)]
pub enum fib_event_type {
    _Unused = 0,
}

extern "C" {
    static THIS_MODULE: *mut core::ffi::c_void;

    fn call_fib_notifier(
        nb: *mut notifier_block,
        event_type: fib_event_type,
        info: *mut fib_notifier_info,
    ) -> c_int;
    fn call_fib_notifiers(
        net: *mut net,
        event_type: fib_event_type,
        info: *mut fib_notifier_info,
    ) -> c_int;
    fn fib6_tables_seq_read(net: *const net) -> u32;
    fn fib6_rules_seq_read(net: *const net) -> u32;
    fn fib6_rules_dump(
        net: *mut net,
        nb: *mut notifier_block,
        extack: *mut netlink_ext_ack,
    ) -> c_int;
    fn fib6_tables_dump(
        net: *mut net,
        nb: *mut notifier_block,
        extack: *mut netlink_ext_ack,
    ) -> c_int;
    fn fib_notifier_ops_register(
        ops: *const fib_notifier_ops,
        net: *mut net,
    ) -> *mut fib_notifier_ops;
    fn fib_notifier_ops_unregister(ops: *mut fib_notifier_ops);
    fn is_err<T>(ptr: *const T) -> bool;
    fn ptr_err<T>(ptr: *const T) -> c_int;
}

pub unsafe extern "C" fn call_fib6_notifier(
    nb: *mut notifier_block,
    event_type: fib_event_type,
    info: *mut fib_notifier_info,
) -> c_int {
    (*info).family = AF_INET6;
    call_fib_notifier(nb, event_type, info)
}

pub unsafe extern "C" fn call_fib6_notifiers(
    net: *mut net,
    event_type: fib_event_type,
    info: *mut fib_notifier_info,
) -> c_int {
    (*info).family = AF_INET6;
    call_fib_notifiers(net, event_type, info)
}

unsafe extern "C" fn fib6_seq_read(net: *const net) -> u32 {
    fib6_tables_seq_read(net).wrapping_add(fib6_rules_seq_read(net))
}

unsafe extern "C" fn fib6_dump(
    net: *mut net,
    nb: *mut notifier_block,
    extack: *mut netlink_ext_ack,
) -> c_int {
    let err = fib6_rules_dump(net, nb, extack);
    if err != 0 {
        return err;
    }
    fib6_tables_dump(net, nb, extack)
}

static fib6_notifier_ops_template: fib_notifier_ops = fib_notifier_ops {
    family: AF_INET6,
    fib_seq_read: Some(fib6_seq_read),
    fib_dump: Some(fib6_dump),
    owner: unsafe { THIS_MODULE },
};

pub unsafe extern "C" fn fib6_notifier_init(net: *mut net) -> c_int {
    let ops = fib_notifier_ops_register(&fib6_notifier_ops_template, net);
    if is_err(ops) {
        return ptr_err(ops);
    }
    (*net).ipv6.notifier_ops = ops;
    0
}

pub unsafe extern "C" fn fib6_notifier_exit(net: *mut net) {
    fib_notifier_ops_unregister((*net).ipv6.notifier_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
