// SPDX-License-Identifier: GPL-2.0-only
/*
 *  cht-bsw-max98090.c - ASoc Machine driver for Intel Cherryview-based
 *  platforms Cherrytrail and Braswell, with max98090 & TI codec.
 *
 *  Copyright (C) 2015 Intel Corp
 *  Author: Fang, Yang A <yang.a.fang@intel.com>
 *  This file is modified from cht_bsw_rt5645.c
 *  ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 *
 * ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

const CHT_PLAT_CLK_3_HZ: u32 = 19_200_000;
const CHT_CODEC_DAI: *const c_char = c"HiFi".as_ptr();

const QUIRK_PMC_PLT_CLK_0: c_int = 0x01;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_jack {
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct snd_soc_card {
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub aux_dev: *mut snd_soc_aux_dev,
    pub num_aux_devs: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_int,
    pub dev: *mut device,
    pub name: *const c_char,
    pub driver_name: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime_data,
}
#[repr(C)]
pub struct snd_pcm_runtime_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct notifier_block {
    pub notifier_call:
        Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}
#[repr(C)]
pub struct snd_soc_jack_gpio {
    pub name: *const c_char,
    pub report: c_int,
    pub debounce_time: c_int,
    pub invert: c_int,
}
#[repr(C)]
pub struct acpi_gpio_params {
    pub crs_entry_index: c_uint,
    pub line_index: c_uint,
    pub active_low: bool,
}
type c_uint = u32;
#[repr(C)]
pub struct acpi_gpio_mapping {
    pub name: *const c_char,
    pub data: *const acpi_gpio_params,
    pub size: c_uint,
}
#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
}
#[repr(C)]
pub struct snd_soc_component {
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_aux_dev {
    pub dlc: snd_soc_dai_link_component,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
}
#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}
#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub nonatomic: bool,
    pub dynamic: c_int,
    pub playback_only: c_int,
    pub ops: *const snd_soc_ops,
    pub id: c_int,
    pub no_pcm: c_int,
    pub dai_fmt: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub be_hw_params_fixup:
        Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime, *mut snd_pcm_hw_params) -> c_int>,
    pub cpus: *const snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub codecs: *const snd_soc_dai_link_component,
    pub num_codecs: c_int,
    pub platforms: *const snd_soc_dai_link_component,
    pub num_platforms: c_int,
}
#[repr(C)]
pub struct dmi_system_id {
    pub matches: [dmi_strmatch; 4],
    pub driver_data: *mut c_void,
}
#[repr(C)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub platform_data: *mut c_void,
    pub driver: *mut device_driver,
}
#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const c_void,
}
#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub mach_params: snd_soc_acpi_mach_params,
}
#[repr(C)]
pub struct snd_soc_acpi_mach_params {
    pub platform: *const c_char,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}
#[repr(C)]
pub struct cht_mc_private {
    pub mclk: *mut clk,
    pub jack: snd_soc_jack,
    pub ts3a227e_present: bool,
    pub quirks: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    fn snd_soc_dapm_to_card(dapm: *mut c_void) -> *mut snd_soc_card;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_get_codec_dai(
        card: *mut snd_soc_card,
        dai_name: *const c_char,
    ) -> *mut snd_soc_dai;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_dapm_force_enable_pin(dapm: *mut c_void, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut c_void, pin: *const c_char) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut c_void) -> c_int;
    fn snd_soc_jack_notifier_register(
        jack: *mut snd_soc_jack,
        nb: *mut notifier_block,
    ) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        ty: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_jack_add_gpiods(
        gpiod_dev: *mut device,
        jack: *mut snd_soc_jack,
        count: c_int,
        gpios: *mut snd_soc_jack_gpio,
    ) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn params_set_format(params: *mut snd_pcm_hw_params, val: c_int);
    fn snd_pcm_hw_constraint_single(
        runtime: *mut snd_pcm_runtime_data,
        var: c_int,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_card_jack_new(
        card: *mut snd_soc_card,
        id: *const c_char,
        ty: c_int,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    fn ts3a227e_enable_jack_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dmi_first_match(ids: *const dmi_system_id) -> *const dmi_system_id;
    fn acpi_dev_found(hid: *const c_char) -> bool;
    fn devm_acpi_dev_add_driver_gpios(
        dev: *mut device,
        gpios: *const acpi_gpio_mapping,
    ) -> c_int;
    fn snd_soc_fixup_dai_links_platform_name(
        card: *mut snd_soc_card,
        platform_name: *const c_char,
    ) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn snd_soc_acpi_sof_parent(dev: *mut device) -> bool;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
}

const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const M98090_REG_SYSTEM_CLOCK: c_int = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_JACK_MICROPHONE: c_int = 0x0004;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_LINEOUT: c_int = 0x0002;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 1;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 2;
const SND_SOC_DAIFMT_BP_FP: c_uint = 4;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 8;
const MERR_DPCM_AUDIO: usize = 0;
const MERR_DPCM_DEEP_BUFFER: usize = 1;
const DMI_PRODUCT_NAME: c_int = 0;

unsafe fn SND_SOC_DAPM_EVENT_ON(event: c_int) -> bool {
    event != 0
}

unsafe extern "C" fn platform_clock_control(
    w: *mut snd_soc_dapm_widget,
    _k: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let card = snd_soc_dapm_to_card((*w.cast::<WidgetWithDapm>()).dapm);
    let ctx = snd_soc_card_get_drvdata(card) as *mut cht_mc_private;
    let ret: c_int;

    /* See the comment in snd_cht_mc_probe() */
    if ((*ctx).quirks & QUIRK_PMC_PLT_CLK_0) != 0 {
        return 0;
    }

    let codec_dai = snd_soc_card_get_codec_dai(card, CHT_CODEC_DAI);
    if codec_dai.is_null() {
        dev_err(
            (*card).dev,
            c"Codec dai not found; Unable to set platform clock\n".as_ptr(),
        );
        return -EIO;
    }

    if SND_SOC_DAPM_EVENT_ON(event) {
        ret = clk_prepare_enable((*ctx).mclk);
        if ret < 0 {
            dev_err((*card).dev, c"could not configure MCLK state".as_ptr());
            return ret;
        }
    } else {
        clk_disable_unprepare((*ctx).mclk);
    }

    0
}

#[repr(C)]
struct WidgetWithDapm {
    dapm: *mut c_void,
}

static CHT_DAPM_WIDGETS: [snd_soc_dapm_widget; 5] = [
    snd_soc_dapm_widget { _private: [] }, /* SND_SOC_DAPM_HP("Headphone", NULL) */
    snd_soc_dapm_widget { _private: [] }, /* SND_SOC_DAPM_MIC("Headset Mic", NULL) */
    snd_soc_dapm_widget { _private: [] }, /* SND_SOC_DAPM_MIC("Int Mic", NULL) */
    snd_soc_dapm_widget { _private: [] }, /* SND_SOC_DAPM_SPK("Ext Spk", NULL) */
    snd_soc_dapm_widget { _private: [] }, /* SND_SOC_DAPM_SUPPLY("Platform Clock", ..., platform_clock_control, PRE_PMU | POST_PMD) */
];

static CHT_AUDIO_MAP: [snd_soc_dapm_route; 17] = [
    snd_soc_dapm_route { sink: c"IN34".as_ptr(), control: ptr::null(), source: c"Headset Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Headset Mic".as_ptr(), control: ptr::null(), source: c"MICBIAS".as_ptr() },
    snd_soc_dapm_route { sink: c"DMICL".as_ptr(), control: ptr::null(), source: c"Int Mic".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPL".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"HPR".as_ptr() },
    snd_soc_dapm_route { sink: c"Ext Spk".as_ptr(), control: ptr::null(), source: c"SPKL".as_ptr() },
    snd_soc_dapm_route { sink: c"Ext Spk".as_ptr(), control: ptr::null(), source: c"SPKR".as_ptr() },
    snd_soc_dapm_route { sink: c"HiFi Playback".as_ptr(), control: ptr::null(), source: c"ssp2 Tx".as_ptr() },
    snd_soc_dapm_route { sink: c"ssp2 Tx".as_ptr(), control: ptr::null(), source: c"codec_out0".as_ptr() },
    snd_soc_dapm_route { sink: c"ssp2 Tx".as_ptr(), control: ptr::null(), source: c"codec_out1".as_ptr() },
    snd_soc_dapm_route { sink: c"codec_in0".as_ptr(), control: ptr::null(), source: c"ssp2 Rx".as_ptr() },
    snd_soc_dapm_route { sink: c"codec_in1".as_ptr(), control: ptr::null(), source: c"ssp2 Rx".as_ptr() },
    snd_soc_dapm_route { sink: c"ssp2 Rx".as_ptr(), control: ptr::null(), source: c"HiFi Capture".as_ptr() },
    snd_soc_dapm_route { sink: c"Headphone".as_ptr(), control: ptr::null(), source: c"Platform Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"Headset Mic".as_ptr(), control: ptr::null(), source: c"Platform Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"Int Mic".as_ptr(), control: ptr::null(), source: c"Platform Clock".as_ptr() },
    snd_soc_dapm_route { sink: c"Ext Spk".as_ptr(), control: ptr::null(), source: c"Platform Clock".as_ptr() },
];

static CHT_MC_CONTROLS: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new { name: c"Headphone".as_ptr() },
    snd_kcontrol_new { name: c"Headset Mic".as_ptr() },
    snd_kcontrol_new { name: c"Int Mic".as_ptr() },
    snd_kcontrol_new { name: c"Ext Spk".as_ptr() },
];

unsafe extern "C" fn cht_aif1_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);

    let ret = snd_soc_dai_set_sysclk(
        codec_dai,
        M98090_REG_SYSTEM_CLOCK,
        CHT_PLAT_CLK_3_HZ,
        SND_SOC_CLOCK_IN,
    );
    if ret < 0 {
        dev_err((*rtd).dev, c"can't set codec sysclk: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

unsafe extern "C" fn cht_ti_jack_event(
    _nb: *mut notifier_block,
    event: c_ulong,
    data: *mut c_void,
) -> c_int {
    let jack = data as *mut snd_soc_jack;
    let dapm = snd_soc_card_to_dapm((*jack).card);

    if (event & SND_JACK_MICROPHONE as c_ulong) != 0 {
        snd_soc_dapm_force_enable_pin(dapm, c"SHDN".as_ptr());
        snd_soc_dapm_force_enable_pin(dapm, c"MICBIAS".as_ptr());
        snd_soc_dapm_sync(dapm);
    } else {
        snd_soc_dapm_disable_pin(dapm, c"MICBIAS".as_ptr());
        snd_soc_dapm_disable_pin(dapm, c"SHDN".as_ptr());
        snd_soc_dapm_sync(dapm);
    }

    0
}

static mut CHT_JACK_NB: notifier_block = notifier_block {
    notifier_call: Some(cht_ti_jack_event),
};

static mut HS_JACK_PINS: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin { pin: c"Headphone".as_ptr(), mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: c"Headset Mic".as_ptr(), mask: SND_JACK_MICROPHONE },
];

static mut HS_JACK_GPIOS: [snd_soc_jack_gpio; 2] = [
    snd_soc_jack_gpio {
        name: c"hp".as_ptr(),
        report: SND_JACK_HEADPHONE | SND_JACK_LINEOUT,
        debounce_time: 200,
        invert: 0,
    },
    snd_soc_jack_gpio {
        name: c"mic".as_ptr(),
        invert: 1,
        report: SND_JACK_MICROPHONE,
        debounce_time: 200,
    },
];

static HP_GPIOS: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 0, line_index: 0, active_low: false };
static MIC_GPIOS: acpi_gpio_params = acpi_gpio_params { crs_entry_index: 1, line_index: 0, active_low: false };

static ACPI_MAX98090_GPIOS: [acpi_gpio_mapping; 3] = [
    acpi_gpio_mapping { name: c"hp-gpios".as_ptr(), data: &HP_GPIOS, size: 1 },
    acpi_gpio_mapping { name: c"mic-gpios".as_ptr(), data: &MIC_GPIOS, size: 1 },
    acpi_gpio_mapping { name: ptr::null(), data: ptr::null(), size: 0 },
];

unsafe extern "C" fn cht_codec_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    let ctx = snd_soc_card_get_drvdata((*runtime).card) as *mut cht_mc_private;
    let jack = &mut (*ctx).jack as *mut snd_soc_jack;

    if (*ctx).ts3a227e_present {
        /*
         * The jack has already been created in the
         * cht_max98090_headset_init() function.
         */
        snd_soc_jack_notifier_register(jack, &mut CHT_JACK_NB);
        return 0;
    }

    let jack_type = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;

    let mut ret = snd_soc_card_jack_new_pins(
        (*runtime).card,
        c"Headset Jack".as_ptr(),
        jack_type,
        jack,
        HS_JACK_PINS.as_mut_ptr(),
        HS_JACK_PINS.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*runtime).dev, c"Headset Jack creation failed %d\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_jack_add_gpiods(
        (*(*(*runtime).card).dev).parent,
        jack,
        HS_JACK_GPIOS.len() as c_int,
        HS_JACK_GPIOS.as_mut_ptr(),
    );
    if ret != 0 {
        /*
         * flag error but don't bail if jack detect is broken
         * due to platform issues or bad BIOS/configuration
         */
        dev_err(
            (*runtime).dev,
            c"jack detection gpios not added, error %d\n".as_ptr(),
            ret,
        );
    }

    /* See the comment in snd_cht_mc_probe() */
    if ((*ctx).quirks & QUIRK_PMC_PLT_CLK_0) != 0 {
        return 0;
    }

    /*
     * The firmware might enable the clock at
     * boot (this information may or may not
     * be reflected in the enable clock register).
     * To change the rate we must disable the clock
     * first to cover these cases. Due to common
     * clock framework restrictions that do not allow
     * to disable a clock that has not been enabled,
     * we need to enable the clock first.
     */
    ret = clk_prepare_enable((*ctx).mclk);
    if ret == 0 {
        clk_disable_unprepare((*ctx).mclk);
    }

    ret = clk_set_rate((*ctx).mclk, CHT_PLAT_CLK_3_HZ);

    if ret != 0 {
        dev_err((*runtime).dev, c"unable to set MCLK rate\n".as_ptr());
    }

    ret
}

unsafe extern "C" fn cht_codec_fixup(
    rtd: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    let channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    let mut ret: c_int = 0;
    let fmt: c_uint;

    ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_cpu(rtd, 0), 0x3, 0x3, 2, 16);
    if ret < 0 {
        dev_err((*rtd).dev, c"can't set cpu_dai slot fmt: %d\n".as_ptr(), ret);
        return ret;
    }

    fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_BP_FP;

    ret = snd_soc_dai_set_fmt(snd_soc_rtd_to_cpu(rtd, 0), fmt);
    if ret < 0 {
        dev_err((*rtd).dev, c"can't set cpu_dai set fmt: %d\n".as_ptr(), ret);
        return ret;
    }

    /* The DSP will convert the FE rate to 48k, stereo, 24bits */
    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;

    /* set SSP2 to 16-bit */
    params_set_format(params, SNDRV_PCM_FORMAT_S16_LE);
    0
}

unsafe extern "C" fn cht_aif1_startup(substream: *mut snd_pcm_substream) -> c_int {
    snd_pcm_hw_constraint_single((*substream).runtime, SNDRV_PCM_HW_PARAM_RATE, 48000)
}

unsafe extern "C" fn cht_max98090_headset_init(component: *mut snd_soc_component) -> c_int {
    let card = (*component).card;
    let ctx = snd_soc_card_get_drvdata(card) as *mut cht_mc_private;
    let jack = &mut (*ctx).jack as *mut snd_soc_jack;

    /*
     * TI supports 4 buttons headset detection
     * KEY_MEDIA
     * KEY_VOICECOMMAND
     * KEY_VOLUMEUP
     * KEY_VOLUMEDOWN
     */
    let jack_type = SND_JACK_HEADPHONE
        | SND_JACK_MICROPHONE
        | SND_JACK_BTN_0
        | SND_JACK_BTN_1
        | SND_JACK_BTN_2
        | SND_JACK_BTN_3;

    let ret = snd_soc_card_jack_new(card, c"Headset Jack".as_ptr(), jack_type, jack);
    if ret != 0 {
        dev_err((*card).dev, c"Headset Jack creation failed %d\n".as_ptr(), ret);
        return ret;
    }

    ts3a227e_enable_jack_detect(component, jack)
}

static CHT_AIF1_OPS: snd_soc_ops = snd_soc_ops {
    startup: Some(cht_aif1_startup),
    hw_params: None,
};

static CHT_BE_SSP2_OPS: snd_soc_ops = snd_soc_ops {
    startup: None,
    hw_params: Some(cht_aif1_hw_params),
};

static mut CHT_MAX98090_HEADSET_DEV: snd_soc_aux_dev = snd_soc_aux_dev {
    dlc: snd_soc_dai_link_component { name: c"i2c-104C227E:00".as_ptr(), dai_name: ptr::null() },
    init: Some(cht_max98090_headset_init),
};

static DUMMY: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { name: c"dummy".as_ptr(), dai_name: ptr::null() }];
static MEDIA: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { name: c"media-cpu-dai".as_ptr(), dai_name: ptr::null() }];
static DEEPBUFFER: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { name: c"deepbuffer-cpu-dai".as_ptr(), dai_name: ptr::null() }];
static SSP2_PORT: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { name: c"ssp2-port".as_ptr(), dai_name: ptr::null() }];
static SSP2_CODEC: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { name: c"i2c-193C9890:00".as_ptr(), dai_name: c"HiFi".as_ptr() }];
static PLATFORM: [snd_soc_dai_link_component; 1] =
    [snd_soc_dai_link_component { name: c"sst-mfld-platform".as_ptr(), dai_name: ptr::null() }];

static mut CHT_DAILINK: [snd_soc_dai_link; 3] = [
    snd_soc_dai_link {
        name: c"Audio Port".as_ptr(),
        stream_name: c"Audio".as_ptr(),
        nonatomic: true,
        dynamic: 1,
        playback_only: 0,
        ops: &CHT_AIF1_OPS,
        id: 0,
        no_pcm: 0,
        dai_fmt: 0,
        init: None,
        be_hw_params_fixup: None,
        cpus: MEDIA.as_ptr(),
        num_cpus: 1,
        codecs: DUMMY.as_ptr(),
        num_codecs: 1,
        platforms: PLATFORM.as_ptr(),
        num_platforms: 1,
    },
    snd_soc_dai_link {
        name: c"Deep-Buffer Audio Port".as_ptr(),
        stream_name: c"Deep-Buffer Audio".as_ptr(),
        nonatomic: true,
        dynamic: 1,
        playback_only: 1,
        ops: &CHT_AIF1_OPS,
        id: 0,
        no_pcm: 0,
        dai_fmt: 0,
        init: None,
        be_hw_params_fixup: None,
        cpus: DEEPBUFFER.as_ptr(),
        num_cpus: 1,
        codecs: DUMMY.as_ptr(),
        num_codecs: 1,
        platforms: PLATFORM.as_ptr(),
        num_platforms: 1,
    },
    /* back ends */
    snd_soc_dai_link {
        name: c"SSP2-Codec".as_ptr(),
        stream_name: ptr::null(),
        nonatomic: false,
        dynamic: 0,
        playback_only: 0,
        ops: &CHT_BE_SSP2_OPS,
        id: 0,
        no_pcm: 1,
        dai_fmt: SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC,
        init: Some(cht_codec_init),
        be_hw_params_fixup: Some(cht_codec_fixup),
        cpus: SSP2_PORT.as_ptr(),
        num_cpus: 1,
        codecs: SSP2_CODEC.as_ptr(),
        num_codecs: 1,
        platforms: PLATFORM.as_ptr(),
        num_platforms: 1,
    },
];

/* use space before codec name to simplify card ID, and simplify driver name */
const SOF_CARD_NAME: *const c_char = c"bytcht max98090".as_ptr(); /* card name will be 'sof-bytcht max98090 */
const SOF_DRIVER_NAME: *const c_char = c"SOF".as_ptr();

const CARD_NAME: *const c_char = c"chtmax98090".as_ptr();
const DRIVER_NAME: *const c_char = ptr::null(); /* card name will be used for driver name */

/* SoC card */
static mut SND_SOC_CARD_CHT: snd_soc_card = snd_soc_card {
    owner: ptr::null_mut(),
    dai_link: ptr::null_mut(),
    num_links: 3,
    aux_dev: ptr::null_mut(),
    num_aux_devs: 1,
    dapm_widgets: CHT_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: 5,
    dapm_routes: CHT_AUDIO_MAP.as_ptr(),
    num_dapm_routes: 17,
    controls: CHT_MC_CONTROLS.as_ptr(),
    num_controls: 4,
    dev: ptr::null_mut(),
    name: ptr::null(),
    driver_name: ptr::null(),
};

const fn dmi_match(product: &'static core::ffi::CStr) -> dmi_strmatch {
    dmi_strmatch { slot: DMI_PRODUCT_NAME, substr: product.as_ptr() }
}

static CHT_MAX98090_QUIRK_TABLE: [dmi_system_id; 18] = [
    dmi_system_id { matches: [dmi_match(c"Banjo"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Candy"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Clapper"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Cyan"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Enguarde"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Glimmer"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Gnawty"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Heli"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Kip"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Ninja"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Orco"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Quawks"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Rambi"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Squawks"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Sumo"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Swanky"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_match(c"Winky"), dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: QUIRK_PMC_PLT_CLK_0 as usize as *mut c_void },
    dmi_system_id { matches: [dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }, dmi_strmatch { slot: 0, substr: ptr::null() }], driver_data: ptr::null_mut() },
];

unsafe extern "C" fn snd_cht_mc_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let mut ret_val: c_int = 0;
    let drv = devm_kzalloc(dev, core::mem::size_of::<cht_mc_private>(), GFP_KERNEL)
        as *mut cht_mc_private;
    if drv.is_null() {
        return -ENOMEM;
    }

    let dmi_id = dmi_first_match(CHT_MAX98090_QUIRK_TABLE.as_ptr());
    if !dmi_id.is_null() {
        (*drv).quirks = (*dmi_id).driver_data as c_ulong as c_int;
    }

    (*drv).ts3a227e_present = acpi_dev_found(c"104C227E".as_ptr());
    if !(*drv).ts3a227e_present {
        /* no need probe TI jack detection chip */
        SND_SOC_CARD_CHT.aux_dev = ptr::null_mut();
        SND_SOC_CARD_CHT.num_aux_devs = 0;

        ret_val = devm_acpi_dev_add_driver_gpios((*dev).parent, ACPI_MAX98090_GPIOS.as_ptr());
        if ret_val != 0 {
            dev_dbg(dev, c"Unable to add GPIO mapping table\n".as_ptr());
        }
    }

    /* override platform name, if required */
    SND_SOC_CARD_CHT.dev = dev;
    let mach = (*dev).platform_data as *mut snd_soc_acpi_mach;
    let platform_name = (*mach).mach_params.platform;

    ret_val = snd_soc_fixup_dai_links_platform_name(&mut SND_SOC_CARD_CHT, platform_name);
    if ret_val != 0 {
        return ret_val;
    }

    /* register the soc card */
    snd_soc_card_set_drvdata(&mut SND_SOC_CARD_CHT, drv as *mut c_void);

    let mclk_name = if ((*drv).quirks & QUIRK_PMC_PLT_CLK_0) != 0 {
        c"pmc_plt_clk_0".as_ptr()
    } else {
        c"pmc_plt_clk_3".as_ptr()
    };

    (*drv).mclk = devm_clk_get(dev, mclk_name);
    if IS_ERR((*drv).mclk as *const c_void) {
        dev_err(
            dev,
            c"Failed to get MCLK from %s: %ld\n".as_ptr(),
            mclk_name,
            PTR_ERR((*drv).mclk as *const c_void),
        );
        return PTR_ERR((*drv).mclk as *const c_void);
    }

    /*
     * Boards which have the MAX98090's clk connected to clk_0 do not seem
     * to like it if we muck with the clock. If we disable the clock when
     * it is unused we get "max98090 i2c-193C9890:00: PLL unlocked" errors
     * and the PLL never seems to lock again.
     * So for these boards we enable it here once and leave it at that.
     */
    if ((*drv).quirks & QUIRK_PMC_PLT_CLK_0) != 0 {
        ret_val = clk_prepare_enable((*drv).mclk);
        if ret_val < 0 {
            dev_err(dev, c"MCLK enable error: %d\n".as_ptr(), ret_val);
            return ret_val;
        }
    }

    let sof_parent = snd_soc_acpi_sof_parent(dev);

    /* set card and driver name */
    if sof_parent {
        SND_SOC_CARD_CHT.name = SOF_CARD_NAME;
        SND_SOC_CARD_CHT.driver_name = SOF_DRIVER_NAME;
    } else {
        SND_SOC_CARD_CHT.name = CARD_NAME;
        SND_SOC_CARD_CHT.driver_name = DRIVER_NAME;
    }

    /* set pm ops */
    if sof_parent {
        (*(*dev).driver).pm = &snd_soc_pm_ops as *const c_void;
    }

    SND_SOC_CARD_CHT.owner = THIS_MODULE;
    SND_SOC_CARD_CHT.dai_link = CHT_DAILINK.as_mut_ptr();
    SND_SOC_CARD_CHT.aux_dev = &mut CHT_MAX98090_HEADSET_DEV;

    ret_val = devm_snd_soc_register_card(dev, &mut SND_SOC_CARD_CHT);
    if ret_val != 0 {
        dev_err(dev, c"snd_soc_register_card failed %d\n".as_ptr(), ret_val);
        return ret_val;
    }
    platform_set_drvdata(pdev, &mut SND_SOC_CARD_CHT as *mut snd_soc_card as *mut c_void);
    ret_val
}

unsafe extern "C" fn snd_cht_mc_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    let ctx = snd_soc_card_get_drvdata(card) as *mut cht_mc_private;

    if ((*ctx).quirks & QUIRK_PMC_PLT_CLK_0) != 0 {
        clk_disable_unprepare((*ctx).mclk);
    }
}

static mut SND_CHT_MC_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: c"cht-bsw-max98090".as_ptr(),
        pm: ptr::null(),
    },
    probe: Some(snd_cht_mc_probe),
    remove: Some(snd_cht_mc_remove),
};

/* module_platform_driver(snd_cht_mc_driver) */

/* MODULE_DESCRIPTION("ASoC Intel(R) Braswell Machine driver"); */
/* MODULE_AUTHOR("Fang, Yang A <yang.a.fang@intel.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:cht-bsw-max98090"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
