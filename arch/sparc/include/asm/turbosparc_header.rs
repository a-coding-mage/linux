/* SPDX-License-Identifier: GPL-2.0 */
/*
 * turbosparc.h:  Defines specific to the TurboSparc module.
 *            This is SRMMU stuff.
 *
 * Copyright (C) 1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 */

/* Dependency intent: symbols from asm/asi.h and asm/pgtsrmmu.h are supplied
 * by the surrounding translation unit. */

/* Bits in the SRMMU control register for TurboSparc modules. */
pub const TURBOSPARC_MMUENABLE: u32 = 0x00000001;
pub const TURBOSPARC_NOFAULT: u32 = 0x00000002;
pub const TURBOSPARC_ICSNOOP: u32 = 0x00000004;
pub const TURBOSPARC_PSO: u32 = 0x00000080;
pub const TURBOSPARC_DCENABLE: u32 = 0x00000100; /* Enable data cache */
pub const TURBOSPARC_ICENABLE: u32 = 0x00000200; /* Enable instruction cache */
pub const TURBOSPARC_BMODE: u32 = 0x00004000;
pub const TURBOSPARC_PARITYODD: u32 = 0x00020000; /* Parity odd, if enabled */
pub const TURBOSPARC_PCENABLE: u32 = 0x00040000; /* Enable parity checking */

/* Bits in the CPU configuration register for TurboSparc modules. */
pub const TURBOSPARC_SCENABLE: u32 = 0x00000008; /* Secondary cache enable */
pub const TURBOSPARC_uS2: u32 = 0x00000010; /* Swift compatibility mode */
pub const TURBOSPARC_WTENABLE: u32 = 0x00000020; /* Write thru for dcache */
pub const TURBOSPARC_SNENABLE: u32 = 0x40000000; /* DVMA snoop enable */

/* External dependency supplied by asm/asi.h. */
extern "Rust" {
    static ASI_M_TXTC_TAG: usize;
    static ASI_M_DATAC_TAG: usize;
    static ASI_M_MMUREGS: usize;
}

/* Bits [13:5] select one of 512 instruction cache tags */
#[inline]
pub unsafe fn turbosparc_inv_insn_tag(addr: usize) {
    core::arch::asm!(
        "sta %g0, [{addr}] {asi}\n\t",
        addr = in(reg) addr,
        asi = const ASI_M_TXTC_TAG,
        options(nostack)
    );
}

/* Bits [13:5] select one of 512 data cache tags */
#[inline]
pub unsafe fn turbosparc_inv_data_tag(addr: usize) {
    core::arch::asm!(
        "sta %g0, [{addr}] {asi}\n\t",
        addr = in(reg) addr,
        asi = const ASI_M_DATAC_TAG,
        options(nostack)
    );
}

#[inline]
pub unsafe fn turbosparc_flush_icache() {
    let mut addr: usize = 0;
    while addr < 0x4000 {
        turbosparc_inv_insn_tag(addr);
        addr += 0x20;
    }
}

#[inline]
pub unsafe fn turbosparc_flush_dcache() {
    let mut addr: usize = 0;
    while addr < 0x4000 {
        turbosparc_inv_data_tag(addr);
        addr += 0x20;
    }
}

#[inline]
pub unsafe fn turbosparc_idflash_clear() {
    let mut addr: usize = 0;
    while addr < 0x4000 {
        turbosparc_inv_insn_tag(addr);
        turbosparc_inv_data_tag(addr);
        addr += 0x20;
    }
}

#[inline]
pub unsafe fn turbosparc_set_ccreg(regval: usize) {
    core::arch::asm!(
        "sta {regval}, [{addr}] {asi}\n\t",
        regval = in(reg) regval,
        addr = in(reg) 0x600usize,
        asi = const ASI_M_MMUREGS,
        options(nostack)
    );
}

#[inline]
pub unsafe fn turbosparc_get_ccreg() -> usize {
    let regval: usize;
    core::arch::asm!(
        "lda [{addr}] {asi}, {regval}\n\t",
        addr = in(reg) 0x600usize,
        asi = const ASI_M_MMUREGS,
        regval = out(reg) regval,
        options(nostack)
    );
    regval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
