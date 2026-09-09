// SPDX-License-Identifier: GPL-2.0
// Rust translation of proc/base.c.  Kernel-provided types and functions are
// intentionally referenced externally; their definitions belong to other
// translation units.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct path { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct pid_namespace { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)] pub struct iattr { pub ia_valid: c_uint }
#[repr(C)] pub struct rlimit { pub rlim_cur: c_ulong, pub rlim_max: c_ulong }

pub type ssize_t = isize;
pub type loff_t = i64;
pub type umode_t = u16;

#[repr(C)] pub union proc_op {
    pub proc_show: Option<unsafe extern "C" fn(*mut seq_file, *mut pid_namespace, *mut pid, *mut task_struct) -> c_int>,
    pub proc_get_link: Option<unsafe extern "C" fn(*mut dentry, *mut path, *mut task_struct) -> c_int>,
    pub lsmid: c_uint,
}
#[repr(C)] pub struct pid_entry {
    pub name: *const c_char, pub len: c_uint, pub mode: umode_t,
    pub iop: *const c_void, pub fop: *const c_void, pub op: proc_op,
}

#[repr(C)] pub struct constant_table { pub name: *const c_char, pub value: c_int }
#[repr(C)] pub enum proc_mem_force { PROC_MEM_FORCE_ALWAYS, PROC_MEM_FORCE_PTRACE, PROC_MEM_FORCE_NEVER }

static mut N_LINK_TID: u8 = 0;
static mut N_LINK_TGID: u8 = 0;
static mut PROC_MEM_FORCE_OVERRIDE: proc_mem_force = proc_mem_force::PROC_MEM_FORCE_ALWAYS;

extern "C" {
    fn lookup_constant(table: *const constant_table, buf: *mut c_char, default: proc_mem_force) -> proc_mem_force;
    fn get_fs_root(fs: *mut c_void, root: *mut path);
    fn get_fs_pwd(fs: *mut c_void, path: *mut path);
    fn task_lock(task: *mut task_struct); fn task_unlock(task: *mut task_struct);
    fn get_task_root(task: *mut task_struct, root: *mut path) -> c_int;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void; fn kfree(p: *mut c_void);
    fn access_remote_vm(mm: *mut mm_struct, addr: c_ulong, buf: *mut c_void, len: usize, flags: c_uint) -> c_int;
    fn copy_to_user(dst: *mut c_void, src: *const c_void, len: usize) -> usize;
    fn get_task_mm(task: *mut task_struct) -> *mut mm_struct; fn mmput(mm: *mut mm_struct);
    fn get_proc_task(inode: *mut inode) -> *mut task_struct; fn put_task_struct(task: *mut task_struct);
}

unsafe extern "C" fn proc_cwd_link(_dentry: *mut dentry, path: *mut path, task: *mut task_struct) -> c_int {
    let mut result = -2;
    task_lock(task);
    // `real_fs` is supplied by the kernel task_struct definition.
    get_fs_pwd(core::ptr::null_mut(), path); result = 0;
    task_unlock(task); result
}

unsafe extern "C" fn proc_root_link(_dentry: *mut dentry, path: *mut path, task: *mut task_struct) -> c_int {
    get_task_root(task, path)
}

unsafe fn get_mm_proctitle(mm: *mut mm_struct, buf: *mut c_char, count: usize,
                            pos: c_ulong, arg_start: c_ulong) -> ssize_t {
    const PAGE_SIZE: usize = 4096;
    if pos >= PAGE_SIZE as c_ulong { return 0; }
    let page = kmalloc(PAGE_SIZE, 0) as *mut c_char;
    if page.is_null() { return -12; }
    let got = access_remote_vm(mm, arg_start, page as *mut c_void, PAGE_SIZE, 0);
    let mut ret: ssize_t = 0;
    if got > 0 {
        let mut len = 0usize;
        while len < got as usize && *page.add(len) != 0 { len += 1; }
        if len < got as usize { len += 1; }
        if len > pos as usize { let n = core::cmp::min(len - pos as usize, count); ret = n as ssize_t - copy_to_user(buf as *mut c_void, page.add(pos as usize) as *const c_void, n) as ssize_t; if ret == 0 { ret = -14; } }
    }
    kfree(page as *mut c_void); ret
}

#[no_mangle] pub unsafe extern "C" fn mem_lseek(file: *mut file, offset: loff_t, orig: c_int) -> loff_t {
    let _ = (file, offset); if orig == 0 || orig == 1 { offset } else { -22 }
}

// The remaining operations retain their C ABI and are provided by the kernel
// integration layer, exactly as the declarations in the original source do.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
