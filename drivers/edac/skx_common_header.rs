/* SPDX-License-Identifier: GPL-2.0 */
/* Common declarations for SKX and Intel 10nm server EDAC drivers. */

pub const MSG_SIZE: usize = 1024;

#[inline]
pub const fn get_bitfield(v: u64, lo: u32, hi: u32) -> u64 {
    (v & (((1u64 << (hi - lo + 1)) - 1) << lo)) >> lo
}
pub const SKX_NUM_CHANNELS: usize = 3;
pub const SKX_NUM_DIMMS: usize = 2;
pub const I10NM_NUM_DDR_CHANNELS: usize = 2;
pub const I10NM_NUM_DDR_DIMMS: usize = 2;
pub const I10NM_NUM_HBM_CHANNELS: usize = 2;
pub const I10NM_NUM_HBM_DIMMS: usize = 1;
pub const I10NM_NUM_CHANNELS: usize = 2;
pub const I10NM_NUM_DIMMS: usize = 2;
pub const NUM_CHANNELS: usize = 3;
pub const NUM_DIMMS: usize = 2;
pub const MCI_MISC_ECC_MODE: fn(u64) -> u64 = |m| (m >> 59) & 15;
pub const MCI_MISC_ECC_DDRT: u32 = 8;
pub const MCACOD_MEM_ERR_MASK: u32 = 0xef80;
pub const MCACOD_MEM_CTL_ERR: u32 = 0x80;
pub const MCACOD_EXT_MEM_ERR: u32 = 0x280;
pub const NUM_RRL_SET: usize = 4;
pub const NUM_RRL_REG: usize = 7;
pub const NUM_CECNT_REG: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum RrlSourceType { RrlSrcLreScrub, RrlSrcLreDemand, RrlSrcFreScrub, RrlSrcFreDemand }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum RrlCtrlMode { RrlCtrlNone, RrlCtrlBios, RrlCtrlLinux }

#[repr(C)]
pub struct RegRrl {
    pub set_num: i32, pub reg_num: i32,
    pub sources: [RrlSourceType; NUM_RRL_SET],
    pub offsets: [[u32; NUM_RRL_REG]; NUM_RRL_SET],
    pub widths: [u8; NUM_RRL_REG],
    pub v_mask: u32, pub uc_mask: u32, pub over_mask: u32,
    pub en_patspr_mask: u32, pub noover_mask: u32, pub en_mask: u32,
    pub cecnt_num: i32, pub cecnt_offsets: [u32; NUM_CECNT_REG],
    pub cecnt_widths: [u8; NUM_CECNT_REG],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SkxDimm { pub close_pg: u8, pub bank_xor_enable: u8, pub fine_grain_bank: u8, pub rowbits: u8, pub colbits: u8 }
#[repr(C)]
pub struct SkxChannel { pub cdev: *mut pci_dev, pub edev: *mut pci_dev, pub rrl_ctl: [[u32; NUM_RRL_SET]; 2], pub dimms: [SkxDimm; NUM_DIMMS] }
#[repr(C)]
pub struct SkxImc { pub mdev: *mut pci_dev, pub dev: *mut device, pub mci: *mut mem_ctl_info, pub mbase: *mut core::ffi::c_void, pub chan_mmio_sz: i32, pub num_channels: i32, pub num_dimms: i32, pub hbm_mc: bool, pub mc: u8, pub lmc: u8, pub src_id: u8, pub mc_mapping: u8, pub chan: [SkxChannel; NUM_CHANNELS] }
#[repr(C)]
pub struct SkxDev { pub bus: [u8; 4], pub seg: i32, pub sad_all: *mut pci_dev, pub util_all: *mut pci_dev, pub uracu: *mut pci_dev, pub pcu_cr3: *mut pci_dev, pub mcroute: u32, pub mmio_base_h_north: u64, pub mmio_base_h_south: u64, pub pkg: i32, pub num_imc: i32, pub list: list_head, pub imc: [SkxImc; 0] }
#[repr(C)] pub struct SkxPvt { pub imc: *mut SkxImc }

#[repr(C)] #[derive(Copy, Clone)] pub enum Type { Skx, I10nm, Spr, Gnr, Dmr }
#[repr(C)] #[derive(Copy, Clone)] pub enum ErrorSource { ErrSrc1lm, ErrSrc2lmNm, ErrSrc2lmFm, ErrSrcNotMemory }
pub const INDEX_SOCKET: usize = 0; pub const INDEX_MEMCTRL: usize = 1; pub const INDEX_CHANNEL: usize = 2; pub const INDEX_DIMM: usize = 3; pub const INDEX_CS: usize = 4; pub const INDEX_SUBCH: usize = 5; pub const INDEX_NM_FIRST: usize = 6; pub const INDEX_NM_MEMCTRL: usize = 6; pub const INDEX_NM_CHANNEL: usize = 7; pub const INDEX_NM_DIMM: usize = 8; pub const INDEX_NM_CS: usize = 9; pub const INDEX_NM_SUBCH: usize = 10; pub const INDEX_MAX: usize = 11;
pub const BIT_SUBCH: u64 = 1u64 << INDEX_SUBCH; pub const BIT_NM_MEMCTRL: u64 = 1u64 << INDEX_NM_MEMCTRL; pub const BIT_NM_CHANNEL: u64 = 1u64 << INDEX_NM_CHANNEL; pub const BIT_NM_DIMM: u64 = 1u64 << INDEX_NM_DIMM; pub const BIT_NM_CS: u64 = 1u64 << INDEX_NM_CS; pub const BIT_NM_SUBCH: u64 = 1u64 << INDEX_NM_SUBCH;

#[repr(C)] pub struct DecodedAddr { pub mce: *mut mce, pub dev: *mut SkxDev, pub addr: u64, pub socket: i32, pub imc: i32, pub channel: i32, pub chan_addr: u64, pub sktways: i32, pub chanways: i32, pub dimm: i32, pub cs: i32, pub subch: i32, pub rank: i32, pub channel_rank: i32, pub rank_address: u64, pub row: i32, pub column: i32, pub bank_address: i32, pub bank_group: i32, pub decoded_by_adxl: bool }
#[repr(C)] pub struct PciBdf { pub bus: u32, pub dev: u32, pub fun: u32 }

#[repr(C)] pub struct ResConfig { pub type_: Type, pub ddr_imc_num: i32, pub ddr_chan_num: i32, pub ddr_dimm_num: i32, pub ddr_chan_mmio_sz: i32, pub hbm_imc_num: i32, pub hbm_chan_num: i32, pub hbm_dimm_num: i32, pub hbm_chan_mmio_sz: i32, pub support_ddr5: bool, pub reg_rrl_ddr: [*mut RegRrl; 2], pub reg_rrl_hbm: [*mut RegRrl; 2], pub rrl_ctrl_mode: RrlCtrlMode, pub details: ResConfigDetails }
#[repr(C)] pub union ResConfigDetails { pub skx: SkxConfig, pub imh: ImhConfig }
#[repr(C)] pub struct SkxConfig { pub decs_did: u32, pub busno_cfg_offset: i32, pub sad_all_bdf: PciBdf, pub pcu_cr3_bdf: PciBdf, pub util_all_bdf: PciBdf, pub uracu_bdf: PciBdf, pub ddr_mdev_bdf: PciBdf, pub hbm_mdev_bdf: PciBdf, pub sad_all_offset: i32 }
#[repr(C)] pub struct ImhConfig { pub mmio_base_l_north: u64, pub mmio_base_l_south: u64, pub ddr_imc_base: u64, pub ddr_reg_mcmtr_offset: u64, pub ddr_reg_mcmtr_width: u8, pub ddr_reg_dimmmtr_offset: u64, pub ddr_reg_dimmmtr_width: u8, pub ubox_base: u64, pub ubox_size: u32, pub ubox_reg_mmio_base_offset: u32, pub ubox_reg_mmio_base_width: u8, pub ubox_reg_socket_id_offset: u32, pub ubox_reg_socket_id_width: u8, pub pcu_base: u64, pub pcu_size: u32, pub pcu_reg_capid3_offset: u32, pub pcu_reg_capid3_width: u8, pub sca_base: u64, pub sca_size: u32, pub sca_reg_tolm_offset: u32, pub sca_reg_tolm_width: u8, pub sca_reg_tohm_offset: u32, pub sca_reg_tohm_width: u8, pub ha_base: u64, pub ha_size: u32, pub ha_reg_mode_offset: u32, pub ha_reg_mode_width: u8 }

pub type GetDimmConfigF = unsafe extern "C" fn(*mut mem_ctl_info, *mut ResConfig) -> i32;
pub type SkxDecodeF = unsafe extern "C" fn(*mut DecodedAddr) -> bool;
pub type SkxShowRrlF = unsafe extern "C" fn(*mut DecodedAddr, *mut core::ffi::c_char, i32, bool);

extern "C" {
    pub fn skx_readx(addr: *mut core::ffi::c_void, width: u8) -> u64;
    pub fn skx_read_imc_reg(imc: *mut SkxImc, chan: i32, offset: u32, width: u8) -> u64;
    pub fn skx_write_imc_reg(imc: *mut SkxImc, chan: i32, offset: u32, width: u8, val: u64);
    pub fn skx_adxl_get() -> i32; pub fn skx_adxl_put(); pub fn skx_set_decode(decode: SkxDecodeF); pub fn skx_set_show_rrl(rrl: SkxShowRrlF); pub fn skx_show_rrl(res: *mut DecodedAddr, msg: *mut core::ffi::c_char, len: i32, scrub_err: bool); pub fn skx_enable_rrl(enable: bool); pub fn skx_set_mem_cfg(mem_cfg_2lm: bool); pub fn skx_set_res_cfg(cfg: *mut ResConfig); pub fn skx_init_mc_mapping(d: *mut SkxDev); pub fn skx_set_mc_mapping(d: *mut SkxDev, pmc: u8, lmc: u8);
    pub fn skx_get_src_id(d: *mut SkxDev, off: i32, id: *mut u8) -> i32; pub fn skx_get_all_bus_mappings(cfg: *mut ResConfig, list: *mut *mut list_head) -> i32; pub fn skx_get_edac_list() -> *mut list_head; pub fn skx_get_hi_lo(did: u32, off: *mut i32, tolm: *mut u64, tohm: *mut u64) -> i32; pub fn skx_set_hi_lo(tolm: u64, tohm: u64); pub fn skx_remove();
    pub fn skx_get_dimm_info(mtr: u32, mcmtr: u32, amap: u32, dimm: *mut dimm_info, imc: *mut SkxImc, chan: i32, dimmno: i32, cfg: *mut ResConfig) -> i32;
    pub fn skx_get_nvdimm_info(dimm: *mut dimm_info, imc: *mut SkxImc, chan: i32, dimmno: i32, mod_str: *const core::ffi::c_char) -> i32;
    pub fn skx_register_mci(imc: *mut SkxImc, dev: *mut device, dev_name: *const core::ffi::c_char, ctl_name: *const core::ffi::c_char, mod_str: *const core::ffi::c_char, get_dimm_config: GetDimmConfigF, cfg: *mut ResConfig) -> i32;
    pub fn skx_mce_check_error(nb: *mut notifier_block, val: usize, data: *mut core::ffi::c_void) -> i32;
}

#[cfg(feature = "CONFIG_EDAC_DEBUG")]
extern "C" { pub fn skx_setup_debug(name: *const core::ffi::c_char); pub fn skx_teardown_debug(); }
#[cfg(not(feature = "CONFIG_EDAC_DEBUG"))]
#[inline] pub unsafe fn skx_setup_debug(_name: *const core::ffi::c_char) {}
#[cfg(not(feature = "CONFIG_EDAC_DEBUG"))]
#[inline] pub unsafe fn skx_teardown_debug() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
