// SPDX-License-Identifier: GPL-2.0
/*
 * debugfs.h - a tiny little debug file system
 *
 * Rust source-level translation of the C header. External kernel types and
 * symbols are intentionally left as dependencies supplied by other headers.
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct file_operations { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }
#[repr(C)]
pub struct inode { _private: [u8; 0] }
#[repr(C)]
pub struct file { _private: [u8; 0] }
#[repr(C)]
pub struct seq_file { _private: [u8; 0] }
#[repr(C)]
pub struct vfsmount { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct atomic_t { _private: [u8; 0] }

pub type umode_t = u16;
pub type loff_t = i64;
pub type ssize_t = isize;
pub type size_t = usize;
pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type __user = c_void;
pub type __iomem = c_void;

#[repr(C)]
pub struct debugfs_blob_wrapper {
    pub data: *mut c_void,
    pub size: c_ulong,
}
#[repr(C)]
pub struct debugfs_reg32 {
    pub name: *mut c_char,
    pub offset: c_ulong,
}
#[repr(C)]
pub struct debugfs_regset32 {
    pub regs: *const debugfs_reg32,
    pub nregs: c_int,
    pub base: *mut __iomem,
    pub dev: *mut device,
}
#[repr(C)]
pub struct debugfs_u32_array {
    pub array: *mut u32,
    pub n_elements: u32,
}

extern "C" {
    pub static mut arch_debugfs_dir: *mut dentry;
}

// The C header selects the implementation with _Generic; Rust callers select
// the corresponding explicitly typed declaration.
#[inline]
pub unsafe fn debugfs_create_file(name: *const c_char, mode: umode_t, parent: *mut dentry,
                                   data: *mut c_void, fops: *const file_operations) -> *mut dentry {
    debugfs_create_file_full(name, mode, parent, data, core::ptr::null(), fops)
}
#[inline]
pub unsafe fn debugfs_create_file_aux(name: *const c_char, mode: umode_t, parent: *mut dentry,
                                      data: *mut c_void, aux: *const c_void,
                                      fops: *const file_operations) -> *mut dentry {
    debugfs_create_file_full(name, mode, parent, data, aux, fops)
}
#[inline]
pub unsafe fn debugfs_remove_recursive(dentry: *mut dentry) { debugfs_remove(dentry); }
#[inline]
pub unsafe fn debugfs_create_file_aux_num(name: *const c_char, mode: umode_t, parent: *mut dentry,
                                           data: *mut c_void, n: c_ulong,
                                           fops: *const file_operations) -> *mut dentry {
    debugfs_create_file_aux(name, mode, parent, data, n as *const c_void, fops)
}
#[inline]
pub unsafe fn debugfs_get_aux_num(f: *const file) -> c_ulong { debugfs_get_aux(f) as c_ulong }

pub type debugfs_automount_t = unsafe extern "C" fn(*mut dentry, *mut c_void) -> *mut vfsmount;
#[repr(C)]
pub struct debugfs_short_fops {
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
}

// CONFIG_DEBUG_FS declarations.
extern "C" {
    pub fn debugfs_lookup(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    pub fn debugfs_create_file_full(name: *const c_char, mode: umode_t, parent: *mut dentry, data: *mut c_void, aux: *const c_void, fops: *const file_operations) -> *mut dentry;
    pub fn debugfs_create_file_short(name: *const c_char, mode: umode_t, parent: *mut dentry, data: *mut c_void, aux: *const c_void, fops: *const debugfs_short_fops) -> *mut dentry;
    pub fn debugfs_create_file_unsafe(name: *const c_char, mode: umode_t, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    pub fn debugfs_create_file_size(name: *const c_char, mode: umode_t, parent: *mut dentry, data: *mut c_void, fops: *const file_operations, file_size: loff_t);
    pub fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    pub fn debugfs_create_symlink(name: *const c_char, parent: *mut dentry, dest: *const c_char) -> *mut dentry;
    pub fn debugfs_create_automount(name: *const c_char, parent: *mut dentry, f: debugfs_automount_t, data: *mut c_void) -> *mut dentry;
    pub fn debugfs_remove(dentry: *mut dentry);
    pub fn debugfs_lookup_and_remove(name: *const c_char, parent: *mut dentry);
    pub fn debugfs_get_aux(file: *const file) -> *mut c_void;
    pub fn debugfs_file_get(dentry: *mut dentry) -> c_int;
    pub fn debugfs_file_put(dentry: *mut dentry);
    pub fn debugfs_attr_read(file: *mut file, buf: *mut c_char, len: size_t, ppos: *mut loff_t) -> ssize_t;
    pub fn debugfs_attr_write(file: *mut file, buf: *const c_char, len: size_t, ppos: *mut loff_t) -> ssize_t;
    pub fn debugfs_attr_write_signed(file: *mut file, buf: *const c_char, len: size_t, ppos: *mut loff_t) -> ssize_t;
    pub fn debugfs_change_name(dentry: *mut dentry, fmt: *const c_char, ...) -> c_int;
    pub fn debugfs_initialized() -> bool;
}

macro_rules! debugfs_create_scalar { ($name:ident, $ty:ty) => {
    extern "C" { pub fn $name(name: *const c_char, mode: umode_t, parent: *mut dentry, value: *mut $ty); }
}; }
debugfs_create_scalar!(debugfs_create_u8, u8);
debugfs_create_scalar!(debugfs_create_u16, u16);
debugfs_create_scalar!(debugfs_create_u32, u32);
debugfs_create_scalar!(debugfs_create_u64, u64);
debugfs_create_scalar!(debugfs_create_x8, u8);
debugfs_create_scalar!(debugfs_create_x16, u16);
debugfs_create_scalar!(debugfs_create_x32, u32);
debugfs_create_scalar!(debugfs_create_x64, u64);
debugfs_create_scalar!(debugfs_create_ulong, c_ulong);
debugfs_create_scalar!(debugfs_create_size_t, size_t);
debugfs_create_scalar!(debugfs_create_atomic_t, atomic_t);
debugfs_create_scalar!(debugfs_create_bool, bool);

extern "C" {
    pub fn debugfs_create_str(name: *const c_char, mode: umode_t, parent: *mut dentry, value: *mut *mut c_char);
    pub fn debugfs_create_blob(name: *const c_char, mode: umode_t, parent: *mut dentry, blob: *mut debugfs_blob_wrapper) -> *mut dentry;
    pub fn debugfs_create_regset32(name: *const c_char, mode: umode_t, parent: *mut dentry, regset: *mut debugfs_regset32);
    pub fn debugfs_print_regs32(s: *mut seq_file, regs: *const debugfs_reg32, nregs: c_int, base: *mut __iomem, prefix: *mut c_char);
    pub fn debugfs_create_u32_array(name: *const c_char, mode: umode_t, parent: *mut dentry, array: *mut debugfs_u32_array);
    pub fn debugfs_create_devm_seqfile(dev: *mut device, name: *const c_char, parent: *mut dentry, read_fn: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>);
    pub fn debugfs_read_file_bool(file: *mut file, user_buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    pub fn debugfs_write_file_bool(file: *mut file, user_buf: *const c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
    pub fn debugfs_read_file_str(file: *mut file, user_buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t;
}

#[repr(C)]
pub struct debugfs_cancellation {
    pub list: list_head,
    pub cancel: Option<unsafe extern "C" fn(*mut dentry, *mut c_void)>,
    pub cancel_data: *mut c_void,
}
extern "C" {
    pub fn debugfs_enter_cancellation(file: *mut file, cancellation: *mut debugfs_cancellation);
    pub fn debugfs_leave_cancellation(file: *mut file, cancellation: *mut debugfs_cancellation);
}

pub unsafe fn debugfs_create_xul(name: *const c_char, mode: umode_t, parent: *mut dentry, value: *mut c_ulong) {
    if core::mem::size_of::<c_ulong>() == core::mem::size_of::<u32>() {
        debugfs_create_x32(name, mode, parent, value as *mut u32);
    } else {
        debugfs_create_x64(name, mode, parent, value as *mut u64);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
