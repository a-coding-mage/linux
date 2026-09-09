/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * DMA request number (DRQ) definitions for non-secure peripherals of
 * the SpacemiT K3 PDMA.
 *
 * Copyright (c) 2025 SpacemiT
 * Copyright (c) 2026 Guodong Xu <docular.xu@gmail.com>
 */

/* Translated from the C header guard: _DTS_SPACEMIT_K3_PDMA_H */

/* UART DMA request numbers */
pub const K3_PDMA_UART0_TX: u32 = 3;
pub const K3_PDMA_UART0_RX: u32 = 4;
pub const K3_PDMA_UART2_TX: u32 = 5;
pub const K3_PDMA_UART2_RX: u32 = 6;
pub const K3_PDMA_UART3_TX: u32 = 7;
pub const K3_PDMA_UART3_RX: u32 = 8;
pub const K3_PDMA_UART4_TX: u32 = 9;
pub const K3_PDMA_UART4_RX: u32 = 10;
pub const K3_PDMA_UART5_TX: u32 = 25;
pub const K3_PDMA_UART5_RX: u32 = 26;
pub const K3_PDMA_UART6_TX: u32 = 27;
pub const K3_PDMA_UART6_RX: u32 = 28;
pub const K3_PDMA_UART7_TX: u32 = 29;
pub const K3_PDMA_UART7_RX: u32 = 30;
pub const K3_PDMA_UART8_TX: u32 = 31;
pub const K3_PDMA_UART8_RX: u32 = 32;
pub const K3_PDMA_UART9_TX: u32 = 33;
pub const K3_PDMA_UART9_RX: u32 = 34;
pub const K3_PDMA_UART10_TX: u32 = 53;
pub const K3_PDMA_UART10_RX: u32 = 54;

/* I2C DMA request numbers */
pub const K3_PDMA_I2C0_TX: u32 = 11;
pub const K3_PDMA_I2C0_RX: u32 = 12;
pub const K3_PDMA_I2C1_TX: u32 = 13;
pub const K3_PDMA_I2C1_RX: u32 = 14;
pub const K3_PDMA_I2C2_TX: u32 = 15;
pub const K3_PDMA_I2C2_RX: u32 = 16;
pub const K3_PDMA_I2C4_TX: u32 = 17;
pub const K3_PDMA_I2C4_RX: u32 = 18;
pub const K3_PDMA_I2C5_TX: u32 = 35;
pub const K3_PDMA_I2C5_RX: u32 = 36;
pub const K3_PDMA_I2C6_TX: u32 = 37;
pub const K3_PDMA_I2C6_RX: u32 = 38;
pub const K3_PDMA_I2C8_TX: u32 = 41;
pub const K3_PDMA_I2C8_RX: u32 = 42;

/* SSP/SPI DMA request numbers */
pub const K3_PDMA_SSP3_TX: u32 = 19;
pub const K3_PDMA_SSP3_RX: u32 = 20;
pub const K3_PDMA_SSPA0_TX: u32 = 21;
pub const K3_PDMA_SSPA0_RX: u32 = 22;
pub const K3_PDMA_SSPA1_TX: u32 = 23;
pub const K3_PDMA_SSPA1_RX: u32 = 24;
pub const K3_PDMA_SSPA2_TX: u32 = 56;
pub const K3_PDMA_SSPA2_RX: u32 = 57;
pub const K3_PDMA_SSPA3_TX: u32 = 58;
pub const K3_PDMA_SSPA3_RX: u32 = 59;
pub const K3_PDMA_SSPA4_TX: u32 = 60;
pub const K3_PDMA_SSPA4_RX: u32 = 61;
pub const K3_PDMA_SSPA5_TX: u32 = 62;
pub const K3_PDMA_SSPA5_RX: u32 = 63;

/* CAN DMA request numbers */
pub const K3_PDMA_CAN0_RX: u32 = 43;
pub const K3_PDMA_CAN1_RX: u32 = 44;
pub const K3_PDMA_CAN2_RX: u32 = 51;
pub const K3_PDMA_CAN3_RX: u32 = 52;

/* SSP0/1 DMA request numbers */
pub const K3_PDMA_SSP0_TX: u32 = 64;
pub const K3_PDMA_SSP0_RX: u32 = 65;
pub const K3_PDMA_SSP1_TX: u32 = 66;
pub const K3_PDMA_SSP1_RX: u32 = 67;

/* QSPI DMA request numbers */
pub const K3_PDMA_QSPI_RX: u32 = 84;
pub const K3_PDMA_QSPI_TX: u32 = 85;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
