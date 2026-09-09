/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from the RISC-V swab header.
// The C condition
// defined(CONFIG_TOOLCHAIN_HAS_ZBB) && defined(CONFIG_RISCV_ISA_ZBB) &&
// !defined(NO_ALTERNATIVE) is a build-time condition supplied externally.

#[inline(always)]
pub const fn ___constant_swab16(x: u16) -> u16 {
    (((x & 0x00ff) << 8) | ((x & 0xff00) >> 8)) as u16
}

#[inline(always)]
pub const fn ___constant_swab32(x: u32) -> u32 {
    (((x & 0x000000ff) << 24)
        | ((x & 0x0000ff00) << 8)
        | ((x & 0x00ff0000) >> 8)
        | ((x & 0xff000000) >> 24)) as u32
}

#[inline(always)]
pub const fn ___constant_swab64(x: u64) -> u64 {
    (((x & 0x00000000000000ff) << 56)
        | ((x & 0x000000000000ff00) << 40)
        | ((x & 0x0000000000ff0000) << 24)
        | ((x & 0x00000000ff000000) << 8)
        | ((x & 0x000000ff00000000) >> 8)
        | ((x & 0x0000ff0000000000) >> 24)
        | ((x & 0x00ff000000000000) >> 40)
        | ((x & 0xff00000000000000) >> 56)) as u64
}

#[inline(always)]
fn arch_swab(size: u32, value: usize) -> usize {
    let mut x = value;

    // `riscv_has_extension_likely` and `RISCV_ISA_EXT_ZBB` are supplied by
    // the architecture feature dependencies of this header.
    if riscv_has_extension_likely(RISCV_ISA_EXT_ZBB) {
        // Corresponds to the volatile RISC-V `rev8` alternative sequence.
        unsafe {
            core::arch::asm!(
                ".option push",
                ".option arch,+zbb",
                "rev8 {0}, {0}",
                ".option pop",
                inout(reg) x,
                options(nostack)
            );
        }
        x >>= (BITS_PER_LONG - size);
    } else {
        x = match size {
            16 => ___constant_swab16(value as u16) as usize,
            32 => ___constant_swab32(value as u32) as usize,
            64 => ___constant_swab64(value as u64) as usize,
            _ => x,
        };
    }
    x
}

#[inline(always)]
pub fn __arch_swab16(value: u16) -> u16 {
    arch_swab(16, value as usize) as u16
}

#[inline(always)]
pub fn __arch_swab32(value: u32) -> u32 {
    arch_swab(32, value as usize) as u32
}

#[cfg(target_pointer_width = "64")]
#[inline(always)]
pub fn __arch_swab64(value: u64) -> u64 {
    arch_swab(64, value as usize) as u64
}

#[cfg(not(target_pointer_width = "64"))]
#[inline(always)]
pub fn __arch_swab64(value: u64) -> u64 {
    let h = (value >> 32) as u32;
    let l = (value & ((1u64 << 32) - 1)) as u32;

    ((__arch_swab32(l) as u64) << 32) | (__arch_swab32(h) as u64)
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
