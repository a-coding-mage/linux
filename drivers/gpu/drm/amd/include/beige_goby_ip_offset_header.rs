/*
 * Copyright (C) 2020  Advanced Micro Devices, Inc.
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
pub const MAX_INSTANCE: u32 = 7;
pub const MAX_SEGMENT: u32 = 6;
#[repr(C)]
pub struct IP_BASE_INSTANCE {
    pub segment: [u32; MAX_SEGMENT as usize],
}

#[repr(C)]
pub struct IP_BASE {
    pub instance: [IP_BASE_INSTANCE; MAX_INSTANCE as usize],
}


pub static ATHUB_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00000C00, 0x02408C00, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static CLK_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00016C00, 0x02401800, 0, 0, 0, 0 ] ],
                                        [ [ 0x00016E00, 0x02401C00, 0, 0, 0, 0 ] ],
                                        [ [ 0x00017000, 0x02402000, 0, 0, 0, 0 ] ],
                                        [ [ 0x00017200, 0x02402400, 0, 0, 0, 0 ] ],
                                        [ [ 0x0001B000, 0x0242D800, 0, 0, 0, 0 ] ],
                                        [ [ 0x0001B200, 0x0242DC00, 0, 0, 0, 0 ] ],
                                        [ [ 0x00017E00, 0x0240BC00, 0, 0, 0, 0 ] ] ] ] };
pub static DBGU_IO0_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x000001E0, 0x0240B400, 0, 0, 0, 0 ] ],
                                        [ [ 0x00000260, 0x02413C00, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static DF_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00007000, 0x0240B800, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static DIO_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x02404000, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static DCN_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00000012, 0x000000C0, 0x000034C0, 0x00009000, 0x02403C00, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static DPCS_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00000012, 0x000000C0, 0x000034C0, 0x00009000, 0x02403C00, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static FUSE_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00017400, 0x02401400, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static GC_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00001260, 0x0000A000, 0x0001C000, 0x02402C00, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static HDA_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x004C0000, 0x02404800, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static HDP_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00000F20, 0x0240A400, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static MMHUB_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x0001A000, 0x02408800, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static MP0_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00016000, 0x00DC0000, 0x00E00000, 0x00E40000, 0x0243FC00, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static MP1_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00016000, 0x00DC0000, 0x00E00000, 0x00E40000, 0x0243FC00, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static NBIO_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00000000, 0x00000014, 0x00000D20, 0x00010400, 0x0241B000, 0x04040000 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static OSSSYS_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x000010A0, 0x0240A000, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static PCIE0_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00000000, 0x00000014, 0x00000D20, 0x00010400, 0x0241B000, 0x04040000 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static SDMA0_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00001260, 0x0000A000, 0x0001C000, 0x02402C00, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static SMUIO_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00016800, 0x00016A00, 0x00440000, 0x02401000, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static THM_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00016600, 0x02400C00, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static UMC_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00014000, 0x02425800, 0, 0, 0, 0 ] ],
                                        [ [ 0x00054000, 0x02425C00, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };
pub static VCN0_BASE: IP_BASE = IP_BASE { instance: [ [ [ [ 0x00007800, 0x00007E00, 0x02403000, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ],
                                        [ [ 0, 0, 0, 0, 0, 0 ] ] ] ] };


pub const ATHUB_BASE__INST0_SEG0: u32 = 0x00000C00;
pub const ATHUB_BASE__INST0_SEG1: u32 = 0x02408C00;
pub const ATHUB_BASE__INST0_SEG2: u32 = 0;
pub const ATHUB_BASE__INST0_SEG3: u32 = 0;
pub const ATHUB_BASE__INST0_SEG4: u32 = 0;
pub const ATHUB_BASE__INST0_SEG5: u32 = 0;
pub const ATHUB_BASE__INST1_SEG0: u32 = 0;
pub const ATHUB_BASE__INST1_SEG1: u32 = 0;
pub const ATHUB_BASE__INST1_SEG2: u32 = 0;
pub const ATHUB_BASE__INST1_SEG3: u32 = 0;
pub const ATHUB_BASE__INST1_SEG4: u32 = 0;
pub const ATHUB_BASE__INST1_SEG5: u32 = 0;
pub const ATHUB_BASE__INST2_SEG0: u32 = 0;
pub const ATHUB_BASE__INST2_SEG1: u32 = 0;
pub const ATHUB_BASE__INST2_SEG2: u32 = 0;
pub const ATHUB_BASE__INST2_SEG3: u32 = 0;
pub const ATHUB_BASE__INST2_SEG4: u32 = 0;
pub const ATHUB_BASE__INST2_SEG5: u32 = 0;
pub const ATHUB_BASE__INST3_SEG0: u32 = 0;
pub const ATHUB_BASE__INST3_SEG1: u32 = 0;
pub const ATHUB_BASE__INST3_SEG2: u32 = 0;
pub const ATHUB_BASE__INST3_SEG3: u32 = 0;
pub const ATHUB_BASE__INST3_SEG4: u32 = 0;
pub const ATHUB_BASE__INST3_SEG5: u32 = 0;
pub const ATHUB_BASE__INST4_SEG0: u32 = 0;
pub const ATHUB_BASE__INST4_SEG1: u32 = 0;
pub const ATHUB_BASE__INST4_SEG2: u32 = 0;
pub const ATHUB_BASE__INST4_SEG3: u32 = 0;
pub const ATHUB_BASE__INST4_SEG4: u32 = 0;
pub const ATHUB_BASE__INST4_SEG5: u32 = 0;
pub const ATHUB_BASE__INST5_SEG0: u32 = 0;
pub const ATHUB_BASE__INST5_SEG1: u32 = 0;
pub const ATHUB_BASE__INST5_SEG2: u32 = 0;
pub const ATHUB_BASE__INST5_SEG3: u32 = 0;
pub const ATHUB_BASE__INST5_SEG4: u32 = 0;
pub const ATHUB_BASE__INST5_SEG5: u32 = 0;
pub const ATHUB_BASE__INST6_SEG0: u32 = 0;
pub const ATHUB_BASE__INST6_SEG1: u32 = 0;
pub const ATHUB_BASE__INST6_SEG2: u32 = 0;
pub const ATHUB_BASE__INST6_SEG3: u32 = 0;
pub const ATHUB_BASE__INST6_SEG4: u32 = 0;
pub const ATHUB_BASE__INST6_SEG5: u32 = 0;
pub const CLK_BASE__INST0_SEG0: u32 = 0x00016C00;
pub const CLK_BASE__INST0_SEG1: u32 = 0x02401800;
pub const CLK_BASE__INST0_SEG2: u32 = 0;
pub const CLK_BASE__INST0_SEG3: u32 = 0;
pub const CLK_BASE__INST0_SEG4: u32 = 0;
pub const CLK_BASE__INST0_SEG5: u32 = 0;
pub const CLK_BASE__INST1_SEG0: u32 = 0x00016E00;
pub const CLK_BASE__INST1_SEG1: u32 = 0x02401C00;
pub const CLK_BASE__INST1_SEG2: u32 = 0;
pub const CLK_BASE__INST1_SEG3: u32 = 0;
pub const CLK_BASE__INST1_SEG4: u32 = 0;
pub const CLK_BASE__INST1_SEG5: u32 = 0;
pub const CLK_BASE__INST2_SEG0: u32 = 0x00017000;
pub const CLK_BASE__INST2_SEG1: u32 = 0x02402000;
pub const CLK_BASE__INST2_SEG2: u32 = 0;
pub const CLK_BASE__INST2_SEG3: u32 = 0;
pub const CLK_BASE__INST2_SEG4: u32 = 0;
pub const CLK_BASE__INST2_SEG5: u32 = 0;
pub const CLK_BASE__INST3_SEG0: u32 = 0x00017200;
pub const CLK_BASE__INST3_SEG1: u32 = 0x02402400;
pub const CLK_BASE__INST3_SEG2: u32 = 0;
pub const CLK_BASE__INST3_SEG3: u32 = 0;
pub const CLK_BASE__INST3_SEG4: u32 = 0;
pub const CLK_BASE__INST3_SEG5: u32 = 0;
pub const CLK_BASE__INST4_SEG0: u32 = 0x0001B000;
pub const CLK_BASE__INST4_SEG1: u32 = 0x0242D800;
pub const CLK_BASE__INST4_SEG2: u32 = 0;
pub const CLK_BASE__INST4_SEG3: u32 = 0;
pub const CLK_BASE__INST4_SEG4: u32 = 0;
pub const CLK_BASE__INST4_SEG5: u32 = 0;
pub const CLK_BASE__INST5_SEG0: u32 = 0x0001B200;
pub const CLK_BASE__INST5_SEG1: u32 = 0x0242DC00;
pub const CLK_BASE__INST5_SEG2: u32 = 0;
pub const CLK_BASE__INST5_SEG3: u32 = 0;
pub const CLK_BASE__INST5_SEG4: u32 = 0;
pub const CLK_BASE__INST5_SEG5: u32 = 0;
pub const CLK_BASE__INST6_SEG0: u32 = 0x00017E00;
pub const CLK_BASE__INST6_SEG1: u32 = 0x0240BC00;
pub const CLK_BASE__INST6_SEG2: u32 = 0;
pub const CLK_BASE__INST6_SEG3: u32 = 0;
pub const CLK_BASE__INST6_SEG4: u32 = 0;
pub const CLK_BASE__INST6_SEG5: u32 = 0;
pub const DBGU_IO0_BASE__INST0_SEG0: u32 = 0x000001E0;
pub const DBGU_IO0_BASE__INST0_SEG1: u32 = 0x0240B400;
pub const DBGU_IO0_BASE__INST0_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST0_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST0_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST0_SEG5: u32 = 0;
pub const DBGU_IO0_BASE__INST1_SEG0: u32 = 0x00000260;
pub const DBGU_IO0_BASE__INST1_SEG1: u32 = 0x02413C00;
pub const DBGU_IO0_BASE__INST1_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST1_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST1_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST1_SEG5: u32 = 0;
pub const DBGU_IO0_BASE__INST2_SEG0: u32 = 0;
pub const DBGU_IO0_BASE__INST2_SEG1: u32 = 0;
pub const DBGU_IO0_BASE__INST2_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST2_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST2_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST2_SEG5: u32 = 0;
pub const DBGU_IO0_BASE__INST3_SEG0: u32 = 0;
pub const DBGU_IO0_BASE__INST3_SEG1: u32 = 0;
pub const DBGU_IO0_BASE__INST3_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST3_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST3_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST3_SEG5: u32 = 0;
pub const DBGU_IO0_BASE__INST4_SEG0: u32 = 0;
pub const DBGU_IO0_BASE__INST4_SEG1: u32 = 0;
pub const DBGU_IO0_BASE__INST4_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST4_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST4_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST4_SEG5: u32 = 0;
pub const DBGU_IO0_BASE__INST5_SEG0: u32 = 0;
pub const DBGU_IO0_BASE__INST5_SEG1: u32 = 0;
pub const DBGU_IO0_BASE__INST5_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST5_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST5_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST5_SEG5: u32 = 0;
pub const DBGU_IO0_BASE__INST6_SEG0: u32 = 0;
pub const DBGU_IO0_BASE__INST6_SEG1: u32 = 0;
pub const DBGU_IO0_BASE__INST6_SEG2: u32 = 0;
pub const DBGU_IO0_BASE__INST6_SEG3: u32 = 0;
pub const DBGU_IO0_BASE__INST6_SEG4: u32 = 0;
pub const DBGU_IO0_BASE__INST6_SEG5: u32 = 0;
pub const DF_BASE__INST0_SEG0: u32 = 0x00007000;
pub const DF_BASE__INST0_SEG1: u32 = 0x0240B800;
pub const DF_BASE__INST0_SEG2: u32 = 0;
pub const DF_BASE__INST0_SEG3: u32 = 0;
pub const DF_BASE__INST0_SEG4: u32 = 0;
pub const DF_BASE__INST0_SEG5: u32 = 0;
pub const DF_BASE__INST1_SEG0: u32 = 0;
pub const DF_BASE__INST1_SEG1: u32 = 0;
pub const DF_BASE__INST1_SEG2: u32 = 0;
pub const DF_BASE__INST1_SEG3: u32 = 0;
pub const DF_BASE__INST1_SEG4: u32 = 0;
pub const DF_BASE__INST1_SEG5: u32 = 0;
pub const DF_BASE__INST2_SEG0: u32 = 0;
pub const DF_BASE__INST2_SEG1: u32 = 0;
pub const DF_BASE__INST2_SEG2: u32 = 0;
pub const DF_BASE__INST2_SEG3: u32 = 0;
pub const DF_BASE__INST2_SEG4: u32 = 0;
pub const DF_BASE__INST2_SEG5: u32 = 0;
pub const DF_BASE__INST3_SEG0: u32 = 0;
pub const DF_BASE__INST3_SEG1: u32 = 0;
pub const DF_BASE__INST3_SEG2: u32 = 0;
pub const DF_BASE__INST3_SEG3: u32 = 0;
pub const DF_BASE__INST3_SEG4: u32 = 0;
pub const DF_BASE__INST3_SEG5: u32 = 0;
pub const DF_BASE__INST4_SEG0: u32 = 0;
pub const DF_BASE__INST4_SEG1: u32 = 0;
pub const DF_BASE__INST4_SEG2: u32 = 0;
pub const DF_BASE__INST4_SEG3: u32 = 0;
pub const DF_BASE__INST4_SEG4: u32 = 0;
pub const DF_BASE__INST4_SEG5: u32 = 0;
pub const DF_BASE__INST5_SEG0: u32 = 0;
pub const DF_BASE__INST5_SEG1: u32 = 0;
pub const DF_BASE__INST5_SEG2: u32 = 0;
pub const DF_BASE__INST5_SEG3: u32 = 0;
pub const DF_BASE__INST5_SEG4: u32 = 0;
pub const DF_BASE__INST5_SEG5: u32 = 0;
pub const DF_BASE__INST6_SEG0: u32 = 0;
pub const DF_BASE__INST6_SEG1: u32 = 0;
pub const DF_BASE__INST6_SEG2: u32 = 0;
pub const DF_BASE__INST6_SEG3: u32 = 0;
pub const DF_BASE__INST6_SEG4: u32 = 0;
pub const DF_BASE__INST6_SEG5: u32 = 0;
pub const DIO_BASE__INST0_SEG0: u32 = 0x02404000;
pub const DIO_BASE__INST0_SEG1: u32 = 0;
pub const DIO_BASE__INST0_SEG2: u32 = 0;
pub const DIO_BASE__INST0_SEG3: u32 = 0;
pub const DIO_BASE__INST0_SEG4: u32 = 0;
pub const DIO_BASE__INST0_SEG5: u32 = 0;
pub const DIO_BASE__INST1_SEG0: u32 = 0;
pub const DIO_BASE__INST1_SEG1: u32 = 0;
pub const DIO_BASE__INST1_SEG2: u32 = 0;
pub const DIO_BASE__INST1_SEG3: u32 = 0;
pub const DIO_BASE__INST1_SEG4: u32 = 0;
pub const DIO_BASE__INST1_SEG5: u32 = 0;
pub const DIO_BASE__INST2_SEG0: u32 = 0;
pub const DIO_BASE__INST2_SEG1: u32 = 0;
pub const DIO_BASE__INST2_SEG2: u32 = 0;
pub const DIO_BASE__INST2_SEG3: u32 = 0;
pub const DIO_BASE__INST2_SEG4: u32 = 0;
pub const DIO_BASE__INST2_SEG5: u32 = 0;
pub const DIO_BASE__INST3_SEG0: u32 = 0;
pub const DIO_BASE__INST3_SEG1: u32 = 0;
pub const DIO_BASE__INST3_SEG2: u32 = 0;
pub const DIO_BASE__INST3_SEG3: u32 = 0;
pub const DIO_BASE__INST3_SEG4: u32 = 0;
pub const DIO_BASE__INST3_SEG5: u32 = 0;
pub const DIO_BASE__INST4_SEG0: u32 = 0;
pub const DIO_BASE__INST4_SEG1: u32 = 0;
pub const DIO_BASE__INST4_SEG2: u32 = 0;
pub const DIO_BASE__INST4_SEG3: u32 = 0;
pub const DIO_BASE__INST4_SEG4: u32 = 0;
pub const DIO_BASE__INST4_SEG5: u32 = 0;
pub const DIO_BASE__INST5_SEG0: u32 = 0;
pub const DIO_BASE__INST5_SEG1: u32 = 0;
pub const DIO_BASE__INST5_SEG2: u32 = 0;
pub const DIO_BASE__INST5_SEG3: u32 = 0;
pub const DIO_BASE__INST5_SEG4: u32 = 0;
pub const DIO_BASE__INST5_SEG5: u32 = 0;
pub const DIO_BASE__INST6_SEG0: u32 = 0;
pub const DIO_BASE__INST6_SEG1: u32 = 0;
pub const DIO_BASE__INST6_SEG2: u32 = 0;
pub const DIO_BASE__INST6_SEG3: u32 = 0;
pub const DIO_BASE__INST6_SEG4: u32 = 0;
pub const DIO_BASE__INST6_SEG5: u32 = 0;
pub const DCN_BASE__INST0_SEG0: u32 = 0x00000012;
pub const DCN_BASE__INST0_SEG1: u32 = 0x000000C0;
pub const DCN_BASE__INST0_SEG2: u32 = 0x000034C0;
pub const DCN_BASE__INST0_SEG3: u32 = 0x00009000;
pub const DCN_BASE__INST0_SEG4: u32 = 0x02403C00;
pub const DCN_BASE__INST0_SEG5: u32 = 0;
pub const DCN_BASE__INST1_SEG0: u32 = 0;
pub const DCN_BASE__INST1_SEG1: u32 = 0;
pub const DCN_BASE__INST1_SEG2: u32 = 0;
pub const DCN_BASE__INST1_SEG3: u32 = 0;
pub const DCN_BASE__INST1_SEG4: u32 = 0;
pub const DCN_BASE__INST1_SEG5: u32 = 0;
pub const DCN_BASE__INST2_SEG0: u32 = 0;
pub const DCN_BASE__INST2_SEG1: u32 = 0;
pub const DCN_BASE__INST2_SEG2: u32 = 0;
pub const DCN_BASE__INST2_SEG3: u32 = 0;
pub const DCN_BASE__INST2_SEG4: u32 = 0;
pub const DCN_BASE__INST2_SEG5: u32 = 0;
pub const DCN_BASE__INST3_SEG0: u32 = 0;
pub const DCN_BASE__INST3_SEG1: u32 = 0;
pub const DCN_BASE__INST3_SEG2: u32 = 0;
pub const DCN_BASE__INST3_SEG3: u32 = 0;
pub const DCN_BASE__INST3_SEG4: u32 = 0;
pub const DCN_BASE__INST3_SEG5: u32 = 0;
pub const DCN_BASE__INST4_SEG0: u32 = 0;
pub const DCN_BASE__INST4_SEG1: u32 = 0;
pub const DCN_BASE__INST4_SEG2: u32 = 0;
pub const DCN_BASE__INST4_SEG3: u32 = 0;
pub const DCN_BASE__INST4_SEG4: u32 = 0;
pub const DCN_BASE__INST4_SEG5: u32 = 0;
pub const DCN_BASE__INST5_SEG0: u32 = 0;
pub const DCN_BASE__INST5_SEG1: u32 = 0;
pub const DCN_BASE__INST5_SEG2: u32 = 0;
pub const DCN_BASE__INST5_SEG3: u32 = 0;
pub const DCN_BASE__INST5_SEG4: u32 = 0;
pub const DCN_BASE__INST5_SEG5: u32 = 0;
pub const DCN_BASE__INST6_SEG0: u32 = 0;
pub const DCN_BASE__INST6_SEG1: u32 = 0;
pub const DCN_BASE__INST6_SEG2: u32 = 0;
pub const DCN_BASE__INST6_SEG3: u32 = 0;
pub const DCN_BASE__INST6_SEG4: u32 = 0;
pub const DCN_BASE__INST6_SEG5: u32 = 0;
pub const DPCS_BASE__INST0_SEG0: u32 = 0x00000012;
pub const DPCS_BASE__INST0_SEG1: u32 = 0x000000C0;
pub const DPCS_BASE__INST0_SEG2: u32 = 0x000034C0;
pub const DPCS_BASE__INST0_SEG3: u32 = 0x00009000;
pub const DPCS_BASE__INST0_SEG4: u32 = 0x02403C00;
pub const DPCS_BASE__INST0_SEG5: u32 = 0;
pub const DPCS_BASE__INST1_SEG0: u32 = 0;
pub const DPCS_BASE__INST1_SEG1: u32 = 0;
pub const DPCS_BASE__INST1_SEG2: u32 = 0;
pub const DPCS_BASE__INST1_SEG3: u32 = 0;
pub const DPCS_BASE__INST1_SEG4: u32 = 0;
pub const DPCS_BASE__INST1_SEG5: u32 = 0;
pub const DPCS_BASE__INST2_SEG0: u32 = 0;
pub const DPCS_BASE__INST2_SEG1: u32 = 0;
pub const DPCS_BASE__INST2_SEG2: u32 = 0;
pub const DPCS_BASE__INST2_SEG3: u32 = 0;
pub const DPCS_BASE__INST2_SEG4: u32 = 0;
pub const DPCS_BASE__INST2_SEG5: u32 = 0;
pub const DPCS_BASE__INST3_SEG0: u32 = 0;
pub const DPCS_BASE__INST3_SEG1: u32 = 0;
pub const DPCS_BASE__INST3_SEG2: u32 = 0;
pub const DPCS_BASE__INST3_SEG3: u32 = 0;
pub const DPCS_BASE__INST3_SEG4: u32 = 0;
pub const DPCS_BASE__INST3_SEG5: u32 = 0;
pub const DPCS_BASE__INST4_SEG0: u32 = 0;
pub const DPCS_BASE__INST4_SEG1: u32 = 0;
pub const DPCS_BASE__INST4_SEG2: u32 = 0;
pub const DPCS_BASE__INST4_SEG3: u32 = 0;
pub const DPCS_BASE__INST4_SEG4: u32 = 0;
pub const DPCS_BASE__INST4_SEG5: u32 = 0;
pub const DPCS_BASE__INST5_SEG0: u32 = 0;
pub const DPCS_BASE__INST5_SEG1: u32 = 0;
pub const DPCS_BASE__INST5_SEG2: u32 = 0;
pub const DPCS_BASE__INST5_SEG3: u32 = 0;
pub const DPCS_BASE__INST5_SEG4: u32 = 0;
pub const DPCS_BASE__INST5_SEG5: u32 = 0;
pub const DPCS_BASE__INST6_SEG0: u32 = 0;
pub const DPCS_BASE__INST6_SEG1: u32 = 0;
pub const DPCS_BASE__INST6_SEG2: u32 = 0;
pub const DPCS_BASE__INST6_SEG3: u32 = 0;
pub const DPCS_BASE__INST6_SEG4: u32 = 0;
pub const DPCS_BASE__INST6_SEG5: u32 = 0;
pub const FUSE_BASE__INST0_SEG0: u32 = 0x00017400;
pub const FUSE_BASE__INST0_SEG1: u32 = 0x02401400;
pub const FUSE_BASE__INST0_SEG2: u32 = 0;
pub const FUSE_BASE__INST0_SEG3: u32 = 0;
pub const FUSE_BASE__INST0_SEG4: u32 = 0;
pub const FUSE_BASE__INST0_SEG5: u32 = 0;
pub const FUSE_BASE__INST1_SEG0: u32 = 0;
pub const FUSE_BASE__INST1_SEG1: u32 = 0;
pub const FUSE_BASE__INST1_SEG2: u32 = 0;
pub const FUSE_BASE__INST1_SEG3: u32 = 0;
pub const FUSE_BASE__INST1_SEG4: u32 = 0;
pub const FUSE_BASE__INST1_SEG5: u32 = 0;
pub const FUSE_BASE__INST2_SEG0: u32 = 0;
pub const FUSE_BASE__INST2_SEG1: u32 = 0;
pub const FUSE_BASE__INST2_SEG2: u32 = 0;
pub const FUSE_BASE__INST2_SEG3: u32 = 0;
pub const FUSE_BASE__INST2_SEG4: u32 = 0;
pub const FUSE_BASE__INST2_SEG5: u32 = 0;
pub const FUSE_BASE__INST3_SEG0: u32 = 0;
pub const FUSE_BASE__INST3_SEG1: u32 = 0;
pub const FUSE_BASE__INST3_SEG2: u32 = 0;
pub const FUSE_BASE__INST3_SEG3: u32 = 0;
pub const FUSE_BASE__INST3_SEG4: u32 = 0;
pub const FUSE_BASE__INST3_SEG5: u32 = 0;
pub const FUSE_BASE__INST4_SEG0: u32 = 0;
pub const FUSE_BASE__INST4_SEG1: u32 = 0;
pub const FUSE_BASE__INST4_SEG2: u32 = 0;
pub const FUSE_BASE__INST4_SEG3: u32 = 0;
pub const FUSE_BASE__INST4_SEG4: u32 = 0;
pub const FUSE_BASE__INST4_SEG5: u32 = 0;
pub const FUSE_BASE__INST5_SEG0: u32 = 0;
pub const FUSE_BASE__INST5_SEG1: u32 = 0;
pub const FUSE_BASE__INST5_SEG2: u32 = 0;
pub const FUSE_BASE__INST5_SEG3: u32 = 0;
pub const FUSE_BASE__INST5_SEG4: u32 = 0;
pub const FUSE_BASE__INST5_SEG5: u32 = 0;
pub const FUSE_BASE__INST6_SEG0: u32 = 0;
pub const FUSE_BASE__INST6_SEG1: u32 = 0;
pub const FUSE_BASE__INST6_SEG2: u32 = 0;
pub const FUSE_BASE__INST6_SEG3: u32 = 0;
pub const FUSE_BASE__INST6_SEG4: u32 = 0;
pub const FUSE_BASE__INST6_SEG5: u32 = 0;
pub const GC_BASE__INST0_SEG0: u32 = 0x00001260;
pub const GC_BASE__INST0_SEG1: u32 = 0x0000A000;
pub const GC_BASE__INST0_SEG2: u32 = 0x0001C000;
pub const GC_BASE__INST0_SEG3: u32 = 0x02402C00;
pub const GC_BASE__INST0_SEG4: u32 = 0;
pub const GC_BASE__INST0_SEG5: u32 = 0;
pub const GC_BASE__INST1_SEG0: u32 = 0;
pub const GC_BASE__INST1_SEG1: u32 = 0;
pub const GC_BASE__INST1_SEG2: u32 = 0;
pub const GC_BASE__INST1_SEG3: u32 = 0;
pub const GC_BASE__INST1_SEG4: u32 = 0;
pub const GC_BASE__INST1_SEG5: u32 = 0;
pub const GC_BASE__INST2_SEG0: u32 = 0;
pub const GC_BASE__INST2_SEG1: u32 = 0;
pub const GC_BASE__INST2_SEG2: u32 = 0;
pub const GC_BASE__INST2_SEG3: u32 = 0;
pub const GC_BASE__INST2_SEG4: u32 = 0;
pub const GC_BASE__INST2_SEG5: u32 = 0;
pub const GC_BASE__INST3_SEG0: u32 = 0;
pub const GC_BASE__INST3_SEG1: u32 = 0;
pub const GC_BASE__INST3_SEG2: u32 = 0;
pub const GC_BASE__INST3_SEG3: u32 = 0;
pub const GC_BASE__INST3_SEG4: u32 = 0;
pub const GC_BASE__INST3_SEG5: u32 = 0;
pub const GC_BASE__INST4_SEG0: u32 = 0;
pub const GC_BASE__INST4_SEG1: u32 = 0;
pub const GC_BASE__INST4_SEG2: u32 = 0;
pub const GC_BASE__INST4_SEG3: u32 = 0;
pub const GC_BASE__INST4_SEG4: u32 = 0;
pub const GC_BASE__INST4_SEG5: u32 = 0;
pub const GC_BASE__INST5_SEG0: u32 = 0;
pub const GC_BASE__INST5_SEG1: u32 = 0;
pub const GC_BASE__INST5_SEG2: u32 = 0;
pub const GC_BASE__INST5_SEG3: u32 = 0;
pub const GC_BASE__INST5_SEG4: u32 = 0;
pub const GC_BASE__INST5_SEG5: u32 = 0;
pub const GC_BASE__INST6_SEG0: u32 = 0;
pub const GC_BASE__INST6_SEG1: u32 = 0;
pub const GC_BASE__INST6_SEG2: u32 = 0;
pub const GC_BASE__INST6_SEG3: u32 = 0;
pub const GC_BASE__INST6_SEG4: u32 = 0;
pub const GC_BASE__INST6_SEG5: u32 = 0;
pub const HDA_BASE__INST0_SEG0: u32 = 0x004C0000;
pub const HDA_BASE__INST0_SEG1: u32 = 0x02404800;
pub const HDA_BASE__INST0_SEG2: u32 = 0;
pub const HDA_BASE__INST0_SEG3: u32 = 0;
pub const HDA_BASE__INST0_SEG4: u32 = 0;
pub const HDA_BASE__INST0_SEG5: u32 = 0;
pub const HDA_BASE__INST1_SEG0: u32 = 0;
pub const HDA_BASE__INST1_SEG1: u32 = 0;
pub const HDA_BASE__INST1_SEG2: u32 = 0;
pub const HDA_BASE__INST1_SEG3: u32 = 0;
pub const HDA_BASE__INST1_SEG4: u32 = 0;
pub const HDA_BASE__INST1_SEG5: u32 = 0;
pub const HDA_BASE__INST2_SEG0: u32 = 0;
pub const HDA_BASE__INST2_SEG1: u32 = 0;
pub const HDA_BASE__INST2_SEG2: u32 = 0;
pub const HDA_BASE__INST2_SEG3: u32 = 0;
pub const HDA_BASE__INST2_SEG4: u32 = 0;
pub const HDA_BASE__INST2_SEG5: u32 = 0;
pub const HDA_BASE__INST3_SEG0: u32 = 0;
pub const HDA_BASE__INST3_SEG1: u32 = 0;
pub const HDA_BASE__INST3_SEG2: u32 = 0;
pub const HDA_BASE__INST3_SEG3: u32 = 0;
pub const HDA_BASE__INST3_SEG4: u32 = 0;
pub const HDA_BASE__INST3_SEG5: u32 = 0;
pub const HDA_BASE__INST4_SEG0: u32 = 0;
pub const HDA_BASE__INST4_SEG1: u32 = 0;
pub const HDA_BASE__INST4_SEG2: u32 = 0;
pub const HDA_BASE__INST4_SEG3: u32 = 0;
pub const HDA_BASE__INST4_SEG4: u32 = 0;
pub const HDA_BASE__INST4_SEG5: u32 = 0;
pub const HDA_BASE__INST5_SEG0: u32 = 0;
pub const HDA_BASE__INST5_SEG1: u32 = 0;
pub const HDA_BASE__INST5_SEG2: u32 = 0;
pub const HDA_BASE__INST5_SEG3: u32 = 0;
pub const HDA_BASE__INST5_SEG4: u32 = 0;
pub const HDA_BASE__INST5_SEG5: u32 = 0;
pub const HDA_BASE__INST6_SEG0: u32 = 0;
pub const HDA_BASE__INST6_SEG1: u32 = 0;
pub const HDA_BASE__INST6_SEG2: u32 = 0;
pub const HDA_BASE__INST6_SEG3: u32 = 0;
pub const HDA_BASE__INST6_SEG4: u32 = 0;
pub const HDA_BASE__INST6_SEG5: u32 = 0;
pub const HDP_BASE__INST0_SEG0: u32 = 0x00000F20;
pub const HDP_BASE__INST0_SEG1: u32 = 0x0240A400;
pub const HDP_BASE__INST0_SEG2: u32 = 0;
pub const HDP_BASE__INST0_SEG3: u32 = 0;
pub const HDP_BASE__INST0_SEG4: u32 = 0;
pub const HDP_BASE__INST0_SEG5: u32 = 0;
pub const HDP_BASE__INST1_SEG0: u32 = 0;
pub const HDP_BASE__INST1_SEG1: u32 = 0;
pub const HDP_BASE__INST1_SEG2: u32 = 0;
pub const HDP_BASE__INST1_SEG3: u32 = 0;
pub const HDP_BASE__INST1_SEG4: u32 = 0;
pub const HDP_BASE__INST1_SEG5: u32 = 0;
pub const HDP_BASE__INST2_SEG0: u32 = 0;
pub const HDP_BASE__INST2_SEG1: u32 = 0;
pub const HDP_BASE__INST2_SEG2: u32 = 0;
pub const HDP_BASE__INST2_SEG3: u32 = 0;
pub const HDP_BASE__INST2_SEG4: u32 = 0;
pub const HDP_BASE__INST2_SEG5: u32 = 0;
pub const HDP_BASE__INST3_SEG0: u32 = 0;
pub const HDP_BASE__INST3_SEG1: u32 = 0;
pub const HDP_BASE__INST3_SEG2: u32 = 0;
pub const HDP_BASE__INST3_SEG3: u32 = 0;
pub const HDP_BASE__INST3_SEG4: u32 = 0;
pub const HDP_BASE__INST3_SEG5: u32 = 0;
pub const HDP_BASE__INST4_SEG0: u32 = 0;
pub const HDP_BASE__INST4_SEG1: u32 = 0;
pub const HDP_BASE__INST4_SEG2: u32 = 0;
pub const HDP_BASE__INST4_SEG3: u32 = 0;
pub const HDP_BASE__INST4_SEG4: u32 = 0;
pub const HDP_BASE__INST4_SEG5: u32 = 0;
pub const HDP_BASE__INST5_SEG0: u32 = 0;
pub const HDP_BASE__INST5_SEG1: u32 = 0;
pub const HDP_BASE__INST5_SEG2: u32 = 0;
pub const HDP_BASE__INST5_SEG3: u32 = 0;
pub const HDP_BASE__INST5_SEG4: u32 = 0;
pub const HDP_BASE__INST5_SEG5: u32 = 0;
pub const HDP_BASE__INST6_SEG0: u32 = 0;
pub const HDP_BASE__INST6_SEG1: u32 = 0;
pub const HDP_BASE__INST6_SEG2: u32 = 0;
pub const HDP_BASE__INST6_SEG3: u32 = 0;
pub const HDP_BASE__INST6_SEG4: u32 = 0;
pub const HDP_BASE__INST6_SEG5: u32 = 0;
pub const MMHUB_BASE__INST0_SEG0: u32 = 0x0001A000;
pub const MMHUB_BASE__INST0_SEG1: u32 = 0x02408800;
pub const MMHUB_BASE__INST0_SEG2: u32 = 0;
pub const MMHUB_BASE__INST0_SEG3: u32 = 0;
pub const MMHUB_BASE__INST0_SEG4: u32 = 0;
pub const MMHUB_BASE__INST0_SEG5: u32 = 0;
pub const MMHUB_BASE__INST1_SEG0: u32 = 0;
pub const MMHUB_BASE__INST1_SEG1: u32 = 0;
pub const MMHUB_BASE__INST1_SEG2: u32 = 0;
pub const MMHUB_BASE__INST1_SEG3: u32 = 0;
pub const MMHUB_BASE__INST1_SEG4: u32 = 0;
pub const MMHUB_BASE__INST1_SEG5: u32 = 0;
pub const MMHUB_BASE__INST2_SEG0: u32 = 0;
pub const MMHUB_BASE__INST2_SEG1: u32 = 0;
pub const MMHUB_BASE__INST2_SEG2: u32 = 0;
pub const MMHUB_BASE__INST2_SEG3: u32 = 0;
pub const MMHUB_BASE__INST2_SEG4: u32 = 0;
pub const MMHUB_BASE__INST2_SEG5: u32 = 0;
pub const MMHUB_BASE__INST3_SEG0: u32 = 0;
pub const MMHUB_BASE__INST3_SEG1: u32 = 0;
pub const MMHUB_BASE__INST3_SEG2: u32 = 0;
pub const MMHUB_BASE__INST3_SEG3: u32 = 0;
pub const MMHUB_BASE__INST3_SEG4: u32 = 0;
pub const MMHUB_BASE__INST3_SEG5: u32 = 0;
pub const MMHUB_BASE__INST4_SEG0: u32 = 0;
pub const MMHUB_BASE__INST4_SEG1: u32 = 0;
pub const MMHUB_BASE__INST4_SEG2: u32 = 0;
pub const MMHUB_BASE__INST4_SEG3: u32 = 0;
pub const MMHUB_BASE__INST4_SEG4: u32 = 0;
pub const MMHUB_BASE__INST4_SEG5: u32 = 0;
pub const MMHUB_BASE__INST5_SEG0: u32 = 0;
pub const MMHUB_BASE__INST5_SEG1: u32 = 0;
pub const MMHUB_BASE__INST5_SEG2: u32 = 0;
pub const MMHUB_BASE__INST5_SEG3: u32 = 0;
pub const MMHUB_BASE__INST5_SEG4: u32 = 0;
pub const MMHUB_BASE__INST5_SEG5: u32 = 0;
pub const MMHUB_BASE__INST6_SEG0: u32 = 0;
pub const MMHUB_BASE__INST6_SEG1: u32 = 0;
pub const MMHUB_BASE__INST6_SEG2: u32 = 0;
pub const MMHUB_BASE__INST6_SEG3: u32 = 0;
pub const MMHUB_BASE__INST6_SEG4: u32 = 0;
pub const MMHUB_BASE__INST6_SEG5: u32 = 0;
pub const MP0_BASE__INST0_SEG0: u32 = 0x00016000;
pub const MP0_BASE__INST0_SEG1: u32 = 0x00DC0000;
pub const MP0_BASE__INST0_SEG2: u32 = 0x00E00000;
pub const MP0_BASE__INST0_SEG3: u32 = 0x00E40000;
pub const MP0_BASE__INST0_SEG4: u32 = 0x0243FC00;
pub const MP0_BASE__INST0_SEG5: u32 = 0;
pub const MP0_BASE__INST1_SEG0: u32 = 0;
pub const MP0_BASE__INST1_SEG1: u32 = 0;
pub const MP0_BASE__INST1_SEG2: u32 = 0;
pub const MP0_BASE__INST1_SEG3: u32 = 0;
pub const MP0_BASE__INST1_SEG4: u32 = 0;
pub const MP0_BASE__INST1_SEG5: u32 = 0;
pub const MP0_BASE__INST2_SEG0: u32 = 0;
pub const MP0_BASE__INST2_SEG1: u32 = 0;
pub const MP0_BASE__INST2_SEG2: u32 = 0;
pub const MP0_BASE__INST2_SEG3: u32 = 0;
pub const MP0_BASE__INST2_SEG4: u32 = 0;
pub const MP0_BASE__INST2_SEG5: u32 = 0;
pub const MP0_BASE__INST3_SEG0: u32 = 0;
pub const MP0_BASE__INST3_SEG1: u32 = 0;
pub const MP0_BASE__INST3_SEG2: u32 = 0;
pub const MP0_BASE__INST3_SEG3: u32 = 0;
pub const MP0_BASE__INST3_SEG4: u32 = 0;
pub const MP0_BASE__INST3_SEG5: u32 = 0;
pub const MP0_BASE__INST4_SEG0: u32 = 0;
pub const MP0_BASE__INST4_SEG1: u32 = 0;
pub const MP0_BASE__INST4_SEG2: u32 = 0;
pub const MP0_BASE__INST4_SEG3: u32 = 0;
pub const MP0_BASE__INST4_SEG4: u32 = 0;
pub const MP0_BASE__INST4_SEG5: u32 = 0;
pub const MP0_BASE__INST5_SEG0: u32 = 0;
pub const MP0_BASE__INST5_SEG1: u32 = 0;
pub const MP0_BASE__INST5_SEG2: u32 = 0;
pub const MP0_BASE__INST5_SEG3: u32 = 0;
pub const MP0_BASE__INST5_SEG4: u32 = 0;
pub const MP0_BASE__INST5_SEG5: u32 = 0;
pub const MP0_BASE__INST6_SEG0: u32 = 0;
pub const MP0_BASE__INST6_SEG1: u32 = 0;
pub const MP0_BASE__INST6_SEG2: u32 = 0;
pub const MP0_BASE__INST6_SEG3: u32 = 0;
pub const MP0_BASE__INST6_SEG4: u32 = 0;
pub const MP0_BASE__INST6_SEG5: u32 = 0;
pub const MP1_BASE__INST0_SEG0: u32 = 0x00016000;
pub const MP1_BASE__INST0_SEG1: u32 = 0x00DC0000;
pub const MP1_BASE__INST0_SEG2: u32 = 0x00E00000;
pub const MP1_BASE__INST0_SEG3: u32 = 0x00E40000;
pub const MP1_BASE__INST0_SEG4: u32 = 0x0243FC00;
pub const MP1_BASE__INST0_SEG5: u32 = 0;
pub const MP1_BASE__INST1_SEG0: u32 = 0;
pub const MP1_BASE__INST1_SEG1: u32 = 0;
pub const MP1_BASE__INST1_SEG2: u32 = 0;
pub const MP1_BASE__INST1_SEG3: u32 = 0;
pub const MP1_BASE__INST1_SEG4: u32 = 0;
pub const MP1_BASE__INST1_SEG5: u32 = 0;
pub const MP1_BASE__INST2_SEG0: u32 = 0;
pub const MP1_BASE__INST2_SEG1: u32 = 0;
pub const MP1_BASE__INST2_SEG2: u32 = 0;
pub const MP1_BASE__INST2_SEG3: u32 = 0;
pub const MP1_BASE__INST2_SEG4: u32 = 0;
pub const MP1_BASE__INST2_SEG5: u32 = 0;
pub const MP1_BASE__INST3_SEG0: u32 = 0;
pub const MP1_BASE__INST3_SEG1: u32 = 0;
pub const MP1_BASE__INST3_SEG2: u32 = 0;
pub const MP1_BASE__INST3_SEG3: u32 = 0;
pub const MP1_BASE__INST3_SEG4: u32 = 0;
pub const MP1_BASE__INST3_SEG5: u32 = 0;
pub const MP1_BASE__INST4_SEG0: u32 = 0;
pub const MP1_BASE__INST4_SEG1: u32 = 0;
pub const MP1_BASE__INST4_SEG2: u32 = 0;
pub const MP1_BASE__INST4_SEG3: u32 = 0;
pub const MP1_BASE__INST4_SEG4: u32 = 0;
pub const MP1_BASE__INST4_SEG5: u32 = 0;
pub const MP1_BASE__INST5_SEG0: u32 = 0;
pub const MP1_BASE__INST5_SEG1: u32 = 0;
pub const MP1_BASE__INST5_SEG2: u32 = 0;
pub const MP1_BASE__INST5_SEG3: u32 = 0;
pub const MP1_BASE__INST5_SEG4: u32 = 0;
pub const MP1_BASE__INST5_SEG5: u32 = 0;
pub const MP1_BASE__INST6_SEG0: u32 = 0;
pub const MP1_BASE__INST6_SEG1: u32 = 0;
pub const MP1_BASE__INST6_SEG2: u32 = 0;
pub const MP1_BASE__INST6_SEG3: u32 = 0;
pub const MP1_BASE__INST6_SEG4: u32 = 0;
pub const MP1_BASE__INST6_SEG5: u32 = 0;
pub const NBIO_BASE__INST0_SEG0: u32 = 0x00000000;
pub const NBIO_BASE__INST0_SEG1: u32 = 0x00000014;
pub const NBIO_BASE__INST0_SEG2: u32 = 0x00000D20;
pub const NBIO_BASE__INST0_SEG3: u32 = 0x00010400;
pub const NBIO_BASE__INST0_SEG4: u32 = 0x0241B000;
pub const NBIO_BASE__INST0_SEG5: u32 = 0x04040000;
pub const NBIO_BASE__INST1_SEG0: u32 = 0;
pub const NBIO_BASE__INST1_SEG1: u32 = 0;
pub const NBIO_BASE__INST1_SEG2: u32 = 0;
pub const NBIO_BASE__INST1_SEG3: u32 = 0;
pub const NBIO_BASE__INST1_SEG4: u32 = 0;
pub const NBIO_BASE__INST1_SEG5: u32 = 0;
pub const NBIO_BASE__INST2_SEG0: u32 = 0;
pub const NBIO_BASE__INST2_SEG1: u32 = 0;
pub const NBIO_BASE__INST2_SEG2: u32 = 0;
pub const NBIO_BASE__INST2_SEG3: u32 = 0;
pub const NBIO_BASE__INST2_SEG4: u32 = 0;
pub const NBIO_BASE__INST2_SEG5: u32 = 0;
pub const NBIO_BASE__INST3_SEG0: u32 = 0;
pub const NBIO_BASE__INST3_SEG1: u32 = 0;
pub const NBIO_BASE__INST3_SEG2: u32 = 0;
pub const NBIO_BASE__INST3_SEG3: u32 = 0;
pub const NBIO_BASE__INST3_SEG4: u32 = 0;
pub const NBIO_BASE__INST3_SEG5: u32 = 0;
pub const NBIO_BASE__INST4_SEG0: u32 = 0;
pub const NBIO_BASE__INST4_SEG1: u32 = 0;
pub const NBIO_BASE__INST4_SEG2: u32 = 0;
pub const NBIO_BASE__INST4_SEG3: u32 = 0;
pub const NBIO_BASE__INST4_SEG4: u32 = 0;
pub const NBIO_BASE__INST4_SEG5: u32 = 0;
pub const NBIO_BASE__INST5_SEG0: u32 = 0;
pub const NBIO_BASE__INST5_SEG1: u32 = 0;
pub const NBIO_BASE__INST5_SEG2: u32 = 0;
pub const NBIO_BASE__INST5_SEG3: u32 = 0;
pub const NBIO_BASE__INST5_SEG4: u32 = 0;
pub const NBIO_BASE__INST5_SEG5: u32 = 0;
pub const NBIO_BASE__INST6_SEG0: u32 = 0;
pub const NBIO_BASE__INST6_SEG1: u32 = 0;
pub const NBIO_BASE__INST6_SEG2: u32 = 0;
pub const NBIO_BASE__INST6_SEG3: u32 = 0;
pub const NBIO_BASE__INST6_SEG4: u32 = 0;
pub const NBIO_BASE__INST6_SEG5: u32 = 0;
pub const OSSSYS_BASE__INST0_SEG0: u32 = 0x000010A0;
pub const OSSSYS_BASE__INST0_SEG1: u32 = 0x0240A000;
pub const OSSSYS_BASE__INST0_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST0_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST0_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST0_SEG5: u32 = 0;
pub const OSSSYS_BASE__INST1_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST1_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST1_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST1_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST1_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST1_SEG5: u32 = 0;
pub const OSSSYS_BASE__INST2_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST2_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST2_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST2_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST2_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST2_SEG5: u32 = 0;
pub const OSSSYS_BASE__INST3_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST3_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST3_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST3_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST3_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST3_SEG5: u32 = 0;
pub const OSSSYS_BASE__INST4_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST4_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST4_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST4_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST4_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST4_SEG5: u32 = 0;
pub const OSSSYS_BASE__INST5_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST5_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST5_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST5_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST5_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST5_SEG5: u32 = 0;
pub const OSSSYS_BASE__INST6_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST6_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST6_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST6_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST6_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST6_SEG5: u32 = 0;
pub const PCIE0_BASE__INST0_SEG0: u32 = 0x00000000;
pub const PCIE0_BASE__INST0_SEG1: u32 = 0x00000014;
pub const PCIE0_BASE__INST0_SEG2: u32 = 0x00000D20;
pub const PCIE0_BASE__INST0_SEG3: u32 = 0x00010400;
pub const PCIE0_BASE__INST0_SEG4: u32 = 0x0241B000;
pub const PCIE0_BASE__INST0_SEG5: u32 = 0x04040000;
pub const PCIE0_BASE__INST1_SEG0: u32 = 0;
pub const PCIE0_BASE__INST1_SEG1: u32 = 0;
pub const PCIE0_BASE__INST1_SEG2: u32 = 0;
pub const PCIE0_BASE__INST1_SEG3: u32 = 0;
pub const PCIE0_BASE__INST1_SEG4: u32 = 0;
pub const PCIE0_BASE__INST1_SEG5: u32 = 0;
pub const PCIE0_BASE__INST2_SEG0: u32 = 0;
pub const PCIE0_BASE__INST2_SEG1: u32 = 0;
pub const PCIE0_BASE__INST2_SEG2: u32 = 0;
pub const PCIE0_BASE__INST2_SEG3: u32 = 0;
pub const PCIE0_BASE__INST2_SEG4: u32 = 0;
pub const PCIE0_BASE__INST2_SEG5: u32 = 0;
pub const PCIE0_BASE__INST3_SEG0: u32 = 0;
pub const PCIE0_BASE__INST3_SEG1: u32 = 0;
pub const PCIE0_BASE__INST3_SEG2: u32 = 0;
pub const PCIE0_BASE__INST3_SEG3: u32 = 0;
pub const PCIE0_BASE__INST3_SEG4: u32 = 0;
pub const PCIE0_BASE__INST3_SEG5: u32 = 0;
pub const PCIE0_BASE__INST4_SEG0: u32 = 0;
pub const PCIE0_BASE__INST4_SEG1: u32 = 0;
pub const PCIE0_BASE__INST4_SEG2: u32 = 0;
pub const PCIE0_BASE__INST4_SEG3: u32 = 0;
pub const PCIE0_BASE__INST4_SEG4: u32 = 0;
pub const PCIE0_BASE__INST4_SEG5: u32 = 0;
pub const PCIE0_BASE__INST5_SEG0: u32 = 0;
pub const PCIE0_BASE__INST5_SEG1: u32 = 0;
pub const PCIE0_BASE__INST5_SEG2: u32 = 0;
pub const PCIE0_BASE__INST5_SEG3: u32 = 0;
pub const PCIE0_BASE__INST5_SEG4: u32 = 0;
pub const PCIE0_BASE__INST5_SEG5: u32 = 0;
pub const PCIE0_BASE__INST6_SEG0: u32 = 0;
pub const PCIE0_BASE__INST6_SEG1: u32 = 0;
pub const PCIE0_BASE__INST6_SEG2: u32 = 0;
pub const PCIE0_BASE__INST6_SEG3: u32 = 0;
pub const PCIE0_BASE__INST6_SEG4: u32 = 0;
pub const PCIE0_BASE__INST6_SEG5: u32 = 0;
pub const SDMA0_BASE__INST0_SEG0: u32 = 0x00001260;
pub const SDMA0_BASE__INST0_SEG1: u32 = 0x0000A000;
pub const SDMA0_BASE__INST0_SEG2: u32 = 0x0001C000;
pub const SDMA0_BASE__INST0_SEG3: u32 = 0x02402C00;
pub const SDMA0_BASE__INST0_SEG4: u32 = 0;
pub const SDMA0_BASE__INST0_SEG5: u32 = 0;
pub const SDMA0_BASE__INST1_SEG0: u32 = 0;
pub const SDMA0_BASE__INST1_SEG1: u32 = 0;
pub const SDMA0_BASE__INST1_SEG2: u32 = 0;
pub const SDMA0_BASE__INST1_SEG3: u32 = 0;
pub const SDMA0_BASE__INST1_SEG4: u32 = 0;
pub const SDMA0_BASE__INST1_SEG5: u32 = 0;
pub const SDMA0_BASE__INST2_SEG0: u32 = 0;
pub const SDMA0_BASE__INST2_SEG1: u32 = 0;
pub const SDMA0_BASE__INST2_SEG2: u32 = 0;
pub const SDMA0_BASE__INST2_SEG3: u32 = 0;
pub const SDMA0_BASE__INST2_SEG4: u32 = 0;
pub const SDMA0_BASE__INST2_SEG5: u32 = 0;
pub const SDMA0_BASE__INST3_SEG0: u32 = 0;
pub const SDMA0_BASE__INST3_SEG1: u32 = 0;
pub const SDMA0_BASE__INST3_SEG2: u32 = 0;
pub const SDMA0_BASE__INST3_SEG3: u32 = 0;
pub const SDMA0_BASE__INST3_SEG4: u32 = 0;
pub const SDMA0_BASE__INST3_SEG5: u32 = 0;
pub const SDMA0_BASE__INST4_SEG0: u32 = 0;
pub const SDMA0_BASE__INST4_SEG1: u32 = 0;
pub const SDMA0_BASE__INST4_SEG2: u32 = 0;
pub const SDMA0_BASE__INST4_SEG3: u32 = 0;
pub const SDMA0_BASE__INST4_SEG4: u32 = 0;
pub const SDMA0_BASE__INST4_SEG5: u32 = 0;
pub const SDMA0_BASE__INST5_SEG0: u32 = 0;
pub const SDMA0_BASE__INST5_SEG1: u32 = 0;
pub const SDMA0_BASE__INST5_SEG2: u32 = 0;
pub const SDMA0_BASE__INST5_SEG3: u32 = 0;
pub const SDMA0_BASE__INST5_SEG4: u32 = 0;
pub const SDMA0_BASE__INST5_SEG5: u32 = 0;
pub const SDMA0_BASE__INST6_SEG0: u32 = 0;
pub const SDMA0_BASE__INST6_SEG1: u32 = 0;
pub const SDMA0_BASE__INST6_SEG2: u32 = 0;
pub const SDMA0_BASE__INST6_SEG3: u32 = 0;
pub const SDMA0_BASE__INST6_SEG4: u32 = 0;
pub const SDMA0_BASE__INST6_SEG5: u32 = 0;
pub const SMUIO_BASE__INST0_SEG0: u32 = 0x00016800;
pub const SMUIO_BASE__INST0_SEG1: u32 = 0x00016A00;
pub const SMUIO_BASE__INST0_SEG2: u32 = 0x00440000;
pub const SMUIO_BASE__INST0_SEG3: u32 = 0x02401000;
pub const SMUIO_BASE__INST0_SEG4: u32 = 0;
pub const SMUIO_BASE__INST0_SEG5: u32 = 0;
pub const SMUIO_BASE__INST1_SEG0: u32 = 0;
pub const SMUIO_BASE__INST1_SEG1: u32 = 0;
pub const SMUIO_BASE__INST1_SEG2: u32 = 0;
pub const SMUIO_BASE__INST1_SEG3: u32 = 0;
pub const SMUIO_BASE__INST1_SEG4: u32 = 0;
pub const SMUIO_BASE__INST1_SEG5: u32 = 0;
pub const SMUIO_BASE__INST2_SEG0: u32 = 0;
pub const SMUIO_BASE__INST2_SEG1: u32 = 0;
pub const SMUIO_BASE__INST2_SEG2: u32 = 0;
pub const SMUIO_BASE__INST2_SEG3: u32 = 0;
pub const SMUIO_BASE__INST2_SEG4: u32 = 0;
pub const SMUIO_BASE__INST2_SEG5: u32 = 0;
pub const SMUIO_BASE__INST3_SEG0: u32 = 0;
pub const SMUIO_BASE__INST3_SEG1: u32 = 0;
pub const SMUIO_BASE__INST3_SEG2: u32 = 0;
pub const SMUIO_BASE__INST3_SEG3: u32 = 0;
pub const SMUIO_BASE__INST3_SEG4: u32 = 0;
pub const SMUIO_BASE__INST3_SEG5: u32 = 0;
pub const SMUIO_BASE__INST4_SEG0: u32 = 0;
pub const SMUIO_BASE__INST4_SEG1: u32 = 0;
pub const SMUIO_BASE__INST4_SEG2: u32 = 0;
pub const SMUIO_BASE__INST4_SEG3: u32 = 0;
pub const SMUIO_BASE__INST4_SEG4: u32 = 0;
pub const SMUIO_BASE__INST4_SEG5: u32 = 0;
pub const SMUIO_BASE__INST5_SEG0: u32 = 0;
pub const SMUIO_BASE__INST5_SEG1: u32 = 0;
pub const SMUIO_BASE__INST5_SEG2: u32 = 0;
pub const SMUIO_BASE__INST5_SEG3: u32 = 0;
pub const SMUIO_BASE__INST5_SEG4: u32 = 0;
pub const SMUIO_BASE__INST5_SEG5: u32 = 0;
pub const SMUIO_BASE__INST6_SEG0: u32 = 0;
pub const SMUIO_BASE__INST6_SEG1: u32 = 0;
pub const SMUIO_BASE__INST6_SEG2: u32 = 0;
pub const SMUIO_BASE__INST6_SEG3: u32 = 0;
pub const SMUIO_BASE__INST6_SEG4: u32 = 0;
pub const SMUIO_BASE__INST6_SEG5: u32 = 0;
pub const THM_BASE__INST0_SEG0: u32 = 0x00016600;
pub const THM_BASE__INST0_SEG1: u32 = 0x02400C00;
pub const THM_BASE__INST0_SEG2: u32 = 0;
pub const THM_BASE__INST0_SEG3: u32 = 0;
pub const THM_BASE__INST0_SEG4: u32 = 0;
pub const THM_BASE__INST0_SEG5: u32 = 0;
pub const THM_BASE__INST1_SEG0: u32 = 0;
pub const THM_BASE__INST1_SEG1: u32 = 0;
pub const THM_BASE__INST1_SEG2: u32 = 0;
pub const THM_BASE__INST1_SEG3: u32 = 0;
pub const THM_BASE__INST1_SEG4: u32 = 0;
pub const THM_BASE__INST1_SEG5: u32 = 0;
pub const THM_BASE__INST2_SEG0: u32 = 0;
pub const THM_BASE__INST2_SEG1: u32 = 0;
pub const THM_BASE__INST2_SEG2: u32 = 0;
pub const THM_BASE__INST2_SEG3: u32 = 0;
pub const THM_BASE__INST2_SEG4: u32 = 0;
pub const THM_BASE__INST2_SEG5: u32 = 0;
pub const THM_BASE__INST3_SEG0: u32 = 0;
pub const THM_BASE__INST3_SEG1: u32 = 0;
pub const THM_BASE__INST3_SEG2: u32 = 0;
pub const THM_BASE__INST3_SEG3: u32 = 0;
pub const THM_BASE__INST3_SEG4: u32 = 0;
pub const THM_BASE__INST3_SEG5: u32 = 0;
pub const THM_BASE__INST4_SEG0: u32 = 0;
pub const THM_BASE__INST4_SEG1: u32 = 0;
pub const THM_BASE__INST4_SEG2: u32 = 0;
pub const THM_BASE__INST4_SEG3: u32 = 0;
pub const THM_BASE__INST4_SEG4: u32 = 0;
pub const THM_BASE__INST4_SEG5: u32 = 0;
pub const THM_BASE__INST5_SEG0: u32 = 0;
pub const THM_BASE__INST5_SEG1: u32 = 0;
pub const THM_BASE__INST5_SEG2: u32 = 0;
pub const THM_BASE__INST5_SEG3: u32 = 0;
pub const THM_BASE__INST5_SEG4: u32 = 0;
pub const THM_BASE__INST5_SEG5: u32 = 0;
pub const THM_BASE__INST6_SEG0: u32 = 0;
pub const THM_BASE__INST6_SEG1: u32 = 0;
pub const THM_BASE__INST6_SEG2: u32 = 0;
pub const THM_BASE__INST6_SEG3: u32 = 0;
pub const THM_BASE__INST6_SEG4: u32 = 0;
pub const THM_BASE__INST6_SEG5: u32 = 0;
pub const UMC_BASE__INST0_SEG0: u32 = 0x00014000;
pub const UMC_BASE__INST0_SEG1: u32 = 0x02425800;
pub const UMC_BASE__INST0_SEG2: u32 = 0;
pub const UMC_BASE__INST0_SEG3: u32 = 0;
pub const UMC_BASE__INST0_SEG4: u32 = 0;
pub const UMC_BASE__INST0_SEG5: u32 = 0;
pub const UMC_BASE__INST1_SEG0: u32 = 0x00054000;
pub const UMC_BASE__INST1_SEG1: u32 = 0x02425C00;
pub const UMC_BASE__INST1_SEG2: u32 = 0;
pub const UMC_BASE__INST1_SEG3: u32 = 0;
pub const UMC_BASE__INST1_SEG4: u32 = 0;
pub const UMC_BASE__INST1_SEG5: u32 = 0;
pub const UMC_BASE__INST2_SEG0: u32 = 0;
pub const UMC_BASE__INST2_SEG1: u32 = 0;
pub const UMC_BASE__INST2_SEG2: u32 = 0;
pub const UMC_BASE__INST2_SEG3: u32 = 0;
pub const UMC_BASE__INST2_SEG4: u32 = 0;
pub const UMC_BASE__INST2_SEG5: u32 = 0;
pub const UMC_BASE__INST3_SEG0: u32 = 0;
pub const UMC_BASE__INST3_SEG1: u32 = 0;
pub const UMC_BASE__INST3_SEG2: u32 = 0;
pub const UMC_BASE__INST3_SEG3: u32 = 0;
pub const UMC_BASE__INST3_SEG4: u32 = 0;
pub const UMC_BASE__INST3_SEG5: u32 = 0;
pub const UMC_BASE__INST4_SEG0: u32 = 0;
pub const UMC_BASE__INST4_SEG1: u32 = 0;
pub const UMC_BASE__INST4_SEG2: u32 = 0;
pub const UMC_BASE__INST4_SEG3: u32 = 0;
pub const UMC_BASE__INST4_SEG4: u32 = 0;
pub const UMC_BASE__INST4_SEG5: u32 = 0;
pub const UMC_BASE__INST5_SEG0: u32 = 0;
pub const UMC_BASE__INST5_SEG1: u32 = 0;
pub const UMC_BASE__INST5_SEG2: u32 = 0;
pub const UMC_BASE__INST5_SEG3: u32 = 0;
pub const UMC_BASE__INST5_SEG4: u32 = 0;
pub const UMC_BASE__INST5_SEG5: u32 = 0;
pub const UMC_BASE__INST6_SEG0: u32 = 0;
pub const UMC_BASE__INST6_SEG1: u32 = 0;
pub const UMC_BASE__INST6_SEG2: u32 = 0;
pub const UMC_BASE__INST6_SEG3: u32 = 0;
pub const UMC_BASE__INST6_SEG4: u32 = 0;
pub const UMC_BASE__INST6_SEG5: u32 = 0;
pub const VCN0_BASE__INST0_SEG0: u32 = 0x00007800;
pub const VCN0_BASE__INST0_SEG1: u32 = 0x00007E00;
pub const VCN0_BASE__INST0_SEG2: u32 = 0x02403000;
pub const VCN0_BASE__INST0_SEG3: u32 = 0;
pub const VCN0_BASE__INST0_SEG4: u32 = 0;
pub const VCN0_BASE__INST0_SEG5: u32 = 0;
pub const VCN0_BASE__INST1_SEG0: u32 = 0;
pub const VCN0_BASE__INST1_SEG1: u32 = 0;
pub const VCN0_BASE__INST1_SEG2: u32 = 0;
pub const VCN0_BASE__INST1_SEG3: u32 = 0;
pub const VCN0_BASE__INST1_SEG4: u32 = 0;
pub const VCN0_BASE__INST1_SEG5: u32 = 0;
pub const VCN0_BASE__INST2_SEG0: u32 = 0;
pub const VCN0_BASE__INST2_SEG1: u32 = 0;
pub const VCN0_BASE__INST2_SEG2: u32 = 0;
pub const VCN0_BASE__INST2_SEG3: u32 = 0;
pub const VCN0_BASE__INST2_SEG4: u32 = 0;
pub const VCN0_BASE__INST2_SEG5: u32 = 0;
pub const VCN0_BASE__INST3_SEG0: u32 = 0;
pub const VCN0_BASE__INST3_SEG1: u32 = 0;
pub const VCN0_BASE__INST3_SEG2: u32 = 0;
pub const VCN0_BASE__INST3_SEG3: u32 = 0;
pub const VCN0_BASE__INST3_SEG4: u32 = 0;
pub const VCN0_BASE__INST3_SEG5: u32 = 0;
pub const VCN0_BASE__INST4_SEG0: u32 = 0;
pub const VCN0_BASE__INST4_SEG1: u32 = 0;
pub const VCN0_BASE__INST4_SEG2: u32 = 0;
pub const VCN0_BASE__INST4_SEG3: u32 = 0;
pub const VCN0_BASE__INST4_SEG4: u32 = 0;
pub const VCN0_BASE__INST4_SEG5: u32 = 0;
pub const VCN0_BASE__INST5_SEG0: u32 = 0;
pub const VCN0_BASE__INST5_SEG1: u32 = 0;
pub const VCN0_BASE__INST5_SEG2: u32 = 0;
pub const VCN0_BASE__INST5_SEG3: u32 = 0;
pub const VCN0_BASE__INST5_SEG4: u32 = 0;
pub const VCN0_BASE__INST5_SEG5: u32 = 0;
pub const VCN0_BASE__INST6_SEG0: u32 = 0;
pub const VCN0_BASE__INST6_SEG1: u32 = 0;
pub const VCN0_BASE__INST6_SEG2: u32 = 0;
pub const VCN0_BASE__INST6_SEG3: u32 = 0;
pub const VCN0_BASE__INST6_SEG4: u32 = 0;
pub const VCN0_BASE__INST6_SEG5: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
