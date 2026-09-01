// SPDX-License-Identifier: GPL-2.0-only
/*
 * Support for gpio amplifier
 *   Copyright 2026 CS GROUP France
 *   Author: Herve Codina <herve.codina@bootlin.com>
 *
 * Basic simple amplifier driver
 *   Copyright (c) 2017 BayLibre, SAS.
 *   Author: Jerome Brunet <jbrunet@baylibre.com>
 */

// Dependencies from the original C file:
// linux/bitmap.h, linux/bits.h, linux/gpio/consumer.h, linux/math.h,
// linux/minmax.h, linux/module.h, linux/platform_device.h,
// linux/regulator/consumer.h, linux/slab.h, sound/soc.h, linux/sort.h,
// sound/tlv.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type u32 = u32;
type s32 = i32;

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_descs {
    pub ndescs: c_uint,
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
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

type c_long = i64;

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_kcontrol_new_tlv {
    pub p: *mut c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
    pub access: c_uint,
    pub tlv: snd_kcontrol_new_tlv,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
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
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct driver_private {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: driver_private,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct simple_amp_single {
    pub gpio: *mut gpio_desc,
    pub is_inverted: bool,
    pub kctrl_val: c_int,
    pub control_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_amp_point {
    pub gpio_val: u32,
    pub gain_db: c_int,
}

#[repr(C)]
pub struct simple_amp_range {
    pub nb_points: c_uint,
    pub min: simple_amp_point,
    pub max: simple_amp_point,
}

#[repr(C)]
pub struct simple_amp_ranges {
    pub nb_ranges: c_uint,
    pub tab_ranges: *mut simple_amp_range,
}

#[repr(C)]
pub struct simple_amp_labels {
    pub nb_labels: c_uint,
    pub tab_labels: *mut *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum simple_amp_mode {
    SIMPLE_AMP_MODE_NONE,
    SIMPLE_AMP_MODE_RANGES,
    SIMPLE_AMP_MODE_LABELS,
}

#[repr(C)]
pub union simple_amp_multi_u {
    pub ranges: core::mem::ManuallyDrop<simple_amp_ranges>,
    pub labels: core::mem::ManuallyDrop<simple_amp_labels>,
}

#[repr(C)]
pub struct simple_amp_multi {
    pub gpios: *mut gpio_descs,
    pub kctrl_val: u32,
    pub kctrl_max: u32,
    pub control_name: *const c_char,
    pub tlv_array: *mut c_uint,
    pub mode: simple_amp_mode,
    pub u: simple_amp_multi_u,
}

#[repr(C)]
pub struct simple_amp_data {
    pub supports: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
}

const SIMPLE_AUDIO_SUPPORT_PGA: c_uint = 1 << 0;
const SIMPLE_AUDIO_SUPPORT_POWER_SUPPLIES: c_uint = 1 << 1;
const SIMPLE_AUDIO_SUPPORT_MUTE: c_uint = 1 << 2;
const SIMPLE_AUDIO_SUPPORT_BYPASS: c_uint = 1 << 3;

#[repr(C)]
pub struct simple_amp {
    pub data: *const simple_amp_data,
    pub gpiod_enable: *mut gpio_desc,
    pub mute: simple_amp_single,
    pub bypass: simple_amp_single,
    pub gain: simple_amp_multi,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const SND_SOC_DAPM_POST_PMU: c_int = 0x1;
const SND_SOC_DAPM_PRE_PMD: c_int = 0x2;
const SND_SOC_NOPM: c_int = 0;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 2;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 1 << 2;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 3;
const SNDRV_CTL_TLVT_DB_RANGE: c_uint = 0x0001;
const SNDRV_CTL_TLVT_DB_MINMAX: c_uint = 0x0004;

extern "C" {
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int) -> c_int;
    fn gpiod_multi_set_value_cansleep(descs: *mut gpio_descs, values: *mut c_ulong) -> c_int;
    fn bitmap_from_arr32(bitmap: *mut c_ulong, buf: *const u32, nbits: c_uint);
    fn snd_soc_add_component_controls(
        component: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;
    fn snd_ctl_enum_info(
        info: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        names: *mut *const c_char,
    ) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_new_controls(
        dapm: *mut snd_soc_dapm_context,
        widget: *const snd_soc_dapm_widget,
        num: c_int,
    ) -> c_int;
    fn snd_soc_dapm_add_routes(
        dapm: *mut snd_soc_dapm_context,
        route: *const snd_soc_dapm_route,
        num: c_int,
    ) -> c_int;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> bool;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn devm_gpiod_get_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn of_property_count_u32_elems(np: *mut device_node, propname: *const c_char) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn of_property_read_u32_index(
        np: *mut device_node,
        propname: *const c_char,
        index: c_uint,
        out_value: *mut u32,
    ) -> c_int;
    fn of_property_read_s32_index(
        np: *mut device_node,
        propname: *const c_char,
        index: c_uint,
        out_value: *mut s32,
    ) -> c_int;
    fn sort(
        base: *mut c_void,
        num: usize,
        size: usize,
        cmp_func: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
        swap_func: *mut c_void,
    );
    fn of_property_count_strings(np: *mut device_node, propname: *const c_char) -> c_int;
    fn of_property_read_string_array(
        np: *mut device_node,
        propname: *const c_char,
        out_strs: *mut *const c_char,
        sz: usize,
    ) -> c_int;
    fn devm_gpiod_get_array_optional(
        dev: *mut device,
        con_id: *const c_char,
        flags: c_uint,
    ) -> *mut gpio_descs;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
}

unsafe extern "C" fn simple_amp_power_event(
    w: *mut snd_soc_dapm_widget,
    _control: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let c = snd_soc_dapm_to_component((*w).dapm);
    let simple_amp = snd_soc_component_get_drvdata(c) as *mut simple_amp;
    let val: c_int;

    match event {
        SND_SOC_DAPM_POST_PMU => val = 1,
        SND_SOC_DAPM_PRE_PMD => val = 0,
        _ => {
            WARN(1, c"Unexpected event".as_ptr());
            return -EINVAL;
        }
    }

    gpiod_set_value_cansleep((*simple_amp).gpiod_enable, val);

    0
}

// The following static DAPM widget initializers use Linux ASoC macros in C:
// SND_SOC_DAPM_INPUT, SND_SOC_DAPM_OUT_DRV_E, SND_SOC_DAPM_OUTPUT,
// SND_SOC_DAPM_REGULATOR_SUPPLY, and SND_SOC_DAPM_PGA_E.
extern "C" {
    static simple_amp_dapm_widgets: [snd_soc_dapm_widget; 6];
    static simple_amp_mono_pga_dapm_widgets: [snd_soc_dapm_widget; 4];
    static simple_amp_stereo_pga_dapm_widgets: [snd_soc_dapm_widget; 6];
}

static simple_amp_dapm_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: c"DRV".as_ptr(), control: ptr::null(), source: c"INL".as_ptr() },
    snd_soc_dapm_route { sink: c"DRV".as_ptr(), control: ptr::null(), source: c"INR".as_ptr() },
    snd_soc_dapm_route { sink: c"OUTL".as_ptr(), control: ptr::null(), source: c"VCC".as_ptr() },
    snd_soc_dapm_route { sink: c"OUTR".as_ptr(), control: ptr::null(), source: c"VCC".as_ptr() },
    snd_soc_dapm_route { sink: c"OUTL".as_ptr(), control: ptr::null(), source: c"DRV".as_ptr() },
    snd_soc_dapm_route { sink: c"OUTR".as_ptr(), control: ptr::null(), source: c"DRV".as_ptr() },
];

static simple_amp_mono_pga_dapm_routes: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route { sink: c"PGA".as_ptr(), control: ptr::null(), source: c"IN".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA".as_ptr(), control: ptr::null(), source: c"vdd".as_ptr() },
    snd_soc_dapm_route { sink: c"OUT".as_ptr(), control: ptr::null(), source: c"PGA".as_ptr() },
];

static simple_amp_stereo_pga_dapm_routes: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route { sink: c"PGA".as_ptr(), control: ptr::null(), source: c"INL".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA".as_ptr(), control: ptr::null(), source: c"INR".as_ptr() },
    snd_soc_dapm_route { sink: c"PGA".as_ptr(), control: ptr::null(), source: c"vdd".as_ptr() },
    snd_soc_dapm_route { sink: c"OUTL".as_ptr(), control: ptr::null(), source: c"PGA".as_ptr() },
    snd_soc_dapm_route { sink: c"OUTR".as_ptr(), control: ptr::null(), source: c"PGA".as_ptr() },
];

unsafe extern "C" fn simple_amp_single_kctrl_write_gpio(
    single: *mut simple_amp_single,
    kctrl_val: c_int,
) -> c_int {
    let gpio_val = if (*single).is_inverted {
        if kctrl_val == 0 { 1 } else { 0 }
    } else {
        kctrl_val
    };

    gpiod_set_value_cansleep((*single).gpio, gpio_val)
}

unsafe extern "C" fn simple_amp_single_kctrl_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    0
}

unsafe extern "C" fn simple_amp_single_kctrl_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let single = (*kcontrol).private_value as *mut simple_amp_single;

    (*ucontrol).value.integer.value[0] = (*single).kctrl_val as c_long;

    0
}

unsafe extern "C" fn simple_amp_single_kctrl_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let single = (*kcontrol).private_value as *mut simple_amp_single;
    let kctrl_val: c_int;
    let err: c_int;

    kctrl_val = if (*ucontrol).value.integer.value[0] != 0 { 1 } else { 0 };

    if kctrl_val == (*single).kctrl_val {
        return 0;
    }

    err = simple_amp_single_kctrl_write_gpio(single, kctrl_val);
    if err != 0 {
        return err;
    }

    (*single).kctrl_val = kctrl_val;

    1 /* The value changed */
}

unsafe extern "C" fn simple_amp_single_add_kcontrol(
    component: *mut snd_soc_component,
    single: *mut simple_amp_single,
) -> c_int {
    let mut control = snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: (*single).control_name,
        info: Some(simple_amp_single_kctrl_info),
        get: Some(simple_amp_single_kctrl_get),
        put: Some(simple_amp_single_kctrl_put),
        private_value: single as c_ulong,
        access: 0,
        tlv: snd_kcontrol_new_tlv { p: ptr::null_mut() },
    };
    let ret: c_int;

    /* Be consistent between single->kctrl_val value and the GPIO value */
    ret = simple_amp_single_kctrl_write_gpio(single, (*single).kctrl_val);
    if ret != 0 {
        return ret;
    }

    snd_soc_add_component_controls(component, &mut control, 1)
}

unsafe extern "C" fn simple_amp_multi_ranges_kctrl_to_gpio(
    kctrl_val: u32,
    ranges: *mut simple_amp_ranges,
) -> u32 {
    let mut range: *mut simple_amp_range;
    let mut index = kctrl_val;
    let mut i: c_uint;

    i = 0;
    while i < (*ranges).nb_ranges {
        range = (*ranges).tab_ranges.add(i as usize);

        if index < (*range).nb_points {
            return if (*range).max.gpio_val >= (*range).min.gpio_val {
                (*range).min.gpio_val.wrapping_add(index)
            } else {
                (*range).min.gpio_val.wrapping_sub(index)
            };
        }

        index = index.wrapping_sub((*range).nb_points);
        i += 1;
    }

    /*
     * Given index out of possible ranges. This is shouldn't happen.
     * Signal the issue and return the maximum value
     */
    WARN(1, c"kctrl_val %u out of ranges\n".as_ptr(), kctrl_val);
    (*(*ranges).tab_ranges.add(((*ranges).nb_ranges - 1) as usize)).max.gpio_val
}

unsafe extern "C" fn simple_amp_multi_kctrl_write_gpios(
    multi: *mut simple_amp_multi,
    kctrl_val: u32,
) -> c_int {
    let mut bm: [c_ulong; 1] = [0; 1];
    let gpio_val: u32;

    if kctrl_val > (*multi).kctrl_max {
        return -EINVAL;
    }

    if (*multi).mode == simple_amp_mode::SIMPLE_AMP_MODE_RANGES {
        gpio_val = simple_amp_multi_ranges_kctrl_to_gpio(
            kctrl_val,
            &mut *(*multi).u.ranges as *mut simple_amp_ranges,
        );
    } else {
        gpio_val = kctrl_val;
    }

    bitmap_from_arr32(bm.as_mut_ptr(), &gpio_val, (*(*multi).gpios).ndescs);

    gpiod_multi_set_value_cansleep((*multi).gpios, bm.as_mut_ptr())
}

unsafe extern "C" fn simple_amp_multi_kctrl_int_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let multi = (*kcontrol).private_value as *mut simple_amp_multi;

    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*multi).kctrl_max as c_long;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    0
}

unsafe extern "C" fn simple_amp_multi_kctrl_int_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let multi = (*kcontrol).private_value as *mut simple_amp_multi;

    (*ucontrol).value.integer.value[0] = (*multi).kctrl_val as c_long;
    0
}

unsafe extern "C" fn simple_amp_multi_kctrl_int_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let multi = (*kcontrol).private_value as *mut simple_amp_multi;
    let kctrl_val: u32;
    let ret: c_int;

    kctrl_val = (*ucontrol).value.integer.value[0] as u32;

    if kctrl_val == (*multi).kctrl_val {
        return 0;
    }

    ret = simple_amp_multi_kctrl_write_gpios(multi, kctrl_val);
    if ret != 0 {
        return ret;
    }

    (*multi).kctrl_val = kctrl_val;

    1 /* The value changed */
}

unsafe extern "C" fn simple_amp_multi_kctrl_enum_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let multi = (*kcontrol).private_value as *mut simple_amp_multi;

    snd_ctl_enum_info(
        uinfo,
        1,
        (*(*multi).u.labels).nb_labels,
        (*(*multi).u.labels).tab_labels,
    )
}

unsafe extern "C" fn simple_amp_multi_kctrl_enum_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let multi = (*kcontrol).private_value as *mut simple_amp_multi;

    (*ucontrol).value.enumerated.item[0] = (*multi).kctrl_val;
    0
}

unsafe extern "C" fn simple_amp_multi_kctrl_enum_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let multi = (*kcontrol).private_value as *mut simple_amp_multi;
    let kctrl_val: u32;
    let ret: c_int;

    kctrl_val = (*ucontrol).value.enumerated.item[0];

    if kctrl_val == (*multi).kctrl_val {
        return 0;
    }

    ret = simple_amp_multi_kctrl_write_gpios(multi, kctrl_val);
    if ret != 0 {
        return ret;
    }

    (*multi).kctrl_val = kctrl_val;

    1 /* The value changed */
}

unsafe extern "C" fn simple_amp_alloc_tlv_ranges(
    ranges: *const simple_amp_ranges,
) -> *mut c_uint {
    let mut index: c_uint;
    let tlv: *mut c_uint;
    let mut t: *mut c_uint;
    let mut i: c_uint;

    tlv = kzalloc(
        (2usize + (*ranges).nb_ranges as usize * 6) * core::mem::size_of::<c_uint>(),
        GFP_KERNEL,
    ) as *mut c_uint;
    if tlv.is_null() {
        return ptr::null_mut();
    }

    t = tlv;

    /* Fill first TLV */
    *t = SNDRV_CTL_TLVT_DB_RANGE; /* Tag */
    t = t.add(1);
    *t = (*ranges).nb_ranges * 6 * core::mem::size_of::<c_uint>() as c_uint; /* Len */
    t = t.add(1);
    /* Ranges are sorted from lower to higher value */
    index = 0;
    i = 0;
    while i < (*ranges).nb_ranges {
        /* Fill range item i */
        *t = index; /* min */
        t = t.add(1);
        index = index.wrapping_add((*(*ranges).tab_ranges.add(i as usize)).nb_points);
        *t = index - 1; /* max */
        t = t.add(1);
        *t = SNDRV_CTL_TLVT_DB_MINMAX; /* Tag */
        t = t.add(1);
        *t = 2 * core::mem::size_of::<c_uint>() as c_uint; /* Len */
        t = t.add(1);
        *t = (*(*ranges).tab_ranges.add(i as usize)).min.gain_db as c_uint; /* min_dB */
        t = t.add(1);
        *t = (*(*ranges).tab_ranges.add(i as usize)).max.gain_db as c_uint; /* max_dB */
        t = t.add(1);
        i += 1;
    }

    tlv
}

unsafe extern "C" fn simple_amp_multi_add_kcontrol(
    component: *mut snd_soc_component,
    multi: *mut simple_amp_multi,
) -> c_int {
    let mut control = snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: (*multi).control_name,
        info: Some(simple_amp_multi_kctrl_int_info),
        get: Some(simple_amp_multi_kctrl_int_get),
        put: Some(simple_amp_multi_kctrl_int_put),
        private_value: multi as c_ulong,
        access: 0,
        tlv: snd_kcontrol_new_tlv { p: ptr::null_mut() },
    };
    let mut ret: c_int;

    match (*multi).mode {
        simple_amp_mode::SIMPLE_AMP_MODE_RANGES => {
            (*multi).tlv_array = simple_amp_alloc_tlv_ranges(&*(*multi).u.ranges);
            if (*multi).tlv_array.is_null() {
                return -ENOMEM;
            }

            control.access = SNDRV_CTL_ELEM_ACCESS_TLV_READ | SNDRV_CTL_ELEM_ACCESS_READWRITE;
            control.tlv.p = (*multi).tlv_array;
        }
        simple_amp_mode::SIMPLE_AMP_MODE_LABELS => {
            /* Use enumerated values */
            control.info = Some(simple_amp_multi_kctrl_enum_info);
            control.get = Some(simple_amp_multi_kctrl_enum_get);
            control.put = Some(simple_amp_multi_kctrl_enum_put);
        }
        simple_amp_mode::SIMPLE_AMP_MODE_NONE => {
            /* Already set control configuration is enough */
        }
    }

    /* Be consistent between multi->kctrl_val value and the GPIOs value */
    ret = simple_amp_multi_kctrl_write_gpios(multi, (*multi).kctrl_val);
    if ret != 0 {
        kfree((*multi).tlv_array as *mut c_void);
        return ret;
    }

    ret = snd_soc_add_component_controls(component, &control, 1);
    if ret != 0 {
        kfree((*multi).tlv_array as *mut c_void);
        return ret;
    }

    0
}

unsafe extern "C" fn simple_amp_add_basic_dapm(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let simple_amp = snd_soc_component_get_drvdata(component) as *mut simple_amp;
    let dev = (*component).dev;
    let mut ret: c_int;

    /* Add basic dapm widgets and routes */
    ret = snd_soc_dapm_new_controls(
        dapm,
        (*(*simple_amp).data).dapm_widgets,
        (*(*simple_amp).data).num_dapm_widgets as c_int,
    );
    if ret != 0 {
        dev_err(dev, c"Failed to add basic dapm widgets (%d)\n".as_ptr(), ret);
        return ret;
    }

    ret = snd_soc_dapm_add_routes(
        dapm,
        (*(*simple_amp).data).dapm_routes,
        (*(*simple_amp).data).num_dapm_routes as c_int,
    );
    if ret != 0 {
        dev_err(dev, c"Failed to add basic dapm routes (%d)\n".as_ptr(), ret);
        return ret;
    }

    0
}

#[repr(C)]
pub struct simple_amp_supply {
    pub prop_name: *const c_char,
    pub dapm_widget: snd_soc_dapm_widget,
    pub dapm_route: snd_soc_dapm_route,
}

// The original simple_amp_supplies static uses SND_SOC_DAPM_REGULATOR_SUPPLY.
extern "C" {
    static simple_amp_supplies: [simple_amp_supply; 4];
}

unsafe extern "C" fn simple_amp_add_power_supplies(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let simple_amp = snd_soc_component_get_drvdata(component) as *mut simple_amp;
    let mut supply: *const simple_amp_supply;
    let dev = (*component).dev;
    let mut ret: c_int;

    /*
     * Those additional power supplies are attached to the PGA.
     * If PGA is not supported, simply skipped them.
     */
    if ((*(*simple_amp).data).supports & SIMPLE_AUDIO_SUPPORT_PGA) == 0 {
        dev_err(dev, c"Extra power supplied need PGA\n".as_ptr());
        return -EINVAL;
    }

    supply = simple_amp_supplies.as_ptr();
    loop {
        if !of_property_present((*dev).of_node, (*supply).prop_name) {
            supply = supply.add(1);
            if (*supply).prop_name.is_null() {
                break;
            }
            continue;
        }

        ret = snd_soc_dapm_new_controls(dapm, &(*supply).dapm_widget, 1);
        if ret != 0 {
            dev_err(
                dev,
                c"Failed to add control for '%s' (%d)\n".as_ptr(),
                (*supply).prop_name,
                ret,
            );
            return ret;
        }
        ret = snd_soc_dapm_add_routes(dapm, &(*supply).dapm_route, 1);
        if ret != 0 {
            dev_err(
                dev,
                c"Failed to add route for '%s' (%d)\n".as_ptr(),
                (*supply).prop_name,
                ret,
            );
            return ret;
        }
        supply = supply.add(1);
        if (*supply).prop_name.is_null() {
            break;
        }
    }

    0
}

unsafe extern "C" fn simple_amp_component_probe(component: *mut snd_soc_component) -> c_int {
    let simple_amp = snd_soc_component_get_drvdata(component) as *mut simple_amp;
    let mut ret: c_int;

    /* Add basic dapm widgets and routes */
    ret = simple_amp_add_basic_dapm(component);
    if ret != 0 {
        return ret;
    }

    /* Add additional power supplies */
    if ((*(*simple_amp).data).supports & SIMPLE_AUDIO_SUPPORT_POWER_SUPPLIES) != 0 {
        ret = simple_amp_add_power_supplies(component);
        if ret != 0 {
            return ret;
        }
    }

    if !(*simple_amp).mute.gpio.is_null() {
        /*
         * The name of the GPIO used is mute. According to this name, 1
         * means muted and 0 means un-muted.
         *
         * An inversion is expected by ALSA. Indeed from ALSA point of
         * view, 1 means 'on' (un-muted) and 0 means 'off' (muted).
         */
        (*simple_amp).mute.is_inverted = true;
        (*simple_amp).mute.kctrl_val = 1; /* Un-muted */
        ret = simple_amp_single_add_kcontrol(component, &mut (*simple_amp).mute);
        if ret != 0 {
            return ret;
        }
    }

    if !(*simple_amp).bypass.gpio.is_null() {
        ret = simple_amp_single_add_kcontrol(component, &mut (*simple_amp).bypass);
        if ret != 0 {
            return ret;
        }
    }

    if !(*simple_amp).gain.gpios.is_null() {
        ret = simple_amp_multi_add_kcontrol(component, &mut (*simple_amp).gain);
        if ret != 0 {
            return ret;
        }
    }

    0
}

unsafe extern "C" fn simple_amp_component_remove(component: *mut snd_soc_component) {
    let simple_amp = snd_soc_component_get_drvdata(component) as *mut simple_amp;

    kfree((*simple_amp).gain.tlv_array as *mut c_void);
    (*simple_amp).gain.tlv_array = ptr::null_mut();
}

static simple_amp_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(simple_amp_component_probe),
    remove: Some(simple_amp_component_remove),
};

unsafe extern "C" fn simple_amp_parse_single_gpio(
    dev: *mut device,
    single: *mut simple_amp_single,
    gpio_property: *const c_char,
) -> c_int {
    /* Start with the inactive value */
    (*single).is_inverted = false;
    (*single).kctrl_val = 0;
    (*single).gpio = devm_gpiod_get_optional(dev, gpio_property, GPIOD_OUT_LOW);
    if IS_ERR((*single).gpio as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*single).gpio as *const c_void),
            c"Failed to get '%s' gpio\n".as_ptr(),
            gpio_property,
        );
    }
    0
}

unsafe extern "C" fn simple_amp_cmp_ranges(a: *const c_void, b: *const c_void) -> c_int {
    let a_range = a as *const simple_amp_range;
    let b_range = b as *const simple_amp_range;

    /* Ranges a and b don't overlap. This has been already checked */

    (*a_range).min.gain_db - (*b_range).max.gain_db
}

unsafe extern "C" fn simple_amp_check_new_range(
    new_range: *const simple_amp_range,
    tab_ranges: *const simple_amp_range,
    nb_ranges: c_uint,
) -> c_int {
    let mut i: c_uint;

    i = 0;
    while i < nb_ranges {
        /* Check for range overlaps */
        if (*new_range).min.gain_db >= (*tab_ranges.add(i as usize)).min.gain_db
            && (*new_range).min.gain_db <= (*tab_ranges.add(i as usize)).max.gain_db
        {
            return -EINVAL;
        }

        if (*new_range).max.gain_db >= (*tab_ranges.add(i as usize)).min.gain_db
            && (*new_range).max.gain_db <= (*tab_ranges.add(i as usize)).max.gain_db
        {
            return -EINVAL;
        }

        if (*new_range).min.gain_db <= (*tab_ranges.add(i as usize)).min.gain_db
            && (*new_range).max.gain_db >= (*tab_ranges.add(i as usize)).max.gain_db
        {
            return -EINVAL;
        }
        i += 1;
    }
    0
}

unsafe extern "C" fn simple_amp_parse_ranges(
    dev: *mut device,
    multi: *mut simple_amp_multi,
    ranges_property: *const c_char,
) -> c_int {
    let ranges = &mut *(*multi).u.ranges as *mut simple_amp_ranges;
    let mut range: *mut simple_amp_range;
    let np = (*dev).of_node;
    let mut first_point: simple_amp_point = core::mem::zeroed();
    let max_gpio_val: c_uint;
    let mut i: c_uint;
    let mut ret: c_int;
    let mut u: u32 = 0;
    let mut s: s32 = 0;

    max_gpio_val = (1u32 << (*(*multi).gpios).ndescs) - 1;

    ret = of_property_count_u32_elems(np, ranges_property);
    if ret < 0 {
        return ret;
    }

    /* The ranges array cannot be empty */
    if ret == 0 {
        return -EINVAL;
    }
    /*
     * One range item is composed of 2 points and each point is composed of
     * 2 values.
     */
    if ret % 4 != 0 {
        return -EINVAL;
    }

    (*ranges).nb_ranges = (ret / 4) as c_uint;

    /* The worst case is one range per possible gpio value */
    if (*ranges).nb_ranges > max_gpio_val + 1 {
        return -EINVAL;
    }

    (*ranges).tab_ranges = devm_kcalloc(
        dev,
        (*ranges).nb_ranges as usize,
        core::mem::size_of::<simple_amp_range>(),
        GFP_KERNEL,
    ) as *mut simple_amp_range;
    if (*ranges).tab_ranges.is_null() {
        return -ENOMEM;
    }

    (*multi).kctrl_max = 0;
    i = 0;
    while i < (*ranges).nb_ranges {
        range = (*ranges).tab_ranges.add(i as usize);

        /* First gpios value */
        ret = of_property_read_u32_index(np, ranges_property, i * 4, &mut u);
        if ret != 0 {
            return ret;
        }
        if u > max_gpio_val {
            return -EINVAL;
        }

        (*range).min.gpio_val = u;

        /* First Gain value */
        ret = of_property_read_s32_index(np, ranges_property, i * 4 + 1, &mut s);
        if ret != 0 {
            return ret;
        }

        (*range).min.gain_db = s;

        /* Second gpios value */
        ret = of_property_read_u32_index(np, ranges_property, i * 4 + 2, &mut u);
        if ret != 0 {
            return ret;
        }
        if u > max_gpio_val {
            return -EINVAL;
        }

        (*range).max.gpio_val = u;

        /* Second Gain value */
        ret = of_property_read_s32_index(np, ranges_property, i * 4 + 3, &mut s);
        if ret != 0 {
            return ret;
        }

        (*range).max.gain_db = s;

        /* Save the first point for later usage */
        if i == 0 {
            first_point = (*range).min;
        }

        /* Fix min and max if needed */
        if (*range).min.gain_db > (*range).max.gain_db {
            core::mem::swap(&mut (*range).min, &mut (*range).max);
        }

        ret = simple_amp_check_new_range(range, (*ranges).tab_ranges, i);
        if ret != 0 {
            return ret;
        }

        (*range).nb_points =
            ((*range).min.gpio_val.abs_diff((*range).max.gpio_val) + 1) as c_uint;

        (*multi).kctrl_max = (*multi).kctrl_max.wrapping_add((*range).nb_points);
        i += 1;
    }

    (*multi).kctrl_max = (*multi).kctrl_max.wrapping_sub(1);

    /* Sort the tab_range array by gain_db value */
    sort(
        (*ranges).tab_ranges as *mut c_void,
        (*ranges).nb_ranges as usize,
        core::mem::size_of::<simple_amp_range>(),
        Some(simple_amp_cmp_ranges),
        ptr::null_mut(),
    );

    /*
     * multi->kctrl_val is the index in tab_ranges.
     *
     * Choose to have the initial amplification value set to the first point
     * available in the first range available in the tab_ranges array before
     * sorting.
     *
     * This first point has been identified before sorting. Search for it in
     * the sorted array in order to set the multi->kctrl_val initial value.
     */
    (*multi).kctrl_val = 0;
    i = 0;
    while i < (*ranges).nb_ranges {
        range = (*ranges).tab_ranges.add(i as usize);

        if (*range).min.gpio_val == first_point.gpio_val
            && (*range).min.gain_db == first_point.gain_db
        {
            break;
        }

        (*multi).kctrl_val = (*multi).kctrl_val.wrapping_add((*range).nb_points);

        if (*range).max.gpio_val == first_point.gpio_val
            && (*range).max.gain_db == first_point.gain_db
        {
            (*multi).kctrl_val = (*multi).kctrl_val.wrapping_sub(1);
            break;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn simple_amp_parse_labels(
    dev: *mut device,
    multi: *mut simple_amp_multi,
    labels_property: *const c_char,
) -> c_int {
    let labels = &mut *(*multi).u.labels as *mut simple_amp_labels;
    let np = (*dev).of_node;
    let mut ret: c_int;

    ret = of_property_count_strings(np, labels_property);
    if ret < 0 {
        return ret;
    }

    /* The labels array cannot be empty */
    if ret == 0 {
        return -EINVAL;
    }

    (*labels).nb_labels = ret as c_uint;
    if (*labels).nb_labels > (1u32 << (*(*multi).gpios).ndescs) {
        return -EINVAL;
    }

    (*labels).tab_labels = devm_kcalloc(
        dev,
        (*labels).nb_labels as usize,
        core::mem::size_of::<*const c_char>(),
        GFP_KERNEL,
    ) as *mut *const c_char;
    if (*labels).tab_labels.is_null() {
        return -ENOMEM;
    }

    (*multi).kctrl_max = (*labels).nb_labels - 1;
    (*multi).kctrl_val = 0;

    of_property_read_string_array(
        np,
        labels_property,
        (*labels).tab_labels,
        (*labels).nb_labels as usize,
    )
}

unsafe extern "C" fn simple_amp_parse_multi_gpio(
    dev: *mut device,
    multi: *mut simple_amp_multi,
    gpios_property: *const c_char,
    ranges_property: *const c_char,
    labels_property: *const c_char,
) -> c_int {
    let np = (*dev).of_node;
    let mut ret: c_int;

    /* Start with the value 0 (GPIO inactive). Can be changed later */
    (*multi).kctrl_val = 0;
    (*multi).gpios = devm_gpiod_get_array_optional(dev, gpios_property, GPIOD_OUT_LOW);
    if IS_ERR((*multi).gpios as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*multi).gpios as *const c_void),
            c"Failed to get '%s' gpios\n".as_ptr(),
            gpios_property,
        );
    }
    if (*multi).gpios.is_null() {
        return 0;
    }

    if (*(*multi).gpios).ndescs > 16 {
        return dev_err_probe(
            dev,
            -EINVAL,
            c"Number of '%s' gpios limited to 16\n".as_ptr(),
            gpios_property,
        );
    }

    /* Set default value for the kctrl_max. Can be changed later */
    (*multi).kctrl_max = (1u32 << (*(*multi).gpios).ndescs) - 1;

    (*multi).mode = simple_amp_mode::SIMPLE_AMP_MODE_NONE;
    if of_property_present(np, ranges_property) {
        ret = simple_amp_parse_ranges(dev, multi, ranges_property);
        if ret < 0 {
            return dev_err_probe(
                dev,
                ret,
                c"Failed to parse '%s'\n".as_ptr(),
                ranges_property,
            );
        }
        (*multi).mode = simple_amp_mode::SIMPLE_AMP_MODE_RANGES;
    } else if of_property_present(np, labels_property) {
        ret = simple_amp_parse_labels(dev, multi, labels_property);
        if ret < 0 {
            return dev_err_probe(
                dev,
                ret,
                c"Failed to parse '%s'\n".as_ptr(),
                labels_property,
            );
        }

        (*multi).mode = simple_amp_mode::SIMPLE_AMP_MODE_LABELS;
    }

    0
}

unsafe extern "C" fn simple_amp_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let simple_amp: *mut simple_amp;
    let mut ret: c_int;

    simple_amp = devm_kzalloc(dev, core::mem::size_of::<simple_amp>(), GFP_KERNEL) as *mut simple_amp;
    if simple_amp.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, simple_amp as *mut c_void);

    (*simple_amp).data = of_device_get_match_data(dev) as *const simple_amp_data;
    if (*simple_amp).data.is_null() {
        return -EINVAL;
    }

    (*simple_amp).gpiod_enable = devm_gpiod_get_optional(dev, c"enable".as_ptr(), GPIOD_OUT_LOW);
    if IS_ERR((*simple_amp).gpiod_enable as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*simple_amp).gpiod_enable as *const c_void),
            c"Failed to get 'enable' gpio".as_ptr(),
        );
    }

    if ((*(*simple_amp).data).supports & SIMPLE_AUDIO_SUPPORT_MUTE) != 0 {
        ret = simple_amp_parse_single_gpio(dev, &mut (*simple_amp).mute, c"mute".as_ptr());
        if ret != 0 {
            return ret;
        }
    }

    if ((*(*simple_amp).data).supports & SIMPLE_AUDIO_SUPPORT_BYPASS) != 0 {
        ret = simple_amp_parse_single_gpio(dev, &mut (*simple_amp).bypass, c"bypass".as_ptr());
        if ret != 0 {
            return ret;
        }
    }

    if ((*(*simple_amp).data).supports & SIMPLE_AUDIO_SUPPORT_PGA) != 0 {
        ret = simple_amp_parse_multi_gpio(
            dev,
            &mut (*simple_amp).gain,
            c"gain".as_ptr(),
            c"gain-ranges".as_ptr(),
            c"gain-labels".as_ptr(),
        );
        if ret != 0 {
            return ret;
        }
    }

    /* Set controls name */
    (*simple_amp).gain.control_name = c"Volume".as_ptr();
    (*simple_amp).mute.control_name = c"Switch".as_ptr();
    (*simple_amp).bypass.control_name = c"Bypass Switch".as_ptr();

    if (*simple_amp).gain.mode == simple_amp_mode::SIMPLE_AMP_MODE_LABELS {
        /*
         * The gain widget control will use enumerated values.
         *
         * Having just "Voltage" and "Switch" widget names with
         * enumerated values and boolean value can confuse ALSA in terms
         * of possible values (strings).
         *
         * Make things clear and avoid the just "Switch" name in that
         * case.
         */
        (*simple_amp).mute.control_name = c"Out Switch".as_ptr();
    }

    devm_snd_soc_register_component(dev, &simple_amp_component_driver, ptr::null_mut(), 0)
}

static simple_audio_amplifier_data: simple_amp_data = simple_amp_data {
    supports: 0,
    dapm_widgets: unsafe { simple_amp_dapm_widgets.as_ptr() },
    num_dapm_widgets: 6,
    dapm_routes: simple_amp_dapm_routes.as_ptr(),
    num_dapm_routes: 6,
};

static simple_audio_mono_pga_data: simple_amp_data = simple_amp_data {
    supports: SIMPLE_AUDIO_SUPPORT_PGA
        | SIMPLE_AUDIO_SUPPORT_POWER_SUPPLIES
        | SIMPLE_AUDIO_SUPPORT_MUTE
        | SIMPLE_AUDIO_SUPPORT_BYPASS,
    dapm_widgets: unsafe { simple_amp_mono_pga_dapm_widgets.as_ptr() },
    num_dapm_widgets: 4,
    dapm_routes: simple_amp_mono_pga_dapm_routes.as_ptr(),
    num_dapm_routes: 3,
};

static simple_audio_stereo_pga_data: simple_amp_data = simple_amp_data {
    supports: SIMPLE_AUDIO_SUPPORT_PGA
        | SIMPLE_AUDIO_SUPPORT_POWER_SUPPLIES
        | SIMPLE_AUDIO_SUPPORT_MUTE
        | SIMPLE_AUDIO_SUPPORT_BYPASS,
    dapm_widgets: unsafe { simple_amp_stereo_pga_dapm_widgets.as_ptr() },
    num_dapm_widgets: 6,
    dapm_routes: simple_amp_stereo_pga_dapm_routes.as_ptr(),
    num_dapm_routes: 5,
};

static simple_amp_ids: [of_device_id; 5] = [
    of_device_id { compatible: c"dioo,dio2125".as_ptr(), data: &simple_audio_amplifier_data as *const _ as *const c_void },
    of_device_id { compatible: c"simple-audio-amplifier".as_ptr(), data: &simple_audio_amplifier_data as *const _ as *const c_void },
    of_device_id { compatible: c"gpio-audio-amp-mono".as_ptr(), data: &simple_audio_mono_pga_data as *const _ as *const c_void },
    of_device_id { compatible: c"gpio-audio-amp-stereo".as_ptr(), data: &simple_audio_stereo_pga_data as *const _ as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, simple_amp_ids);

static mut simple_amp_driver: platform_driver = platform_driver {
    driver: driver_private {
        name: c"simple-amplifier".as_ptr(),
        of_match_table: simple_amp_ids.as_ptr(),
    },
    probe: Some(simple_amp_probe),
};

// module_platform_driver(simple_amp_driver);
// MODULE_DESCRIPTION("ASoC Simple Audio Amplifier driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_AUTHOR("Herve Codina <herve.codina@bootlin.com>");
// MODULE_LICENSE("GPL");

extern "C" {
    fn WARN(condition: c_int, fmt: *const c_char, ...) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
