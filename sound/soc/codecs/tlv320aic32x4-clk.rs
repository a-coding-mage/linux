// SPDX-License-Identifier: GPL-2.0
/*
 * Clock Tree for the Texas Instruments TLV320AIC32x4
 *
 * Copyright 2019 Annaliese McDermond
 *
 * Author: Annaliese McDermond <nh6z@nh6z.net>
 */

// Dependencies from:
// <linux/clk-provider.h>
// <linux/clkdev.h>
// <linux/delay.h>
// <linux/regmap.h>
// <linux/device.h>
// "tlv320aic32x4.h"

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

extern "C" {
    static AIC32X4_PLLPR: c_uint;
    static AIC32X4_PLLEN: c_uint;
    static AIC32X4_PLL_R_MASK: c_uint;
    static AIC32X4_PLL_P_MASK: c_uint;
    static AIC32X4_PLL_P_SHIFT: c_uint;
    static AIC32X4_PLLJ: c_uint;
    static AIC32X4_PLLDMSB: c_uint;
    static AIC32X4_PLLDLSB: c_uint;
    static AIC32X4_MAX_PLL_CLKIN: c_ulong;
    static AIC32X4_CLKMUX: c_uint;
    static AIC32X4_PLL_CLKIN_MASK: c_uint;
    static AIC32X4_PLL_CLKIN_SHIFT: c_uint;
    static AIC32X4_CODEC_CLKIN_MASK: c_uint;
    static AIC32X4_CODEC_CLKIN_SHIFT: c_uint;
    static AIC32X4_DIVEN: c_uint;
    static AIC32X4_DIV_MAX: c_uint;
    static AIC32X4_DIV_MASK: c_uint;
    static AIC32X4_IFACE3: c_uint;
    static AIC32X4_BDIVCLK_MASK: c_uint;
    static AIC32X4_NDAC: c_uint;
    static AIC32X4_MDAC: c_uint;
    static AIC32X4_NADC: c_uint;
    static AIC32X4_MADC: c_uint;
    static AIC32X4_BCLKN: c_uint;

    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn usleep_range(min: c_ulong, max: c_ulong);
    fn clk_hw_determine_rate_no_reparent(
        hw: *mut clk_hw,
        req: *mut clk_rate_request,
    ) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn dev_get_regmap(dev: *mut device, name: *const c_char) -> *mut regmap;
    fn clk_hw_register_clkdev(
        hw: *mut clk_hw,
        con_id: *const c_char,
        dev_id: *const c_char,
    ) -> c_int;
    fn devm_clk_register(dev: *mut device, hw: *mut clk_hw) -> *mut clk;
    fn ERR_PTR(error: c_long) -> *mut clk;
}

type c_long = isize;

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_rate_request {
    pub rate: c_ulong,
    pub best_parent_rate: c_ulong,
}

#[repr(C)]
pub struct clk_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub unprepare: Option<unsafe extern "C" fn(*mut clk_hw)>,
    pub is_prepared: Option<unsafe extern "C" fn(*mut clk_hw) -> c_int>,
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> c_int>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong, c_ulong) -> c_int>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> c_int>,
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub parent_names: *const *const c_char,
    pub num_parents: c_uint,
    pub flags: c_uint,
}

#[repr(C)]
struct clk_aic32x4 {
    hw: clk_hw,
    dev: *mut device,
    regmap: *mut regmap,
    reg: c_uint,
}

unsafe fn to_clk_aic32x4(hw: *mut clk_hw) -> *mut clk_aic32x4 {
    hw as *mut clk_aic32x4
}

/*
 * struct clk_aic32x4_pll_muldiv - Multiplier/divider settings
 * @p:		Divider
 * @r:		first multiplier
 * @j:		integer part of second multiplier
 * @d:		decimal part of second multiplier
 */
#[repr(C)]
struct clk_aic32x4_pll_muldiv {
    p: u8,
    r: u16,
    j: u8,
    d: u16,
}

#[repr(C)]
struct aic32x4_clkdesc {
    name: *const c_char,
    parent_names: *const *const c_char,
    num_parents: c_uint,
    ops: *const clk_ops,
    reg: c_uint,
}

unsafe extern "C" fn clk_aic32x4_pll_prepare(hw: *mut clk_hw) -> c_int {
    let pll = to_clk_aic32x4(hw);

    regmap_update_bits(
        (*pll).regmap,
        AIC32X4_PLLPR,
        AIC32X4_PLLEN,
        AIC32X4_PLLEN,
    )
}

unsafe extern "C" fn clk_aic32x4_pll_unprepare(hw: *mut clk_hw) {
    let pll = to_clk_aic32x4(hw);

    regmap_update_bits((*pll).regmap, AIC32X4_PLLPR, AIC32X4_PLLEN, 0);
}

unsafe extern "C" fn clk_aic32x4_pll_is_prepared(hw: *mut clk_hw) -> c_int {
    let pll = to_clk_aic32x4(hw);

    let mut val: c_uint = 0;
    let ret: c_int;

    ret = regmap_read((*pll).regmap, AIC32X4_PLLPR, &mut val);
    if ret < 0 {
        return ret;
    }

    ((val & AIC32X4_PLLEN) != 0) as c_int
}

unsafe fn clk_aic32x4_pll_get_muldiv(
    pll: *mut clk_aic32x4,
    settings: *mut clk_aic32x4_pll_muldiv,
) -> c_int {
    /*	Change to use regmap_bulk_read? */
    let mut val: c_uint = 0;
    let mut ret: c_int;

    ret = regmap_read((*pll).regmap, AIC32X4_PLLPR, &mut val);
    if ret < 0 {
        return ret;
    }
    (*settings).r = (val & AIC32X4_PLL_R_MASK) as u16;
    (*settings).p = ((val & AIC32X4_PLL_P_MASK) >> AIC32X4_PLL_P_SHIFT) as u8;

    ret = regmap_read((*pll).regmap, AIC32X4_PLLJ, &mut val);
    if ret < 0 {
        return ret;
    }
    (*settings).j = val as u8;

    ret = regmap_read((*pll).regmap, AIC32X4_PLLDMSB, &mut val);
    if ret < 0 {
        return ret;
    }
    (*settings).d = (val << 8) as u16;

    ret = regmap_read((*pll).regmap, AIC32X4_PLLDLSB, &mut val);
    if ret < 0 {
        return ret;
    }
    (*settings).d |= val as u16;

    0
}

unsafe fn clk_aic32x4_pll_set_muldiv(
    pll: *mut clk_aic32x4,
    settings: *mut clk_aic32x4_pll_muldiv,
) -> c_int {
    let mut ret: c_int;
    /*	Change to use regmap_bulk_write for some if not all? */

    ret = regmap_update_bits(
        (*pll).regmap,
        AIC32X4_PLLPR,
        AIC32X4_PLL_R_MASK,
        (*settings).r as c_uint,
    );
    if ret < 0 {
        return ret;
    }

    ret = regmap_update_bits(
        (*pll).regmap,
        AIC32X4_PLLPR,
        AIC32X4_PLL_P_MASK,
        ((*settings).p as c_uint) << AIC32X4_PLL_P_SHIFT,
    );
    if ret < 0 {
        return ret;
    }

    ret = regmap_write((*pll).regmap, AIC32X4_PLLJ, (*settings).j as c_uint);
    if ret < 0 {
        return ret;
    }

    ret = regmap_write((*pll).regmap, AIC32X4_PLLDMSB, ((*settings).d >> 8) as c_uint);
    if ret < 0 {
        return ret;
    }
    ret = regmap_write((*pll).regmap, AIC32X4_PLLDLSB, ((*settings).d & 0xff) as c_uint);
    if ret < 0 {
        return ret;
    }

    0
}

unsafe fn div_round_up_ull(n: u64, d: u64) -> u64 {
    n.wrapping_add(d).wrapping_sub(1) / d
}

unsafe fn div_round_up(n: c_ulong, d: c_ulong) -> c_ulong {
    n.wrapping_add(d).wrapping_sub(1) / d
}

unsafe fn do_div(n: &mut u64, base: u32) -> u32 {
    let rem = (*n % base as u64) as u32;
    *n /= base as u64;
    rem
}

unsafe fn clk_aic32x4_pll_calc_rate(
    settings: *mut clk_aic32x4_pll_muldiv,
    parent_rate: c_ulong,
) -> c_ulong {
    let rate: u64;
    /*
     * We scale j by 10000 to account for the decimal part of P and divide
     * it back out later.
     */
    rate = (parent_rate as u64)
        .wrapping_mul((*settings).r as u64)
        .wrapping_mul((((*settings).j as u64).wrapping_mul(10000)).wrapping_add((*settings).d as u64));

    div_round_up_ull(rate, ((*settings).p as u64).wrapping_mul(10000)) as c_ulong
}

unsafe fn clk_aic32x4_pll_calc_muldiv(
    settings: *mut clk_aic32x4_pll_muldiv,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let mut multiplier: u64;

    (*settings).p = (parent_rate / AIC32X4_MAX_PLL_CLKIN + 1) as u8;
    if (*settings).p > 8 {
        return -1;
    }

    /*
     * We scale this figure by 10000 so that we can get the decimal part
     * of the multiplier.	This is because we can't do floating point
     * math in the kernel.
     */
    multiplier = (rate as u64)
        .wrapping_mul((*settings).p as u64)
        .wrapping_mul(10000);
    do_div(&mut multiplier, parent_rate as u32);

    /*
     * J can't be over 64, so R can scale this.
     * R can't be greater than 4.
     */
    (*settings).r = ((multiplier as u32 / 640000) + 1) as u16;
    if (*settings).r > 4 {
        return -1;
    }
    do_div(&mut multiplier, (*settings).r as u32);

    /*
     * J can't be < 1.
     */
    if multiplier < 10000 {
        return -1;
    }

    /* Figure out the integer part, J, and the fractional part, D. */
    (*settings).j = (multiplier as u32 / 10000) as u8;
    (*settings).d = (multiplier as u32 % 10000) as u16;

    0
}

unsafe extern "C" fn clk_aic32x4_pll_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
) -> c_ulong {
    let pll = to_clk_aic32x4(hw);
    let mut settings = MaybeUninit::<clk_aic32x4_pll_muldiv>::uninit();
    let ret: c_int;

    ret = clk_aic32x4_pll_get_muldiv(pll, settings.as_mut_ptr());
    if ret < 0 {
        return 0;
    }

    clk_aic32x4_pll_calc_rate(settings.as_mut_ptr(), parent_rate)
}

unsafe extern "C" fn clk_aic32x4_pll_determine_rate(
    _hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let mut settings = MaybeUninit::<clk_aic32x4_pll_muldiv>::uninit();
    let ret: c_int;

    ret = clk_aic32x4_pll_calc_muldiv(settings.as_mut_ptr(), (*req).rate, (*req).best_parent_rate);
    if ret < 0 {
        return -EINVAL;
    }

    (*req).rate = clk_aic32x4_pll_calc_rate(settings.as_mut_ptr(), (*req).best_parent_rate);

    0
}

unsafe extern "C" fn clk_aic32x4_pll_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let pll = to_clk_aic32x4(hw);
    let mut settings = MaybeUninit::<clk_aic32x4_pll_muldiv>::uninit();
    let mut ret: c_int;

    ret = clk_aic32x4_pll_calc_muldiv(settings.as_mut_ptr(), rate, parent_rate);
    if ret < 0 {
        return -EINVAL;
    }

    ret = clk_aic32x4_pll_set_muldiv(pll, settings.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    /* 10ms is the delay to wait before the clocks are stable */
    usleep_range(10000, 20000);

    0
}

unsafe extern "C" fn clk_aic32x4_pll_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let pll = to_clk_aic32x4(hw);

    regmap_update_bits(
        (*pll).regmap,
        AIC32X4_CLKMUX,
        AIC32X4_PLL_CLKIN_MASK,
        (index as c_uint) << AIC32X4_PLL_CLKIN_SHIFT,
    )
}

unsafe extern "C" fn clk_aic32x4_pll_get_parent(hw: *mut clk_hw) -> u8 {
    let pll = to_clk_aic32x4(hw);
    let mut val: c_uint = 0;

    regmap_read((*pll).regmap, AIC32X4_PLLPR, &mut val);

    ((val & AIC32X4_PLL_CLKIN_MASK) >> AIC32X4_PLL_CLKIN_SHIFT) as u8
}

static aic32x4_pll_ops: clk_ops = clk_ops {
    prepare: Some(clk_aic32x4_pll_prepare),
    unprepare: Some(clk_aic32x4_pll_unprepare),
    is_prepared: Some(clk_aic32x4_pll_is_prepared),
    recalc_rate: Some(clk_aic32x4_pll_recalc_rate),
    determine_rate: Some(clk_aic32x4_pll_determine_rate),
    set_rate: Some(clk_aic32x4_pll_set_rate),
    set_parent: Some(clk_aic32x4_pll_set_parent),
    get_parent: Some(clk_aic32x4_pll_get_parent),
};

unsafe extern "C" fn clk_aic32x4_codec_clkin_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let mux = to_clk_aic32x4(hw);

    regmap_update_bits(
        (*mux).regmap,
        AIC32X4_CLKMUX,
        AIC32X4_CODEC_CLKIN_MASK,
        (index as c_uint) << AIC32X4_CODEC_CLKIN_SHIFT,
    )
}

unsafe extern "C" fn clk_aic32x4_codec_clkin_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = to_clk_aic32x4(hw);
    let mut val: c_uint = 0;

    regmap_read((*mux).regmap, AIC32X4_CLKMUX, &mut val);

    ((val & AIC32X4_CODEC_CLKIN_MASK) >> AIC32X4_CODEC_CLKIN_SHIFT) as u8
}

static aic32x4_codec_clkin_ops: clk_ops = clk_ops {
    prepare: None,
    unprepare: None,
    is_prepared: None,
    recalc_rate: None,
    determine_rate: Some(clk_hw_determine_rate_no_reparent),
    set_rate: None,
    set_parent: Some(clk_aic32x4_codec_clkin_set_parent),
    get_parent: Some(clk_aic32x4_codec_clkin_get_parent),
};

unsafe extern "C" fn clk_aic32x4_div_prepare(hw: *mut clk_hw) -> c_int {
    let div = to_clk_aic32x4(hw);

    regmap_update_bits((*div).regmap, (*div).reg, AIC32X4_DIVEN, AIC32X4_DIVEN)
}

unsafe extern "C" fn clk_aic32x4_div_unprepare(hw: *mut clk_hw) {
    let div = to_clk_aic32x4(hw);

    regmap_update_bits((*div).regmap, (*div).reg, AIC32X4_DIVEN, 0);
}

unsafe extern "C" fn clk_aic32x4_div_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let div = to_clk_aic32x4(hw);
    let divisor: u8;

    divisor = div_round_up(parent_rate, rate) as u8;
    if divisor as c_uint > AIC32X4_DIV_MAX {
        return -EINVAL;
    }

    regmap_update_bits((*div).regmap, (*div).reg, AIC32X4_DIV_MASK, divisor as c_uint)
}

unsafe extern "C" fn clk_aic32x4_div_determine_rate(
    _hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let divisor: c_ulong;

    divisor = div_round_up((*req).best_parent_rate, (*req).rate);
    if divisor > AIC32X4_DIV_MAX as c_ulong {
        return -EINVAL;
    }

    (*req).rate = div_round_up((*req).best_parent_rate, divisor);
    0
}

unsafe extern "C" fn clk_aic32x4_div_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
) -> c_ulong {
    let div = to_clk_aic32x4(hw);
    let mut val: c_uint = 0;
    let err: c_int;

    err = regmap_read((*div).regmap, (*div).reg, &mut val);
    if err != 0 {
        return 0;
    }

    val &= AIC32X4_DIV_MASK;
    if val == 0 {
        val = AIC32X4_DIV_MAX;
    }

    div_round_up(parent_rate, val as c_ulong)
}

static aic32x4_div_ops: clk_ops = clk_ops {
    prepare: Some(clk_aic32x4_div_prepare),
    unprepare: Some(clk_aic32x4_div_unprepare),
    is_prepared: None,
    recalc_rate: Some(clk_aic32x4_div_recalc_rate),
    determine_rate: Some(clk_aic32x4_div_determine_rate),
    set_rate: Some(clk_aic32x4_div_set_rate),
    set_parent: None,
    get_parent: None,
};

unsafe extern "C" fn clk_aic32x4_bdiv_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let mux = to_clk_aic32x4(hw);

    regmap_update_bits(
        (*mux).regmap,
        AIC32X4_IFACE3,
        AIC32X4_BDIVCLK_MASK,
        index as c_uint,
    )
}

unsafe extern "C" fn clk_aic32x4_bdiv_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = to_clk_aic32x4(hw);
    let mut val: c_uint = 0;

    regmap_read((*mux).regmap, AIC32X4_IFACE3, &mut val);

    (val & AIC32X4_BDIVCLK_MASK) as u8
}

static aic32x4_bdiv_ops: clk_ops = clk_ops {
    prepare: Some(clk_aic32x4_div_prepare),
    unprepare: Some(clk_aic32x4_div_unprepare),
    is_prepared: None,
    recalc_rate: Some(clk_aic32x4_div_recalc_rate),
    determine_rate: Some(clk_aic32x4_div_determine_rate),
    set_rate: Some(clk_aic32x4_div_set_rate),
    set_parent: Some(clk_aic32x4_bdiv_set_parent),
    get_parent: Some(clk_aic32x4_bdiv_get_parent),
};

static mut AIC32X4_PLL_PARENT_NAMES: [*const c_char; 4] = [
    b"mclk\0".as_ptr() as *const c_char,
    b"bclk\0".as_ptr() as *const c_char,
    b"gpio\0".as_ptr() as *const c_char,
    b"din\0".as_ptr() as *const c_char,
];
static mut AIC32X4_CODEC_CLKIN_PARENT_NAMES: [*const c_char; 4] = [
    b"mclk\0".as_ptr() as *const c_char,
    b"bclk\0".as_ptr() as *const c_char,
    b"gpio\0".as_ptr() as *const c_char,
    b"pll\0".as_ptr() as *const c_char,
];
static AIC32X4_NDAC_PARENT_NAMES: [*const c_char; 1] = [b"codec_clkin\0".as_ptr() as *const c_char];
static AIC32X4_MDAC_PARENT_NAMES: [*const c_char; 1] = [b"ndac\0".as_ptr() as *const c_char];
static AIC32X4_NADC_PARENT_NAMES: [*const c_char; 1] = [b"codec_clkin\0".as_ptr() as *const c_char];
static AIC32X4_MADC_PARENT_NAMES: [*const c_char; 1] = [b"nadc\0".as_ptr() as *const c_char];
static AIC32X4_BDIV_PARENT_NAMES: [*const c_char; 4] = [
    b"ndac\0".as_ptr() as *const c_char,
    b"mdac\0".as_ptr() as *const c_char,
    b"nadc\0".as_ptr() as *const c_char,
    b"madc\0".as_ptr() as *const c_char,
];

static mut aic32x4_clkdesc_array: [aic32x4_clkdesc; 7] = [
    aic32x4_clkdesc {
        name: b"pll\0".as_ptr() as *const c_char,
        parent_names: unsafe { AIC32X4_PLL_PARENT_NAMES.as_ptr() },
        num_parents: 4,
        ops: &aic32x4_pll_ops,
        reg: 0,
    },
    aic32x4_clkdesc {
        name: b"codec_clkin\0".as_ptr() as *const c_char,
        parent_names: unsafe { AIC32X4_CODEC_CLKIN_PARENT_NAMES.as_ptr() },
        num_parents: 4,
        ops: &aic32x4_codec_clkin_ops,
        reg: 0,
    },
    aic32x4_clkdesc {
        name: b"ndac\0".as_ptr() as *const c_char,
        parent_names: AIC32X4_NDAC_PARENT_NAMES.as_ptr(),
        num_parents: 1,
        ops: &aic32x4_div_ops,
        reg: unsafe { AIC32X4_NDAC },
    },
    aic32x4_clkdesc {
        name: b"mdac\0".as_ptr() as *const c_char,
        parent_names: AIC32X4_MDAC_PARENT_NAMES.as_ptr(),
        num_parents: 1,
        ops: &aic32x4_div_ops,
        reg: unsafe { AIC32X4_MDAC },
    },
    aic32x4_clkdesc {
        name: b"nadc\0".as_ptr() as *const c_char,
        parent_names: AIC32X4_NADC_PARENT_NAMES.as_ptr(),
        num_parents: 1,
        ops: &aic32x4_div_ops,
        reg: unsafe { AIC32X4_NADC },
    },
    aic32x4_clkdesc {
        name: b"madc\0".as_ptr() as *const c_char,
        parent_names: AIC32X4_MADC_PARENT_NAMES.as_ptr(),
        num_parents: 1,
        ops: &aic32x4_div_ops,
        reg: unsafe { AIC32X4_MADC },
    },
    aic32x4_clkdesc {
        name: b"bdiv\0".as_ptr() as *const c_char,
        parent_names: AIC32X4_BDIV_PARENT_NAMES.as_ptr(),
        num_parents: 4,
        ops: &aic32x4_bdiv_ops,
        reg: unsafe { AIC32X4_BCLKN },
    },
];

unsafe fn aic32x4_register_clk(dev: *mut device, desc: *mut aic32x4_clkdesc) -> *mut clk {
    let mut init: clk_init_data;
    let priv_: *mut clk_aic32x4;
    let devname: *const c_char = dev_name(dev);

    init = clk_init_data {
        ops: (*desc).ops,
        name: (*desc).name,
        parent_names: (*desc).parent_names,
        num_parents: (*desc).num_parents,
        flags: 0,
    };

    priv_ = devm_kzalloc(
        dev,
        core::mem::size_of::<clk_aic32x4>(),
        GFP_KERNEL,
    ) as *mut clk_aic32x4;
    if priv_.is_null() {
        return ERR_PTR(-(ENOMEM as c_long));
    }

    (*priv_).dev = dev;
    (*priv_).hw.init = &init;
    (*priv_).regmap = dev_get_regmap(dev, ptr::null());
    (*priv_).reg = (*desc).reg;

    clk_hw_register_clkdev(&mut (*priv_).hw, (*desc).name, devname);
    devm_clk_register(dev, &mut (*priv_).hw)
}

#[no_mangle]
pub unsafe extern "C" fn aic32x4_register_clocks(
    dev: *mut device,
    mclk_name: *const c_char,
) -> c_int {
    let mut i: c_int;

    /*
     * These lines are here to preserve the current functionality of
     * the driver with regard to the DT.  These should eventually be set
     * by DT nodes so that the connections can be set up in configuration
     * rather than code.
     */
    AIC32X4_PLL_PARENT_NAMES = [
        mclk_name,
        b"bclk\0".as_ptr() as *const c_char,
        b"gpio\0".as_ptr() as *const c_char,
        b"din\0".as_ptr() as *const c_char,
    ];
    aic32x4_clkdesc_array[0].parent_names = AIC32X4_PLL_PARENT_NAMES.as_ptr();
    AIC32X4_CODEC_CLKIN_PARENT_NAMES = [
        mclk_name,
        b"bclk\0".as_ptr() as *const c_char,
        b"gpio\0".as_ptr() as *const c_char,
        b"pll\0".as_ptr() as *const c_char,
    ];
    aic32x4_clkdesc_array[1].parent_names = AIC32X4_CODEC_CLKIN_PARENT_NAMES.as_ptr();

    i = 0;
    while (i as usize) < aic32x4_clkdesc_array.len() {
        aic32x4_register_clk(dev, &mut aic32x4_clkdesc_array[i as usize]);
        i += 1;
    }

    0
}

// EXPORT_SYMBOL_GPL(aic32x4_register_clocks);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
