/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ELF definitions for the Hexagon architecture
 *
 * Copyright (c) 2010-2013, The Linux Foundation. All rights reserved.
 */

// C dependencies supplied by the surrounding kernel translation.

pub struct elf32_hdr;

/* ELF header e_flags defines. */

/* Hexagon relocations -- V2 */
pub const R_HEXAGON_NONE: u32 = 0;
pub const R_HEXAGON_B22_PCREL: u32 = 1;
pub const R_HEXAGON_B15_PCREL: u32 = 2;
pub const R_HEXAGON_B7_PCREL: u32 = 3;
pub const R_HEXAGON_LO16: u32 = 4;
pub const R_HEXAGON_HI16: u32 = 5;
pub const R_HEXAGON_32: u32 = 6;
pub const R_HEXAGON_16: u32 = 7;
pub const R_HEXAGON_8: u32 = 8;
pub const R_HEXAGON_GPREL16_0: u32 = 9;
pub const R_HEXAGON_GPREL16_1: u32 = 10;
pub const R_HEXAGON_GPREL16_2: u32 = 11;
pub const R_HEXAGON_GPREL16_3: u32 = 12;
pub const R_HEXAGON_HL16: u32 = 13;
/* V3 */
pub const R_HEXAGON_B13_PCREL: u32 = 14;
/* V4 */
pub const R_HEXAGON_B9_PCREL: u32 = 15;
/* V4 (extenders) */
pub const R_HEXAGON_B32_PCREL_X: u32 = 16;
pub const R_HEXAGON_32_6_X: u32 = 17;
/* V4 (extended) */
pub const R_HEXAGON_B22_PCREL_X: u32 = 18;
pub const R_HEXAGON_B15_PCREL_X: u32 = 19;
pub const R_HEXAGON_B13_PCREL_X: u32 = 20;
pub const R_HEXAGON_B9_PCREL_X: u32 = 21;
pub const R_HEXAGON_B7_PCREL_X: u32 = 22;
pub const R_HEXAGON_16_X: u32 = 23;
pub const R_HEXAGON_12_X: u32 = 24;
pub const R_HEXAGON_11_X: u32 = 25;
pub const R_HEXAGON_10_X: u32 = 26;
pub const R_HEXAGON_9_X: u32 = 27;
pub const R_HEXAGON_8_X: u32 = 28;
pub const R_HEXAGON_7_X: u32 = 29;
pub const R_HEXAGON_6_X: u32 = 30;
/* V2 PIC */
pub const R_HEXAGON_32_PCREL: u32 = 31;
pub const R_HEXAGON_COPY: u32 = 32;
pub const R_HEXAGON_GLOB_DAT: u32 = 33;
pub const R_HEXAGON_JMP_SLOT: u32 = 34;
pub const R_HEXAGON_RELATIVE: u32 = 35;
pub const R_HEXAGON_PLT_B22_PCREL: u32 = 36;
pub const R_HEXAGON_GOTOFF_LO16: u32 = 37;
pub const R_HEXAGON_GOTOFF_HI16: u32 = 38;
pub const R_HEXAGON_GOTOFF_32: u32 = 39;
pub const R_HEXAGON_GOT_LO16: u32 = 40;
pub const R_HEXAGON_GOT_HI16: u32 = 41;
pub const R_HEXAGON_GOT_32: u32 = 42;
pub const R_HEXAGON_GOT_16: u32 = 43;

/* ELF register definitions. */
pub type elf_greg_t = ::core::ffi::c_ulong;
pub type elf_gregset_t = user_regs_struct;
pub const ELF_NGREG: usize = core::mem::size_of::<elf_gregset_t>() / core::mem::size_of::<::core::ffi::c_ulong>();
pub type elf_fpregset_t = ::core::ffi::c_ulong;

/* Build-time architecture conditions are retained in these macro definitions. */
#[cfg(any())]
macro_rules! CS_COPYREGS { ($dest:expr, $regs:expr) => {{ $dest.cs0 = $regs.cs0; $dest.cs1 = $regs.cs1; }}; }
#[cfg(not(any()))]
macro_rules! CS_COPYREGS { ($dest:expr, $regs:expr) => {{ }}; }

macro_rules! ELF_CORE_COPY_REGS {
    ($dest:expr, $regs:expr) => {{
        $dest.r0 = $regs.r00; $dest.r1 = $regs.r01; $dest.r2 = $regs.r02; $dest.r3 = $regs.r03;
        $dest.r4 = $regs.r04; $dest.r5 = $regs.r05; $dest.r6 = $regs.r06; $dest.r7 = $regs.r07;
        $dest.r8 = $regs.r08; $dest.r9 = $regs.r09; $dest.r10 = $regs.r10; $dest.r11 = $regs.r11;
        $dest.r12 = $regs.r12; $dest.r13 = $regs.r13; $dest.r14 = $regs.r14; $dest.r15 = $regs.r15;
        $dest.r16 = $regs.r16; $dest.r17 = $regs.r17; $dest.r18 = $regs.r18; $dest.r19 = $regs.r19;
        $dest.r20 = $regs.r20; $dest.r21 = $regs.r21; $dest.r22 = $regs.r22; $dest.r23 = $regs.r23;
        $dest.r24 = $regs.r24; $dest.r25 = $regs.r25; $dest.r26 = $regs.r26; $dest.r27 = $regs.r27;
        $dest.r28 = $regs.r28; $dest.r29 = pt_psp($regs); $dest.r30 = $regs.r30; $dest.r31 = $regs.r31;
        $dest.sa0 = $regs.sa0; $dest.lc0 = $regs.lc0; $dest.sa1 = $regs.sa1; $dest.lc1 = $regs.lc1;
        $dest.m0 = $regs.m0; $dest.m1 = $regs.m1; $dest.usr = $regs.usr; $dest.p3_0 = $regs.preds;
        $dest.gp = $regs.gp; $dest.ugp = $regs.ugp; CS_COPYREGS!($dest, $regs);
        $dest.pc = pt_elr($regs); $dest.cause = pt_cause($regs); $dest.badva = pt_badva($regs);
    }};
}

macro_rules! elf_check_arch { ($hdr:expr) => { $hdr.e_machine == EM_HEXAGON }; }

pub const ELF_CLASS: u32 = ELFCLASS32;
pub const ELF_DATA: u32 = ELFDATA2LSB;
pub const ELF_ARCH: u32 = EM_HEXAGON;

// CONFIG_HEXAGON_ARCH_VERSION selects ELF_CORE_EFLAGS: 2 => 0x1, 3 => 0x2, 4 => 0x3.
#[cfg(feature = "hexagon_arch_version_2")]
pub const ELF_CORE_EFLAGS: u32 = 0x1;
#[cfg(feature = "hexagon_arch_version_3")]
pub const ELF_CORE_EFLAGS: u32 = 0x2;
#[cfg(feature = "hexagon_arch_version_4")]
pub const ELF_CORE_EFLAGS: u32 = 0x3;

macro_rules! ELF_PLAT_INIT { ($regs:expr, $load_addr:expr) => {{ }}; }
pub const CORE_DUMP_USE_REGSET: bool = true;
pub const ELF_EXEC_PAGESIZE: usize = PAGE_SIZE;
pub const ELF_ET_DYN_BASE: ::core::ffi::c_ulong = 0x08000000;
pub const ELF_HWCAP: u32 = 0;
pub const ELF_PLATFORM: *const core::ffi::c_char = core::ptr::null();
pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: i32 = 1;

pub struct linux_binprm;
unsafe extern "C" {
    pub fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
