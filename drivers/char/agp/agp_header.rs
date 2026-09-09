/*
 * AGPGART
 * Copyright (C) 2004 Silicon Graphics, Inc.
 * Copyright (C) 2002-2004 Dave Jones
 * Copyright (C) 1999 Jeff Hartmann
 * Copyright (C) 1999 Precision Insight, Inc.
 * Copyright (C) 1999 Xi Graphics, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependency: <asm/agp.h> supplies flush_agp_cache and related kernel types.

pub const PFX: &str = "agpgart: ";

#[repr(C)]
#[derive(Copy, Clone)]
pub enum aper_size_type {
    U8_APER_SIZE,
    U16_APER_SIZE,
    U32_APER_SIZE,
    LVL2_APER_SIZE,
    FIXED_APER_SIZE,
}

pub const AGP_PAGE_DESTROY_UNMAP: i32 = 1;
pub const AGP_PAGE_DESTROY_FREE: i32 = 2;

#[repr(C)]
pub struct gatt_mask {
    pub mask: ::core::ffi::c_ulong,
    pub r#type: u32,
}

#[repr(C)]
pub struct aper_size_info_8 {
    pub size: i32,
    pub num_entries: i32,
    pub page_order: i32,
    pub size_value: u8,
}

#[repr(C)]
pub struct aper_size_info_16 {
    pub size: i32,
    pub num_entries: i32,
    pub page_order: i32,
    pub size_value: u16,
}

#[repr(C)]
pub struct aper_size_info_32 {
    pub size: i32,
    pub num_entries: i32,
    pub page_order: i32,
    pub size_value: u32,
}

#[repr(C)]
pub struct aper_size_info_lvl2 {
    pub size: i32,
    pub num_entries: i32,
    pub size_value: u32,
}

#[repr(C)]
pub struct aper_size_info_fixed {
    pub size: i32,
    pub num_entries: i32,
    pub page_order: i32,
}

#[repr(C)]
pub struct agp_bridge_driver {
    pub owner: *mut module,
    pub aperture_sizes: *const core::ffi::c_void,
    pub num_aperture_sizes: i32,
    pub size_type: aper_size_type,
    pub cant_use_aperture: bool,
    pub needs_scratch_page: bool,
    pub masks: *const gatt_mask,
    pub fetch_size: Option<unsafe extern "C" fn() -> i32>,
    pub configure: Option<unsafe extern "C" fn() -> i32>,
    pub agp_enable: Option<unsafe extern "C" fn(*mut agp_bridge_data, u32)>,
    pub cleanup: Option<unsafe extern "C" fn()>,
    pub tlb_flush: Option<unsafe extern "C" fn(*mut agp_memory)>,
    pub mask_memory: Option<unsafe extern "C" fn(*mut agp_bridge_data, dma_addr_t, i32) -> ::core::ffi::c_ulong>,
    pub cache_flush: Option<unsafe extern "C" fn()>,
    pub create_gatt_table: Option<unsafe extern "C" fn(*mut agp_bridge_data) -> i32>,
    pub free_gatt_table: Option<unsafe extern "C" fn(*mut agp_bridge_data) -> i32>,
    pub insert_memory: Option<unsafe extern "C" fn(*mut agp_memory, off_t, i32) -> i32>,
    pub remove_memory: Option<unsafe extern "C" fn(*mut agp_memory, off_t, i32) -> i32>,
    pub alloc_by_type: Option<unsafe extern "C" fn(usize, i32) -> *mut agp_memory>,
    pub free_by_type: Option<unsafe extern "C" fn(*mut agp_memory)>,
    pub agp_alloc_page: Option<unsafe extern "C" fn(*mut agp_bridge_data) -> *mut page>,
    pub agp_alloc_pages: Option<unsafe extern "C" fn(*mut agp_bridge_data, *mut agp_memory, usize) -> i32>,
    pub agp_destroy_page: Option<unsafe extern "C" fn(*mut page, i32)>,
    pub agp_destroy_pages: Option<unsafe extern "C" fn(*mut agp_memory)>,
    pub agp_type_to_mask_type: Option<unsafe extern "C" fn(*mut agp_bridge_data, i32) -> i32>,
}

#[repr(C)]
pub struct agp_bridge_data {
    pub version: *const agp_version,
    pub driver: *const agp_bridge_driver,
    pub vm_ops: *const vm_operations_struct,
    pub previous_size: *mut core::ffi::c_void,
    pub current_size: *mut core::ffi::c_void,
    pub dev_private_data: *mut core::ffi::c_void,
    pub dev: *mut pci_dev,
    pub gatt_table: *mut u32,
    pub gatt_table_real: *mut u32,
    pub scratch_page: ::core::ffi::c_ulong,
    pub scratch_page_page: *mut page,
    pub scratch_page_dma: dma_addr_t,
    pub gart_bus_addr: ::core::ffi::c_ulong,
    pub gatt_bus_addr: ::core::ffi::c_ulong,
    pub mode: u32,
    pub key_list: *mut ::core::ffi::c_ulong,
    pub current_memory_agp: atomic_t,
    pub agp_in_use: atomic_t,
    pub max_memory_agp: i32,
    pub aperture_size_idx: i32,
    pub capndx: i32,
    pub flags: i32,
    pub major_version: i8,
    pub minor_version: i8,
    pub list: list_head,
    pub apbase_config: u32,
    pub mapped_list: list_head,
    pub mapped_lock: spinlock_t,
}

pub const fn KB(x: usize) -> usize { x * 1024 }
pub const fn MB(x: usize) -> usize { KB(KB(x)) }
pub const fn GB(x: usize) -> usize { MB(KB(x)) }

#[macro_export]
macro_rules! A_SIZE_8 { ($x:expr) => { $x as *mut $crate::aper_size_info_8 }; }
#[macro_export]
macro_rules! A_SIZE_16 { ($x:expr) => { $x as *mut $crate::aper_size_info_16 }; }
#[macro_export]
macro_rules! A_SIZE_32 { ($x:expr) => { $x as *mut $crate::aper_size_info_32 }; }
#[macro_export]
macro_rules! A_SIZE_LVL2 { ($x:expr) => { $x as *mut $crate::aper_size_info_lvl2 }; }
#[macro_export]
macro_rules! A_SIZE_FIX { ($x:expr) => { $x as *mut $crate::aper_size_info_fixed }; }
pub const MAXKEY: usize = 4096 * 32;

#[macro_export]
macro_rules! PGE_EMPTY { ($b:expr, $p:expr) => { (($p) == 0 || ($p) == ($b).scratch_page) }; }

#[repr(C)]
pub struct agp_device_ids {
    pub device_id: u16,
    pub chipset: chipset_type,
    pub chipset_name: *const i8,
    pub chipset_setup: Option<unsafe extern "C" fn(*mut pci_dev) -> i32>,
}

extern "C" {
    pub static mut agp_bridge: *mut agp_bridge_data;
    pub fn agp_alloc_bridge() -> *mut agp_bridge_data;
    pub fn agp_put_bridge(bridge: *mut agp_bridge_data);
    pub fn agp_add_bridge(bridge: *mut agp_bridge_data) -> i32;
    pub fn agp_remove_bridge(bridge: *mut agp_bridge_data);
    pub fn agp_generic_enable(bridge: *mut agp_bridge_data, mode: u32);
    pub fn agp_generic_create_gatt_table(bridge: *mut agp_bridge_data) -> i32;
    pub fn agp_generic_free_gatt_table(bridge: *mut agp_bridge_data) -> i32;
    pub fn agp_create_memory(scratch_pages: i32) -> *mut agp_memory;
    pub fn agp_generic_insert_memory(mem: *mut agp_memory, pg_start: off_t, r#type: i32) -> i32;
    pub fn agp_generic_remove_memory(mem: *mut agp_memory, pg_start: off_t, r#type: i32) -> i32;
    pub fn agp_generic_alloc_by_type(page_count: usize, r#type: i32) -> *mut agp_memory;
    pub fn agp_generic_free_by_type(curr: *mut agp_memory);
    pub fn agp_generic_alloc_page(bridge: *mut agp_bridge_data) -> *mut page;
    pub fn agp_generic_alloc_pages(agp_bridge: *mut agp_bridge_data, memory: *mut agp_memory, page_count: usize) -> i32;
    pub fn agp_generic_destroy_page(page: *mut page, flags: i32);
    pub fn agp_generic_destroy_pages(memory: *mut agp_memory);
    pub fn agp_free_key(key: i32);
    pub fn agp_num_entries() -> i32;
    pub fn agp_collect_device_status(bridge: *mut agp_bridge_data, mode: u32, command: u32) -> u32;
    pub fn agp_device_command(command: u32, agp_v3: bool);
    pub fn agp_3_5_enable(bridge: *mut agp_bridge_data) -> i32;
    pub fn global_cache_flush();
    pub fn get_agp_version(bridge: *mut agp_bridge_data);
    pub fn agp_generic_mask_memory(bridge: *mut agp_bridge_data, phys: dma_addr_t, r#type: i32) -> ::core::ffi::c_ulong;
    pub fn agp_generic_type_to_mask_type(bridge: *mut agp_bridge_data, r#type: i32) -> i32;
    pub fn agp_generic_find_bridge(pdev: *mut pci_dev) -> *mut agp_bridge_data;
    pub fn agp_generic_alloc_user(page_count: usize, r#type: i32) -> *mut agp_memory;
    pub fn agp_alloc_page_array(size: usize, mem: *mut agp_memory);
    pub fn agp3_generic_fetch_size() -> i32;
    pub fn agp3_generic_tlbflush(mem: *mut agp_memory);
    pub fn agp3_generic_configure() -> i32;
    pub fn agp3_generic_cleanup();
    pub static agp3_generic_sizes: [aper_size_info_16; 11];
    pub static mut agp_off: i32;
    pub static mut agp_try_unsupported_boot: i32;
}

#[inline]
pub unsafe fn agp_free_page_array(mem: *mut agp_memory) {
    kvfree((*mem).pages as *mut core::ffi::c_void);
}

// The following C preprocessor helpers retain their intended dependency on kernel page APIs.
#[macro_export]
macro_rules! alloc_gatt_pages { ($order:expr) => { __get_free_pages(GFP_KERNEL, $order) as *mut i8 }; }
#[macro_export]
macro_rules! free_gatt_pages { ($table:expr, $order:expr) => { free_pages($table as usize, $order) }; }
pub const AGP_GENERIC_SIZES_ENTRIES: usize = 11;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
