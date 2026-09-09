// SPDX-License-Identifier: GPL-2.0
/*
 * Procfs support for lockd
 *
 * Copyright (c) 2014 Jeff Layton <jlayton@primarydata.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

use core::ffi::{c_char, c_int, c_uint, c_void};

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct proc_dir_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lockd_net {
    pub lockd_manager: lock_manager,
}

#[repr(C)]
pub struct lock_manager {
    pub list: list_head,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct proc_ops {
    pub proc_write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    pub proc_read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    pub proc_lseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_uint) -> loff_t>,
    pub proc_release: Option<unsafe extern "C" fn(*mut file) -> c_int>,
}

extern "C" {
    static mut current: *mut task_struct;
    static lockd_net_id: c_int;

    fn net_generic(net: *mut net, id: c_int) -> *mut c_void;
    fn simple_transaction_get(file: *mut file, buf: *const c_char, size: size_t) -> *mut c_char;
    fn ptr_err(ptr: *const c_void) -> isize;
    fn locks_end_grace(manager: *mut lock_manager);
    fn list_empty(head: *const list_head) -> bool;
    fn simple_read_from_buffer(
        buf: *mut c_char,
        size: size_t,
        pos: *mut loff_t,
        from: *const c_void,
        count: size_t,
    ) -> ssize_t;
    fn default_llseek(file: *mut file, offset: loff_t, whence: c_uint) -> loff_t;
    fn simple_transaction_release(file: *mut file) -> c_int;
    fn proc_mkdir(name: *const c_char, parent: *mut proc_dir_entry) -> *mut proc_dir_entry;
    fn proc_create(
        name: *const c_char,
        mode: c_uint,
        parent: *mut proc_dir_entry,
        ops: *const proc_ops,
    ) -> *mut proc_dir_entry;
    fn remove_proc_entry(name: *const c_char, parent: *mut proc_dir_entry);
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsproxy {
    pub net_ns: *mut net,
}

#[repr(C)]
pub struct task_struct {
    pub nsproxy: *mut nsproxy,
}

const EINVAL: ssize_t = 22;
const ENOMEM: c_int = 12;
const S_IRUGO: c_uint = 0o444;
const S_IWUSR: c_uint = 0o200;

/*
 * We only allow strings that start with 'Y', 'y', or '1'.
 */
unsafe extern "C" fn nlm_end_grace_write(
    file: *mut file,
    buf: *const c_char,
    size: size_t,
    _pos: *mut loff_t,
) -> ssize_t {
    let ln = net_generic((*(*current).nsproxy).net_ns, lockd_net_id) as *mut lockd_net;

    if size < 1 {
        return -EINVAL;
    }

    let data = simple_transaction_get(file, buf, size);
    if data.is_null() {
        // IS_ERR(data) and PTR_ERR(data); the kernel error-pointer predicate
        // is supplied by the surrounding kernel translation.
        return ptr_err(data as *const c_void);
    }

    match *data as u8 {
        b'Y' | b'y' | b'1' => {
            locks_end_grace(&mut (*ln).lockd_manager);
        }
        _ => return -EINVAL,
    }

    size as ssize_t
}

unsafe extern "C" fn nlm_end_grace_read(
    _file: *mut file,
    buf: *mut c_char,
    size: size_t,
    pos: *mut loff_t,
) -> ssize_t {
    let ln = net_generic((*(*current).nsproxy).net_ns, lockd_net_id) as *mut lockd_net;
    let mut resp = [0u8; 3];

    resp[0] = if list_empty(&(*ln).lockd_manager.list) { b'Y' } else { b'N' };
    resp[1] = b'\n';
    resp[2] = b'\0';

    simple_read_from_buffer(buf, size, pos, resp.as_ptr() as *const c_void, core::mem::size_of_val(&resp))
}

static lockd_end_grace_proc_ops: proc_ops = proc_ops {
    proc_write: Some(nlm_end_grace_write),
    proc_read: Some(nlm_end_grace_read),
    proc_lseek: Some(default_llseek),
    proc_release: Some(simple_transaction_release),
};

pub unsafe extern "C" fn lockd_create_procfs() -> c_int {
    let mut entry = proc_mkdir(b"fs/lockd\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if entry.is_null() {
        return -ENOMEM;
    }
    entry = proc_create(
        b"nlm_end_grace\0".as_ptr() as *const c_char,
        S_IRUGO | S_IWUSR,
        entry,
        &lockd_end_grace_proc_ops,
    );
    if entry.is_null() {
        remove_proc_entry(b"fs/lockd\0".as_ptr() as *const c_char, core::ptr::null_mut());
        return -ENOMEM;
    }
    0
}

pub unsafe extern "C" fn lockd_remove_procfs() {
    remove_proc_entry(b"fs/lockd/nlm_end_grace\0".as_ptr() as *const c_char, core::ptr::null_mut());
    remove_proc_entry(b"fs/lockd\0".as_ptr() as *const c_char, core::ptr::null_mut());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
