// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Flattened device-tree wire-format constants, independent of host layout.
// Copyright (C) 2006 David Gibson, IBM Corporation.
// Copyright 2012 Kim Phillips, Freescale Semiconductor.

pub(crate) const FDT_MAGIC: u32 = 0xd00dfeed;
pub(crate) const FDT_SW_MAGIC: u32 = !FDT_MAGIC;
pub(crate) const FDT_TAGSIZE: usize = 4;
pub(crate) const FDT_BEGIN_NODE: u32 = 1;
pub(crate) const FDT_END_NODE: u32 = 2;
pub(crate) const FDT_PROP: u32 = 3;
pub(crate) const FDT_NOP: u32 = 4;
pub(crate) const FDT_END: u32 = 9;
pub(crate) const FDT_V1_SIZE: usize = 28;
pub(crate) const FDT_V2_SIZE: usize = 32;
pub(crate) const FDT_V3_SIZE: usize = 36;
pub(crate) const FDT_V16_SIZE: usize = 36;
pub(crate) const FDT_V17_SIZE: usize = 40;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
