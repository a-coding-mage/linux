// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017-2020 NXP

// Rust translation of soc/fsl/imx-rpmsg.c.
// C include dependencies:
// linux/module.h, linux/of_platform.h, linux/of_reserved_mem.h, linux/i2c.h,
// linux/slab.h, linux/clk.h, sound/soc.h, sound/jack.h, sound/control.h,
// sound/pcm_params.h, sound/soc-dapm.h, sound/simple_card_utils.h,
// "imx-pcm-rpmsg.h"

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type snd_pcm_format_t = c_int;

const GFP_KERNEL: c_uint = 0;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;

const SNDRV_PCM_FORMAT_DSD_U8: snd_pcm_format_t = 48;
const SNDRV_PCM_FORMAT_DSD_U16_LE: snd_pcm_format_t = 49;
const SNDRV_PCM_FORMAT_DSD_U16_BE: snd_pcm_format_t = 50;
const SNDRV_PCM_FORMAT_DSD_U32_LE: snd_pcm_format_t = 51;
const SNDRV_PCM_FORMAT_DSD_U32_BE: snd_pcm_format_t = 52;

const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_PDM: c_uint = 9;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0 << 8;
const SND_SOC_DAIFMT_CBC_CFC: c_uint = 1 << 12;
const SND_SOC_CLOCK_IN: c_int = 0;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub complete: Option<unsafe extern "C" fn(*mut device)>,
    pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub freeze: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub thaw: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub poweroff: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub restore: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub pm: *mut dev_pm_ops,
}

#[repr(C)]
pub struct device {
    pub driver: *mut device_driver,
    pub of_node: *mut device_node,
    pub platform_data: *mut c_char,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct clk {
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
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub startup: *const c_void,
    pub shutdown: *const c_void,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int,
    >,
}

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
    pub dai_fmt: c_uint,
    pub ops: *const snd_soc_ops,
    pub ignore_pmdown_time: c_uint,
    pub playback_only: bool_,
    pub capture_only: bool_,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub driver_name: *const c_char,
    pub num_links: c_uint,
    pub dai_link: *mut snd_soc_dai_link,
    pub rtd_list: list_head,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub list: list_head,
    pub card: *mut snd_soc_card,
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_uchar_compat,
    pub mask: c_uint,
    pub on_val: c_uint,
    pub off_val: c_uint,
}

type c_uchar_compat = u8;

#[repr(C)]
pub struct simple_util_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_phandle_args {
    pub np: *mut device_node,
    pub args_count: c_int,
    pub args: [c_uint; 16],
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct imx_rpmsg {
    dai: snd_soc_dai_link,
    card: snd_soc_card,
    sysclk: c_ulong,
    lpa: bool_,
    hp_jack: simple_util_jack,
}

static mut lpa_pm: dev_pm_ops = dev_pm_ops {
    prepare: None,
    complete: None,
    suspend: None,
    resume: None,
    freeze: None,
    thaw: None,
    poweroff: None,
    restore: None,
};

const fn dapm_widget(_kind: c_int, name: *const c_char) -> snd_soc_dapm_widget {
    snd_soc_dapm_widget {
        id: _kind,
        name,
        reg: -1,
        shift: 0,
        mask: 0,
        on_val: 0,
        off_val: 0,
    }
}

static imx_rpmsg_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    dapm_widget(0, c"Headphone Jack".as_ptr()),
    dapm_widget(0, c"Ext Spk".as_ptr()),
    dapm_widget(0, c"Mic Jack".as_ptr()),
    dapm_widget(0, c"Main MIC".as_ptr()),
];

unsafe extern "C" fn imx_rpmsg_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let format: snd_pcm_format_t = params_format(params);
    let dev = (*(*rtd).card).dev;
    let mut fmt: c_uint = (*(*rtd).dai_link).dai_fmt;
    let mut format_is_dsd = false;
    let mut ret: c_int;

    match format {
        SNDRV_PCM_FORMAT_DSD_U8
        | SNDRV_PCM_FORMAT_DSD_U16_LE
        | SNDRV_PCM_FORMAT_DSD_U16_BE
        | SNDRV_PCM_FORMAT_DSD_U32_LE
        | SNDRV_PCM_FORMAT_DSD_U32_BE => {
            format_is_dsd = true;
        }
        _ => {
            format_is_dsd = false;
        }
    }

    if format_is_dsd {
        fmt = ((*(*rtd).dai_link).dai_fmt & !SND_SOC_DAIFMT_FORMAT_MASK) | SND_SOC_DAIFMT_PDM;
    }

    ret = snd_soc_dai_set_fmt(cpu_dai, fmt);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(dev, c"failed to set cpu dai fmt: %d\n".as_ptr(), ret);
        return ret;
    }
    ret = snd_soc_dai_set_fmt(codec_dai, fmt);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(dev, c"failed to set codec dai fmt: %d\n".as_ptr(), ret);
        return ret;
    }

    0
}

static imx_rpmsg_ops: snd_soc_ops = snd_soc_ops {
    startup: ptr::null(),
    shutdown: ptr::null(),
    hw_params: Some(imx_rpmsg_hw_params),
};

unsafe extern "C" fn imx_rpmsg_late_probe(card: *mut snd_soc_card) -> c_int {
    let data = snd_soc_card_get_drvdata(card) as *mut imx_rpmsg;
    let rtd = list_first_entry(&mut (*card).rtd_list as *mut list_head) as *mut snd_soc_pcm_runtime;
    let codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    let dev = (*card).dev;
    let mut ret: c_int;

    if of_property_present((*(*card).dev).of_node, c"hp-det-gpios".as_ptr()) {
        ret = simple_util_init_jack(
            card,
            &mut (*data).hp_jack,
            1,
            ptr::null_mut(),
            c"Headphone Jack".as_ptr(),
        );
        if ret != 0 {
            dev_err(dev, c"failed to init hp jack\n".as_ptr());
            return ret;
        }
    }

    if (*data).lpa {
        let mut codec_np: *mut device_node;
        let codec_drv: *mut device_driver;
        let mut codec_dev: *mut device = ptr::null_mut();

        codec_np = (*(*data).dai.codecs).of_node;
        if !codec_np.is_null() {
            let codec_pdev: *mut platform_device;
            let codec_i2c: *mut i2c_client;

            codec_i2c = of_find_i2c_device_by_node(codec_np);
            if !codec_i2c.is_null() {
                codec_dev = &mut (*codec_i2c).dev;
            }
            if codec_dev.is_null() {
                codec_pdev = of_find_device_by_node(codec_np);
                if !codec_pdev.is_null() {
                    codec_dev = &mut (*codec_pdev).dev;
                }
            }
        }
        if !codec_dev.is_null() {
            codec_drv = (*codec_dev).driver;
            if !(*codec_drv).pm.is_null() {
                ptr::copy_nonoverlapping((*codec_drv).pm, &mut lpa_pm, 1);
                lpa_pm.suspend = None;
                lpa_pm.resume = None;
                lpa_pm.freeze = None;
                lpa_pm.thaw = None;
                lpa_pm.poweroff = None;
                lpa_pm.restore = None;
                (*codec_drv).pm = &mut lpa_pm;
            }
            put_device(codec_dev);
        }
    }

    if (*data).sysclk == 0 {
        return 0;
    }

    ret = snd_soc_dai_set_sysclk(codec_dai, 0, (*data).sysclk, SND_SOC_CLOCK_IN);
    if ret != 0 && ret != -ENOTSUPP {
        dev_err(dev, c"failed to set sysclk in %s\n".as_ptr(), c"imx_rpmsg_late_probe".as_ptr());
        return ret;
    }

    0
}

unsafe extern "C" fn imx_rpmsg_probe(pdev: *mut platform_device) -> c_int {
    let dlc: *mut snd_soc_dai_link_component;
    let cpu_dai: *mut snd_soc_dai;
    let mut np: *mut device_node = ptr::null_mut();
    let mut args: of_phandle_args = core::mem::zeroed();
    let mut platform_name: *const c_char = ptr::null();
    let data: *mut imx_rpmsg;
    let mut ret: c_int = 0;

    dlc = devm_kzalloc(
        &mut (*pdev).dev,
        (3 * size_of::<snd_soc_dai_link_component>()) as c_ulong,
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if dlc.is_null() {
        return -ENOMEM;
    }

    data = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<imx_rpmsg>() as c_ulong,
        GFP_KERNEL,
    ) as *mut imx_rpmsg;
    if data.is_null() {
        ret = -ENOMEM;
        return fail(pdev, ret);
    }

    (*data).dai.cpus = dlc.add(0);
    (*data).dai.num_cpus = 1;
    (*data).dai.platforms = dlc.add(1);
    (*data).dai.num_platforms = 1;
    (*data).dai.codecs = dlc.add(2);
    (*data).dai.num_codecs = 1;

    (*data).dai.name = c"rpmsg hifi".as_ptr();
    (*data).dai.stream_name = c"rpmsg hifi".as_ptr();
    (*data).dai.dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBC_CFC;
    (*data).dai.ops = &imx_rpmsg_ops;

    /*
     * i.MX rpmsg sound cards work on codec slave mode. MCLK will be
     * disabled by CPU DAI driver in hw_free(). Some codec requires MCLK
     * present at power up/down sequence. So need to set ignore_pmdown_time
     * to power down codec immediately before MCLK is turned off.
     */
    (*data).dai.ignore_pmdown_time = 1;

    (*(*data).dai.cpus).dai_name = (*pdev).dev.platform_data;
    cpu_dai = snd_soc_find_dai_with_mutex((*data).dai.cpus);
    if cpu_dai.is_null() {
        ret = -EPROBE_DEFER;
        return fail(pdev, ret);
    }
    np = (*(*cpu_dai).dev).of_node;
    if np.is_null() {
        dev_err(
            &mut (*pdev).dev,
            c"failed to parse CPU DAI device node\n".as_ptr(),
        );
        ret = -ENODEV;
        return fail(pdev, ret);
    }

    ret = of_reserved_mem_device_init_by_idx(&mut (*pdev).dev, np, 0);
    if ret != 0 {
        dev_warn(&mut (*pdev).dev, c"no reserved DMA memory\n".as_ptr());
    }

    /* Optional codec node */
    ret = of_parse_phandle_with_fixed_args(
        np,
        c"audio-codec".as_ptr(),
        0,
        0,
        &mut args,
    );
    if ret != 0 {
        *(*data).dai.codecs = snd_soc_dummy_dlc;
    } else {
        let clk: *mut clk;

        ret = snd_soc_get_dlc(&mut args, (*data).dai.codecs);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, c"Unable to get codec_dai_name\n".as_ptr());
            return fail(pdev, ret);
        }

        clk = devm_get_clk_from_child(&mut (*pdev).dev, args.np, ptr::null());
        if !IS_ERR(clk as *const c_void) {
            (*data).sysclk = clk_get_rate(clk);
        }
    }

    if !of_property_read_string(
        np,
        c"fsl,rpmsg-channel-name".as_ptr(),
        &mut platform_name,
    ) {
        (*(*data).dai.platforms).name = platform_name;
    } else {
        (*(*data).dai.platforms).name = c"rpmsg-audio-channel".as_ptr();
    }
    (*data).dai.playback_only = true;
    (*data).dai.capture_only = true;
    (*data).card.num_links = 1;
    (*data).card.dai_link = &mut (*data).dai;

    if of_property_read_bool(np, c"fsl,rpmsg-out".as_ptr()) {
        (*data).dai.capture_only = false;
    }

    if of_property_read_bool(np, c"fsl,rpmsg-in".as_ptr()) {
        (*data).dai.playback_only = false;
    }

    if (*data).dai.playback_only && (*data).dai.capture_only {
        dev_err(&mut (*pdev).dev, c"no enabled rpmsg DAI link\n".as_ptr());
        ret = -EINVAL;
        return fail(pdev, ret);
    }

    if of_property_read_bool(np, c"fsl,enable-lpa".as_ptr()) {
        (*data).lpa = true;
    }

    (*data).card.dev = &mut (*pdev).dev;
    (*data).card.owner = THIS_MODULE;
    (*data).card.dapm_widgets = imx_rpmsg_dapm_widgets.as_ptr();
    (*data).card.num_dapm_widgets = imx_rpmsg_dapm_widgets.len() as c_uint;
    (*data).card.late_probe = Some(imx_rpmsg_late_probe);
    (*data).card.driver_name = c"imx-audio-rpmsg".as_ptr();
    /*
     * Inoder to use common api to get card name and audio routing.
     * Use parent of_node for this device, revert it after finishing using
     */
    (*(*data).card.dev).of_node = np;

    ret = snd_soc_of_parse_card_name(&mut (*data).card, c"model".as_ptr());
    if ret != 0 {
        return fail(pdev, ret);
    }

    if of_property_present(np, c"audio-routing".as_ptr()) {
        ret = snd_soc_of_parse_audio_routing(&mut (*data).card, c"audio-routing".as_ptr());
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                c"failed to parse audio-routing: %d\n".as_ptr(),
                ret,
            );
            return fail(pdev, ret);
        }
    }

    if (*data).lpa && of_property_present(np, c"ignore-suspend-widgets".as_ptr()) {
        ret = snd_soc_of_parse_ignore_suspend_widgets(
            &mut (*data).card,
            c"ignore-suspend-widgets".as_ptr(),
        );
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                c"failed to parse ignore-suspend-widgets: %d\n".as_ptr(),
                ret,
            );
            return fail(pdev, ret);
        }
    }

    platform_set_drvdata(pdev, &mut (*data).card as *mut snd_soc_card as *mut c_void);
    snd_soc_card_set_drvdata(&mut (*data).card, data as *mut c_void);
    ret = devm_snd_soc_register_card(&mut (*pdev).dev, &mut (*data).card);
    if ret != 0 {
        dev_err_probe(
            &mut (*pdev).dev,
            ret,
            c"snd_soc_register_card failed\n".as_ptr(),
        );
        return fail(pdev, ret);
    }

    fail(pdev, ret)
}

unsafe fn fail(pdev: *mut platform_device, ret: c_int) -> c_int {
    (*pdev).dev.of_node = ptr::null_mut();
    ret
}

static mut imx_rpmsg_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: c"imx-audio-rpmsg".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(imx_rpmsg_probe),
};

// module_platform_driver(imx_rpmsg_driver);
// MODULE_DESCRIPTION("Freescale SoC Audio RPMSG Machine Driver");
// MODULE_AUTHOR("Shengjiu Wang <shengjiu.wang@nxp.com>");
// MODULE_ALIAS("platform:imx-audio-rpmsg");
// MODULE_LICENSE("GPL v2");

unsafe fn list_first_entry(head: *mut list_head) -> *mut c_void {
    (*head).next as *mut c_void
}

extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static snd_soc_pm_ops: dev_pm_ops;
    static snd_soc_dummy_dlc: snd_soc_dai_link_component;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_ulong,
        dir: c_int,
    ) -> c_int;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn simple_util_init_jack(
        card: *mut snd_soc_card,
        jack: *mut simple_util_jack,
        pins: c_int,
        pin: *mut c_void,
        prefix: *const c_char,
    ) -> c_int;
    fn snd_soc_find_dai_with_mutex(
        dlc: *mut snd_soc_dai_link_component,
    ) -> *mut snd_soc_dai;
    fn snd_soc_get_dlc(
        args: *mut of_phandle_args,
        dlc: *mut snd_soc_dai_link_component,
    ) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_ignore_suspend_widgets(
        card: *mut snd_soc_card,
        propname: *const c_char,
    ) -> c_int;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;

    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool_;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool_;
    fn of_property_read_string(
        np: *mut device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> bool_;
    fn of_parse_phandle_with_fixed_args(
        np: *mut device_node,
        list_name: *const c_char,
        cell_count: c_int,
        index: c_int,
        out_args: *mut of_phandle_args,
    ) -> c_int;
    fn of_reserved_mem_device_init_by_idx(
        dev: *mut device,
        np: *mut device_node,
        idx: c_int,
    ) -> c_int;
    fn of_find_i2c_device_by_node(np: *mut device_node) -> *mut i2c_client;
    fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;

    fn devm_kzalloc(dev: *mut device, size: c_ulong, flags: c_uint) -> *mut c_void;
    fn devm_get_clk_from_child(
        dev: *mut device,
        np: *mut device_node,
        con_id: *const c_char,
    ) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn put_device(dev: *mut device);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
