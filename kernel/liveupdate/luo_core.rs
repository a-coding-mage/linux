// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2025, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>

//! Live Update Orchestrator (LUO) core implementation.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// Kernel-provided types and functions are supplied by the surrounding tree.
#[repr(C)] pub struct luo_ser { pub compatible: [c_char; 64], pub liveupdate_num: u64, pub sessions_pa: u64, pub flbs_pa: u64 }
#[repr(C)] pub struct luo_ucmd { pub ubuffer: *mut c_void, pub user_size: u32, pub cmd: *mut c_void }
#[repr(C)] pub struct liveupdate_ioctl_create_session { pub fd: c_int, pub name: [c_char; 256] }
#[repr(C)] pub struct liveupdate_ioctl_retrieve_session { pub fd: c_int, pub name: [c_char; 256] }
#[repr(C)] pub struct inode;
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct miscdevice { pub minor: c_int, pub name: *const c_char, pub fops: *const file_operations }
#[repr(C)] pub struct file_operations { pub owner: *mut c_void, pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>, pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>, pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long> }
#[repr(C)] pub struct atomic_t(pub c_int);

extern "C" {
    fn kho_is_enabled() -> bool;
    fn kho_retrieve_subtree(*const c_char, *mut u64, *mut usize) -> c_int;
    fn phys_to_virt(u64) -> *mut c_void;
    fn kho_restore_free(*mut luo_ser);
    fn luo_session_setup_incoming(u64) -> c_int;
    fn luo_flb_setup_incoming(u64);
    fn luo_session_setup_outgoing(*mut u64);
    fn luo_flb_setup_outgoing(*mut u64) -> c_int;
    fn kho_alloc_preserve(usize) -> *mut luo_ser;
    fn kho_unpreserve_free(*mut luo_ser);
    fn kho_add_subtree(*const c_char, *mut luo_ser, usize) -> c_int;
    fn luo_session_serialize() -> c_int;
    fn luo_flb_serialize();
    fn luo_session_deserialize() -> c_int;
    fn luo_session_create(*const c_char, *mut *mut file) -> c_int;
    fn luo_session_retrieve(*const c_char, *mut *mut file) -> c_int;
    fn luo_ucmd_respond(*mut luo_ucmd, usize) -> c_int;
    fn get_unused_fd_flags(c_uint) -> c_int;
    fn put_unused_fd(c_int);
    fn fd_install(c_int, *mut file);
    fn fput(*mut file);
    fn misc_register(*mut miscdevice) -> c_int;
    fn atomic_cmpxchg(*mut atomic_t, c_int, c_int) -> c_int;
    fn atomic_set(*mut atomic_t, c_int);
    fn luo_restore_fail(*const c_char, ...);
}

static mut LUO_GLOBAL: LuOGlobal = LuOGlobal { enabled: false, luo_ser_out: core::ptr::null_mut(), liveupdate_num: 0 };
#[repr(C)] struct LuOGlobal { enabled: bool, luo_ser_out: *mut luo_ser, liveupdate_num: u64 }

#[repr(C)] union ucmd_buffer { create: liveupdate_ioctl_create_session, retrieve: liveupdate_ioctl_retrieve_session }
#[repr(C)] struct luo_device_state { miscdev: miscdevice, in_use: atomic_t }
#[repr(C)] struct luo_ioctl_op { size: usize, min_size: usize, ioctl_num: c_uint, execute: Option<unsafe extern "C" fn(*mut luo_ucmd) -> c_int> }

unsafe fn liveupdate_enabled() -> bool { LUO_GLOBAL.enabled }

unsafe fn luo_early_startup() -> c_int {
    if !kho_is_enabled() { LUO_GLOBAL.enabled = false; return 0; }
    let mut phys = 0u64; let mut len = 0usize;
    let err = kho_retrieve_subtree(b"luo\0".as_ptr() as *const c_char, &mut phys, &mut len);
    if err != 0 { return if err == -2 { 0 } else { err }; }
    if len < core::mem::size_of::<luo_ser>() { return -22; }
    let ser = phys_to_virt(phys) as *mut luo_ser;
    LUO_GLOBAL.liveupdate_num = (*ser).liveupdate_num;
    let mut err = luo_session_setup_incoming((*ser).sessions_pa);
    if err == 0 { luo_flb_setup_incoming((*ser).flbs_pa); }
    kho_restore_free(ser); err
}

unsafe fn liveupdate_early_init() -> c_int {
    let err = luo_early_startup();
    if err != 0 { LUO_GLOBAL.enabled = false; }
    err
}

unsafe fn luo_state_setup() -> c_int {
    let ser = kho_alloc_preserve(core::mem::size_of::<luo_ser>());
    if ser.is_null() { return -12; }
    (*ser).liveupdate_num = LUO_GLOBAL.liveupdate_num.wrapping_add(1);
    luo_session_setup_outgoing(&mut (*ser).sessions_pa);
    let mut err = luo_flb_setup_outgoing(&mut (*ser).flbs_pa);
    if err == 0 { err = kho_add_subtree(b"luo\0".as_ptr() as *const c_char, ser, core::mem::size_of::<luo_ser>()); }
    if err != 0 { kho_unpreserve_free(ser); } else { LUO_GLOBAL.luo_ser_out = ser; }
    err
}

unsafe fn luo_late_startup() -> c_int { if !liveupdate_enabled() { 0 } else { let e = luo_state_setup(); if e != 0 { LUO_GLOBAL.enabled = false; } e } }

#[no_mangle] pub unsafe extern "C" fn liveupdate_reboot() -> c_int {
    if !liveupdate_enabled() { return 0; }
    let err = luo_session_serialize(); if err != 0 { return err; } luo_flb_serialize(); 0
}

#[no_mangle] pub unsafe extern "C" fn liveupdate_enabled_export() -> bool { liveupdate_enabled() }

unsafe extern "C" fn luo_ioctl_create_session(u: *mut luo_ucmd) -> c_int {
    let a = (*u).cmd as *mut liveupdate_ioctl_create_session; let fd = get_unused_fd_flags(0x80000); (*a).fd = fd;
    if fd < 0 { return fd; }
    let mut f = core::ptr::null_mut(); let mut err = luo_session_create((*a).name.as_ptr(), &mut f);
    if err == 0 { err = luo_ucmd_respond(u, core::mem::size_of::<liveupdate_ioctl_create_session>()); }
    if err == 0 { fd_install(fd, f); } else { if !f.is_null() { fput(f); } put_unused_fd(fd); } err
}

unsafe extern "C" fn luo_ioctl_retrieve_session(u: *mut luo_ucmd) -> c_int {
    let a = (*u).cmd as *mut liveupdate_ioctl_retrieve_session; let fd = get_unused_fd_flags(0x80000); (*a).fd = fd;
    if fd < 0 { return fd; }
    let mut f = core::ptr::null_mut(); let mut err = luo_session_retrieve((*a).name.as_ptr(), &mut f);
    if err == 0 { err = luo_ucmd_respond(u, core::mem::size_of::<liveupdate_ioctl_retrieve_session>()); }
    if err == 0 { fd_install(fd, f); } else { if !f.is_null() { fput(f); } put_unused_fd(fd); } err
}

unsafe extern "C" fn luo_open(_i: *mut inode, f: *mut file) -> c_int {
    let d = (*f).private_data as *mut luo_device_state;
    if atomic_cmpxchg(&mut (*d).in_use, 0, 1) != 0 { return -16; }
    if luo_session_deserialize() != 0 { atomic_set(&mut (*d).in_use, 0); return -5; } 0
}
unsafe extern "C" fn luo_release(_i: *mut inode, f: *mut file) -> c_int {
    let d = (*f).private_data as *mut luo_device_state; atomic_set(&mut (*d).in_use, 0); 0
}

static LUO_IOCTL_OPS: [luo_ioctl_op; 2] = [
    luo_ioctl_op { size: core::mem::size_of::<liveupdate_ioctl_create_session>(), min_size: 0, ioctl_num: 0, execute: Some(luo_ioctl_create_session) },
    luo_ioctl_op { size: core::mem::size_of::<liveupdate_ioctl_retrieve_session>(), min_size: 0, ioctl_num: 1, execute: Some(luo_ioctl_retrieve_session) },
];
unsafe extern "C" fn luo_ioctl(_f: *mut file, _cmd: c_uint, _arg: c_ulong) -> c_long { -22 }

static LUO_FOPS: file_operations = file_operations { owner: core::ptr::null_mut(), open: Some(luo_open), release: Some(luo_release), unlocked_ioctl: Some(luo_ioctl) };
static mut LUO_DEV: luo_device_state = luo_device_state { miscdev: miscdevice { minor: -1, name: b"liveupdate\0".as_ptr() as *const c_char, fops: &LUO_FOPS }, in_use: atomic_t(0) };

#[allow(dead_code)]
unsafe fn liveupdate_ioctl_init() -> c_int { if !liveupdate_enabled() { 0 } else { misc_register(&mut LUO_DEV.miscdev) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
