// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2021-3 ARM Limited.

pub const SVCR_SM_SHIFT: u32 = 0;
pub const SVCR_ZA_SHIFT: u32 = 1;

pub const SVCR_SM: u32 = 1u32 << SVCR_SM_SHIFT;
pub const SVCR_ZA: u32 = 1u32 << SVCR_ZA_SHIFT;

pub const HAVE_SVE_SHIFT: u32 = 0;
pub const HAVE_SME_SHIFT: u32 = 1;
pub const HAVE_SME2_SHIFT: u32 = 2;
pub const HAVE_FA64_SHIFT: u32 = 3;
pub const HAVE_FPMR_SHIFT: u32 = 4;

pub const HAVE_SVE: u32 = 1u32 << HAVE_SVE_SHIFT;
pub const HAVE_SME: u32 = 1u32 << HAVE_SME_SHIFT;
pub const HAVE_SME2: u32 = 1u32 << HAVE_SME2_SHIFT;
pub const HAVE_FA64: u32 = 1u32 << HAVE_FA64_SHIFT;
pub const HAVE_FPMR: u32 = 1u32 << HAVE_FPMR_SHIFT;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
