/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Atheros AR933X UART defines
 *
 *  Copyright (C) 2011 Gabor Juhos <juhosg@openwrt.org>
 */

pub const AR933X_UART_REGS_SIZE: u32 = 20;
pub const AR933X_UART_FIFO_SIZE: u32 = 16;

pub const AR933X_UART_DATA_REG: u32 = 0x00;
pub const AR933X_UART_CS_REG: u32 = 0x04;
pub const AR933X_UART_CLOCK_REG: u32 = 0x08;
pub const AR933X_UART_INT_REG: u32 = 0x0c;
pub const AR933X_UART_INT_EN_REG: u32 = 0x10;

pub const AR933X_UART_DATA_TX_RX_MASK: u32 = 0xff;
pub const AR933X_UART_DATA_RX_CSR: u32 = 1 << 8;
pub const AR933X_UART_DATA_TX_CSR: u32 = 1 << 9;

pub const AR933X_UART_CS_PARITY_S: u32 = 0;
pub const AR933X_UART_CS_PARITY_M: u32 = 0x3;
pub const AR933X_UART_CS_PARITY_NONE: u32 = 0;
pub const AR933X_UART_CS_PARITY_ODD: u32 = 2;
pub const AR933X_UART_CS_PARITY_EVEN: u32 = 3;
pub const AR933X_UART_CS_IF_MODE_S: u32 = 2;
pub const AR933X_UART_CS_IF_MODE_M: u32 = 0x3;
pub const AR933X_UART_CS_IF_MODE_NONE: u32 = 0;
pub const AR933X_UART_CS_IF_MODE_DTE: u32 = 1;
pub const AR933X_UART_CS_IF_MODE_DCE: u32 = 2;
pub const AR933X_UART_CS_FLOW_CTRL_S: u32 = 4;
pub const AR933X_UART_CS_FLOW_CTRL_M: u32 = 0x3;
pub const AR933X_UART_CS_DMA_EN: u32 = 1 << 6;
pub const AR933X_UART_CS_TX_READY_ORIDE: u32 = 1 << 7;
pub const AR933X_UART_CS_RX_READY_ORIDE: u32 = 1 << 8;
pub const AR933X_UART_CS_TX_READY: u32 = 1 << 9;
pub const AR933X_UART_CS_RX_BREAK: u32 = 1 << 10;
pub const AR933X_UART_CS_TX_BREAK: u32 = 1 << 11;
pub const AR933X_UART_CS_HOST_INT: u32 = 1 << 12;
pub const AR933X_UART_CS_HOST_INT_EN: u32 = 1 << 13;
pub const AR933X_UART_CS_TX_BUSY: u32 = 1 << 14;
pub const AR933X_UART_CS_RX_BUSY: u32 = 1 << 15;

pub const AR933X_UART_CLOCK_STEP_M: u32 = 0xffff;
pub const AR933X_UART_CLOCK_SCALE_M: u32 = 0xfff;
pub const AR933X_UART_CLOCK_SCALE_S: u32 = 16;
// The C header repeats AR933X_UART_CLOCK_STEP_M; Rust bindings retain one item.

pub const AR933X_UART_INT_RX_VALID: u32 = 1 << 0;
pub const AR933X_UART_INT_TX_READY: u32 = 1 << 1;
pub const AR933X_UART_INT_RX_FRAMING_ERR: u32 = 1 << 2;
pub const AR933X_UART_INT_RX_OFLOW_ERR: u32 = 1 << 3;
pub const AR933X_UART_INT_TX_OFLOW_ERR: u32 = 1 << 4;
pub const AR933X_UART_INT_RX_PARITY_ERR: u32 = 1 << 5;
pub const AR933X_UART_INT_RX_BREAK_ON: u32 = 1 << 6;
pub const AR933X_UART_INT_RX_BREAK_OFF: u32 = 1 << 7;
pub const AR933X_UART_INT_RX_FULL: u32 = 1 << 8;
pub const AR933X_UART_INT_TX_EMPTY: u32 = 1 << 9;
pub const AR933X_UART_INT_ALLINTS: u32 = 0x3ff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
