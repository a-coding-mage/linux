// SPDX-License-Identifier: GPL-2.0-only
/*
 * dmic.c  --  SoC audio for Generic Digital MICs
 *
 * Author: Liam Girdwood <lrg@slimlogic.co.uk>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const MAX_MODESWITCH_DELAY: c_int = 70;

static mut modeswitch_delay: c_int = 0;
static mut wakeup_delay: c_int = 0;

#[repr(C)]
struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_widget {
    dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
struct dmic {
    gpio_en: *mut gpio_desc,
    vref: *mut regulator,
    wakeup_delay: c_int,
    /* Delay after DMIC mode switch */
    modeswitch_delay: c_int,
}

#[repr(C)]
struct snd_soc_dai_ops {
    trigger: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            cmd: c_int,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_dapm_widget_def {
    name: *const c_char,
    stream_name: *const c_char,
    reg: c_int,
    shift: c_uint,
    invert: c_uint,
    event: Option<
        unsafe extern "C" fn(
            w: *mut snd_soc_dapm_widget,
            kcontrol: *mut snd_kcontrol,
            event: c_int,
        ) -> c_int,
    >,
    event_flags: c_uint,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(component: *mut snd_soc_component) -> c_int>,
    dapm_widgets: *const snd_soc_dapm_widget_def,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    idle_bias_on: c_uint,
    use_pmdown_time: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct platform_driver_inner {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    driver: platform_driver_inner,
    probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
}

const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 0x10;
const SND_SOC_DAPM_POST_PMD: c_int = 0x20;
const SND_SOC_NOPM: c_int = -1;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
const SNDRV_PCM_FMTBIT_DSD_U8: u64 = 1 << 48;
const SNDRV_PCM_FMTBIT_DSD_U16_LE: u64 = 1 << 49;
const SNDRV_PCM_FMTBIT_DSD_U32_LE: u64 = 1 << 50;
const SNDRV_PCM_FMTBIT_DSD_U16_BE: u64 = 1 << 51;
const SNDRV_PCM_FMTBIT_DSD_U32_BE: u64 = 1 << 52;
const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const GPIOD_OUT_LOW: c_int = 0;

unsafe extern "C" {
    fn mdelay(msecs: c_uint);
    fn msleep(msecs: c_uint);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_regulator_get_optional(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn device_property_read_u32(dev: *mut device, propname: *const c_char, val: *mut c_int)
        -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn of_property_read_u32(
        np: *mut device_node,
        propname: *const c_char,
        out_value: *mut c_uint,
    ) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as isize) >= -4095isize && (ptr as isize) < 0
}

unsafe fn PTR_ERR<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

unsafe extern "C" fn dmic_daiops_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let dmic = snd_soc_component_get_drvdata(component) as *mut dmic;

    match cmd {
        SNDRV_PCM_TRIGGER_STOP => {
            if (*dmic).modeswitch_delay != 0 {
                mdelay((*dmic).modeswitch_delay as c_uint);
            }
        }
        _ => {}
    }

    0
}

static dmic_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    trigger: Some(dmic_daiops_trigger),
};

unsafe extern "C" fn dmic_aif_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let dmic = snd_soc_component_get_drvdata(component) as *mut dmic;
    let mut ret: c_int = 0;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            if !(*dmic).gpio_en.is_null() {
                gpiod_set_value_cansleep((*dmic).gpio_en, 1);
            }

            if !(*dmic).vref.is_null() {
                ret = regulator_enable((*dmic).vref);
                if ret != 0 {
                    return ret;
                }
            }

            if (*dmic).wakeup_delay != 0 {
                msleep((*dmic).wakeup_delay as c_uint);
            }
        }
        SND_SOC_DAPM_POST_PMD => {
            if !(*dmic).gpio_en.is_null() {
                gpiod_set_value_cansleep((*dmic).gpio_en, 0);
            }

            if !(*dmic).vref.is_null() {
                ret = regulator_disable((*dmic).vref);
            }
        }
        _ => {}
    }

    ret
}

static mut dmic_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"dmic-hifi\0".as_ptr() as *const c_char,
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        formats: SNDRV_PCM_FMTBIT_S32_LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_DSD_U8
            | SNDRV_PCM_FMTBIT_DSD_U16_LE
            | SNDRV_PCM_FMTBIT_DSD_U32_LE
            | SNDRV_PCM_FMTBIT_DSD_U16_BE
            | SNDRV_PCM_FMTBIT_DSD_U32_BE,
    },
    ops: &dmic_dai_ops,
};

unsafe extern "C" fn dmic_component_probe(component: *mut snd_soc_component) -> c_int {
    let dmic: *mut dmic;

    dmic = devm_kzalloc((*component).dev, size_of::<dmic>(), GFP_KERNEL) as *mut dmic;
    if dmic.is_null() {
        return -ENOMEM;
    }

    (*dmic).vref = devm_regulator_get_optional((*component).dev, b"vref\0".as_ptr() as *const c_char);
    if IS_ERR((*dmic).vref) {
        if PTR_ERR((*dmic).vref) != -ENODEV {
            return dev_err_probe(
                (*component).dev,
                PTR_ERR((*dmic).vref),
                b"Failed to get vref\n\0".as_ptr() as *const c_char,
            );
        }
        (*dmic).vref = ptr::null_mut();
    }

    (*dmic).gpio_en = devm_gpiod_get_optional(
        (*component).dev,
        b"dmicen\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*dmic).gpio_en) {
        return PTR_ERR((*dmic).gpio_en);
    }

    device_property_read_u32(
        (*component).dev,
        b"wakeup-delay-ms\0".as_ptr() as *const c_char,
        &mut (*dmic).wakeup_delay,
    );
    device_property_read_u32(
        (*component).dev,
        b"modeswitch-delay-ms\0".as_ptr() as *const c_char,
        &mut (*dmic).modeswitch_delay,
    );
    if wakeup_delay != 0 {
        (*dmic).wakeup_delay = wakeup_delay;
    }
    if modeswitch_delay != 0 {
        (*dmic).modeswitch_delay = modeswitch_delay;
    }

    if (*dmic).modeswitch_delay > MAX_MODESWITCH_DELAY {
        (*dmic).modeswitch_delay = MAX_MODESWITCH_DELAY;
    }

    snd_soc_component_set_drvdata(component, dmic as *mut c_void);

    0
}

static dmic_dapm_widgets: [snd_soc_dapm_widget_def; 2] = [
    snd_soc_dapm_widget_def {
        name: b"DMIC AIF\0".as_ptr() as *const c_char,
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        event: Some(dmic_aif_event),
        event_flags: (SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD) as c_uint,
    },
    snd_soc_dapm_widget_def {
        name: b"DMic\0".as_ptr() as *const c_char,
        stream_name: ptr::null(),
        reg: 0,
        shift: 0,
        invert: 0,
        event: None,
        event_flags: 0,
    },
];

static intercon: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: b"DMIC AIF\0".as_ptr() as *const c_char,
    control: ptr::null(),
    source: b"DMic\0".as_ptr() as *const c_char,
}];

static soc_dmic: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(dmic_component_probe),
    dapm_widgets: dmic_dapm_widgets.as_ptr(),
    num_dapm_widgets: dmic_dapm_widgets.len() as c_uint,
    dapm_routes: intercon.as_ptr(),
    num_dapm_routes: intercon.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn dmic_dev_probe(pdev: *mut platform_device) -> c_int {
    let mut err: c_int;
    let mut chans: c_uint = 0;
    let mut dai_drv: *mut snd_soc_dai_driver = &mut dmic_dai;

    if !(*pdev).dev.of_node.is_null() {
        err = of_property_read_u32(
            (*pdev).dev.of_node,
            b"num-channels\0".as_ptr() as *const c_char,
            &mut chans,
        );
        if err != 0 && err != -EINVAL {
            return err;
        }

        if err == 0 {
            if chans < 1 || chans > 8 {
                return -EINVAL;
            }

            dai_drv = devm_kzalloc(
                &mut (*pdev).dev,
                size_of::<snd_soc_dai_driver>(),
                GFP_KERNEL,
            ) as *mut snd_soc_dai_driver;
            if dai_drv.is_null() {
                return -ENOMEM;
            }

            memcpy(
                dai_drv as *mut c_void,
                &dmic_dai as *const snd_soc_dai_driver as *const c_void,
                size_of::<snd_soc_dai_driver>(),
            );
            (*dai_drv).capture.channels_max = chans;
        }
    }

    devm_snd_soc_register_component(&mut (*pdev).dev, &soc_dmic, dai_drv, 1)
}

/* MODULE_ALIAS("platform:dmic-codec"); */

static dmic_dev_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"dmic-codec\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, dmic_dev_match); */

static mut dmic_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"dmic-codec\0".as_ptr() as *const c_char,
        of_match_table: dmic_dev_match.as_ptr(),
    },
    probe: Some(dmic_dev_probe),
};

/* module_platform_driver(dmic_driver); */

/* MODULE_DESCRIPTION("Generic DMIC driver"); */
/* MODULE_AUTHOR("Liam Girdwood <lrg@slimlogic.co.uk>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
