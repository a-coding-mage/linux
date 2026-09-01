// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2021, 2023 Advanced Micro Devices, Inc.
//
// Authors: Ajit Kumar Pandey <AjitKumar.Pandey@amd.com>
//	    Vijendar Mukunda <Vijendar.Mukunda@amd.com>
//

/*
 * Machine Driver Interface for ACP HW block
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const PCO_PLAT_CLK: c_uint = 48000000;
const RT5682_PLL_FREQ: c_uint = 48000 * 512;
const DUAL_CHANNEL: c_uint = 2;
const FOUR_CHANNEL: c_uint = 4;
const NAU8821_CODEC_DAI: *const c_char = b"nau8821-hifi\0".as_ptr() as *const c_char;
const NAU8821_BCLK: c_uint = 1536000;
const NAU8821_FREQ_OUT: c_uint = 12288000;
const MAX98388_CODEC_DAI: *const c_char = b"max98388-aif1\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct dmi_system_id {
    pub matches: [dmi_match; 4],
    pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct dmi_match {
    pub slot: c_int,
    pub substr: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub channels_max: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    pub count: c_uint,
    pub list: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acp_card_drvdata {
    pub wclk: *mut clk,
    pub bclk: *mut clk,
    pub hs_codec_id: c_uint,
    pub amp_codec_id: c_uint,
    pub bt_codec_id: c_uint,
    pub dmic_codec_id: c_uint,
    pub hs_cpu_id: c_uint,
    pub bt_cpu_id: c_uint,
    pub amp_cpu_id: c_uint,
    pub dmic_cpu_id: c_uint,
    pub tdm_mode: bool,
    pub soc_mclk: bool,
    pub acp_rev: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub drvdata: *mut acp_card_drvdata,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_uint,
    pub set_bias_level: Option<
        unsafe extern "C" fn(*mut snd_soc_card, *mut snd_soc_dapm_context, snd_soc_bias_level) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub name: *const c_char,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_int,
}

#[repr(C)]
pub struct snd_kcontrol_new {
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
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub id: c_int,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub playback_only: c_uint,
    pub capture_only: c_uint,
    pub nonatomic: bool,
    pub no_pcm: c_uint,
}

pub type snd_soc_bias_level = c_uint;

extern "C" {
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;

    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn dmi_first_match(ids: *const dmi_system_id) -> *const dmi_system_id;

    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_uint) -> *mut snd_soc_dai;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_codec_dai(card: *mut snd_soc_card, dai_name: *const c_char) -> *mut snd_soc_dai;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_uint,
    ) -> c_int;
    fn snd_soc_add_card_controls(
        card: *mut snd_soc_card,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_jack_set_key(jack: *mut snd_jack, type_: c_int, keytype: c_int);
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_pcm_hw_constraint_list(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        l: *const snd_pcm_hw_constraint_list,
    ) -> c_int;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(dai: *mut snd_soc_dai, clk_id: c_int, freq: c_uint, dir: c_int) -> c_int;
    fn snd_soc_dai_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_physical_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn nau8821_enable_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack);
    fn acp_ops_configure_link(card: *mut snd_soc_card, link: *mut snd_soc_dai_link) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

extern "C" {
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

const EINVAL: c_int = 22;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;
const EIO: c_int = 5;
const GFP_KERNEL: c_uint = 0;
const DMI_PRODUCT_FAMILY: c_int = 0;
const DMI_SYS_VENDOR: c_int = 1;
const DMI_PRODUCT_NAME: c_int = 2;
const QUIRK_TDM_MODE_ENABLE: usize = 1;
const QUIRK_REMAP_DMIC_BT: usize = 2;
const SND_JACK_HEADPHONE: c_int = 0x0001;
const SND_JACK_MICROPHONE: c_int = 0x0002;
const SND_JACK_HEADSET: c_int = SND_JACK_HEADPHONE | SND_JACK_MICROPHONE;
const SND_JACK_LINEOUT: c_int = 0x0004;
const SND_JACK_BTN_0: c_int = 0x4000;
const SND_JACK_BTN_1: c_int = 0x2000;
const SND_JACK_BTN_2: c_int = 0x1000;
const SND_JACK_BTN_3: c_int = 0x0800;
const KEY_PLAYPAUSE: c_int = 164;
const KEY_VOICECOMMAND: c_int = 246;
const KEY_VOLUMEUP: c_int = 115;
const KEY_VOLUMEDOWN: c_int = 114;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_uint = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_uint = 1;
const SNDRV_PCM_HW_PARAM_SAMPLE_BITS: c_uint = 2;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0x0003;
const SND_SOC_DAIFMT_I2S: c_uint = 0x0001;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0x0000;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 0x4000;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 0x1000;
const SND_SOC_CLOCK_IN: c_int = 0;
const SND_SOC_BIAS_OFF: snd_soc_bias_level = 0;
const SND_SOC_BIAS_STANDBY: snd_soc_bias_level = 1;
const SND_SOC_DAPM_PRE_PMU: c_int = 0x1;
const SND_SOC_DAPM_POST_PMD: c_int = 0x2;
const RT5682: c_uint = 1;
const RT5682S: c_uint = 2;
const NAU8825: c_uint = 3;
const NAU8821: c_uint = 4;
const RT1019: c_uint = 5;
const MAX98360A: c_uint = 6;
const MAX98388: c_uint = 7;
const ES83XX: c_uint = 8;
const I2S_SP: c_uint = 1;
const I2S_HS: c_uint = 2;
const I2S_BT: c_uint = 3;
const DMIC: c_uint = 4;
const HEADSET_BE_ID: c_int = 0;
const AMP_BE_ID: c_int = 1;
const BT_BE_ID: c_int = 2;
const DMIC_BE_ID: c_int = 3;
const ACP_RN_PCI_ID: c_uint = 0x15e2;
const ACP_RMB_PCI_ID: c_uint = 0x15e2;
const ACP63_PCI_ID: c_uint = 0x15e2;
const ACP70_PCI_ID: c_uint = 0x15e2;
const ACP71_PCI_ID: c_uint = 0x15e2;
const ACP72_PCI_ID: c_uint = 0x15e2;
const RT5682_PLL2: c_int = 0;
const RT5682_PLL2_S_MCLK: c_int = 0;
const RT5682_SCLK_S_PLL2: c_int = 0;
const RT5682S_PLL2: c_int = 0;
const RT5682S_PLL_S_MCLK: c_int = 0;
const RT5682S_SCLK_S_PLL2: c_int = 0;
const RT5682S_PLL1: c_int = 1;
const RT5682S_PLL_S_BCLK1: c_int = 1;
const RT5682S_SCLK_S_PLL1: c_int = 1;
const RT1019_PLL_S_BCLK: c_int = 0;
const RT1019_SCLK_S_PLL: c_int = 0;
const TDM_CHANNELS: c_uint = 8;
const NAU8825_CLK_FLL_FS: c_int = 0;
const NAU8821_CLK_INTERNAL: c_int = 0;
const NAU8821_CLK_FLL_BLK: c_int = 1;

unsafe fn snd_soc_dapm_event_off(event: c_int) -> bool {
    event == SND_SOC_DAPM_POST_PMD
}

pub static mut acp_quirk_table: [dmi_system_id; 3] = [
    dmi_system_id {
        /* Google skyrim proto-0 */
        matches: [
            dmi_match { slot: DMI_PRODUCT_FAMILY, substr: b"Google_Skyrim\0".as_ptr() as *const c_char },
            dmi_match { slot: 0, substr: ptr::null() },
            dmi_match { slot: 0, substr: ptr::null() },
            dmi_match { slot: 0, substr: ptr::null() },
        ],
        driver_data: QUIRK_TDM_MODE_ENABLE as *mut c_void,
    },
    dmi_system_id {
        /* Valve Steam Deck OLED */
        matches: [
            dmi_match { slot: DMI_SYS_VENDOR, substr: b"Valve\0".as_ptr() as *const c_char },
            dmi_match { slot: DMI_PRODUCT_NAME, substr: b"Galileo\0".as_ptr() as *const c_char },
            dmi_match { slot: 0, substr: ptr::null() },
            dmi_match { slot: 0, substr: ptr::null() },
        ],
        driver_data: QUIRK_REMAP_DMIC_BT as *mut c_void,
    },
    dmi_system_id {
        matches: [
            dmi_match { slot: 0, substr: ptr::null() },
            dmi_match { slot: 0, substr: ptr::null() },
            dmi_match { slot: 0, substr: ptr::null() },
            dmi_match { slot: 0, substr: ptr::null() },
        ],
        driver_data: ptr::null_mut(),
    },
];

static channels: [c_uint; 1] = [DUAL_CHANNEL];
static rates: [c_uint; 1] = [48000];
static constraints_rates: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: rates.len() as c_uint,
    list: rates.as_ptr(),
    mask: 0,
};
static constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: channels.len() as c_uint,
    list: channels.as_ptr(),
    mask: 0,
};

unsafe extern "C" fn acp_clk_enable(
    drvdata: *mut acp_card_drvdata,
    srate: c_uint,
    bclk_ratio: c_uint,
) -> c_int {
    clk_set_rate((*drvdata).wclk, srate);
    clk_set_rate((*drvdata).bclk, srate.wrapping_mul(bclk_ratio));
    clk_prepare_enable((*drvdata).wclk)
}

/* DAILINK_COMP_ARRAY(COMP_CODEC(...)) translated as static component arrays. */
static mut rt5682: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"i2c-10EC5682:00\0".as_ptr() as *const c_char,
    dai_name: b"rt5682-aif1\0".as_ptr() as *const c_char,
}];
static mut rt5682_jack: snd_soc_jack = snd_soc_jack { jack: ptr::null_mut() };
static mut rt5682_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin { pin: b"Headphone Jack\0".as_ptr() as *const c_char, mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: b"Headset Mic\0".as_ptr() as *const c_char, mask: SND_JACK_MICROPHONE },
];
/* SOC_DAPM_PIN_SWITCH/SND_SOC_DAPM_* initializers depend on ASoC macros. */
static rt5682_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }];
static rt5682_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
];
static rt5682_map: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"IN1P\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn acp_card_rt5682_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let drvdata = (*card).drvdata;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component = (*codec_dai).component;
    let mut ret: c_int;

    dev_info((*rtd).dev, b"codec dai name = %s\n\0".as_ptr() as *const c_char, (*codec_dai).name);
    if (*drvdata).hs_codec_id != RT5682 {
        return -EINVAL;
    }
    (*drvdata).wclk = devm_clk_get((*component).dev, b"rt5682-dai-wclk\0".as_ptr() as *const c_char);
    if IS_ERR((*drvdata).wclk as *const c_void) {
        return PTR_ERR((*drvdata).wclk as *const c_void);
    }
    (*drvdata).bclk = devm_clk_get((*component).dev, b"rt5682-dai-bclk\0".as_ptr() as *const c_char);
    if IS_ERR((*drvdata).bclk as *const c_void) {
        return PTR_ERR((*drvdata).bclk as *const c_void);
    }
    ret = snd_soc_dapm_new_controls(dapm, rt5682_widgets.as_ptr(), rt5682_widgets.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add widget dapm controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_add_card_controls(card, rt5682_controls.as_ptr(), rt5682_controls.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add card controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_card_jack_new_pins(
        card,
        b"Headset Jack\0".as_ptr() as *const c_char,
        SND_JACK_HEADSET | SND_JACK_LINEOUT | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3,
        &mut rt5682_jack,
        rt5682_jack_pins.as_mut_ptr(),
        rt5682_jack_pins.len() as c_uint,
    );
    if ret != 0 {
        dev_err((*card).dev, b"HP jack creation failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    snd_jack_set_key(rt5682_jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(rt5682_jack.jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key(rt5682_jack.jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key(rt5682_jack.jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    ret = snd_soc_component_set_jack(component, &mut rt5682_jack, ptr::null_mut());
    if ret != 0 {
        dev_err((*rtd).dev, b"Headset Jack call-back failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    snd_soc_dapm_add_routes(dapm, rt5682_map.as_ptr(), rt5682_map.len() as c_int)
}

unsafe extern "C" fn acp_card_hs_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let drvdata = (*card).drvdata;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let mut fmt: c_uint = if (*drvdata).tdm_mode { SND_SOC_DAIFMT_DSP_A } else { SND_SOC_DAIFMT_I2S };
    if (*drvdata).soc_mclk {
        fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    } else {
        fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;
    }
    let ret = snd_soc_dai_set_fmt(codec_dai, fmt);
    if ret < 0 {
        dev_err((*(*rtd).card).dev, b"Failed to set dai fmt: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    ret
}

unsafe extern "C" fn acp_card_shutdown(substream: *mut snd_pcm_substream) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let drvdata = (*card).drvdata;
    if !(*drvdata).soc_mclk {
        clk_disable_unprepare((*drvdata).wclk);
    }
}

unsafe extern "C" fn set_common_hs_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    is_rt5682s: bool,
) -> c_int {
    let rtd = if is_rt5682s { (*substream).private_data as *mut snd_soc_pcm_runtime } else { snd_soc_substream_to_rtd(substream) };
    let card = (*rtd).card;
    let drvdata = (*card).drvdata;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let srate = params_rate(params);
    let ch = params_channels(params);
    let format = params_physical_width(params);
    let mut fmt: c_uint = if (*drvdata).tdm_mode { SND_SOC_DAIFMT_DSP_A } else { SND_SOC_DAIFMT_I2S };
    if (*drvdata).soc_mclk {
        fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    } else {
        fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;
    }
    let mut ret = snd_soc_dai_set_fmt(cpu_dai, fmt);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err((*rtd).dev, b"Failed to set dai fmt: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_dai_set_fmt(codec_dai, fmt);
    if ret < 0 {
        dev_err((*(*rtd).card).dev, b"Failed to set dai fmt: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    if (*drvdata).tdm_mode {
        /**
         * As codec supports slot 0 and slot 1 for playback and capture.
         */
        ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0x3, 0x3, 8, 16);
        if ret != 0 && ret != -ENOTSUPP {
            dev_err((*rtd).dev, b"set TDM slot err: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x3, 0x3, 8, 16);
        if ret < 0 {
            dev_warn((*rtd).dev, b"set TDM slot err:%d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    if is_rt5682s {
        ret = snd_soc_dai_set_pll(codec_dai, RT5682S_PLL2, RT5682S_PLL_S_MCLK, PCO_PLAT_CLK, RT5682_PLL_FREQ);
        if ret < 0 {
            dev_err((*rtd).dev, b"Failed to set codec PLL: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        ret = snd_soc_dai_set_sysclk(codec_dai, RT5682S_SCLK_S_PLL2, RT5682_PLL_FREQ, SND_SOC_CLOCK_IN);
    } else {
        ret = snd_soc_dai_set_pll(codec_dai, RT5682_PLL2, RT5682_PLL2_S_MCLK, PCO_PLAT_CLK, RT5682_PLL_FREQ);
        if ret < 0 {
            dev_err((*rtd).dev, b"Failed to set codec PLL: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        ret = snd_soc_dai_set_sysclk(codec_dai, RT5682_SCLK_S_PLL2, RT5682_PLL_FREQ, SND_SOC_CLOCK_IN);
    }
    if ret < 0 {
        dev_err((*rtd).dev, b"Failed to set codec SYSCLK: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    if (*drvdata).tdm_mode {
        ret = snd_soc_dai_set_pll(codec_dai, RT5682S_PLL1, RT5682S_PLL_S_BCLK1, 6144000, 49152000);
        if ret < 0 {
            dev_err((*rtd).dev, b"Failed to set codec PLL: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        ret = snd_soc_dai_set_sysclk(codec_dai, RT5682S_SCLK_S_PLL1, 49152000, SND_SOC_CLOCK_IN);
        if ret < 0 {
            dev_err((*rtd).dev, b"Failed to set codec SYSCLK: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    /* Set tdm/i2s1 master bclk ratio */
    ret = snd_soc_dai_set_bclk_ratio(codec_dai, ch.wrapping_mul(format));
    if ret < 0 {
        dev_err((*rtd).dev, b"Failed to set rt5682 tdm bclk ratio: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    if is_rt5682s {
        clk_set_rate((*drvdata).wclk, srate);
        clk_set_rate((*drvdata).bclk, srate.wrapping_mul(ch).wrapping_mul(format));
    }
    if !(*drvdata).soc_mclk {
        ret = acp_clk_enable(drvdata, srate, ch.wrapping_mul(format));
        if ret < 0 {
            dev_err((*(*rtd).card).dev, b"Failed to enable HS clk: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    0
}

unsafe extern "C" fn acp_card_rt5682_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    set_common_hs_params(substream, params, false)
}

static acp_card_rt5682_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp_card_hs_startup),
    shutdown: Some(acp_card_shutdown),
    hw_params: Some(acp_card_rt5682_hw_params),
};

static mut rt5682s: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"i2c-RTL5682:00\0".as_ptr() as *const c_char,
    dai_name: b"rt5682s-aif1\0".as_ptr() as *const c_char,
}];
static mut rt5682s_jack: snd_soc_jack = snd_soc_jack { jack: ptr::null_mut() };
static mut rt5682s_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin { pin: b"Headphone Jack\0".as_ptr() as *const c_char, mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: b"Headset Mic\0".as_ptr() as *const c_char, mask: SND_JACK_MICROPHONE },
];
static rt5682s_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }];
static rt5682s_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
];
static rt5682s_map: [snd_soc_dapm_route; 3] = rt5682_map;

unsafe extern "C" fn acp_card_rt5682s_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let drvdata = (*card).drvdata;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component = (*codec_dai).component;
    let mut ret: c_int;
    dev_info((*rtd).dev, b"codec dai name = %s\n\0".as_ptr() as *const c_char, (*codec_dai).name);
    if (*drvdata).hs_codec_id != RT5682S {
        return -EINVAL;
    }
    if !(*drvdata).soc_mclk {
        (*drvdata).wclk = devm_clk_get((*component).dev, b"rt5682-dai-wclk\0".as_ptr() as *const c_char);
        if IS_ERR((*drvdata).wclk as *const c_void) {
            return PTR_ERR((*drvdata).wclk as *const c_void);
        }
        (*drvdata).bclk = devm_clk_get((*component).dev, b"rt5682-dai-bclk\0".as_ptr() as *const c_char);
        if IS_ERR((*drvdata).bclk as *const c_void) {
            return PTR_ERR((*drvdata).bclk as *const c_void);
        }
    }
    ret = snd_soc_dapm_new_controls(dapm, rt5682s_widgets.as_ptr(), rt5682s_widgets.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add widget dapm controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_add_card_controls(card, rt5682s_controls.as_ptr(), rt5682s_controls.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add card controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_card_jack_new_pins(card, b"Headset Jack\0".as_ptr() as *const c_char, SND_JACK_HEADSET | SND_JACK_LINEOUT | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3, &mut rt5682s_jack, rt5682s_jack_pins.as_mut_ptr(), rt5682s_jack_pins.len() as c_uint);
    if ret != 0 {
        dev_err((*card).dev, b"HP jack creation failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    snd_jack_set_key(rt5682s_jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(rt5682s_jack.jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key(rt5682s_jack.jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key(rt5682s_jack.jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    ret = snd_soc_component_set_jack(component, &mut rt5682s_jack, ptr::null_mut());
    if ret != 0 {
        dev_err((*rtd).dev, b"Headset Jack call-back failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    snd_soc_dapm_add_routes(dapm, rt5682s_map.as_ptr(), rt5682s_map.len() as c_int)
}

unsafe extern "C" fn acp_card_rt5682s_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    set_common_hs_params(substream, params, true)
}

static acp_card_rt5682s_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp_card_hs_startup),
    shutdown: None,
    hw_params: Some(acp_card_rt5682s_hw_params),
};

static dmic_channels: [c_uint; 2] = [DUAL_CHANNEL, FOUR_CHANNEL];
static dmic_constraints_channels: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    count: dmic_channels.len() as c_uint,
    list: dmic_channels.as_ptr(),
    mask: 0,
};

unsafe extern "C" fn acp_card_dmic_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &dmic_constraints_channels);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    0
}

static acp_card_dmic_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp_card_dmic_startup),
    shutdown: None,
    hw_params: None,
};

static mut rt1019: [snd_soc_dai_link_component; 2] = [
    snd_soc_dai_link_component { name: b"i2c-10EC1019:00\0".as_ptr() as *const c_char, dai_name: b"rt1019-aif\0".as_ptr() as *const c_char },
    snd_soc_dai_link_component { name: b"i2c-10EC1019:01\0".as_ptr() as *const c_char, dai_name: b"rt1019-aif\0".as_ptr() as *const c_char },
];
static rt1019_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }];
static rt1019_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
];
static rt1019_map_lr: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: b"Left Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Left SPO\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Right SPO\0".as_ptr() as *const c_char },
];
static mut rt1019_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: b"i2c-10EC1019:01\0".as_ptr() as *const c_char, dai_name: ptr::null() }, name_prefix: b"Left\0".as_ptr() as *const c_char },
    snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: b"i2c-10EC1019:00\0".as_ptr() as *const c_char, dai_name: ptr::null() }, name_prefix: b"Right\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn acp_card_rt1019_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let drvdata = (*card).drvdata;
    let mut ret: c_int;
    if (*drvdata).amp_codec_id != RT1019 {
        return -EINVAL;
    }
    ret = snd_soc_dapm_new_controls(dapm, rt1019_widgets.as_ptr(), rt1019_widgets.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add widget dapm controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_add_card_controls(card, rt1019_controls.as_ptr(), rt1019_controls.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add card controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    snd_soc_dapm_add_routes(dapm, rt1019_map_lr.as_ptr(), rt1019_map_lr.len() as c_int)
}

unsafe extern "C" fn acp_card_rt1019_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = (*substream).private_data as *mut snd_soc_pcm_runtime;
    let card = (*rtd).card;
    let drvdata = (*card).drvdata;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let srate = params_rate(params);
    let ch = params_channels(params);
    let format = params_physical_width(params);
    if (*drvdata).amp_codec_id != RT1019 {
        return -EINVAL;
    }
    let mut fmt: c_uint = if (*drvdata).tdm_mode { SND_SOC_DAIFMT_DSP_A } else { SND_SOC_DAIFMT_I2S };
    if (*drvdata).soc_mclk {
        fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    } else {
        fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;
    }
    let mut ret = snd_soc_dai_set_fmt(cpu_dai, fmt);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err((*rtd).dev, b"Failed to set dai fmt: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    if (*drvdata).tdm_mode {
        /**
         * As codec supports slot 2 and slot 3 for playback.
         */
        ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0xC, 0, 8, 16);
        if ret != 0 && ret != -ENOTSUPP {
            dev_err((*rtd).dev, b"set TDM slot err: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    let mut i = 0usize;
    while i < rt1019.len() {
        let codec_dai = snd_soc_rtd_to_codec(rtd, i as c_uint);
        if strcmp((*codec_dai).name, b"rt1019-aif\0".as_ptr() as *const c_char) != 0 {
            i += 1;
            continue;
        }
        if (*drvdata).tdm_mode {
            ret = snd_soc_dai_set_pll(codec_dai, 0, RT1019_PLL_S_BCLK, TDM_CHANNELS.wrapping_mul(format).wrapping_mul(srate), 256u32.wrapping_mul(srate));
        } else {
            ret = snd_soc_dai_set_pll(codec_dai, 0, RT1019_PLL_S_BCLK, ch.wrapping_mul(format).wrapping_mul(srate), 256u32.wrapping_mul(srate));
        }
        if ret < 0 {
            return ret;
        }
        ret = snd_soc_dai_set_sysclk(codec_dai, RT1019_SCLK_S_PLL, 256u32.wrapping_mul(srate), SND_SOC_CLOCK_IN);
        if ret < 0 {
            return ret;
        }
        if (*drvdata).tdm_mode {
            ret = snd_soc_dai_set_fmt(codec_dai, SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_NB_NF);
            if ret < 0 {
                dev_err((*(*rtd).card).dev, b"Failed to set dai fmt: %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
            /**
             * As codec supports slot 2 for left channel playback.
             */
            if strcmp((*(*codec_dai).component).name, b"i2c-10EC1019:00\0".as_ptr() as *const c_char) == 0 {
                ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x4, 0x4, 8, 16);
                if ret < 0 {
                    break;
                }
            }
            /**
             * As codec supports slot 3 for right channel playback.
             */
            if strcmp((*(*codec_dai).component).name, b"i2c-10EC1019:01\0".as_ptr() as *const c_char) == 0 {
                ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x8, 0x8, 8, 16);
                if ret < 0 {
                    break;
                }
            }
        }
        i += 1;
    }
    if !(*drvdata).soc_mclk {
        ret = acp_clk_enable(drvdata, srate, ch.wrapping_mul(format));
        if ret < 0 {
            dev_err((*(*rtd).card).dev, b"Failed to enable AMP clk: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    0
}

unsafe extern "C" fn acp_card_amp_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    0
}

static acp_card_rt1019_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp_card_amp_startup),
    shutdown: Some(acp_card_shutdown),
    hw_params: Some(acp_card_rt1019_hw_params),
};

static mut max98360a: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"MX98360A:00\0".as_ptr() as *const c_char,
    dai_name: b"HiFi\0".as_ptr() as *const c_char,
}];
static max98360a_controls: [snd_kcontrol_new; 1] = [snd_kcontrol_new { _private: [] }];
static max98360a_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget { dapm: ptr::null_mut() }];
static max98360a_map: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: b"Spk\0".as_ptr() as *const c_char,
    control: ptr::null(),
    source: b"Speaker\0".as_ptr() as *const c_char,
}];

unsafe extern "C" fn acp_card_maxim_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let drvdata = (*card).drvdata;
    let mut ret: c_int;
    if (*drvdata).amp_codec_id != MAX98360A {
        return -EINVAL;
    }
    ret = snd_soc_dapm_new_controls(dapm, max98360a_widgets.as_ptr(), max98360a_widgets.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add widget dapm controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_add_card_controls(card, max98360a_controls.as_ptr(), max98360a_controls.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add card controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    snd_soc_dapm_add_routes(dapm, max98360a_map.as_ptr(), max98360a_map.len() as c_int)
}

unsafe extern "C" fn acp_card_maxim_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = (*substream).private_data as *mut snd_soc_pcm_runtime;
    let card = (*rtd).card;
    let drvdata = (*card).drvdata;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let srate = params_rate(params);
    let ch = params_channels(params);
    let format = params_physical_width(params);
    let mut fmt: c_uint = if (*drvdata).tdm_mode { SND_SOC_DAIFMT_DSP_A } else { SND_SOC_DAIFMT_I2S };
    if (*drvdata).soc_mclk {
        fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    } else {
        fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;
    }
    let mut ret = snd_soc_dai_set_fmt(cpu_dai, fmt);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err((*rtd).dev, b"Failed to set dai fmt: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    if (*drvdata).tdm_mode {
        /**
         * As codec supports slot 2 and slot 3 for playback.
         */
        ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0xC, 0, 8, 16);
        if ret != 0 && ret != -ENOTSUPP {
            dev_err((*rtd).dev, b"set TDM slot err: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    if !(*drvdata).soc_mclk {
        ret = acp_clk_enable(drvdata, srate, ch.wrapping_mul(format));
        if ret < 0 {
            dev_err((*(*rtd).card).dev, b"Failed to enable AMP clk: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    0
}

static acp_card_maxim_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp_card_amp_startup),
    shutdown: Some(acp_card_shutdown),
    hw_params: Some(acp_card_maxim_hw_params),
};

static mut max98388: [snd_soc_dai_link_component; 2] = [
    snd_soc_dai_link_component { name: b"i2c-ADS8388:00\0".as_ptr() as *const c_char, dai_name: MAX98388_CODEC_DAI },
    snd_soc_dai_link_component { name: b"i2c-ADS8388:01\0".as_ptr() as *const c_char, dai_name: MAX98388_CODEC_DAI },
];
static max98388_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }];
static max98388_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
];
static max98388_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: b"Left Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Left BE_OUT\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Spk\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Right BE_OUT\0".as_ptr() as *const c_char },
];
static mut max98388_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: b"i2c-ADS8388:00\0".as_ptr() as *const c_char, dai_name: ptr::null() }, name_prefix: b"Left\0".as_ptr() as *const c_char },
    snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: b"i2c-ADS8388:01\0".as_ptr() as *const c_char, dai_name: ptr::null() }, name_prefix: b"Right\0".as_ptr() as *const c_char },
];
static max98388_format: [c_uint; 1] = [16];
static mut constraints_sample_bits_max: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: max98388_format.as_ptr(),
    count: max98388_format.len() as c_uint,
    mask: 0,
};

unsafe extern "C" fn acp_card_max98388_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_SAMPLE_BITS, &constraints_sample_bits_max);
    0
}

unsafe extern "C" fn acp_card_max98388_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let drvdata = (*card).drvdata;
    let mut ret: c_int;
    if (*drvdata).amp_codec_id != MAX98388 {
        return -EINVAL;
    }
    ret = snd_soc_dapm_new_controls(dapm, max98388_widgets.as_ptr(), max98388_widgets.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add widget dapm controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        /* Don't need to add routes if widget addition failed */
        return ret;
    }
    ret = snd_soc_add_card_controls(card, max98388_controls.as_ptr(), max98388_controls.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add card controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    snd_soc_dapm_add_routes(dapm, max98388_map.as_ptr(), max98388_map.len() as c_int)
}

unsafe extern "C" fn acp_max98388_hw_params(substream: *mut snd_pcm_substream, _params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = (*substream).private_data as *mut snd_soc_pcm_runtime;
    let card = (*rtd).card;
    let codec_dai = snd_soc_card_get_codec_dai(card, MAX98388_CODEC_DAI);
    snd_soc_dai_set_fmt(codec_dai, SND_SOC_DAIFMT_CBC_CFC | SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF)
}

static acp_max98388_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp_card_max98388_startup),
    shutdown: None,
    hw_params: Some(acp_max98388_hw_params),
};

static mut nau8825: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"i2c-10508825:00\0".as_ptr() as *const c_char,
    dai_name: b"nau8825-hifi\0".as_ptr() as *const c_char,
}];
static mut nau8825_jack: snd_soc_jack = snd_soc_jack { jack: ptr::null_mut() };
static mut nau8825_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin { pin: b"Headphone Jack\0".as_ptr() as *const c_char, mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: b"Headset Mic\0".as_ptr() as *const c_char, mask: SND_JACK_MICROPHONE },
];
static nau8825_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }];
static nau8825_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
];
static nau8825_map: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOR\0".as_ptr() as *const c_char },
];

unsafe extern "C" fn acp_card_nau8825_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let drvdata = (*card).drvdata;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component = (*codec_dai).component;
    let mut ret: c_int;
    dev_info((*rtd).dev, b"codec dai name = %s\n\0".as_ptr() as *const c_char, (*codec_dai).name);
    if (*drvdata).hs_codec_id != NAU8825 {
        return -EINVAL;
    }
    ret = snd_soc_dapm_new_controls(dapm, nau8825_widgets.as_ptr(), nau8825_widgets.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add widget dapm controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_add_card_controls(card, nau8825_controls.as_ptr(), nau8825_controls.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add card controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_card_jack_new_pins(card, b"Headset Jack\0".as_ptr() as *const c_char, SND_JACK_HEADSET | SND_JACK_LINEOUT | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3, &mut nau8825_jack, nau8825_jack_pins.as_mut_ptr(), nau8825_jack_pins.len() as c_uint);
    if ret != 0 {
        dev_err((*card).dev, b"HP jack creation failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    snd_jack_set_key(nau8825_jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(nau8825_jack.jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key(nau8825_jack.jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key(nau8825_jack.jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    ret = snd_soc_component_set_jack(component, &mut nau8825_jack, ptr::null_mut());
    if ret != 0 {
        dev_err((*rtd).dev, b"Headset Jack call-back failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    snd_soc_dapm_add_routes(dapm, nau8825_map.as_ptr(), nau8825_map.len() as c_int)
}

unsafe extern "C" fn acp_nau8825_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let drvdata = (*card).drvdata;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let mut ret = snd_soc_dai_set_sysclk(codec_dai, NAU8825_CLK_FLL_FS, 48000 * 256, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*rtd).dev, b"snd_soc_dai_set_sysclk err = %d\n\0".as_ptr() as *const c_char, ret);
    }
    ret = snd_soc_dai_set_pll(codec_dai, 0, 0, params_rate(params), params_rate(params).wrapping_mul(256));
    if ret < 0 {
        dev_err((*rtd).dev, b"can't set FLL: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    let mut fmt: c_uint = if (*drvdata).tdm_mode { SND_SOC_DAIFMT_DSP_A } else { SND_SOC_DAIFMT_I2S };
    if (*drvdata).soc_mclk {
        fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    } else {
        fmt |= SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;
    }
    ret = snd_soc_dai_set_fmt(cpu_dai, fmt);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err((*rtd).dev, b"Failed to set dai fmt: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_dai_set_fmt(codec_dai, fmt);
    if ret < 0 {
        dev_err((*(*rtd).card).dev, b"Failed to set dai fmt: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    if (*drvdata).tdm_mode {
        /**
         * As codec supports slot 4 and slot 5 for playback and slot 6 for capture.
         */
        ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0x30, 0xC0, 8, 16);
        if ret != 0 && ret != -ENOTSUPP {
            dev_err((*rtd).dev, b"set TDM slot err: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        ret = snd_soc_dai_set_tdm_slot(codec_dai, 0x40, 0x30, 8, 16);
        if ret < 0 {
            dev_warn((*rtd).dev, b"set TDM slot err:%d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }
    ret
}

unsafe extern "C" fn acp_nau8825_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    (*runtime).hw.channels_max = 2;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);
    (*runtime).hw.formats = SNDRV_PCM_FMTBIT_S16_LE;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    0
}

static acp_card_nau8825_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp_nau8825_startup),
    shutdown: None,
    hw_params: Some(acp_nau8825_hw_params),
};

unsafe extern "C" fn platform_clock_control(w: *mut snd_soc_dapm_widget, _k: *mut snd_kcontrol, event: c_int) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let codec_dai = snd_soc_card_get_codec_dai(card, NAU8821_CODEC_DAI);
    let mut ret: c_int = 0;
    if codec_dai.is_null() {
        dev_err((*card).dev, b"Codec dai not found\n\0".as_ptr() as *const c_char);
        return -EIO;
    }
    if snd_soc_dapm_event_off(event) {
        ret = snd_soc_dai_set_sysclk(codec_dai, NAU8821_CLK_INTERNAL, 0, SND_SOC_CLOCK_IN);
        if ret < 0 {
            dev_err((*card).dev, b"set sysclk err = %d\n\0".as_ptr() as *const c_char, ret);
            return -EIO;
        }
    } else {
        ret = snd_soc_dai_set_sysclk(codec_dai, NAU8821_CLK_FLL_BLK, 0, SND_SOC_CLOCK_IN);
        if ret < 0 {
            dev_err((*codec_dai).dev, b"can't set FS clock %d\n\0".as_ptr() as *const c_char, ret);
        }
        ret = snd_soc_dai_set_pll(codec_dai, 0, 0, NAU8821_BCLK, NAU8821_FREQ_OUT);
        if ret < 0 {
            dev_err((*codec_dai).dev, b"can't set FLL: %d\n\0".as_ptr() as *const c_char, ret);
        }
    }
    ret
}

static mut nau8821_jack: snd_soc_jack = snd_soc_jack { jack: ptr::null_mut() };
static mut nau8821_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin { pin: b"Headphone Jack\0".as_ptr() as *const c_char, mask: SND_JACK_HEADPHONE },
    snd_soc_jack_pin { pin: b"Headset Mic\0".as_ptr() as *const c_char, mask: SND_JACK_MICROPHONE },
];
static nau8821_controls: [snd_kcontrol_new; 2] = [snd_kcontrol_new { _private: [] }, snd_kcontrol_new { _private: [] }];
static nau8821_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
    snd_soc_dapm_widget { dapm: ptr::null_mut() },
];
static nau8821_audio_route: [snd_soc_dapm_route; 8] = [
    /* HP jack connectors - unknown if we have jack detection */
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"HPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MICL\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"MICR\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Headset Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DMIC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Int Mic\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headphone Jack\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Headset Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Int Mic\0".as_ptr() as *const c_char, control: ptr::null(), source: b"Platform Clock\0".as_ptr() as *const c_char },
];
static nau8821_format: [c_uint; 1] = [16];
static mut constraints_sample_bits: snd_pcm_hw_constraint_list = snd_pcm_hw_constraint_list {
    list: nau8821_format.as_ptr(),
    count: nau8821_format.len() as c_uint,
    mask: 0,
};

unsafe extern "C" fn acp_8821_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component = (*codec_dai).component;
    let mut ret: c_int;
    dev_info((*rtd).dev, b"codec dai name = %s\n\0".as_ptr() as *const c_char, (*codec_dai).name);
    ret = snd_soc_dapm_new_controls(dapm, nau8821_widgets.as_ptr(), nau8821_widgets.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add widget dapm controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        // Don't need to add routes if widget addition failed
        return ret;
    }
    ret = snd_soc_add_card_controls(card, nau8821_controls.as_ptr(), nau8821_controls.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"unable to add card controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_card_jack_new_pins(card, b"Headset Jack\0".as_ptr() as *const c_char, SND_JACK_HEADSET | SND_JACK_LINEOUT | SND_JACK_BTN_0 | SND_JACK_BTN_1 | SND_JACK_BTN_2 | SND_JACK_BTN_3, &mut nau8821_jack, nau8821_jack_pins.as_mut_ptr(), nau8821_jack_pins.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"Headset Jack creation failed %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    snd_jack_set_key(nau8821_jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(nau8821_jack.jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key(nau8821_jack.jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key(nau8821_jack.jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    nau8821_enable_jack_detect(component, &mut nau8821_jack);
    snd_soc_dapm_add_routes(dapm, nau8821_audio_route.as_ptr(), nau8821_audio_route.len() as c_int)
}

unsafe extern "C" fn acp_8821_startup(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    (*runtime).hw.channels_max = DUAL_CHANNEL;
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_CHANNELS, &constraints_channels);
    snd_pcm_hw_constraint_list(runtime, 0, SNDRV_PCM_HW_PARAM_RATE, &constraints_rates);
    snd_pcm_hw_constraint_list((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_SAMPLE_BITS, &constraints_sample_bits);
    0
}

unsafe extern "C" fn acp_nau8821_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let drvdata = (*card).drvdata;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let fmt = if (*drvdata).soc_mclk {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC
    } else {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP
    };
    let mut ret = snd_soc_dai_set_fmt(codec_dai, fmt);
    if ret < 0 {
        dev_err((*(*rtd).card).dev, b"Failed to set dai fmt: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret = snd_soc_dai_set_sysclk(codec_dai, NAU8821_CLK_FLL_BLK, 0, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*card).dev, b"can't set FS clock %d\n\0".as_ptr() as *const c_char, ret);
    }
    ret = snd_soc_dai_set_pll(codec_dai, 0, 0, snd_soc_params_to_bclk(params), params_rate(params).wrapping_mul(256));
    if ret < 0 {
        dev_err((*card).dev, b"can't set FLL: %d\n\0".as_ptr() as *const c_char, ret);
    }
    ret
}

static acp_8821_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(acp_8821_startup),
    shutdown: None,
    hw_params: Some(acp_nau8821_hw_params),
};

static mut nau8821: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"i2c-NVTN2020:00\0".as_ptr() as *const c_char,
    dai_name: NAU8821_CODEC_DAI,
}];
static mut dmic_codec: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"dmic-codec\0".as_ptr() as *const c_char,
    dai_name: b"dmic-hifi\0".as_ptr() as *const c_char,
}];

static mut platform_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"acp_asoc_renoir.0\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];
static mut platform_rmb_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"acp_asoc_rembrandt.0\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];
static mut platform_acp63_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"acp_asoc_acp63.0\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];
static mut platform_acp70_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"acp_asoc_acp70.0\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];
static mut sof_component: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component {
    name: b"0000:04:00.5\0".as_ptr() as *const c_char,
    dai_name: ptr::null(),
}];

static mut i2s_sp: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"acp-i2s-sp\0".as_ptr() as *const c_char, dai_name: ptr::null() }];
static mut i2s_hs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"acp-i2s-hs\0".as_ptr() as *const c_char, dai_name: ptr::null() }];
static mut sof_sp: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"acp-sof-sp\0".as_ptr() as *const c_char, dai_name: ptr::null() }];
static mut sof_sp_virtual: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"acp-sof-sp-virtual\0".as_ptr() as *const c_char, dai_name: ptr::null() }];
static mut sof_hs: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"acp-sof-hs\0".as_ptr() as *const c_char, dai_name: ptr::null() }];
static mut sof_hs_virtual: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"acp-sof-hs-virtual\0".as_ptr() as *const c_char, dai_name: ptr::null() }];
static mut sof_bt: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"acp-sof-bt\0".as_ptr() as *const c_char, dai_name: ptr::null() }];
static mut sof_dmic: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"acp-sof-dmic\0".as_ptr() as *const c_char, dai_name: ptr::null() }];
static mut pdm_dmic: [snd_soc_dai_link_component; 1] = [snd_soc_dai_link_component { name: b"acp-pdm-dmic\0".as_ptr() as *const c_char, dai_name: ptr::null() }];

unsafe extern "C" fn acp_rtk_set_bias_level(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    let component = snd_soc_dapm_to_component(dapm);
    let drvdata = (*card).drvdata;
    let mut ret: c_int = 0;
    if component.is_null() {
        return 0;
    }
    if strncmp((*component).name, b"i2c-RTL5682\0".as_ptr() as *const c_char, 11) != 0
        && strncmp((*component).name, b"i2c-10EC1019\0".as_ptr() as *const c_char, 12) != 0
    {
        return 0;
    }
    /*
     * For Realtek's codec and amplifier components,
     * the lrck and bclk must be enabled brfore their all dapms be powered on,
     * and must be disabled after their all dapms be powered down
     * to avoid any pop.
     */
    match level {
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                /* Increase bclk's enable_count */
                ret = clk_prepare_enable((*drvdata).bclk);
                if ret < 0 {
                    dev_err((*component).dev, b"Failed to enable bclk %d\n\0".as_ptr() as *const c_char, ret);
                }
            } else {
                /*
                 * Decrease bclk's enable_count.
                 * While the enable_count is 0, the bclk would be closed.
                 */
                clk_disable_unprepare((*drvdata).bclk);
            }
        }
        _ => {}
    }
    ret
}

unsafe fn set_dummy_codec(link: *mut snd_soc_dai_link) {
    (*link).codecs = &mut snd_soc_dummy_dlc;
    (*link).num_codecs = 1;
}

#[no_mangle]
pub unsafe extern "C" fn acp_sofdsp_dai_links_create(card: *mut snd_soc_card) -> c_int {
    let dev = (*card).dev;
    let drv_data = (*card).drvdata;
    let dmi_id = dmi_first_match(acp_quirk_table.as_ptr());
    let mut i: c_int = 0;
    let mut num_links: c_int = 0;
    if (*drv_data).hs_cpu_id != 0 { num_links += 1; }
    if (*drv_data).bt_cpu_id != 0 { num_links += 1; }
    if (*drv_data).amp_cpu_id != 0 { num_links += 1; }
    if (*drv_data).dmic_cpu_id != 0 { num_links += 1; }
    let links = devm_kcalloc(dev, num_links as usize, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    if links.is_null() {
        return -ENOMEM;
    }

    if (*drv_data).hs_cpu_id == I2S_SP {
        let link = links.add(i as usize);
        (*link).name = b"acp-headset-codec\0".as_ptr() as *const c_char;
        (*link).id = HEADSET_BE_ID;
        (*link).cpus = sof_sp.as_mut_ptr();
        (*link).num_cpus = sof_sp.len() as c_uint;
        (*link).platforms = sof_component.as_mut_ptr();
        (*link).num_platforms = sof_component.len() as c_uint;
        (*link).nonatomic = true;
        (*link).no_pcm = 1;
        if (*drv_data).hs_codec_id == 0 { set_dummy_codec(link); }
        if (*drv_data).hs_codec_id == RT5682 {
            (*link).codecs = rt5682.as_mut_ptr();
            (*link).num_codecs = rt5682.len() as c_uint;
            (*link).init = Some(acp_card_rt5682_init);
            (*link).ops = &acp_card_rt5682_ops;
        }
        if (*drv_data).hs_codec_id == RT5682S {
            (*link).codecs = rt5682s.as_mut_ptr();
            (*link).num_codecs = rt5682s.len() as c_uint;
            (*link).init = Some(acp_card_rt5682s_init);
            (*link).ops = &acp_card_rt5682s_ops;
        }
        if (*drv_data).hs_codec_id == NAU8821 {
            (*link).codecs = nau8821.as_mut_ptr();
            (*link).num_codecs = nau8821.len() as c_uint;
            (*link).init = Some(acp_8821_init);
            (*link).ops = &acp_8821_ops;
        }
        i += 1;
    }

    if (*drv_data).hs_cpu_id == I2S_HS {
        let link = links.add(i as usize);
        (*link).name = b"acp-headset-codec\0".as_ptr() as *const c_char;
        (*link).id = HEADSET_BE_ID;
        (*link).cpus = sof_hs.as_mut_ptr();
        (*link).num_cpus = sof_hs.len() as c_uint;
        (*link).platforms = sof_component.as_mut_ptr();
        (*link).num_platforms = sof_component.len() as c_uint;
        (*link).nonatomic = true;
        (*link).no_pcm = 1;
        if (*drv_data).hs_codec_id == 0 { set_dummy_codec(link); }
        if (*drv_data).hs_codec_id == NAU8825 {
            (*link).codecs = nau8825.as_mut_ptr();
            (*link).num_codecs = nau8825.len() as c_uint;
            (*link).init = Some(acp_card_nau8825_init);
            (*link).ops = &acp_card_nau8825_ops;
        }
        if (*drv_data).hs_codec_id == RT5682S {
            (*link).codecs = rt5682s.as_mut_ptr();
            (*link).num_codecs = rt5682s.len() as c_uint;
            (*link).init = Some(acp_card_rt5682s_init);
            (*link).ops = &acp_card_rt5682s_ops;
        }
        i += 1;
    }

    if (*drv_data).amp_cpu_id == I2S_SP {
        let link = links.add(i as usize);
        (*link).name = b"acp-amp-codec\0".as_ptr() as *const c_char;
        (*link).id = AMP_BE_ID;
        if (*drv_data).acp_rev == ACP_RN_PCI_ID {
            (*link).cpus = sof_sp.as_mut_ptr();
            (*link).num_cpus = sof_sp.len() as c_uint;
        } else {
            (*link).cpus = sof_sp_virtual.as_mut_ptr();
            (*link).num_cpus = sof_sp_virtual.len() as c_uint;
        }
        (*link).platforms = sof_component.as_mut_ptr();
        (*link).num_platforms = sof_component.len() as c_uint;
        (*link).playback_only = 1;
        (*link).nonatomic = true;
        (*link).no_pcm = 1;
        if (*drv_data).amp_codec_id == 0 { set_dummy_codec(link); }
        if (*drv_data).amp_codec_id == RT1019 {
            (*link).codecs = rt1019.as_mut_ptr();
            (*link).num_codecs = rt1019.len() as c_uint;
            (*link).ops = &acp_card_rt1019_ops;
            (*link).init = Some(acp_card_rt1019_init);
            (*card).codec_conf = rt1019_conf.as_mut_ptr();
            (*card).num_configs = rt1019_conf.len() as c_uint;
        }
        if (*drv_data).amp_codec_id == MAX98360A {
            (*link).codecs = max98360a.as_mut_ptr();
            (*link).num_codecs = max98360a.len() as c_uint;
            (*link).ops = &acp_card_maxim_ops;
            (*link).init = Some(acp_card_maxim_init);
        }
        i += 1;
    }

    if (*drv_data).amp_cpu_id == I2S_HS {
        let link = links.add(i as usize);
        (*link).name = b"acp-amp-codec\0".as_ptr() as *const c_char;
        (*link).id = AMP_BE_ID;
        (*link).cpus = sof_hs_virtual.as_mut_ptr();
        (*link).num_cpus = sof_hs_virtual.len() as c_uint;
        (*link).platforms = sof_component.as_mut_ptr();
        (*link).num_platforms = sof_component.len() as c_uint;
        (*link).playback_only = 1;
        (*link).nonatomic = true;
        (*link).no_pcm = 1;
        if (*drv_data).amp_codec_id == 0 { set_dummy_codec(link); }
        if (*drv_data).amp_codec_id == MAX98360A {
            (*link).codecs = max98360a.as_mut_ptr();
            (*link).num_codecs = max98360a.len() as c_uint;
            (*link).ops = &acp_card_maxim_ops;
            (*link).init = Some(acp_card_maxim_init);
        }
        if (*drv_data).amp_codec_id == MAX98388 {
            (*link).playback_only = 0;
            (*link).codecs = max98388.as_mut_ptr();
            (*link).num_codecs = max98388.len() as c_uint;
            (*link).ops = &acp_max98388_ops;
            (*link).init = Some(acp_card_max98388_init);
            (*card).codec_conf = max98388_conf.as_mut_ptr();
            (*card).num_configs = max98388_conf.len() as c_uint;
        }
        if (*drv_data).amp_codec_id == RT1019 {
            (*link).codecs = rt1019.as_mut_ptr();
            (*link).num_codecs = rt1019.len() as c_uint;
            (*link).ops = &acp_card_rt1019_ops;
            (*link).init = Some(acp_card_rt1019_init);
            (*card).codec_conf = rt1019_conf.as_mut_ptr();
            (*card).num_configs = rt1019_conf.len() as c_uint;
        }
        i += 1;
    }

    if (*drv_data).bt_cpu_id == I2S_BT {
        let link = links.add(i as usize);
        (*link).name = b"acp-bt-codec\0".as_ptr() as *const c_char;
        (*link).id = BT_BE_ID;
        (*link).cpus = sof_bt.as_mut_ptr();
        (*link).num_cpus = sof_bt.len() as c_uint;
        (*link).platforms = sof_component.as_mut_ptr();
        (*link).num_platforms = sof_component.len() as c_uint;
        (*link).nonatomic = true;
        (*link).no_pcm = 1;
        if (*drv_data).bt_codec_id == 0 { set_dummy_codec(link); }
        if !dmi_id.is_null() && (*dmi_id).driver_data == QUIRK_REMAP_DMIC_BT as *mut c_void {
            (*link).id = DMIC_BE_ID;
        }
        i += 1;
    }

    if (*drv_data).dmic_cpu_id == DMIC {
        let link = links.add(i as usize);
        (*link).name = b"acp-dmic-codec\0".as_ptr() as *const c_char;
        (*link).id = DMIC_BE_ID;
        (*link).codecs = dmic_codec.as_mut_ptr();
        (*link).num_codecs = dmic_codec.len() as c_uint;
        (*link).cpus = sof_dmic.as_mut_ptr();
        (*link).num_cpus = sof_dmic.len() as c_uint;
        (*link).platforms = sof_component.as_mut_ptr();
        (*link).num_platforms = sof_component.len() as c_uint;
        (*link).capture_only = 1;
        (*link).nonatomic = true;
        (*link).no_pcm = 1;
        if !dmi_id.is_null() && (*dmi_id).driver_data == QUIRK_REMAP_DMIC_BT as *mut c_void {
            (*link).id = BT_BE_ID;
            dev_dbg(dev, b"quirk REMAP_DMIC_BT enabled\n\0".as_ptr() as *const c_char);
        }
    }

    (*card).dai_link = links;
    (*card).num_links = num_links;
    (*card).set_bias_level = Some(acp_rtk_set_bias_level);
    0
}

unsafe fn set_legacy_platform(link: *mut snd_soc_dai_link, drv_data: *mut acp_card_drvdata) {
    match (*drv_data).acp_rev {
        ACP_RMB_PCI_ID => {
            (*link).platforms = platform_rmb_component.as_mut_ptr();
            (*link).num_platforms = platform_rmb_component.len() as c_uint;
        }
        ACP63_PCI_ID => {
            (*link).platforms = platform_acp63_component.as_mut_ptr();
            (*link).num_platforms = platform_acp63_component.len() as c_uint;
        }
        _ => {
            (*link).platforms = platform_component.as_mut_ptr();
            (*link).num_platforms = platform_component.len() as c_uint;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn acp_legacy_dai_links_create(card: *mut snd_soc_card) -> c_int {
    let dev = (*card).dev;
    let drv_data = (*card).drvdata;
    let mut i: c_int = 0;
    let mut num_links: c_int = 0;
    let mut rc: c_int;
    if (*drv_data).hs_cpu_id != 0 { num_links += 1; }
    if (*drv_data).amp_cpu_id != 0 { num_links += 1; }
    if (*drv_data).dmic_cpu_id != 0 { num_links += 1; }
    let links = devm_kcalloc(dev, num_links as usize, size_of::<snd_soc_dai_link>(), GFP_KERNEL) as *mut snd_soc_dai_link;
    if links.is_null() {
        return -ENOMEM;
    }
    if (*drv_data).hs_cpu_id == I2S_SP {
        let link = links.add(i as usize);
        (*link).name = b"acp-headset-codec\0".as_ptr() as *const c_char;
        (*link).id = HEADSET_BE_ID;
        (*link).cpus = i2s_sp.as_mut_ptr();
        (*link).num_cpus = i2s_sp.len() as c_uint;
        (*link).platforms = platform_component.as_mut_ptr();
        (*link).num_platforms = platform_component.len() as c_uint;
        if (*drv_data).hs_codec_id == 0 { set_dummy_codec(link); }
        if (*drv_data).hs_codec_id == RT5682 {
            (*link).codecs = rt5682.as_mut_ptr();
            (*link).num_codecs = rt5682.len() as c_uint;
            (*link).init = Some(acp_card_rt5682_init);
            (*link).ops = &acp_card_rt5682_ops;
        }
        if (*drv_data).hs_codec_id == RT5682S {
            (*link).codecs = rt5682s.as_mut_ptr();
            (*link).num_codecs = rt5682s.len() as c_uint;
            (*link).init = Some(acp_card_rt5682s_init);
            (*link).ops = &acp_card_rt5682s_ops;
        }
        if (*drv_data).hs_codec_id == ES83XX {
            rc = acp_ops_configure_link(card, link);
            if rc != 0 {
                dev_err(dev, b"Failed to configure link for ES83XX: %d\n\0".as_ptr() as *const c_char, rc);
                return rc;
            }
        }
        i += 1;
    }
    if (*drv_data).hs_cpu_id == I2S_HS {
        let link = links.add(i as usize);
        (*link).name = b"acp-headset-codec\0".as_ptr() as *const c_char;
        (*link).id = HEADSET_BE_ID;
        (*link).cpus = i2s_hs.as_mut_ptr();
        (*link).num_cpus = i2s_hs.len() as c_uint;
        set_legacy_platform(link, drv_data);
        if (*drv_data).hs_codec_id == 0 { set_dummy_codec(link); }
        if (*drv_data).hs_codec_id == NAU8825 {
            (*link).codecs = nau8825.as_mut_ptr();
            (*link).num_codecs = nau8825.len() as c_uint;
            (*link).init = Some(acp_card_nau8825_init);
            (*link).ops = &acp_card_nau8825_ops;
        }
        if (*drv_data).hs_codec_id == RT5682S {
            (*link).codecs = rt5682s.as_mut_ptr();
            (*link).num_codecs = rt5682s.len() as c_uint;
            (*link).init = Some(acp_card_rt5682s_init);
            (*link).ops = &acp_card_rt5682s_ops;
        }
        i += 1;
    }
    if (*drv_data).amp_cpu_id == I2S_SP {
        let link = links.add(i as usize);
        (*link).name = b"acp-amp-codec\0".as_ptr() as *const c_char;
        (*link).id = AMP_BE_ID;
        (*link).cpus = i2s_sp.as_mut_ptr();
        (*link).num_cpus = i2s_sp.len() as c_uint;
        (*link).platforms = platform_component.as_mut_ptr();
        (*link).num_platforms = platform_component.len() as c_uint;
        (*link).playback_only = 1;
        if (*drv_data).amp_codec_id == 0 { set_dummy_codec(link); }
        if (*drv_data).amp_codec_id == RT1019 {
            (*link).codecs = rt1019.as_mut_ptr();
            (*link).num_codecs = rt1019.len() as c_uint;
            (*link).ops = &acp_card_rt1019_ops;
            (*link).init = Some(acp_card_rt1019_init);
            (*card).codec_conf = rt1019_conf.as_mut_ptr();
            (*card).num_configs = rt1019_conf.len() as c_uint;
        }
        if (*drv_data).amp_codec_id == MAX98360A {
            (*link).codecs = max98360a.as_mut_ptr();
            (*link).num_codecs = max98360a.len() as c_uint;
            (*link).ops = &acp_card_maxim_ops;
            (*link).init = Some(acp_card_maxim_init);
        }
        i += 1;
    }
    if (*drv_data).amp_cpu_id == I2S_HS {
        let link = links.add(i as usize);
        (*link).name = b"acp-amp-codec\0".as_ptr() as *const c_char;
        (*link).id = AMP_BE_ID;
        (*link).cpus = i2s_hs.as_mut_ptr();
        (*link).num_cpus = i2s_hs.len() as c_uint;
        set_legacy_platform(link, drv_data);
        (*link).playback_only = 1;
        if (*drv_data).amp_codec_id == 0 { set_dummy_codec(link); }
        if (*drv_data).amp_codec_id == MAX98360A {
            (*link).codecs = max98360a.as_mut_ptr();
            (*link).num_codecs = max98360a.len() as c_uint;
            (*link).ops = &acp_card_maxim_ops;
            (*link).init = Some(acp_card_maxim_init);
        }
        if (*drv_data).amp_codec_id == RT1019 {
            (*link).codecs = rt1019.as_mut_ptr();
            (*link).num_codecs = rt1019.len() as c_uint;
            (*link).ops = &acp_card_rt1019_ops;
            (*link).init = Some(acp_card_rt1019_init);
            (*card).codec_conf = rt1019_conf.as_mut_ptr();
            (*card).num_configs = rt1019_conf.len() as c_uint;
        }
        i += 1;
    }
    if (*drv_data).dmic_cpu_id == DMIC {
        let link = links.add(i as usize);
        (*link).name = b"acp-dmic-codec\0".as_ptr() as *const c_char;
        (*link).stream_name = b"DMIC capture\0".as_ptr() as *const c_char;
        (*link).id = DMIC_BE_ID;
        if (*drv_data).dmic_codec_id == DMIC {
            (*link).codecs = dmic_codec.as_mut_ptr();
            (*link).num_codecs = dmic_codec.len() as c_uint;
        } else {
            /* Use dummy codec if codec id not specified */
            set_dummy_codec(link);
        }
        (*link).cpus = pdm_dmic.as_mut_ptr();
        (*link).num_cpus = pdm_dmic.len() as c_uint;
        match (*drv_data).acp_rev {
            ACP_RMB_PCI_ID => {
                (*link).platforms = platform_rmb_component.as_mut_ptr();
                (*link).num_platforms = platform_rmb_component.len() as c_uint;
            }
            ACP63_PCI_ID => {
                (*link).platforms = platform_acp63_component.as_mut_ptr();
                (*link).num_platforms = platform_acp63_component.len() as c_uint;
            }
            ACP70_PCI_ID | ACP71_PCI_ID | ACP72_PCI_ID => {
                (*link).platforms = platform_acp70_component.as_mut_ptr();
                (*link).num_platforms = platform_acp70_component.len() as c_uint;
            }
            _ => {
                (*link).platforms = platform_component.as_mut_ptr();
                (*link).num_platforms = platform_component.len() as c_uint;
            }
        }
        (*link).ops = &acp_card_dmic_ops;
        (*link).capture_only = 1;
    }
    (*card).dai_link = links;
    (*card).num_links = num_links;
    (*card).set_bias_level = Some(acp_rtk_set_bias_level);
    0
}

/* MODULE_DESCRIPTION("AMD ACP Common Machine driver"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
