// SPDX-License-Identifier: GPL-2.0-only
/*
 * Code for looking up block devices in the early boot code before mounting the
 * root file system.
 */

// Kernel declarations supplied by the surrounding build.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct uuidcmp {
    pub uuid: *const c_char,
    pub len: c_int,
}

extern "C" {
    static mut block_class: c_void;
    static mut disk_type: c_void;
    fn dev_to_bdev(dev: *mut device) -> *mut block_device;
    fn dev_to_disk(dev: *mut device) -> *mut gendisk;
    fn bdev_partno(bdev: *mut block_device) -> c_int;
    fn bdev_nr_sectors(bdev: *mut block_device) -> u64;
    fn bdev_is_partition(bdev: *mut block_device) -> c_int;
    fn part_devt(disk: *mut gendisk, partno: c_int) -> dev_t;
    fn class_find_device(class: *mut c_void, start: *mut device, data: *const c_void,
                         match_fn: unsafe extern "C" fn(*mut device, *const c_void) -> c_int) -> *mut device;
    fn class_dev_iter_init(iter: *mut class_dev_iter, class: *mut c_void, start: *mut device, typ: *mut c_void);
    fn class_dev_iter_next(iter: *mut class_dev_iter) -> *mut device;
    fn class_dev_iter_exit(iter: *mut class_dev_iter);
    fn put_device(dev: *mut device);
    fn new_decode_dev(dev: u32) -> dev_t;
    fn simple_strtoul(s: *const c_char, end: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn printk(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

pub type dev_t = u64;
pub type c_ulong = usize;

#[repr(C)] pub struct device { pub devt: dev_t, pub parent: *mut device, pub driver: *mut driver }
#[repr(C)] pub struct driver { pub name: *const c_char }
#[repr(C)] pub struct block_device { pub bd_meta_info: *mut partition_meta_info, pub bd_dev: dev_t }
#[repr(C)] pub struct partition_meta_info { pub uuid: *const c_char, pub volname: *const c_char }
#[repr(C)] pub struct gendisk { pub minors: c_int, pub flags: c_uint, pub part_tbl: c_void }
#[repr(C)] pub struct class_dev_iter { _private: [u8; 0] }

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const GENHD_FL_HIDDEN: c_uint = 1 << 4;
const BDEVT_SIZE: usize = 16;

unsafe extern "C" fn match_dev_by_uuid(dev: *mut device, data: *const c_void) -> c_int {
    let bdev = dev_to_bdev(dev);
    let cmp = data as *const uuidcmp;
    if (*bdev).bd_meta_info.is_null() ||
       strncasecmp((*cmp).uuid, (*(*bdev).bd_meta_info).uuid, (*cmp).len as usize) != 0 { 0 } else { 1 }
}

unsafe extern "C" fn devt_from_partuuid(uuid_str: *const c_char, devt: *mut dev_t) -> c_int {
    let mut cmp = uuidcmp { uuid: uuid_str, len: 0 };
    let mut dev: *mut device = core::ptr::null_mut();
    let mut offset = 0;
    let slash = strchr(uuid_str, b'/' as c_int);
    if !slash.is_null() {
        let mut c = 0i8;
        if sscanf(slash.add(1), b"PARTNROFF=%d%c\0".as_ptr() as *const c_char, &mut offset, &mut c) != 1 { goto_invalid(); return -EINVAL; }
        cmp.len = slash.offset_from(uuid_str) as c_int;
    } else { cmp.len = strlen(uuid_str) as c_int; }
    if cmp.len == 0 { goto_invalid(); return -EINVAL; }
    dev = class_find_device(&mut block_class, core::ptr::null_mut(), &cmp as *const _ as *const c_void, match_dev_by_uuid);
    if dev.is_null() { return -ENODEV; }
    if offset != 0 { *devt = part_devt(dev_to_disk(dev), bdev_partno(dev_to_bdev(dev)) + offset); } else { *devt = (*dev).devt; }
    put_device(dev); 0
}

unsafe fn goto_invalid() { pr_err(b"VFS: PARTUUID= is invalid.\nExpected PARTUUID=<valid-uuid-id>[/PARTNROFF=%%d]\n\0".as_ptr() as *const c_char); }

unsafe extern "C" fn match_dev_by_label(dev: *mut device, data: *const c_void) -> c_int {
    let bdev = dev_to_bdev(dev); let label = data as *const c_char;
    if (*bdev).bd_meta_info.is_null() || strcmp(label, (*(*bdev).bd_meta_info).volname) != 0 { 0 } else { 1 }
}

unsafe extern "C" fn devt_from_partlabel(label: *const c_char, devt: *mut dev_t) -> c_int {
    let dev = class_find_device(&mut block_class, core::ptr::null_mut(), label as *const c_void, match_dev_by_label);
    if dev.is_null() { return -ENODEV; } *devt = (*dev).devt; put_device(dev); 0
}

unsafe extern "C" fn devt_from_devnum(name: *const c_char, devt: *mut dev_t) -> c_int {
    let mut maj = 0u32; let mut min = 0u32; let mut offset = 0u32; let mut dummy = 0i8; let mut p = core::ptr::null_mut();
    if sscanf(name, b"%u:%u%c\0".as_ptr() as *const c_char, &mut maj, &mut min, &mut dummy) == 2 || sscanf(name, b"%u:%u:%u:%c\0".as_ptr() as *const c_char, &mut maj, &mut min, &mut offset, &mut dummy) == 3 { *devt = ((maj as u64) << 32) | min as u64; } else { *devt = new_decode_dev(simple_strtoul(name, &mut p, 16) as u32); if !p.is_null() && *p != 0 { return -EINVAL; } } 0
}

pub unsafe extern "C" fn early_lookup_bdev(name: *const c_char, devt: *mut dev_t) -> c_int {
    if strncmp(name, b"PARTUUID=\0".as_ptr() as *const c_char, 9) == 0 { return devt_from_partuuid(name.add(9), devt); }
    if strncmp(name, b"PARTLABEL=\0".as_ptr() as *const c_char, 10) == 0 { return devt_from_partlabel(name.add(10), devt); }
    if strncmp(name, b"/dev/\0".as_ptr() as *const c_char, 5) == 0 { return devt_from_devname(name.add(5), devt); }
    devt_from_devnum(name, devt)
}

unsafe extern "C" fn blk_lookup_devt(name: *const c_char, partno: c_int) -> dev_t {
    let mut iter = core::mem::MaybeUninit::<class_dev_iter>::uninit(); let mut result = 0;
    class_dev_iter_init(iter.as_mut_ptr(), &mut block_class, core::ptr::null_mut(), &mut disk_type);
    let it = iter.as_mut_ptr();
    while { let dev = class_dev_iter_next(it); if dev.is_null() { false } else {
        let disk = dev_to_disk(dev);
        if strcmp(dev_name(dev), name) == 0 { if partno < (*disk).minors { result = ((*dev).devt & 0xffff_ffff_0000_0000) | (((*dev).devt & 0xffff_ffff) + partno as u64); } else { result = part_devt(disk, partno); if result != 0 { class_dev_iter_exit(it); return result; } } } true
    }} {} class_dev_iter_exit(it); result
}

unsafe extern "C" fn devt_from_devname(name: *const c_char, devt: *mut dev_t) -> c_int {
    let len = strlen(name); if len > 31 { return -EINVAL; }
    let mut s = [0i8; 32]; core::ptr::copy_nonoverlapping(name, s.as_mut_ptr(), len + 1);
    for i in 0..len { if s[i] == b'/' as i8 { s[i] = b'!' as i8; } }
    *devt = blk_lookup_devt(s.as_ptr(), 0); if *devt != 0 { return 0; }
    let mut p = len; while p > 0 && isdigit(s[p - 1]) != 0 { p -= 1; }
    if p == 0 || s[p] == 0 || s[p] == b'0' as i8 { return -ENODEV; }
    let part = simple_strtoul(s.as_ptr().add(p), core::ptr::null_mut(), 10) as c_int; s[p] = 0;
    *devt = blk_lookup_devt(s.as_ptr(), part); if *devt != 0 { return 0; }
    if p < 2 || isdigit(s[p - 2]) == 0 || s[p - 1] != b'p' as i8 { return -ENODEV; }
    s[p - 1] = 0; *devt = blk_lookup_devt(s.as_ptr(), part); if *devt != 0 { 0 } else { -ENODEV }
}

extern "C" { fn dev_name(dev: *mut device) -> *const c_char; fn isdigit(c: c_int) -> c_int; }

pub unsafe extern "C" fn printk_all_partitions() {
    let mut iter = core::mem::MaybeUninit::<class_dev_iter>::uninit();
    class_dev_iter_init(iter.as_mut_ptr(), &mut block_class, core::ptr::null_mut(), &mut disk_type);
    let it = iter.as_mut_ptr();
    while { let dev = class_dev_iter_next(it); if dev.is_null() { false } else {
        let disk = dev_to_disk(dev); if get_capacity(disk) != 0 && ((*disk).flags & GENHD_FL_HIDDEN) == 0 { /* xa_for_each over disk->part_tbl is supplied by the kernel. */ }
        true
    }} {} class_dev_iter_exit(it);
}

extern "C" { fn get_capacity(disk: *mut gendisk) -> u64; }

extern "C" {
    fn strlen(s: *const c_char) -> usize; fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strncasecmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
