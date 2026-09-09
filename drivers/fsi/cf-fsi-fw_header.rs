/* SPDX-License-Identifier: GPL-2.0+ */

/*
 * uCode file layout
 *
 * 0000...03ff : m68k exception vectors
 * 0400...04ff : Header info & boot config block
 * 0500....... : Code & stack
 */

/*
 * Header info & boot config area
 *
 * The Header info is built into the ucode and provide version and
 * platform information.
 *
 * the Boot config needs to be adjusted by the ARM prior to starting
 * the ucode if the Command/Status area isn't at 0x320000 in CF space
 * (ie. beginning of SRAM).
 */

pub const HDR_OFFSET: u32 = 0x400;

/* Info: Signature & version */
pub const HDR_SYS_SIG: u32 = 0x00; /* 2 bytes system signature */
pub const SYS_SIG_SHARED: u32 = 0x5348;
pub const SYS_SIG_SPLIT: u32 = 0x5350;
pub const HDR_FW_VERS: u32 = 0x02; /* 2 bytes Major.Minor */
pub const HDR_API_VERS: u32 = 0x04; /* 2 bytes Major.Minor */
pub const API_VERSION_MAJ: u32 = 2; /* Current version */
pub const API_VERSION_MIN: u32 = 1;
pub const HDR_FW_OPTIONS: u32 = 0x08; /* 4 bytes option flags */
pub const FW_OPTION_TRACE_EN: u32 = 0x00000001; /* FW tracing enabled */
pub const FW_OPTION_CONT_CLOCK: u32 = 0x00000002; /* Continuous clocking supported */
pub const HDR_FW_SIZE: u32 = 0x10; /* 4 bytes size for combo image */

/* Boot Config: Address of Command/Status area */
pub const HDR_CMD_STAT_AREA: u32 = 0x80; /* 4 bytes CF address */
pub const HDR_FW_CONTROL: u32 = 0x84; /* 4 bytes control flags */
pub const FW_CONTROL_CONT_CLOCK: u32 = 0x00000002; /* Continuous clocking enabled */
pub const FW_CONTROL_DUMMY_RD: u32 = 0x00000004; /* Extra dummy read (AST2400) */
pub const FW_CONTROL_USE_STOP: u32 = 0x00000008; /* Use STOP instructions */
pub const HDR_CLOCK_GPIO_VADDR: u32 = 0x90; /* 2 bytes offset from GPIO base */
pub const HDR_CLOCK_GPIO_DADDR: u32 = 0x92; /* 2 bytes offset from GPIO base */
pub const HDR_DATA_GPIO_VADDR: u32 = 0x94; /* 2 bytes offset from GPIO base */
pub const HDR_DATA_GPIO_DADDR: u32 = 0x96; /* 2 bytes offset from GPIO base */
pub const HDR_TRANS_GPIO_VADDR: u32 = 0x98; /* 2 bytes offset from GPIO base */
pub const HDR_TRANS_GPIO_DADDR: u32 = 0x9a; /* 2 bytes offset from GPIO base */
pub const HDR_CLOCK_GPIO_BIT: u32 = 0x9c; /* 1 byte bit number */
pub const HDR_DATA_GPIO_BIT: u32 = 0x9d; /* 1 byte bit number */
pub const HDR_TRANS_GPIO_BIT: u32 = 0x9e; /* 1 byte bit number */

/* Command/Status area layout: Main part */
pub const CMD_STAT_REG: u32 = 0x00;
pub const CMD_REG_CMD_MASK: u32 = 0x000000ff;
pub const CMD_REG_CMD_SHIFT: u32 = 0;
pub const CMD_NONE: u32 = 0x00;
pub const CMD_COMMAND: u32 = 0x01;
pub const CMD_BREAK: u32 = 0x02;
pub const CMD_IDLE_CLOCKS: u32 = 0x03; /* clen = #clocks */
pub const CMD_INVALID: u32 = 0xff;
pub const CMD_REG_CLEN_MASK: u32 = 0x0000ff00;
pub const CMD_REG_CLEN_SHIFT: u32 = 8;
pub const CMD_REG_RLEN_MASK: u32 = 0x00ff0000;
pub const CMD_REG_RLEN_SHIFT: u32 = 16;
pub const CMD_REG_STAT_MASK: u32 = 0xff000000;
pub const CMD_REG_STAT_SHIFT: u32 = 24;
pub const STAT_WORKING: u32 = 0x00;
pub const STAT_COMPLETE: u32 = 0x01;
pub const STAT_ERR_INVAL_CMD: u32 = 0x80;
pub const STAT_ERR_INVAL_IRQ: u32 = 0x81;
pub const STAT_ERR_MTOE: u32 = 0x82;

/* Response tag & CRC */
pub const STAT_RTAG: u32 = 0x04;
/* Response CRC */
pub const STAT_RCRC: u32 = 0x05;
/* Echo and Send delay */
pub const ECHO_DLY_REG: u32 = 0x08;
pub const SEND_DLY_REG: u32 = 0x09;
/* Command data area
 *
 * Last byte of message must be left aligned
 */
pub const CMD_DATA: u32 = 0x10; /* 64 bit of data */
/* Response data area, right aligned, unused top bits are 1 */
pub const RSP_DATA: u32 = 0x20; /* 32 bit of data */
/* Misc */
pub const INT_CNT: u32 = 0x30; /* 32-bit interrupt count */
pub const BAD_INT_VEC: u32 = 0x34; /* 32-bit bad interrupt vector # */
pub const CF_STARTED: u32 = 0x38; /* byte, set to -1 when copro started */
pub const CLK_CNT: u32 = 0x3c; /* 32-bit, clock count (debug only) */

/* SRAM layout: GPIO arbitration part */
pub const ARB_REG: u32 = 0x40;
pub const ARB_ARM_REQ: u32 = 0x01;
pub const ARB_ARM_ACK: u32 = 0x02;
/* Misc2 */
pub const CF_RESET_D0: u32 = 0x50;
pub const CF_RESET_D1: u32 = 0x54;
pub const BAD_INT_S0: u32 = 0x58;
pub const BAD_INT_S1: u32 = 0x5c;
pub const STOP_CNT: u32 = 0x60;

/* Internal */
/* SRAM layout: Trace buffer (debug builds only) */
pub const TRACEBUF: u32 = 0x100;
pub const TR_CLKOBIT0: u32 = 0xc0;
pub const TR_CLKOBIT1: u32 = 0xc1;
pub const TR_CLKOSTART: u32 = 0x82;
pub const TR_OLEN: u32 = 0x83; /* + len */
pub const TR_CLKZ: u32 = 0x84; /* + count */
pub const TR_CLKWSTART: u32 = 0x85;
pub const TR_CLKTAG: u32 = 0x86; /* + tag */
pub const TR_CLKDATA: u32 = 0x87; /* + len */
pub const TR_CLKCRC: u32 = 0x88; /* + raw crc */
pub const TR_CLKIBIT0: u32 = 0x90;
pub const TR_CLKIBIT1: u32 = 0x91;
pub const TR_END: u32 = 0xff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
