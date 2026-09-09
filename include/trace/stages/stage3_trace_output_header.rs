/* SPDX-License-Identifier: GPL-2.0 */

/* Stage 3 definitions for creating trace events */

/* C preprocessor: __entry expands to field. */
/* C preprocessor: TP_printk(fmt, args...) expands to fmt "\n", args. */

#[macro_export]
macro_rules! __get_dynamic_array {
    ($entry:expr, $field:ident) => {
        (($entry as *mut u8).wrapping_add(((*$entry).__data_loc_$field & 0xffff) as usize)) as *mut core::ffi::c_void
    };
}

#[macro_export]
macro_rules! __get_dynamic_array_len {
    ($entry:expr, $field:ident) => {
        (((*$entry).__data_loc_$field >> 16) & 0xffff)
    };
}

#[macro_export]
macro_rules! __get_str {
    ($entry:expr, $field:ident) => {
        $crate::__get_dynamic_array!($entry, $field) as *mut core::ffi::c_char
    };
}

#[macro_export]
macro_rules! __get_rel_dynamic_array {
    ($entry:expr, $field:ident, $entry_type:ty) => {
        (($entry as *mut u8)
            .wrapping_add(core::mem::offset_of!($entry_type, __rel_loc_$field))
            .wrapping_add(core::mem::size_of_val(&(*$entry).__rel_loc_$field))
            .wrapping_add(((*$entry).__rel_loc_$field & 0xffff) as usize)) as *mut core::ffi::c_void
    };
}

#[macro_export]
macro_rules! __get_rel_dynamic_array_len {
    ($entry:expr, $field:ident) => {
        (((*$entry).__rel_loc_$field >> 16) & 0xffff)
    };
}

#[macro_export]
macro_rules! __get_rel_str {
    ($entry:expr, $field:ident, $entry_type:ty) => {
        $crate::__get_rel_dynamic_array!($entry, $field, $entry_type) as *mut core::ffi::c_char
    };
}

#[macro_export]
macro_rules! __get_bitmask {
    ($entry:expr, $field:ident, $iter:expr) => {{
        let __bitmask = $crate::__get_dynamic_array!($entry, $field);
        let __bitmask_size: u32 = $crate::__get_dynamic_array_len!($entry, $field);
        trace_print_bitmask_seq($iter, __bitmask, __bitmask_size);
    }};
}

#[macro_export]
macro_rules! __get_cpumask {
    ($($args:tt)*) => { $crate::__get_bitmask!($($args)*) };
}

#[macro_export]
macro_rules! __get_rel_bitmask {
    ($entry:expr, $field:ident, $entry_type:ty, $iter:expr) => {{
        let __bitmask = $crate::__get_rel_dynamic_array!($entry, $field, $entry_type);
        let __bitmask_size: u32 = $crate::__get_rel_dynamic_array_len!($entry, $field);
        trace_print_bitmask_seq($iter, __bitmask, __bitmask_size);
    }};
}

#[macro_export]
macro_rules! __get_rel_cpumask {
    ($($args:tt)*) => { $crate::__get_rel_bitmask!($($args)*) };
}

#[macro_export]
macro_rules! __get_sockaddr {
    ($entry:expr, $field:ident) => {
        $crate::__get_dynamic_array!($entry, $field) as *mut sockaddr
    };
}

#[macro_export]
macro_rules! __get_rel_sockaddr {
    ($entry:expr, $field:ident, $entry_type:ty) => {
        $crate::__get_rel_dynamic_array!($entry, $field, $entry_type) as *mut sockaddr
    };
}

/* __print_flags and __print_symbolic retain their C variadic-array intent. */
#[macro_export]
macro_rules! __print_flags {
    ($flag:expr, $delim:expr, $($flag_array:expr),+ $(,)?) => {{
        static __FLAGS: &[trace_print_flags] = &[$($flag_array),+];
        trace_print_flags_seq(p, $delim, $flag, __FLAGS.as_ptr(), __FLAGS.len());
    }};
}

#[macro_export]
macro_rules! __print_symbolic {
    ($value:expr, $($symbol_array:expr),+ $(,)?) => {{
        static SYMBOLS: &[trace_print_flags] = &[$($symbol_array),+];
        trace_print_symbols_seq(p, $value, SYMBOLS.as_ptr(), SYMBOLS.len());
    }};
}

/* #if BITS_PER_LONG == 32: use the u64-specific trace printers. */
#[macro_export]
macro_rules! __print_flags_u64 {
    ($flag:expr, $delim:expr, $($flag_array:expr),+ $(,)?) => {
        __print_flags!($flag, $delim, $($flag_array),+)
    };
}

#[macro_export]
macro_rules! __print_symbolic_u64 {
    ($value:expr, $($symbol_array:expr),+ $(,)?) => {
        __print_symbolic!($value, $($symbol_array),+)
    };
}

#[macro_export]
macro_rules! __print_hex {
    ($buf:expr, $buf_len:expr) => { trace_print_hex_seq(p, $buf, $buf_len, false) };
}

#[macro_export]
macro_rules! __print_hex_str {
    ($buf:expr, $buf_len:expr) => { trace_print_hex_seq(p, $buf, $buf_len, true) };
}

#[macro_export]
macro_rules! __print_array {
    ($array:expr, $count:expr, $el_size:expr) => {{
        BUILD_BUG_ON!($el_size != 1 && $el_size != 2 && $el_size != 4 && $el_size != 8);
        trace_print_array_seq(p, $array, $count, $el_size);
    }};
}

#[macro_export]
macro_rules! __print_dynamic_array {
    ($entry:expr, $array:ident, $el_size:expr) => {
        __print_array!($crate::__get_dynamic_array!($entry, $array),
                       $crate::__get_dynamic_array_len!($entry, $array) / ($el_size),
                       $el_size)
    };
}

#[macro_export]
macro_rules! __print_hex_dump {
    ($prefix_str:expr, $prefix_type:expr, $rowsize:expr, $groupsize:expr,
     $buf:expr, $len:expr, $ascii:expr) => {
        trace_print_hex_dump_seq(p, $prefix_str, $prefix_type, $rowsize,
                                 $groupsize, $buf, $len, $ascii)
    };
}

#[macro_export]
macro_rules! __print_ns_to_secs {
    ($value:expr) => {{
        let mut ____val: u64 = $value as u64;
        ____val /= NSEC_PER_SEC;
        ____val
    }};
}

#[macro_export]
macro_rules! __print_ns_without_secs {
    ($value:expr) => {{
        let mut ____val: u64 = $value as u64;
        let result = ____val;
        ____val /= NSEC_PER_SEC;
        result as u32
    }};
}

#[macro_export]
macro_rules! __get_buf {
    ($len:expr) => { trace_seq_acquire(p, $len) };
}

#[inline]
pub unsafe fn __event_in_hardirq(entry: *const EntryWithEnt) -> u32 {
    (*entry).ent.flags & TRACE_FLAG_HARDIRQ
}

#[inline]
pub unsafe fn __event_in_softirq(entry: *const EntryWithEnt) -> u32 {
    (*entry).ent.flags & TRACE_FLAG_SOFTIRQ
}

#[inline]
pub unsafe fn __event_in_irq(entry: *const EntryWithEnt) -> u32 {
    (*entry).ent.flags & (TRACE_FLAG_HARDIRQ | TRACE_FLAG_SOFTIRQ)
}

/* EntryWithEnt corresponds to the translation unit's typeof(*__entry). */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
