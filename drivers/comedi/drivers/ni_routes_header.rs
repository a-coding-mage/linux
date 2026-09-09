/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Rust translation of comedi/drivers/ni_routes.h.
 * External kernel/comedi constants and functions are supplied by dependencies.
 */

use core::ffi::c_char;

#[repr(C)]
pub struct ni_route_set {
    pub dest: i32,
    pub n_src: i32,
    pub src: *mut i32,
}

#[repr(C)]
pub struct ni_device_routes {
    pub device: *const c_char,
    pub n_route_sets: i32,
    pub routes: *mut ni_route_set,
}

#[repr(C)]
pub struct ni_route_tables {
    pub valid_routes: *const ni_device_routes,
    pub route_values: *const u8,
}

extern "C" {
    pub fn ni_assign_device_routes(
        device_family: *const c_char,
        board_name: *const c_char,
        alt_board_name: *const c_char,
        tables: *mut ni_route_tables,
    ) -> i32;

    pub fn ni_find_route_set(
        destination: i32,
        valid_routes: *const ni_device_routes,
    ) -> *const ni_route_set;

    pub fn ni_route_set_has_source(routes: *const ni_route_set, src: i32) -> bool;

    pub fn ni_route_to_register(
        src: i32,
        dest: i32,
        tables: *const ni_route_tables,
    ) -> i8;

    pub fn ni_lookup_route_register(
        src: i32,
        dest: i32,
        tables: *const ni_route_tables,
    ) -> i8;

    pub fn ni_is_cmd_dest(dest: i32) -> bool;

    pub fn ni_count_valid_routes(tables: *const ni_route_tables) -> u32;

    pub fn ni_get_valid_routes(
        tables: *const ni_route_tables,
        n_pairs: u32,
        pair_data: *mut u32,
    ) -> u32;

    pub fn ni_sort_device_routes(valid_routes: *mut ni_device_routes);

    pub fn ni_find_route_source(
        src_sel_reg_value: u8,
        dest: i32,
        tables: *const ni_route_tables,
    ) -> i32;
}

#[inline]
pub fn ni_rtsi_route_requires_mux(value: i8) -> bool {
    (value & (1i8 << 6)) != 0
}

#[inline]
pub unsafe fn route_is_valid(
    src: i32,
    dest: i32,
    tables: *const ni_route_tables,
) -> bool {
    ni_route_to_register(src, dest, tables) >= 0
}

/* NI_PFI(0) <= channel && channel <= NI_PFI(-1). */
#[inline]
pub fn channel_is_pfi(channel: i32) -> bool {
    unsafe { NI_PFI(0) <= channel && channel <= NI_PFI(-1) }
}

/* TRIGGER_LINE(0) <= channel && channel <= TRIGGER_LINE(-1). */
#[inline]
pub fn channel_is_rtsi(channel: i32) -> bool {
    unsafe { TRIGGER_LINE(0) <= channel && channel <= TRIGGER_LINE(-1) }
}

#[inline]
pub fn channel_is_ctr(channel: i32) -> bool {
    channel >= NI_COUNTER_NAMES_BASE && channel <= NI_COUNTER_NAMES_MAX
}

#[inline]
pub unsafe fn route_register_is_valid(
    src_sel_reg_value: u8,
    dest: i32,
    tables: *const ni_route_tables,
) -> bool {
    ni_find_route_source(src_sel_reg_value, dest, tables) >= 0
}

#[inline]
pub unsafe fn ni_get_reg_value_roffs(
    mut src: i32,
    dest: i32,
    tables: *const ni_route_tables,
    direct_reg_offset: i32,
) -> i8 {
    if src < NI_NAMES_BASE {
        src += direct_reg_offset;
        if route_register_is_valid(src as u8, dest, tables) {
            return src as i8;
        }
        return -1;
    }
    ni_route_to_register(src, dest, tables)
}

#[inline]
pub unsafe fn ni_get_reg_value(
    src: i32,
    dest: i32,
    tables: *const ni_route_tables,
) -> i32 {
    ni_get_reg_value_roffs(src, dest, tables, 0) as i32
}

#[inline]
pub unsafe fn ni_check_trigger_arg_roffs(
    src: i32,
    dest: i32,
    tables: *const ni_route_tables,
    direct_reg_offset: i32,
) -> i32 {
    if ni_get_reg_value_roffs(src, dest, tables, direct_reg_offset) < 0 {
        return -22; // -EINVAL
    }
    0
}

#[inline]
pub unsafe fn ni_check_trigger_arg(
    src: i32,
    dest: i32,
    tables: *const ni_route_tables,
) -> i32 {
    ni_check_trigger_arg_roffs(src, dest, tables, 0)
}

/* External constants/macros supplied by linux/comedi.h. */
extern "C" {
    pub static NI_NAMES_BASE: i32;
    pub static NI_COUNTER_NAMES_BASE: i32;
    pub static NI_COUNTER_NAMES_MAX: i32;
}

extern "Rust" {
    fn NI_PFI(index: i32) -> i32;
    fn TRIGGER_LINE(index: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
