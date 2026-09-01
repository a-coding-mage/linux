// SPDX-License-Identifier: GPL-2.0
// Audio driver for PCM1789
// Copyright (C) 2018 Bootlin
// Mylene Josserand <mylene.josserand@bootlin.com>

// C dependencies:
// <linux/gpio/consumer.h>
// <linux/module.h>
// <linux/workqueue.h>
// <sound/pcm_params.h>
// <sound/soc.h>
// <sound/tlv.h>
// "pcm1789.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

const PCM1789_MUTE_CONTROL: c_uint = 0x10;
const PCM1789_FMT_CONTROL: c_uint = 0x11;
const PCM1789_SOFT_MUTE: c_uint = 0x14;
const PCM1789_DAC_VOL_LEFT: c_uint = 0x18;
const PCM1789_DAC_VOL_RIGHT: c_uint = 0x19;

const PCM1789_FMT_MASK: c_uint = 0x07;
const PCM1789_MUTE_MASK: c_uint = 0x03;
const PCM1789_MUTE_SRET: c_uint = 0x06;

#[repr(C)]
struct pcm1789_private {
    regmap: *mut regmap,
    format: c_uint,
    rate: c_uint,
    reset: *mut gpio_desc,
    work: work_struct,
    dev: *mut device,
}

static pcm1789_reg_defaults: [reg_default; 4] = [
    reg_default {
        reg: PCM1789_FMT_CONTROL,
        def: 0x00,
    },
    reg_default {
        reg: PCM1789_SOFT_MUTE,
        def: 0x00,
    },
    reg_default {
        reg: PCM1789_DAC_VOL_LEFT,
        def: 0xff,
    },
    reg_default {
        reg: PCM1789_DAC_VOL_RIGHT,
        def: 0xff,
    },
];

unsafe extern "C" fn pcm1789_accessible_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg >= PCM1789_MUTE_CONTROL && reg <= PCM1789_DAC_VOL_RIGHT
}

unsafe extern "C" fn pcm1789_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    unsafe { pcm1789_accessible_reg(dev, reg) }
}

unsafe extern "C" fn pcm1789_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    format: c_uint,
) -> c_int {
    let component = unsafe { (*codec_dai).component };
    let priv_ = unsafe { snd_soc_component_get_drvdata(component) as *mut pcm1789_private };

    unsafe {
        (*priv_).format = format;
    }

    0
}

unsafe extern "C" fn pcm1789_mute(
    codec_dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = unsafe { (*codec_dai).component };
    let priv_ = unsafe { snd_soc_component_get_drvdata(component) as *mut pcm1789_private };

    unsafe {
        regmap_update_bits(
            (*priv_).regmap,
            PCM1789_SOFT_MUTE,
            PCM1789_MUTE_MASK,
            if mute != 0 { 0 } else { PCM1789_MUTE_MASK },
        )
    }
}

unsafe extern "C" fn pcm1789_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    codec_dai: *mut snd_soc_dai,
) -> c_int {
    let component = unsafe { (*codec_dai).component };
    let priv_ = unsafe { snd_soc_component_get_drvdata(component) as *mut pcm1789_private };
    let mut val: c_int = 0;
    let ret: c_int;

    unsafe {
        (*priv_).rate = params_rate(params);
    }

    match unsafe { (*priv_).format & SND_SOC_DAIFMT_FORMAT_MASK } {
        SND_SOC_DAIFMT_RIGHT_J => {
            match unsafe { params_width(params) } {
                24 => {
                    val = 2;
                }
                16 => {
                    val = 3;
                }
                _ => {
                    return -EINVAL;
                }
            }
        }
        SND_SOC_DAIFMT_I2S => {
            match unsafe { params_width(params) } {
                16 | 24 | 32 => {
                    val = 0;
                }
                _ => {
                    return -EINVAL;
                }
            }
        }
        SND_SOC_DAIFMT_LEFT_J => {
            match unsafe { params_width(params) } {
                16 | 24 | 32 => {
                    val = 1;
                }
                _ => {
                    return -EINVAL;
                }
            }
        }
        _ => {
            unsafe {
                dev_err((*component).dev, c"Invalid DAI format\n".as_ptr());
            }
            return -EINVAL;
        }
    }

    ret = unsafe {
        regmap_update_bits(
            (*priv_).regmap,
            PCM1789_FMT_CONTROL,
            PCM1789_FMT_MASK,
            val as c_uint,
        )
    };
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn pcm1789_work_queue(work: *mut work_struct) {
    let priv_ = container_of!(work, pcm1789_private, work);

    /* Perform a software reset to remove codec from desynchronized state */
    if unsafe {
        regmap_update_bits(
            (*priv_).regmap,
            PCM1789_MUTE_CONTROL,
            0x3 << PCM1789_MUTE_SRET,
            0,
        )
    } < 0
    {
        unsafe {
            dev_err((*priv_).dev, c"Error while setting SRET".as_ptr());
        }
    }
}

unsafe extern "C" fn pcm1789_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = unsafe { (*dai).component };
    let priv_ = unsafe { snd_soc_component_get_drvdata(component) as *mut pcm1789_private };
    let mut ret: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            unsafe {
                schedule_work(&mut (*priv_).work);
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {}
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

static pcm1789_selectable_formats: u64 = SND_SOC_POSSIBLE_DAIFMT_I2S
    | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
    | SND_SOC_POSSIBLE_DAIFMT_LEFT_J;

static pcm1789_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(pcm1789_set_dai_fmt),
    hw_params: Some(pcm1789_hw_params),
    mute_stream: Some(pcm1789_mute),
    trigger: Some(pcm1789_trigger),
    auto_selectable_formats: &pcm1789_selectable_formats,
    num_auto_selectable_formats: 1,
    no_capture_mute: 1,
    ..unsafe { core::mem::zeroed() }
};

// static const DECLARE_TLV_DB_SCALE(pcm1789_dac_tlv, -12000, 50, 1);
static pcm1789_dac_tlv: [c_uint; 4] = TLV_DB_SCALE_ITEM(-12000, 50, 1);

static pcm1789_controls: [snd_kcontrol_new; 1] = [SOC_DOUBLE_R_RANGE_TLV(
    c"DAC Playback Volume".as_ptr(),
    PCM1789_DAC_VOL_LEFT,
    PCM1789_DAC_VOL_RIGHT,
    0,
    0xf,
    0xff,
    0,
    pcm1789_dac_tlv.as_ptr(),
)];

static pcm1789_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_OUTPUT(c"IOUTL+".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"IOUTL-".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"IOUTR+".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"IOUTR-".as_ptr()),
];

static pcm1789_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: c"IOUTL+".as_ptr(),
        control: core::ptr::null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"IOUTL-".as_ptr(),
        control: core::ptr::null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"IOUTR+".as_ptr(),
        control: core::ptr::null(),
        source: c"Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"IOUTR-".as_ptr(),
        control: core::ptr::null(),
        source: c"Playback".as_ptr(),
    },
];

static mut pcm1789_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"pcm1789-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 10000,
        rate_max: 200000,
        formats: PCM1789_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &pcm1789_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

#[no_mangle]
pub static pcm1789_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: PCM1789_DAC_VOL_RIGHT,
    reg_defaults: pcm1789_reg_defaults.as_ptr(),
    num_reg_defaults: pcm1789_reg_defaults.len() as c_uint,
    writeable_reg: Some(pcm1789_writeable_reg),
    readable_reg: Some(pcm1789_accessible_reg),
    ..unsafe { core::mem::zeroed() }
};
// EXPORT_SYMBOL_GPL(pcm1789_regmap_config);

static soc_component_dev_pcm1789: snd_soc_component_driver = snd_soc_component_driver {
    controls: pcm1789_controls.as_ptr(),
    num_controls: pcm1789_controls.len() as c_uint,
    dapm_widgets: pcm1789_dapm_widgets.as_ptr(),
    num_dapm_widgets: pcm1789_dapm_widgets.len() as c_uint,
    dapm_routes: pcm1789_dapm_routes.as_ptr(),
    num_dapm_routes: pcm1789_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

#[no_mangle]
pub unsafe extern "C" fn pcm1789_common_init(
    dev: *mut device,
    regmap: *mut regmap,
) -> c_int {
    let mut pcm1789: *mut pcm1789_private;

    pcm1789 = unsafe {
        devm_kzalloc(
            dev,
            core::mem::size_of::<pcm1789_private>(),
            GFP_KERNEL,
        ) as *mut pcm1789_private
    };
    if pcm1789.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*pcm1789).regmap = regmap;
        (*pcm1789).dev = dev;
        dev_set_drvdata(dev, pcm1789 as *mut c_void);
    }

    unsafe {
        (*pcm1789).reset = devm_gpiod_get_optional(dev, c"reset".as_ptr(), GPIOD_OUT_HIGH);
    }
    if unsafe { IS_ERR((*pcm1789).reset as *const c_void) } {
        return unsafe { PTR_ERR((*pcm1789).reset as *const c_void) as c_int };
    }

    unsafe {
        gpiod_set_value_cansleep((*pcm1789).reset, 0);
        msleep(300);

        INIT_WORK(&mut (*pcm1789).work, Some(pcm1789_work_queue));

        devm_snd_soc_register_component(
            dev,
            &soc_component_dev_pcm1789,
            &mut pcm1789_dai,
            1,
        )
    }
}
// EXPORT_SYMBOL_GPL(pcm1789_common_init);

#[no_mangle]
pub unsafe extern "C" fn pcm1789_common_exit(dev: *mut device) {
    let priv_ = unsafe { dev_get_drvdata(dev) as *mut pcm1789_private };

    unsafe {
        flush_work(&mut (*priv_).work);
    }
}
// EXPORT_SYMBOL_GPL(pcm1789_common_exit);

// MODULE_DESCRIPTION("ASoC PCM1789 driver");
// MODULE_AUTHOR("Mylene Josserand <mylene.josserand@free-electrons.com>");
// MODULE_LICENSE("GPL");

unsafe extern "C" {
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_POSSIBLE_DAIFMT_I2S: u64;
    static SND_SOC_POSSIBLE_DAIFMT_RIGHT_J: u64;
    static SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static PCM1789_FORMATS: u64;
    static GFP_KERNEL: gfp_t;
    static GPIOD_OUT_HIGH: gpiod_flags;
    static EINVAL: c_int;
    static ENOMEM: c_int;

    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn schedule_work(work: *mut work_struct) -> bool;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: gpiod_flags,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn msleep(msecs: c_uint);
    fn INIT_WORK(work: *mut work_struct, func: Option<unsafe extern "C" fn(*mut work_struct)>);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn flush_work(work: *mut work_struct) -> bool;

    fn TLV_DB_SCALE_ITEM(min: c_int, step: c_int, mute: c_int) -> [c_uint; 4];
    fn SOC_DOUBLE_R_RANGE_TLV(
        xname: *const c_char,
        reg_left: c_uint,
        reg_right: c_uint,
        xshift: c_uint,
        xmin: c_uint,
        xmax: c_uint,
        xinvert: c_uint,
        tlv_array: *const c_uint,
    ) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_OUTPUT(wname: *const c_char) -> snd_soc_dapm_widget;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
