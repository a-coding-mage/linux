// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2012 Freescale Semiconductor, Inc.
// Copyright 2012 Linaro Ltd.

// C dependencies translated as external Rust dependencies:
// linux/module.h, linux/of.h, linux/of_platform.h, linux/i2c.h, linux/clk.h,
// sound/soc.h, ../codecs/sgtl5000.h, imx-audmux.h

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const DAI_NAME_SIZE: usize = 32;

const EINVAL: c_int = 22;
const EPROBE_DEFER: c_int = 517;
const ENOMEM: c_int = 12;

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
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
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
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
    pub of_node: *mut device_node,
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
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
    pub dai_fmt: c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub num_links: c_int,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
struct imx_sgtl5000_data {
    dai: snd_soc_dai_link,
    card: snd_soc_card,
    codec_dai_name: [c_char; DAI_NAME_SIZE],
    platform_name: [c_char; DAI_NAME_SIZE],
    codec_clk: *mut clk,
    clk_frequency: c_uint,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;
    static GFP_KERNEL: c_uint;
    static SGTL5000_SYSCLK: c_int;
    static SND_SOC_CLOCK_IN: c_int;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static IMX_AUDMUX_V2_PTCR_SYN: c_uint;
    static IMX_AUDMUX_V2_PTCR_TFSDIR: c_uint;
    static IMX_AUDMUX_V2_PTCR_TCLKDIR: c_uint;

    fn IMX_AUDMUX_V2_PTCR_TFSEL(port: c_int) -> c_uint;
    fn IMX_AUDMUX_V2_PTCR_TCSEL(port: c_int) -> c_uint;
    fn IMX_AUDMUX_V2_PDCR_RXDSEL(port: c_int) -> c_uint;

    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut c_void);
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;

    fn of_property_read_u32(
        np: *mut device_node,
        propname: *const c_char,
        out_value: *mut c_int,
    ) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;
    fn of_find_i2c_device_by_node(np: *mut device_node) -> *mut i2c_client;
    fn of_node_put(np: *mut device_node);

    fn put_device(dev: *mut device);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_put(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;

    fn imx_audmux_v2_configure_port(port: c_int, ptcr: c_uint, pdcr: c_uint) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
}

unsafe extern "C" fn imx_sgtl5000_dai_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let data = snd_soc_card_get_drvdata((*rtd).card) as *mut imx_sgtl5000_data;
    let dev = (*(*rtd).card).dev;
    let mut ret: c_int;

    ret = snd_soc_dai_set_sysclk(
        snd_soc_rtd_to_codec(rtd, 0),
        SGTL5000_SYSCLK,
        (*data).clk_frequency,
        SND_SOC_CLOCK_IN,
    );
    if ret != 0 {
        dev_err(
            dev,
            c"could not set codec driver clock params\n".as_ptr(),
        );
        return ret;
    }

    0
}

// static const struct snd_soc_dapm_widget imx_sgtl5000_dapm_widgets[] = {
//     SND_SOC_DAPM_MIC("Mic Jack", NULL),
//     SND_SOC_DAPM_LINE("Line In Jack", NULL),
//     SND_SOC_DAPM_HP("Headphone Jack", NULL),
//     SND_SOC_DAPM_SPK("Line Out Jack", NULL),
//     SND_SOC_DAPM_SPK("Ext Spk", NULL),
// };
unsafe extern "C" {
    static imx_sgtl5000_dapm_widgets: [snd_soc_dapm_widget; 5];
}

unsafe extern "C" fn imx_sgtl5000_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut ssi_np: *mut device_node;
    let mut codec_np: *mut device_node;
    let mut ssi_pdev: *mut platform_device;
    let mut codec_dev: *mut i2c_client;
    let mut data: *mut imx_sgtl5000_data = ptr::null_mut();
    let mut comp: *mut snd_soc_dai_link_component;
    let mut int_port: c_int = 0;
    let mut ext_port: c_int = 0;
    let mut ret: c_int;

    ret = of_property_read_u32(np, c"mux-int-port".as_ptr(), &mut int_port);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            c"mux-int-port missing or invalid\n".as_ptr(),
        );
        return ret;
    }
    ret = of_property_read_u32(np, c"mux-ext-port".as_ptr(), &mut ext_port);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            c"mux-ext-port missing or invalid\n".as_ptr(),
        );
        return ret;
    }

    /*
     * The port numbering in the hardware manual starts at 1, while
     * the audmux API expects it starts at 0.
     */
    int_port -= 1;
    ext_port -= 1;
    ret = imx_audmux_v2_configure_port(
        int_port,
        IMX_AUDMUX_V2_PTCR_SYN
            | IMX_AUDMUX_V2_PTCR_TFSEL(ext_port)
            | IMX_AUDMUX_V2_PTCR_TCSEL(ext_port)
            | IMX_AUDMUX_V2_PTCR_TFSDIR
            | IMX_AUDMUX_V2_PTCR_TCLKDIR,
        IMX_AUDMUX_V2_PDCR_RXDSEL(ext_port),
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            c"audmux internal port setup failed\n".as_ptr(),
        );
        return ret;
    }
    ret = imx_audmux_v2_configure_port(
        ext_port,
        IMX_AUDMUX_V2_PTCR_SYN,
        IMX_AUDMUX_V2_PDCR_RXDSEL(int_port),
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            c"audmux external port setup failed\n".as_ptr(),
        );
        return ret;
    }

    ssi_np = of_parse_phandle((*pdev).dev.of_node, c"ssi-controller".as_ptr(), 0);
    codec_np = of_parse_phandle((*pdev).dev.of_node, c"audio-codec".as_ptr(), 0);
    if ssi_np.is_null() || codec_np.is_null() {
        dev_err(&mut (*pdev).dev, c"phandle missing or invalid\n".as_ptr());
        ret = -EINVAL;
        goto_fail(data, ssi_np, codec_np, ret);
        return ret;
    }

    ssi_pdev = of_find_device_by_node(ssi_np);
    if ssi_pdev.is_null() {
        dev_dbg(
            &mut (*pdev).dev,
            c"failed to find SSI platform device\n".as_ptr(),
        );
        ret = -EPROBE_DEFER;
        goto_fail(data, ssi_np, codec_np, ret);
        return ret;
    }
    put_device(&mut (*ssi_pdev).dev);
    codec_dev = of_find_i2c_device_by_node(codec_np);
    if codec_dev.is_null() {
        dev_dbg(
            &mut (*pdev).dev,
            c"failed to find codec platform device\n".as_ptr(),
        );
        ret = -EPROBE_DEFER;
        goto_fail(data, ssi_np, codec_np, ret);
        return ret;
    }

    data = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<imx_sgtl5000_data>(),
        GFP_KERNEL,
    ) as *mut imx_sgtl5000_data;
    if data.is_null() {
        ret = -ENOMEM;
        goto_put_device(codec_dev, data, ssi_np, codec_np, ret);
        return ret;
    }

    comp = devm_kzalloc(
        &mut (*pdev).dev,
        3 * size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if comp.is_null() {
        ret = -ENOMEM;
        goto_put_device(codec_dev, data, ssi_np, codec_np, ret);
        return ret;
    }

    (*data).codec_clk = clk_get(&mut (*codec_dev).dev, ptr::null());
    if IS_ERR((*data).codec_clk as *const c_void) {
        ret = PTR_ERR((*data).codec_clk as *const c_void);
        goto_put_device(codec_dev, data, ssi_np, codec_np, ret);
        return ret;
    }

    (*data).clk_frequency = clk_get_rate((*data).codec_clk) as c_uint;

    (*data).dai.cpus = comp.add(0);
    (*data).dai.codecs = comp.add(1);
    (*data).dai.platforms = comp.add(2);

    (*data).dai.num_cpus = 1;
    (*data).dai.num_codecs = 1;
    (*data).dai.num_platforms = 1;

    (*data).dai.name = c"HiFi".as_ptr();
    (*data).dai.stream_name = c"HiFi".as_ptr();
    (*(*data).dai.codecs).dai_name = c"sgtl5000".as_ptr();
    (*(*data).dai.codecs).of_node = codec_np;
    (*(*data).dai.cpus).of_node = ssi_np;
    (*(*data).dai.platforms).of_node = ssi_np;
    (*data).dai.init = Some(imx_sgtl5000_dai_init);
    (*data).dai.dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;

    (*data).card.dev = &mut (*pdev).dev;
    ret = snd_soc_of_parse_card_name(&mut (*data).card, c"model".as_ptr());
    if ret != 0 {
        goto_put_device(codec_dev, data, ssi_np, codec_np, ret);
        return ret;
    }
    ret = snd_soc_of_parse_audio_routing(&mut (*data).card, c"audio-routing".as_ptr());
    if ret != 0 {
        goto_put_device(codec_dev, data, ssi_np, codec_np, ret);
        return ret;
    }
    (*data).card.num_links = 1;
    (*data).card.owner = THIS_MODULE;
    (*data).card.dai_link = &mut (*data).dai;
    (*data).card.dapm_widgets = imx_sgtl5000_dapm_widgets.as_ptr();
    (*data).card.num_dapm_widgets = imx_sgtl5000_dapm_widgets.len() as c_int;

    platform_set_drvdata(pdev, &mut (*data).card as *mut snd_soc_card as *mut c_void);
    snd_soc_card_set_drvdata(
        &mut (*data).card,
        data as *mut imx_sgtl5000_data as *mut c_void,
    );

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, &mut (*data).card);
    if ret != 0 {
        dev_err_probe(
            &mut (*pdev).dev,
            ret,
            c"snd_soc_register_card failed\n".as_ptr(),
        );
        goto_put_device(codec_dev, data, ssi_np, codec_np, ret);
        return ret;
    }

    of_node_put(ssi_np);
    of_node_put(codec_np);

    0
}

unsafe fn goto_put_device(
    codec_dev: *mut i2c_client,
    data: *mut imx_sgtl5000_data,
    ssi_np: *mut device_node,
    codec_np: *mut device_node,
    ret: c_int,
) {
    put_device(&mut (*codec_dev).dev);
    goto_fail(data, ssi_np, codec_np, ret);
}

unsafe fn goto_fail(
    data: *mut imx_sgtl5000_data,
    ssi_np: *mut device_node,
    codec_np: *mut device_node,
    _ret: c_int,
) {
    if !data.is_null() && !IS_ERR((*data).codec_clk as *const c_void) {
        clk_put((*data).codec_clk);
    }
    of_node_put(ssi_np);
    of_node_put(codec_np);
}

unsafe extern "C" fn imx_sgtl5000_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;
    let data = snd_soc_card_get_drvdata(card) as *mut imx_sgtl5000_data;

    clk_put((*data).codec_clk);
}

static imx_sgtl5000_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: c"fsl,imx-audio-sgtl5000".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, imx_sgtl5000_dt_ids);

static mut imx_sgtl5000_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: c"imx-sgtl5000".as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
        of_match_table: imx_sgtl5000_dt_ids.as_ptr(),
    },
    probe: Some(imx_sgtl5000_probe),
    remove: Some(imx_sgtl5000_remove),
};
// module_platform_driver(imx_sgtl5000_driver);

// MODULE_AUTHOR("Shawn Guo <shawn.guo@linaro.org>");
// MODULE_DESCRIPTION("Freescale i.MX SGTL5000 ASoC machine driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:imx-sgtl5000");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
