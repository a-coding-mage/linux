// SPDX-License-Identifier: GPL-2.0
// Translated from include/asm-generic/bitops/arch_hweight.h.
// C dependency intent: #include <asm/types.h> supplies __u64.

pub type __u64 = u64;

extern "C" {
    fn __sw_hweight32(w: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
    fn __sw_hweight16(w: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
    fn __sw_hweight8(w: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
    fn __sw_hweight64(w: __u64) -> ::std::os::raw::c_ulong;
}

#[inline]
pub unsafe fn __arch_hweight32(w: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint {
    unsafe { __sw_hweight32(w) }
}

#[inline]
pub unsafe fn __arch_hweight16(w: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint {
    unsafe { __sw_hweight16(w) }
}

#[inline]
pub unsafe fn __arch_hweight8(w: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint {
    unsafe { __sw_hweight8(w) }
}

#[inline]
pub unsafe fn __arch_hweight64(w: __u64) -> ::std::os::raw::c_ulong {
    unsafe { __sw_hweight64(w) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
