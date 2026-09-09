/* SPDX-License-Identifier: GPL-2.0-only */
/* OMAP L3 Interconnect error handling driver header. */

use core::ffi::{c_char, c_void};

/* Dependencies supplied by the surrounding translation unit. */
extern "C" {
    pub type device;
}

pub const MAX_L3_MODULES: usize = 3;
pub const MAX_CLKDM_TARGETS: usize = 31;
pub const CLEAR_STDERR_LOG: u32 = 1u32 << 31;
pub const CUSTOM_ERROR: u32 = 0x2;
pub const STANDARD_ERROR: u32 = 0x0;
pub const INBAND_ERROR: u32 = 0x0;
pub const L3_APPLICATION_ERROR: u32 = 0x0;
pub const L3_DEBUG_ERROR: u32 = 0x1;

pub const L3_TARG_STDERRLOG_MAIN: u32 = 0x48;
pub const L3_TARG_STDERRLOG_HDR: u32 = 0x4c;
pub const L3_TARG_STDERRLOG_MSTADDR: u32 = 0x50;
pub const L3_TARG_STDERRLOG_INFO: u32 = 0x58;
pub const L3_TARG_STDERRLOG_SLVOFSLSB: u32 = 0x5c;
pub const L3_TARG_STDERRLOG_CINFO_INFO: u32 = 0x64;
pub const L3_TARG_STDERRLOG_CINFO_MSTADDR: u32 = 0x68;
pub const L3_TARG_STDERRLOG_CINFO_OPCODE: u32 = 0x6c;
pub const L3_FLAGMUX_REGERR0: u32 = 0xc;
pub const L3_FLAGMUX_MASK0: u32 = 0x8;

pub const L3_TARGET_NOT_SUPPORTED: *mut c_char = core::ptr::null_mut();
pub const L3_BASE_IS_SUBMODULE: *mut c_void = 1 as *mut c_void;

pub static L3_TRANSACTION_TYPE: [&[u8]; 8] = [b"Idle\0", b"Write\0", b"Read\0", b"ReadEx\0", b"Read Link\0", b"Write Non-Posted\0", b"Write Conditional\0", b"Write Broadcast\0"];

#[repr(C)]
pub struct l3_masters_data { pub id: u32, pub name: *mut c_char }
#[repr(C)]
pub struct l3_target_data { pub offset: u32, pub name: *mut c_char }
#[repr(C)]
pub struct l3_flagmux_data { pub offset: u32, pub l3_targ: *mut l3_target_data, pub num_targ_data: u8, pub mask_app_bits: u32, pub mask_dbg_bits: u32 }
#[repr(C)]
pub struct omap_l3 {
    pub dev: *mut device,
    pub l3_base: [*mut c_void; MAX_L3_MODULES],
    pub l3_flagmux: *mut *mut l3_flagmux_data,
    pub num_modules: i32,
    pub l3_masters: *mut l3_masters_data,
    pub num_masters: i32,
    pub mst_addr_mask: u32,
    pub debug_irq: i32,
    pub app_irq: i32,
}

macro_rules! targets { ($($off:expr => $name:expr),* $(,)?) => { [$(l3_target_data { offset: $off, name: $name.as_ptr() as *mut c_char }),*] }; }

pub static mut omap_l3_target_data_clk1: [l3_target_data; 7] = targets![0x100=>b"DMM1\0",0x200=>b"DMM2\0",0x300=>b"ABE\0",0x400=>b"L4CFG\0",0x600=>b"CLK2PWRDISC\0",0x0=>b"HOSTCLK1\0",0x900=>b"L4WAKEUP\0"];
pub static mut omap_l3_flagmux_clk1: l3_flagmux_data = l3_flagmux_data { offset: 0x500, l3_targ: omap_l3_target_data_clk1.as_mut_ptr(), num_targ_data: 7, mask_app_bits: 0, mask_dbg_bits: 0 };
pub static mut omap_l3_target_data_clk2: [l3_target_data; 21] = targets![0x500=>b"CORTEXM3\0",0x300=>b"DSS\0",0x100=>b"GPMC\0",0x400=>b"ISS\0",0x700=>b"IVAHD\0",0xD00=>b"AES1\0",0x900=>b"L4PER0\0",0x200=>b"OCMRAM\0",0x100=>b"GPMCsERROR\0",0x600=>b"SGX\0",0x800=>b"SL2\0",0x1600=>b"C2C\0",0x1100=>b"PWRDISCCLK1\0",0xF00=>b"SHA1\0",0xE00=>b"AES2\0",0xC00=>b"L4PER3\0",0xA00=>b"L4PER1\0",0xB00=>b"L4PER2\0",0x0=>b"HOSTCLK2\0",0x1800=>b"CAL\0",0x1700=>b"LLI\0"];
pub static mut omap_l3_flagmux_clk2: l3_flagmux_data = l3_flagmux_data { offset: 0x1000, l3_targ: omap_l3_target_data_clk2.as_mut_ptr(), num_targ_data: 21, mask_app_bits: 0, mask_dbg_bits: 0 };
pub static mut omap4_l3_target_data_clk3: [l3_target_data; 1] = targets![0x0100=>b"DEBUGSS\0"];
pub static mut omap4_l3_flagmux_clk3: l3_flagmux_data = l3_flagmux_data { offset: 0x0200, l3_targ: omap4_l3_target_data_clk3.as_mut_ptr(), num_targ_data: 1, mask_app_bits: 0, mask_dbg_bits: 0 };

pub static mut omap_l3_masters: [l3_masters_data; 25] = [
    (0x00,b"MPU\0"),(0x04,b"CS_ADP\0"),(0x05,b"xxx\0"),(0x08,b"DSP\0"),(0x0C,b"IVAHD\0"),(0x10,b"ISS\0"),(0x11,b"DucatiM3\0"),(0x12,b"FaceDetect\0"),(0x14,b"SDMA_Rd\0"),(0x15,b"SDMA_Wr\0"),(0x16,b"xxx\0"),(0x17,b"xxx\0"),(0x18,b"SGX\0"),(0x1C,b"DSS\0"),(0x20,b"C2C\0"),(0x22,b"xxx\0"),(0x23,b"xxx\0"),(0x24,b"HSI\0"),(0x28,b"MMC1\0"),(0x29,b"MMC2\0"),(0x2A,b"MMC6\0"),(0x2C,b"UNIPRO1\0"),(0x30,b"USBHOSTHS\0"),(0x31,b"USBOTGHS\0"),(0x32,b"USBHOSTFS\0")
].map(|(id,n)| l3_masters_data{id,name:n.as_ptr() as *mut c_char});

pub static mut omap4_l3_flagmux: [*mut l3_flagmux_data; 3] = [core::ptr::addr_of_mut!(omap_l3_flagmux_clk1),core::ptr::addr_of_mut!(omap_l3_flagmux_clk2),core::ptr::addr_of_mut!(omap4_l3_flagmux_clk3)];
pub static omap4_l3_data: omap_l3 = omap_l3 { dev: core::ptr::null_mut(), l3_base: [core::ptr::null_mut();3], l3_flagmux: omap4_l3_flagmux.as_ptr() as *mut _, num_modules: 3, l3_masters: omap_l3_masters.as_ptr() as *mut _, num_masters: 25, mst_addr_mask: 0xFC, debug_irq: 0, app_irq: 0 };

macro_rules! masters { ($($id:expr=>$name:expr),* $(,)?) => { [$(l3_masters_data{id:$id,name:$name.as_ptr() as *mut c_char}),*] }; }
pub static mut omap5_l3_target_data_clk3:[l3_target_data;3]=targets![0x100=>b"L3INSTR\0",0x300=>b"DEBUGSS\0",0=>b"HOSTCLK3\0"];
pub static mut omap5_l3_flagmux_clk3:l3_flagmux_data=l3_flagmux_data{offset:0x200,l3_targ:omap5_l3_target_data_clk3.as_mut_ptr(),num_targ_data:3,mask_app_bits:0,mask_dbg_bits:0};
pub static mut omap5_l3_flagmux:[*mut l3_flagmux_data;3]=[core::ptr::addr_of_mut!(omap_l3_flagmux_clk1),core::ptr::addr_of_mut!(omap_l3_flagmux_clk2),core::ptr::addr_of_mut!(omap5_l3_flagmux_clk3)];
pub static omap5_l3_data:omap_l3=omap_l3{dev:core::ptr::null_mut(),l3_base:[core::ptr::null_mut();3],l3_flagmux:omap5_l3_flagmux.as_ptr() as *mut _,num_modules:3,l3_masters:omap_l3_masters.as_ptr() as *mut _,num_masters:25,mst_addr_mask:0x7E0,debug_irq:0,app_irq:0};

pub static mut dra_l3_target_data_clk1:[l3_target_data;32]=targets![0x2a00=>b"AES1\0",0x0200=>b"DMM_P1\0",0x0600=>b"DSP2_SDMA\0",0x0b00=>b"EVE2\0",0x1300=>b"DMM_P2\0",0x2c00=>b"AES2\0",0x0300=>b"DSP1_SDMA\0",0x0a00=>b"EVE1\0",0x0c00=>b"EVE3\0",0x0d00=>b"EVE4\0",0x2900=>b"DSS\0",0x0100=>b"GPMC\0",0x3700=>b"PCIE1\0",0x1600=>b"IVA_CONFIG\0",0x1800=>b"IVA_SL2IF\0",0x0500=>b"L4_CFG\0",0x1d00=>b"L4_WKUP\0",0x3800=>b"PCIE2\0",0x3300=>b"SHA2_1\0",0x1200=>b"GPU\0",0x1000=>b"IPU1\0",0x1100=>b"IPU2\0",0x2000=>b"TPCC_EDMA\0",0x2e00=>b"TPTC1_EDMA\0",0x2b00=>b"TPTC2_EDMA\0",0x0700=>b"VCP1\0",0x2500=>b"L4_PER2_P3\0",0x0e00=>b"L4_PER3_P3\0",0x2200=>b"MMU1\0",0x1400=>b"PRUSS1\0",0x1500=>b"PRUSS2\0",0x0800=>b"VCP1\0"];
pub static mut dra_l3_flagmux_clk1:l3_flagmux_data=l3_flagmux_data{offset:0x803500,l3_targ:dra_l3_target_data_clk1.as_mut_ptr(),num_targ_data:32,mask_app_bits:0,mask_dbg_bits:0};
pub static mut dra_l3_target_data_clk2:[l3_target_data;21]=targets![0=>b"HOST CLK1\0",0x800000=>b"HOST CLK2\0",0xdead=>b"\0",0x3400=>b"SHA2_2\0",0x0900=>b"BB2D\0",0xdead=>b"\0",0x2100=>b"L4_PER1_P3\0",0x1c00=>b"L4_PER1_P1\0",0x1f00=>b"L4_PER1_P2\0",0x2300=>b"L4_PER2_P1\0",0x2400=>b"L4_PER2_P2\0",0x2600=>b"L4_PER3_P1\0",0x2700=>b"L4_PER3_P2\0",0x2f00=>b"MCASP1\0",0x3000=>b"MCASP2\0",0x3100=>b"MCASP3\0",0x2800=>b"MMU2\0",0x0f00=>b"OCMC_RAM1\0",0x1700=>b"OCMC_RAM2\0",0x1900=>b"OCMC_RAM3\0",0x1e00=>b"OCMC_ROM\0"];
pub static mut dra_l3_flagmux_clk2:l3_flagmux_data=l3_flagmux_data{offset:0x803600,l3_targ:dra_l3_target_data_clk2.as_mut_ptr(),num_targ_data:21,mask_app_bits:0,mask_dbg_bits:0};
pub static mut dra_l3_target_data_clk3:[l3_target_data;3]=targets![0x100=>b"L3_INSTR\0",0x300=>b"DEBUGSS_CT_TBR\0",0=>b"HOST CLK3\0"];
pub static mut dra_l3_flagmux_clk3:l3_flagmux_data=l3_flagmux_data{offset:0x200,l3_targ:dra_l3_target_data_clk3.as_mut_ptr(),num_targ_data:3,mask_app_bits:0,mask_dbg_bits:0};
pub static mut dra_l3_masters:[l3_masters_data;47]=masters![0=>b"MPU\0",4=>b"CS_DAP\0",5=>b"IEEE1500_2_OCP\0",8=>b"DSP1_MDMA\0",9=>b"DSP1_CFG\0",10=>b"DSP1_DMA\0",11=>b"DSP2_MDMA\0",12=>b"DSP2_CFG\0",13=>b"DSP2_DMA\0",14=>b"IVA\0",0x10=>b"EVE1_P1\0",0x11=>b"EVE2_P1\0",0x12=>b"EVE3_P1\0",0x13=>b"EVE4_P1\0",0x14=>b"PRUSS1 PRU1\0",0x15=>b"PRUSS1 PRU2\0",0x16=>b"PRUSS2 PRU1\0",0x17=>b"PRUSS2 PRU2\0",0x18=>b"IPU1\0",0x19=>b"IPU2\0",0x1A=>b"SDMA\0",0x1B=>b"CDMA\0",0x1C=>b"TC1_EDMA\0",0x1D=>b"TC2_EDMA\0",0x20=>b"DSS\0",0x21=>b"MMU1\0",0x22=>b"PCIE1\0",0x23=>b"MMU2\0",0x24=>b"VIP1\0",0x25=>b"VIP2\0",0x26=>b"VIP3\0",0x27=>b"VPE\0",0x28=>b"GPU_P1\0",0x29=>b"BB2D\0",0x29=>b"GPU_P2\0",0x2B=>b"GMAC_SW\0",0x2C=>b"USB3\0",0x2D=>b"USB2_SS\0",0x2E=>b"USB2_ULPI_SS1\0",0x2F=>b"USB2_ULPI_SS2\0",0x30=>b"CSI2_1\0",0x31=>b"CSI2_2\0",0x33=>b"SATA\0",0x34=>b"EVE1_P2\0",0x35=>b"EVE2_P2\0",0x36=>b"EVE3_P2\0",0x37=>b"EVE4_P2\0"];
pub static mut dra_l3_flagmux:[*mut l3_flagmux_data;3]=[core::ptr::addr_of_mut!(dra_l3_flagmux_clk1),core::ptr::addr_of_mut!(dra_l3_flagmux_clk2),core::ptr::addr_of_mut!(dra_l3_flagmux_clk3)];
pub static dra_l3_data:omap_l3=omap_l3{dev:core::ptr::null_mut(),l3_base:[core::ptr::null_mut(),L3_BASE_IS_SUBMODULE,core::ptr::null_mut()],l3_flagmux:dra_l3_flagmux.as_ptr() as *mut _,num_modules:3,l3_masters:dra_l3_masters.as_ptr() as *mut _,num_masters:47,mst_addr_mask:0xFC,debug_irq:0,app_irq:0};
pub static mut am4372_l3_target_data_200f:[l3_target_data;15]=targets![0xf00=>b"EMIF\0",0x1200=>b"DES\0",0x400=>b"OCMCRAM\0",0x700=>b"TPTC0\0",0x800=>b"TPTC1\0",0x900=>b"TPTC2\0",0xb00=>b"TPCC\0",0xd00=>b"DEBUGSS\0",0xdead=>b"\0",0x200=>b"SHA\0",0xc00=>b"SGX530\0",0x500=>b"AES0\0",0xa00=>b"L4_FAST\0",0x300=>b"MPUSS_L2_RAM\0",0x100=>b"ICSS\0"];
pub static mut am4372_l3_flagmux_200f:l3_flagmux_data=l3_flagmux_data{offset:0x1000,l3_targ:am4372_l3_target_data_200f.as_mut_ptr(),num_targ_data:15,mask_app_bits:0,mask_dbg_bits:0};
pub static mut am4372_l3_target_data_100s:[l3_target_data;13]=targets![0x100=>b"L4_PER_0\0",0x200=>b"L4_PER_1\0",0x300=>b"L4_PER_2\0",0x400=>b"L4_PER_3\0",0x800=>b"McASP0\0",0x900=>b"McASP1\0",0xC00=>b"MMCHS2\0",0x700=>b"GPMC\0",0xD00=>b"L4_FW\0",0xdead=>b"\0",0x500=>b"ADCTSC\0",0xE00=>b"L4_WKUP\0",0xA00=>b"MAG_CARD\0"];
pub static mut am4372_l3_flagmux_100s:l3_flagmux_data=l3_flagmux_data{offset:0x600,l3_targ:am4372_l3_target_data_100s.as_mut_ptr(),num_targ_data:13,mask_app_bits:0,mask_dbg_bits:0};
pub static mut am4372_l3_flagmux:[*mut l3_flagmux_data;2]=[core::ptr::addr_of_mut!(am4372_l3_flagmux_200f),core::ptr::addr_of_mut!(am4372_l3_flagmux_100s)];
pub static mut am4372_l3_masters:[l3_masters_data;28]=masters![0=>b"M1 (128-bit)\0",1=>b"M2 (64-bit)\0",4=>b"DAP\0",5=>b"P1500\0",0xC=>b"ICSS0\0",0xD=>b"ICSS1\0",0x14=>b"Wakeup Processor\0",0x18=>b"TPTC0 Read\0",0x19=>b"TPTC0 Write\0",0x1A=>b"TPTC1 Read\0",0x1B=>b"TPTC1 Write\0",0x1C=>b"TPTC2 Read\0",0x1D=>b"TPTC2 Write\0",0x20=>b"SGX530\0",0x21=>b"OCP WP Traffic Probe\0",0x22=>b"OCP WP DMA Profiling\0",0x23=>b"OCP WP Event Trace\0",0x25=>b"DSS\0",0x28=>b"Crypto DMA RD\0",0x29=>b"Crypto DMA WR\0",0x2C=>b"VPFE0\0",0x2D=>b"VPFE1\0",0x30=>b"GEMAC\0",0x34=>b"USB0 RD\0",0x35=>b"USB0 WR\0",0x36=>b"USB1 RD\0",0x37=>b"USB1 WR\0"];
pub static am4372_l3_data:omap_l3=omap_l3{dev:core::ptr::null_mut(),l3_base:[core::ptr::null_mut();3],l3_flagmux:am4372_l3_flagmux.as_ptr() as *mut _,num_modules:2,l3_masters:am4372_l3_masters.as_ptr() as *mut _,num_masters:28,mst_addr_mask:0x3F,debug_irq:0,app_irq:0};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
