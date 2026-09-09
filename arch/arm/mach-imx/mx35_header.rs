/* SPDX-License-Identifier: GPL-2.0 */

pub const MX35_AIPS1_BASE_ADDR: u32 = 0x43f00000;
pub const MX35_AIPS1_SIZE: u32 = SZ_1M;
pub const MX35_SPBA0_BASE_ADDR: u32 = 0x50000000;
pub const MX35_SPBA0_SIZE: u32 = SZ_1M;
pub const MX35_AIPS2_BASE_ADDR: u32 = 0x53f00000;
pub const MX35_AIPS2_SIZE: u32 = SZ_1M;
pub const MX35_AVIC_BASE_ADDR: u32 = 0x68000000;
pub const MX35_AVIC_SIZE: u32 = SZ_1M;
pub const MX35_X_MEMC_BASE_ADDR: u32 = 0xb8000000;
pub const MX35_X_MEMC_SIZE: u32 = SZ_64K;

macro_rules! MX35_IO_P2V {
    ($x:expr) => {
        IMX_IO_P2V!($x)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
