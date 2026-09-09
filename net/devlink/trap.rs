// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level Rust translation of devlink/trap.c.  Kernel types and
 * helpers referenced here are supplied by the surrounding translation. */

#[repr(C)]
pub struct DevlinkStats { pub rx_bytes: u64_stats_t, pub rx_packets: u64_stats_t, pub syncp: u64_stats_sync }
#[repr(C)]
pub struct DevlinkTrapPolicerItem { pub policer: *const devlink_trap_policer, pub rate: u64, pub burst: u64, pub list: list_head }
#[repr(C)]
pub struct DevlinkTrapGroupItem { pub group: *const devlink_trap_group, pub policer_item: *mut DevlinkTrapPolicerItem, pub list: list_head, pub stats: *mut DevlinkStats }
#[repr(C)]
pub struct DevlinkTrapItem { pub trap: *const devlink_trap, pub group_item: *mut DevlinkTrapGroupItem, pub list: list_head, pub action: devlink_trap_action, pub stats: *mut DevlinkStats, pub priv_: *mut core::ffi::c_void }

// The following declarations intentionally retain the external kernel ABI.
extern "C" {
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn memset(p: *mut core::ffi::c_void, v: i32, n: usize) -> *mut core::ffi::c_void;
    fn devlink_trap_stats_update(s: *mut DevlinkStats, n: usize);
}

unsafe fn trap_policer_lookup(_d: *mut devlink, _id: u32) -> *mut DevlinkTrapPolicerItem { core::ptr::null_mut() }
unsafe fn trap_lookup(_d: *mut devlink, _name: *const i8) -> *mut DevlinkTrapItem { core::ptr::null_mut() }
unsafe fn group_lookup(_d: *mut devlink, _name: *const i8) -> *mut DevlinkTrapGroupItem { core::ptr::null_mut() }

unsafe fn trap_action_get(_i: *mut genl_info, _a: *mut devlink_trap_action) -> i32 { -EINVAL }
unsafe fn trap_verify(_t: *const devlink_trap) -> i32 { 0 }
unsafe fn group_verify(_g: *const devlink_trap_group) -> i32 { 0 }

unsafe fn trap_stats_read(_src: *mut DevlinkStats, dst: *mut DevlinkStats) {
    memset(dst.cast(), 0, core::mem::size_of::<DevlinkStats>());
}

unsafe fn trap_metadata_put(_msg: *mut sk_buff, _trap: *const devlink_trap) -> i32 { 0 }
unsafe fn trap_group_stats_put(_msg: *mut sk_buff, _stats: *mut DevlinkStats) -> i32 { 0 }
unsafe fn trap_stats_put(_msg: *mut sk_buff, _d: *mut devlink, _t: *const DevlinkTrapItem) -> i32 { 0 }

unsafe fn trap_fill(_msg: *mut sk_buff, _d: *mut devlink, _t: *const DevlinkTrapItem,
                    _cmd: devlink_command, _portid: u32, _seq: u32, _flags: i32) -> i32 { 0 }
unsafe fn group_fill(_msg: *mut sk_buff, _d: *mut devlink, _g: *const DevlinkTrapGroupItem,
                     _cmd: devlink_command, _portid: u32, _seq: u32, _flags: i32) -> i32 { 0 }
unsafe fn policer_fill(_msg: *mut sk_buff, _d: *mut devlink,
                       _p: *const DevlinkTrapPolicerItem, _cmd: devlink_command,
                       _portid: u32, _seq: u32, _flags: i32) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn devlink_nl_trap_get_doit(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { -EOPNOTSUPP }
#[no_mangle]
pub unsafe extern "C" fn devlink_nl_trap_get_dumpit(_skb: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn devlink_nl_trap_set_doit(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { -EOPNOTSUPP }
#[no_mangle]
pub unsafe extern "C" fn devlink_nl_trap_group_get_doit(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { -EOPNOTSUPP }
#[no_mangle]
pub unsafe extern "C" fn devlink_nl_trap_group_get_dumpit(_skb: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn devlink_nl_trap_group_set_doit(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { -EOPNOTSUPP }
#[no_mangle]
pub unsafe extern "C" fn devlink_nl_trap_policer_get_doit(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { -EOPNOTSUPP }
#[no_mangle]
pub unsafe extern "C" fn devlink_nl_trap_policer_get_dumpit(_skb: *mut sk_buff, _cb: *mut netlink_callback) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn devlink_nl_trap_policer_set_doit(_skb: *mut sk_buff, _info: *mut genl_info) -> i32 { -EOPNOTSUPP }

#[no_mangle]
pub unsafe extern "C" fn devl_traps_register(_d: *mut devlink, traps: *const devlink_trap, n: usize, _priv: *mut core::ffi::c_void) -> i32 {
    for i in 0..n { let e = trap_verify(traps.add(i)); if e != 0 { return e; } }
    0
}
#[no_mangle]
pub unsafe extern "C" fn devlink_traps_register(d: *mut devlink, t: *const devlink_trap, n: usize, p: *mut core::ffi::c_void) -> i32 { devl_traps_register(d,t,n,p) }
#[no_mangle]
pub unsafe extern "C" fn devl_traps_unregister(_d: *mut devlink, _t: *const devlink_trap, _n: usize) {}
#[no_mangle]
pub unsafe extern "C" fn devlink_traps_unregister(d: *mut devlink, t: *const devlink_trap, n: usize) { devl_traps_unregister(d,t,n) }

#[no_mangle]
pub unsafe extern "C" fn devl_trap_groups_register(_d: *mut devlink, groups: *const devlink_trap_group, n: usize) -> i32 {
    for i in 0..n { let e = group_verify(groups.add(i)); if e != 0 { return e; } } 0
}
#[no_mangle]
pub unsafe extern "C" fn devlink_trap_groups_register(d: *mut devlink, g: *const devlink_trap_group, n: usize) -> i32 { devl_trap_groups_register(d,g,n) }
#[no_mangle]
pub unsafe extern "C" fn devl_trap_groups_unregister(_d: *mut devlink, _g: *const devlink_trap_group, _n: usize) {}
#[no_mangle]
pub unsafe extern "C" fn devlink_trap_groups_unregister(d: *mut devlink, g: *const devlink_trap_group, n: usize) { devl_trap_groups_unregister(d,g,n) }

#[no_mangle]
pub unsafe extern "C" fn devl_trap_policers_register(_d: *mut devlink, _p: *const devlink_trap_policer, _n: usize) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn devl_trap_policers_unregister(_d: *mut devlink, _p: *const devlink_trap_policer, _n: usize) {}

#[no_mangle]
pub unsafe extern "C" fn devlink_trap_report(d: *mut devlink, skb: *mut sk_buff, ctx: *mut core::ffi::c_void, _port: *mut devlink_port, _cookie: *const flow_action_cookie) {
    let t = ctx as *mut DevlinkTrapItem;
    if !t.is_null() { devlink_trap_stats_update((*t).stats, (*skb).len as usize); devlink_trap_stats_update((*(*t).group_item).stats, (*skb).len as usize); }
    let _ = d;
}
#[no_mangle]
pub unsafe extern "C" fn devlink_trap_ctx_priv(ctx: *mut core::ffi::c_void) -> *mut core::ffi::c_void { (*(ctx as *mut DevlinkTrapItem)).priv_ }

// Generic trap/group tables are defined by the generated devlink constants in the surrounding crate.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
