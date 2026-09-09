/* SPDX-License-Identifier: GPL-2.0-only */

/* Kernel symbol flags bitset. */
#[repr(C)]
pub enum ksym_flags {
    KSYM_FLAG_GPL_ONLY = 1 << 0,
}

/* This ignores the intensely annoying "mapping symbols" found in ELF files. */
#[inline]
pub unsafe fn is_mapping_symbol(str_: *const core::ffi::c_char) -> bool {
    if *str_ == b'.' as core::ffi::c_char &&
       *str_.add(1) == b'L' as core::ffi::c_char
    {
        return true;
    }
    if *str_ == b'L' as core::ffi::c_char &&
       *str_.add(1) == b'0' as core::ffi::c_char
    {
        return true;
    }
    *str_ == b'$' as core::ffi::c_char
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
