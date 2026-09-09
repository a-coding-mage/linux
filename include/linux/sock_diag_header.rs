/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

use core::ffi::c_int;

#[repr(C)]
pub struct sock_diag_handler {
    pub owner: *mut module,
    pub family: __u8,
    pub dump: Option<unsafe extern "C" fn(*mut sk_buff, *mut nlmsghdr) -> c_int>,
    pub get_info: Option<unsafe extern "C" fn(*mut sk_buff, *mut sock) -> c_int>,
    pub destroy: Option<unsafe extern "C" fn(*mut sk_buff, *mut nlmsghdr) -> c_int>,
}

unsafe extern "C" {
    pub fn sock_diag_register(h: *const sock_diag_handler) -> c_int;
    pub fn sock_diag_unregister(h: *const sock_diag_handler);
}

#[repr(C)]
pub struct sock_diag_inet_compat {
    pub owner: *mut module,
    pub fn_: Option<unsafe extern "C" fn(*mut sk_buff, *mut nlmsghdr) -> c_int>,
}

unsafe extern "C" {
    pub fn sock_diag_register_inet_compat(ptr: *const sock_diag_inet_compat);
    pub fn sock_diag_unregister_inet_compat(ptr: *const sock_diag_inet_compat);
    pub fn __sock_gen_cookie(sk: *mut sock) -> u64;
}

pub unsafe fn sock_gen_cookie(sk: *mut sock) -> u64 {
    let cookie: u64;

    preempt_disable();
    cookie = __sock_gen_cookie(sk);
    preempt_enable();

    cookie
}

unsafe extern "C" {
    pub fn sock_diag_check_cookie(sk: *mut sock, cookie: *const __u32) -> c_int;
    pub fn sock_diag_save_cookie(sk: *mut sock, cookie: *mut __u32);
    pub fn sock_diag_put_meminfo(sk: *mut sock, skb: *mut sk_buff, attr: c_int) -> c_int;
    pub fn sock_diag_put_filterinfo(
        may_report_filterinfo: bool,
        sk: *mut sock,
        skb: *mut sk_buff,
        attrtype: c_int,
    ) -> c_int;
}

pub unsafe fn sock_diag_destroy_group(sk: *const sock) -> enum_sknetlink_groups {
    match (*sk).sk_family {
        AF_INET => {
            if (*sk).sk_type == SOCK_RAW {
                return SKNLGRP_NONE;
            }
            match (*sk).sk_protocol {
                IPPROTO_TCP => SKNLGRP_INET_TCP_DESTROY,
                IPPROTO_UDP => SKNLGRP_INET_UDP_DESTROY,
                _ => SKNLGRP_NONE,
            }
        }
        AF_INET6 => {
            if (*sk).sk_type == SOCK_RAW {
                return SKNLGRP_NONE;
            }
            match (*sk).sk_protocol {
                IPPROTO_TCP => SKNLGRP_INET6_TCP_DESTROY,
                IPPROTO_UDP => SKNLGRP_INET6_UDP_DESTROY,
                _ => SKNLGRP_NONE,
            }
        }
        _ => SKNLGRP_NONE,
    }
}

pub unsafe fn sock_diag_has_destroy_listeners(sk: *const sock) -> bool {
    let n: *const net = sock_net(sk);
    let group: enum_sknetlink_groups = sock_diag_destroy_group(sk);

    group != SKNLGRP_NONE
        && !(*n).diag_nlsk.is_null()
        && netlink_has_listeners((*n).diag_nlsk, group)
}

unsafe extern "C" {
    pub fn sock_diag_broadcast_destroy(sk: *mut sock);
    pub fn sock_diag_destroy(sk: *mut sock, err: c_int) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
