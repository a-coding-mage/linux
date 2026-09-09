// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding kernel translation are referenced
// here as external types and functions.

use core::ffi::c_int;

extern "C" {
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
    fn fib4_rules_seq_read(net: *const net) -> u32;
    fn fib4_rules_dump(
        net: *mut net,
        nb: *mut notifier_block,
        extack: *mut netlink_ext_ack,
    ) -> c_int;
    fn fib_notify(
        net: *mut net,
        nb: *mut notifier_block,
        extack: *mut netlink_ext_ack,
    ) -> c_int;
    fn fib_notifier_ops_register(
        template: *const fib_notifier_ops,
        net: *mut net,
    ) -> *mut fib_notifier_ops;
    fn fib_notifier_ops_unregister(ops: *mut fib_notifier_ops);
    fn ptr_err<T>(ptr: *const T) -> c_int;
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct netlink_ext_ack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fib_notifier_info {
    pub family: c_int,
}

pub type fib_event_type = c_int;

#[repr(C)]
pub struct fib_notifier_ops {
    pub family: c_int,
    pub fib_seq_read: Option<unsafe extern "C" fn(*const net) -> u32>,
    pub fib_dump: Option<unsafe extern "C" fn(
        *mut net,
        *mut notifier_block,
        *mut netlink_ext_ack,
    ) -> c_int>,
    pub owner: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct net_ipv4 {
    pub fib_seq: u32,
    pub notifier_ops: *mut fib_notifier_ops,
}

#[repr(C)]
pub struct net {
    pub ipv4: net_ipv4,
}

const AF_INET: c_int = 2;

#[inline]
pub unsafe extern "C" fn call_fib4_notifier(
    nb: *mut notifier_block,
    event_type: fib_event_type,
    info: *mut fib_notifier_info,
) -> c_int {
    (*info).family = AF_INET;
    call_fib_notifier(nb, event_type, info)
}

#[inline]
pub unsafe extern "C" fn call_fib4_notifiers(
    net: *mut net,
    event_type: fib_event_type,
    info: *mut fib_notifier_info,
) -> c_int {
    // ASSERT_RTNL();
    (*info).family = AF_INET;
    // Paired with READ_ONCE() in fib4_seq_read().
    (*net).ipv4.fib_seq = (*net).ipv4.fib_seq.wrapping_add(1);
    call_fib_notifiers(net, event_type, info)
}

unsafe extern "C" fn fib4_seq_read(net: *const net) -> u32 {
    // Paired with WRITE_ONCE() in call_fib4_notifiers().
    (*net).ipv4.fib_seq.wrapping_add(fib4_rules_seq_read(net))
}

unsafe extern "C" fn fib4_dump(
    net: *mut net,
    nb: *mut notifier_block,
    extack: *mut netlink_ext_ack,
) -> c_int {
    let err = fib4_rules_dump(net, nb, extack);
    if err != 0 {
        return err;
    }
    fib_notify(net, nb, extack)
}

static mut fib4_notifier_ops_template: fib_notifier_ops = fib_notifier_ops {
    family: AF_INET,
    fib_seq_read: Some(fib4_seq_read),
    fib_dump: Some(fib4_dump),
    owner: core::ptr::null_mut(), // THIS_MODULE
};

pub unsafe extern "C" fn fib4_notifier_init(net: *mut net) -> c_int {
    (*net).ipv4.fib_seq = 0;

    let ops = fib_notifier_ops_register(&fib4_notifier_ops_template, net);
    if (ops as isize) < 0 {
        return ptr_err(ops);
    }
    (*net).ipv4.notifier_ops = ops;
    0
}

pub unsafe extern "C" fn fib4_notifier_exit(net: *mut net) {
    fib_notifier_ops_unregister((*net).ipv4.notifier_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
