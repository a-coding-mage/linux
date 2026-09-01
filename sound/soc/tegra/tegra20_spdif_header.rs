// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra20_spdif.h - Definitions for Tegra20 SPDIF driver
 *
 * Author: Stephen Warren <swarren@nvidia.com>
 * Copyright (C) 2011 - NVIDIA, Inc.
 *
 * Based on code copyright/by:
 * Copyright (c) 2008-2009, NVIDIA Corporation
 */

// C header dependency: "tegra_pcm.h"

/* Offsets from TEGRA20_SPDIF_BASE */

pub const TEGRA20_SPDIF_CTRL: u32 = 0x0;
pub const TEGRA20_SPDIF_STATUS: u32 = 0x4;
pub const TEGRA20_SPDIF_STROBE_CTRL: u32 = 0x8;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR: u32 = 0x0C;
pub const TEGRA20_SPDIF_DATA_OUT: u32 = 0x40;
pub const TEGRA20_SPDIF_DATA_IN: u32 = 0x80;
pub const TEGRA20_SPDIF_CH_STA_RX_A: u32 = 0x100;
pub const TEGRA20_SPDIF_CH_STA_RX_B: u32 = 0x104;
pub const TEGRA20_SPDIF_CH_STA_RX_C: u32 = 0x108;
pub const TEGRA20_SPDIF_CH_STA_RX_D: u32 = 0x10C;
pub const TEGRA20_SPDIF_CH_STA_RX_E: u32 = 0x110;
pub const TEGRA20_SPDIF_CH_STA_RX_F: u32 = 0x114;
pub const TEGRA20_SPDIF_CH_STA_TX_A: u32 = 0x140;
pub const TEGRA20_SPDIF_CH_STA_TX_B: u32 = 0x144;
pub const TEGRA20_SPDIF_CH_STA_TX_C: u32 = 0x148;
pub const TEGRA20_SPDIF_CH_STA_TX_D: u32 = 0x14C;
pub const TEGRA20_SPDIF_CH_STA_TX_E: u32 = 0x150;
pub const TEGRA20_SPDIF_CH_STA_TX_F: u32 = 0x154;
pub const TEGRA20_SPDIF_USR_STA_RX_A: u32 = 0x180;
pub const TEGRA20_SPDIF_USR_DAT_TX_A: u32 = 0x1C0;

/* Fields in TEGRA20_SPDIF_CTRL */

/* Start capturing from 0=right, 1=left channel */
pub const TEGRA20_SPDIF_CTRL_CAP_LC: u32 = 1_u32 << 30;

/* SPDIF receiver(RX) enable */
pub const TEGRA20_SPDIF_CTRL_RX_EN: u32 = 1_u32 << 29;

/* SPDIF Transmitter(TX) enable */
pub const TEGRA20_SPDIF_CTRL_TX_EN: u32 = 1_u32 << 28;

/* Transmit Channel status */
pub const TEGRA20_SPDIF_CTRL_TC_EN: u32 = 1_u32 << 27;

/* Transmit user Data */
pub const TEGRA20_SPDIF_CTRL_TU_EN: u32 = 1_u32 << 26;

/* Interrupt on transmit error */
pub const TEGRA20_SPDIF_CTRL_IE_TXE: u32 = 1_u32 << 25;

/* Interrupt on receive error */
pub const TEGRA20_SPDIF_CTRL_IE_RXE: u32 = 1_u32 << 24;

/* Interrupt on invalid preamble */
pub const TEGRA20_SPDIF_CTRL_IE_P: u32 = 1_u32 << 23;

/* Interrupt on "B" preamble */
pub const TEGRA20_SPDIF_CTRL_IE_B: u32 = 1_u32 << 22;

/* Interrupt when block of channel status received */
pub const TEGRA20_SPDIF_CTRL_IE_C: u32 = 1_u32 << 21;

/* Interrupt when a valid information unit (IU) is received */
pub const TEGRA20_SPDIF_CTRL_IE_U: u32 = 1_u32 << 20;

/* Interrupt when RX user FIFO attention level is reached */
pub const TEGRA20_SPDIF_CTRL_QE_RU: u32 = 1_u32 << 19;

/* Interrupt when TX user FIFO attention level is reached */
pub const TEGRA20_SPDIF_CTRL_QE_TU: u32 = 1_u32 << 18;

/* Interrupt when RX data FIFO attention level is reached */
pub const TEGRA20_SPDIF_CTRL_QE_RX: u32 = 1_u32 << 17;

/* Interrupt when TX data FIFO attention level is reached */
pub const TEGRA20_SPDIF_CTRL_QE_TX: u32 = 1_u32 << 16;

/* Loopback test mode enable */
pub const TEGRA20_SPDIF_CTRL_LBK_EN: u32 = 1_u32 << 15;

/*
 * Pack data mode:
 * 0 = Single data (16 bit needs to be  padded to match the
 *     interface data bit size).
 * 1 = Packeted left/right channel data into a single word.
 */
pub const TEGRA20_SPDIF_CTRL_PACK: u32 = 1_u32 << 14;

/*
 * 00 = 16bit data
 * 01 = 20bit data
 * 10 = 24bit data
 * 11 = raw data
 */
pub const TEGRA20_SPDIF_BIT_MODE_16BIT: u32 = 0;
pub const TEGRA20_SPDIF_BIT_MODE_20BIT: u32 = 1;
pub const TEGRA20_SPDIF_BIT_MODE_24BIT: u32 = 2;
pub const TEGRA20_SPDIF_BIT_MODE_RAW: u32 = 3;

pub const TEGRA20_SPDIF_CTRL_BIT_MODE_SHIFT: u32 = 12;
pub const TEGRA20_SPDIF_CTRL_BIT_MODE_MASK: u32 =
    3_u32 << TEGRA20_SPDIF_CTRL_BIT_MODE_SHIFT;
pub const TEGRA20_SPDIF_CTRL_BIT_MODE_16BIT: u32 =
    TEGRA20_SPDIF_BIT_MODE_16BIT << TEGRA20_SPDIF_CTRL_BIT_MODE_SHIFT;
pub const TEGRA20_SPDIF_CTRL_BIT_MODE_20BIT: u32 =
    TEGRA20_SPDIF_BIT_MODE_20BIT << TEGRA20_SPDIF_CTRL_BIT_MODE_SHIFT;
pub const TEGRA20_SPDIF_CTRL_BIT_MODE_24BIT: u32 =
    TEGRA20_SPDIF_BIT_MODE_24BIT << TEGRA20_SPDIF_CTRL_BIT_MODE_SHIFT;
pub const TEGRA20_SPDIF_CTRL_BIT_MODE_RAW: u32 =
    TEGRA20_SPDIF_BIT_MODE_RAW << TEGRA20_SPDIF_CTRL_BIT_MODE_SHIFT;

/* Fields in TEGRA20_SPDIF_STATUS */

/*
 * Note: IS_P, IS_B, IS_C, and IS_U are sticky bits. Software must
 * write a 1 to the corresponding bit location to clear the status.
 */

/*
 * Receiver(RX) shifter is busy receiving data.
 * This bit is asserted when the receiver first locked onto the
 * preamble of the data stream after RX_EN is asserted. This bit is
 * deasserted when either,
 * (a) the end of a frame is reached after RX_EN is deeasserted, or
 * (b) the SPDIF data stream becomes inactive.
 */
pub const TEGRA20_SPDIF_STATUS_RX_BSY: u32 = 1_u32 << 29;

/*
 * Transmitter(TX) shifter is busy transmitting data.
 * This bit is asserted when TX_EN is asserted.
 * This bit is deasserted when the end of a frame is reached after
 * TX_EN is deasserted.
 */
pub const TEGRA20_SPDIF_STATUS_TX_BSY: u32 = 1_u32 << 28;

/*
 * TX is busy shifting out channel status.
 * This bit is asserted when both TX_EN and TC_EN are asserted and
 * data from CH_STA_TX_A register is loaded into the internal shifter.
 * This bit is deasserted when either,
 * (a) the end of a frame is reached after TX_EN is deasserted, or
 * (b) CH_STA_TX_F register is loaded into the internal shifter.
 */
pub const TEGRA20_SPDIF_STATUS_TC_BSY: u32 = 1_u32 << 27;

/*
 * TX User data FIFO busy.
 * This bit is asserted when TX_EN and TXU_EN are asserted and
 * there's data in the TX user FIFO.  This bit is deassert when either,
 * (a) the end of a frame is reached after TX_EN is deasserted, or
 * (b) there's no data left in the TX user FIFO.
 */
pub const TEGRA20_SPDIF_STATUS_TU_BSY: u32 = 1_u32 << 26;

/* TX FIFO Underrun error status */
pub const TEGRA20_SPDIF_STATUS_TX_ERR: u32 = 1_u32 << 25;

/* RX FIFO Overrun error status */
pub const TEGRA20_SPDIF_STATUS_RX_ERR: u32 = 1_u32 << 24;

/* Preamble status: 0=Preamble OK, 1=bad/missing preamble */
pub const TEGRA20_SPDIF_STATUS_IS_P: u32 = 1_u32 << 23;

/* B-preamble detection status: 0=not detected, 1=B-preamble detected */
pub const TEGRA20_SPDIF_STATUS_IS_B: u32 = 1_u32 << 22;

/*
 * RX channel block data receive status:
 * 0=entire block not received yet.
 * 1=received entire block of channel status,
 */
pub const TEGRA20_SPDIF_STATUS_IS_C: u32 = 1_u32 << 21;

/* RX User Data Valid flag:  1=valid IU detected, 0 = no IU detected. */
pub const TEGRA20_SPDIF_STATUS_IS_U: u32 = 1_u32 << 20;

/*
 * RX User FIFO Status:
 * 1=attention level reached, 0=attention level not reached.
 */
pub const TEGRA20_SPDIF_STATUS_QS_RU: u32 = 1_u32 << 19;

/*
 * TX User FIFO Status:
 * 1=attention level reached, 0=attention level not reached.
 */
pub const TEGRA20_SPDIF_STATUS_QS_TU: u32 = 1_u32 << 18;

/*
 * RX Data FIFO Status:
 * 1=attention level reached, 0=attention level not reached.
 */
pub const TEGRA20_SPDIF_STATUS_QS_RX: u32 = 1_u32 << 17;

/*
 * TX Data FIFO Status:
 * 1=attention level reached, 0=attention level not reached.
 */
pub const TEGRA20_SPDIF_STATUS_QS_TX: u32 = 1_u32 << 16;

/* Fields in TEGRA20_SPDIF_STROBE_CTRL */

/*
 * Indicates the approximate number of detected SPDIFIN clocks within a
 * bi-phase period.
 */
pub const TEGRA20_SPDIF_STROBE_CTRL_PERIOD_SHIFT: u32 = 16;
pub const TEGRA20_SPDIF_STROBE_CTRL_PERIOD_MASK: u32 =
    0xff_u32 << TEGRA20_SPDIF_STROBE_CTRL_PERIOD_SHIFT;

/* Data strobe mode: 0=Auto-locked 1=Manual locked */
pub const TEGRA20_SPDIF_STROBE_CTRL_STROBE: u32 = 1_u32 << 15;

/*
 * Manual data strobe time within the bi-phase clock period (in terms of
 * the number of over-sampling clocks).
 */
pub const TEGRA20_SPDIF_STROBE_CTRL_DATA_STROBES_SHIFT: u32 = 8;
pub const TEGRA20_SPDIF_STROBE_CTRL_DATA_STROBES_MASK: u32 =
    0x1f_u32 << TEGRA20_SPDIF_STROBE_CTRL_DATA_STROBES_SHIFT;

/*
 * Manual SPDIFIN bi-phase clock period (in terms of the number of
 * over-sampling clocks).
 */
pub const TEGRA20_SPDIF_STROBE_CTRL_CLOCK_PERIOD_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_STROBE_CTRL_CLOCK_PERIOD_MASK: u32 =
    0x3f_u32 << TEGRA20_SPDIF_STROBE_CTRL_CLOCK_PERIOD_SHIFT;

/* Fields in SPDIF_DATA_FIFO_CSR */

/* Clear Receiver User FIFO (RX USR.FIFO) */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RU_CLR: u32 = 1_u32 << 31;

pub const TEGRA20_SPDIF_FIFO_ATN_LVL_U_ONE_SLOT: u32 = 0;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_U_TWO_SLOTS: u32 = 1;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_U_THREE_SLOTS: u32 = 2;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_U_FOUR_SLOTS: u32 = 3;

/* RU FIFO attention level */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_SHIFT: u32 = 29;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_MASK: u32 =
    0x3_u32 << TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_RU1_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_U_ONE_SLOT << TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_RU2_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_U_TWO_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_RU3_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_U_THREE_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_RU4_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_U_FOUR_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_RU_ATN_LVL_SHIFT;

/* Number of RX USR.FIFO levels with valid data. */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RU_FULL_COUNT_SHIFT: u32 = 24;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RU_FULL_COUNT_MASK: u32 =
    0x1f_u32 << TEGRA20_SPDIF_DATA_FIFO_CSR_RU_FULL_COUNT_SHIFT;

/* Clear Transmitter User FIFO (TX USR.FIFO) */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TU_CLR: u32 = 1_u32 << 23;

/* TU FIFO attention level */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_SHIFT: u32 = 21;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_MASK: u32 =
    0x3_u32 << TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_TU1_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_U_ONE_SLOT << TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_TU2_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_U_TWO_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_TU3_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_U_THREE_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_TU4_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_U_FOUR_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_TU_ATN_LVL_SHIFT;

/* Number of TX USR.FIFO levels that could be filled. */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TU_EMPTY_COUNT_SHIFT: u32 = 16;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TU_EMPTY_COUNT_MASK: u32 =
    0x1f_u32 << SPDIF_DATA_FIFO_CSR_TU_EMPTY_COUNT_SHIFT;

/* Clear Receiver Data FIFO (RX DATA.FIFO) */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RX_CLR: u32 = 1_u32 << 15;

pub const TEGRA20_SPDIF_FIFO_ATN_LVL_D_ONE_SLOT: u32 = 0;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_D_FOUR_SLOTS: u32 = 1;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_D_EIGHT_SLOTS: u32 = 2;
pub const TEGRA20_SPDIF_FIFO_ATN_LVL_D_TWELVE_SLOTS: u32 = 3;

/* RU FIFO attention level */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_SHIFT: u32 = 13;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_MASK: u32 =
    0x3_u32 << TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_RU1_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_D_ONE_SLOT << TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_RU4_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_D_FOUR_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_RU8_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_D_EIGHT_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_RU12_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_D_TWELVE_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_RX_ATN_LVL_SHIFT;

/* Number of RX DATA.FIFO levels with valid data. */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RX_FULL_COUNT_SHIFT: u32 = 8;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_RX_FULL_COUNT_MASK: u32 =
    0x1f_u32 << TEGRA20_SPDIF_DATA_FIFO_CSR_RX_FULL_COUNT_SHIFT;

/* Clear Transmitter Data FIFO (TX DATA.FIFO) */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TX_CLR: u32 = 1_u32 << 7;

/* TU FIFO attention level */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_SHIFT: u32 = 5;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_MASK: u32 =
    0x3_u32 << TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_TU1_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_D_ONE_SLOT << TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_TU4_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_D_FOUR_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_TU8_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_D_EIGHT_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_SHIFT;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_TU12_WORD_FULL: u32 =
    TEGRA20_SPDIF_FIFO_ATN_LVL_D_TWELVE_SLOTS << TEGRA20_SPDIF_DATA_FIFO_CSR_TX_ATN_LVL_SHIFT;

/* Number of TX DATA.FIFO levels that could be filled. */
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TX_EMPTY_COUNT_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_DATA_FIFO_CSR_TX_EMPTY_COUNT_MASK: u32 =
    0x1f_u32 << SPDIF_DATA_FIFO_CSR_TX_EMPTY_COUNT_SHIFT;

/* Fields in TEGRA20_SPDIF_DATA_OUT */

/*
 * This register has 5 different formats:
 * 16-bit        (BIT_MODE=00, PACK=0)
 * 20-bit        (BIT_MODE=01, PACK=0)
 * 24-bit        (BIT_MODE=10, PACK=0)
 * raw           (BIT_MODE=11, PACK=0)
 * 16-bit packed (BIT_MODE=00, PACK=1)
 */

pub const TEGRA20_SPDIF_DATA_OUT_DATA_16_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_DATA_OUT_DATA_16_MASK: u32 =
    0xffff_u32 << TEGRA20_SPDIF_DATA_OUT_DATA_16_SHIFT;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_20_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_DATA_OUT_DATA_20_MASK: u32 =
    0xfffff_u32 << TEGRA20_SPDIF_DATA_OUT_DATA_20_SHIFT;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_24_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_DATA_OUT_DATA_24_MASK: u32 =
    0xffffff_u32 << TEGRA20_SPDIF_DATA_OUT_DATA_24_SHIFT;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_P: u32 = 1_u32 << 31;
pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_C: u32 = 1_u32 << 30;
pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_U: u32 = 1_u32 << 29;
pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_V: u32 = 1_u32 << 28;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_DATA_SHIFT: u32 = 8;
pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_DATA_MASK: u32 =
    0xfffff_u32 << TEGRA20_SPDIF_DATA_OUT_DATA_RAW_DATA_SHIFT;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_AUX_SHIFT: u32 = 4;
pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_AUX_MASK: u32 =
    0xf_u32 << TEGRA20_SPDIF_DATA_OUT_DATA_RAW_AUX_SHIFT;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_PREAMBLE_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_DATA_OUT_DATA_RAW_PREAMBLE_MASK: u32 =
    0xf_u32 << TEGRA20_SPDIF_DATA_OUT_DATA_RAW_PREAMBLE_SHIFT;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_16_PACKED_RIGHT_SHIFT: u32 = 16;
pub const TEGRA20_SPDIF_DATA_OUT_DATA_16_PACKED_RIGHT_MASK: u32 =
    0xffff_u32 << TEGRA20_SPDIF_DATA_OUT_DATA_16_PACKED_RIGHT_SHIFT;

pub const TEGRA20_SPDIF_DATA_OUT_DATA_16_PACKED_LEFT_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_DATA_OUT_DATA_16_PACKED_LEFT_MASK: u32 =
    0xffff_u32 << TEGRA20_SPDIF_DATA_OUT_DATA_16_PACKED_LEFT_SHIFT;

/* Fields in TEGRA20_SPDIF_DATA_IN */

/*
 * This register has 5 different formats:
 * 16-bit        (BIT_MODE=00, PACK=0)
 * 20-bit        (BIT_MODE=01, PACK=0)
 * 24-bit        (BIT_MODE=10, PACK=0)
 * raw           (BIT_MODE=11, PACK=0)
 * 16-bit packed (BIT_MODE=00, PACK=1)
 *
 * Bits 31:24 are common to all modes except 16-bit packed
 */

pub const TEGRA20_SPDIF_DATA_IN_DATA_P: u32 = 1_u32 << 31;
pub const TEGRA20_SPDIF_DATA_IN_DATA_C: u32 = 1_u32 << 30;
pub const TEGRA20_SPDIF_DATA_IN_DATA_U: u32 = 1_u32 << 29;
pub const TEGRA20_SPDIF_DATA_IN_DATA_V: u32 = 1_u32 << 28;

pub const TEGRA20_SPDIF_DATA_IN_DATA_PREAMBLE_SHIFT: u32 = 24;
pub const TEGRA20_SPDIF_DATA_IN_DATA_PREAMBLE_MASK: u32 =
    0xf_u32 << TEGRA20_SPDIF_DATA_IN_DATA_PREAMBLE_SHIFT;

pub const TEGRA20_SPDIF_DATA_IN_DATA_16_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_DATA_IN_DATA_16_MASK: u32 =
    0xffff_u32 << TEGRA20_SPDIF_DATA_IN_DATA_16_SHIFT;

pub const TEGRA20_SPDIF_DATA_IN_DATA_20_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_DATA_IN_DATA_20_MASK: u32 =
    0xfffff_u32 << TEGRA20_SPDIF_DATA_IN_DATA_20_SHIFT;

pub const TEGRA20_SPDIF_DATA_IN_DATA_24_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_DATA_IN_DATA_24_MASK: u32 =
    0xffffff_u32 << TEGRA20_SPDIF_DATA_IN_DATA_24_SHIFT;

pub const TEGRA20_SPDIF_DATA_IN_DATA_RAW_DATA_SHIFT: u32 = 8;
pub const TEGRA20_SPDIF_DATA_IN_DATA_RAW_DATA_MASK: u32 =
    0xfffff_u32 << TEGRA20_SPDIF_DATA_IN_DATA_RAW_DATA_SHIFT;

pub const TEGRA20_SPDIF_DATA_IN_DATA_RAW_AUX_SHIFT: u32 = 4;
pub const TEGRA20_SPDIF_DATA_IN_DATA_RAW_AUX_MASK: u32 =
    0xf_u32 << TEGRA20_SPDIF_DATA_IN_DATA_RAW_AUX_SHIFT;

pub const TEGRA20_SPDIF_DATA_IN_DATA_RAW_PREAMBLE_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_DATA_IN_DATA_RAW_PREAMBLE_MASK: u32 =
    0xf_u32 << TEGRA20_SPDIF_DATA_IN_DATA_RAW_PREAMBLE_SHIFT;

pub const TEGRA20_SPDIF_DATA_IN_DATA_16_PACKED_RIGHT_SHIFT: u32 = 16;
pub const TEGRA20_SPDIF_DATA_IN_DATA_16_PACKED_RIGHT_MASK: u32 =
    0xffff_u32 << TEGRA20_SPDIF_DATA_IN_DATA_16_PACKED_RIGHT_SHIFT;

pub const TEGRA20_SPDIF_DATA_IN_DATA_16_PACKED_LEFT_SHIFT: u32 = 0;
pub const TEGRA20_SPDIF_DATA_IN_DATA_16_PACKED_LEFT_MASK: u32 =
    0xffff_u32 << TEGRA20_SPDIF_DATA_IN_DATA_16_PACKED_LEFT_SHIFT;

/* Fields in TEGRA20_SPDIF_CH_STA_RX_A */
/* Fields in TEGRA20_SPDIF_CH_STA_RX_B */
/* Fields in TEGRA20_SPDIF_CH_STA_RX_C */
/* Fields in TEGRA20_SPDIF_CH_STA_RX_D */
/* Fields in TEGRA20_SPDIF_CH_STA_RX_E */
/* Fields in TEGRA20_SPDIF_CH_STA_RX_F */

/*
 * The 6-word receive channel data page buffer holds a block (192 frames) of
 * channel status information. The order of receive is from LSB to MSB
 * bit, and from CH_STA_RX_A to CH_STA_RX_F then back to CH_STA_RX_A.
 */

/* Fields in TEGRA20_SPDIF_CH_STA_TX_A */
/* Fields in TEGRA20_SPDIF_CH_STA_TX_B */
/* Fields in TEGRA20_SPDIF_CH_STA_TX_C */
/* Fields in TEGRA20_SPDIF_CH_STA_TX_D */
/* Fields in TEGRA20_SPDIF_CH_STA_TX_E */
/* Fields in TEGRA20_SPDIF_CH_STA_TX_F */

/*
 * The 6-word transmit channel data page buffer holds a block (192 frames) of
 * channel status information. The order of transmission is from LSB to MSB
 * bit, and from CH_STA_TX_A to CH_STA_TX_F then back to CH_STA_TX_A.
 */

/* Fields in TEGRA20_SPDIF_USR_STA_RX_A */

/*
 * This 4-word deep FIFO receives user FIFO field information. The order of
 * receive is from LSB to MSB bit.
 */

/* Fields in TEGRA20_SPDIF_USR_DAT_TX_A */

/*
 * This 4-word deep FIFO transmits user FIFO field information. The order of
 * transmission is from LSB to MSB bit.
 */

#[repr(C)]
pub struct tegra20_spdif {
    pub clk_spdif_out: *mut clk,
    pub capture_dma_data: snd_dmaengine_dai_dma_data,
    pub playback_dma_data: snd_dmaengine_dai_dma_data,
    pub regmap: *mut regmap,
    pub reset: *mut reset_control,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
