// SPDX-License-Identifier: GPL-2.0-only

/*
 * FPDT support for exporting boot and suspend/resume performance data
 *
 * Copyright (C) 2021 Intel Corporation. All rights reserved.
 */

// Dependency supplied by the Linux kernel: <linux/acpi.h>

/*
 * FPDT contains ACPI table header and a number of fpdt_subtable_entries.
 * Each fpdt_subtable_entry points to a subtable: FBPT or S3PT.
 * Each FPDT subtable (FBPT/S3PT) is composed of a fpdt_subtable_header
 * and a number of fpdt performance records.
 * Each FPDT performance record is composed of a fpdt_record_header and
 * performance data fields, for boot or suspend or resume phase.
 */
#[repr(u32)]
enum FpdtSubtableType {
    SUBTABLE_FBPT,
    SUBTABLE_S3PT,
}

#[repr(C)]
struct FpdtSubtableEntry {
    type_: u16,
    length: u8,
    revision: u8,
    reserved: u32,
    address: u64,
}

#[repr(C)]
struct FpdtSubtableHeader {
    signature: u32,
    length: u32,
}

#[repr(u32)]
enum FpdtRecordType {
    RECORD_S3_RESUME,
    RECORD_S3_SUSPEND,
    RECORD_BOOT,
}

#[repr(C)]
struct FpdtRecordHeader {
    type_: u16,
    length: u8,
    revision: u8,
}

#[repr(C, packed)]
struct ResumePerformanceRecord {
    header: FpdtRecordHeader,
    resume_count: u32,
    resume_prev: u64,
    resume_avg: u64,
}

#[repr(C, packed)]
struct BootPerformanceRecord {
    header: FpdtRecordHeader,
    reserved: u32,
    firmware_start: u64,
    bootloader_load: u64,
    bootloader_launch: u64,
    exitbootservice_start: u64,
    exitbootservice_end: u64,
}

#[repr(C, packed)]
struct SuspendPerformanceRecord {
    header: FpdtRecordHeader,
    suspend_start: u64,
    suspend_end: u64,
}

static mut record_resume: *mut ResumePerformanceRecord = core::ptr::null_mut();
static mut record_suspend: *mut SuspendPerformanceRecord = core::ptr::null_mut();
static mut record_boot: *mut BootPerformanceRecord = core::ptr::null_mut();

// FPDT_ATTR generates the sysfs show function and read-only attribute.
macro_rules! FPDT_ATTR {
    ($phase:ident, $name:ident) => {
        unsafe extern "C" fn $name##_show(
            _kobj: *mut Kobject,
            _attr: *mut KobjAttribute,
            buf: *mut core::ffi::c_char,
        ) -> isize {
            sprintf_u64(buf, (*$crate::record_$phase).$name)
        }
        static mut $name##_attr: KobjAttribute = __ATTR!($name##_ns, 0o444, $name##_show, None);
    };
}

FPDT_ATTR!(resume, resume_prev);
FPDT_ATTR!(resume, resume_avg);
FPDT_ATTR!(suspend, suspend_start);
FPDT_ATTR!(suspend, suspend_end);
FPDT_ATTR!(boot, firmware_start);
FPDT_ATTR!(boot, bootloader_load);
FPDT_ATTR!(boot, bootloader_launch);
FPDT_ATTR!(boot, exitbootservice_start);
FPDT_ATTR!(boot, exitbootservice_end);

unsafe extern "C" fn resume_count_show(
    _kobj: *mut Kobject,
    _attr: *mut KobjAttribute,
    buf: *mut core::ffi::c_char,
) -> isize {
    sprintf_u32(buf, (*record_resume).resume_count)
}

static mut resume_count_attr: KobjAttribute = __ATTR_RO!(resume_count);

static mut resume_attrs: [*mut Attribute; 4] = [
    &mut resume_count_attr.attr,
    &mut resume_prev_attr.attr,
    &mut resume_avg_attr.attr,
    core::ptr::null_mut(),
];
static resume_attr_group: AttributeGroup = AttributeGroup { attrs: resume_attrs.as_mut_ptr(), name: "resume" };

static mut suspend_attrs: [*mut Attribute; 3] = [
    &mut suspend_start_attr.attr,
    &mut suspend_end_attr.attr,
    core::ptr::null_mut(),
];
static suspend_attr_group: AttributeGroup = AttributeGroup { attrs: suspend_attrs.as_mut_ptr(), name: "suspend" };

static mut boot_attrs: [*mut Attribute; 6] = [
    &mut firmware_start_attr.attr,
    &mut bootloader_load_attr.attr,
    &mut bootloader_launch_attr.attr,
    &mut exitbootservice_start_attr.attr,
    &mut exitbootservice_end_attr.attr,
    core::ptr::null_mut(),
];
static boot_attr_group: AttributeGroup = AttributeGroup { attrs: boot_attrs.as_mut_ptr(), name: "boot" };

static mut bin_attr_FBPT: BinAttribute = BIN_ATTR!(FBPT, 0o400, sysfs_bin_attr_simple_read, None, 0);
static mut bin_attr_S3PT: BinAttribute = BIN_ATTR!(S3PT, 0o400, sysfs_bin_attr_simple_read, None, 0);
static mut fpdt_kobj: *mut Kobject = core::ptr::null_mut();

#[cfg(all(CONFIG_X86, CONFIG_PHYS_ADDR_T_64BIT))]
unsafe fn fpdt_address_valid(address: u64) -> bool {
    !(address >> boot_cpu_data.x86_phys_bits)
}

#[cfg(not(all(CONFIG_X86, CONFIG_PHYS_ADDR_T_64BIT)))]
unsafe fn fpdt_address_valid(_address: u64) -> bool { true }

unsafe fn fpdt_process_subtable(address: u64, subtable_type: u32) -> i32 {
    let signature = if subtable_type == SUBTABLE_FBPT as u32 { b"FBPT" } else { b"S3PT" };
    if !fpdt_address_valid(address) { pr_info!("invalid physical address: 0x{:x}!\n", address); return -EINVAL; }
    let mut subtable_header = acpi_os_map_memory(address, core::mem::size_of::<FpdtSubtableHeader>()) as *mut FpdtSubtableHeader;
    if subtable_header.is_null() { return -ENOMEM; }
    if core::slice::from_raw_parts((&(*subtable_header).signature as *const u32) as *const u8, 4) != signature {
        pr_info!("subtable signature and type mismatch!\n"); acpi_os_unmap_memory(subtable_header as *mut _, core::mem::size_of::<FpdtSubtableHeader>()); return -EINVAL;
    }
    let length = (*subtable_header).length as usize;
    if length < core::mem::size_of::<FpdtSubtableHeader>() { pr_err!("Invalid FPDT subtable length {}.\n", length); acpi_os_unmap_memory(subtable_header as *mut _, core::mem::size_of::<FpdtSubtableHeader>()); return -EINVAL; }
    acpi_os_unmap_memory(subtable_header as *mut _, core::mem::size_of::<FpdtSubtableHeader>());
    subtable_header = acpi_os_map_memory(address, length) as *mut FpdtSubtableHeader;
    if subtable_header.is_null() { return -ENOMEM; }
    let mut offset = core::mem::size_of::<FpdtSubtableHeader>();
    while offset < length {
        let remaining = length - offset;
        if remaining < core::mem::size_of::<FpdtRecordHeader>() { pr_err!("Truncated FPDT record header.\n"); return fpdt_process_error(); }
        let record_header = (subtable_header as *mut u8).add(offset) as *mut FpdtRecordHeader;
        let record_length = (*record_header).length as usize;
        if record_length < core::mem::size_of::<FpdtRecordHeader>() || record_length > remaining { pr_err!("Invalid FPDT record length {}.\n", record_length); return fpdt_process_error(); }
        offset += record_length;
        match (*record_header).type_ as u32 {
            x if x == RECORD_S3_RESUME as u32 => { if record_length < core::mem::size_of::<ResumePerformanceRecord>() || subtable_type != SUBTABLE_S3PT as u32 { return fpdt_process_error(); } if !record_resume.is_null() { pr_err!("Duplicate resume performance record found.\n"); continue; } record_resume = record_header as *mut _; if sysfs_create_group(fpdt_kobj, &resume_attr_group) != 0 { return fpdt_process_error(); } }
            x if x == RECORD_S3_SUSPEND as u32 => { if record_length < core::mem::size_of::<SuspendPerformanceRecord>() || subtable_type != SUBTABLE_S3PT as u32 { return fpdt_process_error(); } if !record_suspend.is_null() { pr_err!("Duplicate suspend performance record found.\n"); continue; } record_suspend = record_header as *mut _; if sysfs_create_group(fpdt_kobj, &suspend_attr_group) != 0 { return fpdt_process_error(); } }
            x if x == RECORD_BOOT as u32 => { if record_length < core::mem::size_of::<BootPerformanceRecord>() || subtable_type != SUBTABLE_FBPT as u32 { return fpdt_process_error(); } if !record_boot.is_null() { pr_err!("Duplicate boot performance record found.\n"); continue; } record_boot = record_header as *mut _; if sysfs_create_group(fpdt_kobj, &boot_attr_group) != 0 { return fpdt_process_error(); } }
            _ => {},
        }
    }
    if subtable_type == SUBTABLE_FBPT as u32 { bin_attr_FBPT.private_ = subtable_header as *mut _; bin_attr_FBPT.size = length; let _ = sysfs_create_bin_file(fpdt_kobj, &bin_attr_FBPT); }
    else if subtable_type == SUBTABLE_S3PT as u32 { bin_attr_S3PT.private_ = subtable_header as *mut _; bin_attr_S3PT.size = length; let _ = sysfs_create_bin_file(fpdt_kobj, &bin_attr_S3PT); }
    0
}

unsafe fn fpdt_process_error() -> i32 {
    if !bin_attr_FBPT.private_.is_null() { sysfs_remove_bin_file(fpdt_kobj, &bin_attr_FBPT); bin_attr_FBPT.private_ = core::ptr::null_mut(); }
    if !bin_attr_S3PT.private_.is_null() { sysfs_remove_bin_file(fpdt_kobj, &bin_attr_S3PT); bin_attr_S3PT.private_ = core::ptr::null_mut(); }
    if !record_boot.is_null() { sysfs_remove_group(fpdt_kobj, &boot_attr_group); }
    if !record_suspend.is_null() { sysfs_remove_group(fpdt_kobj, &suspend_attr_group); }
    if !record_resume.is_null() { sysfs_remove_group(fpdt_kobj, &resume_attr_group); }
    -EINVAL
}

unsafe extern "C" fn acpi_init_fpdt() -> i32 {
    let mut header: *mut AcpiTableHeader = core::ptr::null_mut();
    let mut offset = core::mem::size_of::<AcpiTableHeader>();
    let status = acpi_get_table(ACPI_SIG_FPDT, 0, &mut header);
    if ACPI_FAILURE(status) { return 0; }
    fpdt_kobj = kobject_create_and_add(b"fpdt\\0".as_ptr() as *const _, acpi_kobj);
    if fpdt_kobj.is_null() { acpi_put_table(header); return -ENOMEM; }
    while offset < (*header).length as usize {
        if (*header).length as usize - offset < core::mem::size_of::<FpdtSubtableEntry>() { pr_err!("Truncated FPDT subtable entry.\n"); kobject_put(fpdt_kobj); acpi_put_table(header); return -EINVAL; }
        let subtable = (header as *mut u8).add(offset) as *mut FpdtSubtableEntry;
        let subtable_length = (*subtable).length as usize;
        if subtable_length < core::mem::size_of::<FpdtSubtableEntry>() || subtable_length > (*header).length as usize - offset { pr_err!("Invalid FPDT subtable entry length {}.\n", subtable_length); kobject_put(fpdt_kobj); acpi_put_table(header); return -EINVAL; }
        match (*subtable).type_ as u32 {
            x if x == SUBTABLE_FBPT as u32 || x == SUBTABLE_S3PT as u32 => { let result = fpdt_process_subtable((*subtable).address, (*subtable).type_ as u32); if result != 0 { kobject_put(fpdt_kobj); acpi_put_table(header); return result; } }
            _ => {},
        }
        offset += subtable_length;
    }
    0
}

// Equivalent of fs_initcall(acpi_init_fpdt).
fs_initcall!(acpi_init_fpdt);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
