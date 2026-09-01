/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2023 Meta Platforms, Inc. and affiliates.
 * Copyright (c) 2023 Tejun Heo <tj@kernel.org>
 * Copyright (c) 2023 David Vernet <dvernet@meta.com>
 */

/* C header guard and include directives omitted in Rust.
 *
 * Dependency intent from the original header:
 * - Must not be included by BPF programs when __KERNEL__ is defined.
 * - Depends on stdarg.h, stdio.h, stdlib.h, stdint.h, errno.h.
 * - Depends on enum_defs.autogen.h, user_exit_info.h, compat.h, enums.h,
 *   and bpf_arena_common.h.
 */

pub type u8 = ::std::os::raw::c_uchar;
pub type u16 = ::std::os::raw::c_ushort;
pub type u32 = ::std::os::raw::c_uint;
pub type u64 = ::std::os::raw::c_ulonglong;
pub type s8 = ::std::os::raw::c_schar;
pub type s16 = ::std::os::raw::c_short;
pub type s32 = ::std::os::raw::c_int;
pub type s64 = ::std::os::raw::c_longlong;

extern "C" {
    static mut errno: ::std::os::raw::c_int;

    fn strerror(errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    fn exit(status: ::std::os::raw::c_int) -> !;
}

pub const EXIT_FAILURE: ::std::os::raw::c_int = 1;

#[macro_export]
macro_rules! SCX_BUG {
    ($fmt:expr $(, $args:expr)* $(,)?) => {{
        unsafe {
            eprint!("[SCX_BUG] {}:{}", file!(), line!());
            if errno != 0 {
                let __err = strerror(errno);
                let __err = if __err.is_null() {
                    ::std::borrow::Cow::Borrowed("<null>")
                } else {
                    ::std::ffi::CStr::from_ptr(__err).to_string_lossy()
                };
                eprintln!(" ({})", __err);
            } else {
                eprintln!();
            }
            eprintln!($fmt $(, $args)*);

            exit(EXIT_FAILURE);
        }
    }};
}

#[macro_export]
macro_rules! SCX_BUG_ON {
    ($cond:expr, $fmt:expr $(, $args:expr)* $(,)?) => {{
        if $cond {
            SCX_BUG!($fmt $(, $args)*);
        }
    }};
}

/* RESIZE_ARRAY - Convenience macro for resizing a BPF array
 * @__skel: the skeleton containing the array
 * @elfsec: the data section of the BPF program in which the array exists
 * @arr: the name of the array
 * @n: the desired array element count
 *
 * For BPF arrays declared with RESIZABLE_ARRAY(), this macro performs two
 * operations. It resizes the map which corresponds to the custom data
 * section that contains the target array. As a side effect, the BTF info for
 * the array is adjusted so that the array length is sized to cover the new
 * data section size. The second operation is reassigning the skeleton pointer
 * for that custom data section so that it points to the newly memory mapped
 * region.
 *
 * The original C macro relies on token pasting, typeof(), and BPF skeleton
 * field names:
 *
 * bpf_map__set_value_size((__skel)->maps.elfsec##_##arr,
 *         sizeof((__skel)->elfsec##_##arr->arr[0]) * (n));
 * (__skel)->elfsec##_##arr =
 *     (typeof((__skel)->elfsec##_##arr))
 *     bpf_map__initial_value((__skel)->maps.elfsec##_##arr, &__sz);
 *
 * Rust macro_rules! cannot form field identifiers by token-pasting arbitrary
 * idents without an external helper, so callers must pass the already-pasted
 * Rust field identifiers.
 */
#[macro_export]
macro_rules! RESIZE_ARRAY {
    ($skel:expr, $map_field:ident, $section_field:ident, $arr:ident, $n:expr) => {{
        let mut __sz: usize = 0;
        unsafe {
            bpf_map__set_value_size(
                (*$skel).maps.$map_field,
                ::std::mem::size_of_val(&(*(*$skel).$section_field).$arr[0]) * ($n as usize),
            );
            (*$skel).$section_field = bpf_map__initial_value(
                (*$skel).maps.$map_field,
                &mut __sz as *mut usize,
            ) as _;
        }
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
