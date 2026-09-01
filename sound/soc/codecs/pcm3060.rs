// SPDX-License-Identifier: GPL-2.0
//
// PCM3060 codec driver
//
// Copyright (C) 2018 Kirill Marinushkin <k.marinushkin@gmail.com>

// C dependencies translated as external Rust dependencies:
// linux/module.h, sound/pcm_params.h, sound/soc.h, sound/tlv.h, "pcm3060.h"

use core::ptr;

/* dai */

unsafe extern "C" fn pcm3060_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let comp: *mut snd_soc_component = unsafe { (*dai).component };
    let priv_: *mut pcm3060_priv =
        unsafe { snd_soc_component_get_drvdata(comp) as *mut pcm3060_priv };
    let reg: c_uint;
    let val: c_uint;

    if dir != SND_SOC_CLOCK_IN {
        unsafe {
            dev_err(
                (*comp).dev,
                c_str!("unsupported sysclock dir: %d\n").as_ptr(),
                dir,
            );
        }
        return -EINVAL;
    }

    match clk_id {
        PCM3060_CLK_DEF => {
            val = 0;
        }

        PCM3060_CLK1 => {
            val = if unsafe { (*dai).id } == PCM3060_DAI_ID_DAC {
                PCM3060_REG_CSEL
            } else {
                0
            };
        }

        PCM3060_CLK2 => {
            val = if unsafe { (*dai).id } == PCM3060_DAI_ID_DAC {
                0
            } else {
                PCM3060_REG_CSEL
            };
        }

        _ => {
            unsafe {
                dev_err(
                    (*comp).dev,
                    c_str!("unsupported sysclock id: %d\n").as_ptr(),
                    clk_id,
                );
            }
            return -EINVAL;
        }
    }

    if unsafe { (*dai).id } == PCM3060_DAI_ID_DAC {
        reg = PCM3060_REG67;
    } else {
        reg = PCM3060_REG72;
    }

    unsafe {
        regmap_update_bits((*priv_).regmap, reg, PCM3060_REG_CSEL, val);

        (*priv_).dai[(*dai).id as usize].sclk_freq = freq;
    }

    0
}

unsafe extern "C" fn pcm3060_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let comp: *mut snd_soc_component = unsafe { (*dai).component };
    let priv_: *mut pcm3060_priv =
        unsafe { snd_soc_component_get_drvdata(comp) as *mut pcm3060_priv };
    let reg: c_uint;
    let val: c_uint;

    if (fmt & SND_SOC_DAIFMT_INV_MASK) != SND_SOC_DAIFMT_NB_NF {
        unsafe {
            dev_err(
                (*comp).dev,
                c_str!("unsupported DAI polarity: 0x%x\n").as_ptr(),
                fmt,
            );
        }
        return -EINVAL;
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => unsafe {
            (*priv_).dai[(*dai).id as usize].is_provider = true;
        },
        SND_SOC_DAIFMT_CBC_CFC => unsafe {
            (*priv_).dai[(*dai).id as usize].is_provider = false;
        },
        _ => {
            unsafe {
                dev_err(
                    (*comp).dev,
                    c_str!("unsupported DAI mode: 0x%x\n").as_ptr(),
                    fmt,
                );
            }
            return -EINVAL;
        }
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            val = PCM3060_REG_FMT_I2S;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            val = PCM3060_REG_FMT_RJ;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            val = PCM3060_REG_FMT_LJ;
        }
        _ => {
            unsafe {
                dev_err(
                    (*comp).dev,
                    c_str!("unsupported DAI format: 0x%x\n").as_ptr(),
                    fmt,
                );
            }
            return -EINVAL;
        }
    }

    if unsafe { (*dai).id } == PCM3060_DAI_ID_DAC {
        reg = PCM3060_REG67;
    } else {
        reg = PCM3060_REG72;
    }

    unsafe {
        regmap_update_bits((*priv_).regmap, reg, PCM3060_REG_MASK_FMT, val);
    }

    0
}

unsafe extern "C" fn pcm3060_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let comp: *mut snd_soc_component = unsafe { (*dai).component };
    let priv_: *mut pcm3060_priv =
        unsafe { snd_soc_component_get_drvdata(comp) as *mut pcm3060_priv };
    let rate: c_uint;
    let ratio: c_uint;
    let reg: c_uint;
    let val: c_uint;

    let _ = substream;

    if unsafe { !(*priv_).dai[(*dai).id as usize].is_provider } {
        val = PCM3060_REG_MS_S;
    } else {
        rate = unsafe { params_rate(params) };
        if rate == 0 {
            unsafe {
                dev_err((*comp).dev, c_str!("rate is not configured\n").as_ptr());
            }
            return -EINVAL;
        }

        ratio = unsafe { (*priv_).dai[(*dai).id as usize].sclk_freq / rate };

        match ratio {
            768 => {
                val = PCM3060_REG_MS_M768;
            }
            512 => {
                val = PCM3060_REG_MS_M512;
            }
            384 => {
                val = PCM3060_REG_MS_M384;
            }
            256 => {
                val = PCM3060_REG_MS_M256;
            }
            192 => {
                val = PCM3060_REG_MS_M192;
            }
            128 => {
                val = PCM3060_REG_MS_M128;
            }
            _ => {
                unsafe {
                    dev_err((*comp).dev, c_str!("unsupported ratio: %d\n").as_ptr(), ratio);
                }
                return -EINVAL;
            }
        }
    }

    if unsafe { (*dai).id } == PCM3060_DAI_ID_DAC {
        reg = PCM3060_REG67;
    } else {
        reg = PCM3060_REG72;
    }

    unsafe {
        regmap_update_bits((*priv_).regmap, reg, PCM3060_REG_MASK_MS, val);
    }

    0
}

static pcm3060_selectable_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_I2S
        | SND_SOC_POSSIBLE_DAIFMT_RIGHT_J
        | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
        | SND_SOC_POSSIBLE_DAIFMT_NB_NF;

static pcm3060_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(pcm3060_set_sysclk),
    set_fmt: Some(pcm3060_set_fmt),
    hw_params: Some(pcm3060_hw_params),
    auto_selectable_formats: &pcm3060_selectable_formats,
    num_auto_selectable_formats: 1,
};

const PCM3060_DAI_RATES_ADC: c_uint = SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000;

const PCM3060_DAI_RATES_DAC: c_uint =
    PCM3060_DAI_RATES_ADC | SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_192000;

static mut pcm3060_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c_str!("pcm3060-dac").as_ptr(),
        id: PCM3060_DAI_ID_DAC,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("Playback").as_ptr(),
            channels_min: 2,
            channels_max: 2,
            rates: PCM3060_DAI_RATES_DAC,
            formats: SNDRV_PCM_FMTBIT_S24_LE,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &pcm3060_dai_ops,
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("pcm3060-adc").as_ptr(),
        id: PCM3060_DAI_ID_ADC,
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("Capture").as_ptr(),
            channels_min: 2,
            channels_max: 2,
            rates: PCM3060_DAI_RATES_ADC,
            formats: SNDRV_PCM_FMTBIT_S24_LE,
            ..unsafe { core::mem::zeroed() }
        },
        ops: &pcm3060_dai_ops,
        ..unsafe { core::mem::zeroed() }
    },
];

/* dapm */

static pcm3060_dapm_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-10050, 50, 1);

static pcm3060_dapm_controls: [snd_kcontrol_new; 4] = [
    SOC_DOUBLE_R_RANGE_TLV!(
        "Master Playback Volume",
        PCM3060_REG65,
        PCM3060_REG66,
        0,
        PCM3060_REG_AT2_MIN,
        PCM3060_REG_AT2_MAX,
        0,
        pcm3060_dapm_tlv
    ),
    SOC_DOUBLE!(
        "Master Playback Switch",
        PCM3060_REG68,
        PCM3060_REG_SHIFT_MUT21,
        PCM3060_REG_SHIFT_MUT22,
        1,
        1
    ),
    SOC_DOUBLE_R_RANGE_TLV!(
        "Master Capture Volume",
        PCM3060_REG70,
        PCM3060_REG71,
        0,
        PCM3060_REG_AT1_MIN,
        PCM3060_REG_AT1_MAX,
        0,
        pcm3060_dapm_tlv
    ),
    SOC_DOUBLE!(
        "Master Capture Switch",
        PCM3060_REG73,
        PCM3060_REG_SHIFT_MUT11,
        PCM3060_REG_SHIFT_MUT12,
        1,
        1
    ),
];

static pcm3060_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_DAC!(
        "DAC",
        "Playback",
        PCM3060_REG64,
        PCM3060_REG_SHIFT_DAPSV,
        1
    ),
    SND_SOC_DAPM_OUTPUT!("OUTL"),
    SND_SOC_DAPM_OUTPUT!("OUTR"),
    SND_SOC_DAPM_INPUT!("INL"),
    SND_SOC_DAPM_INPUT!("INR"),
    SND_SOC_DAPM_ADC!(
        "ADC",
        "Capture",
        PCM3060_REG64,
        PCM3060_REG_SHIFT_ADPSV,
        1
    ),
];

static pcm3060_dapm_map: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: c_str!("OUTL").as_ptr(),
        control: ptr::null(),
        source: c_str!("DAC").as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c_str!("OUTR").as_ptr(),
        control: ptr::null(),
        source: c_str!("DAC").as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c_str!("ADC").as_ptr(),
        control: ptr::null(),
        source: c_str!("INL").as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c_str!("ADC").as_ptr(),
        control: ptr::null(),
        source: c_str!("INR").as_ptr(),
    },
];

/* soc component */

static pcm3060_soc_comp_driver: snd_soc_component_driver = snd_soc_component_driver {
    controls: pcm3060_dapm_controls.as_ptr(),
    num_controls: pcm3060_dapm_controls.len() as c_uint,
    dapm_widgets: pcm3060_dapm_widgets.as_ptr(),
    num_dapm_widgets: pcm3060_dapm_widgets.len() as c_uint,
    dapm_routes: pcm3060_dapm_map.as_ptr(),
    num_dapm_routes: pcm3060_dapm_map.len() as c_uint,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

/* regmap */

unsafe extern "C" fn pcm3060_reg_writeable(dev: *mut device, reg: c_uint) -> bool {
    let _ = dev;

    reg >= PCM3060_REG64
}

unsafe extern "C" fn pcm3060_reg_readable(dev: *mut device, reg: c_uint) -> bool {
    let _ = dev;

    reg >= PCM3060_REG64
}

unsafe extern "C" fn pcm3060_reg_volatile(dev: *mut device, reg: c_uint) -> bool {
    let _ = dev;

    /* PCM3060_REG64 is volatile */
    reg == PCM3060_REG64
}

static pcm3060_reg_defaults: [reg_default; 10] = [
    reg_default {
        reg: PCM3060_REG64,
        def: 0xF0,
    },
    reg_default {
        reg: PCM3060_REG65,
        def: 0xFF,
    },
    reg_default {
        reg: PCM3060_REG66,
        def: 0xFF,
    },
    reg_default {
        reg: PCM3060_REG67,
        def: 0x00,
    },
    reg_default {
        reg: PCM3060_REG68,
        def: 0x00,
    },
    reg_default {
        reg: PCM3060_REG69,
        def: 0x00,
    },
    reg_default {
        reg: PCM3060_REG70,
        def: 0xD7,
    },
    reg_default {
        reg: PCM3060_REG71,
        def: 0xD7,
    },
    reg_default {
        reg: PCM3060_REG72,
        def: 0x00,
    },
    reg_default {
        reg: PCM3060_REG73,
        def: 0x00,
    },
];

#[no_mangle]
pub static pcm3060_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    writeable_reg: Some(pcm3060_reg_writeable),
    readable_reg: Some(pcm3060_reg_readable),
    volatile_reg: Some(pcm3060_reg_volatile),
    max_register: PCM3060_REG73,
    reg_defaults: pcm3060_reg_defaults.as_ptr(),
    num_reg_defaults: pcm3060_reg_defaults.len() as c_uint,
    cache_type: REGCACHE_RBTREE,
    ..unsafe { core::mem::zeroed() }
};
// EXPORT_SYMBOL(pcm3060_regmap);

/* device */

unsafe fn pcm3060_parse_dt(np: *const device_node, priv_: *mut pcm3060_priv) {
    unsafe {
        (*priv_).out_se = of_property_read_bool(np, c_str!("ti,out-single-ended").as_ptr());
    }
}

#[no_mangle]
pub unsafe extern "C" fn pcm3060_probe(dev: *mut device) -> c_int {
    let mut rc: c_int;
    let priv_: *mut pcm3060_priv = unsafe { dev_get_drvdata(dev) as *mut pcm3060_priv };

    /* soft reset */
    rc = unsafe { regmap_update_bits((*priv_).regmap, PCM3060_REG64, PCM3060_REG_MRST, 0) };
    if rc != 0 {
        unsafe {
            dev_err(
                dev,
                c_str!("failed to reset component, rc=%d\n").as_ptr(),
                rc,
            );
        }
        return rc;
    }

    if unsafe { !(*dev).of_node.is_null() } {
        unsafe {
            pcm3060_parse_dt((*dev).of_node, priv_);
        }
    }

    if unsafe { (*priv_).out_se } {
        unsafe {
            regmap_update_bits(
                (*priv_).regmap,
                PCM3060_REG64,
                PCM3060_REG_SE,
                PCM3060_REG_SE,
            );
        }
    }

    rc = unsafe {
        devm_snd_soc_register_component(
            dev,
            &pcm3060_soc_comp_driver,
            core::ptr::addr_of_mut!(pcm3060_dai[0]),
            pcm3060_dai.len() as c_int,
        )
    };
    if rc != 0 {
        unsafe {
            dev_err(
                dev,
                c_str!("failed to register component, rc=%d\n").as_ptr(),
                rc,
            );
        }
        return rc;
    }

    0
}
// EXPORT_SYMBOL(pcm3060_probe);

MODULE_DESCRIPTION!("PCM3060 codec driver");
MODULE_AUTHOR!("Kirill Marinushkin <k.marinushkin@gmail.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
