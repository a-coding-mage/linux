/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Common registers for PPC AES implementation
 *
 * Copyright (c) 2015 Markus Stockhausen <stockhausen@collogia.de>
 */

// Register aliases from the original PowerPC assembly implementation.
pub const rKS: u32 = 0; /* copy of en-/decryption key pointer */
pub const rDP: u32 = 3; /* destination pointer */
pub const rSP: u32 = 4; /* source pointer */
pub const rKP: u32 = 5; /* pointer to en-/decryption key pointer */
pub const rRR: u32 = 6; /* en-/decryption rounds */
pub const rLN: u32 = 7; /* length of data to be processed */
pub const rIP: u32 = 8; /* potiner to IV (CBC/CTR/XTS modes) */
pub const rKT: u32 = 9; /* pointer to tweak key (XTS mode) */
pub const rT0: u32 = 11; /* pointers to en-/decryption tables */
pub const rT1: u32 = 10;
pub const rD0: u32 = 9; /* data */
pub const rD1: u32 = 14;
pub const rD2: u32 = 12;
pub const rD3: u32 = 15;
pub const rW0: u32 = 16; /* working registers */
pub const rW1: u32 = 17;
pub const rW2: u32 = 18;
pub const rW3: u32 = 19;
pub const rW4: u32 = 20;
pub const rW5: u32 = 21;
pub const rW6: u32 = 22;
pub const rW7: u32 = 23;
pub const rI0: u32 = 24; /* IV */
pub const rI1: u32 = 25;
pub const rI2: u32 = 26;
pub const rI3: u32 = 27;
pub const rG0: u32 = 28; /* endian reversed tweak (XTS mode) */
pub const rG1: u32 = 29;
pub const rG2: u32 = 30;
pub const rG3: u32 = 31;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
