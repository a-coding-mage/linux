/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Definitions for working with the Flattened Device Tree data format
 *
 * Copyright 2009 Benjamin Herrenschmidt, IBM Corp
 * benh@kernel.crashing.org
 */

/* Definitions used by the flattened device tree */
pub const OF_DT_HEADER: u32 = 0xd00dfeed; /* marker */

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

/* CONFIG_OF_FLATTREE declarations. The build configuration selects these. */
extern "C" {
    pub fn of_fdt_unflatten_tree(
        blob: *const ::kernel::ffi::c_ulong,
        dad: *mut device_node,
        mynodes: *mut *mut device_node,
    ) -> *mut ::kernel::ffi::c_void;

    /* TBD: Temporary export of fdt globals - remove when code fully merged */
    pub static mut dt_root_addr_cells: ::kernel::ffi::c_int;
    pub static mut dt_root_size_cells: ::kernel::ffi::c_int;
    pub static mut initial_boot_params: *mut ::kernel::ffi::c_void;
    pub static mut initial_boot_params_pa: phys_addr_t;

    pub static mut __dtb_start: [::kernel::ffi::c_char; 0];
    pub static mut __dtb_end: [::kernel::ffi::c_char; 0];

    /* Other Prototypes */
    pub fn of_flat_dt_translate_address(node: ::kernel::ffi::c_ulong) -> u64;
    pub fn of_fdt_limit_memory(limit: ::kernel::ffi::c_int);
}

/* CONFIG_OF_EARLY_FLATTREE declarations. The build configuration selects these. */
extern "C" {
    pub fn of_scan_flat_dt(
        it: Option<unsafe extern "C" fn(
            node: ::kernel::ffi::c_ulong,
            uname: *const ::kernel::ffi::c_char,
            depth: ::kernel::ffi::c_int,
            data: *mut ::kernel::ffi::c_void,
        ) -> ::kernel::ffi::c_int>,
        data: *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_int;
    pub fn of_scan_flat_dt_subnodes(
        node: ::kernel::ffi::c_ulong,
        it: Option<unsafe extern "C" fn(
            node: ::kernel::ffi::c_ulong,
            uname: *const ::kernel::ffi::c_char,
            data: *mut ::kernel::ffi::c_void,
        ) -> ::kernel::ffi::c_int>,
        data: *mut ::kernel::ffi::c_void,
    ) -> ::kernel::ffi::c_int;
    pub fn of_get_flat_dt_subnode_by_name(
        node: ::kernel::ffi::c_ulong,
        uname: *const ::kernel::ffi::c_char,
    ) -> ::kernel::ffi::c_int;
    pub fn of_get_flat_dt_prop(
        node: ::kernel::ffi::c_ulong,
        name: *const ::kernel::ffi::c_char,
        size: *mut ::kernel::ffi::c_int,
    ) -> *const ::kernel::ffi::c_void;
    pub fn of_flat_dt_get_addr_size_prop(
        node: ::kernel::ffi::c_ulong,
        name: *const ::kernel::ffi::c_char,
        entries: *mut ::kernel::ffi::c_int,
    ) -> *const __be32;
    pub fn of_flat_dt_get_addr_size(
        node: ::kernel::ffi::c_ulong,
        name: *const ::kernel::ffi::c_char,
        addr: *mut u64,
        size: *mut u64,
    ) -> bool;
    pub fn of_flat_dt_read_addr_size(prop: *const __be32, entry_index: ::kernel::ffi::c_int, addr: *mut u64, size: *mut u64);
    pub fn of_flat_dt_is_compatible(node: ::kernel::ffi::c_ulong, name: *const ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn of_get_flat_dt_root() -> ::kernel::ffi::c_ulong;
    pub fn of_get_flat_dt_phandle(node: ::kernel::ffi::c_ulong) -> u32;
    pub fn early_init_dt_scan_chosen(cmdline: *mut ::kernel::ffi::c_char) -> ::kernel::ffi::c_int;
    pub fn early_init_dt_scan_memory() -> ::kernel::ffi::c_int;
    pub fn early_init_dt_check_for_usable_mem_range();
    pub fn early_init_dt_scan_chosen_stdout() -> ::kernel::ffi::c_int;
    pub fn early_init_fdt_scan_reserved_mem();
    pub fn early_init_fdt_reserve_self();
    pub fn early_init_dt_add_memory_arch(base: u64, size: u64);
    pub fn dt_mem_next_cell(s: ::kernel::ffi::c_int, cellp: *mut *const __be32) -> u64;
    pub fn early_init_dt_scan_root() -> ::kernel::ffi::c_int;
    pub fn early_init_dt_scan(dt_virt: *mut ::kernel::ffi::c_void, dt_phys: phys_addr_t) -> bool;
    pub fn early_init_dt_verify(dt_virt: *mut ::kernel::ffi::c_void, dt_phys: phys_addr_t) -> bool;
    pub fn early_init_dt_scan_nodes();
    pub fn of_flat_dt_get_machine_name() -> *const ::kernel::ffi::c_char;
    pub fn of_flat_dt_match_machine(
        default_match: *const ::kernel::ffi::c_void,
        get_next_compat: Option<unsafe extern "C" fn(*const *const ::kernel::ffi::c_char) -> *const ::kernel::ffi::c_void>,
    ) -> *const ::kernel::ffi::c_void;
    pub fn unflatten_device_tree();
    pub fn unflatten_and_copy_device_tree();
    pub fn early_init_devtree(arg: *mut ::kernel::ffi::c_void);
    pub fn early_get_first_memblock_info(arg: *mut ::kernel::ffi::c_void, val: *mut phys_addr_t);
}

/* CONFIG_OF_EARLY_FLATTREE fallback definitions. */
#[inline]
pub unsafe fn early_init_dt_check_for_usable_mem_range_fallback() {}
#[inline]
pub unsafe fn early_init_dt_scan_chosen_stdout_fallback() -> ::kernel::ffi::c_int { -19 }
#[inline]
pub unsafe fn early_init_fdt_scan_reserved_mem_fallback() {}
#[inline]
pub unsafe fn early_init_fdt_reserve_self_fallback() {}
#[inline]
pub unsafe fn of_flat_dt_get_machine_name_fallback() -> *const ::kernel::ffi::c_char { ::core::ptr::null() }
#[inline]
pub unsafe fn unflatten_device_tree_fallback() {}
#[inline]
pub unsafe fn unflatten_and_copy_device_tree_fallback() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
