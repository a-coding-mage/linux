/* Copyright (C) 2018 Advanced Micro Devices, Inc. */
/* Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software. */
/* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED. */

pub const MAX_INSTANCE: usize = 6;
pub const MAX_SEGMENT: usize = 5;
#[repr(C)] #[derive(Copy, Clone)] pub struct IP_BASE_INSTANCE { pub segment: [u32; MAX_SEGMENT] }
#[repr(C)] #[derive(Copy, Clone)] pub struct IP_BASE { pub instance: [IP_BASE_INSTANCE; MAX_INSTANCE] }

pub const ATHUB_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000C00, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const CLK_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016C00, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00016E00, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00017000, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00017200, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00017E00, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x0001B000, 0, 0, 0, 0] },
] };

pub const DF_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00007000, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const DMU_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000012, 0x000000C0, 0x000034C0, 0x00009000, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const FUSE_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00017400, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const GC_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00001260, 0x0000A000, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const HDP_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000F20, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const MMHUB_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x0001A000, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const MP0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016000, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const MP1_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016000, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const NBIO_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000000, 0x00000014, 0x00000D20, 0x00010400, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const OSSSYS_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x000010A0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const SMUIO_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016800, 0x00016A00, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const THM_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016600, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const UMC0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00014000, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const UVD0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00007800, 0x00007E00, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0] },
] };

pub const MAX_INSTANCE: u32 = 6;
pub const MAX_SEGMENT: u32 = 5;
pub const ATHUB_BASE__INST0_SEG0: u32 = 0x00000C00;
pub const ATHUB_BASE__INST0_SEG1: u32 = 0;
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
pub const CLK_BASE__INST0_SEG0: u32 = 0x00016C00;
pub const CLK_BASE__INST0_SEG1: u32 = 0;
pub const CLK_BASE__INST0_SEG2: u32 = 0;
pub const CLK_BASE__INST0_SEG3: u32 = 0;
pub const CLK_BASE__INST0_SEG4: u32 = 0;
pub const CLK_BASE__INST1_SEG0: u32 = 0x00016E00;
pub const CLK_BASE__INST1_SEG1: u32 = 0;
pub const CLK_BASE__INST1_SEG2: u32 = 0;
pub const CLK_BASE__INST1_SEG3: u32 = 0;
pub const CLK_BASE__INST1_SEG4: u32 = 0;
pub const CLK_BASE__INST2_SEG0: u32 = 0x00017000;
pub const CLK_BASE__INST2_SEG1: u32 = 0;
pub const CLK_BASE__INST2_SEG2: u32 = 0;
pub const CLK_BASE__INST2_SEG3: u32 = 0;
pub const CLK_BASE__INST2_SEG4: u32 = 0;
pub const CLK_BASE__INST3_SEG0: u32 = 0x00017200;
pub const CLK_BASE__INST3_SEG1: u32 = 0;
pub const CLK_BASE__INST3_SEG2: u32 = 0;
pub const CLK_BASE__INST3_SEG3: u32 = 0;
pub const CLK_BASE__INST3_SEG4: u32 = 0;
pub const CLK_BASE__INST4_SEG0: u32 = 0x00017E00;
pub const CLK_BASE__INST4_SEG1: u32 = 0;
pub const CLK_BASE__INST4_SEG2: u32 = 0;
pub const CLK_BASE__INST4_SEG3: u32 = 0;
pub const CLK_BASE__INST4_SEG4: u32 = 0;
pub const CLK_BASE__INST5_SEG0: u32 = 0x0001B000;
pub const CLK_BASE__INST5_SEG1: u32 = 0;
pub const CLK_BASE__INST5_SEG2: u32 = 0;
pub const CLK_BASE__INST5_SEG3: u32 = 0;
pub const CLK_BASE__INST5_SEG4: u32 = 0;
pub const DF_BASE__INST0_SEG0: u32 = 0x00007000;
pub const DF_BASE__INST0_SEG1: u32 = 0;
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
pub const DMU_BASE__INST0_SEG0: u32 = 0x00000012;
pub const DMU_BASE__INST0_SEG1: u32 = 0x000000C0;
pub const DMU_BASE__INST0_SEG2: u32 = 0x000034C0;
pub const DMU_BASE__INST0_SEG3: u32 = 0x00009000;
pub const DMU_BASE__INST0_SEG4: u32 = 0;
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
pub const FUSE_BASE__INST0_SEG0: u32 = 0x00017400;
pub const FUSE_BASE__INST0_SEG1: u32 = 0;
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
pub const GC_BASE__INST0_SEG0: u32 = 0x00001260;
pub const GC_BASE__INST0_SEG1: u32 = 0x0000A000;
pub const GC_BASE__INST0_SEG2: u32 = 0;
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
pub const HDP_BASE__INST0_SEG0: u32 = 0x00000F20;
pub const HDP_BASE__INST0_SEG1: u32 = 0;
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
pub const MMHUB_BASE__INST0_SEG0: u32 = 0x0001A000;
pub const MMHUB_BASE__INST0_SEG1: u32 = 0;
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
pub const MP0_BASE__INST0_SEG0: u32 = 0x00016000;
pub const MP0_BASE__INST0_SEG1: u32 = 0;
pub const MP0_BASE__INST0_SEG2: u32 = 0;
pub const MP0_BASE__INST0_SEG3: u32 = 0;
pub const MP0_BASE__INST0_SEG4: u32 = 0;
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
pub const MP1_BASE__INST0_SEG0: u32 = 0x00016000;
pub const MP1_BASE__INST0_SEG1: u32 = 0;
pub const MP1_BASE__INST0_SEG2: u32 = 0;
pub const MP1_BASE__INST0_SEG3: u32 = 0;
pub const MP1_BASE__INST0_SEG4: u32 = 0;
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
pub const NBIO_BASE__INST0_SEG0: u32 = 0x00000000;
pub const NBIO_BASE__INST0_SEG1: u32 = 0x00000014;
pub const NBIO_BASE__INST0_SEG2: u32 = 0x00000D20;
pub const NBIO_BASE__INST0_SEG3: u32 = 0x00010400;
pub const NBIO_BASE__INST0_SEG4: u32 = 0;
pub const NBIO_BASE__INST1_SEG0: u32 = 0;
pub const NBIO_BASE__INST1_SEG1: u32 = 0;
pub const NBIO_BASE__INST1_SEG2: u32 = 0;
pub const NBIO_BASE__INST1_SEG3: u32 = 0;
pub const NBIO_BASE__INST1_SEG4: u32 = 0;
pub const NBIO_BASE__INST2_SEG0: u32 = 0;
pub const NBIO_BASE__INST2_SEG1: u32 = 0;
pub const NBIO_BASE__INST2_SEG2: u32 = 0;
pub const NBIO_BASE__INST2_SEG3: u32 = 0;
pub const NBIO_BASE__INST2_SEG4: u32 = 0;
pub const NBIO_BASE__INST3_SEG0: u32 = 0;
pub const NBIO_BASE__INST3_SEG1: u32 = 0;
pub const NBIO_BASE__INST3_SEG2: u32 = 0;
pub const NBIO_BASE__INST3_SEG3: u32 = 0;
pub const NBIO_BASE__INST3_SEG4: u32 = 0;
pub const NBIO_BASE__INST4_SEG0: u32 = 0;
pub const NBIO_BASE__INST4_SEG1: u32 = 0;
pub const NBIO_BASE__INST4_SEG2: u32 = 0;
pub const NBIO_BASE__INST4_SEG3: u32 = 0;
pub const NBIO_BASE__INST4_SEG4: u32 = 0;
pub const NBIO_BASE__INST5_SEG0: u32 = 0;
pub const NBIO_BASE__INST5_SEG1: u32 = 0;
pub const NBIO_BASE__INST5_SEG2: u32 = 0;
pub const NBIO_BASE__INST5_SEG3: u32 = 0;
pub const NBIO_BASE__INST5_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST0_SEG0: u32 = 0x000010A0;
pub const OSSSYS_BASE__INST0_SEG1: u32 = 0;
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
pub const SMUIO_BASE__INST0_SEG0: u32 = 0x00016800;
pub const SMUIO_BASE__INST0_SEG1: u32 = 0x00016A00;
pub const SMUIO_BASE__INST0_SEG2: u32 = 0;
pub const SMUIO_BASE__INST0_SEG3: u32 = 0;
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
pub const THM_BASE__INST0_SEG0: u32 = 0x00016600;
pub const THM_BASE__INST0_SEG1: u32 = 0;
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
pub const UMC0_BASE__INST0_SEG0: u32 = 0x00014000;
pub const UMC0_BASE__INST0_SEG1: u32 = 0;
pub const UMC0_BASE__INST0_SEG2: u32 = 0;
pub const UMC0_BASE__INST0_SEG3: u32 = 0;
pub const UMC0_BASE__INST0_SEG4: u32 = 0;
pub const UMC0_BASE__INST1_SEG0: u32 = 0;
pub const UMC0_BASE__INST1_SEG1: u32 = 0;
pub const UMC0_BASE__INST1_SEG2: u32 = 0;
pub const UMC0_BASE__INST1_SEG3: u32 = 0;
pub const UMC0_BASE__INST1_SEG4: u32 = 0;
pub const UMC0_BASE__INST2_SEG0: u32 = 0;
pub const UMC0_BASE__INST2_SEG1: u32 = 0;
pub const UMC0_BASE__INST2_SEG2: u32 = 0;
pub const UMC0_BASE__INST2_SEG3: u32 = 0;
pub const UMC0_BASE__INST2_SEG4: u32 = 0;
pub const UMC0_BASE__INST3_SEG0: u32 = 0;
pub const UMC0_BASE__INST3_SEG1: u32 = 0;
pub const UMC0_BASE__INST3_SEG2: u32 = 0;
pub const UMC0_BASE__INST3_SEG3: u32 = 0;
pub const UMC0_BASE__INST3_SEG4: u32 = 0;
pub const UMC0_BASE__INST4_SEG0: u32 = 0;
pub const UMC0_BASE__INST4_SEG1: u32 = 0;
pub const UMC0_BASE__INST4_SEG2: u32 = 0;
pub const UMC0_BASE__INST4_SEG3: u32 = 0;
pub const UMC0_BASE__INST4_SEG4: u32 = 0;
pub const UMC0_BASE__INST5_SEG0: u32 = 0;
pub const UMC0_BASE__INST5_SEG1: u32 = 0;
pub const UMC0_BASE__INST5_SEG2: u32 = 0;
pub const UMC0_BASE__INST5_SEG3: u32 = 0;
pub const UMC0_BASE__INST5_SEG4: u32 = 0;
pub const UVD0_BASE__INST0_SEG0: u32 = 0x00007800;
pub const UVD0_BASE__INST0_SEG1: u32 = 0x00007E00;
pub const UVD0_BASE__INST0_SEG2: u32 = 0;
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

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
