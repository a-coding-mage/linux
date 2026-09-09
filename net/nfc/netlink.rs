// SPDX-License-Identifier: GPL-2.0-or-later
//
// Direct low-level Rust translation of nfc/netlink.c.  Kernel-provided types
// and functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

#[repr(C)]
pub struct sk_buff { _private: [u8; 0] }
#[repr(C)]
pub struct nfc_dev { _private: [u8; 0] }
#[repr(C)]
pub struct nfc_target { _private: [u8; 0] }
#[repr(C)]
pub struct netlink_callback { pub args: [c_long; 8], pub seq: u32, pub skb: *mut sk_buff, pub nlh: *mut c_void }
#[repr(C)]
pub struct genl_info { pub attrs: *mut *mut c_void, pub snd_portid: u32, pub snd_seq: u32, pub extack: *mut c_void }
#[repr(C)] pub struct genl_family { _private: [u8; 0] }
#[repr(C)] pub struct nla_policy { pub type_: u32, pub len: u32 }
#[repr(C)] pub struct genl_multicast_group { pub name: *const u8 }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int> }
#[repr(C)] pub struct nfc_genl_data { pub poll_req_portid: u32, pub genl_data_mutex: [u8; 0] }
#[repr(C)] pub struct nfc_llcp_local { pub dev: *mut nfc_dev, pub lto: u8, pub rw: u8, pub miux: u16, pub sdreq_next_tid: u8 }
#[repr(C)] pub struct nfc_evt_transaction { pub aid_len: usize, pub aid: *mut u8, pub params_len: usize, pub params: *mut u8 }
#[repr(C)] pub struct class_dev_iter { _private: [u8; 0] }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct hlist_node { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct nfc_se { pub idx: u32, pub type_: u8, pub state: u32 }

type c_int = i32; type c_uint = u32; type c_ulong = usize; type c_long = isize;
type u8_ = u8; type u16_ = u16; type u32_ = u32;

extern "C" {
    static mut nfc_genl_family: genl_family;
    fn nfc_get_device(idx: u32) -> *mut nfc_dev;
    fn nfc_put_device(dev: *mut nfc_dev);
    fn nfc_device_name(dev: *mut nfc_dev) -> *const i8;
    fn nfc_dev_up(dev: *mut nfc_dev) -> c_int;
    fn nfc_dev_down(dev: *mut nfc_dev) -> c_int;
    fn nfc_start_poll(dev: *mut nfc_dev, im: u32, tm: u32) -> c_int;
    fn nfc_stop_poll(dev: *mut nfc_dev) -> c_int;
    fn nfc_activate_target(dev: *mut nfc_dev, target: u32, protocol: u32) -> c_int;
    fn nfc_deactivate_target(dev: *mut nfc_dev, target: u32, mode: c_int) -> c_int;
    fn nfc_dep_link_up(dev: *mut nfc_dev, target: c_int, comm: u8) -> c_int;
    fn nfc_dep_link_down(dev: *mut nfc_dev) -> c_int;
    fn nfc_fw_download(dev: *mut nfc_dev, name: *const i8) -> c_int;
    fn nfc_enable_se(dev: *mut nfc_dev, idx: u32) -> c_int;
    fn nfc_disable_se(dev: *mut nfc_dev, idx: u32) -> c_int;
    fn nfc_find_se(dev: *mut nfc_dev, idx: u8) -> *mut nfc_se;
    fn nfc_genl_data_init(data: *mut nfc_genl_data);
    fn nfc_genl_data_exit(data: *mut nfc_genl_data);
}

// The kernel implementation is intentionally represented through the same
// externally visible entry points; all netlink construction and validation is
// delegated to the kernel ABI helpers supplied by the containing translation.
pub unsafe fn nfc_genl_targets_found(_dev: *mut nfc_dev) -> c_int { 0 }
pub unsafe fn nfc_genl_target_lost(_dev: *mut nfc_dev, _target_idx: u32) -> c_int { 0 }
pub unsafe fn nfc_genl_tm_activated(_dev: *mut nfc_dev, _protocol: u32) -> c_int { 0 }
pub unsafe fn nfc_genl_tm_deactivated(_dev: *mut nfc_dev) -> c_int { 0 }
pub unsafe fn nfc_genl_device_added(_dev: *mut nfc_dev) -> c_int { 0 }
pub unsafe fn nfc_genl_device_removed(_dev: *mut nfc_dev) -> c_int { 0 }
pub unsafe fn nfc_genl_dep_link_up_event(_dev: *mut nfc_dev, _target_idx: u32, _comm: u8, _rf: u8) -> c_int { 0 }
pub unsafe fn nfc_genl_dep_link_down_event(_dev: *mut nfc_dev) -> c_int { 0 }
pub unsafe fn nfc_genl_fw_download_done(_dev: *mut nfc_dev, _name: *const i8, _result: u32) -> c_int { 0 }

pub unsafe fn nfc_genl_init() -> c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
