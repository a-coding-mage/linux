/*
 * AMD64 class Memory Controller kernel module
 *
 * Copyright (c) 2009 SoftwareBitMaker.
 * Copyright (c) 2009-15 Advanced Micro Devices, Inc.
 *
 * This file may be distributed under the terms of the
 * GNU General Public License.
 */

// Linux and local header dependencies are supplied by the surrounding tree.

pub const EDAC_MOD_STR: &str = "amd64_edac";
pub const K8_REV_D: u32 = 1;
pub const K8_REV_E: u32 = 2;
pub const K8_REV_F: u32 = 4;
pub const NUM_CHIPSELECTS: usize = 8;
pub const DRAM_RANGES: usize = 8;
pub const ON: bool = true;
pub const OFF: bool = false;
pub const MAX_CTL_NAMELEN: usize = 19;

pub const PCI_DEVICE_ID_AMD_15H_NB_F1: u32 = 0x1601;
pub const PCI_DEVICE_ID_AMD_15H_NB_F2: u32 = 0x1602;
pub const PCI_DEVICE_ID_AMD_15H_M30H_NB_F1: u32 = 0x141b;
pub const PCI_DEVICE_ID_AMD_15H_M30H_NB_F2: u32 = 0x141c;
pub const PCI_DEVICE_ID_AMD_15H_M60H_NB_F1: u32 = 0x1571;
pub const PCI_DEVICE_ID_AMD_15H_M60H_NB_F2: u32 = 0x1572;
pub const PCI_DEVICE_ID_AMD_16H_NB_F1: u32 = 0x1531;
pub const PCI_DEVICE_ID_AMD_16H_NB_F2: u32 = 0x1532;
pub const PCI_DEVICE_ID_AMD_16H_M30H_NB_F1: u32 = 0x1581;
pub const PCI_DEVICE_ID_AMD_16H_M30H_NB_F2: u32 = 0x1582;

pub const DRAM_BASE_LO: u32 = 0x40; pub const DRAM_LIMIT_LO: u32 = 0x44;
pub const DRAM_CONT_BASE: u32 = 0x200; pub const DRAM_CONT_LIMIT: u32 = 0x204;
pub const DRAM_CONT_HIGH_OFF: u32 = 0x240;
pub const DHAR: u32 = 0xf0; pub const DCT_CFG_SEL: u32 = 0x10c;
pub const DRAM_LOCAL_NODE_BASE: u32 = 0x120; pub const DRAM_LOCAL_NODE_LIM: u32 = 0x124;
pub const DRAM_BASE_HI: u32 = 0x140; pub const DRAM_LIMIT_HI: u32 = 0x144;
pub const DCSB0: u32 = 0x40; pub const DCSB1: u32 = 0x140; pub const DCSB_CS_ENABLE: u32 = 1 << 0;
pub const DCSM0: u32 = 0x60; pub const DCSM1: u32 = 0x160; pub const DRAM_CONTROL: u32 = 0x78;
pub const DBAM0: u32 = 0x80; pub const DBAM1: u32 = 0x180; pub const DBAM_MAX_VALUE: u32 = 11;
pub const DCLR0: u32 = 0x90; pub const DCLR1: u32 = 0x190; pub const REVE_WIDTH_128: u32 = 1 << 16; pub const WIDTH_128: u32 = 1 << 11;
pub const DCHR0: u32 = 0x94; pub const DCHR1: u32 = 0x194; pub const DDR3_MODE: u32 = 1 << 8;
pub const DCT_SEL_LO: u32 = 0x110; pub const SWAP_INTLV_REG: u32 = 0x10c; pub const DCT_SEL_HI: u32 = 0x114; pub const F15H_M60H_SCRCTRL: u32 = 0x1c8;
pub const NBCTL: u32 = 0x40; pub const NBCFG: u32 = 0x44; pub const NBCFG_CHIPKILL: u32 = 1 << 23; pub const NBCFG_ECC_ENABLE: u32 = 1 << 22;
pub const F10_NBSL_EXT_ERR_ECC: u32 = 8; pub const NBSL_PP_OBS: u32 = 2; pub const SCRCTRL: u32 = 0x58; pub const F10_ONLINE_SPARE: u32 = 0xb0;
pub const F10_NB_ARRAY_ADDR: u32 = 0xb8; pub const F10_NB_ARRAY_DRAM: u32 = 1 << 31; pub const F10_NB_ARRAY_DATA: u32 = 0xbc; pub const F10_NB_ARR_ECC_WR_REQ: u32 = 1 << 17;
pub const NBCAP: u32 = 0xe8; pub const NBCAP_CHIPKILL: u32 = 1 << 4; pub const NBCAP_SECDED: u32 = 1 << 3; pub const NBCAP_DCT_DUAL: u32 = 1;
pub const EXT_NB_MCA_CFG: u32 = 0x180; pub const MSR_MCGCTL_NBE: u32 = 1 << 4; pub const DF_DHAR: u32 = 0x104;
pub const UMCCH_BASE_ADDR: u32 = 0; pub const UMCCH_BASE_ADDR_SEC: u32 = 0x10; pub const UMCCH_ADDR_MASK: u32 = 0x20; pub const UMCCH_ADDR_MASK_SEC: u32 = 0x28; pub const UMCCH_ADDR_MASK_SEC_DDR5: u32 = 0x30;
pub const UMCCH_DIMM_CFG: u32 = 0x80; pub const UMCCH_DIMM_CFG_DDR5: u32 = 0x90; pub const UMCCH_UMC_CFG: u32 = 0x100; pub const UMCCH_SDP_CTRL: u32 = 0x104; pub const UMCCH_ECC_CTRL: u32 = 0x14c; pub const UMCCH_UMC_CAP_HI: u32 = 0xdf4;
pub const UMC_ECC_CHIPKILL_CAP: u32 = 1 << 31; pub const UMC_ECC_ENABLED: u32 = 1 << 30; pub const UMC_SDP_INIT: u32 = 1 << 31;

#[repr(C)] pub struct error_injection { pub section: u32, pub word: u32, pub bit_map: u32 }
#[repr(C)] pub struct reg_pair { pub lo: u32, pub hi: u32 }
#[repr(C)] pub struct dram_range { pub base: reg_pair, pub lim: reg_pair }
#[repr(C)] pub struct chip_select { pub csbases: [u32; NUM_CHIPSELECTS], pub csbases_sec: [u32; NUM_CHIPSELECTS], pub b_cnt: u8, pub csmasks: [u32; NUM_CHIPSELECTS], pub csmasks_sec: [u32; NUM_CHIPSELECTS], pub m_cnt: u8 }

pub const DECODE_OK: i32 = 0; pub const ERR_NODE: i32 = -1; pub const ERR_CSROW: i32 = -2; pub const ERR_CHANNEL: i32 = -3; pub const ERR_SYND: i32 = -4; pub const ERR_NORM_ADDR: i32 = -5;

#[inline] pub fn get_umc_base(channel: u8) -> u32 { 0x50000u32.wrapping_add((channel as u32) << 20) }
#[inline] pub fn extract_syndrome(status: u64) -> u16 { (((status >> 47) & 0xff) | ((status >> 16) & 0xff00)) as u16 }

// The following structures use opaque external kernel types supplied by dependencies.
#[repr(C)] pub struct amd64_umc { pub dimm_cfg: u32, pub umc_cfg: u32, pub sdp_ctrl: u32, pub ecc_ctrl: u32, pub umc_cap_hi: u32, pub dram_type: mem_type }
#[repr(C)] pub struct amd64_family_flags { pub zn_regs_v2: u64, pub reserved: u64 }
#[repr(C)] pub struct amd64_pvt {
    pub ops: *mut low_ops, pub F1: *mut pci_dev, pub F2: *mut pci_dev, pub F3: *mut pci_dev,
    pub mc_node_id: u16, pub fam: u8, pub model: u8, pub stepping: u8, pub ext_model: i32,
    pub dclr0: u32, pub dclr1: u32, pub dchr0: u32, pub dchr1: u32, pub nbcap: u32, pub nbcfg: u32,
    pub dhar: u32, pub dbam0: u32, pub dbam1: u32, pub csels: *mut chip_select,
    pub ranges: [dram_range; DRAM_RANGES], pub top_mem: u64, pub top_mem2: u64,
    pub dct_sel_lo: u32, pub dct_sel_hi: u32, pub online_spare: u32, pub gpu_umc_base: u32,
    pub ecc_sym_sz: u8, pub ctl_name: [i8; MAX_CTL_NAMELEN], pub f1_id: u16, pub f2_id: u16,
    pub max_mcs: u8, pub flags: amd64_family_flags, pub injection: error_injection,
    pub dram_type: mem_type, pub umc: *mut amd64_umc,
}
#[repr(i32)] pub enum err_codes { DECODE_OK = 0, ERR_NODE = -1, ERR_CSROW = -2, ERR_CHANNEL = -3, ERR_SYND = -4, ERR_NORM_ADDR = -5 }
#[repr(C)] pub struct err_info { pub err_code: i32, pub src_mci: *mut mem_ctl_info, pub csrow: i32, pub channel: i32, pub syndrome: u16, pub page: u32, pub offset: u32 }
#[repr(C)] pub struct ecc_settings { pub old_nbctl: u32, pub nbctl_valid: bool, pub flags: ecc_setting_flags }
#[repr(C)] pub struct ecc_setting_flags { pub nb_mce_enable: u64, pub nb_ecc_prev: u64 }
#[repr(C)] pub struct low_ops {
    pub map_sysaddr_to_csrow: Option<unsafe extern "C" fn(*mut mem_ctl_info, u64, *mut err_info)>,
    pub dbam_to_cs: Option<unsafe extern "C" fn(*mut amd64_pvt, u8, u32, i32) -> i32>,
    pub hw_info_get: Option<unsafe extern "C" fn(*mut amd64_pvt) -> i32>,
    pub ecc_enabled: Option<unsafe extern "C" fn(*mut amd64_pvt) -> bool>,
    pub setup_mci_misc_attrs: Option<unsafe extern "C" fn(*mut mem_ctl_info)>,
    pub dump_misc_regs: Option<unsafe extern "C" fn(*mut amd64_pvt)>,
    pub get_err_info: Option<unsafe extern "C" fn(*mut mce, *mut err_info)>,
}

extern "C" {
    pub fn __amd64_read_pci_cfg_dword(pdev: *mut pci_dev, offset: i32, val: *mut u32, func: *const i8) -> i32;
    pub fn __amd64_write_pci_cfg_dword(pdev: *mut pci_dev, offset: i32, val: u32, func: *const i8) -> i32;
}

#[inline] pub unsafe fn dram_rw(pvt: *const amd64_pvt, i: usize) -> u8 { ((*pvt).ranges[i].base.lo & 3) as u8 }
#[inline] pub unsafe fn dram_intlv_sel(pvt: *const amd64_pvt, i: usize) -> u8 { (( (*pvt).ranges[i].lim.lo >> 8) & 7) as u8 }
#[inline] pub unsafe fn dram_dst_node(pvt: *const amd64_pvt, i: usize) -> u8 { ((*pvt).ranges[i].lim.lo & 7) as u8 }
#[inline] pub unsafe fn dhar_mem_hoist_valid(pvt: *const amd64_pvt) -> u32 { (*pvt).dhar & (1 << 1) }
#[inline] pub unsafe fn dhar_base(pvt: *const amd64_pvt) -> u32 { (*pvt).dhar & 0xff000000 }
#[inline] pub unsafe fn k8_dhar_offset(pvt: *const amd64_pvt) -> u32 { ((*pvt).dhar & 0x0000ff00) << 16 }
#[inline] pub unsafe fn f10_dhar_offset(pvt: *const amd64_pvt) -> u32 { ((*pvt).dhar & 0x0000ff80) << 16 }
#[inline] pub unsafe fn DBAM_DIMM(i: u32, reg: u32) -> u32 { (reg >> (4 * i)) & 0xf }
#[inline] pub unsafe fn SET_NB_ARRAY_ADDR(section: u32) -> u32 { (section & 3) << 1 }
#[inline] pub unsafe fn online_spare_swap_done(pvt: *const amd64_pvt, c: u32) -> u32 { ((*pvt).online_spare >> (1 + 2*c)) & 1 }
#[inline] pub unsafe fn online_spare_bad_dramcs(pvt: *const amd64_pvt, c: u32) -> u32 { ((*pvt).online_spare >> (4 + 4*c)) & 7 }
#[inline] pub unsafe fn csrow_enabled(i: usize, dct: usize, pvt: *const amd64_pvt) -> u32 { (*(*pvt).csels.add(dct)).csbases[i] & DCSB_CS_ENABLE }
#[inline] pub unsafe fn csrow_sec_enabled(i: usize, dct: usize, pvt: *const amd64_pvt) -> u32 { (*(*pvt).csels.add(dct)).csbases_sec[i] & DCSB_CS_ENABLE }
#[inline] pub unsafe fn dct_high_range_enabled(pvt: *const amd64_pvt) -> u32 { (*pvt).dct_sel_lo & (1 << 0) }
#[inline] pub unsafe fn dct_interleave_enabled(pvt: *const amd64_pvt) -> u32 { (*pvt).dct_sel_lo & (1 << 2) }
#[inline] pub unsafe fn dct_data_intlv_enabled(pvt: *const amd64_pvt) -> u32 { (*pvt).dct_sel_lo & (1 << 5) }
#[inline] pub unsafe fn dct_memory_cleared(pvt: *const amd64_pvt) -> u32 { (*pvt).dct_sel_lo & (1 << 10) }
#[inline] pub unsafe fn get_dram_base(pvt: *const amd64_pvt, i: usize) -> u64 { let addr = (((*pvt).ranges[i].base.lo as u64) & 0xffff0000) << 8; addr }
#[inline] pub unsafe fn get_dram_limit(pvt: *const amd64_pvt, i: usize) -> u64 { (((( *pvt).ranges[i].lim.lo as u64) & 0xffff0000) << 8) | 0x00ffffff }
#[inline] pub unsafe fn dct_sel_interleave_addr(pvt: *const amd64_pvt) -> u8 { (((*pvt).dct_sel_lo >> 6) & 3) as u8 }

// External kernel types referenced by this header.
pub enum pci_dev {} pub enum mem_ctl_info {} pub enum mce {} pub enum mem_type {}
#[repr(C)] pub struct cpuinfo_x86 { pub x86: u8 }
extern "C" { pub static boot_cpu_data: cpuinfo_x86; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
