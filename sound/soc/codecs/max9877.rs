// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * max9877.rs  --  amp driver for max9877
 *
 * Copyright (C) 2009 Samsung Electronics Co.Ltd
 * Author: Joonyoung Shim <jy0922.shim@samsung.com>
 */

/* Rust translation of Linux kernel implementation source:
 * includes from linux/module.h, linux/init.h, linux/i2c.h, linux/regmap.h,
 * sound/soc.h, sound/tlv.h, and "max9877.h" are external dependencies.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
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
pub struct snd_soc_component_driver {
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_int,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [c_char; 20],
    pub driver_data: usize,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

unsafe extern "C" {
    fn devm_regmap_init_i2c(client: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;

    static REGCACHE_RBTREE: c_int;
    static MAX9877_INPUT_MODE: c_uint;
    static MAX9877_OUTPUT_MODE: c_uint;
    static MAX9877_SPK_VOLUME: c_uint;
    static MAX9877_HPL_VOLUME: c_uint;
    static MAX9877_HPR_VOLUME: c_uint;
    static MAX9877_OSC_OFFSET: c_uint;

    fn TLV_DB_SCALE_ITEM(min: c_int, step: c_int, mute: c_int) -> c_uint;
    fn DECLARE_TLV_DB_RANGE(name: *const c_char, items: ...) -> c_uint;
    fn SOC_ENUM_SINGLE(
        reg: c_uint,
        shift_l: c_uint,
        items: c_uint,
        texts: *const *const c_char,
    ) -> soc_enum;
    fn SOC_SINGLE_TLV(
        xname: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
        tlv_array: *const c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_DOUBLE_R_TLV(
        xname: *const c_char,
        reg_left: c_uint,
        reg_right: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
        tlv_array: *const c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_SINGLE(
        xname: *const c_char,
        reg: c_uint,
        shift: c_uint,
        max: c_uint,
        invert: c_uint,
    ) -> snd_kcontrol_new;
    fn SOC_ENUM(xname: *const c_char, xenum: soc_enum) -> snd_kcontrol_new;
    fn SND_SOC_DAPM_INPUT(wname: *const c_char) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_PGA(
        wname: *const c_char,
        wreg: c_uint,
        wshift: c_uint,
        winvert: c_uint,
        wcontrols: *mut c_void,
        wncontrols: c_int,
    ) -> snd_soc_dapm_widget;
    fn SND_SOC_DAPM_OUTPUT(wname: *const c_char) -> snd_soc_dapm_widget;
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

static max9877_regs: [reg_default; 5] = [
    reg_default { reg: 0, def: 0x40 },
    reg_default { reg: 1, def: 0x00 },
    reg_default { reg: 2, def: 0x00 },
    reg_default { reg: 3, def: 0x00 },
    reg_default { reg: 4, def: 0x49 },
];

static max9877_pgain_tlv: [c_uint; 2] = unsafe {
    [
        DECLARE_TLV_DB_RANGE(
            c"max9877_pgain_tlv".as_ptr(),
            0,
            1,
            TLV_DB_SCALE_ITEM(0, 900, 0),
            2,
            2,
            TLV_DB_SCALE_ITEM(2000, 0, 0),
        ),
        0,
    ]
};

static max9877_output_tlv: [c_uint; 4] = unsafe {
    [
        DECLARE_TLV_DB_RANGE(
            c"max9877_output_tlv".as_ptr(),
            0,
            7,
            TLV_DB_SCALE_ITEM(-7900, 400, 1),
            8,
            15,
            TLV_DB_SCALE_ITEM(-4700, 300, 0),
            16,
            23,
            TLV_DB_SCALE_ITEM(-2300, 200, 0),
            24,
            31,
            TLV_DB_SCALE_ITEM(-700, 100, 0),
        ),
        0,
        0,
        0,
    ]
};

static max9877_out_mode: [*const c_char; 9] = [
    c"INA -> SPK".as_ptr(),
    c"INA -> HP".as_ptr(),
    c"INA -> SPK and HP".as_ptr(),
    c"INB -> SPK".as_ptr(),
    c"INB -> HP".as_ptr(),
    c"INB -> SPK and HP".as_ptr(),
    c"INA + INB -> SPK".as_ptr(),
    c"INA + INB -> HP".as_ptr(),
    c"INA + INB -> SPK and HP".as_ptr(),
];

static max9877_osc_mode: [*const c_char; 3] = [
    c"1176KHz".as_ptr(),
    c"1100KHz".as_ptr(),
    c"700KHz".as_ptr(),
];

static max9877_enum: [soc_enum; 2] = unsafe {
    [
        SOC_ENUM_SINGLE(
            MAX9877_OUTPUT_MODE,
            0,
            ARRAY_SIZE(&max9877_out_mode),
            max9877_out_mode.as_ptr(),
        ),
        SOC_ENUM_SINGLE(
            MAX9877_OUTPUT_MODE,
            MAX9877_OSC_OFFSET,
            ARRAY_SIZE(&max9877_osc_mode),
            max9877_osc_mode.as_ptr(),
        ),
    ]
};

static max9877_controls: [snd_kcontrol_new; 10] = unsafe {
    [
        SOC_SINGLE_TLV(
            c"MAX9877 PGAINA Playback Volume".as_ptr(),
            MAX9877_INPUT_MODE,
            0,
            2,
            0,
            max9877_pgain_tlv.as_ptr(),
        ),
        SOC_SINGLE_TLV(
            c"MAX9877 PGAINB Playback Volume".as_ptr(),
            MAX9877_INPUT_MODE,
            2,
            2,
            0,
            max9877_pgain_tlv.as_ptr(),
        ),
        SOC_SINGLE_TLV(
            c"MAX9877 Amp Speaker Playback Volume".as_ptr(),
            MAX9877_SPK_VOLUME,
            0,
            31,
            0,
            max9877_output_tlv.as_ptr(),
        ),
        SOC_DOUBLE_R_TLV(
            c"MAX9877 Amp HP Playback Volume".as_ptr(),
            MAX9877_HPL_VOLUME,
            MAX9877_HPR_VOLUME,
            0,
            31,
            0,
            max9877_output_tlv.as_ptr(),
        ),
        SOC_SINGLE(
            c"MAX9877 INB Stereo Switch".as_ptr(),
            MAX9877_INPUT_MODE,
            4,
            1,
            1,
        ),
        SOC_SINGLE(
            c"MAX9877 INA Stereo Switch".as_ptr(),
            MAX9877_INPUT_MODE,
            5,
            1,
            1,
        ),
        SOC_SINGLE(
            c"MAX9877 Zero-crossing detection Switch".as_ptr(),
            MAX9877_INPUT_MODE,
            6,
            1,
            0,
        ),
        SOC_SINGLE(
            c"MAX9877 Bypass Mode Switch".as_ptr(),
            MAX9877_OUTPUT_MODE,
            6,
            1,
            0,
        ),
        SOC_ENUM(c"MAX9877 Output Mode".as_ptr(), max9877_enum[0]),
        SOC_ENUM(c"MAX9877 Oscillator Mode".as_ptr(), max9877_enum[1]),
    ]
};

static max9877_dapm_widgets: [snd_soc_dapm_widget; 11] = unsafe {
    [
        SND_SOC_DAPM_INPUT(c"INA1".as_ptr()),
        SND_SOC_DAPM_INPUT(c"INA2".as_ptr()),
        SND_SOC_DAPM_INPUT(c"INB1".as_ptr()),
        SND_SOC_DAPM_INPUT(c"INB2".as_ptr()),
        SND_SOC_DAPM_INPUT(c"RXIN+".as_ptr()),
        SND_SOC_DAPM_INPUT(c"RXIN-".as_ptr()),
        SND_SOC_DAPM_PGA(c"SHDN".as_ptr(), MAX9877_OUTPUT_MODE, 7, 1, ptr::null_mut(), 0),
        SND_SOC_DAPM_OUTPUT(c"OUT+".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"OUT-".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"HPL".as_ptr()),
        SND_SOC_DAPM_OUTPUT(c"HPR".as_ptr()),
    ]
};

static max9877_dapm_routes: [snd_soc_dapm_route; 10] = [
    snd_soc_dapm_route {
        sink: c"SHDN".as_ptr(),
        control: ptr::null(),
        source: c"INA1".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"SHDN".as_ptr(),
        control: ptr::null(),
        source: c"INA2".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"SHDN".as_ptr(),
        control: ptr::null(),
        source: c"INB1".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"SHDN".as_ptr(),
        control: ptr::null(),
        source: c"INB2".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUT+".as_ptr(),
        control: ptr::null(),
        source: c"RXIN+".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUT+".as_ptr(),
        control: ptr::null(),
        source: c"SHDN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUT-".as_ptr(),
        control: ptr::null(),
        source: c"SHDN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUT-".as_ptr(),
        control: ptr::null(),
        source: c"RXIN-".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"HPL".as_ptr(),
        control: ptr::null(),
        source: c"SHDN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"HPR".as_ptr(),
        control: ptr::null(),
        source: c"SHDN".as_ptr(),
    },
];

static max9877_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    controls: max9877_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&max9877_controls),
    dapm_widgets: max9877_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&max9877_dapm_widgets),
    dapm_routes: max9877_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&max9877_dapm_routes),
};

static max9877_regmap: regmap_config = unsafe {
    regmap_config {
        reg_bits: 8,
        val_bits: 8,
        reg_defaults: max9877_regs.as_ptr(),
        num_reg_defaults: ARRAY_SIZE(&max9877_regs),
        cache_type: REGCACHE_RBTREE,
    }
};

unsafe extern "C" fn max9877_i2c_probe(client: *mut i2c_client) -> c_int {
    let mut regmap: *mut regmap;
    let mut i: c_int;

    regmap = devm_regmap_init_i2c(client, &max9877_regmap);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void);
    }

    /* Ensure the device is in reset state */
    i = 0;
    while i < ARRAY_SIZE(&max9877_regs) as c_int {
        regmap_write(
            regmap,
            max9877_regs[i as usize].reg,
            max9877_regs[i as usize].def,
        );
        i += 1;
    }

    devm_snd_soc_register_component(
        &mut (*client).dev,
        &max9877_component_driver,
        ptr::null_mut(),
        0,
    )
}

static max9877_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'm' as c_char,
            b'a' as c_char,
            b'x' as c_char,
            b'9' as c_char,
            b'8' as c_char,
            b'7' as c_char,
            b'7' as c_char,
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
        driver_data: 0,
    },
    i2c_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];
/* MODULE_DEVICE_TABLE(i2c, max9877_i2c_id); */

static mut max9877_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"max9877".as_ptr(),
    },
    probe: Some(max9877_i2c_probe),
    id_table: max9877_i2c_id.as_ptr(),
};

/* module_i2c_driver(max9877_i2c_driver); */

/* MODULE_DESCRIPTION("ASoC MAX9877 amp driver"); */
/* MODULE_AUTHOR("Joonyoung Shim <jy0922.shim@samsung.com>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
