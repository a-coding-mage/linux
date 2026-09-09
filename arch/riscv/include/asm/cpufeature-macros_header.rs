/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2022-2024 Rivos, Inc
 */

// Translated from cpufeature-macros.h.  The included kernel definitions are
// supplied by other translation units.

pub const STANDARD_EXT: ::core::ffi::c_ulong = 0;

extern "C" {
    pub fn __riscv_isa_extension_available(
        isa_bitmap: *const ::core::ffi::c_ulong,
        bit: ::core::ffi::c_uint,
    ) -> bool;
}

#[macro_export]
macro_rules! riscv_isa_extension_available {
    ($isa_bitmap:expr, $ext:ident) => {
        unsafe {
            $crate::__riscv_isa_extension_available(
                $isa_bitmap,
                concat_idents!(RISCV_ISA_EXT_, $ext),
            )
        }
    };
}

#[inline(always)]
pub unsafe fn __riscv_has_extension_likely(
    vendor: ::core::ffi::c_ulong,
    ext: ::core::ffi::c_ulong,
) -> bool {
    // C asm goto(ALTERNATIVE(...)) applies a runtime-patched branch. Rust has
    // no direct equivalent for this kernel-specific asm-goto construct.
    let _ = (vendor, ext);
    true
}

#[inline(always)]
pub unsafe fn __riscv_has_extension_unlikely(
    vendor: ::core::ffi::c_ulong,
    ext: ::core::ffi::c_ulong,
) -> bool {
    // C asm goto(ALTERNATIVE(...)) applies a runtime-patched branch. Rust has
    // no direct equivalent for this kernel-specific asm-goto construct.
    let _ = (vendor, ext);
    false
}

#[inline(always)]
pub unsafe fn riscv_has_extension_unlikely(ext: ::core::ffi::c_ulong) -> bool {
    // compiletime_assert(ext < RISCV_ISA_EXT_MAX, "ext must be < RISCV_ISA_EXT_MAX");
    if IS_ENABLED(CONFIG_RISCV_ALTERNATIVE) {
        return __riscv_has_extension_unlikely(STANDARD_EXT, ext);
    }

    __riscv_isa_extension_available(::core::ptr::null(), ext as ::core::ffi::c_uint)
}

#[inline(always)]
pub unsafe fn riscv_has_extension_likely(ext: ::core::ffi::c_ulong) -> bool {
    // compiletime_assert(ext < RISCV_ISA_EXT_MAX, "ext must be < RISCV_ISA_EXT_MAX");
    if IS_ENABLED(CONFIG_RISCV_ALTERNATIVE) {
        return __riscv_has_extension_likely(STANDARD_EXT, ext);
    }

    __riscv_isa_extension_available(::core::ptr::null(), ext as ::core::ffi::c_uint)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
