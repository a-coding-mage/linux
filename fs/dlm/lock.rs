// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of dlm/lock.c.  Kernel and DLM types,
// constants, and functions are supplied by the surrounding repository.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// The original includes provide these external declarations:
// linux/types.h, linux/rbtree.h, linux/slab.h, dlm_internal.h,
// linux/dlm_device.h, memory.h, midcomms.h, requestqueue.h, util.h, dir.h,
// member.h, lockspace.h, ast.h, lock.h, rcom.h, recover.h, lvb_table.h,
// user.h, and config.h.

extern "C" {
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn test_bit(nr: i32, addr: *const usize) -> i32;
    fn test_and_clear_bit(nr: i32, addr: *mut usize) -> i32;
    fn dlm_iflags_val(lkb: *const dlm_lkb) -> i32;
    fn dlm_sbflags_val(lkb: *const dlm_lkb) -> i32;
    fn dlm_add_cb(lkb: *mut dlm_lkb, cb_type: i32, mode: i32, rv: i32, flags: i32);
    fn send_bast(r: *mut dlm_rsb, lkb: *mut dlm_lkb, mode: i32) -> i32;
    fn dlm_our_nodeid() -> i32;
    fn dlm_locking_stopped(ls: *mut dlm_ls) -> bool;
    fn mod_timer(timer: *mut timer_list, expires: usize) -> i32;
    fn timer_pending(timer: *const timer_list) -> bool;
    fn timer_delete(timer: *mut timer_list);
}

#[repr(C)]
pub struct dlm_lkb {
    pub lkb_nodeid: i32,
    pub lkb_id: u32,
    pub lkb_remid: u32,
    pub lkb_exflags: u32,
    pub lkb_status: i32,
    pub lkb_rqmode: i32,
    pub lkb_grmode: i32,
    pub lkb_wait_type: i32,
    pub lkb_wait_nodeid: i32,
    pub lkb_recover_seq: u64,
    pub lkb_sbflags: usize,
    pub lkb_iflags: usize,
    pub lkb_lksb: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct dlm_rsb { pub res_nodeid: i32 }
#[repr(C)] pub struct dlm_ls { pub _opaque: [u8; 0] }
#[repr(C)] pub struct kref { pub refcount: usize }
#[repr(C)] pub struct rwlock_t { pub _opaque: [u8; 0] }
#[repr(C)] pub struct timer_list { pub _opaque: [u8; 0] }

pub const DLM_LOCK_PR: i32 = 4;
pub const DLM_LOCK_CW: i32 = 3;
pub const DLM_LKSTS_GRANTED: i32 = 1;
pub const DLM_LKF_NOQUEUE: u32 = 0x0000_0001;
pub const DLM_LKF_NOQUEUEBAST: u32 = 0x0000_0002;
pub const DLM_SBF_DEMOTED_BIT: i32 = 0;
pub const DLM_SBF_ALTMODE_BIT: i32 = 1;
pub const DLM_IFL_MSTCPY_BIT: i32 = 0;
pub const DLM_IFL_OVERLAP_UNLOCK_BIT: i32 = 1;
pub const DLM_IFL_OVERLAP_CANCEL_BIT: i32 = 2;

static __DLM_COMPAT_MATRIX: [[i32; 8]; 8] = [
    [1,1,1,1,1,1,1,0], [1,1,1,1,1,1,1,0], [1,1,1,1,1,1,0,0],
    [1,1,1,1,0,0,0,0], [1,1,1,0,1,0,0,0], [1,1,1,0,0,0,0,0],
    [1,1,0,0,0,0,0,0], [0,0,0,0,0,0,0,0],
];

#[no_mangle]
pub unsafe extern "C" fn dlm_modes_compat(mode1: i32, mode2: i32) -> i32 {
    __DLM_COMPAT_MATRIX[(mode1 + 1) as usize][(mode2 + 1) as usize]
}

pub const dlm_lvb_operations: [[i32; 8]; 8] = [
    [-1,1,1,1,1,1,1,-1], [-1,1,1,1,1,1,1,0], [-1,-1,1,1,1,1,1,0],
    [-1,-1,-1,1,1,1,1,0], [-1,-1,-1,-1,1,1,1,0], [-1,0,0,0,0,0,1,0],
    [-1,0,0,0,0,0,0,0], [-1,0,0,0,0,0,0,0],
];

#[inline]
unsafe fn middle_conversion(lkb: *const dlm_lkb) -> i32 {
    ((*lkb).lkb_grmode == DLM_LOCK_PR && (*lkb).lkb_rqmode == DLM_LOCK_CW ||
     (*lkb).lkb_rqmode == DLM_LOCK_PR && (*lkb).lkb_grmode == DLM_LOCK_CW) as i32
}

#[inline]
unsafe fn down_conversion(lkb: *const dlm_lkb) -> i32 {
    ((!middle_conversion(lkb) != 0 && (*lkb).lkb_rqmode < (*lkb).lkb_grmode)) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
