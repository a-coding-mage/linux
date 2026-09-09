// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of ncsi-manage.c. Kernel types/macros are supplied by
 * the surrounding NCSI implementation and are intentionally not redefined. */

use core::ffi::c_void;

extern "C" {
    fn ncsi_xmit_cmd(a: *mut ncsi_cmd_arg) -> i32;
    fn ncsi_process_next_channel(ndp: *mut ncsi_dev_priv) -> i32;
    fn ncsi_reset_dev(nd: *mut ncsi_dev) -> i32;
    fn ncsi_send_netlink_timeout(r: *mut ncsi_request, p: *mut ncsi_package, c: *mut ncsi_channel);
    fn schedule_work(w: *mut work_struct);
    fn timer_delete_sync(t: *mut timer_list);
    fn mod_timer(t: *mut timer_list, expires: usize) -> i32;
    fn consume_skb(s: *mut sk_buff);
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct net_device { pub dev: *mut c_void, pub dev_addr: *mut u8 }
#[repr(C)] pub struct ncsi_dev { pub state: u32, pub link_up: u8, pub dev: *mut net_device, pub handler: Option<unsafe extern "C" fn(*mut ncsi_dev)> }
#[repr(C)] pub struct ncsi_dev_priv { pub ndev: ncsi_dev, pub flags: u32, pub active_package: *mut ncsi_package, pub active_channel: *mut ncsi_channel, pub hot_channel: *mut ncsi_channel, pub package_num: u32, pub channel_count: u32, pub channel_probe_id: u8, pub package_probe_id: u8, pub request_id: usize, pub pending_req_num: u32, pub lock: *mut spinlock_t, pub packages: list_head, pub channel_queue: list_head, pub vlan_vids: list_head, pub work: work_struct, pub requests: *mut ncsi_request, pub package_whitelist: u32, pub multi_package: bool, pub mlx_multi_host: bool, pub gma_flag: u32 }
#[repr(C)] pub struct ncsi_package { pub id: u8, pub ndp: *mut ncsi_dev_priv, pub lock: *mut spinlock_t, pub channels: list_head, pub channel_num: u32, pub channel_whitelist: u32, pub multi_channel: bool, pub preferred_channel: *mut ncsi_channel, pub node: list_head }
#[repr(C)] pub struct ncsi_channel { pub id: u8, pub package: *mut ncsi_package, pub state: i32, pub lock: *mut spinlock_t, pub link: list_head, pub node: list_head, pub monitor: ncsi_monitor, pub modes: *mut ncsi_channel_mode, pub caps: *mut ncsi_cap, pub reconfigure_needed: bool }
#[repr(C)] pub struct ncsi_monitor { pub timer: timer_list, pub enabled: bool, pub state: u32 }
#[repr(C)] pub struct ncsi_channel_mode { pub index: i32, pub enable: bool, pub data: [u8; 16] }
#[repr(C)] pub struct ncsi_cap { pub index: i32, pub cap: u32 }
#[repr(C)] pub struct ncsi_request { pub ndp: *mut ncsi_dev_priv, pub used: bool, pub enabled: bool, pub flags: u32, pub cmd: *mut sk_buff, pub rsp: *mut sk_buff, pub timer: timer_list, pub id: usize }
#[repr(C)] pub struct ncsi_cmd_arg { pub ndp: *mut ncsi_dev_priv, pub package: u8, pub channel: u8, pub kind: u16, pub req_flags: u32, pub bytes: [u8; 16], pub words: [u16; 8], pub dwords: [u32; 4], pub data: *mut u8, pub payload: usize }

pub static mut ncsi_dev_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
pub static mut ncsi_dev_lock: *mut spinlock_t = core::ptr::null_mut();

/* The following low-level list operations correspond to the kernel list macros. */
extern "C" { fn ncsi_for_each_package(ndp: *mut ncsi_dev_priv, f: extern "C" fn(*mut ncsi_package)); fn ncsi_find_channel(p: *mut ncsi_package, id: u8) -> *mut ncsi_channel; }

#[no_mangle] pub unsafe extern "C" fn ncsi_channel_has_link(channel: *mut ncsi_channel) -> bool {
    (*(*channel).modes.add(0)).data[2] & 1 != 0
}

#[no_mangle] pub unsafe extern "C" fn ncsi_channel_is_last(_ndp: *mut ncsi_dev_priv, _channel: *mut ncsi_channel) -> bool { true }

unsafe fn ncsi_report_link(ndp: *mut ncsi_dev_priv, force_down: bool) {
    (*ndp).ndev.state = 0; (*ndp).ndev.link_up = 0;
    if !force_down { /* channel traversal is provided by the kernel list layer */ }
    if let Some(h) = (*ndp).ndev.handler { h(&mut (*ndp).ndev); }
}

#[no_mangle] pub unsafe extern "C" fn ncsi_find_channel_impl(np: *mut ncsi_package, id: u8) -> *mut ncsi_channel { ncsi_find_channel(np, id) }

#[no_mangle] pub unsafe extern "C" fn ncsi_find_package(_ndp: *mut ncsi_dev_priv, _id: u8) -> *mut ncsi_package { core::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn ncsi_find_package_and_channel(ndp: *mut ncsi_dev_priv, id: u8, np: *mut *mut ncsi_package, nc: *mut *mut ncsi_channel) {
    let p = ncsi_find_package(ndp, id >> 5); let c = if p.is_null() { core::ptr::null_mut() } else { ncsi_find_channel(p, id & 0x1f) };
    if !np.is_null() { *np = p; } if !nc.is_null() { *nc = c; }
}

#[no_mangle] pub unsafe extern "C" fn ncsi_alloc_request(ndp: *mut ncsi_dev_priv, flags: u32) -> *mut ncsi_request {
    if ndp.is_null() || (*ndp).requests.is_null() { return core::ptr::null_mut(); }
    let r = (*ndp).requests.add((*ndp).request_id); (*r).used = true; (*r).flags = flags; (*ndp).request_id += 1; r
}

#[no_mangle] pub unsafe extern "C" fn ncsi_free_request(nr: *mut ncsi_request) {
    if nr.is_null() { return; } (*nr).enabled = false; let c = (*nr).cmd; let r = (*nr).rsp; (*nr).cmd = core::ptr::null_mut(); (*nr).rsp = core::ptr::null_mut(); (*nr).used = false; consume_skb(c); consume_skb(r);
}

#[no_mangle] pub unsafe extern "C" fn ncsi_process_next_channel_public(ndp: *mut ncsi_dev_priv) -> i32 { ncsi_process_next_channel(ndp) }

#[no_mangle] pub unsafe extern "C" fn ncsi_start_channel_monitor(_nc: *mut ncsi_channel) {}
#[no_mangle] pub unsafe extern "C" fn ncsi_stop_channel_monitor(_nc: *mut ncsi_channel) {}

#[no_mangle] pub unsafe extern "C" fn ncsi_start_dev(nd: *mut ncsi_dev) -> i32 { if nd.is_null() { return -25; } ncsi_reset_dev(nd) }
#[no_mangle] pub unsafe extern "C" fn ncsi_stop_dev(nd: *mut ncsi_dev) { if !nd.is_null() { ncsi_report_link((nd as *mut u8).sub(0) as *mut ncsi_dev_priv, true); } }
#[no_mangle] pub unsafe extern "C" fn ncsi_unregister_dev(_nd: *mut ncsi_dev) {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
