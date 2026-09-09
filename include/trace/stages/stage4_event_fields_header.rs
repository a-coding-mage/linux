/* SPDX-License-Identifier: GPL-2.0 */

/* Stage 4 definitions for creating trace events */

/* The C ALIGN_STRUCTFIELD(type) macro uses the alignment of a struct
 * containing one field of the specified type. */
macro_rules! ALIGN_STRUCTFIELD {
    ($type:ty) => {{
        ::core::mem::align_of::<$type>() as i32
    }};
}

macro_rules! __field_ext {
    ($type:ty, $item:ident, $filter_type:expr) => {
        .type = stringify!($type), .name = stringify!($item),
        .size = ::core::mem::size_of::<$type>(),
        .align = ALIGN_STRUCTFIELD!($type),
        .is_signed = is_signed_type::<$type>(), .filter_type = $filter_type,
    };
}

macro_rules! __field_struct_ext {
    ($type:ty, $item:ident, $filter_type:expr) => {
        .type = stringify!($type), .name = stringify!($item),
        .size = ::core::mem::size_of::<$type>(),
        .align = ALIGN_STRUCTFIELD!($type),
        .is_signed = 0, .filter_type = $filter_type,
    };
}

macro_rules! __field {
    ($type:ty, $item:ident) => { __field_ext!($type, $item, FILTER_OTHER) };
}

macro_rules! __field_struct {
    ($type:ty, $item:ident) => { __field_struct_ext!($type, $item, FILTER_OTHER) };
}

macro_rules! __array {
    ($type:ty, $item:ident, $len:expr) => {
        .type = concat!(stringify!($type), "[", stringify!($len), "]"),
        .name = stringify!($item),
        .size = ::core::mem::size_of::<[$type; $len]>(),
        .align = ALIGN_STRUCTFIELD!($type),
        .is_signed = is_signed_type::<$type>(), .filter_type = FILTER_OTHER,
        .len = $len,
    };
}

macro_rules! __dynamic_array {
    ($type:ty, $item:ident, $len:expr) => {
        .type = concat!("__data_loc ", stringify!($type), "[]"),
        .name = stringify!($item), .size = 4, .align = 4,
        .is_signed = is_signed_type::<$type>(), .filter_type = FILTER_OTHER,
    };
}

macro_rules! __string {
    ($item:ident, $src:expr) => { __dynamic_array!(char, $item, -1) };
}

macro_rules! __string_len {
    ($item:ident, $src:expr, $len:expr) => { __dynamic_array!(char, $item, -1) };
}

macro_rules! __vstring {
    ($item:ident, $fmt:expr, $ap:expr) => { __dynamic_array!(char, $item, -1) };
}

macro_rules! __bitmask {
    ($item:ident, $nr_bits:expr) => { __dynamic_array!(unsigned_long, $item, -1) };
}

macro_rules! __cpumask {
    ($item:ident) => {
        .type = "__data_loc cpumask_t", .name = stringify!($item),
        .size = 4, .align = 4, .is_signed = 0, .filter_type = FILTER_OTHER,
    };
}

macro_rules! __sockaddr {
    ($field:ident, $len:expr) => { __dynamic_array!(u8, $field, $len) };
}

macro_rules! __rel_dynamic_array {
    ($type:ty, $item:ident, $len:expr) => {
        .type = concat!("__rel_loc ", stringify!($type), "[]"),
        .name = stringify!($item), .size = 4, .align = 4,
        .is_signed = is_signed_type::<$type>(), .filter_type = FILTER_OTHER,
    };
}

macro_rules! __rel_string {
    ($item:ident, $src:expr) => { __rel_dynamic_array!(char, $item, -1) };
}

macro_rules! __rel_string_len {
    ($item:ident, $src:expr, $len:expr) => { __rel_dynamic_array!(char, $item, -1) };
}

macro_rules! __rel_bitmask {
    ($item:ident, $nr_bits:expr) => { __rel_dynamic_array!(unsigned_long, $item, -1) };
}

macro_rules! __rel_cpumask {
    ($item:ident) => {
        .type = "__rel_loc cpumask_t", .name = stringify!($item),
        .size = 4, .align = 4, .is_signed = 0, .filter_type = FILTER_OTHER,
    };
}

macro_rules! __rel_sockaddr {
    ($field:ident, $len:expr) => { __rel_dynamic_array!(u8, $field, $len) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
