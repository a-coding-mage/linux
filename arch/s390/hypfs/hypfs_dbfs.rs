// SPDX-License-Identifier: GPL-2.0
/*
 * Hypervisor filesystem for Linux on s390 - debugfs interface
 *
 * Copyright IBM Corp. 2010
 * Author(s): Michael Holzheu <holzheu@linux.vnet.ibm.com>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// Types and operations supplied by the surrounding kernel/hypfs sources.
#[repr(C)]
pub struct dentry;
#[repr(C)]
pub struct inode { pub i_private: *mut c_void }
#[repr(C)]
pub struct file;
#[repr(C)]
pub struct mutex;
#[repr(C)]
pub struct hypfs_dbfs_file {
    pub name: *const c_char,
    pub data_create: Option<unsafe extern "C" fn(*mut *mut c_void, *mut *mut c_void, *mut usize) -> isize>,
    pub data_free: Option<unsafe extern "C" fn(*mut c_void)>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
    pub dentry: *mut dentry,
    pub lock: mutex,
}
#[repr(C)]
pub struct hypfs_dbfs_data {
    pub dbfs_file: *mut hypfs_dbfs_file,
    pub buf: *mut c_void,
    pub buf_free_ptr: *mut c_void,
    pub size: usize,
}

extern "C" {
    static mut dbfs_dir: *mut dentry;
    fn kmalloc_obj<T>() -> *mut T;
    fn kfree(ptr: *mut c_void);
    fn mutex_lock_interruptible(lock: *mut mutex) -> c_int;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn file_inode(file: *mut file) -> *mut inode;
    fn simple_read_from_buffer(buf: *mut c_void, size: usize, ppos: *mut i64,
                               from: *const c_void, available: usize) -> isize;
    fn security_locked_down(what: c_int) -> c_int;
    fn debugfs_create_file(name: *const c_char, mode: c_uint, parent: *mut dentry,
                           data: *mut hypfs_dbfs_file, fops: *const file_operations) -> *mut dentry;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_remove(dentry: *mut dentry);
    fn hypfs_diag_init() -> c_int;
    fn hypfs_vm_init() -> c_int;
    fn hypfs_sprp_init();
    fn hypfs_diag0c_init() -> c_int;
    fn hypfs_fs_init() -> c_int;
    fn hypfs_diag0c_exit();
    fn hypfs_sprp_exit();
    fn hypfs_vm_exit();
    fn hypfs_diag_exit();
    fn pr_err(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct file_operations {
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_void, usize, *mut i64) -> isize>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut file, c_uint, c_ulong) -> c_long>,
}

const ERESTARTSYS: isize = 512;
const ENOMEM: isize = 12;
const ENODATA: c_int = 61;
const LOCKDOWN_DEBUGFS: c_int = 0;

static mut DBFS_DIR: *mut dentry = core::ptr::null_mut();

unsafe fn hypfs_dbfs_data_alloc(f: *mut hypfs_dbfs_file) -> *mut hypfs_dbfs_data {
    let data = kmalloc_obj::<hypfs_dbfs_data>();
    if data.is_null() { return core::ptr::null_mut(); }
    (*data).dbfs_file = f;
    data
}

unsafe fn hypfs_dbfs_data_free(data: *mut hypfs_dbfs_data) {
    if let Some(free) = (*(*data).dbfs_file).data_free { free((*data).buf_free_ptr); }
    kfree(data.cast());
}

unsafe extern "C" fn dbfs_read(file: *mut file, buf: *mut c_void, size: usize, ppos: *mut i64) -> isize {
    if *ppos != 0 { return 0; }
    let df = (*file_inode(file)).i_private.cast::<hypfs_dbfs_file>();
    if mutex_lock_interruptible(&mut (*df).lock) != 0 { return -ERESTARTSYS; }
    let data = hypfs_dbfs_data_alloc(df);
    if data.is_null() { mutex_unlock(&mut (*df).lock); return -ENOMEM; }
    let rc = ((*df).data_create.unwrap())(&mut (*data).buf, &mut (*data).buf_free_ptr, &mut (*data).size);
    if rc != 0 { mutex_unlock(&mut (*df).lock); kfree(data.cast()); return rc; }
    mutex_unlock(&mut (*df).lock);
    let rc = simple_read_from_buffer(buf, size, ppos, (*data).buf, (*data).size);
    hypfs_dbfs_data_free(data);
    rc
}

unsafe extern "C" fn dbfs_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let df = (*file_inode(file)).i_private.cast::<hypfs_dbfs_file>();
    mutex_lock(&mut (*df).lock);
    let rc = ((*df).unlocked_ioctl.unwrap())(file, cmd, arg);
    mutex_unlock(&mut (*df).lock);
    rc
}

static DBFS_OPS_IOCTL: file_operations = file_operations { read: Some(dbfs_read), unlocked_ioctl: Some(dbfs_ioctl) };
static DBFS_OPS: file_operations = file_operations { read: Some(dbfs_read), unlocked_ioctl: None };

pub unsafe extern "C" fn hypfs_dbfs_create_file(df: *mut hypfs_dbfs_file) {
    let mut fops = &DBFS_OPS;
    if (*df).unlocked_ioctl.is_some() && security_locked_down(LOCKDOWN_DEBUGFS) == 0 { fops = &DBFS_OPS_IOCTL; }
    (*df).dentry = debugfs_create_file((*df).name, 0o400, DBFS_DIR, df, fops);
    mutex_init(&mut (*df).lock);
}

pub unsafe extern "C" fn hypfs_dbfs_remove_file(df: *mut hypfs_dbfs_file) { debugfs_remove((*df).dentry); }

unsafe extern "C" fn hypfs_dbfs_init() -> c_int {
    let mut rc: c_int = -ENODATA;
    DBFS_DIR = debugfs_create_dir(c"s390_hypfs".as_ptr(), core::ptr::null_mut());
    if hypfs_diag_init() != 0 { debugfs_remove(DBFS_DIR); return rc; }
    if hypfs_vm_init() != 0 { hypfs_diag_exit(); debugfs_remove(DBFS_DIR); return rc; }
    hypfs_sprp_init();
    if hypfs_diag0c_init() != 0 { hypfs_sprp_exit(); hypfs_vm_exit(); hypfs_diag_exit(); debugfs_remove(DBFS_DIR); return rc; }
    rc = hypfs_fs_init();
    if rc != 0 { hypfs_diag0c_exit(); hypfs_sprp_exit(); hypfs_vm_exit(); hypfs_diag_exit(); pr_err(c"Initialization of hypfs failed with rc=%i\n".as_ptr()); debugfs_remove(DBFS_DIR); return rc; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
