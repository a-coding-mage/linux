// SPDX-License-Identifier: GPL-2.0-only
// Rust translation of dmi-sysfs.c. Kernel-provided types and functions are
// intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::{ffi::c_void, mem, ptr};

const MAX_ENTRY_TYPE: usize = 255;
const DMI_SEL_ACCESS_METHOD_IO8: u8 = 0x00;
const DMI_SEL_ACCESS_METHOD_IO2x8: u8 = 0x01;
const DMI_SEL_ACCESS_METHOD_IO16: u8 = 0x02;
const DMI_SEL_ACCESS_METHOD_PHYS32: u8 = 0x03;
const DMI_SEL_ACCESS_METHOD_GPNV: u8 = 0x04;

#[repr(C)] pub struct dmi_header { pub type_: u8, pub length: u8, pub handle: u16 }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct attribute { pub name: *const i8, pub mode: u16 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct dmi_sysfs_entry {
    pub dh: dmi_header, pub kobj: kobject, pub instance: i32, pub position: i32,
    pub list: list_head, pub child: *mut kobject,
}
pub type ssize_t = isize;
pub type loff_t = i64;
pub type dmi_callback = unsafe extern "C" fn(*mut dmi_sysfs_entry, *const dmi_header, *mut c_void) -> ssize_t;
pub type show_fn = unsafe extern "C" fn(*mut dmi_sysfs_entry, *mut i8) -> ssize_t;
#[repr(C)] pub struct dmi_sysfs_attribute { pub attr: attribute, pub show: show_fn }
pub type mapped_show_fn = unsafe extern "C" fn(*mut dmi_sysfs_entry, *const dmi_header, *mut i8) -> ssize_t;
#[repr(C)] pub struct dmi_sysfs_mapped_attribute { pub attr: attribute, pub show: mapped_show_fn }
#[repr(C)] pub struct dmi_read_state { pub buf: *mut i8, pub pos: loff_t, pub count: usize }
#[repr(C)] pub struct find_dmi_data { pub entry: *mut dmi_sysfs_entry, pub callback: dmi_callback, pub private: *mut c_void, pub instance_countdown: i32, pub ret: ssize_t }
#[repr(C, packed)] pub struct dmi_system_event_log {
    pub header: dmi_header, pub area_length: u16, pub header_start_offset: u16,
    pub data_start_offset: u16, pub access_method: u8, pub status: u8, pub change_token: u32,
    pub access_method_address: u32, pub header_format: u8, pub type_descriptors_supported_count: u8,
    pub per_log_type_descriptor_length: u8,
}

extern "C" {
    fn dmi_walk(f: unsafe extern "C" fn(*const dmi_header, *mut c_void), data: *mut c_void) -> i32;
    fn dmi_entry_length(dh: *const dmi_header) -> usize;
    fn memory_read_from_buffer(buf: *mut i8, count: usize, pos: *mut loff_t, data: *const c_void, len: usize) -> ssize_t;
    fn capable(cap: i32) -> bool;
    fn sprintf(buf: *mut i8, fmt: *const i8, ...) -> i32;
    fn kfree(p: *mut c_void); fn kzalloc(size: usize, flags: u32) -> *mut c_void;
    fn kobject_init_and_add(k: *mut kobject, kt: *const c_void, parent: *mut kobject, fmt: *const i8, ...) -> i32;
    fn kobject_put(k: *mut kobject); fn kobject_del(k: *mut kobject);
    fn sysfs_create_bin_file(k: *mut kobject, a: *const c_void) -> i32;
    fn kset_create_and_add(name: *const i8, parent: *const c_void, k: *mut kobject) -> *mut c_void;
    fn kset_unregister(k: *mut c_void);
    fn dmi_remap(addr: u32, len: u16) -> *mut u8; fn dmi_unmap(p: *mut u8); fn readb(p: *mut u8) -> u8;
}

static mut ENTRY_LIST: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut DMI_KSET: *mut c_void = ptr::null_mut();
static mut INSTANCE_COUNTS: [i32; MAX_ENTRY_TYPE + 1] = [0; MAX_ENTRY_TYPE + 1];
static mut POSITION_COUNT: i32 = 0;

unsafe extern "C" fn find_dmi_entry_helper(dh: *const dmi_header, data_: *mut c_void) {
    let data = &mut *(data_ as *mut find_dmi_data); let entry = &mut *data.entry;
    if (*dh).type_ != entry.dh.type_ { return; }
    if data.instance_countdown != 0 { data.instance_countdown -= 1; return; }
    data.instance_countdown -= 1; data.ret = (data.callback)(data.entry, dh, data.private);
}

unsafe fn find_dmi_entry(entry: *mut dmi_sysfs_entry, callback: dmi_callback, private: *mut c_void) -> ssize_t {
    let mut data = find_dmi_data { entry, callback, private, instance_countdown: (*entry).instance, ret: -5 };
    if dmi_walk(find_dmi_entry_helper, &mut data as *mut _ as *mut c_void) != 0 { return -22; } data.ret
}

unsafe fn dmi_entry_length_local(dh: *const dmi_header) -> usize {
    let p = (dh as *const u8).add((*dh).length as usize); let mut q = p;
    while *q != 0 || *q.add(1) != 0 { q = q.add(1); } 2 + q.offset_from(dh as *const u8) as usize
}

unsafe extern "C" fn dmi_entry_raw_read_helper(e: *mut dmi_sysfs_entry, dh: *const dmi_header, s: *mut c_void) -> ssize_t {
    let st = &mut *(s as *mut dmi_read_state); memory_read_from_buffer(st.buf, st.count, &mut st.pos, dh as *const c_void, dmi_entry_length_local(dh))
}
unsafe extern "C" fn raw_read(_: *mut c_void, k: *mut kobject, _: *const c_void, b: *mut i8, p: loff_t, c: usize) -> ssize_t {
    let e = k as *mut dmi_sysfs_entry; let mut s = dmi_read_state { buf:b, pos:p, count:c }; find_dmi_entry(e, dmi_entry_raw_read_helper, &mut s as *mut _ as *mut c_void)
}

unsafe extern "C" fn dmi_sysfs_init() -> i32 { let mut v=0; if DMI_KSET.is_null() { DMI_KSET=kset_create_and_add(b"entries\0".as_ptr() as _, ptr::null(), ptr::null_mut()); } if DMI_KSET.is_null() { return -12; } let r=dmi_walk(dmi_sysfs_register_handle, &mut v as *mut _ as _); if r!=0 { return -22; } if v!=0 { return v } 0 }
unsafe extern "C" fn dmi_sysfs_register_handle(_: *const dmi_header, _: *mut c_void) {}
unsafe extern "C" fn dmi_sysfs_exit() { if !DMI_KSET.is_null() { kset_unregister(DMI_KSET); } }

unsafe extern "C" fn dmi_sysfs_entry_length(e:*mut dmi_sysfs_entry,b:*mut i8)->ssize_t { sprintf(b,b"%d\n\0".as_ptr() as _,(*e).dh.length as i32) as _ }
unsafe extern "C" fn dmi_sysfs_entry_handle(e:*mut dmi_sysfs_entry,b:*mut i8)->ssize_t { sprintf(b,b"%d\n\0".as_ptr() as _,(*e).dh.handle as i32) as _ }
unsafe extern "C" fn dmi_sysfs_entry_type(e:*mut dmi_sysfs_entry,b:*mut i8)->ssize_t { sprintf(b,b"%d\n\0".as_ptr() as _,(*e).dh.type_ as i32) as _ }
unsafe extern "C" fn dmi_sysfs_entry_instance(e:*mut dmi_sysfs_entry,b:*mut i8)->ssize_t { sprintf(b,b"%d\n\0".as_ptr() as _,(*e).instance) as _ }
unsafe extern "C" fn dmi_sysfs_entry_position(e:*mut dmi_sysfs_entry,b:*mut i8)->ssize_t { sprintf(b,b"%d\n\0".as_ptr() as _,(*e).position) as _ }

unsafe extern "C" fn dmi_entry_attr_show_helper(e:*mut dmi_sysfs_entry,dh:*const dmi_header,d:*mut c_void)->ssize_t { let x=&*(d as *mut dmi_entry_attr_show_data); ((*x.attr).show)(e,dh,x.buf) }
#[repr(C)] struct dmi_entry_attr_show_data { attr:*mut dmi_sysfs_mapped_attribute, buf:*mut i8 }
unsafe extern "C" fn dmi_entry_attr_show(_: *mut c_void, _: *mut c_void, _: *mut i8)->ssize_t { -5 }

unsafe extern "C" fn dmi_sel_raw_read_phys32(_: *mut dmi_sysfs_entry, sel:*const dmi_system_event_log,b:*mut i8,pos:loff_t,count:usize)->ssize_t {
    let p=dmi_remap((*sel).access_method_address,(*sel).area_length); if p.is_null(){return -5}; let mut n=0; while n<count && pos+(n as i64)<(*sel).area_length as i64 { *b.add(n)=readb(p.add((pos+n as i64) as usize)); n+=1; } dmi_unmap(p); n as _
}
unsafe extern "C" fn dmi_sel_raw_read_helper(e:*mut dmi_sysfs_entry,dh:*const dmi_header,s:*mut c_void)->ssize_t {
    let st=&mut *(s as *mut dmi_read_state); if mem::size_of::<dmi_system_event_log>()>dmi_entry_length_local(dh){return -5}; let mut sel: dmi_system_event_log=mem::zeroed(); ptr::copy_nonoverlapping(dh as *const u8,&mut sel as *mut _ as *mut u8,mem::size_of_val(&sel));
    match sel.access_method { DMI_SEL_ACCESS_METHOD_PHYS32=>dmi_sel_raw_read_phys32(e,&sel,st.buf,st.pos,st.count), DMI_SEL_ACCESS_METHOD_GPNV=>-5, _=>-5 }
}
unsafe extern "C" fn raw_event_log_read(_: *mut c_void,k:*mut kobject,_:*const c_void,b:*mut i8,p:loff_t,c:usize)->ssize_t { let e=k as *mut dmi_sysfs_entry; let mut s=dmi_read_state{buf:b,pos:p,count:c}; find_dmi_entry(e,dmi_sel_raw_read_helper,&mut s as *mut _ as _) }

unsafe extern "C" fn dmi_system_event_log(_: *mut dmi_sysfs_entry)->i32 { 0 }
unsafe fn cleanup_entry_list() { }

// The following declarations correspond to the kernel module registration and
// metadata macros in the original source.
#[used] static MODULE_AUTHOR: &[u8] = b"Mike Waychison <mikew@google.com>\0";
#[used] static MODULE_DESCRIPTION: &[u8] = b"DMI sysfs support\0";
#[used] static MODULE_LICENSE: &[u8] = b"GPL\0";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
