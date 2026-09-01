// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2012 Freescale Semiconductor, Inc.
// Copyright 2012 Linaro Ltd.

// C dependencies:
// linux/gpio/consumer.h, linux/module.h, linux/of.h, linux/of_platform.h,
// linux/i2c.h, sound/soc.h, sound/jack.h, "imx-audmux.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const DAI_NAME_SIZE: usize = 32;
const MUX_PORT_MAX: u32 = 7;

const SND_JACK_HEADSET: c_uint = 0x0001 | 0x0002;
const SND_JACK_HEADPHONE: c_uint = 0x0001;
const SND_JACK_MICROPHONE: c_uint = 0x0002;
const SND_JACK_BTN_0: c_uint = 0x4000;

const SND_SOC_DAIFMT_I2S: c_uint = 1;
const SND_SOC_DAIFMT_NB_NF: c_uint = 0 << 8;
const SND_SOC_DAIFMT_CBP_CFP: c_uint = 3 << 12;

const GFP_KERNEL: c_uint = 0;
const GPIOD_IN: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const IMX_AUDMUX_V2_PTCR_SYN: u32 = 1 << 11;
const IMX_AUDMUX_V2_PTCR_TFSDIR: u32 = 1 << 31;
const IMX_AUDMUX_V2_PTCR_TCLKDIR: u32 = 1 << 30;

const fn IMX_AUDMUX_V2_PTCR_TFSEL(x: u32) -> u32 {
    x << 27
}

const fn IMX_AUDMUX_V2_PTCR_TCSEL(x: u32) -> u32 {
    x << 22
}

const fn IMX_AUDMUX_V2_PDCR_RXDSEL(x: u32) -> u32 {
    x << 13
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device_with_of_node,
}

#[repr(C)]
pub struct device_with_of_node {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
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
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub num_links: c_uint,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
pub struct snd_soc_jack_gpio {
    pub name: *const c_char,
    pub report: c_uint,
    pub invert: c_int,
    pub debounce_time: c_int,
    pub desc: *mut gpio_desc,
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub pin: *const c_char,
    pub mask: c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub kcontrol_news: *const c_void,
    pub num_kcontrols: c_int,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct imx_es8328_data {
    pub dev: *mut device,
    pub dai: snd_soc_dai_link,
    pub card: snd_soc_card,
    pub codec_dai_name: [c_char; DAI_NAME_SIZE],
    pub platform_name: [c_char; DAI_NAME_SIZE],
    pub jack_gpiod: *mut gpio_desc,
}

extern "C" {
    static mut THIS_MODULE: *mut module;

    fn snd_soc_card_jack_new_pins(
        card: *mut snd_soc_card,
        id: *const c_char,
        type_: c_uint,
        jack: *mut snd_soc_jack,
        pins: *mut snd_soc_jack_pin,
        num_pins: c_uint,
    ) -> c_int;
    fn snd_soc_jack_add_gpios(
        jack: *mut snd_soc_jack,
        count: c_int,
        gpios: *mut snd_soc_jack_gpio,
    ) -> c_int;
    fn of_property_read_u32(
        np: *mut device_node,
        propname: *const c_char,
        out_value: *mut u32,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn imx_audmux_v2_configure_port(port: u32, ptcr: u32, pdcr: u32) -> c_int;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const c_char,
        index: c_int,
    ) -> *mut device_node;
    fn of_find_device_by_node(np: *mut device_node) -> *mut platform_device;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn snd_soc_of_parse_card_name(card: *mut snd_soc_card, propname: *const c_char) -> c_int;
    fn snd_soc_of_parse_audio_routing(card: *mut snd_soc_card, propname: *const c_char)
        -> c_int;
    fn devm_snd_soc_register_card(dev: *mut device_with_of_node, card: *mut snd_soc_card)
        -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn put_device(dev: *mut device_with_of_node);
    fn of_node_put(node: *mut device_node);
}

static mut headset_jack_gpios: [snd_soc_jack_gpio; 1] = [snd_soc_jack_gpio {
    name: b"headset-gpio\0".as_ptr() as *const c_char,
    report: SND_JACK_HEADSET,
    invert: 0,
    debounce_time: 200,
    desc: ptr::null_mut(),
}];

static mut headset_jack: snd_soc_jack = snd_soc_jack { _private: [] };
static mut headset_jack_pins: [snd_soc_jack_pin; 2] = [
    snd_soc_jack_pin {
        pin: b"Headphone\0".as_ptr() as *const c_char,
        mask: SND_JACK_HEADPHONE,
    },
    snd_soc_jack_pin {
        pin: b"Mic Jack\0".as_ptr() as *const c_char,
        mask: SND_JACK_MICROPHONE,
    },
];

unsafe extern "C" fn imx_es8328_dai_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    let data = ((*rtd).card as *mut u8).sub(core::mem::offset_of!(imx_es8328_data, card))
        as *mut imx_es8328_data;
    let mut ret: c_int = 0;

    if !(*data).jack_gpiod.is_null() {
        /* Headphone jack detection */
        ret = snd_soc_card_jack_new_pins(
            (*rtd).card,
            b"Headphone\0".as_ptr() as *const c_char,
            SND_JACK_HEADSET | SND_JACK_BTN_0,
            &raw mut headset_jack,
            headset_jack_pins.as_mut_ptr(),
            headset_jack_pins.len() as c_uint,
        );
        if ret != 0 {
            return ret;
        }

        headset_jack_gpios[0].desc = (*data).jack_gpiod;
        ret = snd_soc_jack_add_gpios(
            &raw mut headset_jack,
            headset_jack_gpios.len() as c_int,
            headset_jack_gpios.as_mut_ptr(),
        );
    }

    ret
}

static imx_es8328_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget {
        id: 0,
        name: b"Mic Jack\0".as_ptr() as *const c_char,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 1,
        name: b"Headphone\0".as_ptr() as *const c_char,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 2,
        name: b"Speaker\0".as_ptr() as *const c_char,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
    snd_soc_dapm_widget {
        id: 3,
        name: b"audio-amp\0".as_ptr() as *const c_char,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
    },
];

static imx_es8328_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: 0,
        name: b"Headphone\0".as_ptr() as *const c_char,
    },
    snd_kcontrol_new {
        iface: 0,
        name: b"Mic Jack\0".as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn imx_es8328_probe(pdev: *mut platform_device) -> c_int {
    let np: *mut device_node = (*pdev).dev.of_node;
    let mut ssi_np: *mut device_node = ptr::null_mut();
    let mut codec_np: *mut device_node = ptr::null_mut();
    let mut ssi_pdev: *mut platform_device;
    let mut data: *mut imx_es8328_data;
    let mut comp: *mut snd_soc_dai_link_component;
    let mut int_port: u32 = 0;
    let mut ext_port: u32 = 0;
    let mut ret: c_int;
    let dev: *mut device = &mut (*pdev).dev as *mut device_with_of_node as *mut device;

    ret = of_property_read_u32(np, b"mux-int-port\0".as_ptr() as *const c_char, &mut int_port);
    if ret != 0 {
        dev_err(
            dev,
            b"mux-int-port missing or invalid\n\0".as_ptr() as *const c_char,
        );
        goto_fail(&mut ssi_np, &mut codec_np);
        return ret;
    }
    if int_port > MUX_PORT_MAX || int_port == 0 {
        dev_err(
            dev,
            b"mux-int-port: hardware only has %d mux ports\n\0".as_ptr() as *const c_char,
            MUX_PORT_MAX as c_int,
        );
        ret = -EINVAL;
        goto_fail(&mut ssi_np, &mut codec_np);
        return ret;
    }

    ret = of_property_read_u32(np, b"mux-ext-port\0".as_ptr() as *const c_char, &mut ext_port);
    if ret != 0 {
        dev_err(
            dev,
            b"mux-ext-port missing or invalid\n\0".as_ptr() as *const c_char,
        );
        goto_fail(&mut ssi_np, &mut codec_np);
        return ret;
    }
    if ext_port > MUX_PORT_MAX || ext_port == 0 {
        dev_err(
            dev,
            b"mux-ext-port: hardware only has %d mux ports\n\0".as_ptr() as *const c_char,
            MUX_PORT_MAX as c_int,
        );
        ret = -EINVAL;
        goto_fail(&mut ssi_np, &mut codec_np);
        return ret;
    }

    /*
     * The port numbering in the hardware manual starts at 1, while
     * the audmux API expects it starts at 0.
     */
    int_port = int_port.wrapping_sub(1);
    ext_port = ext_port.wrapping_sub(1);
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
            dev,
            b"audmux internal port setup failed\n\0".as_ptr() as *const c_char,
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
            dev,
            b"audmux external port setup failed\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    ssi_np = of_parse_phandle((*pdev).dev.of_node, b"ssi-controller\0".as_ptr() as *const c_char, 0);
    codec_np = of_parse_phandle((*pdev).dev.of_node, b"audio-codec\0".as_ptr() as *const c_char, 0);
    if ssi_np.is_null() || codec_np.is_null() {
        dev_err(dev, b"phandle missing or invalid\n\0".as_ptr() as *const c_char);
        ret = -EINVAL;
        goto_fail(&mut ssi_np, &mut codec_np);
        return ret;
    }

    ssi_pdev = of_find_device_by_node(ssi_np);
    if ssi_pdev.is_null() {
        dev_err(
            dev,
            b"failed to find SSI platform device\n\0".as_ptr() as *const c_char,
        );
        ret = -EINVAL;
        goto_fail(&mut ssi_np, &mut codec_np);
        return ret;
    }

    data = devm_kzalloc(dev, size_of::<imx_es8328_data>(), GFP_KERNEL) as *mut imx_es8328_data;
    if data.is_null() {
        ret = -ENOMEM;
        goto_put_device(ssi_pdev, &mut ssi_np, &mut codec_np);
        return ret;
    }

    comp = devm_kzalloc(
        dev,
        2usize.wrapping_mul(size_of::<snd_soc_dai_link_component>()),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if comp.is_null() {
        ret = -ENOMEM;
        goto_put_device(ssi_pdev, &mut ssi_np, &mut codec_np);
        return ret;
    }

    (*data).dev = dev;

    (*data).jack_gpiod =
        devm_gpiod_get_optional(dev, b"jack\0".as_ptr() as *const c_char, GPIOD_IN);
    if IS_ERR((*data).jack_gpiod as *const c_void) {
        ret = PTR_ERR((*data).jack_gpiod as *const c_void);
        goto_put_device(ssi_pdev, &mut ssi_np, &mut codec_np);
        return ret;
    }

    /*
     * CPU == Platform
     * platform is using soc-generic-dmaengine-pcm
     */
    (*data).dai.cpus = comp.add(0);
    (*data).dai.platforms = comp.add(0);
    (*data).dai.codecs = comp.add(1);

    (*data).dai.num_cpus = 1;
    (*data).dai.num_codecs = 1;
    (*data).dai.num_platforms = 1;

    (*data).dai.name = b"hifi\0".as_ptr() as *const c_char;
    (*data).dai.stream_name = b"hifi\0".as_ptr() as *const c_char;
    (*(*data).dai.codecs).dai_name = b"es8328-hifi-analog\0".as_ptr() as *const c_char;
    (*(*data).dai.codecs).of_node = codec_np;
    (*(*data).dai.cpus).of_node = ssi_np;
    (*data).dai.init = Some(imx_es8328_dai_init);
    (*data).dai.dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF | SND_SOC_DAIFMT_CBP_CFP;

    (*data).card.dev = dev;
    (*data).card.dapm_widgets = imx_es8328_dapm_widgets.as_ptr();
    (*data).card.num_dapm_widgets = imx_es8328_dapm_widgets.len() as c_uint;
    (*data).card.controls = imx_es8328_controls.as_ptr();
    (*data).card.num_controls = imx_es8328_controls.len() as c_uint;
    ret = snd_soc_of_parse_card_name(&mut (*data).card, b"model\0".as_ptr() as *const c_char);
    if ret != 0 {
        dev_err(dev, b"Unable to parse card name\n\0".as_ptr() as *const c_char);
        goto_put_device(ssi_pdev, &mut ssi_np, &mut codec_np);
        return ret;
    }
    ret = snd_soc_of_parse_audio_routing(
        &mut (*data).card,
        b"audio-routing\0".as_ptr() as *const c_char,
    );
    if ret != 0 {
        dev_err(
            dev,
            b"Unable to parse routing: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        goto_put_device(ssi_pdev, &mut ssi_np, &mut codec_np);
        return ret;
    }
    (*data).card.num_links = 1;
    (*data).card.owner = THIS_MODULE;
    (*data).card.dai_link = &mut (*data).dai;

    ret = devm_snd_soc_register_card(&mut (*pdev).dev, &mut (*data).card);
    if ret != 0 {
        dev_err(
            dev,
            b"Unable to register: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        goto_put_device(ssi_pdev, &mut ssi_np, &mut codec_np);
        return ret;
    }

    platform_set_drvdata(pdev, data as *mut c_void);
    put_device(&mut (*ssi_pdev).dev);
    goto_fail(&mut ssi_np, &mut codec_np);

    ret
}

unsafe fn goto_put_device(
    ssi_pdev: *mut platform_device,
    ssi_np: &mut *mut device_node,
    codec_np: &mut *mut device_node,
) {
    put_device(&mut (*ssi_pdev).dev);
    goto_fail(ssi_np, codec_np);
}

unsafe fn goto_fail(ssi_np: &mut *mut device_node, codec_np: &mut *mut device_node) {
    of_node_put(*ssi_np);
    of_node_put(*codec_np);
}

static imx_es8328_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: b"fsl,imx-audio-es8328\0".as_ptr() as *const c_char,
    },
    of_device_id {
        /* sentinel */
        compatible: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, imx_es8328_dt_ids);

static mut imx_es8328_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"imx-es8328\0".as_ptr() as *const c_char,
        of_match_table: imx_es8328_dt_ids.as_ptr(),
    },
    probe: Some(imx_es8328_probe),
};

// module_platform_driver(imx_es8328_driver);

// MODULE_AUTHOR("Sean Cross <xobs@kosagi.com>");
// MODULE_DESCRIPTION("Kosagi i.MX6 ES8328 ASoC machine driver");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS("platform:imx-audio-es8328");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
