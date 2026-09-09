/* Rust translation of arct_ip_offset.h. */
#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub const MAX_INSTANCE: usize = 8;
pub const MAX_SEGMENT: usize = 6;

#[repr(C)]
pub struct IP_BASE_INSTANCE { pub segment: [u32; MAX_SEGMENT] }
#[repr(C)]
pub struct IP_BASE { pub instance: [IP_BASE_INSTANCE; MAX_INSTANCE] }

pub static ATHUB_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000C20, 0x00012460, 0x00408C00, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static CLK_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x000120C0, 0x00016C00, 0x00401800, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x000120E0, 0x00016E00, 0x00401C00, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00012100, 0x00017000, 0x00402000, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00012120, 0x00017200, 0x00402400, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x000136C0, 0x0001B000, 0x0042D800, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00013720, 0x0001B200, 0x0042E400, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x000125E0, 0x00017E00, 0x0040BC00, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static DF_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00007000, 0x000125C0, 0x0040B800, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static FUSE_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x000120A0, 0x00017400, 0x00401400, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static GC_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00002000, 0x0000A000, 0x00012160, 0x00402C00, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static HDP_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000F20, 0x00012520, 0x0040A400, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static MMHUB_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00012440, 0x0001A000, 0x00408800, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static MP0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016000, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static MP1_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016000, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static NBIF0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00000000, 0x00000014, 0x00000D20, 0x00010400, 0x00012D80, 0x0041B000] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static OSSSYS_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x000010A0, 0x00012500, 0x0040A000, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static PCIE0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x000128C0, 0x00411800, 0x04440000, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static SDMA0_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00001260, 0x00012540, 0x0040A800, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static SDMA1_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00001860, 0x00012560, 0x0040AC00, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static SDMA2_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00013760, 0x0001E000, 0x0042EC00, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static SDMA3_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00013780, 0x0001E400, 0x0042F000, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static SDMA4_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x000137A0, 0x0001E800, 0x0042F400, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static SDMA5_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x000137C0, 0x0001EC00, 0x0042F800, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static SDMA6_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x000137E0, 0x0001F000, 0x0042FC00, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static SDMA7_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00013800, 0x0001F400, 0x00430000, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static SMUIO_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016800, 0x00016A00, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [, , , , , ] },
    IP_BASE_INSTANCE { segment: [, , , , , ] }
] };

pub static THM_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00016600, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [, , , , , ] },
    IP_BASE_INSTANCE { segment: [, , , , , ] }
] };

pub static UMC_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x000132C0, 0x00014000, 0x00425800, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x000132E0, 0x00054000, 0x00425C00, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00013300, 0x00094000, 0x00426000, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00013320, 0x000D4000, 0x00426400, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00013340, 0x00114000, 0x00426800, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00013360, 0x00154000, 0x00426C00, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00013380, 0x00194000, 0x00427000, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x000133A0, 0x001D4000, 0x00427400, 0, 0, 0] }
] };

pub static UVD_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00007800, 0x00007E00, 0x00012180, 0x00403000, 0, 0] },
    IP_BASE_INSTANCE { segment: [0x00007A00, 0x00009000, 0x000136E0, 0x0042DC00, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static DBGU_IO_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x000001E0, 0x000125A0, 0x0040B400, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub static RSMU_BASE: IP_BASE = IP_BASE { instance: [
    IP_BASE_INSTANCE { segment: [0x00012000, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] },
    IP_BASE_INSTANCE { segment: [0, 0, 0, 0, 0, 0] }
] };

pub const ATHUB_BASE__INST0_SEG0: u32 = 0x00000C20;
pub const ATHUB_BASE__INST0_SEG1: u32 = 0x00012460;
pub const ATHUB_BASE__INST0_SEG2: u32 = 0x00408C00;
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
pub const ATHUB_BASE__INST7_SEG0: u32 = 0;
pub const ATHUB_BASE__INST7_SEG1: u32 = 0;
pub const ATHUB_BASE__INST7_SEG2: u32 = 0;
pub const ATHUB_BASE__INST7_SEG3: u32 = 0;
pub const ATHUB_BASE__INST7_SEG4: u32 = 0;
pub const ATHUB_BASE__INST7_SEG5: u32 = 0;
pub const CLK_BASE__INST0_SEG0: u32 = 0x000120C0;
pub const CLK_BASE__INST0_SEG1: u32 = 0x00016C00;
pub const CLK_BASE__INST0_SEG2: u32 = 0x00401800;
pub const CLK_BASE__INST0_SEG3: u32 = 0;
pub const CLK_BASE__INST0_SEG4: u32 = 0;
pub const CLK_BASE__INST0_SEG5: u32 = 0;
pub const CLK_BASE__INST1_SEG0: u32 = 0x000120E0;
pub const CLK_BASE__INST1_SEG1: u32 = 0x00016E00;
pub const CLK_BASE__INST1_SEG2: u32 = 0x00401C00;
pub const CLK_BASE__INST1_SEG3: u32 = 0;
pub const CLK_BASE__INST1_SEG4: u32 = 0;
pub const CLK_BASE__INST1_SEG5: u32 = 0;
pub const CLK_BASE__INST2_SEG0: u32 = 0x00012100;
pub const CLK_BASE__INST2_SEG1: u32 = 0x00017000;
pub const CLK_BASE__INST2_SEG2: u32 = 0x00402000;
pub const CLK_BASE__INST2_SEG3: u32 = 0;
pub const CLK_BASE__INST2_SEG4: u32 = 0;
pub const CLK_BASE__INST2_SEG5: u32 = 0;
pub const CLK_BASE__INST3_SEG0: u32 = 0x00012120;
pub const CLK_BASE__INST3_SEG1: u32 = 0x00017200;
pub const CLK_BASE__INST3_SEG2: u32 = 0x00402400;
pub const CLK_BASE__INST3_SEG3: u32 = 0;
pub const CLK_BASE__INST3_SEG4: u32 = 0;
pub const CLK_BASE__INST3_SEG5: u32 = 0;
pub const CLK_BASE__INST4_SEG0: u32 = 0x000136C0;
pub const CLK_BASE__INST4_SEG1: u32 = 0x0001B000;
pub const CLK_BASE__INST4_SEG2: u32 = 0x0042D800;
pub const CLK_BASE__INST4_SEG3: u32 = 0;
pub const CLK_BASE__INST4_SEG4: u32 = 0;
pub const CLK_BASE__INST4_SEG5: u32 = 0;
pub const CLK_BASE__INST5_SEG0: u32 = 0x00013720;
pub const CLK_BASE__INST5_SEG1: u32 = 0x0001B200;
pub const CLK_BASE__INST5_SEG2: u32 = 0x0042E400;
pub const CLK_BASE__INST5_SEG3: u32 = 0;
pub const CLK_BASE__INST5_SEG4: u32 = 0;
pub const CLK_BASE__INST5_SEG5: u32 = 0;
pub const CLK_BASE__INST6_SEG0: u32 = 0x000125E0;
pub const CLK_BASE__INST6_SEG1: u32 = 0x00017E00;
pub const CLK_BASE__INST6_SEG2: u32 = 0x0040BC00;
pub const CLK_BASE__INST6_SEG3: u32 = 0;
pub const CLK_BASE__INST6_SEG4: u32 = 0;
pub const CLK_BASE__INST6_SEG5: u32 = 0;
pub const CLK_BASE__INST7_SEG0: u32 = 0;
pub const CLK_BASE__INST7_SEG1: u32 = 0;
pub const CLK_BASE__INST7_SEG2: u32 = 0;
pub const CLK_BASE__INST7_SEG3: u32 = 0;
pub const CLK_BASE__INST7_SEG4: u32 = 0;
pub const CLK_BASE__INST7_SEG5: u32 = 0;
pub const DF_BASE__INST0_SEG0: u32 = 0x00007000;
pub const DF_BASE__INST0_SEG1: u32 = 0x000125C0;
pub const DF_BASE__INST0_SEG2: u32 = 0x0040B800;
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
pub const DF_BASE__INST7_SEG0: u32 = 0;
pub const DF_BASE__INST7_SEG1: u32 = 0;
pub const DF_BASE__INST7_SEG2: u32 = 0;
pub const DF_BASE__INST7_SEG3: u32 = 0;
pub const DF_BASE__INST7_SEG4: u32 = 0;
pub const DF_BASE__INST7_SEG5: u32 = 0;
pub const FUSE_BASE__INST0_SEG0: u32 = 0x000120A0;
pub const FUSE_BASE__INST0_SEG1: u32 = 0x00017400;
pub const FUSE_BASE__INST0_SEG2: u32 = 0x00401400;
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
pub const FUSE_BASE__INST7_SEG0: u32 = 0;
pub const FUSE_BASE__INST7_SEG1: u32 = 0;
pub const FUSE_BASE__INST7_SEG2: u32 = 0;
pub const FUSE_BASE__INST7_SEG3: u32 = 0;
pub const FUSE_BASE__INST7_SEG4: u32 = 0;
pub const FUSE_BASE__INST7_SEG5: u32 = 0;
pub const GC_BASE__INST0_SEG0: u32 = 0x00002000;
pub const GC_BASE__INST0_SEG1: u32 = 0x0000A000;
pub const GC_BASE__INST0_SEG2: u32 = 0x00012160;
pub const GC_BASE__INST0_SEG3: u32 = 0x00402C00;
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
pub const GC_BASE__INST7_SEG0: u32 = 0;
pub const GC_BASE__INST7_SEG1: u32 = 0;
pub const GC_BASE__INST7_SEG2: u32 = 0;
pub const GC_BASE__INST7_SEG3: u32 = 0;
pub const GC_BASE__INST7_SEG4: u32 = 0;
pub const GC_BASE__INST7_SEG5: u32 = 0;
pub const HDP_BASE__INST0_SEG0: u32 = 0x00000F20;
pub const HDP_BASE__INST0_SEG1: u32 = 0x00012520;
pub const HDP_BASE__INST0_SEG2: u32 = 0x0040A400;
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
pub const HDP_BASE__INST7_SEG0: u32 = 0;
pub const HDP_BASE__INST7_SEG1: u32 = 0;
pub const HDP_BASE__INST7_SEG2: u32 = 0;
pub const HDP_BASE__INST7_SEG3: u32 = 0;
pub const HDP_BASE__INST7_SEG4: u32 = 0;
pub const HDP_BASE__INST7_SEG5: u32 = 0;
pub const MMHUB_BASE__INST0_SEG0: u32 = 0x00012440;
pub const MMHUB_BASE__INST0_SEG1: u32 = 0x0001A000;
pub const MMHUB_BASE__INST0_SEG2: u32 = 0x00408800;
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
pub const MMHUB_BASE__INST7_SEG0: u32 = 0;
pub const MMHUB_BASE__INST7_SEG1: u32 = 0;
pub const MMHUB_BASE__INST7_SEG2: u32 = 0;
pub const MMHUB_BASE__INST7_SEG3: u32 = 0;
pub const MMHUB_BASE__INST7_SEG4: u32 = 0;
pub const MMHUB_BASE__INST7_SEG5: u32 = 0;
pub const MP0_BASE__INST0_SEG0: u32 = 0x00013FE0;
pub const MP0_BASE__INST0_SEG1: u32 = 0x00016000;
pub const MP0_BASE__INST0_SEG2: u32 = 0x0043FC00;
pub const MP0_BASE__INST0_SEG3: u32 = 0x00DC0000;
pub const MP0_BASE__INST0_SEG4: u32 = 0x00E00000;
pub const MP0_BASE__INST0_SEG5: u32 = 0x00E40000;
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
pub const MP0_BASE__INST7_SEG0: u32 = 0;
pub const MP0_BASE__INST7_SEG1: u32 = 0;
pub const MP0_BASE__INST7_SEG2: u32 = 0;
pub const MP0_BASE__INST7_SEG3: u32 = 0;
pub const MP0_BASE__INST7_SEG4: u32 = 0;
pub const MP0_BASE__INST7_SEG5: u32 = 0;
pub const MP1_BASE__INST0_SEG0: u32 = 0x00012020;
pub const MP1_BASE__INST0_SEG1: u32 = 0x00016200;
pub const MP1_BASE__INST0_SEG2: u32 = 0x00400400;
pub const MP1_BASE__INST0_SEG3: u32 = 0x00E80000;
pub const MP1_BASE__INST0_SEG4: u32 = 0x00EC0000;
pub const MP1_BASE__INST0_SEG5: u32 = 0x00F00000;
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
pub const MP1_BASE__INST7_SEG0: u32 = 0;
pub const MP1_BASE__INST7_SEG1: u32 = 0;
pub const MP1_BASE__INST7_SEG2: u32 = 0;
pub const MP1_BASE__INST7_SEG3: u32 = 0;
pub const MP1_BASE__INST7_SEG4: u32 = 0;
pub const MP1_BASE__INST7_SEG5: u32 = 0;
pub const NBIF0_BASE__INST0_SEG0: u32 = 0x00000000;
pub const NBIF0_BASE__INST0_SEG1: u32 = 0x00000014;
pub const NBIF0_BASE__INST0_SEG2: u32 = 0x00000D20;
pub const NBIF0_BASE__INST0_SEG3: u32 = 0x00010400;
pub const NBIF0_BASE__INST0_SEG4: u32 = 0x00012D80;
pub const NBIF0_BASE__INST0_SEG5: u32 = 0x0041B000;
pub const NBIF0_BASE__INST1_SEG0: u32 = 0;
pub const NBIF0_BASE__INST1_SEG1: u32 = 0;
pub const NBIF0_BASE__INST1_SEG2: u32 = 0;
pub const NBIF0_BASE__INST1_SEG3: u32 = 0;
pub const NBIF0_BASE__INST1_SEG4: u32 = 0;
pub const NBIF0_BASE__INST1_SEG5: u32 = 0;
pub const NBIF0_BASE__INST2_SEG0: u32 = 0;
pub const NBIF0_BASE__INST2_SEG1: u32 = 0;
pub const NBIF0_BASE__INST2_SEG2: u32 = 0;
pub const NBIF0_BASE__INST2_SEG3: u32 = 0;
pub const NBIF0_BASE__INST2_SEG4: u32 = 0;
pub const NBIF0_BASE__INST2_SEG5: u32 = 0;
pub const NBIF0_BASE__INST3_SEG0: u32 = 0;
pub const NBIF0_BASE__INST3_SEG1: u32 = 0;
pub const NBIF0_BASE__INST3_SEG2: u32 = 0;
pub const NBIF0_BASE__INST3_SEG3: u32 = 0;
pub const NBIF0_BASE__INST3_SEG4: u32 = 0;
pub const NBIF0_BASE__INST3_SEG5: u32 = 0;
pub const NBIF0_BASE__INST4_SEG0: u32 = 0;
pub const NBIF0_BASE__INST4_SEG1: u32 = 0;
pub const NBIF0_BASE__INST4_SEG2: u32 = 0;
pub const NBIF0_BASE__INST4_SEG3: u32 = 0;
pub const NBIF0_BASE__INST4_SEG4: u32 = 0;
pub const NBIF0_BASE__INST4_SEG5: u32 = 0;
pub const NBIF0_BASE__INST5_SEG0: u32 = 0;
pub const NBIF0_BASE__INST5_SEG1: u32 = 0;
pub const NBIF0_BASE__INST5_SEG2: u32 = 0;
pub const NBIF0_BASE__INST5_SEG3: u32 = 0;
pub const NBIF0_BASE__INST5_SEG4: u32 = 0;
pub const NBIF0_BASE__INST5_SEG5: u32 = 0;
pub const NBIF0_BASE__INST6_SEG0: u32 = 0;
pub const NBIF0_BASE__INST6_SEG1: u32 = 0;
pub const NBIF0_BASE__INST6_SEG2: u32 = 0;
pub const NBIF0_BASE__INST6_SEG3: u32 = 0;
pub const NBIF0_BASE__INST6_SEG4: u32 = 0;
pub const NBIF0_BASE__INST6_SEG5: u32 = 0;
pub const NBIF0_BASE__INST7_SEG0: u32 = 0;
pub const NBIF0_BASE__INST7_SEG1: u32 = 0;
pub const NBIF0_BASE__INST7_SEG2: u32 = 0;
pub const NBIF0_BASE__INST7_SEG3: u32 = 0;
pub const NBIF0_BASE__INST7_SEG4: u32 = 0;
pub const NBIF0_BASE__INST7_SEG5: u32 = 0;
pub const OSSSYS_BASE__INST0_SEG0: u32 = 0x000010A0;
pub const OSSSYS_BASE__INST0_SEG1: u32 = 0x00012500;
pub const OSSSYS_BASE__INST0_SEG2: u32 = 0x0040A000;
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
pub const OSSSYS_BASE__INST7_SEG0: u32 = 0;
pub const OSSSYS_BASE__INST7_SEG1: u32 = 0;
pub const OSSSYS_BASE__INST7_SEG2: u32 = 0;
pub const OSSSYS_BASE__INST7_SEG3: u32 = 0;
pub const OSSSYS_BASE__INST7_SEG4: u32 = 0;
pub const OSSSYS_BASE__INST7_SEG5: u32 = 0;
pub const PCIE0_BASE__INST0_SEG0: u32 = 0x000128C0;
pub const PCIE0_BASE__INST0_SEG1: u32 = 0x00411800;
pub const PCIE0_BASE__INST0_SEG2: u32 = 0x04440000;
pub const PCIE0_BASE__INST0_SEG3: u32 = 0;
pub const PCIE0_BASE__INST0_SEG4: u32 = 0;
pub const PCIE0_BASE__INST0_SEG5: u32 = 0;
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
pub const PCIE0_BASE__INST7_SEG0: u32 = 0;
pub const PCIE0_BASE__INST7_SEG1: u32 = 0;
pub const PCIE0_BASE__INST7_SEG2: u32 = 0;
pub const PCIE0_BASE__INST7_SEG3: u32 = 0;
pub const PCIE0_BASE__INST7_SEG4: u32 = 0;
pub const PCIE0_BASE__INST7_SEG5: u32 = 0;
pub const SDMA0_BASE__INST0_SEG0: u32 = 0x00001260;
pub const SDMA0_BASE__INST0_SEG1: u32 = 0x00012540;
pub const SDMA0_BASE__INST0_SEG2: u32 = 0x0040A800;
pub const SDMA0_BASE__INST0_SEG3: u32 = 0;
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
pub const SDMA1_BASE__INST0_SEG0: u32 = 0x00001860;
pub const SDMA1_BASE__INST0_SEG1: u32 = 0x00012560;
pub const SDMA1_BASE__INST0_SEG2: u32 = 0x0040AC00;
pub const SDMA1_BASE__INST0_SEG3: u32 = 0;
pub const SDMA1_BASE__INST0_SEG4: u32 = 0;
pub const SDMA1_BASE__INST0_SEG5: u32 = 0;
pub const SDMA1_BASE__INST1_SEG0: u32 = 0;
pub const SDMA1_BASE__INST1_SEG1: u32 = 0;
pub const SDMA1_BASE__INST1_SEG2: u32 = 0;
pub const SDMA1_BASE__INST1_SEG3: u32 = 0;
pub const SDMA1_BASE__INST1_SEG4: u32 = 0;
pub const SDMA1_BASE__INST1_SEG5: u32 = 0;
pub const SDMA1_BASE__INST2_SEG0: u32 = 0;
pub const SDMA1_BASE__INST2_SEG1: u32 = 0;
pub const SDMA1_BASE__INST2_SEG2: u32 = 0;
pub const SDMA1_BASE__INST2_SEG3: u32 = 0;
pub const SDMA1_BASE__INST2_SEG4: u32 = 0;
pub const SDMA1_BASE__INST2_SEG5: u32 = 0;
pub const SDMA1_BASE__INST3_SEG0: u32 = 0;
pub const SDMA1_BASE__INST3_SEG1: u32 = 0;
pub const SDMA1_BASE__INST3_SEG2: u32 = 0;
pub const SDMA1_BASE__INST3_SEG3: u32 = 0;
pub const SDMA1_BASE__INST3_SEG4: u32 = 0;
pub const SDMA1_BASE__INST3_SEG5: u32 = 0;
pub const SDMA1_BASE__INST4_SEG0: u32 = 0;
pub const SDMA1_BASE__INST4_SEG1: u32 = 0;
pub const SDMA1_BASE__INST4_SEG2: u32 = 0;
pub const SDMA1_BASE__INST4_SEG3: u32 = 0;
pub const SDMA1_BASE__INST4_SEG4: u32 = 0;
pub const SDMA1_BASE__INST4_SEG5: u32 = 0;
pub const SDMA1_BASE__INST5_SEG0: u32 = 0;
pub const SDMA1_BASE__INST5_SEG1: u32 = 0;
pub const SDMA1_BASE__INST5_SEG2: u32 = 0;
pub const SDMA1_BASE__INST5_SEG3: u32 = 0;
pub const SDMA1_BASE__INST5_SEG4: u32 = 0;
pub const SDMA1_BASE__INST5_SEG5: u32 = 0;
pub const SDMA1_BASE__INST6_SEG0: u32 = 0;
pub const SDMA1_BASE__INST6_SEG1: u32 = 0;
pub const SDMA1_BASE__INST6_SEG2: u32 = 0;
pub const SDMA1_BASE__INST6_SEG3: u32 = 0;
pub const SDMA1_BASE__INST6_SEG4: u32 = 0;
pub const SDMA1_BASE__INST6_SEG5: u32 = 0;
pub const SDMA2_BASE__INST0_SEG0: u32 = 0x00013760;
pub const SDMA2_BASE__INST0_SEG1: u32 = 0x0001E000;
pub const SDMA2_BASE__INST0_SEG2: u32 = 0x0042EC00;
pub const SDMA2_BASE__INST0_SEG3: u32 = 0;
pub const SDMA2_BASE__INST0_SEG4: u32 = 0;
pub const SDMA2_BASE__INST0_SEG5: u32 = 0;
pub const SDMA2_BASE__INST1_SEG0: u32 = 0;
pub const SDMA2_BASE__INST1_SEG1: u32 = 0;
pub const SDMA2_BASE__INST1_SEG2: u32 = 0;
pub const SDMA2_BASE__INST1_SEG3: u32 = 0;
pub const SDMA2_BASE__INST1_SEG4: u32 = 0;
pub const SDMA2_BASE__INST1_SEG5: u32 = 0;
pub const SDMA2_BASE__INST2_SEG0: u32 = 0;
pub const SDMA2_BASE__INST2_SEG1: u32 = 0;
pub const SDMA2_BASE__INST2_SEG2: u32 = 0;
pub const SDMA2_BASE__INST2_SEG3: u32 = 0;
pub const SDMA2_BASE__INST2_SEG4: u32 = 0;
pub const SDMA2_BASE__INST2_SEG5: u32 = 0;
pub const SDMA2_BASE__INST3_SEG0: u32 = 0;
pub const SDMA2_BASE__INST3_SEG1: u32 = 0;
pub const SDMA2_BASE__INST3_SEG2: u32 = 0;
pub const SDMA2_BASE__INST3_SEG3: u32 = 0;
pub const SDMA2_BASE__INST3_SEG4: u32 = 0;
pub const SDMA2_BASE__INST3_SEG5: u32 = 0;
pub const SDMA2_BASE__INST4_SEG0: u32 = 0;
pub const SDMA2_BASE__INST4_SEG1: u32 = 0;
pub const SDMA2_BASE__INST4_SEG2: u32 = 0;
pub const SDMA2_BASE__INST4_SEG3: u32 = 0;
pub const SDMA2_BASE__INST4_SEG4: u32 = 0;
pub const SDMA2_BASE__INST4_SEG5: u32 = 0;
pub const SDMA2_BASE__INST5_SEG0: u32 = 0;
pub const SDMA2_BASE__INST5_SEG1: u32 = 0;
pub const SDMA2_BASE__INST5_SEG2: u32 = 0;
pub const SDMA2_BASE__INST5_SEG3: u32 = 0;
pub const SDMA2_BASE__INST5_SEG4: u32 = 0;
pub const SDMA2_BASE__INST5_SEG5: u32 = 0;
pub const SDMA2_BASE__INST6_SEG0: u32 = 0;
pub const SDMA2_BASE__INST6_SEG1: u32 = 0;
pub const SDMA2_BASE__INST6_SEG2: u32 = 0;
pub const SDMA2_BASE__INST6_SEG3: u32 = 0;
pub const SDMA2_BASE__INST6_SEG4: u32 = 0;
pub const SDMA2_BASE__INST6_SEG5: u32 = 0;
pub const SDMA3_BASE__INST0_SEG0: u32 = 0x00013780;
pub const SDMA3_BASE__INST0_SEG1: u32 = 0x0001E400;
pub const SDMA3_BASE__INST0_SEG2: u32 = 0x0042F000;
pub const SDMA3_BASE__INST0_SEG3: u32 = 0;
pub const SDMA3_BASE__INST0_SEG4: u32 = 0;
pub const SDMA3_BASE__INST0_SEG5: u32 = 0;
pub const SDMA3_BASE__INST1_SEG0: u32 = 0;
pub const SDMA3_BASE__INST1_SEG1: u32 = 0;
pub const SDMA3_BASE__INST1_SEG2: u32 = 0;
pub const SDMA3_BASE__INST1_SEG3: u32 = 0;
pub const SDMA3_BASE__INST1_SEG4: u32 = 0;
pub const SDMA3_BASE__INST1_SEG5: u32 = 0;
pub const SDMA3_BASE__INST2_SEG0: u32 = 0;
pub const SDMA3_BASE__INST2_SEG1: u32 = 0;
pub const SDMA3_BASE__INST2_SEG2: u32 = 0;
pub const SDMA3_BASE__INST2_SEG3: u32 = 0;
pub const SDMA3_BASE__INST2_SEG4: u32 = 0;
pub const SDMA3_BASE__INST2_SEG5: u32 = 0;
pub const SDMA3_BASE__INST3_SEG0: u32 = 0;
pub const SDMA3_BASE__INST3_SEG1: u32 = 0;
pub const SDMA3_BASE__INST3_SEG2: u32 = 0;
pub const SDMA3_BASE__INST3_SEG3: u32 = 0;
pub const SDMA3_BASE__INST3_SEG4: u32 = 0;
pub const SDMA3_BASE__INST3_SEG5: u32 = 0;
pub const SDMA3_BASE__INST4_SEG0: u32 = 0;
pub const SDMA3_BASE__INST4_SEG1: u32 = 0;
pub const SDMA3_BASE__INST4_SEG2: u32 = 0;
pub const SDMA3_BASE__INST4_SEG3: u32 = 0;
pub const SDMA3_BASE__INST4_SEG4: u32 = 0;
pub const SDMA3_BASE__INST4_SEG5: u32 = 0;
pub const SDMA3_BASE__INST5_SEG0: u32 = 0;
pub const SDMA3_BASE__INST5_SEG1: u32 = 0;
pub const SDMA3_BASE__INST5_SEG2: u32 = 0;
pub const SDMA3_BASE__INST5_SEG3: u32 = 0;
pub const SDMA3_BASE__INST5_SEG4: u32 = 0;
pub const SDMA3_BASE__INST5_SEG5: u32 = 0;
pub const SDMA3_BASE__INST6_SEG0: u32 = 0;
pub const SDMA3_BASE__INST6_SEG1: u32 = 0;
pub const SDMA3_BASE__INST6_SEG2: u32 = 0;
pub const SDMA3_BASE__INST6_SEG3: u32 = 0;
pub const SDMA3_BASE__INST6_SEG4: u32 = 0;
pub const SDMA3_BASE__INST6_SEG5: u32 = 0;
pub const SDMA4_BASE__INST0_SEG0: u32 = 0x000137A0;
pub const SDMA4_BASE__INST0_SEG1: u32 = 0x0001E800;
pub const SDMA4_BASE__INST0_SEG2: u32 = 0x0042F400;
pub const SDMA4_BASE__INST0_SEG3: u32 = 0;
pub const SDMA4_BASE__INST0_SEG4: u32 = 0;
pub const SDMA4_BASE__INST0_SEG5: u32 = 0;
pub const SDMA4_BASE__INST1_SEG0: u32 = 0;
pub const SDMA4_BASE__INST1_SEG1: u32 = 0;
pub const SDMA4_BASE__INST1_SEG2: u32 = 0;
pub const SDMA4_BASE__INST1_SEG3: u32 = 0;
pub const SDMA4_BASE__INST1_SEG4: u32 = 0;
pub const SDMA4_BASE__INST1_SEG5: u32 = 0;
pub const SDMA4_BASE__INST2_SEG0: u32 = 0;
pub const SDMA4_BASE__INST2_SEG1: u32 = 0;
pub const SDMA4_BASE__INST2_SEG2: u32 = 0;
pub const SDMA4_BASE__INST2_SEG3: u32 = 0;
pub const SDMA4_BASE__INST2_SEG4: u32 = 0;
pub const SDMA4_BASE__INST2_SEG5: u32 = 0;
pub const SDMA4_BASE__INST3_SEG0: u32 = 0;
pub const SDMA4_BASE__INST3_SEG1: u32 = 0;
pub const SDMA4_BASE__INST3_SEG2: u32 = 0;
pub const SDMA4_BASE__INST3_SEG3: u32 = 0;
pub const SDMA4_BASE__INST3_SEG4: u32 = 0;
pub const SDMA4_BASE__INST3_SEG5: u32 = 0;
pub const SDMA4_BASE__INST4_SEG0: u32 = 0;
pub const SDMA4_BASE__INST4_SEG1: u32 = 0;
pub const SDMA4_BASE__INST4_SEG2: u32 = 0;
pub const SDMA4_BASE__INST4_SEG3: u32 = 0;
pub const SDMA4_BASE__INST4_SEG4: u32 = 0;
pub const SDMA4_BASE__INST4_SEG5: u32 = 0;
pub const SDMA4_BASE__INST5_SEG0: u32 = 0;
pub const SDMA4_BASE__INST5_SEG1: u32 = 0;
pub const SDMA4_BASE__INST5_SEG2: u32 = 0;
pub const SDMA4_BASE__INST5_SEG3: u32 = 0;
pub const SDMA4_BASE__INST5_SEG4: u32 = 0;
pub const SDMA4_BASE__INST5_SEG5: u32 = 0;
pub const SDMA4_BASE__INST6_SEG0: u32 = 0;
pub const SDMA4_BASE__INST6_SEG1: u32 = 0;
pub const SDMA4_BASE__INST6_SEG2: u32 = 0;
pub const SDMA4_BASE__INST6_SEG3: u32 = 0;
pub const SDMA4_BASE__INST6_SEG4: u32 = 0;
pub const SDMA4_BASE__INST6_SEG5: u32 = 0;
pub const SDMA5_BASE__INST0_SEG0: u32 = 0x000137C0;
pub const SDMA5_BASE__INST0_SEG1: u32 = 0x0001EC00;
pub const SDMA5_BASE__INST0_SEG2: u32 = 0x0042F800;
pub const SDMA5_BASE__INST0_SEG3: u32 = 0;
pub const SDMA5_BASE__INST0_SEG4: u32 = 0;
pub const SDMA5_BASE__INST0_SEG5: u32 = 0;
pub const SDMA5_BASE__INST1_SEG0: u32 = 0;
pub const SDMA5_BASE__INST1_SEG1: u32 = 0;
pub const SDMA5_BASE__INST1_SEG2: u32 = 0;
pub const SDMA5_BASE__INST1_SEG3: u32 = 0;
pub const SDMA5_BASE__INST1_SEG4: u32 = 0;
pub const SDMA5_BASE__INST1_SEG5: u32 = 0;
pub const SDMA5_BASE__INST2_SEG0: u32 = 0;
pub const SDMA5_BASE__INST2_SEG1: u32 = 0;
pub const SDMA5_BASE__INST2_SEG2: u32 = 0;
pub const SDMA5_BASE__INST2_SEG3: u32 = 0;
pub const SDMA5_BASE__INST2_SEG4: u32 = 0;
pub const SDMA5_BASE__INST2_SEG5: u32 = 0;
pub const SDMA5_BASE__INST3_SEG0: u32 = 0;
pub const SDMA5_BASE__INST3_SEG1: u32 = 0;
pub const SDMA5_BASE__INST3_SEG2: u32 = 0;
pub const SDMA5_BASE__INST3_SEG3: u32 = 0;
pub const SDMA5_BASE__INST3_SEG4: u32 = 0;
pub const SDMA5_BASE__INST3_SEG5: u32 = 0;
pub const SDMA5_BASE__INST4_SEG0: u32 = 0;
pub const SDMA5_BASE__INST4_SEG1: u32 = 0;
pub const SDMA5_BASE__INST4_SEG2: u32 = 0;
pub const SDMA5_BASE__INST4_SEG3: u32 = 0;
pub const SDMA5_BASE__INST4_SEG4: u32 = 0;
pub const SDMA5_BASE__INST4_SEG5: u32 = 0;
pub const SDMA5_BASE__INST5_SEG0: u32 = 0;
pub const SDMA5_BASE__INST5_SEG1: u32 = 0;
pub const SDMA5_BASE__INST5_SEG2: u32 = 0;
pub const SDMA5_BASE__INST5_SEG3: u32 = 0;
pub const SDMA5_BASE__INST5_SEG4: u32 = 0;
pub const SDMA5_BASE__INST5_SEG5: u32 = 0;
pub const SDMA5_BASE__INST6_SEG0: u32 = 0;
pub const SDMA5_BASE__INST6_SEG1: u32 = 0;
pub const SDMA5_BASE__INST6_SEG2: u32 = 0;
pub const SDMA5_BASE__INST6_SEG3: u32 = 0;
pub const SDMA5_BASE__INST6_SEG4: u32 = 0;
pub const SDMA5_BASE__INST6_SEG5: u32 = 0;
pub const SDMA6_BASE__INST0_SEG0: u32 = 0x000137E0;
pub const SDMA6_BASE__INST0_SEG1: u32 = 0x0001F000;
pub const SDMA6_BASE__INST0_SEG2: u32 = 0x0042FC00;
pub const SDMA6_BASE__INST0_SEG3: u32 = 0;
pub const SDMA6_BASE__INST0_SEG4: u32 = 0;
pub const SDMA6_BASE__INST0_SEG5: u32 = 0;
pub const SDMA6_BASE__INST1_SEG0: u32 = 0;
pub const SDMA6_BASE__INST1_SEG1: u32 = 0;
pub const SDMA6_BASE__INST1_SEG2: u32 = 0;
pub const SDMA6_BASE__INST1_SEG3: u32 = 0;
pub const SDMA6_BASE__INST1_SEG4: u32 = 0;
pub const SDMA6_BASE__INST1_SEG5: u32 = 0;
pub const SDMA6_BASE__INST2_SEG0: u32 = 0;
pub const SDMA6_BASE__INST2_SEG1: u32 = 0;
pub const SDMA6_BASE__INST2_SEG2: u32 = 0;
pub const SDMA6_BASE__INST2_SEG3: u32 = 0;
pub const SDMA6_BASE__INST2_SEG4: u32 = 0;
pub const SDMA6_BASE__INST2_SEG5: u32 = 0;
pub const SDMA6_BASE__INST3_SEG0: u32 = 0;
pub const SDMA6_BASE__INST3_SEG1: u32 = 0;
pub const SDMA6_BASE__INST3_SEG2: u32 = 0;
pub const SDMA6_BASE__INST3_SEG3: u32 = 0;
pub const SDMA6_BASE__INST3_SEG4: u32 = 0;
pub const SDMA6_BASE__INST3_SEG5: u32 = 0;
pub const SDMA6_BASE__INST4_SEG0: u32 = 0;
pub const SDMA6_BASE__INST4_SEG1: u32 = 0;
pub const SDMA6_BASE__INST4_SEG2: u32 = 0;
pub const SDMA6_BASE__INST4_SEG3: u32 = 0;
pub const SDMA6_BASE__INST4_SEG4: u32 = 0;
pub const SDMA6_BASE__INST4_SEG5: u32 = 0;
pub const SDMA6_BASE__INST5_SEG0: u32 = 0;
pub const SDMA6_BASE__INST5_SEG1: u32 = 0;
pub const SDMA6_BASE__INST5_SEG2: u32 = 0;
pub const SDMA6_BASE__INST5_SEG3: u32 = 0;
pub const SDMA6_BASE__INST5_SEG4: u32 = 0;
pub const SDMA6_BASE__INST5_SEG5: u32 = 0;
pub const SDMA6_BASE__INST6_SEG0: u32 = 0;
pub const SDMA6_BASE__INST6_SEG1: u32 = 0;
pub const SDMA6_BASE__INST6_SEG2: u32 = 0;
pub const SDMA6_BASE__INST6_SEG3: u32 = 0;
pub const SDMA6_BASE__INST6_SEG4: u32 = 0;
pub const SDMA6_BASE__INST6_SEG5: u32 = 0;
pub const SDMA7_BASE__INST0_SEG0: u32 = 0x00013800;
pub const SDMA7_BASE__INST0_SEG1: u32 = 0x0001F400;
pub const SDMA7_BASE__INST0_SEG2: u32 = 0x00430000;
pub const SDMA7_BASE__INST0_SEG3: u32 = 0;
pub const SDMA7_BASE__INST0_SEG4: u32 = 0;
pub const SDMA7_BASE__INST0_SEG5: u32 = 0;
pub const SDMA7_BASE__INST1_SEG0: u32 = 0;
pub const SDMA7_BASE__INST1_SEG1: u32 = 0;
pub const SDMA7_BASE__INST1_SEG2: u32 = 0;
pub const SDMA7_BASE__INST1_SEG3: u32 = 0;
pub const SDMA7_BASE__INST1_SEG4: u32 = 0;
pub const SDMA7_BASE__INST1_SEG5: u32 = 0;
pub const SDMA7_BASE__INST2_SEG0: u32 = 0;
pub const SDMA7_BASE__INST2_SEG1: u32 = 0;
pub const SDMA7_BASE__INST2_SEG2: u32 = 0;
pub const SDMA7_BASE__INST2_SEG3: u32 = 0;
pub const SDMA7_BASE__INST2_SEG4: u32 = 0;
pub const SDMA7_BASE__INST2_SEG5: u32 = 0;
pub const SDMA7_BASE__INST3_SEG0: u32 = 0;
pub const SDMA7_BASE__INST3_SEG1: u32 = 0;
pub const SDMA7_BASE__INST3_SEG2: u32 = 0;
pub const SDMA7_BASE__INST3_SEG3: u32 = 0;
pub const SDMA7_BASE__INST3_SEG4: u32 = 0;
pub const SDMA7_BASE__INST3_SEG5: u32 = 0;
pub const SDMA7_BASE__INST4_SEG0: u32 = 0;
pub const SDMA7_BASE__INST4_SEG1: u32 = 0;
pub const SDMA7_BASE__INST4_SEG2: u32 = 0;
pub const SDMA7_BASE__INST4_SEG3: u32 = 0;
pub const SDMA7_BASE__INST4_SEG4: u32 = 0;
pub const SDMA7_BASE__INST4_SEG5: u32 = 0;
pub const SDMA7_BASE__INST5_SEG0: u32 = 0;
pub const SDMA7_BASE__INST5_SEG1: u32 = 0;
pub const SDMA7_BASE__INST5_SEG2: u32 = 0;
pub const SDMA7_BASE__INST5_SEG3: u32 = 0;
pub const SDMA7_BASE__INST5_SEG4: u32 = 0;
pub const SDMA7_BASE__INST5_SEG5: u32 = 0;
pub const SDMA7_BASE__INST6_SEG0: u32 = 0;
pub const SDMA7_BASE__INST6_SEG1: u32 = 0;
pub const SDMA7_BASE__INST6_SEG2: u32 = 0;
pub const SDMA7_BASE__INST6_SEG3: u32 = 0;
pub const SDMA7_BASE__INST6_SEG4: u32 = 0;
pub const SDMA7_BASE__INST6_SEG5: u32 = 0;
pub const SMUIO_BASE__INST0_SEG0: u32 = 0x00012080;
pub const SMUIO_BASE__INST0_SEG1: u32 = 0x00016800;
pub const SMUIO_BASE__INST0_SEG2: u32 = 0x00016A00;
pub const SMUIO_BASE__INST0_SEG3: u32 = 0x00401000;
pub const SMUIO_BASE__INST0_SEG4: u32 = 0x00440000;
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
pub const SMUIO_BASE__INST7_SEG0: u32 = 0;
pub const SMUIO_BASE__INST7_SEG1: u32 = 0;
pub const SMUIO_BASE__INST7_SEG2: u32 = 0;
pub const SMUIO_BASE__INST7_SEG3: u32 = 0;
pub const SMUIO_BASE__INST7_SEG4: u32 = 0;
pub const SMUIO_BASE__INST7_SEG5: u32 = 0;
pub const THM_BASE__INST0_SEG0: u32 = 0x00012060;
pub const THM_BASE__INST0_SEG1: u32 = 0x00016600;
pub const THM_BASE__INST0_SEG2: u32 = 0x00400C00;
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
pub const THM_BASE__INST7_SEG0: u32 = 0;
pub const THM_BASE__INST7_SEG1: u32 = 0;
pub const THM_BASE__INST7_SEG2: u32 = 0;
pub const THM_BASE__INST7_SEG3: u32 = 0;
pub const THM_BASE__INST7_SEG4: u32 = 0;
pub const THM_BASE__INST7_SEG5: u32 = 0;
pub const UMC_BASE__INST0_SEG0: u32 = 0x000132C0;
pub const UMC_BASE__INST0_SEG1: u32 = 0x00014000;
pub const UMC_BASE__INST0_SEG2: u32 = 0x00425800;
pub const UMC_BASE__INST0_SEG3: u32 = 0;
pub const UMC_BASE__INST0_SEG4: u32 = 0;
pub const UMC_BASE__INST0_SEG5: u32 = 0;
pub const UMC_BASE__INST1_SEG0: u32 = 0x000132E0;
pub const UMC_BASE__INST1_SEG1: u32 = 0x00054000;
pub const UMC_BASE__INST1_SEG2: u32 = 0x00425C00;
pub const UMC_BASE__INST1_SEG3: u32 = 0;
pub const UMC_BASE__INST1_SEG4: u32 = 0;
pub const UMC_BASE__INST1_SEG5: u32 = 0;
pub const UMC_BASE__INST2_SEG0: u32 = 0x00013300;
pub const UMC_BASE__INST2_SEG1: u32 = 0x00094000;
pub const UMC_BASE__INST2_SEG2: u32 = 0x00426000;
pub const UMC_BASE__INST2_SEG3: u32 = 0;
pub const UMC_BASE__INST2_SEG4: u32 = 0;
pub const UMC_BASE__INST2_SEG5: u32 = 0;
pub const UMC_BASE__INST3_SEG0: u32 = 0x00013320;
pub const UMC_BASE__INST3_SEG1: u32 = 0x000D4000;
pub const UMC_BASE__INST3_SEG2: u32 = 0x00426400;
pub const UMC_BASE__INST3_SEG3: u32 = 0;
pub const UMC_BASE__INST3_SEG4: u32 = 0;
pub const UMC_BASE__INST3_SEG5: u32 = 0;
pub const UMC_BASE__INST4_SEG0: u32 = 0x00013340;
pub const UMC_BASE__INST4_SEG1: u32 = 0x00114000;
pub const UMC_BASE__INST4_SEG2: u32 = 0x00426800;
pub const UMC_BASE__INST4_SEG3: u32 = 0;
pub const UMC_BASE__INST4_SEG4: u32 = 0;
pub const UMC_BASE__INST4_SEG5: u32 = 0;
pub const UMC_BASE__INST5_SEG0: u32 = 0x00013360;
pub const UMC_BASE__INST5_SEG1: u32 = 0x00154000;
pub const UMC_BASE__INST5_SEG2: u32 = 0x00426C00;
pub const UMC_BASE__INST5_SEG3: u32 = 0;
pub const UMC_BASE__INST5_SEG4: u32 = 0;
pub const UMC_BASE__INST5_SEG5: u32 = 0;
pub const UMC_BASE__INST6_SEG0: u32 = 0x00013380;
pub const UMC_BASE__INST6_SEG1: u32 = 0x00194000;
pub const UMC_BASE__INST6_SEG2: u32 = 0x00427000;
pub const UMC_BASE__INST6_SEG3: u32 = 0;
pub const UMC_BASE__INST6_SEG4: u32 = 0;
pub const UMC_BASE__INST6_SEG5: u32 = 0;
pub const UMC_BASE__INST7_SEG0: u32 = 0x000133A0;
pub const UMC_BASE__INST7_SEG1: u32 = 0x001D4000;
pub const UMC_BASE__INST7_SEG2: u32 = 0x00427400;
pub const UMC_BASE__INST7_SEG3: u32 = 0;
pub const UMC_BASE__INST7_SEG4: u32 = 0;
pub const UMC_BASE__INST7_SEG5: u32 = 0;
pub const UVD_BASE__INST0_SEG0: u32 = 0x00007800;
pub const UVD_BASE__INST0_SEG1: u32 = 0x00007E00;
pub const UVD_BASE__INST0_SEG2: u32 = 0x00012180;
pub const UVD_BASE__INST0_SEG3: u32 = 0x00403000;
pub const UVD_BASE__INST0_SEG4: u32 = 0;
pub const UVD_BASE__INST0_SEG5: u32 = 0;
pub const UVD_BASE__INST1_SEG0: u32 = 0x00007A00;
pub const UVD_BASE__INST1_SEG1: u32 = 0x00009000;
pub const UVD_BASE__INST1_SEG2: u32 = 0x000136E0;
pub const UVD_BASE__INST1_SEG3: u32 = 0x0042DC00;
pub const UVD_BASE__INST1_SEG4: u32 = 0;
pub const UVD_BASE__INST1_SEG5: u32 = 0;
pub const UVD_BASE__INST2_SEG0: u32 = 0;
pub const UVD_BASE__INST2_SEG1: u32 = 0;
pub const UVD_BASE__INST2_SEG2: u32 = 0;
pub const UVD_BASE__INST2_SEG3: u32 = 0;
pub const UVD_BASE__INST2_SEG4: u32 = 0;
pub const UVD_BASE__INST2_SEG5: u32 = 0;
pub const UVD_BASE__INST3_SEG0: u32 = 0;
pub const UVD_BASE__INST3_SEG1: u32 = 0;
pub const UVD_BASE__INST3_SEG2: u32 = 0;
pub const UVD_BASE__INST3_SEG3: u32 = 0;
pub const UVD_BASE__INST3_SEG4: u32 = 0;
pub const UVD_BASE__INST3_SEG5: u32 = 0;
pub const UVD_BASE__INST4_SEG0: u32 = 0;
pub const UVD_BASE__INST4_SEG1: u32 = 0;
pub const UVD_BASE__INST4_SEG2: u32 = 0;
pub const UVD_BASE__INST4_SEG3: u32 = 0;
pub const UVD_BASE__INST4_SEG4: u32 = 0;
pub const UVD_BASE__INST4_SEG5: u32 = 0;
pub const UVD_BASE__INST5_SEG0: u32 = 0;
pub const UVD_BASE__INST5_SEG1: u32 = 0;
pub const UVD_BASE__INST5_SEG2: u32 = 0;
pub const UVD_BASE__INST5_SEG3: u32 = 0;
pub const UVD_BASE__INST5_SEG4: u32 = 0;
pub const UVD_BASE__INST5_SEG5: u32 = 0;
pub const UVD_BASE__INST6_SEG0: u32 = 0;
pub const UVD_BASE__INST6_SEG1: u32 = 0;
pub const UVD_BASE__INST6_SEG2: u32 = 0;
pub const UVD_BASE__INST6_SEG3: u32 = 0;
pub const UVD_BASE__INST6_SEG4: u32 = 0;
pub const UVD_BASE__INST6_SEG5: u32 = 0;
pub const UVD_BASE__INST7_SEG0: u32 = 0;
pub const UVD_BASE__INST7_SEG1: u32 = 0;
pub const UVD_BASE__INST7_SEG2: u32 = 0;
pub const UVD_BASE__INST7_SEG3: u32 = 0;
pub const UVD_BASE__INST7_SEG4: u32 = 0;
pub const UVD_BASE__INST7_SEG5: u32 = 0;
pub const DBGU_IO_BASE__INST0_SEG0: u32 = 0x000001E0;
pub const DBGU_IO_BASE__INST0_SEG1: u32 = 0x000125A0;
pub const DBGU_IO_BASE__INST0_SEG2: u32 = 0x0040B400;
pub const DBGU_IO_BASE__INST0_SEG3: u32 = 0;
pub const DBGU_IO_BASE__INST0_SEG4: u32 = 0;
pub const DBGU_IO_BASE__INST0_SEG5: u32 = 0;
pub const DBGU_IO_BASE__INST1_SEG0: u32 = 0;
pub const DBGU_IO_BASE__INST1_SEG1: u32 = 0;
pub const DBGU_IO_BASE__INST1_SEG2: u32 = 0;
pub const DBGU_IO_BASE__INST1_SEG3: u32 = 0;
pub const DBGU_IO_BASE__INST1_SEG4: u32 = 0;
pub const DBGU_IO_BASE__INST1_SEG5: u32 = 0;
pub const DBGU_IO_BASE__INST2_SEG0: u32 = 0;
pub const DBGU_IO_BASE__INST2_SEG1: u32 = 0;
pub const DBGU_IO_BASE__INST2_SEG2: u32 = 0;
pub const DBGU_IO_BASE__INST2_SEG3: u32 = 0;
pub const DBGU_IO_BASE__INST2_SEG4: u32 = 0;
pub const DBGU_IO_BASE__INST2_SEG5: u32 = 0;
pub const DBGU_IO_BASE__INST3_SEG0: u32 = 0;
pub const DBGU_IO_BASE__INST3_SEG1: u32 = 0;
pub const DBGU_IO_BASE__INST3_SEG2: u32 = 0;
pub const DBGU_IO_BASE__INST3_SEG3: u32 = 0;
pub const DBGU_IO_BASE__INST3_SEG4: u32 = 0;
pub const DBGU_IO_BASE__INST3_SEG5: u32 = 0;
pub const DBGU_IO_BASE__INST4_SEG0: u32 = 0;
pub const DBGU_IO_BASE__INST4_SEG1: u32 = 0;
pub const DBGU_IO_BASE__INST4_SEG2: u32 = 0;
pub const DBGU_IO_BASE__INST4_SEG3: u32 = 0;
pub const DBGU_IO_BASE__INST4_SEG4: u32 = 0;
pub const DBGU_IO_BASE__INST4_SEG5: u32 = 0;
pub const DBGU_IO_BASE__INST5_SEG0: u32 = 0;
pub const DBGU_IO_BASE__INST5_SEG1: u32 = 0;
pub const DBGU_IO_BASE__INST5_SEG2: u32 = 0;
pub const DBGU_IO_BASE__INST5_SEG3: u32 = 0;
pub const DBGU_IO_BASE__INST5_SEG4: u32 = 0;
pub const DBGU_IO_BASE__INST5_SEG5: u32 = 0;
pub const DBGU_IO_BASE__INST6_SEG0: u32 = 0;
pub const DBGU_IO_BASE__INST6_SEG1: u32 = 0;
pub const DBGU_IO_BASE__INST6_SEG2: u32 = 0;
pub const DBGU_IO_BASE__INST6_SEG3: u32 = 0;
pub const DBGU_IO_BASE__INST6_SEG4: u32 = 0;
pub const DBGU_IO_BASE__INST6_SEG5: u32 = 0;
pub const DBGU_IO_BASE__INST7_SEG0: u32 = 0;
pub const DBGU_IO_BASE__INST7_SEG1: u32 = 0;
pub const DBGU_IO_BASE__INST7_SEG2: u32 = 0;
pub const DBGU_IO_BASE__INST7_SEG3: u32 = 0;
pub const DBGU_IO_BASE__INST7_SEG4: u32 = 0;
pub const DBGU_IO_BASE__INST7_SEG5: u32 = 0;
pub const RSMU_BASE__INST0_SEG0: u32 = 0x00012000;
pub const RSMU_BASE__INST0_SEG1: u32 = 0;
pub const RSMU_BASE__INST0_SEG2: u32 = 0;
pub const RSMU_BASE__INST0_SEG3: u32 = 0;
pub const RSMU_BASE__INST0_SEG4: u32 = 0;
pub const RSMU_BASE__INST0_SEG5: u32 = 0;
pub const RSMU_BASE__INST1_SEG0: u32 = 0;
pub const RSMU_BASE__INST1_SEG1: u32 = 0;
pub const RSMU_BASE__INST1_SEG2: u32 = 0;
pub const RSMU_BASE__INST1_SEG3: u32 = 0;
pub const RSMU_BASE__INST1_SEG4: u32 = 0;
pub const RSMU_BASE__INST1_SEG5: u32 = 0;
pub const RSMU_BASE__INST2_SEG0: u32 = 0;
pub const RSMU_BASE__INST2_SEG1: u32 = 0;
pub const RSMU_BASE__INST2_SEG2: u32 = 0;
pub const RSMU_BASE__INST2_SEG3: u32 = 0;
pub const RSMU_BASE__INST2_SEG4: u32 = 0;
pub const RSMU_BASE__INST2_SEG5: u32 = 0;
pub const RSMU_BASE__INST3_SEG0: u32 = 0;
pub const RSMU_BASE__INST3_SEG1: u32 = 0;
pub const RSMU_BASE__INST3_SEG2: u32 = 0;
pub const RSMU_BASE__INST3_SEG3: u32 = 0;
pub const RSMU_BASE__INST3_SEG4: u32 = 0;
pub const RSMU_BASE__INST3_SEG5: u32 = 0;
pub const RSMU_BASE__INST4_SEG0: u32 = 0;
pub const RSMU_BASE__INST4_SEG1: u32 = 0;
pub const RSMU_BASE__INST4_SEG2: u32 = 0;
pub const RSMU_BASE__INST4_SEG3: u32 = 0;
pub const RSMU_BASE__INST4_SEG4: u32 = 0;
pub const RSMU_BASE__INST4_SEG5: u32 = 0;
pub const RSMU_BASE__INST5_SEG0: u32 = 0;
pub const RSMU_BASE__INST5_SEG1: u32 = 0;
pub const RSMU_BASE__INST5_SEG2: u32 = 0;
pub const RSMU_BASE__INST5_SEG3: u32 = 0;
pub const RSMU_BASE__INST5_SEG4: u32 = 0;
pub const RSMU_BASE__INST5_SEG5: u32 = 0;
pub const RSMU_BASE__INST6_SEG0: u32 = 0;
pub const RSMU_BASE__INST6_SEG1: u32 = 0;
pub const RSMU_BASE__INST6_SEG2: u32 = 0;
pub const RSMU_BASE__INST6_SEG3: u32 = 0;
pub const RSMU_BASE__INST6_SEG4: u32 = 0;
pub const RSMU_BASE__INST6_SEG5: u32 = 0;
pub const RSMU_BASE__INST7_SEG0: u32 = 0;
pub const RSMU_BASE__INST7_SEG1: u32 = 0;
pub const RSMU_BASE__INST7_SEG2: u32 = 0;
pub const RSMU_BASE__INST7_SEG3: u32 = 0;
pub const RSMU_BASE__INST7_SEG4: u32 = 0;
pub const RSMU_BASE__INST7_SEG5: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
