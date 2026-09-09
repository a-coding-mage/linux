// SPDX-License-Identifier: GPL-2.0-only
/*
 * Architecture specific debugfs files
 *
 * Copyright (C) 2007, Intel Corp.
 *	Huang Ying <ying.huang@intel.com>
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// External kernel types, constants, globals, and functions supplied by dependencies.
#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}
#[repr(C)]
pub struct setup_data {
    pub next: u64,
    pub type_: u32,
    pub len: u32,
    pub data: [u8; 0],
}
#[repr(C)]
pub struct setup_indirect {
    pub addr: u64,
    pub type_: u32,
    pub len: u32,
}
#[repr(C)]
pub struct file_operations {
    pub read: Option<unsafe extern "C" fn(*mut file, *mut c_char, usize, *mut i64) -> isize>,
    pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    pub llseek: Option<unsafe extern "C" fn(*mut file, i64, c_int) -> i64>,
}
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}
#[repr(C)]
pub struct debugfs_blob_wrapper {
    pub data: *mut c_void,
    pub size: usize,
}
#[repr(C)]
pub struct boot_params_header {
    pub setup_data: u64,
    pub version: u16,
}
#[repr(C)]
pub struct boot_params_type {
    pub hdr: boot_params_header,
}

extern "C" {
    static mut boot_params: boot_params_type;
    fn memremap(addr: u64, size: usize, flags: c_ulong) -> *mut c_void;
    fn memunmap(addr: *mut c_void);
    fn copy_to_user(to: *mut c_char, from: *const c_void, n: usize) -> usize;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_x32(name: *const c_char, mode: c_ulong, parent: *mut dentry, value: *mut u32) -> *mut dentry;
    fn debugfs_create_x16(name: *const c_char, mode: c_ulong, parent: *mut dentry, value: *mut u16) -> *mut dentry;
    fn debugfs_create_file(name: *const c_char, mode: c_ulong, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_create_blob(name: *const c_char, mode: c_ulong, parent: *mut dentry, blob: *mut debugfs_blob_wrapper) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn simple_open(inode: *mut inode, file: *mut file) -> c_int;
    fn default_llseek(file: *mut file, offset: i64, whence: c_int) -> i64;
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
    fn kmalloc(size: usize, flags: c_ulong) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

pub const SETUP_INDIRECT: u32 = 1;
pub const MEMREMAP_WB: c_ulong = 1;
pub const S_IRUGO: c_ulong = 0o444;
pub const EINVAL: isize = 22;
pub const ENOMEM: isize = 12;
pub const EFAULT: isize = 14;

#[no_mangle]
pub static mut arch_debugfs_dir: *mut dentry = core::ptr::null_mut();

#[cfg(feature = "CONFIG_DEBUG_BOOT_PARAMS")]
#[repr(C)]
pub struct setup_data_node {
    pub paddr: u64,
    pub type_: u32,
    pub len: u32,
}

#[cfg(feature = "CONFIG_DEBUG_BOOT_PARAMS")]
unsafe extern "C" fn setup_data_read(file: *mut file, user_buf: *mut c_char, count: usize, ppos: *mut i64) -> isize {
    let node = (*file).private_data as *mut setup_data_node;
    let mut remain: usize;
    let pos = *ppos;
    let mut pa: u64;
    let p: *mut c_void;

    if pos < 0 { return -EINVAL; }
    if pos >= (*node).len as i64 { return 0; }

    let mut count = count;
    if count > ((*node).len as i64 - pos) as usize { count = ((*node).len as i64 - pos) as usize; }
    pa = (*node).paddr.wrapping_add(pos as u64);

    /* Is it direct data or invalid indirect one? */
    if ((*node).type_ & SETUP_INDIRECT) == 0 || (*node).type_ == SETUP_INDIRECT {
        pa = pa.wrapping_add(core::mem::size_of::<setup_data>() as u64);
    }

    p = memremap(pa, count, MEMREMAP_WB);
    if p.is_null() { return -ENOMEM; }
    remain = copy_to_user(user_buf, p, count);
    memunmap(p);
    if remain != 0 { return -EFAULT; }
    *ppos = pos.wrapping_add(count as i64);
    count as isize
}

#[cfg(feature = "CONFIG_DEBUG_BOOT_PARAMS")]
static fops_setup_data: file_operations = file_operations {
    read: Some(setup_data_read),
    open: Some(simple_open),
    llseek: Some(default_llseek),
};

#[cfg(feature = "CONFIG_DEBUG_BOOT_PARAMS")]
unsafe fn create_setup_data_node(parent: *mut dentry, no: c_int, node: *mut setup_data_node) {
    let mut buf = [0 as c_char; 16];
    sprintf(buf.as_mut_ptr(), b"%d\0".as_ptr() as *const c_char, no);
    let d = debugfs_create_dir(buf.as_ptr(), parent);
    debugfs_create_x32(b"type\0".as_ptr() as *const c_char, S_IRUGO, d, &mut (*node).type_);
    debugfs_create_file(b"data\0".as_ptr() as *const c_char, S_IRUGO, d, node as *mut c_void, &fops_setup_data);
}

#[cfg(feature = "CONFIG_DEBUG_BOOT_PARAMS")]
unsafe fn create_setup_data_nodes(parent: *mut dentry) -> c_int {
    let d = debugfs_create_dir(b"setup_data\0".as_ptr() as *const c_char, parent);
    let mut pa_data = boot_params.hdr.setup_data;
    let mut no: c_int = 0;

    while pa_data != 0 {
        let node = kmalloc(core::mem::size_of::<setup_data_node>(), 0) as *mut setup_data_node;
        if node.is_null() { debugfs_remove_recursive(d); return -ENOMEM as c_int; }
        let mut data = memremap(pa_data, core::mem::size_of::<setup_data>(), MEMREMAP_WB) as *mut setup_data;
        if data.is_null() { kfree(node as *mut c_void); debugfs_remove_recursive(d); return -ENOMEM as c_int; }
        let pa_next = (*data).next;
        if (*data).type_ == SETUP_INDIRECT {
            let len = core::mem::size_of::<setup_data>() + (*data).len as usize;
            memunmap(data as *mut c_void);
            data = memremap(pa_data, len, MEMREMAP_WB) as *mut setup_data;
            if data.is_null() { kfree(node as *mut c_void); debugfs_remove_recursive(d); return -ENOMEM as c_int; }
            let indirect = (*data).data.as_ptr() as *const setup_indirect;
            if (*indirect).type_ != SETUP_INDIRECT {
                (*node).paddr = (*indirect).addr; (*node).type_ = (*indirect).type_; (*node).len = (*indirect).len;
            } else {
                (*node).paddr = pa_data; (*node).type_ = (*data).type_; (*node).len = (*data).len;
            }
        } else {
            (*node).paddr = pa_data; (*node).type_ = (*data).type_; (*node).len = (*data).len;
        }
        create_setup_data_node(d, no, node);
        pa_data = pa_next;
        memunmap(data as *mut c_void);
        no += 1;
    }
    0
}

#[cfg(feature = "CONFIG_DEBUG_BOOT_PARAMS")]
static mut boot_params_blob: debugfs_blob_wrapper = debugfs_blob_wrapper { data: core::ptr::null_mut(), size: 0 };

#[cfg(feature = "CONFIG_DEBUG_BOOT_PARAMS")]
unsafe fn boot_params_kdebugfs_init() -> c_int {
    let dbp = debugfs_create_dir(b"boot_params\0".as_ptr() as *const c_char, arch_debugfs_dir);
    debugfs_create_x16(b"version\0".as_ptr() as *const c_char, S_IRUGO, dbp, &mut boot_params.hdr.version);
    boot_params_blob = debugfs_blob_wrapper { data: &mut boot_params as *mut _ as *mut c_void, size: core::mem::size_of_val(&boot_params) };
    debugfs_create_blob(b"data\0".as_ptr() as *const c_char, S_IRUGO, dbp, &mut boot_params_blob);
    let error = create_setup_data_nodes(dbp);
    if error != 0 { debugfs_remove_recursive(dbp); }
    error
}

unsafe fn arch_kdebugfs_init() -> c_int {
    arch_debugfs_dir = debugfs_create_dir(b"x86\0".as_ptr() as *const c_char, core::ptr::null_mut());
    #[cfg(feature = "CONFIG_DEBUG_BOOT_PARAMS")]
    { return boot_params_kdebugfs_init(); }
    #[cfg(not(feature = "CONFIG_DEBUG_BOOT_PARAMS"))]
    { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
