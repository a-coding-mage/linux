/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the SH interrupt-controller definitions:
// `evt2irq`.

/*
 * SH7750/SH7751/SH7760
 */
pub const DMTE0_IRQ: u32 = evt2irq(0x640);
pub const DMTE4_IRQ: u32 = evt2irq(0x780);
pub const DMTE6_IRQ: u32 = evt2irq(0x7c0);
pub const DMAE0_IRQ: u32 = evt2irq(0x6c0);

pub const SH_DMAC_BASE0: u32 = 0xffa00000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
