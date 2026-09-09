/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <asm-generic/hardirq.h> are supplied
// by the corresponding translated generic header.

unsafe extern "C" {
    pub fn ack_bad_irq(irq: u32);
}

// C macro: #define ack_bad_irq ack_bad_irq
// The Rust function declaration above provides the same symbol directly.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
