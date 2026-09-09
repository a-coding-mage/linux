/* Translated from navi12_ip_offset.h. */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IP_BASE_INSTANCE { pub segment: [u32; 5], }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IP_BASE { pub instance: [IP_BASE_INSTANCE; 7], }

pub const MAX_INSTANCE: usize = 7;
pub const MAX_SEGMENT: usize = 5;

pub const MAX_INSTANCE: u32 = 7u32;
pub const MAX_SEGMENT: u32 = 5u32;
pub const ATHUB_BASE__INST0_SEG0: u32 = 0x00000C00u32;
pub const ATHUB_BASE__INST0_SEG1: u32 = 0x02408C00u32;
pub const ATHUB_BASE__INST0_SEG2: u32 = 0u32;
pub const ATHUB_BASE__INST0_SEG3: u32 = 0u32;
pub const ATHUB_BASE__INST0_SEG4: u32 = 0u32;
pub const ATHUB_BASE__INST1_SEG0: u32 = 0u32;
pub const ATHUB_BASE__INST1_SEG1: u32 = 0u32;
pub const ATHUB_BASE__INST1_SEG2: u32 = 0u32;
pub const ATHUB_BASE__INST1_SEG3: u32 = 0u32;
pub const ATHUB_BASE__INST1_SEG4: u32 = 0u32;
pub const ATHUB_BASE__INST2_SEG0: u32 = 0u32;
pub const ATHUB_BASE__INST2_SEG1: u32 = 0u32;
pub const ATHUB_BASE__INST2_SEG2: u32 = 0u32;
pub const ATHUB_BASE__INST2_SEG3: u32 = 0u32;
pub const ATHUB_BASE__INST2_SEG4: u32 = 0u32;
pub const ATHUB_BASE__INST3_SEG0: u32 = 0u32;
pub const ATHUB_BASE__INST3_SEG1: u32 = 0u32;
pub const ATHUB_BASE__INST3_SEG2: u32 = 0u32;
pub const ATHUB_BASE__INST3_SEG3: u32 = 0u32;
pub const ATHUB_BASE__INST3_SEG4: u32 = 0u32;
pub const ATHUB_BASE__INST4_SEG0: u32 = 0u32;
pub const ATHUB_BASE__INST4_SEG1: u32 = 0u32;
pub const ATHUB_BASE__INST4_SEG2: u32 = 0u32;
pub const ATHUB_BASE__INST4_SEG3: u32 = 0u32;
pub const ATHUB_BASE__INST4_SEG4: u32 = 0u32;
pub const ATHUB_BASE__INST5_SEG0: u32 = 0u32;
pub const ATHUB_BASE__INST5_SEG1: u32 = 0u32;
pub const ATHUB_BASE__INST5_SEG2: u32 = 0u32;
pub const ATHUB_BASE__INST5_SEG3: u32 = 0u32;
pub const ATHUB_BASE__INST5_SEG4: u32 = 0u32;
pub const ATHUB_BASE__INST6_SEG0: u32 = 0u32;
pub const ATHUB_BASE__INST6_SEG1: u32 = 0u32;
pub const ATHUB_BASE__INST6_SEG2: u32 = 0u32;
pub const ATHUB_BASE__INST6_SEG3: u32 = 0u32;
pub const ATHUB_BASE__INST6_SEG4: u32 = 0u32;
pub const CLK_BASE__INST0_SEG0: u32 = 0x00016C00u32;
pub const CLK_BASE__INST0_SEG1: u32 = 0x02401800u32;
pub const CLK_BASE__INST0_SEG2: u32 = 0u32;
pub const CLK_BASE__INST0_SEG3: u32 = 0u32;
pub const CLK_BASE__INST0_SEG4: u32 = 0u32;
pub const CLK_BASE__INST1_SEG0: u32 = 0x00016E00u32;
pub const CLK_BASE__INST1_SEG1: u32 = 0x02401C00u32;
pub const CLK_BASE__INST1_SEG2: u32 = 0u32;
pub const CLK_BASE__INST1_SEG3: u32 = 0u32;
pub const CLK_BASE__INST1_SEG4: u32 = 0u32;
pub const CLK_BASE__INST2_SEG0: u32 = 0x00017000u32;
pub const CLK_BASE__INST2_SEG1: u32 = 0x02402000u32;
pub const CLK_BASE__INST2_SEG2: u32 = 0u32;
pub const CLK_BASE__INST2_SEG3: u32 = 0u32;
pub const CLK_BASE__INST2_SEG4: u32 = 0u32;
pub const CLK_BASE__INST3_SEG0: u32 = 0x00017200u32;
pub const CLK_BASE__INST3_SEG1: u32 = 0x02402400u32;
pub const CLK_BASE__INST3_SEG2: u32 = 0u32;
pub const CLK_BASE__INST3_SEG3: u32 = 0u32;
pub const CLK_BASE__INST3_SEG4: u32 = 0u32;
pub const CLK_BASE__INST4_SEG0: u32 = 0x0001B000u32;
pub const CLK_BASE__INST4_SEG1: u32 = 0x0242D800u32;
pub const CLK_BASE__INST4_SEG2: u32 = 0u32;
pub const CLK_BASE__INST4_SEG3: u32 = 0u32;
pub const CLK_BASE__INST4_SEG4: u32 = 0u32;
pub const CLK_BASE__INST5_SEG0: u32 = 0x00017E00u32;
pub const CLK_BASE__INST5_SEG1: u32 = 0x0240BC00u32;
pub const CLK_BASE__INST5_SEG2: u32 = 0u32;
pub const CLK_BASE__INST5_SEG3: u32 = 0u32;
pub const CLK_BASE__INST5_SEG4: u32 = 0u32;
pub const CLK_BASE__INST6_SEG0: u32 = 0u32;
pub const CLK_BASE__INST6_SEG1: u32 = 0u32;
pub const CLK_BASE__INST6_SEG2: u32 = 0u32;
pub const CLK_BASE__INST6_SEG3: u32 = 0u32;
pub const CLK_BASE__INST6_SEG4: u32 = 0u32;
pub const DF_BASE__INST0_SEG0: u32 = 0x00007000u32;
pub const DF_BASE__INST0_SEG1: u32 = 0x0240B800u32;
pub const DF_BASE__INST0_SEG2: u32 = 0u32;
pub const DF_BASE__INST0_SEG3: u32 = 0u32;
pub const DF_BASE__INST0_SEG4: u32 = 0u32;
pub const DF_BASE__INST1_SEG0: u32 = 0u32;
pub const DF_BASE__INST1_SEG1: u32 = 0u32;
pub const DF_BASE__INST1_SEG2: u32 = 0u32;
pub const DF_BASE__INST1_SEG3: u32 = 0u32;
pub const DF_BASE__INST1_SEG4: u32 = 0u32;
pub const DF_BASE__INST2_SEG0: u32 = 0u32;
pub const DF_BASE__INST2_SEG1: u32 = 0u32;
pub const DF_BASE__INST2_SEG2: u32 = 0u32;
pub const DF_BASE__INST2_SEG3: u32 = 0u32;
pub const DF_BASE__INST2_SEG4: u32 = 0u32;
pub const DF_BASE__INST3_SEG0: u32 = 0u32;
pub const DF_BASE__INST3_SEG1: u32 = 0u32;
pub const DF_BASE__INST3_SEG2: u32 = 0u32;
pub const DF_BASE__INST3_SEG3: u32 = 0u32;
pub const DF_BASE__INST3_SEG4: u32 = 0u32;
pub const DF_BASE__INST4_SEG0: u32 = 0u32;
pub const DF_BASE__INST4_SEG1: u32 = 0u32;
pub const DF_BASE__INST4_SEG2: u32 = 0u32;
pub const DF_BASE__INST4_SEG3: u32 = 0u32;
pub const DF_BASE__INST4_SEG4: u32 = 0u32;
pub const DF_BASE__INST5_SEG0: u32 = 0u32;
pub const DF_BASE__INST5_SEG1: u32 = 0u32;
pub const DF_BASE__INST5_SEG2: u32 = 0u32;
pub const DF_BASE__INST5_SEG3: u32 = 0u32;
pub const DF_BASE__INST5_SEG4: u32 = 0u32;
pub const DF_BASE__INST6_SEG0: u32 = 0u32;
pub const DF_BASE__INST6_SEG1: u32 = 0u32;
pub const DF_BASE__INST6_SEG2: u32 = 0u32;
pub const DF_BASE__INST6_SEG3: u32 = 0u32;
pub const DF_BASE__INST6_SEG4: u32 = 0u32;
pub const DIO_BASE__INST0_SEG0: u32 = 0x02404000u32;
pub const DIO_BASE__INST0_SEG1: u32 = 0u32;
pub const DIO_BASE__INST0_SEG2: u32 = 0u32;
pub const DIO_BASE__INST0_SEG3: u32 = 0u32;
pub const DIO_BASE__INST0_SEG4: u32 = 0u32;
pub const DIO_BASE__INST1_SEG0: u32 = 0u32;
pub const DIO_BASE__INST1_SEG1: u32 = 0u32;
pub const DIO_BASE__INST1_SEG2: u32 = 0u32;
pub const DIO_BASE__INST1_SEG3: u32 = 0u32;
pub const DIO_BASE__INST1_SEG4: u32 = 0u32;
pub const DIO_BASE__INST2_SEG0: u32 = 0u32;
pub const DIO_BASE__INST2_SEG1: u32 = 0u32;
pub const DIO_BASE__INST2_SEG2: u32 = 0u32;
pub const DIO_BASE__INST2_SEG3: u32 = 0u32;
pub const DIO_BASE__INST2_SEG4: u32 = 0u32;
pub const DIO_BASE__INST3_SEG0: u32 = 0u32;
pub const DIO_BASE__INST3_SEG1: u32 = 0u32;
pub const DIO_BASE__INST3_SEG2: u32 = 0u32;
pub const DIO_BASE__INST3_SEG3: u32 = 0u32;
pub const DIO_BASE__INST3_SEG4: u32 = 0u32;
pub const DIO_BASE__INST4_SEG0: u32 = 0u32;
pub const DIO_BASE__INST4_SEG1: u32 = 0u32;
pub const DIO_BASE__INST4_SEG2: u32 = 0u32;
pub const DIO_BASE__INST4_SEG3: u32 = 0u32;
pub const DIO_BASE__INST4_SEG4: u32 = 0u32;
pub const DIO_BASE__INST5_SEG0: u32 = 0u32;
pub const DIO_BASE__INST5_SEG1: u32 = 0u32;
pub const DIO_BASE__INST5_SEG2: u32 = 0u32;
pub const DIO_BASE__INST5_SEG3: u32 = 0u32;
pub const DIO_BASE__INST5_SEG4: u32 = 0u32;
pub const DIO_BASE__INST6_SEG0: u32 = 0u32;
pub const DIO_BASE__INST6_SEG1: u32 = 0u32;
pub const DIO_BASE__INST6_SEG2: u32 = 0u32;
pub const DIO_BASE__INST6_SEG3: u32 = 0u32;
pub const DIO_BASE__INST6_SEG4: u32 = 0u32;
pub const DMU_BASE__INST0_SEG0: u32 = 0x00000012u32;
pub const DMU_BASE__INST0_SEG1: u32 = 0x000000C0u32;
pub const DMU_BASE__INST0_SEG2: u32 = 0x000034C0u32;
pub const DMU_BASE__INST0_SEG3: u32 = 0x00009000u32;
pub const DMU_BASE__INST0_SEG4: u32 = 0x02403C00u32;
pub const DMU_BASE__INST1_SEG0: u32 = 0u32;
pub const DMU_BASE__INST1_SEG1: u32 = 0u32;
pub const DMU_BASE__INST1_SEG2: u32 = 0u32;
pub const DMU_BASE__INST1_SEG3: u32 = 0u32;
pub const DMU_BASE__INST1_SEG4: u32 = 0u32;
pub const DMU_BASE__INST2_SEG0: u32 = 0u32;
pub const DMU_BASE__INST2_SEG1: u32 = 0u32;
pub const DMU_BASE__INST2_SEG2: u32 = 0u32;
pub const DMU_BASE__INST2_SEG3: u32 = 0u32;
pub const DMU_BASE__INST2_SEG4: u32 = 0u32;
pub const DMU_BASE__INST3_SEG0: u32 = 0u32;
pub const DMU_BASE__INST3_SEG1: u32 = 0u32;
pub const DMU_BASE__INST3_SEG2: u32 = 0u32;
pub const DMU_BASE__INST3_SEG3: u32 = 0u32;
pub const DMU_BASE__INST3_SEG4: u32 = 0u32;
pub const DMU_BASE__INST4_SEG0: u32 = 0u32;
pub const DMU_BASE__INST4_SEG1: u32 = 0u32;
pub const DMU_BASE__INST4_SEG2: u32 = 0u32;
pub const DMU_BASE__INST4_SEG3: u32 = 0u32;
pub const DMU_BASE__INST4_SEG4: u32 = 0u32;
pub const DMU_BASE__INST5_SEG0: u32 = 0u32;
pub const DMU_BASE__INST5_SEG1: u32 = 0u32;
pub const DMU_BASE__INST5_SEG2: u32 = 0u32;
pub const DMU_BASE__INST5_SEG3: u32 = 0u32;
pub const DMU_BASE__INST5_SEG4: u32 = 0u32;
pub const DMU_BASE__INST6_SEG0: u32 = 0u32;
pub const DMU_BASE__INST6_SEG1: u32 = 0u32;
pub const DMU_BASE__INST6_SEG2: u32 = 0u32;
pub const DMU_BASE__INST6_SEG3: u32 = 0u32;
pub const DMU_BASE__INST6_SEG4: u32 = 0u32;
pub const DPCS_BASE__INST0_SEG0: u32 = 0x00000012u32;
pub const DPCS_BASE__INST0_SEG1: u32 = 0x000000C0u32;
pub const DPCS_BASE__INST0_SEG2: u32 = 0x000034C0u32;
pub const DPCS_BASE__INST0_SEG3: u32 = 0x00009000u32;
pub const DPCS_BASE__INST0_SEG4: u32 = 0x02403C00u32;
pub const DPCS_BASE__INST1_SEG0: u32 = 0u32;
pub const DPCS_BASE__INST1_SEG1: u32 = 0u32;
pub const DPCS_BASE__INST1_SEG2: u32 = 0u32;
pub const DPCS_BASE__INST1_SEG3: u32 = 0u32;
pub const DPCS_BASE__INST1_SEG4: u32 = 0u32;
pub const DPCS_BASE__INST2_SEG0: u32 = 0u32;
pub const DPCS_BASE__INST2_SEG1: u32 = 0u32;
pub const DPCS_BASE__INST2_SEG2: u32 = 0u32;
pub const DPCS_BASE__INST2_SEG3: u32 = 0u32;
pub const DPCS_BASE__INST2_SEG4: u32 = 0u32;
pub const DPCS_BASE__INST3_SEG0: u32 = 0u32;
pub const DPCS_BASE__INST3_SEG1: u32 = 0u32;
pub const DPCS_BASE__INST3_SEG2: u32 = 0u32;
pub const DPCS_BASE__INST3_SEG3: u32 = 0u32;
pub const DPCS_BASE__INST3_SEG4: u32 = 0u32;
pub const DPCS_BASE__INST4_SEG0: u32 = 0u32;
pub const DPCS_BASE__INST4_SEG1: u32 = 0u32;
pub const DPCS_BASE__INST4_SEG2: u32 = 0u32;
pub const DPCS_BASE__INST4_SEG3: u32 = 0u32;
pub const DPCS_BASE__INST4_SEG4: u32 = 0u32;
pub const DPCS_BASE__INST5_SEG0: u32 = 0u32;
pub const DPCS_BASE__INST5_SEG1: u32 = 0u32;
pub const DPCS_BASE__INST5_SEG2: u32 = 0u32;
pub const DPCS_BASE__INST5_SEG3: u32 = 0u32;
pub const DPCS_BASE__INST5_SEG4: u32 = 0u32;
pub const DPCS_BASE__INST6_SEG0: u32 = 0u32;
pub const DPCS_BASE__INST6_SEG1: u32 = 0u32;
pub const DPCS_BASE__INST6_SEG2: u32 = 0u32;
pub const DPCS_BASE__INST6_SEG3: u32 = 0u32;
pub const DPCS_BASE__INST6_SEG4: u32 = 0u32;
pub const FUSE_BASE__INST0_SEG0: u32 = 0x00017400u32;
pub const FUSE_BASE__INST0_SEG1: u32 = 0x02401400u32;
pub const FUSE_BASE__INST0_SEG2: u32 = 0u32;
pub const FUSE_BASE__INST0_SEG3: u32 = 0u32;
pub const FUSE_BASE__INST0_SEG4: u32 = 0u32;
pub const FUSE_BASE__INST1_SEG0: u32 = 0u32;
pub const FUSE_BASE__INST1_SEG1: u32 = 0u32;
pub const FUSE_BASE__INST1_SEG2: u32 = 0u32;
pub const FUSE_BASE__INST1_SEG3: u32 = 0u32;
pub const FUSE_BASE__INST1_SEG4: u32 = 0u32;
pub const FUSE_BASE__INST2_SEG0: u32 = 0u32;
pub const FUSE_BASE__INST2_SEG1: u32 = 0u32;
pub const FUSE_BASE__INST2_SEG2: u32 = 0u32;
pub const FUSE_BASE__INST2_SEG3: u32 = 0u32;
pub const FUSE_BASE__INST2_SEG4: u32 = 0u32;
pub const FUSE_BASE__INST3_SEG0: u32 = 0u32;
pub const FUSE_BASE__INST3_SEG1: u32 = 0u32;
pub const FUSE_BASE__INST3_SEG2: u32 = 0u32;
pub const FUSE_BASE__INST3_SEG3: u32 = 0u32;
pub const FUSE_BASE__INST3_SEG4: u32 = 0u32;
pub const FUSE_BASE__INST4_SEG0: u32 = 0u32;
pub const FUSE_BASE__INST4_SEG1: u32 = 0u32;
pub const FUSE_BASE__INST4_SEG2: u32 = 0u32;
pub const FUSE_BASE__INST4_SEG3: u32 = 0u32;
pub const FUSE_BASE__INST4_SEG4: u32 = 0u32;
pub const FUSE_BASE__INST5_SEG0: u32 = 0u32;
pub const FUSE_BASE__INST5_SEG1: u32 = 0u32;
pub const FUSE_BASE__INST5_SEG2: u32 = 0u32;
pub const FUSE_BASE__INST5_SEG3: u32 = 0u32;
pub const FUSE_BASE__INST5_SEG4: u32 = 0u32;
pub const FUSE_BASE__INST6_SEG0: u32 = 0u32;
pub const FUSE_BASE__INST6_SEG1: u32 = 0u32;
pub const FUSE_BASE__INST6_SEG2: u32 = 0u32;
pub const FUSE_BASE__INST6_SEG3: u32 = 0u32;
pub const FUSE_BASE__INST6_SEG4: u32 = 0u32;
pub const GC_BASE__INST0_SEG0: u32 = 0x00001260u32;
pub const GC_BASE__INST0_SEG1: u32 = 0x0000A000u32;
pub const GC_BASE__INST0_SEG2: u32 = 0x02402C00u32;
pub const GC_BASE__INST0_SEG3: u32 = 0u32;
pub const GC_BASE__INST0_SEG4: u32 = 0u32;
pub const GC_BASE__INST1_SEG0: u32 = 0u32;
pub const GC_BASE__INST1_SEG1: u32 = 0u32;
pub const GC_BASE__INST1_SEG2: u32 = 0u32;
pub const GC_BASE__INST1_SEG3: u32 = 0u32;
pub const GC_BASE__INST1_SEG4: u32 = 0u32;
pub const GC_BASE__INST2_SEG0: u32 = 0u32;
pub const GC_BASE__INST2_SEG1: u32 = 0u32;
pub const GC_BASE__INST2_SEG2: u32 = 0u32;
pub const GC_BASE__INST2_SEG3: u32 = 0u32;
pub const GC_BASE__INST2_SEG4: u32 = 0u32;
pub const GC_BASE__INST3_SEG0: u32 = 0u32;
pub const GC_BASE__INST3_SEG1: u32 = 0u32;
pub const GC_BASE__INST3_SEG2: u32 = 0u32;
pub const GC_BASE__INST3_SEG3: u32 = 0u32;
pub const GC_BASE__INST3_SEG4: u32 = 0u32;
pub const GC_BASE__INST4_SEG0: u32 = 0u32;
pub const GC_BASE__INST4_SEG1: u32 = 0u32;
pub const GC_BASE__INST4_SEG2: u32 = 0u32;
pub const GC_BASE__INST4_SEG3: u32 = 0u32;
pub const GC_BASE__INST4_SEG4: u32 = 0u32;
pub const GC_BASE__INST5_SEG0: u32 = 0u32;
pub const GC_BASE__INST5_SEG1: u32 = 0u32;
pub const GC_BASE__INST5_SEG2: u32 = 0u32;
pub const GC_BASE__INST5_SEG3: u32 = 0u32;
pub const GC_BASE__INST5_SEG4: u32 = 0u32;
pub const GC_BASE__INST6_SEG0: u32 = 0u32;
pub const GC_BASE__INST6_SEG1: u32 = 0u32;
pub const GC_BASE__INST6_SEG2: u32 = 0u32;
pub const GC_BASE__INST6_SEG3: u32 = 0u32;
pub const GC_BASE__INST6_SEG4: u32 = 0u32;
pub const HDA_BASE__INST0_SEG0: u32 = 0x004C0000u32;
pub const HDA_BASE__INST0_SEG1: u32 = 0x02404800u32;
pub const HDA_BASE__INST0_SEG2: u32 = 0u32;
pub const HDA_BASE__INST0_SEG3: u32 = 0u32;
pub const HDA_BASE__INST0_SEG4: u32 = 0u32;
pub const HDA_BASE__INST1_SEG0: u32 = 0u32;
pub const HDA_BASE__INST1_SEG1: u32 = 0u32;
pub const HDA_BASE__INST1_SEG2: u32 = 0u32;
pub const HDA_BASE__INST1_SEG3: u32 = 0u32;
pub const HDA_BASE__INST1_SEG4: u32 = 0u32;
pub const HDA_BASE__INST2_SEG0: u32 = 0u32;
pub const HDA_BASE__INST2_SEG1: u32 = 0u32;
pub const HDA_BASE__INST2_SEG2: u32 = 0u32;
pub const HDA_BASE__INST2_SEG3: u32 = 0u32;
pub const HDA_BASE__INST2_SEG4: u32 = 0u32;
pub const HDA_BASE__INST3_SEG0: u32 = 0u32;
pub const HDA_BASE__INST3_SEG1: u32 = 0u32;
pub const HDA_BASE__INST3_SEG2: u32 = 0u32;
pub const HDA_BASE__INST3_SEG3: u32 = 0u32;
pub const HDA_BASE__INST3_SEG4: u32 = 0u32;
pub const HDA_BASE__INST4_SEG0: u32 = 0u32;
pub const HDA_BASE__INST4_SEG1: u32 = 0u32;
pub const HDA_BASE__INST4_SEG2: u32 = 0u32;
pub const HDA_BASE__INST4_SEG3: u32 = 0u32;
pub const HDA_BASE__INST4_SEG4: u32 = 0u32;
pub const HDA_BASE__INST5_SEG0: u32 = 0u32;
pub const HDA_BASE__INST5_SEG1: u32 = 0u32;
pub const HDA_BASE__INST5_SEG2: u32 = 0u32;
pub const HDA_BASE__INST5_SEG3: u32 = 0u32;
pub const HDA_BASE__INST5_SEG4: u32 = 0u32;
pub const HDA_BASE__INST6_SEG0: u32 = 0u32;
pub const HDA_BASE__INST6_SEG1: u32 = 0u32;
pub const HDA_BASE__INST6_SEG2: u32 = 0u32;
pub const HDA_BASE__INST6_SEG3: u32 = 0u32;
pub const HDA_BASE__INST6_SEG4: u32 = 0u32;
pub const HDP_BASE__INST0_SEG0: u32 = 0x00000F20u32;
pub const HDP_BASE__INST0_SEG1: u32 = 0x0240A400u32;
pub const HDP_BASE__INST0_SEG2: u32 = 0u32;
pub const HDP_BASE__INST0_SEG3: u32 = 0u32;
pub const HDP_BASE__INST0_SEG4: u32 = 0u32;
pub const HDP_BASE__INST1_SEG0: u32 = 0u32;
pub const HDP_BASE__INST1_SEG1: u32 = 0u32;
pub const HDP_BASE__INST1_SEG2: u32 = 0u32;
pub const HDP_BASE__INST1_SEG3: u32 = 0u32;
pub const HDP_BASE__INST1_SEG4: u32 = 0u32;
pub const HDP_BASE__INST2_SEG0: u32 = 0u32;
pub const HDP_BASE__INST2_SEG1: u32 = 0u32;
pub const HDP_BASE__INST2_SEG2: u32 = 0u32;
pub const HDP_BASE__INST2_SEG3: u32 = 0u32;
pub const HDP_BASE__INST2_SEG4: u32 = 0u32;
pub const HDP_BASE__INST3_SEG0: u32 = 0u32;
pub const HDP_BASE__INST3_SEG1: u32 = 0u32;
pub const HDP_BASE__INST3_SEG2: u32 = 0u32;
pub const HDP_BASE__INST3_SEG3: u32 = 0u32;
pub const HDP_BASE__INST3_SEG4: u32 = 0u32;
pub const HDP_BASE__INST4_SEG0: u32 = 0u32;
pub const HDP_BASE__INST4_SEG1: u32 = 0u32;
pub const HDP_BASE__INST4_SEG2: u32 = 0u32;
pub const HDP_BASE__INST4_SEG3: u32 = 0u32;
pub const HDP_BASE__INST4_SEG4: u32 = 0u32;
pub const HDP_BASE__INST5_SEG0: u32 = 0u32;
pub const HDP_BASE__INST5_SEG1: u32 = 0u32;
pub const HDP_BASE__INST5_SEG2: u32 = 0u32;
pub const HDP_BASE__INST5_SEG3: u32 = 0u32;
pub const HDP_BASE__INST5_SEG4: u32 = 0u32;
pub const HDP_BASE__INST6_SEG0: u32 = 0u32;
pub const HDP_BASE__INST6_SEG1: u32 = 0u32;
pub const HDP_BASE__INST6_SEG2: u32 = 0u32;
pub const HDP_BASE__INST6_SEG3: u32 = 0u32;
pub const HDP_BASE__INST6_SEG4: u32 = 0u32;
pub const MMHUB_BASE__INST0_SEG0: u32 = 0x0001A000u32;
pub const MMHUB_BASE__INST0_SEG1: u32 = 0x02408800u32;
pub const MMHUB_BASE__INST0_SEG2: u32 = 0u32;
pub const MMHUB_BASE__INST0_SEG3: u32 = 0u32;
pub const MMHUB_BASE__INST0_SEG4: u32 = 0u32;
pub const MMHUB_BASE__INST1_SEG0: u32 = 0u32;
pub const MMHUB_BASE__INST1_SEG1: u32 = 0u32;
pub const MMHUB_BASE__INST1_SEG2: u32 = 0u32;
pub const MMHUB_BASE__INST1_SEG3: u32 = 0u32;
pub const MMHUB_BASE__INST1_SEG4: u32 = 0u32;
pub const MMHUB_BASE__INST2_SEG0: u32 = 0u32;
pub const MMHUB_BASE__INST2_SEG1: u32 = 0u32;
pub const MMHUB_BASE__INST2_SEG2: u32 = 0u32;
pub const MMHUB_BASE__INST2_SEG3: u32 = 0u32;
pub const MMHUB_BASE__INST2_SEG4: u32 = 0u32;
pub const MMHUB_BASE__INST3_SEG0: u32 = 0u32;
pub const MMHUB_BASE__INST3_SEG1: u32 = 0u32;
pub const MMHUB_BASE__INST3_SEG2: u32 = 0u32;
pub const MMHUB_BASE__INST3_SEG3: u32 = 0u32;
pub const MMHUB_BASE__INST3_SEG4: u32 = 0u32;
pub const MMHUB_BASE__INST4_SEG0: u32 = 0u32;
pub const MMHUB_BASE__INST4_SEG1: u32 = 0u32;
pub const MMHUB_BASE__INST4_SEG2: u32 = 0u32;
pub const MMHUB_BASE__INST4_SEG3: u32 = 0u32;
pub const MMHUB_BASE__INST4_SEG4: u32 = 0u32;
pub const MMHUB_BASE__INST5_SEG0: u32 = 0u32;
pub const MMHUB_BASE__INST5_SEG1: u32 = 0u32;
pub const MMHUB_BASE__INST5_SEG2: u32 = 0u32;
pub const MMHUB_BASE__INST5_SEG3: u32 = 0u32;
pub const MMHUB_BASE__INST5_SEG4: u32 = 0u32;
pub const MMHUB_BASE__INST6_SEG0: u32 = 0u32;
pub const MMHUB_BASE__INST6_SEG1: u32 = 0u32;
pub const MMHUB_BASE__INST6_SEG2: u32 = 0u32;
pub const MMHUB_BASE__INST6_SEG3: u32 = 0u32;
pub const MMHUB_BASE__INST6_SEG4: u32 = 0u32;
pub const MP0_BASE__INST0_SEG0: u32 = 0x00016000u32;
pub const MP0_BASE__INST0_SEG1: u32 = 0x00DC0000u32;
pub const MP0_BASE__INST0_SEG2: u32 = 0x00E00000u32;
pub const MP0_BASE__INST0_SEG3: u32 = 0x00E40000u32;
pub const MP0_BASE__INST0_SEG4: u32 = 0x0243FC00u32;
pub const MP0_BASE__INST1_SEG0: u32 = 0u32;
pub const MP0_BASE__INST1_SEG1: u32 = 0u32;
pub const MP0_BASE__INST1_SEG2: u32 = 0u32;
pub const MP0_BASE__INST1_SEG3: u32 = 0u32;
pub const MP0_BASE__INST1_SEG4: u32 = 0u32;
pub const MP0_BASE__INST2_SEG0: u32 = 0u32;
pub const MP0_BASE__INST2_SEG1: u32 = 0u32;
pub const MP0_BASE__INST2_SEG2: u32 = 0u32;
pub const MP0_BASE__INST2_SEG3: u32 = 0u32;
pub const MP0_BASE__INST2_SEG4: u32 = 0u32;
pub const MP0_BASE__INST3_SEG0: u32 = 0u32;
pub const MP0_BASE__INST3_SEG1: u32 = 0u32;
pub const MP0_BASE__INST3_SEG2: u32 = 0u32;
pub const MP0_BASE__INST3_SEG3: u32 = 0u32;
pub const MP0_BASE__INST3_SEG4: u32 = 0u32;
pub const MP0_BASE__INST4_SEG0: u32 = 0u32;
pub const MP0_BASE__INST4_SEG1: u32 = 0u32;
pub const MP0_BASE__INST4_SEG2: u32 = 0u32;
pub const MP0_BASE__INST4_SEG3: u32 = 0u32;
pub const MP0_BASE__INST4_SEG4: u32 = 0u32;
pub const MP0_BASE__INST5_SEG0: u32 = 0u32;
pub const MP0_BASE__INST5_SEG1: u32 = 0u32;
pub const MP0_BASE__INST5_SEG2: u32 = 0u32;
pub const MP0_BASE__INST5_SEG3: u32 = 0u32;
pub const MP0_BASE__INST5_SEG4: u32 = 0u32;
pub const MP0_BASE__INST6_SEG0: u32 = 0u32;
pub const MP0_BASE__INST6_SEG1: u32 = 0u32;
pub const MP0_BASE__INST6_SEG2: u32 = 0u32;
pub const MP0_BASE__INST6_SEG3: u32 = 0u32;
pub const MP0_BASE__INST6_SEG4: u32 = 0u32;
pub const MP1_BASE__INST0_SEG0: u32 = 0x00016200u32;
pub const MP1_BASE__INST0_SEG1: u32 = 0x00E80000u32;
pub const MP1_BASE__INST0_SEG2: u32 = 0x00EC0000u32;
pub const MP1_BASE__INST0_SEG3: u32 = 0x00F00000u32;
pub const MP1_BASE__INST0_SEG4: u32 = 0x02400400u32;
pub const MP1_BASE__INST1_SEG0: u32 = 0u32;
pub const MP1_BASE__INST1_SEG1: u32 = 0u32;
pub const MP1_BASE__INST1_SEG2: u32 = 0u32;
pub const MP1_BASE__INST1_SEG3: u32 = 0u32;
pub const MP1_BASE__INST1_SEG4: u32 = 0u32;
pub const MP1_BASE__INST2_SEG0: u32 = 0u32;
pub const MP1_BASE__INST2_SEG1: u32 = 0u32;
pub const MP1_BASE__INST2_SEG2: u32 = 0u32;
pub const MP1_BASE__INST2_SEG3: u32 = 0u32;
pub const MP1_BASE__INST2_SEG4: u32 = 0u32;
pub const MP1_BASE__INST3_SEG0: u32 = 0u32;
pub const MP1_BASE__INST3_SEG1: u32 = 0u32;
pub const MP1_BASE__INST3_SEG2: u32 = 0u32;
pub const MP1_BASE__INST3_SEG3: u32 = 0u32;
pub const MP1_BASE__INST3_SEG4: u32 = 0u32;
pub const MP1_BASE__INST4_SEG0: u32 = 0u32;
pub const MP1_BASE__INST4_SEG1: u32 = 0u32;
pub const MP1_BASE__INST4_SEG2: u32 = 0u32;
pub const MP1_BASE__INST4_SEG3: u32 = 0u32;
pub const MP1_BASE__INST4_SEG4: u32 = 0u32;
pub const MP1_BASE__INST5_SEG0: u32 = 0u32;
pub const MP1_BASE__INST5_SEG1: u32 = 0u32;
pub const MP1_BASE__INST5_SEG2: u32 = 0u32;
pub const MP1_BASE__INST5_SEG3: u32 = 0u32;
pub const MP1_BASE__INST5_SEG4: u32 = 0u32;
pub const MP1_BASE__INST6_SEG0: u32 = 0u32;
pub const MP1_BASE__INST6_SEG1: u32 = 0u32;
pub const MP1_BASE__INST6_SEG2: u32 = 0u32;
pub const MP1_BASE__INST6_SEG3: u32 = 0u32;
pub const MP1_BASE__INST6_SEG4: u32 = 0u32;
pub const NBIF0_BASE__INST0_SEG0: u32 = 0x00000000u32;
pub const NBIF0_BASE__INST0_SEG1: u32 = 0x00000014u32;
pub const NBIF0_BASE__INST0_SEG2: u32 = 0x00000D20u32;
pub const NBIF0_BASE__INST0_SEG3: u32 = 0x00010400u32;
pub const NBIF0_BASE__INST0_SEG4: u32 = 0x0241B000u32;
pub const NBIF0_BASE__INST1_SEG0: u32 = 0u32;
pub const NBIF0_BASE__INST1_SEG1: u32 = 0u32;
pub const NBIF0_BASE__INST1_SEG2: u32 = 0u32;
pub const NBIF0_BASE__INST1_SEG3: u32 = 0u32;
pub const NBIF0_BASE__INST1_SEG4: u32 = 0u32;
pub const NBIF0_BASE__INST2_SEG0: u32 = 0u32;
pub const NBIF0_BASE__INST2_SEG1: u32 = 0u32;
pub const NBIF0_BASE__INST2_SEG2: u32 = 0u32;
pub const NBIF0_BASE__INST2_SEG3: u32 = 0u32;
pub const NBIF0_BASE__INST2_SEG4: u32 = 0u32;
pub const NBIF0_BASE__INST3_SEG0: u32 = 0u32;
pub const NBIF0_BASE__INST3_SEG1: u32 = 0u32;
pub const NBIF0_BASE__INST3_SEG2: u32 = 0u32;
pub const NBIF0_BASE__INST3_SEG3: u32 = 0u32;
pub const NBIF0_BASE__INST3_SEG4: u32 = 0u32;
pub const NBIF0_BASE__INST4_SEG0: u32 = 0u32;
pub const NBIF0_BASE__INST4_SEG1: u32 = 0u32;
pub const NBIF0_BASE__INST4_SEG2: u32 = 0u32;
pub const NBIF0_BASE__INST4_SEG3: u32 = 0u32;
pub const NBIF0_BASE__INST4_SEG4: u32 = 0u32;
pub const NBIF0_BASE__INST5_SEG0: u32 = 0u32;
pub const NBIF0_BASE__INST5_SEG1: u32 = 0u32;
pub const NBIF0_BASE__INST5_SEG2: u32 = 0u32;
pub const NBIF0_BASE__INST5_SEG3: u32 = 0u32;
pub const NBIF0_BASE__INST5_SEG4: u32 = 0u32;
pub const NBIF0_BASE__INST6_SEG0: u32 = 0u32;
pub const NBIF0_BASE__INST6_SEG1: u32 = 0u32;
pub const NBIF0_BASE__INST6_SEG2: u32 = 0u32;
pub const NBIF0_BASE__INST6_SEG3: u32 = 0u32;
pub const NBIF0_BASE__INST6_SEG4: u32 = 0u32;
pub const OSSSYS_BASE__INST0_SEG0: u32 = 0x000010A0u32;
pub const OSSSYS_BASE__INST0_SEG1: u32 = 0x0240A000u32;
pub const OSSSYS_BASE__INST0_SEG2: u32 = 0u32;
pub const OSSSYS_BASE__INST0_SEG3: u32 = 0u32;
pub const OSSSYS_BASE__INST0_SEG4: u32 = 0u32;
pub const OSSSYS_BASE__INST1_SEG0: u32 = 0u32;
pub const OSSSYS_BASE__INST1_SEG1: u32 = 0u32;
pub const OSSSYS_BASE__INST1_SEG2: u32 = 0u32;
pub const OSSSYS_BASE__INST1_SEG3: u32 = 0u32;
pub const OSSSYS_BASE__INST1_SEG4: u32 = 0u32;
pub const OSSSYS_BASE__INST2_SEG0: u32 = 0u32;
pub const OSSSYS_BASE__INST2_SEG1: u32 = 0u32;
pub const OSSSYS_BASE__INST2_SEG2: u32 = 0u32;
pub const OSSSYS_BASE__INST2_SEG3: u32 = 0u32;
pub const OSSSYS_BASE__INST2_SEG4: u32 = 0u32;
pub const OSSSYS_BASE__INST3_SEG0: u32 = 0u32;
pub const OSSSYS_BASE__INST3_SEG1: u32 = 0u32;
pub const OSSSYS_BASE__INST3_SEG2: u32 = 0u32;
pub const OSSSYS_BASE__INST3_SEG3: u32 = 0u32;
pub const OSSSYS_BASE__INST3_SEG4: u32 = 0u32;
pub const OSSSYS_BASE__INST4_SEG0: u32 = 0u32;
pub const OSSSYS_BASE__INST4_SEG1: u32 = 0u32;
pub const OSSSYS_BASE__INST4_SEG2: u32 = 0u32;
pub const OSSSYS_BASE__INST4_SEG3: u32 = 0u32;
pub const OSSSYS_BASE__INST4_SEG4: u32 = 0u32;
pub const OSSSYS_BASE__INST5_SEG0: u32 = 0u32;
pub const OSSSYS_BASE__INST5_SEG1: u32 = 0u32;
pub const OSSSYS_BASE__INST5_SEG2: u32 = 0u32;
pub const OSSSYS_BASE__INST5_SEG3: u32 = 0u32;
pub const OSSSYS_BASE__INST5_SEG4: u32 = 0u32;
pub const OSSSYS_BASE__INST6_SEG0: u32 = 0u32;
pub const OSSSYS_BASE__INST6_SEG1: u32 = 0u32;
pub const OSSSYS_BASE__INST6_SEG2: u32 = 0u32;
pub const OSSSYS_BASE__INST6_SEG3: u32 = 0u32;
pub const OSSSYS_BASE__INST6_SEG4: u32 = 0u32;
pub const PCIE0_BASE__INST0_SEG0: u32 = 0x02411800u32;
pub const PCIE0_BASE__INST0_SEG1: u32 = 0x04440000u32;
pub const PCIE0_BASE__INST0_SEG2: u32 = 0u32;
pub const PCIE0_BASE__INST0_SEG3: u32 = 0u32;
pub const PCIE0_BASE__INST0_SEG4: u32 = 0u32;
pub const PCIE0_BASE__INST1_SEG0: u32 = 0u32;
pub const PCIE0_BASE__INST1_SEG1: u32 = 0u32;
pub const PCIE0_BASE__INST1_SEG2: u32 = 0u32;
pub const PCIE0_BASE__INST1_SEG3: u32 = 0u32;
pub const PCIE0_BASE__INST1_SEG4: u32 = 0u32;
pub const PCIE0_BASE__INST2_SEG0: u32 = 0u32;
pub const PCIE0_BASE__INST2_SEG1: u32 = 0u32;
pub const PCIE0_BASE__INST2_SEG2: u32 = 0u32;
pub const PCIE0_BASE__INST2_SEG3: u32 = 0u32;
pub const PCIE0_BASE__INST2_SEG4: u32 = 0u32;
pub const PCIE0_BASE__INST3_SEG0: u32 = 0u32;
pub const PCIE0_BASE__INST3_SEG1: u32 = 0u32;
pub const PCIE0_BASE__INST3_SEG2: u32 = 0u32;
pub const PCIE0_BASE__INST3_SEG3: u32 = 0u32;
pub const PCIE0_BASE__INST3_SEG4: u32 = 0u32;
pub const PCIE0_BASE__INST4_SEG0: u32 = 0u32;
pub const PCIE0_BASE__INST4_SEG1: u32 = 0u32;
pub const PCIE0_BASE__INST4_SEG2: u32 = 0u32;
pub const PCIE0_BASE__INST4_SEG3: u32 = 0u32;
pub const PCIE0_BASE__INST4_SEG4: u32 = 0u32;
pub const PCIE0_BASE__INST5_SEG0: u32 = 0u32;
pub const PCIE0_BASE__INST5_SEG1: u32 = 0u32;
pub const PCIE0_BASE__INST5_SEG2: u32 = 0u32;
pub const PCIE0_BASE__INST5_SEG3: u32 = 0u32;
pub const PCIE0_BASE__INST5_SEG4: u32 = 0u32;
pub const PCIE0_BASE__INST6_SEG0: u32 = 0u32;
pub const PCIE0_BASE__INST6_SEG1: u32 = 0u32;
pub const PCIE0_BASE__INST6_SEG2: u32 = 0u32;
pub const PCIE0_BASE__INST6_SEG3: u32 = 0u32;
pub const PCIE0_BASE__INST6_SEG4: u32 = 0u32;
pub const SDMA_BASE__INST0_SEG0: u32 = 0x00001260u32;
pub const SDMA_BASE__INST0_SEG1: u32 = 0x0000A000u32;
pub const SDMA_BASE__INST0_SEG2: u32 = 0x02402C00u32;
pub const SDMA_BASE__INST0_SEG3: u32 = 0u32;
pub const SDMA_BASE__INST0_SEG4: u32 = 0u32;
pub const SDMA_BASE__INST1_SEG0: u32 = 0x00001260u32;
pub const SDMA_BASE__INST1_SEG1: u32 = 0x0000A000u32;
pub const SDMA_BASE__INST1_SEG2: u32 = 0x02402C00u32;
pub const SDMA_BASE__INST1_SEG3: u32 = 0u32;
pub const SDMA_BASE__INST1_SEG4: u32 = 0u32;
pub const SDMA_BASE__INST2_SEG0: u32 = 0u32;
pub const SDMA_BASE__INST2_SEG1: u32 = 0u32;
pub const SDMA_BASE__INST2_SEG2: u32 = 0u32;
pub const SDMA_BASE__INST2_SEG3: u32 = 0u32;
pub const SDMA_BASE__INST2_SEG4: u32 = 0u32;
pub const SDMA_BASE__INST3_SEG0: u32 = 0u32;
pub const SDMA_BASE__INST3_SEG1: u32 = 0u32;
pub const SDMA_BASE__INST3_SEG2: u32 = 0u32;
pub const SDMA_BASE__INST3_SEG3: u32 = 0u32;
pub const SDMA_BASE__INST3_SEG4: u32 = 0u32;
pub const SDMA_BASE__INST4_SEG0: u32 = 0u32;
pub const SDMA_BASE__INST4_SEG1: u32 = 0u32;
pub const SDMA_BASE__INST4_SEG2: u32 = 0u32;
pub const SDMA_BASE__INST4_SEG3: u32 = 0u32;
pub const SDMA_BASE__INST4_SEG4: u32 = 0u32;
pub const SDMA_BASE__INST5_SEG0: u32 = 0u32;
pub const SDMA_BASE__INST5_SEG1: u32 = 0u32;
pub const SDMA_BASE__INST5_SEG2: u32 = 0u32;
pub const SDMA_BASE__INST5_SEG3: u32 = 0u32;
pub const SDMA_BASE__INST5_SEG4: u32 = 0u32;
pub const SDMA_BASE__INST6_SEG0: u32 = 0u32;
pub const SDMA_BASE__INST6_SEG1: u32 = 0u32;
pub const SDMA_BASE__INST6_SEG2: u32 = 0u32;
pub const SDMA_BASE__INST6_SEG3: u32 = 0u32;
pub const SDMA_BASE__INST6_SEG4: u32 = 0u32;
pub const SMUIO_BASE__INST0_SEG0: u32 = 0x00016800u32;
pub const SMUIO_BASE__INST0_SEG1: u32 = 0x00016A00u32;
pub const SMUIO_BASE__INST0_SEG2: u32 = 0x00440000u32;
pub const SMUIO_BASE__INST0_SEG3: u32 = 0x02401000u32;
pub const SMUIO_BASE__INST0_SEG4: u32 = 0u32;
pub const SMUIO_BASE__INST1_SEG0: u32 = 0u32;
pub const SMUIO_BASE__INST1_SEG1: u32 = 0u32;
pub const SMUIO_BASE__INST1_SEG2: u32 = 0u32;
pub const SMUIO_BASE__INST1_SEG3: u32 = 0u32;
pub const SMUIO_BASE__INST1_SEG4: u32 = 0u32;
pub const SMUIO_BASE__INST2_SEG0: u32 = 0u32;
pub const SMUIO_BASE__INST2_SEG1: u32 = 0u32;
pub const SMUIO_BASE__INST2_SEG2: u32 = 0u32;
pub const SMUIO_BASE__INST2_SEG3: u32 = 0u32;
pub const SMUIO_BASE__INST2_SEG4: u32 = 0u32;
pub const SMUIO_BASE__INST3_SEG0: u32 = 0u32;
pub const SMUIO_BASE__INST3_SEG1: u32 = 0u32;
pub const SMUIO_BASE__INST3_SEG2: u32 = 0u32;
pub const SMUIO_BASE__INST3_SEG3: u32 = 0u32;
pub const SMUIO_BASE__INST3_SEG4: u32 = 0u32;
pub const SMUIO_BASE__INST4_SEG0: u32 = 0u32;
pub const SMUIO_BASE__INST4_SEG1: u32 = 0u32;
pub const SMUIO_BASE__INST4_SEG2: u32 = 0u32;
pub const SMUIO_BASE__INST4_SEG3: u32 = 0u32;
pub const SMUIO_BASE__INST4_SEG4: u32 = 0u32;
pub const SMUIO_BASE__INST5_SEG0: u32 = 0u32;
pub const SMUIO_BASE__INST5_SEG1: u32 = 0u32;
pub const SMUIO_BASE__INST5_SEG2: u32 = 0u32;
pub const SMUIO_BASE__INST5_SEG3: u32 = 0u32;
pub const SMUIO_BASE__INST5_SEG4: u32 = 0u32;
pub const SMUIO_BASE__INST6_SEG0: u32 = 0u32;
pub const SMUIO_BASE__INST6_SEG1: u32 = 0u32;
pub const SMUIO_BASE__INST6_SEG2: u32 = 0u32;
pub const SMUIO_BASE__INST6_SEG3: u32 = 0u32;
pub const SMUIO_BASE__INST6_SEG4: u32 = 0u32;
pub const THM_BASE__INST0_SEG0: u32 = 0x00016600u32;
pub const THM_BASE__INST0_SEG1: u32 = 0x02400C00u32;
pub const THM_BASE__INST0_SEG2: u32 = 0u32;
pub const THM_BASE__INST0_SEG3: u32 = 0u32;
pub const THM_BASE__INST0_SEG4: u32 = 0u32;
pub const THM_BASE__INST1_SEG0: u32 = 0u32;
pub const THM_BASE__INST1_SEG1: u32 = 0u32;
pub const THM_BASE__INST1_SEG2: u32 = 0u32;
pub const THM_BASE__INST1_SEG3: u32 = 0u32;
pub const THM_BASE__INST1_SEG4: u32 = 0u32;
pub const THM_BASE__INST2_SEG0: u32 = 0u32;
pub const THM_BASE__INST2_SEG1: u32 = 0u32;
pub const THM_BASE__INST2_SEG2: u32 = 0u32;
pub const THM_BASE__INST2_SEG3: u32 = 0u32;
pub const THM_BASE__INST2_SEG4: u32 = 0u32;
pub const THM_BASE__INST3_SEG0: u32 = 0u32;
pub const THM_BASE__INST3_SEG1: u32 = 0u32;
pub const THM_BASE__INST3_SEG2: u32 = 0u32;
pub const THM_BASE__INST3_SEG3: u32 = 0u32;
pub const THM_BASE__INST3_SEG4: u32 = 0u32;
pub const THM_BASE__INST4_SEG0: u32 = 0u32;
pub const THM_BASE__INST4_SEG1: u32 = 0u32;
pub const THM_BASE__INST4_SEG2: u32 = 0u32;
pub const THM_BASE__INST4_SEG3: u32 = 0u32;
pub const THM_BASE__INST4_SEG4: u32 = 0u32;
pub const THM_BASE__INST5_SEG0: u32 = 0u32;
pub const THM_BASE__INST5_SEG1: u32 = 0u32;
pub const THM_BASE__INST5_SEG2: u32 = 0u32;
pub const THM_BASE__INST5_SEG3: u32 = 0u32;
pub const THM_BASE__INST5_SEG4: u32 = 0u32;
pub const THM_BASE__INST6_SEG0: u32 = 0u32;
pub const THM_BASE__INST6_SEG1: u32 = 0u32;
pub const THM_BASE__INST6_SEG2: u32 = 0u32;
pub const THM_BASE__INST6_SEG3: u32 = 0u32;
pub const THM_BASE__INST6_SEG4: u32 = 0u32;
pub const UMC_BASE__INST0_SEG0: u32 = 0x00014000u32;
pub const UMC_BASE__INST0_SEG1: u32 = 0x02425800u32;
pub const UMC_BASE__INST0_SEG2: u32 = 0u32;
pub const UMC_BASE__INST0_SEG3: u32 = 0u32;
pub const UMC_BASE__INST0_SEG4: u32 = 0u32;
pub const UMC_BASE__INST1_SEG0: u32 = 0x00054000u32;
pub const UMC_BASE__INST1_SEG1: u32 = 0x02425C00u32;
pub const UMC_BASE__INST1_SEG2: u32 = 0u32;
pub const UMC_BASE__INST1_SEG3: u32 = 0u32;
pub const UMC_BASE__INST1_SEG4: u32 = 0u32;
pub const UMC_BASE__INST2_SEG0: u32 = 0x00094000u32;
pub const UMC_BASE__INST2_SEG1: u32 = 0x02426000u32;
pub const UMC_BASE__INST2_SEG2: u32 = 0u32;
pub const UMC_BASE__INST2_SEG3: u32 = 0u32;
pub const UMC_BASE__INST2_SEG4: u32 = 0u32;
pub const UMC_BASE__INST3_SEG0: u32 = 0x000D4000u32;
pub const UMC_BASE__INST3_SEG1: u32 = 0x02426400u32;
pub const UMC_BASE__INST3_SEG2: u32 = 0u32;
pub const UMC_BASE__INST3_SEG3: u32 = 0u32;
pub const UMC_BASE__INST3_SEG4: u32 = 0u32;
pub const UMC_BASE__INST4_SEG0: u32 = 0u32;
pub const UMC_BASE__INST4_SEG1: u32 = 0u32;
pub const UMC_BASE__INST4_SEG2: u32 = 0u32;
pub const UMC_BASE__INST4_SEG3: u32 = 0u32;
pub const UMC_BASE__INST4_SEG4: u32 = 0u32;
pub const UMC_BASE__INST5_SEG0: u32 = 0u32;
pub const UMC_BASE__INST5_SEG1: u32 = 0u32;
pub const UMC_BASE__INST5_SEG2: u32 = 0u32;
pub const UMC_BASE__INST5_SEG3: u32 = 0u32;
pub const UMC_BASE__INST5_SEG4: u32 = 0u32;
pub const UMC_BASE__INST6_SEG0: u32 = 0u32;
pub const UMC_BASE__INST6_SEG1: u32 = 0u32;
pub const UMC_BASE__INST6_SEG2: u32 = 0u32;
pub const UMC_BASE__INST6_SEG3: u32 = 0u32;
pub const UMC_BASE__INST6_SEG4: u32 = 0u32;
pub const USB0_BASE__INST0_SEG0: u32 = 0x0242A800u32;
pub const USB0_BASE__INST0_SEG1: u32 = 0x05B00000u32;
pub const USB0_BASE__INST0_SEG2: u32 = 0u32;
pub const USB0_BASE__INST0_SEG3: u32 = 0u32;
pub const USB0_BASE__INST0_SEG4: u32 = 0u32;
pub const USB0_BASE__INST1_SEG0: u32 = 0u32;
pub const USB0_BASE__INST1_SEG1: u32 = 0u32;
pub const USB0_BASE__INST1_SEG2: u32 = 0u32;
pub const USB0_BASE__INST1_SEG3: u32 = 0u32;
pub const USB0_BASE__INST1_SEG4: u32 = 0u32;
pub const USB0_BASE__INST2_SEG0: u32 = 0u32;
pub const USB0_BASE__INST2_SEG1: u32 = 0u32;
pub const USB0_BASE__INST2_SEG2: u32 = 0u32;
pub const USB0_BASE__INST2_SEG3: u32 = 0u32;
pub const USB0_BASE__INST2_SEG4: u32 = 0u32;
pub const USB0_BASE__INST3_SEG0: u32 = 0u32;
pub const USB0_BASE__INST3_SEG1: u32 = 0u32;
pub const USB0_BASE__INST3_SEG2: u32 = 0u32;
pub const USB0_BASE__INST3_SEG3: u32 = 0u32;
pub const USB0_BASE__INST3_SEG4: u32 = 0u32;
pub const USB0_BASE__INST4_SEG0: u32 = 0u32;
pub const USB0_BASE__INST4_SEG1: u32 = 0u32;
pub const USB0_BASE__INST4_SEG2: u32 = 0u32;
pub const USB0_BASE__INST4_SEG3: u32 = 0u32;
pub const USB0_BASE__INST4_SEG4: u32 = 0u32;
pub const USB0_BASE__INST5_SEG0: u32 = 0u32;
pub const USB0_BASE__INST5_SEG1: u32 = 0u32;
pub const USB0_BASE__INST5_SEG2: u32 = 0u32;
pub const USB0_BASE__INST5_SEG3: u32 = 0u32;
pub const USB0_BASE__INST5_SEG4: u32 = 0u32;
pub const USB0_BASE__INST6_SEG0: u32 = 0u32;
pub const USB0_BASE__INST6_SEG1: u32 = 0u32;
pub const USB0_BASE__INST6_SEG2: u32 = 0u32;
pub const USB0_BASE__INST6_SEG3: u32 = 0u32;
pub const USB0_BASE__INST6_SEG4: u32 = 0u32;
pub const UVD0_BASE__INST0_SEG0: u32 = 0x00007800u32;
pub const UVD0_BASE__INST0_SEG1: u32 = 0x00007E00u32;
pub const UVD0_BASE__INST0_SEG2: u32 = 0x02403000u32;
pub const UVD0_BASE__INST0_SEG3: u32 = 0u32;
pub const UVD0_BASE__INST0_SEG4: u32 = 0u32;
pub const UVD0_BASE__INST1_SEG0: u32 = 0u32;
pub const UVD0_BASE__INST1_SEG1: u32 = 0u32;
pub const UVD0_BASE__INST1_SEG2: u32 = 0u32;
pub const UVD0_BASE__INST1_SEG3: u32 = 0u32;
pub const UVD0_BASE__INST1_SEG4: u32 = 0u32;
pub const UVD0_BASE__INST2_SEG0: u32 = 0u32;
pub const UVD0_BASE__INST2_SEG1: u32 = 0u32;
pub const UVD0_BASE__INST2_SEG2: u32 = 0u32;
pub const UVD0_BASE__INST2_SEG3: u32 = 0u32;
pub const UVD0_BASE__INST2_SEG4: u32 = 0u32;
pub const UVD0_BASE__INST3_SEG0: u32 = 0u32;
pub const UVD0_BASE__INST3_SEG1: u32 = 0u32;
pub const UVD0_BASE__INST3_SEG2: u32 = 0u32;
pub const UVD0_BASE__INST3_SEG3: u32 = 0u32;
pub const UVD0_BASE__INST3_SEG4: u32 = 0u32;
pub const UVD0_BASE__INST4_SEG0: u32 = 0u32;
pub const UVD0_BASE__INST4_SEG1: u32 = 0u32;
pub const UVD0_BASE__INST4_SEG2: u32 = 0u32;
pub const UVD0_BASE__INST4_SEG3: u32 = 0u32;
pub const UVD0_BASE__INST4_SEG4: u32 = 0u32;
pub const UVD0_BASE__INST5_SEG0: u32 = 0u32;
pub const UVD0_BASE__INST5_SEG1: u32 = 0u32;
pub const UVD0_BASE__INST5_SEG2: u32 = 0u32;
pub const UVD0_BASE__INST5_SEG3: u32 = 0u32;
pub const UVD0_BASE__INST5_SEG4: u32 = 0u32;
pub const UVD0_BASE__INST6_SEG0: u32 = 0u32;
pub const UVD0_BASE__INST6_SEG1: u32 = 0u32;
pub const UVD0_BASE__INST6_SEG2: u32 = 0u32;
pub const UVD0_BASE__INST6_SEG3: u32 = 0u32;
pub const UVD0_BASE__INST6_SEG4: u32 = 0u32;
pub static ATHUB_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000C00u32, 0x02408C00u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static CLK_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016C00u32, 0x02401800u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0x00016E00u32, 0x02401C00u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0x00017000u32, 0x02402000u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0x00017200u32, 0x02402400u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0x0001B000u32, 0x0242D800u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0x00017E00u32, 0x0240BC00u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static DF_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00007000u32, 0x0240B800u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static DIO_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x02404000u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static DMU_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000012u32, 0x000000C0u32, 0x000034C0u32, 0x00009000u32, 0x02403C00u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static DPCS_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000012u32, 0x000000C0u32, 0x000034C0u32, 0x00009000u32, 0x02403C00u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static FUSE_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00017400u32, 0x02401400u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static GC_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00001260u32, 0x0000A000u32, 0x02402C00u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static HDA_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x004C0000u32, 0x02404800u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static HDP_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000F20u32, 0x0240A400u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static MMHUB_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x0001A000u32, 0x02408800u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static MP0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016000u32, 0x00DC0000u32, 0x00E00000u32, 0x00E40000u32, 0x0243FC00u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static MP1_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016000u32, 0x00E80000u32, 0x00EC0000u32, 0x00F00000u32, 0x02400400u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static NBIF0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000000u32, 0x00000014u32, 0x00000D20u32, 0x00010400u32, 0x0241B000u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static OSSSYS_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x000010A0u32, 0x0240A000u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static PCIE0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x02411800u32, 0x04440000u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static SDMA_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00001260u32, 0x0000A000u32, 0x02402C00u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0x00001260u32, 0x0000A000u32, 0x02402C00u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static SMUIO_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016800u32, 0x00016A00u32, 0x00440000u32, 0x02401000u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static THM_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016600u32, 0x02400C00u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static UMC_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00014000u32, 0x02425800u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0x00054000u32, 0x02425C00u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0x00094000u32, 0x02426000u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0x000D4000u32, 0x02426400u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static USB0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x0242A800u32, 0x05B00000u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };
pub static UVD0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00007800u32, 0x00007E00u32, 0x02403000u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
    IP_BASE_INSTANCE { segment: [0u32, 0u32, 0u32, 0u32, 0u32] },
] };


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
