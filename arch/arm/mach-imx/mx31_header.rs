/* SPDX-License-Identifier: GPL-2.0 */

// `SZ_1M`, `SZ_64K`, and `IMX_IO_P2V` are supplied by the corresponding
// dependencies in the translated tree.

pub const MX31_AIPS1_BASE_ADDR: u32 = 0x43f00000;
pub const MX31_AIPS1_SIZE: _ = SZ_1M;
pub const MX31_SPBA0_BASE_ADDR: u32 = 0x50000000;
pub const MX31_SPBA0_SIZE: _ = SZ_1M;
pub const MX31_AIPS2_BASE_ADDR: u32 = 0x53f00000;
pub const MX31_AIPS2_SIZE: _ = SZ_1M;
pub const MX31_AVIC_BASE_ADDR: u32 = 0x68000000;
pub const MX31_AVIC_SIZE: _ = SZ_1M;
pub const MX31_X_MEMC_BASE_ADDR: u32 = 0xb8000000;
pub const MX31_X_MEMC_SIZE: _ = SZ_64K;

#[macro_export]
macro_rules! MX31_IO_P2V {
    ($x:expr) => {
        IMX_IO_P2V!($x)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
