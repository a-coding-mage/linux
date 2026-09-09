/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm/e820/types.h>

extern "C" {
    pub static mut e820_table: *mut e820_table;
    pub static mut e820_table_kexec: *mut e820_table;
    pub static mut e820_table_firmware: *mut e820_table;

    pub static mut pci_mem_start: core::ffi::c_ulong;

    pub fn e820__mapped_raw_any(start: u64, end: u64, type_: e820_type) -> bool;
    pub fn e820__mapped_any(start: u64, end: u64, type_: e820_type) -> bool;
    pub fn e820__mapped_all(start: u64, end: u64, type_: e820_type) -> bool;

    pub fn e820__range_add(start: u64, size: u64, type_: e820_type);
    pub fn e820__range_update(
        start: u64,
        size: u64,
        old_type: e820_type,
        new_type: e820_type,
    ) -> u64;
    pub fn e820__range_remove(start: u64, size: u64, filter_type: e820_type);
    pub fn e820__range_update_table(
        t: *mut e820_table,
        start: u64,
        size: u64,
        old_type: e820_type,
        new_type: e820_type,
    ) -> u64;

    pub fn e820__update_table(table: *mut e820_table) -> core::ffi::c_int;
    pub fn e820__update_table_print();

    pub fn e820__end_of_ram_pfn() -> core::ffi::c_ulong;
    pub fn e820__end_of_low_ram_pfn() -> core::ffi::c_ulong;

    pub fn e820__memblock_alloc_reserved(size: u64, align: u64) -> u64;
    pub fn e820__memblock_setup();

    pub fn e820__finish_early_params();
    pub fn e820__reserve_resources();
    pub fn e820__reserve_resources_late();

    pub fn e820__memory_setup();
    pub fn e820__memory_setup_extended(phys_addr: u64, data_len: u32);
    pub fn e820__memory_setup_default() -> *mut core::ffi::c_char;
    pub fn e820__setup_pci_gap();

    pub fn e820__reallocate_tables();
    pub fn e820__register_nosave_regions(limit_pfn: core::ffi::c_ulong);

    pub fn e820__get_entry_type(start: u64, end: u64) -> core::ffi::c_int;
}

/*
 * Returns true iff the specified range [start,end) is completely contained inside
 * the ISA region.
 */
#[inline]
pub fn is_ISA_range(start: u64, end: u64) -> bool {
    start >= ISA_START_ADDRESS && end <= ISA_END_ADDRESS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
