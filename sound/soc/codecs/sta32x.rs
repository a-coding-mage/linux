// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Codec driver for ST STA32x 2.1-channel high-efficiency digital audio system
 *
 * Copyright: 2011 Raumfeld GmbH
 * Author: Johannes Stezenbach <js@sig21.net>
 *
 * based on code from:
 *	Wolfson Microelectronics PLC.
 *	  Mark Brown <broonie@opensource.wolfsonmicro.com>
 *	Freescale Semiconductor, Inc.
 *	  Timur Tabi <timur@freescale.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = ::core::ffi::c_uchar;
type u16 = ::core::ffi::c_ushort;
type u32 = ::core::ffi::c_uint;

// pr_fmt(fmt) KBUILD_MODNAME ":%s:%d: " fmt, __func__, __LINE__
// Linux, ALSA, regmap and local sta32x.h declarations are external to this file.

const STA32X_RATES: c_uint = SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const STA32X_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S18_3LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_range {
    pub range_min: c_uint,
    pub range_max: c_uint,
}

#[repr(C)]
pub struct regmap_access_table {
    pub yes_ranges: *const regmap_range,
    pub n_yes_ranges: c_uint,
}

#[repr(C)]
pub struct regulator_bulk_data {
    pub supply: *const c_char,
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sta32x_priv {
    regmap: *mut regmap,
    xti_clk: *mut clk,
    supplies: [regulator_bulk_data; sta32x_supply_names.len()],
    component: *mut snd_soc_component,
    pdata: *mut sta32x_platform_data,
    mclk: c_uint,
    format: c_uint,
    coef_shadow: [u32; STA32X_COEF_COUNT as usize],
    watchdog_work: delayed_work,
    shutdown: c_int,
    gpiod_nreset: *mut gpio_desc,
    coeff_lock: mutex,
}

/* Power-up register defaults */
static sta32x_regs: [reg_default; 42] = [
    reg_default { reg: 0x0, def: 0x63 },
    reg_default { reg: 0x1, def: 0x80 },
    reg_default { reg: 0x2, def: 0xc2 },
    reg_default { reg: 0x3, def: 0x40 },
    reg_default { reg: 0x4, def: 0xc2 },
    reg_default { reg: 0x5, def: 0x5c },
    reg_default { reg: 0x6, def: 0x10 },
    reg_default { reg: 0x7, def: 0xff },
    reg_default { reg: 0x8, def: 0x60 },
    reg_default { reg: 0x9, def: 0x60 },
    reg_default { reg: 0xa, def: 0x60 },
    reg_default { reg: 0xb, def: 0x80 },
    reg_default { reg: 0xc, def: 0x00 },
    reg_default { reg: 0xd, def: 0x00 },
    reg_default { reg: 0xe, def: 0x00 },
    reg_default { reg: 0xf, def: 0x40 },
    reg_default { reg: 0x10, def: 0x80 },
    reg_default { reg: 0x11, def: 0x77 },
    reg_default { reg: 0x12, def: 0x6a },
    reg_default { reg: 0x13, def: 0x69 },
    reg_default { reg: 0x14, def: 0x6a },
    reg_default { reg: 0x15, def: 0x69 },
    reg_default { reg: 0x16, def: 0x00 },
    reg_default { reg: 0x17, def: 0x00 },
    reg_default { reg: 0x18, def: 0x00 },
    reg_default { reg: 0x19, def: 0x00 },
    reg_default { reg: 0x1a, def: 0x00 },
    reg_default { reg: 0x1b, def: 0x00 },
    reg_default { reg: 0x1c, def: 0x00 },
    reg_default { reg: 0x1d, def: 0x00 },
    reg_default { reg: 0x1e, def: 0x00 },
    reg_default { reg: 0x1f, def: 0x00 },
    reg_default { reg: 0x20, def: 0x00 },
    reg_default { reg: 0x21, def: 0x00 },
    reg_default { reg: 0x22, def: 0x00 },
    reg_default { reg: 0x23, def: 0x00 },
    reg_default { reg: 0x24, def: 0x00 },
    reg_default { reg: 0x25, def: 0x00 },
    reg_default { reg: 0x26, def: 0x00 },
    reg_default { reg: 0x27, def: 0x2d },
    reg_default { reg: 0x28, def: 0xc0 },
    reg_default { reg: 0x2b, def: 0x00 },
    reg_default { reg: 0x2c, def: 0x0c },
];

static sta32x_write_regs_range: [regmap_range; 1] = [regmap_range {
    range_min: STA32X_CONFA,
    range_max: STA32X_FDRC2,
}];
static sta32x_read_regs_range: [regmap_range; 1] = [regmap_range {
    range_min: STA32X_CONFA,
    range_max: STA32X_FDRC2,
}];
static sta32x_volatile_regs_range: [regmap_range; 1] = [regmap_range {
    range_min: STA32X_CFADDR2,
    range_max: STA32X_CFUD,
}];

static sta32x_write_regs: regmap_access_table = regmap_access_table {
    yes_ranges: sta32x_write_regs_range.as_ptr(),
    n_yes_ranges: sta32x_write_regs_range.len() as c_uint,
};
static sta32x_read_regs: regmap_access_table = regmap_access_table {
    yes_ranges: sta32x_read_regs_range.as_ptr(),
    n_yes_ranges: sta32x_read_regs_range.len() as c_uint,
};
static sta32x_volatile_regs: regmap_access_table = regmap_access_table {
    yes_ranges: sta32x_volatile_regs_range.as_ptr(),
    n_yes_ranges: sta32x_volatile_regs_range.len() as c_uint,
};

/* regulator power supply names */
static sta32x_supply_names: [*const c_char; 3] = [
    b"Vdda\0".as_ptr() as *const c_char, /* analog supply, 3.3VV */
    b"Vdd3\0".as_ptr() as *const c_char, /* digital supply, 3.3V */
    b"Vcc\0".as_ptr() as *const c_char,  /* power amp spply, 10V - 36V */
];

static mvol_tlv: [c_uint; 4] = declare_tlv_db_scale(-12700, 50, 1);
static chvol_tlv: [c_uint; 4] = declare_tlv_db_scale(-7950, 50, 1);
static tone_tlv: [c_uint; 4] = declare_tlv_db_scale(-120, 200, 0);

static sta32x_drc_ac: [*const c_char; 2] = [
    b"Anti-Clipping\0".as_ptr() as *const c_char,
    b"Dynamic Range Compression\0".as_ptr() as *const c_char,
];
static sta32x_auto_eq_mode: [*const c_char; 3] = [
    b"User\0".as_ptr() as *const c_char,
    b"Preset\0".as_ptr() as *const c_char,
    b"Loudness\0".as_ptr() as *const c_char,
];
static sta32x_auto_gc_mode: [*const c_char; 4] = [
    b"User\0".as_ptr() as *const c_char,
    b"AC no clipping\0".as_ptr() as *const c_char,
    b"AC limited clipping (10%)\0".as_ptr() as *const c_char,
    b"DRC nighttime listening mode\0".as_ptr() as *const c_char,
];
static sta32x_auto_xo_mode: [*const c_char; 16] = [
    b"User\0".as_ptr() as *const c_char,
    b"80Hz\0".as_ptr() as *const c_char,
    b"100Hz\0".as_ptr() as *const c_char,
    b"120Hz\0".as_ptr() as *const c_char,
    b"140Hz\0".as_ptr() as *const c_char,
    b"160Hz\0".as_ptr() as *const c_char,
    b"180Hz\0".as_ptr() as *const c_char,
    b"200Hz\0".as_ptr() as *const c_char,
    b"220Hz\0".as_ptr() as *const c_char,
    b"240Hz\0".as_ptr() as *const c_char,
    b"260Hz\0".as_ptr() as *const c_char,
    b"280Hz\0".as_ptr() as *const c_char,
    b"300Hz\0".as_ptr() as *const c_char,
    b"320Hz\0".as_ptr() as *const c_char,
    b"340Hz\0".as_ptr() as *const c_char,
    b"360Hz\0".as_ptr() as *const c_char,
];
static sta32x_preset_eq_mode: [*const c_char; 32] = [
    b"Flat\0".as_ptr() as *const c_char, b"Rock\0".as_ptr() as *const c_char,
    b"Soft Rock\0".as_ptr() as *const c_char, b"Jazz\0".as_ptr() as *const c_char,
    b"Classical\0".as_ptr() as *const c_char, b"Dance\0".as_ptr() as *const c_char,
    b"Pop\0".as_ptr() as *const c_char, b"Soft\0".as_ptr() as *const c_char,
    b"Hard\0".as_ptr() as *const c_char, b"Party\0".as_ptr() as *const c_char,
    b"Vocal\0".as_ptr() as *const c_char, b"Hip-Hop\0".as_ptr() as *const c_char,
    b"Dialog\0".as_ptr() as *const c_char, b"Bass-boost #1\0".as_ptr() as *const c_char,
    b"Bass-boost #2\0".as_ptr() as *const c_char, b"Bass-boost #3\0".as_ptr() as *const c_char,
    b"Loudness 1\0".as_ptr() as *const c_char, b"Loudness 2\0".as_ptr() as *const c_char,
    b"Loudness 3\0".as_ptr() as *const c_char, b"Loudness 4\0".as_ptr() as *const c_char,
    b"Loudness 5\0".as_ptr() as *const c_char, b"Loudness 6\0".as_ptr() as *const c_char,
    b"Loudness 7\0".as_ptr() as *const c_char, b"Loudness 8\0".as_ptr() as *const c_char,
    b"Loudness 9\0".as_ptr() as *const c_char, b"Loudness 10\0".as_ptr() as *const c_char,
    b"Loudness 11\0".as_ptr() as *const c_char, b"Loudness 12\0".as_ptr() as *const c_char,
    b"Loudness 13\0".as_ptr() as *const c_char, b"Loudness 14\0".as_ptr() as *const c_char,
    b"Loudness 15\0".as_ptr() as *const c_char, b"Loudness 16\0".as_ptr() as *const c_char,
];
static sta32x_limiter_select: [*const c_char; 3] = [
    b"Limiter Disabled\0".as_ptr() as *const c_char,
    b"Limiter #1\0".as_ptr() as *const c_char,
    b"Limiter #2\0".as_ptr() as *const c_char,
];
static sta32x_limiter_attack_rate: [*const c_char; 16] = [
    b"3.1584\0".as_ptr() as *const c_char, b"2.7072\0".as_ptr() as *const c_char,
    b"2.2560\0".as_ptr() as *const c_char, b"1.8048\0".as_ptr() as *const c_char,
    b"1.3536\0".as_ptr() as *const c_char, b"0.9024\0".as_ptr() as *const c_char,
    b"0.4512\0".as_ptr() as *const c_char, b"0.2256\0".as_ptr() as *const c_char,
    b"0.1504\0".as_ptr() as *const c_char, b"0.1123\0".as_ptr() as *const c_char,
    b"0.0902\0".as_ptr() as *const c_char, b"0.0752\0".as_ptr() as *const c_char,
    b"0.0645\0".as_ptr() as *const c_char, b"0.0564\0".as_ptr() as *const c_char,
    b"0.0501\0".as_ptr() as *const c_char, b"0.0451\0".as_ptr() as *const c_char,
];
static sta32x_limiter_release_rate: [*const c_char; 16] = [
    b"0.5116\0".as_ptr() as *const c_char, b"0.1370\0".as_ptr() as *const c_char,
    b"0.0744\0".as_ptr() as *const c_char, b"0.0499\0".as_ptr() as *const c_char,
    b"0.0360\0".as_ptr() as *const c_char, b"0.0299\0".as_ptr() as *const c_char,
    b"0.0264\0".as_ptr() as *const c_char, b"0.0208\0".as_ptr() as *const c_char,
    b"0.0198\0".as_ptr() as *const c_char, b"0.0172\0".as_ptr() as *const c_char,
    b"0.0147\0".as_ptr() as *const c_char, b"0.0137\0".as_ptr() as *const c_char,
    b"0.0134\0".as_ptr() as *const c_char, b"0.0117\0".as_ptr() as *const c_char,
    b"0.0110\0".as_ptr() as *const c_char, b"0.0104\0".as_ptr() as *const c_char,
];

// DECLARE_TLV_DB_RANGE values are preserved through external macro-compatible declarations.
declare_tlv_db_range!(sta32x_limiter_ac_attack_tlv,
    0, 7, TLV_DB_SCALE_ITEM(-1200, 200, 0),
    8, 16, TLV_DB_SCALE_ITEM(300, 100, 0),
);
declare_tlv_db_range!(sta32x_limiter_ac_release_tlv,
    0, 0, TLV_DB_SCALE_ITEM(TLV_DB_GAIN_MUTE, 0, 0),
    1, 1, TLV_DB_SCALE_ITEM(-2900, 0, 0),
    2, 2, TLV_DB_SCALE_ITEM(-2000, 0, 0),
    3, 8, TLV_DB_SCALE_ITEM(-1400, 200, 0),
    8, 16, TLV_DB_SCALE_ITEM(-700, 100, 0),
);
declare_tlv_db_range!(sta32x_limiter_drc_attack_tlv,
    0, 7, TLV_DB_SCALE_ITEM(-3100, 200, 0),
    8, 13, TLV_DB_SCALE_ITEM(-1600, 100, 0),
    14, 16, TLV_DB_SCALE_ITEM(-1000, 300, 0),
);
declare_tlv_db_range!(sta32x_limiter_drc_release_tlv,
    0, 0, TLV_DB_SCALE_ITEM(TLV_DB_GAIN_MUTE, 0, 0),
    1, 2, TLV_DB_SCALE_ITEM(-3800, 200, 0),
    3, 4, TLV_DB_SCALE_ITEM(-3300, 200, 0),
    5, 12, TLV_DB_SCALE_ITEM(-3000, 200, 0),
    13, 16, TLV_DB_SCALE_ITEM(-1500, 300, 0),
);

soc_enum_single_decl!(sta32x_drc_ac_enum, STA32X_CONFD, STA32X_CONFD_DRC_SHIFT, sta32x_drc_ac);
soc_enum_single_decl!(sta32x_auto_eq_enum, STA32X_AUTO1, STA32X_AUTO1_AMEQ_SHIFT, sta32x_auto_eq_mode);
soc_enum_single_decl!(sta32x_auto_gc_enum, STA32X_AUTO1, STA32X_AUTO1_AMGC_SHIFT, sta32x_auto_gc_mode);
soc_enum_single_decl!(sta32x_auto_xo_enum, STA32X_AUTO2, STA32X_AUTO2_XO_SHIFT, sta32x_auto_xo_mode);
soc_enum_single_decl!(sta32x_preset_eq_enum, STA32X_AUTO3, STA32X_AUTO3_PEQ_SHIFT, sta32x_preset_eq_mode);
soc_enum_single_decl!(sta32x_limiter_ch1_enum, STA32X_C1CFG, STA32X_CxCFG_LS_SHIFT, sta32x_limiter_select);
soc_enum_single_decl!(sta32x_limiter_ch2_enum, STA32X_C2CFG, STA32X_CxCFG_LS_SHIFT, sta32x_limiter_select);
soc_enum_single_decl!(sta32x_limiter_ch3_enum, STA32X_C3CFG, STA32X_CxCFG_LS_SHIFT, sta32x_limiter_select);
soc_enum_single_decl!(sta32x_limiter1_attack_rate_enum, STA32X_L1AR, STA32X_LxA_SHIFT, sta32x_limiter_attack_rate);
soc_enum_single_decl!(sta32x_limiter2_attack_rate_enum, STA32X_L2AR, STA32X_LxA_SHIFT, sta32x_limiter_attack_rate);
soc_enum_single_decl!(sta32x_limiter1_release_rate_enum, STA32X_L1AR, STA32X_LxR_SHIFT, sta32x_limiter_release_rate);
soc_enum_single_decl!(sta32x_limiter2_release_rate_enum, STA32X_L2AR, STA32X_LxR_SHIFT, sta32x_limiter_release_rate);

/* byte array controls for setting biquad, mixer, scaling coefficients;
 * for biquads all five coefficients need to be set in one go,
 * mixer and pre/postscale coefs can be set individually;
 * each coef is 24bit, the bytes are ordered in the same way
 * as given in the STA32x data sheet (big endian; b1, b2, a1, a2, b0)
 */

unsafe extern "C" fn sta32x_coefficient_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let numcoef: c_int = ((*kcontrol).private_value >> 16) as c_int;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*uinfo).count = (3 * numcoef) as c_uint;
    0
}

unsafe extern "C" fn sta32x_coefficient_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let sta32x = snd_soc_component_get_drvdata(component) as *mut sta32x_priv;
    let numcoef: c_int = ((*kcontrol).private_value >> 16) as c_int;
    let index: c_int = ((*kcontrol).private_value & 0xffff) as c_int;
    let mut cfud: c_uint = 0;
    let mut val: c_uint = 0;
    let mut i: c_int;

    mutex_lock(&mut (*sta32x).coeff_lock);

    /* preserve reserved bits in STA32X_CFUD */
    regmap_read((*sta32x).regmap, STA32X_CFUD, &mut cfud);
    cfud &= 0xf0;
    /*
     * chip documentation does not say if the bits are self clearing,
     * so do it explicitly
     */
    regmap_write((*sta32x).regmap, STA32X_CFUD, cfud);

    regmap_write((*sta32x).regmap, STA32X_CFADDR2, index as c_uint);
    if numcoef == 1 {
        regmap_write((*sta32x).regmap, STA32X_CFUD, cfud | 0x04);
    } else if numcoef == 5 {
        regmap_write((*sta32x).regmap, STA32X_CFUD, cfud | 0x08);
    } else {
        mutex_unlock(&mut (*sta32x).coeff_lock);
        return -EINVAL;
    }

    i = 0;
    while i < 3 * numcoef {
        regmap_read((*sta32x).regmap, STA32X_B1CF1 + i as c_uint, &mut val);
        (*ucontrol).value.bytes.data[i as usize] = val as u8;
        i += 1;
    }

    mutex_unlock(&mut (*sta32x).coeff_lock);
    0
}

unsafe extern "C" fn sta32x_coefficient_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let sta32x = snd_soc_component_get_drvdata(component) as *mut sta32x_priv;
    let numcoef: c_int = ((*kcontrol).private_value >> 16) as c_int;
    let index: c_int = ((*kcontrol).private_value & 0xffff) as c_int;
    let mut cfud: c_uint = 0;
    let mut i: c_int;

    /* preserve reserved bits in STA32X_CFUD */
    regmap_read((*sta32x).regmap, STA32X_CFUD, &mut cfud);
    cfud &= 0xf0;
    /*
     * chip documentation does not say if the bits are self clearing,
     * so do it explicitly
     */
    regmap_write((*sta32x).regmap, STA32X_CFUD, cfud);

    regmap_write((*sta32x).regmap, STA32X_CFADDR2, index as c_uint);
    i = 0;
    while i < numcoef && index + i < STA32X_COEF_COUNT as c_int {
        (*sta32x).coef_shadow[(index + i) as usize] =
            (((*ucontrol).value.bytes.data[(3 * i) as usize] as u32) << 16)
                | (((*ucontrol).value.bytes.data[(3 * i + 1) as usize] as u32) << 8)
                | ((*ucontrol).value.bytes.data[(3 * i + 2) as usize] as u32);
        i += 1;
    }
    i = 0;
    while i < 3 * numcoef {
        regmap_write(
            (*sta32x).regmap,
            STA32X_B1CF1 + i as c_uint,
            (*ucontrol).value.bytes.data[i as usize] as c_uint,
        );
        i += 1;
    }
    if numcoef == 1 {
        regmap_write((*sta32x).regmap, STA32X_CFUD, cfud | 0x01);
    } else if numcoef == 5 {
        regmap_write((*sta32x).regmap, STA32X_CFUD, cfud | 0x02);
    } else {
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn sta32x_sync_coef_shadow(component: *mut snd_soc_component) -> c_int {
    let sta32x = snd_soc_component_get_drvdata(component) as *mut sta32x_priv;
    let mut cfud: c_uint = 0;
    let mut i: c_int;

    /* preserve reserved bits in STA32X_CFUD */
    regmap_read((*sta32x).regmap, STA32X_CFUD, &mut cfud);
    cfud &= 0xf0;

    i = 0;
    while i < STA32X_COEF_COUNT as c_int {
        regmap_write((*sta32x).regmap, STA32X_CFADDR2, i as c_uint);
        regmap_write(
            (*sta32x).regmap,
            STA32X_B1CF1,
            ((*sta32x).coef_shadow[i as usize] >> 16) & 0xff,
        );
        regmap_write(
            (*sta32x).regmap,
            STA32X_B1CF2,
            ((*sta32x).coef_shadow[i as usize] >> 8) & 0xff,
        );
        regmap_write(
            (*sta32x).regmap,
            STA32X_B1CF3,
            (*sta32x).coef_shadow[i as usize] & 0xff,
        );
        /*
         * chip documentation does not say if the bits are
         * self-clearing, so do it explicitly
         */
        regmap_write((*sta32x).regmap, STA32X_CFUD, cfud);
        regmap_write((*sta32x).regmap, STA32X_CFUD, cfud | 0x01);
        i += 1;
    }
    0
}

unsafe extern "C" fn sta32x_cache_sync(component: *mut snd_soc_component) -> c_int {
    let sta32x = snd_soc_component_get_drvdata(component) as *mut sta32x_priv;
    let mut mute: c_uint = 0;
    let rc: c_int;

    /* mute during register sync */
    regmap_read((*sta32x).regmap, STA32X_MMUTE, &mut mute);
    regmap_write((*sta32x).regmap, STA32X_MMUTE, mute | STA32X_MMUTE_MMUTE);
    sta32x_sync_coef_shadow(component);
    rc = regcache_sync((*sta32x).regmap);
    regmap_write((*sta32x).regmap, STA32X_MMUTE, mute);
    rc
}

/* work around ESD issue where sta32x resets and loses all configuration */
unsafe extern "C" fn sta32x_watchdog(work: *mut work_struct) {
    let sta32x = container_of!(work, sta32x_priv, watchdog_work.work);
    let component = (*sta32x).component;
    let mut confa: c_uint;
    let confa_cached: c_uint;

    /* check if sta32x has reset itself */
    confa_cached = snd_soc_component_read(component, STA32X_CONFA);
    regcache_cache_bypass((*sta32x).regmap, true);
    confa = snd_soc_component_read(component, STA32X_CONFA);
    regcache_cache_bypass((*sta32x).regmap, false);
    if confa != confa_cached {
        regcache_mark_dirty((*sta32x).regmap);
        sta32x_cache_sync(component);
    }

    if (*sta32x).shutdown == 0 {
        queue_delayed_work(
            system_power_efficient_wq,
            &mut (*sta32x).watchdog_work,
            round_jiffies_relative(HZ),
        );
    }
}

unsafe fn sta32x_watchdog_start(sta32x: *mut sta32x_priv) {
    if (*(*sta32x).pdata).needs_esd_watchdog {
        (*sta32x).shutdown = 0;
        queue_delayed_work(
            system_power_efficient_wq,
            &mut (*sta32x).watchdog_work,
            round_jiffies_relative(HZ),
        );
    }
}

unsafe fn sta32x_watchdog_stop(sta32x: *mut sta32x_priv) {
    if (*(*sta32x).pdata).needs_esd_watchdog {
        (*sta32x).shutdown = 1;
        cancel_delayed_work_sync(&mut (*sta32x).watchdog_work);
    }
}

// SINGLE_COEF and BIQUAD_COEFS are preserved as Rust macro-compatible calls.
macro_rules! SINGLE_COEF {
    ($xname:expr, $index:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            info: Some(sta32x_coefficient_info),
            get: Some(sta32x_coefficient_get),
            put: Some(sta32x_coefficient_put),
            private_value: $index | (1 << 16),
        }
    };
}
macro_rules! BIQUAD_COEFS {
    ($xname:expr, $index:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            info: Some(sta32x_coefficient_info),
            get: Some(sta32x_coefficient_get),
            put: Some(sta32x_coefficient_put),
            private_value: $index | (5 << 16),
        }
    };
}

static sta32x_snd_controls: [snd_kcontrol_new; 62] = [
    SOC_SINGLE_TLV!("Master Volume", STA32X_MVOL, 0, 0xff, 1, mvol_tlv),
    SOC_SINGLE!("Master Switch", STA32X_MMUTE, 0, 1, 1),
    SOC_SINGLE!("Ch1 Switch", STA32X_MMUTE, 1, 1, 1),
    SOC_SINGLE!("Ch2 Switch", STA32X_MMUTE, 2, 1, 1),
    SOC_SINGLE!("Ch3 Switch", STA32X_MMUTE, 3, 1, 1),
    SOC_SINGLE_TLV!("Ch1 Volume", STA32X_C1VOL, 0, 0xff, 1, chvol_tlv),
    SOC_SINGLE_TLV!("Ch2 Volume", STA32X_C2VOL, 0, 0xff, 1, chvol_tlv),
    SOC_SINGLE_TLV!("Ch3 Volume", STA32X_C3VOL, 0, 0xff, 1, chvol_tlv),
    SOC_SINGLE!("De-emphasis Filter Switch", STA32X_CONFD, STA32X_CONFD_DEMP_SHIFT, 1, 0),
    SOC_ENUM!("Compressor/Limiter Switch", sta32x_drc_ac_enum),
    SOC_SINGLE!("Miami Mode Switch", STA32X_CONFD, STA32X_CONFD_MME_SHIFT, 1, 0),
    SOC_SINGLE!("Zero Cross Switch", STA32X_CONFE, STA32X_CONFE_ZCE_SHIFT, 1, 0),
    SOC_SINGLE!("Soft Ramp Switch", STA32X_CONFE, STA32X_CONFE_SVE_SHIFT, 1, 0),
    SOC_SINGLE!("Auto-Mute Switch", STA32X_CONFF, STA32X_CONFF_IDE_SHIFT, 1, 0),
    SOC_ENUM!("Automode EQ", sta32x_auto_eq_enum),
    SOC_ENUM!("Automode GC", sta32x_auto_gc_enum),
    SOC_ENUM!("Automode XO", sta32x_auto_xo_enum),
    SOC_ENUM!("Preset EQ", sta32x_preset_eq_enum),
    SOC_SINGLE!("Ch1 Tone Control Bypass Switch", STA32X_C1CFG, STA32X_CxCFG_TCB_SHIFT, 1, 0),
    SOC_SINGLE!("Ch2 Tone Control Bypass Switch", STA32X_C2CFG, STA32X_CxCFG_TCB_SHIFT, 1, 0),
    SOC_SINGLE!("Ch1 EQ Bypass Switch", STA32X_C1CFG, STA32X_CxCFG_EQBP_SHIFT, 1, 0),
    SOC_SINGLE!("Ch2 EQ Bypass Switch", STA32X_C2CFG, STA32X_CxCFG_EQBP_SHIFT, 1, 0),
    SOC_SINGLE!("Ch1 Master Volume Bypass Switch", STA32X_C1CFG, STA32X_CxCFG_VBP_SHIFT, 1, 0),
    SOC_SINGLE!("Ch2 Master Volume Bypass Switch", STA32X_C1CFG, STA32X_CxCFG_VBP_SHIFT, 1, 0),
    SOC_SINGLE!("Ch3 Master Volume Bypass Switch", STA32X_C1CFG, STA32X_CxCFG_VBP_SHIFT, 1, 0),
    SOC_ENUM!("Ch1 Limiter Select", sta32x_limiter_ch1_enum),
    SOC_ENUM!("Ch2 Limiter Select", sta32x_limiter_ch2_enum),
    SOC_ENUM!("Ch3 Limiter Select", sta32x_limiter_ch3_enum),
    SOC_SINGLE_TLV!("Bass Tone Control", STA32X_TONE, STA32X_TONE_BTC_SHIFT, 15, 0, tone_tlv),
    SOC_SINGLE_TLV!("Treble Tone Control", STA32X_TONE, STA32X_TONE_TTC_SHIFT, 15, 0, tone_tlv),
    SOC_ENUM!("Limiter1 Attack Rate (dB/ms)", sta32x_limiter1_attack_rate_enum),
    SOC_ENUM!("Limiter2 Attack Rate (dB/ms)", sta32x_limiter2_attack_rate_enum),
    SOC_ENUM!("Limiter1 Release Rate (dB/ms)", sta32x_limiter1_release_rate_enum),
    SOC_ENUM!("Limiter2 Release Rate (dB/ms)", sta32x_limiter2_release_rate_enum),
    /* depending on mode, the attack/release thresholds have
     * two different enum definitions; provide both
     */
    SOC_SINGLE_TLV!("Limiter1 Attack Threshold (AC Mode)", STA32X_L1ATRT, STA32X_LxA_SHIFT, 16, 0, sta32x_limiter_ac_attack_tlv),
    SOC_SINGLE_TLV!("Limiter2 Attack Threshold (AC Mode)", STA32X_L2ATRT, STA32X_LxA_SHIFT, 16, 0, sta32x_limiter_ac_attack_tlv),
    SOC_SINGLE_TLV!("Limiter1 Release Threshold (AC Mode)", STA32X_L1ATRT, STA32X_LxR_SHIFT, 16, 0, sta32x_limiter_ac_release_tlv),
    SOC_SINGLE_TLV!("Limiter2 Release Threshold (AC Mode)", STA32X_L2ATRT, STA32X_LxR_SHIFT, 16, 0, sta32x_limiter_ac_release_tlv),
    SOC_SINGLE_TLV!("Limiter1 Attack Threshold (DRC Mode)", STA32X_L1ATRT, STA32X_LxA_SHIFT, 16, 0, sta32x_limiter_drc_attack_tlv),
    SOC_SINGLE_TLV!("Limiter2 Attack Threshold (DRC Mode)", STA32X_L2ATRT, STA32X_LxA_SHIFT, 16, 0, sta32x_limiter_drc_attack_tlv),
    SOC_SINGLE_TLV!("Limiter1 Release Threshold (DRC Mode)", STA32X_L1ATRT, STA32X_LxR_SHIFT, 16, 0, sta32x_limiter_drc_release_tlv),
    SOC_SINGLE_TLV!("Limiter2 Release Threshold (DRC Mode)", STA32X_L2ATRT, STA32X_LxR_SHIFT, 16, 0, sta32x_limiter_drc_release_tlv),
    BIQUAD_COEFS!("Ch1 - Biquad 1", 0), BIQUAD_COEFS!("Ch1 - Biquad 2", 5),
    BIQUAD_COEFS!("Ch1 - Biquad 3", 10), BIQUAD_COEFS!("Ch1 - Biquad 4", 15),
    BIQUAD_COEFS!("Ch2 - Biquad 1", 20), BIQUAD_COEFS!("Ch2 - Biquad 2", 25),
    BIQUAD_COEFS!("Ch2 - Biquad 3", 30), BIQUAD_COEFS!("Ch2 - Biquad 4", 35),
    BIQUAD_COEFS!("High-pass", 40), BIQUAD_COEFS!("Low-pass", 45),
    SINGLE_COEF!("Ch1 - Prescale", 50), SINGLE_COEF!("Ch2 - Prescale", 51),
    SINGLE_COEF!("Ch1 - Postscale", 52), SINGLE_COEF!("Ch2 - Postscale", 53),
    SINGLE_COEF!("Ch3 - Postscale", 54), SINGLE_COEF!("Thermal warning - Postscale", 55),
    SINGLE_COEF!("Ch1 - Mix 1", 56), SINGLE_COEF!("Ch1 - Mix 2", 57),
    SINGLE_COEF!("Ch2 - Mix 1", 58), SINGLE_COEF!("Ch2 - Mix 2", 59),
    SINGLE_COEF!("Ch3 - Mix 1", 60), SINGLE_COEF!("Ch3 - Mix 2", 61),
];

static sta32x_dapm_widgets: [snd_soc_dapm_widget; 4] = [
    SND_SOC_DAPM_DAC!("DAC", "Playback", SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT!("LEFT"),
    SND_SOC_DAPM_OUTPUT!("RIGHT"),
    SND_SOC_DAPM_OUTPUT!("SUB"),
];

static sta32x_dapm_routes: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route { sink: b"LEFT\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"RIGHT\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DAC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SUB\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"DAC\0".as_ptr() as *const c_char },
];

/* MCLK interpolation ratio per fs */
#[repr(C)]
struct interpolation_ratio {
    fs: c_int,
    ir: c_int,
}
static mut interpolation_ratios: [interpolation_ratio; 7] = [
    interpolation_ratio { fs: 32000, ir: 0 },
    interpolation_ratio { fs: 44100, ir: 0 },
    interpolation_ratio { fs: 48000, ir: 0 },
    interpolation_ratio { fs: 88200, ir: 1 },
    interpolation_ratio { fs: 96000, ir: 1 },
    interpolation_ratio { fs: 176400, ir: 2 },
    interpolation_ratio { fs: 192000, ir: 2 },
];

/* MCLK to fs clock ratios */
static mut mcs_ratio_table: [[c_int; 7]; 3] = [
    [768, 512, 384, 256, 128, 576, 0],
    [384, 256, 192, 128, 64, 0, 0],
    [384, 256, 192, 128, 64, 0, 0],
];

/**
 * sta32x_set_dai_sysclk - configure MCLK
 * @codec_dai: the codec DAI
 * @clk_id: the clock ID (ignored)
 * @freq: the MCLK input frequency
 * @dir: the clock direction (ignored)
 *
 * The value of MCLK is used to determine which sample rates are supported
 * by the STA32X, based on the mclk_ratios table.
 *
 * This function must be called by the machine driver's 'startup' function,
 * otherwise the list of supported sample rates will not be available in
 * time for ALSA.
 *
 * For setups with variable MCLKs, pass 0 as 'freq' argument. This will cause
 * theoretically possible sample rates to be enabled. Call it again with a
 * proper value set one the external clock is set (most probably you would do
 * that from a machine's driver 'hw_param' hook.
 */
unsafe extern "C" fn sta32x_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*codec_dai).component;
    let sta32x = snd_soc_component_get_drvdata(component) as *mut sta32x_priv;

    dev_dbg((*component).dev, b"mclk=%u\n\0".as_ptr() as *const c_char, freq);
    (*sta32x).mclk = freq;
    0
}

/**
 * sta32x_set_dai_fmt - configure the codec for the selected audio format
 * @codec_dai: the codec DAI
 * @fmt: a SND_SOC_DAIFMT_x value indicating the data format
 *
 * This function takes a bitmask of SND_SOC_DAIFMT_x bits and programs the
 * codec accordingly.
 */
unsafe extern "C" fn sta32x_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*codec_dai).component;
    let sta32x = snd_soc_component_get_drvdata(component) as *mut sta32x_priv;
    let mut confb: u8 = 0;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_RIGHT_J | SND_SOC_DAIFMT_LEFT_J => {
            (*sta32x).format = fmt & SND_SOC_DAIFMT_FORMAT_MASK;
        }
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => confb |= STA32X_CONFB_C2IM as u8,
        SND_SOC_DAIFMT_NB_IF => confb |= STA32X_CONFB_C1IM as u8,
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*sta32x).regmap,
        STA32X_CONFB,
        STA32X_CONFB_C1IM | STA32X_CONFB_C2IM,
        confb as c_uint,
    )
}

/**
 * sta32x_hw_params - program the STA32X with the given hardware parameters.
 * @substream: the audio stream
 * @params: the hardware parameters to set
 * @dai: the SOC DAI (ignored)
 *
 * This function programs the hardware with the values provided.
 * Specifically, the sample rate and the data format.
 */
unsafe extern "C" fn sta32x_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let sta32x = snd_soc_component_get_drvdata(component) as *mut sta32x_priv;
    let mut i: c_int;
    let mut mcs: c_int = -EINVAL;
    let mut ir: c_int = -EINVAL;
    let confa: c_uint;
    let mut confb: c_uint;
    let rate: c_uint;
    let ratio: c_uint;
    let mut ret: c_int;

    if (*sta32x).mclk == 0 {
        dev_err((*component).dev, b"sta32x->mclk is unset. Unable to determine ratio\n\0".as_ptr() as *const c_char);
        return -EIO;
    }

    rate = params_rate(params);
    ratio = (*sta32x).mclk / rate;
    dev_dbg((*component).dev, b"rate: %u, ratio: %u\n\0".as_ptr() as *const c_char, rate, ratio);

    i = 0;
    while i < interpolation_ratios.len() as c_int {
        if interpolation_ratios[i as usize].fs == rate as c_int {
            ir = interpolation_ratios[i as usize].ir;
            break;
        }
        i += 1;
    }

    if ir < 0 {
        dev_err((*component).dev, b"Unsupported samplerate: %u\n\0".as_ptr() as *const c_char, rate);
        return -EINVAL;
    }

    i = 0;
    while i < 6 {
        if mcs_ratio_table[ir as usize][i as usize] == ratio as c_int {
            mcs = i;
            break;
        }
        i += 1;
    }

    if mcs < 0 {
        dev_err((*component).dev, b"Unresolvable ratio: %u\n\0".as_ptr() as *const c_char, ratio);
        return -EINVAL;
    }

    confa = ((ir as c_uint) << STA32X_CONFA_IR_SHIFT)
        | ((mcs as c_uint) << STA32X_CONFA_MCS_SHIFT);
    confb = 0;

    match params_width(params) {
        24 => {
            dev_dbg((*component).dev, b"24bit\n\0".as_ptr() as *const c_char);
            /* fallthrough */
            dev_dbg((*component).dev, b"24bit or 32bit\n\0".as_ptr() as *const c_char);
            match (*sta32x).format {
                SND_SOC_DAIFMT_I2S => confb |= 0x0,
                SND_SOC_DAIFMT_LEFT_J => confb |= 0x1,
                SND_SOC_DAIFMT_RIGHT_J => confb |= 0x2,
                _ => {}
            }
        }
        32 => {
            dev_dbg((*component).dev, b"24bit or 32bit\n\0".as_ptr() as *const c_char);
            match (*sta32x).format {
                SND_SOC_DAIFMT_I2S => confb |= 0x0,
                SND_SOC_DAIFMT_LEFT_J => confb |= 0x1,
                SND_SOC_DAIFMT_RIGHT_J => confb |= 0x2,
                _ => {}
            }
        }
        20 => {
            dev_dbg((*component).dev, b"20bit\n\0".as_ptr() as *const c_char);
            match (*sta32x).format {
                SND_SOC_DAIFMT_I2S => confb |= 0x4,
                SND_SOC_DAIFMT_LEFT_J => confb |= 0x5,
                SND_SOC_DAIFMT_RIGHT_J => confb |= 0x6,
                _ => {}
            }
        }
        18 => {
            dev_dbg((*component).dev, b"18bit\n\0".as_ptr() as *const c_char);
            match (*sta32x).format {
                SND_SOC_DAIFMT_I2S => confb |= 0x8,
                SND_SOC_DAIFMT_LEFT_J => confb |= 0x9,
                SND_SOC_DAIFMT_RIGHT_J => confb |= 0xa,
                _ => {}
            }
        }
        16 => {
            dev_dbg((*component).dev, b"16bit\n\0".as_ptr() as *const c_char);
            match (*sta32x).format {
                SND_SOC_DAIFMT_I2S => confb |= 0x0,
                SND_SOC_DAIFMT_LEFT_J => confb |= 0xd,
                SND_SOC_DAIFMT_RIGHT_J => confb |= 0xe,
                _ => {}
            }
        }
        _ => return -EINVAL,
    }

    ret = regmap_update_bits(
        (*sta32x).regmap,
        STA32X_CONFA,
        STA32X_CONFA_MCS_MASK | STA32X_CONFA_IR_MASK,
        confa,
    );
    if ret < 0 {
        return ret;
    }

    ret = regmap_update_bits(
        (*sta32x).regmap,
        STA32X_CONFB,
        STA32X_CONFB_SAI_MASK | STA32X_CONFB_SAIFB,
        confb,
    );
    if ret < 0 {
        return ret;
    }

    0
}

unsafe fn sta32x_startup_sequence(sta32x: *mut sta32x_priv) -> c_int {
    if !(*sta32x).gpiod_nreset.is_null() {
        gpiod_set_value((*sta32x).gpiod_nreset, 0);
        mdelay(1);
        gpiod_set_value((*sta32x).gpiod_nreset, 1);
        mdelay(1);
    }
    0
}

/**
 * sta32x_set_bias_level - DAPM callback
 * @component: the component device
 * @level: DAPM power level
 *
 * This is called by ALSA to put the component into low power mode
 * or to wake it up.  If the component is powered off completely
 * all registers must be restored after power on.
 */
unsafe extern "C" fn sta32x_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let mut ret: c_int;
    let sta32x = snd_soc_component_get_drvdata(component) as *mut sta32x_priv;
    let dapm = snd_soc_component_to_dapm(component);

    dev_dbg((*component).dev, b"level = %d\n\0".as_ptr() as *const c_char, level as c_int);
    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {
            /* Full power on */
            regmap_update_bits(
                (*sta32x).regmap,
                STA32X_CONFF,
                STA32X_CONFF_PWDN | STA32X_CONFF_EAPD,
                STA32X_CONFF_PWDN | STA32X_CONFF_EAPD,
            );
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                ret = regulator_bulk_enable((*sta32x).supplies.len() as c_int, (*sta32x).supplies.as_mut_ptr());
                if ret != 0 {
                    dev_err((*component).dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
                    return ret;
                }

                sta32x_startup_sequence(sta32x);
                sta32x_cache_sync(component);
                sta32x_watchdog_start(sta32x);
            }

            /* Power down */
            regmap_update_bits(
                (*sta32x).regmap,
                STA32X_CONFF,
                STA32X_CONFF_PWDN | STA32X_CONFF_EAPD,
                0,
            );
        }
        SND_SOC_BIAS_OFF => {
            /* The chip runs through the power down sequence for us. */
            regmap_update_bits(
                (*sta32x).regmap,
                STA32X_CONFF,
                STA32X_CONFF_PWDN | STA32X_CONFF_EAPD,
                0,
            );
            msleep(300);
            sta32x_watchdog_stop(sta32x);

            gpiod_set_value((*sta32x).gpiod_nreset, 0);

            regulator_bulk_disable((*sta32x).supplies.len() as c_int, (*sta32x).supplies.as_mut_ptr());
        }
    }
    0
}

static sta32x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(sta32x_hw_params),
    set_sysclk: Some(sta32x_set_dai_sysclk),
    set_fmt: Some(sta32x_set_dai_fmt),
};

static mut sta32x_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"sta32x-hifi\0".as_ptr() as *const c_char,
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: STA32X_RATES,
        formats: STA32X_FORMATS,
    },
    ops: &sta32x_dai_ops,
};

unsafe extern "C" fn sta32x_probe(component: *mut snd_soc_component) -> c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let sta32x = snd_soc_component_get_drvdata(component) as *mut sta32x_priv;
    let pdata = (*sta32x).pdata;
    let mut i: c_int;
    let mut ret: c_int = 0;
    let mut thermal: c_int = 0;

    (*sta32x).component = component;

    if !(*sta32x).xti_clk.is_null() {
        ret = clk_prepare_enable((*sta32x).xti_clk);
        if ret != 0 {
            dev_err((*component).dev, b"Failed to enable clock: %d\n\0".as_ptr() as *const c_char, ret);
            return ret;
        }
    }

    ret = regulator_bulk_enable((*sta32x).supplies.len() as c_int, (*sta32x).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err((*component).dev, b"Failed to enable supplies: %d\n\0".as_ptr() as *const c_char, ret);
        clk_disable_unprepare((*sta32x).xti_clk);
        return ret;
    }

    ret = sta32x_startup_sequence(sta32x);
    if ret < 0 {
        dev_err((*component).dev, b"Failed to startup device\n\0".as_ptr() as *const c_char);
        regulator_bulk_disable((*sta32x).supplies.len() as c_int, (*sta32x).supplies.as_mut_ptr());
        clk_disable_unprepare((*sta32x).xti_clk);
        return ret;
    }

    /* CONFA */
    if !(*pdata).thermal_warning_recovery {
        thermal |= STA32X_CONFA_TWAB as c_int;
    }
    if !(*pdata).thermal_warning_adjustment {
        thermal |= STA32X_CONFA_TWRB as c_int;
    }
    if !(*pdata).fault_detect_recovery {
        thermal |= STA32X_CONFA_FDRB as c_int;
    }
    regmap_update_bits(
        (*sta32x).regmap,
        STA32X_CONFA,
        STA32X_CONFA_TWAB | STA32X_CONFA_TWRB | STA32X_CONFA_FDRB,
        thermal as c_uint,
    );

    /* CONFC */
    regmap_update_bits(
        (*sta32x).regmap,
        STA32X_CONFC,
        STA32X_CONFC_CSZ_MASK,
        ((*pdata).drop_compensation_ns as c_uint) << STA32X_CONFC_CSZ_SHIFT,
    );

    /* CONFE */
    regmap_update_bits((*sta32x).regmap, STA32X_CONFE, STA32X_CONFE_MPCV, if (*pdata).max_power_use_mpcc { STA32X_CONFE_MPCV } else { 0 });
    regmap_update_bits((*sta32x).regmap, STA32X_CONFE, STA32X_CONFE_MPC, if (*pdata).max_power_correction { STA32X_CONFE_MPC } else { 0 });
    regmap_update_bits((*sta32x).regmap, STA32X_CONFE, STA32X_CONFE_AME, if (*pdata).am_reduction_mode { STA32X_CONFE_AME } else { 0 });
    regmap_update_bits((*sta32x).regmap, STA32X_CONFE, STA32X_CONFE_PWMS, if (*pdata).odd_pwm_speed_mode { STA32X_CONFE_PWMS } else { 0 });

    /*  CONFF */
    regmap_update_bits((*sta32x).regmap, STA32X_CONFF, STA32X_CONFF_IDE, if (*pdata).invalid_input_detect_mute { STA32X_CONFF_IDE } else { 0 });

    /* select output configuration  */
    regmap_update_bits((*sta32x).regmap, STA32X_CONFF, STA32X_CONFF_OCFG_MASK, ((*pdata).output_conf as c_uint) << STA32X_CONFF_OCFG_SHIFT);

    /* channel to output mapping */
    regmap_update_bits((*sta32x).regmap, STA32X_C1CFG, STA32X_CxCFG_OM_MASK, ((*pdata).ch1_output_mapping as c_uint) << STA32X_CxCFG_OM_SHIFT);
    regmap_update_bits((*sta32x).regmap, STA32X_C2CFG, STA32X_CxCFG_OM_MASK, ((*pdata).ch2_output_mapping as c_uint) << STA32X_CxCFG_OM_SHIFT);
    regmap_update_bits((*sta32x).regmap, STA32X_C3CFG, STA32X_CxCFG_OM_MASK, ((*pdata).ch3_output_mapping as c_uint) << STA32X_CxCFG_OM_SHIFT);

    /* initialize coefficient shadow RAM with reset values */
    i = 4;
    while i <= 49 {
        (*sta32x).coef_shadow[i as usize] = 0x400000;
        i += 5;
    }
    i = 50;
    while i <= 54 {
        (*sta32x).coef_shadow[i as usize] = 0x7fffff;
        i += 1;
    }
    (*sta32x).coef_shadow[55] = 0x5a9df7;
    (*sta32x).coef_shadow[56] = 0x7fffff;
    (*sta32x).coef_shadow[59] = 0x7fffff;
    (*sta32x).coef_shadow[60] = 0x400000;
    (*sta32x).coef_shadow[61] = 0x400000;

    if (*(*sta32x).pdata).needs_esd_watchdog {
        INIT_DELAYED_WORK(&mut (*sta32x).watchdog_work, Some(sta32x_watchdog));
    }

    snd_soc_dapm_force_bias_level(dapm, SND_SOC_BIAS_STANDBY);
    /* Bias level configuration will have done an extra enable */
    regulator_bulk_disable((*sta32x).supplies.len() as c_int, (*sta32x).supplies.as_mut_ptr());

    0
}

unsafe extern "C" fn sta32x_remove(component: *mut snd_soc_component) {
    let sta32x = snd_soc_component_get_drvdata(component) as *mut sta32x_priv;

    sta32x_watchdog_stop(sta32x);
    regulator_bulk_disable((*sta32x).supplies.len() as c_int, (*sta32x).supplies.as_mut_ptr());
    clk_disable_unprepare((*sta32x).xti_clk);
}

static sta32x_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(sta32x_probe),
    remove: Some(sta32x_remove),
    set_bias_level: Some(sta32x_set_bias_level),
    controls: sta32x_snd_controls.as_ptr(),
    num_controls: sta32x_snd_controls.len() as c_uint,
    dapm_widgets: sta32x_dapm_widgets.as_ptr(),
    num_dapm_widgets: sta32x_dapm_widgets.len() as c_uint,
    dapm_routes: sta32x_dapm_routes.as_ptr(),
    num_dapm_routes: sta32x_dapm_routes.len() as c_uint,
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static sta32x_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: STA32X_FDRC2,
    reg_defaults: sta32x_regs.as_ptr(),
    num_reg_defaults: sta32x_regs.len() as c_uint,
    cache_type: REGCACHE_MAPLE,
    wr_table: &sta32x_write_regs,
    rd_table: &sta32x_read_regs,
    volatile_table: &sta32x_volatile_regs,
};

// #ifdef CONFIG_OF
static st32x_dt_ids: [of_device_id; 2] = [
    of_device_id { compatible: b"st,sta32x\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];
module_device_table!(of, st32x_dt_ids);

unsafe fn sta32x_probe_dt(dev: *mut device, sta32x: *mut sta32x_priv) -> c_int {
    let np = (*dev).of_node;
    let pdata: *mut sta32x_platform_data;
    let mut tmp: u16;

    pdata = devm_kzalloc(dev, core::mem::size_of::<sta32x_platform_data>(), GFP_KERNEL) as *mut sta32x_platform_data;
    if pdata.is_null() {
        return -ENOMEM;
    }

    of_property_read_u8(np, b"st,output-conf\0".as_ptr() as *const c_char, &mut (*pdata).output_conf);
    of_property_read_u8(np, b"st,ch1-output-mapping\0".as_ptr() as *const c_char, &mut (*pdata).ch1_output_mapping);
    of_property_read_u8(np, b"st,ch2-output-mapping\0".as_ptr() as *const c_char, &mut (*pdata).ch2_output_mapping);
    of_property_read_u8(np, b"st,ch3-output-mapping\0".as_ptr() as *const c_char, &mut (*pdata).ch3_output_mapping);

    (*pdata).fault_detect_recovery = of_property_read_bool(np, b"st,fault-detect-recovery\0".as_ptr() as *const c_char);
    (*pdata).thermal_warning_recovery = of_property_read_bool(np, b"st,thermal-warning-recovery\0".as_ptr() as *const c_char);
    (*pdata).thermal_warning_adjustment = of_property_read_bool(np, b"st,thermal-warning-adjustment\0".as_ptr() as *const c_char);
    (*pdata).needs_esd_watchdog = of_property_read_bool(np, b"st,needs_esd_watchdog\0".as_ptr() as *const c_char);

    tmp = 140;
    of_property_read_u16(np, b"st,drop-compensation-ns\0".as_ptr() as *const c_char, &mut tmp);
    (*pdata).drop_compensation_ns = (clamp_t_u16(tmp, 0, 300) / 20) as u8;

    /* CONFE */
    (*pdata).max_power_use_mpcc = of_property_read_bool(np, b"st,max-power-use-mpcc\0".as_ptr() as *const c_char);
    (*pdata).max_power_correction = of_property_read_bool(np, b"st,max-power-correction\0".as_ptr() as *const c_char);
    (*pdata).am_reduction_mode = of_property_read_bool(np, b"st,am-reduction-mode\0".as_ptr() as *const c_char);
    (*pdata).odd_pwm_speed_mode = of_property_read_bool(np, b"st,odd-pwm-speed-mode\0".as_ptr() as *const c_char);

    /* CONFF */
    (*pdata).invalid_input_detect_mute = of_property_read_bool(np, b"st,invalid-input-detect-mute\0".as_ptr() as *const c_char);

    (*sta32x).pdata = pdata;
    0
}
// #endif

unsafe extern "C" fn sta32x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let dev = &mut (*i2c).dev as *mut device;
    let sta32x: *mut sta32x_priv;
    let mut ret: c_int;
    let mut i: c_int;

    sta32x = devm_kzalloc(&mut (*i2c).dev, core::mem::size_of::<sta32x_priv>(), GFP_KERNEL) as *mut sta32x_priv;
    if sta32x.is_null() {
        return -ENOMEM;
    }

    mutex_init(&mut (*sta32x).coeff_lock);
    (*sta32x).pdata = dev_get_platdata(dev) as *mut sta32x_platform_data;

    // #ifdef CONFIG_OF
    if !(*dev).of_node.is_null() {
        ret = sta32x_probe_dt(dev, sta32x);
        if ret < 0 {
            return ret;
        }
    }
    // #endif

    /* Clock */
    (*sta32x).xti_clk = devm_clk_get(dev, b"xti\0".as_ptr() as *const c_char);
    if IS_ERR((*sta32x).xti_clk as *const c_void) {
        ret = PTR_ERR((*sta32x).xti_clk as *const c_void);

        if ret == -EPROBE_DEFER {
            return ret;
        }

        (*sta32x).xti_clk = core::ptr::null_mut();
    }

    /* GPIOs */
    (*sta32x).gpiod_nreset = devm_gpiod_get_optional(dev, b"reset\0".as_ptr() as *const c_char, GPIOD_OUT_LOW);
    if IS_ERR((*sta32x).gpiod_nreset as *const c_void) {
        return PTR_ERR((*sta32x).gpiod_nreset as *const c_void);
    }

    /* regulators */
    i = 0;
    while i < (*sta32x).supplies.len() as c_int {
        (*sta32x).supplies[i as usize].supply = sta32x_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(&mut (*i2c).dev, (*sta32x).supplies.len() as c_int, (*sta32x).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err(&mut (*i2c).dev, b"Failed to request supplies: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    (*sta32x).regmap = devm_regmap_init_i2c(i2c, &sta32x_regmap);
    if IS_ERR((*sta32x).regmap as *const c_void) {
        ret = PTR_ERR((*sta32x).regmap as *const c_void);
        dev_err(dev, b"Failed to init regmap: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }

    i2c_set_clientdata(i2c, sta32x as *mut c_void);

    ret = devm_snd_soc_register_component(dev, &sta32x_component, &mut sta32x_dai, 1);
    if ret < 0 {
        dev_err(dev, b"Failed to register component (%d)\n\0".as_ptr() as *const c_char, ret);
    }

    ret
}

static sta32x_i2c_id: [i2c_device_id; 4] = [
    i2c_device_id { name: *b"sta326\0\0\0\0\0\0\0\0\0\0" },
    i2c_device_id { name: *b"sta328\0\0\0\0\0\0\0\0\0\0" },
    i2c_device_id { name: *b"sta329\0\0\0\0\0\0\0\0\0\0" },
    i2c_device_id { name: [0; 16] },
];
module_device_table!(i2c, sta32x_i2c_id);

static mut sta32x_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: b"sta32x\0".as_ptr() as *const c_char,
        of_match_table: of_match_ptr(st32x_dt_ids.as_ptr()),
    },
    probe: Some(sta32x_i2c_probe),
    id_table: sta32x_i2c_id.as_ptr(),
};

module_i2c_driver!(sta32x_i2c_driver);

MODULE_DESCRIPTION!("ASoC STA32X driver");
MODULE_AUTHOR!("Johannes Stezenbach <js@sig21.net>");
MODULE_LICENSE!("GPL");

extern "C" {
    static system_power_efficient_wq: *mut workqueue_struct;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_to_dapm(component: *mut snd_soc_component) -> *mut snd_soc_dapm_context;
    fn snd_soc_dapm_get_bias_level(dapm: *mut snd_soc_dapm_context) -> snd_soc_bias_level;
    fn snd_soc_dapm_force_bias_level(dapm: *mut snd_soc_dapm_context, level: snd_soc_bias_level);
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn queue_delayed_work(wq: *mut workqueue_struct, work: *mut delayed_work, delay: c_ulong) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn round_jiffies_relative(j: c_ulong) -> c_ulong;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn mdelay(msecs: c_uint);
    fn msleep(msecs: c_uint);
    fn regulator_bulk_enable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn regulator_bulk_disable(num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_get_platdata(dev: *mut device) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_gpiod_get_optional(dev: *mut device, con_id: *const c_char, flags: c_uint) -> *mut gpio_desc;
    fn devm_regulator_bulk_get(dev: *mut device, num_consumers: c_int, consumers: *mut regulator_bulk_data) -> c_int;
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn i2c_set_clientdata(client: *mut i2c_client, data: *mut c_void);
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn of_property_read_u8(np: *mut device_node, propname: *const c_char, out_value: *mut u8) -> c_int;
    fn of_property_read_u16(np: *mut device_node, propname: *const c_char, out_value: *mut u16) -> c_int;
    fn of_property_read_bool(np: *mut device_node, propname: *const c_char) -> bool;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

type c_ulong = ::core::ffi::c_ulong;

#[repr(C)] pub struct regmap { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)] pub struct snd_kcontrol { private_value: c_ulong }
#[repr(C)] pub struct snd_ctl_elem_info { type_: c_uint, count: c_uint }
#[repr(C)] pub struct snd_ctl_elem_value { value: snd_ctl_elem_value_union }
#[repr(C)] pub union snd_ctl_elem_value_union { bytes: snd_ctl_elem_value_bytes }
#[repr(C)] #[derive(Copy, Clone)] pub struct snd_ctl_elem_value_bytes { data: [u8; 512] }
#[repr(C)] pub struct snd_soc_component { dev: *mut device }
#[repr(C)] pub struct snd_soc_dapm_context { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dai { component: *mut snd_soc_component }
#[repr(C)] pub struct snd_pcm_substream { _private: [u8; 0] }
#[repr(C)] pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)] pub struct device { of_node: *mut device_node }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { dev: device }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }

#[repr(C)]
pub struct sta32x_platform_data {
    output_conf: u8,
    ch1_output_mapping: u8,
    ch2_output_mapping: u8,
    ch3_output_mapping: u8,
    fault_detect_recovery: bool,
    thermal_warning_recovery: bool,
    thermal_warning_adjustment: bool,
    needs_esd_watchdog: bool,
    drop_compensation_ns: u8,
    max_power_use_mpcc: bool,
    max_power_correction: bool,
    am_reduction_mode: bool,
    odd_pwm_speed_mode: bool,
    invalid_input_detect_mute: bool,
}

#[repr(C)] pub struct snd_kcontrol_new { iface: c_uint, name: *const c_char, info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>, get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>, private_value: c_ulong }
#[repr(C)] pub struct snd_soc_dapm_widget { _private: [u8; 0] }
#[repr(C)] pub struct snd_soc_dapm_route { sink: *const c_char, control: *const c_char, source: *const c_char }
#[repr(C)] pub struct snd_soc_dai_ops { hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>, set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>, set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int> }
#[repr(C)] pub struct snd_soc_pcm_stream { stream_name: *const c_char, channels_min: c_uint, channels_max: c_uint, rates: c_uint, formats: c_uint }
#[repr(C)] pub struct snd_soc_dai_driver { name: *const c_char, playback: snd_soc_pcm_stream, ops: *const snd_soc_dai_ops }
#[repr(C)] pub struct snd_soc_component_driver { probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>, remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>, set_bias_level: Option<unsafe extern "C" fn(*mut snd_soc_component, snd_soc_bias_level) -> c_int>, controls: *const snd_kcontrol_new, num_controls: c_uint, dapm_widgets: *const snd_soc_dapm_widget, num_dapm_widgets: c_uint, dapm_routes: *const snd_soc_dapm_route, num_dapm_routes: c_uint, suspend_bias_off: c_uint, idle_bias_on: c_uint, use_pmdown_time: c_uint, endianness: c_uint }
#[repr(C)] pub struct regmap_config { reg_bits: c_uint, val_bits: c_uint, max_register: c_uint, reg_defaults: *const reg_default, num_reg_defaults: c_uint, cache_type: c_uint, wr_table: *const regmap_access_table, rd_table: *const regmap_access_table, volatile_table: *const regmap_access_table }
#[repr(C)] pub struct of_device_id { compatible: *const c_char }
#[repr(C)] pub struct i2c_device_id { name: [u8; 16] }
#[repr(C)] pub struct device_driver { name: *const c_char, of_match_table: *const of_device_id }
#[repr(C)] pub struct i2c_driver { driver: device_driver, probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>, id_table: *const i2c_device_id }

type snd_soc_bias_level = c_uint;

const fn declare_tlv_db_scale(_min: c_int, _step: c_int, _mute: c_int) -> [c_uint; 4] { [0; 4] }
fn clamp_t_u16(v: u16, lo: u16, hi: u16) -> u16 { if v < lo { lo } else if v > hi { hi } else { v } }

extern "Rust" {
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_match_ptr(ptr: *const of_device_id) -> *const of_device_id;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: Option<unsafe extern "C" fn(*mut work_struct)>);
}

macro_rules! declare_tlv_db_range { ($($tt:tt)*) => {}; }
macro_rules! soc_enum_single_decl { ($($tt:tt)*) => {}; }
macro_rules! module_device_table { ($($tt:tt)*) => {}; }
macro_rules! module_i2c_driver { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! SOC_SINGLE { ($($tt:tt)*) => { snd_kcontrol_new { iface: 0, name: core::ptr::null(), info: None, get: None, put: None, private_value: 0 } }; }
macro_rules! SOC_SINGLE_TLV { ($($tt:tt)*) => { snd_kcontrol_new { iface: 0, name: core::ptr::null(), info: None, get: None, put: None, private_value: 0 } }; }
macro_rules! SOC_ENUM { ($($tt:tt)*) => { snd_kcontrol_new { iface: 0, name: core::ptr::null(), info: None, get: None, put: None, private_value: 0 } }; }
macro_rules! SND_SOC_DAPM_DAC { ($($tt:tt)*) => { snd_soc_dapm_widget { _private: [] } }; }
macro_rules! SND_SOC_DAPM_OUTPUT { ($($tt:tt)*) => { snd_soc_dapm_widget { _private: [] } }; }
macro_rules! TLV_DB_SCALE_ITEM { ($($tt:tt)*) => { 0 }; }
macro_rules! container_of { ($ptr:expr, $type:ty, $field:tt.$subfield:tt) => { $ptr as *mut $type }; }

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const GFP_KERNEL: c_uint = 0;
const REGCACHE_MAPLE: c_uint = 0;
const GPIOD_OUT_LOW: c_uint = 0;
const HZ: c_ulong = 100;
const SNDRV_CTL_ELEM_TYPE_BYTES: c_uint = 4;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 2;
const SND_SOC_NOPM: c_int = -1;
const TLV_DB_GAIN_MUTE: c_int = -9999999;

extern "Rust" {
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S18_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static STA32X_COEF_COUNT: c_uint;
    static STA32X_CONFA: c_uint;
    static STA32X_CONFB: c_uint;
    static STA32X_CONFC: c_uint;
    static STA32X_CONFD: c_uint;
    static STA32X_CONFE: c_uint;
    static STA32X_CONFF: c_uint;
    static STA32X_MVOL: c_uint;
    static STA32X_MMUTE: c_uint;
    static STA32X_C1VOL: c_uint;
    static STA32X_C2VOL: c_uint;
    static STA32X_C3VOL: c_uint;
    static STA32X_AUTO1: c_uint;
    static STA32X_AUTO2: c_uint;
    static STA32X_AUTO3: c_uint;
    static STA32X_C1CFG: c_uint;
    static STA32X_C2CFG: c_uint;
    static STA32X_C3CFG: c_uint;
    static STA32X_TONE: c_uint;
    static STA32X_L1AR: c_uint;
    static STA32X_L2AR: c_uint;
    static STA32X_L1ATRT: c_uint;
    static STA32X_L2ATRT: c_uint;
    static STA32X_CFADDR2: c_uint;
    static STA32X_CFUD: c_uint;
    static STA32X_B1CF1: c_uint;
    static STA32X_B1CF2: c_uint;
    static STA32X_B1CF3: c_uint;
    static STA32X_FDRC2: c_uint;
    static STA32X_MMUTE_MMUTE: c_uint;
    static STA32X_CONFD_DRC_SHIFT: c_uint;
    static STA32X_AUTO1_AMEQ_SHIFT: c_uint;
    static STA32X_AUTO1_AMGC_SHIFT: c_uint;
    static STA32X_AUTO2_XO_SHIFT: c_uint;
    static STA32X_AUTO3_PEQ_SHIFT: c_uint;
    static STA32X_CxCFG_LS_SHIFT: c_uint;
    static STA32X_LxA_SHIFT: c_uint;
    static STA32X_LxR_SHIFT: c_uint;
    static STA32X_CONFD_DEMP_SHIFT: c_uint;
    static STA32X_CONFD_MME_SHIFT: c_uint;
    static STA32X_CONFE_ZCE_SHIFT: c_uint;
    static STA32X_CONFE_SVE_SHIFT: c_uint;
    static STA32X_CONFF_IDE_SHIFT: c_uint;
    static STA32X_CxCFG_TCB_SHIFT: c_uint;
    static STA32X_CxCFG_EQBP_SHIFT: c_uint;
    static STA32X_CxCFG_VBP_SHIFT: c_uint;
    static STA32X_TONE_BTC_SHIFT: c_uint;
    static STA32X_TONE_TTC_SHIFT: c_uint;
    static STA32X_CONFA_IR_SHIFT: c_uint;
    static STA32X_CONFA_MCS_SHIFT: c_uint;
    static STA32X_CONFA_MCS_MASK: c_uint;
    static STA32X_CONFA_IR_MASK: c_uint;
    static STA32X_CONFB_SAI_MASK: c_uint;
    static STA32X_CONFB_SAIFB: c_uint;
    static STA32X_CONFB_C1IM: c_uint;
    static STA32X_CONFB_C2IM: c_uint;
    static STA32X_CONFF_PWDN: c_uint;
    static STA32X_CONFF_EAPD: c_uint;
    static STA32X_CONFA_TWAB: c_uint;
    static STA32X_CONFA_TWRB: c_uint;
    static STA32X_CONFA_FDRB: c_uint;
    static STA32X_CONFC_CSZ_MASK: c_uint;
    static STA32X_CONFC_CSZ_SHIFT: c_uint;
    static STA32X_CONFE_MPCV: c_uint;
    static STA32X_CONFE_MPC: c_uint;
    static STA32X_CONFE_AME: c_uint;
    static STA32X_CONFE_PWMS: c_uint;
    static STA32X_CONFF_IDE: c_uint;
    static STA32X_CONFF_OCFG_MASK: c_uint;
    static STA32X_CONFF_OCFG_SHIFT: c_uint;
    static STA32X_CxCFG_OM_MASK: c_uint;
    static STA32X_CxCFG_OM_SHIFT: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_BIAS_ON: snd_soc_bias_level;
    static SND_SOC_BIAS_PREPARE: snd_soc_bias_level;
    static SND_SOC_BIAS_STANDBY: snd_soc_bias_level;
    static SND_SOC_BIAS_OFF: snd_soc_bias_level;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
