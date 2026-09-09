/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * IRQ definitions for Orion SoC
 *
 *  Maintainer: Tzachi Perelstein <tzachi@marvell.com>
 */

/*
 * Orion Main Interrupt Controller
 */
pub const IRQ_ORION5X_BRIDGE: i32 = 1 + 0;
pub const IRQ_ORION5X_DOORBELL_H2C: i32 = 1 + 1;
pub const IRQ_ORION5X_DOORBELL_C2H: i32 = 1 + 2;
pub const IRQ_ORION5X_UART0: i32 = 1 + 3;
pub const IRQ_ORION5X_UART1: i32 = 1 + 4;
pub const IRQ_ORION5X_I2C: i32 = 1 + 5;
pub const IRQ_ORION5X_GPIO_0_7: i32 = 1 + 6;
pub const IRQ_ORION5X_GPIO_8_15: i32 = 1 + 7;
pub const IRQ_ORION5X_GPIO_16_23: i32 = 1 + 8;
pub const IRQ_ORION5X_GPIO_24_31: i32 = 1 + 9;
pub const IRQ_ORION5X_PCIE0_ERR: i32 = 1 + 10;
pub const IRQ_ORION5X_PCIE0_INT: i32 = 1 + 11;
pub const IRQ_ORION5X_USB1_CTRL: i32 = 1 + 12;
pub const IRQ_ORION5X_DEV_BUS_ERR: i32 = 1 + 14;
pub const IRQ_ORION5X_PCI_ERR: i32 = 1 + 15;
pub const IRQ_ORION5X_USB_BR_ERR: i32 = 1 + 16;
pub const IRQ_ORION5X_USB0_CTRL: i32 = 1 + 17;
pub const IRQ_ORION5X_ETH_RX: i32 = 1 + 18;
pub const IRQ_ORION5X_ETH_TX: i32 = 1 + 19;
pub const IRQ_ORION5X_ETH_MISC: i32 = 1 + 20;
pub const IRQ_ORION5X_ETH_SUM: i32 = 1 + 21;
pub const IRQ_ORION5X_ETH_ERR: i32 = 1 + 22;
pub const IRQ_ORION5X_IDMA_ERR: i32 = 1 + 23;
pub const IRQ_ORION5X_IDMA_0: i32 = 1 + 24;
pub const IRQ_ORION5X_IDMA_1: i32 = 1 + 25;
pub const IRQ_ORION5X_IDMA_2: i32 = 1 + 26;
pub const IRQ_ORION5X_IDMA_3: i32 = 1 + 27;
pub const IRQ_ORION5X_CESA: i32 = 1 + 28;
pub const IRQ_ORION5X_SATA: i32 = 1 + 29;
pub const IRQ_ORION5X_XOR0: i32 = 1 + 30;
pub const IRQ_ORION5X_XOR1: i32 = 1 + 31;

/*
 * Orion General Purpose Pins
 */
pub const IRQ_ORION5X_GPIO_START: i32 = 33;
pub const NR_GPIO_IRQS: i32 = 32;

pub const ORION5X_NR_IRQS: i32 = IRQ_ORION5X_GPIO_START + NR_GPIO_IRQS;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
