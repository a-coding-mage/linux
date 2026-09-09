/*
 * Copyright (C) 2018  Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

pub const MAX_INSTANCE: usize = 5;
pub const MAX_SEGMENT: usize = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IP_BASE_INSTANCE {
    pub segment: [u32; MAX_SEGMENT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IP_BASE {
    pub instance: [IP_BASE_INSTANCE; MAX_INSTANCE],
}

const Z: IP_BASE_INSTANCE = IP_BASE_INSTANCE { segment: [0; MAX_SEGMENT] };
const fn base(a: [u32; MAX_SEGMENT], b: [u32; MAX_SEGMENT], c: [u32; MAX_SEGMENT], d: [u32; MAX_SEGMENT], e: [u32; MAX_SEGMENT]) -> IP_BASE {
    IP_BASE { instance: [IP_BASE_INSTANCE { segment: a }, IP_BASE_INSTANCE { segment: b }, IP_BASE_INSTANCE { segment: c }, IP_BASE_INSTANCE { segment: d }, IP_BASE_INSTANCE { segment: e }] }
}

pub const NBIF_BASE: IP_BASE = base([0x00000000, 0x00000014, 0x00000D20, 0x00010400, 0], [0;5], [0;5], [0;5], [0;5]);
pub const NBIO_BASE: IP_BASE = NBIF_BASE;
pub const DCE_BASE: IP_BASE = base([0x12, 0xC0, 0x34C0, 0, 0], [0;5], [0;5], [0;5], [0;5]);
pub const DCN_BASE: IP_BASE = DCE_BASE;
pub const MP0_BASE: IP_BASE = base([0x16000,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const MP1_BASE: IP_BASE = base([0x16200,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const MP2_BASE: IP_BASE = base([0x16400,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const DF_BASE: IP_BASE = base([0x7000,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const UVD_BASE: IP_BASE = base([0x7800,0x7E00,0,0,0], [0;5], [0;5], [0;5], [0;5]); // note: GLN does not use the first segment
pub const VCN_BASE: IP_BASE = UVD_BASE; // note: GLN does not use the first segment
pub const DBGU_BASE: IP_BASE = base([0x180,0x1A0,0,0,0], [0;5], [0;5], [0;5], [0;5]); // not exist
pub const DBGU_NBIO_BASE: IP_BASE = base([0x1C0,0,0,0,0], [0;5], [0;5], [0;5], [0;5]); // not exist
pub const DBGU_IO_BASE: IP_BASE = base([0x1E0,0,0,0,0], [0;5], [0;5], [0;5], [0;5]); // not exist
pub const DFX_DAP_BASE: IP_BASE = base([0x5A0,0,0,0,0], [0;5], [0;5], [0;5], [0;5]); // not exist
pub const DFX_BASE: IP_BASE = base([0x580,0,0,0,0], [0;5], [0;5], [0;5], [0;5]); // this file does not contain registers
pub const ISP_BASE: IP_BASE = base([0x18000,0,0,0,0], [0;5], [0;5], [0;5], [0;5]); // not exist
pub const SYSTEMHUB_BASE: IP_BASE = base([0xEA0,0,0,0,0], [0;5], [0;5], [0;5], [0;5]); // not exist
pub const L2IMU_BASE: IP_BASE = base([0x7DC0,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const IOHC_BASE: IP_BASE = base([0x10000,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const ATHUB_BASE: IP_BASE = base([0xC20,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const VCE_BASE: IP_BASE = base([0x7E00,0x48800,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const GC_BASE: IP_BASE = base([0x2000,0xA000,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const MMHUB_BASE: IP_BASE = base([0x1A000,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const RSMU_BASE: IP_BASE = base([0x12000,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const HDP_BASE: IP_BASE = base([0xF20,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const OSSSYS_BASE: IP_BASE = base([0x10A0,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const SDMA0_BASE: IP_BASE = base([0x1260,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const SDMA1_BASE: IP_BASE = base([0x1460,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const XDMA_BASE: IP_BASE = base([0x3400,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const UMC_BASE: IP_BASE = base([0x14000,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const THM_BASE: IP_BASE = base([0x16600,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const SMUIO_BASE: IP_BASE = base([0x16800,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const PWR_BASE: IP_BASE = base([0x16A00,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);
pub const CLK_BASE: IP_BASE = base([0x16C00,0,0,0,0], [0x16E00,0,0,0,0], [0x17000,0,0,0,0], [0x17200,0,0,0,0], [0x17E00,0,0,0,0]);
pub const FUSE_BASE: IP_BASE = base([0x17400,0,0,0,0], [0;5], [0;5], [0;5], [0;5]);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
