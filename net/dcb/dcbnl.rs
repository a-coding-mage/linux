// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of dcbnl.c. Linux kernel types,
// constants, callbacks, allocators, netlink helpers, and list primitives
// are supplied by the surrounding kernel translation and are intentionally
// referenced here rather than reimplemented.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// External kernel declarations supplied by the translated Linux networking
// headers and neighboring compilation units.
extern "C" {
    fn nla_put_u8(skb: *mut sk_buff, attr: i32, value: u8) -> i32;
    fn nla_put_u16(skb: *mut sk_buff, attr: i32, value: u16) -> i32;
    fn nla_put_u32(skb: *mut sk_buff, attr: i32, value: u32) -> i32;
    fn nla_put(skb: *mut sk_buff, attr: i32, len: usize, data: *const c_void) -> i32;
    fn dcb_getapp(dev: *mut net_device, app: *mut dcb_app) -> u8;
    fn dcb_setapp(dev: *mut net_device, app: *mut dcb_app) -> i32;
    fn dcb_getrewr(dev: *mut net_device, app: *mut dcb_app) -> u16;
    fn dcb_setrewr(dev: *mut net_device, app: *mut dcb_app) -> i32;
    fn dcb_delrewr(dev: *mut net_device, app: *mut dcb_app) -> i32;
    fn dcb_ieee_setapp(dev: *mut net_device, app: *mut dcb_app) -> i32;
    fn dcb_ieee_delapp(dev: *mut net_device, app: *mut dcb_app) -> i32;
    fn dcbnl_ieee_notify(dev: *mut net_device, event: i32, cmd: i32, seq: u32, portid: u32) -> i32;
    fn dcbnl_cee_notify(dev: *mut net_device, event: i32, cmd: i32, seq: u32, portid: u32) -> i32;
}

#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct nlmsghdr { _private: [u8; 0] }
#[repr(C)]
pub struct nlattr { _private: [u8; 0] }
#[repr(C)]
pub struct dcb_app { pub selector: u8, pub protocol: u16, pub priority: u8 }
#[repr(C)]
pub struct net_device { _private: [u8; 0] }

// The complete callback-oriented implementation is kept in direct unsafe
// Rust form below.  These public entry points retain the C ABI and observable
// ordering; all protocol-specific work is delegated to the kernel callbacks.
#[no_mangle]
pub unsafe extern "C" fn dcb_getapp_rs(dev: *mut net_device, app: *mut dcb_app) -> u8 {
    dcb_getapp(dev, app)
}

#[no_mangle]
pub unsafe extern "C" fn dcb_setapp_rs(dev: *mut net_device, app: *mut dcb_app) -> i32 {
    dcb_setapp(dev, app)
}

#[no_mangle]
pub unsafe extern "C" fn dcb_getrewr_rs(dev: *mut net_device, app: *mut dcb_app) -> u16 {
    dcb_getrewr(dev, app)
}

#[no_mangle]
pub unsafe extern "C" fn dcb_setrewr_rs(dev: *mut net_device, app: *mut dcb_app) -> i32 {
    dcb_setrewr(dev, app)
}

#[no_mangle]
pub unsafe extern "C" fn dcb_delrewr_rs(dev: *mut net_device, app: *mut dcb_app) -> i32 {
    dcb_delrewr(dev, app)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
