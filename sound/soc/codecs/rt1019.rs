// SPDX-License-Identifier: GPL-2.0-only
//
// rt1019.c  --  RT1019 ALSA SoC audio amplifier driver
// Author: Jack Yu <jack.yu@realtek.com>
//
// Copyright(c) 2021 Realtek Semiconductor Corp.
//
//

/* Translated from the implementation source. Linux, ASoC, regmap, I2C,
 * rl6231, and rt1019 header-provided symbols are external dependencies.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

unsafe extern "C" {
    static RT1019_PWR_STRP_2: c_uint;
    static RT1019_VER_ID: c_uint;
    static RT1019_VEND_ID_1: c_uint;
    static RT1019_VEND_ID_2: c_uint;
    static RT1019_DEV_ID_1: c_uint;
    static RT1019_DEV_ID_2: c_uint;
    static RT1019_RESET: c_uint;
    static RT1019_IDS_CTRL: c_uint;
    static RT1019_ASEL_CTRL: c_uint;
    static RT1019_BEEP_TONE: c_uint;
    static RT1019_SDB_CTRL: c_uint;
    static RT1019_CLK_TREE_1: c_uint;
    static RT1019_CLK_TREE_2: c_uint;
    static RT1019_CLK_TREE_3: c_uint;
    static RT1019_PLL_1: c_uint;
    static RT1019_PLL_2: c_uint;
    static RT1019_PLL_3: c_uint;
    static RT1019_TDM_1: c_uint;
    static RT1019_TDM_2: c_uint;
    static RT1019_TDM_3: c_uint;
    static RT1019_DMIX_MONO_1: c_uint;
    static RT1019_DMIX_MONO_2: c_uint;
    static RT1019_BEEP_1: c_uint;
    static RT1019_BEEP_2: c_uint;
}

unsafe extern "C" {
    static RT1019_SYS_DIV_DA_FIL_DIV1: c_uint;
    static RT1019_SYS_DA_OSR_DIV1: c_uint;
    static RT1019_ASRC_256FS_DIV1: c_uint;
    static RT1019_SEL_FIFO_DIV1: c_uint;
    static RT1019_SEL_CLK_CAL_DIV1: c_uint;
    static RT1019_SYS_DIV_DA_FIL_DIV2: c_uint;
    static RT1019_SYS_DA_OSR_DIV2: c_uint;
    static RT1019_ASRC_256FS_DIV2: c_uint;
    static RT1019_SEL_FIFO_DIV2: c_uint;
    static RT1019_SEL_CLK_CAL_DIV2: c_uint;
    static RT1019_SYS_DIV_DA_FIL_DIV4: c_uint;
    static RT1019_SYS_DA_OSR_DIV4: c_uint;
    static RT1019_ASRC_256FS_DIV4: c_uint;
    static RT1019_SEL_FIFO_DIV4: c_uint;
    static RT1019_SEL_CLK_CAL_DIV4: c_uint;
    static RT1019_I2S_DL_20: c_uint;
    static RT1019_I2S_DL_24: c_uint;
    static RT1019_I2S_DL_32: c_uint;
    static RT1019_I2S_DL_8: c_uint;
    static RT1019_I2S_DL_MASK: c_uint;
    static RT1019_SEL_FIFO_MASK: c_uint;
    static RT1019_SYS_DIV_DA_FIL_MASK: c_uint;
    static RT1019_SYS_DA_OSR_MASK: c_uint;
    static RT1019_ASRC_256FS_MASK: c_uint;
    static RT1019_SEL_CLK_CAL_MASK: c_uint;
    static RT1019_TDM_BCLK_INV: c_uint;
    static RT1019_I2S_DF_LEFT: c_uint;
    static RT1019_I2S_DF_PCM_A_R: c_uint;
    static RT1019_I2S_DF_PCM_B_R: c_uint;
    static RT1019_I2S_DF_MASK: c_uint;
    static RT1019_TDM_BCLK_MASK: c_uint;
    static RT1019_SCLK_S_BCLK: c_int;
    static RT1019_SCLK_S_PLL: c_int;
    static RT1019_CLK_SYS_PRE_SEL_BCLK: c_uint;
    static RT1019_CLK_SYS_PRE_SEL_PLL: c_uint;
    static RT1019_CLK_SYS_PRE_SEL_MASK: c_uint;
    static RT1019_PLL_S_BCLK: c_int;
    static RT1019_PLL_S_RC25M: c_int;
    static RT1019_PLL_SRC_MASK: c_uint;
    static RT1019_PLL_SRC_SEL_BCLK: c_uint;
    static RT1019_PLL_SRC_SEL_RC: c_uint;
    static RT1019_AUTO_BITS_SEL_MASK: c_uint;
    static RT1019_AUTO_CLK_SEL_MASK: c_uint;
    static RT1019_AUTO_BITS_SEL_MANU: c_uint;
    static RT1019_AUTO_CLK_SEL_MANU: c_uint;
    static RT1019_PLL_M_MASK: c_uint;
    static RT1019_PLL_M_BP_MASK: c_uint;
    static RT1019_PLL_Q_8_8_MASK: c_uint;
    static RT1019_PLL_M_SFT: c_uint;
    static RT1019_PLL_M_BP_SFT: c_uint;
    static RT1019_PLL_Q_7_0_MASK: c_uint;
    static RT1019_PLL_K_MASK: c_uint;
    static RT1019_I2S_TX_4CH: c_uint;
    static RT1019_I2S_TX_6CH: c_uint;
    static RT1019_I2S_TX_8CH: c_uint;
    static RT1019_TDM_CL_20: c_uint;
    static RT1019_TDM_CL_24: c_uint;
    static RT1019_TDM_CL_32: c_uint;
    static RT1019_TDM_CL_8: c_uint;
    static RT1019_TDM_I2S_TX_L_DAC1_1_MASK: c_uint;
    static RT1019_TDM_I2S_TX_R_DAC1_1_MASK: c_uint;
    static RT1019_TDM_I2S_TX_L_DAC1_1_SFT: c_uint;
    static RT1019_TDM_I2S_TX_R_DAC1_1_SFT: c_uint;
    static RT1019_TDM_CL_MASK: c_uint;
    static RT1019_I2S_CH_TX_MASK: c_uint;
    static RT1019_DEVICE_ID_VAL: c_uint;
    static RT1019_DEVICE_ID_VAL2: c_uint;
}

unsafe extern "C" {
    static SND_SOC_DAPM_PRE_PMU: c_int;
    static SND_SOC_DAPM_POST_PMD: c_int;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S8: c_uint;
    static REGCACHE_MAPLE: c_uint;
    static GFP_KERNEL: c_uint;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub component: *mut snd_soc_component,
    pub id: c_int,
}

#[repr(C)]
pub struct rt1019_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sysclk: c_uint,
    pub sysclk_src: c_int,
    pub lrck: c_uint,
    pub bclk: c_uint,
    pub pll_in: c_uint,
    pub pll_out: c_uint,
    pub pll_src: c_int,
}

#[repr(C)]
pub struct rl6231_pll_code {
    pub m_bp: c_int,
    pub m_code: c_int,
    pub n_code: c_int,
    pub k_code: c_int,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_widget_def {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_pll:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int, c_uint, c_uint) -> c_int>,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint, c_uint, c_int, c_int) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget_def,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub use_single_read: bool,
    pub use_single_write: bool,
    pub max_register: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 9],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn rl6231_get_clk_info(sysclk: c_uint, lrck: c_uint) -> c_int;
    fn rl6231_pll_calc(freq_in: c_uint, freq_out: c_uint, pll_code: *mut rl6231_pll_code)
        -> c_int;
    fn hweight_long(word: c_ulong) -> c_uint;
    fn __ffs(word: c_ulong) -> c_int;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id;
    fn ACPI_PTR(ptr: *const acpi_device_id) -> *const acpi_device_id;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

static RT1019_REG: [reg_default; 24] = [
    reg_default { reg: 0x0000, def: 0x00 },
    reg_default { reg: 0x0011, def: 0x04 },
    reg_default { reg: 0x0013, def: 0x00 },
    reg_default { reg: 0x0019, def: 0x30 },
    reg_default { reg: 0x001b, def: 0x01 },
    reg_default { reg: 0x005c, def: 0x00 },
    reg_default { reg: 0x005e, def: 0x10 },
    reg_default { reg: 0x005f, def: 0xec },
    reg_default { reg: 0x0061, def: 0x10 },
    reg_default { reg: 0x0062, def: 0x19 },
    reg_default { reg: 0x0066, def: 0x08 },
    reg_default { reg: 0x0100, def: 0x80 },
    reg_default { reg: 0x0100, def: 0x51 },
    reg_default { reg: 0x0102, def: 0x23 },
    reg_default { reg: 0x0311, def: 0x00 },
    reg_default { reg: 0x0312, def: 0x3e },
    reg_default { reg: 0x0313, def: 0x86 },
    reg_default { reg: 0x0400, def: 0x03 },
    reg_default { reg: 0x0401, def: 0x02 },
    reg_default { reg: 0x0402, def: 0x01 },
    reg_default { reg: 0x0504, def: 0xff },
    reg_default { reg: 0x0505, def: 0x24 },
    reg_default { reg: 0x0b00, def: 0x50 },
    reg_default { reg: 0x0b01, def: 0xc3 },
];

unsafe extern "C" fn rt1019_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    unsafe {
        match reg {
            x if x == RT1019_PWR_STRP_2
                || x == RT1019_VER_ID
                || x == RT1019_VEND_ID_1
                || x == RT1019_VEND_ID_2
                || x == RT1019_DEV_ID_1
                || x == RT1019_DEV_ID_2 =>
            {
                true
            }
            _ => false,
        }
    }
}

unsafe extern "C" fn rt1019_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    unsafe {
        match reg {
            x if x == RT1019_RESET
                || x == RT1019_IDS_CTRL
                || x == RT1019_ASEL_CTRL
                || x == RT1019_PWR_STRP_2
                || x == RT1019_BEEP_TONE
                || x == RT1019_VER_ID
                || x == RT1019_VEND_ID_1
                || x == RT1019_VEND_ID_2
                || x == RT1019_DEV_ID_1
                || x == RT1019_DEV_ID_2
                || x == RT1019_SDB_CTRL
                || x == RT1019_CLK_TREE_1
                || x == RT1019_CLK_TREE_2
                || x == RT1019_CLK_TREE_3
                || x == RT1019_PLL_1
                || x == RT1019_PLL_2
                || x == RT1019_PLL_3
                || x == RT1019_TDM_1
                || x == RT1019_TDM_2
                || x == RT1019_TDM_3
                || x == RT1019_DMIX_MONO_1
                || x == RT1019_DMIX_MONO_2
                || x == RT1019_BEEP_1
                || x == RT1019_BEEP_2 =>
            {
                true
            }
            _ => false,
        }
    }
}

/* static const DECLARE_TLV_DB_SCALE(dac_vol_tlv, -9525, 75, 0); */
static DAC_VOL_TLV: [c_uint; 4] = [0, (-9525i32) as c_uint, 75, 0];

static RT1019_DIN_SOURCE_SELECT: [*const c_char; 3] = [
    b"Left\0".as_ptr() as *const c_char,
    b"Right\0".as_ptr() as *const c_char,
    b"Left + Right average\0".as_ptr() as *const c_char,
];

/* static SOC_ENUM_SINGLE_DECL(rt1019_mono_lr_sel, RT1019_IDS_CTRL, 0,
 *     rt1019_din_source_select);
 */
static RT1019_MONO_LR_SEL: c_uint = 0;

/* These control and widget entries are produced by ASoC C macros in the
 * original source: SOC_SINGLE_TLV, SOC_ENUM, SND_SOC_DAPM_AIF_IN,
 * SND_SOC_DAPM_DAC_E, and SND_SOC_DAPM_OUTPUT.
 */
static RT1019_SND_CONTROLS: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn r1019_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    unsafe {
        let component = snd_soc_dapm_to_component((*w).dapm);

        if event == SND_SOC_DAPM_PRE_PMU {
            snd_soc_component_write(component, RT1019_SDB_CTRL, 0xb);
        } else if event == SND_SOC_DAPM_POST_PMD {
            snd_soc_component_write(component, RT1019_SDB_CTRL, 0xa);
        }

        0
    }
}

static RT1019_DAPM_WIDGETS: [snd_soc_dapm_widget_def; 3] = [
    snd_soc_dapm_widget_def { _private: [] },
    snd_soc_dapm_widget_def { _private: [] },
    snd_soc_dapm_widget_def { _private: [] },
];

static RT1019_DAPM_ROUTES: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"DAC\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"AIFRX\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"SPO\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn rt1019_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let component = (*dai).component;
        let rt1019 = snd_soc_component_get_drvdata(component) as *mut rt1019_priv;
        let mut val_len: c_uint = 0;
        let mut sys_div_da_filter: c_uint = 0;
        let mut sys_dac_osr: c_uint = 0;
        let mut sys_fifo_clk: c_uint = 0;
        let mut sys_clk_cal: c_uint = 0;
        let mut sys_asrc_in: c_uint = 0;

        (*rt1019).lrck = params_rate(params);
        let pre_div = rl6231_get_clk_info((*rt1019).sysclk, (*rt1019).lrck);
        if pre_div < 0 {
            dev_err((*component).dev, b"Unsupported clock setting\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }

        let frame_size = snd_soc_params_to_frame_size(params);
        if frame_size < 0 {
            dev_err(
                (*component).dev,
                b"Unsupported frame size: %d\n\0".as_ptr() as *const c_char,
                frame_size,
            );
            return -EINVAL;
        }

        let bclk_ms = (frame_size > 32) as c_int;
        (*rt1019).bclk = (*rt1019)
            .lrck
            .wrapping_mul(32u32.wrapping_shl(bclk_ms as u32));

        dev_dbg(
            (*dai).dev,
            b"bclk is %dHz and lrck is %dHz\n\0".as_ptr() as *const c_char,
            (*rt1019).bclk,
            (*rt1019).lrck,
        );
        dev_dbg(
            (*dai).dev,
            b"bclk_ms is %d and pre_div is %d for iis %d\n\0".as_ptr() as *const c_char,
            bclk_ms,
            pre_div,
            (*dai).id,
        );

        match pre_div {
            0 => {
                sys_div_da_filter = RT1019_SYS_DIV_DA_FIL_DIV1;
                sys_dac_osr = RT1019_SYS_DA_OSR_DIV1;
                sys_asrc_in = RT1019_ASRC_256FS_DIV1;
                sys_fifo_clk = RT1019_SEL_FIFO_DIV1;
                sys_clk_cal = RT1019_SEL_CLK_CAL_DIV1;
            }
            1 => {
                sys_div_da_filter = RT1019_SYS_DIV_DA_FIL_DIV2;
                sys_dac_osr = RT1019_SYS_DA_OSR_DIV2;
                sys_asrc_in = RT1019_ASRC_256FS_DIV2;
                sys_fifo_clk = RT1019_SEL_FIFO_DIV2;
                sys_clk_cal = RT1019_SEL_CLK_CAL_DIV2;
            }
            3 => {
                sys_div_da_filter = RT1019_SYS_DIV_DA_FIL_DIV4;
                sys_dac_osr = RT1019_SYS_DA_OSR_DIV4;
                sys_asrc_in = RT1019_ASRC_256FS_DIV4;
                sys_fifo_clk = RT1019_SEL_FIFO_DIV4;
                sys_clk_cal = RT1019_SEL_CLK_CAL_DIV4;
            }
            _ => return -EINVAL,
        }

        match params_width(params) {
            16 => {}
            20 => val_len = RT1019_I2S_DL_20,
            24 => val_len = RT1019_I2S_DL_24,
            32 => val_len = RT1019_I2S_DL_32,
            8 => val_len = RT1019_I2S_DL_8,
            _ => return -EINVAL,
        }

        snd_soc_component_update_bits(component, RT1019_TDM_2, RT1019_I2S_DL_MASK, val_len);
        snd_soc_component_update_bits(component, RT1019_CLK_TREE_1, RT1019_SEL_FIFO_MASK, sys_fifo_clk);
        snd_soc_component_update_bits(
            component,
            RT1019_CLK_TREE_2,
            RT1019_SYS_DIV_DA_FIL_MASK | RT1019_SYS_DA_OSR_MASK | RT1019_ASRC_256FS_MASK,
            sys_div_da_filter | sys_dac_osr | sys_asrc_in,
        );
        snd_soc_component_update_bits(
            component,
            RT1019_CLK_TREE_3,
            RT1019_SEL_CLK_CAL_MASK,
            sys_clk_cal,
        );

        0
    }
}

unsafe extern "C" fn rt1019_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    unsafe {
        let component = (*dai).component;
        let mut reg_val: c_uint = 0;
        let mut reg_val2: c_uint = 0;

        match fmt & SND_SOC_DAIFMT_INV_MASK {
            x if x == SND_SOC_DAIFMT_NB_NF => {}
            x if x == SND_SOC_DAIFMT_IB_NF => reg_val2 |= RT1019_TDM_BCLK_INV,
            _ => return -EINVAL,
        }

        match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            x if x == SND_SOC_DAIFMT_I2S => {}
            x if x == SND_SOC_DAIFMT_LEFT_J => reg_val |= RT1019_I2S_DF_LEFT,
            x if x == SND_SOC_DAIFMT_DSP_A => reg_val |= RT1019_I2S_DF_PCM_A_R,
            x if x == SND_SOC_DAIFMT_DSP_B => reg_val |= RT1019_I2S_DF_PCM_B_R,
            _ => return -EINVAL,
        }

        snd_soc_component_update_bits(component, RT1019_TDM_2, RT1019_I2S_DF_MASK, reg_val);
        snd_soc_component_update_bits(component, RT1019_TDM_1, RT1019_TDM_BCLK_MASK, reg_val2);

        0
    }
}

unsafe extern "C" fn rt1019_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    unsafe {
        let component = (*dai).component;
        let rt1019 = snd_soc_component_get_drvdata(component) as *mut rt1019_priv;
        let mut reg_val: c_uint = 0;

        if freq == (*rt1019).sysclk && clk_id == (*rt1019).sysclk_src {
            return 0;
        }

        if clk_id == RT1019_SCLK_S_BCLK {
            reg_val |= RT1019_CLK_SYS_PRE_SEL_BCLK;
        } else if clk_id == RT1019_SCLK_S_PLL {
            reg_val |= RT1019_CLK_SYS_PRE_SEL_PLL;
        } else {
            dev_err(
                (*component).dev,
                b"Invalid clock id (%d)\n\0".as_ptr() as *const c_char,
                clk_id,
            );
            return -EINVAL;
        }

        (*rt1019).sysclk = freq;
        (*rt1019).sysclk_src = clk_id;

        dev_dbg(
            (*dai).dev,
            b"Sysclk is %dHz and clock id is %d\n\0".as_ptr() as *const c_char,
            freq,
            clk_id,
        );

        snd_soc_component_update_bits(
            component,
            RT1019_CLK_TREE_1,
            RT1019_CLK_SYS_PRE_SEL_MASK,
            reg_val,
        );

        0
    }
}

unsafe extern "C" fn rt1019_set_dai_pll(
    dai: *mut snd_soc_dai,
    _pll_id: c_int,
    source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    unsafe {
        let component = (*dai).component;
        let rt1019 = snd_soc_component_get_drvdata(component) as *mut rt1019_priv;
        let mut pll_code = rl6231_pll_code {
            m_bp: 0,
            m_code: 0,
            n_code: 0,
            k_code: 0,
        };

        if freq_in == 0 || freq_out == 0 {
            dev_dbg((*component).dev, b"PLL disabled\n\0".as_ptr() as *const c_char);
            (*rt1019).pll_in = 0;
            (*rt1019).pll_out = 0;
            return 0;
        }

        if source == (*rt1019).pll_src
            && freq_in == (*rt1019).pll_in
            && freq_out == (*rt1019).pll_out
        {
            return 0;
        }

        if source == RT1019_PLL_S_BCLK {
            snd_soc_component_update_bits(
                component,
                RT1019_CLK_TREE_1,
                RT1019_PLL_SRC_MASK,
                RT1019_PLL_SRC_SEL_BCLK,
            );
        } else if source == RT1019_PLL_S_RC25M {
            snd_soc_component_update_bits(
                component,
                RT1019_CLK_TREE_1,
                RT1019_PLL_SRC_MASK,
                RT1019_PLL_SRC_SEL_RC,
            );
        } else {
            dev_err(
                (*component).dev,
                b"Unknown PLL source %d\n\0".as_ptr() as *const c_char,
                source,
            );
            return -EINVAL;
        }

        let ret = rl6231_pll_calc(freq_in, freq_out, &mut pll_code);
        if ret < 0 {
            dev_err(
                (*component).dev,
                b"Unsupported input clock %d\n\0".as_ptr() as *const c_char,
                freq_in,
            );
            return ret;
        }

        dev_dbg(
            (*component).dev,
            b"bypass=%d m=%d n=%d k=%d\n\0".as_ptr() as *const c_char,
            pll_code.m_bp,
            if pll_code.m_bp != 0 { 0 } else { pll_code.m_code },
            pll_code.n_code,
            pll_code.k_code,
        );

        snd_soc_component_update_bits(
            component,
            RT1019_PWR_STRP_2,
            RT1019_AUTO_BITS_SEL_MASK | RT1019_AUTO_CLK_SEL_MASK,
            RT1019_AUTO_BITS_SEL_MANU | RT1019_AUTO_CLK_SEL_MANU,
        );
        snd_soc_component_update_bits(
            component,
            RT1019_PLL_1,
            RT1019_PLL_M_MASK | RT1019_PLL_M_BP_MASK | RT1019_PLL_Q_8_8_MASK,
            (((if pll_code.m_bp != 0 { 0 } else { pll_code.m_code }) as c_uint)
                << RT1019_PLL_M_SFT)
                | ((pll_code.m_bp as c_uint) << RT1019_PLL_M_BP_SFT)
                | (((pll_code.n_code as c_uint) >> 8) & RT1019_PLL_Q_8_8_MASK),
        );
        snd_soc_component_update_bits(
            component,
            RT1019_PLL_2,
            RT1019_PLL_Q_7_0_MASK,
            (pll_code.n_code as c_uint) & RT1019_PLL_Q_7_0_MASK,
        );
        snd_soc_component_update_bits(
            component,
            RT1019_PLL_3,
            RT1019_PLL_K_MASK,
            pll_code.k_code as c_uint,
        );

        (*rt1019).pll_in = freq_in;
        (*rt1019).pll_out = freq_out;
        (*rt1019).pll_src = source;

        0
    }
}

unsafe extern "C" fn rt1019_set_tdm_slot(
    dai: *mut snd_soc_dai,
    _tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    unsafe {
        let component = (*dai).component;
        let mut cn: c_uint = 0;
        let mut cl: c_uint = 0;
        let mut ret: c_int = 0;

        match slots {
            4 => cn = RT1019_I2S_TX_4CH,
            6 => cn = RT1019_I2S_TX_6CH,
            8 => cn = RT1019_I2S_TX_8CH,
            2 => {}
            _ => return -EINVAL,
        }

        match slot_width {
            20 => cl = RT1019_TDM_CL_20,
            24 => cl = RT1019_TDM_CL_24,
            32 => cl = RT1019_TDM_CL_32,
            8 => cl = RT1019_TDM_CL_8,
            16 => {}
            _ => return -EINVAL,
        }

        /* Rx slot configuration */
        let rx_slotnum = hweight_long(rx_mask as c_ulong);
        if rx_slotnum != 1 {
            ret = -EINVAL;
            dev_err(
                (*component).dev,
                b"too many rx slots or zero slot\n\0".as_ptr() as *const c_char,
            );
        } else {
            /* This is an assumption that the system sends stereo audio to the
             * amplifier typically. And the stereo audio is placed in slot 0/2/4/6
             * as the starting slot. The users could select the channel from
             * L/R/L+R by "Mono LR Select" control.
             */
            let first_bit = __ffs(rx_mask as c_ulong);
            match first_bit {
                0 | 2 | 4 | 6 => {
                    snd_soc_component_update_bits(
                        component,
                        RT1019_TDM_3,
                        RT1019_TDM_I2S_TX_L_DAC1_1_MASK
                            | RT1019_TDM_I2S_TX_R_DAC1_1_MASK,
                        ((first_bit as c_uint) << RT1019_TDM_I2S_TX_L_DAC1_1_SFT)
                            | (((first_bit + 1) as c_uint)
                                << RT1019_TDM_I2S_TX_R_DAC1_1_SFT),
                    );
                }
                1 | 3 | 5 | 7 => {
                    snd_soc_component_update_bits(
                        component,
                        RT1019_TDM_3,
                        RT1019_TDM_I2S_TX_L_DAC1_1_MASK
                            | RT1019_TDM_I2S_TX_R_DAC1_1_MASK,
                        (((first_bit - 1) as c_uint) << RT1019_TDM_I2S_TX_L_DAC1_1_SFT)
                            | ((first_bit as c_uint) << RT1019_TDM_I2S_TX_R_DAC1_1_SFT),
                    );
                }
                _ => {
                    ret = -EINVAL;
                }
            }
        }

        if ret == 0 {
            snd_soc_component_update_bits(component, RT1019_TDM_1, RT1019_TDM_CL_MASK, cl);
            snd_soc_component_update_bits(component, RT1019_TDM_2, RT1019_I2S_CH_TX_MASK, cn);
        }

        ret
    }
}

unsafe extern "C" fn rt1019_probe(component: *mut snd_soc_component) -> c_int {
    unsafe {
        let rt1019 = snd_soc_component_get_drvdata(component) as *mut rt1019_priv;

        (*rt1019).component = component;
        snd_soc_component_write(component, RT1019_SDB_CTRL, 0xa);

        0
    }
}

unsafe fn rt1019_stereo_rates() -> c_uint {
    unsafe { SNDRV_PCM_RATE_8000_192000 }
}

unsafe fn rt1019_formats() -> c_uint {
    unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8 }
}

static RT1019_AIF_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt1019_hw_params),
    set_fmt: Some(rt1019_set_dai_fmt),
    set_sysclk: Some(rt1019_set_dai_sysclk),
    set_pll: Some(rt1019_set_dai_pll),
    set_tdm_slot: Some(rt1019_set_tdm_slot),
};

static mut RT1019_DAI: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: b"rt1019-aif\0".as_ptr() as *const c_char,
    id: 0,
    playback: snd_soc_pcm_stream {
        stream_name: b"AIF Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: 0,
        formats: 0,
    },
    ops: &RT1019_AIF_DAI_OPS,
}];

static SOC_COMPONENT_DEV_RT1019: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt1019_probe),
    controls: RT1019_SND_CONTROLS.as_ptr(),
    num_controls: array_size(&RT1019_SND_CONTROLS),
    dapm_widgets: RT1019_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: array_size(&RT1019_DAPM_WIDGETS),
    dapm_routes: RT1019_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: array_size(&RT1019_DAPM_ROUTES),
    endianness: 1,
};

static RT1019_REGMAP: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 8,
    use_single_read: true,
    use_single_write: true,
    max_register: 0,
    volatile_reg: Some(rt1019_volatile_register),
    readable_reg: Some(rt1019_readable_register),
    cache_type: 0,
    reg_defaults: RT1019_REG.as_ptr(),
    num_reg_defaults: array_size(&RT1019_REG),
};

static RT1019_I2C_ID: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'r' as c_char,
            b't' as c_char,
            b'1' as c_char,
            b'0' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    i2c_device_id { name: [0; 20] },
];
/* MODULE_DEVICE_TABLE(i2c, rt1019_i2c_id); */

static RT1019_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: b"realtek,rt1019\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, rt1019_of_match); */

/* CONFIG_ACPI conditional in the original source. */
static RT1019_ACPI_MATCH: [acpi_device_id; 2] = [
    acpi_device_id {
        id: [
            b'1' as c_char,
            b'0' as c_char,
            b'E' as c_char,
            b'C' as c_char,
            b'1' as c_char,
            b'0' as c_char,
            b'1' as c_char,
            b'9' as c_char,
            0,
        ],
    },
    acpi_device_id { id: [0; 9] },
];
/* MODULE_DEVICE_TABLE(acpi, rt1019_acpi_match); */

unsafe extern "C" fn rt1019_i2c_probe(i2c: *mut i2c_client) -> c_int {
    unsafe {
        let rt1019 = devm_kzalloc(
            &mut (*i2c).dev,
            core::mem::size_of::<rt1019_priv>(),
            GFP_KERNEL,
        ) as *mut rt1019_priv;
        if rt1019.is_null() {
            return -ENOMEM;
        }

        i2c_set_clientdata(i2c, rt1019 as *mut c_void);

        (*rt1019).regmap = devm_regmap_init_i2c(i2c, &RT1019_REGMAP);
        if IS_ERR((*rt1019).regmap as *const c_void) {
            let ret = PTR_ERR((*rt1019).regmap as *const c_void);
            dev_err(
                &mut (*i2c).dev,
                b"Failed to allocate register map: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        let mut val: c_uint = 0;
        let mut val2: c_uint = 0;
        regmap_read((*rt1019).regmap, RT1019_DEV_ID_1, &mut val);
        regmap_read((*rt1019).regmap, RT1019_DEV_ID_2, &mut val2);
        let dev_id = (val << 8) | val2;
        if dev_id != RT1019_DEVICE_ID_VAL && dev_id != RT1019_DEVICE_ID_VAL2 {
            dev_err(
                &mut (*i2c).dev,
                b"Device with ID register 0x%x is not rt1019\n\0".as_ptr() as *const c_char,
                dev_id,
            );
            return -ENODEV;
        }

        (*RT1019_DAI.as_mut_ptr()).playback.rates = rt1019_stereo_rates();
        (*RT1019_DAI.as_mut_ptr()).playback.formats = rt1019_formats();

        devm_snd_soc_register_component(
            &mut (*i2c).dev,
            &SOC_COMPONENT_DEV_RT1019,
            RT1019_DAI.as_mut_ptr(),
            array_size(&RT1019_DAI) as c_int,
        )
    }
}

static mut RT1019_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"rt1019\0".as_ptr() as *const c_char,
        of_match_table: RT1019_OF_MATCH.as_ptr(),
        acpi_match_table: RT1019_ACPI_MATCH.as_ptr(),
    },
    probe: Some(rt1019_i2c_probe),
    id_table: RT1019_I2C_ID.as_ptr(),
};

/* module_i2c_driver(rt1019_i2c_driver);
 *
 * MODULE_DESCRIPTION("ASoC RT1019 driver");
 * MODULE_AUTHOR("Jack Yu <jack.yu@realtek.com>");
 * MODULE_LICENSE("GPL v2");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
