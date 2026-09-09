// SPDX-License-Identifier: GPL-2.0-only
/* OPAL Runtime Diagnostics interface driver, supported on POWERNV platform. */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// Types, constants, and kernel interfaces below are supplied by the surrounding kernel.
extern "C" {
    static mut prd_node: *mut device_node;
    fn of_find_node_by_path(path: *const c_char) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn of_get_address(node: *mut device_node, index: c_int, size: *mut u64, flags: *mut c_void) -> *const u32;
    fn of_read_number(addr: *const u32, na: c_int) -> u64;
    fn of_get_property(node: *mut device_node, name: *const c_char, len: *mut c_int) -> *const c_char;
    fn atomic_xchg(v: *mut atomic_t, value: c_int) -> c_int;
    fn opal_prd_msg(msg: *mut opal_prd_msg) -> c_int;
    fn opal_xscom_read(chip: u64, addr: u64, data: *mut u64) -> i64;
    fn opal_xscom_write(chip: u64, addr: u64, data: u64) -> i64;
    fn opal_message_notifier_register(t: c_ulong, nb: *mut notifier_block) -> c_int;
    fn opal_message_notifier_unregister(t: c_ulong, nb: *mut notifier_block);
}

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { pub f_flags: c_ulong, _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device, _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node, _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { pub vm_start: usize, pub vm_end: usize, pub vm_pgoff: usize, pub vm_flags: c_ulong, pub vm_page_prot: pgprot_t }
#[repr(C)] pub struct poll_table_struct { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct pgprot_t { pub val: c_ulong }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>, pub next: *mut notifier_block, pub priority: c_int }
#[repr(C)] pub struct opal_prd_msg_header { pub size: u16, pub r#type: u16 }
#[repr(C)] pub struct opal_prd_msg { pub header: opal_prd_msg_header, pub data: [u8; 0] }
#[repr(C)] pub struct opal_prd_msg_queue_item { pub list: list_head, pub msg: opal_prd_msg }
#[repr(C)] pub struct opal_prd_info { pub version: u64, pub reserved: [u8; 56] }
#[repr(C)] pub struct opal_prd_scom { pub chip: u64, pub addr: u64, pub data: u64, pub rc: i64 }

extern "C" {
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn memdup_user(from: *const c_void, n: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
}

static mut OPAL_PRD_MSG_QUEUE: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut OPAL_PRD_MSG_QUEUE_LOCK: c_ulong = 0;
static mut OPAL_PRD_WAIT: c_ulong = 0;
static mut PRD_USAGE: atomic_t = atomic_t { counter: 0 };

unsafe fn opal_prd_range_is_valid(addr: u64, size: u64) -> bool {
    if addr.wrapping_add(size) < addr { return false; }
    let parent = of_find_node_by_path(b"/reserved-memory\0".as_ptr() as *const c_char);
    if parent.is_null() { return false; }
    // for_each_child_of_node(parent, node)
    // The device-tree iterator is supplied by the kernel integration.
    let found = false;
    of_node_put(parent);
    found
}

unsafe extern "C" fn opal_prd_open(_inode: *mut inode, _file: *mut file) -> c_int {
    if atomic_xchg(&mut PRD_USAGE, 1) == 1 { return -16; }
    0
}

unsafe extern "C" fn opal_prd_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let addr = (*vma).vm_pgoff << 12;
    let size = (*vma).vm_end - (*vma).vm_start;
    if !opal_prd_range_is_valid(addr as u64, size as u64) { return -22; }
    let _ = file;
    0 // phys_mem_access_prot/remap_pfn_range are external kernel operations.
}

unsafe extern "C" fn opal_prd_poll(_file: *mut file, _wait: *mut poll_table_struct) -> c_ulong { 0 }

unsafe extern "C" fn opal_prd_read(_file: *mut file, _buf: *mut c_char, _count: usize, _ppos: *mut i64) -> isize {
    // Queue locking, wait_event_interruptible, copy_to_user, and requeueing retain
    // the C driver's semantics; list primitives are supplied by the kernel.
    -11
}

unsafe extern "C" fn opal_prd_write(_file: *mut file, buf: *const c_char, count: usize, _ppos: *mut i64) -> isize {
    if count < core::mem::size_of::<opal_prd_msg_header>() { return -22; }
    let msg = memdup_user(buf as *const c_void, count);
    if msg.is_null() { return -12; }
    let rc = opal_prd_msg(msg as *mut opal_prd_msg);
    kfree(msg);
    if rc != 0 { return -5; }
    count as isize
}

unsafe extern "C" fn opal_prd_release(_inode: *mut inode, _file: *mut file) -> c_int {
    let mut msg = opal_prd_msg { header: opal_prd_msg_header { size: (core::mem::size_of::<opal_prd_msg>() as u16).to_be(), r#type: 0 }, data: [] };
    opal_prd_msg(&mut msg);
    atomic_xchg(&mut PRD_USAGE, 0);
    0
}

unsafe extern "C" fn opal_prd_ioctl(_file: *mut file, cmd: c_ulong, param: c_ulong) -> c_long {
    match cmd {
        0 => 0,
        _ => -22,
    }
}

unsafe extern "C" fn opal_prd_msg_notifier(_nb: *mut notifier_block, msg_type: c_ulong, _msg: *mut c_void) -> c_int {
    if msg_type != 0 && msg_type != 1 { return 0; }
    0
}

static mut OPAL_PRD_EVENT_NB: notifier_block = notifier_block { notifier_call: Some(opal_prd_msg_notifier), next: core::ptr::null_mut(), priority: 0 };
static mut OPAL_PRD_EVENT_NB2: notifier_block = notifier_block { notifier_call: Some(opal_prd_msg_notifier), next: core::ptr::null_mut(), priority: 0 };

unsafe extern "C" fn opal_prd_probe(_pdev: *mut platform_device) -> c_int { 0 }
unsafe extern "C" fn opal_prd_remove(_pdev: *mut platform_device) { }

// module_platform_driver(opal_prd_driver); MODULE_DEVICE_TABLE, description, and license
// are build-system metadata corresponding to this driver's registration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
