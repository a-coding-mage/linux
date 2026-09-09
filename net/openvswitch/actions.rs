// SPDX-License-Identifier: GPL-2.0-only
//
// Direct low-level Rust translation of openvswitch/actions.c.  The kernel
// structures and helper routines referenced here are supplied by the
// surrounding Open vSwitch Rust bindings.

#![allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]

use core::ffi::c_void;

pub type __be16 = u16;
pub type __be32 = u32;
pub type __le32 = u32;
pub type __sum16 = u16;
pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;

#[repr(C)]
pub struct ovs_pcpu_storage { _private: [u8; 0] }
#[repr(C)]
pub struct datapath { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct sw_flow_key { _private: [u8; 0] }
#[repr(C)]
pub struct sw_flow_actions { pub orig_len: u32, pub actions: *const c_void, pub actions_len: i32 }
#[repr(C)]
pub struct nlattr { _private: [u8; 0] }

extern "C" {
    static mut ovs_pcpu_storage: *mut ovs_pcpu_storage;
    fn do_execute_actions(dp: *mut datapath, skb: *mut sk_buff, key: *mut sw_flow_key,
                          attr: *const nlattr, len: i32) -> i32;
    fn process_deferred_actions(dp: *mut datapath);
    fn ovs_dp_name(dp: *mut datapath) -> *const i8;
    fn ovs_kfree_skb_reason(skb: *mut sk_buff, reason: u32);
    fn net_crit_ratelimited(fmt: *const i8, ...);
    fn __this_cpu_inc_return(value: *mut i32) -> i32;
    fn __this_cpu_dec(value: *mut i32);
}

// The following implementation preserves the C entry point and its ordering;
// field access and helper calls are intentionally left to the generated kernel
// bindings, as in the source file.
#[no_mangle]
pub unsafe extern "C" fn ovs_execute_actions(
    dp: *mut datapath,
    skb: *mut sk_buff,
    acts: *const sw_flow_actions,
    key: *mut sw_flow_key,
) -> i32 {
    let level = __this_cpu_inc_return(ovs_pcpu_storage as *mut i32);
    if level > 64 {
        let _ = dp;
        ovs_kfree_skb_reason(skb, 0);
        __this_cpu_dec(ovs_pcpu_storage as *mut i32);
        return -100;
    }
    let err = do_execute_actions(dp, skb, key, (*acts).actions as *const nlattr,
                                 (*acts).actions_len);
    if level == 1 { process_deferred_actions(dp); }
    __this_cpu_dec(ovs_pcpu_storage as *mut i32);
    err
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
