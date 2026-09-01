// SPDX-License-Identifier: GPL-2.0-only
/*
 * MAX9768 AMP driver
 *
 * Copyright (C) 2011, 2012 by Wolfram Sang, Pengutronix e.K.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* Dependencies originally supplied by:
 * <linux/init.h>, <linux/module.h>, <linux/i2c.h>, <linux/slab.h>,
 * <linux/gpio/consumer.h>, <linux/regmap.h>, <sound/core.h>,
 * <sound/soc.h>, <sound/tlv.h>, and <sound/max9768.h>.
 */

type bool_ = bool;
type u32 = u32;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const GPIOD_OUT_HIGH: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;
const MAX9768_FLAG_CLASSIC_PWM: u32 = 1;

/* "Registers" */
const MAX9768_VOL: c_uint = 0;
const MAX9768_CTRL: c_uint = 3;

/* Commands */
const MAX9768_CTRL_PWM: c_uint = 0x15;
const MAX9768_CTRL_FILTERLESS: c_uint = 0x16;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
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
pub struct device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max9768_pdata {
    pub flags: u32,
}

#[repr(C)]
struct max9768 {
    regmap: *mut regmap,
    mute: *mut gpio_desc,
    shdn: *mut gpio_desc,
    flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
    pub reg: c_uint,
    pub shift: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
    pub tlv: *const c_uint,
    pub index: c_uint,
    pub get: Option<
        unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int,
    >,
    pub put: Option<
        unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int,
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dapm_widget {
    pub id: c_uint,
    pub name: *const c_char,
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
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn gpiod_get_value_cansleep(desc: *mut gpio_desc) -> c_int;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: bool_);
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(
        client: *mut i2c_client,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *const c_void,
        num_dai: c_int,
    ) -> c_int;
}

unsafe fn IS_ERR(ptr: *const c_void) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

static max9768_default_regs: [reg_default; 2] = [
    reg_default { reg: 0, def: 0 },
    reg_default {
        reg: 3,
        def: MAX9768_CTRL_FILTERLESS,
    },
];

unsafe extern "C" fn max9768_get_gpio(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let c: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let max9768: *mut max9768 = snd_soc_component_get_drvdata(c) as *mut max9768;
    let val: c_int = gpiod_get_value_cansleep((*max9768).mute);

    (*ucontrol).value.integer.value[0] = (!val) as i64;

    0
}

unsafe extern "C" fn max9768_set_gpio(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let c: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let max9768: *mut max9768 = snd_soc_component_get_drvdata(c) as *mut max9768;
    let val: bool_ = !((*ucontrol).value.integer.value[0] != 0);
    let ret: c_int;

    if val != (gpiod_get_value_cansleep((*max9768).mute) != 0) {
        ret = 1;
    } else {
        ret = 0;
    }

    gpiod_set_value_cansleep((*max9768).mute, val);

    ret
}

const fn TLV_DB_SCALE_ITEM(min: c_int, step: c_int, mute: c_int) -> [c_uint; 3] {
    [min as c_uint, step as c_uint, mute as c_uint]
}

/* static const DECLARE_TLV_DB_RANGE(volume_tlv, ...); */
static volume_tlv: [(c_uint, c_uint, [c_uint; 3]); 43] = [
    (0, 0, TLV_DB_SCALE_ITEM(-16150, 0, 0)),
    (1, 1, TLV_DB_SCALE_ITEM(-9280, 0, 0)),
    (2, 2, TLV_DB_SCALE_ITEM(-9030, 0, 0)),
    (3, 3, TLV_DB_SCALE_ITEM(-8680, 0, 0)),
    (4, 4, TLV_DB_SCALE_ITEM(-8430, 0, 0)),
    (5, 5, TLV_DB_SCALE_ITEM(-8080, 0, 0)),
    (6, 6, TLV_DB_SCALE_ITEM(-7830, 0, 0)),
    (7, 7, TLV_DB_SCALE_ITEM(-7470, 0, 0)),
    (8, 8, TLV_DB_SCALE_ITEM(-7220, 0, 0)),
    (9, 9, TLV_DB_SCALE_ITEM(-6870, 0, 0)),
    (10, 10, TLV_DB_SCALE_ITEM(-6620, 0, 0)),
    (11, 11, TLV_DB_SCALE_ITEM(-6270, 0, 0)),
    (12, 12, TLV_DB_SCALE_ITEM(-6020, 0, 0)),
    (13, 13, TLV_DB_SCALE_ITEM(-5670, 0, 0)),
    (14, 14, TLV_DB_SCALE_ITEM(-5420, 0, 0)),
    (15, 17, TLV_DB_SCALE_ITEM(-5060, 250, 0)),
    (18, 18, TLV_DB_SCALE_ITEM(-4370, 0, 0)),
    (19, 19, TLV_DB_SCALE_ITEM(-4210, 0, 0)),
    (20, 20, TLV_DB_SCALE_ITEM(-3960, 0, 0)),
    (21, 21, TLV_DB_SCALE_ITEM(-3760, 0, 0)),
    (22, 22, TLV_DB_SCALE_ITEM(-3600, 0, 0)),
    (23, 23, TLV_DB_SCALE_ITEM(-3340, 0, 0)),
    (24, 24, TLV_DB_SCALE_ITEM(-3150, 0, 0)),
    (25, 25, TLV_DB_SCALE_ITEM(-2980, 0, 0)),
    (26, 26, TLV_DB_SCALE_ITEM(-2720, 0, 0)),
    (27, 27, TLV_DB_SCALE_ITEM(-2520, 0, 0)),
    (28, 30, TLV_DB_SCALE_ITEM(-2350, 190, 0)),
    (31, 31, TLV_DB_SCALE_ITEM(-1750, 0, 0)),
    (32, 34, TLV_DB_SCALE_ITEM(-1640, 100, 0)),
    (35, 37, TLV_DB_SCALE_ITEM(-1310, 110, 0)),
    (38, 39, TLV_DB_SCALE_ITEM(-990, 100, 0)),
    (40, 40, TLV_DB_SCALE_ITEM(-710, 0, 0)),
    (41, 41, TLV_DB_SCALE_ITEM(-600, 0, 0)),
    (42, 42, TLV_DB_SCALE_ITEM(-500, 0, 0)),
    (43, 43, TLV_DB_SCALE_ITEM(-340, 0, 0)),
    (44, 44, TLV_DB_SCALE_ITEM(-190, 0, 0)),
    (45, 45, TLV_DB_SCALE_ITEM(-50, 0, 0)),
    (46, 46, TLV_DB_SCALE_ITEM(50, 0, 0)),
    (47, 50, TLV_DB_SCALE_ITEM(120, 40, 0)),
    (51, 57, TLV_DB_SCALE_ITEM(290, 50, 0)),
    (58, 58, TLV_DB_SCALE_ITEM(650, 0, 0)),
    (59, 62, TLV_DB_SCALE_ITEM(700, 60, 0)),
    (63, 63, TLV_DB_SCALE_ITEM(950, 0, 0)),
];

static max9768_volume: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    name: b"Playback Volume\0".as_ptr() as *const c_char,
    reg: MAX9768_VOL,
    shift: 0,
    max: 63,
    invert: 0,
    tlv: volume_tlv.as_ptr() as *const c_uint,
    index: 0,
    get: None,
    put: None,
}];

static max9768_mute: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    name: b"Playback Switch\0".as_ptr() as *const c_char,
    reg: 0,
    shift: 0,
    max: 1,
    invert: 0,
    tlv: ptr::null(),
    index: 0,
    get: Some(max9768_get_gpio),
    put: Some(max9768_set_gpio),
}];

static max9768_dapm_widgets: [snd_soc_dapm_widget; 3] = [
    snd_soc_dapm_widget {
        id: 0,
        name: b"IN\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_widget {
        id: 1,
        name: b"OUT+\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_widget {
        id: 1,
        name: b"OUT-\0".as_ptr() as *const c_char,
    },
];

static max9768_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: b"OUT+\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"IN\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"OUT-\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"IN\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn max9768_probe(component: *mut snd_soc_component) -> c_int {
    let max9768: *mut max9768 = snd_soc_component_get_drvdata(component) as *mut max9768;
    let mut ret: c_int;

    if ((*max9768).flags & MAX9768_FLAG_CLASSIC_PWM) != 0 {
        ret = regmap_write((*max9768).regmap, MAX9768_CTRL, MAX9768_CTRL_PWM);
        if ret != 0 {
            return ret;
        }
    }

    if !(*max9768).mute.is_null() {
        ret = snd_soc_add_component_controls(
            component,
            max9768_mute.as_ptr(),
            ARRAY_SIZE(&max9768_mute),
        );
        if ret != 0 {
            return ret;
        }
    }

    0
}

static max9768_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(max9768_probe),
    controls: max9768_volume.as_ptr(),
    num_controls: ARRAY_SIZE(&max9768_volume),
    dapm_widgets: max9768_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&max9768_dapm_widgets),
    dapm_routes: max9768_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&max9768_dapm_routes),
};

static max9768_i2c_regmap_config: regmap_config = regmap_config {
    reg_bits: 2,
    val_bits: 6,
    max_register: 3,
    reg_defaults: max9768_default_regs.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&max9768_default_regs),
    cache_type: REGCACHE_RBTREE,
};

unsafe extern "C" fn max9768_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut max9768: *mut max9768;
    let pdata: *mut max9768_pdata = (*client).dev.platform_data as *mut max9768_pdata;

    max9768 = devm_kzalloc(
        &mut (*client).dev,
        size_of::<max9768>(),
        GFP_KERNEL,
    ) as *mut max9768;
    if max9768.is_null() {
        return -ENOMEM;
    }

    /* Mute on powerup to avoid clicks */
    (*max9768).mute = devm_gpiod_get_optional(
        &mut (*client).dev,
        b"mute\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR((*max9768).mute as *const c_void) {
        return PTR_ERR((*max9768).mute);
    }
    gpiod_set_consumer_name(
        (*max9768).mute,
        b"MAX9768 Mute\0".as_ptr() as *const c_char,
    );

    /* Activate chip by releasing shutdown, enables I2C */
    (*max9768).shdn = devm_gpiod_get_optional(
        &mut (*client).dev,
        b"shutdown\0".as_ptr() as *const c_char,
        GPIOD_OUT_HIGH,
    );
    if IS_ERR((*max9768).shdn as *const c_void) {
        return PTR_ERR((*max9768).shdn);
    }
    gpiod_set_consumer_name(
        (*max9768).shdn,
        b"MAX9768 Shutdown\0".as_ptr() as *const c_char,
    );

    if !pdata.is_null() {
        (*max9768).flags = (*pdata).flags;
    }

    i2c_set_clientdata(client, max9768 as *mut c_void);

    (*max9768).regmap = devm_regmap_init_i2c(client, &max9768_i2c_regmap_config);
    if IS_ERR((*max9768).regmap as *const c_void) {
        return PTR_ERR((*max9768).regmap);
    }

    devm_snd_soc_register_component(
        &mut (*client).dev,
        &max9768_component_driver,
        ptr::null(),
        0,
    )
}

static max9768_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'm' as c_char,
            b'a' as c_char,
            b'x' as c_char,
            b'9' as c_char,
            b'7' as c_char,
            b'6' as c_char,
            b'8' as c_char,
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
            0,
            0,
            0,
        ],
    },
    i2c_device_id { name: [0; 20] },
];
/* MODULE_DEVICE_TABLE(i2c, max9768_i2c_id); */

static mut max9768_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"max9768\0".as_ptr() as *const c_char,
    },
    probe: Some(max9768_i2c_probe),
    id_table: max9768_i2c_id.as_ptr(),
};
/* module_i2c_driver(max9768_i2c_driver); */

/* MODULE_AUTHOR("Wolfram Sang <kernel@pengutronix.de>"); */
/* MODULE_DESCRIPTION("ASoC MAX9768 amplifier driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
