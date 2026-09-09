/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Synopsys DesignWare I2C register definitions
 *
 * Copyright (C) 2026, Intel Corporation
 */

/*
 * Registers offset
 */
pub const DW_IC_CON: u32 = 0x00;
pub const DW_IC_TAR: u32 = 0x04;
pub const DW_IC_SAR: u32 = 0x08;
pub const DW_IC_DATA_CMD: u32 = 0x10;
pub const DW_IC_SS_SCL_HCNT: u32 = 0x14;
pub const DW_IC_SS_SCL_LCNT: u32 = 0x18;
pub const DW_IC_FS_SCL_HCNT: u32 = 0x1c;
pub const DW_IC_FS_SCL_LCNT: u32 = 0x20;
pub const DW_IC_HS_SCL_HCNT: u32 = 0x24;
pub const DW_IC_HS_SCL_LCNT: u32 = 0x28;
pub const DW_IC_INTR_STAT: u32 = 0x2c;
pub const DW_IC_INTR_MASK: u32 = 0x30;
pub const DW_IC_RAW_INTR_STAT: u32 = 0x34;
pub const DW_IC_RX_TL: u32 = 0x38;
pub const DW_IC_TX_TL: u32 = 0x3c;
pub const DW_IC_CLR_INTR: u32 = 0x40;
pub const DW_IC_CLR_RX_UNDER: u32 = 0x44;
pub const DW_IC_CLR_RX_OVER: u32 = 0x48;
pub const DW_IC_CLR_TX_OVER: u32 = 0x4c;
pub const DW_IC_CLR_RD_REQ: u32 = 0x50;
pub const DW_IC_CLR_TX_ABRT: u32 = 0x54;
pub const DW_IC_CLR_RX_DONE: u32 = 0x58;
pub const DW_IC_CLR_ACTIVITY: u32 = 0x5c;
pub const DW_IC_CLR_STOP_DET: u32 = 0x60;
pub const DW_IC_CLR_START_DET: u32 = 0x64;
pub const DW_IC_CLR_GEN_CALL: u32 = 0x68;
pub const DW_IC_ENABLE: u32 = 0x6c;
pub const DW_IC_STATUS: u32 = 0x70;
pub const DW_IC_TXFLR: u32 = 0x74;
pub const DW_IC_RXFLR: u32 = 0x78;
pub const DW_IC_SDA_HOLD: u32 = 0x7c;
pub const DW_IC_TX_ABRT_SOURCE: u32 = 0x80;
pub const DW_IC_ENABLE_STATUS: u32 = 0x9c;
pub const DW_IC_CLR_RESTART_DET: u32 = 0xa8;
pub const DW_IC_SMBUS_INTR_STAT: u32 = 0xc8;
pub const DW_IC_SMBUS_INTR_MASK: u32 = 0xcc;
pub const DW_IC_CLR_SMBUS_INTR: u32 = 0xd4;
pub const DW_IC_COMP_PARAM_1: u32 = 0xf4;
pub const DW_IC_COMP_VERSION: u32 = 0xf8;
pub const DW_IC_COMP_TYPE: u32 = 0xfc;

/* DW_IC_CON bits */
pub const DW_IC_CON_MASTER: u32 = 1u32 << 0;
pub const DW_IC_CON_SPEED_STD: u32 = 1u32 << 1;
pub const DW_IC_CON_SPEED_FAST: u32 = 2u32 << 1;
pub const DW_IC_CON_SPEED_HIGH: u32 = 3u32 << 1;
pub const DW_IC_CON_SPEED_MASK: u32 = 0b11u32 << 1;
pub const DW_IC_CON_10BITADDR_SLAVE: u32 = 1u32 << 3;
pub const DW_IC_CON_10BITADDR_MASTER: u32 = 1u32 << 4;
pub const DW_IC_CON_RESTART_EN: u32 = 1u32 << 5;
pub const DW_IC_CON_SLAVE_DISABLE: u32 = 1u32 << 6;
pub const DW_IC_CON_STOP_DET_IFADDRESSED: u32 = 1u32 << 7;
pub const DW_IC_CON_TX_EMPTY_CTRL: u32 = 1u32 << 8;
pub const DW_IC_CON_RX_FIFO_FULL_HLD_CTRL: u32 = 1u32 << 9;
pub const DW_IC_CON_BUS_CLEAR_CTRL: u32 = 1u32 << 11;

/* DW_IC_DATA_CMD bits */
pub const DW_IC_DATA_CMD_DAT: u32 = 0xff;
pub const DW_IC_DATA_CMD_FIRST_DATA_BYTE: u32 = 1u32 << 11;

/* DW_IC_INTR_* bits */
pub const DW_IC_INTR_RX_UNDER: u32 = 1u32 << 0;
pub const DW_IC_INTR_RX_OVER: u32 = 1u32 << 1;
pub const DW_IC_INTR_RX_FULL: u32 = 1u32 << 2;
pub const DW_IC_INTR_TX_OVER: u32 = 1u32 << 3;
pub const DW_IC_INTR_TX_EMPTY: u32 = 1u32 << 4;
pub const DW_IC_INTR_RD_REQ: u32 = 1u32 << 5;
pub const DW_IC_INTR_TX_ABRT: u32 = 1u32 << 6;
pub const DW_IC_INTR_RX_DONE: u32 = 1u32 << 7;
pub const DW_IC_INTR_ACTIVITY: u32 = 1u32 << 8;
pub const DW_IC_INTR_STOP_DET: u32 = 1u32 << 9;
pub const DW_IC_INTR_START_DET: u32 = 1u32 << 10;
pub const DW_IC_INTR_GEN_CALL: u32 = 1u32 << 11;
pub const DW_IC_INTR_RESTART_DET: u32 = 1u32 << 12;
pub const DW_IC_INTR_MST_ON_HOLD: u32 = 1u32 << 13;

/* DW_IC_ENABLE bits */
pub const DW_IC_ENABLE_ENABLE: u32 = 1u32 << 0;
pub const DW_IC_ENABLE_ABORT: u32 = 1u32 << 1;

/* DW_IC_STATUS bits */
pub const DW_IC_STATUS_ACTIVITY: u32 = 1u32 << 0;
pub const DW_IC_STATUS_TFE: u32 = 1u32 << 2;
pub const DW_IC_STATUS_RFNE: u32 = 1u32 << 3;
pub const DW_IC_STATUS_MASTER_ACTIVITY: u32 = 1u32 << 5;
pub const DW_IC_STATUS_SLAVE_ACTIVITY: u32 = 1u32 << 6;
pub const DW_IC_STATUS_MASTER_HOLD_TX_FIFO_EMPTY: u32 = 1u32 << 7;

/* DW_IC_SMBUS_INTR_* bits */
pub const DW_IC_SMBUS_INTR_ALERT: u32 = 1u32 << 10;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
