/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2021 sigma star gmbh
 *
 * Specifies paes key slot handles for NXP's DCP (Data Co-Processor) to be used
 * with the crypto_skcipher_setkey().
 */

pub const DCP_PAES_KEYSIZE: i32 = 1;
pub const DCP_PAES_KEY_SLOT0: i32 = 0x00;
pub const DCP_PAES_KEY_SLOT1: i32 = 0x01;
pub const DCP_PAES_KEY_SLOT2: i32 = 0x02;
pub const DCP_PAES_KEY_SLOT3: i32 = 0x03;
pub const DCP_PAES_KEY_UNIQUE: i32 = 0xfe;
pub const DCP_PAES_KEY_OTP: i32 = 0xff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
