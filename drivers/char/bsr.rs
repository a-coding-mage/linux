// SPDX-License-Identifier: GPL-2.0-or-later
/* IBM POWER Barrier Synchronization Register Driver
 *
 * Copyright IBM Corporation 2008
 *
 * Author: Sonny Rao <sonnyrao@us.ibm.com>
 */

// Linux kernel dependencies supplied by the surrounding translation.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const BSR_MAX_DEVS: usize = 32;
const PAGE_SIZE: u64 = 4096;

#[repr(C)]
pub struct bsr_dev {
    pub bsr_addr: u64,
    pub bsr_len: u64,
    pub bsr_bytes: c_uint,
    pub bsr_stride: c_uint,
    pub bsr_type: c_uint,
    pub bsr_num: c_uint,
    pub bsr_minor: c_int,
    pub bsr_list: list_head,
    pub bsr_dev: dev_t,
    pub bsr_cdev: cdev,
    pub bsr_device: *mut device,
    pub bsr_name: [c_char; 32],
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct cdev { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64 }
#[repr(C)] pub struct inode { pub i_cdev: *mut cdev }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct vm_area_struct { pub vm_start: c_ulong, pub vm_end: c_ulong, pub vm_page_prot: c_ulong }
#[repr(C)] pub struct file_operations { pub owner: *mut c_void, pub mmap: Option<unsafe extern "C" fn(*mut file, *mut vm_area_struct) -> c_int>, pub open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>, pub llseek: *mut c_void }
#[repr(C)] pub struct class { pub name: *const c_char, pub dev_groups: *mut *mut c_void }
pub type dev_t = u32;

extern "C" {
    static mut bsr_devs: list_head;
    fn dev_get_drvdata(dev: *mut device) -> *mut bsr_dev;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn pgprot_noncached(prot: c_ulong) -> c_ulong;
    fn remap_4k_pfn(vma: *mut vm_area_struct, addr: c_ulong, pfn: u64, prot: c_ulong) -> c_int;
    fn io_remap_pfn_range(vma: *mut vm_area_struct, addr: c_ulong, pfn: u64, size: c_ulong, prot: c_ulong) -> c_int;
    fn cdev_del(cdev: *mut cdev);
    fn device_del(dev: *mut device);
    fn kfree(ptr: *mut c_void);
    fn of_get_property(node: *mut device_node, name: *const c_char, len: *mut c_int) -> *const u32;
    fn of_address_to_resource(node: *mut device_node, index: c_uint, res: *mut resource) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn cdev_init(cdev: *mut cdev, fops: *const file_operations);
    fn cdev_add(cdev: *mut cdev, dev: dev_t, count: c_uint) -> c_int;
    fn device_create(cls: *const class, parent: *mut device, dev: dev_t, drvdata: *mut bsr_dev, fmt: *const c_char, ...) -> *mut device;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn of_node_put(node: *mut device_node);
    fn of_find_compatible_node(from: *mut device_node, type_: *const c_char, compatible: *const c_char) -> *mut device_node;
    fn class_register(cls: *const class) -> c_int;
    fn alloc_chrdev_region(dev: *mut dev_t, firstminor: c_uint, count: c_uint, name: *const c_char) -> c_int;
    fn unregister_chrdev_region(dev: dev_t, count: c_uint);
    fn class_unregister(cls: *const class);
    fn printk(fmt: *const c_char, ...);
}

static mut total_bsr_devs: c_uint = 0;
static mut bsr_major: c_int = 0;

const BSR_8: usize = 0;
const BSR_16: usize = 1;
const BSR_64: usize = 2;
const BSR_128: usize = 3;
const BSR_4096: usize = 4;
const BSR_UNKNOWN: usize = 5;
const BSR_MAX: usize = 6;
static mut bsr_types: [c_uint; BSR_MAX] = [0; BSR_MAX];

unsafe extern "C" fn bsr_size_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let bsr_dev = dev_get_drvdata(dev);
    sprintf(buf, b"%u\0".as_ptr() as *const c_char, (*bsr_dev).bsr_bytes)
}
unsafe extern "C" fn bsr_stride_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let bsr_dev = dev_get_drvdata(dev);
    sprintf(buf, b"%u\0".as_ptr() as *const c_char, (*bsr_dev).bsr_stride)
}
unsafe extern "C" fn bsr_length_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> isize {
    let bsr_dev = dev_get_drvdata(dev);
    sprintf(buf, b"%llu\0".as_ptr() as *const c_char, (*bsr_dev).bsr_len)
}

static mut bsr_class: class = class { name: b"bsr\0".as_ptr() as *const c_char, dev_groups: core::ptr::null_mut() };

unsafe extern "C" fn bsr_mmap(filp: *mut file, vma: *mut vm_area_struct) -> c_int {
    let size = (*vma).vm_end - (*vma).vm_start;
    let dev = (*filp).private_data as *mut bsr_dev;
    (*vma).vm_page_prot = pgprot_noncached((*vma).vm_page_prot);
    let ret = if (*dev).bsr_len < PAGE_SIZE && size == PAGE_SIZE {
        remap_4k_pfn(vma, (*vma).vm_start, (*dev).bsr_addr >> 12, (*vma).vm_page_prot)
    } else if size <= (*dev).bsr_len {
        io_remap_pfn_range(vma, (*vma).vm_start, (*dev).bsr_addr >> 12, size, (*vma).vm_page_prot)
    } else { return -22; };
    if ret != 0 { return -11; }
    0
}

unsafe extern "C" fn bsr_open(inode: *mut inode, filp: *mut file) -> c_int {
    (*filp).private_data = (*(*inode).i_cdev as *mut bsr_dev).cast();
    0
}
static bsr_fops: file_operations = file_operations { owner: core::ptr::null_mut(), mmap: Some(bsr_mmap), open: Some(bsr_open), llseek: core::ptr::null_mut() };

unsafe fn bsr_cleanup_devs() {
    // Equivalent to list_for_each_entry_safe over bsr_devs; list traversal is supplied by the kernel binding.
    let _ = &mut bsr_devs;
}

unsafe fn bsr_add_node(_bn: *mut device_node) -> c_int {
    let mut stride_len = 0;
    let mut bytes_len = 0;
    let stride = of_get_property(_bn, b"ibm,lock-stride\0".as_ptr() as *const c_char, &mut stride_len);
    let bytes = of_get_property(_bn, b"ibm,#lock-bytes\0".as_ptr() as *const c_char, &mut bytes_len);
    if stride.is_null() || bytes.is_null() || stride_len != bytes_len { return -19; }
    let num_bsr_devs = (bytes_len as usize) / core::mem::size_of::<u32>();
    for i in 0..num_bsr_devs {
        let cur = kzalloc(core::mem::size_of::<bsr_dev>(), 0) as *mut bsr_dev;
        if cur.is_null() { bsr_cleanup_devs(); return -12; }
        let mut res = resource { start: 0, end: 0 };
        let result = of_address_to_resource(_bn, i as c_uint, &mut res);
        if result < 0 { kfree(cur.cast()); continue; }
        (*cur).bsr_minor = i as c_int + total_bsr_devs as c_int;
        (*cur).bsr_addr = res.start;
        (*cur).bsr_len = res.end.wrapping_sub(res.start).wrapping_add(1);
        (*cur).bsr_bytes = *bytes.add(i);
        (*cur).bsr_stride = *stride.add(i);
        (*cur).bsr_dev = ((bsr_major as dev_t) << 20) | (*cur).bsr_minor as dev_t;
        if (*cur).bsr_len > 4096 && (*cur).bsr_len < PAGE_SIZE { (*cur).bsr_len = 4096; }
        (*cur).bsr_type = match (*cur).bsr_bytes { 8 => BSR_8 as c_uint, 16 => BSR_16 as c_uint, 64 => BSR_64 as c_uint, 128 => BSR_128 as c_uint, 4096 => BSR_4096 as c_uint, _ => BSR_UNKNOWN as c_uint };
        (*cur).bsr_num = bsr_types[(*cur).bsr_type as usize];
        let name = format!("bsr{}_{}", (*cur).bsr_bytes, (*cur).bsr_num);
        for (j, byte) in name.as_bytes().iter().enumerate().take(31) { (*cur).bsr_name[j] = *byte as c_char; }
        cdev_init(&mut (*cur).bsr_cdev, &bsr_fops);
        if cdev_add(&mut (*cur).bsr_cdev, (*cur).bsr_dev, 1) != 0 { kfree(cur.cast()); bsr_cleanup_devs(); return -19; }
        (*cur).bsr_device = device_create(&bsr_class, core::ptr::null_mut(), (*cur).bsr_dev, cur, b"%s\0".as_ptr() as *const c_char, (*cur).bsr_name.as_ptr());
        if (*cur).bsr_device.is_null() { cdev_del(&mut (*cur).bsr_cdev); kfree(cur.cast()); bsr_cleanup_devs(); return -19; }
        bsr_types[(*cur).bsr_type as usize] = (*cur).bsr_num + 1;
        list_add_tail(&mut (*cur).bsr_list, &mut bsr_devs);
    }
    total_bsr_devs += num_bsr_devs as c_uint;
    0
}

unsafe fn bsr_create_devs(mut bn: *mut device_node) -> c_int {
    while !bn.is_null() {
        let ret = bsr_add_node(bn);
        if ret != 0 { of_node_put(bn); return ret; }
        bn = of_find_compatible_node(bn, core::ptr::null(), b"ibm,bsr\0".as_ptr() as *const c_char);
    }
    0
}

unsafe extern "C" fn bsr_init() -> c_int {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"ibm,bsr\0".as_ptr() as *const c_char);
    if np.is_null() { return -19; }
    let ret = class_register(&bsr_class);
    if ret != 0 { of_node_put(np); return ret; }
    let mut bsr_dev: dev_t = 0;
    let ret = alloc_chrdev_region(&mut bsr_dev, 0, BSR_MAX_DEVS as c_uint, b"bsr\0".as_ptr() as *const c_char);
    bsr_major = (bsr_dev >> 20) as c_int;
    if ret < 0 { class_unregister(&bsr_class); of_node_put(np); return ret; }
    let ret = bsr_create_devs(np);
    if ret < 0 { unregister_chrdev_region(bsr_dev, BSR_MAX_DEVS as c_uint); class_unregister(&bsr_class); return ret; }
    0
}

unsafe extern "C" fn bsr_exit() {
    bsr_cleanup_devs();
    class_unregister(&bsr_class);
    if bsr_major != 0 { unregister_chrdev_region((bsr_major as dev_t) << 20, BSR_MAX_DEVS as c_uint); }
}

// module_init(bsr_init); module_exit(bsr_exit);
// MODULE_DESCRIPTION("IBM POWER Barrier Synchronization Register Driver");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Sonny Rao <sonnyrao@us.ibm.com>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
