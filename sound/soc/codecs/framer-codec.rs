// SPDX-License-Identifier: GPL-2.0
//
// Framer ALSA SoC driver
//
// Copyright 2023 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>

// Dependencies from:
// linux/clk.h, linux/framer/framer.h, linux/module.h, linux/notifier.h,
// linux/platform_device.h, linux/slab.h, sound/jack.h, sound/pcm_params.h,
// sound/soc.h, sound/tlv.h

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

const FRAMER_NB_CHANNEL: c_int = 32;
const FRAMER_JACK_MASK: c_int = SND_JACK_LINEIN | SND_JACK_LINEOUT;

#[repr(C)]
pub struct framer_codec {
    framer: *mut framer,
    dev: *mut device,
    jack: snd_soc_jack,
    nb: notifier_block,
    carrier_work: work_struct,
    max_chan_playback: c_int,
    max_chan_capture: c_int,
}

#[repr(C)]
pub struct framer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call:
        Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_rule {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct snd_interval {
    pub min: u32,
    pub max: u32,
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

pub type snd_pcm_format_t = c_int;
pub type snd_pcm_hw_rule_func_t =
    Option<unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int>;

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub set_tdm_slot:
        Option<unsafe extern "C" fn(*mut snd_soc_dai, u32, u32, c_int, c_int) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
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
pub struct framer_status {
    pub link_is_on: bool,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub endianness: c_int,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_int = 0;
const NOTIFY_DONE: c_int = 0;
const NOTIFY_OK: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 0;
const SNDRV_PCM_HW_PARAM_FORMAT: c_int = 1;
const SNDRV_PCM_HW_PARAM_FRAME_BITS: c_int = 2;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_RATE_8000: u32 = 1;
const SND_JACK_LINEIN: c_int = 0x0002;
const SND_JACK_LINEOUT: c_int = 0x0004;
const SND_SOC_POSSIBLE_DAIFMT_DSP_B: u64 = 0;
const FRAMER_EVENT_STATUS: c_ulong = 0;

unsafe extern "C" {
    static mut system_power_efficient_wq: *mut c_void;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn hweight32(w: u32) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: c_int,
    ) -> *mut snd_interval;
    fn hw_param_mask(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_mask;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn params_channels(params: *mut snd_pcm_hw_params) -> u32;
    fn snd_pcm_format_physical_width(format: snd_pcm_format_t) -> c_int;
    fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_test_format(mask: *mut snd_mask, format: snd_pcm_format_t) -> bool;
    fn snd_mask_set_format(mask: *mut snd_mask, format: snd_pcm_format_t);
    fn snd_mask_refine(old: *mut snd_mask, new: *const snd_mask) -> c_int;
    fn pcm_format_to_bits(format: snd_pcm_format_t) -> u64;
    fn snd_pcm_hw_constraint_mask64(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        mask: u64,
    ) -> c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_int,
        var: c_int,
        func: snd_pcm_hw_rule_func_t,
        private: *mut snd_soc_dai,
        dep: c_int,
        terminator: c_int,
    ) -> c_int;
    fn snd_pcm_hw_constraint_single(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        val: u32,
    ) -> c_int;
    fn framer_get_status(framer: *mut framer, status: *mut framer_status) -> c_int;
    fn snd_soc_jack_report(jack: *mut snd_soc_jack, status: c_int, mask: c_int);
    fn queue_work(wq: *mut c_void, work: *mut work_struct) -> bool;
    fn INIT_WORK(
        work: *mut work_struct,
        func: Option<unsafe extern "C" fn(*mut work_struct)>,
    );
    fn kasprintf(gfp: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn snd_soc_card_jack_new(
        card: *mut snd_soc_card,
        id: *const c_char,
        typ: c_int,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    fn framer_init(framer: *mut framer) -> c_int;
    fn framer_power_on(framer: *mut framer) -> c_int;
    fn framer_power_off(framer: *mut framer);
    fn framer_exit(framer: *mut framer);
    fn framer_notifier_register(framer: *mut framer, nb: *mut notifier_block) -> c_int;
    fn framer_notifier_unregister(framer: *mut framer, nb: *mut notifier_block);
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_int) -> *mut c_void;
    fn devm_framer_get(dev: *mut device, con_id: *const c_char) -> *mut framer;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe fn container_of_work_struct_framer_codec_carrier_work(
    ptr: *mut work_struct,
) -> *mut framer_codec {
    (ptr as *mut u8).sub(core::mem::offset_of!(framer_codec, carrier_work)) as *mut framer_codec
}

unsafe fn container_of_notifier_block_framer_codec_nb(
    ptr: *mut notifier_block,
) -> *mut framer_codec {
    (ptr as *mut u8).sub(core::mem::offset_of!(framer_codec, nb)) as *mut framer_codec
}

unsafe extern "C" fn framer_dai_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: u32,
    rx_mask: u32,
    _slots: c_int,
    width: c_int,
) -> c_int {
    let framer =
        snd_soc_component_get_drvdata((*dai).component) as *mut framer_codec;

    match width {
        0 => {
            /* Not set -> default 8 */
        }
        8 => {}
        _ => {
            dev_err(
                (*dai).dev,
                c"tdm slot width %d not supported\n".as_ptr(),
                width,
            );
            return -EINVAL;
        }
    }

    (*framer).max_chan_playback = hweight32(tx_mask);
    if (*framer).max_chan_playback > FRAMER_NB_CHANNEL {
        dev_err(
            (*dai).dev,
            c"too many tx slots defined (mask = 0x%x) supported max %d\n".as_ptr(),
            tx_mask,
            FRAMER_NB_CHANNEL,
        );
        return -EINVAL;
    }

    (*framer).max_chan_capture = hweight32(rx_mask);
    if (*framer).max_chan_capture > FRAMER_NB_CHANNEL {
        dev_err(
            (*dai).dev,
            c"too many rx slots defined (mask = 0x%x) supported max %d\n".as_ptr(),
            rx_mask,
            FRAMER_NB_CHANNEL,
        );
        return -EINVAL;
    }

    0
}

/*
 * The constraints for format/channel is to match with the number of 8bit
 * time-slots available.
 */
unsafe fn framer_dai_hw_rule_channels_by_format(
    dai: *mut snd_soc_dai,
    params: *mut snd_pcm_hw_params,
    nb_ts: u32,
) -> c_int {
    let c = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let format = params_format(params);
    let mut ch = snd_interval { min: 0, max: 0 };
    let width: c_int;

    width = snd_pcm_format_physical_width(format);
    if width == 8 || width == 16 || width == 32 || width == 64 {
        ch.max = nb_ts.wrapping_mul(8).wrapping_div(width as u32);
    } else {
        dev_err(
            (*dai).dev,
            c"format physical width %d not supported\n".as_ptr(),
            width,
        );
        return -EINVAL;
    }

    ch.min = if ch.max != 0 { 1 } else { 0 };

    snd_interval_refine(c, &ch)
}

unsafe extern "C" fn framer_dai_hw_rule_playback_channels_by_format(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let dai = (*rule).private as *mut snd_soc_dai;
    let framer =
        snd_soc_component_get_drvdata((*dai).component) as *mut framer_codec;

    framer_dai_hw_rule_channels_by_format(dai, params, (*framer).max_chan_playback as u32)
}

unsafe extern "C" fn framer_dai_hw_rule_capture_channels_by_format(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let dai = (*rule).private as *mut snd_soc_dai;
    let framer =
        snd_soc_component_get_drvdata((*dai).component) as *mut framer_codec;

    framer_dai_hw_rule_channels_by_format(dai, params, (*framer).max_chan_capture as u32)
}

unsafe fn framer_dai_hw_rule_format_by_channels(
    dai: *mut snd_soc_dai,
    params: *mut snd_pcm_hw_params,
    nb_ts: u32,
) -> c_int {
    let f_old = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);
    let channels = params_channels(params);
    let slot_width: u32;
    let mut format: snd_pcm_format_t;
    let mut f_new = MaybeUninit::<snd_mask>::uninit();

    if channels == 0 || channels > nb_ts {
        dev_err(
            (*dai).dev,
            c"channels %u not supported\n".as_ptr(),
            nb_ts,
        );
        return -EINVAL;
    }

    slot_width = nb_ts.wrapping_div(channels).wrapping_mul(8);

    snd_mask_none(f_new.as_mut_ptr());
    format = 0;
    while format <= SND_PCM_FORMAT_LAST {
        if snd_mask_test_format(f_old, format) {
            if snd_pcm_format_physical_width(format) <= slot_width as c_int {
                snd_mask_set_format(f_new.as_mut_ptr(), format);
            }
        }
        format += 1;
    }

    snd_mask_refine(f_old, f_new.as_ptr())
}

unsafe extern "C" fn framer_dai_hw_rule_playback_format_by_channels(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let dai = (*rule).private as *mut snd_soc_dai;
    let framer =
        snd_soc_component_get_drvdata((*dai).component) as *mut framer_codec;

    framer_dai_hw_rule_format_by_channels(dai, params, (*framer).max_chan_playback as u32)
}

unsafe extern "C" fn framer_dai_hw_rule_capture_format_by_channels(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let dai = (*rule).private as *mut snd_soc_dai;
    let framer =
        snd_soc_component_get_drvdata((*dai).component) as *mut framer_codec;

    framer_dai_hw_rule_format_by_channels(dai, params, (*framer).max_chan_capture as u32)
}

const SND_PCM_FORMAT_LAST: snd_pcm_format_t = 64;

unsafe fn framer_formats(nb_ts: u8) -> u64 {
    let mut format_width: u32;
    let chan_width: u32;
    let mut format: snd_pcm_format_t;
    let mut formats_mask: u64;

    if nb_ts == 0 {
        return 0;
    }

    formats_mask = 0;
    chan_width = (nb_ts as u32).wrapping_mul(8);
    format = 0;
    while format <= SND_PCM_FORMAT_LAST {
        /* Support physical width multiple of 8bit */
        format_width = snd_pcm_format_physical_width(format) as u32;
        if format_width == 0 || format_width % 8 != 0 {
            format += 1;
            continue;
        }

        /*
         * And support physical width that can fit N times in the
         * channel
         */
        if format_width > chan_width || chan_width % format_width != 0 {
            format += 1;
            continue;
        }

        formats_mask |= pcm_format_to_bits(format);
        format += 1;
    }
    formats_mask
}

unsafe extern "C" fn framer_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let framer =
        snd_soc_component_get_drvdata((*dai).component) as *mut framer_codec;
    let hw_rule_channels_by_format: snd_pcm_hw_rule_func_t;
    let hw_rule_format_by_channels: snd_pcm_hw_rule_func_t;
    let frame_bits: u32;
    let format: u64;
    let mut ret: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        format = framer_formats((*framer).max_chan_capture as u8);
        hw_rule_channels_by_format = Some(framer_dai_hw_rule_capture_channels_by_format);
        hw_rule_format_by_channels = Some(framer_dai_hw_rule_capture_format_by_channels);
        frame_bits = ((*framer).max_chan_capture as u32).wrapping_mul(8);
    } else {
        format = framer_formats((*framer).max_chan_playback as u8);
        hw_rule_channels_by_format = Some(framer_dai_hw_rule_playback_channels_by_format);
        hw_rule_format_by_channels = Some(framer_dai_hw_rule_playback_format_by_channels);
        frame_bits = ((*framer).max_chan_playback as u32).wrapping_mul(8);
    }

    ret = snd_pcm_hw_constraint_mask64(
        (*substream).runtime,
        SNDRV_PCM_HW_PARAM_FORMAT,
        format,
    );
    if ret != 0 {
        dev_err(
            (*dai).dev,
            c"Failed to add format constraint (%d)\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        hw_rule_channels_by_format,
        dai,
        SNDRV_PCM_HW_PARAM_FORMAT,
        -1,
    );
    if ret != 0 {
        dev_err(
            (*dai).dev,
            c"Failed to add channels rule (%d)\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_FORMAT,
        hw_rule_format_by_channels,
        dai,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if ret != 0 {
        dev_err(
            (*dai).dev,
            c"Failed to add format rule (%d)\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = snd_pcm_hw_constraint_single(
        (*substream).runtime,
        SNDRV_PCM_HW_PARAM_FRAME_BITS,
        frame_bits,
    );
    if ret < 0 {
        dev_err(
            (*dai).dev,
            c"Failed to add frame_bits constraint (%d)\n".as_ptr(),
            ret,
        );
        return ret;
    }

    0
}

static FRAMER_DAI_FORMATS: u64 = SND_SOC_POSSIBLE_DAIFMT_DSP_B;

static FRAMER_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(framer_dai_startup),
    set_tdm_slot: Some(framer_dai_set_tdm_slot),
    auto_selectable_formats: &FRAMER_DAI_FORMATS,
    num_auto_selectable_formats: 1,
};

static mut FRAMER_DAI_DRIVER: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"framer".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: FRAMER_NB_CHANNEL as u32,
        rates: SNDRV_PCM_RATE_8000,
        formats: u64::MAX, /* Will be refined on DAI .startup() */
    },
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: FRAMER_NB_CHANNEL as u32,
        rates: SNDRV_PCM_RATE_8000,
        formats: u64::MAX, /* Will be refined on DAI .startup() */
    },
    ops: &FRAMER_DAI_OPS,
};

unsafe extern "C" fn framer_carrier_work(work: *mut work_struct) {
    let framer = container_of_work_struct_framer_codec_carrier_work(work);
    let mut framer_status = MaybeUninit::<framer_status>::uninit();
    let jack_status: c_int;
    let ret: c_int;

    ret = framer_get_status((*framer).framer, framer_status.as_mut_ptr());
    if ret != 0 {
        dev_err(
            (*framer).dev,
            c"get framer status failed (%d)\n".as_ptr(),
            ret,
        );
        return;
    }

    let framer_status = framer_status.assume_init();
    jack_status = if framer_status.link_is_on {
        FRAMER_JACK_MASK
    } else {
        0
    };
    snd_soc_jack_report(&mut (*framer).jack, jack_status, FRAMER_JACK_MASK);
}

unsafe extern "C" fn framer_carrier_notifier(
    nb: *mut notifier_block,
    action: c_ulong,
    _data: *mut c_void,
) -> c_int {
    let framer = container_of_notifier_block_framer_codec_nb(nb);

    match action {
        FRAMER_EVENT_STATUS => {
            queue_work(system_power_efficient_wq, &mut (*framer).carrier_work);
        }
        _ => {
            return NOTIFY_DONE;
        }
    }

    NOTIFY_OK
}

unsafe extern "C" fn framer_component_probe(component: *mut snd_soc_component) -> c_int {
    let framer = snd_soc_component_get_drvdata(component) as *mut framer_codec;
    let mut status = MaybeUninit::<framer_status>::uninit();
    let mut name: *const c_char;
    let mut allocated_name: *mut c_char = ptr::null_mut();
    let mut ret: c_int;

    INIT_WORK(&mut (*framer).carrier_work, Some(framer_carrier_work));

    name = c"carrier".as_ptr();
    if !(*component).name_prefix.is_null() {
        allocated_name = kasprintf(
            GFP_KERNEL,
            c"%s carrier".as_ptr(),
            (*component).name_prefix,
        );
        if allocated_name.is_null() {
            return -ENOMEM;
        }
        name = allocated_name;
    }

    ret = snd_soc_card_jack_new((*component).card, name, FRAMER_JACK_MASK, &mut (*framer).jack);
    if !(*component).name_prefix.is_null() {
        kfree(allocated_name as *mut c_void); /* A copy is done by snd_soc_card_jack_new */
    }
    if ret != 0 {
        dev_err((*component).dev, c"Cannot create jack\n".as_ptr());
        return ret;
    }

    ret = framer_init((*framer).framer);
    if ret != 0 {
        dev_err(
            (*component).dev,
            c"framer init failed (%d)\n".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = framer_power_on((*framer).framer);
    if ret != 0 {
        dev_err(
            (*component).dev,
            c"framer power-on failed (%d)\n".as_ptr(),
            ret,
        );
        framer_exit((*framer).framer);
        return ret;
    }

    /* Be sure that get_status is supported */
    ret = framer_get_status((*framer).framer, status.as_mut_ptr());
    if ret != 0 {
        dev_err(
            (*component).dev,
            c"get framer status failed (%d)\n".as_ptr(),
            ret,
        );
        framer_power_off((*framer).framer);
        framer_exit((*framer).framer);
        return ret;
    }

    (*framer).nb.notifier_call = Some(framer_carrier_notifier);
    ret = framer_notifier_register((*framer).framer, &mut (*framer).nb);
    if ret != 0 {
        dev_err((*component).dev, c"Cannot register event notifier\n".as_ptr());
        framer_power_off((*framer).framer);
        framer_exit((*framer).framer);
        return ret;
    }

    /* Queue work to set the initial value */
    queue_work(system_power_efficient_wq, &mut (*framer).carrier_work);

    0
}

unsafe extern "C" fn framer_component_remove(component: *mut snd_soc_component) {
    let framer = snd_soc_component_get_drvdata(component) as *mut framer_codec;

    framer_notifier_unregister((*framer).framer, &mut (*framer).nb);
    cancel_work_sync(&mut (*framer).carrier_work);
    framer_power_off((*framer).framer);
    framer_exit((*framer).framer);
}

static FRAMER_COMPONENT_DRIVER: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(framer_component_probe),
    remove: Some(framer_component_remove),
    endianness: 1,
};

unsafe extern "C" fn framer_codec_probe(pdev: *mut platform_device) -> c_int {
    let framer: *mut framer_codec;

    framer = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<framer_codec>(),
        GFP_KERNEL,
    ) as *mut framer_codec;
    if framer.is_null() {
        return -ENOMEM;
    }

    (*framer).dev = &mut (*pdev).dev;

    /* Get framer from parents node */
    (*framer).framer = devm_framer_get(&mut (*pdev).dev, ptr::null());
    if IS_ERR((*framer).framer as *const c_void) {
        return dev_err_probe(
            &mut (*pdev).dev,
            PTR_ERR((*framer).framer as *const c_void),
            c"get framer failed\n".as_ptr(),
        );
    }

    platform_set_drvdata(pdev, framer as *mut c_void);

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &FRAMER_COMPONENT_DRIVER,
        &raw mut FRAMER_DAI_DRIVER,
        1,
    )
}

static mut FRAMER_CODEC_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: c"framer-codec".as_ptr(),
    },
    probe: Some(framer_codec_probe),
};

// module_platform_driver(framer_codec_driver);
// MODULE_AUTHOR("Herve Codina <herve.codina@bootlin.com>");
// MODULE_DESCRIPTION("FRAMER ALSA SoC driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
