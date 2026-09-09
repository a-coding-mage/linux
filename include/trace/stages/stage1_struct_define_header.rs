/* SPDX-License-Identifier: GPL-2.0 */

/* Stage 1 definitions for creating trace events */

/* C preprocessor macros translated as Rust macros.  The C token-pasting
 * dynamic-location field names are represented by the supplied item name. */
macro_rules! __field {
    ($type:ty, $item:ident) => { $item: $type, };
}

macro_rules! __field_ext {
    ($type:ty, $item:ident, $filter_type:ty) => { $item: $type, };
}

macro_rules! __field_struct {
    ($type:ty, $item:ident) => { $item: $type, };
}

macro_rules! __field_struct_ext {
    ($type:ty, $item:ident, $filter_type:ty) => { $item: $type, };
}

macro_rules! __array {
    ($type:ty, $item:ident, $len:expr) => { $item: [$type; $len], };
}

macro_rules! __dynamic_array {
    ($type:ty, $item:ident, $len:expr) => { $item: u32, };
}

macro_rules! __string {
    ($item:ident, $src:expr) => { __dynamic_array!(u8, $item, -1); };
}

macro_rules! __string_len {
    ($item:ident, $src:expr, $len:expr) => { __dynamic_array!(u8, $item, -1); };
}

macro_rules! __vstring {
    ($item:ident, $fmt:expr, $ap:expr) => { __dynamic_array!(u8, $item, -1); };
}

macro_rules! __bitmask {
    ($item:ident, $nr_bits:expr) => { __dynamic_array!(u8, $item, -1); };
}

macro_rules! __cpumask {
    ($item:ident) => { __dynamic_array!(u8, $item, -1); };
}

macro_rules! __sockaddr {
    ($field:ident, $len:expr) => { __dynamic_array!(u8, $field, $len); };
}

macro_rules! __rel_dynamic_array {
    ($type:ty, $item:ident, $len:expr) => { $item: u32, };
}

macro_rules! __rel_string {
    ($item:ident, $src:expr) => { __rel_dynamic_array!(u8, $item, -1); };
}

macro_rules! __rel_string_len {
    ($item:ident, $src:expr, $len:expr) => { __rel_dynamic_array!(u8, $item, -1); };
}

macro_rules! __rel_bitmask {
    ($item:ident, $nr_bits:expr) => { __rel_dynamic_array!(u8, $item, -1); };
}

macro_rules! __rel_cpumask {
    ($item:ident) => { __rel_dynamic_array!(u8, $item, -1); };
}

macro_rules! __rel_sockaddr {
    ($field:ident, $len:expr) => { __rel_dynamic_array!(u8, $field, $len); };
}

macro_rules! TP_STRUCT__entry {
    ($($args:tt)*) => { $($args)* };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
