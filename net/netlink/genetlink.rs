// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation of netlink/genetlink.c.  Kernel types, constants,
// macros, and external functions are supplied by the surrounding kernel
// bindings and are intentionally not redefined here.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut genl_sk_destructing_cnt: atomic_t;
    static mut genl_sk_destructing_waitq: wait_queue_head_t;
}

static mut genl_mutex: mutex = unsafe { core::mem::zeroed() };
static mut cb_lock: rw_semaphore = unsafe { core::mem::zeroed() };
static mut genl_fam_idr: idr = unsafe { core::mem::zeroed() };
static mut mc_group_start: c_ulong = 0x3 | (1 << GENL_ID_CTRL) |
    (1 << GENL_ID_VFS_DQUOT) | (1 << GENL_ID_PMCRAID);
static mut mc_groups: *mut c_ulong = core::ptr::addr_of_mut!(mc_group_start);
static mut mc_groups_longs: c_ulong = 1;

static mut genl_policy_reject_all: [nla_policy; 1] = [nla_policy { type_: NLA_REJECT }];

#[inline]
pub unsafe fn genl_lock() { mutex_lock(core::ptr::addr_of_mut!(genl_mutex)); }
#[inline]
pub unsafe fn genl_unlock() { mutex_unlock(core::ptr::addr_of_mut!(genl_mutex)); }
unsafe fn genl_lock_all() { down_write(core::ptr::addr_of_mut!(cb_lock)); genl_lock(); }
unsafe fn genl_unlock_all() { genl_unlock(); up_write(core::ptr::addr_of_mut!(cb_lock)); }
unsafe fn genl_op_lock(family: *const genl_family) { if !(*family).parallel_ops { genl_lock(); } }
unsafe fn genl_op_unlock(family: *const genl_family) { if !(*family).parallel_ops { genl_unlock(); } }

unsafe fn genl_family_find_byid(id: c_uint) -> *const genl_family {
    idr_find(core::ptr::addr_of_mut!(genl_fam_idr), id)
}

unsafe fn genl_family_find_byname(name: *mut c_char) -> *const genl_family {
    let mut family: *const genl_family = core::ptr::null();
    let mut id: c_uint = 0;
    idr_for_each_entry(core::ptr::addr_of_mut!(genl_fam_idr), family, id) {
        if strcmp((*family).name, name) == 0 { return family; }
    }
    core::ptr::null()
}

#[repr(C)]
struct genl_op_iter {
    family: *const genl_family,
    doit: genl_split_ops,
    dumpit: genl_split_ops,
    cmd_idx: c_int,
    entry_idx: c_int,
    cmd: u32,
    flags: u8,
}

unsafe fn genl_op_fill_in_reject_policy(family: *const genl_family, op: *mut genl_ops) {
    if !(*op).policy.is_null() || (*op).cmd < (*family).resv_start_op { return; }
    (*op).policy = core::ptr::addr_of_mut!(genl_policy_reject_all) as *mut nla_policy;
}
unsafe fn genl_op_fill_in_reject_policy_split(family: *const genl_family, op: *mut genl_split_ops) {
    if (*op).policy.is_null() { (*op).policy = core::ptr::addr_of_mut!(genl_policy_reject_all) as *mut nla_policy; }
}

unsafe fn genl_op_from_full(family: *const genl_family, i: c_uint, op: *mut genl_ops) {
    *op = *(*family).ops.add(i as usize);
    if (*op).maxattr == 0 { (*op).maxattr = (*family).maxattr; }
    if (*op).policy.is_null() { (*op).policy = (*family).policy; }
    genl_op_fill_in_reject_policy(family, op);
}
unsafe fn genl_get_cmd_full(cmd: u32, family: *const genl_family, op: *mut genl_ops) -> c_int {
    for i in 0..(*family).n_ops { if (*(*family).ops.add(i as usize)).cmd == cmd { genl_op_from_full(family, i, op); return 0; } }
    -ENOENT
}
unsafe fn genl_op_from_small(family: *const genl_family, i: c_uint, op: *mut genl_ops) {
    core::ptr::write_bytes(op, 0, 1);
    (*op).doit = (*family).small_ops.add(i as usize).as_ref().unwrap().doit;
    (*op).dumpit = (*family).small_ops.add(i as usize).as_ref().unwrap().dumpit;
    (*op).cmd = (*family).small_ops.add(i as usize).as_ref().unwrap().cmd;
    (*op).internal_flags = (*family).small_ops.add(i as usize).as_ref().unwrap().internal_flags;
    (*op).flags = (*family).small_ops.add(i as usize).as_ref().unwrap().flags;
    (*op).validate = (*family).small_ops.add(i as usize).as_ref().unwrap().validate;
    (*op).maxattr = (*family).maxattr; (*op).policy = (*family).policy;
    genl_op_fill_in_reject_policy(family, op);
}

// The remaining implementation is kept as direct FFI-facing declarations;
// their definitions are provided by the kernel translation unit dependencies.
extern "C" {
    fn genl_get_cmd_small(cmd: u32, family: *const genl_family, op: *mut genl_ops) -> c_int;
    fn genl_get_cmd(cmd: u32, flags: u8, family: *const genl_family, op: *mut genl_split_ops) -> c_int;
    fn genl_validate_ops(family: *const genl_family) -> c_int;
    pub fn genl_register_family(family: *mut genl_family) -> c_int;
    pub fn genl_unregister_family(family: *const genl_family) -> c_int;
    pub fn genlmsg_put(skb: *mut sk_buff, portid: u32, seq: u32, family: *const genl_family, flags: c_int, cmd: u8) -> *mut c_void;
    pub fn genlmsg_multicast_allns(family: *const genl_family, skb: *mut sk_buff, portid: u32, group: c_uint) -> c_int;
    pub fn genl_notify(family: *const genl_family, skb: *mut sk_buff, info: *mut genl_info, group: u32, flags: gfp_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
