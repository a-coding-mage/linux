/* SPDX-License-Identifier: GPL-2.0-only */
/* Translation of pxa2xx_ssp.h. */

use core::ffi::{c_char, c_int, c_ulong, c_void};

/* External kernel types and operations supplied by other files. */
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }

extern "C" {
    fn __raw_writel(value: u32, addr: *mut c_void);
    fn __raw_readl(addr: *const c_void) -> u32;
}

macro_rules! bit { ($n:expr) => { 1u32 << ($n) }; }
macro_rules! genmask { ($h:expr, $l:expr) => { (((1u32 << (($h) - ($l) + 1)) - 1) << ($l)) }; }
macro_rules! SSCR0_DataSize { ($x:expr) => { ($x) - 1 }; }
macro_rules! SSCR0_SCR { ($x:expr) => { ($x) << 8 }; }
macro_rules! SSCR0_SlotsPerFrm { ($x:expr) => { (($x) - 1) << 24 }; }
macro_rules! SSCR1_TxTresh { ($x:expr) => { (($x) - 1) << 6 }; }
macro_rules! SSCR1_RxTresh { ($x:expr) => { (($x) - 1) << 10 }; }
macro_rules! CE4100_SSCR1_TxTresh { ($x:expr) => { (($x) - 1) << 6 }; }
macro_rules! CE4100_SSCR1_RxTresh { ($x:expr) => { (($x) - 1) << 10 }; }
macro_rules! QUARK_X1000_SSCR0_DataSize { ($x:expr) => { ($x) - 1 }; }
macro_rules! QUARK_X1000_SSCR1_TxTresh { ($x:expr) => { (($x) - 1) << 6 }; }
macro_rules! QUARK_X1000_SSCR1_RxTresh { ($x:expr) => { (($x) - 1) << 11 }; }
macro_rules! SSPSP_SCMODE { ($x:expr) => { ($x) << 0 }; }
macro_rules! SSPSP_STRTDLY { ($x:expr) => { ($x) << 4 }; }
macro_rules! SSPSP_DMYSTRT { ($x:expr) => { ($x) << 7 }; }
macro_rules! SSPSP_SFRMDLY { ($x:expr) => { ($x) << 9 }; }
macro_rules! SSPSP_SFRMWDTH { ($x:expr) => { ($x) << 16 }; }
macro_rules! SSPSP_DMYSTOP { ($x:expr) => { ($x) << 23 }; }
macro_rules! SSPSP_EDMYSTRT { ($x:expr) => { ($x) << 26 }; }
macro_rules! SSPSP_EDMYSTOP { ($x:expr) => { ($x) << 28 }; }
macro_rules! SSACD_ACDS { ($x:expr) => { ($x) << 0 }; }
macro_rules! SSACD_ACPS { ($x:expr) => { ($x) << 4 }; }
macro_rules! SFIFOTT_TxThresh { ($x:expr) => { (($x) - 1) << 0 }; }
macro_rules! SFIFOTT_RxThresh { ($x:expr) => { (($x) - 1) << 16 }; }
macro_rules! SSITF_TxHiThresh { ($x:expr) => { (($x) - 1) << 0 }; }
macro_rules! SSITF_TxLoThresh { ($x:expr) => { (($x) - 1) << 8 }; }
macro_rules! SSIRF_RxThresh { ($x:expr) => { ($x) - 1 }; }

pub const SSCR0: u32 = 0x00; pub const SSCR1: u32 = 0x04; pub const SSSR: u32 = 0x08;
pub const SSITR: u32 = 0x0C; pub const SSDR: u32 = 0x10; pub const SSTO: u32 = 0x28;
pub const SSPSP: u32 = 0x2C; pub const SSTSA: u32 = 0x30; pub const SSRSA: u32 = 0x34;
pub const SSTSS: u32 = 0x38; pub const SSACD: u32 = 0x3C; pub const SSACDD: u32 = 0x40;
pub const SSCR0_DSS: u32 = genmask!(3, 0); pub const SSCR0_FRF: u32 = genmask!(5, 4);
pub const SSCR0_Motorola: u32 = 0x0 << 4; pub const SSCR0_TI: u32 = 0x1 << 4; pub const SSCR0_National: u32 = 0x2 << 4;
pub const SSCR0_ECS: u32 = bit!(6); pub const SSCR0_SSE: u32 = bit!(7); pub const SSCR0_EDSS: u32 = bit!(20);
pub const SSCR0_NCS: u32 = bit!(21); pub const SSCR0_RIM: u32 = bit!(22); pub const SSCR0_TUM: u32 = bit!(23);
pub const SSCR0_FRDC: u32 = genmask!(26,24); pub const SSCR0_FPCKE: u32 = bit!(29); pub const SSCR0_ACS: u32 = bit!(30); pub const SSCR0_MOD: u32 = bit!(31);
pub const SSCR1_RIE:u32=bit!(0); pub const SSCR1_TIE:u32=bit!(1); pub const SSCR1_LBM:u32=bit!(2); pub const SSCR1_SPO:u32=bit!(3); pub const SSCR1_SPH:u32=bit!(4); pub const SSCR1_MWDS:u32=bit!(5);
pub const SSSR_ALT_FRM_MASK:u32=genmask!(1,0); pub const SSSR_TNF:u32=bit!(2); pub const SSSR_RNE:u32=bit!(3); pub const SSSR_BSY:u32=bit!(4); pub const SSSR_TFS:u32=bit!(5); pub const SSSR_RFS:u32=bit!(6); pub const SSSR_ROR:u32=bit!(7);
pub const RX_THRESH_DFLT:u32=8; pub const TX_THRESH_DFLT:u32=8; pub const SSSR_TFL_MASK:u32=genmask!(11,8); pub const SSSR_RFL_MASK:u32=genmask!(15,12);
pub const SSCR1_TFT:u32=genmask!(9,6); pub const SSCR1_RFT:u32=genmask!(13,10); pub const RX_THRESH_CE4100_DFLT:u32=2; pub const TX_THRESH_CE4100_DFLT:u32=2;
pub const CE4100_SSSR_TFL_MASK:u32=genmask!(9,8); pub const CE4100_SSSR_RFL_MASK:u32=genmask!(13,12); pub const CE4100_SSCR1_TFT:u32=genmask!(7,6); pub const CE4100_SSCR1_RFT:u32=genmask!(11,10);
pub const DDS_RATE:u32=0x28; pub const QUARK_X1000_SSCR0_DSS:u32=genmask!(4,0); pub const QUARK_X1000_SSCR0_FRF:u32=genmask!(6,5); pub const QUARK_X1000_SSCR0_Motorola:u32=0;
pub const RX_THRESH_QUARK_X1000_DFLT:u32=1; pub const TX_THRESH_QUARK_X1000_DFLT:u32=16; pub const QUARK_X1000_SSSR_TFL_MASK:u32=genmask!(12,8); pub const QUARK_X1000_SSSR_RFL_MASK:u32=genmask!(17,13); pub const QUARK_X1000_SSCR1_TFT:u32=genmask!(10,6); pub const QUARK_X1000_SSCR1_RFT:u32=genmask!(15,11); pub const QUARK_X1000_SSCR1_EFWR:u32=bit!(16); pub const QUARK_X1000_SSCR1_STRF:u32=bit!(17);
pub const SSCR0_TISSP:u32=1<<4; pub const SSCR0_PSP:u32=3<<4; pub const SSCR1_EFWR:u32=bit!(14); pub const SSCR1_STRF:u32=bit!(15); pub const SSCR1_IFS:u32=bit!(16); pub const SSCR1_PINTE:u32=bit!(18); pub const SSCR1_TINTE:u32=bit!(19); pub const SSCR1_RSRE:u32=bit!(20); pub const SSCR1_TSRE:u32=bit!(21); pub const SSCR1_TRAIL:u32=bit!(22); pub const SSCR1_RWOT:u32=bit!(23); pub const SSCR1_SFRMDIR:u32=bit!(24); pub const SSCR1_SCLKDIR:u32=bit!(25); pub const SSCR1_ECRB:u32=bit!(26); pub const SSCR1_ECRA:u32=bit!(27); pub const SSCR1_SCFR:u32=bit!(28); pub const SSCR1_EBCEI:u32=bit!(29); pub const SSCR1_TTE:u32=bit!(30); pub const SSCR1_TTELP:u32=bit!(31);
pub const SSSR_PINT:u32=bit!(18); pub const SSSR_TINT:u32=bit!(19); pub const SSSR_EOC:u32=bit!(20); pub const SSSR_TUR:u32=bit!(21); pub const SSSR_CSS:u32=bit!(22); pub const SSSR_BCE:u32=bit!(23);
pub const SSPSP_SFRMP:u32=bit!(2); pub const SSPSP_ETDS:u32=bit!(3); pub const SSPSP_FSRT:u32=bit!(25); pub const SSPSP_TIMING_MASK:u32=0x7f8001f0; pub const SSACD_ACDS_1:u32=0; pub const SSACD_ACDS_2:u32=1; pub const SSACD_ACDS_4:u32=2; pub const SSACD_ACDS_8:u32=3; pub const SSACD_ACDS_16:u32=4; pub const SSACD_ACDS_32:u32=5; pub const SSACD_SCDB:u32=bit!(3); pub const SSACD_SCDB_4X:u32=0; pub const SSACD_SCDB_1X:u32=1; pub const SSACD_SCDX8:u32=bit!(7);
pub const SFIFOL:u32=0x68; pub const SFIFOTT:u32=0x6c; pub const RX_THRESH_MRFLD_DFLT:u32=16; pub const TX_THRESH_MRFLD_DFLT:u32=16; pub const SFIFOL_TFL_MASK:u32=genmask!(15,0); pub const SFIFOL_RFL_MASK:u32=genmask!(31,16); pub const SFIFOTT_TFT:u32=genmask!(15,0); pub const SFIFOTT_RFT:u32=genmask!(31,16); pub const SSITF:u32=0x44; pub const SSIRF:u32=0x48; pub const SSCR2:u32=0x40; pub const SSPSP2:u32=0x44;

#[repr(C)]
pub struct ssp_device { pub dev:*mut device, pub node:list_head, pub clk:*mut clk, pub mmio_base:*mut c_void, pub phys_base:c_ulong, pub label:*const c_char, pub port_id:c_int, pub r#type:pxa_ssp_type, pub use_count:c_int, pub irq:c_int, pub of_node:*mut device_node }
#[repr(C)] #[derive(Copy,Clone)] pub enum pxa_ssp_type { SSP_UNDEFINED=0, PXA25x_SSP, PXA25x_NSSP, PXA27x_SSP, PXA3xx_SSP, PXA168_SSP, PXA910_SSP, CE4100_SSP, MMP2_SSP, MRFLD_SSP, QUARK_X1000_SSP, LPSS_LPT_SSP, LPSS_BYT_SSP, LPSS_BSW_SSP, LPSS_SPT_SSP, LPSS_BXT_SSP, LPSS_CNL_SSP, SSP_MAX }

pub unsafe fn pxa_ssp_write_reg(dev:*mut ssp_device, reg:u32, val:u32) { __raw_writel(val, (*dev).mmio_base.add(reg as usize)); }
pub unsafe fn pxa_ssp_read_reg(dev:*mut ssp_device, reg:u32) -> u32 { __raw_readl((*dev).mmio_base.add(reg as usize)) }
pub unsafe fn pxa_ssp_enable(ssp:*mut ssp_device) { let v=pxa_ssp_read_reg(ssp,SSCR0)|SSCR0_SSE; pxa_ssp_write_reg(ssp,SSCR0,v); }
pub unsafe fn pxa_ssp_disable(ssp:*mut ssp_device) { let v=pxa_ssp_read_reg(ssp,SSCR0)&!SSCR0_SSE; pxa_ssp_write_reg(ssp,SSCR0,v); }

extern "C" { pub fn pxa_ssp_request(port:c_int, label:*const c_char)->*mut ssp_device; pub fn pxa_ssp_free(ssp:*mut ssp_device); pub fn pxa_ssp_request_of(of_node:*const device_node, label:*const c_char)->*mut ssp_device; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
