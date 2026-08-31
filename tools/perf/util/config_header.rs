/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::{c_char, c_int, c_void};

// C dependencies from:
// #include <stdbool.h>
// #include <linux/list.h>

#[repr(C)]
pub struct perf_config_item {
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub from_system_config: bool,
    pub node: list_head,
}

#[repr(C)]
pub struct perf_config_section {
    pub name: *mut c_char,
    pub items: list_head,
    pub from_system_config: bool,
    pub node: list_head,
}

#[repr(C)]
pub struct perf_config_set {
    pub sections: list_head,
}

extern "C" {
    pub static config_exclusive_filename: *const c_char;
}

pub type config_fn_t =
    Option<unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int>;

extern "C" {
    pub fn perf_default_config(arg1: *const c_char, arg2: *const c_char, arg3: *mut c_void)
        -> c_int;
    pub fn perf_config(fn_: config_fn_t, arg2: *mut c_void) -> c_int;

    // C declaration carried scanf checking attribute: __scanf(2, 3).
    pub fn perf_config_scan(name: *const c_char, fmt: *const c_char, ...) -> c_int;

    pub fn perf_config_get(name: *const c_char) -> *const c_char;
    pub fn perf_config_set(
        set: *mut perf_config_set,
        fn_: config_fn_t,
        data: *mut c_void,
    ) -> c_int;
    pub fn perf_config_int(dest: *mut c_int, arg2: *const c_char, arg3: *const c_char) -> c_int;
    pub fn perf_config_u8(dest: *mut u8, name: *const c_char, value: *const c_char) -> c_int;
    pub fn perf_config_u64(dest: *mut u64, arg2: *const c_char, arg3: *const c_char) -> c_int;
    pub fn perf_config_bool(arg1: *const c_char, arg2: *const c_char) -> c_int;
    pub fn config_error_nonbool(arg1: *const c_char) -> c_int;
    pub fn perf_etc_perfconfig() -> *const c_char;
    pub fn perf_home_perfconfig() -> *const c_char;
    pub fn perf_config_system() -> c_int;
    pub fn perf_config_global() -> c_int;

    pub fn perf_config_set__new() -> *mut perf_config_set;
    pub fn perf_config_set__load_file(file: *const c_char) -> *mut perf_config_set;
    pub fn perf_config_set__delete(set: *mut perf_config_set);
    pub fn perf_config_set__collect(
        set: *mut perf_config_set,
        file_name: *const c_char,
        var: *const c_char,
        value: *const c_char,
    ) -> c_int;
    pub fn perf_config__exit();
    pub fn perf_config__set_variable(var: *const c_char, value: *const c_char) -> c_int;
}

/**
 * perf_config_sections__for_each - iterate thru all the sections
 * @list: list_head instance to iterate
 * @section: struct perf_config_section iterator
 */
#[macro_export]
macro_rules! perf_config_sections__for_each_entry {
    ($list:expr, $section:ident) => {
        list_for_each_entry!($section, $list, node)
    };
}

/**
 * perf_config_items__for_each - iterate thru all the items
 * @list: list_head instance to iterate
 * @item: struct perf_config_item iterator
 */
#[macro_export]
macro_rules! perf_config_items__for_each_entry {
    ($list:expr, $item:ident) => {
        list_for_each_entry!($item, $list, node)
    };
}

/**
 * perf_config_set__for_each - iterate thru all the config section-item pairs
 * @set: evlist instance to iterate
 * @section: struct perf_config_section iterator
 * @item: struct perf_config_item iterator
 */
#[macro_export]
macro_rules! perf_config_set__for_each_entry {
    ($set:expr, $section:ident, $item:ident) => {
        perf_config_sections__for_each_entry!(&(*$set).sections, $section);
        perf_config_items__for_each_entry!(&(*$section).items, $item)
    };
}
