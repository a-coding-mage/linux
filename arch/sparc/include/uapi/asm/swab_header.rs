/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent: `__u16`, `__u32`, `__u64`, and `ASI_PL` are supplied by
// the corresponding Linux types and SPARC ASI definitions.

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[inline]
pub unsafe fn __arch_swab16p(addr: *const u16) -> u16 {
    let ret: u16;
    core::arch::asm!(
        "lduha [{addr}] {asi}, {ret}",
        addr = in(reg) addr,
        asi = const ASI_PL,
        ret = out(reg) ret,
        options(nostack, preserves_flags),
    );
    ret
}

// C macro alias: #define __arch_swab16p __arch_swab16p

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[inline]
pub unsafe fn __arch_swab32p(addr: *const u32) -> u32 {
    let ret: u32;
    core::arch::asm!(
        "lduwa [{addr}] {asi}, {ret}",
        addr = in(reg) addr,
        asi = const ASI_PL,
        ret = out(reg) ret,
        options(nostack, preserves_flags),
    );
    ret
}

// C macro alias: #define __arch_swab32p __arch_swab32p

#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
#[inline]
pub unsafe fn __arch_swab64p(addr: *const u64) -> u64 {
    let ret: u64;
    core::arch::asm!(
        "ldxa [{addr}] {asi}, {ret}",
        addr = in(reg) addr,
        asi = const ASI_PL,
        ret = out(reg) ret,
        options(nostack, preserves_flags),
    );
    ret
}

// C macro alias: #define __arch_swab64p __arch_swab64p

// Equivalent of the build-time fallback:
// #define __SWAB_64_THRU_32__
#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
pub const __SWAB_64_THRU_32__: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
