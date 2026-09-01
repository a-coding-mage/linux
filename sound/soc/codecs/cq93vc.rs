// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ALSA SoC CQ0093 Voice Codec Driver for DaVinci platforms
 *
 * Copyright (C) 2010 Texas Instruments, Inc
 *
 * Author: Miguel Aguilar <miguel.aguilar@ridgerun.com>
 */

use core::ffi::{c_char, c_int, c_uint};

extern "C" {
    static DAVINCI_VC_REG05: c_uint;
    static DAVINCI_VC_REG09: c_uint;
    static DAVINCI_VC_REG09_MUTE: u8;
    static DAVINCI_VC_REG12: c_uint;
    static DAVINCI_VC_REG12_POWER_ALL_ON: c_uint;
    static DAVINCI_VC_REG12_POWER_ALL_OFF: c_uint;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_FMTBIT_U8: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;

    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint)
        -> c_int;
    fn snd_soc_component_init_regmap(component: *mut snd_soc_component, regmap: *mut regmap);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const EINVAL: c_int = 22;

#[repr(C)]
pub struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    info: Option<unsafe extern "C" fn()>,
    get: Option<unsafe extern "C" fn()>,
    put: Option<unsafe extern "C" fn()>,
    private_value: usize,
}

#[repr(C)]
pub struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
pub struct device {
    platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct davinci_vc {
    regmap: *mut regmap,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    mute_stream:
        Option<unsafe extern "C" fn(dai: *mut snd_soc_dai, mute: c_int, direction: c_int) -> c_int>,
    set_sysclk: Option<
        unsafe extern "C" fn(
            codec_dai: *mut snd_soc_dai,
            clk_id: c_int,
            freq: c_uint,
            dir: c_int,
        ) -> c_int,
    >,
    no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    set_bias_level:
        Option<unsafe extern "C" fn(component: *mut snd_soc_component, level: snd_soc_bias_level) -> c_int>,
    probe: Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct platform_driver_inner {
    name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    driver: platform_driver_inner,
    probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
}

const fn soc_single(
    name: *const c_char,
    reg: c_uint,
    shift: c_uint,
    max: c_uint,
    invert: c_uint,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: 0,
        name,
        info: None,
        get: None,
        put: None,
        private_value: ((reg as usize) << 24)
            | ((shift as usize) << 16)
            | ((max as usize) << 8)
            | invert as usize,
    }
}

static CQ93VC_SND_CONTROLS: [snd_kcontrol_new; 2] = unsafe {
    [
        soc_single(
            b"PGA Capture Volume\0".as_ptr() as *const c_char,
            DAVINCI_VC_REG05,
            0,
            0x03,
            0,
        ),
        soc_single(
            b"Mono DAC Playback Volume\0".as_ptr() as *const c_char,
            DAVINCI_VC_REG09,
            0,
            0x3f,
            0,
        ),
    ]
};

unsafe extern "C" fn cq93vc_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let reg: u8;

    if mute != 0 {
        reg = DAVINCI_VC_REG09_MUTE;
    } else {
        reg = 0;
    }

    snd_soc_component_update_bits(
        component,
        DAVINCI_VC_REG09,
        DAVINCI_VC_REG09_MUTE as c_uint,
        reg as c_uint,
    );

    0
}

unsafe extern "C" fn cq93vc_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    match freq {
        22579200 | 27000000 | 33868800 => return 0,
        _ => {}
    }

    -EINVAL
}

unsafe extern "C" fn cq93vc_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {
            snd_soc_component_write(component, DAVINCI_VC_REG12, DAVINCI_VC_REG12_POWER_ALL_ON);
        }
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            snd_soc_component_write(component, DAVINCI_VC_REG12, DAVINCI_VC_REG12_POWER_ALL_OFF);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            /* force all power off */
            snd_soc_component_write(component, DAVINCI_VC_REG12, DAVINCI_VC_REG12_POWER_ALL_OFF);
        }
    }

    0
}

static CQ93VC_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    mute_stream: Some(cq93vc_mute),
    set_sysclk: Some(cq93vc_set_dai_sysclk),
    no_capture_mute: 1,
};

static mut CQ93VC_DAI: snd_soc_dai_driver = unsafe {
    snd_soc_dai_driver {
        name: b"cq93vc-hifi\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000,
            formats: SNDRV_PCM_FMTBIT_U8 | SNDRV_PCM_FMTBIT_S16_LE,
        },
        ops: &CQ93VC_DAI_OPS,
    }
};

unsafe extern "C" fn cq93vc_probe(component: *mut snd_soc_component) -> c_int {
    let davinci_vc: *mut davinci_vc = (*(*component).dev).platform_data as *mut davinci_vc;

    snd_soc_component_init_regmap(component, (*davinci_vc).regmap);

    0
}

static SOC_COMPONENT_DEV_CQ93VC: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(cq93vc_set_bias_level),
    probe: Some(cq93vc_probe),
    controls: CQ93VC_SND_CONTROLS.as_ptr(),
    num_controls: CQ93VC_SND_CONTROLS.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn cq93vc_platform_probe(pdev: *mut platform_device) -> c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &SOC_COMPONENT_DEV_CQ93VC,
        &mut CQ93VC_DAI,
        1,
    )
}

static CQ93VC_CODEC_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"cq93vc-codec\0".as_ptr() as *const c_char,
    },

    probe: Some(cq93vc_platform_probe),
};

/* module_platform_driver(cq93vc_codec_driver); */

/* MODULE_DESCRIPTION("Texas Instruments DaVinci ASoC CQ0093 Voice Codec Driver"); */
/* MODULE_AUTHOR("Miguel Aguilar"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
