// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) IBM Corporation, 2024
 */

// #define pr_fmt(fmt) "htmdump: " fmt

use core::ffi::c_void;

#[repr(C)]
pub struct file {
    pub private_data: *mut c_void,
}
#[repr(C)]
pub struct dentry;
#[repr(C)]
pub struct file_operations {
    pub llseek: Option<unsafe extern "C" fn()>,
    pub read: Option<unsafe extern "C" fn(*mut file, *mut i8, usize, *mut i64) -> isize>,
    pub open: Option<unsafe extern "C" fn()>,
}

extern "C" {
    static mut arch_debugfs_dir: *mut dentry;
    fn htm_hcall_wrapper(a: u64, b: u32, c: u32, d: u32, e: u32, f: u64, g: u64, h: u64, i: u64) -> i64;
    fn virt_to_phys(p: *mut c_void) -> u64;
    fn simple_read_from_buffer(ubuf: *mut i8, count: usize, ppos: *mut i64, from: *mut c_void, available: u64) -> isize;
    fn simple_open();
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn debugfs_create_dir(name: *const i8, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_u32(name: *const i8, mode: u32, parent: *mut dentry, value: *mut u32) -> *mut dentry;
    fn debugfs_create_file(name: *const i8, mode: u32, parent: *mut dentry, data: *mut c_void, fops: *const file_operations) -> *mut dentry;
    fn debugfs_remove_recursive(dir: *mut dentry);
    fn is_kvm_guest() -> bool;
}

const PAGE_SIZE: u64 = 4096;
const GFP_KERNEL: u32 = 0;
const H_SUCCESS: i64 = 0;
const H_PARTIAL: i64 = 1;
const H_NOT_AVAILABLE: i64 = 2;
const H_BUSY: i64 = -3;
const H_LONG_BUSY_ORDER_1_MSEC: i64 = -4;
const H_LONG_BUSY_ORDER_10_MSEC: i64 = -5;
const H_LONG_BUSY_ORDER_100_MSEC: i64 = -6;
const H_LONG_BUSY_ORDER_1_SEC: i64 = -7;
const H_LONG_BUSY_ORDER_10_SEC: i64 = -8;
const H_LONG_BUSY_ORDER_100_SEC: i64 = -9;
const H_PARAMETER: i64 = -10;
const H_P2: i64 = -11;
const H_P3: i64 = -12;
const H_P4: i64 = -13;
const H_P5: i64 = -14;
const H_P6: i64 = -15;
const H_STATE: i64 = -16;
const H_AUTHORITY: i64 = -17;
const H_HTM_OP_DUMP_DATA: u64 = 0;
const H_HTM_OP_DUMP_SYSMEM_CONF: u64 = 1;
const H_HTM_OP_CONFIGURE: u64 = 2;
const H_HTM_OP_DECONFIGURE: u64 = 3;
const H_HTM_OP_START: u64 = 4;
const H_HTM_OP_STOP: u64 = 5;
const H_HTM_OP_STATUS: u64 = 6;
const H_HTM_OP_DUMP_SYSPROC_CONF: u64 = 7;
const H_HTM_OP_CAPABILITIES: u64 = 8;
const H_HTM_OP_SETUP: u64 = 9;
const H_HTM_FLAGS_NOWRAP: u64 = 1;
const ENOMEM: i64 = 12;
const EINVAL: i64 = 22;
const EBUSY: i64 = 16;
const EIO: i64 = 5;
const EPERM: i64 = 1;
const EOPNOTSUPP: i64 = 95;

static mut htm_buf: *mut c_void = core::ptr::null_mut();
static mut htm_status_buf: *mut c_void = core::ptr::null_mut();
static mut htm_info_buf: *mut c_void = core::ptr::null_mut();
static mut htm_caps_buf: *mut c_void = core::ptr::null_mut();
static mut htm_mem_buf: *mut c_void = core::ptr::null_mut();
static mut nodeindex: u32 = 0;
static mut nodalchipindex: u32 = 0;
static mut coreindexonchip: u32 = 0;
static mut htmtype: u32 = 0;
static mut htmconfigure: u32 = 0;
static mut htmstart: u32 = 0;
static mut htmsetup: u32 = 0;
static mut htmflags: u64 = 0;
static mut htmdump_debugfs_dir: *mut dentry = core::ptr::null_mut();

const HTM_ENABLE: u64 = 1;
const HTM_DISABLE: u64 = 0;
const HTM_NOWRAP: u64 = 1;
const HTM_WRAP: u64 = 0;

unsafe fn htm_return_check(rc: i64) -> isize {
    match rc {
        H_SUCCESS | H_PARTIAL => {}
        H_NOT_AVAILABLE => return 0,
        H_BUSY | H_LONG_BUSY_ORDER_1_MSEC | H_LONG_BUSY_ORDER_10_MSEC |
        H_LONG_BUSY_ORDER_100_MSEC | H_LONG_BUSY_ORDER_1_SEC |
        H_LONG_BUSY_ORDER_10_SEC | H_LONG_BUSY_ORDER_100_SEC => return -EBUSY as isize,
        H_PARAMETER | H_P2 | H_P3 | H_P4 | H_P5 | H_P6 => return -EINVAL as isize,
        H_STATE => return -EIO as isize,
        H_AUTHORITY => return -EPERM as isize,
        _ => {}
    }
    1
}

unsafe extern "C" fn htmdump_read(filp: *mut file, ubuf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    let data = (*filp).private_data;
    let page = (*ppos as u64 / PAGE_SIZE) * PAGE_SIZE;
    let offset = (*ppos as u64 % PAGE_SIZE) as i64;
    let rc = htm_hcall_wrapper(htmflags, nodeindex, nodalchipindex, coreindexonchip, htmtype, H_HTM_OP_DUMP_DATA, virt_to_phys(data), PAGE_SIZE, page);
    let ret = htm_return_check(rc);
    if ret <= 0 { return ret; }
    let read_size = core::cmp::min(count as u64, PAGE_SIZE);
    *ppos += read_size as i64;
    simple_read_from_buffer(ubuf, count, &mut (offset as i64), data, PAGE_SIZE)
}

unsafe extern "C" fn htmsystem_mem_read(filp: *mut file, ubuf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    let data = (*filp).private_data;
    let mut mem_offset = 0u64;
    if *ppos != 0 { mem_offset = u64::from_be(*(data.add(8) as *mut u64)); if mem_offset == u64::MAX { return 0; } }
    let rc = htm_hcall_wrapper(htmflags, nodeindex, nodalchipindex, coreindexonchip, htmtype, H_HTM_OP_DUMP_SYSMEM_CONF, virt_to_phys(data), PAGE_SIZE, mem_offset);
    let ret = htm_return_check(rc); if ret <= 0 { return ret; }
    let entries = u64::from_be(*(data.add(0x10) as *mut u64)); let to_copy = 32 + entries * 32;
    *ppos += to_copy as i64;
    simple_read_from_buffer(ubuf, count, &mut 0, data, to_copy)
}

unsafe extern "C" fn htmstatus_read(filp: *mut file, ubuf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    let data = (*filp).private_data; let mut off = 0u64;
    if *ppos != 0 { off = u64::from_be(*(data.add(8) as *mut u64)); if off == u64::MAX { return 0; } }
    let ret = htm_return_check(htm_hcall_wrapper(htmflags,nodeindex,nodalchipindex,coreindexonchip,htmtype,H_HTM_OP_STATUS,virt_to_phys(data),PAGE_SIZE,off)); if ret <= 0 { return ret; }
    let n = u64::from_be(*(data.add(0x10) as *mut u64)); let size = 32 + n * if htmtype == 2 { 8 } else { 6 }; *ppos += size as i64;
    simple_read_from_buffer(ubuf,count,&mut 0,data,size)
}

unsafe extern "C" fn htminfo_read(filp: *mut file, ubuf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    let data = (*filp).private_data; let mut off = 0u64;
    if *ppos != 0 { off = u64::from_be(*(data.add(8) as *mut u64)); if off == u64::MAX { return 0; } }
    let ret = htm_return_check(htm_hcall_wrapper(htmflags,nodeindex,nodalchipindex,coreindexonchip,htmtype,H_HTM_OP_DUMP_SYSPROC_CONF,virt_to_phys(data),PAGE_SIZE,off)); if ret <= 0 { return ret; }
    let size = 32 + u64::from_be(*(data.add(0x10) as *mut u64)) * 16; *ppos += size as i64; simple_read_from_buffer(ubuf,count,&mut 0,data,size)
}

unsafe extern "C" fn htmcaps_read(filp: *mut file, ubuf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    let data = (*filp).private_data; let ret = htm_return_check(htm_hcall_wrapper(htmflags,nodeindex,nodalchipindex,coreindexonchip,htmtype,H_HTM_OP_CAPABILITIES,virt_to_phys(data),0x80,0)); if ret <= 0 { return ret; } simple_read_from_buffer(ubuf,count,ppos,data,0x80)
}

unsafe fn htmconfigure_set(_data: *mut c_void, val: u64) -> i32 {
    let (op,p1,p2) = if val == HTM_ENABLE { (H_HTM_OP_CONFIGURE, if htmflags == 0 {0} else {u64::MAX}, if htmflags == 0 {0} else {u64::MAX}) } else if val == HTM_DISABLE { (H_HTM_OP_DECONFIGURE,0,0) } else { return -EINVAL as i32 };
    let ret=htm_return_check(htm_hcall_wrapper(htmflags,nodeindex,nodalchipindex,coreindexonchip,htmtype,op,p1,p2,0)); if ret<=0{return ret as i32;} htmconfigure=val as u32; 0
}
unsafe fn htmstart_set(_data:*mut c_void,val:u64)->i32 { let op=if val==1{H_HTM_OP_START}else if val==0{H_HTM_OP_STOP}else{return -EINVAL as i32}; let r=htm_return_check(htm_hcall_wrapper(htmflags,nodeindex,nodalchipindex,coreindexonchip,htmtype,op,0,0,0)); if r<=0{return r as i32;} htmstart=val as u32; 0 }
unsafe fn htmsetup_set(_data:*mut c_void,val:u64)->i32 { let r=htm_return_check(htm_hcall_wrapper(htmflags,nodeindex,nodalchipindex,coreindexonchip,htmtype,H_HTM_OP_SETUP,val,0,0)); if r<=0{return r as i32;} htmsetup=val as u32; 0 }
unsafe fn htmflags_set(_data:*mut c_void,val:u64)->i32 { if val==HTM_NOWRAP{htmflags=H_HTM_FLAGS_NOWRAP}else if val==HTM_WRAP{htmflags=0}else{return -EINVAL as i32}; 0 }
unsafe fn htmconfigure_get(_data:*mut c_void,val:*mut u64)->i32{*val=htmconfigure as u64;0}
unsafe fn htmstart_get(_data:*mut c_void,val:*mut u64)->i32{*val=htmstart as u64;0}
unsafe fn htmsetup_get(_data:*mut c_void,val:*mut u64)->i32{*val=htmsetup as u64;0}
unsafe fn htmflags_get(_data:*mut c_void,val:*mut u64)->i32{*val=htmflags;0}

static htmdump_fops: file_operations = file_operations { llseek: None, read: Some(htmdump_read), open: Some(simple_open) };
static htmsystem_mem_fops: file_operations = file_operations { llseek: None, read: Some(htmsystem_mem_read), open: Some(simple_open) };
static htmstatus_fops: file_operations = file_operations { llseek: None, read: Some(htmstatus_read), open: Some(simple_open) };
static htminfo_fops: file_operations = file_operations { llseek: None, read: Some(htminfo_read), open: Some(simple_open) };
static htmcaps_fops: file_operations = file_operations { llseek: None, read: Some(htmcaps_read), open: Some(simple_open) };

#[no_mangle] pub unsafe extern "C" fn htmdump_init_debugfs()->i32 { htm_buf=kmalloc(PAGE_SIZE as usize,GFP_KERNEL); if htm_buf.is_null(){return -ENOMEM as i32;} htmdump_debugfs_dir=debugfs_create_dir(b"htmdump\0".as_ptr() as *const i8,arch_debugfs_dir); debugfs_create_u32(b"nodeindex\0".as_ptr() as *const i8,0o600,htmdump_debugfs_dir,&mut nodeindex); debugfs_create_u32(b"nodalchipindex\0".as_ptr() as *const i8,0o600,htmdump_debugfs_dir,&mut nodalchipindex); debugfs_create_u32(b"coreindexonchip\0".as_ptr() as *const i8,0o600,htmdump_debugfs_dir,&mut coreindexonchip); debugfs_create_u32(b"htmtype\0".as_ptr() as *const i8,0o600,htmdump_debugfs_dir,&mut htmtype); debugfs_create_file(b"trace\0".as_ptr() as *const i8,0o400,htmdump_debugfs_dir,htm_buf,&htmdump_fops); htm_status_buf=kmalloc(PAGE_SIZE as usize,GFP_KERNEL); if htm_status_buf.is_null(){return -ENOMEM as i32;} htm_info_buf=kmalloc(PAGE_SIZE as usize,GFP_KERNEL); if htm_info_buf.is_null(){return -ENOMEM as i32;} htm_caps_buf=kmalloc(PAGE_SIZE as usize,GFP_KERNEL); if htm_caps_buf.is_null(){return -ENOMEM as i32;} htm_mem_buf=kmalloc(PAGE_SIZE as usize,GFP_KERNEL); if htm_mem_buf.is_null(){return -ENOMEM as i32;} debugfs_create_file(b"htmstatus\0".as_ptr() as *const i8,0o400,htmdump_debugfs_dir,htm_status_buf,&htmstatus_fops); debugfs_create_file(b"htminfo\0".as_ptr() as *const i8,0o400,htmdump_debugfs_dir,htm_info_buf,&htminfo_fops); debugfs_create_file(b"htmcaps\0".as_ptr() as *const i8,0o400,htmdump_debugfs_dir,htm_caps_buf,&htmcaps_fops); debugfs_create_file(b"htmsystem_mem\0".as_ptr() as *const i8,0o400,htmdump_debugfs_dir,htm_mem_buf,&htmsystem_mem_fops); 0 }

#[no_mangle] pub unsafe extern "C" fn htmdump_init()->i32 { if is_kvm_guest(){return -EOPNOTSUPP as i32;} if htmdump_init_debugfs()!=0{return -ENOMEM as i32;} 0 }
#[no_mangle] pub unsafe extern "C" fn htmdump_exit(){debugfs_remove_recursive(htmdump_debugfs_dir);kfree(htm_buf);kfree(htm_status_buf);kfree(htm_info_buf);kfree(htm_caps_buf);kfree(htm_mem_buf);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
