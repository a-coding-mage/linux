/* SPDX-License-Identifier: GPL-2.0 */
/*
 * obio.h:  Some useful locations in 0xFXXXXXXXX PA obio space on sun4d.
 *
 * Copyright (C) 1997 Jakub Jelinek <jj@sunsite.mff.cuni.cz>
 */

/* Dependency supplied by the surrounding SPARC environment: asm/asi.h. */

/* This weird monster likes to use the very upper parts of
   36bit PA for these things :) */

/* CSR space (for each XDBUS)
 *  ------------------------------------------------------------------------
 *  |   0xFE  |   DEVID    |                | XDBUS ID |                   |
 *  ------------------------------------------------------------------------
 *  35      28 27        20 19            10 9        8 7                 0
 */

pub const CSR_BASE_ADDR: u32 = 0xe0000000;
pub const CSR_CPU_SHIFT: u32 = 32 - 4 - 5;
pub const CSR_XDBUS_SHIFT: u32 = 8;

#[inline]
pub const fn csr_base(cpu: u32) -> u32 {
    ((CSR_BASE_ADDR >> CSR_CPU_SHIFT) + cpu) << CSR_CPU_SHIFT
}

/* ECSR space (not for each XDBUS)
 *  ------------------------------------------------------------------------
 *  |   0xF  | DEVID[7:1] |                			           |
 *  ------------------------------------------------------------------------
 *  35     32 31        25 24                 				  0
 */

pub const ECSR_BASE_ADDR: u32 = 0x00000000;
pub const ECSR_CPU_SHIFT: u32 = 32 - 5;
pub const ECSR_DEV_SHIFT: u32 = 32 - 8;

#[inline]
pub const fn ecsr_base(cpu: u32) -> u32 { cpu << ECSR_CPU_SHIFT }

#[inline]
pub const fn ecsr_dev_base(devid: u32) -> u32 { devid << ECSR_DEV_SHIFT }

/* Bus Watcher */
pub const BW_LOCAL_BASE: u32 = 0xfff00000;

pub const BW_CID: u32 = 0x00000000;
pub const BW_DBUS_CTRL: u32 = 0x00000008;
pub const BW_DBUS_DATA: u32 = 0x00000010;
pub const BW_CTRL: u32 = 0x00001000;
pub const BW_INTR_TABLE: u32 = 0x00001040;
pub const BW_INTR_TABLE_CLEAR: u32 = 0x00001080;
pub const BW_PRESCALER: u32 = 0x000010c0;
pub const BW_PTIMER_LIMIT: u32 = 0x00002000;
pub const BW_PTIMER_COUNTER2: u32 = 0x00002004;
pub const BW_PTIMER_NDLIMIT: u32 = 0x00002008;
pub const BW_PTIMER_CTRL: u32 = 0x0000200c;
pub const BW_PTIMER_COUNTER: u32 = 0x00002010;
pub const BW_TIMER_LIMIT: u32 = 0x00003000;
pub const BW_TIMER_COUNTER2: u32 = 0x00003004;
pub const BW_TIMER_NDLIMIT: u32 = 0x00003008;
pub const BW_TIMER_CTRL: u32 = 0x0000300c;
pub const BW_TIMER_COUNTER: u32 = 0x00003010;

/* BW Control */
pub const BW_CTRL_USER_TIMER: u32 = 0x00000004; /* Is User Timer Free run enabled */

/* Boot Bus */
pub const BB_LOCAL_BASE: u32 = 0xf0000000;

pub const BB_STAT1: u32 = 0x00100000;
pub const BB_STAT2: u32 = 0x00120000;
pub const BB_STAT3: u32 = 0x00140000;
pub const BB_LEDS: u32 = 0x002e0000;

/* Bits in BB_STAT2 */
pub const BB_STAT2_AC_INTR: u32 = 0x04; /* Aiee! 5ms and power is gone... */
pub const BB_STAT2_TMP_INTR: u32 = 0x10; /* My Penguins are burning. Are you able to smell it? */
pub const BB_STAT2_FAN_INTR: u32 = 0x20; /* My fan refuses to work */
pub const BB_STAT2_PWR_INTR: u32 = 0x40; /* On SC2000, one of the two ACs died. Ok, we go on... */
pub const BB_STAT2_MASK: u32 = BB_STAT2_AC_INTR | BB_STAT2_TMP_INTR | BB_STAT2_FAN_INTR | BB_STAT2_PWR_INTR;

/* Cache Controller */
pub const CC_BASE: u32 = 0x1F00000;
pub const CC_DATSTREAM: u32 = 0x1F00000; /* Data stream register */
pub const CC_DATSIZE: u32 = 0x1F0003F; /* Size */
pub const CC_SRCSTREAM: u32 = 0x1F00100; /* Source stream register */
pub const CC_DESSTREAM: u32 = 0x1F00200; /* Destination stream register */
pub const CC_RMCOUNT: u32 = 0x1F00300; /* Count of references and misses */
pub const CC_IPEN: u32 = 0x1F00406; /* Pending Interrupts */
pub const CC_IMSK: u32 = 0x1F00506; /* Interrupt Mask */
pub const CC_ICLR: u32 = 0x1F00606; /* Clear pending Interrupts */
pub const CC_IGEN: u32 = 0x1F00704; /* Generate Interrupt register */
pub const CC_STEST: u32 = 0x1F00804; /* Internal self-test */
pub const CC_CREG: u32 = 0x1F00A04; /* Control register */
pub const CC_SREG: u32 = 0x1F00B00; /* Status register */
pub const CC_RREG: u32 = 0x1F00C04; /* Reset register */
pub const CC_EREG: u32 = 0x1F00E00; /* Error code register */
pub const CC_CID: u32 = 0x1F00F04; /* Component ID */

/* The following helpers retain the original SPARC ASI operations.  The ASI
 * symbols are supplied by asm/asi.h in the containing translation unit. */

#[inline]
pub unsafe fn bw_get_intr_mask(sbus_level: i32) -> i32 {
    let address = BW_LOCAL_BASE.wrapping_add(BW_INTR_TABLE).wrapping_add((sbus_level << 3) as u32);
    let mut value: i32;
    core::arch::asm!("lduha [{address}] {asi}, {value}", address = in(reg) address, asi = const ASI_M_CTL, value = out(reg) value);
    value
}

#[inline]
pub unsafe fn bw_clear_intr_mask(sbus_level: i32, mask: i32) {
    let address = BW_LOCAL_BASE.wrapping_add(BW_INTR_TABLE_CLEAR).wrapping_add((sbus_level << 3) as u32);
    core::arch::asm!("stha {mask}, [{address}] {asi}", mask = in(reg) mask, address = in(reg) address, asi = const ASI_M_CTL);
}

#[inline]
pub unsafe fn bw_get_prof_limit(cpu: i32) -> u32 {
    let address = csr_base(cpu as u32).wrapping_add(BW_PTIMER_LIMIT);
    let mut value: u32;
    core::arch::asm!("lda [{address}] {asi}, {value}", address = in(reg) address, asi = const ASI_M_CTL, value = out(reg) value);
    value
}

#[inline]
pub unsafe fn bw_set_prof_limit(cpu: i32, limit: u32) {
    let address = csr_base(cpu as u32).wrapping_add(BW_PTIMER_LIMIT);
    core::arch::asm!("sta {limit}, [{address}] {asi}", limit = in(reg) limit, address = in(reg) address, asi = const ASI_M_CTL);
}

#[inline]
pub unsafe fn bw_get_ctrl(cpu: i32) -> u32 {
    let address = csr_base(cpu as u32).wrapping_add(BW_CTRL);
    let mut value: u32;
    core::arch::asm!("lda [{address}] {asi}, {value}", address = in(reg) address, asi = const ASI_M_CTL, value = out(reg) value);
    value
}

#[inline]
pub unsafe fn bw_set_ctrl(cpu: i32, ctrl: u32) {
    let address = csr_base(cpu as u32).wrapping_add(BW_CTRL);
    core::arch::asm!("sta {ctrl}, [{address}] {asi}", ctrl = in(reg) ctrl, address = in(reg) address, asi = const ASI_M_CTL);
}

#[inline]
pub unsafe fn cc_get_ipen() -> u32 {
    let mut value: u32;
    core::arch::asm!("lduha [{address}] {asi}, {value}", address = in(reg) CC_IPEN, asi = const ASI_M_MXCC, value = out(reg) value);
    value
}

#[inline]
pub unsafe fn cc_set_iclr(clear: u32) {
    core::arch::asm!("stha {clear}, [{address}] {asi}", clear = in(reg) clear, address = in(reg) CC_ICLR, asi = const ASI_M_MXCC);
}

#[inline]
pub unsafe fn cc_get_imsk() -> u32 {
    let mut value: u32;
    core::arch::asm!("lduha [{address}] {asi}, {value}", address = in(reg) CC_IMSK, asi = const ASI_M_MXCC, value = out(reg) value);
    value
}

#[inline]
pub unsafe fn cc_set_imsk(mask: u32) {
    core::arch::asm!("stha {mask}, [{address}] {asi}", mask = in(reg) mask, address = in(reg) CC_IMSK, asi = const ASI_M_MXCC);
}

#[inline]
pub unsafe fn cc_get_imsk_other(cpuid: i32) -> u32 {
    let address = ecsr_base(cpuid as u32) | CC_IMSK;
    let mut value: u32;
    core::arch::asm!("lduha [{address}] {asi}, {value}", address = in(reg) address, asi = const ASI_M_CTL, value = out(reg) value);
    value
}

#[inline]
pub unsafe fn cc_set_imsk_other(cpuid: i32, mask: u32) {
    let address = ecsr_base(cpuid as u32) | CC_IMSK;
    core::arch::asm!("stha {mask}, [{address}] {asi}", mask = in(reg) mask, address = in(reg) address, asi = const ASI_M_CTL);
}

#[inline]
pub unsafe fn cc_set_igen(gen: u32) {
    core::arch::asm!("sta {gen}, [{address}] {asi}", gen = in(reg) gen, address = in(reg) CC_IGEN, asi = const ASI_M_MXCC);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
