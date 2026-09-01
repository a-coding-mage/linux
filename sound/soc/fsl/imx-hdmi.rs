// SPDX-License-Identifier: GPL-2.0
// Copyright 2017-2020 NXP

// Translated from C implementation source. Kernel/ASoC symbols referenced here
// are supplied by the surrounding repository bindings.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{addr_of_mut, null, null_mut};

type u32 = c_uint;
type bool_ = bool;

const GFP_KERNEL: c_uint = 0;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: c_void;

    static SND_JACK_LINEOUT: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SND_SOC_CLOCK_OUT: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static FSL_SAI_CLK_MAST1: u32;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_component_set_jack(
        component: *mut snd_soc_component,
        jack: *mut snd_soc_jack,
        data: *mut c_void,
    ) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool_;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int)
        -> *mut device_node;
    fn of_node_name_eq(np: *mut device_node, name: *const c_char) -> bool_;
    fn of_device_is_compatible(np: *mut device_node, compat: *const c_char) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn of_node_put(node: *mut device_node);
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
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
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
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
pub struct snd_soc_dai_link_component {
    pub of_node: *mut device_node,
    pub dai_name: *const c_char,
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_uchar,
    pub mask: c_uint,
    pub on_val: c_uint,
    pub off_val: c_uint,
    pub kcontrol_news: *const c_void,
    pub num_kcontrols: c_int,
    pub event: *mut c_void,
    pub event_flags: c_uchar,
    pub subseq: c_uchar,
}

type c_uchar = u8;

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub stream_name: *const c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_uint,
    pub ops: *const snd_soc_ops,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub dai_fmt: c_uint,
    pub playback_only: bool_,
    pub capture_only: bool_,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const c_void,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

/**
 * struct cpu_priv - CPU private data
 * @sysclk_id: SYSCLK ids for set_sysclk()
 * @slot_width: Slot width of each frame
 *
 * Note: [1] for tx and [0] for rx
 */
#[repr(C)]
pub struct cpu_priv {
    pub sysclk_id: [u32; 2],
    pub slot_width: u32,
}

#[repr(C)]
pub struct imx_hdmi_data {
    pub dai: snd_soc_dai_link,
    pub card: snd_soc_card,
    pub hdmi_jack: snd_soc_jack,
    pub hdmi_jack_pin: snd_soc_jack_pin,
    pub cpu_priv: cpu_priv,
    pub dai_fmt: u32,
}

unsafe extern "C" fn imx_hdmi_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let data: *mut imx_hdmi_data = snd_soc_card_get_drvdata((*rtd).card) as *mut imx_hdmi_data;
    let tx: bool_ = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let cpu_dai: *mut snd_soc_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let card: *mut snd_soc_card = (*rtd).card;
    let dev: *mut device = (*card).dev;
    let slot_width: u32 = (*data).cpu_priv.slot_width;
    let mut ret: c_int;

    /* MCLK always is (256 or 192) * rate. */
    ret = snd_soc_dai_set_sysclk(
        cpu_dai,
        (*data).cpu_priv.sysclk_id[tx as usize] as c_int,
        8u32.wrapping_mul(slot_width).wrapping_mul(params_rate(params)),
        if tx { SND_SOC_CLOCK_OUT } else { SND_SOC_CLOCK_IN },
    );
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(dev, b"failed to set cpu sysclk: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    ret = snd_soc_dai_set_tdm_slot(cpu_dai, 0, 0, 2, slot_width as c_int);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(
            dev,
            b"failed to set cpu dai tdm slot: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    return 0;
}

static imx_hdmi_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(imx_hdmi_hw_params),
};

// SND_SOC_DAPM_LINE("HDMI Jack", NULL)
static imx_hdmi_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget {
    id: 0,
    name: b"HDMI Jack\0".as_ptr() as *const c_char,
    reg: 0,
    shift: 0,
    mask: 0,
    on_val: 0,
    off_val: 0,
    kcontrol_news: null(),
    num_kcontrols: 0,
    event: null_mut(),
    event_flags: 0,
    subseq: 0,
}];

unsafe extern "C" fn imx_hdmi_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let component: *mut snd_soc_component = (*codec_dai).component;
    let data: *mut imx_hdmi_data = snd_soc_card_get_drvdata(card) as *mut imx_hdmi_data;
    let mut ret: c_int;

    (*data).hdmi_jack_pin.pin = b"HDMI Jack\0".as_ptr() as *const c_char;
    (*data).hdmi_jack_pin.mask = SND_JACK_LINEOUT as c_int;
    /* enable jack detection */
    ret = snd_soc_card_jack_new_pins(
        card,
        b"HDMI Jack\0".as_ptr() as *const c_char,
        SND_JACK_LINEOUT as c_int,
        addr_of_mut!((*data).hdmi_jack),
        addr_of_mut!((*data).hdmi_jack_pin),
        1,
    );
    if ret != 0 {
        dev_err(
            (*card).dev,
            b"Can't new HDMI Jack %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    ret = snd_soc_component_set_jack(component, addr_of_mut!((*data).hdmi_jack), null_mut());
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(
            (*card).dev,
            b"Can't set HDMI Jack %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    return 0;
}

unsafe extern "C" fn imx_hdmi_probe(pdev: *mut platform_device) -> c_int {
    let np: *mut device_node = (*pdev).dev.of_node;
    let hdmi_out: bool_ = of_property_read_bool(np, b"hdmi-out\0".as_ptr() as *const c_char);
    let hdmi_in: bool_ = of_property_read_bool(np, b"hdmi-in\0".as_ptr() as *const c_char);
    let mut dlc: *mut snd_soc_dai_link_component;
    let mut cpu_np: *mut device_node;
    let mut data: *mut imx_hdmi_data;
    let mut ret: c_int;

    dlc = devm_kzalloc(
        addr_of_mut!((*pdev).dev),
        3usize.wrapping_mul(size_of::<snd_soc_dai_link_component>()),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if dlc.is_null() {
        return -ENOMEM;
    }

    cpu_np = of_parse_phandle(np, b"audio-cpu\0".as_ptr() as *const c_char, 0);
    if cpu_np.is_null() {
        dev_err(
            addr_of_mut!((*pdev).dev),
            b"cpu dai phandle missing or invalid\n\0".as_ptr() as *const c_char,
        );
        ret = -EINVAL;
        goto_fail(cpu_np);
        return ret;
    }

    data = devm_kzalloc(
        addr_of_mut!((*pdev).dev),
        size_of::<imx_hdmi_data>(),
        GFP_KERNEL,
    ) as *mut imx_hdmi_data;
    if data.is_null() {
        ret = -ENOMEM;
        goto_fail(cpu_np);
        return ret;
    }

    (*data).dai.cpus = dlc.add(0);
    (*data).dai.num_cpus = 1;
    (*data).dai.platforms = dlc.add(1);
    (*data).dai.num_platforms = 1;
    (*data).dai.codecs = dlc.add(2);
    (*data).dai.num_codecs = 1;

    (*data).dai.name = b"i.MX HDMI\0".as_ptr() as *const c_char;
    (*data).dai.stream_name = b"i.MX HDMI\0".as_ptr() as *const c_char;
    (*(*data).dai.cpus).of_node = cpu_np;
    (*(*data).dai.platforms).of_node = cpu_np;
    (*data).dai.ops = &imx_hdmi_ops;
    (*data).dai.playback_only = true;
    (*data).dai.capture_only = false;
    (*data).dai.init = Some(imx_hdmi_init);

    if of_node_name_eq(cpu_np, b"sai\0".as_ptr() as *const c_char) {
        (*data).cpu_priv.sysclk_id[1] = FSL_SAI_CLK_MAST1;
        (*data).cpu_priv.sysclk_id[0] = FSL_SAI_CLK_MAST1;
    }

    if of_device_is_compatible(np, b"fsl,imx-audio-sii902x\0".as_ptr() as *const c_char) != 0 {
        (*data).dai_fmt = SND_SOC_DAIFMT_LEFT_J;
        (*data).cpu_priv.slot_width = 24;
    } else {
        (*data).dai_fmt = SND_SOC_DAIFMT_I2S;
        (*data).cpu_priv.slot_width = 32;
    }

    if (hdmi_out && hdmi_in) || (!hdmi_out && !hdmi_in) {
        dev_err(
            addr_of_mut!((*pdev).dev),
            b"Invalid HDMI DAI link\n\0".as_ptr() as *const c_char,
        );
        ret = -EINVAL;
        goto_fail(cpu_np);
        return ret;
    }

    if hdmi_out {
        (*data).dai.playback_only = true;
        (*data).dai.capture_only = false;
        (*(*data).dai.codecs).dai_name = b"i2s-hifi\0".as_ptr() as *const c_char;
        (*(*data).dai.codecs).name = b"hdmi-audio-codec.1\0".as_ptr() as *const c_char;
        (*data).dai.dai_fmt =
            (*data).dai_fmt | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    }

    if hdmi_in {
        (*data).dai.playback_only = false;
        (*data).dai.capture_only = true;
        (*(*data).dai.codecs).dai_name = b"i2s-hifi\0".as_ptr() as *const c_char;
        (*(*data).dai.codecs).name = b"hdmi-audio-codec.2\0".as_ptr() as *const c_char;
        (*data).dai.dai_fmt =
            (*data).dai_fmt | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;
    }

    (*data).card.dapm_widgets = imx_hdmi_widgets.as_ptr();
    (*data).card.num_dapm_widgets = imx_hdmi_widgets.len() as c_int;
    (*data).card.dev = addr_of_mut!((*pdev).dev);
    (*data).card.owner = THIS_MODULE;
    ret = snd_soc_of_parse_card_name(addr_of_mut!((*data).card), b"model\0".as_ptr() as *const c_char);
    if ret != 0 {
        goto_fail(cpu_np);
        return ret;
    }

    (*data).card.num_links = 1;
    (*data).card.dai_link = addr_of_mut!((*data).dai);

    snd_soc_card_set_drvdata(addr_of_mut!((*data).card), data as *mut c_void);
    ret = devm_snd_soc_register_card(addr_of_mut!((*pdev).dev), addr_of_mut!((*data).card));
    if ret != 0 {
        dev_err_probe(
            addr_of_mut!((*pdev).dev),
            ret,
            b"snd_soc_register_card failed\n\0".as_ptr() as *const c_char,
        );
        goto_fail(cpu_np);
        return ret;
    }

    goto_fail(cpu_np);
    return ret;
}

unsafe fn goto_fail(cpu_np: *mut device_node) {
    of_node_put(cpu_np);
}

static imx_hdmi_dt_ids: [of_device_id; 3] = [
    of_device_id {
        compatible: b"fsl,imx-audio-hdmi\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: b"fsl,imx-audio-sii902x\0".as_ptr() as *const c_char,
    },
    of_device_id {
        /* sentinel */
        compatible: null(),
    },
];
// MODULE_DEVICE_TABLE(of, imx_hdmi_dt_ids);

static mut imx_hdmi_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"imx-hdmi\0".as_ptr() as *const c_char,
        pm: unsafe { &snd_soc_pm_ops as *const c_void },
        of_match_table: imx_hdmi_dt_ids.as_ptr(),
    },
    probe: Some(imx_hdmi_probe),
};
// module_platform_driver(imx_hdmi_driver);

// MODULE_AUTHOR("Freescale Semiconductor, Inc.");
// MODULE_DESCRIPTION("Freescale i.MX hdmi audio ASoC machine driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:imx-hdmi");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
