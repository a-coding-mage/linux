// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * LM4857 AMP driver
 *
 * Copyright 2007 Wolfson Microelectronics PLC.
 * Author: Graeme Gregory
 *         graeme.gregory@wolfsonmicro.com
 * Copyright 2011 Lars-Peter Clausen <lars@metafoo.de>
 */

/* C dependencies:
 * linux/init.h, linux/module.h, linux/i2c.h, linux/regmap.h, linux/slab.h
 * sound/core.h, sound/soc.h, sound/tlv.h
 */

static lm4857_default_regs: [reg_default; 4] = [
    reg_default {
        reg: 0x0,
        def: 0x00,
    },
    reg_default {
        reg: 0x1,
        def: 0x00,
    },
    reg_default {
        reg: 0x2,
        def: 0x00,
    },
    reg_default {
        reg: 0x3,
        def: 0x00,
    },
];

/* The register offsets in the cache array */
const LM4857_MVOL: u32 = 0;
const LM4857_LVOL: u32 = 1;
const LM4857_RVOL: u32 = 2;
const LM4857_CTRL: u32 = 3;

/* the shifts required to set these bits */
const LM4857_3D: u32 = 5;
const LM4857_WAKEUP: u32 = 5;
const LM4857_EPGAIN: u32 = 4;

static lm4857_mode_values: [c_uint; 5] = [
    0,
    6,
    7,
    8,
    9,
];

static lm4857_mode_texts: [*const c_char; 5] = [
    c"Off".as_ptr(),
    c"Earpiece".as_ptr(),
    c"Loudspeaker".as_ptr(),
    c"Loudspeaker + Headphone".as_ptr(),
    c"Headphone".as_ptr(),
];

SOC_VALUE_ENUM_SINGLE_AUTODISABLE_DECL!(
    lm4857_mode_enum,
    LM4857_CTRL,
    0,
    0xf,
    lm4857_mode_texts,
    lm4857_mode_values
);

static lm4857_mode_ctrl: snd_kcontrol_new = SOC_DAPM_ENUM!(c"Mode".as_ptr(), lm4857_mode_enum);

static lm4857_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_INPUT!(c"IN".as_ptr()),
    SND_SOC_DAPM_DEMUX!(
        c"Mode".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        &lm4857_mode_ctrl as *const snd_kcontrol_new
    ),
    SND_SOC_DAPM_OUTPUT!(c"LS".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"HP".as_ptr()),
    SND_SOC_DAPM_OUTPUT!(c"EP".as_ptr()),
];

static stereo_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-4050, 150, 0);
static mono_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-3450, 150, 0);

static lm4857_controls: [snd_kcontrol_new; 7] = [
    SOC_SINGLE_TLV!(
        c"Left Playback Volume".as_ptr(),
        LM4857_LVOL,
        0,
        31,
        0,
        stereo_tlv
    ),
    SOC_SINGLE_TLV!(
        c"Right Playback Volume".as_ptr(),
        LM4857_RVOL,
        0,
        31,
        0,
        stereo_tlv
    ),
    SOC_SINGLE_TLV!(
        c"Mono Playback Volume".as_ptr(),
        LM4857_MVOL,
        0,
        31,
        0,
        mono_tlv
    ),
    SOC_SINGLE!(
        c"Spk 3D Playback Switch".as_ptr(),
        LM4857_LVOL,
        LM4857_3D,
        1,
        0
    ),
    SOC_SINGLE!(
        c"HP 3D Playback Switch".as_ptr(),
        LM4857_RVOL,
        LM4857_3D,
        1,
        0
    ),
    SOC_SINGLE!(
        c"Fast Wakeup Playback Switch".as_ptr(),
        LM4857_CTRL,
        LM4857_WAKEUP,
        1,
        0
    ),
    SOC_SINGLE!(
        c"Earpiece 6dB Playback Switch".as_ptr(),
        LM4857_CTRL,
        LM4857_EPGAIN,
        1,
        0
    ),
];

static lm4857_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route {
        sink: c"Mode".as_ptr(),
        control: core::ptr::null(),
        source: c"IN".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"LS".as_ptr(),
        control: c"Loudspeaker".as_ptr(),
        source: c"Mode".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"LS".as_ptr(),
        control: c"Loudspeaker + Headphone".as_ptr(),
        source: c"Mode".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"HP".as_ptr(),
        control: c"Headphone".as_ptr(),
        source: c"Mode".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"HP".as_ptr(),
        control: c"Loudspeaker + Headphone".as_ptr(),
        source: c"Mode".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"EP".as_ptr(),
        control: c"Earpiece".as_ptr(),
        source: c"Mode".as_ptr(),
    },
];

static lm4857_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    controls: lm4857_controls.as_ptr(),
    num_controls: lm4857_controls.len() as c_uint,
    dapm_widgets: lm4857_dapm_widgets.as_ptr(),
    num_dapm_widgets: lm4857_dapm_widgets.len() as c_uint,
    dapm_routes: lm4857_routes.as_ptr(),
    num_dapm_routes: lm4857_routes.len() as c_uint,
};

static lm4857_regmap_config: regmap_config = regmap_config {
    val_bits: 6,
    reg_bits: 2,

    max_register: LM4857_CTRL,

    cache_type: REGCACHE_FLAT,
    reg_defaults: lm4857_default_regs.as_ptr(),
    num_reg_defaults: lm4857_default_regs.len() as c_uint,
};

unsafe extern "C" fn lm4857_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let mut regmap: *mut regmap;

    regmap = devm_regmap_init_i2c(i2c, &lm4857_regmap_config as *const regmap_config);
    if IS_ERR(regmap as *const c_void) {
        return PTR_ERR(regmap as *const c_void) as c_int;
    }

    return devm_snd_soc_register_component(
        &mut (*i2c).dev as *mut device,
        &lm4857_component_driver as *const snd_soc_component_driver,
        core::ptr::null_mut(),
        0,
    );
}

static lm4857_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: *b"lm4857\0",
        driver_data: 0,
    },
    i2c_device_id {
        name: [0; I2C_NAME_SIZE],
        driver_data: 0,
    },
];
MODULE_DEVICE_TABLE!(i2c, lm4857_i2c_id);

static mut lm4857_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"lm4857".as_ptr(),
    },
    probe: Some(lm4857_i2c_probe),
    id_table: lm4857_i2c_id.as_ptr(),
};

module_i2c_driver!(lm4857_i2c_driver);

MODULE_AUTHOR!(c"Lars-Peter Clausen <lars@metafoo.de>".as_ptr());
MODULE_DESCRIPTION!(c"LM4857 amplifier driver".as_ptr());
MODULE_LICENSE!(c"GPL".as_ptr());

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
