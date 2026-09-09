/* SPDX-License-Identifier: GPL-2.0 */

/* Stage 2 definitions for creating trace events */

/* C preprocessor #undef directives have no Rust equivalent. */

macro_rules! TRACE_DEFINE_ENUM {
    ($a:expr) => {};
}

macro_rules! TRACE_DEFINE_SIZEOF {
    ($a:expr) => {};
}

macro_rules! __field {
    ($type:ty, $item:ident) => {};
}

macro_rules! __field_ext {
    ($type:ty, $item:ident, $filter_type:ty) => {};
}

macro_rules! __field_struct {
    ($type:ty, $item:ident) => {};
}

macro_rules! __field_struct_ext {
    ($type:ty, $item:ident, $filter_type:ty) => {};
}

macro_rules! __array {
    ($type:ty, $item:ident, $len:expr) => {};
}

/* Rust cannot directly form an identifier by concatenating $item and _ptr_. */
macro_rules! __dynamic_array {
    ($type:ty, $item:ident, $len:expr, $item_ptr:ident) => {
        $item: u32,
        $item_ptr: *const core::ffi::c_void,
    };
}

macro_rules! __string {
    ($item:ident, $src:expr, $item_ptr:ident) => {
        __dynamic_array!(char, $item, -1, $item_ptr);
    };
}

macro_rules! __string_len {
    ($item:ident, $src:expr, $len:expr, $item_ptr:ident) => {
        __dynamic_array!(char, $item, -1, $item_ptr);
    };
}

macro_rules! __vstring {
    ($item:ident, $fmt:expr, $ap:expr, $item_ptr:ident) => {
        __dynamic_array!(char, $item, -1, $item_ptr);
    };
}

macro_rules! __bitmask {
    ($item:ident, $nr_bits:expr, $item_ptr:ident) => {
        __dynamic_array!(unsigned_long, $item, -1, $item_ptr);
    };
}

macro_rules! __cpumask {
    ($item:ident, $item_ptr:ident) => {
        __dynamic_array!(unsigned_long, $item, -1, $item_ptr);
    };
}

macro_rules! __sockaddr {
    ($field:ident, $len:expr, $field_ptr:ident) => {
        __dynamic_array!(u8, $field, $len, $field_ptr);
    };
}

macro_rules! __rel_dynamic_array {
    ($type:ty, $item:ident, $len:expr, $item_ptr:ident) => {
        $item: u32,
        $item_ptr: *const core::ffi::c_void,
    };
}

macro_rules! __rel_string {
    ($item:ident, $src:expr, $item_ptr:ident) => {
        __rel_dynamic_array!(char, $item, -1, $item_ptr);
    };
}

macro_rules! __rel_string_len {
    ($item:ident, $src:expr, $len:expr, $item_ptr:ident) => {
        __rel_dynamic_array!(char, $item, -1, $item_ptr);
    };
}

macro_rules! __rel_bitmask {
    ($item:ident, $nr_bits:expr, $item_ptr:ident) => {
        __rel_dynamic_array!(unsigned_long, $item, -1, $item_ptr);
    };
}

macro_rules! __rel_cpumask {
    ($item:ident, $item_ptr:ident) => {
        __rel_dynamic_array!(unsigned_long, $item, -1, $item_ptr);
    };
}

macro_rules! __rel_sockaddr {
    ($field:ident, $len:expr, $field_ptr:ident) => {
        __rel_dynamic_array!(u8, $field, $len, $field_ptr);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
