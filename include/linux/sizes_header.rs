/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/linux/sizes.h
 */

// Dependency intent from <linux/const.h>: _AC(value, ULL) denotes an
// unsigned long long constant.

pub const SZ_1: u64 = 0x00000001;
pub const SZ_2: u64 = 0x00000002;
pub const SZ_4: u64 = 0x00000004;
pub const SZ_8: u64 = 0x00000008;
pub const SZ_16: u64 = 0x00000010;
pub const SZ_32: u64 = 0x00000020;
pub const SZ_64: u64 = 0x00000040;
pub const SZ_128: u64 = 0x00000080;
pub const SZ_256: u64 = 0x00000100;
pub const SZ_512: u64 = 0x00000200;

pub const SZ_1K: u64 = 0x00000400;
pub const SZ_2K: u64 = 0x00000800;
pub const SZ_4K: u64 = 0x00001000;
pub const SZ_8K: u64 = 0x00002000;
pub const SZ_16K: u64 = 0x00004000;
pub const SZ_24K: u64 = 0x00006000;
pub const SZ_32K: u64 = 0x00008000;
pub const SZ_64K: u64 = 0x00010000;
pub const SZ_128K: u64 = 0x00020000;
pub const SZ_192K: u64 = 0x00030000;
pub const SZ_256K: u64 = 0x00040000;
pub const SZ_384K: u64 = 0x00060000;
pub const SZ_512K: u64 = 0x00080000;

pub const SZ_1M: u64 = 0x00100000;
pub const SZ_2M: u64 = 0x00200000;
pub const SZ_3M: u64 = 0x00300000;
pub const SZ_4M: u64 = 0x00400000;
pub const SZ_6M: u64 = 0x00600000;
pub const SZ_8M: u64 = 0x00800000;
pub const SZ_12M: u64 = 0x00c00000;
pub const SZ_16M: u64 = 0x01000000;
pub const SZ_18M: u64 = 0x01200000;
pub const SZ_24M: u64 = 0x01800000;
pub const SZ_32M: u64 = 0x02000000;
pub const SZ_64M: u64 = 0x04000000;
pub const SZ_128M: u64 = 0x08000000;
pub const SZ_256M: u64 = 0x10000000;
pub const SZ_512M: u64 = 0x20000000;

pub const SZ_1G: u64 = 0x40000000;
pub const SZ_2G: u64 = 0x80000000;

pub const SZ_4G: u64 = 0x100000000;
pub const SZ_8G: u64 = 0x200000000;
pub const SZ_16G: u64 = 0x400000000;
pub const SZ_32G: u64 = 0x800000000;
pub const SZ_64G: u64 = 0x1000000000;
pub const SZ_128G: u64 = 0x2000000000;
pub const SZ_256G: u64 = 0x4000000000;
pub const SZ_512G: u64 = 0x8000000000;

pub const SZ_1T: u64 = 0x10000000000;
pub const SZ_2T: u64 = 0x20000000000;
pub const SZ_4T: u64 = 0x40000000000;
pub const SZ_8T: u64 = 0x80000000000;
pub const SZ_16T: u64 = 0x100000000000;
pub const SZ_32T: u64 = 0x200000000000;
pub const SZ_64T: u64 = 0x400000000000;
pub const SZ_128T: u64 = 0x800000000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
