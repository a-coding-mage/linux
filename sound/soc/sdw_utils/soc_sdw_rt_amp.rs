// SPDX-License-Identifier: GPL-2.0-only
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2022 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.

/*
 *  soc_sdw_rt_amp - Helpers to handle RT1308/RT1316/RT1318 from generic machine driver
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::MaybeUninit;
use core::ptr;

const CODEC_NAME_SIZE: usize = 7;

/* choose a larger value to resolve compatibility issues */
const RT_AMP_MAX_BQ_REG: usize = RT1316_MAX_BQ_REG as usize;

const EINVAL: c_int = 22;
const EPROBE_DEFER: c_int = 517;
const SND_SOC_CLOCK_IN: c_int = 0;
const RT1308_PLL_S_MCLK: c_int = 0;
const RT1308_FS_SYS_S_PLL: c_int = 0;
const RT1316_MAX_BQ_REG: c_uint = 0;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct software_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub name_prefix: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub name: *const c_char,
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
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
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub codecs: *mut snd_soc_dai_link_component,
}

#[repr(C)]
pub struct asoc_sdw_mc_private {
    pub amp_dev1: *mut device,
    pub amp_dev2: *mut device,
}

#[repr(C)]
pub struct asoc_sdw_codec_info {
    pub amp_num: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_ops {
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
}

#[repr(C)]
pub struct property_entry {
    _private: [usize; 4],
}

#[repr(C)]
pub struct dmi_system_id {
    /* DMI_MATCH/DMI_EXACT_MATCH initializer details are supplied by linux/dmi.h. */
    pub matches: [dmi_strmatch; 4],
    pub driver_data: *mut c_void,
}

#[repr(C)]
pub struct dmi_strmatch {
    pub slot: c_int,
    pub substr: *const c_char,
    pub exact_match: bool,
}

#[repr(C)]
struct rt_amp_platform_data {
    bq_params: *const u8,
    bq_params_cnt: c_uint,
}

unsafe extern "C" {
    static dell_0a5d_bq_params: [u8; 0];
    static dell_0b00_bq_params: [u8; 0];
    static sdw_bus_type: bus_type;

    fn dmi_first_match(ids: *const dmi_system_id) -> *const dmi_system_id;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn fwnode_create_software_node(
        props: *const property_entry,
        parent: *const software_node,
    ) -> *mut fwnode_handle;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn device_add_software_node(dev: *mut device, swnode: *const software_node) -> c_int;
    fn to_software_node(fwnode: *mut fwnode_handle) -> *const software_node;
    fn fwnode_handle_put(fwnode: *mut fwnode_handle);
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snd_soc_card_to_dapm(card: *mut snd_soc_card) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_codec(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_dai_set_pll(
        dai: *mut snd_soc_dai,
        pll_id: c_int,
        source: c_int,
        freq_in: c_uint,
        freq_out: c_uint,
    ) -> c_int;
    fn snd_soc_dai_set_sysclk(
        dai: *mut snd_soc_dai,
        clk_id: c_int,
        freq: c_uint,
        dir: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut c_void;
    fn device_remove_software_node(dev: *mut device);
    fn put_device(dev: *mut device);
    fn bus_find_device_by_name(
        bus: *mut bus_type,
        start: *mut device,
        name: *const c_char,
    ) -> *mut device;
}

#[repr(C)]
pub struct bus_type {
    _private: [u8; 0],
}

unsafe fn PROPERTY_ENTRY_U8_ARRAY(_name: *const c_char, _array: *const u8) -> property_entry {
    MaybeUninit::<property_entry>::zeroed().assume_init()
}

unsafe fn PROPERTY_ENTRY_U32(_name: *const c_char, _val: c_uint) -> property_entry {
    MaybeUninit::<property_entry>::zeroed().assume_init()
}

static dell_0a5d_platform_data: rt_amp_platform_data = rt_amp_platform_data {
    bq_params: unsafe { dell_0a5d_bq_params.as_ptr() },
    bq_params_cnt: unsafe { dell_0a5d_bq_params.len() as c_uint },
};

static dell_0b00_platform_data: rt_amp_platform_data = rt_amp_platform_data {
    bq_params: unsafe { dell_0b00_bq_params.as_ptr() },
    bq_params_cnt: unsafe { dell_0b00_bq_params.len() as c_uint },
};

const DMI_SYS_VENDOR: c_int = 0;
const DMI_PRODUCT_SKU: c_int = 0;

const fn DMI_MATCH(slot: c_int, substr: *const c_char) -> dmi_strmatch {
    dmi_strmatch {
        slot,
        substr,
        exact_match: false,
    }
}

const fn DMI_EXACT_MATCH(slot: c_int, substr: *const c_char) -> dmi_strmatch {
    dmi_strmatch {
        slot,
        substr,
        exact_match: true,
    }
}

static dmi_platform_data: [dmi_system_id; 9] = [
    /* CometLake devices */
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"0990\0".as_ptr() as *const c_char),
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
        ],
        driver_data: &dell_0a5d_platform_data as *const _ as *mut c_void,
    },
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"098F\0".as_ptr() as *const c_char),
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
        ],
        driver_data: &dell_0a5d_platform_data as *const _ as *mut c_void,
    },
    /* TigerLake devices */
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"0A5D\0".as_ptr() as *const c_char),
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
        ],
        driver_data: &dell_0a5d_platform_data as *const _ as *mut c_void,
    },
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"0A5E\0".as_ptr() as *const c_char),
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
        ],
        driver_data: &dell_0a5d_platform_data as *const _ as *mut c_void,
    },
    /* AlderLake devices */
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"0B00\0".as_ptr() as *const c_char),
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
        ],
        driver_data: &dell_0b00_platform_data as *const _ as *mut c_void,
    },
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"0B01\0".as_ptr() as *const c_char),
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
        ],
        driver_data: &dell_0b00_platform_data as *const _ as *mut c_void,
    },
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"0AFF\0".as_ptr() as *const c_char),
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
        ],
        driver_data: &dell_0b00_platform_data as *const _ as *mut c_void,
    },
    dmi_system_id {
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, b"Dell Inc\0".as_ptr() as *const c_char),
            DMI_EXACT_MATCH(DMI_PRODUCT_SKU, b"0AFE\0".as_ptr() as *const c_char),
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
        ],
        driver_data: &dell_0b00_platform_data as *const _ as *mut c_void,
    },
    dmi_system_id {
        matches: [
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
            dmi_strmatch {
                slot: 0,
                substr: ptr::null(),
                exact_match: false,
            },
        ],
        driver_data: ptr::null_mut(),
    },
];

unsafe fn rt_amp_add_device_props(sdw_dev: *mut device) -> c_int {
    let mut props: [property_entry; 3] = MaybeUninit::zeroed().assume_init();
    let fwnode: *mut fwnode_handle;
    let dmi_data: *const dmi_system_id;
    let pdata: *const rt_amp_platform_data;
    let mut params: [u8; RT_AMP_MAX_BQ_REG] = [0; RT_AMP_MAX_BQ_REG];
    let ret: c_int;

    dmi_data = dmi_first_match(dmi_platform_data.as_ptr());
    if dmi_data.is_null() {
        return 0;
    }

    pdata = (*dmi_data).driver_data as *const rt_amp_platform_data;
    memcpy(
        params.as_mut_ptr() as *mut c_void,
        (*pdata).bq_params as *const c_void,
        core::mem::size_of::<u8>() * (*pdata).bq_params_cnt as usize,
    );

    props[0] = PROPERTY_ENTRY_U8_ARRAY(b"realtek,bq-params\0".as_ptr() as *const c_char, params.as_ptr());
    props[1] = PROPERTY_ENTRY_U32(
        b"realtek,bq-params-cnt\0".as_ptr() as *const c_char,
        (*pdata).bq_params_cnt,
    );

    fwnode = fwnode_create_software_node(props.as_ptr(), ptr::null());
    if IS_ERR(fwnode as *const c_void) {
        return PTR_ERR(fwnode as *const c_void);
    }

    ret = device_add_software_node(sdw_dev, to_software_node(fwnode));

    fwnode_handle_put(fwnode);

    ret
}

/*
 * dapm routes for rt1308/rt1316/rt1318 will be registered dynamically
 * according to the number of rt1308/rt1316/rt1318 used. The first two
 * entries will be registered for one codec case, and the last two entries
 * are also registered if two 1308s/1316s/1318s are used.
 */
static rt1308_map: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1308-1 SPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1308-1 SPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1308-2 SPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1308-2 SPOR\0".as_ptr() as *const c_char },
];

static rt1316_map: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1316-1 SPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1316-1 SPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1316-2 SPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1316-2 SPOR\0".as_ptr() as *const c_char },
];

static rt1318_map: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1318-1 SPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1318-1 SPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1318-2 SPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1318-2 SPOR\0".as_ptr() as *const c_char },
];

static rt1320_map: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1320-1 SPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1320-1 SPOR\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1320-2 SPOL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"Speaker\0".as_ptr() as *const c_char, control: ptr::null(), source: b"rt1320-2 SPOR\0".as_ptr() as *const c_char },
];

unsafe fn get_codec_name_and_route(
    dai: *mut snd_soc_dai,
    codec_name: *mut c_char,
) -> *const snd_soc_dapm_route {
    /* get the codec name */
    snprintf(codec_name, CODEC_NAME_SIZE, b"%s\0".as_ptr() as *const c_char, (*dai).name);

    /* choose the right codec's map  */
    if strcmp(codec_name, b"rt1308\0".as_ptr() as *const c_char) == 0 {
        rt1308_map.as_ptr()
    } else if strcmp(codec_name, b"rt1316\0".as_ptr() as *const c_char) == 0 {
        rt1316_map.as_ptr()
    } else if strcmp(codec_name, b"rt1318\0".as_ptr() as *const c_char) == 0 {
        rt1318_map.as_ptr()
    } else {
        rt1320_map.as_ptr()
    }
}

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt_amp_spk_rtd_init(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card: *mut snd_soc_card = (*rtd).card;
    let dapm: *mut snd_soc_dapm_context = snd_soc_card_to_dapm(card);
    let rt_amp_map: *const snd_soc_dapm_route;
    let mut codec_name: [c_char; CODEC_NAME_SIZE] = [0; CODEC_NAME_SIZE];
    let mut codec_dai: *mut snd_soc_dai;
    let mut ret: c_int = -EINVAL;
    let mut i: c_int = 0;

    rt_amp_map = get_codec_name_and_route(dai, codec_name.as_mut_ptr());

    /*
     * for_each_rtd_codec_dais(rtd, i, codec_dai)
     * The actual iterator is supplied by ASoC headers.
     */
    while {
        codec_dai = for_each_rtd_codec_dais_next(rtd, &mut i);
        !codec_dai.is_null()
    } {
        if !strstr((*(*codec_dai).component).name_prefix, b"-1\0".as_ptr() as *const c_char).is_null()
        {
            ret = snd_soc_dapm_add_routes(dapm, rt_amp_map, 2);
        } else if !strstr((*(*codec_dai).component).name_prefix, b"-2\0".as_ptr() as *const c_char)
            .is_null()
        {
            ret = snd_soc_dapm_add_routes(dapm, rt_amp_map.add(2), 2);
        }
    }

    ret
}
/* EXPORT_SYMBOL_NS(asoc_sdw_rt_amp_spk_rtd_init, "SND_SOC_SDW_UTILS"); */

unsafe extern "C" {
    fn for_each_rtd_codec_dais_next(
        rtd: *mut snd_soc_pcm_runtime,
        index: *mut c_int,
    ) -> *mut snd_soc_dai;
}

unsafe extern "C" fn rt1308_i2s_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let card: *mut snd_soc_card = (*rtd).card;
    let codec_dai: *mut snd_soc_dai = snd_soc_rtd_to_codec(rtd, 0);
    let clk_id: c_int;
    let clk_freq: c_int;
    let pll_out: c_int;
    let mut err: c_int;

    clk_id = RT1308_PLL_S_MCLK;
    clk_freq = 38400000;

    pll_out = params_rate(params) * 512;

    /* Set rt1308 pll */
    err = snd_soc_dai_set_pll(codec_dai, 0, clk_id, clk_freq as c_uint, pll_out as c_uint);
    if err < 0 {
        dev_err(
            (*card).dev,
            b"Failed to set RT1308 PLL: %d\n\0".as_ptr() as *const c_char,
            err,
        );
        return err;
    }

    /* Set rt1308 sysclk */
    err = snd_soc_dai_set_sysclk(
        codec_dai,
        RT1308_FS_SYS_S_PLL,
        pll_out as c_uint,
        SND_SOC_CLOCK_IN,
    );
    if err < 0 {
        dev_err(
            (*card).dev,
            b"Failed to set RT1308 SYSCLK: %d\n\0".as_ptr() as *const c_char,
            err,
        );
        return err;
    }

    0
}

/* machine stream operations */
#[no_mangle]
pub static soc_sdw_rt1308_i2s_ops: snd_soc_ops = snd_soc_ops {
    hw_params: Some(rt1308_i2s_hw_params),
};
/* EXPORT_SYMBOL_NS(soc_sdw_rt1308_i2s_ops, "SND_SOC_SDW_UTILS"); */

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt_amp_exit(
    card: *mut snd_soc_card,
    _dai_link: *mut snd_soc_dai_link,
) -> c_int {
    let ctx: *mut asoc_sdw_mc_private =
        snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;

    if !(*ctx).amp_dev1.is_null() {
        device_remove_software_node((*ctx).amp_dev1);
        put_device((*ctx).amp_dev1);
        (*ctx).amp_dev1 = ptr::null_mut();
    }

    if !(*ctx).amp_dev2.is_null() {
        device_remove_software_node((*ctx).amp_dev2);
        put_device((*ctx).amp_dev2);
        (*ctx).amp_dev2 = ptr::null_mut();
    }

    0
}
/* EXPORT_SYMBOL_NS(asoc_sdw_rt_amp_exit, "SND_SOC_SDW_UTILS"); */

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt_amp_init(
    card: *mut snd_soc_card,
    dai_links: *mut snd_soc_dai_link,
    info: *mut asoc_sdw_codec_info,
    playback: bool,
) -> c_int {
    let ctx: *mut asoc_sdw_mc_private =
        snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let sdw_dev1: *mut device;
    let sdw_dev2: *mut device;
    let mut ret: c_int;

    /* Count amp number and do init on playback link only. */
    if !playback {
        return 0;
    }

    (*info).amp_num += 1;

    if (*info).amp_num == 2 {
        sdw_dev1 = bus_find_device_by_name(
            &sdw_bus_type as *const _ as *mut bus_type,
            ptr::null_mut(),
            (*(*dai_links).codecs.add(0)).name,
        );
        if sdw_dev1.is_null() {
            return -EPROBE_DEFER;
        }

        ret = rt_amp_add_device_props(sdw_dev1);
        if ret < 0 {
            put_device(sdw_dev1);
            return ret;
        }
        (*ctx).amp_dev1 = sdw_dev1;

        sdw_dev2 = bus_find_device_by_name(
            &sdw_bus_type as *const _ as *mut bus_type,
            ptr::null_mut(),
            (*(*dai_links).codecs.add(1)).name,
        );
        if sdw_dev2.is_null() {
            return -EPROBE_DEFER;
        }

        ret = rt_amp_add_device_props(sdw_dev2);
        if ret < 0 {
            put_device(sdw_dev2);
            return ret;
        }
        (*ctx).amp_dev2 = sdw_dev2;
    }

    0
}
/* EXPORT_SYMBOL_NS(asoc_sdw_rt_amp_init, "SND_SOC_SDW_UTILS"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
