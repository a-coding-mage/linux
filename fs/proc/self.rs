// SPDX-License-Identifier: GPL-2.0
// Dependencies corresponding to the Linux kernel headers and "internal.h"
// are supplied by the surrounding translation unit.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct dentry;
#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
    pub i_ino: c_uint,
    pub i_mode: c_uint,
    pub i_uid: c_uint,
    pub i_gid: c_uint,
    pub i_op: *const inode_operations,
}
#[repr(C)]
pub struct super_block {
    pub s_root: *mut dentry,
}
#[repr(C)]
pub struct delayed_call;
#[repr(C)]
pub struct pid_namespace;
#[repr(C)]
pub struct task_struct;

pub type pid_t = i32;

#[repr(C)]
pub struct inode_operations {
    pub get_link: Option<unsafe extern "C" fn(
        *mut dentry,
        *mut inode,
        *mut delayed_call,
    ) -> *const c_char>,
}

extern "C" {
    static mut current: *mut task_struct;
    static mut self_inum: c_uint;

    fn proc_pid_ns(sb: *mut super_block) -> *mut pid_namespace;
    fn task_tgid_nr_ns(task: *mut task_struct, ns: *mut pid_namespace) -> pid_t;
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
    fn set_delayed_call(done: *mut delayed_call, func: unsafe extern "C" fn(*mut c_void), arg: *mut c_void);
    fn kfree_link(arg: *mut c_void);
    fn d_alloc_name(parent: *mut dentry, name: *const c_char) -> *mut dentry;
    fn new_inode(sb: *mut super_block) -> *mut inode;
    fn simple_inode_init_ts(inode: *mut inode);
    fn d_make_persistent(dentry: *mut dentry, inode: *mut inode);
    fn dput(dentry: *mut dentry);
    fn pr_err(format: *const c_char, ...);
    fn proc_alloc_inum(ino: *mut c_uint);
}

const GFP_KERNEL: c_uint = 0;
const GFP_ATOMIC: c_uint = 0;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const ECHILD: c_int = 10;
const S_IFLNK: c_uint = 0o120000;
const S_IRWXUGO: c_uint = 0o777;
const GLOBAL_ROOT_UID: c_uint = 0;
const GLOBAL_ROOT_GID: c_uint = 0;

#[inline]
unsafe fn err_ptr(error: c_int) -> *const c_char {
    error as isize as *const c_char
}

unsafe extern "C" fn proc_self_get_link(
    dentry: *mut dentry,
    inode: *mut inode,
    done: *mut delayed_call,
) -> *const c_char {
    let ns = proc_pid_ns((*inode).i_sb);
    let tgid = task_tgid_nr_ns(current, ns);
    let mut name: *mut c_char;

    if tgid == 0 {
        return err_ptr(-ENOENT);
    }
    // max length of unsigned int in decimal + NULL term
    name = kmalloc(10 + 1, if !dentry.is_null() { GFP_KERNEL } else { GFP_ATOMIC }) as *mut c_char;
    if name.is_null() {
        return if !dentry.is_null() { err_ptr(-ENOMEM) } else { err_ptr(-ECHILD) };
    }
    sprintf(name, b"%u\0".as_ptr() as *const c_char, tgid as c_uint);
    set_delayed_call(done, kfree_link, name as *mut c_void);
    name as *const c_char
}

static proc_self_inode_operations: inode_operations = inode_operations {
    get_link: Some(proc_self_get_link),
};

#[no_mangle]
pub unsafe extern "C" fn proc_setup_self(s: *mut super_block) -> c_int {
    let mut self_dentry: *mut dentry;
    let mut ret: c_int = -ENOMEM;

    self_dentry = d_alloc_name((*s).s_root, b"self\0".as_ptr() as *const c_char);
    if !self_dentry.is_null() {
        let inode = new_inode(s);
        if !inode.is_null() {
            (*inode).i_ino = self_inum;
            simple_inode_init_ts(inode);
            (*inode).i_mode = S_IFLNK | S_IRWXUGO;
            (*inode).i_uid = GLOBAL_ROOT_UID;
            (*inode).i_gid = GLOBAL_ROOT_GID;
            (*inode).i_op = &proc_self_inode_operations;
            d_make_persistent(self_dentry, inode);
            ret = 0;
        }
        dput(self_dentry);
    }

    if ret != 0 {
        pr_err(b"proc_fill_super: can't allocate /proc/self\n\0".as_ptr() as *const c_char);
    }

    ret
}

pub unsafe extern "C" fn proc_self_init() {
    proc_alloc_inum(&mut self_inum);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
