// SPDX-License-Identifier: GPL-2.0+
//
// soc-util.c  --  ALSA SoC Audio Layer utility functions
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//         Liam Girdwood <lrg@slimlogic.co.uk>

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type U64 = u64;

const NULL: *mut c_void = ptr::null_mut();
const EPROBE_DEFER: c_int = 517;
const ENOTSUPP: c_int = 524;
const EOPNOTSUPP: c_int = 95;
const ENODEV: c_int = 19;

const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 0;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 1 << 1;
const SNDRV_PCM_RATE_CONTINUOUS: u32 = 1 << 30;

const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_U8: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_U16_LE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 4;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << 5;
const SNDRV_PCM_FMTBIT_U24_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 7;
const SNDRV_PCM_FMTBIT_U32_LE: u64 = 1 << 8;
const SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE: u64 = 1 << 9;

const SND_SOC_POSSIBLE_DAIFMT_I2S: U64 = 1 << 0;
const SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: U64 = 1 << 1;
const SND_SOC_POSSIBLE_DAIFMT_LEFT_J: U64 = 1 << 2;
const SND_SOC_POSSIBLE_DAIFMT_DSP_A: U64 = 1 << 3;
const SND_SOC_POSSIBLE_DAIFMT_DSP_B: U64 = 1 << 4;
const SND_SOC_POSSIBLE_DAIFMT_AC97: U64 = 1 << 5;
const SND_SOC_POSSIBLE_DAIFMT_PDM: U64 = 1 << 6;
const SND_SOC_POSSIBLE_DAIFMT_GATED: U64 = 1 << 7;
const SND_SOC_POSSIBLE_DAIFMT_CONT: U64 = 1 << 8;
const SND_SOC_POSSIBLE_DAIFMT_NB_NF: U64 = 1 << 9;
const SND_SOC_POSSIBLE_DAIFMT_NB_IF: U64 = 1 << 10;
const SND_SOC_POSSIBLE_DAIFMT_IB_NF: U64 = 1 << 11;
const SND_SOC_POSSIBLE_DAIFMT_IB_IF: U64 = 1 << 12;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct faux_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub no_pcm: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
}

#[repr(C)]
pub struct snd_soc_component {
    pub driver: *const snd_soc_component_driver,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub open: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            substream: *mut snd_pcm_substream,
        ) -> c_int,
    >,
    pub idle_bias_on: c_int,
    pub use_pmdown_time: c_int,
    pub endianness: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub auto_selectable_formats: *const U64,
    pub num_auto_selectable_formats: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub driver: *const snd_soc_dai_driver,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub of_node: *mut c_void,
    pub dai_name: *const c_char,
    pub name: *const c_char,
}

#[repr(C)]
pub struct faux_device_ops {
    pub probe: Option<unsafe extern "C" fn(fdev: *mut faux_device) -> c_int>,
}

unsafe extern "C" {
    fn dev_err(dev: *const device, fmt: *const c_char, ...);
    fn snd_pcm_format_width(format: c_int) -> c_int;
    fn params_format(params: *const snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *const snd_pcm_hw_params) -> c_int;
    fn params_rate(params: *const snd_pcm_hw_params) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    );
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn faux_device_create(
        name: *const c_char,
        parent: *mut c_void,
        ops: *mut faux_device_ops,
    ) -> *mut faux_device;
    fn faux_device_destroy(fdev: *mut faux_device);
}

const fn c_str(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

const fn roundup(x: c_int, y: c_int) -> c_int {
    ((x + y - 1) / y) * y
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_ret(
    dev: *const device,
    ret: c_int,
    fmt: *const c_char,
    mut args: ...
) -> c_int {
    /* Positive, Zero values are not errors */
    if ret >= 0 {
        return ret;
    }

    /* Negative values might be errors */
    match ret {
        x if x == -EPROBE_DEFER || x == -ENOTSUPP || x == -EOPNOTSUPP => {}
        _ => {
            /*
             * C passes fmt and its varargs through struct va_format and %pV.
             * Rust has no stable source-level equivalent for constructing a
             * kernel va_format here, so preserve the diagnostic call shape.
             */
            dev_err(dev, c_str(b"ASoC error (%d): %pV\0"), ret, fmt, args);
        }
    }

    ret
}
// EXPORT_SYMBOL_GPL(snd_soc_ret);

#[unsafe(no_mangle)]
pub extern "C" fn snd_soc_calc_frame_size(
    sample_size: c_int,
    channels: c_int,
    tdm_slots: c_int,
) -> c_int {
    sample_size * channels * tdm_slots
}
// EXPORT_SYMBOL_GPL(snd_soc_calc_frame_size);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_params_to_frame_size(
    params: *const snd_pcm_hw_params,
) -> c_int {
    let sample_size = snd_pcm_format_width(params_format(params));
    if sample_size < 0 {
        return sample_size;
    }

    snd_soc_calc_frame_size(sample_size, params_channels(params), 1)
}
// EXPORT_SYMBOL_GPL(snd_soc_params_to_frame_size);

#[unsafe(no_mangle)]
pub extern "C" fn snd_soc_calc_bclk(
    fs: c_int,
    sample_size: c_int,
    channels: c_int,
    tdm_slots: c_int,
) -> c_int {
    fs * snd_soc_calc_frame_size(sample_size, channels, tdm_slots)
}
// EXPORT_SYMBOL_GPL(snd_soc_calc_bclk);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_params_to_bclk(params: *const snd_pcm_hw_params) -> c_int {
    let ret = snd_soc_params_to_frame_size(params);

    if ret > 0 {
        ret * params_rate(params)
    } else {
        ret
    }
}
// EXPORT_SYMBOL_GPL(snd_soc_params_to_bclk);

/**
 * snd_soc_tdm_params_to_bclk - calculate bclk from params and tdm slot info.
 *
 * Calculate the bclk from the params sample rate, the tdm slot count and the
 * tdm slot width. Optionally round-up the slot count to a given multiple.
 * Either or both of tdm_width and tdm_slots can be 0.
 *
 * If tdm_width == 0:	use params_width() as the slot width.
 * If tdm_slots == 0:	use params_channels() as the slot count.
 *
 * If slot_multiple > 1 the slot count (or params_channels() if tdm_slots == 0)
 * will be rounded up to a multiple of slot_multiple. This is mainly useful for
 * I2S mode, which has a left and right phase so the number of slots is always
 * a multiple of 2.
 *
 * If tdm_width == 0 && tdm_slots == 0 && slot_multiple < 2, this is equivalent
 * to calling snd_soc_params_to_bclk().
 *
 * @params:        Pointer to struct_pcm_hw_params.
 * @tdm_width:     Width in bits of the tdm slots. Must be >= 0.
 * @tdm_slots:     Number of tdm slots per frame. Must be >= 0.
 * @slot_multiple: If >1 roundup slot count to a multiple of this value.
 *
 * Return: bclk frequency in Hz, else a negative error code if params format
 *	   is invalid.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_tdm_params_to_bclk(
    params: *const snd_pcm_hw_params,
    mut tdm_width: c_int,
    mut tdm_slots: c_int,
    slot_multiple: c_int,
) -> c_int {
    if tdm_slots == 0 {
        tdm_slots = params_channels(params);
    }

    if slot_multiple > 1 {
        tdm_slots = roundup(tdm_slots, slot_multiple);
    }

    if tdm_width == 0 {
        tdm_width = snd_pcm_format_width(params_format(params));
        if tdm_width < 0 {
            return tdm_width;
        }
    }

    snd_soc_calc_bclk(params_rate(params), tdm_width, 1, tdm_slots)
}
// EXPORT_SYMBOL_GPL(snd_soc_tdm_params_to_bclk);

static DUMMY_DMA_HARDWARE: snd_pcm_hardware = snd_pcm_hardware {
    /* Random values to keep userspace happy when checking constraints */
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 4096,
    period_bytes_max: 4096 * 2,
    periods_min: 2,
    periods_max: 128,
};

static mut DUMMY_PLATFORM: snd_soc_component_driver = snd_soc_component_driver {
    open: Some(dummy_dma_open),
    idle_bias_on: 0,
    use_pmdown_time: 0,
    endianness: 0,
};

unsafe extern "C" fn dummy_dma_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);

    /*
     * If there are other components associated with rtd, we shouldn't
     * override their hwparams
     *
     * The for_each_rtd_components() macro depends on rtd internals supplied by
     * other files. Preserve the single visible branch body from this file.
     */
    if !component.is_null() && (*component).driver == &raw const DUMMY_PLATFORM {
        return 0;
    }

    /* BE's dont need dummy params */
    if !(*(*rtd).dai_link).no_pcm != 0 {
        snd_soc_set_runtime_hwparams(substream, &DUMMY_DMA_HARDWARE);
    }

    0
}

static DUMMY_CODEC: snd_soc_component_driver = snd_soc_component_driver {
    open: None,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

const STUB_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_U8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_U16_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_U24_LE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_U32_LE
    | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;

static DUMMY_DAI_FORMATS: U64 = SND_SOC_POSSIBLE_DAIFMT_I2S
    | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
    | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
    | SND_SOC_POSSIBLE_DAIFMT_DSP_A
    | SND_SOC_POSSIBLE_DAIFMT_DSP_B
    | SND_SOC_POSSIBLE_DAIFMT_AC97
    | SND_SOC_POSSIBLE_DAIFMT_PDM
    | SND_SOC_POSSIBLE_DAIFMT_GATED
    | SND_SOC_POSSIBLE_DAIFMT_CONT
    | SND_SOC_POSSIBLE_DAIFMT_NB_NF
    | SND_SOC_POSSIBLE_DAIFMT_NB_IF
    | SND_SOC_POSSIBLE_DAIFMT_IB_NF
    | SND_SOC_POSSIBLE_DAIFMT_IB_IF;

static DUMMY_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    auto_selectable_formats: &DUMMY_DAI_FORMATS,
    num_auto_selectable_formats: 1,
};

/*
 * The dummy CODEC is only meant to be used in situations where there is no
 * actual hardware.
 *
 * If there is actual hardware even if it does not have a control bus
 * the hardware will still have constraints like supported samplerates, etc.
 * which should be modelled. And the data flow graph also should be modelled
 * using DAPM.
 */
static mut DUMMY_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str(b"snd-soc-dummy-dai\0"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str(b"Playback\0"),
        channels_min: 1,
        channels_max: 384,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 5512,
        rate_max: 768000,
        formats: STUB_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c_str(b"Capture\0"),
        channels_min: 1,
        channels_max: 384,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 5512,
        rate_max: 768000,
        formats: STUB_FORMATS,
    },
    ops: &DUMMY_DAI_OPS,
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_dai_is_dummy(dai: *const snd_soc_dai) -> c_int {
    if (*dai).driver == &raw const DUMMY_DAI {
        return 1;
    }
    0
}
// EXPORT_SYMBOL_GPL(snd_soc_dai_is_dummy);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_component_is_dummy(
    component: *mut snd_soc_component,
) -> c_int {
    (((*component).driver == &raw const DUMMY_PLATFORM)
        || ((*component).driver == &DUMMY_CODEC)) as c_int
}

#[unsafe(no_mangle)]
pub static mut snd_soc_dummy_dlc: snd_soc_dai_link_component = snd_soc_dai_link_component {
    of_node: NULL,
    dai_name: c_str(b"snd-soc-dummy-dai\0"),
    name: c_str(b"snd-soc-dummy\0"),
};
// EXPORT_SYMBOL_GPL(snd_soc_dummy_dlc);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_dlc_is_dummy(
    dlc: *mut snd_soc_dai_link_component,
) -> c_int {
    if dlc == &raw mut snd_soc_dummy_dlc {
        return true as c_int;
    }

    if (!(*dlc).name.is_null()
        && strcmp((*dlc).name, snd_soc_dummy_dlc.name) == 0)
        || (!(*dlc).dai_name.is_null()
            && strcmp((*dlc).dai_name, snd_soc_dummy_dlc.dai_name) == 0)
    {
        return true as c_int;
    }

    false as c_int
}
// EXPORT_SYMBOL_GPL(snd_soc_dlc_is_dummy);

unsafe extern "C" fn snd_soc_dummy_probe(fdev: *mut faux_device) -> c_int {
    let mut ret: c_int;

    ret = devm_snd_soc_register_component(
        &mut (*fdev).dev,
        &DUMMY_CODEC,
        &raw mut DUMMY_DAI,
        1,
    );
    if ret < 0 {
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*fdev).dev,
        &raw const DUMMY_PLATFORM,
        ptr::null_mut(),
        0,
    );

    ret
}

static mut SOC_DUMMY_OPS: faux_device_ops = faux_device_ops {
    probe: Some(snd_soc_dummy_probe),
};

static mut SOC_DUMMY_DEV: *mut faux_device = ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_util_init() -> c_int {
    SOC_DUMMY_DEV = faux_device_create(
        c_str(b"snd-soc-dummy\0"),
        ptr::null_mut(),
        &raw mut SOC_DUMMY_OPS,
    );
    if SOC_DUMMY_DEV.is_null() {
        return -ENODEV;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_util_exit() {
    faux_device_destroy(SOC_DUMMY_DEV);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
