// SPDX-License-Identifier: GPL-2.0
/*
 * MAX9759 Amplifier Driver
 *
 * Copyright (c) 2017 BayLibre, SAS.
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 */

// Dependencies from:
// linux/gpio/consumer.h
// linux/module.h
// sound/soc.h
// sound/soc-dapm.h
// sound/tlv.h

const DRV_NAME: *const ::core::ffi::c_char = b"max9759\0".as_ptr() as *const ::core::ffi::c_char;

#[repr(C)]
struct max9759 {
    gpiod_shutdown: *mut gpio_desc,
    gpiod_mute: *mut gpio_desc,
    gpiod_gain: *mut gpio_descs,
    is_mute: bool,
    gain: ::core::ffi::c_uint,
}

unsafe extern "C" fn pga_event(
    w: *mut snd_soc_dapm_widget,
    control: *mut snd_kcontrol,
    event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let c: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let priv_: *mut max9759 = snd_soc_component_get_drvdata(c) as *mut max9759;

    if SND_SOC_DAPM_EVENT_ON(event) {
        gpiod_set_value_cansleep((*priv_).gpiod_shutdown, 0);
    } else {
        gpiod_set_value_cansleep((*priv_).gpiod_shutdown, 1);
    }

    0
}

/* From 6dB to 24dB in steps of 6dB */
// static const DECLARE_TLV_DB_SCALE(speaker_gain_tlv, 600, 600, 0);
static speaker_gain_tlv: [::core::ffi::c_uint; 4] = DECLARE_TLV_DB_SCALE(600, 600, 0);

unsafe extern "C" fn speaker_gain_control_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> ::core::ffi::c_int {
    let c: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let priv_: *mut max9759 = snd_soc_component_get_drvdata(c) as *mut max9759;

    (*ucontrol).value.integer.value[0] = (*priv_).gain as _;

    0
}

static speaker_gain_table: [[bool; 2]; 4] = [
    /* G1, G2 */
    [true, true],   /* +6dB */
    [false, true],  /* +12dB */
    [true, false],  /* +18dB */
    [false, false], /* +24dB */
];

unsafe extern "C" fn speaker_gain_control_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> ::core::ffi::c_int {
    let c: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let priv_: *mut max9759 = snd_soc_component_get_drvdata(c) as *mut max9759;

    if (*ucontrol).value.integer.value[0] < 0 || (*ucontrol).value.integer.value[0] > 3 {
        return -EINVAL;
    }

    (*priv_).gain = (*ucontrol).value.integer.value[0] as ::core::ffi::c_uint;

    /* G1 */
    gpiod_set_value_cansleep(
        *(*(*priv_).gpiod_gain).desc.add(0),
        speaker_gain_table[(*priv_).gain as usize][0] as ::core::ffi::c_int,
    );
    /* G2 */
    gpiod_set_value_cansleep(
        *(*(*priv_).gpiod_gain).desc.add(1),
        speaker_gain_table[(*priv_).gain as usize][1] as ::core::ffi::c_int,
    );

    1
}

unsafe extern "C" fn speaker_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> ::core::ffi::c_int {
    let c: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let priv_: *mut max9759 = snd_soc_component_get_drvdata(c) as *mut max9759;

    (*ucontrol).value.integer.value[0] = (!(*priv_).is_mute) as _;

    0
}

unsafe extern "C" fn speaker_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> ::core::ffi::c_int {
    let c: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let priv_: *mut max9759 = snd_soc_component_get_drvdata(c) as *mut max9759;

    (*priv_).is_mute = !((*ucontrol).value.integer.value[0] != 0);

    gpiod_set_value_cansleep((*priv_).gpiod_mute, (*priv_).is_mute as ::core::ffi::c_int);

    1
}

static max9759_dapm_controls: [snd_kcontrol_new; 2] = [
    SOC_SINGLE_EXT_TLV(
        c"Speaker Gain Volume".as_ptr(),
        0,
        0,
        3,
        0,
        Some(speaker_gain_control_get),
        Some(speaker_gain_control_put),
        speaker_gain_tlv.as_ptr(),
    ),
    SOC_SINGLE_BOOL_EXT(
        c"Playback Switch".as_ptr(),
        0,
        Some(speaker_mute_get),
        Some(speaker_mute_put),
    ),
];

static max9759_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_INPUT(c"INL".as_ptr()),
    SND_SOC_DAPM_INPUT(c"INR".as_ptr()),
    SND_SOC_DAPM_PGA_E(
        c"PGA".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        ::core::ptr::null(),
        0,
        Some(pga_event),
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD,
    ),
    SND_SOC_DAPM_OUTPUT(c"OUTL".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"OUTR".as_ptr()),
];

static max9759_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: c"PGA".as_ptr(),
        control: ::core::ptr::null(),
        source: c"INL".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"PGA".as_ptr(),
        control: ::core::ptr::null(),
        source: c"INR".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUTL".as_ptr(),
        control: ::core::ptr::null(),
        source: c"PGA".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"OUTR".as_ptr(),
        control: ::core::ptr::null(),
        source: c"PGA".as_ptr(),
    },
];

static max9759_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    controls: max9759_dapm_controls.as_ptr(),
    num_controls: max9759_dapm_controls.len() as ::core::ffi::c_uint,
    dapm_widgets: max9759_dapm_widgets.as_ptr(),
    num_dapm_widgets: max9759_dapm_widgets.len() as ::core::ffi::c_uint,
    dapm_routes: max9759_dapm_routes.as_ptr(),
    num_dapm_routes: max9759_dapm_routes.len() as ::core::ffi::c_uint,
};

unsafe extern "C" fn max9759_probe(pdev: *mut platform_device) -> ::core::ffi::c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mut priv_: *mut max9759;

    priv_ = devm_kzalloc(
        dev,
        ::core::mem::size_of::<max9759>(),
        GFP_KERNEL,
    ) as *mut max9759;
    if priv_.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, priv_ as *mut ::core::ffi::c_void);

    (*priv_).gpiod_shutdown = devm_gpiod_get(dev, c"shutdown".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*priv_).gpiod_shutdown as *const ::core::ffi::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).gpiod_shutdown as *const ::core::ffi::c_void),
            c"Failed to get 'shutdown' gpio".as_ptr(),
        );
    }

    (*priv_).gpiod_mute = devm_gpiod_get(dev, c"mute".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*priv_).gpiod_mute as *const ::core::ffi::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).gpiod_mute as *const ::core::ffi::c_void),
            c"Failed to get 'mute' gpio".as_ptr(),
        );
    }
    (*priv_).is_mute = true;

    (*priv_).gpiod_gain = devm_gpiod_get_array(dev, c"gain".as_ptr(), GPIOD_OUT_HIGH);
    if IS_ERR((*priv_).gpiod_gain as *const ::core::ffi::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*priv_).gpiod_gain as *const ::core::ffi::c_void),
            c"Failed to get 'gain' gpios".as_ptr(),
        );
    }
    (*priv_).gain = 0;

    if (*(*priv_).gpiod_gain).ndescs != 2 {
        dev_err(
            dev,
            c"Invalid 'gain' gpios count: %d".as_ptr(),
            (*(*priv_).gpiod_gain).ndescs,
        );
        return -EINVAL;
    }

    devm_snd_soc_register_component(
        dev,
        &max9759_component_driver,
        ::core::ptr::null(),
        0,
    )
}

// #ifdef CONFIG_OF
static max9759_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"maxim,max9759".as_ptr(),
    },
    of_device_id {
        compatible: ::core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, max9759_ids);
// #endif

static mut max9759_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: DRV_NAME,
        of_match_table: of_match_ptr(max9759_ids.as_ptr()),
    },
    probe: Some(max9759_probe),
};

module_platform_driver!(max9759_driver);

MODULE_DESCRIPTION!(c"ASoC MAX9759 amplifier driver");
MODULE_AUTHOR!(c"Neil Armstrong <narmstrong@baylibre.com>");
MODULE_LICENSE!(c"GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
