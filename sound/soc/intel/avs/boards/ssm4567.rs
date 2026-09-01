// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

// C dependencies translated as external Rust dependencies:
// linux/module.h, linux/platform_device.h, sound/core.h, sound/pcm.h,
// sound/pcm_params.h, sound/soc.h, sound/soc-acpi.h,
// ../../../codecs/nau8825.h, ../utils.h

const SKL_SSM_CODEC_DAI: *const i8 = b"ssm4567-hifi\0".as_ptr() as *const i8;

extern "C" {
    static mut THIS_MODULE: *mut module;
    static snd_soc_pm_ops: dev_pm_ops;

    fn snd_soc_rtd_to_codec(
        rtd: *mut snd_soc_pcm_runtime,
        num: ::core::ffi::c_int,
    ) -> *mut snd_soc_dai;
    fn snd_soc_dai_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: ::core::ffi::c_uint,
        rx_mask: ::core::ffi::c_uint,
        slots: ::core::ffi::c_int,
        slot_width: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn hw_param_interval(
        p: *mut snd_pcm_hw_params,
        var: ::core::ffi::c_int,
    ) -> *mut snd_interval;
    fn hw_param_mask(p: *mut snd_pcm_hw_params, var: ::core::ffi::c_int) -> *mut snd_mask;
    fn snd_mask_none(mask: *mut snd_mask);
    fn snd_mask_set_format(mask: *mut snd_mask, val: ::core::ffi::c_int);
    fn devm_kzalloc(
        dev: *mut device,
        size: usize,
        flags: gfp_t,
    ) -> *mut ::core::ffi::c_void;
    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: gfp_t,
    ) -> *mut ::core::ffi::c_void;
    fn devm_kasprintf(
        dev: *mut device,
        flags: gfp_t,
        fmt: *const i8,
        ...
    ) -> *mut i8;
    fn dev_name(dev: *const device) -> *const i8;
    fn dev_get_platdata(dev: *const device) -> *mut ::core::ffi::c_void;
    fn avs_mach_get_ssp_tdm(
        dev: *mut device,
        mach: *mut snd_soc_acpi_mach,
        ssp_port: *mut ::core::ffi::c_int,
        tdm_slot: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn dev_err(dev: *const device, fmt: *const i8, ...);
    fn devm_snd_soc_register_deferrable_card(
        dev: *mut device,
        card: *mut snd_soc_card,
    ) -> ::core::ffi::c_int;
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
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_mask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_interval {
    pub min: ::core::ffi::c_uint,
    pub max: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub pdata: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct avs_mach_pdata {
    pub obsolete_card_names: bool,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const i8,
    pub dai_name: *const i8,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const i8,
    pub stream_name: *const i8,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: ::core::ffi::c_uint,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: ::core::ffi::c_uint,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: ::core::ffi::c_uint,
    pub id: ::core::ffi::c_int,
    pub dai_fmt: ::core::ffi::c_uint,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> ::core::ffi::c_int>,
    pub be_hw_params_fixup: Option<
        unsafe extern "C" fn(
            *mut snd_soc_pcm_runtime,
            *mut snd_pcm_hw_params,
        ) -> ::core::ffi::c_int,
    >,
    pub nonatomic: ::core::ffi::c_uint,
    pub no_pcm: ::core::ffi::c_uint,
    pub ignore_pmdown_time: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_soc_codec_conf {
    pub dlc: snd_soc_dai_link_component,
    pub name_prefix: *const i8,
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
    pub sink: *const i8,
    pub control: *const i8,
    pub source: *const i8,
}

#[repr(C)]
pub struct snd_soc_card {
    pub name: *const i8,
    pub driver_name: *const i8,
    pub long_name: *const i8,
    pub dev: *mut device,
    pub owner: *mut module,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: ::core::ffi::c_int,
    pub codec_conf: *mut snd_soc_codec_conf,
    pub num_configs: ::core::ffi::c_int,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: ::core::ffi::c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: ::core::ffi::c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: ::core::ffi::c_uint,
    pub fully_routed: bool,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [i8; 20],
    pub driver_data: kernel_ulong_t,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const i8,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> ::core::ffi::c_int>,
    pub driver: device_driver,
    pub id_table: *const platform_device_id,
}

type gfp_t = ::core::ffi::c_uint;
type kernel_ulong_t = ::core::ffi::c_ulong;

const GFP_KERNEL: gfp_t = 0;
const ENOMEM: ::core::ffi::c_int = 12;
const SNDRV_PCM_HW_PARAM_RATE: ::core::ffi::c_int = 0;
const SNDRV_PCM_HW_PARAM_CHANNELS: ::core::ffi::c_int = 1;
const SNDRV_PCM_HW_PARAM_FORMAT: ::core::ffi::c_int = 2;
const SNDRV_PCM_FORMAT_S24_LE: ::core::ffi::c_int = 6;
const SND_SOC_DAIFMT_DSP_A: ::core::ffi::c_uint = 0;
const SND_SOC_DAIFMT_IB_NF: ::core::ffi::c_uint = 0;
const SND_SOC_DAIFMT_CBC_CFC: ::core::ffi::c_uint = 0;

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const i8
    };
}

macro_rules! comp_codec_conf {
    ($name:literal) => {
        snd_soc_dai_link_component {
            name: c_str!($name),
            dai_name: ::core::ptr::null(),
        }
    };
}

macro_rules! soc_dapm_pin_switch {
    ($name:literal) => {
        snd_kcontrol_new { _private: [] }
    };
}

macro_rules! snd_soc_dapm_spk {
    ($name:literal, $event:expr) => {
        snd_soc_dapm_widget { _private: [] }
    };
}

static mut card_codec_conf: [snd_soc_codec_conf; 2] = [
    snd_soc_codec_conf {
        dlc: comp_codec_conf!("i2c-INT343B:00"),
        name_prefix: c_str!("Left"),
    },
    snd_soc_codec_conf {
        dlc: comp_codec_conf!("i2c-INT343B:01"),
        name_prefix: c_str!("Right"),
    },
];

static card_controls: [snd_kcontrol_new; 2] = [
    soc_dapm_pin_switch!("Left Speaker"),
    soc_dapm_pin_switch!("Right Speaker"),
];

static card_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_spk!("Left Speaker", ::core::ptr::null_mut::<::core::ffi::c_void>()),
    snd_soc_dapm_spk!("Right Speaker", ::core::ptr::null_mut::<::core::ffi::c_void>()),
];

static card_base_routes: [snd_soc_dapm_route; 2] = [
    snd_soc_dapm_route {
        sink: c_str!("Left Speaker"),
        control: ::core::ptr::null(),
        source: c_str!("Left OUT"),
    },
    snd_soc_dapm_route {
        sink: c_str!("Right Speaker"),
        control: ::core::ptr::null(),
        source: c_str!("Right OUT"),
    },
];

unsafe extern "C" fn avs_ssm4567_codec_init(
    runtime: *mut snd_soc_pcm_runtime,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int;

    /* Slot 1 for left */
    ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_codec(runtime, 0), 0x01, 0x01, 2, 48);
    if ret < 0 {
        return ret;
    }

    /* Slot 2 for right */
    ret = snd_soc_dai_set_tdm_slot(snd_soc_rtd_to_codec(runtime, 1), 0x02, 0x02, 2, 48);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn avs_ssm4567_be_fixup(
    runrime: *mut snd_soc_pcm_runtime,
    params: *mut snd_pcm_hw_params,
) -> ::core::ffi::c_int {
    let rate: *mut snd_interval;
    let channels: *mut snd_interval;
    let fmt: *mut snd_mask;

    rate = hw_param_interval(params, SNDRV_PCM_HW_PARAM_RATE);
    channels = hw_param_interval(params, SNDRV_PCM_HW_PARAM_CHANNELS);
    fmt = hw_param_mask(params, SNDRV_PCM_HW_PARAM_FORMAT);

    /* The ADSP will convert the FE rate to 48k, stereo */
    (*rate).max = 48000;
    (*rate).min = (*rate).max;
    (*channels).max = 2;
    (*channels).min = (*channels).max;

    /* set SSP0 to 24 bit */
    snd_mask_none(fmt);
    snd_mask_set_format(fmt, SNDRV_PCM_FORMAT_S24_LE);
    0
}

unsafe extern "C" fn avs_create_dai_link(
    dev: *mut device,
    ssp_port: ::core::ffi::c_int,
    tdm_slot: ::core::ffi::c_int,
    dai_link: *mut *mut snd_soc_dai_link,
) -> ::core::ffi::c_int {
    let platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;

    dl = devm_kzalloc(dev, ::core::mem::size_of::<snd_soc_dai_link>(), GFP_KERNEL)
        as *mut snd_soc_dai_link;
    platform = devm_kzalloc(
        dev,
        ::core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    (*dl).name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        c_str!("%s%d%s%d"),
        c_str!("SSP"),
        ssp_port,
        c_str!("-Codec"),
        tdm_slot,
    );
    (*dl).cpus = devm_kzalloc(
        dev,
        ::core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    (*dl).codecs = devm_kcalloc(
        dev,
        2,
        ::core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if (*dl).name.is_null() || (*dl).cpus.is_null() || (*dl).codecs.is_null() {
        return -ENOMEM;
    }

    (*(*dl).cpus).dai_name = devm_kasprintf(
        dev,
        GFP_KERNEL,
        c_str!("%s%d%s%d"),
        c_str!("SSP"),
        ssp_port,
        c_str!(" Pin"),
        tdm_slot,
    );
    (*(*dl).codecs.add(0)).name = devm_kasprintf(dev, GFP_KERNEL, c_str!("i2c-INT343B:00"));
    (*(*dl).codecs.add(0)).dai_name = devm_kasprintf(dev, GFP_KERNEL, c_str!("ssm4567-hifi"));
    (*(*dl).codecs.add(1)).name = devm_kasprintf(dev, GFP_KERNEL, c_str!("i2c-INT343B:01"));
    (*(*dl).codecs.add(1)).dai_name = devm_kasprintf(dev, GFP_KERNEL, c_str!("ssm4567-hifi"));
    if (*(*dl).cpus).dai_name.is_null()
        || (*(*dl).codecs.add(0)).name.is_null()
        || (*(*dl).codecs.add(0)).dai_name.is_null()
        || (*(*dl).codecs.add(1)).name.is_null()
        || (*(*dl).codecs.add(1)).dai_name.is_null()
    {
        return -ENOMEM;
    }

    (*platform).name = dev_name(dev);
    (*dl).num_cpus = 1;
    (*dl).num_codecs = 2;
    (*dl).platforms = platform;
    (*dl).num_platforms = 1;
    (*dl).id = 0;
    (*dl).dai_fmt = SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_IB_NF | SND_SOC_DAIFMT_CBC_CFC;
    (*dl).init = Some(avs_ssm4567_codec_init);
    (*dl).be_hw_params_fixup = Some(avs_ssm4567_be_fixup);
    (*dl).nonatomic = 1;
    (*dl).no_pcm = 1;
    (*dl).ignore_pmdown_time = 1;

    *dai_link = dl;

    0
}

unsafe extern "C" fn avs_ssm4567_probe(pdev: *mut platform_device) -> ::core::ffi::c_int {
    let mut dai_link: *mut snd_soc_dai_link = ::core::ptr::null_mut();
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
    let dev: *mut device = &mut (*pdev).dev;
    let mut ssp_port: ::core::ffi::c_int = 0;
    let mut tdm_slot: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int;

    mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
    pdata = (*mach).pdata as *mut avs_mach_pdata;

    ret = avs_mach_get_ssp_tdm(dev, mach, &mut ssp_port, &mut tdm_slot);
    if ret != 0 {
        return ret;
    }

    ret = avs_create_dai_link(dev, ssp_port, tdm_slot, &mut dai_link);
    if ret != 0 {
        dev_err(dev, c_str!("Failed to create dai link: %d"), ret);
        return ret;
    }

    card = devm_kzalloc(dev, ::core::mem::size_of::<snd_soc_card>(), GFP_KERNEL)
        as *mut snd_soc_card;
    if card.is_null() {
        return -ENOMEM;
    }

    if (*pdata).obsolete_card_names {
        (*card).name = c_str!("avs_ssm4567");
    } else {
        (*card).driver_name = c_str!("avs_ssm4567");
        (*card).name = c_str!("AVS I2S SSM4567");
        (*card).long_name = (*card).name;
    }
    (*card).dev = dev;
    (*card).owner = THIS_MODULE;
    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).codec_conf = card_codec_conf.as_mut_ptr();
    (*card).num_configs = card_codec_conf.len() as ::core::ffi::c_int;
    (*card).controls = card_controls.as_ptr();
    (*card).num_controls = card_controls.len() as ::core::ffi::c_uint;
    (*card).dapm_widgets = card_widgets.as_ptr();
    (*card).num_dapm_widgets = card_widgets.len() as ::core::ffi::c_uint;
    (*card).dapm_routes = card_base_routes.as_ptr();
    (*card).num_dapm_routes = card_base_routes.len() as ::core::ffi::c_uint;
    (*card).fully_routed = true;

    devm_snd_soc_register_deferrable_card(dev, card)
}

static avs_ssm4567_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as i8, b'v' as i8, b's' as i8, b'_' as i8, b's' as i8, b's' as i8, b'm' as i8,
            b'4' as i8, b'5' as i8, b'6' as i8, b'7' as i8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        driver_data: 0,
    },
    platform_device_id {
        name: [0; 20],
        driver_data: 0,
    },
];

// MODULE_DEVICE_TABLE(platform, avs_ssm4567_driver_ids);

static mut avs_ssm4567_driver: platform_driver = platform_driver {
    probe: Some(avs_ssm4567_probe),
    driver: device_driver {
        name: c_str!("avs_ssm4567"),
        pm: unsafe { &snd_soc_pm_ops },
    },
    id_table: avs_ssm4567_driver_ids.as_ptr(),
};

// module_platform_driver(avs_ssm4567_driver)

// MODULE_DESCRIPTION("Intel ssm4567 machine driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
