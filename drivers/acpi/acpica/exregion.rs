// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: exregion - ACPI default op_region (address space) handlers
//
// Copyright (C) 2000 - 2026, Intel Corp.

use core::ffi::c_void;

// Dependencies supplied by the ACPI implementation.
extern "C" {
    fn acpi_os_map_memory(address: acpi_physical_address, length: acpi_size) -> *mut c_void;
    fn acpi_hw_read_port(address: acpi_io_address, value: *mut u32, width: u32) -> acpi_status;
    fn acpi_hw_write_port(address: acpi_io_address, value: u32, width: u32) -> acpi_status;
    fn acpi_os_read_pci_configuration(id: *const acpi_pci_id, reg: u16, value: *mut u64, width: u32) -> acpi_status;
    fn acpi_os_write_pci_configuration(id: *const acpi_pci_id, reg: u16, value: u64, width: u32) -> acpi_status;
}

#[repr(C)]
pub struct acpi_mem_mapping {
    pub logical_address: *mut u8,
    pub physical_address: acpi_physical_address,
    pub length: acpi_size,
    pub next_mm: *mut acpi_mem_mapping,
}

#[repr(C)]
pub struct acpi_mem_space_context {
    pub address: acpi_physical_address,
    pub length: acpi_size,
    pub cur_mm: *mut acpi_mem_mapping,
    pub first_mm: *mut acpi_mem_mapping,
}

#[repr(C)]
pub struct acpi_pci_id {
    pub segment: u16,
    pub bus: u16,
    pub device: u16,
    pub function: u16,
}

#[repr(C)]
pub struct acpi_data_table_mapping {
    pub pointer: *mut c_void,
}

extern "C" {
    fn acpi_allocate_zeroed(size: usize) -> *mut c_void;
    fn acpi_free(pointer: *mut c_void);
}

pub type u32_ = u32;
pub type u64_ = u64;
pub type acpi_status = u32;
pub type acpi_size = usize;
pub type acpi_physical_address = u64;
pub type acpi_io_address = u64;

const AE_OK: acpi_status = 0;
const AE_NO_MEMORY: acpi_status = 1;
const AE_BAD_PARAMETER: acpi_status = 2;
const AE_AML_OPERAND_VALUE: acpi_status = 3;
const AE_AML_ALIGNMENT: acpi_status = 4;
const ACPI_READ: u32 = 0;
const ACPI_WRITE: u32 = 1;
const ACPI_DEFAULT_PAGE_SIZE: usize = 4096;

#[inline]
unsafe fn get8(p: *mut u8) -> u8 { core::ptr::read_volatile(p) }
#[inline]
unsafe fn get16(p: *mut u8) -> u16 { u16::from_le(core::ptr::read_unaligned(p as *const u16)) }
#[inline]
unsafe fn get32(p: *mut u8) -> u32 { u32::from_le(core::ptr::read_unaligned(p as *const u32)) }
#[inline]
unsafe fn get64(p: *mut u8) -> u64 { u64::from_le(core::ptr::read_unaligned(p as *const u64)) }
#[inline]
unsafe fn set8(p: *mut u8, v: u64) { core::ptr::write_volatile(p, v as u8); }
#[inline]
unsafe fn set16(p: *mut u8, v: u64) { core::ptr::write_unaligned(p as *mut u16, (v as u16).to_le()); }
#[inline]
unsafe fn set32(p: *mut u8, v: u64) { core::ptr::write_unaligned(p as *mut u32, (v as u32).to_le()); }
#[inline]
unsafe fn set64(p: *mut u8, v: u64) { core::ptr::write_unaligned(p as *mut u64, v.to_le()); }

pub unsafe extern "C" fn acpi_ex_system_memory_space_handler(function: u32, address: acpi_physical_address, bit_width: u32, value: *mut u64, _handler_context: *mut c_void, region_context: *mut c_void) -> acpi_status {
    let length = match bit_width { 8 => 1, 16 => 2, 32 => 4, 64 => 8, _ => return AE_AML_OPERAND_VALUE };
    let mem_info = region_context as *mut acpi_mem_space_context;
    let mut mm = (*mem_info).cur_mm;
    if mm.is_null() || address < (*mm).physical_address || address + length as u64 > (*mm).physical_address + (*mm).length as u64 {
        let mut candidate = (*mem_info).first_mm;
        while !candidate.is_null() {
            if candidate != (*mem_info).cur_mm && address >= (*candidate).physical_address && address + length as u64 <= (*candidate).physical_address + (*candidate).length as u64 {
                mm = candidate;
                (*mem_info).cur_mm = mm;
                break;
            }
            candidate = (*candidate).next_mm;
        }
        if mm.is_null() || address < (*mm).physical_address || address + length as u64 > (*mm).physical_address + (*mm).length as u64 {
            mm = acpi_allocate_zeroed(core::mem::size_of::<acpi_mem_mapping>()) as *mut acpi_mem_mapping;
            if mm.is_null() { return AE_NO_MEMORY; }
            let mut map_length = ((*mem_info).address + (*mem_info).length as u64 - address) as usize;
            if map_length > ACPI_DEFAULT_PAGE_SIZE { map_length = ACPI_DEFAULT_PAGE_SIZE; }
            let logical = acpi_os_map_memory(address, map_length);
            if logical.is_null() { acpi_free(mm as *mut c_void); return AE_NO_MEMORY; }
            (*mm).logical_address = logical as *mut u8;
            (*mm).physical_address = address;
            (*mm).length = map_length;
            (*mm).next_mm = (*mem_info).first_mm;
            (*mem_info).first_mm = mm;
            (*mem_info).cur_mm = mm;
        }
    }
    let logical = (*mm).logical_address.add((address - (*mm).physical_address) as usize);
    match function {
        ACPI_READ => { *value = match bit_width { 8 => get8(logical) as u64, 16 => get16(logical) as u64, 32 => get32(logical) as u64, 64 => get64(logical), _ => 0 }; AE_OK }
        ACPI_WRITE => { match bit_width { 8 => set8(logical, *value), 16 => set16(logical, *value), 32 => set32(logical, *value), 64 => set64(logical, *value), _ => {} }; AE_OK }
        _ => AE_BAD_PARAMETER,
    }
}

pub unsafe extern "C" fn acpi_ex_system_io_space_handler(function: u32, address: acpi_physical_address, bit_width: u32, value: *mut u64, _handler_context: *mut c_void, _region_context: *mut c_void) -> acpi_status {
    let mut value32 = 0;
    match function { ACPI_READ => { let s = acpi_hw_read_port(address as acpi_io_address, &mut value32, bit_width); *value = value32 as u64; s }, ACPI_WRITE => acpi_hw_write_port(address as acpi_io_address, *value as u32, bit_width), _ => AE_BAD_PARAMETER }
}

#[cfg(feature = "ACPI_PCI_CONFIGURED")]
pub unsafe extern "C" fn acpi_ex_pci_config_space_handler(function: u32, address: acpi_physical_address, bit_width: u32, value: *mut u64, _handler_context: *mut c_void, region_context: *mut c_void) -> acpi_status {
    let id = region_context as *const acpi_pci_id;
    let reg = address as u16;
    match function { ACPI_READ => { *value = 0; acpi_os_read_pci_configuration(id, reg, value, bit_width) }, ACPI_WRITE => acpi_os_write_pci_configuration(id, reg, *value, bit_width), _ => AE_BAD_PARAMETER }
}

pub unsafe extern "C" fn acpi_ex_cmos_space_handler(_function: u32, _address: acpi_physical_address, _bit_width: u32, _value: *mut u64, _handler_context: *mut c_void, _region_context: *mut c_void) -> acpi_status { AE_OK }

#[cfg(feature = "ACPI_PCI_CONFIGURED")]
pub unsafe extern "C" fn acpi_ex_pci_bar_space_handler(_function: u32, _address: acpi_physical_address, _bit_width: u32, _value: *mut u64, _handler_context: *mut c_void, _region_context: *mut c_void) -> acpi_status { AE_OK }

pub unsafe extern "C" fn acpi_ex_data_table_space_handler(function: u32, address: acpi_physical_address, bit_width: u32, value: *mut u64, _handler_context: *mut c_void, region_context: *mut c_void) -> acpi_status {
    let mapping = &*(region_context as *const acpi_data_table_mapping);
    let pointer = (mapping.pointer as *mut u8).add((address - mapping.pointer as u64) as usize);
    let length = (bit_width / 8) as usize;
    match function { ACPI_READ => core::ptr::copy_nonoverlapping(pointer, value as *mut u8, length), ACPI_WRITE => core::ptr::copy_nonoverlapping(value as *const u8, pointer, length), _ => return AE_BAD_PARAMETER }
    AE_OK
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
