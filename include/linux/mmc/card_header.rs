/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/include/linux/mmc/card.h
 *
 * Card driver specific definitions.
 *
 * C dependencies supplied by other headers are intentionally left external.
 */

#[repr(C)]
pub struct mmc_cid {
    pub manfid: ::core::ffi::c_uint,
    pub prod_name: [::core::ffi::c_char; 8],
    pub prv: u8,
    pub serial: ::core::ffi::c_uint,
    pub oemid: u16,
    pub year: u16,
    pub hwrev: u8,
    pub fwrev: u8,
    pub month: u8,
}

#[repr(C)]
pub struct mmc_csd {
    pub structure: u8,
    pub mmca_vsn: u8,
    pub cmdclass: u16,
    pub taac_clks: u16,
    pub taac_ns: ::core::ffi::c_uint,
    pub c_size: ::core::ffi::c_uint,
    pub r2w_factor: ::core::ffi::c_uint,
    pub max_dtr: ::core::ffi::c_uint,
    pub erase_size: ::core::ffi::c_uint,
    pub wp_grp_size: ::core::ffi::c_uint,
    pub read_blkbits: ::core::ffi::c_uint,
    pub write_blkbits: ::core::ffi::c_uint,
    pub capacity: sector_t,
    // C bitfields: read_partial, read_misalign, write_partial, write_misalign, dsr_imp.
    pub bitfields: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct mmc_ext_csd {
    pub rev: u8,
    pub erase_group_def: u8,
    pub sec_feature_support: u8,
    pub rel_sectors: u8,
    pub rel_param: u8,
    pub enhanced_rpmb_supported: bool,
    pub part_config: u8,
    pub cache_ctrl: u8,
    pub rst_n_function: u8,
    pub part_time: ::core::ffi::c_uint,
    pub sa_timeout: ::core::ffi::c_uint,
    pub generic_cmd6_time: ::core::ffi::c_uint,
    pub power_off_longtime: ::core::ffi::c_uint,
    pub power_off_notification: u8,
    pub hs_max_dtr: ::core::ffi::c_uint,
    pub hs200_max_dtr: ::core::ffi::c_uint,
    pub sectors: ::core::ffi::c_uint,
    pub hc_erase_size: ::core::ffi::c_uint,
    pub hc_erase_timeout: ::core::ffi::c_uint,
    pub sec_trim_mult: ::core::ffi::c_uint,
    pub sec_erase_mult: ::core::ffi::c_uint,
    pub trim_timeout: ::core::ffi::c_uint,
    pub partition_setting_completed: bool,
    pub enhanced_area_offset: u64,
    pub enhanced_area_size: ::core::ffi::c_uint,
    pub cache_size: ::core::ffi::c_uint,
    pub hpi_en: bool,
    pub hpi: bool,
    pub hpi_cmd: ::core::ffi::c_uint,
    pub bkops: bool,
    pub man_bkops_en: bool,
    pub auto_bkops_en: bool,
    pub data_sector_size: ::core::ffi::c_uint,
    pub data_tag_unit_size: ::core::ffi::c_uint,
    pub boot_ro_lock: ::core::ffi::c_uint,
    pub boot_ro_lockable: bool,
    pub ffu_capable: bool,
    pub cmdq_en: bool,
    pub cmdq_support: bool,
    pub cmdq_depth: ::core::ffi::c_uint,
    pub fwrev: [u8; MMC_FIRMWARE_LEN],
    pub raw_exception_status: u8,
    pub raw_partition_support: u8,
    pub raw_rpmb_size_mult: u8,
    pub raw_erased_mem_count: u8,
    pub strobe_support: u8,
    pub raw_ext_csd_structure: u8,
    pub raw_card_type: u8,
    pub raw_driver_strength: u8,
    pub out_of_int_time: u8,
    pub raw_pwr_cl_52_195: u8,
    pub raw_pwr_cl_26_195: u8,
    pub raw_pwr_cl_52_360: u8,
    pub raw_pwr_cl_26_360: u8,
    pub raw_s_a_timeout: u8,
    pub raw_hc_erase_gap_size: u8,
    pub raw_erase_timeout_mult: u8,
    pub raw_hc_erase_grp_size: u8,
    pub raw_boot_mult: u8,
    pub raw_sec_trim_mult: u8,
    pub raw_sec_erase_mult: u8,
    pub raw_sec_feature_support: u8,
    pub raw_trim_mult: u8,
    pub raw_pwr_cl_200_195: u8,
    pub raw_pwr_cl_200_360: u8,
    pub raw_pwr_cl_ddr_52_195: u8,
    pub raw_pwr_cl_ddr_52_360: u8,
    pub raw_pwr_cl_ddr_200_360: u8,
    pub raw_bkops_status: u8,
    pub raw_sectors: [u8; 4],
    pub pre_eol_info: u8,
    pub device_life_time_est_typ_a: u8,
    pub device_life_time_est_typ_b: u8,
    pub feature_support: ::core::ffi::c_uint,
}

pub const MMC_HIGH_26_MAX_DTR: ::core::ffi::c_uint = 26000000;
pub const MMC_HIGH_52_MAX_DTR: ::core::ffi::c_uint = 52000000;
pub const MMC_HIGH_DDR_MAX_DTR: ::core::ffi::c_uint = 52000000;
pub const MMC_HS200_MAX_DTR: ::core::ffi::c_uint = 200000000;
pub const MMC_FIRMWARE_LEN: usize = 8;
pub const MMC_DISCARD_FEATURE: ::core::ffi::c_uint = 1 << 0;

#[repr(C)]
pub struct sd_scr {
    pub sda_vsn: u8, pub sda_spec3: u8, pub sda_spec4: u8, pub sda_specx: u8,
    pub bus_widths: u8, pub cmds: u8,
}
pub const SD_SCR_BUS_WIDTH_1: ::core::ffi::c_uint = 1 << 0;
pub const SD_SCR_BUS_WIDTH_4: ::core::ffi::c_uint = 1 << 2;
pub const SD_SCR_CMD20_SUPPORT: ::core::ffi::c_uint = 1 << 0;
pub const SD_SCR_CMD23_SUPPORT: ::core::ffi::c_uint = 1 << 1;
pub const SD_SCR_CMD48_SUPPORT: ::core::ffi::c_uint = 1 << 2;
pub const SD_SCR_CMD58_SUPPORT: ::core::ffi::c_uint = 1 << 3;

#[repr(C)] pub struct sd_ssr { pub au: u32, pub erase_timeout: u32, pub erase_offset: u32 }
#[repr(C)] pub struct sd_switch_caps { pub hs_max_dtr: u32, pub uhs_max_dtr: u32, pub sd3_bus_mode: u32, pub sd3_drv_type: u32, pub sd3_curr_limit: u32 }
#[repr(C)] pub struct sd_ext_reg { pub fno: u8, pub page: u8, pub offset: u16, pub rev: u8, pub feature_enabled: u8, pub feature_support: u8 }
#[repr(C)] pub struct sd_uhs2_config { pub node_id: u32, pub n_fcu: u32, pub maxblk_len: u32, pub n_lanes: u8, pub dadr_len: u8, pub app_type: u8, pub phy_minor_rev: u8, pub phy_major_rev: u8, pub can_hibernate: u8, pub n_lss_sync: u8, pub n_lss_dir: u8, pub link_minor_rev: u8, pub link_major_rev: u8, pub dev_type: u8, pub n_data_gap: u8, pub n_fcu_set: u32, pub maxblk_len_set: u32, pub n_lanes_set: u8, pub speed_range_set: u8, pub n_lss_sync_set: u8, pub n_lss_dir_set: u8, pub n_data_gap_set: u8, pub max_retry_set: u8 }
#[repr(C)] pub struct sdio_cccr { pub sdio_vsn: u32, pub sd_vsn: u32, pub bitfields: u32 }
#[repr(C)] pub struct sdio_cis { pub vendor: u16, pub device: u16, pub blksize: u16, pub max_dtr: u32 }

pub struct mmc_host;
pub struct sdio_func;
pub struct sdio_func_tuple;
pub struct mmc_queue_req;

pub const SDIO_MAX_FUNCS: usize = 7;
pub const MMC_NUM_BOOT_PARTITION: usize = 2;
pub const MMC_NUM_GP_PARTITION: usize = 4;
pub const MMC_NUM_PHY_PARTITION: usize = 7;
pub const MAX_MMC_PART_NAME_LEN: usize = 20;

#[repr(C)]
pub struct mmc_part { pub size: u64, pub part_cfg: u32, pub name: [::core::ffi::c_char; MAX_MMC_PART_NAME_LEN], pub force_ro: bool, pub area_type: u32 }
pub const MMC_BLK_DATA_AREA_MAIN: u32 = 1 << 0;
pub const MMC_BLK_DATA_AREA_BOOT: u32 = 1 << 1;
pub const MMC_BLK_DATA_AREA_GP: u32 = 1 << 2;
pub const MMC_BLK_DATA_AREA_RPMB: u32 = 1 << 3;

#[repr(C)]
pub struct mmc_card {
    pub host: *mut mmc_host, pub dev: device, pub ocr: u32, pub rca: u32, pub type_: u32,
    pub state: u32, pub quirks: u32, pub quirk_max_rate: u32, pub written_flag: bool, pub reenable_cmdq: bool,
    pub erase_size: u32, pub erase_shift: u32, pub pref_erase: u32, pub eg_boundary: u32, pub erase_arg: u32, pub erased_byte: u8, pub wp_grp_size: u32,
    pub raw_cid: [u32; 4], pub raw_csd: [u32; 4], pub raw_scr: [u32; 2], pub raw_ssr: [u32; 16],
    pub cid: mmc_cid, pub csd: mmc_csd, pub ext_csd: mmc_ext_csd, pub scr: sd_scr, pub ssr: sd_ssr, pub sw_caps: sd_switch_caps, pub ext_power: sd_ext_reg, pub ext_perf: sd_ext_reg,
    pub uhs2_config: sd_uhs2_config, pub sdio_funcs: u32, pub sdio_funcs_probed: atomic_t, pub cccr: sdio_cccr, pub cis: sdio_cis,
    pub sdio_func: [*mut sdio_func; SDIO_MAX_FUNCS], pub sdio_single_irq: *mut sdio_func, pub major_rev: u8, pub minor_rev: u8, pub num_info: u32, pub info: *const *const ::core::ffi::c_char, pub tuples: *mut sdio_func_tuple,
    pub sd_bus_speed: u32, pub mmc_avail_type: u32, pub drive_strength: u32, pub debugfs_root: *mut dentry, pub part: [mmc_part; MMC_NUM_PHY_PARTITION], pub nr_parts: u32, pub complete_wq: *mut workqueue_struct,
}

pub const MMC_TYPE_MMC: u32 = 0;
pub const MMC_TYPE_SD: u32 = 1;
pub const MMC_TYPE_SDIO: u32 = 2;
pub const MMC_TYPE_SD_COMBO: u32 = 3;

pub unsafe fn mmc_large_sector(card: *mut mmc_card) -> bool { (*card).ext_csd.data_sector_size == 4096 }
pub unsafe fn mmc_card_enable_async_irq(card: *mut mmc_card) -> i32 { (*card).cccr.bitfields }
extern "C" { pub fn mmc_card_is_blockaddr(card: *mut mmc_card) -> bool; }

pub const MMC_QUIRK_LENIENT_FN0: u32 = 1 << 0;
pub const MMC_QUIRK_BLKSZ_FOR_BYTE_MODE: u32 = 1 << 1;
pub const MMC_QUIRK_NONSTD_SDIO: u32 = 1 << 2;
pub const MMC_QUIRK_NONSTD_FUNC_IF: u32 = 1 << 4;
pub const MMC_QUIRK_DISABLE_CD: u32 = 1 << 5;
pub const MMC_QUIRK_INAND_CMD38: u32 = 1 << 6;
pub const MMC_QUIRK_BLK_NO_CMD23: u32 = 1 << 7;
pub const MMC_QUIRK_BROKEN_BYTE_MODE_512: u32 = 1 << 8;
pub const MMC_QUIRK_LONG_READ_TIME: u32 = 1 << 9;
pub const MMC_QUIRK_SEC_ERASE_TRIM_BROKEN: u32 = 1 << 10;
pub const MMC_QUIRK_BROKEN_IRQ_POLLING: u32 = 1 << 11;
pub const MMC_QUIRK_TRIM_BROKEN: u32 = 1 << 12;
pub const MMC_QUIRK_BROKEN_HPI: u32 = 1 << 13;
pub const MMC_QUIRK_BROKEN_SD_DISCARD: u32 = 1 << 14;
pub const MMC_QUIRK_BROKEN_SD_CACHE: u32 = 1 << 15;
pub const MMC_QUIRK_BROKEN_CACHE_FLUSH: u32 = 1 << 16;
pub const MMC_QUIRK_BROKEN_SD_POWEROFF_NOTIFY: u32 = 1 << 17;
pub const MMC_QUIRK_NO_UHS_DDR50_TUNING: u32 = 1 << 18;
pub const MMC_QUIRK_BROKEN_MDT: u32 = 1 << 19;
pub const MMC_QUIRK_FIXED_SECURE_ERASE_TRIM_TIME: u32 = 1 << 20;

pub const HIGH_SPEED_MAX_DTR: u32 = 50000000;
pub const UHS_SDR104_MAX_DTR: u32 = 208000000;
pub const UHS_SDR50_MAX_DTR: u32 = 100000000;
pub const UHS_DDR50_MAX_DTR: u32 = 50000000;
pub const UHS_SDR25_MAX_DTR: u32 = UHS_DDR50_MAX_DTR;
pub const UHS_SDR12_MAX_DTR: u32 = 25000000;
pub const DEFAULT_SPEED_MAX_DTR: u32 = UHS_SDR12_MAX_DTR;
pub const UHS_SDR12_BUS_SPEED: u32 = 0;
pub const HIGH_SPEED_BUS_SPEED: u32 = 1;
pub const UHS_SDR25_BUS_SPEED: u32 = 1;
pub const UHS_SDR50_BUS_SPEED: u32 = 2;
pub const UHS_SDR104_BUS_SPEED: u32 = 3;
pub const UHS_DDR50_BUS_SPEED: u32 = 4;
pub const SD_MODE_HIGH_SPEED: u32 = 1 << HIGH_SPEED_BUS_SPEED;
pub const SD_MODE_UHS_SDR12: u32 = 1 << UHS_SDR12_BUS_SPEED;
pub const SD_MODE_UHS_SDR25: u32 = 1 << UHS_SDR25_BUS_SPEED;
pub const SD_MODE_UHS_SDR50: u32 = 1 << UHS_SDR50_BUS_SPEED;
pub const SD_MODE_UHS_SDR104: u32 = 1 << UHS_SDR104_BUS_SPEED;
pub const SD_MODE_UHS_DDR50: u32 = 1 << UHS_DDR50_BUS_SPEED;
pub const SD_DRIVER_TYPE_B: u32 = 0x01;
pub const SD_DRIVER_TYPE_A: u32 = 0x02;
pub const SD_DRIVER_TYPE_C: u32 = 0x04;
pub const SD_DRIVER_TYPE_D: u32 = 0x08;
pub const SD_SET_CURRENT_LIMIT_200: u32 = 0;
pub const SD_SET_CURRENT_LIMIT_400: u32 = 1;
pub const SD_SET_CURRENT_LIMIT_600: u32 = 2;
pub const SD_SET_CURRENT_LIMIT_800: u32 = 3;
pub const SD_MAX_CURRENT_200: u32 = 1 << SD_SET_CURRENT_LIMIT_200;
pub const SD_MAX_CURRENT_400: u32 = 1 << SD_SET_CURRENT_LIMIT_400;
pub const SD_MAX_CURRENT_600: u32 = 1 << SD_SET_CURRENT_LIMIT_600;
pub const SD_MAX_CURRENT_800: u32 = 1 << SD_SET_CURRENT_LIMIT_800;
pub const SD4_SET_POWER_LIMIT_0_72W: u32 = 0;
pub const SD4_SET_POWER_LIMIT_1_44W: u32 = 1;
pub const SD4_SET_POWER_LIMIT_2_16W: u32 = 2;
pub const SD4_SET_POWER_LIMIT_2_88W: u32 = 3;
pub const SD4_SET_POWER_LIMIT_1_80W: u32 = 4;
pub const SD_EXT_POWER_OFF_NOTIFY: u8 = 1 << 0;
pub const SD_EXT_POWER_SUSTENANCE: u8 = 1 << 1;
pub const SD_EXT_POWER_DOWN_MODE: u8 = 1 << 2;
pub const SD_EXT_PERF_FX_EVENT: u8 = 1 << 0;
pub const SD_EXT_PERF_CARD_MAINT: u8 = 1 << 1;
pub const SD_EXT_PERF_HOST_MAINT: u8 = 1 << 2;
pub const SD_EXT_PERF_CACHE: u8 = 1 << 3;
pub const SD_EXT_PERF_CMD_QUEUE: u8 = 1 << 4;

#[inline]
pub unsafe fn mmc_card_mmc(c: *const mmc_card) -> bool { (*c).type_ == MMC_TYPE_MMC }
#[inline]
pub unsafe fn mmc_card_sd(c: *const mmc_card) -> bool { (*c).type_ == MMC_TYPE_SD }
#[inline]
pub unsafe fn mmc_card_sdio(c: *const mmc_card) -> bool { (*c).type_ == MMC_TYPE_SDIO }
#[inline]
pub unsafe fn mmc_card_sd_combo(c: *const mmc_card) -> bool { (*c).type_ == MMC_TYPE_SD_COMBO }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
