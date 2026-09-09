/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mxcc.h: Definitions of the Viking MXCC registers
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

/* These registers are accessed through ASI 0x2. */
pub const MXCC_DATSTREAM: usize = 0x1C00000; /* Data stream register */
pub const MXCC_SRCSTREAM: usize = 0x1C00100; /* Source stream register */
pub const MXCC_DESSTREAM: usize = 0x1C00200; /* Destination stream register */
pub const MXCC_RMCOUNT: usize = 0x1C00300; /* Count of references and misses */
pub const MXCC_STEST: usize = 0x1C00804; /* Internal self-test */
pub const MXCC_CREG: usize = 0x1C00A04; /* Control register */
pub const MXCC_SREG: usize = 0x1C00B00; /* Status register */
pub const MXCC_RREG: usize = 0x1C00C04; /* Reset register */
pub const MXCC_EREG: usize = 0x1C00E00; /* Error code register */
pub const MXCC_PREG: usize = 0x1C00F04; /* Address port register */

/* Some MXCC constants. */
pub const MXCC_STREAM_SIZE: usize = 0x20; /* Size in bytes of one stream r/w */

/* The MXCC Control Register. */
pub const MXCC_CTL_RRC: usize = 0x00000200;
pub const MXCC_CTL_PRE: usize = 0x00000020;
pub const MXCC_CTL_MCE: usize = 0x00000010;
pub const MXCC_CTL_PARE: usize = 0x00000008;
pub const MXCC_CTL_ECE: usize = 0x00000004;

/* The MXCC Error Register. */
pub const MXCC_ERR_ME: usize = 0x80000000;
pub const MXCC_ERR_CE: usize = 0x20000000;
pub const MXCC_ERR_PEW: usize = 0x10000000;
pub const MXCC_ERR_PEE: usize = 0x08000000;
pub const MXCC_ERR_ASE: usize = 0x04000000;
pub const MXCC_ERR_EIV: usize = 0x02000000;
pub const MXCC_ERR_MOPC: usize = 0x01FF8000;
pub const MXCC_ERR_ECODE: usize = 0x00007F80;
pub const MXCC_ERR_PRIV: usize = 0x00000040;
pub const MXCC_ERR_HPADDR: usize = 0x0000000f;

/* The MXCC Port register contains the module ID in bits 20-18. */

/* ASI_M_MXCC is supplied by the target architecture dependencies. */
pub unsafe fn mxcc_set_stream_src(paddr: *mut usize) {
    let data0 = *paddr;
    let data1 = *paddr.add(1);
    core::arch::asm!(
        "or %g0, {data0}, %g2",
        "or %g0, {data1}, %g3",
        "stda %g2, [{addr}] {asi}",
        data0 = in(reg) data0,
        data1 = in(reg) data1,
        addr = in(reg) MXCC_SRCSTREAM,
        asi = const ASI_M_MXCC,
        options(nostack)
    );
}

pub unsafe fn mxcc_set_stream_dst(paddr: *mut usize) {
    let data0 = *paddr;
    let data1 = *paddr.add(1);
    core::arch::asm!(
        "or %g0, {data0}, %g2",
        "or %g0, {data1}, %g3",
        "stda %g2, [{addr}] {asi}",
        data0 = in(reg) data0,
        data1 = in(reg) data1,
        addr = in(reg) MXCC_DESSTREAM,
        asi = const ASI_M_MXCC,
        options(nostack)
    );
}

pub unsafe fn mxcc_get_creg() -> usize {
    let mut mxcc_control: usize;
    core::arch::asm!(
        "set 0xffffffff, %g2",
        "set 0xffffffff, %g3",
        "stda %g2, [{ereg}] {asi}",
        "lda [{creg}] {asi}, {control}",
        ereg = in(reg) MXCC_EREG,
        asi = const ASI_M_MXCC,
        creg = in(reg) MXCC_CREG,
        control = lateout(reg) mxcc_control,
        options(nostack)
    );
    mxcc_control
}

pub unsafe fn mxcc_set_creg(mxcc_control: usize) {
    core::arch::asm!(
        "sta {control}, [{creg}] {asi}",
        control = in(reg) mxcc_control,
        creg = in(reg) MXCC_CREG,
        asi = const ASI_M_MXCC,
        options(nostack)
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
