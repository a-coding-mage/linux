/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Advanced Micro Devices, Inc.
 *
 * Author: Steven Kinney <Steven.Kinney@amd.com>
 * Author: Suravee Suthikulpanit <Suraveee.Suthikulpanit@amd.com>
 */

/* iommu pc mmio region register indexes */
pub const IOMMU_PC_COUNTER_REG: u64 = 0x00;
pub const IOMMU_PC_COUNTER_SRC_REG: u64 = 0x08;
pub const IOMMU_PC_PASID_MATCH_REG: u64 = 0x10;
pub const IOMMU_PC_DOMID_MATCH_REG: u64 = 0x18;
pub const IOMMU_PC_DEVID_MATCH_REG: u64 = 0x20;
pub const IOMMU_PC_COUNTER_REPORT_REG: u64 = 0x28;

/* maximum specified bank/counters */
pub const PC_MAX_SPEC_BNKS: u64 = 64;
pub const PC_MAX_SPEC_CNTRS: u64 = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
