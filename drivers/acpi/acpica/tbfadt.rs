// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// FADT table utilities

use core::{mem, ptr};

// Dependencies supplied by the ACPICA translation environment.

const ACPI_FADT_OPTIONAL: u8 = 0;
const ACPI_FADT_REQUIRED: u8 = 1;
const ACPI_FADT_SEPARATE_LENGTH: u8 = 2;
const ACPI_FADT_GPE_REGISTER: u8 = 4;

#[repr(C)]
pub struct AcpiFadtInfo {
    pub name: *const core::ffi::c_char,
    pub address64: u16,
    pub address32: u16,
    pub length: u16,
    pub default_length: u8,
    pub flags: u8,
}

#[repr(C)]
pub struct AcpiFadtPmInfo {
    pub target: *mut AcpiGenericAddress,
    pub source: u16,
    pub register_num: u8,
}

extern "C" {
    pub static mut acpi_gbl_xpm1a_status: AcpiGenericAddress;
    pub static mut acpi_gbl_xpm1a_enable: AcpiGenericAddress;
    pub static mut acpi_gbl_xpm1b_status: AcpiGenericAddress;
    pub static mut acpi_gbl_xpm1b_enable: AcpiGenericAddress;
    pub static mut acpi_gbl_root_table_list: AcpiRootTableList;
    pub static mut acpi_gbl_fadt_index: u32;
    pub static mut acpi_gbl_dsdt_index: u32;
    pub static mut acpi_gbl_facs_index: u32;
    pub static mut acpi_gbl_xfacs_index: u32;
    pub static mut acpi_gbl_FADT: AcpiTableFadt;
    pub static mut acpi_gbl_reduced_hardware: u8;
    pub static mut acpi_gbl_use32_bit_fadt_addresses: u8;
    pub static mut acpi_gbl_use_default_register_widths: u8;

    fn acpi_tb_get_table(desc: *mut AcpiTableDesc, table: *mut *mut AcpiTableHeader) -> AcpiStatus;
    fn acpi_ut_verify_checksum(table: *mut AcpiTableHeader, length: u32) -> AcpiStatus;
    fn acpi_tb_put_table(desc: *mut AcpiTableDesc);
    fn acpi_tb_install_standard_table(address: AcpiPhysicalAddress, origin: u32, arg: *mut core::ffi::c_void, a: u8, b: u8, index: *mut u32);
}

// The following ACPICA types and constants are supplied by included headers.
use crate::*;

static mut FADT_INFO_TABLE: [AcpiFadtInfo; 8] = [
    AcpiFadtInfo { name: b"Pm1aEventBlock\0" as *const u8 as _, address64: ACPI_FADT_OFFSET!(xpm1a_event_block), address32: ACPI_FADT_OFFSET!(pm1a_event_block), length: ACPI_FADT_OFFSET!(pm1_event_length), default_length: ACPI_PM1_REGISTER_WIDTH * 2, flags: ACPI_FADT_REQUIRED },
    AcpiFadtInfo { name: b"Pm1bEventBlock\0" as *const u8 as _, address64: ACPI_FADT_OFFSET!(xpm1b_event_block), address32: ACPI_FADT_OFFSET!(pm1b_event_block), length: ACPI_FADT_OFFSET!(pm1_event_length), default_length: ACPI_PM1_REGISTER_WIDTH * 2, flags: ACPI_FADT_OPTIONAL },
    AcpiFadtInfo { name: b"Pm1aControlBlock\0" as *const u8 as _, address64: ACPI_FADT_OFFSET!(xpm1a_control_block), address32: ACPI_FADT_OFFSET!(pm1a_control_block), length: ACPI_FADT_OFFSET!(pm1_control_length), default_length: ACPI_PM1_REGISTER_WIDTH, flags: ACPI_FADT_REQUIRED },
    AcpiFadtInfo { name: b"Pm1bControlBlock\0" as *const u8 as _, address64: ACPI_FADT_OFFSET!(xpm1b_control_block), address32: ACPI_FADT_OFFSET!(pm1b_control_block), length: ACPI_FADT_OFFSET!(pm1_control_length), default_length: ACPI_PM1_REGISTER_WIDTH, flags: ACPI_FADT_OPTIONAL },
    AcpiFadtInfo { name: b"Pm2ControlBlock\0" as *const u8 as _, address64: ACPI_FADT_OFFSET!(xpm2_control_block), address32: ACPI_FADT_OFFSET!(pm2_control_block), length: ACPI_FADT_OFFSET!(pm2_control_length), default_length: ACPI_PM2_REGISTER_WIDTH, flags: ACPI_FADT_SEPARATE_LENGTH },
    AcpiFadtInfo { name: b"PmTimerBlock\0" as *const u8 as _, address64: ACPI_FADT_OFFSET!(xpm_timer_block), address32: ACPI_FADT_OFFSET!(pm_timer_block), length: ACPI_FADT_OFFSET!(pm_timer_length), default_length: ACPI_PM_TIMER_WIDTH, flags: ACPI_FADT_SEPARATE_LENGTH },
    AcpiFadtInfo { name: b"Gpe0Block\0" as *const u8 as _, address64: ACPI_FADT_OFFSET!(xgpe0_block), address32: ACPI_FADT_OFFSET!(gpe0_block), length: ACPI_FADT_OFFSET!(gpe0_block_length), default_length: 0, flags: ACPI_FADT_SEPARATE_LENGTH | ACPI_FADT_GPE_REGISTER },
    AcpiFadtInfo { name: b"Gpe1Block\0" as *const u8 as _, address64: ACPI_FADT_OFFSET!(xgpe1_block), address32: ACPI_FADT_OFFSET!(gpe1_block), length: ACPI_FADT_OFFSET!(gpe1_block_length), default_length: 0, flags: ACPI_FADT_SEPARATE_LENGTH | ACPI_FADT_GPE_REGISTER },
];

static mut FADT_PM_INFO_TABLE: [AcpiFadtPmInfo; 4] = [
    AcpiFadtPmInfo { target: unsafe { &raw mut acpi_gbl_xpm1a_status }, source: ACPI_FADT_OFFSET!(xpm1a_event_block), register_num: 0 },
    AcpiFadtPmInfo { target: unsafe { &raw mut acpi_gbl_xpm1a_enable }, source: ACPI_FADT_OFFSET!(xpm1a_event_block), register_num: 1 },
    AcpiFadtPmInfo { target: unsafe { &raw mut acpi_gbl_xpm1b_status }, source: ACPI_FADT_OFFSET!(xpm1b_event_block), register_num: 0 },
    AcpiFadtPmInfo { target: unsafe { &raw mut acpi_gbl_xpm1b_enable }, source: ACPI_FADT_OFFSET!(xpm1b_event_block), register_num: 1 },
];

unsafe fn acpi_tb_init_generic_address(g: *mut AcpiGenericAddress, space_id: u8, byte_width: u8, address: u64, register_name: *const core::ffi::c_char, flags: u8) {
    let mut bit_width = byte_width.wrapping_mul(8);
    if byte_width > 31 {
        if flags & ACPI_FADT_GPE_REGISTER == 0 { ACPI_ERROR!(AE_INFO, "%s - 32-bit FADT register is too long", register_name, byte_width, byte_width as u32 * 8); }
        bit_width = 255;
    }
    (*g).address = address;
    (*g).space_id = space_id;
    (*g).bit_width = bit_width;
    (*g).bit_offset = 0;
    (*g).access_width = 0;
}

unsafe fn acpi_tb_select_address(register_name: *mut core::ffi::c_char, address32: u32, address64: u64) -> u64 {
    if address64 == 0 { return address32 as u64; }
    if address32 != 0 && address64 != address32 as u64 {
        ACPI_BIOS_WARNING!(AE_INFO, "32/64X %s address mismatch in FADT", register_name, address32, address64, if acpi_gbl_use32_bit_fadt_addresses != 0 { 32 } else { 64 });
        if acpi_gbl_use32_bit_fadt_addresses != 0 { return address32 as u64; }
    }
    address64
}

pub unsafe fn acpi_tb_parse_fadt() {
    let desc = &mut acpi_gbl_root_table_list.tables[acpi_gbl_fadt_index as usize] as *mut _;
    let mut table = ptr::null_mut();
    let status = acpi_tb_get_table(desc, &mut table);
    if ACPI_FAILURE!(status) { return; }
    let length = (*desc).length;
    let _ = acpi_ut_verify_checksum(table, length);
    acpi_tb_create_local_fadt(table, length);
    acpi_tb_put_table(desc);
    acpi_tb_install_standard_table(acpi_gbl_FADT.Xdsdt as AcpiPhysicalAddress, ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL, ptr::null_mut(), 0, 1, &mut acpi_gbl_dsdt_index);
    if acpi_gbl_FADT.facs != 0 { acpi_tb_install_standard_table(acpi_gbl_FADT.facs as AcpiPhysicalAddress, ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL, ptr::null_mut(), 0, 1, &mut acpi_gbl_facs_index); }
    if acpi_gbl_FADT.Xfacs != 0 { acpi_tb_install_standard_table(acpi_gbl_FADT.Xfacs as AcpiPhysicalAddress, ACPI_TABLE_ORIGIN_INTERNAL_PHYSICAL, ptr::null_mut(), 0, 1, &mut acpi_gbl_xfacs_index); }
}

pub unsafe fn acpi_tb_create_local_fadt(table: *mut AcpiTableHeader, length: u32) {
    if length > mem::size_of::<AcpiTableFadt>() as u32 { ACPI_BIOS_WARNING!(AE_INFO, "FADT is longer than expected, truncating length", (*table).revision, length, mem::size_of::<AcpiTableFadt>()); }
    ptr::write_bytes(&mut acpi_gbl_FADT as *mut _, 0, 1);
    ptr::copy_nonoverlapping(table as *const u8, &mut acpi_gbl_FADT as *mut _ as *mut u8, core::cmp::min(length as usize, mem::size_of::<AcpiTableFadt>()));
    acpi_gbl_reduced_hardware = if acpi_gbl_FADT.flags & ACPI_FADT_HW_REDUCED != 0 { 1 } else { 0 };
    acpi_tb_convert_fadt();
    acpi_tb_setup_fadt_registers();
}

unsafe fn acpi_tb_convert_fadt() {
    if acpi_gbl_FADT.header.length <= ACPI_FADT_V2_SIZE { acpi_gbl_FADT.preferred_profile = 0; acpi_gbl_FADT.pstate_control = 0; acpi_gbl_FADT.cst_control = 0; acpi_gbl_FADT.boot_flags = 0; }
    acpi_gbl_FADT.header.length = mem::size_of::<AcpiTableFadt>() as u32;
    acpi_gbl_FADT.Xdsdt = acpi_tb_select_address(b"DSDT\0" as *const u8 as *mut _, acpi_gbl_FADT.dsdt, acpi_gbl_FADT.Xdsdt);
    if acpi_gbl_reduced_hardware != 0 { return; }
    for i in 0..FADT_INFO_TABLE.len() {
        let info = &FADT_INFO_TABLE[i];
        let address32 = *((&acpi_gbl_FADT as *const _ as *const u8).add(info.address32 as usize) as *const u32);
        let address64 = (&mut acpi_gbl_FADT as *mut _ as *mut u8).add(info.address64 as usize) as *mut AcpiGenericAddress;
        let length = *((&acpi_gbl_FADT as *const _ as *const u8).add(info.length as usize) as *const u8);
        if address32 != 0 {
            if (*address64).address != 0 && (*address64).address != address32 as u64 { ACPI_BIOS_WARNING!(AE_INFO, "32/64X address mismatch in FADT", info.name, address32, (*address64).address, if acpi_gbl_use32_bit_fadt_addresses != 0 { 32 } else { 64 }); }
            if acpi_gbl_use32_bit_fadt_addresses != 0 { acpi_tb_init_generic_address(address64, ACPI_ADR_SPACE_SYSTEM_IO, length, address32 as u64, info.name, info.flags); }
            if (*address64).address == 0 { acpi_tb_init_generic_address(address64, ACPI_ADR_SPACE_SYSTEM_IO, length, address32 as u64, info.name, info.flags); }
        }
        if info.flags & ACPI_FADT_REQUIRED != 0 && ((*address64).address == 0 || length == 0) { ACPI_BIOS_ERROR!(AE_INFO, "Required FADT field has zero address and/or length", info.name, (*address64).address, length); }
        else if info.flags & ACPI_FADT_SEPARATE_LENGTH != 0 && (((*address64).address != 0) != (length != 0)) { ACPI_BIOS_WARNING!(AE_INFO, "Optional FADT field has mismatched address/length", info.name, (*address64).address, length); }
    }
}

unsafe fn acpi_tb_setup_fadt_registers() {
    if acpi_gbl_use_default_register_widths != 0 { for info in FADT_INFO_TABLE.iter() { let g = (&mut acpi_gbl_FADT as *mut _ as *mut u8).add(info.address64 as usize) as *mut AcpiGenericAddress; if (*g).address != 0 && info.default_length != 0 && info.default_length != (*g).bit_width { ACPI_BIOS_WARNING!(AE_INFO, "Invalid length for FADT, using default", info.name, (*g).bit_width, info.default_length); (*g).bit_width = info.default_length; } } }
    let width = (acpi_gbl_FADT.xpm1a_event_block.bit_width / 16) as u8;
    for info in FADT_PM_INFO_TABLE.iter() { let source = (&acpi_gbl_FADT as *const _ as *const u8).add(info.source as usize) as *const AcpiGenericAddress; if (*source).address != 0 { acpi_tb_init_generic_address(info.target, (*source).space_id, width, (*source).address + info.register_num as u64 * width as u64, b"PmRegisters\0" as *const u8 as _, 0); } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
