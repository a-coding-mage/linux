/* SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause) */
/* Copyright(c) 2015-17 Intel Corporation. */

/* Translated from sdw_intel.h. External kernel types are supplied by dependencies. */

const fn bit(n: u32) -> u32 { 1u32 << n }
const fn genmask(h: u32, l: u32) -> u32 { ((1u32 << (h - l + 1)) - 1) << l }

pub const SDW_SHIM_BASE: u32 = 0x2c000;
pub const SDW_ALH_BASE: u32 = 0x2c800;
pub const SDW_SHIM_BASE_ACE: u32 = 0x38000;
pub const SDW_ALH_BASE_ACE: u32 = 0x24000;
pub const SDW_LINK_BASE: u32 = 0x30000;
pub const SDW_LINK_SIZE: u32 = 0x10000;
pub const SDW_SHIM_LCAP: u32 = 0x0;
pub const SDW_SHIM_LCAP_LCOUNT_MASK: u32 = genmask(2, 0);
pub const SDW_SHIM_LCAP_MLCS_MASK: u32 = bit(8);
pub const SDW_SHIM_LCTL: u32 = 0x4;
pub const SDW_SHIM_LCTL_SPA: u32 = bit(0);
pub const SDW_SHIM_LCTL_SPA_MASK: u32 = genmask(3, 0);
pub const SDW_SHIM_LCTL_CPA: u32 = bit(8);
pub const SDW_SHIM_LCTL_CPA_MASK: u32 = genmask(11, 8);
pub const SDW_SHIM_LCTL_MLCS_MASK: u32 = genmask(29, 27);
pub const SDW_SHIM_MLCS_XTAL_CLK: u32 = 0x0;
pub const SDW_SHIM_MLCS_CARDINAL_CLK: u32 = 0x1;
pub const SDW_SHIM_MLCS_AUDIO_PLL_CLK: u32 = 0x2;
pub const SDW_SHIM_SYNC: u32 = 0xc;
/* SDW_CADENCE_GSYNC_KHZ is supplied by the SoundWire dependency. */
pub const SDW_SHIM_SYNC_SYNCPRD_VAL_24: u32 = 24000 / SDW_CADENCE_GSYNC_KHZ - 1;
pub const SDW_SHIM_SYNC_SYNCPRD_VAL_24_576: u32 = 24576 / SDW_CADENCE_GSYNC_KHZ - 1;
pub const SDW_SHIM_SYNC_SYNCPRD_VAL_38_4: u32 = 38400 / SDW_CADENCE_GSYNC_KHZ - 1;
pub const SDW_SHIM_SYNC_SYNCPRD_VAL_96: u32 = 96000 / SDW_CADENCE_GSYNC_KHZ - 1;
pub const SDW_SHIM_SYNC_SYNCPRD: u32 = genmask(14, 0);
pub const SDW_SHIM_SYNC_SYNCCPU: u32 = bit(15);
pub const SDW_SHIM_SYNC_CMDSYNC_MASK: u32 = genmask(19, 16);
pub const SDW_SHIM_SYNC_CMDSYNC: u32 = bit(16);
pub const SDW_SHIM_SYNC_SYNCGO: u32 = bit(24);

pub const fn SDW_SHIM_CTLSCAP(x: u32) -> u32 { 0x010 + 0x60 * x }
pub const fn SDW_SHIM_CTLS0CM(x: u32) -> u32 { 0x012 + 0x60 * x }
pub const fn SDW_SHIM_CTLS1CM(x: u32) -> u32 { 0x014 + 0x60 * x }
pub const fn SDW_SHIM_CTLS2CM(x: u32) -> u32 { 0x016 + 0x60 * x }
pub const fn SDW_SHIM_CTLS3CM(x: u32) -> u32 { 0x018 + 0x60 * x }
pub const fn SDW_SHIM_PCMSCAP(x: u32) -> u32 { 0x020 + 0x60 * x }
pub const SDW_SHIM_PCMSCAP_ISS: u32 = genmask(3, 0);
pub const SDW_SHIM_PCMSCAP_OSS: u32 = genmask(7, 4);
pub const SDW_SHIM_PCMSCAP_BSS: u32 = genmask(12, 8);
pub const fn SDW_SHIM_PCMSYCHM(x: u32, y: u32) -> u32 { 0x022 + 0x60*x + 0x2*y }
pub const fn SDW_SHIM_PCMSYCHC(x: u32, y: u32) -> u32 { 0x042 + 0x60*x + 0x2*y }
pub const SDW_SHIM_PCMSYCM_LCHN: u32 = genmask(3, 0);
pub const SDW_SHIM_PCMSYCM_HCHN: u32 = genmask(7, 4);
pub const SDW_SHIM_PCMSYCM_STREAM: u32 = genmask(13, 8);
pub const SDW_SHIM_PCMSYCM_DIR: u32 = bit(15);
pub const fn SDW_SHIM_IOCTL(x: u32) -> u32 { 0x06c + 0x60*x }
pub const SDW_SHIM_IOCTL_MIF: u32 = bit(0); pub const SDW_SHIM_IOCTL_CO: u32 = bit(1);
pub const SDW_SHIM_IOCTL_COE: u32 = bit(2); pub const SDW_SHIM_IOCTL_DO: u32 = bit(3);
pub const SDW_SHIM_IOCTL_DOE: u32 = bit(4); pub const SDW_SHIM_IOCTL_BKE: u32 = bit(5);
pub const SDW_SHIM_IOCTL_WPDD: u32 = bit(6); pub const SDW_SHIM_IOCTL_CIBD: u32 = bit(8);
pub const SDW_SHIM_IOCTL_DIBD: u32 = bit(9);
pub const SDW_SHIM_WAKEEN: u32 = 0x190; pub const SDW_SHIM_WAKEEN_ENABLE: u32 = bit(0);
pub const SDW_SHIM_WAKESTS: u32 = 0x192; pub const SDW_SHIM_WAKESTS_STATUS: u32 = bit(0);
pub const fn SDW_SHIM_CTMCTL(x: u32) -> u32 { 0x06e + 0x60*x }
pub const SDW_SHIM_CTMCTL_DACTQE: u32 = bit(0); pub const SDW_SHIM_CTMCTL_DODS: u32 = bit(1);
pub const SDW_SHIM_CTMCTL_DOAIS: u32 = genmask(4, 3);
pub const fn SDW_ALH_STRMZCFG(x: u32) -> u32 { 0x000 + 4*x }
pub const SDW_ALH_NUM_STREAMS: u32 = 64; pub const SDW_ALH_STRMZCFG_DMAT_VAL: u32 = 3;
pub const SDW_ALH_STRMZCFG_DMAT: u32 = genmask(7, 0); pub const SDW_ALH_STRMZCFG_CHN: u32 = genmask(19, 16);

pub const fn SDW_SHIM2_GENERIC_BASE(x: u32) -> u32 { 0x00030000 + 0x8000*x }
pub const fn SDW_IP_BASE(x: u32) -> u32 { 0x00030100 + 0x8000*x }
pub const fn SDW_SHIM2_VS_BASE(x: u32) -> u32 { 0x00036000 + 0x8000*x }
pub const SDW_SHIM2_LECAP: u32 = 0; pub const SDW_SHIM2_LECAP_HDS: u32 = bit(0); pub const SDW_SHIM2_LECAP_MLC: u32 = genmask(3,1);
pub const SDW_SHIM2_PCMSCAP: u32 = 0x10; pub const SDW_SHIM2_PCMSCAP_ISS: u32 = genmask(3,0); pub const SDW_SHIM2_PCMSCAP_OSS: u32 = genmask(7,4); pub const SDW_SHIM2_PCMSCAP_BSS: u32 = genmask(12,8);
pub const fn SDW_SHIM2_PCMSYCHC(y: u32) -> u32 { 0x14 + 4*y } pub const SDW_SHIM2_PCMSYCHC_CS: u32 = genmask(3,0);
pub const fn SDW_SHIM2_PCMSYCHM(y: u32) -> u32 { 0x16 + 4*y } pub const SDW_SHIM2_PCMSYCHM_LCHAN: u32 = genmask(3,0); pub const SDW_SHIM2_PCMSYCHM_HCHAN: u32 = genmask(7,4); pub const SDW_SHIM2_PCMSYCHM_STRM: u32 = genmask(13,8); pub const SDW_SHIM2_PCMSYCHM_DIR: u32 = bit(15);
pub const SDW_SHIM2_INTEL_VS_LVSCTL: u32 = 4; pub const SDW_SHIM2_INTEL_VS_LVSCTL_FCG: u32 = bit(26); pub const SDW_SHIM2_INTEL_VS_LVSCTL_MLCS: u32 = genmask(29,27); pub const SDW_SHIM2_INTEL_VS_LVSCTL_DCGD: u32 = bit(30); pub const SDW_SHIM2_INTEL_VS_LVSCTL_ICGD: u32 = bit(31);
pub const SDW_SHIM2_MLCS_XTAL_CLK: u32 = 0; pub const SDW_SHIM2_MLCS_CARDINAL_CLK: u32 = 1; pub const SDW_SHIM2_MLCS_AUDIO_PLL_CLK: u32 = 2; pub const SDW_SHIM2_MLCS_MCLK_INPUT_CLK: u32 = 3; pub const SDW_SHIM2_MLCS_WOV_RING_OSC_CLK: u32 = 4;
pub const SDW_SHIM2_INTEL_VS_WAKEEN: u32 = 8; pub const SDW_SHIM2_INTEL_VS_WAKEEN_PWE: u32 = bit(0); pub const SDW_SHIM2_INTEL_VS_WAKESTS: u32 = 0x0a; pub const SDW_SHIM2_INTEL_VS_WAKEEN_PWS: u32 = bit(0);
pub const SDW_SHIM2_INTEL_VS_IOCTL: u32 = 0x0c; pub const SDW_SHIM2_INTEL_VS_IOCTL_MIF: u32 = bit(0); pub const SDW_SHIM2_INTEL_VS_IOCTL_CO: u32 = bit(1); pub const SDW_SHIM2_INTEL_VS_IOCTL_COE: u32 = bit(2); pub const SDW_SHIM2_INTEL_VS_IOCTL_DO: u32 = bit(3); pub const SDW_SHIM2_INTEL_VS_IOCTL_DOE: u32 = bit(4); pub const SDW_SHIM2_INTEL_VS_IOCTL_BKE: u32 = bit(5); pub const SDW_SHIM2_INTEL_VS_IOCTL_WPDD: u32 = bit(6); pub const SDW_SHIM2_INTEL_VS_IOCTL_ODC: u32 = bit(7); pub const SDW_SHIM2_INTEL_VS_IOCTL_CIBD: u32 = bit(8); pub const SDW_SHIM2_INTEL_VS_IOCTL_DIBD: u32 = bit(9); pub const SDW_SHIM2_INTEL_VS_IOCTL_HAMIFD: u32 = bit(10);
pub const SDW_SHIM2_INTEL_VS_ACTMCTL: u32 = 0x0e; pub const SDW_SHIM2_INTEL_VS_ACTMCTL_DACTQE: u32 = bit(0); pub const SDW_SHIM2_INTEL_VS_ACTMCTL_DODS: u32 = bit(1); pub const SDW_SHIM2_INTEL_VS_ACTMCTL_DODSE: u32 = bit(2); pub const SDW_SHIM2_INTEL_VS_ACTMCTL_DOAIS: u32 = genmask(4,3); pub const SDW_SHIM2_INTEL_VS_ACTMCTL_DOAISE: u32 = bit(5); pub const SDW_SHIM3_INTEL_VS_ACTMCTL_CLSS: u32 = bit(6); pub const SDW_SHIM3_INTEL_VS_ACTMCTL_CLDS: u32 = genmask(11,7); pub const SDW_SHIM3_INTEL_VS_ACTMCTL_DODSE2: u32 = genmask(13,12); pub const SDW_SHIM3_INTEL_VS_ACTMCTL_DOAISE2: u32 = bit(14); pub const SDW_SHIM3_INTEL_VS_ACTMCTL_CLDE: u32 = bit(15);
pub const SDW_SHIM2_INTEL_VS_PVCCS: u32 = 0x10;

#[repr(C)] pub struct sdw_intel_stream_params_data { pub substream: *mut snd_pcm_substream, pub dai: *mut snd_soc_dai, pub hw_params: *mut snd_pcm_hw_params, pub link_id: i32, pub alh_stream_id: i32 }
#[repr(C)] pub struct sdw_intel_stream_free_data { pub substream: *mut snd_pcm_substream, pub dai: *mut snd_soc_dai, pub link_id: i32 }
#[repr(C)] pub struct sdw_intel_ops { pub params_stream: Option<unsafe extern "C" fn(*mut device, *mut sdw_intel_stream_params_data) -> i32>, pub free_stream: Option<unsafe extern "C" fn(*mut device, *mut sdw_intel_stream_free_data) -> i32>, pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, i32, *mut snd_soc_dai) -> i32> }
#[repr(C)] pub struct sdw_intel_acpi_info { pub handle: acpi_handle, pub count: i32, pub link_mask: u32 }
pub struct sdw_intel_link_dev;
pub const SDW_INTEL_CLK_STOP_NOT_ALLOWED: u32 = bit(0); pub const SDW_INTEL_CLK_STOP_TEARDOWN: u32 = bit(1); pub const SDW_INTEL_CLK_STOP_WAKE_CAPABLE_ONLY: u32 = bit(2); pub const SDW_INTEL_CLK_STOP_BUS_RESET: u32 = bit(3);
#[repr(C)] pub struct sdw_intel_ctx { pub count: i32, pub mmio_base: *mut core::ffi::c_void, pub link_mask: u32, pub handle: acpi_handle, pub ldev: *mut *mut sdw_intel_link_dev, pub link_list: list_head, pub shim_lock: mutex, pub shim_mask: u32, pub shim_base: u32, pub alh_base: u32, pub peripherals: *mut sdw_peripherals }
#[repr(C)] pub struct sdw_intel_res { pub hw_ops: *const sdw_intel_hw_ops, pub count: i32, pub mmio_base: *mut core::ffi::c_void, pub irq: i32, pub handle: acpi_handle, pub parent: *mut device, pub ops: *const sdw_intel_ops, pub dev: *mut device, pub link_mask: u32, pub clock_stop_quirks: u32, pub shim_base: u32, pub alh_base: u32, pub ext: bool, pub mic_privacy: bool, pub hbus: *mut hdac_bus, pub eml_lock: *mut mutex }
pub const SDW_INTEL_QUIRK_MASK_BUS_DISABLE: u32 = bit(1); pub struct sdw_intel;
#[repr(C)] pub struct sdw_intel_hw_ops { pub debugfs_init: Option<unsafe extern "C" fn(*mut sdw_intel)>, pub debugfs_exit: Option<unsafe extern "C" fn(*mut sdw_intel)>, pub get_link_count: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub register_dai: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub check_clock_stop: Option<unsafe extern "C" fn(*mut sdw_intel)>, pub start_bus: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub start_bus_after_reset: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub start_bus_after_clock_stop: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub stop_bus: Option<unsafe extern "C" fn(*mut sdw_intel,bool)->i32>, pub link_power_up: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub link_power_down: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub shim_check_wake: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub shim_wake: Option<unsafe extern "C" fn(*mut sdw_intel,bool)>, pub pre_bank_switch: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub post_bank_switch: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub sync_arm: Option<unsafe extern "C" fn(*mut sdw_intel)>, pub sync_go_unlocked: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub sync_go: Option<unsafe extern "C" fn(*mut sdw_intel)->i32>, pub sync_check_cmdsync_unlocked: Option<unsafe extern "C" fn(*mut sdw_intel)->bool>, pub program_sdi: Option<unsafe extern "C" fn(*mut sdw_intel,i32)>, pub bpt_send_async: Option<unsafe extern "C" fn(*mut sdw_intel,*mut sdw_slave,*mut sdw_bpt_msg)->i32>, pub bpt_wait: Option<unsafe extern "C" fn(*mut sdw_intel,*mut sdw_slave,*mut sdw_bpt_msg)->i32> }
pub const SDW_INTEL_DEV_NUM_IDA_MIN: u32 = 6; pub const SDW_INTEL_MAX_LINKS: u32 = 5;

extern "C" {
    pub fn sdw_intel_acpi_scan(parent_handle: acpi_handle, info: *mut sdw_intel_acpi_info) -> i32;
    pub fn sdw_intel_process_wakeen_event(ctx: *mut sdw_intel_ctx);
    pub fn sdw_intel_probe(res: *mut sdw_intel_res) -> *mut sdw_intel_ctx;
    pub fn sdw_intel_startup(ctx: *mut sdw_intel_ctx) -> i32;
    pub fn sdw_intel_exit(ctx: *mut sdw_intel_ctx);
    pub fn sdw_intel_thread(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t;
    pub static sdw_intel_cnl_hw_ops: sdw_intel_hw_ops;
    pub static sdw_intel_lnl_hw_ops: sdw_intel_hw_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
