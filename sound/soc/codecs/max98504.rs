// SPDX-License-Identifier: GPL-2.0-only
/*
 * MAX98504 ALSA SoC Audio driver
 *
 * Copyright 2013 - 2014 Maxim Integrated Products
 * Copyright 2016 Samsung Electronics Co., Ltd.
 */

// C dependencies: linux/delay.h, linux/i2c.h, linux/module.h,
// linux/regulator/consumer.h, linux/slab.h, linux/types.h, sound/soc.h,
// and "max98504.h".

use core::ffi::{c_char, c_int, c_uint};
use core::mem::size_of;
use core::ptr;

const max98504_supply_names: [*const c_char; 3] = [
    b"DVDD\0".as_ptr() as *const c_char,
    b"DIOVDD\0".as_ptr() as *const c_char,
    b"PVDD\0".as_ptr() as *const c_char,
];
const MAX98504_NUM_SUPPLIES: usize = max98504_supply_names.len();

#[repr(C)]
struct max98504_priv {
    regmap: *mut regmap,
    supplies: [regulator_bulk_data; MAX98504_NUM_SUPPLIES],
    pcm_rx_channels: c_uint,
    brownout_enable: bool,
    brownout_threshold: c_uint,
    brownout_attenuation: c_uint,
    brownout_attack_hold: c_uint,
    brownout_timed_hold: c_uint,
    brownout_release_rate: c_uint,
}

static max98504_reg_defaults: [reg_default; 36] = [
    reg_default { reg: 0x01, def: 0 },
    reg_default { reg: 0x02, def: 0 },
    reg_default { reg: 0x03, def: 0 },
    reg_default { reg: 0x04, def: 0 },
    reg_default { reg: 0x10, def: 0 },
    reg_default { reg: 0x11, def: 0 },
    reg_default { reg: 0x12, def: 0 },
    reg_default { reg: 0x13, def: 0 },
    reg_default { reg: 0x14, def: 0 },
    reg_default { reg: 0x15, def: 0 },
    reg_default { reg: 0x16, def: 0 },
    reg_default { reg: 0x17, def: 0 },
    reg_default { reg: 0x18, def: 0 },
    reg_default { reg: 0x19, def: 0 },
    reg_default { reg: 0x1A, def: 0 },
    reg_default { reg: 0x20, def: 0 },
    reg_default { reg: 0x21, def: 0 },
    reg_default { reg: 0x22, def: 0 },
    reg_default { reg: 0x23, def: 0 },
    reg_default { reg: 0x24, def: 0 },
    reg_default { reg: 0x25, def: 0 },
    reg_default { reg: 0x26, def: 0 },
    reg_default { reg: 0x27, def: 0 },
    reg_default { reg: 0x28, def: 0 },
    reg_default { reg: 0x30, def: 0 },
    reg_default { reg: 0x31, def: 0 },
    reg_default { reg: 0x32, def: 0 },
    reg_default { reg: 0x33, def: 0 },
    reg_default { reg: 0x34, def: 0 },
    reg_default { reg: 0x35, def: 0 },
    reg_default { reg: 0x36, def: 0 },
    reg_default { reg: 0x37, def: 0 },
    reg_default { reg: 0x38, def: 0 },
    reg_default { reg: 0x39, def: 0 },
    reg_default { reg: 0x40, def: 0 },
    reg_default { reg: 0x41, def: 0 },
];

unsafe extern "C" fn max98504_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        MAX98504_INTERRUPT_STATUS
        | MAX98504_INTERRUPT_FLAGS
        | MAX98504_INTERRUPT_FLAG_CLEARS
        | MAX98504_WATCHDOG_CLEAR
        | MAX98504_GLOBAL_ENABLE
        | MAX98504_SOFTWARE_RESET => true,
        _ => false,
    }
}

unsafe extern "C" fn max98504_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        MAX98504_SOFTWARE_RESET | MAX98504_WATCHDOG_CLEAR | MAX98504_INTERRUPT_FLAG_CLEARS => false,
        _ => true,
    }
}

unsafe extern "C" fn max98504_pcm_rx_ev(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let c = snd_soc_dapm_to_component((*w).dapm);
    let max98504 = snd_soc_component_get_drvdata(c) as *mut max98504_priv;

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            regmap_write(
                (*max98504).regmap,
                MAX98504_PCM_RX_ENABLE,
                (*max98504).pcm_rx_channels,
            );
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_write((*max98504).regmap, MAX98504_PCM_RX_ENABLE, 0);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn max98504_component_probe(c: *mut snd_soc_component) -> c_int {
    let max98504 = snd_soc_component_get_drvdata(c) as *mut max98504_priv;
    let map = (*max98504).regmap;
    let mut ret: c_int;

    ret = regulator_bulk_enable(MAX98504_NUM_SUPPLIES as c_int, (*max98504).supplies.as_mut_ptr());
    if ret < 0 {
        return ret;
    }

    regmap_write(map, MAX98504_SOFTWARE_RESET, 0x1);
    msleep(20);

    if !(*max98504).brownout_enable {
        return 0;
    }

    regmap_write(map, MAX98504_PVDD_BROWNOUT_ENABLE, 0x1);

    regmap_write(
        map,
        MAX98504_PVDD_BROWNOUT_CONFIG_1,
        ((*max98504).brownout_threshold & 0x1f) << 3 | ((*max98504).brownout_attenuation & 0x3),
    );

    regmap_write(
        map,
        MAX98504_PVDD_BROWNOUT_CONFIG_2,
        (*max98504).brownout_attack_hold & 0xff,
    );

    regmap_write(
        map,
        MAX98504_PVDD_BROWNOUT_CONFIG_3,
        (*max98504).brownout_timed_hold & 0xff,
    );

    regmap_write(
        map,
        MAX98504_PVDD_BROWNOUT_CONFIG_4,
        (*max98504).brownout_release_rate & 0xff,
    );

    0
}

unsafe extern "C" fn max98504_component_remove(c: *mut snd_soc_component) {
    let max98504 = snd_soc_component_get_drvdata(c) as *mut max98504_priv;

    regulator_bulk_disable(MAX98504_NUM_SUPPLIES as c_int, (*max98504).supplies.as_mut_ptr());
}

static spk_source_mux_text: [*const c_char; 4] = [
    b"PCM Monomix\0".as_ptr() as *const c_char,
    b"Analog In\0".as_ptr() as *const c_char,
    b"PDM Left\0".as_ptr() as *const c_char,
    b"PDM Right\0".as_ptr() as *const c_char,
];

static spk_source_mux_enum: soc_enum = SOC_ENUM_SINGLE(
    MAX98504_SPEAKER_SOURCE_SELECT,
    0,
    spk_source_mux_text.len() as c_uint,
    spk_source_mux_text.as_ptr(),
);

static spk_source_mux: snd_kcontrol_new = SOC_DAPM_ENUM(b"SPK Source\0".as_ptr() as *const c_char, &spk_source_mux_enum);

static max98504_dapm_routes: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: b"SPKOUT\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Global Enable\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"SPK Source\0".as_ptr() as *const c_char,
        control: b"PCM Monomix\0".as_ptr() as *const c_char,
        source: b"DAC PCM\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"SPK Source\0".as_ptr() as *const c_char,
        control: b"Analog In\0".as_ptr() as *const c_char,
        source: b"AIN\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"SPK Source\0".as_ptr() as *const c_char,
        control: b"PDM Left\0".as_ptr() as *const c_char,
        source: b"DAC PDM\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"SPK Source\0".as_ptr() as *const c_char,
        control: b"PDM Right\0".as_ptr() as *const c_char,
        source: b"DAC PDM\0".as_ptr() as *const c_char,
    },
];

static max98504_dapm_widgets: [snd_soc_dapm_widget; 8] = [
    SND_SOC_DAPM_SUPPLY(b"Global Enable\0".as_ptr() as *const c_char, MAX98504_GLOBAL_ENABLE, 0, 0, None, 0),
    SND_SOC_DAPM_INPUT(b"AIN\0".as_ptr() as *const c_char),
    SND_SOC_DAPM_AIF_OUT(b"AIF2OUTL\0".as_ptr() as *const c_char, b"AIF2 Capture\0".as_ptr() as *const c_char, 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT(b"AIF2OUTR\0".as_ptr() as *const c_char, b"AIF2 Capture\0".as_ptr() as *const c_char, 1, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC_E(
        b"DAC PCM\0".as_ptr() as *const c_char,
        ptr::null(),
        SND_SOC_NOPM,
        0,
        0,
        Some(max98504_pcm_rx_ev),
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD,
    ),
    SND_SOC_DAPM_DAC(b"DAC PDM\0".as_ptr() as *const c_char, ptr::null(), MAX98504_PDM_RX_ENABLE, 0, 0),
    SND_SOC_DAPM_MUX(b"SPK Source\0".as_ptr() as *const c_char, SND_SOC_NOPM, 0, 0, &spk_source_mux),
    SND_SOC_DAPM_REG(snd_soc_dapm_spk, b"SPKOUT\0".as_ptr() as *const c_char, MAX98504_SPEAKER_ENABLE, 0, 1, 1, 0),
];

unsafe extern "C" fn max98504_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    _slots: c_int,
    _slot_width: c_int,
) -> c_int {
    let max98504 = snd_soc_dai_get_drvdata(dai) as *mut max98504_priv;
    let map = (*max98504).regmap;

    match (*dai).id {
        MAX98504_DAI_ID_PCM => {
            regmap_write(map, MAX98504_PCM_TX_ENABLE, tx_mask);
            (*max98504).pcm_rx_channels = rx_mask;
        }

        MAX98504_DAI_ID_PDM => {
            regmap_write(map, MAX98504_PDM_TX_ENABLE, tx_mask);
        }
        _ => {
            WARN_ON(1);
        }
    }

    0
}

unsafe extern "C" fn max98504_set_channel_map(
    dai: *mut snd_soc_dai,
    tx_num: c_uint,
    tx_slot: *const c_uint,
    _rx_num: c_uint,
    _rx_slot: *const c_uint,
) -> c_int {
    let max98504 = snd_soc_dai_get_drvdata(dai) as *mut max98504_priv;
    let map = (*max98504).regmap;
    let mut i: c_uint;
    let mut sources: c_uint = 0;

    i = 0;
    while i < tx_num {
        if *tx_slot.add(i as usize) != 0 {
            sources |= 1 << i;
        }
        i += 1;
    }

    match (*dai).id {
        MAX98504_DAI_ID_PCM => {
            regmap_write(map, MAX98504_PCM_TX_CHANNEL_SOURCES, sources);
        }

        MAX98504_DAI_ID_PDM => {
            regmap_write(map, MAX98504_PDM_TX_CONTROL, sources);
        }
        _ => {
            WARN_ON(1);
        }
    }

    regmap_write(
        map,
        MAX98504_MEASUREMENT_ENABLE,
        if sources != 0 { 0x3 } else { 0x01 },
    );

    0
}

static max98504_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_tdm_slot: Some(max98504_set_tdm_slot),
    set_channel_map: Some(max98504_set_channel_map),
};

const MAX98504_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;
const MAX98504_PDM_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000;

static mut max98504_dai: [snd_soc_dai_driver; 1] = [
    /* TODO: Add the PCM interface definitions */
    snd_soc_dai_driver {
        name: b"max98504-aif2\0".as_ptr() as *const c_char,
        id: MAX98504_DAI_ID_PDM,
        playback: snd_soc_pcm_stream {
            stream_name: b"AIF2 Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: MAX98504_PDM_RATES,
            formats: MAX98504_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"AIF2 Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: MAX98504_PDM_RATES,
            formats: MAX98504_FORMATS,
        },
        ops: &max98504_dai_ops,
    },
];

static max98504_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(max98504_component_probe),
    remove: Some(max98504_component_remove),
    dapm_widgets: max98504_dapm_widgets.as_ptr(),
    num_dapm_widgets: max98504_dapm_widgets.len() as c_uint,
    dapm_routes: max98504_dapm_routes.as_ptr(),
    num_dapm_routes: max98504_dapm_routes.len() as c_uint,
    endianness: 1,
};

static max98504_regmap: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 8,
    max_register: MAX98504_MAX_REGISTER,
    reg_defaults: max98504_reg_defaults.as_ptr(),
    num_reg_defaults: max98504_reg_defaults.len() as c_uint,
    volatile_reg: Some(max98504_volatile_register),
    readable_reg: Some(max98504_readable_register),
    cache_type: REGCACHE_RBTREE,
};

unsafe extern "C" fn max98504_i2c_probe(client: *mut i2c_client) -> c_int {
    let dev = &mut (*client).dev as *mut device;
    let node = (*dev).of_node;
    let max98504: *mut max98504_priv;
    let mut i: c_int;
    let mut ret: c_int;

    max98504 = devm_kzalloc(dev, size_of::<max98504_priv>(), GFP_KERNEL) as *mut max98504_priv;
    if max98504.is_null() {
        return -ENOMEM;
    }

    if !node.is_null() {
        if of_property_read_u32(
            node,
            b"maxim,brownout-threshold\0".as_ptr() as *const c_char,
            &mut (*max98504).brownout_threshold,
        ) == 0
        {
            (*max98504).brownout_enable = true;
        }

        of_property_read_u32(
            node,
            b"maxim,brownout-attenuation\0".as_ptr() as *const c_char,
            &mut (*max98504).brownout_attenuation,
        );
        of_property_read_u32(
            node,
            b"maxim,brownout-attack-hold-ms\0".as_ptr() as *const c_char,
            &mut (*max98504).brownout_attack_hold,
        );
        of_property_read_u32(
            node,
            b"maxim,brownout-timed-hold-ms\0".as_ptr() as *const c_char,
            &mut (*max98504).brownout_timed_hold,
        );
        of_property_read_u32(
            node,
            b"maxim,brownout-release-rate-ms\0".as_ptr() as *const c_char,
            &mut (*max98504).brownout_release_rate,
        );
    }

    (*max98504).regmap = devm_regmap_init_i2c(client, &max98504_regmap);
    if IS_ERR((*max98504).regmap as *const _) {
        ret = PTR_ERR((*max98504).regmap as *const _);
        dev_err(
            &mut (*client).dev,
            b"regmap initialization failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    i = 0;
    while i < MAX98504_NUM_SUPPLIES as c_int {
        (*max98504).supplies[i as usize].supply = max98504_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(
        dev,
        MAX98504_NUM_SUPPLIES as c_int,
        (*max98504).supplies.as_mut_ptr(),
    );
    if ret < 0 {
        return ret;
    }

    i2c_set_clientdata(client, max98504 as *mut _);

    devm_snd_soc_register_component(
        dev,
        &max98504_component_driver,
        max98504_dai.as_mut_ptr(),
        max98504_dai.len() as c_int,
    )
}

// Original C conditional: #ifdef CONFIG_OF
#[cfg(CONFIG_OF)]
static max98504_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"maxim,max98504\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
#[cfg(CONFIG_OF)]
MODULE_DEVICE_TABLE!(of, max98504_of_match);

static max98504_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: b"max98504\0".as_ptr() as *const c_char,
    },
    i2c_device_id { name: ptr::null() },
];
MODULE_DEVICE_TABLE!(i2c, max98504_i2c_id);

static mut max98504_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"max98504\0".as_ptr() as *const c_char,
        of_match_table: of_match_ptr(max98504_of_match.as_ptr()),
    },
    probe: Some(max98504_i2c_probe),
    id_table: max98504_i2c_id.as_ptr(),
};
module_i2c_driver!(max98504_i2c_driver);

MODULE_DESCRIPTION!("ASoC MAX98504 driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
