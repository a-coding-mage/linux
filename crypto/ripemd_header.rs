/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common values for RIPEMD algorithms
 */

pub const RMD160_DIGEST_SIZE: usize = 20;
pub const RMD160_BLOCK_SIZE: usize = 64;

/* initial values  */
pub const RMD_H0: u32 = 0x67452301;
pub const RMD_H1: u32 = 0xefcdab89;
pub const RMD_H2: u32 = 0x98badcfe;
pub const RMD_H3: u32 = 0x10325476;
pub const RMD_H4: u32 = 0xc3d2e1f0;

/* constants */
pub const RMD_K1: u32 = 0x00000000;
pub const RMD_K2: u32 = 0x5a827999;
pub const RMD_K3: u32 = 0x6ed9eba1;
pub const RMD_K4: u32 = 0x8f1bbcdc;
pub const RMD_K5: u32 = 0xa953fd4e;
pub const RMD_K6: u32 = 0x50a28be6;
pub const RMD_K7: u32 = 0x5c4dd124;
pub const RMD_K8: u32 = 0x6d703ef3;
pub const RMD_K9: u32 = 0x7a6d76e9;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
