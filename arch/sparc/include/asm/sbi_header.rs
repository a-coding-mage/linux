/* SPDX-License-Identifier: GPL-2.0 */
/*
 * sbi.h: SBI (Sbus Interface on sun4d) definitions
 *
 * Copyright (C) 1997 Jakub Jelinek <jj@sunsite.mff.cuni.cz>
 */

// Dependency supplied by the surrounding translation unit: asm/obio.h.

/* SBI */
#[repr(C)]
pub struct SbiRegs {
    pub cid: u32,        /* Component ID */
    pub ctl: u32,        /* Control */
    pub status: u32,     /* Status */
    pub _unused1: u32,

    pub cfg0: u32,       /* Slot0 config reg */
    pub cfg1: u32,       /* Slot1 config reg */
    pub cfg2: u32,       /* Slot2 config reg */
    pub cfg3: u32,       /* Slot3 config reg */

    pub stb0: u32,       /* Streaming buf control for slot 0 */
    pub stb1: u32,       /* Streaming buf control for slot 1 */
    pub stb2: u32,       /* Streaming buf control for slot 2 */
    pub stb3: u32,       /* Streaming buf control for slot 3 */

    pub intr_state: u32, /* Interrupt state */
    pub intr_tid: u32,   /* Interrupt target ID */
    pub intr_diag: u32,  /* Interrupt diagnostics */
}

pub const SBI_CID: u32 = 0x02800000;
pub const SBI_CTL: u32 = 0x02800004;
pub const SBI_STATUS: u32 = 0x02800008;
pub const SBI_CFG0: u32 = 0x02800010;
pub const SBI_CFG1: u32 = 0x02800014;
pub const SBI_CFG2: u32 = 0x02800018;
pub const SBI_CFG3: u32 = 0x0280001c;
pub const SBI_STB0: u32 = 0x02800020;
pub const SBI_STB1: u32 = 0x02800024;
pub const SBI_STB2: u32 = 0x02800028;
pub const SBI_STB3: u32 = 0x0280002c;
pub const SBI_INTR_STATE: u32 = 0x02800030;
pub const SBI_INTR_TID: u32 = 0x02800034;
pub const SBI_INTR_DIAG: u32 = 0x02800038;

/* Burst bits for 8, 16, 32, 64 are in cfgX registers at bits 2, 3, 4, 5 respectively */
pub const SBI_CFG_BURST_MASK: u32 = 0x0000001e;

/* How to make devid from sbi no */
#[inline]
pub const fn sbi2devid(sbino: i32) -> i32 {
    (sbino << 4) | 2
}

/* intr_state has 4 bits for slots 0 .. 3 and these bits are repeated for each sbus irq level */

/*
 *		   +-------+-------+-------+-------+-------+-------+-------+-------+
 *  SBUS IRQ LEVEL |   7   |   6   |   5   |   4   |   3   |   2   |   1   |       |
 *		   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+ Reser |
 *  SLOT #         |3|2|1|0|3|2|1|0|3|2|1|0|3|2|1|0|3|2|1|0|3|2|1|0|3|2|1|0|  ved  |
 *                 +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-------+
 *  Bits           31      27      23      19      15      11      7       3      0
 */

// The following functions preserve the original SPARC inline assembly operations.
// ECSR_DEV_BASE and ASI_M_CTL are supplied by asm/obio.h.
#[inline]
pub unsafe fn acquire_sbi(devid: i32, mut mask: i32) -> i32 {
    core::arch::asm!(
        "swapa [{addr}] {asi}, {mask}",
        addr = in(reg) (ECSR_DEV_BASE(devid) | SBI_INTR_STATE as i32),
        asi = const ASI_M_CTL,
        mask = inout(reg) mask,
    );
    mask
}

#[inline]
pub unsafe fn release_sbi(devid: i32, mask: i32) {
    core::arch::asm!(
        "sta {mask}, [{addr}] {asi}",
        mask = in(reg) mask,
        addr = in(reg) (ECSR_DEV_BASE(devid) | SBI_INTR_STATE as i32),
        asi = const ASI_M_CTL,
    );
}

#[inline]
pub unsafe fn set_sbi_tid(devid: i32, targetid: i32) {
    core::arch::asm!(
        "sta {targetid}, [{addr}] {asi}",
        targetid = in(reg) targetid,
        addr = in(reg) (ECSR_DEV_BASE(devid) | SBI_INTR_TID as i32),
        asi = const ASI_M_CTL,
    );
}

#[inline]
pub unsafe fn get_sbi_ctl(devid: i32, cfgno: i32) -> i32 {
    let cfg: i32;
    core::arch::asm!(
        "lda [{addr}] {asi}, {cfg}",
        addr = in(reg) ((ECSR_DEV_BASE(devid) | SBI_CFG0 as i32) + (cfgno << 2)),
        asi = const ASI_M_CTL,
        cfg = lateout(reg) cfg,
    );
    cfg
}

#[inline]
pub unsafe fn set_sbi_ctl(devid: i32, cfgno: i32, cfg: i32) {
    core::arch::asm!(
        "sta {cfg}, [{addr}] {asi}",
        cfg = in(reg) cfg,
        addr = in(reg) ((ECSR_DEV_BASE(devid) | SBI_CFG0 as i32) + (cfgno << 2)),
        asi = const ASI_M_CTL,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
