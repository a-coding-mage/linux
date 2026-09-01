// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2019-2020 Intel Corporation.

/*
 * Intel SOF Machine Driver with Realtek rt5682 Codec
 * and speaker codec MAX98357A or RT1015.
 */

/* Dependencies from the original C includes:
 * linux/i2c.h, linux/input.h, linux/module.h, linux/platform_device.h,
 * linux/clk.h, linux/dmi.h, sound/core.h, sound/jack.h, sound/pcm.h,
 * sound/pcm_params.h, sound/soc.h, sound/sof.h, sound/rt5682.h,
 * sound/rt5682s.h, sound/soc-acpi.h, codec and board helper headers.
 */

type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type kernel_ulong_t = usize;
type bool_t = bool;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_CLOCK_IN: c_int = 0;

const fn BIT(n: c_uint) -> c_ulong {
    1usize << n
}

/* Driver-specific board quirks: from bit 0 to 7 */
const SOF_RT5682_MCLK_EN: c_ulong = BIT(0);

extern "C" {
    static THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn SOF_SSP_PORT_CODEC(port: c_int) -> c_ulong;
    fn SOF_SSP_PORT_AMP(port: c_int) -> c_ulong;
    fn SOF_NUM_IDISP_HDMI(num: c_int) -> c_ulong;
    fn SOF_SSP_PORT_BT_OFFLOAD(port: c_int) -> c_ulong;
    static SOF_BT_OFFLOAD_PRESENT: c_ulong;
    fn SOF_SSP_MASK_HDMI_CAPTURE(mask: c_int) -> c_ulong;
    fn SOF_LINK_ORDER(a: c_int, b: c_int, c: c_int, d: c_int, e: c_int, f: c_int, g: c_int) -> c_ulong;

    static SOF_LINK_AMP: c_int;
    static SOF_LINK_CODEC: c_int;
    static SOF_LINK_DMIC01: c_int;
    static SOF_LINK_IDISP_HDMI: c_int;
    static SOF_LINK_NONE: c_int;

    static CODEC_NONE: c_int;
    static CODEC_MAX98357A: c_int;
    static CODEC_MAX98360A: c_int;
    static CODEC_MAX98373: c_int;
    static CODEC_MAX98390: c_int;
    static CODEC_RT1011: c_int;
    static CODEC_RT1015: c_int;
    static CODEC_RT1015P: c_int;
    static CODEC_RT1019P: c_int;
    static CODEC_RT5650: c_int;
    static CODEC_RT5682: c_int;
    static CODEC_RT5682S: c_int;
    static CODEC_TAS2563: c_int;

    static RT5645_DA_STEREO_FILTER: c_int;
    static RT5645_AD_STEREO_FILTER: c_int;
    static RT5645_DA_MONO_L_FILTER: c_int;
    static RT5645_DA_MONO_R_FILTER: c_int;
    static RT5645_CLK_SEL_I2S1_ASRC: c_int;
    static RT5645_CLK_SEL_I2S2_ASRC: c_int;
    static RT5645_PLL1_S_MCLK: c_int;
    static RT5645_PLL1_S_BCLK1: c_int;
    static RT5645_SCLK_S_MCLK: c_int;
    static RT5645_SCLK_S_PLL1: c_int;

    static RT5682_DA_STEREO1_FILTER: c_int;
    static RT5682_AD_STEREO1_FILTER: c_int;
    static RT5682_CLK_SEL_I2S1_ASRC: c_int;
    static RT5682_PLL1_S_MCLK: c_int;
    static RT5682_PLL1_S_BCLK1: c_int;
    static RT5682_SCLK_S_MCLK: c_int;
    static RT5682_PLL1: c_int;
    static RT5682_SCLK_S_PLL1: c_int;

    static RT5682S_DA_STEREO1_FILTER: c_int;
    static RT5682S_AD_STEREO1_FILTER: c_int;
    static RT5682S_CLK_SEL_I2S1_ASRC: c_int;
    static RT5682S_PLL_S_MCLK: c_int;
    static RT5682S_PLL_S_BCLK1: c_int;
    static RT5682S_SCLK_S_MCLK: c_int;
    static RT5682S_PLL1: c_int;
    static RT5682S_PLL2: c_int;
    static RT5682S_SCLK_S_PLL1: c_int;
    static RT5682S_SCLK_S_PLL2: c_int;

    static SND_JACK_HEADPHONE: c_int;
    static SND_JACK_MICROPHONE: c_int;
    static SND_JACK_HEADSET: c_int;
    static SND_JACK_BTN_0: c_int;
    static SND_JACK_BTN_1: c_int;
    static SND_JACK_BTN_2: c_int;
    static SND_JACK_BTN_3: c_int;
    static KEY_PLAYPAUSE: c_int;
    static KEY_VOICECOMMAND: c_int;
    static KEY_VOLUMEUP: c_int;
    static KEY_VOLUMEDOWN: c_int;
    static IDISP_CODEC_MASK: c_uint;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut sof_card_private;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut sof_card_private);
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn sof_dai_get_mclk(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn sof_dai_get_bclk(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;

    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_info(dev: *mut device, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dmi_check_system(table: *const dmi_system_id) -> c_int;
    fn devm_kstrdup(dev: *mut device, s: *const u8, gfp: c_uint) -> *mut i8;
    fn devm_clk_get(dev: *mut device, id: *const u8) -> *mut clk;
    fn IS_ERR(ptr: *const clk) -> bool_t;
    fn PTR_ERR(ptr: *const clk) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;

    fn rt5645_sel_asrc_clk_src(component: *mut snd_soc_component, filter: c_int, src: c_int);
    fn rt5682_sel_asrc_clk_src(component: *mut snd_soc_component, filter: c_int, src: c_int);
    fn rt5682s_sel_asrc_clk_src(component: *mut snd_soc_component, filter: c_int, src: c_int);
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const u8,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_int,
    ) -> c_int;
    fn snd_soc_dai_set_pll(dai: *mut snd_soc_dai, pll_id: c_int, source: c_int, freq_in: c_int, freq_out: c_int) -> c_int;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_int, dir: c_int) -> c_int;
    fn snd_soc_dai_set_tdm_slot(dai: *mut snd_soc_dai, tx_mask: c_uint, rx_mask: c_uint, slots: c_int, slot_width: c_uint) -> c_int;
    fn snd_soc_dapm_disable_pin(dapm: *mut snd_soc_dapm_context, pin: *const u8) -> c_int;
    fn snd_soc_dapm_sync(dapm: *mut snd_soc_dapm_context) -> c_int;
    fn sof_intel_board_card_late_probe(card: *mut snd_soc_card) -> c_int;
    fn snd_soc_dapm_new_controls(dapm: *mut snd_soc_dapm_context, widget: *const snd_soc_dapm_widget, num: c_int) -> c_int;
    fn snd_soc_add_card_controls(card: *mut snd_soc_card, controls: *const snd_kcontrol_new, num: c_int) -> c_int;
    fn snd_soc_dapm_add_routes(dapm: *mut snd_soc_dapm_context, route: *const snd_soc_dapm_route, num: c_int) -> c_int;
    fn sof_intel_board_set_dai_link(dev: *mut device, card: *mut snd_soc_card, ctx: *mut sof_card_private) -> c_int;
    fn max_98357a_dai_link(link: *mut snd_soc_dai_link);
    fn max_98360a_dai_link(link: *mut snd_soc_dai_link);
    fn max_98373_dai_link(dev: *mut device, link: *mut snd_soc_dai_link);
    fn max_98390_dai_link(dev: *mut device, link: *mut snd_soc_dai_link);
    fn sof_rt1011_dai_link(dev: *mut device, link: *mut snd_soc_dai_link);
    fn sof_rt1015_dai_link(link: *mut snd_soc_dai_link);
    fn sof_rt1015p_dai_link(link: *mut snd_soc_dai_link);
    fn sof_rt1019p_dai_link(link: *mut snd_soc_dai_link);
    fn sof_tas2563_dai_link(link: *mut snd_soc_dai_link);
    fn sof_intel_board_get_ctx(dev: *mut device, quirk: c_ulong) -> *mut sof_card_private;
    fn soc_intel_is_byt() -> bool_t;
    fn soc_intel_is_cht() -> bool_t;
    fn soc_intel_is_glk() -> bool_t;
    fn soc_intel_is_cml() -> bool_t;
    fn max_98373_set_codec_conf(card: *mut snd_soc_card);
    fn max_98390_set_codec_conf(dev: *mut device, card: *mut snd_soc_card);
    fn sof_rt1011_codec_conf(dev: *mut device, card: *mut snd_soc_card);
    fn sof_rt1015_codec_conf(card: *mut snd_soc_card);
    fn sof_rt1015p_codec_conf(card: *mut snd_soc_card);
    fn snd_soc_fixup_dai_links_platform_name(card: *mut snd_soc_card, platform: *const i8) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

#[repr(C)]
struct module;
#[repr(C)]
struct dev_pm_ops;
#[repr(C)]
struct clk;
#[repr(C)]
struct snd_jack;
#[repr(C)]
struct snd_soc_component;
#[repr(C)]
struct snd_soc_dapm_context;
#[repr(C)]
struct snd_pcm_substream;
#[repr(C)]
struct snd_pcm_hw_params;

#[repr(C)]
struct device {
    platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
struct platform_device {
    dev: device,
    id_entry: *const platform_device_id,
}

#[repr(C)]
struct platform_device_id {
    name: [u8; 32],
    driver_data: kernel_ulong_t,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: device_driver,
    id_table: *const platform_device_id,
}

#[repr(C)]
struct device_driver {
    name: *const u8,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct snd_soc_acpi_mach {
    mach_params: snd_soc_acpi_mach_params,
}

#[repr(C)]
struct snd_soc_acpi_mach_params {
    codec_mask: c_uint,
    platform: *const i8,
}

#[repr(C)]
struct snd_soc_card {
    name: *mut i8,
    owner: *mut module,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    fully_routed: bool_t,
    late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    card: *mut snd_soc_card,
    dev: *mut device,
}

#[repr(C)]
struct snd_soc_dai {
    component: *mut snd_soc_component,
}

#[repr(C)]
struct snd_soc_jack {
    jack: *mut snd_jack,
}

#[repr(C)]
struct snd_soc_jack_pin {
    pin: *const u8,
    mask: c_int,
}

#[repr(C)]
struct snd_soc_ops {
    hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
struct snd_kcontrol_new {
    name: *const u8,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    name: *const u8,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const u8,
    control: *const u8,
    source: *const u8,
}

#[repr(C)]
struct snd_soc_dai_link_component {
    name: *const u8,
    dai_name: *const u8,
}

#[repr(C)]
struct snd_soc_dai_link {
    codecs: *mut snd_soc_dai_link_component,
    num_codecs: c_uint,
    init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    exit: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime)>,
    ops: *const snd_soc_ops,
    ignore_pmdown_time: c_int,
}

#[repr(C)]
struct sof_rt5682_private {
    mclk_en: bool_t,
    is_legacy_cpu: bool_t,
    mclk: *mut clk,
}

#[repr(C)]
struct sof_hdmi_private {
    idisp_codec: bool_t,
}

#[repr(C)]
struct sof_card_private {
    headset_jack: snd_soc_jack,
    rt5682: sof_rt5682_private,
    codec_type: c_int,
    amp_type: c_int,
    codec_link: *mut snd_soc_dai_link,
    amp_link: *mut snd_soc_dai_link,
    hdmi: sof_hdmi_private,
    dmic_be_num: c_int,
    hdmi_num: c_int,
    link_order_overwrite: c_ulong,
}

#[repr(C)]
struct dmi_system_id {
    callback: Option<unsafe extern "C" fn(*const dmi_system_id) -> c_int>,
    matches: [dmi_strmatch; 4],
    driver_data: *mut core::ffi::c_void,
}

#[repr(C)]
struct dmi_strmatch {
    slot: c_int,
    substr: *const u8,
}

const DMI_SYS_VENDOR: c_int = 0;
const DMI_PRODUCT_NAME: c_int = 1;
const DMI_PRODUCT_FAMILY: c_int = 2;
const DMI_OEM_STRING: c_int = 3;

const fn DMI_MATCH(slot: c_int, substr: *const u8) -> dmi_strmatch {
    dmi_strmatch { slot, substr }
}

/* Default: MCLK on, MCLK 19.2M, SSP0 */
static mut sof_rt5682_quirk: c_ulong = SOF_RT5682_MCLK_EN | unsafe { SOF_SSP_PORT_CODEC(0) };

static mut quirk_override: c_int = -1;
/* module_param_named(quirk, quirk_override, int, 0444);
 * MODULE_PARM_DESC(quirk, "Board-specific quirk override");
 */

unsafe extern "C" fn sof_rt5682_quirk_cb(id: *const dmi_system_id) -> c_int {
    sof_rt5682_quirk = (*id).driver_data as c_ulong;
    1
}

static sof_rt5682_quirk_table: [dmi_system_id; 8] = unsafe {
    [
        dmi_system_id {
            callback: Some(sof_rt5682_quirk_cb),
            matches: [
                DMI_MATCH(DMI_SYS_VENDOR, b"Circuitco\0".as_ptr()),
                DMI_MATCH(DMI_PRODUCT_NAME, b"Minnowboard Max\0".as_ptr()),
                DMI_MATCH(0, core::ptr::null()),
                DMI_MATCH(0, core::ptr::null()),
            ],
            driver_data: SOF_SSP_PORT_CODEC(2) as *mut core::ffi::c_void,
        },
        dmi_system_id {
            callback: Some(sof_rt5682_quirk_cb),
            matches: [
                DMI_MATCH(DMI_SYS_VENDOR, b"AAEON\0".as_ptr()),
                DMI_MATCH(DMI_PRODUCT_NAME, b"UP-CHT01\0".as_ptr()),
                DMI_MATCH(0, core::ptr::null()),
                DMI_MATCH(0, core::ptr::null()),
            ],
            driver_data: SOF_SSP_PORT_CODEC(2) as *mut core::ffi::c_void,
        },
        dmi_system_id {
            callback: Some(sof_rt5682_quirk_cb),
            matches: [
                DMI_MATCH(DMI_SYS_VENDOR, b"Intel Corporation\0".as_ptr()),
                DMI_MATCH(DMI_PRODUCT_NAME, b"WhiskeyLake Client\0".as_ptr()),
                DMI_MATCH(0, core::ptr::null()),
                DMI_MATCH(0, core::ptr::null()),
            ],
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(1)) as *mut core::ffi::c_void,
        },
        dmi_system_id {
            callback: Some(sof_rt5682_quirk_cb),
            matches: [
                DMI_MATCH(DMI_PRODUCT_FAMILY, b"Google_Volteer\0".as_ptr()),
                DMI_MATCH(DMI_OEM_STRING, b"AUDIO-MAX98373_ALC5682I_I2S_UP4\0".as_ptr()),
                DMI_MATCH(0, core::ptr::null()),
                DMI_MATCH(0, core::ptr::null()),
            ],
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(2) | SOF_NUM_IDISP_HDMI(4)) as *mut core::ffi::c_void,
        },
        dmi_system_id {
            callback: Some(sof_rt5682_quirk_cb),
            matches: [
                DMI_MATCH(DMI_SYS_VENDOR, b"Intel Corporation\0".as_ptr()),
                DMI_MATCH(DMI_PRODUCT_NAME, b"Alder Lake Client Platform\0".as_ptr()),
                DMI_MATCH(DMI_OEM_STRING, b"AUDIO-ADL_MAX98373_ALC5682I_I2S\0".as_ptr()),
                DMI_MATCH(0, core::ptr::null()),
            ],
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(2) | SOF_NUM_IDISP_HDMI(4)) as *mut core::ffi::c_void,
        },
        dmi_system_id {
            callback: Some(sof_rt5682_quirk_cb),
            matches: [
                DMI_MATCH(DMI_PRODUCT_FAMILY, b"Google_Brya\0".as_ptr()),
                DMI_MATCH(DMI_OEM_STRING, b"AUDIO-MAX98390_ALC5682I_I2S\0".as_ptr()),
                DMI_MATCH(0, core::ptr::null()),
                DMI_MATCH(0, core::ptr::null()),
            ],
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(2) | SOF_NUM_IDISP_HDMI(4)) as *mut core::ffi::c_void,
        },
        dmi_system_id {
            callback: Some(sof_rt5682_quirk_cb),
            matches: [
                DMI_MATCH(DMI_PRODUCT_FAMILY, b"Google_Brya\0".as_ptr()),
                DMI_MATCH(DMI_OEM_STRING, b"AUDIO-MAX98360_ALC5682I_I2S_AMP_SSP2\0".as_ptr()),
                DMI_MATCH(0, core::ptr::null()),
                DMI_MATCH(0, core::ptr::null()),
            ],
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(2) | SOF_NUM_IDISP_HDMI(4)) as *mut core::ffi::c_void,
        },
        dmi_system_id {
            callback: Some(sof_rt5682_quirk_cb),
            matches: [
                DMI_MATCH(DMI_PRODUCT_FAMILY, b"Google_Rex\0".as_ptr()),
                DMI_MATCH(0, core::ptr::null()),
                DMI_MATCH(0, core::ptr::null()),
                DMI_MATCH(0, core::ptr::null()),
            ],
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(2) | SOF_SSP_PORT_AMP(0) | SOF_SSP_PORT_BT_OFFLOAD(1) | SOF_BT_OFFLOAD_PRESENT) as *mut core::ffi::c_void,
        },
    ]
};

static mut jack_pins: [snd_soc_jack_pin; 2] = unsafe {
    [
        snd_soc_jack_pin { pin: b"Headphone Jack\0".as_ptr(), mask: SND_JACK_HEADPHONE },
        snd_soc_jack_pin { pin: b"Headset Mic\0".as_ptr(), mask: SND_JACK_MICROPHONE },
    ]
};

unsafe extern "C" fn sof_rt5682_codec_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let ctx = snd_soc_card_get_drvdata((*rtd).card);
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let jack = &mut (*ctx).headset_jack as *mut snd_soc_jack;
    let mut extra_jack_data: c_int;
    let mut ret: c_int;
    let mclk_freq: c_int;

    if (*ctx).rt5682.mclk_en {
        mclk_freq = sof_dai_get_mclk(rtd);
        if mclk_freq <= 0 {
            dev_err((*rtd).dev, b"invalid mclk freq %d\n\0".as_ptr(), mclk_freq);
            return -EINVAL;
        }

        /* need to enable ASRC function for 24MHz mclk rate */
        if mclk_freq == 24000000 {
            dev_info((*rtd).dev, b"enable ASRC\n\0".as_ptr());

            if (*ctx).codec_type == CODEC_RT5650 {
                rt5645_sel_asrc_clk_src(component, RT5645_DA_STEREO_FILTER | RT5645_AD_STEREO_FILTER, RT5645_CLK_SEL_I2S1_ASRC);
                rt5645_sel_asrc_clk_src(component, RT5645_DA_MONO_L_FILTER | RT5645_DA_MONO_R_FILTER, RT5645_CLK_SEL_I2S2_ASRC);
            } else if (*ctx).codec_type == CODEC_RT5682 {
                rt5682_sel_asrc_clk_src(component, RT5682_DA_STEREO1_FILTER | RT5682_AD_STEREO1_FILTER, RT5682_CLK_SEL_I2S1_ASRC);
            } else if (*ctx).codec_type == CODEC_RT5682S {
                rt5682s_sel_asrc_clk_src(component, RT5682S_DA_STEREO1_FILTER | RT5682S_AD_STEREO1_FILTER, RT5682S_CLK_SEL_I2S1_ASRC);
            } else {
                dev_err((*rtd).dev, b"invalid codec type %d\n\0".as_ptr(), (*ctx).codec_type);
                return -EINVAL;
            }
        }

        if (*ctx).rt5682.is_legacy_cpu {
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
            ret = clk_prepare_enable((*ctx).rt5682.mclk);
            if ret == 0 {
                clk_disable_unprepare((*ctx).rt5682.mclk);
            }

            ret = clk_set_rate((*ctx).rt5682.mclk, 19200000);

            if ret != 0 {
                dev_err((*rtd).dev, b"unable to set MCLK rate\n\0".as_ptr());
            }
        }
    }

    /*
     * Headset buttons map to the google Reference headset.
     * These can be configured by userspace.
     */
    ret = snd_soc_card_jack_new_pins(
        (*rtd).card,
        b"Headset Jack\0".as_ptr(),
        SND_JACK_HEADSET | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        jack,
        jack_pins.as_mut_ptr(),
        jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*rtd).dev, b"Headset Jack creation failed: %d\n\0".as_ptr(), ret);
        return ret;
    }

    snd_jack_set_key((*jack).jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key((*jack).jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);

    if (*ctx).codec_type == CODEC_RT5650 {
        extra_jack_data = SND_JACK_MICROPHONE | SND_JACK_BTN_0;
        ret = snd_soc_component_set_jack(component, jack, &mut extra_jack_data);
    } else {
        ret = snd_soc_component_set_jack(component, jack, core::ptr::null_mut());
    }

    if ret != 0 {
        dev_err((*rtd).dev, b"Headset Jack call-back failed: %d\n\0".as_ptr(), ret);
        return ret;
    }

    ret
}

unsafe extern "C" fn sof_rt5682_codec_exit(rtd: *mut snd_soc_pcm_runtime) {
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;

    snd_soc_component_set_jack(component, core::ptr::null_mut(), core::ptr::null_mut());
}

unsafe extern "C" fn sof_rt5682_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let ctx = snd_soc_card_get_drvdata((*rtd).card);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut pll_id: c_int = 0;
    let pll_source: c_int;
    let pll_in: c_int;
    let pll_out: c_int;
    let clk_id: c_int;
    let mut ret: c_int;

    if (*ctx).rt5682.mclk_en {
        if (*ctx).rt5682.is_legacy_cpu {
            ret = clk_prepare_enable((*ctx).rt5682.mclk);
            if ret < 0 {
                dev_err((*rtd).dev, b"could not configure MCLK state\0".as_ptr());
                return ret;
            }
        }

        if (*ctx).codec_type == CODEC_RT5650 {
            pll_source = RT5645_PLL1_S_MCLK;
        } else if (*ctx).codec_type == CODEC_RT5682 {
            pll_source = RT5682_PLL1_S_MCLK;
        } else if (*ctx).codec_type == CODEC_RT5682S {
            pll_source = RT5682S_PLL_S_MCLK;
        } else {
            dev_err((*rtd).dev, b"invalid codec type %d\n\0".as_ptr(), (*ctx).codec_type);
            return -EINVAL;
        }

        /* get the tplg configured mclk. */
        pll_in = sof_dai_get_mclk(rtd);
        if pll_in <= 0 {
            dev_err((*rtd).dev, b"invalid mclk freq %d\n\0".as_ptr(), pll_in);
            return -EINVAL;
        }
    } else {
        if (*ctx).codec_type == CODEC_RT5650 {
            pll_source = RT5645_PLL1_S_BCLK1;
        } else if (*ctx).codec_type == CODEC_RT5682 {
            pll_source = RT5682_PLL1_S_BCLK1;
        } else if (*ctx).codec_type == CODEC_RT5682S {
            pll_source = RT5682S_PLL_S_BCLK1;
        } else {
            dev_err((*rtd).dev, b"invalid codec type %d\n\0".as_ptr(), (*ctx).codec_type);
            return -EINVAL;
        }

        /* get the tplg configured bclk. */
        pll_in = sof_dai_get_bclk(rtd);
        if pll_in <= 0 {
            dev_err((*rtd).dev, b"invalid bclk freq %d\n\0".as_ptr(), pll_in);
            return -EINVAL;
        }
    }

    pll_out = params_rate(params) * 512;

    /* when MCLK is 512FS, no need to set PLL configuration additionally. */
    if pll_in == pll_out {
        if (*ctx).codec_type == CODEC_RT5650 {
            clk_id = RT5645_SCLK_S_MCLK;
        } else if (*ctx).codec_type == CODEC_RT5682 {
            clk_id = RT5682_SCLK_S_MCLK;
        } else if (*ctx).codec_type == CODEC_RT5682S {
            clk_id = RT5682S_SCLK_S_MCLK;
        } else {
            dev_err((*rtd).dev, b"invalid codec type %d\n\0".as_ptr(), (*ctx).codec_type);
            return -EINVAL;
        }
    } else {
        if (*ctx).codec_type == CODEC_RT5650 {
            pll_id = 0; /* not used in codec driver */
            clk_id = RT5645_SCLK_S_PLL1;
        } else if (*ctx).codec_type == CODEC_RT5682 {
            pll_id = RT5682_PLL1;
            clk_id = RT5682_SCLK_S_PLL1;
        } else if (*ctx).codec_type == CODEC_RT5682S {
            /* check plla_table and pllb_table in rt5682s.c */
            match pll_in {
                3072000 | 24576000 => {
                    /*
                     * For MCLK = 24.576MHz and sample rate = 96KHz case, use PLL1  We don't test
                     * pll_out or params_rate() here since rt5682s PLL2 doesn't support 24.576MHz
                     * input, so we have no choice but to use PLL1. Besides, we will not use PLL at
                     * all if pll_in == pll_out. ex, MCLK = 24.576Mhz and sample rate = 48KHz
                     */
                    pll_id = RT5682S_PLL1;
                    clk_id = RT5682S_SCLK_S_PLL1;
                }
                _ => {
                    pll_id = RT5682S_PLL2;
                    clk_id = RT5682S_SCLK_S_PLL2;
                }
            }
        } else {
            dev_err((*rtd).dev, b"invalid codec type %d\n\0".as_ptr(), (*ctx).codec_type);
            return -EINVAL;
        }

        /* Configure pll for codec */
        ret = snd_soc_dai_set_pll(codec_dai, pll_id, pll_source, pll_in, pll_out);
        if ret < 0 {
            dev_err((*rtd).dev, b"snd_soc_dai_set_pll err = %d\n\0".as_ptr(), ret);
        }
    }

    /* Configure sysclk for codec */
    ret = snd_soc_dai_set_sysclk(codec_dai, clk_id, pll_out, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*rtd).dev, b"snd_soc_dai_set_sysclk err = %d\n\0".as_ptr(), ret);
    }

    /*
     * slot_width should equal or large than data length, set them
     * be the same
     */
    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x0, 0x0, 2, params_width(params));
    if ret < 0 {
        dev_err((*rtd).dev, b"set TDM slot err:%d\n\0".as_ptr(), ret);
        return ret;
    }

    ret
}

static sof_rt5682_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(sof_rt5682_hw_params),
};

unsafe extern "C" fn sof_card_late_probe(card: *mut snd_soc_card) -> c_int {
    let ctx = snd_soc_card_get_drvdata(card);
    let dapm = snd_soc_card_to_dapm(card);
    let err: c_int;

    if (*ctx).amp_type == CODEC_MAX98373 {
        /* Disable Left and Right Spk pin after boot */
        snd_soc_dapm_disable_pin(dapm, b"Left Spk\0".as_ptr());
        snd_soc_dapm_disable_pin(dapm, b"Right Spk\0".as_ptr());
        err = snd_soc_dapm_sync(dapm);
        if err < 0 {
            return err;
        }
    }

    sof_intel_board_card_late_probe(card)
}

static sof_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { name: b"Headphone Jack\0".as_ptr() },
    snd_kcontrol_new { name: b"Headset Mic\0".as_ptr() },
];

static sof_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { name: b"Headphone Jack\0".as_ptr() },
    snd_soc_dapm_widget { name: b"Headset Mic\0".as_ptr() },
];

static sof_map: [snd_soc_dapm_route; 3] = [
    /* HP jack connectors - unknown if we have jack detection */
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr(), control: core::ptr::null(), source: b"HPOL\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr(), control: core::ptr::null(), source: b"HPOR\0".as_ptr() },

    /* other jacks */
    snd_soc_dapm_route { sink: b"IN1P\0".as_ptr(), control: core::ptr::null(), source: b"Headset Mic\0".as_ptr() },
];

static rt5650_spk_kcontrols: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { name: b"Left Spk\0".as_ptr() },
    snd_kcontrol_new { name: b"Right Spk\0".as_ptr() },
];

static rt5650_spk_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { name: b"Left Spk\0".as_ptr() },
    snd_soc_dapm_widget { name: b"Right Spk\0".as_ptr() },
];

static rt5650_spk_dapm_routes: [snd_soc_dapm_route; 2] = [
    /* speaker */
    snd_soc_dapm_route { sink: b"Left Spk\0".as_ptr(), control: core::ptr::null(), source: b"SPOL\0".as_ptr() },
    snd_soc_dapm_route { sink: b"Right Spk\0".as_ptr(), control: core::ptr::null(), source: b"SPOR\0".as_ptr() },
];

unsafe extern "C" fn rt5650_spk_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, rt5650_spk_widgets.as_ptr(), rt5650_spk_widgets.len() as c_int);
    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add rt5650 spk widgets, ret %d\n\0".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_add_card_controls(card, rt5650_spk_kcontrols.as_ptr(), rt5650_spk_kcontrols.len() as c_int);
    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add rt5650 spk kcontrols, ret %d\n\0".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, rt5650_spk_dapm_routes.as_ptr(), rt5650_spk_dapm_routes.len() as c_int);
    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add dapm routes, ret=%d\n\0".as_ptr(), ret);
    }

    ret
}

/* sof audio machine driver for rt5682 codec */
static mut sof_audio_card_rt5682: snd_soc_card = unsafe {
    snd_soc_card {
        name: b"rt5682\0".as_ptr() as *mut i8, /* the sof- prefix is added by the core */
        owner: THIS_MODULE,
        controls: sof_controls.as_ptr(),
        num_controls: sof_controls.len() as c_uint,
        dapm_widgets: sof_widgets.as_ptr(),
        num_dapm_widgets: sof_widgets.len() as c_uint,
        dapm_routes: sof_map.as_ptr(),
        num_dapm_routes: sof_map.len() as c_uint,
        fully_routed: true,
        late_probe: Some(sof_card_late_probe),
        dev: core::ptr::null_mut(),
    }
};

static mut rt5682_component: [snd_soc_dai_link_component; 1] = [
    snd_soc_dai_link_component { name: b"i2c-10EC5682:00\0".as_ptr(), dai_name: b"rt5682-aif1\0".as_ptr() },
];

static mut rt5682s_component: [snd_soc_dai_link_component; 1] = [
    snd_soc_dai_link_component { name: b"i2c-RTL5682:00\0".as_ptr(), dai_name: b"rt5682s-aif1\0".as_ptr() },
];

static mut rt5650_components: [snd_soc_dai_link_component; 2] = [
    snd_soc_dai_link_component { name: b"i2c-10EC5650:00\0".as_ptr(), dai_name: b"rt5645-aif1\0".as_ptr() },
    snd_soc_dai_link_component { name: b"i2c-10EC5650:00\0".as_ptr(), dai_name: b"rt5645-aif2\0".as_ptr() },
];

unsafe extern "C" fn sof_card_dai_links_create(
    dev: *mut device,
    card: *mut snd_soc_card,
    ctx: *mut sof_card_private,
) -> c_int {
    let mut ret: c_int;

    ret = sof_intel_board_set_dai_link(dev, card, ctx);
    if ret != 0 {
        return ret;
    }

    if (*ctx).codec_link.is_null() {
        dev_err(dev, b"codec link not available\0".as_ptr());
        return -EINVAL;
    }

    /* codec-specific fields for headphone codec */
    if (*ctx).codec_type == CODEC_RT5650 {
        (*(*ctx).codec_link).codecs = &mut rt5650_components[0];
        (*(*ctx).codec_link).num_codecs = 1;
    } else if (*ctx).codec_type == CODEC_RT5682 {
        (*(*ctx).codec_link).codecs = rt5682_component.as_mut_ptr();
        (*(*ctx).codec_link).num_codecs = rt5682_component.len() as c_uint;
    } else if (*ctx).codec_type == CODEC_RT5682S {
        (*(*ctx).codec_link).codecs = rt5682s_component.as_mut_ptr();
        (*(*ctx).codec_link).num_codecs = rt5682s_component.len() as c_uint;
    } else {
        dev_err(dev, b"invalid codec type %d\n\0".as_ptr(), (*ctx).codec_type);
        return -EINVAL;
    }

    (*(*ctx).codec_link).init = Some(sof_rt5682_codec_init);
    (*(*ctx).codec_link).exit = Some(sof_rt5682_codec_exit);
    (*(*ctx).codec_link).ops = &sof_rt5682_ops;

    if !(*ctx).rt5682.is_legacy_cpu {
        /*
         * Currently, On SKL+ platforms MCLK will be turned off in sof
         * runtime suspended, and it will go into runtime suspended
         * right after playback is stop. However, rt5682 will output
         * static noise if sysclk turns off during playback. Set
         * ignore_pmdown_time to power down rt5682 immediately and
         * avoid the noise.
         * It can be removed once we can control MCLK by driver.
         */
        (*(*ctx).codec_link).ignore_pmdown_time = 1;
    }

    if (*ctx).amp_type == CODEC_NONE {
        return 0;
    }

    if (*ctx).amp_link.is_null() {
        dev_err(dev, b"amp link not available\0".as_ptr());
        return -EINVAL;
    }

    /* codec-specific fields for speaker amplifier */
    if (*ctx).amp_type == CODEC_MAX98357A {
        max_98357a_dai_link((*ctx).amp_link);
    } else if (*ctx).amp_type == CODEC_MAX98360A {
        max_98360a_dai_link((*ctx).amp_link);
    } else if (*ctx).amp_type == CODEC_MAX98373 {
        max_98373_dai_link(dev, (*ctx).amp_link);
    } else if (*ctx).amp_type == CODEC_MAX98390 {
        max_98390_dai_link(dev, (*ctx).amp_link);
    } else if (*ctx).amp_type == CODEC_RT1011 {
        sof_rt1011_dai_link(dev, (*ctx).amp_link);
    } else if (*ctx).amp_type == CODEC_RT1015 {
        sof_rt1015_dai_link((*ctx).amp_link);
    } else if (*ctx).amp_type == CODEC_RT1015P {
        sof_rt1015p_dai_link((*ctx).amp_link);
    } else if (*ctx).amp_type == CODEC_RT1019P {
        sof_rt1019p_dai_link((*ctx).amp_link);
    } else if (*ctx).amp_type == CODEC_RT5650 {
        /* use AIF2 to support speaker pipeline */
        (*(*ctx).amp_link).codecs = &mut rt5650_components[1];
        (*(*ctx).amp_link).num_codecs = 1;
        (*(*ctx).amp_link).init = Some(rt5650_spk_init);
        (*(*ctx).amp_link).ops = &sof_rt5682_ops;
    } else if (*ctx).amp_type == CODEC_TAS2563 {
        sof_tas2563_dai_link((*ctx).amp_link);
    } else {
        dev_err(dev, b"invalid amp type %d\n\0".as_ptr(), (*ctx).amp_type);
        return -EINVAL;
    }

    0
}

static mut GLK_LINK_ORDER: c_ulong = unsafe {
    SOF_LINK_ORDER(
        SOF_LINK_AMP,
        SOF_LINK_CODEC,
        SOF_LINK_DMIC01,
        SOF_LINK_IDISP_HDMI,
        SOF_LINK_NONE,
        SOF_LINK_NONE,
        SOF_LINK_NONE,
    )
};

unsafe extern "C" fn sof_audio_probe(pdev: *mut platform_device) -> c_int {
    let mach = (*pdev).dev.platform_data as *mut snd_soc_acpi_mach;
    let ctx: *mut sof_card_private;
    let mut card_name: *mut i8;
    let mut ret: c_int;

    if !(*pdev).id_entry.is_null() && (*(*pdev).id_entry).driver_data != 0 {
        sof_rt5682_quirk = (*(*pdev).id_entry).driver_data as c_ulong;
    }

    dmi_check_system(sof_rt5682_quirk_table.as_ptr());

    if quirk_override != -1 {
        dev_info(&mut (*pdev).dev, b"Overriding quirk 0x%lx => 0x%x\n\0".as_ptr(), sof_rt5682_quirk, quirk_override);
        sof_rt5682_quirk = quirk_override as c_ulong;
    }

    dev_dbg(&mut (*pdev).dev, b"sof_rt5682_quirk = %lx\n\0".as_ptr(), sof_rt5682_quirk);

    /* initialize ctx with board quirk */
    ctx = sof_intel_board_get_ctx(&mut (*pdev).dev, sof_rt5682_quirk);
    if ctx.is_null() {
        return -ENOMEM;
    }

    if (*ctx).codec_type == CODEC_RT5650 {
        card_name = devm_kstrdup(&mut (*pdev).dev, b"rt5650\0".as_ptr(), GFP_KERNEL);
        if card_name.is_null() {
            return -ENOMEM;
        }

        sof_audio_card_rt5682.name = card_name;

        /* create speaker dai link also */
        if (*ctx).amp_type == CODEC_NONE {
            (*ctx).amp_type = CODEC_RT5650;
        }
    }

    if ((*mach).mach_params.codec_mask & IDISP_CODEC_MASK) != 0 {
        (*ctx).hdmi.idisp_codec = true;
    }

    if soc_intel_is_byt() || soc_intel_is_cht() {
        (*ctx).rt5682.is_legacy_cpu = true;
        (*ctx).dmic_be_num = 0;
        /* HDMI is not supported by SOF on Baytrail/CherryTrail */
        (*ctx).hdmi_num = 0;
    } else if soc_intel_is_glk() {
        /* dmic16k not support */
        (*ctx).dmic_be_num = 1;

        /* overwrite the DAI link order for GLK boards */
        (*ctx).link_order_overwrite = GLK_LINK_ORDER;

        /* backward-compatible with existing devices */
        if (*ctx).amp_type == CODEC_MAX98357A {
            card_name = devm_kstrdup(&mut (*pdev).dev, b"glkrt5682max\0".as_ptr(), GFP_KERNEL);
            if card_name.is_null() {
                return -ENOMEM;
            }

            sof_audio_card_rt5682.name = card_name;
        }
    } else if soc_intel_is_cml() {
        /* backward-compatible with existing devices */
        if (*ctx).amp_type == CODEC_RT1011 {
            card_name = devm_kstrdup(&mut (*pdev).dev, b"cml_rt1011_rt5682\0".as_ptr(), GFP_KERNEL);
            if card_name.is_null() {
                return -ENOMEM;
            }

            sof_audio_card_rt5682.name = card_name;
        }
    }

    if (sof_rt5682_quirk & SOF_RT5682_MCLK_EN) != 0 {
        (*ctx).rt5682.mclk_en = true;

        /* need to get main clock from pmc */
        if (*ctx).rt5682.is_legacy_cpu {
            (*ctx).rt5682.mclk = devm_clk_get(&mut (*pdev).dev, b"pmc_plt_clk_3\0".as_ptr());
            if IS_ERR((*ctx).rt5682.mclk) {
                ret = PTR_ERR((*ctx).rt5682.mclk);

                dev_err(&mut (*pdev).dev, b"Failed to get MCLK from pmc_plt_clk_3: %d\n\0".as_ptr(), ret);
                return ret;
            }

            ret = clk_prepare_enable((*ctx).rt5682.mclk);
            if ret < 0 {
                dev_err(&mut (*pdev).dev, b"could not configure MCLK state\0".as_ptr());
                return ret;
            }
        }
    }

    /* update dai_link */
    ret = sof_card_dai_links_create(&mut (*pdev).dev, &mut sof_audio_card_rt5682, ctx);
    if ret != 0 {
        return ret;
    }

    /* update codec_conf */
    if (*ctx).amp_type == CODEC_MAX98373 {
        max_98373_set_codec_conf(&mut sof_audio_card_rt5682);
    } else if (*ctx).amp_type == CODEC_MAX98390 {
        max_98390_set_codec_conf(&mut (*pdev).dev, &mut sof_audio_card_rt5682);
    } else if (*ctx).amp_type == CODEC_RT1011 {
        sof_rt1011_codec_conf(&mut (*pdev).dev, &mut sof_audio_card_rt5682);
    } else if (*ctx).amp_type == CODEC_RT1015 {
        sof_rt1015_codec_conf(&mut sof_audio_card_rt5682);
    } else if (*ctx).amp_type == CODEC_RT1015P {
        sof_rt1015p_codec_conf(&mut sof_audio_card_rt5682);
    } else if (*ctx).amp_type == CODEC_MAX98357A
        || (*ctx).amp_type == CODEC_MAX98360A
        || (*ctx).amp_type == CODEC_RT1019P
        || (*ctx).amp_type == CODEC_RT5650
        || (*ctx).amp_type == CODEC_TAS2563
        || (*ctx).amp_type == CODEC_NONE
    {
        /* no codec conf required */
    } else {
        dev_err(&mut (*pdev).dev, b"invalid amp type %d\n\0".as_ptr(), (*ctx).amp_type);
        return -EINVAL;
    }

    sof_audio_card_rt5682.dev = &mut (*pdev).dev;

    /* set platform name for each dailink */
    ret = snd_soc_fixup_dai_links_platform_name(&mut sof_audio_card_rt5682, (*mach).mach_params.platform);
    if ret != 0 {
        return ret;
    }

    snd_soc_card_set_drvdata(&mut sof_audio_card_rt5682, ctx);

    devm_snd_soc_register_card(&mut (*pdev).dev, &mut sof_audio_card_rt5682)
}

static board_ids: [platform_device_id; 19] = unsafe {
    [
        platform_device_id { name: *b"sof_rt5682\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(2)) as kernel_ulong_t },
        platform_device_id { name: *b"glk_rt5682_def\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(2) | SOF_SSP_PORT_AMP(1)) as kernel_ulong_t },
        platform_device_id { name: *b"icl_rt5682_def\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0)) as kernel_ulong_t },
        platform_device_id { name: *b"cml_rt5682_def\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(1)) as kernel_ulong_t },
        platform_device_id { name: *b"jsl_rt5682_def\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(1)) as kernel_ulong_t },
        platform_device_id { name: *b"tgl_rt5682_def\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(1) | SOF_NUM_IDISP_HDMI(4) | SOF_SSP_PORT_BT_OFFLOAD(2) | SOF_BT_OFFLOAD_PRESENT) as kernel_ulong_t },
        platform_device_id { name: *b"adl_rt5682_def\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(1) | SOF_NUM_IDISP_HDMI(4) | SOF_SSP_PORT_BT_OFFLOAD(2) | SOF_BT_OFFLOAD_PRESENT) as kernel_ulong_t },
        platform_device_id { name: *b"adl_mx98357_rt5682\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(2) | SOF_NUM_IDISP_HDMI(4)) as kernel_ulong_t },
        platform_device_id {
            name: *b"adl_rt5682_c1_h02\0\0\0\0\0\0\0\0\0\0\0\0",
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(1) |
                /* SSP 0 and SSP 2 are used for HDMI IN */
                SOF_SSP_MASK_HDMI_CAPTURE(0x5)) as kernel_ulong_t,
        },
        platform_device_id { name: *b"rpl_mx98357_rt5682\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(2) | SOF_NUM_IDISP_HDMI(4)) as kernel_ulong_t },
        platform_device_id { name: *b"rpl_rt5682_def\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(1) | SOF_NUM_IDISP_HDMI(4) | SOF_SSP_PORT_BT_OFFLOAD(2) | SOF_BT_OFFLOAD_PRESENT) as kernel_ulong_t },
        platform_device_id {
            name: *b"rpl_rt5682_c1_h02\0\0\0\0\0\0\0\0\0\0\0\0",
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(1) |
                /* SSP 0 and SSP 2 are used for HDMI IN */
                SOF_SSP_MASK_HDMI_CAPTURE(0x5)) as kernel_ulong_t,
        },
        platform_device_id { name: *b"mtl_rt5682_def\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(1) | SOF_SSP_PORT_BT_OFFLOAD(2) | SOF_BT_OFFLOAD_PRESENT) as kernel_ulong_t },
        platform_device_id {
            name: *b"mtl_rt5682_c1_h02\0\0\0\0\0\0\0\0\0\0\0\0",
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(1) |
                /* SSP 0 and SSP 2 are used for HDMI IN */
                SOF_SSP_MASK_HDMI_CAPTURE(0x5)) as kernel_ulong_t,
        },
        platform_device_id {
            name: *b"arl_rt5682_c1_h02\0\0\0\0\0\0\0\0\0\0\0\0",
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(1) |
                /* SSP 0 and SSP 2 are used for HDMI IN */
                SOF_SSP_MASK_HDMI_CAPTURE(0x5)) as kernel_ulong_t,
        },
        platform_device_id { name: *b"ptl_rt5682_def\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0", driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(0) | SOF_SSP_PORT_AMP(1) | SOF_SSP_PORT_BT_OFFLOAD(2) | SOF_BT_OFFLOAD_PRESENT) as kernel_ulong_t },
        platform_device_id {
            name: *b"ptl_rt5682_c1_h02\0\0\0\0\0\0\0\0\0\0\0\0",
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(1) |
                /* SSP 0 and SSP 2 are used for HDMI IN */
                SOF_SSP_MASK_HDMI_CAPTURE(0x5)) as kernel_ulong_t,
        },
        platform_device_id {
            name: *b"nvl_rt5682_c1_h02\0\0\0\0\0\0\0\0\0\0\0\0",
            driver_data: (SOF_RT5682_MCLK_EN | SOF_SSP_PORT_CODEC(1) |
                /* SSP 0 and SSP 2 are used for HDMI IN */
                SOF_SSP_MASK_HDMI_CAPTURE(0x5)) as kernel_ulong_t,
        },
        platform_device_id { name: [0; 32], driver_data: 0 },
    ]
};
/* MODULE_DEVICE_TABLE(platform, board_ids); */

static mut sof_audio: platform_driver = unsafe {
    platform_driver {
        probe: Some(sof_audio_probe),
        driver: device_driver {
            name: b"sof_rt5682\0".as_ptr(),
            pm: &snd_soc_pm_ops,
        },
        id_table: board_ids.as_ptr(),
    }
};
/* module_platform_driver(sof_audio) */

/* Module information */
/* MODULE_DESCRIPTION("SOF Audio Machine driver");
 * MODULE_AUTHOR("Bard Liao <bard.liao@intel.com>");
 * MODULE_AUTHOR("Sathya Prakash M R <sathya.prakash.m.r@intel.com>");
 * MODULE_AUTHOR("Brent Lu <brent.lu@intel.com>");
 * MODULE_AUTHOR("Mac Chiang <mac.chiang@intel.com>");
 * MODULE_LICENSE("GPL v2");
 * MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_BOARD_HELPERS");
 * MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_MAXIM_COMMON");
 * MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_REALTEK_COMMON");
 * MODULE_IMPORT_NS("SND_SOC_INTEL_SOF_TI_COMMON");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
