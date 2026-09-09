/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  include/linux/mmc/sdio.h
 *
 *  Copyright 2006-2007 Pierre Ossman
 */

/* SDIO commands                         type  argument     response */
pub const SD_IO_SEND_OP_COND: u32 = 5; /* bcr  [23:0] OCR         R4  */
pub const SD_IO_RW_DIRECT: u32 = 52; /* ac   [31:0] See below   R5  */
pub const SD_IO_RW_EXTENDED: u32 = 53; /* adtc [31:0] See below   R5  */

/*
 * SD_IO_RW_DIRECT argument format:
 *
 *      [31] R/W flag
 *      [30:28] Function number
 *      [27] RAW flag
 *      [25:9] Register address
 *      [7:0] Data
 */

/*
 * SD_IO_RW_EXTENDED argument format:
 *
 *      [31] R/W flag
 *      [30:28] Function number
 *      [27] Block mode
 *      [26] Increment address
 *      [25:9] Register address
 *      [8:0] Byte/block count
 */

pub const R4_18V_PRESENT: u32 = 1 << 24;
pub const R4_MEMORY_PRESENT: u32 = 1 << 27;

/*
  SDIO status in R5
  Type
	e : error bit
	s : status bit
	r : detected and set for the actual command response
	x : detected and set during command execution. the host must poll
            the card by sending status command in order to read these bits.
  Clear condition
    a : according to the card state
    b : always related to the previous command. Reception of
            a valid command will clear it (with a delay of one command)
    c : clear by read
 */

pub const R5_COM_CRC_ERROR: u32 = 1 << 15; /* er, b */
pub const R5_ILLEGAL_COMMAND: u32 = 1 << 14; /* er, b */
pub const R5_ERROR: u32 = 1 << 11; /* erx, c */
pub const R5_FUNCTION_NUMBER: u32 = 1 << 9; /* er, c */
pub const R5_OUT_OF_RANGE: u32 = 1 << 8; /* er, c */
#[inline]
pub const fn R5_STATUS(x: u32) -> u32 {
    x & 0xCB00
}
#[inline]
pub const fn R5_IO_CURRENT_STATE(x: u32) -> u32 {
    (x & 0x3000) >> 12 /* s, b */
}

/*
 * Card Common Control Registers (CCCR)
 */

pub const SDIO_CCCR_CCCR: u32 = 0x00;

pub const SDIO_CCCR_REV_1_00: u32 = 0; /* CCCR/FBR Version 1.00 */
pub const SDIO_CCCR_REV_1_10: u32 = 1; /* CCCR/FBR Version 1.10 */
pub const SDIO_CCCR_REV_1_20: u32 = 2; /* CCCR/FBR Version 1.20 */
pub const SDIO_CCCR_REV_3_00: u32 = 3; /* CCCR/FBR Version 3.00 */

pub const SDIO_SDIO_REV_1_00: u32 = 0; /* SDIO Spec Version 1.00 */
pub const SDIO_SDIO_REV_1_10: u32 = 1; /* SDIO Spec Version 1.10 */
pub const SDIO_SDIO_REV_1_20: u32 = 2; /* SDIO Spec Version 1.20 */
pub const SDIO_SDIO_REV_2_00: u32 = 3; /* SDIO Spec Version 2.00 */
pub const SDIO_SDIO_REV_3_00: u32 = 4; /* SDIO Spec Version 3.00 */

pub const SDIO_CCCR_SD: u32 = 0x01;

pub const SDIO_SD_REV_1_01: u32 = 0; /* SD Physical Spec Version 1.01 */
pub const SDIO_SD_REV_1_10: u32 = 1; /* SD Physical Spec Version 1.10 */
pub const SDIO_SD_REV_2_00: u32 = 2; /* SD Physical Spec Version 2.00 */
pub const SDIO_SD_REV_3_00: u32 = 3; /* SD Physical Spec Version 3.00 */

pub const SDIO_CCCR_IOEx: u32 = 0x02;
pub const SDIO_CCCR_IORx: u32 = 0x03;

pub const SDIO_CCCR_IENx: u32 = 0x04; /* Function/Master Interrupt Enable */
pub const SDIO_CCCR_INTx: u32 = 0x05; /* Function Interrupt Pending */

pub const SDIO_CCCR_ABORT: u32 = 0x06; /* function abort/card reset */

pub const SDIO_CCCR_IF: u32 = 0x07; /* bus interface controls */

pub const SDIO_BUS_WIDTH_MASK: u32 = 0x03; /* data bus width setting */
pub const SDIO_BUS_WIDTH_1BIT: u32 = 0x00;
pub const SDIO_BUS_WIDTH_RESERVED: u32 = 0x01;
pub const SDIO_BUS_WIDTH_4BIT: u32 = 0x02;
pub const SDIO_BUS_ECSI: u32 = 0x20; /* Enable continuous SPI interrupt */
pub const SDIO_BUS_SCSI: u32 = 0x40; /* Support continuous SPI interrupt */

pub const SDIO_BUS_ASYNC_INT: u32 = 0x20;

pub const SDIO_BUS_CD_DISABLE: u32 = 0x80; /* disable pull-up on DAT3 (pin 1) */

pub const SDIO_CCCR_CAPS: u32 = 0x08;

pub const SDIO_CCCR_CAP_SDC: u32 = 0x01; /* can do CMD52 while data transfer */
pub const SDIO_CCCR_CAP_SMB: u32 = 0x02; /* can do multi-block xfers (CMD53) */
pub const SDIO_CCCR_CAP_SRW: u32 = 0x04; /* supports read-wait protocol */
pub const SDIO_CCCR_CAP_SBS: u32 = 0x08; /* supports suspend/resume */
pub const SDIO_CCCR_CAP_S4MI: u32 = 0x10; /* interrupt during 4-bit CMD53 */
pub const SDIO_CCCR_CAP_E4MI: u32 = 0x20; /* enable ints during 4-bit CMD53 */
pub const SDIO_CCCR_CAP_LSC: u32 = 0x40; /* low speed card */
pub const SDIO_CCCR_CAP_4BLS: u32 = 0x80; /* 4 bit low speed card */

pub const SDIO_CCCR_CIS: u32 = 0x09; /* common CIS pointer (3 bytes) */

/* Following 4 regs are valid only if SBS is set */
pub const SDIO_CCCR_SUSPEND: u32 = 0x0c;
pub const SDIO_CCCR_SELx: u32 = 0x0d;
pub const SDIO_CCCR_EXECx: u32 = 0x0e;
pub const SDIO_CCCR_READYx: u32 = 0x0f;

pub const SDIO_CCCR_BLKSIZE: u32 = 0x10;

pub const SDIO_CCCR_POWER: u32 = 0x12;

pub const SDIO_POWER_SMPC: u32 = 0x01; /* Supports Master Power Control */
pub const SDIO_POWER_EMPC: u32 = 0x02; /* Enable Master Power Control */

pub const SDIO_CCCR_SPEED: u32 = 0x13;

pub const SDIO_SPEED_SHS: u32 = 0x01; /* Supports High-Speed mode */
pub const SDIO_SPEED_BSS_SHIFT: u32 = 1;
pub const SDIO_SPEED_BSS_MASK: u32 = 7 << SDIO_SPEED_BSS_SHIFT;
pub const SDIO_SPEED_SDR12: u32 = 0 << SDIO_SPEED_BSS_SHIFT;
pub const SDIO_SPEED_SDR25: u32 = 1 << SDIO_SPEED_BSS_SHIFT;
pub const SDIO_SPEED_SDR50: u32 = 2 << SDIO_SPEED_BSS_SHIFT;
pub const SDIO_SPEED_SDR104: u32 = 3 << SDIO_SPEED_BSS_SHIFT;
pub const SDIO_SPEED_DDR50: u32 = 4 << SDIO_SPEED_BSS_SHIFT;
pub const SDIO_SPEED_EHS: u32 = SDIO_SPEED_SDR25; /* Enable High-Speed */

pub const SDIO_CCCR_UHS: u32 = 0x14;
pub const SDIO_UHS_SDR50: u32 = 0x01;
pub const SDIO_UHS_SDR104: u32 = 0x02;
pub const SDIO_UHS_DDR50: u32 = 0x04;

pub const SDIO_CCCR_DRIVE_STRENGTH: u32 = 0x15;
pub const SDIO_SDTx_MASK: u32 = 0x07;
pub const SDIO_DRIVE_SDTA: u32 = 1 << 0;
pub const SDIO_DRIVE_SDTC: u32 = 1 << 1;
pub const SDIO_DRIVE_SDTD: u32 = 1 << 2;
pub const SDIO_DRIVE_DTSx_MASK: u32 = 0x03;
pub const SDIO_DRIVE_DTSx_SHIFT: u32 = 4;
pub const SDIO_DTSx_SET_TYPE_B: u32 = 0 << SDIO_DRIVE_DTSx_SHIFT;
pub const SDIO_DTSx_SET_TYPE_A: u32 = 1 << SDIO_DRIVE_DTSx_SHIFT;
pub const SDIO_DTSx_SET_TYPE_C: u32 = 2 << SDIO_DRIVE_DTSx_SHIFT;
pub const SDIO_DTSx_SET_TYPE_D: u32 = 3 << SDIO_DRIVE_DTSx_SHIFT;

pub const SDIO_CCCR_INTERRUPT_EXT: u32 = 0x16;
pub const SDIO_INTERRUPT_EXT_SAI: u32 = 1 << 0;
pub const SDIO_INTERRUPT_EXT_EAI: u32 = 1 << 1;

/*
 * Function Basic Registers (FBR)
 */

#[inline]
pub const fn SDIO_FBR_BASE(f: u32) -> u32 {
    f * 0x100 /* base of function f's FBRs */
}

pub const SDIO_FBR_STD_IF: u32 = 0x00;

pub const SDIO_FBR_SUPPORTS_CSA: u32 = 0x40; /* supports Code Storage Area */
pub const SDIO_FBR_ENABLE_CSA: u32 = 0x80; /* enable Code Storage Area */

pub const SDIO_FBR_STD_IF_EXT: u32 = 0x01;

pub const SDIO_FBR_POWER: u32 = 0x02;

pub const SDIO_FBR_POWER_SPS: u32 = 0x01; /* Supports Power Selection */
pub const SDIO_FBR_POWER_EPS: u32 = 0x02; /* Enable (low) Power Selection */

pub const SDIO_FBR_CIS: u32 = 0x09; /* CIS pointer (3 bytes) */

pub const SDIO_FBR_CSA: u32 = 0x0C; /* CSA pointer (3 bytes) */

pub const SDIO_FBR_CSA_DATA: u32 = 0x0F;

pub const SDIO_FBR_BLKSIZE: u32 = 0x10; /* block size (2 bytes) */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
