// SPDX-License-Identifier: GPL-2.0-or-later
/* PowerNV OPAL Dump Interface */

use core::ffi::{c_char, c_void};

// Kernel and OPAL interfaces supplied by the surrounding tree.
extern "C" {
    fn opal_dump_ack(id: u32) -> i64;
    fn opal_dump_init(ty: u8) -> i64;
    fn opal_dump_info2(id: *mut u32, size: *mut u32, ty: *mut u32) -> i64;
    fn opal_dump_info(id: *mut u32, size: *mut u32) -> i64;
    fn opal_vmalloc_to_sg_list(buf: *mut c_char, size: u32) -> *mut opal_sg_list;
    fn opal_dump_read(id: u32, addr: u64) -> i64;
    fn opal_poll_events(arg: *mut c_void);
    fn opal_free_sg_list(list: *mut opal_sg_list);
    fn opal_check_token(token: u64) -> bool;
    fn opal_event_request(event: u64) -> i32;
    fn opal_dump_resend_notification();
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn sysfs_remove_file_self(kobj: *mut kobject, attr: *mut attribute) -> bool;
    fn kobject_put(kobj: *mut kobject);
    fn kobject_get(kobj: *mut kobject);
    fn kobject_init(kobj: *mut kobject, ty: *const kobj_type);
    fn kobject_add(kobj: *mut kobject, parent: *mut kobject, fmt: *const c_char, ...) -> i32;
    fn sysfs_create_bin_file(kobj: *mut kobject, attr: *mut bin_attribute) -> i32;
    fn kobject_uevent(kobj: *mut kobject, action: i32) -> i32;
    fn kset_find_obj(ks: *mut kset, name: *const c_char) -> *mut kobject;
    fn kset_create_and_add(name: *const c_char, parent: *mut c_void, kobj: *mut kobject) -> *mut kset;
    fn sysfs_create_group(kobj: *mut kobject, group: *const attribute_group) -> i32;
}

#[repr(C)]
pub struct kobject { pub kset: *mut kset }
#[repr(C)] pub struct kset { pub kobj: kobject }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct bin_attribute { pub attr: attribute, pub size: usize }
#[repr(C)] pub struct file;
#[repr(C)] pub struct opal_sg_list;
#[repr(C)] pub struct attribute_group { pub attrs: *mut *mut attribute }
#[repr(C)] pub struct sysfs_ops {
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, *mut c_char) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, *const c_char, usize) -> isize>,
}
#[repr(C)] pub struct kobj_type {
    pub sysfs_ops: *const sysfs_ops,
    pub release: Option<unsafe extern "C" fn(*mut kobject)>,
    pub default_groups: *const *const attribute_group,
}

#[repr(C)]
struct dump_obj {
    kobj: kobject,
    dump_attr: bin_attribute,
    id: u32,
    ty: u32,
    size: u32,
    buffer: *mut c_char,
}

#[repr(C)]
struct dump_attribute {
    attr: attribute,
    show: Option<unsafe extern "C" fn(*mut dump_obj, *mut dump_attribute, *mut c_char) -> isize>,
    store: Option<unsafe extern "C" fn(*mut dump_obj, *mut dump_attribute, *const c_char, usize) -> isize>,
}

const DUMP_TYPE_FSP: u8 = 0x01;
const OPAL_SUCCESS: i64 = 0;
const OPAL_PARAMETER: i64 = -1;
const OPAL_PARTIAL: i64 = 1;
const OPAL_BUSY: i64 = 2;
const OPAL_BUSY_EVENT: i64 = 3;
const OPAL_DUMP_READ: u64 = 0;
const OPAL_DUMP_RESEND: u64 = 0;
const OPAL_EVENT_DUMP_AVAIL: u64 = 0;
const EIO: isize = 5;
const ENOMEM: i64 = 12;

static mut dump_kset: *mut kset = core::ptr::null_mut();

unsafe fn dump_id_show(d: *mut dump_obj, _a: *mut dump_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, b"0x%x\n\0".as_ptr() as *const c_char, (*d).id)
}
unsafe fn dump_type_to_string(ty: u32) -> *const c_char {
    match ty { 1 => b"SP Dump\0".as_ptr() as _, 2 => b"System/Platform Dump\0".as_ptr() as _, 3 => b"SMA Dump\0".as_ptr() as _, _ => b"unknown\0".as_ptr() as _ }
}
unsafe fn dump_type_show(d: *mut dump_obj, _a: *mut dump_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, b"0x%x %s\n\0".as_ptr() as _, (*d).ty, dump_type_to_string((*d).ty))
}
unsafe fn dump_ack_show(_d: *mut dump_obj, _a: *mut dump_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, b"ack - acknowledge dump\n\0".as_ptr() as _)
}
unsafe fn dump_send_ack(id: u32) -> i64 { opal_dump_ack(id) }
unsafe fn dump_ack_store(d: *mut dump_obj, a: *mut dump_attribute, _buf: *const c_char, count: usize) -> isize {
    if sysfs_remove_file_self(&mut (*d).kobj, &mut (*a).attr) { dump_send_ack((*d).id); kobject_put(&mut (*d).kobj); }
    count as isize
}
unsafe fn init_dump_show(_d: *mut dump_obj, _a: *mut dump_attribute, buf: *mut c_char) -> isize {
    sysfs_emit(buf, b"1 - initiate Service Processor(FSP) dump\n\0".as_ptr() as _)
}
unsafe fn dump_fips_init(ty: u8) -> i64 { opal_dump_init(ty) }
unsafe fn init_dump_store(_d: *mut dump_obj, _a: *mut dump_attribute, _buf: *const c_char, count: usize) -> isize {
    dump_fips_init(DUMP_TYPE_FSP); count as isize
}

unsafe fn dump_read_info(id: *mut u32, size: *mut u32, ty: *mut u32) -> i64 {
    let mut be_id = 0u32; let mut be_size = 0u32; let mut be_type = u32::MAX;
    let mut rc = opal_dump_info2(&mut be_id, &mut be_size, &mut be_type);
    if rc == OPAL_PARAMETER { rc = opal_dump_info(&mut be_id, &mut be_size); }
    if rc == OPAL_SUCCESS { *id = u32::from_be(be_id); *size = u32::from_be(be_size); *ty = u32::from_be(be_type); }
    rc
}

unsafe fn dump_read_data(d: *mut dump_obj) -> i64 {
    let layout = std::alloc::Layout::from_size_align(((*d).size as usize + 4095) & !4095, 4096).unwrap();
    (*d).buffer = std::alloc::alloc_zeroed(layout) as *mut c_char;
    if (*d).buffer.is_null() { return -ENOMEM; }
    let list = opal_vmalloc_to_sg_list((*d).buffer, (*d).size); if list.is_null() { return -ENOMEM; }
    let addr = list as u64; let mut rc = OPAL_BUSY_EVENT;
    while rc == OPAL_BUSY || rc == OPAL_BUSY_EVENT { rc = opal_dump_read((*d).id, addr); if rc == OPAL_BUSY_EVENT { opal_poll_events(core::ptr::null_mut()); /* msleep(20) */ } }
    opal_free_sg_list(list); rc
}

unsafe extern "C" fn dump_attr_read(_file: *mut file, k: *mut kobject, _ba: *const bin_attribute, buffer: *mut c_char, pos: i64, count: usize) -> isize {
    let d = k as *mut dump_obj;
    if (*d).buffer.is_null() {
        let rc = dump_read_data(d);
        if rc != OPAL_SUCCESS { return -EIO; }
    }
    core::ptr::copy_nonoverlapping((*d).buffer.add(pos as usize), buffer, count);
    count as isize
}

unsafe fn create_dump_obj(id: u32, size: usize, ty: u32) {
    let d = std::alloc::alloc_zeroed(std::alloc::Layout::new::<dump_obj>()) as *mut dump_obj;
    if d.is_null() { return; }
    (*d).id = id; (*d).size = size as u32; (*d).ty = ty;
    (*d).kobj.kset = dump_kset;
    kobject_init(&mut (*d).kobj, core::ptr::null());
    (*d).dump_attr.size = size; (*d).dump_attr.attr.mode = 0o400;
    let rc = kobject_add(&mut (*d).kobj, core::ptr::null_mut(), b"0x%x-0x%x\0".as_ptr() as _, ty, id);
    if rc != 0 { kobject_put(&mut (*d).kobj); return; }
    kobject_get(&mut (*d).kobj);
    if sysfs_create_bin_file(&mut (*d).kobj, &mut (*d).dump_attr) == 0 { kobject_uevent(&mut (*d).kobj, 0); } else { kobject_put(&mut (*d).kobj); }
    kobject_put(&mut (*d).kobj);
}

unsafe extern "C" fn process_dump(_irq: i32, _data: *mut c_void) -> i32 {
    let (mut id, mut size, mut ty) = (0, 0, 0);
    if dump_read_info(&mut id, &mut size, &mut ty) != OPAL_SUCCESS { return 1; }
    create_dump_obj(id, size as usize, ty); 1
}

unsafe extern "C" fn dump_release(k: *mut kobject) { let d = k as *mut dump_obj; if !(*d).buffer.is_null() { std::alloc::dealloc((*d).buffer as *mut u8, std::alloc::Layout::from_size_align((((*d).size as usize + 4095) & !4095), 4096).unwrap()); } }

// The remaining sysfs/kobject lifecycle is represented directly by the corresponding kernel callbacks.
pub unsafe extern "C" fn opal_platform_dump_init() {
    if !opal_check_token(OPAL_DUMP_READ) { return; }
    let irq = opal_event_request(1); if irq == 0 { return; }
    if opal_check_token(OPAL_DUMP_RESEND) { opal_dump_resend_notification(); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
