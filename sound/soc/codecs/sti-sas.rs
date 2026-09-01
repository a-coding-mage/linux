// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) STMicroelectronics SA 2015
 * Authors: Arnaud Pouliquen <arnaud.pouliquen@st.com>
 *          for STMicroelectronics.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

type u32 = c_uint;

/* External Linux/ALSA dependencies supplied by other files. */
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub id: c_int,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_uint,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

type reg_read_fn =
    Option<unsafe extern "C" fn(context: *mut c_void, reg: c_uint, value: *mut c_uint) -> c_int>;
type reg_write_fn =
    Option<unsafe extern "C" fn(context: *mut c_void, reg: c_uint, value: c_uint) -> c_int>;
type volatile_reg_fn = Option<unsafe extern "C" fn(dev: *mut device, reg: c_uint) -> bool>;

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub fast_io: bool,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub volatile_reg: volatile_reg_fn,
    pub cache_type: c_uint,
    pub reg_read: reg_read_fn,
    pub reg_write: reg_write_fn,
}

type dai_set_fmt_fn = Option<unsafe extern "C" fn(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int>;
type dai_mute_stream_fn =
    Option<unsafe extern "C" fn(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int>;
type dai_prepare_fn =
    Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int>;
type dai_set_sysclk_fn = Option<
    unsafe extern "C" fn(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int,
>;
type dai_trigger_fn = Option<
    unsafe extern "C" fn(
        substream: *mut snd_pcm_substream,
        cmd: c_int,
        dai: *mut snd_soc_dai,
    ) -> c_int,
>;

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: dai_set_fmt_fn,
    pub mute_stream: dai_mute_stream_fn,
    pub prepare: dai_prepare_fn,
    pub set_sysclk: dai_set_sysclk_fn,
    pub trigger: dai_trigger_fn,
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
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub sname: *const c_char,
    pub reg: c_uint,
    pub shift: c_uint,
    pub invert: c_uint,
    pub event: *const c_void,
    pub event_flags: c_uint,
    pub kind: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

type component_probe_fn =
    Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> c_int>;
type component_resume_fn =
    Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> c_int>;

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: component_probe_fn,
    pub resume: component_resume_fn,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

type platform_probe_fn = Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>;

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: platform_probe_fn,
}

unsafe extern "C" {
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const c_void,
        context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn syscon_regmap_lookup_by_phandle(np: *mut device_node, property: *const c_char)
        -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn of_match_node(matches: *const of_device_id, node: *mut device_node) -> *const of_device_id;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;

const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const SND_SOC_CLOCK_OUT: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 1;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 2;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_STOP: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 1;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_64000: c_uint = 1 << 3;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 4;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 5;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 6;
const SNDRV_PCM_RATE_8000_48000: c_uint = 1 << 7;
const SNDRV_PCM_FMTBIT_S16_LE: c_uint = 1 << 0;
const SNDRV_PCM_FMTBIT_S32_LE: c_uint = 1 << 1;

/* DAC definitions */

/* stih407 DAC registers */
/* sysconf 5041: Audio-Gue-Control */
const STIH407_AUDIO_GLUE_CTRL: c_uint = 0x000000A4;
/* sysconf 5042: Audio-DAC-Control */
const STIH407_AUDIO_DAC_CTRL: c_uint = 0x000000A8;

/* DAC definitions */
const STIH407_DAC_SOFTMUTE: c_uint = 0x0;
const STIH407_DAC_STANDBY_ANA: c_uint = 0x1;
const STIH407_DAC_STANDBY: c_uint = 0x2;

const STIH407_DAC_SOFTMUTE_MASK: c_uint = BIT(STIH407_DAC_SOFTMUTE);
const STIH407_DAC_STANDBY_ANA_MASK: c_uint = BIT(STIH407_DAC_STANDBY_ANA);
const STIH407_DAC_STANDBY_MASK: c_uint = BIT(STIH407_DAC_STANDBY);

/* SPDIF definitions */
const SPDIF_BIPHASE_ENABLE: c_uint = 0x6;
const SPDIF_BIPHASE_IDLE: c_uint = 0x7;

const SPDIF_BIPHASE_ENABLE_MASK: c_uint = BIT(SPDIF_BIPHASE_ENABLE);
const SPDIF_BIPHASE_IDLE_MASK: c_uint = BIT(SPDIF_BIPHASE_IDLE);

const STI_SAS_DAI_SPDIF_OUT: c_int = 0;
const STI_SAS_DAI_ANALOG_OUT: c_int = 1;

static stih407_sas_reg_defaults: [reg_default; 2] = [
    reg_default {
        reg: STIH407_AUDIO_GLUE_CTRL,
        def: 0x00000040,
    },
    reg_default {
        reg: STIH407_AUDIO_DAC_CTRL,
        def: 0x000000000,
    },
];

#[repr(C)]
pub struct sti_dac_audio {
    pub regmap: *mut regmap,
    pub virt_regmap: *mut regmap,
    pub mclk: c_int,
}

#[repr(C)]
pub struct sti_spdif_audio {
    pub regmap: *mut regmap,
    pub mclk: c_int,
}

/* device data structure */
#[repr(C)]
pub struct sti_sas_dev_data {
    pub regmap: *const regmap_config,
    pub dac_ops: *const snd_soc_dai_ops, /* DAC function callbacks */
}

/* driver data structure */
#[repr(C)]
pub struct sti_sas_data {
    pub dev: *mut device,
    pub dev_data: *const sti_sas_dev_data,
    pub dac: sti_dac_audio,
    pub spdif: sti_spdif_audio,
}

/* Read a register from the sysconf reg bank */
unsafe extern "C" fn sti_sas_read_reg(
    context: *mut c_void,
    reg: c_uint,
    value: *mut c_uint,
) -> c_int {
    let drvdata = context as *mut sti_sas_data;
    let mut val: u32 = 0;

    let status = unsafe { regmap_read((*drvdata).dac.regmap, reg, &mut val) };
    unsafe {
        *value = val as c_uint;
    }

    status
}

/* Read a register from the sysconf reg bank */
unsafe extern "C" fn sti_sas_write_reg(
    context: *mut c_void,
    reg: c_uint,
    value: c_uint,
) -> c_int {
    let drvdata = context as *mut sti_sas_data;

    unsafe { regmap_write((*drvdata).dac.regmap, reg, value) }
}

unsafe extern "C" fn sti_sas_init_sas_registers(
    component: *mut snd_soc_component,
    data: *mut sti_sas_data,
) -> c_int {
    let mut ret: c_int;
    /*
     * DAC and SPDIF are activated by default
     * put them in IDLE to save power
     */
    let _ = data;

    /* Initialise bi-phase formatter to disabled */
    ret = unsafe {
        snd_soc_component_update_bits(component, STIH407_AUDIO_GLUE_CTRL, SPDIF_BIPHASE_ENABLE_MASK, 0)
    };

    if ret == 0 {
        /* Initialise bi-phase formatter idle value to 0 */
        ret = unsafe {
            snd_soc_component_update_bits(component, STIH407_AUDIO_GLUE_CTRL, SPDIF_BIPHASE_IDLE_MASK, 0)
        };
    }
    if ret < 0 {
        unsafe {
            dev_err(
                (*component).dev,
                b"Failed to update SPDIF registers\n\0".as_ptr() as *const c_char,
            );
        }
        return ret;
    }

    /* Init DAC configuration */
    /* init configuration */
    ret = unsafe {
        snd_soc_component_update_bits(
            component,
            STIH407_AUDIO_DAC_CTRL,
            STIH407_DAC_STANDBY_MASK,
            STIH407_DAC_STANDBY_MASK,
        )
    };

    if ret == 0 {
        ret = unsafe {
            snd_soc_component_update_bits(
                component,
                STIH407_AUDIO_DAC_CTRL,
                STIH407_DAC_STANDBY_ANA_MASK,
                STIH407_DAC_STANDBY_ANA_MASK,
            )
        };
    }
    if ret == 0 {
        ret = unsafe {
            snd_soc_component_update_bits(
                component,
                STIH407_AUDIO_DAC_CTRL,
                STIH407_DAC_SOFTMUTE_MASK,
                STIH407_DAC_SOFTMUTE_MASK,
            )
        };
    }

    if ret < 0 {
        unsafe {
            dev_err(
                (*component).dev,
                b"Failed to update DAC registers\n\0".as_ptr() as *const c_char,
            );
        }
        return ret;
    }

    ret
}

/*
 * DAC
 */
unsafe extern "C" fn sti_sas_dac_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    /* Sanity check only */
    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC {
        unsafe {
            dev_err(
                (*(*dai).component).dev,
                b"%s: ERROR: Unsupported clocking 0x%x\n\0".as_ptr() as *const c_char,
                b"sti_sas_dac_set_fmt\0".as_ptr() as *const c_char,
                fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK,
            );
        }
        return -EINVAL;
    }

    0
}

const SND_SOC_DAPM_OUT_DRV_KIND: c_uint = 0;
const SND_SOC_DAPM_DAC_KIND: c_uint = 1;
const SND_SOC_DAPM_OUTPUT_KIND: c_uint = 2;

static stih407_sas_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget {
        name: b"DAC standby ana\0".as_ptr() as *const c_char,
        sname: ptr::null(),
        reg: STIH407_AUDIO_DAC_CTRL,
        shift: STIH407_DAC_STANDBY_ANA,
        invert: 1,
        event: ptr::null(),
        event_flags: 0,
        kind: SND_SOC_DAPM_OUT_DRV_KIND,
    },
    snd_soc_dapm_widget {
        name: b"DAC standby\0".as_ptr() as *const c_char,
        sname: b"dac_p\0".as_ptr() as *const c_char,
        reg: STIH407_AUDIO_DAC_CTRL,
        shift: STIH407_DAC_STANDBY,
        invert: 1,
        event: ptr::null(),
        event_flags: 0,
        kind: SND_SOC_DAPM_DAC_KIND,
    },
    snd_soc_dapm_widget {
        name: b"DAC Output\0".as_ptr() as *const c_char,
        sname: ptr::null(),
        reg: 0,
        shift: 0,
        invert: 0,
        event: ptr::null(),
        event_flags: 0,
        kind: SND_SOC_DAPM_OUTPUT_KIND,
    },
];

static stih407_sas_route: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"DAC Output\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC standby ana\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"DAC standby ana\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC standby\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn stih407_sas_dac_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    stream: c_int,
) -> c_int {
    let component = unsafe { (*dai).component };
    let _ = stream;

    if mute != 0 {
        unsafe {
            snd_soc_component_update_bits(
                component,
                STIH407_AUDIO_DAC_CTRL,
                STIH407_DAC_SOFTMUTE_MASK,
                STIH407_DAC_SOFTMUTE_MASK,
            )
        }
    } else {
        unsafe {
            snd_soc_component_update_bits(
                component,
                STIH407_AUDIO_DAC_CTRL,
                STIH407_DAC_SOFTMUTE_MASK,
                0,
            )
        }
    }
}

/*
 * SPDIF
 */
unsafe extern "C" fn sti_sas_spdif_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC {
        unsafe {
            dev_err(
                (*(*dai).component).dev,
                b"%s: ERROR: Unsupported clocking mask 0x%x\n\0".as_ptr() as *const c_char,
                b"sti_sas_spdif_set_fmt\0".as_ptr() as *const c_char,
                fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK,
            );
        }
        return -EINVAL;
    }

    0
}

/*
 * sti_sas_spdif_trigger:
 * Trigger function is used to ensure that BiPhase Formater is disabled
 * before CPU dai is stopped.
 * This is mandatory to avoid that BPF is stalled
 */
unsafe extern "C" fn sti_sas_spdif_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = unsafe { (*dai).component };
    let _ = substream;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => unsafe {
            snd_soc_component_update_bits(
                component,
                STIH407_AUDIO_GLUE_CTRL,
                SPDIF_BIPHASE_ENABLE_MASK,
                SPDIF_BIPHASE_ENABLE_MASK,
            )
        },
        SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_PUSH
        | SNDRV_PCM_TRIGGER_STOP
        | SNDRV_PCM_TRIGGER_SUSPEND => unsafe {
            snd_soc_component_update_bits(
                component,
                STIH407_AUDIO_GLUE_CTRL,
                SPDIF_BIPHASE_ENABLE_MASK,
                0,
            )
        },
        _ => -EINVAL,
    }
}

unsafe extern "C" fn sti_sas_volatile_register(dev: *mut device, reg: c_uint) -> bool {
    let _ = dev;
    if reg == STIH407_AUDIO_GLUE_CTRL {
        return true;
    }

    false
}

/*
 * CODEC DAIS
 */

/*
 * sti_sas_set_sysclk:
 * get MCLK input frequency to check that MCLK-FS ratio is coherent
 */
unsafe extern "C" fn sti_sas_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let component = unsafe { (*dai).component };
    let drvdata = unsafe { dev_get_drvdata((*component).dev) as *mut sti_sas_data };

    if dir == SND_SOC_CLOCK_OUT {
        return 0;
    }

    if clk_id != 0 {
        return -EINVAL;
    }

    match unsafe { (*dai).id } {
        STI_SAS_DAI_SPDIF_OUT => unsafe {
            (*drvdata).spdif.mclk = freq as c_int;
        },
        STI_SAS_DAI_ANALOG_OUT => unsafe {
            (*drvdata).dac.mclk = freq as c_int;
        },
        _ => {}
    }

    0
}

unsafe extern "C" fn sti_sas_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = unsafe { (*dai).component };
    let drvdata = unsafe { dev_get_drvdata((*component).dev) as *mut sti_sas_data };
    let runtime = unsafe { (*substream).runtime };

    match unsafe { (*dai).id } {
        STI_SAS_DAI_SPDIF_OUT => {
            if unsafe { (*drvdata).spdif.mclk / (*runtime).rate as c_int } != 128 {
                unsafe {
                    dev_err(
                        (*component).dev,
                        b"unexpected mclk-fs ratio\n\0".as_ptr() as *const c_char,
                    );
                }
                return -EINVAL;
            }
        }
        STI_SAS_DAI_ANALOG_OUT => {
            if unsafe { (*drvdata).dac.mclk / (*runtime).rate as c_int } != 256 {
                unsafe {
                    dev_err(
                        (*component).dev,
                        b"unexpected mclk-fs ratio\n\0".as_ptr() as *const c_char,
                    );
                }
                return -EINVAL;
            }
        }
        _ => {}
    }

    0
}

static stih407_dac_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(sti_sas_dac_set_fmt),
    mute_stream: Some(stih407_sas_dac_mute),
    prepare: Some(sti_sas_prepare),
    set_sysclk: Some(sti_sas_set_sysclk),
    trigger: None,
};

static stih407_sas_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    fast_io: true,
    max_register: STIH407_AUDIO_DAC_CTRL,
    reg_defaults: stih407_sas_reg_defaults.as_ptr(),
    num_reg_defaults: stih407_sas_reg_defaults.len() as c_uint,
    volatile_reg: Some(sti_sas_volatile_register),
    cache_type: REGCACHE_MAPLE,
    reg_read: Some(sti_sas_read_reg),
    reg_write: Some(sti_sas_write_reg),
};

static stih407_data: sti_sas_dev_data = sti_sas_dev_data {
    regmap: &stih407_sas_regmap,
    dac_ops: &stih407_dac_ops,
};

static sti_sas_spdif_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(sti_sas_spdif_set_fmt),
    mute_stream: None,
    prepare: Some(sti_sas_prepare),
    set_sysclk: Some(sti_sas_set_sysclk),
    trigger: Some(sti_sas_spdif_trigger),
};

static mut sti_sas_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: b"sas-dai-spdif-out\0".as_ptr() as *const c_char,
        id: STI_SAS_DAI_SPDIF_OUT,
        playback: snd_soc_pcm_stream {
            stream_name: b"spdif_p\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_44100
                | SNDRV_PCM_RATE_48000
                | SNDRV_PCM_RATE_64000
                | SNDRV_PCM_RATE_88200
                | SNDRV_PCM_RATE_96000
                | SNDRV_PCM_RATE_192000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
        },
        ops: &sti_sas_spdif_ops,
    },
    snd_soc_dai_driver {
        name: b"sas-dai-dac\0".as_ptr() as *const c_char,
        id: STI_SAS_DAI_ANALOG_OUT,
        playback: snd_soc_pcm_stream {
            stream_name: b"dac_p\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_48000,
            formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE,
        },
        ops: ptr::null(),
    },
];

/* CONFIG_PM_SLEEP: the C source defines sti_sas_resume only when PM sleep is enabled,
 * otherwise the component resume callback is NULL.
 */
unsafe extern "C" fn sti_sas_resume(component: *mut snd_soc_component) -> c_int {
    let drvdata = unsafe { dev_get_drvdata((*component).dev) as *mut sti_sas_data };

    unsafe { sti_sas_init_sas_registers(component, drvdata) }
}

unsafe extern "C" fn sti_sas_component_probe(component: *mut snd_soc_component) -> c_int {
    let drvdata = unsafe { dev_get_drvdata((*component).dev) as *mut sti_sas_data };

    unsafe { sti_sas_init_sas_registers(component, drvdata) }
}

static sti_sas_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(sti_sas_component_probe),
    resume: Some(sti_sas_resume),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
    dapm_widgets: stih407_sas_dapm_widgets.as_ptr(),
    num_dapm_widgets: stih407_sas_dapm_widgets.len() as c_uint,
    dapm_routes: stih407_sas_route.as_ptr(),
    num_dapm_routes: stih407_sas_route.len() as c_uint,
};

static sti_sas_dev_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"st,stih407-sas-codec\0".as_ptr() as *const c_char,
        data: &stih407_data as *const sti_sas_dev_data as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, sti_sas_dev_match); */

unsafe extern "C" fn sti_sas_driver_probe(pdev: *mut platform_device) -> c_int {
    let pnode = unsafe { (*pdev).dev.of_node };
    let drvdata: *mut sti_sas_data;
    let of_id: *const of_device_id;

    /* Allocate device structure */
    drvdata = unsafe {
        devm_kzalloc(
            &mut (*pdev).dev,
            core::mem::size_of::<sti_sas_data>(),
            GFP_KERNEL,
        ) as *mut sti_sas_data
    };
    if drvdata.is_null() {
        return -ENOMEM;
    }

    /* Populate data structure depending on compatibility */
    of_id = unsafe { of_match_node(sti_sas_dev_match.as_ptr(), pnode) };
    if unsafe { (*of_id).data.is_null() } {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                b"data associated to device is missing\n\0".as_ptr() as *const c_char,
            );
        }
        return -EINVAL;
    }

    unsafe {
        (*drvdata).dev_data = (*of_id).data as *const sti_sas_dev_data;
    }

    /* Initialise device structure */
    unsafe {
        (*drvdata).dev = &mut (*pdev).dev;
    }

    /* Request the DAC & SPDIF registers memory region */
    unsafe {
        (*drvdata).dac.virt_regmap = devm_regmap_init(
            &mut (*pdev).dev,
            ptr::null(),
            drvdata as *mut c_void,
            (*(*drvdata).dev_data).regmap,
        );
    }
    if unsafe { IS_ERR((*drvdata).dac.virt_regmap as *const c_void) } {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                b"audio registers not enabled\n\0".as_ptr() as *const c_char,
            );
            return PTR_ERR((*drvdata).dac.virt_regmap as *const c_void);
        }
    }

    /* Request the syscon region */
    unsafe {
        (*drvdata).dac.regmap =
            syscon_regmap_lookup_by_phandle(pnode, b"st,syscfg\0".as_ptr() as *const c_char);
    }
    if unsafe { IS_ERR((*drvdata).dac.regmap as *const c_void) } {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                b"syscon registers not available\n\0".as_ptr() as *const c_char,
            );
            return PTR_ERR((*drvdata).dac.regmap as *const c_void);
        }
    }
    unsafe {
        (*drvdata).spdif.regmap = (*drvdata).dac.regmap;
    }

    unsafe {
        sti_sas_dai[STI_SAS_DAI_ANALOG_OUT as usize].ops = (*(*drvdata).dev_data).dac_ops;
    }

    /* Store context */
    unsafe {
        dev_set_drvdata(&mut (*pdev).dev, drvdata as *mut c_void);
    }

    unsafe {
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &sti_sas_driver,
            sti_sas_dai.as_mut_ptr(),
            sti_sas_dai.len() as c_int,
        )
    }
}

static mut sti_sas_platform_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"sti-sas-codec\0".as_ptr() as *const c_char,
        of_match_table: sti_sas_dev_match.as_ptr(),
    },
    probe: Some(sti_sas_driver_probe),
};

/* module_platform_driver(sti_sas_platform_driver); */

/* MODULE_DESCRIPTION("audio codec for STMicroelectronics sti platforms"); */
/* MODULE_AUTHOR("Arnaud.pouliquen@st.com"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
