/* SPDX-License-Identifier: GPL-2.0 */
/*
 * contains some Q40 related interrupt definitions
 */

pub const Q40_IRQ_MAX: u32 = 34;

pub const Q40_IRQ_SAMPLE: u32 = 34;
pub const Q40_IRQ_KEYBOARD: u32 = 32;
pub const Q40_IRQ_FRAME: u32 = 33;

/* masks for interrupt regiosters */
/* internal, IIRQ_REG */
pub const Q40_IRQ_KEYB_MASK: u32 = 2;
pub const Q40_IRQ_SER_MASK: u32 = 1 << 2;
pub const Q40_IRQ_FRAME_MASK: u32 = 1 << 3;
pub const Q40_IRQ_EXT_MASK: u32 = 1 << 4; /* is a EIRQ */
/* eirq, EIRQ_REG */
pub const Q40_IRQ3_MASK: u32 = 1;
pub const Q40_IRQ4_MASK: u32 = 1 << 1;
pub const Q40_IRQ5_MASK: u32 = 1 << 2;
pub const Q40_IRQ6_MASK: u32 = 1 << 3;
pub const Q40_IRQ7_MASK: u32 = 1 << 4;
pub const Q40_IRQ10_MASK: u32 = 1 << 5;
pub const Q40_IRQ14_MASK: u32 = 1 << 6;
pub const Q40_IRQ15_MASK: u32 = 1 << 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
