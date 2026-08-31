/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/include/dwarf-regs.h. */
/* C dependencies: "annotate.h" and <elf.h>. */

use core::ffi::{c_char, c_int, c_uint};

use crate::annotate::annotated_op_loc;

pub const EM_AARCH64: c_uint = 183; /* ARM 64 bit */
pub const EM_CSKY: c_uint = 252; /* C-SKY */
pub const EF_CSKY_ABIV1: c_uint = 0x10000000;
pub const EF_CSKY_ABIV2: c_uint = 0x20000000;
pub const EM_LOONGARCH: c_uint = 258; /* LoongArch */

/*
 * EM_HOST gives the ELF machine for host, EF_HOST gives additional flags.
 *
 * The C header maps EM_HOST with preprocessor architecture checks:
 * __x86_64__      -> EM_X86_64
 * __i386__        -> EM_386
 * __aarch64__     -> EM_AARCH64
 * __arm__         -> EM_ARM
 * __alpha__       -> EM_ALPHA
 * __arc__         -> EM_ARC
 * __AVR__         -> EM_AVR
 * __AVR32__       -> EM_AVR32
 * __bfin__        -> EM_BLACKFIN
 * __csky__        -> EM_CSKY, with EF_HOST selected by __CSKYABIV2__
 * __cris__        -> EM_CRIS
 * __hppa__        -> EM_PARISC
 * __loongarch__   -> EM_LOONGARCH
 * __mips__        -> EM_MIPS
 * __m32r__        -> EM_M32R
 * __microblaze__  -> EM_MICROBLAZE
 * __MSP430__      -> EM_MSP430
 * __powerpc64__   -> EM_PPC64
 * __powerpc__     -> EM_PPC
 * __riscv         -> EM_RISCV
 * __s390x__       -> EM_S390
 * __sh__          -> EM_SH
 * __sparc64__ or __sparc__ -> EM_SPARC
 * __xtensa__      -> EM_XTENSA
 * otherwise       -> EM_NONE
 *
 * Those EM_* constants are supplied by <elf.h> in C and by a future Rust
 * dependency in this translation unit's final integration context.
 */

pub const EF_HOST: c_uint = 0;

pub const DWARF_REG_PC: c_uint = 0xd3af9c; /* random number */
pub const DWARF_REG_FB: c_uint = 0xd3affb; /* random number */

#[cfg(feature = "have_libdw_support")]
unsafe extern "C" {
    /**
     * get_dwarf_regstr() - Returns ftrace register string from DWARF regnum.
     * @n: DWARF register number.
     * @machine: ELF machine signature (EM_*).
     * @flags: ELF flags for things like ABI differences.
     */
    pub fn get_dwarf_regstr(n: c_uint, machine: c_uint, flags: c_uint) -> *const c_char;

    pub fn __get_csky_regstr(n: c_uint, flags: c_uint) -> *const c_char;
    pub fn __get_csky_regnum(name: *const c_char, flags: c_uint) -> c_int;

    pub fn __get_dwarf_regnum_i386(name: *const c_char) -> c_int;
    pub fn __get_dwarf_regnum_x86_64(name: *const c_char) -> c_int;
    pub fn __get_dwarf_regnum_for_perf_regnum_i386(perf_regnum: c_int) -> c_int;
    pub fn __get_dwarf_regnum_for_perf_regnum_x86_64(perf_regnum: c_int) -> c_int;

    pub fn __get_dwarf_regnum_for_perf_regnum_arm(perf_regnum: c_int) -> c_int;
    pub fn __get_dwarf_regnum_for_perf_regnum_arm64(perf_regnum: c_int) -> c_int;

    pub fn __get_dwarf_regnum_for_perf_regnum_csky(
        perf_regnum: c_int,
        flags: c_uint,
    ) -> c_int;
    pub fn __get_dwarf_regnum_for_perf_regnum_loongarch(perf_regnum: c_int) -> c_int;
    pub fn __get_dwarf_regnum_for_perf_regnum_powerpc(perf_regnum: c_int) -> c_int;
    pub fn __get_dwarf_regnum_for_perf_regnum_riscv(perf_regnum: c_int) -> c_int;
    pub fn __get_dwarf_regnum_for_perf_regnum_s390(perf_regnum: c_int) -> c_int;
    pub fn __get_dwarf_regnum_for_perf_regnum_mips(perf_regnum: c_int) -> c_int;

    /*
     * get_dwarf_regnum - Returns DWARF regnum from register name
     * name: architecture register name
     * machine: ELF machine signature (EM_*)
     */
    pub fn get_dwarf_regnum(name: *const c_char, machine: c_uint, flags: c_uint) -> c_int;

    /*
     * get_dwarf_regnum - Returns DWARF regnum from perf register number.
     */
    pub fn get_dwarf_regnum_for_perf_regnum(
        perf_regnum: c_int,
        machine: c_uint,
        flags: c_uint,
        only_libdw_supported: bool,
    ) -> c_int;

    pub fn get_powerpc_regs(
        raw_insn: u32,
        is_source: c_int,
        op_loc: *mut annotated_op_loc,
    );
}

#[cfg(not(feature = "have_libdw_support"))]
#[inline]
pub unsafe fn get_dwarf_regnum(
    _name: *const c_char,
    _machine: c_uint,
    _flags: c_uint,
) -> c_int {
    -1
}

#[cfg(not(feature = "have_libdw_support"))]
#[inline]
pub unsafe fn get_powerpc_regs(
    _raw_insn: u32,
    _is_source: c_int,
    _op_loc: *mut annotated_op_loc,
) {
    return;
}
