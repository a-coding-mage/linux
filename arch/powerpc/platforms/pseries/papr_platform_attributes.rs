// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Platform energy and frequency attributes driver
 *
 * This driver creates a sys file at /sys/firmware/papr/ which encapsulates a
 * directory structure containing files in keyword - value pairs that specify
 * energy and frequency configuration of the system.
 *
 * The format of exposing the sysfs information is as follows:
 * /sys/firmware/papr/energy_scale_info/
 *  |-- <id>/
 *    |-- desc
 *    |-- value
 *    |-- value_desc (if exists)
 *  |-- <id>/
 *    |-- desc
 *    |-- value
 *    |-- value_desc (if exists)
 *
 * Copyright 2022 IBM Corp.
 */

// C dependencies from asm/hvcall.h, asm/machdep.h, asm/firmware.h, and pseries.h

const ESI_FLAGS_ALL: u64 = 0;
const ESI_FLAGS_SINGLE: u64 = 1u64 << 63;
const KOBJ_MAX_ATTRS: usize = 3;
const CURR_MAX_ESI_ATTRS: usize = 8;

#[repr(C, packed)]
pub struct energy_scale_attribute {
    pub id: u64,
    pub val: u64,
    pub desc: [u8; 64],
    pub value_desc: [u8; 64],
}

#[repr(C, packed)]
pub struct h_energy_scale_info_hdr {
    pub num_attrs: u64,
    pub array_offset: u64,
    pub data_header_version: u8,
}

#[repr(C)]
pub struct papr_attr {
    pub id: u64,
    pub kobj_attr: kobj_attribute,
}

#[repr(C)]
pub struct papr_group {
    pub pg: attribute_group,
    pub pgattrs: [papr_attr; KOBJ_MAX_ATTRS],
}

// External kernel types and functions supplied by other translation units.
#[repr(C)]
pub struct kobject {
    _private: [u8; 0],
}
#[repr(C)]
pub struct attribute {
    pub name: *const i8,
    pub mode: u16,
}
#[repr(C)]
pub struct kobj_attribute {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *mut i8) -> isize>,
}
#[repr(C)]
pub struct attribute_group {
    pub name: *const i8,
    pub attrs: *mut *mut attribute,
}

extern "C" {
    static mut papr_groups: *mut papr_group;
    static mut papr_kobj: *mut kobject;
    static mut esi_kobj: *mut kobject;
    static mut firmware_kobj: *mut kobject;

    fn kmalloc(size: usize, flags: u32) -> *mut i8;
    fn krealloc(ptr: *mut i8, size: usize, flags: u32) -> *mut i8;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kzalloc_objs<T>(ptr: *mut T, count: u64) -> *mut T;
    fn virt_to_phys(ptr: *mut i8) -> u64;
    fn plpar_hcall_norets(op: u64, flags: u64, id: u64, addr: u64, size: usize) -> i32;
    fn be64_to_cpu(value: u64) -> u64;
    fn firmware_has_feature(feature: u64) -> bool;
    fn sysfs_emit(buf: *mut i8, fmt: *const i8, ...) -> isize;
    fn sysfs_attr_init(attr: *mut attribute);
    fn sysfs_create_group(kobj: *mut kobject, group: *mut attribute_group) -> i32;
    fn kobject_create_and_add(name: *const i8, parent: *mut kobject) -> *mut kobject;
    fn kobject_put(kobj: *mut kobject);
    fn kasprintf(flags: u32, fmt: *const i8, ...) -> *mut i8;
    fn pr_warn(fmt: *const i8, ...);
    fn strcmp(a: *const i8, b: *const i8) -> i32;
}

const GFP_KERNEL: u32 = 0;
const H_GET_ENERGY_SCALE_INFO: u64 = 0;
const H_PARTIAL: i32 = 0;
const H_P4: i32 = 0;
const H_SUCCESS: i32 = 0;
const FW_FEATURE_LPAR: u64 = 0;
const FW_FEATURE_ENERGY_SCALE_INFO: u64 = 0;

static mut OPS_INFO: [papr_ops_info; KOBJ_MAX_ATTRS] = [
    papr_ops_info { attr_name: b"desc\0".as_ptr() as *const i8, show: Some(desc_show) },
    papr_ops_info { attr_name: b"value\0".as_ptr() as *const i8, show: Some(val_show) },
    papr_ops_info { attr_name: b"value_desc\0".as_ptr() as *const i8, show: Some(val_desc_show) },
];

#[repr(C)]
struct papr_ops_info {
    attr_name: *const i8,
    show: Option<unsafe extern "C" fn(*mut kobject, *mut kobj_attribute, *mut i8) -> isize>,
}

unsafe extern "C" fn papr_get_attr(id: u64, esi: *mut energy_scale_attribute) -> i32 {
    let mut esi_buf_size = core::mem::size_of::<h_energy_scale_info_hdr>() + CURR_MAX_ESI_ATTRS * core::mem::size_of::<energy_scale_attribute>();
    let mut max_esi_attrs = CURR_MAX_ESI_ATTRS;
    let mut buf = kmalloc(esi_buf_size, GFP_KERNEL);
    if buf.is_null() { return -12; }
    let ret;
    loop {
        ret = plpar_hcall_norets(H_GET_ENERGY_SCALE_INFO, ESI_FLAGS_SINGLE, id, virt_to_phys(buf), esi_buf_size);
        if ret == H_PARTIAL || ret == H_P4 {
            max_esi_attrs += 4;
            esi_buf_size = core::mem::size_of::<h_energy_scale_info_hdr>() + CURR_MAX_ESI_ATTRS * max_esi_attrs;
            let temp_buf = krealloc(buf, esi_buf_size, GFP_KERNEL);
            if temp_buf.is_null() { kfree(buf as *mut _); return -12; }
            buf = temp_buf;
            continue;
        }
        break;
    }
    if ret != H_SUCCESS { pr_warn(b"hcall failed: H_GET_ENERGY_SCALE_INFO\0".as_ptr() as *const i8); kfree(buf as *mut _); return -5; }
    let hdr = buf as *mut h_energy_scale_info_hdr;
    let offset = be64_to_cpu((*hdr).array_offset) as usize;
    let curr_esi = buf.add(offset) as *mut energy_scale_attribute;
    if esi_buf_size < offset + be64_to_cpu((*hdr).num_attrs) as usize * core::mem::size_of::<energy_scale_attribute>() { kfree(buf as *mut _); return -5; }
    *esi = *curr_esi;
    kfree(buf as *mut _);
    ret
}

unsafe extern "C" fn desc_show(_kobj: *mut kobject, kobj_attr: *mut kobj_attribute, buf: *mut i8) -> isize {
    let pattr = (kobj_attr as *mut u8).sub(core::mem::offset_of!(papr_attr, kobj_attr)) as *mut papr_attr;
    let mut esi = core::mem::zeroed();
    let ret = papr_get_attr((*pattr).id, &mut esi);
    if ret != 0 { return ret as isize; }
    sysfs_emit(buf, b"%s\n\0".as_ptr() as *const i8, esi.desc.as_ptr())
}

unsafe extern "C" fn val_show(_kobj: *mut kobject, kobj_attr: *mut kobj_attribute, buf: *mut i8) -> isize {
    let pattr = (kobj_attr as *mut u8).sub(core::mem::offset_of!(papr_attr, kobj_attr)) as *mut papr_attr;
    let mut esi = core::mem::zeroed();
    let ret = papr_get_attr((*pattr).id, &mut esi);
    if ret != 0 { return ret as isize; }
    sysfs_emit(buf, b"%llu\n\0".as_ptr() as *const i8, be64_to_cpu(esi.val))
}

unsafe extern "C" fn val_desc_show(_kobj: *mut kobject, kobj_attr: *mut kobj_attribute, buf: *mut i8) -> isize {
    let pattr = (kobj_attr as *mut u8).sub(core::mem::offset_of!(papr_attr, kobj_attr)) as *mut papr_attr;
    let mut esi = core::mem::zeroed();
    let ret = papr_get_attr((*pattr).id, &mut esi);
    if ret != 0 { return ret as isize; }
    sysfs_emit(buf, b"%s\n\0".as_ptr() as *const i8, esi.value_desc.as_ptr())
}

unsafe fn add_attr(id: u64, index: usize, attr: *mut papr_attr) {
    (*attr).id = id;
    sysfs_attr_init(&mut (*attr).kobj_attr.attr);
    (*attr).kobj_attr.attr.name = OPS_INFO[index].attr_name;
    (*attr).kobj_attr.attr.mode = 0o444;
    (*attr).kobj_attr.show = OPS_INFO[index].show;
}

unsafe fn add_attr_group(id: u64, pg: *mut papr_group, show_val_desc: bool) -> i32 {
    for i in 0..KOBJ_MAX_ATTRS {
        if strcmp(OPS_INFO[i].attr_name, b"value_desc\0".as_ptr() as *const i8) == 0 && !show_val_desc { continue; }
        add_attr(id, i, &mut (*pg).pgattrs[i]);
        *(*pg).pg.attrs.add(i) = &mut (*pg).pgattrs[i].kobj_attr.attr;
    }
    sysfs_create_group(esi_kobj, &mut (*pg).pg)
}

unsafe extern "C" fn papr_init() -> i32 {
    let mut esi_buf_size = core::mem::size_of::<h_energy_scale_info_hdr>() + CURR_MAX_ESI_ATTRS * core::mem::size_of::<energy_scale_attribute>();
    let mut max_esi_attrs = CURR_MAX_ESI_ATTRS;
    if !firmware_has_feature(FW_FEATURE_LPAR) || !firmware_has_feature(FW_FEATURE_ENERGY_SCALE_INFO) { return -6; }
    let mut esi_buf = kmalloc(esi_buf_size, GFP_KERNEL);
    if esi_buf.is_null() { return -12; }
    let ret;
    loop {
        ret = plpar_hcall_norets(H_GET_ENERGY_SCALE_INFO, ESI_FLAGS_ALL, 0, virt_to_phys(esi_buf), esi_buf_size);
        if ret == H_PARTIAL || ret == H_P4 {
            max_esi_attrs += 4;
            esi_buf_size = core::mem::size_of::<h_energy_scale_info_hdr>() + CURR_MAX_ESI_ATTRS * max_esi_attrs;
            let temp = krealloc(esi_buf, esi_buf_size, GFP_KERNEL);
            if temp.is_null() { kfree(esi_buf as *mut _); return -12; }
            esi_buf = temp;
            continue;
        }
        break;
    }
    if ret != H_SUCCESS { pr_warn(b"hcall failed: H_GET_ENERGY_SCALE_INFO, ret: %d\n\0".as_ptr() as *const i8, ret); kfree(esi_buf as *mut _); return -12; }
    let hdr = esi_buf as *mut h_energy_scale_info_hdr;
    let num_attrs = be64_to_cpu((*hdr).num_attrs);
    let esi_attrs = esi_buf.add(be64_to_cpu((*hdr).array_offset) as usize) as *mut energy_scale_attribute;
    if esi_buf_size < be64_to_cpu((*hdr).array_offset) as usize + num_attrs as usize * core::mem::size_of::<energy_scale_attribute>() { kfree(esi_buf as *mut _); return -12; }
    papr_groups = kzalloc_objs(core::ptr::null_mut(), num_attrs);
    if papr_groups.is_null() { kfree(esi_buf as *mut _); return -12; }
    papr_kobj = kobject_create_and_add(b"papr\0".as_ptr() as *const i8, firmware_kobj);
    if papr_kobj.is_null() { pr_warn(b"kobject_create_and_add papr failed\n\0".as_ptr() as *const i8); kfree(papr_groups as *mut _); kfree(esi_buf as *mut _); return -12; }
    esi_kobj = kobject_create_and_add(b"energy_scale_info\0".as_ptr() as *const i8, papr_kobj);
    if esi_kobj.is_null() { pr_warn(b"kobject_create_and_add energy_scale_info failed\n\0".as_ptr() as *const i8); kobject_put(papr_kobj); kfree(papr_groups as *mut _); kfree(esi_buf as *mut _); return -12; }
    for idx in 0..num_attrs as usize {
        (*papr_groups.add(idx)).pg.attrs = kzalloc_objs(core::ptr::null_mut(), (KOBJ_MAX_ATTRS + 1) as u64);
        if (*papr_groups.add(idx)).pg.attrs.is_null() { kobject_put(esi_kobj); kobject_put(papr_kobj); kfree(papr_groups as *mut _); kfree(esi_buf as *mut _); return -12; }
        (*papr_groups.add(idx)).pg.name = kasprintf(GFP_KERNEL, b"%lld\0".as_ptr() as *const i8, be64_to_cpu((*esi_attrs.add(idx)).id));
        if (*papr_groups.add(idx)).pg.name.is_null() { kobject_put(esi_kobj); kobject_put(papr_kobj); kfree(papr_groups as *mut _); kfree(esi_buf as *mut _); return -12; }
    }
    for idx in 0..num_attrs as usize {
        let show_val_desc = (*esi_attrs.add(idx)).value_desc[0] != 0;
        if add_attr_group(be64_to_cpu((*esi_attrs.add(idx)).id), papr_groups.add(idx), show_val_desc) != 0 { pr_warn(b"Failed to create papr attribute group %s\n\0".as_ptr() as *const i8, (*papr_groups.add(idx)).pg.name); kobject_put(esi_kobj); kobject_put(papr_kobj); kfree(papr_groups as *mut _); kfree(esi_buf as *mut _); return -12; }
    }
    kfree(esi_buf as *mut _);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
