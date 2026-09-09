// SPDX-License-Identifier: GPL-2.0-only
/*
 * ACPI configfs support
 *
 * Copyright (c) 2016 Intel Corporation
 */

// #define pr_fmt(fmt) "ACPI configfs: " fmt
// Linux kernel includes are supplied by other translation units.

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut acpi_table_group: *mut config_group;
}

#[repr(C)]
struct acpi_table {
    cfg: config_item,
    header: *mut acpi_table_header,
    index: u32,
}

unsafe extern "C" {
    fn security_locked_down(reason: c_int) -> c_int;
    fn kmemdup(src: *const c_void, size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn acpi_load_table(table: *mut acpi_table_header, index: *mut u32) -> c_int;
    fn acpi_unload_table(index: u32);
    fn config_item_init_type_name(item: *mut config_item, name: *const c_char, typ: *const config_item_type);
    fn config_item_put(item: *mut config_item);
    fn config_group_init(group: *mut config_group);
    fn configfs_register_subsystem(subsystem: *mut configfs_subsystem) -> c_int;
    fn configfs_unregister_subsystem(subsystem: *mut configfs_subsystem);
    fn configfs_register_default_group(root: *mut config_group, name: *const c_char, typ: *const config_item_type) -> *mut config_group;
    fn configfs_unregister_default_group(group: *mut config_group);
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
}

const LOCKDOWN_ACPI_TABLES: c_int = 0;
const GFP_KERNEL: c_int = 0;
const EBUSY: isize = 16;
const EINVAL: isize = 22;
const ENOMEM: isize = 12;
const MAX_ACPI_TABLE_SIZE: usize = 128 * 1024;
const ACPI_SIG_SSDT: [u8; 4] = *b"SSDT";
const ACPI_NAMESEG_SIZE: usize = 4;
const ACPI_OEM_ID_SIZE: usize = 6;
const ACPI_OEM_TABLE_ID_SIZE: usize = 8;

#[repr(C)] struct config_item { _private: [u8; 0] }
#[repr(C)] struct config_group { cg_item: config_item }
#[repr(C)] struct configfs_bin_attribute { _private: [u8; 0] }
#[repr(C)] struct configfs_attribute { _private: [u8; 0] }
#[repr(C)] struct config_item_type { ct_owner: *mut c_void, ct_bin_attrs: *mut *mut configfs_bin_attribute, ct_attrs: *mut *mut configfs_attribute, ct_group_ops: *const configfs_group_operations }
#[repr(C)] struct configfs_group_operations { make_item: Option<unsafe extern "C" fn(*mut config_group, *const c_char) -> *mut config_item>, drop_item: Option<unsafe extern "C" fn(*mut config_group, *mut config_item)> }
#[repr(C)] struct configfs_subsystem { su_group: config_group, su_mutex: [u8; 40] }
#[repr(C)] struct acpi_table_header { signature: [u8; 4], length: u32, revision: u8, checksum: u8, oem_id: [u8; 6], oem_table_id: [u8; 8], oem_revision: u32, asl_compiler_id: [u8; 4], asl_compiler_revision: u32 }

// CONFIGFS_BIN_ATTR(acpi_table_, aml, NULL, MAX_ACPI_TABLE_SIZE);
// CONFIGFS_ATTR_RO(acpi_table_, signature/length/revision/oem_id/oem_table_id/oem_revision/asl_compiler_id/asl_compiler_revision);
static mut acpi_table_attr_aml: *mut configfs_bin_attribute = core::ptr::null_mut();
static mut acpi_table_bin_attrs: [*mut configfs_bin_attribute; 2] = [core::ptr::null_mut(), core::ptr::null_mut()];

unsafe fn get_header(cfg: *mut config_item) -> *mut acpi_table_header {
    let table = (cfg as *mut u8).sub(0) as *mut acpi_table;
    if (*table).header.is_null() { /* pr_err("table not loaded\n") */ }
    if (*table).header.is_null() { (-EINVAL) as isize as *mut acpi_table_header } else { (*table).header }
}

unsafe extern "C" fn acpi_table_aml_write(cfg: *mut config_item, data: *const c_void, size: usize) -> isize {
    let header = data as *const acpi_table_header;
    let table = cfg as *mut acpi_table;
    let mut ret = security_locked_down(LOCKDOWN_ACPI_TABLES);
    if ret != 0 { return ret as isize; }
    if !(*table).header.is_null() { return -EBUSY; }
    if (*header).length as usize != size { return -EINVAL; }
    if (*header).signature != ACPI_SIG_SSDT { return -EINVAL; }
    (*table).header = kmemdup(data, (*header).length as usize, GFP_KERNEL) as *mut acpi_table_header;
    if (*table).header.is_null() { return -ENOMEM; }
    ret = acpi_load_table((*table).header, &mut (*table).index);
    if ret != 0 { kfree((*table).header as *mut c_void); (*table).header = core::ptr::null_mut(); }
    ret as isize
}

unsafe extern "C" fn acpi_table_aml_read(cfg: *mut config_item, data: *mut c_void, _size: usize) -> isize {
    let h = get_header(cfg);
    if (h as isize) < 0 { return h as isize; }
    if !data.is_null() { core::ptr::copy_nonoverlapping(h as *const u8, data as *mut u8, (*h).length as usize); }
    (*h).length as isize
}

// Attribute show functions retain the source interface and field access.
unsafe extern "C" fn acpi_table_signature_show(cfg: *mut config_item, str_: *mut c_char) -> isize { let h=get_header(cfg); if (h as isize)<0{return h as isize;} sysfs_emit(str_, b"%.*s\0".as_ptr() as _, ACPI_NAMESEG_SIZE as c_int, (*h).signature.as_ptr()) }
unsafe extern "C" fn acpi_table_length_show(cfg: *mut config_item, str_: *mut c_char) -> isize { let h=get_header(cfg); if (h as isize)<0{return h as isize;} sysfs_emit(str_, b"%d\n\0".as_ptr() as _, (*h).length) }
unsafe extern "C" fn acpi_table_revision_show(cfg: *mut config_item, str_: *mut c_char) -> isize { let h=get_header(cfg); if (h as isize)<0{return h as isize;} sysfs_emit(str_, b"%d\n\0".as_ptr() as _, (*h).revision) }
unsafe extern "C" fn acpi_table_oem_id_show(cfg:*mut config_item,s:*mut c_char)->isize { let h=get_header(cfg);if(h as isize)<0{return h as isize;}sysfs_emit(s,b"%.*s\n\0".as_ptr() as _,ACPI_OEM_ID_SIZE as c_int,(*h).oem_id.as_ptr()) }
unsafe extern "C" fn acpi_table_oem_table_id_show(cfg:*mut config_item,s:*mut c_char)->isize { let h=get_header(cfg);if(h as isize)<0{return h as isize;}sysfs_emit(s,b"%.*s\n\0".as_ptr() as _,ACPI_OEM_TABLE_ID_SIZE as c_int,(*h).oem_table_id.as_ptr()) }
unsafe extern "C" fn acpi_table_oem_revision_show(cfg:*mut config_item,s:*mut c_char)->isize { let h=get_header(cfg);if(h as isize)<0{return h as isize;}sysfs_emit(s,b"%d\n\0".as_ptr() as _,(*h).oem_revision) }
unsafe extern "C" fn acpi_table_asl_compiler_id_show(cfg:*mut config_item,s:*mut c_char)->isize { let h=get_header(cfg);if(h as isize)<0{return h as isize;}sysfs_emit(s,b"%.*s\n\0".as_ptr() as _,ACPI_NAMESEG_SIZE as c_int,(*h).asl_compiler_id.as_ptr()) }
unsafe extern "C" fn acpi_table_asl_compiler_revision_show(cfg:*mut config_item,s:*mut c_char)->isize { let h=get_header(cfg);if(h as isize)<0{return h as isize;}sysfs_emit(s,b"%d\n\0".as_ptr() as _,(*h).asl_compiler_revision) }

unsafe extern "C" fn acpi_table_make_item(_group:*mut config_group,name:*const c_char)->*mut config_item { let table=kmemdup(core::ptr::null(),core::mem::size_of::<acpi_table>(),GFP_KERNEL) as *mut acpi_table;if table.is_null(){return (-ENOMEM) as isize as *mut config_item;}config_item_init_type_name(&mut (*table).cfg,name,core::ptr::null());&mut (*table).cfg }
unsafe extern "C" fn acpi_table_drop_item(_group:*mut config_group,cfg:*mut config_item){let table=cfg as *mut acpi_table;acpi_unload_table((*table).index);config_item_put(cfg)}
unsafe extern "C" fn acpi_configfs_init()->c_int { let root=&mut (*(core::ptr::addr_of_mut!(acpi_configfs)));config_group_init(&mut root.su_group);let ret=configfs_register_subsystem(root);if ret!=0{return ret;}acpi_table_group=configfs_register_default_group(&mut root.su_group,b"table\0".as_ptr() as _,core::ptr::null());if(acpi_table_group as isize)<0{configfs_unregister_subsystem(root);return acpi_table_group as isize as c_int;}0 }
unsafe extern "C" fn acpi_configfs_exit(){configfs_unregister_default_group(acpi_table_group);configfs_unregister_subsystem(core::ptr::addr_of_mut!(acpi_configfs));}

static mut acpi_configfs: configfs_subsystem = configfs_subsystem { su_group: config_group { cg_item: config_item { _private: [] } }, su_mutex: [0;40] };

// Remaining CONFIGFS attributes and module registration are provided by the kernel bindings.
// module_init(acpi_configfs_init); module_exit(acpi_configfs_exit);
// MODULE_AUTHOR("Octavian Purdila <octavian.purdila@intel.com>");
// MODULE_DESCRIPTION("ACPI configfs support"); MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
