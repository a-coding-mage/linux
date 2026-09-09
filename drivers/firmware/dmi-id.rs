// SPDX-License-Identifier: GPL-2.0-only
/*
 * Export SMBIOS/DMI info via sysfs to userspace
 *
 * Copyright 2007, Lennart Poettering
 */

// C kernel dependencies are supplied by the surrounding kernel translation.
use core::{ffi::c_char, mem::size_of, ptr};

#[repr(C)]
pub struct DmiDeviceAttribute {
    pub dev_attr: DeviceAttribute,
    pub field: i32,
}

#[repr(C)] pub struct DeviceAttribute { pub attr: Attribute, pub show: Option<unsafe extern "C" fn(*mut Device, *mut DeviceAttribute, *mut c_char) -> isize>, pub store: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct Attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct Device { pub class: *mut Class, pub groups: *const *const AttributeGroup }
#[repr(C)] pub struct AttributeGroup { pub attrs: *mut *mut Attribute }
#[repr(C)] pub struct Class { pub name: *const c_char, pub dev_release: Option<unsafe extern "C" fn(*mut Device)>, pub dev_uevent: Option<unsafe extern "C" fn(*const Device, *mut KobjUeventEnv) -> i32> }
#[repr(C)] pub struct KobjUeventEnv { pub buf: [c_char; 2048], pub buflen: usize }

extern "C" {
    static dmi_available: bool;
    fn dmi_get_system_info(field: i32) -> *const c_char;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> isize;
    fn kmalloc(size: usize, flags: u32) -> *mut c_char;
    fn kfree(p: *mut c_char);
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn add_uevent_var(env: *mut KobjUeventEnv, fmt: *const c_char, ...) -> i32;
    fn class_register(class: *mut Class) -> i32;
    fn class_unregister(class: *mut Class);
    fn device_register(dev: *mut Device) -> i32;
    fn put_device(dev: *mut Device);
    fn dev_set_name(dev: *mut Device, name: *const c_char) -> i32;
    fn kzalloc(size: usize, flags: u32) -> *mut Device;
}

const PAGE_SIZE: usize = 4096;
const GFP_KERNEL: u32 = 0;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;
const DMI_NONE: i32 = -1;
const DMI_BIOS_VENDOR: i32 = 0; const DMI_BIOS_VERSION: i32 = 1; const DMI_BIOS_DATE: i32 = 2;
const DMI_SYS_VENDOR: i32 = 3; const DMI_BIOS_RELEASE: i32 = 4; const DMI_EC_FIRMWARE_RELEASE: i32 = 5;
const DMI_PRODUCT_NAME: i32 = 6; const DMI_PRODUCT_VERSION: i32 = 7; const DMI_PRODUCT_SERIAL: i32 = 8;
const DMI_PRODUCT_UUID: i32 = 9; const DMI_PRODUCT_SKU: i32 = 10; const DMI_PRODUCT_FAMILY: i32 = 11;
const DMI_BOARD_VENDOR: i32 = 12; const DMI_BOARD_NAME: i32 = 13; const DMI_BOARD_VERSION: i32 = 14;
const DMI_BOARD_SERIAL: i32 = 15; const DMI_BOARD_ASSET_TAG: i32 = 16; const DMI_CHASSIS_VENDOR: i32 = 17;
const DMI_CHASSIS_TYPE: i32 = 18; const DMI_CHASSIS_VERSION: i32 = 19; const DMI_CHASSIS_SERIAL: i32 = 20;
const DMI_CHASSIS_ASSET_TAG: i32 = 21; const DMI_STRING_MAX: usize = 22;

unsafe fn ascii_filter(mut d: *mut c_char, mut s: *const c_char) {
    // Filter out characters we don't want to see in the modalias string
    while *s != 0 { let c = *s as u8; if c > b' ' && c < 127 && c != b':' { *d = *s; d = d.add(1); } s = s.add(1); }
    *d = 0;
}

unsafe extern "C" fn sys_dmi_field_show(_dev: *mut Device, attr: *mut DeviceAttribute, page: *mut c_char) -> isize {
    let field = (*(attr as *mut DmiDeviceAttribute)).field;
    let len = scnprintf(page, PAGE_SIZE, b"%s\n\0".as_ptr() as *const c_char, dmi_get_system_info(field));
    page.offset(len - 1).write(b'\n' as c_char); len
}

unsafe extern "C" fn dmi_dev_release(dev: *mut Device) { kfree(dev as *mut c_char); }
unsafe extern "C" fn dmi_dev_uevent(_dev: *const Device, env: *mut KobjUeventEnv) -> i32 {
    if add_uevent_var(env, b"MODALIAS=\0".as_ptr() as *const c_char) != 0 { return -ENOMEM; }
    let remaining = size_of::<KobjUeventEnv>() - (*env).buflen;
    let len = get_modalias((*env).buf.as_mut_ptr().add((*env).buflen - 1), remaining);
    if (len as usize) >= remaining { return -ENOMEM; } (*env).buflen += len as usize; 0
}

unsafe fn get_modalias(buffer: *mut c_char, buffer_size: usize) -> isize {
    // Note new fields need to be added at the end to keep compatibility with udev's hwdb.
    #[repr(C)] struct MaField { prefix: *const c_char, field: i32 }
    static FIELDS: [MaField; 17] = [
        MaField { prefix: b"bvn\0".as_ptr() as *const c_char, field: DMI_BIOS_VENDOR }, MaField { prefix: b"bvr\0".as_ptr() as *const c_char, field: DMI_BIOS_VERSION },
        MaField { prefix: b"bd\0".as_ptr() as *const c_char, field: DMI_BIOS_DATE }, MaField { prefix: b"br\0".as_ptr() as *const c_char, field: DMI_BIOS_RELEASE },
        MaField { prefix: b"efr\0".as_ptr() as *const c_char, field: DMI_EC_FIRMWARE_RELEASE }, MaField { prefix: b"svn\0".as_ptr() as *const c_char, field: DMI_SYS_VENDOR },
        MaField { prefix: b"pn\0".as_ptr() as *const c_char, field: DMI_PRODUCT_NAME }, MaField { prefix: b"pvr\0".as_ptr() as *const c_char, field: DMI_PRODUCT_VERSION },
        MaField { prefix: b"rvn\0".as_ptr() as *const c_char, field: DMI_BOARD_VENDOR }, MaField { prefix: b"rn\0".as_ptr() as *const c_char, field: DMI_BOARD_NAME },
        MaField { prefix: b"rvr\0".as_ptr() as *const c_char, field: DMI_BOARD_VERSION }, MaField { prefix: b"cvn\0".as_ptr() as *const c_char, field: DMI_CHASSIS_VENDOR },
        MaField { prefix: b"ct\0".as_ptr() as *const c_char, field: DMI_CHASSIS_TYPE }, MaField { prefix: b"cvr\0".as_ptr() as *const c_char, field: DMI_CHASSIS_VERSION },
        MaField { prefix: b"sku\0".as_ptr() as *const c_char, field: DMI_PRODUCT_SKU }, MaField { prefix: b"pfa\0".as_ptr() as *const c_char, field: DMI_PRODUCT_FAMILY },
        MaField { prefix: ptr::null(), field: DMI_NONE } ];
    strcpy(buffer, b"dmi\0".as_ptr() as *const c_char);
    let mut p = buffer.add(3); let mut left = buffer_size.wrapping_sub(4);
    for f in FIELDS.iter() { if f.prefix.is_null() || left <= 0 { break; } let c = dmi_get_system_info(f.field); if c.is_null() { continue; }
        let t = kmalloc(strlen(c) + 1, GFP_KERNEL); if t.is_null() { break; } ascii_filter(t, c);
        let l = scnprintf(p, left, b":%s%s\0".as_ptr() as *const c_char, f.prefix, t); kfree(t); p = p.offset(l); left = left.wrapping_sub(l as usize); }
    p.write(b':' as c_char); p.add(1).write(0); p.offset_from(buffer) + 1
}

unsafe extern "C" fn sys_dmi_modalias_show(_dev: *mut Device, _attr: *mut DeviceAttribute, page: *mut c_char) -> isize {
    let r = get_modalias(page, PAGE_SIZE - 1); page.offset(r).write(b'\n' as c_char); page.offset(r + 1).write(0); r + 1
}

static mut SYS_DMI_ATTRIBUTES: [*mut Attribute; DMI_STRING_MAX + 2] = [ptr::null_mut(); DMI_STRING_MAX + 2];
static mut SYS_DMI_ATTRIBUTE_GROUP: AttributeGroup = AttributeGroup { attrs: SYS_DMI_ATTRIBUTES.as_mut_ptr() };
static mut SYS_DMI_ATTRIBUTE_GROUPS: [*const AttributeGroup; 2] = [ptr::addr_of!(SYS_DMI_ATTRIBUTE_GROUP), ptr::null()];
static mut DMI_CLASS: Class = Class { name: b"dmi\0".as_ptr() as *const c_char, dev_release: Some(dmi_dev_release), dev_uevent: Some(dmi_dev_uevent) };
static mut DMI_DEV: *mut Device = ptr::null_mut();

unsafe extern "C" fn dmi_id_init() -> i32 {
    if !dmi_available { return -ENODEV; }
    let dev = kzalloc(size_of::<Device>(), GFP_KERNEL); if dev.is_null() { class_unregister(&mut DMI_CLASS); return -ENOMEM; }
    DMI_DEV = dev; (*dev).class = &mut DMI_CLASS; dev_set_name(dev, b"id\0".as_ptr() as *const c_char); (*dev).groups = SYS_DMI_ATTRIBUTE_GROUPS.as_ptr();
    let ret = device_register(dev); if ret != 0 { put_device(dev); class_unregister(&mut DMI_CLASS); } ret
}

// The C ADD_DMI_ATTR macro conditionally appends each available DMI attribute;
// the generated per-field attribute objects use sys_dmi_field_show and the
// corresponding DMI_* field constants.

// arch_initcall(dmi_id_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
