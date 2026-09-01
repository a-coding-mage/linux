// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC driver for
 *    Asahi Kasei AK5386 Single-ended 24-Bit 192kHz delta-sigma ADC
 *
 * (c) 2013 Daniel Mack <zonque@gmail.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 3;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0;
const SNDRV_PCM_FMTBIT_S8: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << 32;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct device {
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
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}

#[repr(C)]
pub struct ak5386_priv {
    pub reset_gpio: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; SUPPLY_NAMES.len()],
}

unsafe extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_regulator_bulk_get(
        dev: *mut device,
        num_consumers: c_int,
        consumers: *mut regulator_bulk_data,
    ) -> c_int;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

static SUPPLY_NAME_VA: &[u8] = b"va\0";
static SUPPLY_NAME_VD: &[u8] = b"vd\0";
static SUPPLY_NAMES: [*const c_char; 2] = [
    SUPPLY_NAME_VA.as_ptr() as *const c_char,
    SUPPLY_NAME_VD.as_ptr() as *const c_char,
];

static DAPM_AINL: &[u8] = b"AINL\0";
static DAPM_AINR: &[u8] = b"AINR\0";
static CAPTURE: &[u8] = b"Capture\0";
static INVALID_DAI_FORMAT: &[u8] = b"Invalid DAI format\n\0";
static DAI_NAME: &[u8] = b"ak5386-hifi\0";
static COMPATIBLE: &[u8] = b"asahi-kasei,ak5386\0";
static RESET: &[u8] = b"reset\0";
static AK5386_RESET: &[u8] = b"AK5386 Reset\0";
static RESET_GPIO_FAILED: &[u8] = b"Failed to get AK5386 reset GPIO\n\0";
static DRIVER_NAME: &[u8] = b"ak5386\0";

static AK5386_DAPM_WIDGETS: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget {
        id: 0,
        name: DAPM_AINL.as_ptr() as *const c_char,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: DAPM_AINR.as_ptr() as *const c_char,
    },
];

static AK5386_DAPM_ROUTES: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: CAPTURE.as_ptr() as *const c_char,
        control: ptr::null(),
        source: DAPM_AINL.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: CAPTURE.as_ptr() as *const c_char,
        control: ptr::null(),
        source: DAPM_AINR.as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn ak5386_soc_probe(component: *mut snd_soc_component) -> c_int {
    let priv_data = snd_soc_component_get_drvdata(component) as *mut ak5386_priv;
    regulator_bulk_enable(
        (*priv_data).supplies.len() as c_int,
        (*priv_data).supplies.as_mut_ptr(),
    )
}

unsafe extern "C" fn ak5386_soc_remove(component: *mut snd_soc_component) {
    let priv_data = snd_soc_component_get_drvdata(component) as *mut ak5386_priv;
    regulator_bulk_disable(
        (*priv_data).supplies.len() as c_int,
        (*priv_data).supplies.as_mut_ptr(),
    );
}

/* CONFIG_PM */
unsafe extern "C" fn ak5386_soc_suspend(component: *mut snd_soc_component) -> c_int {
    let priv_data = snd_soc_component_get_drvdata(component) as *mut ak5386_priv;
    regulator_bulk_disable(
        (*priv_data).supplies.len() as c_int,
        (*priv_data).supplies.as_mut_ptr(),
    );
    0
}

unsafe extern "C" fn ak5386_soc_resume(component: *mut snd_soc_component) -> c_int {
    let priv_data = snd_soc_component_get_drvdata(component) as *mut ak5386_priv;
    regulator_bulk_enable(
        (*priv_data).supplies.len() as c_int,
        (*priv_data).supplies.as_mut_ptr(),
    )
}

/* Without CONFIG_PM, ak5386_soc_suspend and ak5386_soc_resume are NULL. */

static SOC_COMPONENT_AK5386: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(ak5386_soc_probe),
    remove: Some(ak5386_soc_remove),
    suspend: Some(ak5386_soc_suspend),
    resume: Some(ak5386_soc_resume),
    dapm_widgets: AK5386_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: AK5386_DAPM_WIDGETS.len(),
    dapm_routes: AK5386_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: AK5386_DAPM_ROUTES.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn ak5386_set_dai_fmt(
    codec_dai: *mut snd_soc_dai,
    mut format: c_uint,
) -> c_int {
    let component = (*codec_dai).component;

    format &= SND_SOC_DAIFMT_FORMAT_MASK;
    if format != SND_SOC_DAIFMT_LEFT_J && format != SND_SOC_DAIFMT_I2S {
        dev_err((*component).dev, INVALID_DAI_FORMAT.as_ptr() as *const c_char);
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn ak5386_hw_params(
    _substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let priv_data = snd_soc_component_get_drvdata(component) as *mut ak5386_priv;

    /*
     * From the datasheet:
     *
     * All external clocks (MCLK, SCLK and LRCK) must be present unless
     * PDN pin = “L”. If these clocks are not provided, the AK5386 may
     * draw excess current due to its use of internal dynamically
     * refreshed logic. If the external clocks are not present, place
     * the AK5386 in power-down mode (PDN pin = “L”).
     */

    gpiod_set_value((*priv_data).reset_gpio, 1);

    0
}

unsafe extern "C" fn ak5386_hw_free(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let priv_data = snd_soc_component_get_drvdata(component) as *mut ak5386_priv;

    gpiod_set_value((*priv_data).reset_gpio, 0);

    0
}

static AK5386_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(ak5386_set_dai_fmt),
    hw_params: Some(ak5386_hw_params),
    hw_free: Some(ak5386_hw_free),
};

static mut AK5386_DAI: snd_soc_dai_driver = snd_soc_dai_driver {
    name: DAI_NAME.as_ptr() as *const c_char,
    capture: snd_soc_pcm_stream {
        stream_name: CAPTURE.as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: SNDRV_PCM_FMTBIT_S8
            | SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S24_3LE,
    },
    ops: &AK5386_DAI_OPS as *const snd_soc_dai_ops,
};

/* CONFIG_OF */
static AK5386_DT_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: COMPATIBLE.as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, ak5386_dt_ids); */

unsafe extern "C" fn ak5386_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret: c_int;
    let mut i: c_int;

    let priv_data = devm_kzalloc(
        dev,
        core::mem::size_of::<ak5386_priv>(),
        GFP_KERNEL,
    ) as *mut ak5386_priv;
    if priv_data.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, priv_data as *mut c_void);

    i = 0;
    while i < SUPPLY_NAMES.len() as c_int {
        (*priv_data).supplies[i as usize].supply = SUPPLY_NAMES[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        dev,
        (*priv_data).supplies.len() as c_int,
        (*priv_data).supplies.as_mut_ptr(),
    );
    if ret < 0 {
        return ret;
    }

    (*priv_data).reset_gpio =
        devm_gpiod_get_optional(dev, RESET.as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*priv_data).reset_gpio as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_data).reset_gpio as *const c_void),
            RESET_GPIO_FAILED.as_ptr() as *const c_char,
        );
    }

    gpiod_set_consumer_name(
        (*priv_data).reset_gpio,
        AK5386_RESET.as_ptr() as *const c_char,
    );

    devm_snd_soc_register_component(
        dev,
        &SOC_COMPONENT_AK5386 as *const snd_soc_component_driver,
        &mut AK5386_DAI as *mut snd_soc_dai_driver,
        1,
    )
}

static mut AK5386_DRIVER: platform_driver = platform_driver {
    probe: Some(ak5386_probe),
    driver: device_driver {
        name: DRIVER_NAME.as_ptr() as *const c_char,
        of_match_table: AK5386_DT_IDS.as_ptr(),
    },
};

/* module_platform_driver(ak5386_driver); */

/* MODULE_DESCRIPTION("ASoC driver for AK5386 ADC"); */
/* MODULE_AUTHOR("Daniel Mack <zonque@gmail.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
