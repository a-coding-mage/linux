/* SPDX-License-Identifier: GPL-2.0 */

/* Stage 5 definitions for creating trace events. */
/* Remember the offset of each array from the beginning of the event. */

/* C preprocessor state: __entry is defined as entry. */

#[inline]
pub unsafe fn __string_src(str_: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    if str_.is_null() {
        EVENT_NULL_STR
    } else {
        str_
    }
}

/* EVENT_NULL_STR is supplied by the surrounding trace-event definitions. */
extern "C" {
    static EVENT_NULL_STR: *const ::core::ffi::c_char;
}

/*
 * Fields should never declare an array: i.e. __field(int, arr[5]).
 * If they do, it will cause issues in parsing and possibly corrupt the
 * events.  Use __array instead.
 */

/* The following macros preserve the source header's macro-based interface. */
#[macro_export]
macro_rules! __field {
    ($type:ty, $item:ident) => {{
        let _ = ::core::marker::PhantomData::<$type>;
        let _ = stringify!($item);
    }};
}

#[macro_export]
macro_rules! __field_ext {
    ($type:ty, $item:ident, $filter_type:ty) => {{
        let _ = ::core::marker::PhantomData::<$type>;
        let _ = ::core::marker::PhantomData::<$filter_type>;
        let _ = stringify!($item);
    }};
}

#[macro_export]
macro_rules! __field_struct {
    ($type:ty, $item:ident) => {{
        let _ = ::core::marker::PhantomData::<$type>;
        let _ = stringify!($item);
    }};
}

#[macro_export]
macro_rules! __field_struct_ext {
    ($type:ty, $item:ident, $filter_type:ty) => {{
        let _ = ::core::marker::PhantomData::<$type>;
        let _ = ::core::marker::PhantomData::<$filter_type>;
        let _ = stringify!($item);
    }};
}

#[macro_export]
macro_rules! __array {
    ($type:ty, $item:ident, $len:expr) => {};
}

#[macro_export]
macro_rules! __dynamic_array {
    ($type:ty, $item:ident, $len:expr) => {{
        __item_length = ($len) * ::core::mem::size_of::<$type>();
        __data_offsets.$item = __data_size +
            ::core::mem::offset_of!(typeof!(*entry), __data);
        __data_offsets.$item |= __item_length << 16;
        __data_size += __item_length;
    }};
}

#[macro_export]
macro_rules! __string {
    ($item:ident, $src:expr) => {{
        $crate::__dynamic_array!(::core::ffi::c_char, $item,
            unsafe { ::core::ffi::CStr::from_ptr($crate::__string_src($src)).to_bytes().len() + 1 });
        __data_offsets.$item##_ptr_ = $src;
    }};
}

#[macro_export]
macro_rules! __string_len {
    ($item:ident, $src:expr, $len:expr) => {{
        $crate::__dynamic_array!(::core::ffi::c_char, $item, ($len) + 1);
        __data_offsets.$item##_ptr_ = $src;
    }};
}

#[macro_export]
macro_rules! __vstring {
    ($item:ident, $fmt:expr, $ap:expr) => {{
        $crate::__dynamic_array!(::core::ffi::c_char, $item,
            unsafe { __trace_event_vstr_len($fmt, $ap) });
    }};
}

#[macro_export]
macro_rules! __rel_dynamic_array {
    ($type:ty, $item:ident, $len:expr) => {{
        __item_length = ($len) * ::core::mem::size_of::<$type>();
        __data_offsets.$item = __data_size +
            ::core::mem::offset_of!(typeof!(*entry), __data) -
            ::core::mem::offset_of!(typeof!(*entry), __rel_loc_$item) -
            ::core::mem::size_of::<u32>();
        __data_offsets.$item |= __item_length << 16;
        __data_size += __item_length;
    }};
}

#[macro_export]
macro_rules! __rel_string {
    ($item:ident, $src:expr) => {{
        $crate::__rel_dynamic_array!(::core::ffi::c_char, $item,
            unsafe { ::core::ffi::CStr::from_ptr($crate::__string_src($src)).to_bytes().len() + 1 });
        __data_offsets.$item##_ptr_ = $src;
    }};
}

#[macro_export]
macro_rules! __rel_string_len {
    ($item:ident, $src:expr, $len:expr) => {{
        $crate::__rel_dynamic_array!(::core::ffi::c_char, $item, ($len) + 1);
        __data_offsets.$item##_ptr_ = $src;
    }};
}

#[macro_export]
macro_rules! __bitmask_size_in_bytes_raw { ($nr_bits:expr) => { (($nr_bits) + 7) / 8 }; }
#[macro_export]
macro_rules! __bitmask_size_in_longs { ($nr_bits:expr) => {
    ($crate::__bitmask_size_in_bytes_raw!($nr_bits) + ((BITS_PER_LONG / 8) - 1)) / (BITS_PER_LONG / 8)
}; }
#[macro_export]
macro_rules! __bitmask_size_in_bytes { ($nr_bits:expr) => {
    $crate::__bitmask_size_in_longs!($nr_bits) * (BITS_PER_LONG / 8)
}; }

#[macro_export]
macro_rules! __bitmask { ($item:ident, $nr_bits:expr) => {
    $crate::__dynamic_array!(usize, $item, $crate::__bitmask_size_in_longs!($nr_bits));
}; }
#[macro_export]
macro_rules! __cpumask { ($item:ident) => { $crate::__bitmask!($item, nr_cpumask_bits); }; }
#[macro_export]
macro_rules! __rel_bitmask { ($item:ident, $nr_bits:expr) => {
    $crate::__rel_dynamic_array!(usize, $item, $crate::__bitmask_size_in_longs!($nr_bits));
}; }
#[macro_export]
macro_rules! __rel_cpumask { ($item:ident) => { $crate::__rel_bitmask!($item, nr_cpumask_bits); }; }
#[macro_export]
macro_rules! __sockaddr { ($field:ident, $len:expr) => { $crate::__dynamic_array!(u8, $field, $len); }; }
#[macro_export]
macro_rules! __rel_sockaddr { ($field:ident, $len:expr) => { $crate::__rel_dynamic_array!(u8, $field, $len); }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
