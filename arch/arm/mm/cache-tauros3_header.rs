/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Marvell Tauros3 cache controller includes
 *
 * Sebastian Hesselbarth <sebastian.hesselbarth@gmail.com>
 *
 * based on GPL'ed 2.6 kernel sources
 *  (c) Marvell International Ltd.
 */

/*
 * Marvell Tauros3 L2CC is compatible with PL310 r0p0
 * but with PREFETCH_CTRL (r2p0) and an additional event counter.
 * Also, there is AUX2_CTRL for some Marvell specific control.
 */

pub const TAUROS3_EVENT_CNT2_CFG: u32 = 0x224;
pub const TAUROS3_EVENT_CNT2_VAL: u32 = 0x228;
pub const TAUROS3_INV_ALL: u32 = 0x780;
pub const TAUROS3_CLEAN_ALL: u32 = 0x784;
pub const TAUROS3_AUX2_CTRL: u32 = 0x820;

/* Registers shifts and masks */
pub const TAUROS3_AUX2_CTRL_LINEFILL_BURST8_EN: u32 = 1 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
