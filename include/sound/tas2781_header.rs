/* SPDX-License-Identifier: GPL-2.0 */
// ALSA SoC Texas Instruments TAS2563/TAS2781 Audio Smart Amplifier
// Copyright (C) 2022 - 2026 Texas Instruments Incorporated

// Dependency supplied by the surrounding driver translation: tas2781-dsp.h

pub const TAS2781_DRV_VER: i32 = 1;
pub const SMARTAMP_MODULE_NAME: &str = "tas2781";
pub const TAS2781_GLOBAL_ADDR: u32 = 0x40;
pub const TAS2563_GLOBAL_ADDR: u32 = 0x48;
// SNDRV_PCM_RATE_* and SNDRV_PCM_FMTBIT_* are supplied by the ALSA dependency.
pub const TASDEVICE_RATES: u32 = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 |
    SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_88200;
pub const TASDEVICE_FORMATS: u32 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S32_LE;
pub const TASDEVICE_CRC8_POLYNOMIAL: u8 = 0x4d;

pub const TASDEVICE_PAGE_SELECT: u32 = 0x00;
pub const TASDEVICE_BOOKCTL_PAGE: u32 = 0x00;
pub const TASDEVICE_BOOKCTL_REG: u32 = 127;
#[inline] pub const fn TASDEVICE_BOOK_ID(reg: u32) -> u32 { reg / (256 * 128) }
#[inline] pub const fn TASDEVICE_PAGE_ID(reg: u32) -> u32 { (reg % (256 * 128)) / 128 }
#[inline] pub const fn TASDEVICE_PAGE_REG(reg: u32) -> u32 { (reg % (256 * 128)) % 128 }
#[inline] pub const fn TASDEVICE_PGRG(reg: u32) -> u32 { reg % (256 * 128) }
#[inline] pub const fn TASDEVICE_REG(book: u32, page: u32, reg: u32) -> u32 { (book * 256 * 128) + page * 128 + reg }

pub const TASDEVICE_REG_SWRESET: u32 = TASDEVICE_REG(0, 0, 0x01);
pub const TASDEVICE_REG_SWRESET_RESET: u32 = 1 << 0;
pub const TAS5825_REG_SWRESET_RESET: u32 = (1 << 0) | (1 << 4);
pub const TASDEVICE_CHECKSUM_REG: u32 = TASDEVICE_REG(0, 0, 0x7e);
pub const TASDEVICE_XM_A1_REG: u32 = TASDEVICE_REG(0x64, 0x02, 0x4c);
pub const TASDEVICE_XM_A2_REG: u32 = TASDEVICE_REG(0x64, 0x02, 0x64);
pub const TAS2563_DVC_LVL: u32 = TASDEVICE_REG(0, 2, 0x0c);
pub const TAS2781_DVC_LVL: u32 = TASDEVICE_REG(0, 0, 0x1a);
pub const TAS2781_AMP_LEVEL: u32 = TASDEVICE_REG(0, 0, 0x03);
pub const TAS2781_AMP_LEVEL_MASK: u32 = 0x3e;
pub const TAS2563_IDLE: u32 = TASDEVICE_REG(0, 0, 0x3e);
pub const TAS2563_PRM_R0_REG: u32 = TASDEVICE_REG(0, 0x0f, 0x34);
pub const TAS2563_RUNTIME_RE_REG_TF: u32 = TASDEVICE_REG(0x64, 2, 0x70);
pub const TAS2563_RUNTIME_RE_REG: u32 = TASDEVICE_REG(0x64, 2, 0x48);
pub const TAS2563_PRM_ENFF_REG: u32 = TASDEVICE_REG(0, 0x0d, 0x54);
pub const TAS2563_PRM_DISTCK_REG: u32 = TASDEVICE_REG(0, 0x0d, 0x58);
pub const TAS2563_PRM_TE_SCTHR_REG: u32 = TASDEVICE_REG(0, 0x0f, 0x60);
pub const TAS2563_PRM_PLT_FLAG_REG: u32 = TASDEVICE_REG(0, 0x0d, 0x74);
pub const TAS2563_PRM_SINEGAIN_REG: u32 = TASDEVICE_REG(0, 0x0d, 0x7c);
pub const TAS2563_TE_TA1_REG: u32 = TASDEVICE_REG(0, 0x10, 0x0c);
pub const TAS2563_TE_TA1_AT_REG: u32 = TASDEVICE_REG(0, 0x10, 0x10);
pub const TAS2563_TE_TA2_REG: u32 = TASDEVICE_REG(0, 0x0f, 0x64);
pub const TAS2563_TE_AT_REG: u32 = TASDEVICE_REG(0, 0x0f, 0x68);
pub const TAS2563_TE_DT_REG: u32 = TASDEVICE_REG(0, 0x0f, 0x70);
pub const TAS2781_PRM_INT_MASK_REG: u32 = TASDEVICE_REG(0, 0, 0x3b);
pub const TAS2781_PRM_CLK_CFG_REG: u32 = TASDEVICE_REG(0, 0, 0x5c);
pub const TAS2781_PRM_RSVD_REG: u32 = TASDEVICE_REG(0, 1, 0x19);
pub const TAS2781_PRM_TEST_57_REG: u32 = TASDEVICE_REG(0, 0xfd, 0x39);
pub const TAS2781_PRM_TEST_62_REG: u32 = TASDEVICE_REG(0, 0xfd, 0x3e);
pub const TAS2781_PRM_PVDD_UVLO_REG: u32 = TASDEVICE_REG(0, 0, 0x71);
pub const TAS2781_PRM_CHNL_0_REG: u32 = TASDEVICE_REG(0, 0, 3);
pub const TAS2781_PRM_NG_CFG0_REG: u32 = TASDEVICE_REG(0, 0, 0x35);
pub const TAS2781_PRM_IDLE_CH_DET_REG: u32 = TASDEVICE_REG(0, 0, 0x66);
pub const TAS2781_PRM_PLT_FLAG_REG: u32 = TASDEVICE_REG(0, 0x14, 0x38);
pub const TAS2781_PRM_SINEGAIN_REG: u32 = TASDEVICE_REG(0, 0x14, 0x40);
pub const TAS2781_PRM_SINEGAIN2_REG: u32 = TASDEVICE_REG(0, 0x14, 0x44);
pub const TAS2781_TEST_UNLOCK_REG: u32 = TASDEVICE_REG(0, 0xfd, 0x0d);
pub const TAS2781_TEST_PAGE_UNLOCK: u32 = 0x0d;
pub const TAS2781_RUNTIME_LATCH_RE_REG: u32 = TASDEVICE_REG(0, 0, 0x49);
pub const TAS2781_RUNTIME_RE_REG_TF: u32 = TASDEVICE_REG(0x64, 0x62, 0x48);
pub const TAS2781_RUNTIME_RE_REG: u32 = TASDEVICE_REG(0x64, 0x63, 0x44);

#[repr(C)] pub enum audio_device { TAS2020, TAS2118, TAS2120, TAS2320, TAS2563, TAS2568, TAS2570, TAS2572, TAS2573, TAS2574, TAS2781, TAS5802, TAS5806M, TAS5806MD, TAS5815, TAS5822, TAS5825, TAS5827, TAS5828, TAS5830, TAS5832, TAS_OTHERS }
#[repr(C)] pub enum dspbin_type { TASDEV_BASIC, TASDEV_ALPHA, TASDEV_BETA }

#[repr(C)] pub struct bulk_reg_val { pub reg: i32, pub val: [u8; 4], pub val_len: u8, pub is_locked: bool }
#[repr(C)] pub struct tasdevice { pub cali_data_backup: *mut bulk_reg_val, pub alp_cali_bckp: bulk_reg_val, pub cali_data_fmw: *mut tasdevice_fw, pub cali_specific: *mut core::ffi::c_void, pub dev_addr: u32, pub err_code: u32, pub cur_book: u8, pub cur_prog: i16, pub cur_conf: i16, pub is_loading: bool, pub is_loaderr: bool }
#[repr(C)] pub struct cali_reg { pub r0_reg: u32, pub r0_low_reg: u32, pub invr0_reg: u32, pub pow_reg: u32, pub tlimit_reg: u32 }
#[repr(C)] pub struct calidata { pub data: *mut u8, pub total_sz: usize, pub cali_reg_array: cali_reg, pub cali_dat_sz_per_dev: u32 }

// CONFIG_SND_SOC_TAS2781_ACOUST_I2C conditionally adds the following field/type.
#[cfg(feature = "CONFIG_SND_SOC_TAS2781_ACOUST_I2C")]
pub const TASDEV_DATA_PAYLOAD_SIZE: usize = 128;
#[cfg(feature = "CONFIG_SND_SOC_TAS2781_ACOUST_I2C")]
#[repr(C)] pub struct acoustic_data { pub len: u8, pub id: u8, pub addr: u8, pub book: u8, pub page: u8, pub reg: u8, pub data: [u8; TASDEV_DATA_PAYLOAD_SIZE] }

// External types supplied by included driver headers.
pub struct tasdevice_rca; pub struct tasdevice_fw; pub struct gpio_desc; pub struct mutex; pub struct regmap; pub struct device; pub struct firmware; pub struct tasdev_blk;

#[repr(C)] pub struct tasdevice_priv {
    pub tasdevice: [tasdevice; TASDEVICE_MAX_CHANNELS], pub rcabin: tasdevice_rca, pub cali_data: calidata,
    #[cfg(feature = "CONFIG_SND_SOC_TAS2781_ACOUST_I2C")] pub acou_data: acoustic_data,
    pub fmw: *mut tasdevice_fw, pub reset: *mut gpio_desc, pub codec_lock: mutex, pub regmap: *mut regmap, pub dev: *mut device,
    pub cal_binaryname: [[u8; 64]; TASDEVICE_MAX_CHANNELS], pub crc8_lkp_tbl: [u8; CRC8_TABLE_SIZE], pub coef_binaryname: [u8; 64], pub rca_binaryname: [u8; 64], pub dev_name: [u8; 32], pub dvc_tlv_table: *const [u8; 4], pub name_prefix: *const core::ffi::c_char, pub ndev: u8, pub dspbin_typ: u32, pub magic_num: u32, pub chip_id: u32, pub sysclk: u32, pub speaker_id: i32,
    pub irq: i32, pub cur_prog: i32, pub cur_conf: i32, pub fw_state: i32, pub index: i32, pub client: *mut core::ffi::c_void, pub codec: *mut core::ffi::c_void, pub force_fwload_status: bool, pub playback_started: bool, pub isacpi: bool, pub isspi: bool, pub global_addr: u32,
    pub fw_parse_variable_header: Option<unsafe extern "C" fn(*mut tasdevice_priv, *const firmware, i32) -> i32>,
    pub fw_parse_program_data: Option<unsafe extern "C" fn(*mut tasdevice_priv, *mut tasdevice_fw, *const firmware, i32) -> i32>, pub fw_parse_configuration_data: Option<unsafe extern "C" fn(*mut tasdevice_priv, *mut tasdevice_fw, *const firmware, i32) -> i32>, pub fw_parse_fct_param_address: Option<unsafe extern "C" fn(*mut tasdevice_priv, *mut tasdevice_fw, *const firmware, i32) -> i32>, pub tasdevice_load_block: Option<unsafe extern "C" fn(*mut tasdevice_priv, *mut tasdev_blk) -> i32>,
    pub change_chn_book: Option<unsafe extern "C" fn(*mut tasdevice_priv, u16, i32) -> i32>, pub update_bits: Option<unsafe extern "C" fn(*mut tasdevice_priv, u16, u32, u32, u32) -> i32>, pub dev_read: Option<unsafe extern "C" fn(*mut tasdevice_priv, u16, u32, *mut u32) -> i32>, pub dev_bulk_read: Option<unsafe extern "C" fn(*mut tasdevice_priv, u16, u32, *mut u8, u32) -> i32>,
}

extern "C" { pub fn tasdevice_dev_read(tas_priv: *mut tasdevice_priv, chn: u16, reg: u32, value: *mut u32) -> i32; pub fn tasdevice_dev_bulk_read(tas_priv: *mut tasdevice_priv, chn: u16, reg: u32, p_data: *mut u8, n_length: u32) -> i32; pub fn tasdevice_dev_write(tas_priv: *mut tasdevice_priv, chn: u16, reg: u32, value: u32) -> i32; pub fn tasdevice_dev_bulk_write(tas_priv: *mut tasdevice_priv, chn: u16, reg: u32, p_data: *mut u8, n_length: u32) -> i32; pub fn tasdevice_remove(tas_priv: *mut tasdevice_priv); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
