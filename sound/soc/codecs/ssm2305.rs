// SPDX-License-Identifier: GPL-2.0
//
// Analog Devices SSM2305 Amplifier Driver
//
// Copyright (C) 2018 Pengutronix, Marco Felsch <kernel@pengutronix.de>
//

// C dependencies: <linux/gpio/consumer.h>, <linux/module.h>, <sound/soc.h>

const DRV_NAME: *const ::core::ffi::c_char = c"ssm2305".as_ptr();

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const ::core::ffi::c_char,
    pub control: *const ::core::ffi::c_char,
    pub source: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *const snd_soc_dapm_widget_data,
    pub num_dapm_widgets: ::core::ffi::c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: ::core::ffi::c_uint,
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
pub struct of_device_id {
    pub compatible: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const ::core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> ::core::ffi::c_int>,
}

#[repr(C)]
struct ssm2305 {
    /* shutdown gpio  */
    gpiod_shutdown: *mut gpio_desc,
}

unsafe extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut ::core::ffi::c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: ::core::ffi::c_int);
    fn devm_kzalloc(
        dev: *mut device,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut ::core::ffi::c_void);
    fn devm_gpiod_get(
        dev: *mut device,
        con_id: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> *mut gpio_desc;
    fn dev_err_probe(
        dev: *mut device,
        err: isize,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut ::core::ffi::c_void,
        num_dai: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn IS_ERR(ptr: *const ::core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const ::core::ffi::c_void) -> isize;
    fn SND_SOC_DAPM_EVENT_ON(event: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn of_match_ptr(ids: *const of_device_id) -> *const of_device_id;
}

const GFP_KERNEL: ::core::ffi::c_uint = 0;
const ENOMEM: ::core::ffi::c_int = 12;
const GPIOD_OUT_LOW: ::core::ffi::c_int = 0;
const SND_SOC_NOPM: ::core::ffi::c_int = 0;
const SND_SOC_DAPM_PRE_PMU: ::core::ffi::c_uint = 0;
const SND_SOC_DAPM_POST_PMD: ::core::ffi::c_uint = 0;

// Kernel ASoC DAPM construction macros are represented as external constants.
unsafe extern "C" {
    static SSM2305_DAPM_INPUT_L_IN: snd_soc_dapm_widget_data;
    static SSM2305_DAPM_INPUT_R_IN: snd_soc_dapm_widget_data;
    static SSM2305_DAPM_OUTPUT_L_OUT: snd_soc_dapm_widget_data;
    static SSM2305_DAPM_OUTPUT_R_OUT: snd_soc_dapm_widget_data;
    static SSM2305_DAPM_SUPPLY_POWER: snd_soc_dapm_widget_data;
}

unsafe extern "C" fn ssm2305_power_event(
    w: *mut snd_soc_dapm_widget,
    _kctrl: *mut snd_kcontrol,
    event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let c: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let data: *mut ssm2305 = snd_soc_component_get_drvdata(c) as *mut ssm2305;

    gpiod_set_value_cansleep((*data).gpiod_shutdown, SND_SOC_DAPM_EVENT_ON(event));

    0
}

static ssm2305_dapm_widgets: [snd_soc_dapm_widget_data; 5] = unsafe {
    [
        /* Stereo input/output */
        SSM2305_DAPM_INPUT_L_IN,
        SSM2305_DAPM_INPUT_R_IN,
        SSM2305_DAPM_OUTPUT_L_OUT,
        SSM2305_DAPM_OUTPUT_R_OUT,
        SSM2305_DAPM_SUPPLY_POWER,
    ]
};

static ssm2305_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: c"L_OUT".as_ptr(),
        control: ::core::ptr::null(),
        source: c"L_IN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"R_OUT".as_ptr(),
        control: ::core::ptr::null(),
        source: c"R_IN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"L_IN".as_ptr(),
        control: ::core::ptr::null(),
        source: c"Power".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"R_IN".as_ptr(),
        control: ::core::ptr::null(),
        source: c"Power".as_ptr(),
    },
];

static ssm2305_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: ssm2305_dapm_widgets.as_ptr(),
    num_dapm_widgets: ssm2305_dapm_widgets.len() as ::core::ffi::c_uint,
    dapm_routes: ssm2305_dapm_routes.as_ptr(),
    num_dapm_routes: ssm2305_dapm_routes.len() as ::core::ffi::c_uint,
};

unsafe extern "C" fn ssm2305_probe(pdev: *mut platform_device) -> ::core::ffi::c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let priv_: *mut ssm2305;

    /* Allocate the private data */
    priv_ = devm_kzalloc(dev, ::core::mem::size_of::<ssm2305>(), GFP_KERNEL) as *mut ssm2305;
    if priv_.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, priv_ as *mut ::core::ffi::c_void);

    /* Get shutdown gpio */
    (*priv_).gpiod_shutdown = devm_gpiod_get(dev, c"shutdown".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*priv_).gpiod_shutdown as *const ::core::ffi::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).gpiod_shutdown as *const ::core::ffi::c_void),
            c"Failed to get 'shutdown' gpio\n".as_ptr(),
        );
    }

    devm_snd_soc_register_component(
        dev,
        &ssm2305_component_driver,
        ::core::ptr::null_mut(),
        0,
    )
}

// Original C condition: #ifdef CONFIG_OF
#[cfg(CONFIG_OF)]
static ssm2305_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"adi,ssm2305".as_ptr(),
    },
    of_device_id {
        compatible: ::core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, ssm2305_of_match);

#[cfg(not(CONFIG_OF))]
static ssm2305_of_match: [of_device_id; 1] = [of_device_id {
    compatible: ::core::ptr::null(),
}];

static mut ssm2305_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: DRV_NAME,
        of_match_table: unsafe { of_match_ptr(ssm2305_of_match.as_ptr()) },
    },
    probe: Some(ssm2305_probe),
};

// module_platform_driver(ssm2305_driver);

// MODULE_DESCRIPTION("ASoC SSM2305 amplifier driver");
// MODULE_AUTHOR("Marco Felsch <m.felsch@pengutronix.de>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
