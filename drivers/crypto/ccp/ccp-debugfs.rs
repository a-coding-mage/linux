// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Cryptographic Coprocessor (CCP) driver
 *
 * Copyright (C) 2017 Advanced Micro Devices, Inc.
 *
 * Author: Gary R Hook <gary.hook@amd.com>
 */

// Dependencies supplied by the Linux kernel and other translation units.

const OBUFLEN: usize = 512;
const BUFLEN: usize = 63;

const RI_VERSION_NUM: u32 = 0x0000003F;
const RI_AES_PRESENT: u32 = 0x00000040;
const RI_3DES_PRESENT: u32 = 0x00000080;
const RI_SHA_PRESENT: u32 = 0x00000100;
const RI_RSA_PRESENT: u32 = 0x00000200;
const RI_ECC_PRESENT: u32 = 0x00000400;
const RI_ZDE_PRESENT: u32 = 0x00000800;
const RI_ZCE_PRESENT: u32 = 0x00001000;
const RI_TRNG_PRESENT: u32 = 0x00002000;
const RI_ELFC_PRESENT: u32 = 0x00004000;
const RI_ELFC_SHIFT: u32 = 14;
const RI_NUM_VQM: u32 = 0x00078000;
const RI_NVQM_SHIFT: u32 = 15;
const RI_LSB_ENTRIES: u32 = 0x0FF80000;
const RI_NLSB_SHIFT: u32 = 19;

#[inline]
const fn ri_nvqm(r: u32) -> u32 { (r.wrapping_mul(RI_NUM_VQM)) >> RI_NVQM_SHIFT }
#[inline]
const fn ri_nlsb(r: u32) -> u32 { (r.wrapping_mul(RI_LSB_ENTRIES)) >> RI_NLSB_SHIFT }

const MAX_NAME_LEN: usize = 20;

// The following types and functions are provided by the surrounding kernel translation.
extern "C" {
    static THIS_MODULE: *mut core::ffi::c_void;
    static mut ccp_debugfs_dir: *mut dentry;
    fn kmalloc(size: usize, flags: u32) -> *mut i8;
    fn kfree(ptr: *mut i8);
    fn ioread32(addr: *mut core::ffi::c_void) -> u32;
    fn simple_read_from_buffer(ubuf: *mut i8, count: usize, offp: *mut loff_t,
                               buf: *const i8, len: usize) -> isize;
    fn scnprintf(buf: *mut i8, size: usize, fmt: *const i8, ...) -> usize;
    fn simple_open(filp: *mut file, inode: *mut inode) -> isize;
    fn debugfs_initialized() -> bool;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn debugfs_create_dir(name: *const i8, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(name: *const i8, mode: u32, parent: *mut dentry,
                           data: *mut core::ffi::c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove_recursive(root: *mut dentry);
    fn snprintf(buf: *mut i8, size: usize, fmt: *const i8, ...) -> i32;
}

#[repr(C)] pub struct file { pub private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct inode;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct loff_t;
#[repr(C)] pub struct file_operations {
    pub owner: *mut core::ffi::c_void,
    pub open: Option<unsafe extern "C" fn(*mut file, *mut inode) -> isize>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut i8, usize, *mut loff_t) -> isize>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const i8, usize, *mut loff_t) -> isize>,
}

#[repr(C)] pub struct ccp_device {
    pub name: *const i8, pub rngname: *const i8, pub cmd_q_count: u32, pub cmd_count: u32,
    pub io_regs: *mut core::ffi::c_void, pub cmd_q: *mut ccp_cmd_queue,
    pub total_interrupts: u64, pub debugfs_instance: *mut dentry,
}
#[repr(C)] pub struct ccp_cmd_queue {
    pub total_ops: u64, pub total_aes_ops: u64, pub total_xts_aes_ops: u64,
    pub total_3des_ops: u64, pub total_sha_ops: u64, pub total_rsa_ops: u64,
    pub total_pt_ops: u64, pub total_ecc_ops: u64, pub id: i32,
    pub reg_int_enable: *mut core::ffi::c_void,
}

extern "C" { static mut ccp_debugfs_lock: mutex; }

unsafe extern "C" fn ccp5_debugfs_reset_queue_stats(cmd_q: *mut ccp_cmd_queue) {
    (*cmd_q).total_ops = 0; (*cmd_q).total_aes_ops = 0; (*cmd_q).total_xts_aes_ops = 0;
    (*cmd_q).total_3des_ops = 0; (*cmd_q).total_sha_ops = 0; (*cmd_q).total_rsa_ops = 0;
    (*cmd_q).total_pt_ops = 0; (*cmd_q).total_ecc_ops = 0;
}

// Formatted debugfs reads and writes retain the C implementation's kernel-buffer behavior.
unsafe extern "C" fn ccp5_debugfs_info_read(filp: *mut file, ubuf: *mut i8, count: usize, offp: *mut loff_t) -> isize {
    let ccp = (*filp).private_data as *mut ccp_device; if ccp.is_null() { return 0; }
    let obuf = kmalloc(OBUFLEN, 0); if obuf.is_null() { return -12; }
    let mut oboff = 0usize; let mut regval;
    macro_rules! p { ($($a:tt)*) => {{ let n = scnprintf(obuf.add(oboff), OBUFLEN - oboff, concat!($($a)*, "\0").as_ptr() as *const i8); oboff += n; }} }
    p!("Device name: %s\n", (*ccp).name); p!("   RNG name: %s\n", (*ccp).rngname);
    p!("   # Queues: %d\n", (*ccp).cmd_q_count); p!("     # Cmds: %d\n", (*ccp).cmd_count);
    regval = ioread32((*ccp).io_regs); p!("    Version: %d\n", regval & RI_VERSION_NUM); p!("    Engines:");
    if regval & RI_AES_PRESENT != 0 { p!(" AES"); } if regval & RI_3DES_PRESENT != 0 { p!(" 3DES"); }
    if regval & RI_SHA_PRESENT != 0 { p!(" SHA"); } if regval & RI_RSA_PRESENT != 0 { p!(" RSA"); }
    if regval & RI_ECC_PRESENT != 0 { p!(" ECC"); } if regval & RI_ZDE_PRESENT != 0 { p!(" ZDE"); }
    if regval & RI_ZCE_PRESENT != 0 { p!(" ZCE"); } if regval & RI_TRNG_PRESENT != 0 { p!(" TRNG"); }
    p!("\n"); p!("     Queues: %d\n", (regval & RI_NUM_VQM) >> RI_NVQM_SHIFT);
    p!("LSB Entries: %d\n", (regval & RI_LSB_ENTRIES) >> RI_NLSB_SHIFT);
    let ret = simple_read_from_buffer(ubuf, count, offp, obuf, oboff); kfree(obuf); ret
}

unsafe extern "C" fn ccp5_debugfs_stats_write(filp: *mut file, _ubuf: *const i8, count: usize, _offp: *mut loff_t) -> isize {
    let ccp = (*filp).private_data as *mut ccp_device;
    for i in 0..(*ccp).cmd_q_count { ccp5_debugfs_reset_queue_stats((*ccp).cmd_q.add(i as usize)); }
    (*ccp).total_interrupts = 0; count as isize
}

unsafe extern "C" fn ccp5_debugfs_queue_write(filp: *mut file, _ubuf: *const i8, count: usize, _offp: *mut loff_t) -> isize {
    ccp5_debugfs_reset_queue_stats((*filp).private_data as *mut ccp_cmd_queue); count as isize
}

unsafe extern "C" fn ccp5_debugfs_stats_read(filp: *mut file, ubuf: *mut i8, count: usize, offp: *mut loff_t) -> isize {
    let ccp = (*filp).private_data as *mut ccp_device; let mut totals = [0u64; 8];
    for i in 0..(*ccp).cmd_q_count { let q = &*(*ccp).cmd_q.add(i as usize); totals[0]+=q.total_ops; totals[1]+=q.total_aes_ops; totals[2]+=q.total_xts_aes_ops; totals[3]+=q.total_3des_ops; totals[4]+=q.total_sha_ops; totals[5]+=q.total_rsa_ops; totals[6]+=q.total_pt_ops; totals[7]+=q.total_ecc_ops; }
    let obuf = kmalloc(OBUFLEN, 0); if obuf.is_null() { return -12; } let mut n=0usize;
    macro_rules! p { ($($a:tt)*) => {{ n += scnprintf(obuf.add(n), OBUFLEN-n, concat!($($a)*, "\0").as_ptr() as *const i8); }} }
    p!("Total Interrupts Handled: %ld\n", (*ccp).total_interrupts); p!("        Total Operations: %ld\n", totals[0]); p!("                     AES: %ld\n", totals[1]); p!("                 XTS AES: %ld\n", totals[2]); p!("                     SHA: %ld\n", totals[3]); p!("                     SHA: %ld\n", totals[4]); p!("                     RSA: %ld\n", totals[5]); p!("               Pass-Thru: %ld\n", totals[6]); p!("                     ECC: %ld\n", totals[7]);
    let ret=simple_read_from_buffer(ubuf,count,offp,obuf,n); kfree(obuf); ret
}

unsafe extern "C" fn ccp5_debugfs_queue_read(filp: *mut file, ubuf: *mut i8, count: usize, offp: *mut loff_t) -> isize {
    let q=(*filp).private_data as *mut ccp_cmd_queue; if q.is_null(){return 0;} let obuf=kmalloc(OBUFLEN,0); if obuf.is_null(){return -12;} let mut n=0usize;
    macro_rules! p { ($($a:tt)*) => {{ n += scnprintf(obuf.add(n), OBUFLEN-n, concat!($($a)*, "\0").as_ptr() as *const i8); }} }
    p!("  Total Queue Operations: %ld\n",(*q).total_ops); p!("                     AES: %ld\n",(*q).total_aes_ops); p!("                 XTS AES: %ld\n",(*q).total_xts_aes_ops); p!("                     SHA: %ld\n",(*q).total_3des_ops); p!("                     SHA: %ld\n",(*q).total_sha_ops); p!("                     RSA: %ld\n",(*q).total_rsa_ops); p!("               Pass-Thru: %ld\n",(*q).total_pt_ops); p!("                     ECC: %ld\n",(*q).total_ecc_ops);
    let ret=simple_read_from_buffer(ubuf,count,offp,obuf,n); kfree(obuf); ret
}

pub unsafe extern "C" fn ccp5_debugfs_setup(ccp: *mut ccp_device) {
    if !debugfs_initialized() { return; }
    mutex_lock(&raw mut ccp_debugfs_lock);
    if ccp_debugfs_dir.is_null() { ccp_debugfs_dir = debugfs_create_dir(b"ccp\0".as_ptr() as *const i8, core::ptr::null_mut()); }
    mutex_unlock(&raw mut ccp_debugfs_lock);
    (*ccp).debugfs_instance = debugfs_create_dir((*ccp).name, ccp_debugfs_dir);
}

pub unsafe extern "C" fn ccp5_debugfs_destroy() {
    mutex_lock(&raw mut ccp_debugfs_lock); debugfs_remove_recursive(ccp_debugfs_dir); ccp_debugfs_dir = core::ptr::null_mut(); mutex_unlock(&raw mut ccp_debugfs_lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
