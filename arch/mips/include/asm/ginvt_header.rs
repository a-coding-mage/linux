/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding MIPS environment:
// #include <asm/mipsregs.h>

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ginvt_type {
    GINVT_FULL = 0,
    GINVT_VA = 1,
    GINVT_MMID = 2,
}

// When the toolchain supports the GINV assembler directive, use `.set ginv`.
// Otherwise the original header defines the `ginvt` assembler macro using the
// MIPS/MM instruction encodings and removes it with `.purgem ginvt`.
#[cfg(feature = "toolchain_supports_ginv")]
const _ASM_SET_GINV: &str = ".set\tginv\n";
#[cfg(feature = "toolchain_supports_ginv")]
const _ASM_UNSET_GINV: &str = "";

#[cfg(not(feature = "toolchain_supports_ginv"))]
const _ASM_SET_GINV: &str = ""; // _ASM_MACRO_1R1I / _ASM_INSN_IF_MIPS / _ASM_INSN32_IF_MM are external assembler macros.
#[cfg(not(feature = "toolchain_supports_ginv"))]
const _ASM_UNSET_GINV: &str = ".purgem ginvt\n";

#[inline(always)]
pub unsafe fn ginvt(addr: ::core::ffi::c_ulong, type_: u32) {
    ::core::arch::asm!(
        ".set\tpush",
        "{set_ginv}\\n",
        "\tginvt\t{addr}, {type_}",
        "{unset_ginv}\\n",
        ".set\tpop",
        set_ginv = const _ASM_SET_GINV,
        unset_ginv = const _ASM_UNSET_GINV,
        addr = in(reg) addr,
        type_ = const type_ as u8,
        options(nostack),
    );
}

#[inline]
pub unsafe fn ginvt_full() {
    ginvt(0, ginvt_type::GINVT_FULL as u32);
}

#[inline]
pub unsafe fn ginvt_va(mut addr: ::core::ffi::c_ulong) {
    // PAGE_MASK is supplied by the surrounding MIPS environment.
    addr &= PAGE_MASK << 1;
    ginvt(addr, ginvt_type::GINVT_VA as u32);
}

#[inline]
pub unsafe fn ginvt_mmid() {
    ginvt(0, ginvt_type::GINVT_MMID as u32);
}

#[inline]
pub unsafe fn ginvt_va_mmid(mut addr: ::core::ffi::c_ulong) {
    // PAGE_MASK is supplied by the surrounding MIPS environment.
    addr &= PAGE_MASK << 1;
    ginvt(
        addr,
        ginvt_type::GINVT_VA as u32 | ginvt_type::GINVT_MMID as u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
