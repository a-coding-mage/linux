/*
 * Copyright (C) 2019  Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
pub const _renoir_ip_offset_HEADER: u32 = ;
pub const MAX_INSTANCE: usize = 7;
pub const MAX_SEGMENT: usize = 5;
#[repr(C)]
pub struct IP_BASE_INSTANCE {
    pub segment: [u32; MAX_SEGMENT],
}

#[repr(C)]
pub struct IP_BASE {
    pub instance: [[u32; MAX_SEGMENT]; MAX_INSTANCE],
}

pub const ACP_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const ATHUB_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const CLK_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const DBGU_IO0_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const DF_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const DIO_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const DMU_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const DPCS_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const FUSE_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const GC_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const HDA_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const HDP_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const IOHC0_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const ISP_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const L2IMU0_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const MMHUB_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const MP0_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const MP1_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const NBIF0_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const DCN_BASE  : IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const OSSSYS_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const PCIE0_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const SDMA0_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const SMUIO_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const THM_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const UMC_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const USB0_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const UVD0_BASE: IP_BASE = IP_BASE {
    instance: [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ],
};

pub const ACP_BASE__INST0_SEG0: u32 = 0x02403800;
pub const ACP_BASE__INST0_SEG1: u32 = 0x00480000;
pub const ACP_BASE__INST0_SEG2: u32 = 0;
pub const ACP_BASE__INST0_SEG3: u32 = 0;
pub const ACP_BASE__INST0_SEG4: u32 = 0;
pub const ACP_BASE__INST1_SEG0: u32 = 0;
pub const ACP_BASE__INST1_SEG1: u32 = 0;
pub const ACP_BASE__INST1_SEG2: u32 = 0;
pub const ACP_BASE__INST1_SEG3: u32 = 0;
pub const ACP_BASE__INST1_SEG4: u32 = 0;
pub const ACP_BASE__INST2_SEG0: u32 = 0;
pub const ACP_BASE__INST2_SEG1: u32 = 0;
pub const ACP_BASE__INST2_SEG2: u32 = 0;
pub const ACP_BASE__INST2_SEG3: u32 = 0;
pub const ACP_BASE__INST2_SEG4: u32 = 0;
pub const ACP_BASE__INST3_SEG0: u32 = 0;
pub const ACP_BASE__INST3_SEG1: u32 = 0;
pub const ACP_BASE__INST3_SEG2: u32 = 0;
pub const ACP_BASE__INST3_SEG3: u32 = 0;
pub const ACP_BASE__INST3_SEG4: u32 = 0;
pub const ACP_BASE__INST4_SEG0: u32 = 0;
pub const ACP_BASE__INST4_SEG1: u32 = 0;
pub const ACP_BASE__INST4_SEG2: u32 = 0;
pub const ACP_BASE__INST4_SEG3: u32 = 0;
pub const ACP_BASE__INST4_SEG4: u32 = 0;
pub const ACP_BASE__INST5_SEG0: u32 = 0;
pub const ACP_BASE__INST5_SEG1: u32 = 0;
pub const ACP_BASE__INST5_SEG2: u32 = 0;
pub const ACP_BASE__INST5_SEG3: u32 = 0;
pub const ACP_BASE__INST5_SEG4: u32 = 0;
pub const ACP_BASE__INST6_SEG0: u32 = 0;
pub const ACP_BASE__INST6_SEG1: u32 = 0;
pub const ACP_BASE__INST6_SEG2: u32 = 0;
pub const ACP_BASE__INST6_SEG3: u32 = 0;
pub const ACP_BASE__INST6_SEG4: u32 = 0;
pub const ATHUB_BASE__INST0_SEG0: u32 = 0x00000C20;
pub const ATHUB_BASE__INST0_SEG1: u32 = 0x02408C00;
pub const ATHUB_BASE__INST0_SEG2: u32 = 0;
pub const ATHUB_BASE__INST0_SEG3: u32 = 0;
pub const ATHUB_BASE__INST0_SEG4: u32 = 0;
pub const ATHUB_BASE__INST1_SEG0: u32 = 0;
pub const ATHUB_BASE__INST1_SEG1: u32 = 0;
pub const ATHUB_BASE__INST1_SEG2: u32 = 0;
pub const ATHUB_BASE__INST1_SEG3: u32 = 0;
pub const ATHUB_BASE__INST1_SEG4: u32 = 0;
pub const ATHUB_BASE__INST2_SEG0: u32 = 0;
pub const ATHUB_BASE__INST2_SEG1: u32 = 0;
pub const ATHUB_BASE__INST2_SEG2: u32 = 0;
pub const ATHUB_BASE__INST2_SEG3: u32 = 0;
pub const ATHUB_BASE__INST2_SEG4: u32 = 0;
pub const ATHUB_BASE__INST3_SEG0: u32 = 0;
pub const ATHUB_BASE__INST3_SEG1: u32 = 0;
pub const ATHUB_BASE__INST3_SEG2: u32 = 0;
pub const ATHUB_BASE__INST3_SEG3: u32 = 0;
pub const ATHUB_BASE__INST3_SEG4: u32 = 0;
pub const ATHUB_BASE__INST4_SEG0: u32 = 0;
pub const ATHUB_BASE__INST4_SEG1: u32 = 0;
pub const ATHUB_BASE__INST4_SEG2: u32 = 0;
pub const ATHUB_BASE__INST4_SEG3: u32 = 0;
pub const ATHUB_BASE__INST4_SEG4: u32 = 0;
pub const ATHUB_BASE__INST5_SEG0: u32 = 0;
pub const ATHUB_BASE__INST5_SEG1: u32 = 0;
pub const ATHUB_BASE__INST5_SEG2: u32 = 0;
pub const ATHUB_BASE__INST5_SEG3: u32 = 0;
pub const ATHUB_BASE__INST5_SEG4: u32 = 0;
pub const ATHUB_BASE__INST6_SEG0: u32 = 0;
pub const ATHUB_BASE__INST6_SEG1: u32 = 0;
pub const ATHUB_BASE__INST6_SEG2: u32 = 0;
pub const ATHUB_BASE__INST6_SEG3: u32 = 0;
pub const ATHUB_BASE__INST6_SEG4: u32 = 0;
pub const CLK_BASE__INST0_SEG0: u32 = 0x00016C00;
pub const CLK_BASE__INST0_SEG1: u32 = 0x00016E00;
pub const CLK_BASE__INST0_SEG2: u32 = 0x00017000;
pub const CLK_BASE__INST0_SEG3: u32 = 0x00017E00;
pub const CLK_BASE__INST0_SEG4: u32 = 0;
pub const CLK_BASE__INST1_SEG0: u32 = 0;
pub const CLK_BASE__INST1_SEG1: u32 = 0;
pub const CLK_BASE__INST1_SEG2: u32 = 0;
pub const CLK_BASE__INST1_SEG3: u32 = 0;
pub const CLK_BASE__INST1_SEG4: u32 = 0;
pub const CLK_BASE__INST2_SEG0: u32 = 0;
pub const CLK_BASE__INST2_SEG1: u32 = 0;
pub const CLK_BASE__INST2_SEG2: u32 = 0;
pub const CLK_BASE__INST2_SEG3: u32 = 0;
pub const CLK_BASE__INST2_SEG4: u32 = 0;
pub const CLK_BASE__INST3_SEG0: u32 = 0;
pub const CLK_BASE__INST3_SEG1: u32 = 0;
pub const CLK_BASE__INST3_SEG2: u32 = 0;
pub const CLK_BASE__INST3_SEG3: u32 = 0;
pub const CLK_BASE__INST3_SEG4: u32 = 0;
pub const CLK_BASE__INST4_SEG0: u32 = 0;
pub const CLK_BASE__INST4_SEG1: u32 = 0;
pub const CLK_BASE__INST4_SEG2: u32 = 0;
pub const CLK_BASE__INST4_SEG3: u32 = 0;
pub const CLK_BASE__INST4_SEG4: u32 = 0;
pub const CLK_BASE__INST5_SEG0: u32 = 0;
pub const CLK_BASE__INST5_SEG1: u32 = 0;
pub const CLK_BASE__INST5_SEG2: u32 = 0;
pub const CLK_BASE__INST5_SEG3: u32 = 0;
pub const CLK_BASE__INST5_SEG4: u32 = 0;
pub const CLK_BASE__INST6_SEG0: u32 = 0;
pub const CLK_BASE__INST6_SEG1: u32 = 0;
pub const CLK_BASE__INST6_SEG2: u32 = 0;
pub const CLK_BASE__INST6_SEG3: u32 = 0;
pub const CLK_BASE__INST6_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST0_SEG0: u32 = 0x000001E0;
pub const DBGU_IO0_BASE__INST0_SEG1: u32 = 0x0240B400;
pub const DBGU_IO0_BASE__INST0_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST0_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST0_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST1_SEG0: u32 = 0;
pub const DBGU_IO0_BASE__INST1_SEG1: u32 = 0;
pub const DBGU_IO0_BASE__INST1_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST1_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST1_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST2_SEG0: u32 = 0;
pub const DBGU_IO0_BASE__INST2_SEG1: u32 = 0;
pub const DBGU_IO0_BASE__INST2_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST2_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST2_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST3_SEG0: u32 = 0;
pub const DBGU_IO0_BASE__INST3_SEG1: u32 = 0;
pub const DBGU_IO0_BASE__INST3_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST3_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST3_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST4_SEG0: u32 = 0;
pub const DBGU_IO0_BASE__INST4_SEG1: u32 = 0;
pub const DBGU_IO0_BASE__INST4_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST4_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST4_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST5_SEG0: u32 = 0;
pub const DBGU_IO0_BASE__INST5_SEG1: u32 = 0;
pub const DBGU_IO0_BASE__INST5_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST5_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST5_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST6_SEG0: u32 = 0;
pub const DBGU_IO0_BASE__INST6_SEG1: u32 = 0;
pub const DBGU_IO0_BASE__INST6_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST6_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST6_SEG4: u32 = 0;
pub const DF_BASE__INST0_SEG0: u32 = 0x00007000;
pub const DF_BASE__INST0_SEG1: u32 = 0x0240B800;
pub const DF_BASE__INST0_SEG2: u32 = 0;
pub const DF_BASE__INST0_SEG3: u32 = 0;
pub const DF_BASE__INST0_SEG4: u32 = 0;
pub const DF_BASE__INST1_SEG0: u32 = 0;
pub const DF_BASE__INST1_SEG1: u32 = 0;
pub const DF_BASE__INST1_SEG2: u32 = 0;
pub const DF_BASE__INST1_SEG3: u32 = 0;
pub const DF_BASE__INST1_SEG4: u32 = 0;
pub const DF_BASE__INST2_SEG0: u32 = 0;
pub const DF_BASE__INST2_SEG1: u32 = 0;
pub const DF_BASE__INST2_SEG2: u32 = 0;
pub const DF_BASE__INST2_SEG3: u32 = 0;
pub const DF_BASE__INST2_SEG4: u32 = 0;
pub const DF_BASE__INST3_SEG0: u32 = 0;
pub const DF_BASE__INST3_SEG1: u32 = 0;
pub const DF_BASE__INST3_SEG2: u32 = 0;
pub const DF_BASE__INST3_SEG3: u32 = 0;
pub const DF_BASE__INST3_SEG4: u32 = 0;
pub const DF_BASE__INST4_SEG0: u32 = 0;
pub const DF_BASE__INST4_SEG1: u32 = 0;
pub const DF_BASE__INST4_SEG2: u32 = 0;
pub const DF_BASE__INST4_SEG3: u32 = 0;
pub const DF_BASE__INST4_SEG4: u32 = 0;
pub const DF_BASE__INST5_SEG0: u32 = 0;
pub const DF_BASE__INST5_SEG1: u32 = 0;
pub const DF_BASE__INST5_SEG2: u32 = 0;
pub const DF_BASE__INST5_SEG3: u32 = 0;
pub const DF_BASE__INST5_SEG4: u32 = 0;
pub const DF_BASE__INST6_SEG0: u32 = 0;
pub const DF_BASE__INST6_SEG1: u32 = 0;
pub const DF_BASE__INST6_SEG2: u32 = 0;
pub const DF_BASE__INST6_SEG3: u32 = 0;
pub const DF_BASE__INST6_SEG4: u32 = 0;
pub const DIO_BASE__INST0_SEG0: u32 = 0x02404000;
pub const DIO_BASE__INST0_SEG1: u32 = 0;
pub const DIO_BASE__INST0_SEG2: u32 = 0;
pub const DIO_BASE__INST0_SEG3: u32 = 0;
pub const DIO_BASE__INST0_SEG4: u32 = 0;
pub const DIO_BASE__INST1_SEG0: u32 = 0;
pub const DIO_BASE__INST1_SEG1: u32 = 0;
pub const DIO_BASE__INST1_SEG2: u32 = 0;
pub const DIO_BASE__INST1_SEG3: u32 = 0;
pub const DIO_BASE__INST1_SEG4: u32 = 0;
pub const DIO_BASE__INST2_SEG0: u32 = 0;
pub const DIO_BASE__INST2_SEG1: u32 = 0;
pub const DIO_BASE__INST2_SEG2: u32 = 0;
pub const DIO_BASE__INST2_SEG3: u32 = 0;
pub const DIO_BASE__INST2_SEG4: u32 = 0;
pub const DIO_BASE__INST3_SEG0: u32 = 0;
pub const DIO_BASE__INST3_SEG1: u32 = 0;
pub const DIO_BASE__INST3_SEG2: u32 = 0;
pub const DIO_BASE__INST3_SEG3: u32 = 0;
pub const DIO_BASE__INST3_SEG4: u32 = 0;
pub const DIO_BASE__INST4_SEG0: u32 = 0;
pub const DIO_BASE__INST4_SEG1: u32 = 0;
pub const DIO_BASE__INST4_SEG2: u32 = 0;
pub const DIO_BASE__INST4_SEG3: u32 = 0;
pub const DIO_BASE__INST4_SEG4: u32 = 0;
pub const DIO_BASE__INST5_SEG0: u32 = 0;
pub const DIO_BASE__INST5_SEG1: u32 = 0;
pub const DIO_BASE__INST5_SEG2: u32 = 0;
pub const DIO_BASE__INST5_SEG3: u32 = 0;
pub const DIO_BASE__INST5_SEG4: u32 = 0;
pub const DIO_BASE__INST6_SEG0: u32 = 0;
pub const DIO_BASE__INST6_SEG1: u32 = 0;
pub const DIO_BASE__INST6_SEG2: u32 = 0;
pub const DIO_BASE__INST6_SEG3: u32 = 0;
pub const DIO_BASE__INST6_SEG4: u32 = 0;
pub const DMU_BASE__INST0_SEG0: u32 = 0x00000012;
pub const DMU_BASE__INST0_SEG1: u32 = 0x000000C0;
pub const DMU_BASE__INST0_SEG2: u32 = 0x000034C0;
pub const DMU_BASE__INST0_SEG3: u32 = 0x00009000;
pub const DMU_BASE__INST0_SEG4: u32 = 0x02403C00;
pub const DMU_BASE__INST1_SEG0: u32 = 0;
pub const DMU_BASE__INST1_SEG1: u32 = 0;
pub const DMU_BASE__INST1_SEG2: u32 = 0;
pub const DMU_BASE__INST1_SEG3: u32 = 0;
pub const DMU_BASE__INST1_SEG4: u32 = 0;
pub const DMU_BASE__INST2_SEG0: u32 = 0;
pub const DMU_BASE__INST2_SEG1: u32 = 0;
pub const DMU_BASE__INST2_SEG2: u32 = 0;
pub const DMU_BASE__INST2_SEG3: u32 = 0;
pub const DMU_BASE__INST2_SEG4: u32 = 0;
pub const DMU_BASE__INST3_SEG0: u32 = 0;
pub const DMU_BASE__INST3_SEG1: u32 = 0;
pub const DMU_BASE__INST3_SEG2: u32 = 0;
pub const DMU_BASE__INST3_SEG3: u32 = 0;
pub const DMU_BASE__INST3_SEG4: u32 = 0;
pub const DMU_BASE__INST4_SEG0: u32 = 0;
pub const DMU_BASE__INST4_SEG1: u32 = 0;
pub const DMU_BASE__INST4_SEG2: u32 = 0;
pub const DMU_BASE__INST4_SEG3: u32 = 0;
pub const DMU_BASE__INST4_SEG4: u32 = 0;
pub const DMU_BASE__INST5_SEG0: u32 = 0;
pub const DMU_BASE__INST5_SEG1: u32 = 0;
pub const DMU_BASE__INST5_SEG2: u32 = 0;
pub const DMU_BASE__INST5_SEG3: u32 = 0;
pub const DMU_BASE__INST5_SEG4: u32 = 0;
pub const DMU_BASE__INST6_SEG0: u32 = 0;
pub const DMU_BASE__INST6_SEG1: u32 = 0;
pub const DMU_BASE__INST6_SEG2: u32 = 0;
pub const DMU_BASE__INST6_SEG3: u32 = 0;
pub const DMU_BASE__INST6_SEG4: u32 = 0;
pub const DPCS_BASE__INST0_SEG0: u32 = 0x00000012;
pub const DPCS_BASE__INST0_SEG1: u32 = 0x000000C0;
pub const DPCS_BASE__INST0_SEG2: u32 = 0x000034C0;
pub const DPCS_BASE__INST0_SEG3: u32 = 0x00009000;
pub const DPCS_BASE__INST0_SEG4: u32 = 0x02403C00;
pub const DPCS_BASE__INST1_SEG0: u32 = 0;
pub const DPCS_BASE__INST1_SEG1: u32 = 0;
pub const DPCS_BASE__INST1_SEG2: u32 = 0;
pub const DPCS_BASE__INST1_SEG3: u32 = 0;
pub const DPCS_BASE__INST1_SEG4: u32 = 0;
pub const DPCS_BASE__INST2_SEG0: u32 = 0;
pub const DPCS_BASE__INST2_SEG1: u32 = 0;
pub const DPCS_BASE__INST2_SEG2: u32 = 0;
pub const DPCS_BASE__INST2_SEG3: u32 = 0;
pub const DPCS_BASE__INST2_SEG4: u32 = 0;
pub const DPCS_BASE__INST3_SEG0: u32 = 0;
pub const DPCS_BASE__INST3_SEG1: u32 = 0;
pub const DPCS_BASE__INST3_SEG2: u32 = 0;
pub const DPCS_BASE__INST3_SEG3: u32 = 0;
pub const DPCS_BASE__INST3_SEG4: u32 = 0;
pub const DPCS_BASE__INST4_SEG0: u32 = 0;
pub const DPCS_BASE__INST4_SEG1: u32 = 0;
pub const DPCS_BASE__INST4_SEG2: u32 = 0;
pub const DPCS_BASE__INST4_SEG3: u32 = 0;
pub const DPCS_BASE__INST4_SEG4: u32 = 0;
pub const DPCS_BASE__INST5_SEG0: u32 = 0;
pub const DPCS_BASE__INST5_SEG1: u32 = 0;
pub const DPCS_BASE__INST5_SEG2: u32 = 0;
pub const DPCS_BASE__INST5_SEG3: u32 = 0;
pub const DPCS_BASE__INST5_SEG4: u32 = 0;
pub const DPCS_BASE__INST6_SEG0: u32 = 0;
pub const DPCS_BASE__INST6_SEG1: u32 = 0;
pub const DPCS_BASE__INST6_SEG2: u32 = 0;
pub const DPCS_BASE__INST6_SEG3: u32 = 0;
pub const DPCS_BASE__INST6_SEG4: u32 = 0;
pub const FUSE_BASE__INST0_SEG0: u32 = 0x00017400;
pub const FUSE_BASE__INST0_SEG1: u32 = 0x02401400;
pub const FUSE_BASE__INST0_SEG2: u32 = 0;
pub const FUSE_BASE__INST0_SEG3: u32 = 0;
pub const FUSE_BASE__INST0_SEG4: u32 = 0;
pub const FUSE_BASE__INST1_SEG0: u32 = 0;
pub const FUSE_BASE__INST1_SEG1: u32 = 0;
pub const FUSE_BASE__INST1_SEG2: u32 = 0;
pub const FUSE_BASE__INST1_SEG3: u32 = 0;
pub const FUSE_BASE__INST1_SEG4: u32 = 0;
pub const FUSE_BASE__INST2_SEG0: u32 = 0;
pub const FUSE_BASE__INST2_SEG1: u32 = 0;
pub const FUSE_BASE__INST2_SEG2: u32 = 0;
pub const FUSE_BASE__INST2_SEG3: u32 = 0;
pub const FUSE_BASE__INST2_SEG4: u32 = 0;
pub const FUSE_BASE__INST3_SEG0: u32 = 0;
pub const FUSE_BASE__INST3_SEG1: u32 = 0;
pub const FUSE_BASE__INST3_SEG2: u32 = 0;
pub const FUSE_BASE__INST3_SEG3: u32 = 0;
pub const FUSE_BASE__INST3_SEG4: u32 = 0;
pub const FUSE_BASE__INST4_SEG0: u32 = 0;
pub const FUSE_BASE__INST4_SEG1: u32 = 0;
pub const FUSE_BASE__INST4_SEG2: u32 = 0;
pub const FUSE_BASE__INST4_SEG3: u32 = 0;
pub const FUSE_BASE__INST4_SEG4: u32 = 0;
pub const FUSE_BASE__INST5_SEG0: u32 = 0;
pub const FUSE_BASE__INST5_SEG1: u32 = 0;
pub const FUSE_BASE__INST5_SEG2: u32 = 0;
pub const FUSE_BASE__INST5_SEG3: u32 = 0;
pub const FUSE_BASE__INST5_SEG4: u32 = 0;
pub const FUSE_BASE__INST6_SEG0: u32 = 0;
pub const FUSE_BASE__INST6_SEG1: u32 = 0;
pub const FUSE_BASE__INST6_SEG2: u32 = 0;
pub const FUSE_BASE__INST6_SEG3: u32 = 0;
pub const FUSE_BASE__INST6_SEG4: u32 = 0;
pub const GC_BASE__INST0_SEG0: u32 = 0x00002000;
pub const GC_BASE__INST0_SEG1: u32 = 0x0000A000;
pub const GC_BASE__INST0_SEG2: u32 = 0x02402C00;
pub const GC_BASE__INST0_SEG3: u32 = 0;
pub const GC_BASE__INST0_SEG4: u32 = 0;
pub const GC_BASE__INST1_SEG0: u32 = 0;
pub const GC_BASE__INST1_SEG1: u32 = 0;
pub const GC_BASE__INST1_SEG2: u32 = 0;
pub const GC_BASE__INST1_SEG3: u32 = 0;
pub const GC_BASE__INST1_SEG4: u32 = 0;
pub const GC_BASE__INST2_SEG0: u32 = 0;
pub const GC_BASE__INST2_SEG1: u32 = 0;
pub const GC_BASE__INST2_SEG2: u32 = 0;
pub const GC_BASE__INST2_SEG3: u32 = 0;
pub const GC_BASE__INST2_SEG4: u32 = 0;
pub const GC_BASE__INST3_SEG0: u32 = 0;
pub const GC_BASE__INST3_SEG1: u32 = 0;
pub const GC_BASE__INST3_SEG2: u32 = 0;
pub const GC_BASE__INST3_SEG3: u32 = 0;
pub const GC_BASE__INST3_SEG4: u32 = 0;
pub const GC_BASE__INST4_SEG0: u32 = 0;
pub const GC_BASE__INST4_SEG1: u32 = 0;
pub const GC_BASE__INST4_SEG2: u32 = 0;
pub const GC_BASE__INST4_SEG3: u32 = 0;
pub const GC_BASE__INST4_SEG4: u32 = 0;
pub const GC_BASE__INST5_SEG0: u32 = 0;
pub const GC_BASE__INST5_SEG1: u32 = 0;
pub const GC_BASE__INST5_SEG2: u32 = 0;
pub const GC_BASE__INST5_SEG3: u32 = 0;
pub const GC_BASE__INST5_SEG4: u32 = 0;
pub const GC_BASE__INST6_SEG0: u32 = 0;
pub const GC_BASE__INST6_SEG1: u32 = 0;
pub const GC_BASE__INST6_SEG2: u32 = 0;
pub const GC_BASE__INST6_SEG3: u32 = 0;
pub const GC_BASE__INST6_SEG4: u32 = 0;
pub const HDA_BASE__INST0_SEG0: u32 = 0x02404800;
pub const HDA_BASE__INST0_SEG1: u32 = 0x004C0000;
pub const HDA_BASE__INST0_SEG2: u32 = 0;
pub const HDA_BASE__INST0_SEG3: u32 = 0;
pub const HDA_BASE__INST0_SEG4: u32 = 0;
pub const HDA_BASE__INST1_SEG0: u32 = 0;
pub const HDA_BASE__INST1_SEG1: u32 = 0;
pub const HDA_BASE__INST1_SEG2: u32 = 0;
pub const HDA_BASE__INST1_SEG3: u32 = 0;
pub const HDA_BASE__INST1_SEG4: u32 = 0;
pub const HDA_BASE__INST2_SEG0: u32 = 0;
pub const HDA_BASE__INST2_SEG1: u32 = 0;
pub const HDA_BASE__INST2_SEG2: u32 = 0;
pub const HDA_BASE__INST2_SEG3: u32 = 0;
pub const HDA_BASE__INST2_SEG4: u32 = 0;
pub const HDA_BASE__INST3_SEG0: u32 = 0;
pub const HDA_BASE__INST3_SEG1: u32 = 0;
pub const HDA_BASE__INST3_SEG2: u32 = 0;
pub const HDA_BASE__INST3_SEG3: u32 = 0;
pub const HDA_BASE__INST3_SEG4: u32 = 0;
pub const HDA_BASE__INST4_SEG0: u32 = 0;
pub const HDA_BASE__INST4_SEG1: u32 = 0;
pub const HDA_BASE__INST4_SEG2: u32 = 0;
pub const HDA_BASE__INST4_SEG3: u32 = 0;
pub const HDA_BASE__INST4_SEG4: u32 = 0;
pub const HDA_BASE__INST5_SEG0: u32 = 0;
pub const HDA_BASE__INST5_SEG1: u32 = 0;
pub const HDA_BASE__INST5_SEG2: u32 = 0;
pub const HDA_BASE__INST5_SEG3: u32 = 0;
pub const HDA_BASE__INST5_SEG4: u32 = 0;
pub const HDA_BASE__INST6_SEG0: u32 = 0;
pub const HDA_BASE__INST6_SEG1: u32 = 0;
pub const HDA_BASE__INST6_SEG2: u32 = 0;
pub const HDA_BASE__INST6_SEG3: u32 = 0;
pub const HDA_BASE__INST6_SEG4: u32 = 0;
pub const HDP_BASE__INST0_SEG0: u32 = 0x00000F20;
pub const HDP_BASE__INST0_SEG1: u32 = 0x0240A400;
pub const HDP_BASE__INST0_SEG2: u32 = 0;
pub const HDP_BASE__INST0_SEG3: u32 = 0;
pub const HDP_BASE__INST0_SEG4: u32 = 0;
pub const HDP_BASE__INST1_SEG0: u32 = 0;
pub const HDP_BASE__INST1_SEG1: u32 = 0;
pub const HDP_BASE__INST1_SEG2: u32 = 0;
pub const HDP_BASE__INST1_SEG3: u32 = 0;
pub const HDP_BASE__INST1_SEG4: u32 = 0;
pub const HDP_BASE__INST2_SEG0: u32 = 0;
pub const HDP_BASE__INST2_SEG1: u32 = 0;
pub const HDP_BASE__INST2_SEG2: u32 = 0;
pub const HDP_BASE__INST2_SEG3: u32 = 0;
pub const HDP_BASE__INST2_SEG4: u32 = 0;
pub const HDP_BASE__INST3_SEG0: u32 = 0;
pub const HDP_BASE__INST3_SEG1: u32 = 0;
pub const HDP_BASE__INST3_SEG2: u32 = 0;
pub const HDP_BASE__INST3_SEG3: u32 = 0;
pub const HDP_BASE__INST3_SEG4: u32 = 0;
pub const HDP_BASE__INST4_SEG0: u32 = 0;
pub const HDP_BASE__INST4_SEG1: u32 = 0;
pub const HDP_BASE__INST4_SEG2: u32 = 0;
pub const HDP_BASE__INST4_SEG3: u32 = 0;
pub const HDP_BASE__INST4_SEG4: u32 = 0;
pub const HDP_BASE__INST5_SEG0: u32 = 0;
pub const HDP_BASE__INST5_SEG1: u32 = 0;
pub const HDP_BASE__INST5_SEG2: u32 = 0;
pub const HDP_BASE__INST5_SEG3: u32 = 0;
pub const HDP_BASE__INST5_SEG4: u32 = 0;
pub const HDP_BASE__INST6_SEG0: u32 = 0;
pub const HDP_BASE__INST6_SEG1: u32 = 0;
pub const HDP_BASE__INST6_SEG2: u32 = 0;
pub const HDP_BASE__INST6_SEG3: u32 = 0;
pub const HDP_BASE__INST6_SEG4: u32 = 0;
pub const IOHC0_BASE__INST0_SEG0: u32 = 0x00010000;
pub const IOHC0_BASE__INST0_SEG1: u32 = 0x02406000;
pub const IOHC0_BASE__INST0_SEG2: u32 = 0x04EC0000;
pub const IOHC0_BASE__INST0_SEG3: u32 = 0;
pub const IOHC0_BASE__INST0_SEG4: u32 = 0;
pub const IOHC0_BASE__INST1_SEG0: u32 = 0;
pub const IOHC0_BASE__INST1_SEG1: u32 = 0;
pub const IOHC0_BASE__INST1_SEG2: u32 = 0;
pub const IOHC0_BASE__INST1_SEG3: u32 = 0;
pub const IOHC0_BASE__INST1_SEG4: u32 = 0;
pub const IOHC0_BASE__INST2_SEG0: u32 = 0;
pub const IOHC0_BASE__INST2_SEG1: u32 = 0;
pub const IOHC0_BASE__INST2_SEG2: u32 = 0;
pub const IOHC0_BASE__INST2_SEG3: u32 = 0;
pub const IOHC0_BASE__INST2_SEG4: u32 = 0;
pub const IOHC0_BASE__INST3_SEG0: u32 = 0;
pub const IOHC0_BASE__INST3_SEG1: u32 = 0;
pub const IOHC0_BASE__INST3_SEG2: u32 = 0;
pub const IOHC0_BASE__INST3_SEG3: u32 = 0;
pub const IOHC0_BASE__INST3_SEG4: u32 = 0;
pub const IOHC0_BASE__INST4_SEG0: u32 = 0;
pub const IOHC0_BASE__INST4_SEG1: u32 = 0;
pub const IOHC0_BASE__INST4_SEG2: u32 = 0;
pub const IOHC0_BASE__INST4_SEG3: u32 = 0;
pub const IOHC0_BASE__INST4_SEG4: u32 = 0;
pub const IOHC0_BASE__INST5_SEG0: u32 = 0;
pub const IOHC0_BASE__INST5_SEG1: u32 = 0;
pub const IOHC0_BASE__INST5_SEG2: u32 = 0;
pub const IOHC0_BASE__INST5_SEG3: u32 = 0;
pub const IOHC0_BASE__INST5_SEG4: u32 = 0;
pub const IOHC0_BASE__INST6_SEG0: u32 = 0;
pub const IOHC0_BASE__INST6_SEG1: u32 = 0;
pub const IOHC0_BASE__INST6_SEG2: u32 = 0;
pub const IOHC0_BASE__INST6_SEG3: u32 = 0;
pub const IOHC0_BASE__INST6_SEG4: u32 = 0;
pub const ISP_BASE__INST0_SEG0: u32 = 0x00018000;
pub const ISP_BASE__INST0_SEG1: u32 = 0x0240B000;
pub const ISP_BASE__INST0_SEG2: u32 = 0;
pub const ISP_BASE__INST0_SEG3: u32 = 0;
pub const ISP_BASE__INST0_SEG4: u32 = 0;
pub const ISP_BASE__INST1_SEG0: u32 = 0;
pub const ISP_BASE__INST1_SEG1: u32 = 0;
pub const ISP_BASE__INST1_SEG2: u32 = 0;
pub const ISP_BASE__INST1_SEG3: u32 = 0;
pub const ISP_BASE__INST1_SEG4: u32 = 0;
pub const ISP_BASE__INST2_SEG0: u32 = 0;
pub const ISP_BASE__INST2_SEG1: u32 = 0;
pub const ISP_BASE__INST2_SEG2: u32 = 0;
pub const ISP_BASE__INST2_SEG3: u32 = 0;
pub const ISP_BASE__INST2_SEG4: u32 = 0;
pub const ISP_BASE__INST3_SEG0: u32 = 0;
pub const ISP_BASE__INST3_SEG1: u32 = 0;
pub const ISP_BASE__INST3_SEG2: u32 = 0;
pub const ISP_BASE__INST3_SEG3: u32 = 0;
pub const ISP_BASE__INST3_SEG4: u32 = 0;
pub const ISP_BASE__INST4_SEG0: u32 = 0;
pub const ISP_BASE__INST4_SEG1: u32 = 0;
pub const ISP_BASE__INST4_SEG2: u32 = 0;
pub const ISP_BASE__INST4_SEG3: u32 = 0;
pub const ISP_BASE__INST4_SEG4: u32 = 0;
pub const ISP_BASE__INST5_SEG0: u32 = 0;
pub const ISP_BASE__INST5_SEG1: u32 = 0;
pub const ISP_BASE__INST5_SEG2: u32 = 0;
pub const ISP_BASE__INST5_SEG3: u32 = 0;
pub const ISP_BASE__INST5_SEG4: u32 = 0;
pub const ISP_BASE__INST6_SEG0: u32 = 0;
pub const ISP_BASE__INST6_SEG1: u32 = 0;
pub const ISP_BASE__INST6_SEG2: u32 = 0;
pub const ISP_BASE__INST6_SEG3: u32 = 0;
pub const ISP_BASE__INST6_SEG4: u32 = 0;
pub const L2IMU0_BASE__INST0_SEG0: u32 = 0x00007DC0;
pub const L2IMU0_BASE__INST0_SEG1: u32 = 0x02407000;
pub const L2IMU0_BASE__INST0_SEG2: u32 = 0x00900000;
pub const L2IMU0_BASE__INST0_SEG3: u32 = 0x04FC0000;
pub const L2IMU0_BASE__INST0_SEG4: u32 = 0x055C0000;
pub const L2IMU0_BASE__INST1_SEG0: u32 = 0;
pub const L2IMU0_BASE__INST1_SEG1: u32 = 0;
pub const L2IMU0_BASE__INST1_SEG2: u32 = 0;
pub const L2IMU0_BASE__INST1_SEG3: u32 = 0;
pub const L2IMU0_BASE__INST1_SEG4: u32 = 0;
pub const L2IMU0_BASE__INST2_SEG0: u32 = 0;
pub const L2IMU0_BASE__INST2_SEG1: u32 = 0;
pub const L2IMU0_BASE__INST2_SEG2: u32 = 0;
pub const L2IMU0_BASE__INST2_SEG3: u32 = 0;
pub const L2IMU0_BASE__INST2_SEG4: u32 = 0;
pub const L2IMU0_BASE__INST3_SEG0: u32 = 0;
pub const L2IMU0_BASE__INST3_SEG1: u32 = 0;
pub const L2IMU0_BASE__INST3_SEG2: u32 = 0;
pub const L2IMU0_BASE__INST3_SEG3: u32 = 0;
pub const L2IMU0_BASE__INST3_SEG4: u32 = 0;
pub const L2IMU0_BASE__INST4_SEG0: u32 = 0;
pub const L2IMU0_BASE__INST4_SEG1: u32 = 0;
pub const L2IMU0_BASE__INST4_SEG2: u32 = 0;
pub const L2IMU0_BASE__INST4_SEG3: u32 = 0;
pub const L2IMU0_BASE__INST4_SEG4: u32 = 0;
pub const L2IMU0_BASE__INST5_SEG0: u32 = 0;
pub const L2IMU0_BASE__INST5_SEG1: u32 = 0;
pub const L2IMU0_BASE__INST5_SEG2: u32 = 0;
pub const L2IMU0_BASE__INST5_SEG3: u32 = 0;
pub const L2IMU0_BASE__INST5_SEG4: u32 = 0;
pub const L2IMU0_BASE__INST6_SEG0: u32 = 0;
pub const L2IMU0_BASE__INST6_SEG1: u32 = 0;
pub const L2IMU0_BASE__INST6_SEG2: u32 = 0;
pub const L2IMU0_BASE__INST6_SEG3: u32 = 0;
pub const L2IMU0_BASE__INST6_SEG4: u32 = 0;
pub const MMHUB_BASE__INST0_SEG0: u32 = 0x0001A000;
pub const MMHUB_BASE__INST0_SEG1: u32 = 0x02408800;
pub const MMHUB_BASE__INST0_SEG2: u32 = 0;
pub const MMHUB_BASE__INST0_SEG3: u32 = 0;
pub const MMHUB_BASE__INST0_SEG4: u32 = 0;
pub const MMHUB_BASE__INST1_SEG0: u32 = 0;
pub const MMHUB_BASE__INST1_SEG1: u32 = 0;
pub const MMHUB_BASE__INST1_SEG2: u32 = 0;
pub const MMHUB_BASE__INST1_SEG3: u32 = 0;
pub const MMHUB_BASE__INST1_SEG4: u32 = 0;
pub const MMHUB_BASE__INST2_SEG0: u32 = 0;
pub const MMHUB_BASE__INST2_SEG1: u32 = 0;
pub const MMHUB_BASE__INST2_SEG2: u32 = 0;
pub const MMHUB_BASE__INST2_SEG3: u32 = 0;
pub const MMHUB_BASE__INST2_SEG4: u32 = 0;
pub const MMHUB_BASE__INST3_SEG0: u32 = 0;
pub const MMHUB_BASE__INST3_SEG1: u32 = 0;
pub const MMHUB_BASE__INST3_SEG2: u32 = 0;
pub const MMHUB_BASE__INST3_SEG3: u32 = 0;
pub const MMHUB_BASE__INST3_SEG4: u32 = 0;
pub const MMHUB_BASE__INST4_SEG0: u32 = 0;
pub const MMHUB_BASE__INST4_SEG1: u32 = 0;
pub const MMHUB_BASE__INST4_SEG2: u32 = 0;
pub const MMHUB_BASE__INST4_SEG3: u32 = 0;
pub const MMHUB_BASE__INST4_SEG4: u32 = 0;
pub const MMHUB_BASE__INST5_SEG0: u32 = 0;
pub const MMHUB_BASE__INST5_SEG1: u32 = 0;
pub const MMHUB_BASE__INST5_SEG2: u32 = 0;
pub const MMHUB_BASE__INST5_SEG3: u32 = 0;
pub const MMHUB_BASE__INST5_SEG4: u32 = 0;
pub const MMHUB_BASE__INST6_SEG0: u32 = 0;
pub const MMHUB_BASE__INST6_SEG1: u32 = 0;
pub const MMHUB_BASE__INST6_SEG2: u32 = 0;
pub const MMHUB_BASE__INST6_SEG3: u32 = 0;
pub const MMHUB_BASE__INST6_SEG4: u32 = 0;
pub const MP0_BASE__INST0_SEG0: u32 = 0x00016000;
pub const MP0_BASE__INST0_SEG1: u32 = 0x0243FC00;
pub const MP0_BASE__INST0_SEG2: u32 = 0x00DC0000;
pub const MP0_BASE__INST0_SEG3: u32 = 0x00E00000;
pub const MP0_BASE__INST0_SEG4: u32 = 0x00E40000;
pub const MP0_BASE__INST1_SEG0: u32 = 0;
pub const MP0_BASE__INST1_SEG1: u32 = 0;
pub const MP0_BASE__INST1_SEG2: u32 = 0;
pub const MP0_BASE__INST1_SEG3: u32 = 0;
pub const MP0_BASE__INST1_SEG4: u32 = 0;
pub const MP0_BASE__INST2_SEG0: u32 = 0;
pub const MP0_BASE__INST2_SEG1: u32 = 0;
pub const MP0_BASE__INST2_SEG2: u32 = 0;
pub const MP0_BASE__INST2_SEG3: u32 = 0;
pub const MP0_BASE__INST2_SEG4: u32 = 0;
pub const MP0_BASE__INST3_SEG0: u32 = 0;
pub const MP0_BASE__INST3_SEG1: u32 = 0;
pub const MP0_BASE__INST3_SEG2: u32 = 0;
pub const MP0_BASE__INST3_SEG3: u32 = 0;
pub const MP0_BASE__INST3_SEG4: u32 = 0;
pub const MP0_BASE__INST4_SEG0: u32 = 0;
pub const MP0_BASE__INST4_SEG1: u32 = 0;
pub const MP0_BASE__INST4_SEG2: u32 = 0;
pub const MP0_BASE__INST4_SEG3: u32 = 0;
pub const MP0_BASE__INST4_SEG4: u32 = 0;
pub const MP0_BASE__INST5_SEG0: u32 = 0;
pub const MP0_BASE__INST5_SEG1: u32 = 0;
pub const MP0_BASE__INST5_SEG2: u32 = 0;
pub const MP0_BASE__INST5_SEG3: u32 = 0;
pub const MP0_BASE__INST5_SEG4: u32 = 0;
pub const MP0_BASE__INST6_SEG0: u32 = 0;
pub const MP0_BASE__INST6_SEG1: u32 = 0;
pub const MP0_BASE__INST6_SEG2: u32 = 0;
pub const MP0_BASE__INST6_SEG3: u32 = 0;
pub const MP0_BASE__INST6_SEG4: u32 = 0;
pub const MP1_BASE__INST0_SEG0: u32 = 0x00016200;
pub const MP1_BASE__INST0_SEG1: u32 = 0x02400400;
pub const MP1_BASE__INST0_SEG2: u32 = 0x00E80000;
pub const MP1_BASE__INST0_SEG3: u32 = 0x00EC0000;
pub const MP1_BASE__INST0_SEG4: u32 = 0x00F00000;
pub const MP1_BASE__INST1_SEG0: u32 = 0;
pub const MP1_BASE__INST1_SEG1: u32 = 0;
pub const MP1_BASE__INST1_SEG2: u32 = 0;
pub const MP1_BASE__INST1_SEG3: u32 = 0;
pub const MP1_BASE__INST1_SEG4: u32 = 0;
pub const MP1_BASE__INST2_SEG0: u32 = 0;
pub const MP1_BASE__INST2_SEG1: u32 = 0;
pub const MP1_BASE__INST2_SEG2: u32 = 0;
pub const MP1_BASE__INST2_SEG3: u32 = 0;
pub const MP1_BASE__INST2_SEG4: u32 = 0;
pub const MP1_BASE__INST3_SEG0: u32 = 0;
pub const MP1_BASE__INST3_SEG1: u32 = 0;
pub const MP1_BASE__INST3_SEG2: u32 = 0;
pub const MP1_BASE__INST3_SEG3: u32 = 0;
pub const MP1_BASE__INST3_SEG4: u32 = 0;
pub const MP1_BASE__INST4_SEG0: u32 = 0;
pub const MP1_BASE__INST4_SEG1: u32 = 0;
pub const MP1_BASE__INST4_SEG2: u32 = 0;
pub const MP1_BASE__INST4_SEG3: u32 = 0;
pub const MP1_BASE__INST4_SEG4: u32 = 0;
pub const MP1_BASE__INST5_SEG0: u32 = 0;
pub const MP1_BASE__INST5_SEG1: u32 = 0;
pub const MP1_BASE__INST5_SEG2: u32 = 0;
pub const MP1_BASE__INST5_SEG3: u32 = 0;
pub const MP1_BASE__INST5_SEG4: u32 = 0;
pub const MP1_BASE__INST6_SEG0: u32 = 0;
pub const MP1_BASE__INST6_SEG1: u32 = 0;
pub const MP1_BASE__INST6_SEG2: u32 = 0;
pub const MP1_BASE__INST6_SEG3: u32 = 0;
pub const MP1_BASE__INST6_SEG4: u32 = 0;
pub const NBIF0_BASE__INST0_SEG0: u32 = 0x00000000;
pub const NBIF0_BASE__INST0_SEG1: u32 = 0x00000014;
pub const NBIF0_BASE__INST0_SEG2: u32 = 0x00000D20;
pub const NBIF0_BASE__INST0_SEG3: u32 = 0x00010400;
pub const NBIF0_BASE__INST0_SEG4: u32 = 0x0241B000;
pub const NBIF0_BASE__INST1_SEG0: u32 = 0;
pub const NBIF0_BASE__INST1_SEG1: u32 = 0;
pub const NBIF0_BASE__INST1_SEG2: u32 = 0;
pub const NBIF0_BASE__INST1_SEG3: u32 = 0;
pub const NBIF0_BASE__INST1_SEG4: u32 = 0;
pub const NBIF0_BASE__INST2_SEG0: u32 = 0;
pub const NBIF0_BASE__INST2_SEG1: u32 = 0;
pub const NBIF0_BASE__INST2_SEG2: u32 = 0;
pub const NBIF0_BASE__INST2_SEG3: u32 = 0;
pub const NBIF0_BASE__INST2_SEG4: u32 = 0;
pub const NBIF0_BASE__INST3_SEG0: u32 = 0;
pub const NBIF0_BASE__INST3_SEG1: u32 = 0;
pub const NBIF0_BASE__INST3_SEG2: u32 = 0;
pub const NBIF0_BASE__INST3_SEG3: u32 = 0;
pub const NBIF0_BASE__INST3_SEG4: u32 = 0;
pub const NBIF0_BASE__INST4_SEG0: u32 = 0;
pub const NBIF0_BASE__INST4_SEG1: u32 = 0;
pub const NBIF0_BASE__INST4_SEG2: u32 = 0;
pub const NBIF0_BASE__INST4_SEG3: u32 = 0;
pub const NBIF0_BASE__INST4_SEG4: u32 = 0;
pub const NBIF0_BASE__INST5_SEG0: u32 = 0;
pub const NBIF0_BASE__INST5_SEG1: u32 = 0;
pub const NBIF0_BASE__INST5_SEG2: u32 = 0;
pub const NBIF0_BASE__INST5_SEG3: u32 = 0;
pub const NBIF0_BASE__INST5_SEG4: u32 = 0;
pub const NBIF0_BASE__INST6_SEG0: u32 = 0;
pub const NBIF0_BASE__INST6_SEG1: u32 = 0;
pub const NBIF0_BASE__INST6_SEG2: u32 = 0;
pub const NBIF0_BASE__INST6_SEG3: u32 = 0;
pub const NBIF0_BASE__INST6_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST0_SEG0: u32 = 0x000010A0;
pub const OSSSYS_BASE__INST0_SEG1: u32 = 0x0240A000;
pub const OSSSYS_BASE__INST0_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST0_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST0_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST1_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST1_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST1_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST1_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST1_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST2_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST2_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST2_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST2_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST2_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST3_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST3_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST3_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST3_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST3_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST4_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST4_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST4_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST4_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST4_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST5_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST5_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST5_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST5_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST5_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST6_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST6_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST6_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST6_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST6_SEG4: u32 = 0;
pub const PCIE0_BASE__INST0_SEG0: u32 = 0x02411800;
pub const PCIE0_BASE__INST0_SEG1: u32 = 0x04440000;
pub const PCIE0_BASE__INST0_SEG2: u32 = 0;
pub const PCIE0_BASE__INST0_SEG3: u32 = 0;
pub const PCIE0_BASE__INST0_SEG4: u32 = 0;
pub const PCIE0_BASE__INST1_SEG0: u32 = 0;
pub const PCIE0_BASE__INST1_SEG1: u32 = 0;
pub const PCIE0_BASE__INST1_SEG2: u32 = 0;
pub const PCIE0_BASE__INST1_SEG3: u32 = 0;
pub const PCIE0_BASE__INST1_SEG4: u32 = 0;
pub const PCIE0_BASE__INST2_SEG0: u32 = 0;
pub const PCIE0_BASE__INST2_SEG1: u32 = 0;
pub const PCIE0_BASE__INST2_SEG2: u32 = 0;
pub const PCIE0_BASE__INST2_SEG3: u32 = 0;
pub const PCIE0_BASE__INST2_SEG4: u32 = 0;
pub const PCIE0_BASE__INST3_SEG0: u32 = 0;
pub const PCIE0_BASE__INST3_SEG1: u32 = 0;
pub const PCIE0_BASE__INST3_SEG2: u32 = 0;
pub const PCIE0_BASE__INST3_SEG3: u32 = 0;
pub const PCIE0_BASE__INST3_SEG4: u32 = 0;
pub const PCIE0_BASE__INST4_SEG0: u32 = 0;
pub const PCIE0_BASE__INST4_SEG1: u32 = 0;
pub const PCIE0_BASE__INST4_SEG2: u32 = 0;
pub const PCIE0_BASE__INST4_SEG3: u32 = 0;
pub const PCIE0_BASE__INST4_SEG4: u32 = 0;
pub const PCIE0_BASE__INST5_SEG0: u32 = 0;
pub const PCIE0_BASE__INST5_SEG1: u32 = 0;
pub const PCIE0_BASE__INST5_SEG2: u32 = 0;
pub const PCIE0_BASE__INST5_SEG3: u32 = 0;
pub const PCIE0_BASE__INST5_SEG4: u32 = 0;
pub const PCIE0_BASE__INST6_SEG0: u32 = 0;
pub const PCIE0_BASE__INST6_SEG1: u32 = 0;
pub const PCIE0_BASE__INST6_SEG2: u32 = 0;
pub const PCIE0_BASE__INST6_SEG3: u32 = 0;
pub const PCIE0_BASE__INST6_SEG4: u32 = 0;
pub const SDMA0_BASE__INST0_SEG0: u32 = 0x00001260;
pub const SDMA0_BASE__INST0_SEG1: u32 = 0x0240A800;
pub const SDMA0_BASE__INST0_SEG2: u32 = 0;
pub const SDMA0_BASE__INST0_SEG3: u32 = 0;
pub const SDMA0_BASE__INST0_SEG4: u32 = 0;
pub const SDMA0_BASE__INST1_SEG0: u32 = 0;
pub const SDMA0_BASE__INST1_SEG1: u32 = 0;
pub const SDMA0_BASE__INST1_SEG2: u32 = 0;
pub const SDMA0_BASE__INST1_SEG3: u32 = 0;
pub const SDMA0_BASE__INST1_SEG4: u32 = 0;
pub const SDMA0_BASE__INST2_SEG0: u32 = 0;
pub const SDMA0_BASE__INST2_SEG1: u32 = 0;
pub const SDMA0_BASE__INST2_SEG2: u32 = 0;
pub const SDMA0_BASE__INST2_SEG3: u32 = 0;
pub const SDMA0_BASE__INST2_SEG4: u32 = 0;
pub const SDMA0_BASE__INST3_SEG0: u32 = 0;
pub const SDMA0_BASE__INST3_SEG1: u32 = 0;
pub const SDMA0_BASE__INST3_SEG2: u32 = 0;
pub const SDMA0_BASE__INST3_SEG3: u32 = 0;
pub const SDMA0_BASE__INST3_SEG4: u32 = 0;
pub const SDMA0_BASE__INST4_SEG0: u32 = 0;
pub const SDMA0_BASE__INST4_SEG1: u32 = 0;
pub const SDMA0_BASE__INST4_SEG2: u32 = 0;
pub const SDMA0_BASE__INST4_SEG3: u32 = 0;
pub const SDMA0_BASE__INST4_SEG4: u32 = 0;
pub const SDMA0_BASE__INST5_SEG0: u32 = 0;
pub const SDMA0_BASE__INST5_SEG1: u32 = 0;
pub const SDMA0_BASE__INST5_SEG2: u32 = 0;
pub const SDMA0_BASE__INST5_SEG3: u32 = 0;
pub const SDMA0_BASE__INST5_SEG4: u32 = 0;
pub const SDMA0_BASE__INST6_SEG0: u32 = 0;
pub const SDMA0_BASE__INST6_SEG1: u32 = 0;
pub const SDMA0_BASE__INST6_SEG2: u32 = 0;
pub const SDMA0_BASE__INST6_SEG3: u32 = 0;
pub const SDMA0_BASE__INST6_SEG4: u32 = 0;
pub const SMUIO_BASE__INST0_SEG0: u32 = 0x00016800;
pub const SMUIO_BASE__INST0_SEG1: u32 = 0x00016A00;
pub const SMUIO_BASE__INST0_SEG2: u32 = 0x02401000;
pub const SMUIO_BASE__INST0_SEG3: u32 = 0x00440000;
pub const SMUIO_BASE__INST0_SEG4: u32 = 0;
pub const SMUIO_BASE__INST1_SEG0: u32 = 0;
pub const SMUIO_BASE__INST1_SEG1: u32 = 0;
pub const SMUIO_BASE__INST1_SEG2: u32 = 0;
pub const SMUIO_BASE__INST1_SEG3: u32 = 0;
pub const SMUIO_BASE__INST1_SEG4: u32 = 0;
pub const SMUIO_BASE__INST2_SEG0: u32 = 0;
pub const SMUIO_BASE__INST2_SEG1: u32 = 0;
pub const SMUIO_BASE__INST2_SEG2: u32 = 0;
pub const SMUIO_BASE__INST2_SEG3: u32 = 0;
pub const SMUIO_BASE__INST2_SEG4: u32 = 0;
pub const SMUIO_BASE__INST3_SEG0: u32 = 0;
pub const SMUIO_BASE__INST3_SEG1: u32 = 0;
pub const SMUIO_BASE__INST3_SEG2: u32 = 0;
pub const SMUIO_BASE__INST3_SEG3: u32 = 0;
pub const SMUIO_BASE__INST3_SEG4: u32 = 0;
pub const SMUIO_BASE__INST4_SEG0: u32 = 0;
pub const SMUIO_BASE__INST4_SEG1: u32 = 0;
pub const SMUIO_BASE__INST4_SEG2: u32 = 0;
pub const SMUIO_BASE__INST4_SEG3: u32 = 0;
pub const SMUIO_BASE__INST4_SEG4: u32 = 0;
pub const SMUIO_BASE__INST5_SEG0: u32 = 0;
pub const SMUIO_BASE__INST5_SEG1: u32 = 0;
pub const SMUIO_BASE__INST5_SEG2: u32 = 0;
pub const SMUIO_BASE__INST5_SEG3: u32 = 0;
pub const SMUIO_BASE__INST5_SEG4: u32 = 0;
pub const SMUIO_BASE__INST6_SEG0: u32 = 0;
pub const SMUIO_BASE__INST6_SEG1: u32 = 0;
pub const SMUIO_BASE__INST6_SEG2: u32 = 0;
pub const SMUIO_BASE__INST6_SEG3: u32 = 0;
pub const SMUIO_BASE__INST6_SEG4: u32 = 0;
pub const THM_BASE__INST0_SEG0: u32 = 0x00016600;
pub const THM_BASE__INST0_SEG1: u32 = 0x02400C00;
pub const THM_BASE__INST0_SEG2: u32 = 0;
pub const THM_BASE__INST0_SEG3: u32 = 0;
pub const THM_BASE__INST0_SEG4: u32 = 0;
pub const THM_BASE__INST1_SEG0: u32 = 0;
pub const THM_BASE__INST1_SEG1: u32 = 0;
pub const THM_BASE__INST1_SEG2: u32 = 0;
pub const THM_BASE__INST1_SEG3: u32 = 0;
pub const THM_BASE__INST1_SEG4: u32 = 0;
pub const THM_BASE__INST2_SEG0: u32 = 0;
pub const THM_BASE__INST2_SEG1: u32 = 0;
pub const THM_BASE__INST2_SEG2: u32 = 0;
pub const THM_BASE__INST2_SEG3: u32 = 0;
pub const THM_BASE__INST2_SEG4: u32 = 0;
pub const THM_BASE__INST3_SEG0: u32 = 0;
pub const THM_BASE__INST3_SEG1: u32 = 0;
pub const THM_BASE__INST3_SEG2: u32 = 0;
pub const THM_BASE__INST3_SEG3: u32 = 0;
pub const THM_BASE__INST3_SEG4: u32 = 0;
pub const THM_BASE__INST4_SEG0: u32 = 0;
pub const THM_BASE__INST4_SEG1: u32 = 0;
pub const THM_BASE__INST4_SEG2: u32 = 0;
pub const THM_BASE__INST4_SEG3: u32 = 0;
pub const THM_BASE__INST4_SEG4: u32 = 0;
pub const THM_BASE__INST5_SEG0: u32 = 0;
pub const THM_BASE__INST5_SEG1: u32 = 0;
pub const THM_BASE__INST5_SEG2: u32 = 0;
pub const THM_BASE__INST5_SEG3: u32 = 0;
pub const THM_BASE__INST5_SEG4: u32 = 0;
pub const THM_BASE__INST6_SEG0: u32 = 0;
pub const THM_BASE__INST6_SEG1: u32 = 0;
pub const THM_BASE__INST6_SEG2: u32 = 0;
pub const THM_BASE__INST6_SEG3: u32 = 0;
pub const THM_BASE__INST6_SEG4: u32 = 0;
pub const UMC_BASE__INST0_SEG0: u32 = 0x00014000;
pub const UMC_BASE__INST0_SEG1: u32 = 0x02425800;
pub const UMC_BASE__INST0_SEG2: u32 = 0;
pub const UMC_BASE__INST0_SEG3: u32 = 0;
pub const UMC_BASE__INST0_SEG4: u32 = 0;
pub const UMC_BASE__INST1_SEG0: u32 = 0x00054000;
pub const UMC_BASE__INST1_SEG1: u32 = 0x02425C00;
pub const UMC_BASE__INST1_SEG2: u32 = 0;
pub const UMC_BASE__INST1_SEG3: u32 = 0;
pub const UMC_BASE__INST1_SEG4: u32 = 0;
pub const UMC_BASE__INST2_SEG0: u32 = 0;
pub const UMC_BASE__INST2_SEG1: u32 = 0;
pub const UMC_BASE__INST2_SEG2: u32 = 0;
pub const UMC_BASE__INST2_SEG3: u32 = 0;
pub const UMC_BASE__INST2_SEG4: u32 = 0;
pub const UMC_BASE__INST3_SEG0: u32 = 0;
pub const UMC_BASE__INST3_SEG1: u32 = 0;
pub const UMC_BASE__INST3_SEG2: u32 = 0;
pub const UMC_BASE__INST3_SEG3: u32 = 0;
pub const UMC_BASE__INST3_SEG4: u32 = 0;
pub const UMC_BASE__INST4_SEG0: u32 = 0;
pub const UMC_BASE__INST4_SEG1: u32 = 0;
pub const UMC_BASE__INST4_SEG2: u32 = 0;
pub const UMC_BASE__INST4_SEG3: u32 = 0;
pub const UMC_BASE__INST4_SEG4: u32 = 0;
pub const UMC_BASE__INST5_SEG0: u32 = 0;
pub const UMC_BASE__INST5_SEG1: u32 = 0;
pub const UMC_BASE__INST5_SEG2: u32 = 0;
pub const UMC_BASE__INST5_SEG3: u32 = 0;
pub const UMC_BASE__INST5_SEG4: u32 = 0;
pub const UMC_BASE__INST6_SEG0: u32 = 0;
pub const UMC_BASE__INST6_SEG1: u32 = 0;
pub const UMC_BASE__INST6_SEG2: u32 = 0;
pub const UMC_BASE__INST6_SEG3: u32 = 0;
pub const UMC_BASE__INST6_SEG4: u32 = 0;
pub const USB0_BASE__INST0_SEG0: u32 = 0x0242A800;
pub const USB0_BASE__INST0_SEG1: u32 = 0x05B00000;
pub const USB0_BASE__INST0_SEG2: u32 = 0;
pub const USB0_BASE__INST0_SEG3: u32 = 0;
pub const USB0_BASE__INST0_SEG4: u32 = 0;
pub const USB0_BASE__INST1_SEG0: u32 = 0;
pub const USB0_BASE__INST1_SEG1: u32 = 0;
pub const USB0_BASE__INST1_SEG2: u32 = 0;
pub const USB0_BASE__INST1_SEG3: u32 = 0;
pub const USB0_BASE__INST1_SEG4: u32 = 0;
pub const USB0_BASE__INST2_SEG0: u32 = 0;
pub const USB0_BASE__INST2_SEG1: u32 = 0;
pub const USB0_BASE__INST2_SEG2: u32 = 0;
pub const USB0_BASE__INST2_SEG3: u32 = 0;
pub const USB0_BASE__INST2_SEG4: u32 = 0;
pub const USB0_BASE__INST3_SEG0: u32 = 0;
pub const USB0_BASE__INST3_SEG1: u32 = 0;
pub const USB0_BASE__INST3_SEG2: u32 = 0;
pub const USB0_BASE__INST3_SEG3: u32 = 0;
pub const USB0_BASE__INST3_SEG4: u32 = 0;
pub const USB0_BASE__INST4_SEG0: u32 = 0;
pub const USB0_BASE__INST4_SEG1: u32 = 0;
pub const USB0_BASE__INST4_SEG2: u32 = 0;
pub const USB0_BASE__INST4_SEG3: u32 = 0;
pub const USB0_BASE__INST4_SEG4: u32 = 0;
pub const USB0_BASE__INST5_SEG0: u32 = 0;
pub const USB0_BASE__INST5_SEG1: u32 = 0;
pub const USB0_BASE__INST5_SEG2: u32 = 0;
pub const USB0_BASE__INST5_SEG3: u32 = 0;
pub const USB0_BASE__INST5_SEG4: u32 = 0;
pub const USB0_BASE__INST6_SEG0: u32 = 0;
pub const USB0_BASE__INST6_SEG1: u32 = 0;
pub const USB0_BASE__INST6_SEG2: u32 = 0;
pub const USB0_BASE__INST6_SEG3: u32 = 0;
pub const USB0_BASE__INST6_SEG4: u32 = 0;
pub const UVD0_BASE__INST0_SEG0: u32 = 0x00007800;
pub const UVD0_BASE__INST0_SEG1: u32 = 0x00007E00;
pub const UVD0_BASE__INST0_SEG2: u32 = 0x02403000;
pub const UVD0_BASE__INST0_SEG3: u32 = 0;
pub const UVD0_BASE__INST0_SEG4: u32 = 0;
pub const UVD0_BASE__INST1_SEG0: u32 = 0;
pub const UVD0_BASE__INST1_SEG1: u32 = 0;
pub const UVD0_BASE__INST1_SEG2: u32 = 0;
pub const UVD0_BASE__INST1_SEG3: u32 = 0;
pub const UVD0_BASE__INST1_SEG4: u32 = 0;
pub const UVD0_BASE__INST2_SEG0: u32 = 0;
pub const UVD0_BASE__INST2_SEG1: u32 = 0;
pub const UVD0_BASE__INST2_SEG2: u32 = 0;
pub const UVD0_BASE__INST2_SEG3: u32 = 0;
pub const UVD0_BASE__INST2_SEG4: u32 = 0;
pub const UVD0_BASE__INST3_SEG0: u32 = 0;
pub const UVD0_BASE__INST3_SEG1: u32 = 0;
pub const UVD0_BASE__INST3_SEG2: u32 = 0;
pub const UVD0_BASE__INST3_SEG3: u32 = 0;
pub const UVD0_BASE__INST3_SEG4: u32 = 0;
pub const UVD0_BASE__INST4_SEG0: u32 = 0;
pub const UVD0_BASE__INST4_SEG1: u32 = 0;
pub const UVD0_BASE__INST4_SEG2: u32 = 0;
pub const UVD0_BASE__INST4_SEG3: u32 = 0;
pub const UVD0_BASE__INST4_SEG4: u32 = 0;
pub const UVD0_BASE__INST5_SEG0: u32 = 0;
pub const UVD0_BASE__INST5_SEG1: u32 = 0;
pub const UVD0_BASE__INST5_SEG2: u32 = 0;
pub const UVD0_BASE__INST5_SEG3: u32 = 0;
pub const UVD0_BASE__INST5_SEG4: u32 = 0;
pub const UVD0_BASE__INST6_SEG0: u32 = 0;
pub const UVD0_BASE__INST6_SEG1: u32 = 0;
pub const UVD0_BASE__INST6_SEG2: u32 = 0;
pub const UVD0_BASE__INST6_SEG3: u32 = 0;
pub const UVD0_BASE__INST6_SEG4: u32 = 0;
pub const DCN_BASE__INST0_SEG0: u32 = 0x00000012;
pub const DCN_BASE__INST0_SEG1: u32 = 0x000000C0;
pub const DCN_BASE__INST0_SEG2: u32 = 0x000034C0;
pub const DCN_BASE__INST0_SEG3: u32 = 0;
pub const DCN_BASE__INST0_SEG4: u32 = 0;
pub const DCN_BASE__INST1_SEG0: u32 = 0;
pub const DCN_BASE__INST1_SEG1: u32 = 0;
pub const DCN_BASE__INST1_SEG2: u32 = 0;
pub const DCN_BASE__INST1_SEG3: u32 = 0;
pub const DCN_BASE__INST1_SEG4: u32 = 0;
pub const DCN_BASE__INST2_SEG0: u32 = 0;
pub const DCN_BASE__INST2_SEG1: u32 = 0;
pub const DCN_BASE__INST2_SEG2: u32 = 0;
pub const DCN_BASE__INST2_SEG3: u32 = 0;
pub const DCN_BASE__INST2_SEG4: u32 = 0;
pub const DCN_BASE__INST3_SEG0: u32 = 0;
pub const DCN_BASE__INST3_SEG1: u32 = 0;
pub const DCN_BASE__INST3_SEG2: u32 = 0;
pub const DCN_BASE__INST3_SEG3: u32 = 0;
pub const DCN_BASE__INST3_SEG4: u32 = 0;
pub const DCN_BASE__INST4_SEG0: u32 = 0;
pub const DCN_BASE__INST4_SEG1: u32 = 0;
pub const DCN_BASE__INST4_SEG2: u32 = 0;
pub const DCN_BASE__INST4_SEG3: u32 = 0;
pub const DCN_BASE__INST4_SEG4: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
