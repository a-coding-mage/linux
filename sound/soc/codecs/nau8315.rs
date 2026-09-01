// SPDX-License-Identifier: GPL-2.0-only
//
// nau8315.c  --  NAU8315 ALSA SoC Audio Amplifier Driver
//
// Copyright 2020 Nuvoton Technology Crop.
//
// Author: David Lin <ctlin0@nuvoton.com>
//
// Based on MAX98357A.c

// Dependencies from the original C includes:
// linux/acpi.h, linux/device.h, linux/err.h, linux/gpio/consumer.h,
// linux/kernel.h, linux/module.h, linux/of.h, linux/platform_device.h,
// sound/pcm.h, sound/soc.h, sound/soc-dai.h, sound/soc-dapm.h.

#[repr(C)]
pub struct nau8315_priv {
    pub enable: *mut gpio_desc,
    pub enpin_switch: ::core::ffi::c_int,
}

extern "C" {
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut ::core::ffi::c_void;
    fn gpiod_set_value(desc: *mut gpio_desc, value: ::core::ffi::c_int);
    fn dev_dbg(dev: *mut device, fmt: *const ::core::ffi::c_char, ...);
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn devm_kzalloc(
        dev: *mut device,
        size: usize,
        flags: gfp_t,
    ) -> *mut ::core::ffi::c_void;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const ::core::ffi::c_char,
        flags: gpiod_flags,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const ::core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const ::core::ffi::c_void) -> ::core::ffi::c_long;
    fn dev_set_drvdata(dev: *mut device, data: *mut ::core::ffi::c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

pub unsafe extern "C" fn nau8315_daiops_trigger(
    substream: *mut snd_pcm_substream,
    cmd: ::core::ffi::c_int,
    dai: *mut snd_soc_dai,
) -> ::core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let nau8315: *mut nau8315_priv =
        snd_soc_component_get_drvdata(component) as *mut nau8315_priv;

    if (*nau8315).enable.is_null() {
        return 0;
    }

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if (*nau8315).enpin_switch != 0 {
                gpiod_set_value((*nau8315).enable, 1);
                dev_dbg((*component).dev, c"set enable to 1".as_ptr());
            }
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            gpiod_set_value((*nau8315).enable, 0);
            dev_dbg((*component).dev, c"set enable to 0".as_ptr());
        }
        _ => {}
    }

    0
}

pub unsafe extern "C" fn nau8315_enpin_event(
    w: *mut snd_soc_dapm_widget,
    kcontrol: *mut snd_kcontrol,
    event: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*(*w).dapm));
    let nau8315: *mut nau8315_priv =
        snd_soc_component_get_drvdata(component) as *mut nau8315_priv;

    if (event & SND_SOC_DAPM_PRE_PMU) != 0 {
        (*nau8315).enpin_switch = 1;
    } else if (event & SND_SOC_DAPM_POST_PMD) != 0 {
        (*nau8315).enpin_switch = 0;
    }

    0
}

pub static nau8315_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_OUTPUT!(c"Speaker"),
    SND_SOC_DAPM_OUT_DRV_E!(
        c"EN_Pin",
        SND_SOC_NOPM,
        0,
        0,
        ::core::ptr::null_mut(),
        0,
        nau8315_enpin_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD
    ),
];

pub static nau8315_dapm_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c"EN_Pin".as_ptr(),
        control: ::core::ptr::null(),
        source: c"HiFi Playback".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Speaker".as_ptr(),
        control: ::core::ptr::null(),
        source: c"EN_Pin".as_ptr(),
    },
];

pub static nau8315_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: nau8315_dapm_widgets.as_ptr(),
    num_dapm_widgets: nau8315_dapm_widgets.len(),
    dapm_routes: nau8315_dapm_routes.as_ptr(),
    num_dapm_routes: nau8315_dapm_routes.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

pub static nau8315_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    trigger: Some(nau8315_daiops_trigger),
};

pub const NAU8315_RATES: u32 = SNDRV_PCM_RATE_8000_96000;
pub const NAU8315_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_3LE;

pub static mut nau8315_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"nau8315-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"HiFi Playback".as_ptr(),
        formats: NAU8315_FORMATS,
        rates: NAU8315_RATES,
        channels_min: 1,
        channels_max: 2,
    },
    ops: &nau8315_dai_ops,
};

pub unsafe extern "C" fn nau8315_platform_probe(
    pdev: *mut platform_device,
) -> ::core::ffi::c_int {
    let mut nau8315: *mut nau8315_priv;

    nau8315 = devm_kzalloc(
        &mut (*pdev).dev,
        ::core::mem::size_of::<nau8315_priv>(),
        GFP_KERNEL,
    ) as *mut nau8315_priv;
    if nau8315.is_null() {
        return -ENOMEM;
    }

    (*nau8315).enable = devm_gpiod_get_optional(
        &mut (*pdev).dev,
        c"enable".as_ptr(),
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*nau8315).enable as *const ::core::ffi::c_void) {
        return PTR_ERR((*nau8315).enable as *const ::core::ffi::c_void) as ::core::ffi::c_int;
    }

    dev_set_drvdata(
        &mut (*pdev).dev,
        nau8315 as *mut ::core::ffi::c_void,
    );

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &nau8315_component_driver,
        &mut nau8315_dai_driver,
        1,
    )
}

// Original C condition: #ifdef CONFIG_OF
#[cfg(CONFIG_OF)]
pub static nau8315_device_id: [of_device_id; 3] = [
    of_device_id {
        compatible: c"nuvoton,nau8315".as_ptr(),
    },
    of_device_id {
        compatible: c"nuvoton,nau8318".as_ptr(),
    },
    of_device_id {
        compatible: ::core::ptr::null(),
    },
];
#[cfg(CONFIG_OF)]
MODULE_DEVICE_TABLE!(of, nau8315_device_id);

// Original C condition: #ifdef CONFIG_ACPI
#[cfg(CONFIG_ACPI)]
pub static nau8315_acpi_match: [acpi_device_id; 3] = [
    acpi_device_id {
        id: *c"NVTN2010",
        driver_data: 0,
    },
    acpi_device_id {
        id: *c"NVTN2012",
        driver_data: 0,
    },
    acpi_device_id {
        id: *c"",
        driver_data: 0,
    },
];
#[cfg(CONFIG_ACPI)]
MODULE_DEVICE_TABLE!(acpi, nau8315_acpi_match);

pub static mut nau8315_platform_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"nau8315".as_ptr(),
        of_match_table: of_match_ptr!(nau8315_device_id),
        acpi_match_table: ACPI_PTR!(nau8315_acpi_match),
    },
    probe: Some(nau8315_platform_probe),
};
module_platform_driver!(nau8315_platform_driver);

MODULE_DESCRIPTION!(c"ASoC NAU8315 Mono Class-D Amplifier Driver");
MODULE_AUTHOR!(c"David Lin <ctlin0@nuvoton.com>");
MODULE_LICENSE!(c"GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
