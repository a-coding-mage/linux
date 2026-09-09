/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <asm/mach-generic/irq.h> and
// <asm/mach-rc32434/rb.h> is preserved here; their symbols are supplied by
// the surrounding translation unit.

pub const NR_IRQS: usize = 256;

/* Interrupt Controller */
pub const IC_GROUP0_PEND: usize = REGBASE + 0x38000;
pub const IC_GROUP0_MASK: usize = REGBASE + 0x38008;
pub const IC_GROUP_OFFSET: usize = 0x0C;

pub const NUM_INTR_GROUPS: usize = 5;

/* 16550 UARTs */
pub const GROUP0_IRQ_BASE: usize = 8; // GRP2 IRQ numbers start here
// GRP3 IRQ numbers start here
pub const GROUP1_IRQ_BASE: usize = GROUP0_IRQ_BASE + 32;
// GRP4 IRQ numbers start here
pub const GROUP2_IRQ_BASE: usize = GROUP1_IRQ_BASE + 32;
// GRP5 IRQ numbers start here
pub const GROUP3_IRQ_BASE: usize = GROUP2_IRQ_BASE + 32;
pub const GROUP4_IRQ_BASE: usize = GROUP3_IRQ_BASE + 32;

pub const UART0_IRQ: usize = GROUP3_IRQ_BASE + 0;

pub const ETH0_DMA_RX_IRQ: usize = GROUP1_IRQ_BASE + 0;
pub const ETH0_DMA_TX_IRQ: usize = GROUP1_IRQ_BASE + 1;
pub const ETH0_RX_OVR_IRQ: usize = GROUP3_IRQ_BASE + 9;
pub const ETH0_TX_UND_IRQ: usize = GROUP3_IRQ_BASE + 10;

pub const GPIO_MAPPED_IRQ_BASE: usize = GROUP4_IRQ_BASE;
pub const GPIO_MAPPED_IRQ_GROUP: usize = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
