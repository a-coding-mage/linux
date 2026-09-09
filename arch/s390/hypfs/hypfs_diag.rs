// SPDX-License-Identifier: GPL-2.0
/*
 *    Hypervisor filesystem for Linux on s390. Diag 204 and 224
 *    implementation.
 *
 *    Copyright IBM Corp. 2006, 2008
 *    Author(s): Michael Holzheu <holzheu@de.ibm.com>
 */

// C dependencies supplied by the surrounding kernel translation unit.

const DBFS_D204_HDR_VERSION: u16 = 0;

static mut diag204_store_sc: diag204_sc = DIAG204_SUBC_STIB4;
static mut diag204_info_type: diag204_format = DIAG204_INFO_SIMPLE;
static mut diag204_buf: *mut core::ffi::c_void = core::ptr::null_mut();
static mut diag204_buf_pages: i32 = 0;

extern "C" {
    static current: *mut core::ffi::c_void;
    fn diag204(subcode: usize, pages: i32, buf: *mut core::ffi::c_void) -> i32;
    fn diag204_has_bif() -> bool;
    fn signal_pending(task: *mut core::ffi::c_void) -> bool;
    fn schedule_timeout_interruptible(timeout: i64);
    fn __vmalloc_node(size: usize, align: usize, gfp: usize, node: i32, caller: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn vfree(ptr: *mut core::ffi::c_void);
    fn vzalloc(size: usize) -> *mut core::ffi::c_void;
    fn hypfs_dbfs_create_file(file: *mut hypfs_dbfs_file);
    fn hypfs_dbfs_remove_file(file: *mut hypfs_dbfs_file);
    fn hypfs_diag_fs_init() -> i32;
    fn hypfs_diag_fs_exit();
    fn pr_info(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
}

// These types, constants, and kernel helpers are declared by the included headers.
// Their definitions are intentionally left to the surrounding translation unit.
type diag204_sc = u64;
type diag204_format = u64;
const DIAG204_SUBC_STIB4: diag204_sc = 4;
const DIAG204_SUBC_STIB6: diag204_sc = 6;
const DIAG204_SUBC_STIB7: diag204_sc = 7;
const DIAG204_SUBC_RSI: diag204_sc = 5;
const DIAG204_INFO_SIMPLE: diag204_format = 0;
const DIAG204_INFO_EXT: diag204_format = 1;
const DIAG204_BIF_BIT: usize = 0x8000_0000;
const DIAG204_BUSY_WAIT: i64 = 1;
const PAGE_SIZE: usize = 4096;
const GFP_KERNEL: usize = 0;
const NUMA_NO_NODE: i32 = -1;
const EOPNOTSUPP: i32 = 95;
const ENOMEM: i32 = 12;
const ENODATA: i32 = 61;
const EBUSY: i32 = 16;
const ERESTARTSYS: i32 = 512;

#[repr(C)]
pub struct hypfs_dbfs_file {
    pub name: *const u8,
    pub data_create: Option<unsafe extern "C" fn(*mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut usize) -> i32>,
    pub data_free: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
}

pub unsafe fn diag204_get_info_type() -> diag204_format {
    diag204_info_type
}

unsafe fn diag204_set_info_type(typ: diag204_format) {
    diag204_info_type = typ;
}

unsafe fn diag204_free_buffer() {
    vfree(diag204_buf);
    diag204_buf = core::ptr::null_mut();
}

pub unsafe fn diag204_get_buffer(fmt: diag204_format, pages: *mut i32) -> *mut core::ffi::c_void {
    if !diag204_buf.is_null() {
        *pages = diag204_buf_pages;
        return diag204_buf;
    }
    if fmt == DIAG204_INFO_SIMPLE {
        *pages = 1;
    } else {
        *pages = diag204((DIAG204_SUBC_RSI | DIAG204_INFO_EXT) as usize, 0, core::ptr::null_mut());
        if *pages <= 0 { return (-EOPNOTSUPP) as isize as *mut core::ffi::c_void; }
    }
    diag204_buf = __vmalloc_node((*pages as usize) * PAGE_SIZE, PAGE_SIZE, GFP_KERNEL, NUMA_NO_NODE, core::ptr::null_mut());
    if diag204_buf.is_null() { return (-ENOMEM) as isize as *mut core::ffi::c_void; }
    diag204_buf_pages = *pages;
    diag204_buf
}

unsafe fn diag204_probe() -> i32 {
    let mut pages = 0i32;
    let mut rc: i32;
    let mut buf = diag204_get_buffer(DIAG204_INFO_EXT, &mut pages);
    if (buf as isize) >= 0 {
        if diag204((DIAG204_SUBC_STIB7 | DIAG204_INFO_EXT) as usize, pages, buf) >= 0 {
            diag204_store_sc = DIAG204_SUBC_STIB7; diag204_set_info_type(DIAG204_INFO_EXT); diag204_free_buffer(); return 0;
        }
        if diag204((DIAG204_SUBC_STIB6 | DIAG204_INFO_EXT) as usize, pages, buf) >= 0 {
            diag204_store_sc = DIAG204_SUBC_STIB6; diag204_set_info_type(DIAG204_INFO_EXT); diag204_free_buffer(); return 0;
        }
        diag204_free_buffer();
    }
    buf = diag204_get_buffer(DIAG204_INFO_SIMPLE, &mut pages);
    if (buf as isize) < 0 { rc = buf as isize as i32; return rc; }
    if diag204((DIAG204_SUBC_STIB4 | DIAG204_INFO_SIMPLE) as usize, pages, buf) >= 0 {
        diag204_store_sc = DIAG204_SUBC_STIB4; diag204_set_info_type(DIAG204_INFO_SIMPLE); rc = 0;
    } else { rc = -EOPNOTSUPP; }
    diag204_free_buffer();
    rc
}

pub unsafe fn diag204_store(buf: *mut core::ffi::c_void, pages: i32) -> i32 {
    let mut subcode = diag204_get_info_type() | diag204_store_sc;
    if diag204_has_bif() { subcode |= DIAG204_BIF_BIT as u64; }
    loop {
        let rc = diag204(subcode as usize, pages, buf);
        if rc != -EBUSY { return if rc < 0 { rc } else { 0 }; }
        if signal_pending(current) { return -ERESTARTSYS; }
        schedule_timeout_interruptible(DIAG204_BUSY_WAIT);
    }
}

#[repr(C, packed)]
pub struct dbfs_d204_hdr { pub len: u64, pub version: u16, pub sc: u8, pub reserved: [u8; 53] }

#[repr(C, packed)]
pub struct dbfs_d204 { pub hdr: dbfs_d204_hdr, pub buf: [u8; 0] }

unsafe extern "C" fn dbfs_d204_create(data: *mut *mut core::ffi::c_void, data_free_ptr: *mut *mut core::ffi::c_void, size: *mut usize) -> i32 {
    let buf_size = PAGE_SIZE * (diag204_buf_pages as usize + 1) + core::mem::size_of::<dbfs_d204_hdr>();
    let base = vzalloc(buf_size);
    if base.is_null() { return -ENOMEM; }
    let d204 = (base as usize + core::mem::size_of::<dbfs_d204_hdr>() + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    let d204 = (d204 - core::mem::size_of::<dbfs_d204_hdr>()) as *mut dbfs_d204;
    let rc = diag204_store((d204 as *mut u8).add(core::mem::size_of::<dbfs_d204_hdr>()) as *mut core::ffi::c_void, diag204_buf_pages);
    if rc != 0 { vfree(base); return rc; }
    (*d204).hdr.version = DBFS_D204_HDR_VERSION;
    (*d204).hdr.len = PAGE_SIZE as u64 * diag204_buf_pages as u64;
    (*d204).hdr.sc = diag204_store_sc as u8;
    *data = d204 as *mut core::ffi::c_void; *data_free_ptr = base; *size = (*d204).hdr.len as usize + core::mem::size_of::<dbfs_d204_hdr>(); 0
}

static mut dbfs_file_d204: hypfs_dbfs_file = hypfs_dbfs_file { name: b"diag_204\0".as_ptr(), data_create: Some(dbfs_d204_create), data_free: Some(vfree) };

pub unsafe fn hypfs_diag_init() -> i32 {
    if diag204_probe() != 0 { pr_info(b"The hardware system does not support hypfs\0".as_ptr()); return -ENODATA; }
    if diag204_get_info_type() == DIAG204_INFO_EXT { hypfs_dbfs_create_file(&mut dbfs_file_d204); }
    let rc = hypfs_diag_fs_init();
    if rc != 0 { pr_err(b"The hardware system does not provide all functions required by hypfs\0".as_ptr()); }
    rc
}

pub unsafe fn hypfs_diag_exit() {
    hypfs_diag_fs_exit(); diag204_free_buffer(); hypfs_dbfs_remove_file(&mut dbfs_file_d204);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
