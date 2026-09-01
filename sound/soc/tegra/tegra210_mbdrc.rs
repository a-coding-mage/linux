// SPDX-License-Identifier: GPL-2.0-only
//
// tegra210_mbdrc.c - Tegra210 MBDRC driver
//
// Copyright (c) 2022, NVIDIA CORPORATION. All rights reserved.

// Translated from the implementation source. Kernel, ALSA SoC, regmap, device
// tree, and Tegra header dependencies are expected to be supplied externally.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u32 = u32;
type size_t = usize;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    pub flags: c_uint,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub name: *const c_char,
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool_>,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub reg_default_cb: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_uint>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
    pub shift: c_uint,
    pub max: c_uint,
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub mask: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
}

#[repr(C)]
pub struct soc_bytes {
    pub base: u32,
    pub num_regs: u32,
    pub mask: u32,
}

#[repr(C)]
pub struct tegra_soc_bytes {
    pub soc: soc_bytes,
    pub shift: u32,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_value_bytes {
    pub data: [u8; 512],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub bytes: snd_ctl_elem_value_bytes,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct tegra210_ope {
    pub mbdrc_regmap: *mut regmap,
}

#[repr(C)]
pub struct tegra210_mbdrc_band_params {
    pub band: c_uint,
    pub iir_stages: u32,
    pub in_attack_tc: u32,
    pub in_release_tc: u32,
    pub fast_attack_tc: u32,
    pub in_threshold: [u32; 4],
    pub out_threshold: [u32; 4],
    pub ratio: [u32; 5],
    pub makeup_gain: u32,
    pub gain_init: u32,
    pub gain_attack_tc: u32,
    pub gain_release_tc: u32,
    pub fast_release_tc: u32,
    pub biquad_params: [i32; TEGRA210_MBDRC_MAX_BIQUAD_STAGES as usize * 5],
}

#[repr(C)]
pub struct tegra210_mbdrc_config {
    pub mode: u32,
    pub rms_off: u32,
    pub peak_rms_mode: u32,
    pub filter_structure: u32,
    pub shift_ctrl: u32,
    pub frame_size: u32,
    pub channel_mask: u32,
    pub fa_factor: u32,
    pub fr_factor: u32,
    pub band_params: [tegra210_mbdrc_band_params; MBDRC_NUM_BAND as usize],
}

extern "C" {
    static mut regmap_default_zero_cb: Option<unsafe extern "C" fn(*mut device, c_uint) -> c_uint>;

    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits_check(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
        change: *mut bool_,
    ) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool_);

    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(cmpnt: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_regmap_val_bytes(cmpnt: *mut snd_soc_component) -> c_int;
    fn snd_soc_add_component_controls(
        cmpnt: *mut snd_soc_component,
        controls: *const snd_kcontrol_new,
        num_controls: c_uint,
    ) -> c_int;

    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;

    fn of_get_child_by_name(node: *mut device_node, name: *const c_char) -> *mut device_node;
    fn of_address_to_resource(node: *mut device_node, index: c_uint, res: *mut resource) -> c_int;
    fn of_node_put(node: *mut device_node);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_ioremap_resource(dev: *mut device, res: *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

const fn MBDRC_FILTER_REG(reg: c_uint, id: c_uint) -> c_uint {
    reg + id * TEGRA210_MBDRC_FILTER_PARAM_STRIDE
}

macro_rules! MBDRC_FILTER_PARAM_REG_DEFAULTS {
    ($reg:expr, $val:expr) => {
        reg_default { reg: MBDRC_FILTER_REG($reg, 0), def: $val },
        reg_default { reg: MBDRC_FILTER_REG($reg, 1), def: $val },
        reg_default { reg: MBDRC_FILTER_REG($reg, 2), def: $val }
    };
}

static tegra210_mbdrc_reg_defaults: [reg_default; 54] = [
    reg_default { reg: TEGRA210_MBDRC_CFG, def: 0x0030de51 },
    reg_default { reg: TEGRA210_MBDRC_CHANNEL_MASK, def: 0x00000003 },
    reg_default { reg: TEGRA210_MBDRC_FAST_FACTOR, def: 0x30000800 },
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_IIR_CFG, 0x00000005),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_IN_ATTACK, 0x3e48590c),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_IN_RELEASE, 0x08414e9f),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_FAST_ATTACK, 0x7fffffff),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_IN_THRESHOLD, 0x06145082),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_OUT_THRESHOLD, 0x060d379b),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_RATIO_1ST, 0x0000a000),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_RATIO_2ND, 0x00002000),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_RATIO_3RD, 0x00000b33),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_RATIO_4TH, 0x00000800),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_RATIO_5TH, 0x0000019a),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_MAKEUP_GAIN, 0x00000002),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_INIT_GAIN, 0x00066666),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_GAIN_ATTACK, 0x00d9ba0e),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_GAIN_RELEASE, 0x3e48590c),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_FAST_RELEASE, 0x7ffff26a),
    MBDRC_FILTER_PARAM_REG_DEFAULTS!(TEGRA210_MBDRC_CFG_RAM_CTRL, 0x4000),
];

/* Default MBDRC parameters */
static mbdrc_init_config: tegra210_mbdrc_config = tegra210_mbdrc_config {
    mode: 0, /* Bypass */
    rms_off: 48,
    peak_rms_mode: 1, /* PEAK */
    filter_structure: 0, /* All-pass tree */
    shift_ctrl: 30,
    frame_size: 32,
    channel_mask: 0x3,
    fa_factor: 2048,
    fr_factor: 14747,
    band_params: [
        tegra210_mbdrc_band_params {
            band: MBDRC_LOW_BAND,
            iir_stages: 5,
            in_attack_tc: 1044928780,
            in_release_tc: 138497695,
            fast_attack_tc: 2147483647,
            in_threshold: [130, 80, 20, 6],
            out_threshold: [155, 55, 13, 6],
            ratio: [40960, 8192, 2867, 2048, 410],
            makeup_gain: 4,
            gain_init: 419430,
            gain_attack_tc: 14268942,
            gain_release_tc: 1440547090,
            fast_release_tc: 2147480170,
            biquad_params: [
                /*
                 * Gains:
                 *
                 * b0, b1, a0,
                 * a1, a2,
                 */
                /* Band-0 */
                961046798, -2030431983, 1073741824, 2030431983, -961046798,
                /* Band-1 */
                1030244425, -2099481453, 1073741824, 2099481453, -1030244425,
                /* Band-2 */
                1067169294, -2136327263, 1073741824, 2136327263, -1067169294,
                /* Band-3 */
                434951949, -1306567134, 1073741824, 1306567134, -434951949,
                /* Band-4 */
                780656019, -1605955641, 1073741824, 1605955641, -780656019,
                /* Band-5 */
                1024497031, -1817128152, 1073741824, 1817128152, -1024497031,
                /* Band-6 */
                1073741824, 0, 0, 0, 0,
                /* Band-7 */
                1073741824, 0, 0, 0, 0,
            ],
        },
        tegra210_mbdrc_band_params {
            band: MBDRC_MID_BAND,
            iir_stages: 5,
            in_attack_tc: 1581413104,
            in_release_tc: 35494783,
            fast_attack_tc: 2147483647,
            in_threshold: [130, 50, 30, 6],
            out_threshold: [106, 50, 30, 13],
            ratio: [40960, 2867, 4096, 2867, 410],
            makeup_gain: 6,
            gain_init: 419430,
            gain_attack_tc: 4766887,
            gain_release_tc: 1044928780,
            fast_release_tc: 2147480170,
            biquad_params: [
                /*
                 * Gains:
                 *
                 * b0, b1, a0,
                 * a1, a2,
                 */
                /* Band-0 */
                -1005668963, 1073741824, 0, 1005668963, 0,
                /* Band-1 */
                998437058, -2067742187, 1073741824, 2067742187, -998437058,
                /* Band-2 */
                1051963422, -2121153948, 1073741824, 2121153948, -1051963422,
                /* Band-3 */
                434951949, -1306567134, 1073741824, 1306567134, -434951949,
                /* Band-4 */
                780656019, -1605955641, 1073741824, 1605955641, -780656019,
                /* Band-5 */
                1024497031, -1817128152, 1073741824, 1817128152, -1024497031,
                /* Band-6 */
                1073741824, 0, 0, 0, 0,
                /* Band-7 */
                1073741824, 0, 0, 0, 0,
            ],
        },
        tegra210_mbdrc_band_params {
            band: MBDRC_HIGH_BAND,
            iir_stages: 5,
            in_attack_tc: 2144750688,
            in_release_tc: 70402888,
            fast_attack_tc: 2147483647,
            in_threshold: [130, 50, 30, 6],
            out_threshold: [106, 50, 30, 13],
            ratio: [40960, 2867, 4096, 2867, 410],
            makeup_gain: 6,
            gain_init: 419430,
            gain_attack_tc: 4766887,
            gain_release_tc: 1044928780,
            fast_release_tc: 2147480170,
            biquad_params: [
                /*
                 * Gains:
                 *
                 * b0, b1, a0,
                 * a1, a2,
                 */
                /* Band-0 */
                1073741824, 0, 0, 0, 0,
                /* Band-1 */
                1073741824, 0, 0, 0, 0,
                /* Band-2 */
                1073741824, 0, 0, 0, 0,
                /* Band-3 */
                -619925131, 1073741824, 0, 619925131, 0,
                /* Band-4 */
                606839335, -1455425976, 1073741824, 1455425976, -606839335,
                /* Band-5 */
                917759617, -1724690840, 1073741824, 1724690840, -917759617,
                /* Band-6 */
                1073741824, 0, 0, 0, 0,
                /* Band-7 */
                1073741824, 0, 0, 0, 0,
            ],
        },
    ],
};

unsafe extern "C" fn tegra210_mbdrc_write_ram(
    regmap: *mut regmap,
    reg_ctrl: c_uint,
    reg_data: c_uint,
    ram_offset: c_uint,
    data: *mut c_uint,
    size: size_t,
) {
    let mut val: c_uint;
    let mut i: c_uint;

    val = ram_offset & TEGRA210_MBDRC_RAM_CTRL_RAM_ADDR_MASK;
    val |= TEGRA210_MBDRC_RAM_CTRL_ADDR_INIT_EN;
    val |= TEGRA210_MBDRC_RAM_CTRL_SEQ_ACCESS_EN;
    val |= TEGRA210_MBDRC_RAM_CTRL_RW_WRITE;

    regmap_write(regmap, reg_ctrl, val);

    i = 0;
    while (i as size_t) < size {
        regmap_write(regmap, reg_data, *data.add(i as usize));
        i += 1;
    }
}

unsafe extern "C" fn tegra210_mbdrc_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let mut val: c_uint = 0;

    regmap_read((*ope).mbdrc_regmap, (*mc).reg, &mut val);

    (*ucontrol).value.integer.value[0] = ((val >> (*mc).shift) & (*mc).max) as i64;

    0
}

unsafe extern "C" fn tegra210_mbdrc_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let mut val: c_uint = (*ucontrol).value.integer.value[0] as c_uint;
    let mut change: bool_ = false;

    val <<= (*mc).shift;

    regmap_update_bits_check(
        (*ope).mbdrc_regmap,
        (*mc).reg,
        (*mc).max << (*mc).shift,
        val,
        &mut change,
    );

    if change { 1 } else { 0 }
}

unsafe extern "C" fn tegra210_mbdrc_get_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let mut val: c_uint = 0;

    regmap_read((*ope).mbdrc_regmap, (*e).reg, &mut val);

    (*ucontrol).value.enumerated.item[0] = (val >> (*e).shift_l) & (*e).mask;

    0
}

unsafe extern "C" fn tegra210_mbdrc_put_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let e = (*kcontrol).private_value as *mut soc_enum;
    let mut change: bool_ = false;
    let val: c_uint;
    let mask: c_uint;

    if (*ucontrol).value.enumerated.item[0] > (*e).items - 1 {
        return -EINVAL;
    }

    val = (*ucontrol).value.enumerated.item[0] << (*e).shift_l;
    mask = (*e).mask << (*e).shift_l;

    regmap_update_bits_check((*ope).mbdrc_regmap, (*e).reg, mask, val, &mut change);

    if change { 1 } else { 0 }
}

unsafe extern "C" fn tegra210_mbdrc_band_params_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let params = (*kcontrol).private_value as *mut tegra_soc_bytes;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let val_bytes = snd_soc_component_regmap_val_bytes(cmpnt) as u32;
    let data = (*ucontrol).value.bytes.data.as_mut_ptr() as *mut u32;
    let mut regs = (*params).soc.base;
    let mask = (*params).soc.mask;
    let shift = (*params).shift;
    let mut i: c_uint = 0;

    while i < (*params).soc.num_regs {
        regmap_read((*ope).mbdrc_regmap, regs, data.add(i as usize));
        *data.add(i as usize) = (*data.add(i as usize) & mask) >> shift;
        i += 1;
        regs += val_bytes;
    }

    0
}

unsafe extern "C" fn tegra210_mbdrc_band_params_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let params = (*kcontrol).private_value as *mut tegra_soc_bytes;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let val_bytes = snd_soc_component_regmap_val_bytes(cmpnt) as u32;
    let data = (*ucontrol).value.bytes.data.as_mut_ptr() as *mut u32;
    let mut regs = (*params).soc.base;
    let mask = (*params).soc.mask;
    let shift = (*params).shift;
    let mut change: bool_ = false;
    let mut i: c_uint = 0;

    while i < (*params).soc.num_regs {
        let mut update: bool_ = false;
        regmap_update_bits_check(
            (*ope).mbdrc_regmap,
            regs,
            mask,
            *data.add(i as usize) << shift,
            &mut update,
        );
        change |= update;
        i += 1;
        regs += val_bytes;
    }

    if change { 1 } else { 0 }
}

unsafe extern "C" fn tegra210_mbdrc_threshold_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let params = (*kcontrol).private_value as *mut tegra_soc_bytes;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let val_bytes = snd_soc_component_regmap_val_bytes(cmpnt) as u32;
    let data = (*ucontrol).value.bytes.data.as_mut_ptr() as *mut u32;
    let mut regs = (*params).soc.base;
    let num_regs = (*params).soc.num_regs;
    let mut val: u32 = 0;
    let mut i: c_uint = 0;

    while i < num_regs {
        regmap_read((*ope).mbdrc_regmap, regs, &mut val);

        *data.add(i as usize) =
            (val & TEGRA210_MBDRC_THRESH_1ST_MASK) >> TEGRA210_MBDRC_THRESH_1ST_SHIFT;
        *data.add(i as usize + 1) =
            (val & TEGRA210_MBDRC_THRESH_2ND_MASK) >> TEGRA210_MBDRC_THRESH_2ND_SHIFT;
        *data.add(i as usize + 2) =
            (val & TEGRA210_MBDRC_THRESH_3RD_MASK) >> TEGRA210_MBDRC_THRESH_3RD_SHIFT;
        *data.add(i as usize + 3) =
            (val & TEGRA210_MBDRC_THRESH_4TH_MASK) >> TEGRA210_MBDRC_THRESH_4TH_SHIFT;

        i += 4;
        regs += val_bytes;
    }

    0
}

unsafe extern "C" fn tegra210_mbdrc_threshold_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let params = (*kcontrol).private_value as *mut tegra_soc_bytes;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let val_bytes = snd_soc_component_regmap_val_bytes(cmpnt) as u32;
    let data = (*ucontrol).value.bytes.data.as_mut_ptr() as *mut u32;
    let mut regs = (*params).soc.base;
    let num_regs = (*params).soc.num_regs;
    let mut change: bool_ = false;
    let mut i: c_uint = 0;

    while i < num_regs {
        let mut update: bool_ = false;

        *data.add(i as usize) = (((*data.add(i as usize) >> TEGRA210_MBDRC_THRESH_1ST_SHIFT)
            & TEGRA210_MBDRC_THRESH_1ST_MASK)
            | (((*data.add(i as usize + 1) >> TEGRA210_MBDRC_THRESH_2ND_SHIFT)
                & TEGRA210_MBDRC_THRESH_2ND_MASK))
            | (((*data.add(i as usize + 2) >> TEGRA210_MBDRC_THRESH_3RD_SHIFT)
                & TEGRA210_MBDRC_THRESH_3RD_MASK))
            | (((*data.add(i as usize + 3) >> TEGRA210_MBDRC_THRESH_4TH_SHIFT)
                & TEGRA210_MBDRC_THRESH_4TH_MASK)));

        regmap_update_bits_check(
            (*ope).mbdrc_regmap,
            regs,
            0xffffffff,
            *data.add(i as usize),
            &mut update,
        );

        change |= update;
        i += 4;
        regs += val_bytes;
    }

    if change { 1 } else { 0 }
}

unsafe extern "C" fn tegra210_mbdrc_biquad_coeffs_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let params = (*kcontrol).private_value as *mut tegra_soc_bytes;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let val_bytes = snd_soc_component_regmap_val_bytes(cmpnt) as usize;
    let data = (*ucontrol).value.bytes.data.as_mut_ptr() as *mut u32;

    ptr::write_bytes(data as *mut u8, 0, (*params).soc.num_regs as usize * val_bytes);

    0
}

unsafe extern "C" fn tegra210_mbdrc_biquad_coeffs_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let params = (*kcontrol).private_value as *mut tegra_soc_bytes;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let val_bytes = snd_soc_component_regmap_val_bytes(cmpnt) as u32;
    let reg_ctrl = (*params).soc.base;
    let reg_data = reg_ctrl + val_bytes;
    let data = (*ucontrol).value.bytes.data.as_mut_ptr() as *mut u32;

    tegra210_mbdrc_write_ram(
        (*ope).mbdrc_regmap,
        reg_ctrl,
        reg_data,
        (*params).shift,
        data,
        (*params).soc.num_regs as usize,
    );

    1
}

unsafe extern "C" fn tegra210_mbdrc_param_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let params = (*kcontrol).private_value as *mut soc_bytes;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*uinfo).count = (*params).num_regs * size_of::<u32>() as u32;

    0
}

unsafe extern "C" fn tegra210_mbdrc_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let mut val: c_uint = 0;

    regmap_read((*ope).mbdrc_regmap, (*mc).reg, &mut val);

    (*ucontrol).value.integer.value[0] =
        ((val >> (*mc).shift) as c_int - TEGRA210_MBDRC_MASTER_VOL_MIN) as i64;

    0
}

unsafe extern "C" fn tegra210_mbdrc_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = (*kcontrol).private_value as *mut soc_mixer_control;
    let cmpnt = snd_kcontrol_chip(kcontrol);
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let mut val: c_int = (*ucontrol).value.integer.value[0] as c_int;
    let mut change: bool_ = false;

    val += TEGRA210_MBDRC_MASTER_VOL_MIN;

    regmap_update_bits_check(
        (*ope).mbdrc_regmap,
        (*mc).reg,
        (*mc).max << (*mc).shift,
        (val as c_uint) << (*mc).shift,
        &mut change,
    );

    regmap_read((*ope).mbdrc_regmap, (*mc).reg, &mut (val as c_uint));

    if change { 1 } else { 0 }
}

static tegra210_mbdrc_mode_text: [*const c_char; 4] = [
    b"Bypass\0".as_ptr() as *const c_char,
    b"Fullband\0".as_ptr() as *const c_char,
    b"Dualband\0".as_ptr() as *const c_char,
    b"Multiband\0".as_ptr() as *const c_char,
];

static tegra210_mbdrc_mode_enum: soc_enum = SOC_ENUM_SINGLE!(
    TEGRA210_MBDRC_CFG,
    TEGRA210_MBDRC_CFG_MBDRC_MODE_SHIFT,
    4,
    tegra210_mbdrc_mode_text
);

static tegra210_mbdrc_peak_rms_text: [*const c_char; 2] = [
    b"Peak\0".as_ptr() as *const c_char,
    b"RMS\0".as_ptr() as *const c_char,
];

static tegra210_mbdrc_peak_rms_enum: soc_enum = SOC_ENUM_SINGLE!(
    TEGRA210_MBDRC_CFG,
    TEGRA210_MBDRC_CFG_PEAK_RMS_SHIFT,
    2,
    tegra210_mbdrc_peak_rms_text
);

static tegra210_mbdrc_filter_structure_text: [*const c_char; 2] = [
    b"All-pass-tree\0".as_ptr() as *const c_char,
    b"Flexible\0".as_ptr() as *const c_char,
];

static tegra210_mbdrc_filter_structure_enum: soc_enum = SOC_ENUM_SINGLE!(
    TEGRA210_MBDRC_CFG,
    TEGRA210_MBDRC_CFG_FILTER_STRUCTURE_SHIFT,
    2,
    tegra210_mbdrc_filter_structure_text
);

static tegra210_mbdrc_frame_size_text: [*const c_char; 7] = [
    b"N1\0".as_ptr() as *const c_char,
    b"N2\0".as_ptr() as *const c_char,
    b"N4\0".as_ptr() as *const c_char,
    b"N8\0".as_ptr() as *const c_char,
    b"N16\0".as_ptr() as *const c_char,
    b"N32\0".as_ptr() as *const c_char,
    b"N64\0".as_ptr() as *const c_char,
];

static tegra210_mbdrc_frame_size_enum: soc_enum = SOC_ENUM_SINGLE!(
    TEGRA210_MBDRC_CFG,
    TEGRA210_MBDRC_CFG_FRAME_SIZE_SHIFT,
    7,
    tegra210_mbdrc_frame_size_text
);

macro_rules! TEGRA_MBDRC_BYTES_EXT {
    ($xname:expr, $xbase:expr, $xregs:expr, $xshift:expr, $xmask:expr, $xinfo:expr) => {
        TEGRA_SOC_BYTES_EXT!(
            $xname,
            $xbase,
            $xregs,
            $xshift,
            $xmask,
            tegra210_mbdrc_band_params_get,
            tegra210_mbdrc_band_params_put,
            tegra210_mbdrc_param_info
        )
    };
}

macro_rules! TEGRA_MBDRC_BAND_BYTES_EXT {
    ($xname:expr, $xbase:expr, $xshift:expr, $xmask:expr, $xinfo:expr) => {
        TEGRA_MBDRC_BYTES_EXT!(
            $xname,
            $xbase,
            TEGRA210_MBDRC_FILTER_COUNT,
            $xshift,
            $xmask,
            $xinfo
        )
    };
}

static mdbrc_vol_tlv: [c_uint; 4] = DECLARE_TLV_DB_MINMAX!(-25600, 25500);

static tegra210_mbdrc_controls: [snd_kcontrol_new; 24] = [
    SOC_ENUM_EXT!(
        "MBDRC Peak RMS Mode",
        tegra210_mbdrc_peak_rms_enum,
        tegra210_mbdrc_get_enum,
        tegra210_mbdrc_put_enum
    ),
    SOC_ENUM_EXT!(
        "MBDRC Filter Structure",
        tegra210_mbdrc_filter_structure_enum,
        tegra210_mbdrc_get_enum,
        tegra210_mbdrc_put_enum
    ),
    SOC_ENUM_EXT!(
        "MBDRC Frame Size",
        tegra210_mbdrc_frame_size_enum,
        tegra210_mbdrc_get_enum,
        tegra210_mbdrc_put_enum
    ),
    SOC_ENUM_EXT!(
        "MBDRC Mode",
        tegra210_mbdrc_mode_enum,
        tegra210_mbdrc_get_enum,
        tegra210_mbdrc_put_enum
    ),
    SOC_SINGLE_EXT!(
        "MBDRC RMS Offset",
        TEGRA210_MBDRC_CFG,
        TEGRA210_MBDRC_CFG_RMS_OFFSET_SHIFT,
        0x1ff,
        0,
        tegra210_mbdrc_get,
        tegra210_mbdrc_put
    ),
    SOC_SINGLE_EXT!(
        "MBDRC Shift Control",
        TEGRA210_MBDRC_CFG,
        TEGRA210_MBDRC_CFG_SHIFT_CTRL_SHIFT,
        0x1f,
        0,
        tegra210_mbdrc_get,
        tegra210_mbdrc_put
    ),
    SOC_SINGLE_EXT!(
        "MBDRC Fast Attack Factor",
        TEGRA210_MBDRC_FAST_FACTOR,
        TEGRA210_MBDRC_FAST_FACTOR_ATTACK_SHIFT,
        0xffff,
        0,
        tegra210_mbdrc_get,
        tegra210_mbdrc_put
    ),
    SOC_SINGLE_EXT!(
        "MBDRC Fast Release Factor",
        TEGRA210_MBDRC_FAST_FACTOR,
        TEGRA210_MBDRC_FAST_FACTOR_RELEASE_SHIFT,
        0xffff,
        0,
        tegra210_mbdrc_get,
        tegra210_mbdrc_put
    ),
    SOC_SINGLE_RANGE_EXT_TLV!(
        "MBDRC Master Volume",
        TEGRA210_MBDRC_MASTER_VOL,
        TEGRA210_MBDRC_MASTER_VOL_SHIFT,
        0,
        0x1ff,
        0,
        tegra210_mbdrc_vol_get,
        tegra210_mbdrc_vol_put,
        mdbrc_vol_tlv
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC IIR Stages",
        TEGRA210_MBDRC_IIR_CFG,
        TEGRA210_MBDRC_FILTER_COUNT,
        TEGRA210_MBDRC_IIR_CFG_NUM_STAGES_SHIFT,
        TEGRA210_MBDRC_IIR_CFG_NUM_STAGES_MASK,
        tegra210_mbdrc_band_params_get,
        tegra210_mbdrc_band_params_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC In Attack Time Const",
        TEGRA210_MBDRC_IN_ATTACK,
        TEGRA210_MBDRC_FILTER_COUNT,
        TEGRA210_MBDRC_IN_ATTACK_TC_SHIFT,
        TEGRA210_MBDRC_IN_ATTACK_TC_MASK,
        tegra210_mbdrc_band_params_get,
        tegra210_mbdrc_band_params_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC In Release Time Const",
        TEGRA210_MBDRC_IN_RELEASE,
        TEGRA210_MBDRC_FILTER_COUNT,
        TEGRA210_MBDRC_IN_RELEASE_TC_SHIFT,
        TEGRA210_MBDRC_IN_RELEASE_TC_MASK,
        tegra210_mbdrc_band_params_get,
        tegra210_mbdrc_band_params_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC Fast Attack Time Const",
        TEGRA210_MBDRC_FAST_ATTACK,
        TEGRA210_MBDRC_FILTER_COUNT,
        TEGRA210_MBDRC_FAST_ATTACK_TC_SHIFT,
        TEGRA210_MBDRC_FAST_ATTACK_TC_MASK,
        tegra210_mbdrc_band_params_get,
        tegra210_mbdrc_band_params_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC In Threshold",
        TEGRA210_MBDRC_IN_THRESHOLD,
        TEGRA210_MBDRC_FILTER_COUNT * 4,
        0,
        0xffffffff,
        tegra210_mbdrc_threshold_get,
        tegra210_mbdrc_threshold_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC Out Threshold",
        TEGRA210_MBDRC_OUT_THRESHOLD,
        TEGRA210_MBDRC_FILTER_COUNT * 4,
        0,
        0xffffffff,
        tegra210_mbdrc_threshold_get,
        tegra210_mbdrc_threshold_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC Ratio",
        TEGRA210_MBDRC_RATIO_1ST,
        TEGRA210_MBDRC_FILTER_COUNT * 5,
        TEGRA210_MBDRC_RATIO_1ST_SHIFT,
        TEGRA210_MBDRC_RATIO_1ST_MASK,
        tegra210_mbdrc_band_params_get,
        tegra210_mbdrc_band_params_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC Makeup Gain",
        TEGRA210_MBDRC_MAKEUP_GAIN,
        TEGRA210_MBDRC_FILTER_COUNT,
        TEGRA210_MBDRC_MAKEUP_GAIN_SHIFT,
        TEGRA210_MBDRC_MAKEUP_GAIN_MASK,
        tegra210_mbdrc_band_params_get,
        tegra210_mbdrc_band_params_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC Init Gain",
        TEGRA210_MBDRC_INIT_GAIN,
        TEGRA210_MBDRC_FILTER_COUNT,
        TEGRA210_MBDRC_INIT_GAIN_SHIFT,
        TEGRA210_MBDRC_INIT_GAIN_MASK,
        tegra210_mbdrc_band_params_get,
        tegra210_mbdrc_band_params_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC Attack Gain",
        TEGRA210_MBDRC_GAIN_ATTACK,
        TEGRA210_MBDRC_FILTER_COUNT,
        TEGRA210_MBDRC_GAIN_ATTACK_SHIFT,
        TEGRA210_MBDRC_GAIN_ATTACK_MASK,
        tegra210_mbdrc_band_params_get,
        tegra210_mbdrc_band_params_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC Release Gain",
        TEGRA210_MBDRC_GAIN_RELEASE,
        TEGRA210_MBDRC_FILTER_COUNT,
        TEGRA210_MBDRC_GAIN_RELEASE_SHIFT,
        TEGRA210_MBDRC_GAIN_RELEASE_MASK,
        tegra210_mbdrc_band_params_get,
        tegra210_mbdrc_band_params_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC Fast Release Gain",
        TEGRA210_MBDRC_FAST_RELEASE,
        TEGRA210_MBDRC_FILTER_COUNT,
        TEGRA210_MBDRC_FAST_RELEASE_SHIFT,
        TEGRA210_MBDRC_FAST_RELEASE_MASK,
        tegra210_mbdrc_band_params_get,
        tegra210_mbdrc_band_params_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC Low Band Biquad Coeffs",
        TEGRA210_MBDRC_CFG_RAM_CTRL,
        TEGRA210_MBDRC_MAX_BIQUAD_STAGES * 5,
        0,
        0xffffffff,
        tegra210_mbdrc_biquad_coeffs_get,
        tegra210_mbdrc_biquad_coeffs_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC Mid Band Biquad Coeffs",
        TEGRA210_MBDRC_CFG_RAM_CTRL + TEGRA210_MBDRC_FILTER_PARAM_STRIDE,
        TEGRA210_MBDRC_MAX_BIQUAD_STAGES * 5,
        0,
        0xffffffff,
        tegra210_mbdrc_biquad_coeffs_get,
        tegra210_mbdrc_biquad_coeffs_put,
        tegra210_mbdrc_param_info
    ),
    TEGRA_SOC_BYTES_EXT!(
        "MBDRC High Band Biquad Coeffs",
        TEGRA210_MBDRC_CFG_RAM_CTRL + TEGRA210_MBDRC_FILTER_PARAM_STRIDE * 2,
        TEGRA210_MBDRC_MAX_BIQUAD_STAGES * 5,
        0,
        0xffffffff,
        tegra210_mbdrc_biquad_coeffs_get,
        tegra210_mbdrc_biquad_coeffs_put,
        tegra210_mbdrc_param_info
    ),
];

unsafe extern "C" fn tegra210_mbdrc_wr_reg(_dev: *mut device, mut reg: c_uint) -> bool_ {
    if reg >= TEGRA210_MBDRC_IIR_CFG {
        reg -= (reg - TEGRA210_MBDRC_IIR_CFG)
            % (TEGRA210_MBDRC_FILTER_PARAM_STRIDE * TEGRA210_MBDRC_FILTER_COUNT);
    }

    match reg {
        TEGRA210_MBDRC_SOFT_RESET | TEGRA210_MBDRC_CG => true,
        r if r >= TEGRA210_MBDRC_CFG && r <= TEGRA210_MBDRC_CFG_RAM_DATA => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra210_mbdrc_rd_reg(dev: *mut device, mut reg: c_uint) -> bool_ {
    if tegra210_mbdrc_wr_reg(dev, reg) {
        return true;
    }

    if reg >= TEGRA210_MBDRC_IIR_CFG {
        reg -= (reg - TEGRA210_MBDRC_IIR_CFG)
            % (TEGRA210_MBDRC_FILTER_PARAM_STRIDE * TEGRA210_MBDRC_FILTER_COUNT);
    }

    match reg {
        TEGRA210_MBDRC_STATUS => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra210_mbdrc_volatile_reg(_dev: *mut device, mut reg: c_uint) -> bool_ {
    if reg >= TEGRA210_MBDRC_IIR_CFG {
        reg -= (reg - TEGRA210_MBDRC_IIR_CFG)
            % (TEGRA210_MBDRC_FILTER_PARAM_STRIDE * TEGRA210_MBDRC_FILTER_COUNT);
    }

    match reg {
        TEGRA210_MBDRC_SOFT_RESET
        | TEGRA210_MBDRC_STATUS
        | TEGRA210_MBDRC_CFG_RAM_CTRL
        | TEGRA210_MBDRC_CFG_RAM_DATA => true,
        _ => false,
    }
}

unsafe extern "C" fn tegra210_mbdrc_precious_reg(_dev: *mut device, mut reg: c_uint) -> bool_ {
    if reg >= TEGRA210_MBDRC_IIR_CFG {
        reg -= (reg - TEGRA210_MBDRC_IIR_CFG)
            % (TEGRA210_MBDRC_FILTER_PARAM_STRIDE * TEGRA210_MBDRC_FILTER_COUNT);
    }

    match reg {
        TEGRA210_MBDRC_CFG_RAM_DATA => true,
        _ => false,
    }
}

static tegra210_mbdrc_regmap_cfg: regmap_config = regmap_config {
    name: b"mbdrc\0".as_ptr() as *const c_char,
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: TEGRA210_MBDRC_MAX_REG,
    writeable_reg: Some(tegra210_mbdrc_wr_reg),
    readable_reg: Some(tegra210_mbdrc_rd_reg),
    volatile_reg: Some(tegra210_mbdrc_volatile_reg),
    precious_reg: Some(tegra210_mbdrc_precious_reg),
    reg_defaults: tegra210_mbdrc_reg_defaults.as_ptr(),
    num_reg_defaults: ARRAY_SIZE(&tegra210_mbdrc_reg_defaults),
    reg_default_cb: unsafe { regmap_default_zero_cb },
    cache_type: REGCACHE_FLAT,
};

#[no_mangle]
pub unsafe extern "C" fn tegra210_mbdrc_hw_params(cmpnt: *mut snd_soc_component) -> c_int {
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let conf = &mbdrc_init_config as *const tegra210_mbdrc_config;
    let mut val: u32 = 0;
    let mut i: c_uint;

    regmap_read((*ope).mbdrc_regmap, TEGRA210_MBDRC_CFG, &mut val);

    val &= TEGRA210_MBDRC_CFG_MBDRC_MODE_MASK;

    if val == TEGRA210_MBDRC_CFG_MBDRC_MODE_BYPASS {
        return 0;
    }

    i = 0;
    while i < MBDRC_NUM_BAND {
        let params = &(*conf).band_params[i as usize] as *const tegra210_mbdrc_band_params;
        let reg_off = i * TEGRA210_MBDRC_FILTER_PARAM_STRIDE;

        tegra210_mbdrc_write_ram(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_CFG_RAM_CTRL,
            reg_off + TEGRA210_MBDRC_CFG_RAM_DATA,
            0,
            (*params).biquad_params.as_ptr() as *mut u32,
            (TEGRA210_MBDRC_MAX_BIQUAD_STAGES * 5) as usize,
        );
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn tegra210_mbdrc_component_init(cmpnt: *mut snd_soc_component) -> c_int {
    let ope = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_ope;
    let conf = &mbdrc_init_config as *const tegra210_mbdrc_config;
    let mut i: c_uint;
    let mut val: u32;

    pm_runtime_get_sync((*cmpnt).dev);

    /* Initialize MBDRC registers and AHUB RAM with default params */
    regmap_update_bits(
        (*ope).mbdrc_regmap,
        TEGRA210_MBDRC_CFG,
        TEGRA210_MBDRC_CFG_MBDRC_MODE_MASK,
        (*conf).mode << TEGRA210_MBDRC_CFG_MBDRC_MODE_SHIFT,
    );

    regmap_update_bits(
        (*ope).mbdrc_regmap,
        TEGRA210_MBDRC_CFG,
        TEGRA210_MBDRC_CFG_RMS_OFFSET_MASK,
        (*conf).rms_off << TEGRA210_MBDRC_CFG_RMS_OFFSET_SHIFT,
    );

    regmap_update_bits(
        (*ope).mbdrc_regmap,
        TEGRA210_MBDRC_CFG,
        TEGRA210_MBDRC_CFG_PEAK_RMS_MASK,
        (*conf).peak_rms_mode << TEGRA210_MBDRC_CFG_PEAK_RMS_SHIFT,
    );

    regmap_update_bits(
        (*ope).mbdrc_regmap,
        TEGRA210_MBDRC_CFG,
        TEGRA210_MBDRC_CFG_FILTER_STRUCTURE_MASK,
        (*conf).filter_structure << TEGRA210_MBDRC_CFG_FILTER_STRUCTURE_SHIFT,
    );

    regmap_update_bits(
        (*ope).mbdrc_regmap,
        TEGRA210_MBDRC_CFG,
        TEGRA210_MBDRC_CFG_SHIFT_CTRL_MASK,
        (*conf).shift_ctrl << TEGRA210_MBDRC_CFG_SHIFT_CTRL_SHIFT,
    );

    regmap_update_bits(
        (*ope).mbdrc_regmap,
        TEGRA210_MBDRC_CFG,
        TEGRA210_MBDRC_CFG_FRAME_SIZE_MASK,
        __ffs((*conf).frame_size) << TEGRA210_MBDRC_CFG_FRAME_SIZE_SHIFT,
    );

    regmap_update_bits(
        (*ope).mbdrc_regmap,
        TEGRA210_MBDRC_CHANNEL_MASK,
        TEGRA210_MBDRC_CHANNEL_MASK_MASK,
        (*conf).channel_mask << TEGRA210_MBDRC_CHANNEL_MASK_SHIFT,
    );

    regmap_update_bits(
        (*ope).mbdrc_regmap,
        TEGRA210_MBDRC_FAST_FACTOR,
        TEGRA210_MBDRC_FAST_FACTOR_ATTACK_MASK,
        (*conf).fa_factor << TEGRA210_MBDRC_FAST_FACTOR_ATTACK_SHIFT,
    );

    regmap_update_bits(
        (*ope).mbdrc_regmap,
        TEGRA210_MBDRC_FAST_FACTOR,
        TEGRA210_MBDRC_FAST_FACTOR_ATTACK_MASK,
        (*conf).fr_factor << TEGRA210_MBDRC_FAST_FACTOR_ATTACK_SHIFT,
    );

    i = 0;
    while i < MBDRC_NUM_BAND {
        let params = &(*conf).band_params[i as usize] as *const tegra210_mbdrc_band_params;
        let reg_off = i * TEGRA210_MBDRC_FILTER_PARAM_STRIDE;

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_IIR_CFG,
            TEGRA210_MBDRC_IIR_CFG_NUM_STAGES_MASK,
            (*params).iir_stages << TEGRA210_MBDRC_IIR_CFG_NUM_STAGES_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_IN_ATTACK,
            TEGRA210_MBDRC_IN_ATTACK_TC_MASK,
            (*params).in_attack_tc << TEGRA210_MBDRC_IN_ATTACK_TC_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_IN_RELEASE,
            TEGRA210_MBDRC_IN_RELEASE_TC_MASK,
            (*params).in_release_tc << TEGRA210_MBDRC_IN_RELEASE_TC_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_FAST_ATTACK,
            TEGRA210_MBDRC_FAST_ATTACK_TC_MASK,
            (*params).fast_attack_tc << TEGRA210_MBDRC_FAST_ATTACK_TC_SHIFT,
        );

        val = ((((*params).in_threshold[0] >> TEGRA210_MBDRC_THRESH_1ST_SHIFT)
            & TEGRA210_MBDRC_THRESH_1ST_MASK)
            | (((*params).in_threshold[1] >> TEGRA210_MBDRC_THRESH_2ND_SHIFT)
                & TEGRA210_MBDRC_THRESH_2ND_MASK)
            | (((*params).in_threshold[2] >> TEGRA210_MBDRC_THRESH_3RD_SHIFT)
                & TEGRA210_MBDRC_THRESH_3RD_MASK)
            | (((*params).in_threshold[3] >> TEGRA210_MBDRC_THRESH_4TH_SHIFT)
                & TEGRA210_MBDRC_THRESH_4TH_MASK));

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_IN_THRESHOLD,
            0xffffffff,
            val,
        );

        val = ((((*params).out_threshold[0] >> TEGRA210_MBDRC_THRESH_1ST_SHIFT)
            & TEGRA210_MBDRC_THRESH_1ST_MASK)
            | (((*params).out_threshold[1] >> TEGRA210_MBDRC_THRESH_2ND_SHIFT)
                & TEGRA210_MBDRC_THRESH_2ND_MASK)
            | (((*params).out_threshold[2] >> TEGRA210_MBDRC_THRESH_3RD_SHIFT)
                & TEGRA210_MBDRC_THRESH_3RD_MASK)
            | (((*params).out_threshold[3] >> TEGRA210_MBDRC_THRESH_4TH_SHIFT)
                & TEGRA210_MBDRC_THRESH_4TH_MASK));

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_OUT_THRESHOLD,
            0xffffffff,
            val,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_RATIO_1ST,
            TEGRA210_MBDRC_RATIO_1ST_MASK,
            (*params).ratio[0] << TEGRA210_MBDRC_RATIO_1ST_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_RATIO_2ND,
            TEGRA210_MBDRC_RATIO_2ND_MASK,
            (*params).ratio[1] << TEGRA210_MBDRC_RATIO_2ND_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_RATIO_3RD,
            TEGRA210_MBDRC_RATIO_3RD_MASK,
            (*params).ratio[2] << TEGRA210_MBDRC_RATIO_3RD_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_RATIO_4TH,
            TEGRA210_MBDRC_RATIO_4TH_MASK,
            (*params).ratio[3] << TEGRA210_MBDRC_RATIO_4TH_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_RATIO_5TH,
            TEGRA210_MBDRC_RATIO_5TH_MASK,
            (*params).ratio[4] << TEGRA210_MBDRC_RATIO_5TH_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_MAKEUP_GAIN,
            TEGRA210_MBDRC_MAKEUP_GAIN_MASK,
            (*params).makeup_gain << TEGRA210_MBDRC_MAKEUP_GAIN_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_INIT_GAIN,
            TEGRA210_MBDRC_INIT_GAIN_MASK,
            (*params).gain_init << TEGRA210_MBDRC_INIT_GAIN_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_GAIN_ATTACK,
            TEGRA210_MBDRC_GAIN_ATTACK_MASK,
            (*params).gain_attack_tc << TEGRA210_MBDRC_GAIN_ATTACK_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_GAIN_RELEASE,
            TEGRA210_MBDRC_GAIN_RELEASE_MASK,
            (*params).gain_release_tc << TEGRA210_MBDRC_GAIN_RELEASE_SHIFT,
        );

        regmap_update_bits(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_FAST_RELEASE,
            TEGRA210_MBDRC_FAST_RELEASE_MASK,
            (*params).fast_release_tc << TEGRA210_MBDRC_FAST_RELEASE_SHIFT,
        );

        tegra210_mbdrc_write_ram(
            (*ope).mbdrc_regmap,
            reg_off + TEGRA210_MBDRC_CFG_RAM_CTRL,
            reg_off + TEGRA210_MBDRC_CFG_RAM_DATA,
            0,
            (*params).biquad_params.as_ptr() as *mut u32,
            (TEGRA210_MBDRC_MAX_BIQUAD_STAGES * 5) as usize,
        );

        i += 1;
    }

    pm_runtime_put_sync((*cmpnt).dev);

    snd_soc_add_component_controls(
        cmpnt,
        tegra210_mbdrc_controls.as_ptr(),
        ARRAY_SIZE(&tegra210_mbdrc_controls),
    );

    0
}

#[no_mangle]
pub unsafe extern "C" fn tegra210_mbdrc_regmap_init(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let ope = dev_get_drvdata(dev) as *mut tegra210_ope;
    let mut child: *mut device_node;
    let mut mem: resource = core::mem::zeroed();
    let regs: *mut c_void;
    let mut err: c_int;

    child = of_get_child_by_name(
        (*dev).of_node,
        b"dynamic-range-compressor\0".as_ptr() as *const c_char,
    );
    if child.is_null() {
        return dev_err_probe(
            dev,
            -ENODEV,
            b"missing 'dynamic-range-compressor' DT child node\n\0".as_ptr() as *const c_char,
        );
    }

    err = of_address_to_resource(child, 0, &mut mem);
    of_node_put(child);
    if err < 0 {
        return dev_err_probe(
            dev,
            err,
            b"failed to get MBDRC resource\n\0".as_ptr() as *const c_char,
        );
    }

    mem.flags = IORESOURCE_MEM;
    regs = devm_ioremap_resource(dev, &mut mem);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    (*ope).mbdrc_regmap = devm_regmap_init_mmio(dev, regs, &tegra210_mbdrc_regmap_cfg);
    if IS_ERR((*ope).mbdrc_regmap as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*ope).mbdrc_regmap as *const c_void),
            b"MBDRC regmap init failed\n\0".as_ptr() as *const c_char,
        );
    }

    regcache_cache_only((*ope).mbdrc_regmap, true);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
