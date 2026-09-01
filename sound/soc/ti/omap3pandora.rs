// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap3pandora.c  --  SoC audio for Pandora Handheld Console
 *
 * Author: Gražvydas Ignotas <notasas@gmail.com>
 */

// Dependencies from the original C includes:
// linux/clk.h, linux/platform_device.h, linux/gpio/consumer.h,
// linux/delay.h, linux/regulator/consumer.h, linux/module.h, linux/of.h,
// sound/core.h, sound/pcm.h, sound/soc.h,
// linux/platform_data/asoc-ti-mcbsp.h, and "omap-mcbsp.h".

use core::ffi::{c_char, c_int, c_long, c_uint};
use core::ptr;

const PREFIX: &[u8] = b"ASoC omap3pandora: \0";

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
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
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
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
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    pub init: Option<unsafe extern "C" fn(rtd: *mut snd_soc_pcm_runtime) -> c_int>,
    // SND_SOC_DAILINK_REG(...) appends fields supplied by the ASoC headers.
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const c_char,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget_desc,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_clkdiv(dai: *mut snd_soc_dai, div_id: c_int, div: c_int) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;

    fn snd_soc_dapm_to_dev(dapm: *mut snd_soc_dapm_context) -> *mut device;
    fn regulator_enable(regulator: *mut regulator) -> c_int;
    fn regulator_disable(regulator: *mut regulator) -> c_int;
    fn regulator_get(dev: *mut device, id: *const c_char) -> *mut regulator;
    fn regulator_put(regulator: *mut regulator);
    fn mdelay(msecs: c_uint);
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn devm_gpiod_get(dev: *mut device, con_id: *const c_char, flags: c_int) -> *mut gpio_desc;

    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const c_char) -> c_int;

    fn of_machine_is_compatible(compat: *const c_char) -> c_int;
    fn platform_device_alloc(name: *const c_char, id: c_int) -> *mut platform_device;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn platform_device_add(pdev: *mut platform_device) -> c_int;
    fn platform_device_del(pdev: *mut platform_device);
    fn platform_device_put(pdev: *mut platform_device);
    fn platform_device_unregister(pdev: *mut platform_device);
    fn dev_name(dev: *mut device) -> *const c_char;

    fn IS_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_long;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

const SND_SOC_CLOCK_IN: c_int = 0;
const OMAP_MCBSP_SYSCLK_CLKS_EXT: c_int = 0;
const OMAP_MCBSP_CLKGDV: c_int = 0;
const SND_SOC_NOPM: c_int = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 0;
const SND_SOC_DAPM_PRE_PMD: c_int = 0;
const SND_SOC_DAIFMT_I2S: c_uint = 0;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0;
const GPIOD_OUT_LOW: c_int = 0;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;

static mut omap3pandora_dac_reg: *mut regulator = ptr::null_mut();
static mut dac_power_gpio: *mut gpio_desc = ptr::null_mut();
static mut amp_power_gpio: *mut gpio_desc = ptr::null_mut();

unsafe extern "C" fn omap3pandora_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret: c_int;

    /* Set the codec system clock for DAC and ADC */
    ret = snd_soc_dai_set_sysclk(codec_dai, 0, 26000000, SND_SOC_CLOCK_IN);
    if ret < 0 {
        pr_err(
            b"%s%s\0".as_ptr() as *const c_char,
            PREFIX.as_ptr() as *const c_char,
            b"can't set codec system clock\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    /* Set McBSP clock to external */
    ret = snd_soc_dai_set_sysclk(
        cpu_dai,
        OMAP_MCBSP_SYSCLK_CLKS_EXT,
        256u32.wrapping_mul(params_rate(params)),
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        pr_err(
            b"%s%s\0".as_ptr() as *const c_char,
            PREFIX.as_ptr() as *const c_char,
            b"can't set cpu system clock\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    ret = snd_soc_dai_set_clkdiv(cpu_dai, OMAP_MCBSP_CLKGDV, 8);
    if ret < 0 {
        pr_err(
            b"%s%s\0".as_ptr() as *const c_char,
            PREFIX.as_ptr() as *const c_char,
            b"can't set SRG clock divider\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    0
}

unsafe extern "C" fn omap3pandora_dac_event(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let ret: c_int;

    /*
     * The PCM1773 DAC datasheet requires 1ms delay between switching
     * VCC power on/off and /PD pin high/low
     */
    if SND_SOC_DAPM_EVENT_ON(event) {
        let dev = snd_soc_dapm_to_dev((*w).dapm);

        ret = regulator_enable(omap3pandora_dac_reg);
        if ret != 0 {
            dev_err(dev, b"Failed to power DAC: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        mdelay(1);
        gpiod_set_value(dac_power_gpio, 1);
    } else {
        gpiod_set_value(dac_power_gpio, 0);
        mdelay(1);
        regulator_disable(omap3pandora_dac_reg);
    }

    0
}

unsafe extern "C" fn omap3pandora_hp_event(
    _w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    if SND_SOC_DAPM_EVENT_ON(event) {
        gpiod_set_value(amp_power_gpio, 1);
    } else {
        gpiod_set_value(amp_power_gpio, 0);
    }

    0
}

fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool {
    event != 0
}

/*
 * Audio paths on Pandora board:
 *
 *  |O| ---> PCM DAC +-> AMP -> Headphone Jack
 *  |M|         A    +--------> Line Out
 *  |A| <~~clk~~+
 *  |P| <--- TWL4030 <--------- Line In and MICs
 */
static omap3pandora_dapm_widgets: [snd_soc_dapm_widget_desc; 7] = [
    SND_SOC_DAPM_DAC_E!(
        b"PCM DAC\0",
        b"HiFi Playback\0",
        SND_SOC_NOPM,
        0,
        0,
        omap3pandora_dac_event,
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD
    ),
    SND_SOC_DAPM_PGA_E!(
        b"Headphone Amplifier\0",
        SND_SOC_NOPM,
        0,
        0,
        ptr::null_mut(),
        0,
        omap3pandora_hp_event,
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD
    ),
    SND_SOC_DAPM_HP!(b"Headphone Jack\0", ptr::null_mut()),
    SND_SOC_DAPM_LINE!(b"Line Out\0", ptr::null_mut()),
    SND_SOC_DAPM_MIC!(b"Mic (internal)\0", ptr::null_mut()),
    SND_SOC_DAPM_MIC!(b"Mic (external)\0", ptr::null_mut()),
    SND_SOC_DAPM_LINE!(b"Line In\0", ptr::null_mut()),
];

static omap3pandora_map: [snd_soc_dapm_route; 10] = [
    snd_soc_dapm_route {
        sink: b"PCM DAC\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"APLL Enable\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headphone Amplifier\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"PCM DAC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Line Out\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"PCM DAC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Headphone Jack\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Headphone Amplifier\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"AUXL\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Line In\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"AUXR\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Line In\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"MAINMIC\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Mic (internal)\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Mic (internal)\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Mic Bias 1\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"SUBMIC\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Mic (external)\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"Mic (external)\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"Mic Bias 2\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn omap3pandora_out_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let dapm = snd_soc_card_to_dapm((*rtd).card);

    /* All TWL4030 output pins are floating */
    snd_soc_dapm_disable_pin(dapm, b"EARPIECE\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"PREDRIVEL\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"PREDRIVER\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"HSOL\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"HSOR\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"CARKITL\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"CARKITR\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"HFL\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"HFR\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"VIBRA\0".as_ptr() as *const c_char);

    0
}

unsafe extern "C" fn omap3pandora_in_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let dapm = snd_soc_card_to_dapm((*rtd).card);

    /* Not comnnected */
    snd_soc_dapm_disable_pin(dapm, b"HSMIC\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"CARKITMIC\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"DIGIMIC0\0".as_ptr() as *const c_char);
    snd_soc_dapm_disable_pin(dapm, b"DIGIMIC1\0".as_ptr() as *const c_char);

    0
}

static omap3pandora_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(omap3pandora_hw_params),
};

/* Digital audio interface glue - connects codec <--> CPU */
// Original C:
// SND_SOC_DAILINK_DEFS(out,
//     DAILINK_COMP_ARRAY(COMP_CPU("omap-mcbsp.2")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("twl4030-codec", "twl4030-hifi")),
//     DAILINK_COMP_ARRAY(COMP_PLATFORM("omap-mcbsp.2")));
SND_SOC_DAILINK_DEFS!(
    out,
    DAILINK_COMP_ARRAY!(COMP_CPU!(b"omap-mcbsp.2\0")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(b"twl4030-codec\0", b"twl4030-hifi\0")),
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!(b"omap-mcbsp.2\0"))
);

// Original C:
// SND_SOC_DAILINK_DEFS(in,
//     DAILINK_COMP_ARRAY(COMP_CPU("omap-mcbsp.4")),
//     DAILINK_COMP_ARRAY(COMP_CODEC("twl4030-codec", "twl4030-hifi")),
//     DAILINK_COMP_ARRAY(COMP_PLATFORM("omap-mcbsp.4")));
SND_SOC_DAILINK_DEFS!(
    in,
    DAILINK_COMP_ARRAY!(COMP_CPU!(b"omap-mcbsp.4\0")),
    DAILINK_COMP_ARRAY!(COMP_CODEC!(b"twl4030-codec\0", b"twl4030-hifi\0")),
    DAILINK_COMP_ARRAY!(COMP_PLATFORM!(b"omap-mcbsp.4\0"))
);

static mut omap3pandora_dai: [snd_soc_dai_link; 2] = [
    snd_soc_dai_link {
        name: b"PCM1773\0".as_ptr() as *const c_char,
        stream_name: b"HiFi Out\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        ops: &omap3pandora_ops,
        init: Some(omap3pandora_out_init),
        // SND_SOC_DAILINK_REG(out)
    },
    snd_soc_dai_link {
        name: b"TWL4030\0".as_ptr() as *const c_char,
        stream_name: b"Line/Mic In\0".as_ptr() as *const c_char,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        ops: &omap3pandora_ops,
        init: Some(omap3pandora_in_init),
        // SND_SOC_DAILINK_REG(in)
    },
];

/* SoC card */
static mut snd_soc_card_omap3pandora: snd_soc_card = snd_soc_card {
    name: b"omap3pandora\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    dai_link: ptr::null_mut(),
    num_links: 2,
    dapm_widgets: omap3pandora_dapm_widgets.as_ptr(),
    num_dapm_widgets: 7,
    dapm_routes: omap3pandora_map.as_ptr(),
    num_dapm_routes: 10,
};

static mut omap3pandora_snd_device: *mut platform_device = ptr::null_mut();

unsafe extern "C" fn omap3pandora_soc_init() -> c_int {
    let mut ret: c_int;

    if of_machine_is_compatible(b"openpandora,omap3-pandora-600mhz\0".as_ptr() as *const c_char) == 0
        && of_machine_is_compatible(b"openpandora,omap3-pandora-1ghz\0".as_ptr() as *const c_char) == 0
    {
        return -ENODEV;
    }

    pr_info(b"OMAP3 Pandora SoC init\n\0".as_ptr() as *const c_char);

    omap3pandora_snd_device = platform_device_alloc(b"soc-audio\0".as_ptr() as *const c_char, -1);
    if omap3pandora_snd_device.is_null() {
        pr_err(
            b"%s%s\0".as_ptr() as *const c_char,
            PREFIX.as_ptr() as *const c_char,
            b"Platform device allocation failed\n\0".as_ptr() as *const c_char,
        );
        return -ENOMEM;
    }

    snd_soc_card_omap3pandora.owner = THIS_MODULE;
    snd_soc_card_omap3pandora.dai_link = omap3pandora_dai.as_mut_ptr();

    platform_set_drvdata(
        omap3pandora_snd_device,
        &mut snd_soc_card_omap3pandora as *mut snd_soc_card as *mut core::ffi::c_void,
    );

    ret = platform_device_add(omap3pandora_snd_device);
    if ret != 0 {
        pr_err(
            b"%s%s\0".as_ptr() as *const c_char,
            PREFIX.as_ptr() as *const c_char,
            b"Unable to add platform device\n\0".as_ptr() as *const c_char,
        );
        platform_device_put(omap3pandora_snd_device);
        return ret;
    }

    dac_power_gpio = devm_gpiod_get(
        &mut (*omap3pandora_snd_device).dev,
        b"dac\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR(dac_power_gpio as *const core::ffi::c_void) != 0 {
        ret = PTR_ERR(dac_power_gpio as *const core::ffi::c_void) as c_int;
        platform_device_del(omap3pandora_snd_device);
        platform_device_put(omap3pandora_snd_device);
        return ret;
    }

    amp_power_gpio = devm_gpiod_get(
        &mut (*omap3pandora_snd_device).dev,
        b"amp\0".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR(amp_power_gpio as *const core::ffi::c_void) != 0 {
        ret = PTR_ERR(amp_power_gpio as *const core::ffi::c_void) as c_int;
        platform_device_del(omap3pandora_snd_device);
        platform_device_put(omap3pandora_snd_device);
        return ret;
    }

    omap3pandora_dac_reg =
        regulator_get(&mut (*omap3pandora_snd_device).dev, b"vcc\0".as_ptr() as *const c_char);
    if IS_ERR(omap3pandora_dac_reg as *const core::ffi::c_void) != 0 {
        pr_err(
            b"%s%s%ld\n\0".as_ptr() as *const c_char,
            PREFIX.as_ptr() as *const c_char,
            b"Failed to get DAC regulator from \0".as_ptr() as *const c_char,
            dev_name(&mut (*omap3pandora_snd_device).dev),
            PTR_ERR(omap3pandora_dac_reg as *const core::ffi::c_void),
        );
        ret = PTR_ERR(omap3pandora_dac_reg as *const core::ffi::c_void) as c_int;
        platform_device_del(omap3pandora_snd_device);
        platform_device_put(omap3pandora_snd_device);
        return ret;
    }

    0
}

// module_init(omap3pandora_soc_init);

unsafe extern "C" fn omap3pandora_soc_exit() {
    regulator_put(omap3pandora_dac_reg);
    platform_device_unregister(omap3pandora_snd_device);
}

// module_exit(omap3pandora_soc_exit);

// MODULE_AUTHOR("Grazvydas Ignotas <notasas@gmail.com>");
// MODULE_DESCRIPTION("ALSA SoC OMAP3 Pandora");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
