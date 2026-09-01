// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2020 Intel Corporation

// C includes translated as external dependency intent:
// linux/device.h, linux/kernel.h, sound/pcm.h, sound/pcm_params.h,
// sound/soc.h, sound/soc-acpi.h, sound/soc-dai.h, sound/soc-dapm.h,
// sound/sof.h, uapi/sound/asound.h, codecs/rt1011.h, codecs/rt1015.h,
// codecs/rt1308.h, common/soc-intel-quirks.h, sof_realtek_common.h.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_device {
    _private: [u8; 0],
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
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
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
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub ops: *const snd_soc_ops,
    pub dai_fmt: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub dai_link: *mut snd_soc_dai_link,
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
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
struct tdm_mask {
    tx: c_uint,
    rx: c_uint,
}

const ENODEV: c_int = 19;
const EINVAL: c_int = 22;

unsafe extern "C" {
    static RT1011_DEV0_NAME: c_char;
    static RT1011_DEV1_NAME: c_char;
    static RT1011_DEV2_NAME: c_char;
    static RT1011_DEV3_NAME: c_char;
    static RT1011_CODEC_DAI: c_char;
    static RT1011_ACPI_HID: c_char;
    static RT1015P_DEV0_NAME: c_char;
    static RT1015P_CODEC_DAI: c_char;
    static RT1015_DEV0_NAME: c_char;
    static RT1015_DEV1_NAME: c_char;
    static RT1015_CODEC_DAI: c_char;
    static RT1015_ACPI_HID: c_char;
    static RT1308_DEV0_NAME: c_char;
    static RT1308_CODEC_DAI: c_char;
    static RT1019P_DEV0_NAME: c_char;
    static RT1019P_CODEC_DAI: c_char;

    static RT1011_PLL1_S_BCLK: c_int;
    static RT1011_FS_SYS_PRE_S_PLL1: c_int;
    static RT1015_PLL_S_BCLK: c_int;
    static RT1015_SCLK_S_PLL: c_int;
    static RT1308_PLL_S_MCLK: c_int;
    static RT1308_FS_SYS_S_PLL: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;

    fn for_each_acpi_dev_match_count(hid: *const c_char, data: *mut c_void, score: c_int) -> c_uint;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_int,
        freq_out: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_int,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_uint,
    ) -> c_int;
    fn snd_soc_add_card_controls(
        card: *mut snd_soc_card,
        controls: *const snd_kcontrol_new,
        num: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn sof_dai_get_bclk(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn sof_dai_get_mclk(rtd: *mut snd_soc_pcm_runtime) -> c_int;
    fn soc_intel_is_cml() -> bool;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

// Macro-created controls/widgets are preserved as macro intent.
unsafe extern "C" {
    static realtek_2spk_kcontrols: [snd_kcontrol_new; 2]; // SOC_DAPM_PIN_SWITCH("Left Spk"), "Right Spk"
    static realtek_2spk_widgets: [snd_soc_dapm_widget; 2]; // SND_SOC_DAPM_SPK("Left Spk"), "Right Spk"
    static realtek_4spk_kcontrols: [snd_kcontrol_new; 4]; // SOC_DAPM_PIN_SWITCH("WL/WR/TL/TR Ext Spk")
    static realtek_4spk_widgets: [snd_soc_dapm_widget; 4]; // SND_SOC_DAPM_SPK("WL/WR/TL/TR Ext Spk")
    static rt1308_kcontrols: [snd_kcontrol_new; 1]; // SOC_DAPM_PIN_SWITCH("Speakers")
    static rt1308_dapm_widgets: [snd_soc_dapm_widget; 1]; // SND_SOC_DAPM_SPK("Speakers")
}

unsafe fn cstr(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

/*
 * Common structures and functions
 */

/* helper function to get the number of specific codec */
unsafe extern "C" fn get_num_codecs(hid: *const c_char) -> c_uint {
    let dev_num: c_uint = for_each_acpi_dev_match_count(hid, core::ptr::null_mut(), -1);

    dev_num
}

/*
 * Realtek ALC1011
 */
static speaker_map_lr: [snd_soc_dapm_route; 2] = [
    /* speaker */
    snd_soc_dapm_route { sink: b"Left Spk\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Left SPO\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Spk\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Right SPO\0".as_ptr() as *const c_char },
];

static rt1011_4spk_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"WL Ext Spk\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"WL SPO\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"WR Ext Spk\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"WR SPO\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"TL Ext Spk\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"TL SPO\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"TR Ext Spk\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"TR SPO\0".as_ptr() as *const c_char },
];

static mut rt1011_2spk_codec_confs: [snd_soc_codec_conf; 2] = unsafe {
    [
        snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: &RT1011_DEV0_NAME, dai_name: core::ptr::null() }, name_prefix: b"Left\0".as_ptr() as *const c_char },
        snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: &RT1011_DEV1_NAME, dai_name: core::ptr::null() }, name_prefix: b"Right\0".as_ptr() as *const c_char },
    ]
};

static mut rt1011_4spk_codec_confs: [snd_soc_codec_conf; 4] = unsafe {
    [
        snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: &RT1011_DEV0_NAME, dai_name: core::ptr::null() }, name_prefix: b"WL\0".as_ptr() as *const c_char },
        snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: &RT1011_DEV1_NAME, dai_name: core::ptr::null() }, name_prefix: b"WR\0".as_ptr() as *const c_char },
        snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: &RT1011_DEV2_NAME, dai_name: core::ptr::null() }, name_prefix: b"TL\0".as_ptr() as *const c_char },
        snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: &RT1011_DEV3_NAME, dai_name: core::ptr::null() }, name_prefix: b"TR\0".as_ptr() as *const c_char },
    ]
};

static mut rt1011_dai_link_components: [snd_soc_dai_link_component; 4] = unsafe {
    [
        snd_soc_dai_link_component { name: &RT1011_DEV0_NAME, dai_name: &RT1011_CODEC_DAI },
        snd_soc_dai_link_component { name: &RT1011_DEV1_NAME, dai_name: &RT1011_CODEC_DAI },
        snd_soc_dai_link_component { name: &RT1011_DEV2_NAME, dai_name: &RT1011_CODEC_DAI },
        snd_soc_dai_link_component { name: &RT1011_DEV3_NAME, dai_name: &RT1011_CODEC_DAI },
    ]
};

static rt1011_tdm_mask: [tdm_mask; 4] = [
    tdm_mask { tx: 0x4, rx: 0x1 },
    tdm_mask { tx: 0x8, rx: 0x2 },
    tdm_mask { tx: 0x1, rx: 0x1 },
    tdm_mask { tx: 0x2, rx: 0x2 },
];

unsafe extern "C" fn rt1011_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let mut codec_dai: *mut snd_soc_dai;
    let srate: c_int;
    let mut i: c_int;
    let mut ret: c_int = 0;

    srate = params_rate(params);

    i = 0;
    while i < (*(*rtd).dai_link).num_codecs as c_int {
        codec_dai = snd_soc_rtd_to_codec(rtd, i);
        /* 100 Fs to drive 24 bit data */
        ret = snd_soc_dai_set_pll(codec_dai, 0, RT1011_PLL1_S_BCLK, 100 * srate, 256 * srate);
        if ret < 0 {
            dev_err((*codec_dai).dev, b"fail to set pll, ret %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        ret = snd_soc_dai_set_sysclk(codec_dai, RT1011_FS_SYS_PRE_S_PLL1, 256 * srate, SND_SOC_CLOCK_IN);
        if ret < 0 {
            dev_err((*codec_dai).dev, b"fail to set sysclk, ret %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        if i as usize >= rt1011_tdm_mask.len() {
            dev_err((*codec_dai).dev, b"invalid codec index %d\n\0".as_ptr() as *const c_char, i);
            return -ENODEV;
        }

        ret = snd_soc_dai_set_tdm_slot(
            codec_dai,
            rt1011_tdm_mask[i as usize].tx,
            rt1011_tdm_mask[i as usize].rx,
            4,
            params_width(params),
        );
        if ret < 0 {
            dev_err((*codec_dai).dev, b"fail to set tdm slot, ret %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
        i += 1;
    }

    0
}

static rt1011_ops: snd_soc_ops = snd_soc_ops { hw_params: Some(rt1011_hw_params) };

unsafe extern "C" fn rt1011_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let num_codecs = get_num_codecs(&RT1011_ACPI_HID);
    let mut ret: c_int;

    match num_codecs {
        2 => {
            if !soc_intel_is_cml() {
                ret = snd_soc_dapm_new_controls(dapm, realtek_2spk_widgets.as_ptr(), realtek_2spk_widgets.len() as c_uint);
                if ret != 0 {
                    dev_err((*rtd).dev, b"fail to add rt1011 widgets, ret %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }

                ret = snd_soc_add_card_controls(card, realtek_2spk_kcontrols.as_ptr(), realtek_2spk_kcontrols.len() as c_uint);
                if ret != 0 {
                    dev_err((*rtd).dev, b"fail to add rt1011 kcontrols, ret %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }

                ret = snd_soc_dapm_add_routes(dapm, speaker_map_lr.as_ptr(), speaker_map_lr.len() as c_uint);
                if ret != 0 {
                    dev_err((*rtd).dev, b"fail to add rt1011 routes, ret %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }

                return ret;
            }

            /*
             * register speaker widgets "WL Ext Spk" and "WR Ext Spk" to
             * keep backward compatible with cml devices
             */
            ret = snd_soc_dapm_new_controls(dapm, realtek_4spk_widgets.as_ptr(), num_codecs);
        }
        4 => {
            ret = snd_soc_dapm_new_controls(dapm, realtek_4spk_widgets.as_ptr(), num_codecs);
        }
        _ => {
            dev_err((*rtd).dev, b"rt1011: invalid num_codecs %d\n\0".as_ptr() as *const c_char, num_codecs);
            return -EINVAL;
        }
    }

    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add rt1011 widgets, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_add_card_controls(card, realtek_4spk_kcontrols.as_ptr(), num_codecs);
    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add rt1011 controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, rt1011_4spk_routes.as_ptr(), num_codecs);
    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add rt1011 routes, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn sof_rt1011_dai_link(dev: *mut device, link: *mut snd_soc_dai_link) {
    let num_codecs = get_num_codecs(&RT1011_ACPI_HID);

    (*link).codecs = rt1011_dai_link_components.as_mut_ptr();

    match num_codecs {
        2 | 4 => (*link).num_codecs = num_codecs,
        _ => dev_err(dev, b"rt1011: invalid num_codecs %d\n\0".as_ptr() as *const c_char, num_codecs),
    }

    (*link).init = Some(rt1011_init);
    (*link).ops = &rt1011_ops;
}
// EXPORT_SYMBOL_NS(sof_rt1011_dai_link, "SND_SOC_INTEL_SOF_REALTEK_COMMON");

#[no_mangle]
pub unsafe extern "C" fn sof_rt1011_codec_conf(dev: *mut device, card: *mut snd_soc_card) {
    let num_codecs = get_num_codecs(&RT1011_ACPI_HID);

    match num_codecs {
        2 => {
            if soc_intel_is_cml() {
                /*
                 * use name prefix 'WL' and 'WR' for speaker widgets to
                 * keep backward compatible with cml devices
                 */
                (*card).codec_conf = rt1011_4spk_codec_confs.as_mut_ptr();
            } else {
                (*card).codec_conf = rt1011_2spk_codec_confs.as_mut_ptr();
            }

            (*card).num_configs = num_codecs;
        }
        4 => {
            (*card).codec_conf = rt1011_4spk_codec_confs.as_mut_ptr();
            (*card).num_configs = rt1011_4spk_codec_confs.len() as c_uint;
        }
        _ => dev_err(dev, b"rt1011: invalid num_codecs %d\n\0".as_ptr() as *const c_char, num_codecs),
    }
}
// EXPORT_SYMBOL_NS(sof_rt1011_codec_conf, "SND_SOC_INTEL_SOF_REALTEK_COMMON");

/*
 * rt1015:  i2c mode driver for ALC1015 and ALC1015Q
 * rt1015p: auto-mode driver for ALC1015, ALC1015Q, and ALC1015Q-VB
 *
 * For stereo output, there are always two amplifiers on the board.
 * However, the ACPI implements only one device instance (UID=0) if they
 * are sharing the same enable pin. This is the case of rt1015p.
 */
static rt1015p_dapm_routes: [snd_soc_dapm_route; 2] = [
    /* speaker */
    snd_soc_dapm_route { sink: b"Left Spk\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Speaker\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Spk\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Speaker\0".as_ptr() as *const c_char },
];

static mut rt1015p_dai_link_components: [snd_soc_dai_link_component; 1] = unsafe {
    [snd_soc_dai_link_component { name: &RT1015P_DEV0_NAME, dai_name: &RT1015P_CODEC_DAI }]
};

unsafe extern "C" fn rt1015p_hw_params(
    _substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
) -> c_int {
    /* reserved for debugging purpose */

    0
}

static rt1015p_ops: snd_soc_ops = snd_soc_ops { hw_params: Some(rt1015p_hw_params) };

unsafe extern "C" fn rt1015p_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, realtek_2spk_widgets.as_ptr(), realtek_2spk_widgets.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add rt1015p widgets, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_add_card_controls(card, realtek_2spk_kcontrols.as_ptr(), realtek_2spk_kcontrols.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add rt1015p kcontrols, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, rt1015p_dapm_routes.as_ptr(), rt1015p_dapm_routes.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"Speaker map addition failed: %d\n\0".as_ptr() as *const c_char, ret);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn sof_rt1015p_dai_link(link: *mut snd_soc_dai_link) {
    (*link).codecs = rt1015p_dai_link_components.as_mut_ptr();
    (*link).num_codecs = rt1015p_dai_link_components.len() as c_uint;
    (*link).init = Some(rt1015p_init);
    (*link).ops = &rt1015p_ops;
}
// EXPORT_SYMBOL_NS(sof_rt1015p_dai_link, "SND_SOC_INTEL_SOF_REALTEK_COMMON");

#[no_mangle]
pub unsafe extern "C" fn sof_rt1015p_codec_conf(_card: *mut snd_soc_card) {}
// EXPORT_SYMBOL_NS(sof_rt1015p_codec_conf, "SND_SOC_INTEL_SOF_REALTEK_COMMON");

/*
 * RT1015 audio amplifier
 */

static rt1015_tdm_mask: [tdm_mask; 2] = [
    tdm_mask { tx: 0x0, rx: 0x1 },
    tdm_mask { tx: 0x0, rx: 0x2 },
];

unsafe extern "C" fn rt1015_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let dai_link = (*rtd).dai_link;
    let mut codec_dai: *mut snd_soc_dai;
    let mut i: c_int;
    let clk_freq: c_int;
    let mut ret: c_int = 0;

    clk_freq = sof_dai_get_bclk(rtd);

    if clk_freq <= 0 {
        dev_err((*rtd).dev, b"fail to get bclk freq, ret %d\n\0".as_ptr() as *const c_char, clk_freq);
        return -EINVAL;
    }

    i = 0;
    while i < (*dai_link).num_codecs as c_int {
        codec_dai = snd_soc_rtd_to_codec(rtd, i);
        ret = snd_soc_dai_set_pll(codec_dai, 0, RT1015_PLL_S_BCLK, clk_freq, params_rate(params) * 256);
        if ret != 0 {
            dev_err((*codec_dai).dev, b"fail to set pll, ret %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        ret = snd_soc_dai_set_sysclk(codec_dai, RT1015_SCLK_S_PLL, params_rate(params) * 256, SND_SOC_CLOCK_IN);
        if ret != 0 {
            dev_err((*codec_dai).dev, b"fail to set sysclk, ret %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }

        match (*dai_link).dai_fmt & SND_SOC_DAIFMT_FORMAT_MASK {
            x if x == SND_SOC_DAIFMT_DSP_A || x == SND_SOC_DAIFMT_DSP_B => {
                /* 4-slot TDM */
                ret = snd_soc_dai_set_tdm_slot(
                    codec_dai,
                    rt1015_tdm_mask[i as usize].tx,
                    rt1015_tdm_mask[i as usize].rx,
                    4,
                    params_width(params),
                );
                if ret < 0 {
                    dev_err((*codec_dai).dev, b"fail to set tdm slot, ret %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }
            }
            _ => {
                dev_dbg((*codec_dai).dev, b"codec is in I2S mode\n\0".as_ptr() as *const c_char);
            }
        }
        i += 1;
    }

    ret
}

static rt1015_ops: snd_soc_ops = snd_soc_ops { hw_params: Some(rt1015_hw_params) };

static mut rt1015_amp_conf: [snd_soc_codec_conf; 2] = unsafe {
    [
        snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: &RT1015_DEV0_NAME, dai_name: core::ptr::null() }, name_prefix: b"Left\0".as_ptr() as *const c_char },
        snd_soc_codec_conf { dlc: snd_soc_dai_link_component { name: &RT1015_DEV1_NAME, dai_name: core::ptr::null() }, name_prefix: b"Right\0".as_ptr() as *const c_char },
    ]
};

static mut rt1015_components: [snd_soc_dai_link_component; 2] = unsafe {
    [
        snd_soc_dai_link_component { name: &RT1015_DEV0_NAME, dai_name: &RT1015_CODEC_DAI },
        snd_soc_dai_link_component { name: &RT1015_DEV1_NAME, dai_name: &RT1015_CODEC_DAI },
    ]
};

unsafe extern "C" fn speaker_codec_init_lr(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let num_codecs = get_num_codecs(&RT1015_ACPI_HID);
    let mut ret: c_int;

    match num_codecs {
        2 => {
            ret = snd_soc_dapm_new_controls(dapm, realtek_2spk_widgets.as_ptr(), realtek_2spk_widgets.len() as c_uint);
            if ret != 0 {
                dev_err((*rtd).dev, b"fail to add rt1015 widgets, ret %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }

            ret = snd_soc_add_card_controls(card, realtek_2spk_kcontrols.as_ptr(), realtek_2spk_kcontrols.len() as c_uint);
            if ret != 0 {
                dev_err((*rtd).dev, b"fail to add rt1015 kcontrols, ret %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }

            ret = snd_soc_dapm_add_routes(dapm, speaker_map_lr.as_ptr(), speaker_map_lr.len() as c_uint);
            if ret != 0 {
                dev_err((*rtd).dev, b"fail to add rt1015 routes, ret %d\n\0".as_ptr() as *const c_char, ret);
                return ret;
            }
        }
        _ => {
            dev_err((*rtd).dev, b"rt1015: invalid num_codecs %d\n\0".as_ptr() as *const c_char, num_codecs);
            return -EINVAL;
        }
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn sof_rt1015_codec_conf(card: *mut snd_soc_card) {
    (*card).codec_conf = rt1015_amp_conf.as_mut_ptr();
    (*card).num_configs = rt1015_amp_conf.len() as c_uint;
}
// EXPORT_SYMBOL_NS(sof_rt1015_codec_conf, "SND_SOC_INTEL_SOF_REALTEK_COMMON");

#[no_mangle]
pub unsafe extern "C" fn sof_rt1015_dai_link(link: *mut snd_soc_dai_link) {
    (*link).codecs = rt1015_components.as_mut_ptr();
    (*link).num_codecs = rt1015_components.len() as c_uint;
    (*link).init = Some(speaker_codec_init_lr);
    (*link).ops = &rt1015_ops;
}
// EXPORT_SYMBOL_NS(sof_rt1015_dai_link, "SND_SOC_INTEL_SOF_REALTEK_COMMON");

/*
 * RT1308 audio amplifier
 */

static rt1308_dapm_routes: [snd_soc_dapm_route; 2] = [
    /* speaker */
    snd_soc_dapm_route { sink: b"Speakers\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"SPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speakers\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"SPOR\0".as_ptr() as *const c_char },
];

static mut rt1308_components: [snd_soc_dai_link_component; 1] = unsafe {
    [snd_soc_dai_link_component { name: &RT1308_DEV0_NAME, dai_name: &RT1308_CODEC_DAI }]
};

unsafe extern "C" fn rt1308_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, rt1308_dapm_widgets.as_ptr(), rt1308_dapm_widgets.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add dapm controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_add_card_controls(card, rt1308_kcontrols.as_ptr(), rt1308_kcontrols.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add card controls, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, rt1308_dapm_routes.as_ptr(), rt1308_dapm_routes.len() as c_uint);

    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add dapm routes, ret %d\n\0".as_ptr() as *const c_char, ret);
    }

    ret
}

unsafe extern "C" fn rt1308_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let card = (*rtd).card;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let clk_id: c_int;
    let clk_freq: c_int;
    let pll_out: c_int;
    let mut ret: c_int;

    clk_id = RT1308_PLL_S_MCLK;
    /* get the tplg configured mclk. */
    clk_freq = sof_dai_get_mclk(rtd);

    pll_out = params_rate(params) * 512;

    /* Set rt1308 pll */
    ret = snd_soc_dai_set_pll(codec_dai, 0, clk_id, clk_freq, pll_out);
    if ret < 0 {
        dev_err((*card).dev, b"Failed to set RT1308 PLL: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    /* Set rt1308 sysclk */
    ret = snd_soc_dai_set_sysclk(codec_dai, RT1308_FS_SYS_S_PLL, pll_out, SND_SOC_CLOCK_IN);
    if ret < 0 {
        dev_err((*card).dev, b"Failed to set RT1308 SYSCLK: %d\n\0".as_ptr() as *const c_char, ret);
    }

    ret
}

static rt1308_ops: snd_soc_ops = snd_soc_ops { hw_params: Some(rt1308_hw_params) };

#[no_mangle]
pub unsafe extern "C" fn sof_rt1308_dai_link(link: *mut snd_soc_dai_link) {
    (*link).codecs = rt1308_components.as_mut_ptr();
    (*link).num_codecs = rt1308_components.len() as c_uint;
    (*link).init = Some(rt1308_init);
    (*link).ops = &rt1308_ops;
}
// EXPORT_SYMBOL_NS(sof_rt1308_dai_link, "SND_SOC_INTEL_SOF_REALTEK_COMMON");

/*
 * 2-amp Configuration for RT1019
 */

static rt1019p_dapm_routes: [snd_soc_dapm_route; 2] = [
    /* speaker */
    snd_soc_dapm_route { sink: b"Left Spk\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Speaker\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Right Spk\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"Speaker\0".as_ptr() as *const c_char },
];

static mut rt1019p_components: [snd_soc_dai_link_component; 1] = unsafe {
    [snd_soc_dai_link_component { name: &RT1019P_DEV0_NAME, dai_name: &RT1019P_CODEC_DAI }]
};

unsafe extern "C" fn rt1019p_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card = (*rtd).card;
    let dapm = snd_soc_card_to_dapm(card);
    let mut ret: c_int;

    ret = snd_soc_dapm_new_controls(dapm, realtek_2spk_widgets.as_ptr(), realtek_2spk_widgets.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add rt1019p widgets, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_add_card_controls(card, realtek_2spk_kcontrols.as_ptr(), realtek_2spk_kcontrols.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"fail to add rt1019p kcontrols, ret %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(dapm, rt1019p_dapm_routes.as_ptr(), rt1019p_dapm_routes.len() as c_uint);
    if ret != 0 {
        dev_err((*rtd).dev, b"Speaker map addition failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn sof_rt1019p_dai_link(link: *mut snd_soc_dai_link) {
    (*link).codecs = rt1019p_components.as_mut_ptr();
    (*link).num_codecs = rt1019p_components.len() as c_uint;
    (*link).init = Some(rt1019p_init);
}
// EXPORT_SYMBOL_NS(sof_rt1019p_dai_link, "SND_SOC_INTEL_SOF_REALTEK_COMMON");

// MODULE_DESCRIPTION("ASoC Intel SOF Realtek helpers");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
