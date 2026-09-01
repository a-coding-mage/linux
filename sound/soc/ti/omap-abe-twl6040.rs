// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap-abe-twl6040.c  --  SoC audio for TI OMAP based boards with ABE and
 *			   twl6040 codec
 *
 * Author: Misael Lopez Cruz <misael.lopez@ti.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* Dependencies from Linux, ASoC, OMAP McPDM/DMIC, and twl6040 headers. */
extern "C" {
    static THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn printk(fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        res: *mut c_void,
        num: c_uint,
    ) -> *mut platform_device;
    fn platform_device_unregister(pdev: *mut platform_device);
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;

    fn twl6040_get_clk_id(component: *mut snd_soc_component) -> c_int;
    fn twl6040_get_trim_value(component: *mut snd_soc_component, trim: c_int) -> c_int;
    fn twl6040_hs_jack_detect(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        report: c_int,
    );
    fn omap_mcpdm_configure_dn_offsets(rtd: *mut snd_soc_pcm_runtime, left: c_int, right: c_int);

    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_int)
        -> c_int;
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
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
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub owner: *mut module,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
    pub fully_routed: c_int,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_int,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_int,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub event: *mut c_void,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct abe_twl6040 {
    pub card: snd_soc_card,
    pub dai_links: [snd_soc_dai_link; 2],
    pub jack_detection: c_int, /* board can detect jack events */
    pub mclk_freq: c_int,     /* MCLK frequency speed for twl6040 */
}

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_CLOCK_OUT: c_int = 1;
const SND_JACK_MICROPHONE: c_int = 0x0008;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const TWL6040_SYSCLK_SEL_HPPLL: c_int = 0;
const TWL6040_SYSCLK_SEL_LPPLL: c_int = 1;
const TWL6040_TRIM_HSOTRIM: c_int = 0;
const OMAP_DMIC_SYSCLK_PAD_CLKS: c_int = 0;
const OMAP_DMIC_ABE_DMIC_CLK: c_int = 1;
const KERN_ERR: &[u8] = b"\0";

const fn twl6040_hsf_trim_left(val: c_int) -> c_int {
    val & 0x0f
}

const fn twl6040_hsf_trim_right(val: c_int) -> c_int {
    (val >> 4) & 0x0f
}

static mut link0_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut link0_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"twl6040-codec\0".as_ptr() as *const c_char,
    dai_name: b"twl6040-legacy\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut link0_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut link1_cpus: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];
static mut link1_codecs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"dmic-codec\0".as_ptr() as *const c_char,
    dai_name: b"dmic-hifi\0".as_ptr() as *const c_char,
    of_node: ptr::null_mut(),
}];
static mut link1_platforms: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: ptr::null(),
    dai_name: ptr::null(),
    of_node: ptr::null_mut(),
}];

static mut dmic_codec_dev: *mut platform_device = ptr::null_mut();

unsafe extern "C" fn omap_abe_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let card = (*rtd).card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut abe_twl6040;
    let clk_id: c_int;
    let freq: c_int;
    let ret: c_int;

    clk_id = twl6040_get_clk_id((*codec_dai).component);
    if clk_id == TWL6040_SYSCLK_SEL_HPPLL {
        freq = (*priv_).mclk_freq;
    } else if clk_id == TWL6040_SYSCLK_SEL_LPPLL {
        freq = 32768;
    } else {
        return -EINVAL;
    }

    /* set the codec mclk */
    ret = snd_soc_dai_set_sysclk(codec_dai, clk_id, freq as c_uint, SND_SOC_CLOCK_IN);
    if ret != 0 {
        printk(
            b"%scan't set codec system clock\n\0".as_ptr() as *const c_char,
            KERN_ERR.as_ptr() as *const c_char,
        );
        return ret;
    }
    ret
}

static omap_abe_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(omap_abe_hw_params),
};

unsafe extern "C" fn omap_abe_dmic_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret: c_int = 0;

    ret = snd_soc_dai_set_sysclk(cpu_dai, OMAP_DMIC_SYSCLK_PAD_CLKS, 19200000, SND_SOC_CLOCK_IN);
    if ret < 0 {
        printk(
            b"%scan't set DMIC cpu system clock\n\0".as_ptr() as *const c_char,
            KERN_ERR.as_ptr() as *const c_char,
        );
        return ret;
    }
    ret = snd_soc_dai_set_sysclk(cpu_dai, OMAP_DMIC_ABE_DMIC_CLK, 2400000, SND_SOC_CLOCK_OUT);
    if ret < 0 {
        printk(
            b"%scan't set DMIC output clock\n\0".as_ptr() as *const c_char,
            KERN_ERR.as_ptr() as *const c_char,
        );
        return ret;
    }
    0
}

static omap_abe_dmic_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(omap_abe_dmic_hw_params),
};

/* Headset jack */
static mut hs_jack: snd_soc_jack = snd_soc_jack { _private: [] };

/*Headset jack detection DAPM pins */
static mut hs_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headset Mic\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Headset Stereophone\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
];

/* SDP4430 machine DAPM */
static twl6040_dapm_widgets: [snd_soc_dapm_widget; 10] = [
    /* Outputs */
    snd_soc_dapm_widget { id: 0, name: b"Headset Stereophone\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { id: 0, name: b"Earphone Spk\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { id: 0, name: b"Ext Spk\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { id: 0, name: b"Line Out\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { id: 0, name: b"Vibrator\0".as_ptr() as *const c_char, event: ptr::null_mut() },

    /* Inputs */
    snd_soc_dapm_widget { id: 0, name: b"Headset Mic\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { id: 0, name: b"Main Handset Mic\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { id: 0, name: b"Sub Handset Mic\0".as_ptr() as *const c_char, event: ptr::null_mut() },
    snd_soc_dapm_widget { id: 0, name: b"Line In\0".as_ptr() as *const c_char, event: ptr::null_mut() },

    /* Digital microphones */
    snd_soc_dapm_widget { id: 0, name: b"Digital Mic\0".as_ptr() as *const c_char, event: ptr::null_mut() },
];

static audio_map: [snd_soc_dapm_route; 20] = [
    /* Routings for outputs */
    snd_soc_dapm_route { sink: b"Headset Stereophone\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HSOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headset Stereophone\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HSOR\0".as_ptr() as *const c_char },

    snd_soc_dapm_route { sink: b"Earphone Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"EP\0".as_ptr() as *const c_char },

    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HFL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Ext Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HFR\0".as_ptr() as *const c_char },

    snd_soc_dapm_route { sink: b"Line Out\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AUXL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Line Out\0".as_ptr() as *const c_char, control: ptr::null(), source: b"AUXR\0".as_ptr() as *const c_char },

    snd_soc_dapm_route { sink: b"Vibrator\0".as_ptr() as *const c_char, control: ptr::null(), source: b"VIBRAL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Vibrator\0".as_ptr() as *const c_char, control: ptr::null(), source: b"VIBRAR\0".as_ptr() as *const c_char },

    /* Routings for inputs */
    snd_soc_dapm_route { sink: b"HSMIC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headset Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic Bias\0".as_ptr() as *const c_char },

    snd_soc_dapm_route { sink: b"MAINMIC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Main Handset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Main Handset Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Main Mic Bias\0".as_ptr() as *const c_char },

    snd_soc_dapm_route { sink: b"SUBMIC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Sub Handset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Sub Handset Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Main Mic Bias\0".as_ptr() as *const c_char },

    snd_soc_dapm_route { sink: b"AFML\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Line In\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"AFMR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Line In\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
    snd_soc_dapm_route { sink: ptr::null(), control: ptr::null(), source: ptr::null() },
];

unsafe extern "C" fn omap_abe_twl6040_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let component = (*snd_soc_rtd_to_codec(rtd, 0)).component;
    let card = (*rtd).card;
    let priv_ = snd_soc_card_get_drvdata(card) as *mut abe_twl6040;
    let hs_trim: c_int;
    let ret: c_int;

    /*
     * Configure McPDM offset cancellation based on the HSOTRIM value from
     * twl6040.
     */
    hs_trim = twl6040_get_trim_value(component, TWL6040_TRIM_HSOTRIM);
    omap_mcpdm_configure_dn_offsets(
        rtd,
        twl6040_hsf_trim_left(hs_trim),
        twl6040_hsf_trim_right(hs_trim),
    );

    /* Headset jack detection only if it is supported */
    if (*priv_).jack_detection != 0 {
        ret = snd_soc_card_jack_new_pins(
            (*rtd).card,
            b"Headset Jack\0".as_ptr() as *const c_char,
            SND_JACK_HEADSET,
            &mut hs_jack,
            hs_jack_pins.as_mut_ptr(),
            hs_jack_pins.len() as c_uint,
        );
        if ret != 0 {
            return ret;
        }

        twl6040_hs_jack_detect(component, &mut hs_jack, SND_JACK_HEADSET);
    }

    0
}

static dmic_audio_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: b"DMic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Digital Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Digital Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Digital Mic1 Bias\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn omap_abe_dmic_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let dapm = snd_soc_card_to_dapm((*rtd).card);

    snd_soc_dapm_add_routes(dapm, dmic_audio_map.as_ptr(), dmic_audio_map.len() as c_int)
}

unsafe extern "C" fn omap_abe_probe(pdev: *mut platform_device) -> c_int {
    let node = (*pdev).dev.of_node;
    let mut card: *mut snd_soc_card;
    let mut dai_node: *mut device_node;
    let priv_: *mut abe_twl6040;
    let mut num_links: c_int = 0;
    let mut ret: c_int = 0;

    if node.is_null() {
        dev_err(&mut (*pdev).dev, b"of node is missing.\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    priv_ = devm_kzalloc(&mut (*pdev).dev, size_of::<abe_twl6040>(), GFP_KERNEL) as *mut abe_twl6040;
    if priv_.is_null() {
        return -ENOMEM;
    }

    card = &mut (*priv_).card;
    (*card).dev = &mut (*pdev).dev;
    (*card).owner = THIS_MODULE;
    (*card).dapm_widgets = twl6040_dapm_widgets.as_ptr();
    (*card).num_dapm_widgets = twl6040_dapm_widgets.len() as c_int;
    (*card).dapm_routes = audio_map.as_ptr();
    (*card).num_dapm_routes = audio_map.len() as c_int;

    ret = snd_soc_of_parse_card_name(card, b"ti,model\0".as_ptr() as *const c_char);
    if ret != 0 {
        return ret;
    }

    ret = snd_soc_of_parse_audio_routing(card, b"ti,audio-routing\0".as_ptr() as *const c_char);
    if ret != 0 {
        return ret;
    }

    dai_node = of_parse_phandle(node, b"ti,mcpdm\0".as_ptr() as *const c_char, 0);
    if dai_node.is_null() {
        dev_err(&mut (*pdev).dev, b"McPDM node is not provided\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    (*priv_).dai_links[0].name = b"DMIC\0".as_ptr() as *const c_char;
    (*priv_).dai_links[0].stream_name = b"TWL6040\0".as_ptr() as *const c_char;
    (*priv_).dai_links[0].cpus = link0_cpus.as_mut_ptr();
    (*priv_).dai_links[0].num_cpus = 1;
    (*(*priv_).dai_links[0].cpus).of_node = dai_node;
    (*priv_).dai_links[0].platforms = link0_platforms.as_mut_ptr();
    (*priv_).dai_links[0].num_platforms = 1;
    (*(*priv_).dai_links[0].platforms).of_node = dai_node;
    (*priv_).dai_links[0].codecs = link0_codecs.as_mut_ptr();
    (*priv_).dai_links[0].num_codecs = 1;
    (*priv_).dai_links[0].init = Some(omap_abe_twl6040_init);
    (*priv_).dai_links[0].ops = &omap_abe_ops;

    dai_node = of_parse_phandle(node, b"ti,dmic\0".as_ptr() as *const c_char, 0);
    if !dai_node.is_null() {
        num_links = 2;
        (*priv_).dai_links[1].name = b"TWL6040\0".as_ptr() as *const c_char;
        (*priv_).dai_links[1].stream_name = b"DMIC Capture\0".as_ptr() as *const c_char;
        (*priv_).dai_links[1].cpus = link1_cpus.as_mut_ptr();
        (*priv_).dai_links[1].num_cpus = 1;
        (*(*priv_).dai_links[1].cpus).of_node = dai_node;
        (*priv_).dai_links[1].platforms = link1_platforms.as_mut_ptr();
        (*priv_).dai_links[1].num_platforms = 1;
        (*(*priv_).dai_links[1].platforms).of_node = dai_node;
        (*priv_).dai_links[1].codecs = link1_codecs.as_mut_ptr();
        (*priv_).dai_links[1].num_codecs = 1;
        (*priv_).dai_links[1].init = Some(omap_abe_dmic_init);
        (*priv_).dai_links[1].ops = &omap_abe_dmic_ops;
    } else {
        num_links = 1;
    }

    (*priv_).jack_detection = of_property_read_bool(node, b"ti,jack-detection\0".as_ptr() as *const c_char);
    of_property_read_u32(
        node,
        b"ti,mclk-freq\0".as_ptr() as *const c_char,
        &mut (*priv_).mclk_freq,
    );
    if (*priv_).mclk_freq == 0 {
        dev_err(&mut (*pdev).dev, b"MCLK frequency not provided\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    (*card).fully_routed = 1;

    (*card).dai_link = (*priv_).dai_links.as_mut_ptr();
    (*card).num_links = num_links;

    snd_soc_card_set_drvdata(card, priv_ as *mut c_void);

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        dev_err_probe(
            &mut (*pdev).dev,
            ret,
            b"devm_snd_soc_register_card() failed\n\0".as_ptr() as *const c_char,
        );
    }

    ret
}

static omap_abe_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"ti,abe-twl6040\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, omap_abe_of_match); */

static mut omap_abe_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"omap-abe-twl6040\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops },
        of_match_table: omap_abe_of_match.as_ptr(),
    },
    probe: Some(omap_abe_probe),
};

unsafe extern "C" fn omap_abe_init() -> c_int {
    let ret: c_int;

    dmic_codec_dev = platform_device_register_simple(
        b"dmic-codec\0".as_ptr() as *const c_char,
        -1,
        ptr::null_mut(),
        0,
    );
    if (dmic_codec_dev as isize) < 0 {
        pr_err(
            b"%s: dmic-codec device registration failed\n\0".as_ptr() as *const c_char,
            b"omap_abe_init\0".as_ptr() as *const c_char,
        );
        return dmic_codec_dev as c_int;
    }

    ret = platform_driver_register(&mut omap_abe_driver);
    if ret != 0 {
        pr_err(
            b"%s: platform driver registration failed\n\0".as_ptr() as *const c_char,
            b"omap_abe_init\0".as_ptr() as *const c_char,
        );
        platform_device_unregister(dmic_codec_dev);
    }

    ret
}
/* module_init(omap_abe_init); */

unsafe extern "C" fn omap_abe_exit() {
    platform_driver_unregister(&mut omap_abe_driver);
    platform_device_unregister(dmic_codec_dev);
}
/* module_exit(omap_abe_exit); */

/* MODULE_AUTHOR("Misael Lopez Cruz <misael.lopez@ti.com>"); */
/* MODULE_DESCRIPTION("ALSA SoC for OMAP boards with ABE and twl6040 codec"); */
/* MODULE_LICENSE("GPL"); */
/* MODULE_ALIAS("platform:omap-abe-twl6040"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
