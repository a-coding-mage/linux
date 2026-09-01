// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC Texas Instruments TPA6130A2 headset stereo amplifier driver
 *
 * Copyright (C) Nokia Corporation
 *
 * Author: Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// C dependencies:
// linux/device.h, linux/errno.h, linux/gpio/consumer.h, linux/i2c.h,
// linux/module.h, linux/of.h, linux/regmap.h, linux/regulator/consumer.h,
// linux/slab.h, sound/soc.h, sound/tlv.h, and "tpa6130a2.h".

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type uintptr_t = usize;

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}

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
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
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
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpa_model {
    TPA6130A2,
    TPA6140A2,
}

/* This struct is used to save the context */
#[repr(C)]
pub struct tpa6130a2_data {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub supply: *mut regulator,
    pub power_gpio: *mut gpio_desc,
    pub id: tpa_model,
}

unsafe extern "C" {
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const c_char, ...) -> c_int;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut tpa6130a2_data;
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut core::ffi::c_void;
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn dump_stack();
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut tpa6130a2_data);
    fn i2c_get_match_data(client: *mut i2c_client) -> *const core::ffi::c_void;
    fn devm_regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *const core::ffi::c_void,
        num_dai: c_int,
    ) -> c_int;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> isize;
    fn of_match_ptr(match_table: *const of_device_id) -> *const of_device_id;
}

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const REGCACHE_RBTREE: c_uint = 0;

const TPA6130A2_REG_VOL_MUTE: c_uint = 1;
const TPA6130A2_REG_CONTROL: c_uint = 2;
const TPA6130A2_REG_VERSION: c_uint = 4;
const TPA6130A2_HP_EN_L_SHIFT: c_uint = 6;
const TPA6130A2_HP_EN_R_SHIFT: c_uint = 7;
const TPA6130A2_SWS_SHIFT: c_uint = 0;
const TPA6130A2_SWS: c_uint = 1 << TPA6130A2_SWS_SHIFT;
const TPA6130A2_MUTE_R: c_uint = 1 << TPA6130A2_HP_EN_R_SHIFT;
const TPA6130A2_MUTE_L: c_uint = 1 << TPA6130A2_HP_EN_L_SHIFT;
const TPA6130A2_VERSION_MASK: c_uint = 0x03;

const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        $array.len()
    };
}

macro_rules! SND_SOC_DAPM_EVENT_ON {
    ($event:expr) => {
        (($event) & SND_SOC_DAPM_PRE_PMU) != 0
    };
}

unsafe extern "C" fn tpa6130a2_power(data: *mut tpa6130a2_data, enable: bool) -> c_int {
    let mut ret: c_int = 0;
    let ret2: c_int;

    if enable {
        ret = regulator_enable((*data).supply);
        if ret != 0 {
            dev_err(
                (*data).dev,
                b"Failed to enable supply: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
        /* Power on */
        gpiod_set_value((*data).power_gpio, 1);

        /* Sync registers */
        regcache_cache_only((*data).regmap, false);
        ret = regcache_sync((*data).regmap);
        if ret != 0 {
            dev_err(
                (*data).dev,
                b"Failed to sync registers: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            regcache_cache_only((*data).regmap, true);
            gpiod_set_value((*data).power_gpio, 0);
            ret2 = regulator_disable((*data).supply);
            if ret2 != 0 {
                dev_err(
                    (*data).dev,
                    b"Failed to disable supply: %d\n\0".as_ptr() as *const c_char,
                    ret2,
                );
            }
            return ret;
        }
    } else {
        /* Powered off device does not retain registers. While device
         * is off, any register updates (i.e. volume changes) should
         * happen in cache only.
         */
        regcache_mark_dirty((*data).regmap);
        regcache_cache_only((*data).regmap, true);

        /* Power off */
        gpiod_set_value((*data).power_gpio, 0);

        ret = regulator_disable((*data).supply);
        if ret != 0 {
            dev_err(
                (*data).dev,
                b"Failed to disable supply: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
    }

    ret
}

unsafe extern "C" fn tpa6130a2_power_event(
    w: *mut snd_soc_dapm_widget,
    _kctrl: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let c: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let data: *mut tpa6130a2_data = snd_soc_component_get_drvdata(c);

    if SND_SOC_DAPM_EVENT_ON!(event) {
        /* Before widget power up: turn chip on, sync registers */
        return tpa6130a2_power(data, true);
    } else {
        /* After widget power down: turn chip off */
        return tpa6130a2_power(data, false);
    }
}

/*
 * TPA6130 volume. From -59.5 to 4 dB with increasing step size when going
 * down in gain.
 */
static tpa6130_tlv: &[c_uint] = &DECLARE_TLV_DB_RANGE!(
    0,
    1,
    TLV_DB_SCALE_ITEM!(-5950, 600, 0),
    2,
    3,
    TLV_DB_SCALE_ITEM!(-5000, 250, 0),
    4,
    5,
    TLV_DB_SCALE_ITEM!(-4550, 160, 0),
    6,
    7,
    TLV_DB_SCALE_ITEM!(-4140, 190, 0),
    8,
    9,
    TLV_DB_SCALE_ITEM!(-3650, 120, 0),
    10,
    11,
    TLV_DB_SCALE_ITEM!(-3330, 160, 0),
    12,
    13,
    TLV_DB_SCALE_ITEM!(-3040, 180, 0),
    14,
    20,
    TLV_DB_SCALE_ITEM!(-2710, 110, 0),
    21,
    37,
    TLV_DB_SCALE_ITEM!(-1960, 74, 0),
    38,
    63,
    TLV_DB_SCALE_ITEM!(-720, 45, 0)
);

static tpa6130a2_controls: [snd_kcontrol_new; 1] = [SOC_SINGLE_TLV!(
    b"Headphone Playback Volume\0".as_ptr() as *const c_char,
    TPA6130A2_REG_VOL_MUTE,
    0,
    0x3f,
    0,
    tpa6130_tlv
)];

static tpa6140_tlv: &[c_uint] = &DECLARE_TLV_DB_RANGE!(
    0,
    8,
    TLV_DB_SCALE_ITEM!(-5900, 400, 0),
    9,
    16,
    TLV_DB_SCALE_ITEM!(-2500, 200, 0),
    17,
    31,
    TLV_DB_SCALE_ITEM!(-1000, 100, 0)
);

static tpa6140a2_controls: [snd_kcontrol_new; 1] = [SOC_SINGLE_TLV!(
    b"Headphone Playback Volume\0".as_ptr() as *const c_char,
    TPA6130A2_REG_VOL_MUTE,
    1,
    0x1f,
    0,
    tpa6140_tlv
)];

unsafe extern "C" fn tpa6130a2_component_probe(component: *mut snd_soc_component) -> c_int {
    let data: *mut tpa6130a2_data = snd_soc_component_get_drvdata(component);

    if (*data).id == tpa_model::TPA6140A2 {
        return snd_soc_add_component_controls(
            component,
            tpa6140a2_controls.as_ptr(),
            ARRAY_SIZE!(tpa6140a2_controls) as c_uint,
        );
    } else {
        return snd_soc_add_component_controls(
            component,
            tpa6130a2_controls.as_ptr(),
            ARRAY_SIZE!(tpa6130a2_controls) as c_uint,
        );
    }
}

static tpa6130a2_dapm_widgets: [snd_soc_dapm_widget_desc; 9] = [
    SND_SOC_DAPM_INPUT!(b"LEFTIN\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_INPUT!(b"RIGHTIN\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_OUTPUT!(b"HPLEFT\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_OUTPUT!(b"HPRIGHT\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_PGA!(
        b"Left Mute\0".as_ptr() as *const c_char,
        TPA6130A2_REG_VOL_MUTE,
        TPA6130A2_HP_EN_L_SHIFT,
        1,
        core::ptr::null(),
        0
    ),
    SND_SOC_DAPM_PGA!(
        b"Right Mute\0".as_ptr() as *const c_char,
        TPA6130A2_REG_VOL_MUTE,
        TPA6130A2_HP_EN_R_SHIFT,
        1,
        core::ptr::null(),
        0
    ),
    SND_SOC_DAPM_PGA!(
        b"Left PGA\0".as_ptr() as *const c_char,
        TPA6130A2_REG_CONTROL,
        TPA6130A2_HP_EN_L_SHIFT,
        0,
        core::ptr::null(),
        0
    ),
    SND_SOC_DAPM_PGA!(
        b"Right PGA\0".as_ptr() as *const c_char,
        TPA6130A2_REG_CONTROL,
        TPA6130A2_HP_EN_R_SHIFT,
        0,
        core::ptr::null(),
        0
    ),
    SND_SOC_DAPM_SUPPLY!(
        b"Power\0".as_ptr() as *const c_char,
        TPA6130A2_REG_CONTROL,
        TPA6130A2_SWS_SHIFT,
        1,
        tpa6130a2_power_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
];

static tpa6130a2_dapm_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route {
        sink: b"Left PGA\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"LEFTIN\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Right PGA\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"RIGHTIN\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Left Mute\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Left PGA\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Right Mute\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Right PGA\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"HPLEFT\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Left Mute\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"HPRIGHT\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Right Mute\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Left PGA\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Power\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Right PGA\0".as_ptr() as *const c_char,
        control: core::ptr::null(),
        source: b"Power\0".as_ptr() as *const c_char,
    },
];

static tpa6130a2_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    name: b"tpa6130a2\0".as_ptr() as *const c_char,
    probe: Some(tpa6130a2_component_probe),
    dapm_widgets: tpa6130a2_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(tpa6130a2_dapm_widgets),
    dapm_routes: tpa6130a2_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(tpa6130a2_dapm_routes),
};

static tpa6130a2_reg_defaults: [reg_default; 2] = [
    reg_default {
        reg: TPA6130A2_REG_CONTROL,
        def: TPA6130A2_SWS,
    },
    reg_default {
        reg: TPA6130A2_REG_VOL_MUTE,
        def: TPA6130A2_MUTE_R | TPA6130A2_MUTE_L,
    },
];

static tpa6130a2_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: TPA6130A2_REG_VERSION,
    reg_defaults: tpa6130a2_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE!(tpa6130a2_reg_defaults) as c_uint,
    cache_type: REGCACHE_RBTREE,
};

unsafe extern "C" fn tpa6130a2_probe(client: *mut i2c_client) -> c_int {
    let dev: *mut device;
    let data: *mut tpa6130a2_data;
    let np: *mut device_node = (*client).dev.of_node;
    let regulator: *const c_char;
    let mut version: c_uint = 0;
    let mut ret: c_int;

    dev = &mut (*client).dev;

    data = devm_kzalloc(
        &mut (*client).dev,
        core::mem::size_of::<tpa6130a2_data>(),
        GFP_KERNEL,
    ) as *mut tpa6130a2_data;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).dev = dev;

    (*data).regmap = devm_regmap_init_i2c(client, &tpa6130a2_regmap_config);
    if IS_ERR((*data).regmap as *const core::ffi::c_void) {
        return PTR_ERR((*data).regmap as *const core::ffi::c_void) as c_int;
    }

    if !np.is_null() {
        (*data).power_gpio =
            devm_gpiod_get_optional(dev, b"power\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
        if IS_ERR((*data).power_gpio as *const core::ffi::c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR((*data).power_gpio as *const core::ffi::c_void),
                b"Failed to request power GPIO\n\0".as_ptr() as *const c_char,
            );
        }
        gpiod_set_consumer_name(
            (*data).power_gpio,
            b"tpa6130a2 enable\0".as_ptr() as *const c_char,
        );
    } else {
        dev_err(dev, b"Platform data not set\n\0".as_ptr() as *const c_char);
        dump_stack();
        return -ENODEV;
    }

    i2c_set_clientdata(client, data);

    (*data).id = core::mem::transmute::<uintptr_t, tpa_model>(i2c_get_match_data(client) as uintptr_t);

    match (*data).id {
        tpa_model::TPA6140A2 => {
            regulator = b"AVdd\0".as_ptr() as *const c_char;
        }
        tpa_model::TPA6130A2 => {
            regulator = b"Vdd\0".as_ptr() as *const c_char;
        }
        _ => {
            dev_warn(
                dev,
                b"Unknown TPA model (%d). Assuming 6130A2\n\0".as_ptr() as *const c_char,
                (*data).id as c_int,
            );
            regulator = b"Vdd\0".as_ptr() as *const c_char;
        }
    }

    (*data).supply = devm_regulator_get(dev, regulator);
    if IS_ERR((*data).supply as *const core::ffi::c_void) {
        ret = PTR_ERR((*data).supply as *const core::ffi::c_void) as c_int;
        dev_err(
            dev,
            b"Failed to request supply: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = tpa6130a2_power(data, true);
    if ret != 0 {
        return ret;
    }

    /* Read version */
    regmap_read((*data).regmap, TPA6130A2_REG_VERSION, &mut version);
    version &= TPA6130A2_VERSION_MASK;
    if (version != 1) && (version != 2) {
        dev_warn(
            dev,
            b"UNTESTED version detected (%d)\n\0".as_ptr() as *const c_char,
            version,
        );
    }

    /* Disable the chip */
    ret = tpa6130a2_power(data, false);
    if ret != 0 {
        return ret;
    }

    devm_snd_soc_register_component(
        &mut (*client).dev,
        &tpa6130a2_component_driver,
        core::ptr::null(),
        0,
    )
}

// Original C condition: #if IS_ENABLED(CONFIG_OF)
static tpa6130a2_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: b"ti,tpa6130a2\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"ti,tpa6140a2\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
MODULE_DEVICE_TABLE!(of, tpa6130a2_of_match);

static mut tpa6130a2_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"tpa6130a2\0".as_ptr() as *const c_char,
        of_match_table: unsafe { of_match_ptr(tpa6130a2_of_match.as_ptr()) },
    },
    probe: Some(tpa6130a2_probe),
};

module_i2c_driver!(tpa6130a2_i2c_driver);

MODULE_AUTHOR!("Peter Ujfalusi <peter.ujfalusi@ti.com>");
MODULE_DESCRIPTION!("TPA6130A2 Headphone amplifier driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
