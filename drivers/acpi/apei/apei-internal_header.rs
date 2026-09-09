/* SPDX-License-Identifier: GPL-2.0 */
/*
 * apei-internal.h - ACPI Platform Error Interface internal
 * definitions.
 */

// C dependency: <linux/acpi.h>

use core::ffi::c_char;

#[repr(C)]
pub struct apei_exec_context {
    pub ip: u32,
    pub value: u64,
    pub var1: u64,
    pub var2: u64,
    pub src_base: u64,
    pub dst_base: u64,
    pub ins_table: *mut apei_exec_ins_type,
    pub instructions: u32,
    pub action_table: *mut acpi_whea_header,
    pub entries: u32,
}

#[allow(non_camel_case_types)]
pub type apei_exec_ins_func_t = unsafe extern "C" fn(
    ctx: *mut apei_exec_context,
    entry: *mut acpi_whea_header,
) -> i32;

pub const APEI_EXEC_INS_ACCESS_REGISTER: u32 = 0x0001;

#[repr(C)]
pub struct apei_exec_ins_type {
    pub flags: u32,
    pub run: Option<apei_exec_ins_func_t>,
}

extern "C" {
    pub fn apei_exec_ctx_init(
        ctx: *mut apei_exec_context,
        ins_table: *mut apei_exec_ins_type,
        instructions: u32,
        action_table: *mut acpi_whea_header,
        entries: u32,
    );
}

#[inline]
pub unsafe fn apei_exec_ctx_set_input(ctx: *mut apei_exec_context, input: u64) {
    (*ctx).value = input;
}

#[inline]
pub unsafe fn apei_exec_ctx_get_output(ctx: *mut apei_exec_context) -> u64 {
    (*ctx).value
}

extern "C" {
    pub fn __apei_exec_run(ctx: *mut apei_exec_context, action: u8, optional: bool) -> i32;
}

#[inline]
pub unsafe fn apei_exec_run(ctx: *mut apei_exec_context, action: u8) -> i32 {
    __apei_exec_run(ctx, action, false)
}

/* It is optional whether the firmware provides the action */
#[inline]
pub unsafe fn apei_exec_run_optional(ctx: *mut apei_exec_context, action: u8) -> i32 {
    __apei_exec_run(ctx, action, true)
}

/* Common instruction implementation */

/* IP has been set in instruction function */
pub const APEI_EXEC_SET_IP: u32 = 1;

extern "C" {
    pub fn apei_map_generic_address(reg: *mut acpi_generic_address) -> i32;
}

#[inline]
pub unsafe fn apei_unmap_generic_address(reg: *mut acpi_generic_address) {
    acpi_os_unmap_generic_address(reg);
}

extern "C" {
    pub fn apei_read(val: *mut u64, reg: *mut acpi_generic_address) -> i32;
    pub fn apei_write(val: u64, reg: *mut acpi_generic_address) -> i32;
    pub fn __apei_exec_read_register(entry: *mut acpi_whea_header, val: *mut u64) -> i32;
    pub fn __apei_exec_write_register(entry: *mut acpi_whea_header, val: u64) -> i32;
    pub fn apei_exec_read_register(ctx: *mut apei_exec_context, entry: *mut acpi_whea_header) -> i32;
    pub fn apei_exec_read_register_value(ctx: *mut apei_exec_context, entry: *mut acpi_whea_header) -> i32;
    pub fn apei_exec_write_register(ctx: *mut apei_exec_context, entry: *mut acpi_whea_header) -> i32;
    pub fn apei_exec_write_register_value(ctx: *mut apei_exec_context, entry: *mut acpi_whea_header) -> i32;
    pub fn apei_exec_noop(ctx: *mut apei_exec_context, entry: *mut acpi_whea_header) -> i32;
    pub fn apei_exec_pre_map_gars(ctx: *mut apei_exec_context) -> i32;
    pub fn apei_exec_post_unmap_gars(ctx: *mut apei_exec_context) -> i32;
}

#[repr(C)]
pub struct apei_resources {
    pub iomem: list_head,
    pub ioport: list_head,
}

#[inline]
pub unsafe fn apei_resources_init(resources: *mut apei_resources) {
    INIT_LIST_HEAD(&mut (*resources).iomem);
    INIT_LIST_HEAD(&mut (*resources).ioport);
}

extern "C" {
    pub fn apei_resources_fini(resources: *mut apei_resources);
    pub fn apei_resources_add(resources: *mut apei_resources, start: c_ulong, size: c_ulong, iomem: bool) -> i32;
    pub fn apei_resources_sub(resources1: *mut apei_resources, resources2: *mut apei_resources) -> i32;
    pub fn apei_resources_request(resources: *mut apei_resources, desc: *const c_char) -> i32;
    pub fn apei_resources_release(resources: *mut apei_resources);
    pub fn apei_exec_collect_resources(ctx: *mut apei_exec_context, resources: *mut apei_resources) -> i32;
}

pub enum dentry {}

extern "C" {
    pub fn apei_get_debugfs_dir() -> *mut dentry;
}

#[inline]
pub unsafe fn cper_estatus_len(estatus: *const acpi_hest_generic_status) -> u32 {
    if (*estatus).raw_data_length != 0 {
        (*estatus).raw_data_offset.wrapping_add((*estatus).raw_data_length)
    } else {
        core::mem::size_of::<acpi_hest_generic_status>() as u32 + (*estatus).data_length
    }
}

extern "C" {
    pub fn apei_osc_setup() -> i32;
    pub fn einj_get_available_error_type(type_: *mut u32, einj_action: i32) -> i32;
    pub fn einj_error_inject(type_: u32, flags: u32, param1: u64, param2: u64, param3: u64, param4: u64) -> i32;
    pub fn einj_cxl_rch_error_inject(type_: u32, flags: u32, param1: u64, param2: u64, param3: u64, param4: u64) -> i32;
    pub fn einj_is_cxl_error_type(type_: u64) -> bool;
    pub fn einj_validate_error_type(type_: u64) -> i32;
}

// Preserved from the conditional C definitions; BIT(n) is the Linux bit mask macro.
pub const ACPI_EINJ_CXL_CACHE_CORRECTABLE: u32 = 1u32 << 12;
pub const ACPI_EINJ_CXL_CACHE_UNCORRECTABLE: u32 = 1u32 << 13;
pub const ACPI_EINJ_CXL_CACHE_FATAL: u32 = 1u32 << 14;
pub const ACPI_EINJ_CXL_MEM_CORRECTABLE: u32 = 1u32 << 15;
pub const ACPI_EINJ_CXL_MEM_UNCORRECTABLE: u32 = 1u32 << 16;
pub const ACPI_EINJ_CXL_MEM_FATAL: u32 = 1u32 << 17;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
