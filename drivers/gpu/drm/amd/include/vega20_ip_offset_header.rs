/* Translated from vega20_ip_offset.h. */
#[repr(C)]
pub struct IP_BASE_INSTANCE { pub segment: [u32; 6] }
#[repr(C)]
pub struct IP_BASE { pub instance: [IP_BASE_INSTANCE; 6] }
pub const MAX_INSTANCE: usize = 6;
pub const MAX_SEGMENT: usize = 6;
macro_rules! ip_base { ($($x:expr),*) => { IP_BASE { instance: [IP_BASE_INSTANCE { segment: [$($x),*] }, IP_BASE_INSTANCE { segment: [0; 6] }, IP_BASE_INSTANCE { segment: [0; 6] }, IP_BASE_INSTANCE { segment: [0; 6] }, IP_BASE_INSTANCE { segment: [0; 6] }, IP_BASE_INSTANCE { segment: [0; 6] }] } }; }
pub const ATHUB_BASE: IP_BASE = ip_base!(0x00000C20, 0, 0, 0, 0, 0);
pub const CLK_BASE: IP_BASE = ip_base!(0x00016C00, 0x00016E00, 0x00017000, 0x00017200, 0x0001B000, 0x0001B200);
pub const DCE_BASE: IP_BASE = ip_base!(0x00000012, 0x000000C0, 0x000034C0, 0, 0, 0);
pub const DF_BASE: IP_BASE = ip_base!(0x00007000, 0, 0, 0, 0, 0);
pub const FUSE_BASE: IP_BASE = ip_base!(0x00017400, 0, 0, 0, 0, 0);
pub const GC_BASE: IP_BASE = ip_base!(0x00002000, 0x0000A000, 0, 0, 0, 0);
pub const HDP_BASE: IP_BASE = ip_base!(0x00000F20, 0, 0, 0, 0, 0);
pub const MMHUB_BASE: IP_BASE = ip_base!(0x0001A000, 0, 0, 0, 0, 0);
pub const MP0_BASE: IP_BASE = ip_base!(0x00016000, 0, 0, 0, 0, 0);
pub const MP1_BASE: IP_BASE = ip_base!(0x00016000, 0, 0, 0, 0, 0);
pub const NBIO_BASE: IP_BASE = ip_base!(0, 0x00000014, 0x00000D20, 0x00010400, 0, 0);
pub const OSSSYS_BASE: IP_BASE = ip_base!(0x000010A0, 0, 0, 0, 0, 0);
pub const SDMA0_BASE: IP_BASE = ip_base!(0x00001260, 0, 0, 0, 0, 0);
pub const SDMA1_BASE: IP_BASE = ip_base!(0x00001860, 0, 0, 0, 0, 0);
pub const SMUIO_BASE: IP_BASE = ip_base!(0x00016800, 0x00016A00, 0, 0, 0, 0);
pub const THM_BASE: IP_BASE = ip_base!(0x00016600, 0, 0, 0, 0, 0);
pub const UMC_BASE: IP_BASE = ip_base!(0x00014000, 0, 0, 0, 0, 0);
pub const UVD_BASE: IP_BASE = IP_BASE { instance: [IP_BASE_INSTANCE { segment: [0x00007800, 0x00007E00, 0, 0, 0, 0] }, IP_BASE_INSTANCE { segment: [0, 0x00009000, 0, 0, 0, 0] }, IP_BASE_INSTANCE { segment: [0; 6] }, IP_BASE_INSTANCE { segment: [0; 6] }, IP_BASE_INSTANCE { segment: [0; 6] }, IP_BASE_INSTANCE { segment: [0; 6] }] };
pub const VCE_BASE: IP_BASE = ip_base!(0x00007E00, 0, 0, 0, 0, 0);
pub const XDMA_BASE: IP_BASE = ip_base!(0x00003400, 0, 0, 0, 0, 0);
pub const RSMU_BASE: IP_BASE = ip_base!(0x00012000, 0, 0, 0, 0, 0);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
