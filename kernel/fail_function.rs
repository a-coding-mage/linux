// SPDX-License-Identifier: GPL-2.0
/*
 * fail_function.c: Function-based error injection
 */

// Kernel headers and build-time macros are supplied by the surrounding kernel
// environment and are intentionally not reimplemented in this translation.

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct fei_attr {
    pub list: list_head,
    pub kp: kprobe,
    pub retval: c_ulong,
}

extern "C" {
    static mut fei_lock: mutex;
    static mut fei_attr_list: list_head;
    static mut fei_fault_attr: fault_attr;
    static mut fei_debugfs_dir: *mut dentry;

    fn get_injectable_error_type(addr: c_ulong) -> c_int;
    fn kzalloc_obj<T>(obj: T) -> *mut T;
    fn kstrdup(s: *const c_char, flags: c_ulong) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(
        name: *const c_char,
        mode: c_ulong,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_lookup_and_remove(name: *const c_char, parent: *mut dentry);
    fn should_fail(attr: *mut fault_attr, probability: c_ulong) -> bool;
    fn regs_set_return_value(regs: *mut pt_regs, value: c_ulong);
    fn override_function_with_return(regs: *mut pt_regs);
    fn seq_list_start(list: *mut list_head, pos: loff_t) -> *mut c_void;
    fn seq_list_next(v: *mut c_void, list: *mut list_head, pos: *mut loff_t) -> *mut c_void;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn seq_open(file: *mut file, ops: *const seq_operations) -> c_int;
    fn unregister_kprobe(kp: *mut kprobe) -> c_int;
    fn list_del(entry: *mut list_head);
    fn memdup_user_nul(buffer: *const c_char, count: usize) -> *mut c_char;
    fn ptr_err(ptr: *mut c_char) -> isize;
    fn strstrip(s: *mut c_char) -> *mut c_char;
    fn kallsyms_lookup_name(name: *const c_char) -> c_ulong;
    fn within_error_injection_list(addr: c_ulong) -> bool;
    fn register_kprobe(kp: *mut kprobe) -> c_int;
    fn fault_create_debugfs_attr(name: *const c_char, parent: *mut dentry,
                                 attr: *mut fault_attr) -> *mut dentry;
    fn debugfs_create_symlink(name: *const c_char, parent: *mut dentry,
                              target: *const c_char) -> *mut dentry;
    fn seq_read(file: *mut file, buf: *mut c_char, count: usize, pos: *mut loff_t) -> isize;
    fn seq_lseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn seq_release(inode: *mut inode, file: *mut file) -> c_int;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kprobe { pub symbol_name: *mut c_char, pub pre_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> c_int>, pub post_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs, c_ulong)>, pub addr: *mut c_void }
#[repr(C)] pub struct pt_regs;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct fault_attr;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct inode;
#[repr(C)] pub struct file;
#[repr(C)] pub struct file_operations { pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>, pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, usize, *mut loff_t) -> isize>, pub write: Option<unsafe extern "C" fn(*mut file, *const c_char, usize, *mut loff_t) -> isize>, pub llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>, pub release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int> }
#[repr(C)] pub struct seq_operations { pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut c_void>, pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut loff_t) -> *mut c_void>, pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>, pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int> }
pub type loff_t = i64;

const EI_ETYPE_NULL: c_int = 0;
const EI_ETYPE_ERRNO: c_int = 1;
const EI_ETYPE_ERRNO_NULL: c_int = 2;
const EI_ETYPE_TRUE: c_int = 3;
const MAX_ERRNO: c_ulong = 4095;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ERANGE: c_int = 34;
const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const KSYM_NAME_LEN: usize = 128;
const GFP_KERNEL: c_ulong = 0;

unsafe extern "C" fn fei_post_handler(_kp: *mut kprobe, _regs: *mut pt_regs, _flags: c_ulong) {
    /* A dummy post handler is required to prohibit optimizing, because
     * jump optimization does not support execution path overriding. */
}

unsafe fn adjust_error_retval(addr: c_ulong, retv: c_ulong) -> c_ulong {
    match get_injectable_error_type(addr) {
        EI_ETYPE_NULL => 0,
        EI_ETYPE_ERRNO => if retv < (0usize.wrapping_sub(MAX_ERRNO as usize) as c_ulong) { (0usize.wrapping_sub(EINVAL as usize)) as c_ulong } else { retv },
        EI_ETYPE_ERRNO_NULL => if retv != 0 && retv < (0usize.wrapping_sub(MAX_ERRNO as usize) as c_ulong) { (0usize.wrapping_sub(EINVAL as usize)) as c_ulong } else { retv },
        EI_ETYPE_TRUE => 1,
        _ => retv,
    }
}

unsafe fn fei_attr_free(attr: *mut fei_attr) {
    if !attr.is_null() { kfree((*attr).kp.symbol_name.cast()); kfree(attr.cast()); }
}

unsafe extern "C" fn fei_kprobe_handler(kp: *mut kprobe, regs: *mut pt_regs) -> c_int {
    let attr = (kp as *mut u8).sub(core::mem::offset_of!(fei_attr, kp)) as *mut fei_attr;
    if should_fail(&mut fei_fault_attr, 1) { regs_set_return_value(regs, (*attr).retval); override_function_with_return(regs); return 1; }
    0
}

unsafe fn fei_attr_new(sym: *const c_char, addr: c_ulong) -> *mut fei_attr {
    let attr = kzalloc_obj(fei_attr { list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, kp: kprobe { symbol_name: core::ptr::null_mut(), pre_handler: Some(fei_kprobe_handler), post_handler: Some(fei_post_handler), addr: core::ptr::null_mut() }, retval: 0 });
    if attr.is_null() { return core::ptr::null_mut(); }
    (*attr).kp.symbol_name = kstrdup(sym, GFP_KERNEL);
    if (*attr).kp.symbol_name.is_null() { kfree(attr.cast()); return core::ptr::null_mut(); }
    (*attr).retval = adjust_error_retval(addr, 0);
    attr
}

unsafe fn fei_attr_remove(attr: *mut fei_attr) {
    fei_debugfs_remove_attr(attr); unregister_kprobe(&mut (*attr).kp); list_del(&mut (*attr).list); fei_attr_free(attr);
}
unsafe fn fei_attr_remove_all() {
    let mut attr = (*fei_attr_list).next;
    while attr != &mut fei_attr_list as *mut _ { let next = (*attr).next; fei_attr_remove((attr as *mut u8).sub(core::mem::offset_of!(fei_attr, list)) as *mut fei_attr); attr = next; }
}
unsafe fn fei_debugfs_remove_attr(attr: *mut fei_attr) { debugfs_lookup_and_remove((*attr).kp.symbol_name, fei_debugfs_dir); }

unsafe extern "C" fn fei_write(_file: *mut file, buffer: *const c_char, mut count: usize, _ppos: *mut loff_t) -> isize {
    if count > KSYM_NAME_LEN { count = KSYM_NAME_LEN; }
    let buf = memdup_user_nul(buffer, count); if buf.is_null() { return ptr_err(buf); }
    let sym = strstrip(buf); mutex_lock(&mut fei_lock);
    let ret = if *sym == 0 { fei_attr_remove_all(); count as isize } else if *sym == b'!' as c_char {
        -ENOENT as isize
    } else {
        let addr = kallsyms_lookup_name(sym);
        if addr == 0 { -EINVAL as isize } else if !within_error_injection_list(addr) { -ERANGE as isize } else {
            let attr = fei_attr_new(sym, addr); if attr.is_null() { -ENOMEM as isize } else { let r = register_kprobe(&mut (*attr).kp); if r != 0 { fei_attr_free(attr); r as isize } else { count as isize } }
        }
    };
    mutex_unlock(&mut fei_lock); kfree(buf.cast()); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
