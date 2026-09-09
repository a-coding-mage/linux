/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 - 2021
 *
 * Richard van Schagen <vschagen@icloud.com>
 * Christian Marangi <ansuelsmth@gmail.com>
 */

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(high: u32, low: u32) -> u32 { ((1u32 << (high - low + 1)) - 1) << low }
const fn field_prep(mask: u32, value: u32) -> u32 { (value << mask.trailing_zeros()) & mask }

pub const EIP93_REG_PE_CTRL_STAT: u32 = 0x0;
pub const EIP93_PE_CTRL_PE_PAD_CTRL_STAT: u32 = genmask(31, 24);
pub const EIP93_PE_CTRL_PE_EXT_ERR_CODE: u32 = genmask(23, 20);
pub const EIP93_PE_CTRL_PE_EXT_ERR_PROCESSING: u32 = 0x8;
pub const EIP93_PE_CTRL_PE_EXT_ERR_BLOCK_SIZE_ERR: u32 = 0x7;
pub const EIP93_PE_CTRL_PE_EXT_ERR_INVALID_PK_LENGTH: u32 = 0x6;
pub const EIP93_PE_CTRL_PE_EXT_ERR_ZERO_LENGTH: u32 = 0x5;
pub const EIP93_PE_CTRL_PE_EXT_ERR_SPI: u32 = 0x4;
pub const EIP93_PE_CTRL_PE_EXT_ERR_INVALID_CRYPTO_ALGO: u32 = 0x3;
pub const EIP93_PE_CTRL_PE_EXT_ERR_INVALID_CRYPTO_OP: u32 = 0x2;
pub const EIP93_PE_CTRL_PE_EXT_ERR_DESC_OWNER: u32 = 0x1;
pub const EIP93_PE_CTRL_PE_EXT_ERR_BUS: u32 = 0x0;
pub const EIP93_PE_CTRL_PE_EXT_ERR: u32 = bit(19);
pub const EIP93_PE_CTRL_PE_SEQNUM_ERR: u32 = bit(18);
pub const EIP93_PE_CTRL_PE_PAD_ERR: u32 = bit(17);
pub const EIP93_PE_CTRL_PE_AUTH_ERR: u32 = bit(16);
pub const EIP93_PE_CTRL_PE_PAD_VALUE: u32 = genmask(15, 8);
pub const EIP93_PE_CTRL_PE_PRNG_MODE: u32 = genmask(7, 6);
pub const EIP93_PE_CTRL_PE_HASH_FINAL: u32 = bit(4);
pub const EIP93_PE_CTRL_PE_INIT_ARC4: u32 = bit(3);
pub const EIP93_PE_CTRL_PE_READY_DES_TRING_OWN: u32 = genmask(1, 0);
pub const EIP93_PE_CTRL_PE_READY: u32 = 0x2;
pub const EIP93_PE_CTRL_HOST_READY: u32 = 0x1;
pub const EIP93_REG_PE_SOURCE_ADDR: u32 = 0x4;
pub const EIP93_REG_PE_DEST_ADDR: u32 = 0x8;
pub const EIP93_REG_PE_SA_ADDR: u32 = 0xc;
pub const EIP93_REG_PE_ADDR: u32 = 0x10; // STATE_ADDR

/*
 * Special implementation for user ID
 * user_id in eip93_descriptor is used to identify the
 * descriptor and is opaque and can be used by the driver
 * in custom way.
 *
 * The usage of this should be to put an address to the crypto
 * request struct from the kernel but this can't work in 64bit
 * world.
 *
 * Also it's required to put some flags to identify the last
 * descriptor.
 *
 * To handle this, split the u32 in 2 part:
 * - 31:16 descriptor flags
 * - 15:0 IDR to connect the crypto request address
 */
pub const EIP93_REG_PE_USER_ID: u32 = 0x18;
pub const EIP93_PE_USER_ID_DESC_FLAGS: u32 = genmask(31, 16);
pub const EIP93_PE_USER_ID_CRYPTO_IDR: u32 = genmask(15, 0);
pub const EIP93_REG_PE_LENGTH: u32 = 0x1c;
pub const EIP93_PE_LENGTH_BYPASS: u32 = genmask(31, 24);
pub const EIP93_PE_LENGTH_HOST_PE_READY: u32 = genmask(23, 22);
pub const EIP93_PE_LENGTH_PE_READY: u32 = 0x2;
pub const EIP93_PE_LENGTH_HOST_READY: u32 = 0x1;
pub const EIP93_PE_LENGTH_LENGTH: u32 = genmask(19, 0);

/* PACKET ENGINE RING configuration registers */
pub const EIP93_REG_PE_CDR_BASE: u32 = 0x80;
pub const EIP93_REG_PE_RDR_BASE: u32 = 0x84;
pub const EIP93_REG_PE_RING_CONFIG: u32 = 0x88;
pub const EIP93_PE_EN_EXT_TRIG: u32 = bit(31);
/* Absent in later revision of eip93 */
// pub const EIP93_PE_RING_OFFSET: u32 = genmask(23, 15);
pub const EIP93_PE_RING_SIZE: u32 = genmask(9, 0);
pub const EIP93_REG_PE_RING_THRESH: u32 = 0x8c;
pub const EIPR93_PE_TIMEROUT_EN: u32 = bit(31);
pub const EIPR93_PE_RD_TIMEOUT: u32 = genmask(29, 26);
pub const EIPR93_PE_RDR_THRESH: u32 = genmask(25, 16);
pub const EIPR93_PE_CDR_THRESH: u32 = genmask(9, 0);
pub const EIP93_REG_PE_CD_COUNT: u32 = 0x90;
pub const EIP93_PE_CD_COUNT: u32 = genmask(10, 0);
/* In the same register, writing a value in GENMASK(7, 0) will
 * increment the descriptor count and start DMA action. */
pub const EIP93_PE_CD_COUNT_INCR: u32 = genmask(7, 0);
pub const EIP93_REG_PE_RD_COUNT: u32 = 0x94;
pub const EIP93_PE_RD_COUNT: u32 = genmask(10, 0);
/* In the same register, writing a value in GENMASK(7, 0) will
 * increment the descriptor count and start DMA action. */
pub const EIP93_PE_RD_COUNT_INCR: u32 = genmask(7, 0);
pub const EIP93_REG_PE_RING_RW_PNTR: u32 = 0x98; // RING_PNTR

/* PACKET ENGINE configuration registers */
pub const EIP93_REG_PE_CONFIG: u32 = 0x100;
pub const EIP93_PE_CONFIG_SWAP_TARGET: u32 = bit(20);
pub const EIP93_PE_CONFIG_SWAP_DATA: u32 = bit(18);
pub const EIP93_PE_CONFIG_SWAP_SA: u32 = bit(17);
pub const EIP93_PE_CONFIG_SWAP_CDRD: u32 = bit(16);
pub const EIP93_PE_CONFIG_EN_CDR_UPDATE: u32 = bit(10);
pub const EIP93_PE_CONFIG_PE_MODE: u32 = genmask(9, 8);
pub const EIP93_PE_TARGET_AUTO_RING_MODE: u32 = field_prep(EIP93_PE_CONFIG_PE_MODE, 0x3);
pub const EIP93_PE_TARGET_COMMAND_NO_RDR_MODE: u32 = field_prep(EIP93_PE_CONFIG_PE_MODE, 0x2);
pub const EIP93_PE_TARGET_COMMAND_WITH_RDR_MODE: u32 = field_prep(EIP93_PE_CONFIG_PE_MODE, 0x1);
pub const EIP93_PE_DIRECT_HOST_MODE: u32 = field_prep(EIP93_PE_CONFIG_PE_MODE, 0x0);
pub const EIP93_PE_CONFIG_RST_RING: u32 = bit(1);
pub const EIP93_PE_CONFIG_RST_PE: u32 = bit(0);
pub const EIP93_REG_PE_STATUS: u32 = 0x104;
pub const EIP93_REG_PE_BUF_THRESH: u32 = 0x10c;
pub const EIP93_PE_OUTBUF_THRESH: u32 = genmask(23, 16);
pub const EIP93_PE_INBUF_THRESH: u32 = genmask(7, 0);
pub const EIP93_REG_PE_INBUF_COUNT: u32 = 0x110;
pub const EIP93_REG_PE_OUTBUF_COUNT: u32 = 0x114;
pub const EIP93_REG_PE_BUF_RW_PNTR: u32 = 0x118; // BUF_PNTR

/* PACKET ENGINE endian config */
pub const EIP93_REG_PE_ENDIAN_CONFIG: u32 = 0x1cc;
pub const EIP93_AIROHA_REG_PE_ENDIAN_CONFIG: u32 = 0x1d0;
pub const EIP93_PE_ENDIAN_TARGET_BYTE_SWAP: u32 = genmask(23, 16);
pub const EIP93_PE_ENDIAN_MASTER_BYTE_SWAP: u32 = genmask(7, 0);
/* Byte goes 2 and 2 and are referenced by ID.
 * Split GENMASK(7, 0) in 4 part, one for each byte. */
pub const EIP93_PE_ENDIAN_BYTE0: u32 = 0x0;
pub const EIP93_PE_ENDIAN_BYTE1: u32 = 0x1;
pub const EIP93_PE_ENDIAN_BYTE2: u32 = 0x2;
pub const EIP93_PE_ENDIAN_BYTE3: u32 = 0x3;

/* EIP93 CLOCK control registers */
pub const EIP93_REG_PE_CLOCK_CTRL: u32 = 0x1e8;
pub const EIP93_PE_CLOCK_EN_HASH_CLK: u32 = bit(4);
pub const EIP93_PE_CLOCK_EN_ARC4_CLK: u32 = bit(3);
pub const EIP93_PE_CLOCK_EN_AES_CLK: u32 = bit(2);
pub const EIP93_PE_CLOCK_EN_DES_CLK: u32 = bit(1);
pub const EIP93_PE_CLOCK_EN_PE_CLK: u32 = bit(0);

/* EIP93 Device Option and Revision Register */
pub const EIP93_REG_PE_OPTION_1: u32 = 0x1f4;
pub const EIP93_PE_OPTION_MAC_KEY256: u32 = bit(31);
pub const EIP93_PE_OPTION_MAC_KEY192: u32 = bit(30);
pub const EIP93_PE_OPTION_MAC_KEY128: u32 = bit(29);
pub const EIP93_PE_OPTION_AES_CBC_MAC: u32 = bit(28);
pub const EIP93_PE_OPTION_AES_XCBX: u32 = bit(23);
pub const EIP93_PE_OPTION_SHA_256: u32 = bit(19);
pub const EIP93_PE_OPTION_SHA_224: u32 = bit(18);
pub const EIP93_PE_OPTION_SHA_1: u32 = bit(17);
pub const EIP93_PE_OPTION_MD5: u32 = bit(16);
pub const EIP93_PE_OPTION_AES_KEY256: u32 = bit(15);
pub const EIP93_PE_OPTION_AES_KEY192: u32 = bit(14);
pub const EIP93_PE_OPTION_AES_KEY128: u32 = bit(13);
pub const EIP93_PE_OPTION_AES: u32 = bit(2);
pub const EIP93_PE_OPTION_ARC4: u32 = bit(1);
pub const EIP93_PE_OPTION_TDES: u32 = bit(0); // DES and TDES
pub const EIP93_REG_PE_OPTION_0: u32 = 0x1f8;
pub const EIP93_REG_PE_REVISION: u32 = 0x1fc;
pub const EIP93_PE_REVISION_MAJ_HW_REV: u32 = genmask(27, 24);
pub const EIP93_PE_REVISION_MIN_HW_REV: u32 = genmask(23, 20);
pub const EIP93_PE_REVISION_HW_PATCH: u32 = genmask(19, 16);
pub const EIP93_PE_REVISION_EIP_NO: u32 = genmask(7, 0);

/* EIP93 Interrupt Control Register */
pub const EIP93_REG_INT_UNMASK_STAT: u32 = 0x200;
pub const EIP93_REG_INT_MASK_STAT: u32 = 0x204;
pub const EIP93_REG_INT_CLR: u32 = 0x204;
pub const EIP93_REG_INT_MASK: u32 = 0x208; // INT_EN
pub const EIP93_INT_INTERFACE_ERR: u32 = bit(18);
pub const EIP93_INT_RPOC_ERR: u32 = bit(17);
pub const EIP93_INT_PE_RING_ERR: u32 = bit(16);
pub const EIP93_INT_HALT: u32 = bit(15);
pub const EIP93_INT_OUTBUF_THRESH: u32 = bit(11);
pub const EIP93_INT_INBUF_THRESH: u32 = bit(10);
pub const EIP93_INT_OPERATION_DONE: u32 = bit(9);
pub const EIP93_INT_RDR_THRESH: u32 = bit(1);
pub const EIP93_INT_CDR_THRESH: u32 = bit(0);
pub const EIP93_INT_ALL: u32 = EIP93_INT_INTERFACE_ERR | EIP93_INT_RPOC_ERR |
    EIP93_INT_PE_RING_ERR | EIP93_INT_HALT | EIP93_INT_OUTBUF_THRESH |
    EIP93_INT_INBUF_THRESH | EIP93_INT_OPERATION_DONE | EIP93_INT_RDR_THRESH |
    EIP93_INT_CDR_THRESH;
pub const EIP93_REG_INT_CFG: u32 = 0x20c;
pub const EIP93_INT_TYPE_PULSE: u32 = bit(0);
pub const EIP93_REG_MASK_ENABLE: u32 = 0x210;
pub const EIP93_REG_MASK_DISABLE: u32 = 0x214;

/* EIP93 SA Record register */
pub const EIP93_REG_SA_CMD_0: u32 = 0x400;
pub const EIP93_SA_CMD_SAVE_HASH: u32 = bit(29);
pub const EIP93_SA_CMD_SAVE_IV: u32 = bit(28);
pub const EIP93_SA_CMD_HASH_SOURCE: u32 = genmask(27, 26);
pub const EIP93_SA_CMD_HASH_NO_LOAD: u32 = field_prep(EIP93_SA_CMD_HASH_SOURCE, 0x3);
pub const EIP93_SA_CMD_HASH_FROM_STATE: u32 = field_prep(EIP93_SA_CMD_HASH_SOURCE, 0x2);
pub const EIP93_SA_CMD_HASH_FROM_SA: u32 = field_prep(EIP93_SA_CMD_HASH_SOURCE, 0x0);
pub const EIP93_SA_CMD_IV_SOURCE: u32 = genmask(25, 24);
pub const EIP93_SA_CMD_IV_FROM_PRNG: u32 = field_prep(EIP93_SA_CMD_IV_SOURCE, 0x3);
pub const EIP93_SA_CMD_IV_FROM_STATE: u32 = field_prep(EIP93_SA_CMD_IV_SOURCE, 0x2);
pub const EIP93_SA_CMD_IV_FROM_INPUT: u32 = field_prep(EIP93_SA_CMD_IV_SOURCE, 0x1);
pub const EIP93_SA_CMD_IV_NO_LOAD: u32 = field_prep(EIP93_SA_CMD_IV_SOURCE, 0x0);
pub const EIP93_SA_CMD_DIGEST_LENGTH: u32 = genmask(23, 20);
pub const EIP93_SA_CMD_DIGEST_10WORD: u32 = field_prep(EIP93_SA_CMD_DIGEST_LENGTH, 0xa); // SRTP and TLS
pub const EIP93_SA_CMD_DIGEST_8WORD: u32 = field_prep(EIP93_SA_CMD_DIGEST_LENGTH, 0x8); // SHA-256
pub const EIP93_SA_CMD_DIGEST_7WORD: u32 = field_prep(EIP93_SA_CMD_DIGEST_LENGTH, 0x7); // SHA-224
pub const EIP93_SA_CMD_DIGEST_6WORD: u32 = field_prep(EIP93_SA_CMD_DIGEST_LENGTH, 0x6);
pub const EIP93_SA_CMD_DIGEST_5WORD: u32 = field_prep(EIP93_SA_CMD_DIGEST_LENGTH, 0x5); // SHA1
pub const EIP93_SA_CMD_DIGEST_4WORD: u32 = field_prep(EIP93_SA_CMD_DIGEST_LENGTH, 0x4); // MD5 and AES-based
pub const EIP93_SA_CMD_DIGEST_3WORD_IPSEC: u32 = field_prep(EIP93_SA_CMD_DIGEST_LENGTH, 0x3); // IPSEC
pub const EIP93_SA_CMD_DIGEST_2WORD: u32 = field_prep(EIP93_SA_CMD_DIGEST_LENGTH, 0x2);
pub const EIP93_SA_CMD_DIGEST_1WORD: u32 = field_prep(EIP93_SA_CMD_DIGEST_LENGTH, 0x1);
pub const EIP93_SA_CMD_DIGEST_3WORD: u32 = field_prep(EIP93_SA_CMD_DIGEST_LENGTH, 0x0); // 96bit output
pub const EIP93_SA_CMD_HDR_PROC: u32 = bit(19);
pub const EIP93_SA_CMD_EXT_PAD: u32 = bit(18);
pub const EIP93_SA_CMD_SCPAD: u32 = bit(17);
pub const EIP93_SA_CMD_HASH: u32 = genmask(15, 12);
pub const EIP93_SA_CMD_HASH_NULL: u32 = field_prep(EIP93_SA_CMD_HASH, 0xf);
pub const EIP93_SA_CMD_HASH_SHA256: u32 = field_prep(EIP93_SA_CMD_HASH, 0x3);
pub const EIP93_SA_CMD_HASH_SHA224: u32 = field_prep(EIP93_SA_CMD_HASH, 0x2);
pub const EIP93_SA_CMD_HASH_SHA1: u32 = field_prep(EIP93_SA_CMD_HASH, 0x1);
pub const EIP93_SA_CMD_HASH_MD5: u32 = field_prep(EIP93_SA_CMD_HASH, 0x0);
pub const EIP93_SA_CMD_CIPHER: u32 = genmask(11, 8);
pub const EIP93_SA_CMD_CIPHER_NULL: u32 = field_prep(EIP93_SA_CMD_CIPHER, 0xf);
pub const EIP93_SA_CMD_CIPHER_AES: u32 = field_prep(EIP93_SA_CMD_CIPHER, 0x3);
pub const EIP93_SA_CMD_CIPHER_ARC4: u32 = field_prep(EIP93_SA_CMD_CIPHER, 0x2);
pub const EIP93_SA_CMD_CIPHER_3DES: u32 = field_prep(EIP93_SA_CMD_CIPHER, 0x1);
pub const EIP93_SA_CMD_CIPHER_DES: u32 = field_prep(EIP93_SA_CMD_CIPHER, 0x0);
pub const EIP93_SA_CMD_PAD_TYPE: u32 = genmask(7, 6);
pub const EIP93_SA_CMD_PAD_CONST_SSL: u32 = field_prep(EIP93_SA_CMD_PAD_TYPE, 0x6);
pub const EIP93_SA_CMD_PAD_TLS_DTLS: u32 = field_prep(EIP93_SA_CMD_PAD_TYPE, 0x5);
pub const EIP93_SA_CMD_PAD_ZERO: u32 = field_prep(EIP93_SA_CMD_PAD_TYPE, 0x3);
pub const EIP93_SA_CMD_PAD_CONST: u32 = field_prep(EIP93_SA_CMD_PAD_TYPE, 0x2);
pub const EIP93_SA_CMD_PAD_PKCS7: u32 = field_prep(EIP93_SA_CMD_PAD_TYPE, 0x1);
pub const EIP93_SA_CMD_PAD_IPSEC: u32 = field_prep(EIP93_SA_CMD_PAD_TYPE, 0x0);
pub const EIP93_SA_CMD_OPGROUP: u32 = genmask(5, 4);
pub const EIP93_SA_CMD_OP_EXT: u32 = field_prep(EIP93_SA_CMD_OPGROUP, 0x2);
pub const EIP93_SA_CMD_OP_PROTOCOL: u32 = field_prep(EIP93_SA_CMD_OPGROUP, 0x1);
pub const EIP93_SA_CMD_OP_BASIC: u32 = field_prep(EIP93_SA_CMD_OPGROUP, 0x0);
pub const EIP93_SA_CMD_DIRECTION_IN: u32 = bit(3); // 0: outbount 1: inbound
pub const EIP93_SA_CMD_OPCODE: u32 = genmask(2, 0);
pub const EIP93_SA_CMD_OPCODE_BASIC_OUT_PRNG: u32 = 0x7;
pub const EIP93_SA_CMD_OPCODE_BASIC_OUT_HASH: u32 = 0x3;
pub const EIP93_SA_CMD_OPCODE_BASIC_OUT_ENC_HASH: u32 = 0x1;
pub const EIP93_SA_CMD_OPCODE_BASIC_OUT_ENC: u32 = 0x0;
pub const EIP93_SA_CMD_OPCODE_BASIC_IN_HASH: u32 = 0x3;
pub const EIP93_SA_CMD_OPCODE_BASIC_IN_HASH_DEC: u32 = 0x1;
pub const EIP93_SA_CMD_OPCODE_BASIC_IN_DEC: u32 = 0x0;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_OUT_ESP: u32 = 0x0;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_OUT_SSL: u32 = 0x4;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_OUT_TLS: u32 = 0x5;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_OUT_SRTP: u32 = 0x7;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_IN_ESP: u32 = 0x0;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_IN_SSL: u32 = 0x2;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_IN_TLS: u32 = 0x3;
pub const EIP93_SA_CMD_OPCODE_PROTOCOL_IN_SRTP: u32 = 0x7;
pub const EIP93_SA_CMD_OPCODE_EXT_OUT_DTSL: u32 = 0x1;
pub const EIP93_SA_CMD_OPCODE_EXT_OUT_SSL: u32 = 0x4;
pub const EIP93_SA_CMD_OPCODE_EXT_OUT_TLSV10: u32 = 0x5;
pub const EIP93_SA_CMD_OPCODE_EXT_OUT_TLSV11: u32 = 0x6;
pub const EIP93_SA_CMD_OPCODE_EXT_IN_DTSL: u32 = 0x1;
pub const EIP93_SA_CMD_OPCODE_EXT_IN_SSL: u32 = 0x4;
pub const EIP93_SA_CMD_OPCODE_EXT_IN_TLSV10: u32 = 0x5;
pub const EIP93_SA_CMD_OPCODE_EXT_IN_TLSV11: u32 = 0x6;
pub const EIP93_REG_SA_CMD_1: u32 = 0x404;
pub const EIP93_SA_CMD_EN_SEQNUM_CHK: u32 = bit(29);
pub const EIP93_SA_CMD_ARC4_KEY_LENGHT: u32 = genmask(28, 24);
pub const EIP93_SA_CMD_AES_DEC_KEY: u32 = bit(28); // 0: encrypt key 1: decrypt key
pub const EIP93_SA_CMD_AES_KEY_LENGTH: u32 = genmask(26, 24);
pub const EIP93_SA_CMD_AES_KEY_256BIT: u32 = field_prep(EIP93_SA_CMD_AES_KEY_LENGTH, 0x4);
pub const EIP93_SA_CMD_AES_KEY_192BIT: u32 = field_prep(EIP93_SA_CMD_AES_KEY_LENGTH, 0x3);
pub const EIP93_SA_CMD_AES_KEY_128BIT: u32 = field_prep(EIP93_SA_CMD_AES_KEY_LENGTH, 0x2);
pub const EIP93_SA_CMD_HASH_CRYPT_OFFSET: u32 = genmask(23, 16);
pub const EIP93_SA_CMD_BYTE_OFFSET: u32 = bit(13); // 0: CRYPT_OFFSET in 32bit word 1: CRYPT_OFFSET in 8bit bytes
pub const EIP93_SA_CMD_HMAC: u32 = bit(12);
pub const EIP93_SA_CMD_SSL_MAC: u32 = bit(12);
pub const EIP93_SA_CMD_CHIPER_MODE: u32 = genmask(9, 8);
pub const EIP93_SA_CMD_CHIPER_MODE_ICM: u32 = field_prep(EIP93_SA_CMD_CHIPER_MODE, 0x3);
pub const EIP93_SA_CMD_CHIPER_MODE_CTR: u32 = field_prep(EIP93_SA_CMD_CHIPER_MODE, 0x2);
pub const EIP93_SA_CMD_CHIPER_MODE_CBC: u32 = field_prep(EIP93_SA_CMD_CHIPER_MODE, 0x1);
pub const EIP93_SA_CMD_CHIPER_MODE_ECB: u32 = field_prep(EIP93_SA_CMD_CHIPER_MODE, 0x0);
pub const EIP93_SA_CMD_CHIPER_MODE_STATEFULL: u32 = field_prep(EIP93_SA_CMD_CHIPER_MODE, 0x1);
pub const EIP93_SA_CMD_CHIPER_MODE_STATELESS: u32 = field_prep(EIP93_SA_CMD_CHIPER_MODE, 0x0);
pub const EIP93_SA_CMD_COPY_PAD: u32 = bit(3);
pub const EIP93_SA_CMD_COPY_PAYLOAD: u32 = bit(2);
pub const EIP93_SA_CMD_COPY_HEADER: u32 = bit(1);
pub const EIP93_SA_CMD_COPY_DIGEST: u32 = bit(0); // With this enabled, COPY_PAD is required

/* State save register */
pub const EIP93_REG_STATE_IV_0: u32 = 0x500;
pub const EIP93_REG_STATE_IV_1: u32 = 0x504;
pub const EIP93_REG_PE_ARC4STATE: u32 = 0x700;

#[repr(C, packed)]
pub struct sa_record {
    pub sa_cmd0_word: u32,
    pub sa_cmd1_word: u32,
    pub sa_key: [u32; 8],
    pub sa_i_digest: [u8; 32],
    pub sa_o_digest: [u8; 32],
    pub sa_spi: u32,
    pub sa_seqnum: [u32; 2],
    pub sa_seqmum_mask: [u32; 2],
    pub sa_nonce: u32,
}

#[repr(C, packed)]
pub struct sa_state {
    pub state_iv: [u32; 4],
    pub state_byte_cnt: [u32; 2],
    pub state_i_digest: [u8; 32],
}

#[repr(C, packed)]
pub struct eip93_descriptor {
    pub pe_ctrl_stat_word: u32,
    pub src_addr: u32,
    pub dst_addr: u32,
    pub sa_addr: u32,
    pub state_addr: u32,
    pub arc4_addr: u32,
    pub user_id: u32,
    pub pe_length_word: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
