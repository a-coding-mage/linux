/* SPDX-License-Identifier: GPL-2.0 */

/* Stage 6 definitions for creating trace events */

/* Reuse some of the stage 3 macros. */

/* __entry is a macro alias in the C header. */
macro_rules! __entry { () => { entry }; }

macro_rules! __field { ($type:ty, $item:ident) => {}; }
macro_rules! __field_struct { ($type:ty, $item:ident) => {}; }
macro_rules! __array { ($type:ty, $item:ident, $len:expr) => {}; }

macro_rules! __dynamic_array {
    ($type:ty, $item:ident, $len:expr) => {
        __entry!().__data_loc_$item = __data_offsets.item;
    };
}
macro_rules! __string { ($item:ident, $src:expr) => { __dynamic_array!(char, $item, -1); }; }
macro_rules! __string_len { ($item:ident, $src:expr, $len:expr) => { __dynamic_array!(char, $item, -1); }; }
macro_rules! __vstring { ($item:ident, $fmt:expr, $ap:expr) => { __dynamic_array!(char, $item, -1); }; }

macro_rules! __assign_str {
    ($dst:ident) => {{
        let __str__: *mut i8 = __get_str!($dst);
        let __len__: i32 = __get_dynamic_array_len!($dst) - 1;
        /* C token-pasting: __data_offsets.$dst##_ptr_. */
        unsafe {
            core::ptr::copy_nonoverlapping(
                if __data_offsets.$dst##_ptr_ != core::ptr::null() { __data_offsets.$dst##_ptr_ } else { EVENT_NULL_STR },
                __str__,
                __len__ as usize,
            );
            *__str__.add(__len__ as usize) = 0;
        }
    }};
}

macro_rules! __assign_vstr {
    ($dst:ident, $fmt:expr, $va:expr) => {{
        let mut __cp_va = $va.clone();
        vsnprintf!(__get_str!($dst), TRACE_EVENT_STR_MAX, $fmt, __cp_va);
        va_end!(__cp_va);
    }};
}

macro_rules! __bitmask { ($item:ident, $nr_bits:expr) => { __dynamic_array!(u64, $item, -1); }; }
macro_rules! __get_bitmask { ($field:ident) => { __get_dynamic_array!($field) as *mut i8 }; }
macro_rules! __assign_bitmask {
    ($dst:ident, $src:expr, $nr_bits:expr) => { memcpy!(__get_bitmask!($dst), $src, __bitmask_size_in_bytes!($nr_bits)); };
}
macro_rules! __cpumask { ($item:ident) => { __dynamic_array!(u64, $item, -1); }; }
macro_rules! __get_cpumask { ($field:ident) => { __get_dynamic_array!($field) as *mut i8 }; }
macro_rules! __assign_cpumask {
    ($dst:ident, $src:expr) => { memcpy!(__get_cpumask!($dst), $src, __bitmask_size_in_bytes!(nr_cpumask_bits)); };
}
macro_rules! __sockaddr { ($field:ident, $len:expr) => { __dynamic_array!(u8, $field, $len); }; }
macro_rules! __get_sockaddr { ($field:ident) => { __get_dynamic_array!($field) as *mut sockaddr }; }
macro_rules! __assign_sockaddr { ($dest:ident, $src:expr, $len:expr) => { memcpy!(__get_dynamic_array!($dest), $src, $len); }; }

macro_rules! __rel_dynamic_array {
    ($type:ty, $item:ident, $len:expr) => { __entry!().__rel_loc_$item = __data_offsets.item; };
}
macro_rules! __rel_string { ($item:ident, $src:expr) => { __rel_dynamic_array!(char, $item, -1); }; }
macro_rules! __rel_string_len { ($item:ident, $src:expr, $len:expr) => { __rel_dynamic_array!(char, $item, -1); }; }
macro_rules! __assign_rel_str {
    ($dst:ident) => {{
        let __str__: *mut i8 = __get_rel_str!($dst);
        let __len__: i32 = __get_rel_dynamic_array_len!($dst) - 1;
        /* C token-pasting: __data_offsets.$dst##_ptr_. */
        unsafe { *__str__.add(__len__ as usize) = 0; }
    }};
}
macro_rules! __rel_bitmask { ($item:ident, $nr_bits:expr) => { __rel_dynamic_array!(u64, $item, -1); }; }
macro_rules! __get_rel_bitmask { ($field:ident) => { __get_rel_dynamic_array!($field) as *mut i8 }; }
macro_rules! __assign_rel_bitmask { ($dst:ident, $src:expr, $nr_bits:expr) => { memcpy!(__get_rel_bitmask!($dst), $src, __bitmask_size_in_bytes!($nr_bits)); }; }
macro_rules! __rel_cpumask { ($item:ident) => { __rel_dynamic_array!(u64, $item, -1); }; }
macro_rules! __get_rel_cpumask { ($field:ident) => { __get_rel_dynamic_array!($field) as *mut i8 }; }
macro_rules! __assign_rel_cpumask { ($dst:ident, $src:expr) => { memcpy!(__get_rel_cpumask!($dst), $src, __bitmask_size_in_bytes!(nr_cpumask_bits)); }; }
macro_rules! __rel_sockaddr { ($field:ident, $len:expr) => { __rel_dynamic_array!(u8, $field, $len); }; }
macro_rules! __get_rel_sockaddr { ($field:ident) => { __get_rel_dynamic_array!($field) as *mut sockaddr }; }
macro_rules! __assign_rel_sockaddr { ($dest:ident, $src:expr, $len:expr) => { memcpy!(__get_rel_dynamic_array!($dest), $src, $len); }; }

macro_rules! TP_fast_assign { ($($args:tt)*) => { $($args)* }; }
macro_rules! __perf_count { ($c:expr) => { $c }; }
macro_rules! __perf_task { ($t:expr) => { $t }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
