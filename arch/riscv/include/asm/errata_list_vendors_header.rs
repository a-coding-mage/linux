/* SPDX-License-Identifier: GPL-2.0-only */

// The following conditional declarations preserve the original build-time
// CONFIG_ERRATA_* conditions.

#[cfg(CONFIG_ERRATA_ANDES)]
pub const ERRATA_ANDES_NO_IOCP: u32 = 0;
#[cfg(CONFIG_ERRATA_ANDES)]
pub const ERRATA_ANDES_NUMBER: u32 = 1;

#[cfg(CONFIG_ERRATA_SIFIVE)]
pub const ERRATA_SIFIVE_CIP_453: u32 = 0;
#[cfg(CONFIG_ERRATA_SIFIVE)]
pub const ERRATA_SIFIVE_CIP_1200: u32 = 1;
#[cfg(CONFIG_ERRATA_SIFIVE)]
pub const ERRATA_SIFIVE_NUMBER: u32 = 2;

#[cfg(CONFIG_ERRATA_THEAD)]
pub const ERRATA_THEAD_MAE: u32 = 0;
#[cfg(CONFIG_ERRATA_THEAD)]
pub const ERRATA_THEAD_PMU: u32 = 1;
#[cfg(CONFIG_ERRATA_THEAD)]
pub const ERRATA_THEAD_GHOSTWRITE: u32 = 2;
#[cfg(CONFIG_ERRATA_THEAD)]
pub const ERRATA_THEAD_NUMBER: u32 = 3;

#[cfg(CONFIG_ERRATA_MIPS)]
pub const ERRATA_MIPS_P8700_PAUSE_OPCODE: u32 = 0;
#[cfg(CONFIG_ERRATA_MIPS)]
pub const ERRATA_MIPS_NUMBER: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
