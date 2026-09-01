// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) STMicroelectronics SA 2015
 * Authors: Arnaud Pouliquen <arnaud.pouliquen@st.com>
 *          for STMicroelectronics.
 */

/*
 * Translated from soc/sti/uniperif_player.c.
 * C includes were:
 * <linux/clk.h>, <linux/mfd/syscon.h>, <sound/asoundef.h>,
 * <sound/soc.h>, and "uniperif.h".
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/*
 * Some hardware-related definitions
 */

/* sys config registers definitions */
const SYS_CFG_AUDIO_GLUE: c_uint = 0xA4;

/*
 * Driver specific types.
 */

const UNIPERIF_PLAYER_CLK_ADJ_MIN: c_int = -999999;
const UNIPERIF_PLAYER_CLK_ADJ_MAX: c_int = 1000000;
const UNIPERIF_PLAYER_I2S_OUT: c_uint = 1; /* player id connected to I2S/TDM TX bus */

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub buffer_bytes_max: c_uint,
}

#[repr(C)]
pub struct snd_aes_iec958 {
    pub status: [c_uint; 24],
}

#[repr(C)]
pub struct stream_settings {
    pub iec958: snd_aes_iec958,
    pub encoding_mode: c_int,
}

#[repr(C)]
pub struct uniperif {
    pub irq_lock: c_void,
    pub ctrl_lock: c_void,
    pub substream: *mut snd_pcm_substream,
    pub state: c_int,
    pub underflow_enabled: c_int,
    pub dev: *mut device,
    pub clk_adj: c_int,
    pub clk: *mut clk,
    pub mclk: c_int,
    pub stream_settings: stream_settings,
    pub daifmt: c_uint,
    pub ver: c_int,
    pub r#type: c_int,
    pub clk_sel: *mut regmap_field,
    pub valid_sel: *mut regmap_field,
    pub id: c_uint,
    pub hw: *const snd_pcm_hardware,
    pub dai_ops: *const snd_soc_dai_ops,
    pub irq: c_int,
    pub num_ctrls: usize,
    pub snd_ctrls: *mut snd_kcontrol_new,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_int,
    pub format: c_int,
    pub channels: c_int,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub r#type: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_int,
    pub max: c_int,
    pub step: c_int,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub iec958: snd_aes_iec958,
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_int; 1],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dai_data {
    pub uni: *mut uniperif,
}

#[repr(C)]
pub struct sti_uniperiph_data {
    pub dai_data: dai_data,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub probe: Option<unsafe extern "C" fn() -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn() -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn() -> c_int>,
    pub set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
    pub set_tdm_slot: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_field {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_field {
    pub reg: c_uint,
    pub lsb: c_uint,
    pub msb: c_uint,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

type irqreturn_t = c_int;

extern "C" {
    static uni_tdm_hw: snd_pcm_hardware;

    fn snd_pcm_stream_lock(substream: *mut snd_pcm_substream);
    fn snd_pcm_stream_unlock(substream: *mut snd_pcm_substream);
    fn snd_pcm_stop(substream: *mut snd_pcm_substream, state: c_int);
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn snd_pcm_hw_rule_add(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, func: unsafe extern "C" fn(), private: *mut c_void, dep: c_int, sentinel: c_int) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut sti_uniperiph_data;
    fn clk_set_rate(clk: *mut clk, rate: c_int) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn of_clk_get(node: *mut device_node, index: c_int) -> *mut clk;
    fn regmap_field_write(field: *mut regmap_field, val: c_uint) -> c_int;
    fn syscon_regmap_lookup_by_phandle(node: *mut device_node, property: *const c_char) -> *mut regmap;
    fn devm_regmap_field_alloc(dev: *mut device, regmap: *mut regmap, field: reg_field) -> *mut regmap_field;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_uint, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn mutex_init(lock: *mut c_void);
    fn spin_lock_init(lock: *mut c_void);
    fn sti_uniperiph_get_unip_tdm_frame_size(player: *mut uniperif) -> c_int;
    fn sti_uniperiph_get_user_frame_size(runtime: *mut snd_pcm_runtime) -> c_int;
    fn sti_uniperiph_get_tdm_word_pos(player: *mut uniperif, word_pos: *mut c_uint);
    fn sti_uniperiph_fix_tdm_chan();
    fn sti_uniperiph_fix_tdm_format();
    fn sti_uniperiph_reset(player: *mut uniperif) -> c_int;
    fn sti_uniperiph_dai_probe() -> c_int;
    fn sti_uniperiph_dai_hw_params() -> c_int;
    fn sti_uniperiph_dai_set_fmt() -> c_int;
    fn sti_uniperiph_set_tdm_slot() -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

extern "C" {
    fn GET_UNIPERIF_ITS(player: *mut uniperif) -> c_uint;
    fn SET_UNIPERIF_ITS_BCLR(player: *mut uniperif, status: c_uint);
    fn UNIPERIF_ITS_FIFO_ERROR_MASK(player: *mut uniperif) -> c_uint;
    fn SET_UNIPERIF_ITM_BCLR_FIFO_ERROR(player: *mut uniperif);
    fn UNIPERIF_ITS_DMA_ERROR_MASK(player: *mut uniperif) -> c_uint;
    fn SET_UNIPERIF_ITM_BCLR_DMA_ERROR(player: *mut uniperif);
    fn UNIPERIF_ITM_UNDERFLOW_REC_DONE_MASK(player: *mut uniperif) -> c_uint;
    fn GET_UNIPERIF_STATUS_1_UNDERFLOW_DURATION(player: *mut uniperif) -> c_uint;
    fn SET_UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION(player: *mut uniperif);
    fn UNIPERIF_ITM_UNDERFLOW_REC_FAILED_MASK(player: *mut uniperif) -> c_uint;
    fn SET_UNIPERIF_USER_VALIDITY_VALIDITY_LR(player: *mut uniperif, val: c_int);
    fn SET_UNIPERIF_CHANNEL_STA_REGN(player: *mut uniperif, n: c_int, status: c_uint);
    fn SET_UNIPERIF_CONFIG_CHL_STS_UPDATE(player: *mut uniperif);
    fn SET_UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE(player: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_MEM_FMT_16_16(player: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_MEM_FMT_16_0(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_NBIT_32(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_NBIT_16(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_DATA_SIZE_16(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_DATA_SIZE_24(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_DATA_SIZE_32(player: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_PARITY_CNTR_BY_HW(player: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_CHANNEL_STA_CNTR_BY_HW(player: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_USER_DAT_CNTR_BY_HW(player: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_VALIDITY_DAT_CNTR_BY_HW(player: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_SPDIF_SW_CTRL_DISABLE(player: *mut uniperif);
    fn SET_UNIPERIF_CTRL_ZERO_STUFF_HW(player: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_ONE_BIT_AUD_DISABLE(player: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_REPEAT_CHL_STS_ENABLE(player: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_SUBFRAME_SEL_SUBF1_SUBF0(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_ORDER_MSB(player: *mut uniperif);
    fn SET_UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_ON(player: *mut uniperif);
    fn SET_UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_OFF(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_NUM_CH(player: *mut uniperif, num: c_int);
    fn SET_UNIPERIF_CTRL_ROUNDING_OFF(player: *mut uniperif);
    fn SET_UNIPERIF_CTRL_DIVIDER(player: *mut uniperif, div: c_int);
    fn SET_UNIPERIF_CTRL_SPDIF_LAT_OFF(player: *mut uniperif);
    fn SET_UNIPERIF_CTRL_SPDIF_FMT_OFF(player: *mut uniperif);
    fn SET_UNIPERIF_CTRL_SPDIF_FMT_ON(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_ALIGN_LEFT(player: *mut uniperif);
    fn SET_UNIPERIF_TDM_ENABLE_TDM_ENABLE(player: *mut uniperif);
    fn SET_UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT(player: *mut uniperif, num: c_int);
    fn SET_UNIPERIF_TDM_WORD_POS_1_2(player: *mut uniperif, val: c_uint);
    fn SET_UNIPERIF_TDM_WORD_POS_3_4(player: *mut uniperif, val: c_uint);
    fn SET_UNIPERIF_TDM_WORD_POS_5_6(player: *mut uniperif, val: c_uint);
    fn SET_UNIPERIF_TDM_WORD_POS_7_8(player: *mut uniperif, val: c_uint);
    fn UNIPERIF_TYPE_IS_TDM(player: *mut uniperif) -> bool;
    fn UNIPERIF_TYPE_IS_IEC958(player: *mut uniperif) -> bool;
    fn UNIPERIF_CONFIG_DMA_TRIG_LIMIT_MASK(player: *mut uniperif) -> c_int;
    fn SET_UNIPERIF_CONFIG_DMA_TRIG_LIMIT(player: *mut uniperif, limit: c_int);
    fn SET_UNIPERIF_I2S_FMT_LR_POL_LOW(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_LR_POL_HIG(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_SCLK_EDGE_RISING(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_SCLK_EDGE_FALLING(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_PADDING_I2S_MODE(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_PADDING_SONY_MODE(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_ALIGN_RIGHT(player: *mut uniperif);
    fn SET_UNIPERIF_I2S_FMT_NO_OF_SAMPLES_TO_READ(player: *mut uniperif, num: c_int);
    fn SET_UNIPERIF_ITM_BSET_DMA_ERROR(player: *mut uniperif);
    fn SET_UNIPERIF_ITM_BSET_FIFO_ERROR(player: *mut uniperif);
    fn SET_UNIPERIF_ITM_BSET_UNDERFLOW_REC_DONE(player: *mut uniperif);
    fn SET_UNIPERIF_ITM_BSET_UNDERFLOW_REC_FAILED(player: *mut uniperif);
    fn SET_UNIPERIF_CTRL_OPERATION_PCM_DATA(player: *mut uniperif);
    fn SET_UNIPERIF_CTRL_OPERATION_OFF(player: *mut uniperif);
    fn GET_UNIPERIF_ITM(player: *mut uniperif) -> c_uint;
    fn SET_UNIPERIF_ITM_BCLR(player: *mut uniperif, mask: c_uint);
    fn SET_UNIPERIF_CONFIG_BACK_STALL_REQ_DISABLE(player: *mut uniperif);
    fn SET_UNIPERIF_CONFIG_IDLE_MOD_DISABLE(player: *mut uniperif);
}

const PAGE_SIZE: c_uint = 4096;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 0;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 1;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 2;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 3;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 4;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << 10;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 2;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EPERM: c_int = 1;
const EINVAL: c_int = 22;
const SNDRV_PCM_STATE_XRUN: c_int = 4;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
const IEC958_AES3_CON_FS_22050: c_uint = 0x4;
const IEC958_AES3_CON_FS_44100: c_uint = 0x0;
const IEC958_AES3_CON_FS_88200: c_uint = 0x8;
const IEC958_AES3_CON_FS_176400: c_uint = 0xc;
const IEC958_AES3_CON_FS_24000: c_uint = 0x6;
const IEC958_AES3_CON_FS_48000: c_uint = 0x2;
const IEC958_AES3_CON_FS_96000: c_uint = 0xa;
const IEC958_AES3_CON_FS_192000: c_uint = 0xe;
const IEC958_AES3_CON_FS_32000: c_uint = 0x3;
const IEC958_AES3_CON_FS_NOTID: c_uint = 0x1;
const IEC958_AES0_NONAUDIO: c_uint = 0x02;
const UNIPERIF_IEC958_ENCODING_MODE_ENCODED: c_int = 1;
const UNIPERIF_IEC958_ENCODING_MODE_PCM: c_int = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 2;
const SND_SOC_DAIFMT_RIGHT_J: c_uint = 3;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_NB_IF: c_uint = 0x0100;
const SND_SOC_DAIFMT_IB_NF: c_uint = 0x0200;
const SND_SOC_DAIFMT_IB_IF: c_uint = 0x0300;
const SND_SOC_CLOCK_IN: c_int = 0;
const WORD_1_2: usize = 0;
const WORD_3_4: usize = 1;
const WORD_5_6: usize = 2;
const WORD_7_8: usize = 3;
const SNDRV_CTL_ELEM_TYPE_IEC958: c_int = 4;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_CTL_ELEM_IFACE_PCM: c_int = 2;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 10;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 5;
const SND_ST_UNIPERIF_TYPE_HDMI: c_int = 0;
const SND_ST_UNIPERIF_TYPE_PCM: c_int = 1;
const SND_ST_UNIPERIF_TYPE_SPDIF: c_int = 2;
const SND_ST_UNIPERIF_TYPE_TDM: c_int = 3;
const SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0: c_int = 0x10;
const UNIPERIF_FIFO_FRAMES: c_int = 4;
const UNIPERIF_FIFO_SIZE: c_int = 70;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 4;
const IRQF_SHARED: c_uint = 0x80;
const IEC958_AES1_CON_GENERAL: c_uint = 0x00;
const IEC958_AES2_CON_SOURCE_UNSPEC: c_uint = 0x00;
const IEC958_AES4_CON_MAX_WORDLEN_24: c_uint = 0x01;
const IEC958_AES4_CON_WORDLEN_24_20: c_uint = 0x05;
const UNIPERIF_STATE_STOPPED: c_int = 0;
const UNIPERIF_STATE_STARTED: c_int = 1;
const UNIPERIF_STATE_UNDERFLOW: c_int = 2;

const fn REG_FIELD(reg: c_uint, lsb: c_uint, msb: c_uint) -> reg_field {
    reg_field { reg, lsb, msb }
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe fn div64_u64(dividend: u64, divisor: u64) -> u64 {
    dividend / divisor
}

/*
 * Note: snd_pcm_hardware is linked to DMA controller but is declared here to
 * integrate  DAI_CPU capability in term of rate and supported channels
 */
static uni_player_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER |
        SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_MMAP |
        SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S16_LE,

    rates: SNDRV_PCM_RATE_CONTINUOUS,
    rate_min: 8000,
    rate_max: 192000,

    channels_min: 2,
    channels_max: 8,

    periods_min: 2,
    periods_max: 48,

    period_bytes_min: 128,
    period_bytes_max: 64 * PAGE_SIZE,
    buffer_bytes_max: 256 * PAGE_SIZE,
};

/*
 * uni_player_irq_handler
 * In case of error audio stream is stopped; stop action is protected via PCM
 * stream lock to avoid race condition with trigger callback.
 */
unsafe extern "C" fn uni_player_irq_handler(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let mut ret: irqreturn_t = IRQ_NONE;
    let player = dev_id as *mut uniperif;
    let status: c_uint;
    let tmp: c_uint;

    if (*player).substream.is_null() {
        return ret;
    }

    snd_pcm_stream_lock((*player).substream);
    if (*player).state == UNIPERIF_STATE_STOPPED {
        snd_pcm_stream_unlock((*player).substream);
        return ret;
    }

    /* Get interrupt status & clear them immediately */
    status = GET_UNIPERIF_ITS(player);
    SET_UNIPERIF_ITS_BCLR(player, status);

    /* Check for fifo error (underrun) */
    if status & UNIPERIF_ITS_FIFO_ERROR_MASK(player) != 0 {
        dev_err((*player).dev, b"FIFO underflow error detected\n\0".as_ptr() as *const c_char);

        /* Interrupt is just for information when underflow recovery */
        if (*player).underflow_enabled != 0 {
            /* Update state to underflow */
            (*player).state = UNIPERIF_STATE_UNDERFLOW;
        } else {
            /* Disable interrupt so doesn't continually fire */
            SET_UNIPERIF_ITM_BCLR_FIFO_ERROR(player);

            /* Stop the player */
            snd_pcm_stop((*player).substream, SNDRV_PCM_STATE_XRUN);
        }

        ret = IRQ_HANDLED;
    }

    /* Check for dma error (overrun) */
    if status & UNIPERIF_ITS_DMA_ERROR_MASK(player) != 0 {
        dev_err((*player).dev, b"DMA error detected\n\0".as_ptr() as *const c_char);

        /* Disable interrupt so doesn't continually fire */
        SET_UNIPERIF_ITM_BCLR_DMA_ERROR(player);

        /* Stop the player */
        snd_pcm_stop((*player).substream, SNDRV_PCM_STATE_XRUN);

        ret = IRQ_HANDLED;
    }

    /* Check for underflow recovery done */
    if status & UNIPERIF_ITM_UNDERFLOW_REC_DONE_MASK(player) != 0 {
        if (*player).underflow_enabled == 0 {
            dev_err((*player).dev, b"unexpected Underflow recovering\n\0".as_ptr() as *const c_char);
            ret = -EPERM;
            snd_pcm_stream_unlock((*player).substream);
            return ret;
        }
        /* Read the underflow recovery duration */
        tmp = GET_UNIPERIF_STATUS_1_UNDERFLOW_DURATION(player);
        dev_dbg((*player).dev, b"Underflow recovered (%d LR clocks max)\n\0".as_ptr() as *const c_char, tmp);

        /* Clear the underflow recovery duration */
        SET_UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION(player);

        /* Update state to started */
        (*player).state = UNIPERIF_STATE_STARTED;

        ret = IRQ_HANDLED;
    }

    /* Check if underflow recovery failed */
    if status & UNIPERIF_ITM_UNDERFLOW_REC_FAILED_MASK(player) != 0 {
        dev_err((*player).dev, b"Underflow recovery failed\n\0".as_ptr() as *const c_char);

        /* Stop the player */
        snd_pcm_stop((*player).substream, SNDRV_PCM_STATE_XRUN);

        ret = IRQ_HANDLED;
    }

    snd_pcm_stream_unlock((*player).substream);

    ret
}

unsafe fn uni_player_clk_set_rate(player: *mut uniperif, rate: c_ulong) -> c_int {
    let mut rate_adjusted: c_int;
    let rate_achieved: c_int;
    let mut delta: c_int;
    let mut ret: c_int;
    let mut adjustment: c_int = (*player).clk_adj;

    /*
     *             a
     * F = f + --------- * f = f + d
     *          1000000
     *
     *         a
     * d = --------- * f
     *      1000000
     *
     * where:
     *   f - nominal rate
     *   a - adjustment in ppm (parts per milion)
     *   F - rate to be set in synthesizer
     *   d - delta (difference) between f and F
     */
    if adjustment < 0 {
        /* div64_64 operates on unsigned values... */
        delta = -1;
        adjustment = -adjustment;
    } else {
        delta = 1;
    }
    /* 500000 ppm is 0.5, which is used to round up values */
    delta *= div64_u64(rate as u64 * adjustment as u64 + 500000, 1000000) as c_int;
    rate_adjusted = rate as c_int + delta;

    /* Adjusted rate should never be == 0 */
    if rate_adjusted == 0 {
        return -EINVAL;
    }

    ret = clk_set_rate((*player).clk, rate_adjusted);
    if ret < 0 {
        return ret;
    }

    rate_achieved = clk_get_rate((*player).clk);
    if rate_achieved == 0 {
        /* If value is 0 means that clock or parent not valid */
        return -EINVAL;
    }

    /*
     * Using ALSA's adjustment control, we can modify the rate to be up
     * to twice as much as requested, but no more
     */
    delta = rate_achieved - rate as c_int;
    if delta < 0 {
        /* div64_64 operates on unsigned values... */
        delta = -delta;
        adjustment = -1;
    } else {
        adjustment = 1;
    }
    /* Frequency/2 is added to round up result */
    adjustment *= div64_u64(delta as u64 * 1000000 + rate / 2, rate) as c_int;
    (*player).clk_adj = adjustment;
    0
}

unsafe fn uni_player_set_channel_status(player: *mut uniperif, runtime: *mut snd_pcm_runtime) {
    let mut n: c_int;
    let mut status: c_uint;

    /*
     * Some AVRs and TVs require the channel status to contain a correct
     * sampling frequency. If no sample rate is already specified, then
     * set one.
     */
    if !runtime.is_null() {
        (*player).stream_settings.iec958.status[3] = match (*runtime).rate {
            22050 => IEC958_AES3_CON_FS_22050,
            44100 => IEC958_AES3_CON_FS_44100,
            88200 => IEC958_AES3_CON_FS_88200,
            176400 => IEC958_AES3_CON_FS_176400,
            24000 => IEC958_AES3_CON_FS_24000,
            48000 => IEC958_AES3_CON_FS_48000,
            96000 => IEC958_AES3_CON_FS_96000,
            192000 => IEC958_AES3_CON_FS_192000,
            32000 => IEC958_AES3_CON_FS_32000,
            _ => {
                /* Mark as sampling frequency not indicated */
                IEC958_AES3_CON_FS_NOTID
            }
        };
    }

    /* Audio mode:
     * Use audio mode status to select PCM or encoded mode
     */
    if (*player).stream_settings.iec958.status[0] & IEC958_AES0_NONAUDIO != 0 {
        (*player).stream_settings.encoding_mode = UNIPERIF_IEC958_ENCODING_MODE_ENCODED;
    } else {
        (*player).stream_settings.encoding_mode = UNIPERIF_IEC958_ENCODING_MODE_PCM;
    }

    if (*player).stream_settings.encoding_mode == UNIPERIF_IEC958_ENCODING_MODE_PCM {
        /* Clear user validity bits */
        SET_UNIPERIF_USER_VALIDITY_VALIDITY_LR(player, 0);
    } else {
        /* Set user validity bits */
        SET_UNIPERIF_USER_VALIDITY_VALIDITY_LR(player, 1);
    }

    /* Program the new channel status */
    n = 0;
    while n < 6 {
        status = (*player).stream_settings.iec958.status[(0 + (n * 4)) as usize] & 0xf;
        status |= (*player).stream_settings.iec958.status[(1 + (n * 4)) as usize] << 8;
        status |= (*player).stream_settings.iec958.status[(2 + (n * 4)) as usize] << 16;
        status |= (*player).stream_settings.iec958.status[(3 + (n * 4)) as usize] << 24;
        SET_UNIPERIF_CHANNEL_STA_REGN(player, n, status);
        n += 1;
    }

    /* Update the channel status */
    if (*player).ver < SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 {
        SET_UNIPERIF_CONFIG_CHL_STS_UPDATE(player);
    } else {
        SET_UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE(player);
    }
}

unsafe fn uni_player_prepare_iec958(player: *mut uniperif, runtime: *mut snd_pcm_runtime) -> c_int {
    let clk_div: c_int = (*player).mclk / (*runtime).rate;

    /* Oversampling must be multiple of 128 as iec958 frame is 32-bits */
    if (clk_div % 128) != 0 || clk_div <= 0 {
        dev_err((*player).dev, b"%s: invalid clk_div %d\n\0".as_ptr() as *const c_char, b"uni_player_prepare_iec958\0".as_ptr(), clk_div);
        return -EINVAL;
    }

    match (*runtime).format {
        SNDRV_PCM_FORMAT_S16_LE => {
            /* 16/16 memory format */
            SET_UNIPERIF_CONFIG_MEM_FMT_16_16(player);
            /* 16-bits per sub-frame */
            SET_UNIPERIF_I2S_FMT_NBIT_32(player);
            /* Set 16-bit sample precision */
            SET_UNIPERIF_I2S_FMT_DATA_SIZE_16(player);
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            /* 16/0 memory format */
            SET_UNIPERIF_CONFIG_MEM_FMT_16_0(player);
            /* 32-bits per sub-frame */
            SET_UNIPERIF_I2S_FMT_NBIT_32(player);
            /* Set 24-bit sample precision */
            SET_UNIPERIF_I2S_FMT_DATA_SIZE_24(player);
        }
        _ => {
            dev_err((*player).dev, b"format not supported\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    /* Set parity to be calculated by the hardware */
    SET_UNIPERIF_CONFIG_PARITY_CNTR_BY_HW(player);
    /* Set channel status bits to be inserted by the hardware */
    SET_UNIPERIF_CONFIG_CHANNEL_STA_CNTR_BY_HW(player);
    /* Set user data bits to be inserted by the hardware */
    SET_UNIPERIF_CONFIG_USER_DAT_CNTR_BY_HW(player);
    /* Set validity bits to be inserted by the hardware */
    SET_UNIPERIF_CONFIG_VALIDITY_DAT_CNTR_BY_HW(player);
    /* Set full software control to disabled */
    SET_UNIPERIF_CONFIG_SPDIF_SW_CTRL_DISABLE(player);
    SET_UNIPERIF_CTRL_ZERO_STUFF_HW(player);

    /* Update the channel status */
    uni_player_set_channel_status(player, runtime);

    /* Clear the user validity user bits */
    SET_UNIPERIF_USER_VALIDITY_VALIDITY_LR(player, 0);
    /* Disable one-bit audio mode */
    SET_UNIPERIF_CONFIG_ONE_BIT_AUD_DISABLE(player);
    /* Enable consecutive frames repetition of Z preamble (not for HBRA) */
    SET_UNIPERIF_CONFIG_REPEAT_CHL_STS_ENABLE(player);
    /* Change to SUF0_SUBF1 and left/right channels swap! */
    SET_UNIPERIF_CONFIG_SUBFRAME_SEL_SUBF1_SUBF0(player);
    /* Set data output as MSB first */
    SET_UNIPERIF_I2S_FMT_ORDER_MSB(player);

    if (*player).stream_settings.encoding_mode == UNIPERIF_IEC958_ENCODING_MODE_ENCODED {
        SET_UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_ON(player);
    } else {
        SET_UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_OFF(player);
    }

    SET_UNIPERIF_I2S_FMT_NUM_CH(player, (*runtime).channels / 2);
    /* Set rounding to off */
    SET_UNIPERIF_CTRL_ROUNDING_OFF(player);
    /* Set clock divisor */
    SET_UNIPERIF_CTRL_DIVIDER(player, clk_div / 128);
    /* Set the spdif latency to not wait before starting player */
    SET_UNIPERIF_CTRL_SPDIF_LAT_OFF(player);

    /*
     * Ensure iec958 formatting is off. It will be enabled in function
     * uni_player_start() at the same time as the operation
     * mode is set to work around a silicon issue.
     */
    if (*player).ver < SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 {
        SET_UNIPERIF_CTRL_SPDIF_FMT_OFF(player);
    } else {
        SET_UNIPERIF_CTRL_SPDIF_FMT_ON(player);
    }

    0
}

unsafe fn uni_player_prepare_pcm(player: *mut uniperif, runtime: *mut snd_pcm_runtime) -> c_int {
    let output_frame_size: c_int;
    let slot_width: c_int;
    let clk_div: c_int;

    /* Force slot width to 32 in I2S mode (HW constraint) */
    if ((*player).daifmt & SND_SOC_DAIFMT_FORMAT_MASK) == SND_SOC_DAIFMT_I2S {
        slot_width = 32;
    } else {
        slot_width = snd_pcm_format_width((*runtime).format);
    }

    output_frame_size = slot_width * (*runtime).channels;
    clk_div = (*player).mclk / (*runtime).rate;
    /*
     * For 32 bits subframe clk_div must be a multiple of 128,
     * for 16 bits must be a multiple of 64
     */
    if slot_width == 32 && (clk_div % 128) != 0 {
        dev_err((*player).dev, b"%s: invalid clk_div\n\0".as_ptr() as *const c_char, b"uni_player_prepare_pcm\0".as_ptr());
        return -EINVAL;
    }

    if slot_width == 16 && (clk_div % 64) != 0 {
        dev_err((*player).dev, b"%s: invalid clk_div\n\0".as_ptr() as *const c_char, b"uni_player_prepare_pcm\0".as_ptr());
        return -EINVAL;
    }

    /*
     * Number of bits per subframe (which is one channel sample)
     * on output - Transfer 16 or 32 bits from FIFO
     */
    match slot_width {
        32 => {
            SET_UNIPERIF_I2S_FMT_NBIT_32(player);
            SET_UNIPERIF_I2S_FMT_DATA_SIZE_32(player);
        }
        16 => {
            SET_UNIPERIF_I2S_FMT_NBIT_16(player);
            SET_UNIPERIF_I2S_FMT_DATA_SIZE_16(player);
        }
        _ => {
            dev_err((*player).dev, b"subframe format not supported\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    /* Configure data memory format */
    match (*runtime).format {
        SNDRV_PCM_FORMAT_S16_LE => {
            /* One data word contains two samples */
            SET_UNIPERIF_CONFIG_MEM_FMT_16_16(player);
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            /*
             * Actually "16 bits/0 bits" means "32/28/24/20/18/16 bits
             * on the left than zeros (if less than 32 bytes)"... ;-)
             */
            SET_UNIPERIF_CONFIG_MEM_FMT_16_0(player);
        }
        _ => {
            dev_err((*player).dev, b"format not supported\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    /* Set rounding to off */
    SET_UNIPERIF_CTRL_ROUNDING_OFF(player);
    /* Set clock divisor */
    SET_UNIPERIF_CTRL_DIVIDER(player, clk_div / (2 * output_frame_size));

    /* Number of channelsmust be even*/
    if ((*runtime).channels % 2) != 0 || (*runtime).channels < 2 || (*runtime).channels > 10 {
        dev_err((*player).dev, b"%s: invalid nb of channels\n\0".as_ptr() as *const c_char, b"uni_player_prepare_pcm\0".as_ptr());
        return -EINVAL;
    }

    SET_UNIPERIF_I2S_FMT_NUM_CH(player, (*runtime).channels / 2);
    /* Set 1-bit audio format to disabled */
    SET_UNIPERIF_CONFIG_ONE_BIT_AUD_DISABLE(player);
    SET_UNIPERIF_I2S_FMT_ORDER_MSB(player);
    /* No iec958 formatting as outputting to DAC  */
    SET_UNIPERIF_CTRL_SPDIF_FMT_OFF(player);

    0
}

unsafe fn uni_player_prepare_tdm(player: *mut uniperif, runtime: *mut snd_pcm_runtime) -> c_int {
    let tdm_frame_size: c_int; /* unip tdm frame size in bytes */
    let user_frame_size: c_int; /* user tdm frame size in bytes */
    /* default unip TDM_WORD_POS_X_Y */
    let mut word_pos: [c_uint; 4] = [0x04060002, 0x0C0E080A, 0x14161012, 0x1C1E181A];
    let freq: c_int;
    let mut ret: c_int;

    tdm_frame_size = sti_uniperiph_get_unip_tdm_frame_size(player);
    user_frame_size = sti_uniperiph_get_user_frame_size(runtime);

    /* fix 16/0 format */
    SET_UNIPERIF_CONFIG_MEM_FMT_16_0(player);
    SET_UNIPERIF_I2S_FMT_DATA_SIZE_32(player);

    /* number of words inserted on the TDM line */
    SET_UNIPERIF_I2S_FMT_NUM_CH(player, user_frame_size / 4 / 2);

    SET_UNIPERIF_I2S_FMT_ORDER_MSB(player);
    SET_UNIPERIF_I2S_FMT_ALIGN_LEFT(player);

    /* Enable the tdm functionality */
    SET_UNIPERIF_TDM_ENABLE_TDM_ENABLE(player);

    /* number of 8 bits timeslots avail in unip tdm frame */
    SET_UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT(player, tdm_frame_size);

    /* set the timeslot allocation for words in FIFO */
    sti_uniperiph_get_tdm_word_pos(player, word_pos.as_mut_ptr());
    SET_UNIPERIF_TDM_WORD_POS_1_2(player, word_pos[WORD_1_2]);
    SET_UNIPERIF_TDM_WORD_POS_3_4(player, word_pos[WORD_3_4]);
    SET_UNIPERIF_TDM_WORD_POS_5_6(player, word_pos[WORD_5_6]);
    SET_UNIPERIF_TDM_WORD_POS_7_8(player, word_pos[WORD_7_8]);

    /* set unip clk rate (not done vai set_sysclk ops) */
    freq = (*runtime).rate * tdm_frame_size * 8;
    ret = uni_player_clk_set_rate(player, freq as c_ulong);
    if ret == 0 {
        (*player).mclk = freq;
    }

    0
}

/*
 * ALSA uniperipheral iec958 controls
 */
unsafe extern "C" fn uni_player_ctl_iec958_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).r#type = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;

    0
}

unsafe extern "C" fn uni_player_ctl_iec958_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let player = (*priv_).dai_data.uni;
    let iec958 = &mut (*player).stream_settings.iec958 as *mut snd_aes_iec958;

    (*ucontrol).value.iec958.status[0] = (*iec958).status[0];
    (*ucontrol).value.iec958.status[1] = (*iec958).status[1];
    (*ucontrol).value.iec958.status[2] = (*iec958).status[2];
    (*ucontrol).value.iec958.status[3] = (*iec958).status[3];
    0
}

unsafe extern "C" fn uni_player_ctl_iec958_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let player = (*priv_).dai_data.uni;
    let iec958 = &mut (*player).stream_settings.iec958 as *mut snd_aes_iec958;

    (*iec958).status[0] = (*ucontrol).value.iec958.status[0];
    (*iec958).status[1] = (*ucontrol).value.iec958.status[1];
    (*iec958).status[2] = (*ucontrol).value.iec958.status[2];
    (*iec958).status[3] = (*ucontrol).value.iec958.status[3];

    if !(*player).substream.is_null() && !(*(*player).substream).runtime.is_null() {
        uni_player_set_channel_status(player, (*(*player).substream).runtime);
    } else {
        uni_player_set_channel_status(player, core::ptr::null_mut());
    }

    0
}

static mut uni_player_iec958_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: b"IEC958 Playback Default\0".as_ptr() as *const c_char,
    info: Some(uni_player_ctl_iec958_info),
    get: Some(uni_player_ctl_iec958_get),
    put: Some(uni_player_ctl_iec958_put),
};

/*
 * uniperif rate adjustement control
 */
unsafe extern "C" fn snd_sti_clk_adjustment_info(_kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).r#type = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = UNIPERIF_PLAYER_CLK_ADJ_MIN;
    (*uinfo).value.integer.max = UNIPERIF_PLAYER_CLK_ADJ_MAX;
    (*uinfo).value.integer.step = 1;

    0
}

unsafe extern "C" fn snd_sti_clk_adjustment_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let player = (*priv_).dai_data.uni;

    (*ucontrol).value.integer.value[0] = (*player).clk_adj;

    0
}

unsafe extern "C" fn snd_sti_clk_adjustment_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol);
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let player = (*priv_).dai_data.uni;
    let mut ret: c_int = 0;

    if (*ucontrol).value.integer.value[0] < UNIPERIF_PLAYER_CLK_ADJ_MIN ||
        (*ucontrol).value.integer.value[0] > UNIPERIF_PLAYER_CLK_ADJ_MAX {
        return -EINVAL;
    }

    (*player).clk_adj = (*ucontrol).value.integer.value[0];

    if (*player).mclk != 0 {
        ret = uni_player_clk_set_rate(player, (*player).mclk as c_ulong);
    }

    ret
}

static mut uni_player_clk_adj_ctl: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_PCM,
    name: b"PCM Playback Oversampling Freq. Adjustment\0".as_ptr() as *const c_char,
    info: Some(snd_sti_clk_adjustment_info),
    get: Some(snd_sti_clk_adjustment_get),
    put: Some(snd_sti_clk_adjustment_put),
};

static mut snd_sti_pcm_ctl: [*mut snd_kcontrol_new; 1] = [
    unsafe { &mut uni_player_clk_adj_ctl as *mut snd_kcontrol_new },
];

static mut snd_sti_iec_ctl: [*mut snd_kcontrol_new; 2] = [
    unsafe { &mut uni_player_iec958_ctl as *mut snd_kcontrol_new },
    unsafe { &mut uni_player_clk_adj_ctl as *mut snd_kcontrol_new },
];

unsafe extern "C" fn uni_player_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let player = (*priv_).dai_data.uni;
    let ret: c_int;

    (*player).substream = substream;
    (*player).clk_adj = 0;

    if !UNIPERIF_TYPE_IS_TDM(player) {
        return 0;
    }

    /* refine hw constraint in tdm mode */
    ret = snd_pcm_hw_rule_add((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, sti_uniperiph_fix_tdm_chan, player as *mut c_void, SNDRV_PCM_HW_PARAM_CHANNELS, -1);
    if ret < 0 {
        return ret;
    }

    snd_pcm_hw_rule_add((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_FORMAT, sti_uniperiph_fix_tdm_format, player as *mut c_void, SNDRV_PCM_HW_PARAM_FORMAT, -1)
}

unsafe extern "C" fn uni_player_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let player = (*priv_).dai_data.uni;
    let ret: c_int;

    if UNIPERIF_TYPE_IS_TDM(player) || dir == SND_SOC_CLOCK_IN {
        return 0;
    }

    if clk_id != 0 {
        return -EINVAL;
    }

    ret = uni_player_clk_set_rate(player, freq as c_ulong);
    if ret == 0 {
        (*player).mclk = freq as c_int;
    }

    ret
}

unsafe extern "C" fn uni_player_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let player = (*priv_).dai_data.uni;
    let runtime = (*substream).runtime;
    let transfer_size: c_int;
    let trigger_limit: c_int;
    let ret: c_int;

    /* The player should be stopped */
    if (*player).state != UNIPERIF_STATE_STOPPED {
        dev_err((*player).dev, b"%s: invalid player state %d\n\0".as_ptr() as *const c_char, b"uni_player_prepare\0".as_ptr(), (*player).state);
        return -EINVAL;
    }

    /* Calculate transfer size (in fifo cells and bytes) for frame count */
    if (*player).r#type == SND_ST_UNIPERIF_TYPE_TDM {
        /* transfer size = user frame size (in 32 bits FIFO cell) */
        transfer_size = sti_uniperiph_get_user_frame_size(runtime) / 4;
    } else {
        transfer_size = (*runtime).channels * UNIPERIF_FIFO_FRAMES;
    }

    /* Calculate number of empty cells available before asserting DREQ */
    if (*player).ver < SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 {
        trigger_limit = UNIPERIF_FIFO_SIZE - transfer_size;
    } else {
        /*
         * Since SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0
         * FDMA_TRIGGER_LIMIT also controls when the state switches
         * from OFF or STANDBY to AUDIO DATA.
         */
        trigger_limit = transfer_size;
    }

    /* Trigger limit must be an even number */
    if ((!trigger_limit % 2) != 0) || (trigger_limit != 1 && transfer_size % 2 != 0) ||
        trigger_limit > UNIPERIF_CONFIG_DMA_TRIG_LIMIT_MASK(player) {
        dev_err((*player).dev, b"invalid trigger limit %d\n\0".as_ptr() as *const c_char, trigger_limit);
        return -EINVAL;
    }

    SET_UNIPERIF_CONFIG_DMA_TRIG_LIMIT(player, trigger_limit);

    /* Uniperipheral setup depends on player type */
    ret = match (*player).r#type {
        SND_ST_UNIPERIF_TYPE_HDMI => uni_player_prepare_iec958(player, runtime),
        SND_ST_UNIPERIF_TYPE_PCM => uni_player_prepare_pcm(player, runtime),
        SND_ST_UNIPERIF_TYPE_SPDIF => uni_player_prepare_iec958(player, runtime),
        SND_ST_UNIPERIF_TYPE_TDM => uni_player_prepare_tdm(player, runtime),
        _ => {
            dev_err((*player).dev, b"invalid player type\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    };

    if ret != 0 {
        return ret;
    }

    match (*player).daifmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {
            SET_UNIPERIF_I2S_FMT_LR_POL_LOW(player);
            SET_UNIPERIF_I2S_FMT_SCLK_EDGE_RISING(player);
        }
        SND_SOC_DAIFMT_NB_IF => {
            SET_UNIPERIF_I2S_FMT_LR_POL_HIG(player);
            SET_UNIPERIF_I2S_FMT_SCLK_EDGE_RISING(player);
        }
        SND_SOC_DAIFMT_IB_NF => {
            SET_UNIPERIF_I2S_FMT_LR_POL_LOW(player);
            SET_UNIPERIF_I2S_FMT_SCLK_EDGE_FALLING(player);
        }
        SND_SOC_DAIFMT_IB_IF => {
            SET_UNIPERIF_I2S_FMT_LR_POL_HIG(player);
            SET_UNIPERIF_I2S_FMT_SCLK_EDGE_FALLING(player);
        }
        _ => {}
    }

    match (*player).daifmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            SET_UNIPERIF_I2S_FMT_ALIGN_LEFT(player);
            SET_UNIPERIF_I2S_FMT_PADDING_I2S_MODE(player);
        }
        SND_SOC_DAIFMT_LEFT_J => {
            SET_UNIPERIF_I2S_FMT_ALIGN_LEFT(player);
            SET_UNIPERIF_I2S_FMT_PADDING_SONY_MODE(player);
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            SET_UNIPERIF_I2S_FMT_ALIGN_RIGHT(player);
            SET_UNIPERIF_I2S_FMT_PADDING_SONY_MODE(player);
        }
        _ => {
            dev_err((*player).dev, b"format not supported\n\0".as_ptr() as *const c_char);
            return -EINVAL;
        }
    }

    SET_UNIPERIF_I2S_FMT_NO_OF_SAMPLES_TO_READ(player, 0);

    sti_uniperiph_reset(player)
}

unsafe fn uni_player_start(player: *mut uniperif) -> c_int {
    let mut ret: c_int;

    /* The player should be stopped */
    if (*player).state != UNIPERIF_STATE_STOPPED {
        dev_err((*player).dev, b"%s: invalid player state\n\0".as_ptr() as *const c_char, b"uni_player_start\0".as_ptr());
        return -EINVAL;
    }

    ret = clk_prepare_enable((*player).clk);
    if ret != 0 {
        dev_err((*player).dev, b"%s: Failed to enable clock\n\0".as_ptr() as *const c_char, b"uni_player_start\0".as_ptr());
        return ret;
    }

    /* Clear any pending interrupts */
    SET_UNIPERIF_ITS_BCLR(player, GET_UNIPERIF_ITS(player));

    /* Set the interrupt mask */
    SET_UNIPERIF_ITM_BSET_DMA_ERROR(player);
    SET_UNIPERIF_ITM_BSET_FIFO_ERROR(player);

    /* Enable underflow recovery interrupts */
    if (*player).underflow_enabled != 0 {
        SET_UNIPERIF_ITM_BSET_UNDERFLOW_REC_DONE(player);
        SET_UNIPERIF_ITM_BSET_UNDERFLOW_REC_FAILED(player);
    }

    ret = sti_uniperiph_reset(player);
    if ret < 0 {
        clk_disable_unprepare((*player).clk);
        return ret;
    }

    /*
     * Does not use IEC61937 features of the uniperipheral hardware.
     * Instead it performs IEC61937 in software and inserts it directly
     * into the audio data stream. As such, when encoded mode is selected,
     * linear pcm mode is still used, but with the differences of the
     * channel status bits set for encoded mode and the validity bits set.
     */
    SET_UNIPERIF_CTRL_OPERATION_PCM_DATA(player);

    /*
     * If iec958 formatting is required for hdmi or spdif, then it must be
     * enabled after the operation mode is set. If set prior to this, it
     * will not take affect and hang the player.
     */
    if (*player).ver < SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 {
        if UNIPERIF_TYPE_IS_IEC958(player) {
            SET_UNIPERIF_CTRL_SPDIF_FMT_ON(player);
        }
    }

    /* Force channel status update (no update if clk disable) */
    if (*player).ver < SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 {
        SET_UNIPERIF_CONFIG_CHL_STS_UPDATE(player);
    } else {
        SET_UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE(player);
    }

    /* Update state to started */
    (*player).state = UNIPERIF_STATE_STARTED;

    0
}

unsafe fn uni_player_stop(player: *mut uniperif) -> c_int {
    let ret: c_int;

    /* The player should not be in stopped state */
    if (*player).state == UNIPERIF_STATE_STOPPED {
        dev_err((*player).dev, b"%s: invalid player state\n\0".as_ptr() as *const c_char, b"uni_player_stop\0".as_ptr());
        return -EINVAL;
    }

    /* Turn the player off */
    SET_UNIPERIF_CTRL_OPERATION_OFF(player);

    ret = sti_uniperiph_reset(player);
    if ret < 0 {
        return ret;
    }

    /* Disable interrupts */
    SET_UNIPERIF_ITM_BCLR(player, GET_UNIPERIF_ITM(player));

    /* Disable clock */
    clk_disable_unprepare((*player).clk);

    /* Update state to stopped and return */
    (*player).state = UNIPERIF_STATE_STOPPED;

    0
}

#[no_mangle]
pub unsafe extern "C" fn uni_player_resume(player: *mut uniperif) -> c_int {
    let mut ret: c_int;

    /* Select the frequency synthesizer clock */
    if !(*player).clk_sel.is_null() {
        ret = regmap_field_write((*player).clk_sel, 1);
        if ret != 0 {
            dev_err((*player).dev, b"%s: Failed to select freq synth clock\n\0".as_ptr() as *const c_char, b"uni_player_resume\0".as_ptr());
            return ret;
        }
    }

    SET_UNIPERIF_CONFIG_BACK_STALL_REQ_DISABLE(player);
    SET_UNIPERIF_CTRL_ROUNDING_OFF(player);
    SET_UNIPERIF_CTRL_SPDIF_LAT_OFF(player);
    SET_UNIPERIF_CONFIG_IDLE_MOD_DISABLE(player);

    0
}
/* EXPORT_SYMBOL_GPL(uni_player_resume); */

unsafe extern "C" fn uni_player_trigger(_substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let player = (*priv_).dai_data.uni;

    match cmd {
        SNDRV_PCM_TRIGGER_START => uni_player_start(player),
        SNDRV_PCM_TRIGGER_STOP => uni_player_stop(player),
        SNDRV_PCM_TRIGGER_RESUME => uni_player_resume(player),
        _ => -EINVAL,
    }
}

unsafe extern "C" fn uni_player_shutdown(_substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let priv_ = snd_soc_dai_get_drvdata(dai);
    let player = (*priv_).dai_data.uni;

    if (*player).state != UNIPERIF_STATE_STOPPED {
        /* Stop the player */
        uni_player_stop(player);
    }

    (*player).substream = core::ptr::null_mut();
}

unsafe fn uni_player_parse_dt_audio_glue(pdev: *mut platform_device, player: *mut uniperif) -> c_int {
    let node = (*pdev).dev.of_node;
    let regmap: *mut regmap;
    let regfield: [reg_field; 2] = [
        /* PCM_CLK_SEL */
        REG_FIELD(SYS_CFG_AUDIO_GLUE, 8 + (*player).id, 8 + (*player).id),
        /* PCMP_VALID_SEL */
        REG_FIELD(SYS_CFG_AUDIO_GLUE, 0, 1),
    ];

    regmap = syscon_regmap_lookup_by_phandle(node, b"st,syscfg\0".as_ptr() as *const c_char);

    if IS_ERR(regmap) {
        dev_err(&mut (*pdev).dev as *mut device, b"sti-audio-clk-glue syscf not found\n\0".as_ptr() as *const c_char);
        return PTR_ERR(regmap);
    }

    (*player).clk_sel = devm_regmap_field_alloc(&mut (*pdev).dev as *mut device, regmap, regfield[0]);
    if IS_ERR((*player).clk_sel) {
        return PTR_ERR((*player).clk_sel);
    }

    (*player).valid_sel = devm_regmap_field_alloc(&mut (*pdev).dev as *mut device, regmap, regfield[1]);
    if IS_ERR((*player).valid_sel) {
        return PTR_ERR((*player).valid_sel);
    }

    0
}

static uni_player_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(uni_player_startup),
    shutdown: Some(uni_player_shutdown),
    prepare: Some(uni_player_prepare),
    probe: Some(sti_uniperiph_dai_probe),
    trigger: Some(uni_player_trigger),
    hw_params: Some(sti_uniperiph_dai_hw_params),
    set_fmt: Some(sti_uniperiph_dai_set_fmt),
    set_sysclk: Some(uni_player_set_sysclk),
    set_tdm_slot: Some(sti_uniperiph_set_tdm_slot),
};

#[no_mangle]
pub unsafe extern "C" fn uni_player_init(pdev: *mut platform_device, player: *mut uniperif) -> c_int {
    let mut ret: c_int = 0;

    (*player).dev = &mut (*pdev).dev as *mut device;
    (*player).state = UNIPERIF_STATE_STOPPED;
    (*player).dai_ops = &uni_player_dai_ops;

    /* Get PCM_CLK_SEL & PCMP_VALID_SEL from audio-glue-ctrl SoC reg */
    ret = uni_player_parse_dt_audio_glue(pdev, player);

    if ret < 0 {
        dev_err((*player).dev, b"Failed to parse DeviceTree\n\0".as_ptr() as *const c_char);
        return ret;
    }

    /* Underflow recovery is only supported on later ip revisions */
    if (*player).ver >= SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 {
        (*player).underflow_enabled = 1;
    }

    if UNIPERIF_TYPE_IS_TDM(player) {
        (*player).hw = &uni_tdm_hw;
    } else {
        (*player).hw = &uni_player_pcm_hw;
    }

    /* Get uniperif resource */
    (*player).clk = of_clk_get((*pdev).dev.of_node, 0);
    if IS_ERR((*player).clk) {
        dev_err((*player).dev, b"Failed to get clock\n\0".as_ptr() as *const c_char);
        return PTR_ERR((*player).clk);
    }

    /* Select the frequency synthesizer clock */
    if !(*player).clk_sel.is_null() {
        ret = regmap_field_write((*player).clk_sel, 1);
        if ret != 0 {
            dev_err((*player).dev, b"%s: Failed to select freq synth clock\n\0".as_ptr() as *const c_char, b"uni_player_init\0".as_ptr());
            return ret;
        }
    }

    /* connect to I2S/TDM TX bus */
    if !(*player).valid_sel.is_null() && (*player).id == UNIPERIF_PLAYER_I2S_OUT {
        ret = regmap_field_write((*player).valid_sel, (*player).id);
        if ret != 0 {
            dev_err((*player).dev, b"%s: unable to connect to tdm bus\n\0".as_ptr() as *const c_char, b"uni_player_init\0".as_ptr());
            return ret;
        }
    }

    ret = devm_request_irq(&mut (*pdev).dev as *mut device, (*player).irq,
                           uni_player_irq_handler, IRQF_SHARED,
                           dev_name(&mut (*pdev).dev as *mut device), player as *mut c_void);
    if ret < 0 {
        dev_err((*player).dev, b"unable to request IRQ %d\n\0".as_ptr() as *const c_char, (*player).irq);
        return ret;
    }

    mutex_init(&mut (*player).ctrl_lock as *mut c_void);
    spin_lock_init(&mut (*player).irq_lock as *mut c_void);

    /* Ensure that disabled by default */
    SET_UNIPERIF_CONFIG_BACK_STALL_REQ_DISABLE(player);
    SET_UNIPERIF_CTRL_ROUNDING_OFF(player);
    SET_UNIPERIF_CTRL_SPDIF_LAT_OFF(player);
    SET_UNIPERIF_CONFIG_IDLE_MOD_DISABLE(player);

    if UNIPERIF_TYPE_IS_IEC958(player) {
        /* Set default iec958 status bits  */

        /* Consumer, PCM, copyright, 2ch, mode 0 */
        (*player).stream_settings.iec958.status[0] = 0x00;
        /* Broadcast reception category */
        (*player).stream_settings.iec958.status[1] = IEC958_AES1_CON_GENERAL;
        /* Do not take into account source or channel number */
        (*player).stream_settings.iec958.status[2] = IEC958_AES2_CON_SOURCE_UNSPEC;
        /* Sampling frequency not indicated */
        (*player).stream_settings.iec958.status[3] = IEC958_AES3_CON_FS_NOTID;
        /* Max sample word 24-bit, sample word length not indicated */
        (*player).stream_settings.iec958.status[4] =
            IEC958_AES4_CON_MAX_WORDLEN_24 | IEC958_AES4_CON_WORDLEN_24_20;

        (*player).num_ctrls = snd_sti_iec_ctl.len();
        (*player).snd_ctrls = snd_sti_iec_ctl[0];
    } else {
        (*player).num_ctrls = snd_sti_pcm_ctl.len();
        (*player).snd_ctrls = snd_sti_pcm_ctl[0];
    }

    0
}
/* EXPORT_SYMBOL_GPL(uni_player_init); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
