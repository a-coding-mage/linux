// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for the 1250-EV1 audio I/O module
 *
 * Copyright 2011 Wolfson Microelectronics plc
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
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
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct wm1250_ev1_pdata {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wm1250_priv {
    clk_ena: *mut gpio_desc,
    clk_sel0: *mut gpio_desc,
    clk_sel1: *mut gpio_desc,
    osr: *mut gpio_desc,
    master: *mut gpio_desc,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON,
    SND_SOC_BIAS_PREPARE,
    SND_SOC_BIAS_STANDBY,
    SND_SOC_BIAS_OFF,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
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
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub set_bias_level:
        Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
pub struct driver_data {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: driver_data,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int);
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn i2c_smbus_read_byte_data(client: *mut i2c_client, command: u8) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

extern "Rust" {
    static SND_SOC_NOPM: c_int;
    static SNDRV_PCM_RATE_8000: c_uint;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_64000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static GFP_KERNEL: c_uint;
    static GPIOD_OUT_LOW: c_uint;
    static GPIOD_OUT_HIGH: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
    static ENODEV: c_int;

    fn SND_SOC_DAPM_ADC(
        name: *const c_char,
        stream: *const c_char,
        reg: c_int,
        shift: c_int,
        invert: c_int,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_DAC(
        name: *const c_char,
        stream: *const c_char,
        reg: c_int,
        shift: c_int,
        invert: c_int,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_INPUT(name: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUTPUT(name: *const c_char) -> snd_soc_dapm_widget;
}

unsafe extern "C" fn wm1250_ev1_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let wm1250 = dev_get_drvdata((*component).dev) as *mut wm1250_priv;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {}
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            gpiod_set_value_cansleep((*wm1250).clk_ena, 1);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            gpiod_set_value_cansleep((*wm1250).clk_ena, 0);
        }
    }

    0
}

static wm1250_ev1_dapm_widgets: [snd_soc_dapm_widget; 4] = unsafe {
    [
        SND_SOC_DAPM_ADC(
            b"ADC\0".as_ptr() as *const c_char,
            b"wm1250-ev1 Capture\0".as_ptr() as *const c_char,
            SND_SOC_NOPM,
            0,
            0,
        ),
        SND_SOC_DAPM_DAC(
            b"DAC\0".as_ptr() as *const c_char,
            b"wm1250-ev1 Playback\0".as_ptr() as *const c_char,
            SND_SOC_NOPM,
            0,
            0,
        ),
        SND_SOC_DAPM_INPUT(b"WM1250 Input\0".as_ptr() as *const c_char),
        SND_SOC_DAPM_OUTPUT(b"WM1250 Output\0".as_ptr() as *const c_char),
    ]
};

static wm1250_ev1_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"ADC\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"WM1250 Input\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"WM1250 Output\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"DAC\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn wm1250_ev1_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let wm1250 = snd_soc_component_get_drvdata((*dai).component) as *mut wm1250_priv;

    match params_rate(params) {
        8000 => {
            gpiod_set_value((*wm1250).clk_sel0, 1);
            gpiod_set_value((*wm1250).clk_sel1, 1);
        }
        16000 => {
            gpiod_set_value((*wm1250).clk_sel0, 0);
            gpiod_set_value((*wm1250).clk_sel1, 1);
        }
        32000 => {
            gpiod_set_value((*wm1250).clk_sel0, 1);
            gpiod_set_value((*wm1250).clk_sel1, 0);
        }
        64000 => {
            gpiod_set_value((*wm1250).clk_sel0, 0);
            gpiod_set_value((*wm1250).clk_sel1, 0);
        }
        _ => return -EINVAL,
    }

    0
}

static wm1250_ev1_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(wm1250_ev1_hw_params),
};

/* #define WM1250_EV1_RATES (SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 |\
 *                           SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_64000)
 */
static WM1250_EV1_RATES: c_uint = unsafe {
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_64000
};

static mut wm1250_ev1_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"wm1250-ev1\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: WM1250_EV1_RATES,
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 2,
        rates: WM1250_EV1_RATES,
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
    },
    ops: &wm1250_ev1_ops,
};

static soc_component_dev_wm1250_ev1: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: wm1250_ev1_dapm_widgets.as_ptr(),
    num_dapm_widgets: wm1250_ev1_dapm_widgets.len() as c_uint,
    dapm_routes: wm1250_ev1_dapm_routes.as_ptr(),
    num_dapm_routes: wm1250_ev1_dapm_routes.len() as c_uint,
    set_bias_level: Some(wm1250_ev1_set_bias_level),
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe extern "C" fn wm1250_ev1_pdata(i2c: *mut i2c_client) -> c_int {
    let pdata = dev_get_platdata(&mut (*i2c).dev) as *mut wm1250_ev1_pdata;
    let wm1250: *mut wm1250_priv;

    if pdata.is_null() {
        return 0;
    }

    wm1250 = devm_kzalloc(
        &mut (*i2c).dev,
        size_of::<wm1250_priv>(),
        GFP_KERNEL,
    ) as *mut wm1250_priv;
    if wm1250.is_null() {
        return -ENOMEM;
    }

    (*wm1250).clk_ena = devm_gpiod_get(
        &mut (*i2c).dev,
        b"clk-ena\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*wm1250).clk_ena as *const c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*wm1250).clk_ena as *const c_void),
            b"failed to get clock enable GPIO\n\0".as_ptr() as *const c_char,
        );
    }

    (*wm1250).clk_sel0 = devm_gpiod_get(
        &mut (*i2c).dev,
        b"clk-sel0\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR((*wm1250).clk_sel0 as *const c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*wm1250).clk_sel0 as *const c_void),
            b"failed to get clock sel0 GPIO\n\0".as_ptr() as *const c_char,
        );
    }

    (*wm1250).clk_sel1 = devm_gpiod_get(
        &mut (*i2c).dev,
        b"clk-sel1\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR((*wm1250).clk_sel1 as *const c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*wm1250).clk_sel1 as *const c_void),
            b"failed to get clock sel1 GPIO\n\0".as_ptr() as *const c_char,
        );
    }

    (*wm1250).osr = devm_gpiod_get(
        &mut (*i2c).dev,
        b"osr\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*wm1250).osr as *const c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*wm1250).osr as *const c_void),
            b"failed to get OSR GPIO\n\0".as_ptr() as *const c_char,
        );
    }

    (*wm1250).master = devm_gpiod_get(
        &mut (*i2c).dev,
        b"master\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*wm1250).master as *const c_void) {
        return dev_err_probe(
            &mut (*i2c).dev,
            PTR_ERR((*wm1250).master as *const c_void),
            b"failed to get MASTER GPIO\n\0".as_ptr() as *const c_char,
        );
    }

    dev_set_drvdata(&mut (*i2c).dev, wm1250 as *mut c_void);

    0
}

unsafe extern "C" fn wm1250_ev1_probe(i2c: *mut i2c_client) -> c_int {
    let id: c_int;
    let board: c_int;
    let rev: c_int;
    let mut ret: c_int;

    dev_set_drvdata(&mut (*i2c).dev, ptr::null_mut());

    board = i2c_smbus_read_byte_data(i2c, 0);
    if board < 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to read ID: %d\n\0".as_ptr() as *const c_char,
            board,
        );
        return board;
    }

    id = (board & 0xfe) >> 2;
    rev = board & 0x3;

    if id != 1 {
        dev_err(
            &mut (*i2c).dev,
            b"Unknown board ID %d\n\0".as_ptr() as *const c_char,
            id,
        );
        return -ENODEV;
    }

    dev_info(
        &mut (*i2c).dev,
        b"revision %d\n\0".as_ptr() as *const c_char,
        rev + 1,
    );

    ret = wm1250_ev1_pdata(i2c);
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &soc_component_dev_wm1250_ev1,
        &mut wm1250_ev1_dai,
        1,
    );
    if ret != 0 {
        dev_err(
            &mut (*i2c).dev,
            b"Failed to register CODEC: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    0
}

static wm1250_ev1_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'w' as c_char,
            b'm' as c_char,
            b'1' as c_char,
            b'2' as c_char,
            b'5' as c_char,
            b'0' as c_char,
            b'-' as c_char,
            b'e' as c_char,
            b'v' as c_char,
            b'1' as c_char,
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
/* MODULE_DEVICE_TABLE(i2c, wm1250_ev1_i2c_id); */

static mut wm1250_ev1_i2c_driver: i2c_driver = i2c_driver {
    driver: driver_data {
        name: b"wm1250-ev1\0".as_ptr() as *const c_char,
    },
    probe: Some(wm1250_ev1_probe),
    id_table: wm1250_ev1_i2c_id.as_ptr(),
};

/* module_i2c_driver(wm1250_ev1_i2c_driver); */

/* MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>"); */
/* MODULE_DESCRIPTION("WM1250-EV1 audio I/O module driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
