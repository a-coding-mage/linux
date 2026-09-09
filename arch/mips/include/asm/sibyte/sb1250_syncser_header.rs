/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SB1250 Board Support Package
 * Synchronous Serial Constants, translated from sb1250_syncser.h.
 */

// Dependency: <asm/sibyte/sb1250_defs.h>

/* Serial Mode Configuration Register */
pub const M_SYNCSER_CRC_MODE: u64 = 1u64 << 0;
pub const M_SYNCSER_MSB_FIRST: u64 = 1u64 << 1;
pub const S_SYNCSER_FLAG_NUM: u32 = 2;
pub const M_SYNCSER_FLAG_NUM: u64 = 0xfu64 << S_SYNCSER_FLAG_NUM;
pub const fn V_SYNCSER_FLAG_NUM(x: u64) -> u64 { (x & 0xf) << S_SYNCSER_FLAG_NUM }
pub const M_SYNCSER_FLAG_EN: u64 = 1u64 << 6;
pub const M_SYNCSER_HDLC_EN: u64 = 1u64 << 7;
pub const M_SYNCSER_LOOP_MODE: u64 = 1u64 << 8;
pub const M_SYNCSER_LOOPBACK: u64 = 1u64 << 9;

/* Serial Clock Source and Line Interface Mode Register */
pub const M_SYNCSER_RXCLK_INV: u64 = 1u64 << 0;
pub const M_SYNCSER_RXCLK_EXT: u64 = 1u64 << 1;
pub const S_SYNCSER_RXSYNC_DLY: u32 = 2;
pub const M_SYNCSER_RXSYNC_DLY: u64 = 0x3u64 << S_SYNCSER_RXSYNC_DLY;
pub const fn V_SYNCSER_RXSYNC_DLY(x: u64) -> u64 { (x & 0x3) << S_SYNCSER_RXSYNC_DLY }
pub const M_SYNCSER_RXSYNC_LOW: u64 = 1u64 << 4;
pub const M_SYNCSER_RXSTRB_LOW: u64 = 1u64 << 5;
pub const M_SYNCSER_RXSYNC_EDGE: u64 = 1u64 << 6;
pub const M_SYNCSER_RXSYNC_INT: u64 = 1u64 << 7;
pub const M_SYNCSER_TXCLK_INV: u64 = 1u64 << 8;
pub const M_SYNCSER_TXCLK_EXT: u64 = 1u64 << 9;
pub const S_SYNCSER_TXSYNC_DLY: u32 = 10;
pub const M_SYNCSER_TXSYNC_DLY: u64 = 0x3u64 << S_SYNCSER_TXSYNC_DLY;
pub const fn V_SYNCSER_TXSYNC_DLY(x: u64) -> u64 { (x & 0x3) << S_SYNCSER_TXSYNC_DLY }
pub const M_SYNCSER_TXSYNC_LOW: u64 = 1u64 << 12;
pub const M_SYNCSER_TXSTRB_LOW: u64 = 1u64 << 13;
pub const M_SYNCSER_TXSYNC_EDGE: u64 = 1u64 << 14;
pub const M_SYNCSER_TXSYNC_INT: u64 = 1u64 << 15;

/* Serial Command Register */
pub const M_SYNCSER_CMD_RX_EN: u64 = 1u64 << 0;
pub const M_SYNCSER_CMD_TX_EN: u64 = 1u64 << 1;
pub const M_SYNCSER_CMD_RX_RESET: u64 = 1u64 << 2;
pub const M_SYNCSER_CMD_TX_RESET: u64 = 1u64 << 3;
pub const M_SYNCSER_CMD_TX_PAUSE: u64 = 1u64 << 5;

/* Serial DMA Enable Register */
pub const M_SYNCSER_DMA_RX_EN: u64 = 1u64 << 0;
pub const M_SYNCSER_DMA_TX_EN: u64 = 1u64 << 4;

/* Serial Status Register */
pub const M_SYNCSER_RX_CRCERR: u64 = 1u64 << 0;
pub const M_SYNCSER_RX_ABORT: u64 = 1u64 << 1;
pub const M_SYNCSER_RX_OCTET: u64 = 1u64 << 2;
pub const M_SYNCSER_RX_LONGFRM: u64 = 1u64 << 3;
pub const M_SYNCSER_RX_SHORTFRM: u64 = 1u64 << 4;
pub const M_SYNCSER_RX_OVERRUN: u64 = 1u64 << 5;
pub const M_SYNCSER_RX_SYNC_ERR: u64 = 1u64 << 6;
pub const M_SYNCSER_TX_CRCERR: u64 = 1u64 << 8;
pub const M_SYNCSER_TX_UNDERRUN: u64 = 1u64 << 9;
pub const M_SYNCSER_TX_SYNC_ERR: u64 = 1u64 << 10;
pub const M_SYNCSER_TX_PAUSE_COMPLETE: u64 = 1u64 << 11;
pub const M_SYNCSER_RX_EOP_COUNT: u64 = 1u64 << 16;
pub const M_SYNCSER_RX_EOP_TIMER: u64 = 1u64 << 17;
pub const M_SYNCSER_RX_EOP_SEEN: u64 = 1u64 << 18;
pub const M_SYNCSER_RX_HWM: u64 = 1u64 << 19;
pub const M_SYNCSER_RX_LWM: u64 = 1u64 << 20;
pub const M_SYNCSER_RX_DSCR: u64 = 1u64 << 21;
pub const M_SYNCSER_RX_DERR: u64 = 1u64 << 22;
pub const M_SYNCSER_TX_EOP_COUNT: u64 = 1u64 << 24;
pub const M_SYNCSER_TX_EOP_TIMER: u64 = 1u64 << 25;
pub const M_SYNCSER_TX_EOP_SEEN: u64 = 1u64 << 26;
pub const M_SYNCSER_TX_HWM: u64 = 1u64 << 27;
pub const M_SYNCSER_TX_LWM: u64 = 1u64 << 28;
pub const M_SYNCSER_TX_DSCR: u64 = 1u64 << 29;
pub const M_SYNCSER_TX_DERR: u64 = 1u64 << 30;
pub const M_SYNCSER_TX_DZERO: u64 = 1u64 << 31;

/* Sequencer Table Entry format */
pub const M_SYNCSER_SEQ_LAST: u64 = 1u64 << 0;
pub const M_SYNCSER_SEQ_BYTE: u64 = 1u64 << 1;
pub const S_SYNCSER_SEQ_COUNT: u32 = 2;
pub const M_SYNCSER_SEQ_COUNT: u64 = 0xfu64 << S_SYNCSER_SEQ_COUNT;
pub const fn V_SYNCSER_SEQ_COUNT(x: u64) -> u64 { (x & 0xf) << S_SYNCSER_SEQ_COUNT }
pub const M_SYNCSER_SEQ_ENABLE: u64 = 1u64 << 6;
pub const M_SYNCSER_SEQ_STROBE: u64 = 1u64 << 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
