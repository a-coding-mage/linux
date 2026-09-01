/* SPDX-License-Identifier: GPL-2.0-or-later */

// C dependencies removed from executable Rust:
// <string.h>, <linux/netlink.h>, <linux/rtnetlink.h>
// Expected external Rust bindings: sockaddr_nl, nlmsghdr, rtattr, FILE,
// and NLMSG_ALIGN.

use core::ffi::{c_char, c_void};
use crate::{sockaddr_nl, nlmsghdr, rtattr, FILE, NLMSG_ALIGN};

#[repr(C)]
pub struct rtnl_handle {
    pub fd: i32,
    pub local: sockaddr_nl,
    pub peer: sockaddr_nl,
    pub seq: u32,
    pub dump: u32,
    pub proto: i32,
    pub dump_fp: *mut FILE,
    pub flags: i32,
}

pub const RTNL_HANDLE_F_LISTEN_ALL_NSID: i32 = 0x01;
pub const RTNL_HANDLE_F_SUPPRESS_NLERR: i32 = 0x02;
pub const RTNL_HANDLE_F_STRICT_CHK: i32 = 0x04;

#[inline]
pub unsafe fn NLMSG_TAIL(nmsg: *mut nlmsghdr) -> *mut rtattr {
    (nmsg as *mut u8).add(NLMSG_ALIGN((*nmsg).nlmsg_len) as usize) as *mut rtattr
}

pub type nl_ext_ack_fn_t = Option<
    unsafe extern "C" fn(
        errmsg: *const c_char,
        off: u32,
        inner_nlh: *const nlmsghdr,
    ) -> i32,
>;

unsafe extern "C" {
    #[must_use]
    pub fn rtnl_open(rth: *mut rtnl_handle, subscriptions: u32) -> i32;
    pub fn rtnl_close(rth: *mut rtnl_handle);
    #[must_use]
    pub fn rtnl_talk(
        rtnl: *mut rtnl_handle,
        n: *mut nlmsghdr,
        answer: *mut *mut nlmsghdr,
    ) -> i32;

    pub fn addattr(n: *mut nlmsghdr, maxlen: i32, type_: i32) -> i32;
    pub fn addattr8(n: *mut nlmsghdr, maxlen: i32, type_: i32, data: u8) -> i32;
    pub fn addattr16(n: *mut nlmsghdr, maxlen: i32, type_: i32, data: u16) -> i32;
    pub fn addattr32(n: *mut nlmsghdr, maxlen: i32, type_: i32, data: u32) -> i32;
    pub fn addattr64(n: *mut nlmsghdr, maxlen: i32, type_: i32, data: u64) -> i32;
    pub fn addattrstrz(
        n: *mut nlmsghdr,
        maxlen: i32,
        type_: i32,
        data: *const c_char,
    ) -> i32;
    pub fn addattr_l(
        n: *mut nlmsghdr,
        maxlen: i32,
        type_: i32,
        data: *const c_void,
        alen: i32,
    ) -> i32;
    pub fn addraw_l(
        n: *mut nlmsghdr,
        maxlen: i32,
        data: *const c_void,
        len: i32,
    ) -> i32;
    pub fn addattr_nest(n: *mut nlmsghdr, maxlen: i32, type_: i32) -> *mut rtattr;
    pub fn addattr_nest_end(n: *mut nlmsghdr, nest: *mut rtattr) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
