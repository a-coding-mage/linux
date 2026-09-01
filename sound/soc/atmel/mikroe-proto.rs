// SPDX-License-Identifier: GPL-2.0-only
/*
 * ASoC driver for PROTO AudioCODEC (with a WM8731)
 *
 * Author:      Florian Meier, <koalo@koalo.de>
 *	      Copyright 2013
 */

// C includes translated as external dependencies:
// linux/module.h, linux/platform_device.h
// sound/core.h, sound/pcm.h, sound/soc.h, sound/jack.h
// ../codecs/wm8731.h

const XTAL_RATE: u32 = 12288000; /* This is fixed on this board */

unsafe extern "C" {
    static mut THIS_MODULE: *mut module;

    fn snd_soc_rtd_to_codec(
        rtd: *mut snd_soc_pcm_runtime,
        num: ::core::ffi::c_int,
    ) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: ::core::ffi::c_int,
        freq: ::core::ffi::c_uint,
        dir: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn dev_err(dev: *mut device, fmt: *const ::core::ffi::c_char, ...);
    fn dev_err_probe(
        dev: *mut device,
        err: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snd_soc_of_parse_card_name(
        card: *mut snd_soc_card,
        propname: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn devm_kzalloc(
        dev: *mut device,
        size: usize,
        flags: gfp_t,
    ) -> *mut ::core::ffi::c_void;
    fn of_parse_phandle(
        np: *mut device_node,
        phandle_name: *const ::core::ffi::c_char,
        index: ::core::ffi::c_int,
    ) -> *mut device_node;
    fn snd_soc_daifmt_parse_format(
        np: *mut device_node,
        prefix: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
    fn snd_soc_daifmt_parse_clock_provider_as_phandle(
        np: *mut device_node,
        prefix: *const ::core::ffi::c_char,
        bitclkmaster: *mut *mut device_node,
        framemaster: *mut *mut device_node,
    );
    fn snd_soc_daifmt_parse_clock_provider_as_flag(
        np: *mut device_node,
        prefix: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_uint;
    fn devm_snd_soc_register_card(
        dev: *mut device,
        card: *mut snd_soc_card,
    ) -> ::core::ffi::c_int;
    fn of_node_put(node: *mut device_node);
}

#[repr(C)]
pub struct module {
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
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const ::core::ffi::c_char,
    pub id: ::core::ffi::c_int,
    pub reg: ::core::ffi::c_int,
    pub shift: ::core::ffi::c_uchar,
    pub mask: ::core::ffi::c_uint,
    pub on_val: ::core::ffi::c_uint,
    pub off_val: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const ::core::ffi::c_char,
    pub control: *const ::core::ffi::c_char,
    pub source: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const ::core::ffi::c_char,
    pub dai_name: *const ::core::ffi::c_char,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const ::core::ffi::c_char,
    pub stream_name: *const ::core::ffi::c_char,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: ::core::ffi::c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: ::core::ffi::c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: ::core::ffi::c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> ::core::ffi::c_int>,
    pub dai_fmt: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const ::core::ffi::c_char,
    pub owner: *mut module,
    pub dev: *mut device,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: ::core::ffi::c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: ::core::ffi::c_int,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: ::core::ffi::c_int,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct driver {
    pub name: *const ::core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> ::core::ffi::c_int>,
}

type gfp_t = ::core::ffi::c_uint;

const WM8731_SYSCLK_XTAL: ::core::ffi::c_int = 0;
const SND_SOC_CLOCK_IN: ::core::ffi::c_int = 0;
const SND_SOC_DAIFMT_CBP_CFP: ::core::ffi::c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: ::core::ffi::c_uint = 0;
const GFP_KERNEL: gfp_t = 0;
const EINVAL: ::core::ffi::c_int = 22;
const ENOMEM: ::core::ffi::c_int = 12;

const SND_SOC_DAPM_MIC_ID: ::core::ffi::c_int = 0;
const SND_SOC_DAPM_HP_ID: ::core::ffi::c_int = 0;

unsafe extern "C" fn snd_proto_init(
    rtd: *mut snd_soc_pcm_runtime,
) -> ::core::ffi::c_int {
    let card: *mut snd_soc_card = unsafe { (*rtd).card };
    let codec_dai: *mut snd_soc_dai = unsafe { snd_soc_rtd_to_codec(rtd, 0) };

    /* Set proto sysclk */
    let ret: ::core::ffi::c_int = unsafe {
        snd_soc_dai_set_sysclk(
            codec_dai,
            WM8731_SYSCLK_XTAL,
            XTAL_RATE,
            SND_SOC_CLOCK_IN,
        )
    };
    if ret < 0 {
        unsafe {
            dev_err(
                (*card).dev,
                c"Failed to set WM8731 SYSCLK: %d\n".as_ptr(),
                ret,
            );
        }
        return ret;
    }

    0
}

static snd_proto_widget: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget {
        name: c"Microphone Jack".as_ptr(),
        id: SND_SOC_DAPM_MIC_ID,
        reg: 0,
        shift: 0,
        mask: 0,
        on_val: 0,
        off_val: 0,
    },
    snd_soc_dapm_widget {
        name: c"Headphone Jack".as_ptr(),
        id: SND_SOC_DAPM_HP_ID,
        reg: 0,
        shift: 0,
        mask: 0,
        on_val: 0,
        off_val: 0,
    },
];

static snd_proto_route: [snd_soc_dapm_route; 4] = [
    /* speaker connected to LHPOUT/RHPOUT */
    snd_soc_dapm_route {
        sink: c"Headphone Jack".as_ptr(),
        control: ::core::ptr::null(),
        source: c"LHPOUT".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Headphone Jack".as_ptr(),
        control: ::core::ptr::null(),
        source: c"RHPOUT".as_ptr(),
    },
    /* mic is connected to Mic Jack, with WM8731 Mic Bias */
    snd_soc_dapm_route {
        sink: c"MICIN".as_ptr(),
        control: ::core::ptr::null(),
        source: c"Mic Bias".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"Mic Bias".as_ptr(),
        control: ::core::ptr::null(),
        source: c"Microphone Jack".as_ptr(),
    },
];

/* audio machine driver */
static mut snd_proto: snd_soc_card = snd_soc_card {
    name: c"snd_mikroe_proto".as_ptr(),
    owner: unsafe { THIS_MODULE },
    dev: ::core::ptr::null_mut(),
    dapm_widgets: snd_proto_widget.as_ptr(),
    num_dapm_widgets: snd_proto_widget.len() as ::core::ffi::c_int,
    dapm_routes: snd_proto_route.as_ptr(),
    num_dapm_routes: snd_proto_route.len() as ::core::ffi::c_int,
    dai_link: ::core::ptr::null_mut(),
    num_links: 0,
};

unsafe extern "C" fn snd_proto_probe(
    pdev: *mut platform_device,
) -> ::core::ffi::c_int {
    let dai: *mut snd_soc_dai_link;
    let comp: *mut snd_soc_dai_link_component;
    let np: *mut device_node = unsafe { (*pdev).dev.of_node };
    let codec_np: *mut device_node;
    let cpu_np: *mut device_node;
    let mut bitclkmaster: *mut device_node = ::core::ptr::null_mut();
    let mut framemaster: *mut device_node = ::core::ptr::null_mut();
    let mut dai_fmt: ::core::ffi::c_uint;
    let mut ret: ::core::ffi::c_int = 0;

    if np.is_null() {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                c"No device node supplied\n".as_ptr(),
            );
        }
        return -EINVAL;
    }

    unsafe {
        snd_proto.dev = &mut (*pdev).dev;
    }
    ret = unsafe { snd_soc_of_parse_card_name(&raw mut snd_proto, c"model".as_ptr()) };
    if ret != 0 {
        return ret;
    }

    dai = unsafe {
        devm_kzalloc(
            &mut (*pdev).dev,
            ::core::mem::size_of::<snd_soc_dai_link>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link
    };
    if dai.is_null() {
        return -ENOMEM;
    }

    /* for cpus/codecs/platforms */
    comp = unsafe {
        devm_kzalloc(
            &mut (*pdev).dev,
            3 * ::core::mem::size_of::<snd_soc_dai_link_component>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component
    };
    if comp.is_null() {
        return -ENOMEM;
    }

    unsafe {
        snd_proto.dai_link = dai;
        snd_proto.num_links = 1;

        (*dai).cpus = comp.add(0);
        (*dai).num_cpus = 1;
        (*dai).codecs = comp.add(1);
        (*dai).num_codecs = 1;
        (*dai).platforms = comp.add(2);
        (*dai).num_platforms = 1;

        (*dai).name = c"WM8731".as_ptr();
        (*dai).stream_name = c"WM8731 HiFi".as_ptr();
        (*(*dai).codecs).dai_name = c"wm8731-hifi".as_ptr();
        (*dai).init = Some(snd_proto_init);
    }

    codec_np = unsafe { of_parse_phandle(np, c"audio-codec".as_ptr(), 0) };
    if codec_np.is_null() {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                c"audio-codec node missing\n".as_ptr(),
            );
        }
        return -EINVAL;
    }
    unsafe {
        (*(*dai).codecs).of_node = codec_np;
    }

    cpu_np = unsafe { of_parse_phandle(np, c"i2s-controller".as_ptr(), 0) };
    if cpu_np.is_null() {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                c"i2s-controller missing\n".as_ptr(),
            );
        }
        ret = -EINVAL;
        unsafe {
            of_node_put(codec_np);
        }
        return ret;
    }
    unsafe {
        (*(*dai).cpus).of_node = cpu_np;
        (*(*dai).platforms).of_node = cpu_np;
    }

    dai_fmt = unsafe { snd_soc_daifmt_parse_format(np, ::core::ptr::null()) };
    unsafe {
        snd_soc_daifmt_parse_clock_provider_as_phandle(
            np,
            ::core::ptr::null(),
            &mut bitclkmaster,
            &mut framemaster,
        );
    }
    if bitclkmaster != framemaster {
        unsafe {
            dev_err(
                &mut (*pdev).dev,
                c"Must be the same bitclock and frame master\n".as_ptr(),
            );
        }
        ret = -EINVAL;
        unsafe {
            of_node_put(bitclkmaster);
            of_node_put(framemaster);
            of_node_put(cpu_np);
            of_node_put(codec_np);
        }
        return ret;
    }
    if !bitclkmaster.is_null() {
        if codec_np == bitclkmaster {
            dai_fmt |= SND_SOC_DAIFMT_CBP_CFP;
        } else {
            dai_fmt |= SND_SOC_DAIFMT_CBC_CFC;
        }
    } else {
        dai_fmt |= unsafe {
            snd_soc_daifmt_parse_clock_provider_as_flag(np, ::core::ptr::null())
        };
    }

    unsafe {
        (*dai).dai_fmt = dai_fmt;
    }
    ret = unsafe { devm_snd_soc_register_card(&mut (*pdev).dev, &raw mut snd_proto) };
    if ret != 0 {
        unsafe {
            dev_err_probe(
                &mut (*pdev).dev,
                ret,
                c"snd_soc_register_card() failed\n".as_ptr(),
            );
        }
    }

    unsafe {
        of_node_put(bitclkmaster);
        of_node_put(framemaster);
        of_node_put(cpu_np);
        of_node_put(codec_np);
    }
    ret
}

static snd_proto_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"mikroe,mikroe-proto".as_ptr(),
    },
    of_device_id {
        compatible: ::core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, snd_proto_of_match);

static mut snd_proto_driver: platform_driver = platform_driver {
    driver: driver {
        name: c"snd-mikroe-proto".as_ptr(),
        of_match_table: snd_proto_of_match.as_ptr(),
    },
    probe: Some(snd_proto_probe),
};

// module_platform_driver(snd_proto_driver);

// MODULE_AUTHOR("Florian Meier");
// MODULE_DESCRIPTION("ASoC Driver for PROTO board (WM8731)");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
