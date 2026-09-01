/* SPDX-License-Identifier: GPL-2.0 */
//
// ALSA SoC Texas Instruments PCM6240 Family Audio ADC/DAC/Router
//
// Copyright (C) 2022 - 2024 Texas Instruments Incorporated
// https://www.ti.com
//
// The PCM6240 driver implements a flexible and configurable
// algo coefficient setting for one, two, or even multiple
// PCM6240 Family Audio chips.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
//

// Header guard and C include directives are omitted in Rust.
// External Linux/ALSA types and constants are expected to be supplied by
// the surrounding translation unit or bindings.

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum pcm_device {
    ADC3120,
    ADC5120,
    ADC6120,
    DIX4192,
    PCM1690,
    PCM3120,
    PCM3140,
    PCM5120,
    PCM5140,
    PCM6120,
    PCM6140,
    PCM6240,
    PCM6260,
    PCM9211,
    PCMD3140,
    PCMD3180,
    PCMD512X,
    TAA5212,
    TAA5412,
    TAD5212,
    TAD5412,
    MAX_DEVICE,
}

pub const PCMDEV_GENERIC_VOL_CTRL: u32 = 0x0;
pub const PCMDEV_PCM1690_VOL_CTRL: u32 = 0x1;
pub const PCMDEV_PCM1690_FINE_VOL_CTRL: u32 = 0x2;

/* Maximum number of I2C addresses */
pub const PCMDEVICE_MAX_I2C_DEVICES: usize = 4;
/* Maximum number defined in REGBIN protocol */
pub const PCMDEVICE_MAX_REGBIN_DEVICES: usize = 8;
pub const PCMDEVICE_CONFIG_SUM: usize = 64;
pub const PCMDEVICE_BIN_FILENAME_LEN: usize = 64;

pub const PCMDEVICE_RATES: u32 = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
pub const PCMDEVICE_MAX_CHANNELS: u32 = 8;
pub const PCMDEVICE_FORMATS: u32 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

/* PAGE Control Register (available in page0 of each book) */
pub const PCMDEVICE_PAGE_SELECT: u32 = 0x00;
pub const fn PCMDEVICE_REG(page: u32, reg: u32) -> u32 {
    page * 128 + reg
}
pub const PCMDEVICE_REG_SWRESET: u32 = PCMDEVICE_REG(0x0, 0x01);
pub const PCMDEVICE_REG_SWRESET_RESET: u32 = BIT(0);

pub const ADC5120_REG_CH1_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x3d);
pub const ADC5120_REG_CH1_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x3e);
pub const ADC5120_REG_CH2_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x42);
pub const ADC5120_REG_CH2_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x43);

pub const PCM1690_REG_MODE_CTRL: u32 = PCMDEVICE_REG(0x0, 0x46);
pub const PCM1690_REG_MODE_CTRL_DAMS_MSK: u32 = BIT(7);
pub const PCM1690_REG_MODE_CTRL_DAMS_FINE_STEP: u32 = 0x0;
pub const PCM1690_REG_MODE_CTRL_DAMS_WIDE_RANGE: u32 = 0x80;

pub const PCM1690_REG_CH1_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x48);
pub const PCM1690_REG_CH2_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x49);
pub const PCM1690_REG_CH3_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4a);
pub const PCM1690_REG_CH4_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4b);
pub const PCM1690_REG_CH5_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4c);
pub const PCM1690_REG_CH6_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4d);
pub const PCM1690_REG_CH7_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4e);
pub const PCM1690_REG_CH8_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4f);

pub const PCM6240_REG_CH1_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x3d);
pub const PCM6240_REG_CH1_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x3e);
pub const PCM6240_REG_CH2_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x42);
pub const PCM6240_REG_CH2_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x43);
pub const PCM6240_REG_CH3_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x47);
pub const PCM6240_REG_CH3_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x48);
pub const PCM6240_REG_CH4_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4c);
pub const PCM6240_REG_CH4_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4d);

pub const PCM6260_REG_CH1_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x3d);
pub const PCM6260_REG_CH1_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x3e);
pub const PCM6260_REG_CH2_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x42);
pub const PCM6260_REG_CH2_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x43);
pub const PCM6260_REG_CH3_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x47);
pub const PCM6260_REG_CH3_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x48);
pub const PCM6260_REG_CH4_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4c);
pub const PCM6260_REG_CH4_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4d);
pub const PCM6260_REG_CH5_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x51);
pub const PCM6260_REG_CH5_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x52);
pub const PCM6260_REG_CH6_ANALOG_GAIN: u32 = PCMDEVICE_REG(0x0, 0x56);
pub const PCM6260_REG_CH6_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x57);

pub const PCM9211_REG_SW_CTRL: u32 = PCMDEVICE_REG(0x0, 0x40);
pub const PCM9211_REG_SW_CTRL_MRST_MSK: u32 = BIT(7);
pub const PCM9211_REG_SW_CTRL_MRST: u32 = 0x0;

pub const PCM9211_REG_CH1_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x46);
pub const PCM9211_REG_CH2_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x47);

pub const PCMD3140_REG_CH1_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x3E);
pub const PCMD3140_REG_CH2_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x43);
pub const PCMD3140_REG_CH3_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x48);
pub const PCMD3140_REG_CH4_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4D);

pub const PCMD3140_REG_CH1_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x3F);
pub const PCMD3140_REG_CH2_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x44);
pub const PCMD3140_REG_CH3_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x49);
pub const PCMD3140_REG_CH4_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4E);

pub const PCMD3180_REG_CH1_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x3E);
pub const PCMD3180_REG_CH2_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x43);
pub const PCMD3180_REG_CH3_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x48);
pub const PCMD3180_REG_CH4_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4D);
pub const PCMD3180_REG_CH5_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x52);
pub const PCMD3180_REG_CH6_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x57);
pub const PCMD3180_REG_CH7_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x5C);
pub const PCMD3180_REG_CH8_DIGITAL_GAIN: u32 = PCMDEVICE_REG(0x0, 0x61);

pub const PCMD3180_REG_CH1_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x3F);
pub const PCMD3180_REG_CH2_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x44);
pub const PCMD3180_REG_CH3_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x49);
pub const PCMD3180_REG_CH4_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x4E);
pub const PCMD3180_REG_CH5_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x53);
pub const PCMD3180_REG_CH6_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x58);
pub const PCMD3180_REG_CH7_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x5D);
pub const PCMD3180_REG_CH8_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x62);

pub const TAA5412_REG_CH1_DIGITAL_VOLUME: u32 = PCMDEVICE_REG(0x0, 0x52);
pub const TAA5412_REG_CH2_DIGITAL_VOLUME: u32 = PCMDEVICE_REG(0x0, 0x57);
pub const TAA5412_REG_CH3_DIGITAL_VOLUME: u32 = PCMDEVICE_REG(0x0, 0x5B);
pub const TAA5412_REG_CH4_DIGITAL_VOLUME: u32 = PCMDEVICE_REG(0x0, 0x5F);

pub const TAA5412_REG_CH1_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x53);
pub const TAA5412_REG_CH2_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x58);
pub const TAA5412_REG_CH3_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x5C);
pub const TAA5412_REG_CH4_FINE_GAIN: u32 = PCMDEVICE_REG(0x0, 0x60);

pub const PCMDEVICE_CMD_SING_W: u32 = 0x1;
pub const PCMDEVICE_CMD_BURST: u32 = 0x2;
pub const PCMDEVICE_CMD_DELAY: u32 = 0x3;
pub const PCMDEVICE_CMD_FIELD_W: u32 = 0x4;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum pcmdevice_bin_blk_type {
    PCMDEVICE_BIN_BLK_COEFF = 1,
    PCMDEVICE_BIN_BLK_POST_POWER_UP,
    PCMDEVICE_BIN_BLK_PRE_SHUTDOWN,
    PCMDEVICE_BIN_BLK_PRE_POWER_UP,
    PCMDEVICE_BIN_BLK_POST_SHUTDOWN,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum pcmdevice_fw_state {
    PCMDEVICE_FW_LOAD_OK = 0,
    PCMDEVICE_FW_LOAD_FAILED,
}

#[repr(C)]
pub struct pcmdevice_regbin_hdr {
    pub img_sz: ::core::ffi::c_uint,
    pub checksum: ::core::ffi::c_uint,
    pub binary_version_num: ::core::ffi::c_uint,
    pub drv_fw_version: ::core::ffi::c_uint,
    pub timestamp: ::core::ffi::c_uint,
    pub plat_type: ::core::ffi::c_uchar,
    pub dev_family: ::core::ffi::c_uchar,
    pub reserve: ::core::ffi::c_uchar,
    pub ndev: ::core::ffi::c_uchar,
    pub devs: [::core::ffi::c_uchar; PCMDEVICE_MAX_REGBIN_DEVICES],
    pub nconfig: ::core::ffi::c_uint,
    pub config_size: [::core::ffi::c_uint; PCMDEVICE_CONFIG_SUM],
}

#[repr(C)]
pub struct pcmdevice_block_data {
    pub dev_idx: ::core::ffi::c_uchar,
    pub block_type: ::core::ffi::c_uchar,
    pub yram_checksum: ::core::ffi::c_ushort,
    pub block_size: ::core::ffi::c_uint,
    pub n_subblks: ::core::ffi::c_uint,
    pub regdata: *mut ::core::ffi::c_uchar,
}

#[repr(C)]
pub struct pcmdevice_config_info {
    pub cfg_name: [::core::ffi::c_char; 64],
    pub nblocks: ::core::ffi::c_uint,
    pub real_nblocks: ::core::ffi::c_uint,
    pub active_dev: ::core::ffi::c_uchar,
    /* Flexible array member: struct pcmdevice_block_data *blk_data[] __counted_by(nblocks); */
    pub blk_data: [*mut pcmdevice_block_data; 0],
}

#[repr(C)]
pub struct pcmdevice_regbin {
    pub fw_hdr: pcmdevice_regbin_hdr,
    pub ncfgs: ::core::ffi::c_int,
    pub cfg_info: *mut *mut pcmdevice_config_info,
}

#[repr(C)]
pub struct pcmdevice_priv {
    pub component: *mut snd_soc_component,
    pub client: *mut i2c_client,
    pub dev: *mut device,
    pub codec_lock: mutex,
    pub hw_rst: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub regbin: pcmdevice_regbin,
    pub irq: ::core::ffi::c_int,
    pub addr: [::core::ffi::c_uint; PCMDEVICE_MAX_I2C_DEVICES],
    pub chip_id: ::core::ffi::c_uint,
    pub cur_conf: ::core::ffi::c_int,
    pub fw_state: ::core::ffi::c_int,
    pub ndev: ::core::ffi::c_int,
    pub bin_name: [::core::ffi::c_uchar; PCMDEVICE_BIN_FILENAME_LEN],
    /* used for kcontrol name */
    pub upper_dev_name: [::core::ffi::c_uchar; I2C_NAME_SIZE],
    pub dev_name: [::core::ffi::c_uchar; I2C_NAME_SIZE],
}

/* mixer control */
#[repr(C)]
pub struct pcmdevice_mixer_control {
    pub max: ::core::ffi::c_int,
    pub reg: ::core::ffi::c_int,
    pub dev_no: ::core::ffi::c_uint,
    pub shift: ::core::ffi::c_uint,
    pub invert: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct pcmdev_ctrl_info {
    pub gain: *const ::core::ffi::c_uint,
    pub pcmdev_ctrl: *const pcmdevice_mixer_control,
    pub ctrl_array_size: ::core::ffi::c_uint,
    pub get: *mut snd_kcontrol_get_t,
    pub put: *mut snd_kcontrol_put_t,
    pub pcmdev_ctrl_name_id: ::core::ffi::c_int,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
