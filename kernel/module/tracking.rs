// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Module taint unload tracking support
 *
 * Copyright (C) 2022 Aaron Tomlin
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// Declarations supplied by the kernel headers and by internal.h.
#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct module {
    pub taints: c_ulong,
    pub name: *const c_char,
}

#[repr(C)]
pub struct mod_unload_taint {
    pub list: list_head,
    pub name: [c_char; MODULE_NAME_LEN],
    pub taints: c_ulong,
    pub count: u64,
}

#[repr(C)]
pub struct dentry;
#[repr(C)]
pub struct seq_file;
#[repr(C)]
pub struct inode;
#[repr(C)]
pub struct file;

pub type loff_t = c_long;

pub const MODULE_NAME_LEN: usize = 56;
pub const MODULE_FLAGS_BUF_SIZE: usize = 64;

extern "C" {
    static mut module_mutex: c_void;
    static mut mod_debugfs_root: *mut dentry;

    fn lockdep_is_held(lock: *const c_void) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn kmalloc_obj<T>() -> *mut T;
    fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn module_flags_taint(taints: c_ulong, buf: *mut c_char) -> usize;
    fn printk(fmt: *const c_char, ...);
    fn pr_cont(fmt: *const c_char, ...);

    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn seq_list_start_rcu(head: *mut list_head, pos: loff_t) -> *mut c_void;
    fn seq_list_next_rcu(p: *mut c_void, head: *mut list_head, pos: *mut loff_t) -> *mut c_void;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn seq_open(file: *mut file, ops: *const seq_operations) -> c_int;
    fn seq_read(file: *mut file, buf: *mut c_void, count: usize, pos: *mut loff_t) -> isize;
    fn seq_lseek(file: *mut file, pos: loff_t, whence: c_int) -> loff_t;
    fn seq_release(inode: *mut inode, file: *mut file) -> c_int;
    fn debugfs_create_file(
        name: *const c_char,
        mode: c_ulong,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut c_void>,
    pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut loff_t) -> *mut c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct file_operations {
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_void, usize, *mut loff_t) -> isize>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
    pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
}

static mut unloaded_tainted_modules: list_head = list_head {
    next: core::ptr::null_mut(),
    prev: core::ptr::null_mut(),
};

pub unsafe extern "C" fn try_add_tainted_module(mod_: *mut module) -> c_int {
    let mut mod_taint: *mut mod_unload_taint;

    if (*mod_).taints == 0 {
        return 0;
    }

    // list_for_each_entry_rcu(mod_taint, &unloaded_tainted_modules, list,
    //                         lockdep_is_held(&module_mutex))
    mod_taint = core::ptr::null_mut();
    while !mod_taint.is_null() {
        if strcmp((*mod_taint).name.as_ptr(), (*mod_).name) == 0
            && ((*mod_taint).taints & (*mod_).taints) != 0
        {
            (*mod_taint).count += 1;
            return 0;
        }
        break;
    }

    mod_taint = kmalloc_obj::<mod_unload_taint>();
    if mod_taint.is_null() {
        return -12;
    }
    strscpy((*mod_taint).name.as_mut_ptr(), (*mod_).name, MODULE_NAME_LEN);
    (*mod_taint).taints = (*mod_).taints;
    list_add_rcu(&mut (*mod_taint).list, &mut unloaded_tainted_modules);
    (*mod_taint).count = 1;
    0
}

pub unsafe extern "C" fn print_unloaded_tainted_modules() {
    let mut buf = [0 as c_char; MODULE_FLAGS_BUF_SIZE];
    if !list_empty(&unloaded_tainted_modules) {
        printk(b"Unloaded tainted modules:\0".as_ptr() as *const c_char);
        // list_for_each_entry_rcu(mod_taint, &unloaded_tainted_modules, list)
        let mod_taint: *mut mod_unload_taint = core::ptr::null_mut();
        if !mod_taint.is_null() {
            let l = module_flags_taint((*mod_taint).taints, buf.as_mut_ptr());
            buf[l] = 0;
            pr_cont(b" %s(%s):%llu\0".as_ptr() as *const c_char,
                    (*mod_taint).name.as_ptr(), buf.as_ptr(), (*mod_taint).count);
        }
    }
}

// CONFIG_DEBUG_FS conditional preserved from the C source.
pub unsafe extern "C" fn unloaded_tainted_modules_seq_start(
    _m: *mut seq_file,
    pos: *mut loff_t,
) -> *mut c_void {
    rcu_read_lock();
    seq_list_start_rcu(&mut unloaded_tainted_modules, *pos)
}

pub unsafe extern "C" fn unloaded_tainted_modules_seq_next(
    _m: *mut seq_file,
    p: *mut c_void,
    pos: *mut loff_t,
) -> *mut c_void {
    seq_list_next_rcu(p, &mut unloaded_tainted_modules, pos)
}

pub unsafe extern "C" fn unloaded_tainted_modules_seq_stop(
    _m: *mut seq_file,
    _p: *mut c_void,
) {
    rcu_read_unlock();
}

pub unsafe extern "C" fn unloaded_tainted_modules_seq_show(
    m: *mut seq_file,
    p: *mut c_void,
) -> c_int {
    let mod_taint = p as *mut mod_unload_taint;
    let mut buf = [0 as c_char; MODULE_FLAGS_BUF_SIZE];
    let l = module_flags_taint((*mod_taint).taints, buf.as_mut_ptr());
    buf[l] = 0;
    seq_printf(
        m,
        b"%s (%s) %llu\0".as_ptr() as *const c_char,
        (*mod_taint).name.as_ptr(),
        buf.as_ptr(),
        (*mod_taint).count,
    );
    seq_puts(m, b"\n\0".as_ptr() as *const c_char);
    0
}

static unloaded_tainted_modules_seq_ops: seq_operations = seq_operations {
    start: Some(unloaded_tainted_modules_seq_start),
    next: Some(unloaded_tainted_modules_seq_next),
    stop: Some(unloaded_tainted_modules_seq_stop),
    show: Some(unloaded_tainted_modules_seq_show),
};

pub unsafe extern "C" fn unloaded_tainted_modules_open(
    _inode: *mut inode,
    file: *mut file,
) -> c_int {
    seq_open(file, &unloaded_tainted_modules_seq_ops)
}

static unloaded_tainted_modules_fops: file_operations = file_operations {
    open: Some(unloaded_tainted_modules_open),
    read: Some(seq_read),
    llseek: Some(seq_lseek),
    release: Some(seq_release),
};

pub unsafe extern "C" fn unloaded_tainted_modules_init() -> c_int {
    debugfs_create_file(
        b"unloaded_tainted\0".as_ptr() as *const c_char,
        0o444,
        mod_debugfs_root,
        core::ptr::null_mut(),
        &unloaded_tainted_modules_fops,
    );
    0
}

// module_init(unloaded_tainted_modules_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
