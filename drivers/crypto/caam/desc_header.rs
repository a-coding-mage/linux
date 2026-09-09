/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CAAM descriptor composition header
 * Definitions to support CAAM descriptor instruction generation
 *
 * Copyright 2008-2011 Freescale Semiconductor, Inc.
 * Copyright 2018, 2025 NXP
 */


/*
 * 16-byte hardware scatter/gather table
 * An 8-byte table exists in the hardware spec, but has never been
 * implemented to date. The 8/16 option is selected at RTL-compile-time.
 * and this selection is visible in the Compile Time Parameters Register
 */

pub const SEC4_SG_LEN_EXT: u32 = 0x80000000	/* Entry points to table */;
pub const SEC4_SG_LEN_FIN: u32 = 0x40000000	/* Last entry in table */;
pub const SEC4_SG_BPID_MASK: u32 = 0x000000ff;
pub const SEC4_SG_BPID_SHIFT: u32 = 16;
pub const SEC4_SG_LEN_MASK: u32 = 0x3fffffff	/* Excludes EXT and FINAL */;
pub const SEC4_SG_OFFSET_MASK: u32 = 0x00001fff;

/* Max size of any CAAM descriptor in 32-bit words, inclusive of header */
pub const MAX_CAAM_DESCSIZE: u32 = 64;

/* Block size of any entity covered/uncovered with a KEK/TKEK */
pub const KEK_BLOCKSIZE: u32 = 16;

/*
 * Supported descriptor command types as they show up
 * inside a descriptor command word.
 */
pub const CMD_SHIFT: u32 = 27;
pub const CMD_MASK: u32 = 0xf8000000;

pub const CMD_KEY: u32 = (0x00 << CMD_SHIFT);
pub const CMD_SEQ_KEY: u32 = (0x01 << CMD_SHIFT);
pub const CMD_LOAD: u32 = (0x02 << CMD_SHIFT);
pub const CMD_SEQ_LOAD: u32 = (0x03 << CMD_SHIFT);
pub const CMD_FIFO_LOAD: u32 = (0x04 << CMD_SHIFT);
pub const CMD_SEQ_FIFO_LOAD: u32 = (0x05 << CMD_SHIFT);
pub const CMD_STORE: u32 = (0x0a << CMD_SHIFT);
pub const CMD_SEQ_STORE: u32 = (0x0b << CMD_SHIFT);
pub const CMD_FIFO_STORE: u32 = (0x0c << CMD_SHIFT);
pub const CMD_SEQ_FIFO_STORE: u32 = (0x0d << CMD_SHIFT);
pub const CMD_MOVE_LEN: u32 = (0x0e << CMD_SHIFT);
pub const CMD_MOVE: u32 = (0x0f << CMD_SHIFT);
pub const CMD_OPERATION: u32 = (0x10 << CMD_SHIFT);
pub const CMD_SIGNATURE: u32 = (0x12 << CMD_SHIFT);
pub const CMD_JUMP: u32 = (0x14 << CMD_SHIFT);
pub const CMD_MATH: u32 = (0x15 << CMD_SHIFT);
pub const CMD_DESC_HDR: u32 = (0x16 << CMD_SHIFT);
pub const CMD_SHARED_DESC_HDR: u32 = (0x17 << CMD_SHIFT);
pub const CMD_SEQ_IN_PTR: u32 = (0x1e << CMD_SHIFT);
pub const CMD_SEQ_OUT_PTR: u32 = (0x1f << CMD_SHIFT);

/* General-purpose class selector for all commands */
pub const CLASS_SHIFT: u32 = 25;
pub const CLASS_MASK: u32 = (0x03 << CLASS_SHIFT);

pub const CLASS_NONE: u32 = (0x00 << CLASS_SHIFT);
pub const CLASS_1: u32 = (0x01 << CLASS_SHIFT);
pub const CLASS_2: u32 = (0x02 << CLASS_SHIFT);
pub const CLASS_BOTH: u32 = (0x03 << CLASS_SHIFT);

/*
 * Descriptor header command constructs
 * Covers shared, job, and trusted descriptor headers
 */

/*
 * Do Not Run - marks a descriptor inexecutable if there was
 * a preceding error somewhere
 */
pub const HDR_DNR: u32 = 0x01000000;

/*
 * ONE - should always be set. Combination of ONE (always
 * set) and ZRO (always clear) forms an endianness sanity check
 */
pub const HDR_ONE: u32 = 0x00800000;
pub const HDR_ZRO: u32 = 0x00008000;

/* Start Index or SharedDesc Length */
pub const HDR_START_IDX_SHIFT: u32 = 16;
pub const HDR_START_IDX_MASK: u32 = (0x3f << HDR_START_IDX_SHIFT);

/* If shared descriptor header, 6-bit length */
pub const HDR_DESCLEN_SHR_MASK: u32 = 0x3f;

/* If non-shared header, 7-bit length */
pub const HDR_DESCLEN_MASK: u32 = 0x7f;

/* This is a TrustedDesc (if not SharedDesc) */
pub const HDR_TRUSTED: u32 = 0x00004000;

/* Make into TrustedDesc (if not SharedDesc) */
pub const HDR_MAKE_TRUSTED: u32 = 0x00002000;

/* Save context if self-shared (if SharedDesc) */
pub const HDR_SAVECTX: u32 = 0x00001000;

/* Next item points to SharedDesc */
pub const HDR_SHARED: u32 = 0x00001000;

/*
 * Reverse Execution Order - execute JobDesc first, then
 * execute SharedDesc (normally SharedDesc goes first).
 */
pub const HDR_REVERSE: u32 = 0x00000800;

/* Propagate DNR property to SharedDesc */
pub const HDR_PROP_DNR: u32 = 0x00000800;

/* JobDesc/SharedDesc share property */
pub const HDR_SD_SHARE_SHIFT: u32 = 8;
pub const HDR_SD_SHARE_MASK: u32 = (0x03 << HDR_SD_SHARE_SHIFT);
pub const HDR_JD_SHARE_SHIFT: u32 = 8;
pub const HDR_JD_SHARE_MASK: u32 = (0x07 << HDR_JD_SHARE_SHIFT);

pub const HDR_SHARE_NEVER: u32 = (0x00 << HDR_SD_SHARE_SHIFT);
pub const HDR_SHARE_WAIT: u32 = (0x01 << HDR_SD_SHARE_SHIFT);
pub const HDR_SHARE_SERIAL: u32 = (0x02 << HDR_SD_SHARE_SHIFT);
pub const HDR_SHARE_ALWAYS: u32 = (0x03 << HDR_SD_SHARE_SHIFT);
pub const HDR_SHARE_DEFER: u32 = (0x04 << HDR_SD_SHARE_SHIFT);

/* JobDesc/SharedDesc descriptor length */
pub const HDR_JD_LENGTH_MASK: u32 = 0x7f;
pub const HDR_SD_LENGTH_MASK: u32 = 0x3f;

/*
 * KEY/SEQ_KEY Command Constructs
 */

/* Key Destination Class: 01 = Class 1, 02 - Class 2 */
pub const KEY_DEST_CLASS_SHIFT: u32 = 25	/* use CLASS_1 or CLASS_2 */;
pub const KEY_DEST_CLASS_MASK: u32 = (0x03 << KEY_DEST_CLASS_SHIFT);

/* Scatter-Gather Table/Variable Length Field */
pub const KEY_SGF: u32 = 0x01000000;
pub const KEY_VLF: u32 = 0x01000000;

/* Immediate - Key follows command in the descriptor */
pub const KEY_IMM: u32 = 0x00800000;

/*
 * Encrypted - Key is encrypted either with the KEK, or
 * with the TDKEK if TK is set
 */
pub const KEY_ENC: u32 = 0x00400000;

/*
 * No Write Back - Do not allow key to be FIFO STOREd
 */
pub const KEY_NWB: u32 = 0x00200000;

/*
 * Enhanced Encryption of Key
 */
pub const KEY_EKT: u32 = 0x00100000;
pub const KEY_EKT_OFFSET: u32 = 20;

/*
 * Encrypted with Trusted Key
 */
pub const KEY_TK: u32 = 0x00008000;

/*
 * KDEST - Key Destination: 0 - class key register,
 * 1 - PKHA 'e', 2 - AFHA Sbox, 3 - MDHA split-key
 */
pub const KEY_DEST_SHIFT: u32 = 16;
pub const KEY_DEST_MASK: u32 = (0x03 << KEY_DEST_SHIFT);

pub const KEY_DEST_CLASS_REG: u32 = (0x00 << KEY_DEST_SHIFT);
pub const KEY_DEST_PKHA_E: u32 = (0x01 << KEY_DEST_SHIFT);
pub const KEY_DEST_AFHA_SBOX: u32 = (0x02 << KEY_DEST_SHIFT);
pub const KEY_DEST_MDHA_SPLIT: u32 = (0x03 << KEY_DEST_SHIFT);

/* Length in bytes */
pub const KEY_LENGTH_MASK: u32 = 0x000003ff;

/*
 * LOAD/SEQ_LOAD/STORE/SEQ_STORE Command Constructs
 */

/*
 * Load/Store Destination: 0 = class independent CCB,
 * 1 = class 1 CCB, 2 = class 2 CCB, 3 = DECO
 */
pub const LDST_CLASS_SHIFT: u32 = 25;
pub const LDST_CLASS_MASK: u32 = (0x03 << LDST_CLASS_SHIFT);
pub const LDST_CLASS_IND_CCB: u32 = (0x00 << LDST_CLASS_SHIFT);
pub const LDST_CLASS_1_CCB: u32 = (0x01 << LDST_CLASS_SHIFT);
pub const LDST_CLASS_2_CCB: u32 = (0x02 << LDST_CLASS_SHIFT);
pub const LDST_CLASS_DECO: u32 = (0x03 << LDST_CLASS_SHIFT);

/* Scatter-Gather Table/Variable Length Field */
pub const LDST_SGF: u32 = 0x01000000;
pub const LDST_VLF: u32 = LDST_SGF;

/* Immediate - Key follows this command in descriptor */
pub const LDST_IMM_MASK: u32 = 1;
pub const LDST_IMM_SHIFT: u32 = 23;
pub const LDST_IMM: u32 = (LDST_IMM_MASK << LDST_IMM_SHIFT);

/* SRC/DST - Destination for LOAD, Source for STORE */
pub const LDST_SRCDST_SHIFT: u32 = 16;
pub const LDST_SRCDST_MASK: u32 = (0x7f << LDST_SRCDST_SHIFT);

pub const LDST_SRCDST_BYTE_CONTEXT: u32 = (0x20 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_BYTE_KEY: u32 = (0x40 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_BYTE_INFIFO: u32 = (0x7c << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_BYTE_OUTFIFO: u32 = (0x7e << LDST_SRCDST_SHIFT);

pub const LDST_SRCDST_WORD_MODE_REG: u32 = (0x00 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_KEYSZ_REG: u32 = (0x01 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DATASZ_REG: u32 = (0x02 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_ICVSZ_REG: u32 = (0x03 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_CHACTRL: u32 = (0x06 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DECOCTRL: u32 = (0x06 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_IRQCTRL: u32 = (0x07 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DECO_PCLOVRD: u32 = (0x07 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_CLRW: u32 = (0x08 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DECO_MATH0: u32 = (0x08 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_STAT: u32 = (0x09 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DECO_MATH1: u32 = (0x09 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DECO_MATH2: u32 = (0x0a << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DECO_AAD_SZ: u32 = (0x0b << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DECO_MATH3: u32 = (0x0b << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_CLASS1_IV_SZ: u32 = (0x0c << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_ALTDS_CLASS1: u32 = (0x0f << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_PKHA_A_SZ: u32 = (0x10 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_PKHA_B_SZ: u32 = (0x11 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_PKHA_N_SZ: u32 = (0x12 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_PKHA_E_SZ: u32 = (0x13 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_CLASS_CTX: u32 = (0x20 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DESCBUF: u32 = (0x40 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DESCBUF_JOB: u32 = (0x41 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DESCBUF_SHARED: u32 = (0x42 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DESCBUF_JOB_WE: u32 = (0x45 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_DESCBUF_SHARED_WE: u32 = (0x46 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_INFO_FIFO_SM: u32 = (0x71 << LDST_SRCDST_SHIFT);
pub const LDST_SRCDST_WORD_INFO_FIFO: u32 = (0x7a << LDST_SRCDST_SHIFT);

/* Offset in source/destination */
pub const LDST_OFFSET_SHIFT: u32 = 8;
pub const LDST_OFFSET_MASK: u32 = (0xff << LDST_OFFSET_SHIFT);

/* LDOFF definitions used when DST = LDST_SRCDST_WORD_DECOCTRL */
/* These could also be shifted by LDST_OFFSET_SHIFT - this reads better */
pub const LDOFF_CHG_SHARE_SHIFT: u32 = 0;
pub const LDOFF_CHG_SHARE_MASK: u32 = (0x3 << LDOFF_CHG_SHARE_SHIFT);
pub const LDOFF_CHG_SHARE_NEVER: u32 = (0x1 << LDOFF_CHG_SHARE_SHIFT);
pub const LDOFF_CHG_SHARE_OK_PROP: u32 = (0x2 << LDOFF_CHG_SHARE_SHIFT);
pub const LDOFF_CHG_SHARE_OK_NO_PROP: u32 = (0x3 << LDOFF_CHG_SHARE_SHIFT);

pub const LDOFF_ENABLE_AUTO_NFIFO: u32 = (1 << 2);
pub const LDOFF_DISABLE_AUTO_NFIFO: u32 = (1 << 3);

pub const LDOFF_CHG_NONSEQLIODN_SHIFT: u32 = 4;
pub const LDOFF_CHG_NONSEQLIODN_MASK: u32 = (0x3 << LDOFF_CHG_NONSEQLIODN_SHIFT);
pub const LDOFF_CHG_NONSEQLIODN_SEQ: u32 = (0x1 << LDOFF_CHG_NONSEQLIODN_SHIFT);
pub const LDOFF_CHG_NONSEQLIODN_NON_SEQ: u32 = (0x2 << LDOFF_CHG_NONSEQLIODN_SHIFT);
pub const LDOFF_CHG_NONSEQLIODN_TRUSTED: u32 = (0x3 << LDOFF_CHG_NONSEQLIODN_SHIFT);

pub const LDOFF_CHG_SEQLIODN_SHIFT: u32 = 6;
pub const LDOFF_CHG_SEQLIODN_MASK: u32 = (0x3 << LDOFF_CHG_SEQLIODN_SHIFT);
pub const LDOFF_CHG_SEQLIODN_SEQ: u32 = (0x1 << LDOFF_CHG_SEQLIODN_SHIFT);
pub const LDOFF_CHG_SEQLIODN_NON_SEQ: u32 = (0x2 << LDOFF_CHG_SEQLIODN_SHIFT);
pub const LDOFF_CHG_SEQLIODN_TRUSTED: u32 = (0x3 << LDOFF_CHG_SEQLIODN_SHIFT);

/* Data length in bytes	*/
pub const LDST_LEN_SHIFT: u32 = 0;
pub const LDST_LEN_MASK: u32 = (0xff << LDST_LEN_SHIFT);

/* Special Length definitions when dst=deco-ctrl */
pub const LDLEN_ENABLE_OSL_COUNT: u32 = (1 << 7);
pub const LDLEN_RST_CHA_OFIFO_PTR: u32 = (1 << 6);
pub const LDLEN_RST_OFIFO: u32 = (1 << 5);
pub const LDLEN_SET_OFIFO_OFF_VALID: u32 = (1 << 4);
pub const LDLEN_SET_OFIFO_OFF_RSVD: u32 = (1 << 3);
pub const LDLEN_SET_OFIFO_OFFSET_SHIFT: u32 = 0;
pub const LDLEN_SET_OFIFO_OFFSET_MASK: u32 = (3 << LDLEN_SET_OFIFO_OFFSET_SHIFT);

/* Special Length definitions when dst=sm, nfifo-{sm,m} */
pub const LDLEN_MATH0: u32 = 0;
pub const LDLEN_MATH1: u32 = 1;
pub const LDLEN_MATH2: u32 = 2;
pub const LDLEN_MATH3: u32 = 3;

/*
 * FIFO_LOAD/FIFO_STORE/SEQ_FIFO_LOAD/SEQ_FIFO_STORE
 * Command Constructs
 */

/*
 * Load Destination: 0 = skip (SEQ_FIFO_LOAD only),
 * 1 = Load for Class1, 2 = Load for Class2, 3 = Load both
 * Store Source: 0 = normal, 1 = Class1key, 2 = Class2key
 */
pub const FIFOLD_CLASS_SHIFT: u32 = 25;
pub const FIFOLD_CLASS_MASK: u32 = (0x03 << FIFOLD_CLASS_SHIFT);
pub const FIFOLD_CLASS_SKIP: u32 = (0x00 << FIFOLD_CLASS_SHIFT);
pub const FIFOLD_CLASS_CLASS1: u32 = (0x01 << FIFOLD_CLASS_SHIFT);
pub const FIFOLD_CLASS_CLASS2: u32 = (0x02 << FIFOLD_CLASS_SHIFT);
pub const FIFOLD_CLASS_BOTH: u32 = (0x03 << FIFOLD_CLASS_SHIFT);

pub const FIFOST_CLASS_SHIFT: u32 = 25;
pub const FIFOST_CLASS_MASK: u32 = (0x03 << FIFOST_CLASS_SHIFT);
pub const FIFOST_CLASS_NORMAL: u32 = (0x00 << FIFOST_CLASS_SHIFT);
pub const FIFOST_CLASS_CLASS1KEY: u32 = (0x01 << FIFOST_CLASS_SHIFT);
pub const FIFOST_CLASS_CLASS2KEY: u32 = (0x02 << FIFOST_CLASS_SHIFT);

/*
 * Scatter-Gather Table/Variable Length Field
 * If set for FIFO_LOAD, refers to a SG table. Within
 * SEQ_FIFO_LOAD, is variable input sequence
 */
pub const FIFOLDST_SGF_SHIFT: u32 = 24;
pub const FIFOLDST_SGF_MASK: u32 = (1 << FIFOLDST_SGF_SHIFT);
pub const FIFOLDST_VLF_MASK: u32 = (1 << FIFOLDST_SGF_SHIFT);
pub const FIFOLDST_SGF: u32 = (1 << FIFOLDST_SGF_SHIFT);
pub const FIFOLDST_VLF: u32 = (1 << FIFOLDST_SGF_SHIFT);

/* Immediate - Data follows command in descriptor */
pub const FIFOLD_IMM_SHIFT: u32 = 23;
pub const FIFOLD_IMM_MASK: u32 = (1 << FIFOLD_IMM_SHIFT);
pub const FIFOLD_IMM: u32 = (1 << FIFOLD_IMM_SHIFT);

/* Continue - Not the last FIFO store to come */
pub const FIFOST_CONT_SHIFT: u32 = 23;
pub const FIFOST_CONT_MASK: u32 = (1 << FIFOST_CONT_SHIFT);

/*
 * Extended Length - use 32-bit extended length that
 * follows the pointer field. Illegal with IMM set
 */
pub const FIFOLDST_EXT_SHIFT: u32 = 22;
pub const FIFOLDST_EXT_MASK: u32 = (1 << FIFOLDST_EXT_SHIFT);
pub const FIFOLDST_EXT: u32 = (1 << FIFOLDST_EXT_SHIFT);

/* Input data type.*/
pub const FIFOLD_TYPE_SHIFT: u32 = 16;
pub const FIFOLD_CONT_TYPE_SHIFT: u32 = 19 /* shift past last-flush bits */;
pub const FIFOLD_TYPE_MASK: u32 = (0x3f << FIFOLD_TYPE_SHIFT);

/* PK types */
pub const FIFOLD_TYPE_PK: u32 = (0x00 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_MASK: u32 = (0x30 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_TYPEMASK: u32 = (0x0f << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_A0: u32 = (0x00 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_A1: u32 = (0x01 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_A2: u32 = (0x02 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_A3: u32 = (0x03 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_B0: u32 = (0x04 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_B1: u32 = (0x05 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_B2: u32 = (0x06 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_B3: u32 = (0x07 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_N: u32 = (0x08 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_A: u32 = (0x0c << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_PK_B: u32 = (0x0d << FIFOLD_TYPE_SHIFT);

/* Other types. Need to OR in last/flush bits as desired */
pub const FIFOLD_TYPE_MSG_MASK: u32 = (0x38 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_MSG: u32 = (0x10 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_MSG1OUT2: u32 = (0x18 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_IV: u32 = (0x20 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_BITDATA: u32 = (0x28 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_AAD: u32 = (0x30 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_ICV: u32 = (0x38 << FIFOLD_TYPE_SHIFT);

/* Last/Flush bits for use with "other" types above */
pub const FIFOLD_TYPE_ACT_MASK: u32 = (0x07 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_NOACTION: u32 = (0x00 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_FLUSH1: u32 = (0x01 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_LAST1: u32 = (0x02 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_LAST2FLUSH: u32 = (0x03 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_LAST2: u32 = (0x04 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_LAST2FLUSH1: u32 = (0x05 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_LASTBOTH: u32 = (0x06 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_LASTBOTHFL: u32 = (0x07 << FIFOLD_TYPE_SHIFT);
pub const FIFOLD_TYPE_NOINFOFIFO: u32 = (0x0F << FIFOLD_TYPE_SHIFT);

pub const FIFOLDST_LEN_MASK: u32 = 0xffff;
pub const FIFOLDST_EXT_LEN_MASK: u32 = 0xffffffff;

/* Output data types */
pub const FIFOST_TYPE_SHIFT: u32 = 16;
pub const FIFOST_TYPE_MASK: u32 = (0x3f << FIFOST_TYPE_SHIFT);

pub const FIFOST_TYPE_PKHA_A0: u32 = (0x00 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_A1: u32 = (0x01 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_A2: u32 = (0x02 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_A3: u32 = (0x03 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_B0: u32 = (0x04 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_B1: u32 = (0x05 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_B2: u32 = (0x06 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_B3: u32 = (0x07 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_N: u32 = (0x08 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_A: u32 = (0x0c << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_B: u32 = (0x0d << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_KEY_CCM_JKEK: u32 = (0x14 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_AF_SBOX_JKEK: u32 = (0x20 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_AF_SBOX_TKEK: u32 = (0x21 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_E_JKEK: u32 = (0x22 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_PKHA_E_TKEK: u32 = (0x23 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_KEY_KEK: u32 = (0x24 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_KEY_TKEK: u32 = (0x25 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_SPLIT_KEK: u32 = (0x26 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_SPLIT_TKEK: u32 = (0x27 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_OUTFIFO_KEK: u32 = (0x28 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_OUTFIFO_TKEK: u32 = (0x29 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_MESSAGE_DATA: u32 = (0x30 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_RNGSTORE: u32 = (0x34 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_RNGFIFO: u32 = (0x35 << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_METADATA: u32 = (0x3e << FIFOST_TYPE_SHIFT);
pub const FIFOST_TYPE_SKIP: u32 = (0x3f << FIFOST_TYPE_SHIFT);

/*
 * OPERATION Command Constructs
 */

/* Operation type selectors - OP TYPE */
pub const OP_TYPE_SHIFT: u32 = 24;
pub const OP_TYPE_MASK: u32 = (0x07 << OP_TYPE_SHIFT);

pub const OP_TYPE_UNI_PROTOCOL: u32 = (0x00 << OP_TYPE_SHIFT);
pub const OP_TYPE_PK: u32 = (0x01 << OP_TYPE_SHIFT);
pub const OP_TYPE_CLASS1_ALG: u32 = (0x02 << OP_TYPE_SHIFT);
pub const OP_TYPE_CLASS2_ALG: u32 = (0x04 << OP_TYPE_SHIFT);
pub const OP_TYPE_DECAP_PROTOCOL: u32 = (0x06 << OP_TYPE_SHIFT);
pub const OP_TYPE_ENCAP_PROTOCOL: u32 = (0x07 << OP_TYPE_SHIFT);

/* ProtocolID selectors - PROTID */
pub const OP_PCLID_SHIFT: u32 = 16;
pub const OP_PCLID_MASK: u32 = (0xff << 16);

/* Assuming OP_TYPE = OP_TYPE_UNI_PROTOCOL */
pub const OP_PCLID_IKEV1_PRF: u32 = (0x01 << OP_PCLID_SHIFT);
pub const OP_PCLID_IKEV2_PRF: u32 = (0x02 << OP_PCLID_SHIFT);
pub const OP_PCLID_SSL30_PRF: u32 = (0x08 << OP_PCLID_SHIFT);
pub const OP_PCLID_TLS10_PRF: u32 = (0x09 << OP_PCLID_SHIFT);
pub const OP_PCLID_TLS11_PRF: u32 = (0x0a << OP_PCLID_SHIFT);
pub const OP_PCLID_DTLS10_PRF: u32 = (0x0c << OP_PCLID_SHIFT);
pub const OP_PCLID_PRF: u32 = (0x06 << OP_PCLID_SHIFT);
pub const OP_PCLID_BLOB: u32 = (0x0d << OP_PCLID_SHIFT);
pub const OP_PCLID_SECRETKEY: u32 = (0x11 << OP_PCLID_SHIFT);
pub const OP_PCLID_PUBLICKEYPAIR: u32 = (0x14 << OP_PCLID_SHIFT);
pub const OP_PCLID_DSASIGN: u32 = (0x15 << OP_PCLID_SHIFT);
pub const OP_PCLID_DSAVERIFY: u32 = (0x16 << OP_PCLID_SHIFT);
pub const OP_PCLID_RSAENC_PUBKEY: u32 = (0x18 << OP_PCLID_SHIFT);
pub const OP_PCLID_RSADEC_PRVKEY: u32 = (0x19 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_MD5: u32 = (0x20 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_SHA1: u32 = (0x21 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_SHA224: u32 = (0x22 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_SHA256: u32 = (0x23 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_SHA384: u32 = (0x24 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_SHA512: u32 = (0x25 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_RIF_MD5: u32 = (0x60 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_RIF_SHA1: u32 = (0x61 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_RIF_SHA224: u32 = (0x62 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_RIF_SHA256: u32 = (0x63 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_RIF_SHA384: u32 = (0x64 << OP_PCLID_SHIFT);
pub const OP_PCLID_DKP_RIF_SHA512: u32 = (0x65 << OP_PCLID_SHIFT);

/* Assuming OP_TYPE = OP_TYPE_DECAP_PROTOCOL/ENCAP_PROTOCOL */
pub const OP_PCLID_IPSEC: u32 = (0x01 << OP_PCLID_SHIFT);
pub const OP_PCLID_SRTP: u32 = (0x02 << OP_PCLID_SHIFT);
pub const OP_PCLID_MACSEC: u32 = (0x03 << OP_PCLID_SHIFT);
pub const OP_PCLID_WIFI: u32 = (0x04 << OP_PCLID_SHIFT);
pub const OP_PCLID_WIMAX: u32 = (0x05 << OP_PCLID_SHIFT);
pub const OP_PCLID_SSL30: u32 = (0x08 << OP_PCLID_SHIFT);
pub const OP_PCLID_TLS10: u32 = (0x09 << OP_PCLID_SHIFT);
pub const OP_PCLID_TLS11: u32 = (0x0a << OP_PCLID_SHIFT);
pub const OP_PCLID_TLS12: u32 = (0x0b << OP_PCLID_SHIFT);
pub const OP_PCLID_DTLS: u32 = (0x0c << OP_PCLID_SHIFT);

/*
 * ProtocolInfo selectors
 */
pub const OP_PCLINFO_MASK: u32 = 0xffff;

/* for OP_PCLID_IPSEC */
pub const OP_PCL_IPSEC_CIPHER_MASK: u32 = 0xff00;
pub const OP_PCL_IPSEC_AUTH_MASK: u32 = 0x00ff;

pub const OP_PCL_IPSEC_DES_IV64: u32 = 0x0100;
pub const OP_PCL_IPSEC_DES: u32 = 0x0200;
pub const OP_PCL_IPSEC_3DES: u32 = 0x0300;
pub const OP_PCL_IPSEC_AES_CBC: u32 = 0x0c00;
pub const OP_PCL_IPSEC_AES_CTR: u32 = 0x0d00;
pub const OP_PCL_IPSEC_AES_XTS: u32 = 0x1600;
pub const OP_PCL_IPSEC_AES_CCM8: u32 = 0x0e00;
pub const OP_PCL_IPSEC_AES_CCM12: u32 = 0x0f00;
pub const OP_PCL_IPSEC_AES_CCM16: u32 = 0x1000;
pub const OP_PCL_IPSEC_AES_GCM8: u32 = 0x1200;
pub const OP_PCL_IPSEC_AES_GCM12: u32 = 0x1300;
pub const OP_PCL_IPSEC_AES_GCM16: u32 = 0x1400;

pub const OP_PCL_IPSEC_HMAC_NULL: u32 = 0x0000;
pub const OP_PCL_IPSEC_HMAC_MD5_96: u32 = 0x0001;
pub const OP_PCL_IPSEC_HMAC_SHA1_96: u32 = 0x0002;
pub const OP_PCL_IPSEC_AES_XCBC_MAC_96: u32 = 0x0005;
pub const OP_PCL_IPSEC_HMAC_MD5_128: u32 = 0x0006;
pub const OP_PCL_IPSEC_HMAC_SHA1_160: u32 = 0x0007;
pub const OP_PCL_IPSEC_HMAC_SHA2_256_128: u32 = 0x000c;
pub const OP_PCL_IPSEC_HMAC_SHA2_384_192: u32 = 0x000d;
pub const OP_PCL_IPSEC_HMAC_SHA2_512_256: u32 = 0x000e;

/* For SRTP - OP_PCLID_SRTP */
pub const OP_PCL_SRTP_CIPHER_MASK: u32 = 0xff00;
pub const OP_PCL_SRTP_AUTH_MASK: u32 = 0x00ff;

pub const OP_PCL_SRTP_AES_CTR: u32 = 0x0d00;

pub const OP_PCL_SRTP_HMAC_SHA1_160: u32 = 0x0007;

/* For SSL 3.0 - OP_PCLID_SSL30 */
pub const OP_PCL_SSL30_AES_128_CBC_SHA: u32 = 0x002f;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_2: u32 = 0x0030;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_3: u32 = 0x0031;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_4: u32 = 0x0032;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_5: u32 = 0x0033;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_6: u32 = 0x0034;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_7: u32 = 0x008c;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_8: u32 = 0x0090;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_9: u32 = 0x0094;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_10: u32 = 0xc004;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_11: u32 = 0xc009;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_12: u32 = 0xc00e;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_13: u32 = 0xc013;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_14: u32 = 0xc018;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_15: u32 = 0xc01d;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_16: u32 = 0xc01e;
pub const OP_PCL_SSL30_AES_128_CBC_SHA_17: u32 = 0xc01f;

pub const OP_PCL_SSL30_AES_256_CBC_SHA: u32 = 0x0035;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_2: u32 = 0x0036;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_3: u32 = 0x0037;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_4: u32 = 0x0038;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_5: u32 = 0x0039;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_6: u32 = 0x003a;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_7: u32 = 0x008d;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_8: u32 = 0x0091;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_9: u32 = 0x0095;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_10: u32 = 0xc005;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_11: u32 = 0xc00a;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_12: u32 = 0xc00f;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_13: u32 = 0xc014;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_14: u32 = 0xc019;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_15: u32 = 0xc020;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_16: u32 = 0xc021;
pub const OP_PCL_SSL30_AES_256_CBC_SHA_17: u32 = 0xc022;

pub const OP_PCL_SSL30_3DES_EDE_CBC_MD5: u32 = 0x0023;

pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA: u32 = 0x001f;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_2: u32 = 0x008b;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_3: u32 = 0x008f;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_4: u32 = 0x0093;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_5: u32 = 0x000a;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_6: u32 = 0x000d;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_7: u32 = 0x0010;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_8: u32 = 0x0013;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_9: u32 = 0x0016;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_10: u32 = 0x001b;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_11: u32 = 0xc003;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_12: u32 = 0xc008;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_13: u32 = 0xc00d;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_14: u32 = 0xc012;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_15: u32 = 0xc017;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_16: u32 = 0xc01a;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_17: u32 = 0xc01b;
pub const OP_PCL_SSL30_3DES_EDE_CBC_SHA_18: u32 = 0xc01c;

pub const OP_PCL_SSL30_DES40_CBC_MD5: u32 = 0x0029;

pub const OP_PCL_SSL30_DES_CBC_MD5: u32 = 0x0022;

pub const OP_PCL_SSL30_DES40_CBC_SHA: u32 = 0x0008;
pub const OP_PCL_SSL30_DES40_CBC_SHA_2: u32 = 0x000b;
pub const OP_PCL_SSL30_DES40_CBC_SHA_3: u32 = 0x000e;
pub const OP_PCL_SSL30_DES40_CBC_SHA_4: u32 = 0x0011;
pub const OP_PCL_SSL30_DES40_CBC_SHA_5: u32 = 0x0014;
pub const OP_PCL_SSL30_DES40_CBC_SHA_6: u32 = 0x0019;
pub const OP_PCL_SSL30_DES40_CBC_SHA_7: u32 = 0x0026;

pub const OP_PCL_SSL30_DES_CBC_SHA: u32 = 0x001e;
pub const OP_PCL_SSL30_DES_CBC_SHA_2: u32 = 0x0009;
pub const OP_PCL_SSL30_DES_CBC_SHA_3: u32 = 0x000c;
pub const OP_PCL_SSL30_DES_CBC_SHA_4: u32 = 0x000f;
pub const OP_PCL_SSL30_DES_CBC_SHA_5: u32 = 0x0012;
pub const OP_PCL_SSL30_DES_CBC_SHA_6: u32 = 0x0015;
pub const OP_PCL_SSL30_DES_CBC_SHA_7: u32 = 0x001a;

pub const OP_PCL_SSL30_RC4_128_MD5: u32 = 0x0024;
pub const OP_PCL_SSL30_RC4_128_MD5_2: u32 = 0x0004;
pub const OP_PCL_SSL30_RC4_128_MD5_3: u32 = 0x0018;

pub const OP_PCL_SSL30_RC4_40_MD5: u32 = 0x002b;
pub const OP_PCL_SSL30_RC4_40_MD5_2: u32 = 0x0003;
pub const OP_PCL_SSL30_RC4_40_MD5_3: u32 = 0x0017;

pub const OP_PCL_SSL30_RC4_128_SHA: u32 = 0x0020;
pub const OP_PCL_SSL30_RC4_128_SHA_2: u32 = 0x008a;
pub const OP_PCL_SSL30_RC4_128_SHA_3: u32 = 0x008e;
pub const OP_PCL_SSL30_RC4_128_SHA_4: u32 = 0x0092;
pub const OP_PCL_SSL30_RC4_128_SHA_5: u32 = 0x0005;
pub const OP_PCL_SSL30_RC4_128_SHA_6: u32 = 0xc002;
pub const OP_PCL_SSL30_RC4_128_SHA_7: u32 = 0xc007;
pub const OP_PCL_SSL30_RC4_128_SHA_8: u32 = 0xc00c;
pub const OP_PCL_SSL30_RC4_128_SHA_9: u32 = 0xc011;
pub const OP_PCL_SSL30_RC4_128_SHA_10: u32 = 0xc016;

pub const OP_PCL_SSL30_RC4_40_SHA: u32 = 0x0028;


/* For TLS 1.0 - OP_PCLID_TLS10 */
pub const OP_PCL_TLS10_AES_128_CBC_SHA: u32 = 0x002f;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_2: u32 = 0x0030;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_3: u32 = 0x0031;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_4: u32 = 0x0032;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_5: u32 = 0x0033;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_6: u32 = 0x0034;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_7: u32 = 0x008c;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_8: u32 = 0x0090;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_9: u32 = 0x0094;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_10: u32 = 0xc004;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_11: u32 = 0xc009;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_12: u32 = 0xc00e;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_13: u32 = 0xc013;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_14: u32 = 0xc018;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_15: u32 = 0xc01d;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_16: u32 = 0xc01e;
pub const OP_PCL_TLS10_AES_128_CBC_SHA_17: u32 = 0xc01f;

pub const OP_PCL_TLS10_AES_256_CBC_SHA: u32 = 0x0035;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_2: u32 = 0x0036;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_3: u32 = 0x0037;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_4: u32 = 0x0038;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_5: u32 = 0x0039;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_6: u32 = 0x003a;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_7: u32 = 0x008d;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_8: u32 = 0x0091;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_9: u32 = 0x0095;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_10: u32 = 0xc005;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_11: u32 = 0xc00a;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_12: u32 = 0xc00f;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_13: u32 = 0xc014;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_14: u32 = 0xc019;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_15: u32 = 0xc020;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_16: u32 = 0xc021;
pub const OP_PCL_TLS10_AES_256_CBC_SHA_17: u32 = 0xc022;

/* #define OP_PCL_TLS10_3DES_EDE_CBC_MD5	0x0023 */

pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA: u32 = 0x001f;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_2: u32 = 0x008b;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_3: u32 = 0x008f;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_4: u32 = 0x0093;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_5: u32 = 0x000a;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_6: u32 = 0x000d;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_7: u32 = 0x0010;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_8: u32 = 0x0013;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_9: u32 = 0x0016;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_10: u32 = 0x001b;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_11: u32 = 0xc003;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_12: u32 = 0xc008;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_13: u32 = 0xc00d;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_14: u32 = 0xc012;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_15: u32 = 0xc017;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_16: u32 = 0xc01a;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_17: u32 = 0xc01b;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA_18: u32 = 0xc01c;

pub const OP_PCL_TLS10_DES40_CBC_MD5: u32 = 0x0029;

pub const OP_PCL_TLS10_DES_CBC_MD5: u32 = 0x0022;

pub const OP_PCL_TLS10_DES40_CBC_SHA: u32 = 0x0008;
pub const OP_PCL_TLS10_DES40_CBC_SHA_2: u32 = 0x000b;
pub const OP_PCL_TLS10_DES40_CBC_SHA_3: u32 = 0x000e;
pub const OP_PCL_TLS10_DES40_CBC_SHA_4: u32 = 0x0011;
pub const OP_PCL_TLS10_DES40_CBC_SHA_5: u32 = 0x0014;
pub const OP_PCL_TLS10_DES40_CBC_SHA_6: u32 = 0x0019;
pub const OP_PCL_TLS10_DES40_CBC_SHA_7: u32 = 0x0026;


pub const OP_PCL_TLS10_DES_CBC_SHA: u32 = 0x001e;
pub const OP_PCL_TLS10_DES_CBC_SHA_2: u32 = 0x0009;
pub const OP_PCL_TLS10_DES_CBC_SHA_3: u32 = 0x000c;
pub const OP_PCL_TLS10_DES_CBC_SHA_4: u32 = 0x000f;
pub const OP_PCL_TLS10_DES_CBC_SHA_5: u32 = 0x0012;
pub const OP_PCL_TLS10_DES_CBC_SHA_6: u32 = 0x0015;
pub const OP_PCL_TLS10_DES_CBC_SHA_7: u32 = 0x001a;

pub const OP_PCL_TLS10_RC4_128_MD5: u32 = 0x0024;
pub const OP_PCL_TLS10_RC4_128_MD5_2: u32 = 0x0004;
pub const OP_PCL_TLS10_RC4_128_MD5_3: u32 = 0x0018;

pub const OP_PCL_TLS10_RC4_40_MD5: u32 = 0x002b;
pub const OP_PCL_TLS10_RC4_40_MD5_2: u32 = 0x0003;
pub const OP_PCL_TLS10_RC4_40_MD5_3: u32 = 0x0017;

pub const OP_PCL_TLS10_RC4_128_SHA: u32 = 0x0020;
pub const OP_PCL_TLS10_RC4_128_SHA_2: u32 = 0x008a;
pub const OP_PCL_TLS10_RC4_128_SHA_3: u32 = 0x008e;
pub const OP_PCL_TLS10_RC4_128_SHA_4: u32 = 0x0092;
pub const OP_PCL_TLS10_RC4_128_SHA_5: u32 = 0x0005;
pub const OP_PCL_TLS10_RC4_128_SHA_6: u32 = 0xc002;
pub const OP_PCL_TLS10_RC4_128_SHA_7: u32 = 0xc007;
pub const OP_PCL_TLS10_RC4_128_SHA_8: u32 = 0xc00c;
pub const OP_PCL_TLS10_RC4_128_SHA_9: u32 = 0xc011;
pub const OP_PCL_TLS10_RC4_128_SHA_10: u32 = 0xc016;

pub const OP_PCL_TLS10_RC4_40_SHA: u32 = 0x0028;

pub const OP_PCL_TLS10_3DES_EDE_CBC_MD5: u32 = 0xff23;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA160: u32 = 0xff30;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA224: u32 = 0xff34;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA256: u32 = 0xff36;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA384: u32 = 0xff33;
pub const OP_PCL_TLS10_3DES_EDE_CBC_SHA512: u32 = 0xff35;
pub const OP_PCL_TLS10_AES_128_CBC_SHA160: u32 = 0xff80;
pub const OP_PCL_TLS10_AES_128_CBC_SHA224: u32 = 0xff84;
pub const OP_PCL_TLS10_AES_128_CBC_SHA256: u32 = 0xff86;
pub const OP_PCL_TLS10_AES_128_CBC_SHA384: u32 = 0xff83;
pub const OP_PCL_TLS10_AES_128_CBC_SHA512: u32 = 0xff85;
pub const OP_PCL_TLS10_AES_192_CBC_SHA160: u32 = 0xff20;
pub const OP_PCL_TLS10_AES_192_CBC_SHA224: u32 = 0xff24;
pub const OP_PCL_TLS10_AES_192_CBC_SHA256: u32 = 0xff26;
pub const OP_PCL_TLS10_AES_192_CBC_SHA384: u32 = 0xff23;
pub const OP_PCL_TLS10_AES_192_CBC_SHA512: u32 = 0xff25;
pub const OP_PCL_TLS10_AES_256_CBC_SHA160: u32 = 0xff60;
pub const OP_PCL_TLS10_AES_256_CBC_SHA224: u32 = 0xff64;
pub const OP_PCL_TLS10_AES_256_CBC_SHA256: u32 = 0xff66;
pub const OP_PCL_TLS10_AES_256_CBC_SHA384: u32 = 0xff63;
pub const OP_PCL_TLS10_AES_256_CBC_SHA512: u32 = 0xff65;



/* For TLS 1.1 - OP_PCLID_TLS11 */
pub const OP_PCL_TLS11_AES_128_CBC_SHA: u32 = 0x002f;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_2: u32 = 0x0030;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_3: u32 = 0x0031;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_4: u32 = 0x0032;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_5: u32 = 0x0033;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_6: u32 = 0x0034;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_7: u32 = 0x008c;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_8: u32 = 0x0090;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_9: u32 = 0x0094;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_10: u32 = 0xc004;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_11: u32 = 0xc009;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_12: u32 = 0xc00e;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_13: u32 = 0xc013;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_14: u32 = 0xc018;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_15: u32 = 0xc01d;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_16: u32 = 0xc01e;
pub const OP_PCL_TLS11_AES_128_CBC_SHA_17: u32 = 0xc01f;

pub const OP_PCL_TLS11_AES_256_CBC_SHA: u32 = 0x0035;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_2: u32 = 0x0036;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_3: u32 = 0x0037;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_4: u32 = 0x0038;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_5: u32 = 0x0039;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_6: u32 = 0x003a;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_7: u32 = 0x008d;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_8: u32 = 0x0091;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_9: u32 = 0x0095;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_10: u32 = 0xc005;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_11: u32 = 0xc00a;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_12: u32 = 0xc00f;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_13: u32 = 0xc014;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_14: u32 = 0xc019;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_15: u32 = 0xc020;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_16: u32 = 0xc021;
pub const OP_PCL_TLS11_AES_256_CBC_SHA_17: u32 = 0xc022;

/* #define OP_PCL_TLS11_3DES_EDE_CBC_MD5	0x0023 */

pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA: u32 = 0x001f;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_2: u32 = 0x008b;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_3: u32 = 0x008f;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_4: u32 = 0x0093;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_5: u32 = 0x000a;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_6: u32 = 0x000d;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_7: u32 = 0x0010;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_8: u32 = 0x0013;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_9: u32 = 0x0016;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_10: u32 = 0x001b;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_11: u32 = 0xc003;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_12: u32 = 0xc008;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_13: u32 = 0xc00d;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_14: u32 = 0xc012;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_15: u32 = 0xc017;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_16: u32 = 0xc01a;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_17: u32 = 0xc01b;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA_18: u32 = 0xc01c;

pub const OP_PCL_TLS11_DES40_CBC_MD5: u32 = 0x0029;

pub const OP_PCL_TLS11_DES_CBC_MD5: u32 = 0x0022;

pub const OP_PCL_TLS11_DES40_CBC_SHA: u32 = 0x0008;
pub const OP_PCL_TLS11_DES40_CBC_SHA_2: u32 = 0x000b;
pub const OP_PCL_TLS11_DES40_CBC_SHA_3: u32 = 0x000e;
pub const OP_PCL_TLS11_DES40_CBC_SHA_4: u32 = 0x0011;
pub const OP_PCL_TLS11_DES40_CBC_SHA_5: u32 = 0x0014;
pub const OP_PCL_TLS11_DES40_CBC_SHA_6: u32 = 0x0019;
pub const OP_PCL_TLS11_DES40_CBC_SHA_7: u32 = 0x0026;

pub const OP_PCL_TLS11_DES_CBC_SHA: u32 = 0x001e;
pub const OP_PCL_TLS11_DES_CBC_SHA_2: u32 = 0x0009;
pub const OP_PCL_TLS11_DES_CBC_SHA_3: u32 = 0x000c;
pub const OP_PCL_TLS11_DES_CBC_SHA_4: u32 = 0x000f;
pub const OP_PCL_TLS11_DES_CBC_SHA_5: u32 = 0x0012;
pub const OP_PCL_TLS11_DES_CBC_SHA_6: u32 = 0x0015;
pub const OP_PCL_TLS11_DES_CBC_SHA_7: u32 = 0x001a;

pub const OP_PCL_TLS11_RC4_128_MD5: u32 = 0x0024;
pub const OP_PCL_TLS11_RC4_128_MD5_2: u32 = 0x0004;
pub const OP_PCL_TLS11_RC4_128_MD5_3: u32 = 0x0018;

pub const OP_PCL_TLS11_RC4_40_MD5: u32 = 0x002b;
pub const OP_PCL_TLS11_RC4_40_MD5_2: u32 = 0x0003;
pub const OP_PCL_TLS11_RC4_40_MD5_3: u32 = 0x0017;

pub const OP_PCL_TLS11_RC4_128_SHA: u32 = 0x0020;
pub const OP_PCL_TLS11_RC4_128_SHA_2: u32 = 0x008a;
pub const OP_PCL_TLS11_RC4_128_SHA_3: u32 = 0x008e;
pub const OP_PCL_TLS11_RC4_128_SHA_4: u32 = 0x0092;
pub const OP_PCL_TLS11_RC4_128_SHA_5: u32 = 0x0005;
pub const OP_PCL_TLS11_RC4_128_SHA_6: u32 = 0xc002;
pub const OP_PCL_TLS11_RC4_128_SHA_7: u32 = 0xc007;
pub const OP_PCL_TLS11_RC4_128_SHA_8: u32 = 0xc00c;
pub const OP_PCL_TLS11_RC4_128_SHA_9: u32 = 0xc011;
pub const OP_PCL_TLS11_RC4_128_SHA_10: u32 = 0xc016;

pub const OP_PCL_TLS11_RC4_40_SHA: u32 = 0x0028;

pub const OP_PCL_TLS11_3DES_EDE_CBC_MD5: u32 = 0xff23;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA160: u32 = 0xff30;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA224: u32 = 0xff34;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA256: u32 = 0xff36;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA384: u32 = 0xff33;
pub const OP_PCL_TLS11_3DES_EDE_CBC_SHA512: u32 = 0xff35;
pub const OP_PCL_TLS11_AES_128_CBC_SHA160: u32 = 0xff80;
pub const OP_PCL_TLS11_AES_128_CBC_SHA224: u32 = 0xff84;
pub const OP_PCL_TLS11_AES_128_CBC_SHA256: u32 = 0xff86;
pub const OP_PCL_TLS11_AES_128_CBC_SHA384: u32 = 0xff83;
pub const OP_PCL_TLS11_AES_128_CBC_SHA512: u32 = 0xff85;
pub const OP_PCL_TLS11_AES_192_CBC_SHA160: u32 = 0xff20;
pub const OP_PCL_TLS11_AES_192_CBC_SHA224: u32 = 0xff24;
pub const OP_PCL_TLS11_AES_192_CBC_SHA256: u32 = 0xff26;
pub const OP_PCL_TLS11_AES_192_CBC_SHA384: u32 = 0xff23;
pub const OP_PCL_TLS11_AES_192_CBC_SHA512: u32 = 0xff25;
pub const OP_PCL_TLS11_AES_256_CBC_SHA160: u32 = 0xff60;
pub const OP_PCL_TLS11_AES_256_CBC_SHA224: u32 = 0xff64;
pub const OP_PCL_TLS11_AES_256_CBC_SHA256: u32 = 0xff66;
pub const OP_PCL_TLS11_AES_256_CBC_SHA384: u32 = 0xff63;
pub const OP_PCL_TLS11_AES_256_CBC_SHA512: u32 = 0xff65;


/* For TLS 1.2 - OP_PCLID_TLS12 */
pub const OP_PCL_TLS12_AES_128_CBC_SHA: u32 = 0x002f;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_2: u32 = 0x0030;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_3: u32 = 0x0031;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_4: u32 = 0x0032;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_5: u32 = 0x0033;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_6: u32 = 0x0034;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_7: u32 = 0x008c;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_8: u32 = 0x0090;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_9: u32 = 0x0094;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_10: u32 = 0xc004;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_11: u32 = 0xc009;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_12: u32 = 0xc00e;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_13: u32 = 0xc013;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_14: u32 = 0xc018;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_15: u32 = 0xc01d;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_16: u32 = 0xc01e;
pub const OP_PCL_TLS12_AES_128_CBC_SHA_17: u32 = 0xc01f;

pub const OP_PCL_TLS12_AES_256_CBC_SHA: u32 = 0x0035;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_2: u32 = 0x0036;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_3: u32 = 0x0037;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_4: u32 = 0x0038;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_5: u32 = 0x0039;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_6: u32 = 0x003a;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_7: u32 = 0x008d;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_8: u32 = 0x0091;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_9: u32 = 0x0095;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_10: u32 = 0xc005;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_11: u32 = 0xc00a;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_12: u32 = 0xc00f;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_13: u32 = 0xc014;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_14: u32 = 0xc019;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_15: u32 = 0xc020;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_16: u32 = 0xc021;
pub const OP_PCL_TLS12_AES_256_CBC_SHA_17: u32 = 0xc022;

/* #define OP_PCL_TLS12_3DES_EDE_CBC_MD5	0x0023 */

pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA: u32 = 0x001f;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_2: u32 = 0x008b;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_3: u32 = 0x008f;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_4: u32 = 0x0093;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_5: u32 = 0x000a;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_6: u32 = 0x000d;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_7: u32 = 0x0010;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_8: u32 = 0x0013;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_9: u32 = 0x0016;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_10: u32 = 0x001b;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_11: u32 = 0xc003;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_12: u32 = 0xc008;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_13: u32 = 0xc00d;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_14: u32 = 0xc012;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_15: u32 = 0xc017;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_16: u32 = 0xc01a;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_17: u32 = 0xc01b;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA_18: u32 = 0xc01c;

pub const OP_PCL_TLS12_DES40_CBC_MD5: u32 = 0x0029;

pub const OP_PCL_TLS12_DES_CBC_MD5: u32 = 0x0022;

pub const OP_PCL_TLS12_DES40_CBC_SHA: u32 = 0x0008;
pub const OP_PCL_TLS12_DES40_CBC_SHA_2: u32 = 0x000b;
pub const OP_PCL_TLS12_DES40_CBC_SHA_3: u32 = 0x000e;
pub const OP_PCL_TLS12_DES40_CBC_SHA_4: u32 = 0x0011;
pub const OP_PCL_TLS12_DES40_CBC_SHA_5: u32 = 0x0014;
pub const OP_PCL_TLS12_DES40_CBC_SHA_6: u32 = 0x0019;
pub const OP_PCL_TLS12_DES40_CBC_SHA_7: u32 = 0x0026;

pub const OP_PCL_TLS12_DES_CBC_SHA: u32 = 0x001e;
pub const OP_PCL_TLS12_DES_CBC_SHA_2: u32 = 0x0009;
pub const OP_PCL_TLS12_DES_CBC_SHA_3: u32 = 0x000c;
pub const OP_PCL_TLS12_DES_CBC_SHA_4: u32 = 0x000f;
pub const OP_PCL_TLS12_DES_CBC_SHA_5: u32 = 0x0012;
pub const OP_PCL_TLS12_DES_CBC_SHA_6: u32 = 0x0015;
pub const OP_PCL_TLS12_DES_CBC_SHA_7: u32 = 0x001a;

pub const OP_PCL_TLS12_RC4_128_MD5: u32 = 0x0024;
pub const OP_PCL_TLS12_RC4_128_MD5_2: u32 = 0x0004;
pub const OP_PCL_TLS12_RC4_128_MD5_3: u32 = 0x0018;

pub const OP_PCL_TLS12_RC4_40_MD5: u32 = 0x002b;
pub const OP_PCL_TLS12_RC4_40_MD5_2: u32 = 0x0003;
pub const OP_PCL_TLS12_RC4_40_MD5_3: u32 = 0x0017;

pub const OP_PCL_TLS12_RC4_128_SHA: u32 = 0x0020;
pub const OP_PCL_TLS12_RC4_128_SHA_2: u32 = 0x008a;
pub const OP_PCL_TLS12_RC4_128_SHA_3: u32 = 0x008e;
pub const OP_PCL_TLS12_RC4_128_SHA_4: u32 = 0x0092;
pub const OP_PCL_TLS12_RC4_128_SHA_5: u32 = 0x0005;
pub const OP_PCL_TLS12_RC4_128_SHA_6: u32 = 0xc002;
pub const OP_PCL_TLS12_RC4_128_SHA_7: u32 = 0xc007;
pub const OP_PCL_TLS12_RC4_128_SHA_8: u32 = 0xc00c;
pub const OP_PCL_TLS12_RC4_128_SHA_9: u32 = 0xc011;
pub const OP_PCL_TLS12_RC4_128_SHA_10: u32 = 0xc016;

pub const OP_PCL_TLS12_RC4_40_SHA: u32 = 0x0028;

/* #define OP_PCL_TLS12_AES_128_CBC_SHA256	0x003c */
pub const OP_PCL_TLS12_AES_128_CBC_SHA256_2: u32 = 0x003e;
pub const OP_PCL_TLS12_AES_128_CBC_SHA256_3: u32 = 0x003f;
pub const OP_PCL_TLS12_AES_128_CBC_SHA256_4: u32 = 0x0040;
pub const OP_PCL_TLS12_AES_128_CBC_SHA256_5: u32 = 0x0067;
pub const OP_PCL_TLS12_AES_128_CBC_SHA256_6: u32 = 0x006c;

/* #define OP_PCL_TLS12_AES_256_CBC_SHA256	0x003d */
pub const OP_PCL_TLS12_AES_256_CBC_SHA256_2: u32 = 0x0068;
pub const OP_PCL_TLS12_AES_256_CBC_SHA256_3: u32 = 0x0069;
pub const OP_PCL_TLS12_AES_256_CBC_SHA256_4: u32 = 0x006a;
pub const OP_PCL_TLS12_AES_256_CBC_SHA256_5: u32 = 0x006b;
pub const OP_PCL_TLS12_AES_256_CBC_SHA256_6: u32 = 0x006d;

/* AEAD_AES_xxx_CCM/GCM remain to be defined... */

pub const OP_PCL_TLS12_3DES_EDE_CBC_MD5: u32 = 0xff23;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA160: u32 = 0xff30;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA224: u32 = 0xff34;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA256: u32 = 0xff36;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA384: u32 = 0xff33;
pub const OP_PCL_TLS12_3DES_EDE_CBC_SHA512: u32 = 0xff35;
pub const OP_PCL_TLS12_AES_128_CBC_SHA160: u32 = 0xff80;
pub const OP_PCL_TLS12_AES_128_CBC_SHA224: u32 = 0xff84;
pub const OP_PCL_TLS12_AES_128_CBC_SHA256: u32 = 0xff86;
pub const OP_PCL_TLS12_AES_128_CBC_SHA384: u32 = 0xff83;
pub const OP_PCL_TLS12_AES_128_CBC_SHA512: u32 = 0xff85;
pub const OP_PCL_TLS12_AES_192_CBC_SHA160: u32 = 0xff20;
pub const OP_PCL_TLS12_AES_192_CBC_SHA224: u32 = 0xff24;
pub const OP_PCL_TLS12_AES_192_CBC_SHA256: u32 = 0xff26;
pub const OP_PCL_TLS12_AES_192_CBC_SHA384: u32 = 0xff23;
pub const OP_PCL_TLS12_AES_192_CBC_SHA512: u32 = 0xff25;
pub const OP_PCL_TLS12_AES_256_CBC_SHA160: u32 = 0xff60;
pub const OP_PCL_TLS12_AES_256_CBC_SHA224: u32 = 0xff64;
pub const OP_PCL_TLS12_AES_256_CBC_SHA256: u32 = 0xff66;
pub const OP_PCL_TLS12_AES_256_CBC_SHA384: u32 = 0xff63;
pub const OP_PCL_TLS12_AES_256_CBC_SHA512: u32 = 0xff65;

/* Blob protocol protinfo bits */

pub const OP_PCL_BLOB_BLACK: u32 = 0x0004;
pub const OP_PCL_BLOB_EKT: u32 = 0x0100;

/* For DTLS - OP_PCLID_DTLS */

pub const OP_PCL_DTLS_AES_128_CBC_SHA: u32 = 0x002f;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_2: u32 = 0x0030;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_3: u32 = 0x0031;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_4: u32 = 0x0032;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_5: u32 = 0x0033;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_6: u32 = 0x0034;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_7: u32 = 0x008c;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_8: u32 = 0x0090;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_9: u32 = 0x0094;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_10: u32 = 0xc004;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_11: u32 = 0xc009;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_12: u32 = 0xc00e;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_13: u32 = 0xc013;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_14: u32 = 0xc018;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_15: u32 = 0xc01d;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_16: u32 = 0xc01e;
pub const OP_PCL_DTLS_AES_128_CBC_SHA_17: u32 = 0xc01f;

pub const OP_PCL_DTLS_AES_256_CBC_SHA: u32 = 0x0035;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_2: u32 = 0x0036;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_3: u32 = 0x0037;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_4: u32 = 0x0038;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_5: u32 = 0x0039;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_6: u32 = 0x003a;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_7: u32 = 0x008d;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_8: u32 = 0x0091;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_9: u32 = 0x0095;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_10: u32 = 0xc005;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_11: u32 = 0xc00a;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_12: u32 = 0xc00f;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_13: u32 = 0xc014;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_14: u32 = 0xc019;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_15: u32 = 0xc020;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_16: u32 = 0xc021;
pub const OP_PCL_DTLS_AES_256_CBC_SHA_17: u32 = 0xc022;

/* #define OP_PCL_DTLS_3DES_EDE_CBC_MD5		0x0023 */

pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA: u32 = 0x001f;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_2: u32 = 0x008b;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_3: u32 = 0x008f;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_4: u32 = 0x0093;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_5: u32 = 0x000a;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_6: u32 = 0x000d;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_7: u32 = 0x0010;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_8: u32 = 0x0013;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_9: u32 = 0x0016;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_10: u32 = 0x001b;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_11: u32 = 0xc003;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_12: u32 = 0xc008;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_13: u32 = 0xc00d;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_14: u32 = 0xc012;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_15: u32 = 0xc017;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_16: u32 = 0xc01a;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_17: u32 = 0xc01b;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA_18: u32 = 0xc01c;

pub const OP_PCL_DTLS_DES40_CBC_MD5: u32 = 0x0029;

pub const OP_PCL_DTLS_DES_CBC_MD5: u32 = 0x0022;

pub const OP_PCL_DTLS_DES40_CBC_SHA: u32 = 0x0008;
pub const OP_PCL_DTLS_DES40_CBC_SHA_2: u32 = 0x000b;
pub const OP_PCL_DTLS_DES40_CBC_SHA_3: u32 = 0x000e;
pub const OP_PCL_DTLS_DES40_CBC_SHA_4: u32 = 0x0011;
pub const OP_PCL_DTLS_DES40_CBC_SHA_5: u32 = 0x0014;
pub const OP_PCL_DTLS_DES40_CBC_SHA_6: u32 = 0x0019;
pub const OP_PCL_DTLS_DES40_CBC_SHA_7: u32 = 0x0026;


pub const OP_PCL_DTLS_DES_CBC_SHA: u32 = 0x001e;
pub const OP_PCL_DTLS_DES_CBC_SHA_2: u32 = 0x0009;
pub const OP_PCL_DTLS_DES_CBC_SHA_3: u32 = 0x000c;
pub const OP_PCL_DTLS_DES_CBC_SHA_4: u32 = 0x000f;
pub const OP_PCL_DTLS_DES_CBC_SHA_5: u32 = 0x0012;
pub const OP_PCL_DTLS_DES_CBC_SHA_6: u32 = 0x0015;
pub const OP_PCL_DTLS_DES_CBC_SHA_7: u32 = 0x001a;


pub const OP_PCL_DTLS_3DES_EDE_CBC_MD5: u32 = 0xff23;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA160: u32 = 0xff30;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA224: u32 = 0xff34;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA256: u32 = 0xff36;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA384: u32 = 0xff33;
pub const OP_PCL_DTLS_3DES_EDE_CBC_SHA512: u32 = 0xff35;
pub const OP_PCL_DTLS_AES_128_CBC_SHA160: u32 = 0xff80;
pub const OP_PCL_DTLS_AES_128_CBC_SHA224: u32 = 0xff84;
pub const OP_PCL_DTLS_AES_128_CBC_SHA256: u32 = 0xff86;
pub const OP_PCL_DTLS_AES_128_CBC_SHA384: u32 = 0xff83;
pub const OP_PCL_DTLS_AES_128_CBC_SHA512: u32 = 0xff85;
pub const OP_PCL_DTLS_AES_192_CBC_SHA160: u32 = 0xff20;
pub const OP_PCL_DTLS_AES_192_CBC_SHA224: u32 = 0xff24;
pub const OP_PCL_DTLS_AES_192_CBC_SHA256: u32 = 0xff26;
pub const OP_PCL_DTLS_AES_192_CBC_SHA384: u32 = 0xff23;
pub const OP_PCL_DTLS_AES_192_CBC_SHA512: u32 = 0xff25;
pub const OP_PCL_DTLS_AES_256_CBC_SHA160: u32 = 0xff60;
pub const OP_PCL_DTLS_AES_256_CBC_SHA224: u32 = 0xff64;
pub const OP_PCL_DTLS_AES_256_CBC_SHA256: u32 = 0xff66;
pub const OP_PCL_DTLS_AES_256_CBC_SHA384: u32 = 0xff63;
pub const OP_PCL_DTLS_AES_256_CBC_SHA512: u32 = 0xff65;

/* 802.16 WiMAX protinfos */
pub const OP_PCL_WIMAX_OFDM: u32 = 0x0201;
pub const OP_PCL_WIMAX_OFDMA: u32 = 0x0231;

/* 802.11 WiFi protinfos */
pub const OP_PCL_WIFI: u32 = 0xac04;

/* MacSec protinfos */
pub const OP_PCL_MACSEC: u32 = 0x0001;

/* Derived Key Protocol (DKP) Protinfo */
pub const OP_PCL_DKP_SRC_SHIFT: u32 = 14;
pub const OP_PCL_DKP_SRC_MASK: u32 = (3 << OP_PCL_DKP_SRC_SHIFT);
pub const OP_PCL_DKP_SRC_IMM: u32 = (0 << OP_PCL_DKP_SRC_SHIFT);
pub const OP_PCL_DKP_SRC_SEQ: u32 = (1 << OP_PCL_DKP_SRC_SHIFT);
pub const OP_PCL_DKP_SRC_PTR: u32 = (2 << OP_PCL_DKP_SRC_SHIFT);
pub const OP_PCL_DKP_SRC_SGF: u32 = (3 << OP_PCL_DKP_SRC_SHIFT);
pub const OP_PCL_DKP_DST_SHIFT: u32 = 12;
pub const OP_PCL_DKP_DST_MASK: u32 = (3 << OP_PCL_DKP_DST_SHIFT);
pub const OP_PCL_DKP_DST_IMM: u32 = (0 << OP_PCL_DKP_DST_SHIFT);
pub const OP_PCL_DKP_DST_SEQ: u32 = (1 << OP_PCL_DKP_DST_SHIFT);
pub const OP_PCL_DKP_DST_PTR: u32 = (2 << OP_PCL_DKP_DST_SHIFT);
pub const OP_PCL_DKP_DST_SGF: u32 = (3 << OP_PCL_DKP_DST_SHIFT);
pub const OP_PCL_DKP_KEY_SHIFT: u32 = 0;
pub const OP_PCL_DKP_KEY_MASK: u32 = (0xfff << OP_PCL_DKP_KEY_SHIFT);

/* PKI unidirectional protocol protinfo bits */
pub const OP_PCL_PKPROT_TEST: u32 = 0x0008;
pub const OP_PCL_PKPROT_DECRYPT: u32 = 0x0004;
pub const OP_PCL_PKPROT_ECC: u32 = 0x0002;
pub const OP_PCL_PKPROT_F2M: u32 = 0x0001;

/* For non-protocol/alg-only op commands */
pub const OP_ALG_TYPE_SHIFT: u32 = 24;
pub const OP_ALG_TYPE_MASK: u32 = (0x7 << OP_ALG_TYPE_SHIFT);
pub const OP_ALG_TYPE_CLASS1: u32 = (2 << OP_ALG_TYPE_SHIFT);
pub const OP_ALG_TYPE_CLASS2: u32 = (4 << OP_ALG_TYPE_SHIFT);

/* version register fields */
pub const OP_VER_CCHA_NUM: u32 = 0x000000ff /* Number CCHAs instantiated */;
pub const OP_VER_CCHA_MISC: u32 = 0x0000ff00 /* CCHA Miscellaneous Information */;
pub const OP_VER_CCHA_REV: u32 = 0x00ff0000 /* CCHA Revision Number */;
pub const OP_VER_CCHA_VID: u32 = 0xff000000 /* CCHA Version ID */;

pub const OP_ALG_ALGSEL_SHIFT: u32 = 16;
pub const OP_ALG_ALGSEL_MASK: u32 = (0xff << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_SUBMASK: u32 = (0x0f << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_AES: u32 = (0x10 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_DES: u32 = (0x20 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_3DES: u32 = (0x21 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_ARC4: u32 = (0x30 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_CHA_MDHA: u32 = (0x40 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_MD5: u32 = (0x40 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_SHA1: u32 = (0x41 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_SHA224: u32 = (0x42 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_SHA256: u32 = (0x43 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_SHA384: u32 = (0x44 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_SHA512: u32 = (0x45 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_RNG: u32 = (0x50 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_SNOW: u32 = (0x60 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_SNOW_F8: u32 = (0x60 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_KASUMI: u32 = (0x70 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_CRC: u32 = (0x90 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_SNOW_F9: u32 = (0xA0 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_CHACHA20: u32 = (0xD0 << OP_ALG_ALGSEL_SHIFT);
pub const OP_ALG_ALGSEL_POLY1305: u32 = (0xE0 << OP_ALG_ALGSEL_SHIFT);

pub const OP_ALG_AAI_SHIFT: u32 = 4;
pub const OP_ALG_AAI_MASK: u32 = (0x1ff << OP_ALG_AAI_SHIFT);

/* blockcipher AAI set */
pub const OP_ALG_AAI_CTR_MOD128: u32 = (0x00 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD8: u32 = (0x01 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD16: u32 = (0x02 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD24: u32 = (0x03 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD32: u32 = (0x04 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD40: u32 = (0x05 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD48: u32 = (0x06 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD56: u32 = (0x07 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD64: u32 = (0x08 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD72: u32 = (0x09 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD80: u32 = (0x0a << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD88: u32 = (0x0b << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD96: u32 = (0x0c << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD104: u32 = (0x0d << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD112: u32 = (0x0e << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_MOD120: u32 = (0x0f << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CBC: u32 = (0x10 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_ECB: u32 = (0x20 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CFB: u32 = (0x30 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_OFB: u32 = (0x40 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_XTS: u32 = (0x50 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CMAC: u32 = (0x60 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_XCBC_MAC: u32 = (0x70 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CCM: u32 = (0x80 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_GCM: u32 = (0x90 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CBC_XCBCMAC: u32 = (0xa0 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CTR_XCBCMAC: u32 = (0xb0 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CHECKODD: u32 = (0x80 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_DK: u32 = (0x100 << OP_ALG_AAI_SHIFT);

/* randomizer AAI set */
pub const OP_ALG_AAI_RNG: u32 = (0x00 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_RNG_NZB: u32 = (0x10 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_RNG_OBP: u32 = (0x20 << OP_ALG_AAI_SHIFT);

/* RNG4 AAI set */
pub const OP_ALG_AAI_RNG4_SH_0: u32 = (0x00 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_RNG4_SH_1: u32 = (0x01 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_RNG4_PS: u32 = (0x40 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_RNG4_AI: u32 = (0x80 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_RNG4_SK: u32 = (0x100 << OP_ALG_AAI_SHIFT);

/* Chacha20 AAI set */
pub const OP_ALG_AAI_AEAD: u32 = (0x002 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_KEYSTREAM: u32 = (0x001 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_BC8: u32 = (0x008 << OP_ALG_AAI_SHIFT);

/* hmac/smac AAI set */
pub const OP_ALG_AAI_HASH: u32 = (0x00 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_HMAC: u32 = (0x01 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_SMAC: u32 = (0x02 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_HMAC_PRECOMP: u32 = (0x04 << OP_ALG_AAI_SHIFT);

/* CRC AAI set*/
pub const OP_ALG_AAI_802: u32 = (0x01 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_3385: u32 = (0x02 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_CUST_POLY: u32 = (0x04 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_DIS: u32 = (0x10 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_DOS: u32 = (0x20 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_DOC: u32 = (0x40 << OP_ALG_AAI_SHIFT);

/* Kasumi/SNOW AAI set */
pub const OP_ALG_AAI_F8: u32 = (0xc0 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_F9: u32 = (0xc8 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_GSM: u32 = (0x10 << OP_ALG_AAI_SHIFT);
pub const OP_ALG_AAI_EDGE: u32 = (0x20 << OP_ALG_AAI_SHIFT);

pub const OP_ALG_AS_SHIFT: u32 = 2;
pub const OP_ALG_AS_MASK: u32 = (0x3 << OP_ALG_AS_SHIFT);
pub const OP_ALG_AS_UPDATE: u32 = (0 << OP_ALG_AS_SHIFT);
pub const OP_ALG_AS_INIT: u32 = (1 << OP_ALG_AS_SHIFT);
pub const OP_ALG_AS_FINALIZE: u32 = (2 << OP_ALG_AS_SHIFT);
pub const OP_ALG_AS_INITFINAL: u32 = (3 << OP_ALG_AS_SHIFT);

pub const OP_ALG_ICV_SHIFT: u32 = 1;
pub const OP_ALG_ICV_MASK: u32 = (1 << OP_ALG_ICV_SHIFT);
pub const OP_ALG_ICV_OFF: u32 = (0 << OP_ALG_ICV_SHIFT);
pub const OP_ALG_ICV_ON: u32 = (1 << OP_ALG_ICV_SHIFT);

pub const OP_ALG_PR_ON: u32 = BIT(1);

pub const OP_ALG_DIR_SHIFT: u32 = 0;
pub const OP_ALG_DIR_MASK: u32 = 1;
pub const OP_ALG_DECRYPT: u32 = 0;
pub const OP_ALG_ENCRYPT: u32 = 1;

/* PKHA algorithm type set */
pub const OP_ALG_PK: u32 = 0x00800000;
pub const OP_ALG_PK_FUN_MASK: u32 = 0x3f /* clrmem, modmath, or cpymem */;

/* PKHA mode clear memory functions */
pub const OP_ALG_PKMODE_A_RAM: u32 = 0x80000;
pub const OP_ALG_PKMODE_B_RAM: u32 = 0x40000;
pub const OP_ALG_PKMODE_E_RAM: u32 = 0x20000;
pub const OP_ALG_PKMODE_N_RAM: u32 = 0x10000;
pub const OP_ALG_PKMODE_CLEARMEM: u32 = 0x00001;

/* PKHA mode modular-arithmetic functions */
pub const OP_ALG_PKMODE_MOD_IN_MONTY: u32 = 0x80000;
pub const OP_ALG_PKMODE_MOD_OUT_MONTY: u32 = 0x40000;
pub const OP_ALG_PKMODE_MOD_F2M: u32 = 0x20000;
pub const OP_ALG_PKMODE_MOD_R2_IN: u32 = 0x10000;
pub const OP_ALG_PKMODE_PRJECTV: u32 = 0x00800;
pub const OP_ALG_PKMODE_TIME_EQ: u32 = 0x400;
pub const OP_ALG_PKMODE_OUT_B: u32 = 0x000;
pub const OP_ALG_PKMODE_OUT_A: u32 = 0x100;
pub const OP_ALG_PKMODE_MOD_ADD: u32 = 0x002;
pub const OP_ALG_PKMODE_MOD_SUB_AB: u32 = 0x003;
pub const OP_ALG_PKMODE_MOD_SUB_BA: u32 = 0x004;
pub const OP_ALG_PKMODE_MOD_MULT: u32 = 0x005;
pub const OP_ALG_PKMODE_MOD_EXPO: u32 = 0x006;
pub const OP_ALG_PKMODE_MOD_REDUCT: u32 = 0x007;
pub const OP_ALG_PKMODE_MOD_INV: u32 = 0x008;
pub const OP_ALG_PKMODE_MOD_ECC_ADD: u32 = 0x009;
pub const OP_ALG_PKMODE_MOD_ECC_DBL: u32 = 0x00a;
pub const OP_ALG_PKMODE_MOD_ECC_MULT: u32 = 0x00b;
pub const OP_ALG_PKMODE_MOD_MONT_CNST: u32 = 0x00c;
pub const OP_ALG_PKMODE_MOD_CRT_CNST: u32 = 0x00d;
pub const OP_ALG_PKMODE_MOD_GCD: u32 = 0x00e;
pub const OP_ALG_PKMODE_MOD_PRIMALITY: u32 = 0x00f;

/* PKHA mode copy-memory functions */
pub const OP_ALG_PKMODE_SRC_REG_SHIFT: u32 = 17;
pub const OP_ALG_PKMODE_SRC_REG_MASK: u32 = (7 << OP_ALG_PKMODE_SRC_REG_SHIFT);
pub const OP_ALG_PKMODE_DST_REG_SHIFT: u32 = 10;
pub const OP_ALG_PKMODE_DST_REG_MASK: u32 = (7 << OP_ALG_PKMODE_DST_REG_SHIFT);
pub const OP_ALG_PKMODE_SRC_SEG_SHIFT: u32 = 8;
pub const OP_ALG_PKMODE_SRC_SEG_MASK: u32 = (3 << OP_ALG_PKMODE_SRC_SEG_SHIFT);
pub const OP_ALG_PKMODE_DST_SEG_SHIFT: u32 = 6;
pub const OP_ALG_PKMODE_DST_SEG_MASK: u32 = (3 << OP_ALG_PKMODE_DST_SEG_SHIFT);

pub const OP_ALG_PKMODE_SRC_REG_A: u32 = (0 << OP_ALG_PKMODE_SRC_REG_SHIFT);
pub const OP_ALG_PKMODE_SRC_REG_B: u32 = (1 << OP_ALG_PKMODE_SRC_REG_SHIFT);
pub const OP_ALG_PKMODE_SRC_REG_N: u32 = (3 << OP_ALG_PKMODE_SRC_REG_SHIFT);
pub const OP_ALG_PKMODE_DST_REG_A: u32 = (0 << OP_ALG_PKMODE_DST_REG_SHIFT);
pub const OP_ALG_PKMODE_DST_REG_B: u32 = (1 << OP_ALG_PKMODE_DST_REG_SHIFT);
pub const OP_ALG_PKMODE_DST_REG_E: u32 = (2 << OP_ALG_PKMODE_DST_REG_SHIFT);
pub const OP_ALG_PKMODE_DST_REG_N: u32 = (3 << OP_ALG_PKMODE_DST_REG_SHIFT);
pub const OP_ALG_PKMODE_SRC_SEG_0: u32 = (0 << OP_ALG_PKMODE_SRC_SEG_SHIFT);
pub const OP_ALG_PKMODE_SRC_SEG_1: u32 = (1 << OP_ALG_PKMODE_SRC_SEG_SHIFT);
pub const OP_ALG_PKMODE_SRC_SEG_2: u32 = (2 << OP_ALG_PKMODE_SRC_SEG_SHIFT);
pub const OP_ALG_PKMODE_SRC_SEG_3: u32 = (3 << OP_ALG_PKMODE_SRC_SEG_SHIFT);
pub const OP_ALG_PKMODE_DST_SEG_0: u32 = (0 << OP_ALG_PKMODE_DST_SEG_SHIFT);
pub const OP_ALG_PKMODE_DST_SEG_1: u32 = (1 << OP_ALG_PKMODE_DST_SEG_SHIFT);
pub const OP_ALG_PKMODE_DST_SEG_2: u32 = (2 << OP_ALG_PKMODE_DST_SEG_SHIFT);
pub const OP_ALG_PKMODE_DST_SEG_3: u32 = (3 << OP_ALG_PKMODE_DST_SEG_SHIFT);
pub const OP_ALG_PKMODE_CPYMEM_N_SZ: u32 = 0x80;
pub const OP_ALG_PKMODE_CPYMEM_SRC_SZ: u32 = 0x81;

/*
 * SEQ_IN_PTR Command Constructs
 */

/* Release Buffers */
pub const SQIN_RBS: u32 = 0x04000000;

/* Sequence pointer is really a descriptor */
pub const SQIN_INL: u32 = 0x02000000;

/* Sequence pointer is a scatter-gather table */
pub const SQIN_SGF: u32 = 0x01000000;

/* Appends to a previous pointer */
pub const SQIN_PRE: u32 = 0x00800000;

/* Use extended length following pointer */
pub const SQIN_EXT: u32 = 0x00400000;

/* Restore sequence with pointer/length */
pub const SQIN_RTO: u32 = 0x00200000;

/* Replace job descriptor */
pub const SQIN_RJD: u32 = 0x00100000;

pub const SQIN_LEN_SHIFT: u32 = 0;
pub const SQIN_LEN_MASK: u32 = (0xffff << SQIN_LEN_SHIFT);

/*
 * SEQ_OUT_PTR Command Constructs
 */

/* Sequence pointer is a scatter-gather table */
pub const SQOUT_SGF: u32 = 0x01000000;

/* Appends to a previous pointer */
pub const SQOUT_PRE: u32 = SQIN_PRE;

/* Restore sequence with pointer/length */
pub const SQOUT_RTO: u32 = SQIN_RTO;

/* Use extended length following pointer */
pub const SQOUT_EXT: u32 = 0x00400000;

pub const SQOUT_LEN_SHIFT: u32 = 0;
pub const SQOUT_LEN_MASK: u32 = (0xffff << SQOUT_LEN_SHIFT);


/*
 * SIGNATURE Command Constructs
 */

/* TYPE field is all that's relevant */
pub const SIGN_TYPE_SHIFT: u32 = 16;
pub const SIGN_TYPE_MASK: u32 = (0x0f << SIGN_TYPE_SHIFT);

pub const SIGN_TYPE_FINAL: u32 = (0x00 << SIGN_TYPE_SHIFT);
pub const SIGN_TYPE_FINAL_RESTORE: u32 = (0x01 << SIGN_TYPE_SHIFT);
pub const SIGN_TYPE_FINAL_NONZERO: u32 = (0x02 << SIGN_TYPE_SHIFT);
pub const SIGN_TYPE_IMM_2: u32 = (0x0a << SIGN_TYPE_SHIFT);
pub const SIGN_TYPE_IMM_3: u32 = (0x0b << SIGN_TYPE_SHIFT);
pub const SIGN_TYPE_IMM_4: u32 = (0x0c << SIGN_TYPE_SHIFT);

/*
 * MOVE Command Constructs
 */

pub const MOVE_AUX_SHIFT: u32 = 25;
pub const MOVE_AUX_MASK: u32 = (3 << MOVE_AUX_SHIFT);
pub const MOVE_AUX_MS: u32 = (2 << MOVE_AUX_SHIFT);
pub const MOVE_AUX_LS: u32 = (1 << MOVE_AUX_SHIFT);

pub const MOVE_WAITCOMP_SHIFT: u32 = 24;
pub const MOVE_WAITCOMP_MASK: u32 = (1 << MOVE_WAITCOMP_SHIFT);
pub const MOVE_WAITCOMP: u32 = (1 << MOVE_WAITCOMP_SHIFT);

pub const MOVE_SRC_SHIFT: u32 = 20;
pub const MOVE_SRC_MASK: u32 = (0x0f << MOVE_SRC_SHIFT);
pub const MOVE_SRC_CLASS1CTX: u32 = (0x00 << MOVE_SRC_SHIFT);
pub const MOVE_SRC_CLASS2CTX: u32 = (0x01 << MOVE_SRC_SHIFT);
pub const MOVE_SRC_OUTFIFO: u32 = (0x02 << MOVE_SRC_SHIFT);
pub const MOVE_SRC_DESCBUF: u32 = (0x03 << MOVE_SRC_SHIFT);
pub const MOVE_SRC_MATH0: u32 = (0x04 << MOVE_SRC_SHIFT);
pub const MOVE_SRC_MATH1: u32 = (0x05 << MOVE_SRC_SHIFT);
pub const MOVE_SRC_MATH2: u32 = (0x06 << MOVE_SRC_SHIFT);
pub const MOVE_SRC_MATH3: u32 = (0x07 << MOVE_SRC_SHIFT);
pub const MOVE_SRC_INFIFO: u32 = (0x08 << MOVE_SRC_SHIFT);
pub const MOVE_SRC_INFIFO_CL: u32 = (0x09 << MOVE_SRC_SHIFT);
pub const MOVE_SRC_AUX_ABLK: u32 = (0x0a << MOVE_SRC_SHIFT);

pub const MOVE_DEST_SHIFT: u32 = 16;
pub const MOVE_DEST_MASK: u32 = (0x0f << MOVE_DEST_SHIFT);
pub const MOVE_DEST_CLASS1CTX: u32 = (0x00 << MOVE_DEST_SHIFT);
pub const MOVE_DEST_CLASS2CTX: u32 = (0x01 << MOVE_DEST_SHIFT);
pub const MOVE_DEST_OUTFIFO: u32 = (0x02 << MOVE_DEST_SHIFT);
pub const MOVE_DEST_DESCBUF: u32 = (0x03 << MOVE_DEST_SHIFT);
pub const MOVE_DEST_MATH0: u32 = (0x04 << MOVE_DEST_SHIFT);
pub const MOVE_DEST_MATH1: u32 = (0x05 << MOVE_DEST_SHIFT);
pub const MOVE_DEST_MATH2: u32 = (0x06 << MOVE_DEST_SHIFT);
pub const MOVE_DEST_MATH3: u32 = (0x07 << MOVE_DEST_SHIFT);
pub const MOVE_DEST_CLASS1INFIFO: u32 = (0x08 << MOVE_DEST_SHIFT);
pub const MOVE_DEST_CLASS2INFIFO: u32 = (0x09 << MOVE_DEST_SHIFT);
pub const MOVE_DEST_INFIFO_NOINFO: u32 = (0x0a << MOVE_DEST_SHIFT);
pub const MOVE_DEST_PK_A: u32 = (0x0c << MOVE_DEST_SHIFT);
pub const MOVE_DEST_CLASS1KEY: u32 = (0x0d << MOVE_DEST_SHIFT);
pub const MOVE_DEST_CLASS2KEY: u32 = (0x0e << MOVE_DEST_SHIFT);

pub const MOVE_OFFSET_SHIFT: u32 = 8;
pub const MOVE_OFFSET_MASK: u32 = (0xff << MOVE_OFFSET_SHIFT);

pub const MOVE_LEN_SHIFT: u32 = 0;
pub const MOVE_LEN_MASK: u32 = (0xff << MOVE_LEN_SHIFT);

pub const MOVELEN_MRSEL_SHIFT: u32 = 0;
pub const MOVELEN_MRSEL_MASK: u32 = (0x3 << MOVE_LEN_SHIFT);
pub const MOVELEN_MRSEL_MATH0: u32 = (0 << MOVELEN_MRSEL_SHIFT);
pub const MOVELEN_MRSEL_MATH1: u32 = (1 << MOVELEN_MRSEL_SHIFT);
pub const MOVELEN_MRSEL_MATH2: u32 = (2 << MOVELEN_MRSEL_SHIFT);
pub const MOVELEN_MRSEL_MATH3: u32 = (3 << MOVELEN_MRSEL_SHIFT);

/*
 * MATH Command Constructs
 */

pub const MATH_IFB_SHIFT: u32 = 26;
pub const MATH_IFB_MASK: u32 = (1 << MATH_IFB_SHIFT);
pub const MATH_IFB: u32 = (1 << MATH_IFB_SHIFT);

pub const MATH_NFU_SHIFT: u32 = 25;
pub const MATH_NFU_MASK: u32 = (1 << MATH_NFU_SHIFT);
pub const MATH_NFU: u32 = (1 << MATH_NFU_SHIFT);

pub const MATH_STL_SHIFT: u32 = 24;
pub const MATH_STL_MASK: u32 = (1 << MATH_STL_SHIFT);
pub const MATH_STL: u32 = (1 << MATH_STL_SHIFT);

/* Function selectors */
pub const MATH_FUN_SHIFT: u32 = 20;
pub const MATH_FUN_MASK: u32 = (0x0f << MATH_FUN_SHIFT);
pub const MATH_FUN_ADD: u32 = (0x00 << MATH_FUN_SHIFT);
pub const MATH_FUN_ADDC: u32 = (0x01 << MATH_FUN_SHIFT);
pub const MATH_FUN_SUB: u32 = (0x02 << MATH_FUN_SHIFT);
pub const MATH_FUN_SUBB: u32 = (0x03 << MATH_FUN_SHIFT);
pub const MATH_FUN_OR: u32 = (0x04 << MATH_FUN_SHIFT);
pub const MATH_FUN_AND: u32 = (0x05 << MATH_FUN_SHIFT);
pub const MATH_FUN_XOR: u32 = (0x06 << MATH_FUN_SHIFT);
pub const MATH_FUN_LSHIFT: u32 = (0x07 << MATH_FUN_SHIFT);
pub const MATH_FUN_RSHIFT: u32 = (0x08 << MATH_FUN_SHIFT);
pub const MATH_FUN_SHLD: u32 = (0x09 << MATH_FUN_SHIFT);
pub const MATH_FUN_ZBYT: u32 = (0x0a << MATH_FUN_SHIFT);

/* Source 0 selectors */
pub const MATH_SRC0_SHIFT: u32 = 16;
pub const MATH_SRC0_MASK: u32 = (0x0f << MATH_SRC0_SHIFT);
pub const MATH_SRC0_REG0: u32 = (0x00 << MATH_SRC0_SHIFT);
pub const MATH_SRC0_REG1: u32 = (0x01 << MATH_SRC0_SHIFT);
pub const MATH_SRC0_REG2: u32 = (0x02 << MATH_SRC0_SHIFT);
pub const MATH_SRC0_REG3: u32 = (0x03 << MATH_SRC0_SHIFT);
pub const MATH_SRC0_IMM: u32 = (0x04 << MATH_SRC0_SHIFT);
pub const MATH_SRC0_DPOVRD: u32 = (0x07 << MATH_SRC0_SHIFT);
pub const MATH_SRC0_SEQINLEN: u32 = (0x08 << MATH_SRC0_SHIFT);
pub const MATH_SRC0_SEQOUTLEN: u32 = (0x09 << MATH_SRC0_SHIFT);
pub const MATH_SRC0_VARSEQINLEN: u32 = (0x0a << MATH_SRC0_SHIFT);
pub const MATH_SRC0_VARSEQOUTLEN: u32 = (0x0b << MATH_SRC0_SHIFT);
pub const MATH_SRC0_ZERO: u32 = (0x0c << MATH_SRC0_SHIFT);

/* Source 1 selectors */
pub const MATH_SRC1_SHIFT: u32 = 12;
pub const MATH_SRC1_MASK: u32 = (0x0f << MATH_SRC1_SHIFT);
pub const MATH_SRC1_REG0: u32 = (0x00 << MATH_SRC1_SHIFT);
pub const MATH_SRC1_REG1: u32 = (0x01 << MATH_SRC1_SHIFT);
pub const MATH_SRC1_REG2: u32 = (0x02 << MATH_SRC1_SHIFT);
pub const MATH_SRC1_REG3: u32 = (0x03 << MATH_SRC1_SHIFT);
pub const MATH_SRC1_IMM: u32 = (0x04 << MATH_SRC1_SHIFT);
pub const MATH_SRC1_DPOVRD: u32 = (0x07 << MATH_SRC1_SHIFT);
pub const MATH_SRC1_INFIFO: u32 = (0x0a << MATH_SRC1_SHIFT);
pub const MATH_SRC1_OUTFIFO: u32 = (0x0b << MATH_SRC1_SHIFT);
pub const MATH_SRC1_ONE: u32 = (0x0c << MATH_SRC1_SHIFT);

/* Destination selectors */
pub const MATH_DEST_SHIFT: u32 = 8;
pub const MATH_DEST_MASK: u32 = (0x0f << MATH_DEST_SHIFT);
pub const MATH_DEST_REG0: u32 = (0x00 << MATH_DEST_SHIFT);
pub const MATH_DEST_REG1: u32 = (0x01 << MATH_DEST_SHIFT);
pub const MATH_DEST_REG2: u32 = (0x02 << MATH_DEST_SHIFT);
pub const MATH_DEST_REG3: u32 = (0x03 << MATH_DEST_SHIFT);
pub const MATH_DEST_DPOVRD: u32 = (0x07 << MATH_DEST_SHIFT);
pub const MATH_DEST_SEQINLEN: u32 = (0x08 << MATH_DEST_SHIFT);
pub const MATH_DEST_SEQOUTLEN: u32 = (0x09 << MATH_DEST_SHIFT);
pub const MATH_DEST_VARSEQINLEN: u32 = (0x0a << MATH_DEST_SHIFT);
pub const MATH_DEST_VARSEQOUTLEN: u32 = (0x0b << MATH_DEST_SHIFT);
pub const MATH_DEST_NONE: u32 = (0x0f << MATH_DEST_SHIFT);

/* Length selectors */
pub const MATH_LEN_SHIFT: u32 = 0;
pub const MATH_LEN_MASK: u32 = (0x0f << MATH_LEN_SHIFT);
pub const MATH_LEN_1BYTE: u32 = 0x01;
pub const MATH_LEN_2BYTE: u32 = 0x02;
pub const MATH_LEN_4BYTE: u32 = 0x04;
pub const MATH_LEN_8BYTE: u32 = 0x08;

/*
 * JUMP Command Constructs
 */

pub const JUMP_CLASS_SHIFT: u32 = 25;
pub const JUMP_CLASS_MASK: u32 = (3 << JUMP_CLASS_SHIFT);
pub const JUMP_CLASS_NONE: u32 = 0;
pub const JUMP_CLASS_CLASS1: u32 = (1 << JUMP_CLASS_SHIFT);
pub const JUMP_CLASS_CLASS2: u32 = (2 << JUMP_CLASS_SHIFT);
pub const JUMP_CLASS_BOTH: u32 = (3 << JUMP_CLASS_SHIFT);

pub const JUMP_JSL_SHIFT: u32 = 24;
pub const JUMP_JSL_MASK: u32 = (1 << JUMP_JSL_SHIFT);
pub const JUMP_JSL: u32 = (1 << JUMP_JSL_SHIFT);

pub const JUMP_TYPE_SHIFT: u32 = 22;
pub const JUMP_TYPE_LOCAL: u32 = (0x00 << JUMP_TYPE_SHIFT);
pub const JUMP_TYPE_NONLOCAL: u32 = (0x01 << JUMP_TYPE_SHIFT);
pub const JUMP_TYPE_HALT: u32 = (0x02 << JUMP_TYPE_SHIFT);
pub const JUMP_TYPE_HALT_USER: u32 = (0x03 << JUMP_TYPE_SHIFT);

pub const JUMP_TEST_SHIFT: u32 = 16;
pub const JUMP_TEST_MASK: u32 = (0x03 << JUMP_TEST_SHIFT);
pub const JUMP_TEST_ALL: u32 = (0x00 << JUMP_TEST_SHIFT);
pub const JUMP_TEST_INVALL: u32 = (0x01 << JUMP_TEST_SHIFT);
pub const JUMP_TEST_ANY: u32 = (0x02 << JUMP_TEST_SHIFT);
pub const JUMP_TEST_INVANY: u32 = (0x03 << JUMP_TEST_SHIFT);

/* Condition codes. JSL bit is factored in */
pub const JUMP_COND_SHIFT: u32 = 8;
pub const JUMP_COND_MASK: u32 = (0x100ff << JUMP_COND_SHIFT);
pub const JUMP_COND_PK_0: u32 = (0x80 << JUMP_COND_SHIFT);
pub const JUMP_COND_PK_GCD_1: u32 = (0x40 << JUMP_COND_SHIFT);
pub const JUMP_COND_PK_PRIME: u32 = (0x20 << JUMP_COND_SHIFT);
pub const JUMP_COND_MATH_N: u32 = (0x08 << JUMP_COND_SHIFT);
pub const JUMP_COND_MATH_Z: u32 = (0x04 << JUMP_COND_SHIFT);
pub const JUMP_COND_MATH_C: u32 = (0x02 << JUMP_COND_SHIFT);
pub const JUMP_COND_MATH_NV: u32 = (0x01 << JUMP_COND_SHIFT);

pub const JUMP_COND_JRP: u32 = ((0x80 << JUMP_COND_SHIFT) | JUMP_JSL);
pub const JUMP_COND_SHRD: u32 = ((0x40 << JUMP_COND_SHIFT) | JUMP_JSL);
pub const JUMP_COND_SELF: u32 = ((0x20 << JUMP_COND_SHIFT) | JUMP_JSL);
pub const JUMP_COND_CALM: u32 = ((0x10 << JUMP_COND_SHIFT) | JUMP_JSL);
pub const JUMP_COND_NIP: u32 = ((0x08 << JUMP_COND_SHIFT) | JUMP_JSL);
pub const JUMP_COND_NIFP: u32 = ((0x04 << JUMP_COND_SHIFT) | JUMP_JSL);
pub const JUMP_COND_NOP: u32 = ((0x02 << JUMP_COND_SHIFT) | JUMP_JSL);
pub const JUMP_COND_NCP: u32 = ((0x01 << JUMP_COND_SHIFT) | JUMP_JSL);

pub const JUMP_OFFSET_SHIFT: u32 = 0;
pub const JUMP_OFFSET_MASK: u32 = (0xff << JUMP_OFFSET_SHIFT);

/*
 * NFIFO ENTRY
 * Data Constructs
 *
 */
pub const NFIFOENTRY_DEST_SHIFT: u32 = 30;
pub const NFIFOENTRY_DEST_MASK: u32 = (3 << NFIFOENTRY_DEST_SHIFT);
pub const NFIFOENTRY_DEST_DECO: u32 = (0 << NFIFOENTRY_DEST_SHIFT);
pub const NFIFOENTRY_DEST_CLASS1: u32 = (1 << NFIFOENTRY_DEST_SHIFT);
pub const NFIFOENTRY_DEST_CLASS2: u32 = (2 << NFIFOENTRY_DEST_SHIFT);
pub const NFIFOENTRY_DEST_BOTH: u32 = (3 << NFIFOENTRY_DEST_SHIFT);

pub const NFIFOENTRY_LC2_SHIFT: u32 = 29;
pub const NFIFOENTRY_LC2_MASK: u32 = (1 << NFIFOENTRY_LC2_SHIFT);
pub const NFIFOENTRY_LC2: u32 = (1 << NFIFOENTRY_LC2_SHIFT);

pub const NFIFOENTRY_LC1_SHIFT: u32 = 28;
pub const NFIFOENTRY_LC1_MASK: u32 = (1 << NFIFOENTRY_LC1_SHIFT);
pub const NFIFOENTRY_LC1: u32 = (1 << NFIFOENTRY_LC1_SHIFT);

pub const NFIFOENTRY_FC2_SHIFT: u32 = 27;
pub const NFIFOENTRY_FC2_MASK: u32 = (1 << NFIFOENTRY_FC2_SHIFT);
pub const NFIFOENTRY_FC2: u32 = (1 << NFIFOENTRY_FC2_SHIFT);

pub const NFIFOENTRY_FC1_SHIFT: u32 = 26;
pub const NFIFOENTRY_FC1_MASK: u32 = (1 << NFIFOENTRY_FC1_SHIFT);
pub const NFIFOENTRY_FC1: u32 = (1 << NFIFOENTRY_FC1_SHIFT);

pub const NFIFOENTRY_STYPE_SHIFT: u32 = 24;
pub const NFIFOENTRY_STYPE_MASK: u32 = (3 << NFIFOENTRY_STYPE_SHIFT);
pub const NFIFOENTRY_STYPE_DFIFO: u32 = (0 << NFIFOENTRY_STYPE_SHIFT);
pub const NFIFOENTRY_STYPE_OFIFO: u32 = (1 << NFIFOENTRY_STYPE_SHIFT);
pub const NFIFOENTRY_STYPE_PAD: u32 = (2 << NFIFOENTRY_STYPE_SHIFT);
pub const NFIFOENTRY_STYPE_SNOOP: u32 = (3 << NFIFOENTRY_STYPE_SHIFT);

pub const NFIFOENTRY_DTYPE_SHIFT: u32 = 20;
pub const NFIFOENTRY_DTYPE_MASK: u32 = (0xF << NFIFOENTRY_DTYPE_SHIFT);

pub const NFIFOENTRY_DTYPE_SBOX: u32 = (0x0 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_AAD: u32 = (0x1 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_IV: u32 = (0x2 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_SAD: u32 = (0x3 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_ICV: u32 = (0xA << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_POLY: u32 = (0xB << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_SKIP: u32 = (0xE << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_MSG: u32 = (0xF << NFIFOENTRY_DTYPE_SHIFT);

pub const NFIFOENTRY_DTYPE_PK_A0: u32 = (0x0 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_PK_A1: u32 = (0x1 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_PK_A2: u32 = (0x2 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_PK_A3: u32 = (0x3 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_PK_B0: u32 = (0x4 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_PK_B1: u32 = (0x5 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_PK_B2: u32 = (0x6 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_PK_B3: u32 = (0x7 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_PK_N: u32 = (0x8 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_PK_E: u32 = (0x9 << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_PK_A: u32 = (0xC << NFIFOENTRY_DTYPE_SHIFT);
pub const NFIFOENTRY_DTYPE_PK_B: u32 = (0xD << NFIFOENTRY_DTYPE_SHIFT);


pub const NFIFOENTRY_BND_SHIFT: u32 = 19;
pub const NFIFOENTRY_BND_MASK: u32 = (1 << NFIFOENTRY_BND_SHIFT);
pub const NFIFOENTRY_BND: u32 = (1 << NFIFOENTRY_BND_SHIFT);

pub const NFIFOENTRY_PTYPE_SHIFT: u32 = 16;
pub const NFIFOENTRY_PTYPE_MASK: u32 = (0x7 << NFIFOENTRY_PTYPE_SHIFT);

pub const NFIFOENTRY_PTYPE_ZEROS: u32 = (0x0 << NFIFOENTRY_PTYPE_SHIFT);
pub const NFIFOENTRY_PTYPE_RND_NOZEROS: u32 = (0x1 << NFIFOENTRY_PTYPE_SHIFT);
pub const NFIFOENTRY_PTYPE_INCREMENT: u32 = (0x2 << NFIFOENTRY_PTYPE_SHIFT);
pub const NFIFOENTRY_PTYPE_RND: u32 = (0x3 << NFIFOENTRY_PTYPE_SHIFT);
pub const NFIFOENTRY_PTYPE_ZEROS_NZ: u32 = (0x4 << NFIFOENTRY_PTYPE_SHIFT);
pub const NFIFOENTRY_PTYPE_RND_NZ_LZ: u32 = (0x5 << NFIFOENTRY_PTYPE_SHIFT);
pub const NFIFOENTRY_PTYPE_N: u32 = (0x6 << NFIFOENTRY_PTYPE_SHIFT);
pub const NFIFOENTRY_PTYPE_RND_NZ_N: u32 = (0x7 << NFIFOENTRY_PTYPE_SHIFT);

pub const NFIFOENTRY_OC_SHIFT: u32 = 15;
pub const NFIFOENTRY_OC_MASK: u32 = (1 << NFIFOENTRY_OC_SHIFT);
pub const NFIFOENTRY_OC: u32 = (1 << NFIFOENTRY_OC_SHIFT);

pub const NFIFOENTRY_AST_SHIFT: u32 = 14;
pub const NFIFOENTRY_AST_MASK: u32 = (1 << NFIFOENTRY_OC_SHIFT);
pub const NFIFOENTRY_AST: u32 = (1 << NFIFOENTRY_OC_SHIFT);

pub const NFIFOENTRY_BM_SHIFT: u32 = 11;
pub const NFIFOENTRY_BM_MASK: u32 = (1 << NFIFOENTRY_BM_SHIFT);
pub const NFIFOENTRY_BM: u32 = (1 << NFIFOENTRY_BM_SHIFT);

pub const NFIFOENTRY_PS_SHIFT: u32 = 10;
pub const NFIFOENTRY_PS_MASK: u32 = (1 << NFIFOENTRY_PS_SHIFT);
pub const NFIFOENTRY_PS: u32 = (1 << NFIFOENTRY_PS_SHIFT);

pub const NFIFOENTRY_DLEN_SHIFT: u32 = 0;
pub const NFIFOENTRY_DLEN_MASK: u32 = (0xFFF << NFIFOENTRY_DLEN_SHIFT);

pub const NFIFOENTRY_PLEN_SHIFT: u32 = 0;
pub const NFIFOENTRY_PLEN_MASK: u32 = (0xFF << NFIFOENTRY_PLEN_SHIFT);

/* Append Load Immediate Command */
pub const FD_CMD_APPEND_LOAD_IMMEDIATE: u32 = 0x80000000;

/* Set SEQ LIODN equal to the Non-SEQ LIODN for the job */
pub const FD_CMD_SET_SEQ_LIODN_EQUAL_NONSEQ_LIODN: u32 = 0x40000000;

/* Frame Descriptor Command for Replacement Job Descriptor */
pub const FD_CMD_REPLACE_JOB_DESC: u32 = 0x20000000;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
