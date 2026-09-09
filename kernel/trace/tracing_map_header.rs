// SPDX-License-Identifier: GPL-2.0

pub const TRACING_MAP_BITS_DEFAULT: ::core::ffi::c_uint = 11;
pub const TRACING_MAP_BITS_MAX: ::core::ffi::c_uint = 17;
pub const TRACING_MAP_BITS_MIN: ::core::ffi::c_uint = 7;
pub const TRACING_MAP_KEYS_MAX: usize = 3;
pub const TRACING_MAP_VALS_MAX: usize = 3;
pub const TRACING_MAP_FIELDS_MAX: usize = TRACING_MAP_KEYS_MAX + TRACING_MAP_VALS_MAX;
pub const TRACING_MAP_VARS_MAX: usize = 16;
pub const TRACING_MAP_SORT_KEYS_MAX: usize = 2;

pub type tracing_map_cmp_fn_t = unsafe extern "C" fn(val_a: *mut ::core::ffi::c_void,
                                                       val_b: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;

#[repr(C)]
pub union tracing_map_field_data {
    pub sum: atomic64_t,
    pub offset: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct tracing_map_field {
    pub cmp_fn: tracing_map_cmp_fn_t,
    pub data: tracing_map_field_data,
}

#[repr(C)]
pub struct tracing_map_elt {
    pub map: *mut tracing_map,
    pub fields: *mut tracing_map_field,
    pub vars: *mut atomic64_t,
    pub var_set: *mut bool,
    pub key: *mut ::core::ffi::c_void,
    pub private_data: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct tracing_map_entry {
    pub key: u32,
    pub val: *mut tracing_map_elt,
}

#[repr(C)]
pub struct tracing_map_sort_key {
    pub field_idx: ::core::ffi::c_uint,
    pub descending: bool,
}

#[repr(C)]
pub struct tracing_map_sort_entry {
    pub key: *mut ::core::ffi::c_void,
    pub elt: *mut tracing_map_elt,
    pub elt_copied: bool,
    pub dup: bool,
}

#[repr(C)]
pub struct tracing_map_array {
    pub entries_per_page: ::core::ffi::c_uint,
    pub entry_size_shift: ::core::ffi::c_uint,
    pub entry_shift: ::core::ffi::c_uint,
    pub entry_mask: ::core::ffi::c_uint,
    pub n_pages: ::core::ffi::c_uint,
    pub pages: [*mut ::core::ffi::c_void; 0],
}

#[inline]
pub unsafe fn TRACING_MAP_ARRAY_ELT(array: *mut tracing_map_array, idx: usize) -> *mut ::core::ffi::c_void {
    (*array).pages[idx >> (*array).entry_shift as usize].byte_add(
        (idx & (*array).entry_mask as usize) << (*array).entry_size_shift as usize)
}

#[inline]
pub unsafe fn TRACING_MAP_ENTRY(array: *mut tracing_map_array, idx: usize) -> *mut tracing_map_entry {
    TRACING_MAP_ARRAY_ELT(array, idx) as *mut tracing_map_entry
}

#[inline]
pub unsafe fn TRACING_MAP_ELT(array: *mut tracing_map_array, idx: usize) -> *mut *mut tracing_map_elt {
    TRACING_MAP_ARRAY_ELT(array, idx) as *mut *mut tracing_map_elt
}

#[repr(C)]
pub struct tracing_map {
    pub key_size: ::core::ffi::c_uint,
    pub map_bits: ::core::ffi::c_uint,
    pub map_size: ::core::ffi::c_uint,
    pub max_elts: ::core::ffi::c_uint,
    pub next_elt: atomic_t,
    pub elts: *mut tracing_map_array,
    pub map: *mut tracing_map_array,
    pub ops: *const tracing_map_ops,
    pub private_data: *mut ::core::ffi::c_void,
    pub fields: [tracing_map_field; TRACING_MAP_FIELDS_MAX],
    pub n_fields: ::core::ffi::c_uint,
    pub key_idx: [::core::ffi::c_int; TRACING_MAP_KEYS_MAX],
    pub n_keys: ::core::ffi::c_uint,
    pub sort_key: tracing_map_sort_key,
    pub n_vars: ::core::ffi::c_uint,
    pub hits: atomic64_t,
    pub drops: atomic64_t,
}

#[repr(C)]
pub struct tracing_map_ops {
    pub elt_alloc: Option<unsafe extern "C" fn(*mut tracing_map_elt) -> ::core::ffi::c_int>,
    pub elt_free: Option<unsafe extern "C" fn(*mut tracing_map_elt)>,
    pub elt_clear: Option<unsafe extern "C" fn(*mut tracing_map_elt)>,
    pub elt_init: Option<unsafe extern "C" fn(*mut tracing_map_elt)>,
}

unsafe extern "C" {
    pub fn tracing_map_create(map_bits: ::core::ffi::c_uint, key_size: ::core::ffi::c_uint,
                               ops: *const tracing_map_ops, private_data: *mut ::core::ffi::c_void) -> *mut tracing_map;
    pub fn tracing_map_init(map: *mut tracing_map) -> ::core::ffi::c_int;
    pub fn tracing_map_add_sum_field(map: *mut tracing_map) -> ::core::ffi::c_int;
    pub fn tracing_map_add_var(map: *mut tracing_map) -> ::core::ffi::c_int;
    pub fn tracing_map_add_key_field(map: *mut tracing_map, offset: ::core::ffi::c_uint,
                                     cmp_fn: tracing_map_cmp_fn_t) -> ::core::ffi::c_int;
    pub fn tracing_map_destroy(map: *mut tracing_map);
    pub fn tracing_map_clear(map: *mut tracing_map);
    pub fn tracing_map_insert(map: *mut tracing_map, key: *mut ::core::ffi::c_void) -> *mut tracing_map_elt;
    pub fn tracing_map_lookup(map: *mut tracing_map, key: *mut ::core::ffi::c_void) -> *mut tracing_map_elt;
    pub fn tracing_map_cmp_num(field_size: ::core::ffi::c_int, field_is_signed: ::core::ffi::c_int) -> tracing_map_cmp_fn_t;
    pub fn tracing_map_cmp_string(val_a: *mut ::core::ffi::c_void, val_b: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn tracing_map_cmp_none(val_a: *mut ::core::ffi::c_void, val_b: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    pub fn tracing_map_update_sum(elt: *mut tracing_map_elt, i: ::core::ffi::c_uint, n: u64);
    pub fn tracing_map_set_var(elt: *mut tracing_map_elt, i: ::core::ffi::c_uint, n: u64);
    pub fn tracing_map_var_set(elt: *mut tracing_map_elt, i: ::core::ffi::c_uint) -> bool;
    pub fn tracing_map_read_sum(elt: *mut tracing_map_elt, i: ::core::ffi::c_uint) -> u64;
    pub fn tracing_map_read_var(elt: *mut tracing_map_elt, i: ::core::ffi::c_uint) -> u64;
    pub fn tracing_map_read_var_once(elt: *mut tracing_map_elt, i: ::core::ffi::c_uint) -> u64;
    pub fn tracing_map_sort_entries(map: *mut tracing_map, sort_keys: *mut tracing_map_sort_key,
                                    n_sort_keys: ::core::ffi::c_uint,
                                    sort_entries: *mut *mut *mut tracing_map_sort_entry) -> ::core::ffi::c_int;
    pub fn tracing_map_destroy_sort_entries(entries: *mut *mut tracing_map_sort_entry,
                                            n_entries: ::core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
