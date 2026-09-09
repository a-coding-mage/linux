/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Based on arch/x86/include/asm/arch_hweight.h
 */

// C header guards and includes omitted; required symbols are supplied by
// other translated headers.

#[cfg(target_pointer_width = "64")]
const CPOPW: &str = "cpopw ";
#[cfg(target_pointer_width = "32")]
const CPOPW: &str = "cpop ";

extern "C" {
    fn __sw_hweight32(w: core::ffi::c_uint) -> core::ffi::c_uint;
    fn __sw_hweight64(w: u64) -> core::ffi::c_ulong;
    fn riscv_has_extension_likely(extension: core::ffi::c_uint) -> bool;
}

// IS_ENABLED(CONFIG_RISCV_ISA_ZBB), IS_ENABLED(CONFIG_TOOLCHAIN_HAS_ZBB),
// and RISCV_ISA_EXT_ZBB are supplied by the build configuration and headers.

#[inline(always)]
pub unsafe fn __arch_hweight32(mut w: core::ffi::c_uint) -> core::ffi::c_uint {
    if !(cfg!(feature = "CONFIG_RISCV_ISA_ZBB")
        && cfg!(feature = "CONFIG_TOOLCHAIN_HAS_ZBB")
        && riscv_has_extension_likely(RISCV_ISA_EXT_ZBB))
    {
        return __sw_hweight32(w);
    }

    core::arch::asm!(
        ".option push",
        ".option arch,+zbb",
        "cpopw {0}, {1}",
        ".option pop",
        inout(reg) w,
    );

    w
}

#[inline]
pub unsafe fn __arch_hweight16(w: core::ffi::c_uint) -> core::ffi::c_uint {
    __arch_hweight32(w & 0xffff)
}

#[inline]
pub unsafe fn __arch_hweight8(w: core::ffi::c_uint) -> core::ffi::c_uint {
    __arch_hweight32(w & 0xff)
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub unsafe fn __arch_hweight64(mut w: u64) -> core::ffi::c_ulong {
    if !(cfg!(feature = "CONFIG_RISCV_ISA_ZBB")
        && cfg!(feature = "CONFIG_TOOLCHAIN_HAS_ZBB")
        && riscv_has_extension_likely(RISCV_ISA_EXT_ZBB))
    {
        return __sw_hweight64(w);
    }

    core::arch::asm!(
        ".option push",
        ".option arch,+zbb",
        "cpop {0}, {1}",
        ".option pop",
        inout(reg) w,
    );

    w
}

// BITS_PER_LONG != 64
#[cfg(not(target_pointer_width = "64"))]
#[inline]
pub unsafe fn __arch_hweight64(w: u64) -> core::ffi::c_ulong {
    (__arch_hweight32(w as u32 as core::ffi::c_uint)
        + __arch_hweight32((w >> 32) as u32 as core::ffi::c_uint)) as core::ffi::c_ulong
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
