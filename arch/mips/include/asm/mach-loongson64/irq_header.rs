/* SPDX-License-Identifier: GPL-2.0 */

/* cpu core interrupt numbers */
pub const NR_IRQS_LEGACY: i32 = 16;
pub const NR_MIPS_CPU_IRQS: i32 = 8;
pub const NR_MAX_CHAINED_IRQS: i32 = 40; /* Chained IRQs means those not directly used by devices */
pub const NR_IRQS: i32 = NR_IRQS_LEGACY + NR_MIPS_CPU_IRQS + NR_MAX_CHAINED_IRQS + 256;
pub const MAX_IO_PICS: i32 = 1;
pub const MIPS_CPU_IRQ_BASE: i32 = NR_IRQS_LEGACY;
pub const GSI_MIN_CPU_IRQ: i32 = 0;

/* Dependency supplied by asm/mach-generic/irq.h. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
