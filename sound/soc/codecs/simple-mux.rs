// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2020 Bootlin SA
 * Author: Alexandre Belloni <alexandre.belloni@bootlin.com>
 */

// Original C dependencies:
// <linux/gpio/consumer.h>
// <linux/module.h>
// <linux/mux/driver.h>
// <linux/regulator/consumer.h>
// <sound/soc.h>

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const MUX_TEXT_SIZE: usize = 2;
const MUX_WIDGET_SIZE: usize = 4;
const MUX_ROUTE_SIZE: usize = 3;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const MUX_IDLE_AS_IS: c_uint = c_uint::MAX;
const SND_SOC_BIAS_PREPARE: c_int = 2;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const SND_SOC_NOPM: c_int = -1;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_enum {
    pub items: c_uint,
    pub texts: *const *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
    pub private_value: c_ulong,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub kcontrol_news: *const snd_kcontrol_new,
    pub dapm: *mut snd_soc_dapm_context,
    pub event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    pub event_flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_component_driver {
    pub dapm_widgets: *mut snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *mut snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub read: Option<unsafe extern "C" fn(*mut snd_soc_component, c_uint) -> c_uint>,
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
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct simple_mux {
    pub gpiod_mux: *mut gpio_desc,
    pub mux: c_uint,
    pub mux_texts: [*const c_char; MUX_TEXT_SIZE],
    pub idle_state: c_uint,
    pub mux_enum: soc_enum,
    pub mux_mux: snd_kcontrol_new,
    pub mux_widgets: [snd_soc_dapm_widget; MUX_WIDGET_SIZE],
    pub mux_routes: [snd_soc_dapm_route; MUX_ROUTE_SIZE],
    pub mux_driver: snd_soc_component_driver,
}

unsafe extern "C" {
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_uint);
    fn snd_soc_dapm_mux_update_power(
        dapm: *mut snd_soc_dapm_context,
        kcontrol: *mut snd_kcontrol,
        mux: c_uint,
        e: *mut soc_enum,
        update: *mut c_void,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn of_property_read_string_array(
        np: *mut device_node,
        propname: *const c_char,
        out_strs: *mut *const c_char,
        sz: usize,
    ) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *mut snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
}

static INPUT_1: &[u8] = b"Input 1\0";
static INPUT_2: &[u8] = b"Input 2\0";
static MUXER: &[u8] = b"Muxer\0";
static IN1: &[u8] = b"IN1\0";
static IN2: &[u8] = b"IN2\0";
static MUX: &[u8] = b"MUX\0";
static OUT: &[u8] = b"OUT\0";
static MUX_GPIO: &[u8] = b"mux\0";
static STATE_LABELS: &[u8] = b"state-labels\0";
static IDLE_STATE: &[u8] = b"idle-state\0";
static SIMPLE_AUDIO_MUX: &[u8] = b"simple-audio-mux\0";
static SIMPLE_MUX_NAME: &[u8] = b"simple-mux\0";

static SIMPLE_MUX_TEXTS: [*const c_char; MUX_TEXT_SIZE] = [
    INPUT_1.as_ptr() as *const c_char,
    INPUT_2.as_ptr() as *const c_char,
];

static SIMPLE_MUX_ENUM: soc_enum = soc_enum {
    items: MUX_TEXT_SIZE as c_uint,
    texts: SIMPLE_MUX_TEXTS.as_ptr(),
};

unsafe extern "C" fn simple_mux_control_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let c = snd_soc_dapm_to_component(dapm);
    let priv_ = snd_soc_component_get_drvdata(c) as *mut simple_mux;

    (*ucontrol).value.enumerated.item[0] = (*priv_).mux;

    0
}

unsafe extern "C" fn simple_mux_control_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let c = snd_soc_dapm_to_component(dapm);
    let priv_ = snd_soc_component_get_drvdata(c) as *mut simple_mux;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    if (*priv_).mux == (*ucontrol).value.enumerated.item[0] {
        return 0;
    }

    (*priv_).mux = (*ucontrol).value.enumerated.item[0];

    if (*priv_).idle_state != MUX_IDLE_AS_IS
        && snd_soc_dapm_get_bias_level(dapm) < SND_SOC_BIAS_PREPARE
    {
        return 0;
    }

    gpiod_set_value_cansleep((*priv_).gpiod_mux, (*priv_).mux);

    snd_soc_dapm_mux_update_power(
        dapm,
        kcontrol,
        (*ucontrol).value.enumerated.item[0],
        e,
        ptr::null_mut(),
    )
}

unsafe extern "C" fn simple_mux_read(component: *mut snd_soc_component, _reg: c_uint) -> c_uint {
    let priv_ = snd_soc_component_get_drvdata(component) as *mut simple_mux;

    (*priv_).mux
}

static SIMPLE_MUX_MUX: snd_kcontrol_new = snd_kcontrol_new {
    name: MUXER.as_ptr() as *const c_char,
    private_value: &SIMPLE_MUX_ENUM as *const soc_enum as c_ulong,
    get: Some(simple_mux_control_get),
    put: Some(simple_mux_control_put),
};

unsafe extern "C" fn simple_mux_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let c = snd_soc_dapm_to_component((*w).dapm);
    let priv_ = snd_soc_component_get_drvdata(c) as *mut simple_mux;

    if (*priv_).idle_state != MUX_IDLE_AS_IS {
        match event {
            SND_SOC_DAPM_PRE_PMU => {
                gpiod_set_value_cansleep((*priv_).gpiod_mux, (*priv_).mux);
            }
            SND_SOC_DAPM_POST_PMD => {
                gpiod_set_value_cansleep((*priv_).gpiod_mux, (*priv_).idle_state);
            }
            _ => {}
        }
    }

    0
}

static SIMPLE_MUX_DAPM_WIDGETS: [snd_soc_dapm_widget; MUX_WIDGET_SIZE] = [
    snd_soc_dapm_widget {
        name: IN1.as_ptr() as *const c_char,
        kcontrol_news: ptr::null(),
        dapm: ptr::null_mut(),
        event: None,
        event_flags: 0,
    },
    snd_soc_dapm_widget {
        name: IN2.as_ptr() as *const c_char,
        kcontrol_news: ptr::null(),
        dapm: ptr::null_mut(),
        event: None,
        event_flags: 0,
    },
    snd_soc_dapm_widget {
        name: MUX.as_ptr() as *const c_char,
        kcontrol_news: &SIMPLE_MUX_MUX,
        dapm: ptr::null_mut(),
        event: Some(simple_mux_event),
        event_flags: SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
    },
    snd_soc_dapm_widget {
        name: OUT.as_ptr() as *const c_char,
        kcontrol_news: ptr::null(),
        dapm: ptr::null_mut(),
        event: None,
        event_flags: 0,
    },
];

static SIMPLE_MUX_DAPM_ROUTES: [snd_soc_dapm_route; MUX_ROUTE_SIZE] = [
    snd_soc_dapm_route {
        sink: OUT.as_ptr() as *const c_char,
        control: ptr::null(),
        source: MUX.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: MUX.as_ptr() as *const c_char,
        control: INPUT_1.as_ptr() as *const c_char,
        source: IN1.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: MUX.as_ptr() as *const c_char,
        control: INPUT_2.as_ptr() as *const c_char,
        source: IN2.as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn simple_mux_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let np = (*dev).of_node;
    let priv_: *mut simple_mux;
    let ret: c_int;

    priv_ = devm_kzalloc(dev, size_of::<simple_mux>(), GFP_KERNEL) as *mut simple_mux;
    if priv_.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, priv_ as *mut c_void);

    (*priv_).gpiod_mux = devm_gpiod_get(dev, MUX_GPIO.as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*priv_).gpiod_mux as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).gpiod_mux as *const c_void),
            b"Failed to get 'mux' gpio\0".as_ptr() as *const c_char,
        );
    }

    /* Copy default settings */
    ptr::copy_nonoverlapping(SIMPLE_MUX_TEXTS.as_ptr(), (*priv_).mux_texts.as_mut_ptr(), MUX_TEXT_SIZE);
    ptr::copy_nonoverlapping(&SIMPLE_MUX_ENUM, &mut (*priv_).mux_enum, 1);
    ptr::copy_nonoverlapping(&SIMPLE_MUX_MUX, &mut (*priv_).mux_mux, 1);
    ptr::copy_nonoverlapping(SIMPLE_MUX_DAPM_WIDGETS.as_ptr(), (*priv_).mux_widgets.as_mut_ptr(), MUX_WIDGET_SIZE);
    ptr::copy_nonoverlapping(SIMPLE_MUX_DAPM_ROUTES.as_ptr(), (*priv_).mux_routes.as_mut_ptr(), MUX_ROUTE_SIZE);

    (*priv_).mux_driver.dapm_widgets = (*priv_).mux_widgets.as_mut_ptr();
    (*priv_).mux_driver.num_dapm_widgets = MUX_WIDGET_SIZE as c_uint;
    (*priv_).mux_driver.dapm_routes = (*priv_).mux_routes.as_mut_ptr();
    (*priv_).mux_driver.num_dapm_routes = MUX_ROUTE_SIZE as c_uint;
    (*priv_).mux_driver.read = Some(simple_mux_read);

    /* Overwrite text ("Input 1", "Input 2") if property exists */
    of_property_read_string_array(
        np,
        STATE_LABELS.as_ptr() as *const c_char,
        (*priv_).mux_texts.as_mut_ptr(),
        MUX_TEXT_SIZE,
    );

    ret = of_property_read_u32(
        np,
        IDLE_STATE.as_ptr() as *const c_char,
        &mut (*priv_).idle_state,
    );
    if ret < 0 {
        (*priv_).idle_state = MUX_IDLE_AS_IS;
    } else if (*priv_).idle_state != MUX_IDLE_AS_IS && (*priv_).idle_state >= 2 {
        dev_err(
            dev,
            b"invalid idle-state %u\n\0".as_ptr() as *const c_char,
            (*priv_).idle_state,
        );
        return -EINVAL;
    }

    /* switch to use priv data instead of default */
    (*priv_).mux_enum.texts = (*priv_).mux_texts.as_ptr();
    (*priv_).mux_mux.private_value = &mut (*priv_).mux_enum as *mut soc_enum as c_ulong;
    (*priv_).mux_widgets[2].kcontrol_news = &mut (*priv_).mux_mux;
    (*priv_).mux_routes[1].control = (*priv_).mux_texts[0]; // "Input 1"
    (*priv_).mux_routes[2].control = (*priv_).mux_texts[1]; // "Input 2"

    devm_snd_soc_register_component(dev, &mut (*priv_).mux_driver, ptr::null_mut(), 0)
}

// Original C condition: #ifdef CONFIG_OF
static SIMPLE_MUX_IDS: [of_device_id; 2] = [
    of_device_id {
        compatible: SIMPLE_AUDIO_MUX.as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, simple_mux_ids);

static SIMPLE_MUX_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: SIMPLE_MUX_NAME.as_ptr() as *const c_char,
        of_match_table: SIMPLE_MUX_IDS.as_ptr(),
    },
    probe: Some(simple_mux_probe),
};

// module_platform_driver(simple_mux_driver);
// MODULE_DESCRIPTION("ASoC Simple Audio Mux driver");
// MODULE_AUTHOR("Alexandre Belloni <alexandre.belloni@bootlin.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
