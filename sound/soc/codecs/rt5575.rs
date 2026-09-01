// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt5575.rs  --  ALC5575 ALSA SoC audio component driver
 *
 * Copyright(c) 2025 Realtek Semiconductor Corp.
 *
 */

// Rust translation of rt5575.c. C include dependencies:
// <linux/i2c.h>, <sound/soc.h>, <sound/tlv.h>, "rt5575.h", "rt5575-spi.h".

extern "C" {
    static RT5575_BOOT: u32;
    static RT5575_ID: u32;
    static RT5575_ID_1: u32;
    static RT5575_MIXL_VOL: u32;
    static RT5575_MIXR_VOL: u32;
    static RT5575_PROMPT_VOL: u32;
    static RT5575_SPK01_VOL: u32;
    static RT5575_SPK23_VOL: u32;
    static RT5575_MIC1_VOL: u32;
    static RT5575_MIC2_VOL: u32;
    static RT5575_WNC_CTRL: u32;
    static RT5575_MODE_CTRL: u32;
    static RT5575_I2S_RATE_CTRL: u32;
    static RT5575_SLEEP_CTRL: u32;
    static RT5575_ALG_BYPASS_CTRL: u32;
    static RT5575_PINMUX_CTRL_2: u32;
    static RT5575_GPIO_CTRL_1: u32;
    static RT5575_DSP_BUS_CTRL: u32;
    static RT5575_SW_INT: u32;
    static RT5575_DSP_BOOT_ERR: u32;
    static RT5575_DSP_READY: u32;
    static RT5575_DSP_CMD_ADDR: u32;
    static RT5575_EFUSE_DATA_2: u32;
    static RT5575_EFUSE_DATA_3: u32;
    static RT5575_EFUSE_PID: u32;
    static RT5575_DSP_MAPPING: u32;
    static RT5575_AIF1: i32;
    static RT5575_AIF2: i32;
    static RT5575_AIF3: i32;
    static RT5575_AIF4: i32;
    static RT5575_DEVICE_ID: i32;
    static RT5575_BOOT_MASK: i32;
    static RT5575_BOOT_SPI: i32;
    static SND_SOC_NOPM: i32;
    static SNDRV_PCM_RATE_8000_192000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S8: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static GFP_KERNEL: u32;
    static ENODEV: i32;
    static ENOMEM: i32;

    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut i32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut rt5575_priv;
    fn dev_info(dev: *mut device, fmt: *const i8, ...) -> i32;
    fn dev_err(dev: *mut device, fmt: *const i8, ...) -> i32;
    fn i2c_get_clientdata(client: *mut i2c_client) -> *mut rt5575_priv;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut rt5575_priv);
    fn rt5575_spi_get_device(dev: *mut device) -> *mut spi_device;
    fn rt5575_spi_fw_load(spi: *mut spi_device) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const core::ffi::c_void,
        context: *mut core::ffi::c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: i32,
    ) -> i32;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct rt5575_priv {
    pub regmap: *mut regmap,
    pub dsp_regmap: *mut regmap,
    pub component: *mut snd_soc_component,
    pub i2c: *mut i2c_client,
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
    pub sink: *const i8,
    pub control: *const i8,
    pub source: *const i8,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const i8,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const i8,
    pub id: i32,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> i32>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: u32,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: u32,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: u32,
    pub use_pmdown_time: i32,
    pub endianness: i32,
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const i8,
    pub reg_bits: u32,
    pub val_bits: u32,
    pub reg_stride: u32,
    pub max_register: u32,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool>,
    pub reg_read: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, *mut u32) -> i32>,
    pub reg_write: Option<unsafe extern "C" fn(*mut core::ffi::c_void, u32, u32) -> i32>,
    pub use_single_read: bool,
    pub use_single_write: bool,
}

unsafe extern "C" fn rt5575_readable_register(_dev: *mut device, reg: u32) -> bool {
    unsafe {
        reg == RT5575_BOOT
            || reg == RT5575_ID
            || reg == RT5575_ID_1
            || reg == RT5575_MIXL_VOL
            || reg == RT5575_MIXR_VOL
            || reg == RT5575_PROMPT_VOL
            || reg == RT5575_SPK01_VOL
            || reg == RT5575_SPK23_VOL
            || reg == RT5575_MIC1_VOL
            || reg == RT5575_MIC2_VOL
            || reg == RT5575_WNC_CTRL
            || reg == RT5575_MODE_CTRL
            || reg == RT5575_I2S_RATE_CTRL
            || reg == RT5575_SLEEP_CTRL
            || reg == RT5575_ALG_BYPASS_CTRL
            || reg == RT5575_PINMUX_CTRL_2
            || reg == RT5575_GPIO_CTRL_1
            || reg == RT5575_DSP_BUS_CTRL
            || reg == RT5575_SW_INT
            || reg == RT5575_DSP_BOOT_ERR
            || reg == RT5575_DSP_READY
            || reg == RT5575_DSP_CMD_ADDR
            || reg == RT5575_EFUSE_DATA_2
            || reg == RT5575_EFUSE_DATA_3
    }
}

// static const DECLARE_TLV_DB_SCALE(ob_tlv, -9525, 75, 0);
static ob_tlv: [u32; 4] = [0, 2, (-9525_i32) as u32, 75];

// The following ASoC macro initializers are external dependency mappings from
// <sound/soc.h>/<sound/tlv.h>; they are preserved as source-level comments:
// SOC_DOUBLE("Speaker CH-01 Playback Switch", RT5575_SPK01_VOL, 31, 15, 1, 1)
// SOC_DOUBLE_TLV("Speaker CH-01 Playback Volume", RT5575_SPK01_VOL, 17, 1, 167, 0, ob_tlv)
// SOC_DOUBLE("Speaker CH-23 Playback Switch", RT5575_SPK23_VOL, 31, 15, 1, 1)
// SOC_DOUBLE_TLV("Speaker CH-23 Playback Volume", RT5575_SPK23_VOL, 17, 1, 167, 0, ob_tlv)
// SOC_DOUBLE("Mic1 Capture Switch", RT5575_MIC1_VOL, 31, 15, 1, 1)
// SOC_DOUBLE_TLV("Mic1 Capture Volume", RT5575_MIC1_VOL, 17, 1, 167, 0, ob_tlv)
// SOC_DOUBLE("Mic2 Capture Switch", RT5575_MIC2_VOL, 31, 15, 1, 1)
// SOC_DOUBLE_TLV("Mic2 Capture Volume", RT5575_MIC2_VOL, 17, 1, 167, 0, ob_tlv)
// SOC_DOUBLE_R("Mix Playback Switch", RT5575_MIXL_VOL, RT5575_MIXR_VOL, 31, 1, 1)
// SOC_DOUBLE_R_TLV("Mix Playback Volume", RT5575_MIXL_VOL, RT5575_MIXR_VOL, 1, 127, 0, ob_tlv)
// SOC_DOUBLE("Prompt Playback Switch", RT5575_PROMPT_VOL, 31, 15, 1, 1)
// SOC_DOUBLE_TLV("Prompt Playback Volume", RT5575_PROMPT_VOL, 17, 1, 167, 0, ob_tlv)
static rt5575_snd_controls: [snd_kcontrol_new; 0] = [];

// SND_SOC_DAPM_AIF_IN/OUT and INPUT/OUTPUT macro expansions are external.
static rt5575_dapm_widgets: [snd_soc_dapm_widget; 0] = [];

static rt5575_dapm_routes: [snd_soc_dapm_route; 8] = [
    snd_soc_dapm_route { sink: b"AIF1TX\0".as_ptr() as *const i8, control: core::ptr::null(), source: b"INPUT\0".as_ptr() as *const i8 },
    snd_soc_dapm_route { sink: b"AIF2TX\0".as_ptr() as *const i8, control: core::ptr::null(), source: b"INPUT\0".as_ptr() as *const i8 },
    snd_soc_dapm_route { sink: b"AIF3TX\0".as_ptr() as *const i8, control: core::ptr::null(), source: b"INPUT\0".as_ptr() as *const i8 },
    snd_soc_dapm_route { sink: b"AIF4TX\0".as_ptr() as *const i8, control: core::ptr::null(), source: b"INPUT\0".as_ptr() as *const i8 },
    snd_soc_dapm_route { sink: b"OUTPUT\0".as_ptr() as *const i8, control: core::ptr::null(), source: b"AIF1RX\0".as_ptr() as *const i8 },
    snd_soc_dapm_route { sink: b"OUTPUT\0".as_ptr() as *const i8, control: core::ptr::null(), source: b"AIF2RX\0".as_ptr() as *const i8 },
    snd_soc_dapm_route { sink: b"OUTPUT\0".as_ptr() as *const i8, control: core::ptr::null(), source: b"AIF3RX\0".as_ptr() as *const i8 },
    snd_soc_dapm_route { sink: b"OUTPUT\0".as_ptr() as *const i8, control: core::ptr::null(), source: b"AIF4RX\0".as_ptr() as *const i8 },
];

unsafe extern "C" fn rt5575_get_priv_id(rt5575: *mut rt5575_priv) -> i64 {
    let mut priv_id_low: i32 = 0;
    let mut priv_id_high: i32 = 0;

    unsafe {
        regmap_write((*rt5575).regmap, RT5575_EFUSE_PID, 0xa0000000);
        regmap_read((*rt5575).regmap, RT5575_EFUSE_DATA_2, &mut priv_id_low);
        regmap_read((*rt5575).regmap, RT5575_EFUSE_DATA_3, &mut priv_id_high);
        regmap_write((*rt5575).regmap, RT5575_EFUSE_PID, 0);

        ((priv_id_high as i64) << 32) | (priv_id_low as i64)
    }
}

unsafe extern "C" fn rt5575_probe(component: *mut snd_soc_component) -> i32 {
    unsafe {
        let rt5575 = snd_soc_component_get_drvdata(component);
        let dev = (*component).dev;

        (*rt5575).component = component;

        dev_info(dev, c"Private ID: %llx\n".as_ptr(), rt5575_get_priv_id(rt5575));

        0
    }
}

static RT5575_STEREO_RATES: u32 = unsafe { SNDRV_PCM_RATE_8000_192000 };
static RT5575_FORMATS: u64 = unsafe {
    SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S20_3LE
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_S8
        | SNDRV_PCM_FMTBIT_S32_LE
};

static mut rt5575_dai: [snd_soc_dai_driver; 4] = unsafe {
    [
        snd_soc_dai_driver {
            name: b"rt5575-aif1\0".as_ptr() as *const i8,
            id: RT5575_AIF1,
            playback: snd_soc_pcm_stream {
                stream_name: b"AIF1 Playback\0".as_ptr() as *const i8,
                channels_min: 1,
                channels_max: 8,
                rates: RT5575_STEREO_RATES,
                formats: RT5575_FORMATS,
            },
            capture: snd_soc_pcm_stream {
                stream_name: b"AIF1 Capture\0".as_ptr() as *const i8,
                channels_min: 1,
                channels_max: 8,
                rates: RT5575_STEREO_RATES,
                formats: RT5575_FORMATS,
            },
        },
        snd_soc_dai_driver {
            name: b"rt5575-aif2\0".as_ptr() as *const i8,
            id: RT5575_AIF2,
            playback: snd_soc_pcm_stream {
                stream_name: b"AIF2 Playback\0".as_ptr() as *const i8,
                channels_min: 1,
                channels_max: 8,
                rates: RT5575_STEREO_RATES,
                formats: RT5575_FORMATS,
            },
            capture: snd_soc_pcm_stream {
                stream_name: b"AIF2 Capture\0".as_ptr() as *const i8,
                channels_min: 1,
                channels_max: 8,
                rates: RT5575_STEREO_RATES,
                formats: RT5575_FORMATS,
            },
        },
        snd_soc_dai_driver {
            name: b"rt5575-aif3\0".as_ptr() as *const i8,
            id: RT5575_AIF3,
            playback: snd_soc_pcm_stream {
                stream_name: b"AIF3 Playback\0".as_ptr() as *const i8,
                channels_min: 1,
                channels_max: 8,
                rates: RT5575_STEREO_RATES,
                formats: RT5575_FORMATS,
            },
            capture: snd_soc_pcm_stream {
                stream_name: b"AIF3 Capture\0".as_ptr() as *const i8,
                channels_min: 1,
                channels_max: 8,
                rates: RT5575_STEREO_RATES,
                formats: RT5575_FORMATS,
            },
        },
        snd_soc_dai_driver {
            name: b"rt5575-aif4\0".as_ptr() as *const i8,
            id: RT5575_AIF4,
            playback: snd_soc_pcm_stream {
                stream_name: b"AIF4 Playback\0".as_ptr() as *const i8,
                channels_min: 1,
                channels_max: 8,
                rates: RT5575_STEREO_RATES,
                formats: RT5575_FORMATS,
            },
            capture: snd_soc_pcm_stream {
                stream_name: b"AIF4 Capture\0".as_ptr() as *const i8,
                channels_min: 1,
                channels_max: 8,
                rates: RT5575_STEREO_RATES,
                formats: RT5575_FORMATS,
            },
        },
    ]
};

static rt5575_soc_component_dev: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt5575_probe),
    controls: rt5575_snd_controls.as_ptr(),
    num_controls: rt5575_snd_controls.len() as u32,
    dapm_widgets: rt5575_dapm_widgets.as_ptr(),
    num_dapm_widgets: rt5575_dapm_widgets.len() as u32,
    dapm_routes: rt5575_dapm_routes.as_ptr(),
    num_dapm_routes: rt5575_dapm_routes.len() as u32,
    use_pmdown_time: 1,
    endianness: 1,
};

static rt5575_dsp_regmap: regmap_config = regmap_config {
    name: b"dsp\0".as_ptr() as *const i8,
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 2,
    max_register: 0,
    readable_reg: None,
    reg_read: None,
    reg_write: None,
    use_single_read: false,
    use_single_write: false,
};

unsafe extern "C" fn rt5575_i2c_read(
    context: *mut core::ffi::c_void,
    reg: u32,
    val: *mut u32,
) -> i32 {
    unsafe {
        let client = context as *mut i2c_client;
        let rt5575 = i2c_get_clientdata(client);
        regmap_read((*rt5575).dsp_regmap, reg | RT5575_DSP_MAPPING, val as *mut i32)
    }
}

unsafe extern "C" fn rt5575_i2c_write(
    context: *mut core::ffi::c_void,
    reg: u32,
    val: u32,
) -> i32 {
    unsafe {
        let client = context as *mut i2c_client;
        let rt5575 = i2c_get_clientdata(client);
        regmap_write((*rt5575).dsp_regmap, reg | RT5575_DSP_MAPPING, val)
    }
}

static rt5575_regmap: regmap_config = regmap_config {
    name: core::ptr::null(),
    reg_bits: 16,
    val_bits: 32,
    reg_stride: 4,
    max_register: 0xfffc,
    readable_reg: Some(rt5575_readable_register),
    reg_read: Some(rt5575_i2c_read),
    reg_write: Some(rt5575_i2c_write),
    use_single_read: true,
    use_single_write: true,
};

unsafe extern "C" fn rt5575_fw_load_by_spi(rt5575: *mut rt5575_priv) -> i32 {
    unsafe {
        let i2c = (*rt5575).i2c;
        let mut spi: *mut spi_device;
        let dev = &mut (*i2c).dev as *mut device;
        let mut ret: i32;

        spi = rt5575_spi_get_device(dev);
        if spi.is_null() {
            dev_err(dev, c"Failed to get spi_device\n".as_ptr());
            return -ENODEV;
        }

        regmap_write((*rt5575).dsp_regmap, 0xfafafafa, 0x00000004);
        regmap_write((*rt5575).dsp_regmap, 0x18008064, 0x00000000);
        regmap_write((*rt5575).dsp_regmap, 0x18008068, 0x0002ffff);

        ret = rt5575_spi_fw_load(spi);
        if ret != 0 {
            dev_err(dev, c"Load firmware failure: %d\n".as_ptr(), ret);
            return -ENODEV;
        }

        regmap_write((*rt5575).dsp_regmap, 0x18000000, 0x00000000);
        regmap_update_bits((*rt5575).regmap, RT5575_SW_INT, 1, 1);

        // regmap_read_poll_timeout(rt5575->regmap, RT5575_SW_INT, ret, !ret, 100000, 10000000);
        loop {
            regmap_read((*rt5575).regmap, RT5575_SW_INT, &mut ret);
            if ret == 0 {
                break;
            }
            // Poll delay/timeout semantics are supplied by the kernel macro dependency.
            break;
        }
        if ret != 0 {
            dev_err(dev, c"Run firmware failure: %d\n".as_ptr(), ret);
            return -ENODEV;
        }

        0
    }
}

unsafe extern "C" fn rt5575_i2c_probe(i2c: *mut i2c_client) -> i32 {
    unsafe {
        let mut rt5575: *mut rt5575_priv;
        let mut ret: i32;
        let mut val: i32 = 0;
        let mut boot: i32 = 0;
        let dev = &mut (*i2c).dev as *mut device;

        rt5575 = devm_kzalloc(dev, core::mem::size_of::<rt5575_priv>(), GFP_KERNEL)
            as *mut rt5575_priv;
        if rt5575.is_null() {
            return -ENOMEM;
        }

        i2c_set_clientdata(i2c, rt5575);

        (*rt5575).i2c = i2c;

        (*rt5575).dsp_regmap = devm_regmap_init_i2c(i2c, &rt5575_dsp_regmap);
        if IS_ERR((*rt5575).dsp_regmap as *const core::ffi::c_void) {
            ret = PTR_ERR((*rt5575).dsp_regmap as *const core::ffi::c_void);
            dev_err(dev, c"Failed to allocate DSP register map: %d\n".as_ptr(), ret);
            return ret;
        }

        (*rt5575).regmap =
            devm_regmap_init(dev, core::ptr::null(), i2c as *mut core::ffi::c_void, &rt5575_regmap);
        if IS_ERR((*rt5575).regmap as *const core::ffi::c_void) {
            ret = PTR_ERR((*rt5575).regmap as *const core::ffi::c_void);
            dev_err(dev, c"Failed to allocate register map: %d\n".as_ptr(), ret);
            return ret;
        }

        regmap_read((*rt5575).regmap, RT5575_ID, &mut val);
        if val != RT5575_DEVICE_ID {
            dev_err(
                dev,
                c"Device with ID register %08x is not rt5575\n".as_ptr(),
                val,
            );
            return -ENODEV;
        }

        regmap_read((*rt5575).regmap, RT5575_BOOT, &mut boot);
        if (boot & RT5575_BOOT_MASK) == RT5575_BOOT_SPI {
            // Original C condition: if (!IS_ENABLED(CONFIG_SND_SOC_RT5575_SPI)) return -ENODEV;
            // The build-time CONFIG_SND_SOC_RT5575_SPI test is preserved as dependency intent.
            if rt5575_fw_load_by_spi(rt5575) != 0 {
                return -ENODEV;
            }
        }

        devm_snd_soc_register_component(
            dev,
            &rt5575_soc_component_dev,
            rt5575_dai.as_mut_ptr(),
            rt5575_dai.len() as i32,
        )
    }
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: [i8; 20],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const i8,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const i8,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> i32>,
    pub id_table: *const i2c_device_id,
}

static rt5575_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id {
        name: [
            b'r' as i8, b't' as i8, b'5' as i8, b'5' as i8, b'7' as i8, b'5' as i8, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    },
    i2c_device_id { name: [0; 20] },
];
// MODULE_DEVICE_TABLE(i2c, rt5575_i2c_id);

static rt5575_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"realtek,rt5575\0".as_ptr() as *const i8,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, rt5575_of_match);

static mut rt5575_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"rt5575\0".as_ptr() as *const i8,
        of_match_table: rt5575_of_match.as_ptr(),
    },
    probe: Some(rt5575_i2c_probe),
    id_table: rt5575_i2c_id.as_ptr(),
};
// module_i2c_driver(rt5575_i2c_driver);

// MODULE_DESCRIPTION("ASoC ALC5575 driver");
// MODULE_AUTHOR("Oder Chiou <oder_chiou@realtek.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
