// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation unit.

extern "C" {
    static mut max_pfn: u64;
    static mut max_low_pfn: u64;

    fn memblock_add(base: u64, size: u64);
    fn memblock_reserve(base: u64, size: u64);
    fn memblock_set_current_limit(limit: u64);
    fn memblock_set_node(start: u64, end: u64, type_: *mut memblock_type, nid: i32);

    static mut memblock: memblock_struct;

    fn __pa_symbol(symbol: *const core::ffi::c_void) -> u64;
}

#[repr(C)]
pub struct memblock_struct {
    pub memory: memblock_type,
    pub reserved: memblock_type,
}

#[repr(C)]
pub struct memblock_type {
    _private: [u8; 0],
}

#[repr(C)]
pub struct efi_memory_desc_t {
    pub type_: u32,
    pub pad: u32,
    pub phys_addr: u64,
    pub virt_addr: u64,
    pub num_pages: u64,
    pub attribute: u64,
}

extern "C" {
    static _text: u8;
    static _end: u8;
}

// Build-time kernel constants and helpers supplied by included headers.
const EFI_PAGE_SHIFT: u32 = 12;
const PHYS_OFFSET: u64 = 0;
const PHYS_ADDR_MAX: u64 = u64::MAX;

// The EFI descriptor iterator is provided by the kernel EFI support.
macro_rules! for_each_efi_memory_desc {
    ($md:ident, $body:block) => {{
        let _ = &mut $md;
        $body
    }};
}

// PFN_DOWN, PFN_PHYS, and HIGHMEM_START are supplied by the architecture headers.
unsafe fn pfn_down(value: u64) -> u64 {
    value >> 12
}

unsafe fn pfn_phys(value: u64) -> u64 {
    value << 12
}

extern "C" {
    fn memblock_end_of_DRAM() -> u64;
}

pub unsafe fn memblock_init() {
    let mut mem_type: u32;
    let mut mem_start: u64;
    let mut mem_size: u64;
    let mut md: *mut efi_memory_desc_t = core::ptr::null_mut();

    /* Parse memory information */
    for_each_efi_memory_desc!(md, {
        mem_type = (*md).type_;
        mem_start = (*md).phys_addr;
        mem_size = (*md).num_pages << EFI_PAGE_SHIFT;

        match mem_type {
            EFI_LOADER_CODE
            | EFI_LOADER_DATA
            | EFI_BOOT_SERVICES_CODE
            | EFI_BOOT_SERVICES_DATA
            | EFI_PERSISTENT_MEMORY
            | EFI_CONVENTIONAL_MEMORY => {
                memblock_add(mem_start, mem_size);
            }
            EFI_PAL_CODE | EFI_UNUSABLE_MEMORY | EFI_ACPI_RECLAIM_MEMORY => {
                memblock_add(mem_start, mem_size);
                memblock_reserve(mem_start, mem_size);
            }
            EFI_RESERVED_TYPE
            | EFI_RUNTIME_SERVICES_CODE
            | EFI_RUNTIME_SERVICES_DATA
            | EFI_MEMORY_MAPPED_IO
            | EFI_MEMORY_MAPPED_IO_PORT_SPACE => {
                memblock_reserve(mem_start, mem_size);
            }
            _ => {}
        }
    });

    max_pfn = pfn_down(memblock_end_of_DRAM());
    max_low_pfn = core::cmp::min(pfn_down(HIGHMEM_START), max_pfn);
    memblock_set_current_limit(pfn_phys(max_low_pfn));

    /* Reserve the first 2MB */
    memblock_reserve(PHYS_OFFSET, 0x200000);

    /* Reserve the kernel text/data/bss */
    memblock_reserve(
        __pa_symbol((&_text) as *const u8 as *const core::ffi::c_void),
        __pa_symbol((&_end) as *const u8 as *const core::ffi::c_void)
            - __pa_symbol((&_text) as *const u8 as *const core::ffi::c_void),
    );

    memblock_set_node(0, PHYS_ADDR_MAX, &mut memblock.memory, 0);
    memblock_set_node(0, PHYS_ADDR_MAX, &mut memblock.reserved, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
