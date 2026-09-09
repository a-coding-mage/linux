/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// linux/types.h, asm/cpufeature-macros.h, asm/hwcap.h,
// asm-generic/bitops/__bitrev.h

extern "C" {
    fn riscv_has_extension_likely(extension: u32) -> bool;
    fn generic___bitrev32(x: u32) -> u32;
    fn generic___bitrev8(x: u8) -> u8;
}

// RISCV_ISA_EXT_ZBKB and __riscv_xlen are supplied by the target build.
extern "C" {
    static RISCV_ISA_EXT_ZBKB: u32;
    static __riscv_xlen: u32;
}

#[inline(always)]
pub unsafe fn __arch_bitrev32(x: u32) -> u32 {
    let result: usize;

    if !riscv_has_extension_likely(RISCV_ISA_EXT_ZBKB) {
        return generic___bitrev32(x);
    }

    core::arch::asm!(
        ".option push",
        ".option arch,+zbkb",
        "rev8 {0}, {1}",
        "brev8 {0}, {0}",
        ".option pop",
        out(reg) result,
        in(reg) (x as isize),
    );

    (result >> (__riscv_xlen - 32)) as u32
}

#[inline(always)]
pub unsafe fn __arch_bitrev16(x: u16) -> u16 {
    (__arch_bitrev32(x as u32) >> 16) as u16
}

#[inline(always)]
pub unsafe fn __arch_bitrev8(x: u8) -> u8 {
    let result: usize;

    if !riscv_has_extension_likely(RISCV_ISA_EXT_ZBKB) {
        return generic___bitrev8(x);
    }

    core::arch::asm!(
        ".option push",
        ".option arch,+zbkb",
        "brev8 {0}, {1}",
        ".option pop",
        out(reg) result,
        in(reg) (x as isize),
    );

    result as u8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
