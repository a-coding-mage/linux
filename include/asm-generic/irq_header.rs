/* SPDX-License-Identifier: GPL-2.0 */

/*
 * NR_IRQS is the upper bound of how many interrupts can be handled
 * in the platform. It is used to size the static irq_map array,
 * so don't make it too big.
 */
pub const NR_IRQS: i32 = 64;

pub fn irq_canonicalize(irq: i32) -> i32 {
    irq
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
