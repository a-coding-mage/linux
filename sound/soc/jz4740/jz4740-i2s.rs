// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (C) 2010, Lars-Peter Clausen <lars@metafoo.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;

const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf0;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x10;
const SND_SOC_DAIFMT_BC_FP: c_uint = 0x20;
const SND_SOC_DAIFMT_BP_FC: c_uint = 0x30;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x40;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_MSB: c_uint = 2;
const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x0f00;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;

const SNDRV_PCM_FORMAT_S8: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S20_LE: c_int = 4;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 6;

const SNDRV_PCM_FMTBIT_S8: c_uint = 1 << SNDRV_PCM_FORMAT_S8;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S20_LE: c_uint = 1 << SNDRV_PCM_FORMAT_S20_LE;
const SNDRV_PCM_FMTBIT_S24_LE: c_uint = 1 << SNDRV_PCM_FORMAT_S24_LE;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;
const SND_DMAENGINE_PCM_FLAG_COMPAT: c_uint = 1;

const fn bit(n: c_uint) -> c_uint {
    1u32 << n
}

const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    (!0u32 >> (31 - h)) & (!0u32 << l)
}

const fn field_prep(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

const fn reg_field(reg: c_uint, lsb: c_uint, msb: c_uint) -> RegField {
    RegField { reg, lsb, msb }
}

const JZ_REG_AIC_CONF: c_uint = 0x00;
const JZ_REG_AIC_CTRL: c_uint = 0x04;
const JZ_REG_AIC_I2S_FMT: c_uint = 0x10;
const JZ_REG_AIC_FIFO_STATUS: c_uint = 0x14;
const JZ_REG_AIC_I2S_STATUS: c_uint = 0x1c;
const JZ_REG_AIC_CLK_DIV: c_uint = 0x30;
const JZ_REG_AIC_FIFO: c_uint = 0x34;

const JZ_AIC_CONF_OVERFLOW_PLAY_LAST: c_uint = bit(6);
const JZ_AIC_CONF_INTERNAL_CODEC: c_uint = bit(5);
const JZ_AIC_CONF_I2S: c_uint = bit(4);
const JZ_AIC_CONF_RESET: c_uint = bit(3);
const JZ_AIC_CONF_BIT_CLK_MASTER: c_uint = bit(2);
const JZ_AIC_CONF_SYNC_CLK_MASTER: c_uint = bit(1);
const JZ_AIC_CONF_ENABLE: c_uint = bit(0);

const JZ_AIC_CTRL_OUTPUT_SAMPLE_SIZE: c_uint = genmask(21, 19);
const JZ_AIC_CTRL_INPUT_SAMPLE_SIZE: c_uint = genmask(18, 16);
const JZ_AIC_CTRL_ENABLE_RX_DMA: c_uint = bit(15);
const JZ_AIC_CTRL_ENABLE_TX_DMA: c_uint = bit(14);
const JZ_AIC_CTRL_MONO_TO_STEREO: c_uint = bit(11);
const JZ_AIC_CTRL_SWITCH_ENDIANNESS: c_uint = bit(10);
const JZ_AIC_CTRL_SIGNED_TO_UNSIGNED: c_uint = bit(9);
const JZ_AIC_CTRL_TFLUSH: c_uint = bit(8);
const JZ_AIC_CTRL_RFLUSH: c_uint = bit(7);
const JZ_AIC_CTRL_ENABLE_ROR_INT: c_uint = bit(6);
const JZ_AIC_CTRL_ENABLE_TUR_INT: c_uint = bit(5);
const JZ_AIC_CTRL_ENABLE_RFS_INT: c_uint = bit(4);
const JZ_AIC_CTRL_ENABLE_TFS_INT: c_uint = bit(3);
const JZ_AIC_CTRL_ENABLE_LOOPBACK: c_uint = bit(2);
const JZ_AIC_CTRL_ENABLE_PLAYBACK: c_uint = bit(1);
const JZ_AIC_CTRL_ENABLE_CAPTURE: c_uint = bit(0);

const JZ_AIC_I2S_FMT_DISABLE_BIT_CLK: c_uint = bit(12);
const JZ_AIC_I2S_FMT_DISABLE_BIT_ICLK: c_uint = bit(13);
const JZ_AIC_I2S_FMT_ENABLE_SYS_CLK: c_uint = bit(4);
const JZ_AIC_I2S_FMT_MSB: c_uint = bit(0);

const JZ_AIC_I2S_STATUS_BUSY: c_uint = bit(2);

const JZ4740_I2S_FMTS: c_uint = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_LE
    | SNDRV_PCM_FMTBIT_S24_LE;

#[repr(C)]
pub struct Regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RegmapField {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SndPcmSubstream {
    pub stream: c_int,
}

#[repr(C)]
pub struct SndPcmHwParams {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SndSocDai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SndSocComponent {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Resource {
    pub start: c_ulong,
}

#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RegField {
    pub reg: c_uint,
    pub lsb: c_uint,
    pub msb: c_uint,
}

#[repr(C)]
pub struct SndDmaengineDaiDmaData {
    pub addr: c_ulong,
    pub maxburst: c_uint,
}

#[repr(C)]
pub struct SndSocPcmStream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct SndSocDaiOps {
    pub probe: Option<unsafe extern "C" fn(*mut SndSocDai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut SndPcmSubstream, *mut SndSocDai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut SndPcmSubstream, *mut SndSocDai)>,
    pub trigger: Option<unsafe extern "C" fn(*mut SndPcmSubstream, c_int, *mut SndSocDai) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut SndPcmSubstream, *mut SndPcmHwParams, *mut SndSocDai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut SndSocDai, c_uint) -> c_int>,
}

#[repr(C)]
pub struct SndSocDaiDriver {
    pub playback: SndSocPcmStream,
    pub capture: SndSocPcmStream,
    pub symmetric_rate: c_uint,
    pub ops: *const SndSocDaiOps,
}

#[repr(C)]
pub struct SndSocComponentDriver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut SndSocComponent) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut SndSocComponent)>,
    pub suspend: Option<unsafe extern "C" fn(*mut SndSocComponent) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut SndSocComponent) -> c_int>,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct RegmapConfig {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
}

#[repr(C)]
pub struct DeviceDriver {
    pub name: *const c_char,
    pub of_match_table: *const OfDeviceId,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> c_int>,
    pub driver: DeviceDriver,
}

#[repr(C)]
pub struct I2sSocInfo {
    pub dai: *mut SndSocDaiDriver,
    pub field_rx_fifo_thresh: RegField,
    pub field_tx_fifo_thresh: RegField,
    pub field_i2sdiv_capture: RegField,
    pub field_i2sdiv_playback: RegField,
    pub shared_fifo_flush: bool,
}

#[repr(C)]
pub struct Jz4740I2s {
    pub regmap: *mut Regmap,
    pub field_rx_fifo_thresh: *mut RegmapField,
    pub field_tx_fifo_thresh: *mut RegmapField,
    pub field_i2sdiv_capture: *mut RegmapField,
    pub field_i2sdiv_playback: *mut RegmapField,
    pub clk_aic: *mut Clk,
    pub clk_i2s: *mut Clk,
    pub playback_dma_data: SndDmaengineDaiDmaData,
    pub capture_dma_data: SndDmaengineDaiDmaData,
    pub soc_info: *const I2sSocInfo,
}

unsafe extern "C" {
    fn snd_soc_dai_get_drvdata(dai: *mut SndSocDai) -> *mut c_void;
    fn snd_soc_dai_active(dai: *mut SndSocDai) -> c_int;
    fn snd_soc_dai_init_dma_data(
        dai: *mut SndSocDai,
        playback: *mut SndDmaengineDaiDmaData,
        capture: *mut SndDmaengineDaiDmaData,
    );
    fn snd_soc_component_get_drvdata(component: *mut SndSocComponent) -> *mut c_void;
    fn snd_soc_component_active(component: *mut SndSocComponent) -> c_int;
    fn regmap_set_bits(map: *mut Regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_clear_bits(map: *mut Regmap, reg: c_uint, bits: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut Regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut Regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut Regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_field_write(field: *mut RegmapField, val: c_uint) -> c_int;
    fn devm_regmap_field_alloc(dev: *mut Device, regmap: *mut Regmap, field: RegField)
        -> *mut RegmapField;
    fn clk_prepare_enable(clk: *mut Clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut Clk);
    fn clk_get_rate(clk: *mut Clk) -> c_ulong;
    fn params_format(params: *mut SndPcmHwParams) -> c_int;
    fn params_channels(params: *mut SndPcmHwParams) -> c_uint;
    fn params_rate(params: *mut SndPcmHwParams) -> c_ulong;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: c_uint) -> *mut c_void;
    fn device_get_match_data(dev: *mut Device) -> *const c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut PlatformDevice,
        index: c_uint,
        res: *mut *mut Resource,
    ) -> *mut c_void;
    fn devm_clk_get(dev: *mut Device, id: *const c_char) -> *mut Clk;
    fn devm_regmap_init_mmio(
        dev: *mut Device,
        regs: *mut c_void,
        config: *const RegmapConfig,
    ) -> *mut Regmap;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut Device,
        cmpnt_drv: *const SndSocComponentDriver,
        dai_drv: *mut SndSocDaiDriver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut Device,
        config: *mut c_void,
        flags: c_uint,
    ) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

unsafe extern "C" fn jz4740_i2s_startup(
    substream: *mut SndPcmSubstream,
    dai: *mut SndSocDai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut Jz4740I2s;
    let ret: c_int;

    /*
     * When we can flush FIFOs independently, only flush the FIFO
     * that is starting up. We can do this when the DAI is active
     * because it does not disturb other active substreams.
     */
    if !(*(*i2s).soc_info).shared_fifo_flush {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            regmap_set_bits((*i2s).regmap, JZ_REG_AIC_CTRL, JZ_AIC_CTRL_TFLUSH);
        } else {
            regmap_set_bits((*i2s).regmap, JZ_REG_AIC_CTRL, JZ_AIC_CTRL_RFLUSH);
        }
    }

    if snd_soc_dai_active(dai) != 0 {
        return 0;
    }

    /*
     * When there is a shared flush bit for both FIFOs, the TFLUSH
     * bit flushes both FIFOs. Flushing while the DAI is active would
     * cause FIFO underruns in other active substreams so we have to
     * guard this behind the snd_soc_dai_active() check.
     */
    if (*(*i2s).soc_info).shared_fifo_flush {
        regmap_set_bits((*i2s).regmap, JZ_REG_AIC_CTRL, JZ_AIC_CTRL_TFLUSH);
    }

    ret = clk_prepare_enable((*i2s).clk_i2s);
    if ret != 0 {
        return ret;
    }

    regmap_set_bits((*i2s).regmap, JZ_REG_AIC_CONF, JZ_AIC_CONF_ENABLE);
    0
}

unsafe extern "C" fn jz4740_i2s_shutdown(
    _substream: *mut SndPcmSubstream,
    dai: *mut SndSocDai,
) {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut Jz4740I2s;

    if snd_soc_dai_active(dai) != 0 {
        return;
    }

    regmap_clear_bits((*i2s).regmap, JZ_REG_AIC_CONF, JZ_AIC_CONF_ENABLE);
    clk_disable_unprepare((*i2s).clk_i2s);
}

unsafe extern "C" fn jz4740_i2s_trigger(
    substream: *mut SndPcmSubstream,
    cmd: c_int,
    dai: *mut SndSocDai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut Jz4740I2s;
    let mask: c_uint;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        mask = JZ_AIC_CTRL_ENABLE_PLAYBACK | JZ_AIC_CTRL_ENABLE_TX_DMA;
    } else {
        mask = JZ_AIC_CTRL_ENABLE_CAPTURE | JZ_AIC_CTRL_ENABLE_RX_DMA;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            regmap_set_bits((*i2s).regmap, JZ_REG_AIC_CTRL, mask);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            regmap_clear_bits((*i2s).regmap, JZ_REG_AIC_CTRL, mask);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn jz4740_i2s_set_fmt(dai: *mut SndSocDai, fmt: c_uint) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut Jz4740I2s;
    let conf_mask: c_uint = JZ_AIC_CONF_BIT_CLK_MASTER | JZ_AIC_CONF_SYNC_CLK_MASTER;
    let mut conf: c_uint = 0;
    let mut format: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            conf |= JZ_AIC_CONF_BIT_CLK_MASTER | JZ_AIC_CONF_SYNC_CLK_MASTER;
            format |= JZ_AIC_I2S_FMT_ENABLE_SYS_CLK;
        }
        SND_SOC_DAIFMT_BC_FP => {
            conf |= JZ_AIC_CONF_SYNC_CLK_MASTER;
        }
        SND_SOC_DAIFMT_BP_FC => {
            conf |= JZ_AIC_CONF_BIT_CLK_MASTER;
        }
        SND_SOC_DAIFMT_BC_FC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_MSB => {
            format |= JZ_AIC_I2S_FMT_MSB;
        }
        SND_SOC_DAIFMT_I2S => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        _ => return -EINVAL,
    }

    regmap_update_bits((*i2s).regmap, JZ_REG_AIC_CONF, conf_mask, conf);
    regmap_write((*i2s).regmap, JZ_REG_AIC_I2S_FMT, format);

    0
}

unsafe fn jz4740_i2s_get_i2sdiv(
    mclk: c_ulong,
    rate: c_ulong,
    i2sdiv_max: c_ulong,
) -> c_int {
    let mut div: c_ulong;
    let rate1: c_ulong;
    let rate2: c_ulong;
    let err1: c_ulong;
    let err2: c_ulong;

    div = mclk / (64 * rate);
    if div == 0 {
        div = 1;
    }

    rate1 = mclk / (64 * div);
    rate2 = mclk / (64 * (div + 1));

    err1 = rate1.abs_diff(rate);
    err2 = rate2.abs_diff(rate);

    /*
     * Choose the divider that produces the smallest error in the
     * output rate and reject dividers with a 5% or higher error.
     * In the event that both dividers are outside the acceptable
     * error margin, reject the rate to prevent distorted audio.
     * (The number 5% is arbitrary.)
     */
    if div <= i2sdiv_max && err1 <= err2 && err1 < rate / 20 {
        return div as c_int;
    }
    if div < i2sdiv_max && err2 < rate / 20 {
        return (div + 1) as c_int;
    }

    -EINVAL
}

unsafe extern "C" fn jz4740_i2s_hw_params(
    substream: *mut SndPcmSubstream,
    params: *mut SndPcmHwParams,
    dai: *mut SndSocDai,
) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut Jz4740I2s;
    let div_field: *mut RegmapField;
    let i2sdiv_max: c_ulong;
    let sample_size: c_uint;
    let mut ctrl: c_uint = 0;
    let mut conf: c_uint = 0;
    let mut div: c_int = 1;

    regmap_read((*i2s).regmap, JZ_REG_AIC_CTRL, &mut ctrl);
    regmap_read((*i2s).regmap, JZ_REG_AIC_CONF, &mut conf);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 => sample_size = 0,
        SNDRV_PCM_FORMAT_S16_LE => sample_size = 1,
        SNDRV_PCM_FORMAT_S20_LE => sample_size = 3,
        SNDRV_PCM_FORMAT_S24_LE => sample_size = 4,
        _ => return -EINVAL,
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ctrl &= !JZ_AIC_CTRL_OUTPUT_SAMPLE_SIZE;
        ctrl |= field_prep(JZ_AIC_CTRL_OUTPUT_SAMPLE_SIZE, sample_size);

        if params_channels(params) == 1 {
            ctrl |= JZ_AIC_CTRL_MONO_TO_STEREO;
        } else {
            ctrl &= !JZ_AIC_CTRL_MONO_TO_STEREO;
        }

        div_field = (*i2s).field_i2sdiv_playback;
        i2sdiv_max = genmask(
            (*(*i2s).soc_info).field_i2sdiv_playback.msb,
            (*(*i2s).soc_info).field_i2sdiv_playback.lsb,
        ) as c_ulong;
    } else {
        ctrl &= !JZ_AIC_CTRL_INPUT_SAMPLE_SIZE;
        ctrl |= field_prep(JZ_AIC_CTRL_INPUT_SAMPLE_SIZE, sample_size);

        div_field = (*i2s).field_i2sdiv_capture;
        i2sdiv_max = genmask(
            (*(*i2s).soc_info).field_i2sdiv_capture.msb,
            (*(*i2s).soc_info).field_i2sdiv_capture.lsb,
        ) as c_ulong;
    }

    /*
     * Only calculate I2SDIV if we're supplying the bit or frame clock.
     * If the codec is supplying both clocks then the divider output is
     * unused, and we don't want it to limit the allowed sample rates.
     */
    if conf & (JZ_AIC_CONF_BIT_CLK_MASTER | JZ_AIC_CONF_SYNC_CLK_MASTER) != 0 {
        div = jz4740_i2s_get_i2sdiv(
            clk_get_rate((*i2s).clk_i2s),
            params_rate(params),
            i2sdiv_max,
        );
        if div < 0 {
            return div;
        }
    }

    regmap_write((*i2s).regmap, JZ_REG_AIC_CTRL, ctrl);
    regmap_field_write(div_field, (div - 1) as c_uint);

    0
}

unsafe extern "C" fn jz4740_i2s_dai_probe(dai: *mut SndSocDai) -> c_int {
    let i2s = snd_soc_dai_get_drvdata(dai) as *mut Jz4740I2s;

    snd_soc_dai_init_dma_data(
        dai,
        &mut (*i2s).playback_dma_data,
        &mut (*i2s).capture_dma_data,
    );

    0
}

static JZ4740_I2S_DAI_OPS: SndSocDaiOps = SndSocDaiOps {
    probe: Some(jz4740_i2s_dai_probe),
    startup: Some(jz4740_i2s_startup),
    shutdown: Some(jz4740_i2s_shutdown),
    trigger: Some(jz4740_i2s_trigger),
    hw_params: Some(jz4740_i2s_hw_params),
    set_fmt: Some(jz4740_i2s_set_fmt),
};

static mut JZ4740_I2S_DAI: SndSocDaiDriver = SndSocDaiDriver {
    playback: SndSocPcmStream {
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        formats: JZ4740_I2S_FMTS,
    },
    capture: SndSocPcmStream {
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        formats: JZ4740_I2S_FMTS,
    },
    symmetric_rate: 1,
    ops: &JZ4740_I2S_DAI_OPS,
};

static JZ4740_I2S_SOC_INFO: I2sSocInfo = I2sSocInfo {
    dai: unsafe { &mut JZ4740_I2S_DAI },
    field_rx_fifo_thresh: reg_field(JZ_REG_AIC_CONF, 12, 15),
    field_tx_fifo_thresh: reg_field(JZ_REG_AIC_CONF, 8, 11),
    field_i2sdiv_capture: reg_field(JZ_REG_AIC_CLK_DIV, 0, 3),
    field_i2sdiv_playback: reg_field(JZ_REG_AIC_CLK_DIV, 0, 3),
    shared_fifo_flush: true,
};

static JZ4760_I2S_SOC_INFO: I2sSocInfo = I2sSocInfo {
    dai: unsafe { &mut JZ4740_I2S_DAI },
    field_rx_fifo_thresh: reg_field(JZ_REG_AIC_CONF, 24, 27),
    field_tx_fifo_thresh: reg_field(JZ_REG_AIC_CONF, 16, 20),
    field_i2sdiv_capture: reg_field(JZ_REG_AIC_CLK_DIV, 0, 3),
    field_i2sdiv_playback: reg_field(JZ_REG_AIC_CLK_DIV, 0, 3),
    shared_fifo_flush: false,
};

static X1000_I2S_SOC_INFO: I2sSocInfo = I2sSocInfo {
    dai: unsafe { &mut JZ4740_I2S_DAI },
    field_rx_fifo_thresh: reg_field(JZ_REG_AIC_CONF, 24, 27),
    field_tx_fifo_thresh: reg_field(JZ_REG_AIC_CONF, 16, 20),
    field_i2sdiv_capture: reg_field(JZ_REG_AIC_CLK_DIV, 0, 8),
    field_i2sdiv_playback: reg_field(JZ_REG_AIC_CLK_DIV, 0, 8),
    shared_fifo_flush: false,
};

static mut JZ4770_I2S_DAI: SndSocDaiDriver = SndSocDaiDriver {
    playback: SndSocPcmStream {
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        formats: JZ4740_I2S_FMTS,
    },
    capture: SndSocPcmStream {
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        formats: JZ4740_I2S_FMTS,
    },
    symmetric_rate: 0,
    ops: &JZ4740_I2S_DAI_OPS,
};

static JZ4770_I2S_SOC_INFO: I2sSocInfo = I2sSocInfo {
    dai: unsafe { &mut JZ4770_I2S_DAI },
    field_rx_fifo_thresh: reg_field(JZ_REG_AIC_CONF, 24, 27),
    field_tx_fifo_thresh: reg_field(JZ_REG_AIC_CONF, 16, 20),
    field_i2sdiv_capture: reg_field(JZ_REG_AIC_CLK_DIV, 8, 11),
    field_i2sdiv_playback: reg_field(JZ_REG_AIC_CLK_DIV, 0, 3),
    shared_fifo_flush: false,
};

static JZ4780_I2S_SOC_INFO: I2sSocInfo = I2sSocInfo {
    dai: unsafe { &mut JZ4770_I2S_DAI },
    field_rx_fifo_thresh: reg_field(JZ_REG_AIC_CONF, 24, 27),
    field_tx_fifo_thresh: reg_field(JZ_REG_AIC_CONF, 16, 20),
    field_i2sdiv_capture: reg_field(JZ_REG_AIC_CLK_DIV, 8, 11),
    field_i2sdiv_playback: reg_field(JZ_REG_AIC_CLK_DIV, 0, 3),
    shared_fifo_flush: false,
};

unsafe extern "C" fn jz4740_i2s_suspend(component: *mut SndSocComponent) -> c_int {
    let i2s = snd_soc_component_get_drvdata(component) as *mut Jz4740I2s;

    if snd_soc_component_active(component) != 0 {
        regmap_clear_bits((*i2s).regmap, JZ_REG_AIC_CONF, JZ_AIC_CONF_ENABLE);
        clk_disable_unprepare((*i2s).clk_i2s);
    }

    clk_disable_unprepare((*i2s).clk_aic);

    0
}

unsafe extern "C" fn jz4740_i2s_resume(component: *mut SndSocComponent) -> c_int {
    let i2s = snd_soc_component_get_drvdata(component) as *mut Jz4740I2s;
    let mut ret: c_int;

    ret = clk_prepare_enable((*i2s).clk_aic);
    if ret != 0 {
        return ret;
    }

    if snd_soc_component_active(component) != 0 {
        ret = clk_prepare_enable((*i2s).clk_i2s);
        if ret != 0 {
            clk_disable_unprepare((*i2s).clk_aic);
            return ret;
        }

        regmap_set_bits((*i2s).regmap, JZ_REG_AIC_CONF, JZ_AIC_CONF_ENABLE);
    }

    0
}

unsafe extern "C" fn jz4740_i2s_probe(component: *mut SndSocComponent) -> c_int {
    let i2s = snd_soc_component_get_drvdata(component) as *mut Jz4740I2s;
    let ret: c_int;

    ret = clk_prepare_enable((*i2s).clk_aic);
    if ret != 0 {
        return ret;
    }

    regmap_write((*i2s).regmap, JZ_REG_AIC_CONF, JZ_AIC_CONF_RESET);

    regmap_write(
        (*i2s).regmap,
        JZ_REG_AIC_CONF,
        JZ_AIC_CONF_OVERFLOW_PLAY_LAST | JZ_AIC_CONF_I2S | JZ_AIC_CONF_INTERNAL_CODEC,
    );

    regmap_field_write((*i2s).field_rx_fifo_thresh, 7);
    regmap_field_write((*i2s).field_tx_fifo_thresh, 8);

    0
}

unsafe extern "C" fn jz4740_i2s_remove(component: *mut SndSocComponent) {
    let i2s = snd_soc_component_get_drvdata(component) as *mut Jz4740I2s;

    clk_disable_unprepare((*i2s).clk_aic);
}

static JZ4740_I2S_COMPONENT_NAME: &[u8] = b"jz4740-i2s\0";

static JZ4740_I2S_COMPONENT: SndSocComponentDriver = SndSocComponentDriver {
    name: JZ4740_I2S_COMPONENT_NAME.as_ptr() as *const c_char,
    probe: Some(jz4740_i2s_probe),
    remove: Some(jz4740_i2s_remove),
    suspend: Some(jz4740_i2s_suspend),
    resume: Some(jz4740_i2s_resume),
    legacy_dai_naming: 1,
};

static COMPAT_JZ4740: &[u8] = b"ingenic,jz4740-i2s\0";
static COMPAT_JZ4760: &[u8] = b"ingenic,jz4760-i2s\0";
static COMPAT_JZ4770: &[u8] = b"ingenic,jz4770-i2s\0";
static COMPAT_JZ4780: &[u8] = b"ingenic,jz4780-i2s\0";
static COMPAT_X1000: &[u8] = b"ingenic,x1000-i2s\0";

static JZ4740_OF_MATCHES: [OfDeviceId; 6] = [
    OfDeviceId {
        compatible: COMPAT_JZ4740.as_ptr() as *const c_char,
        data: &JZ4740_I2S_SOC_INFO as *const I2sSocInfo as *const c_void,
    },
    OfDeviceId {
        compatible: COMPAT_JZ4760.as_ptr() as *const c_char,
        data: &JZ4760_I2S_SOC_INFO as *const I2sSocInfo as *const c_void,
    },
    OfDeviceId {
        compatible: COMPAT_JZ4770.as_ptr() as *const c_char,
        data: &JZ4770_I2S_SOC_INFO as *const I2sSocInfo as *const c_void,
    },
    OfDeviceId {
        compatible: COMPAT_JZ4780.as_ptr() as *const c_char,
        data: &JZ4780_I2S_SOC_INFO as *const I2sSocInfo as *const c_void,
    },
    OfDeviceId {
        compatible: COMPAT_X1000.as_ptr() as *const c_char,
        data: &X1000_I2S_SOC_INFO as *const I2sSocInfo as *const c_void,
    },
    OfDeviceId {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

/* MODULE_DEVICE_TABLE(of, jz4740_of_matches); */

unsafe fn jz4740_i2s_init_regmap_fields(dev: *mut Device, i2s: *mut Jz4740I2s) -> c_int {
    (*i2s).field_rx_fifo_thresh = devm_regmap_field_alloc(
        dev,
        (*i2s).regmap,
        (*(*i2s).soc_info).field_rx_fifo_thresh,
    );
    if IS_ERR((*i2s).field_rx_fifo_thresh as *const c_void) {
        return PTR_ERR((*i2s).field_rx_fifo_thresh as *const c_void);
    }

    (*i2s).field_tx_fifo_thresh = devm_regmap_field_alloc(
        dev,
        (*i2s).regmap,
        (*(*i2s).soc_info).field_tx_fifo_thresh,
    );
    if IS_ERR((*i2s).field_tx_fifo_thresh as *const c_void) {
        return PTR_ERR((*i2s).field_tx_fifo_thresh as *const c_void);
    }

    (*i2s).field_i2sdiv_capture = devm_regmap_field_alloc(
        dev,
        (*i2s).regmap,
        (*(*i2s).soc_info).field_i2sdiv_capture,
    );
    if IS_ERR((*i2s).field_i2sdiv_capture as *const c_void) {
        return PTR_ERR((*i2s).field_i2sdiv_capture as *const c_void);
    }

    (*i2s).field_i2sdiv_playback = devm_regmap_field_alloc(
        dev,
        (*i2s).regmap,
        (*(*i2s).soc_info).field_i2sdiv_playback,
    );
    if IS_ERR((*i2s).field_i2sdiv_playback as *const c_void) {
        return PTR_ERR((*i2s).field_i2sdiv_playback as *const c_void);
    }

    0
}

static JZ4740_I2S_REGMAP_CONFIG: RegmapConfig = RegmapConfig {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: JZ_REG_AIC_FIFO,
};

static CLK_AIC: &[u8] = b"aic\0";
static CLK_I2S: &[u8] = b"i2s\0";

unsafe extern "C" fn jz4740_i2s_dev_probe(pdev: *mut PlatformDevice) -> c_int {
    let dev: *mut Device = &mut (*pdev).dev;
    let i2s: *mut Jz4740I2s;
    let mut mem: *mut Resource = ptr::null_mut();
    let regs: *mut c_void;
    let mut ret: c_int;

    i2s = devm_kzalloc(dev, core::mem::size_of::<Jz4740I2s>(), GFP_KERNEL) as *mut Jz4740I2s;
    if i2s.is_null() {
        return -ENOMEM;
    }

    (*i2s).soc_info = device_get_match_data(dev) as *const I2sSocInfo;

    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut mem);
    if IS_ERR(regs as *const c_void) {
        return PTR_ERR(regs as *const c_void);
    }

    (*i2s).playback_dma_data.maxburst = 16;
    (*i2s).playback_dma_data.addr = (*mem).start + JZ_REG_AIC_FIFO as c_ulong;

    (*i2s).capture_dma_data.maxburst = 16;
    (*i2s).capture_dma_data.addr = (*mem).start + JZ_REG_AIC_FIFO as c_ulong;

    (*i2s).clk_aic = devm_clk_get(dev, CLK_AIC.as_ptr() as *const c_char);
    if IS_ERR((*i2s).clk_aic as *const c_void) {
        return PTR_ERR((*i2s).clk_aic as *const c_void);
    }

    (*i2s).clk_i2s = devm_clk_get(dev, CLK_I2S.as_ptr() as *const c_char);
    if IS_ERR((*i2s).clk_i2s as *const c_void) {
        return PTR_ERR((*i2s).clk_i2s as *const c_void);
    }

    (*i2s).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, &JZ4740_I2S_REGMAP_CONFIG);
    if IS_ERR((*i2s).regmap as *const c_void) {
        return PTR_ERR((*i2s).regmap as *const c_void);
    }

    ret = jz4740_i2s_init_regmap_fields(dev, i2s);
    if ret != 0 {
        return ret;
    }

    platform_set_drvdata(pdev, i2s as *mut c_void);

    ret = devm_snd_soc_register_component(dev, &JZ4740_I2S_COMPONENT, (*(*i2s).soc_info).dai, 1);
    if ret != 0 {
        return ret;
    }

    devm_snd_dmaengine_pcm_register(dev, ptr::null_mut(), SND_DMAENGINE_PCM_FLAG_COMPAT)
}

static DRIVER_NAME: &[u8] = b"jz4740-i2s\0";

static mut JZ4740_I2S_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(jz4740_i2s_dev_probe),
    driver: DeviceDriver {
        name: DRIVER_NAME.as_ptr() as *const c_char,
        of_match_table: JZ4740_OF_MATCHES.as_ptr(),
    },
};

/* module_platform_driver(jz4740_i2s_driver); */

/* MODULE_AUTHOR("Lars-Peter Clausen, <lars@metafoo.de>"); */
/* MODULE_DESCRIPTION("Ingenic JZ4740 SoC I2S driver"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:jz4740-i2s"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
