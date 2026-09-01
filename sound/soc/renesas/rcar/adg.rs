// SPDX-License-Identifier: GPL-2.0
//
// Helper routines for R-Car sound ADG.
//
//  Copyright (C) 2013  Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//
// C dependencies translated as external Rust declarations:
// <linux/clk-provider.h>, <linux/clkdev.h>, "rsnd.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type u32 = u32;
type uint32_t = u32;

const CLKA: usize = 0;
const CLKB: usize = 1;
const CLKC: usize = 2;
const CLKI: usize = 3;
const CLKINMAX: usize = 4;

const CLKOUT: usize = 0;
const CLKOUT1: usize = 1;
const CLKOUT2: usize = 2;
const CLKOUT3: usize = 3;
const CLKOUTMAX: usize = 4;

/* Maximum SSI count for per-SSI clocks */
const ADG_SSI_MAX: usize = 10;

const BRGCKR_31: u32 = 1u32 << 31;
const fn BRRx_MASK(x: u32) -> u32 {
    0x3ff & x
}

const ADG_HZ_441: usize = 0;
const ADG_HZ_48: usize = 1;
const ADG_HZ_SIZE: usize = 2;

const EIO: c_int = 5;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const RSND_MOD_SRC: c_int = 0;
const RSND_MOD_SSI: c_int = 0;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct property {
    pub length: c_int,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub rate: c_uint,
}

#[repr(C)]
pub struct rsnd_dai_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rsnd_mod_ops {
    pub name: *const c_char,
}

#[repr(C)]
pub struct rsnd_mod {
    pub clk: *mut clk,
    pub rstc: *mut reset_control,
}

#[repr(C)]
pub struct clk_onecell_data {
    pub clks: *mut *mut clk,
    pub clk_num: c_uint,
}

#[repr(C)]
pub struct rsnd_priv {
    pub adg: *mut rsnd_adg,
}

#[repr(C)]
pub struct rsnd_adg {
    pub adg: *mut clk,
    pub clkin: [*mut clk; CLKINMAX],
    pub clkout: [*mut clk; CLKOUTMAX],
    /* RZ/G3E: per-SSI ADG clocks (adg-ssi-0 through adg-ssi-9) */
    pub clk_adg_ssi: [*mut clk; ADG_SSI_MAX],
    pub clk_ssif_supply: *mut clk,
    pub null_clk: *mut clk,
    pub onecell: clk_onecell_data,
    pub mod_: rsnd_mod,
    pub clkin_rate: [c_int; CLKINMAX],
    pub ssi_clk_prepared: bool,
    pub clk_enabled: bool,
    pub clkin_size: c_int,
    pub clkout_size: c_int,
    pub ckr: u32,
    pub brga: u32,
    pub brgb: u32,

    pub brg_rate: [c_int; ADG_HZ_SIZE], /* BRGA / BRGB */
}

static mut adg_ops: rsnd_mod_ops = rsnd_mod_ops {
    name: b"adg\0".as_ptr() as *const c_char,
};

static clkin_name_gen4: [*const c_char; 1] = [
    b"clkin\0".as_ptr() as *const c_char,
];

static clkin_name_gen2: [*const c_char; CLKINMAX] = [
    b"clk_a\0".as_ptr() as *const c_char,
    b"clk_b\0".as_ptr() as *const c_char,
    b"clk_c\0".as_ptr() as *const c_char,
    b"clk_i\0".as_ptr() as *const c_char,
];

static clkin_name_rzg3e: [*const c_char; CLKINMAX] = [
    b"audio-clka\0".as_ptr() as *const c_char,
    b"audio-clkb\0".as_ptr() as *const c_char,
    b"audio-clkc\0".as_ptr() as *const c_char,
    b"audio-clki\0".as_ptr() as *const c_char,
];

static clkout_name_gen2: [*const c_char; CLKOUTMAX] = [
    b"audio_clkout\0".as_ptr() as *const c_char,
    b"audio_clkout1\0".as_ptr() as *const c_char,
    b"audio_clkout2\0".as_ptr() as *const c_char,
    b"audio_clkout3\0".as_ptr() as *const c_char,
];

extern "C" {
    fn rsnd_io_to_mod_ssi(io: *mut rsnd_dai_stream) -> *mut rsnd_mod;
    fn rsnd_mod_id(mod_: *mut rsnd_mod) -> c_int;
    fn rsnd_ssi_is_pin_sharing(io: *mut rsnd_dai_stream) -> bool;
    fn rsnd_priv_to_dev(priv_: *mut rsnd_priv) -> *mut device;
    fn rsnd_io_to_runtime(io: *mut rsnd_dai_stream) -> *mut snd_pcm_runtime;
    fn rsnd_mod_to_priv(mod_: *mut rsnd_mod) -> *mut rsnd_priv;
    fn rsnd_mod_get(adg: *mut rsnd_adg) -> *mut rsnd_mod;
    fn rsnd_src_get_in_rate(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream) -> c_uint;
    fn rsnd_src_get_out_rate(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream) -> c_uint;
    fn rsnd_mod_bset(mod_: *mut rsnd_mod, reg: c_uint, mask: u32, data: u32);
    fn rsnd_mod_write(mod_: *mut rsnd_mod, reg: c_uint, data: u32);
    fn rsnd_mod_make_sure(mod_: *mut rsnd_mod, id: c_int);
    fn clk_disable(clk: *mut clk);
    fn clk_enable(clk: *mut clk) -> c_int;
    fn clk_prepare(clk: *mut clk) -> c_int;
    fn clk_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_register_fixed_rate(
        dev: *mut device,
        name: *const c_char,
        parent: *const c_char,
        flags: c_ulong,
        fixed_rate: c_ulong,
    ) -> *mut clk;
    fn clk_unregister_fixed_rate(clk: *mut clk);
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn __clk_get_name(clk: *mut clk) -> *const c_char;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool;
    fn ERR_CAST(ptr: *const c_void) -> *mut clk;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn of_find_property(np: *mut device_node, name: *const c_char, lenp: *mut c_int) -> *mut property;
    fn of_property_read_u32_array(
        np: *mut device_node,
        propname: *const c_char,
        out_values: *mut u32,
        sz: c_int,
    ) -> c_int;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut u32) -> c_int;
    fn of_clk_add_provider(np: *mut device_node, clk_src_get: *const c_void, data: *mut c_void) -> c_int;
    fn of_clk_del_provider(np: *mut device_node);
    static of_clk_src_simple_get: c_void;
    static of_clk_src_onecell_get: c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_reset_control_get_optional_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn rsnd_mod_init(
        priv_: *mut rsnd_priv,
        mod_: *mut rsnd_mod,
        ops: *mut rsnd_mod_ops,
        clk: *mut clk,
        rstc: *mut reset_control,
        id: c_int,
        irq: c_int,
    ) -> c_int;
    fn rsnd_adg_clk_enable(priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_adg_clk_disable(priv_: *mut rsnd_priv) -> c_int;
    fn rsnd_suspend_clk_reset(clk: *mut clk, rstc: *mut reset_control);
    fn rsnd_resume_clk_reset(clk: *mut clk, rstc: *mut reset_control);
    fn rsnd_is_gen4(priv_: *mut rsnd_priv) -> bool;
    fn rsnd_is_rzg3e(priv_: *mut rsnd_priv) -> bool;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn seq_puts(m: *mut seq_file, s: *const c_char);
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
}

const fn CMDOUT_TIMSEL() -> c_uint { 0 }
const fn SRCIN_TIMSEL(id: c_int) -> c_uint { id as c_uint }
const fn SRCOUT_TIMSEL(id: c_int) -> c_uint { id as c_uint }
const fn DIV_EN() -> c_uint { 0 }
const fn AUDIO_CLK_SEL(id: c_int) -> c_uint { id as c_uint }
const fn BRGCKR() -> c_uint { 0 }
const fn BRRA() -> c_uint { 0 }
const fn BRRB() -> c_uint { 0 }

#[inline]
unsafe fn rsnd_priv_to_adg(priv_: *mut rsnd_priv) -> *mut rsnd_adg {
    (*priv_).adg
}

unsafe fn rsnd_adg_calculate_brgx(div: c_ulong) -> u32 {
    let mut i: c_int;

    if div == 0 {
        return 0;
    }

    i = 3;
    while i >= 0 {
        let ratio: c_int = 2 << (i * 2);
        if 0 == (div % ratio as c_ulong) {
            return ((i << 8) as c_ulong | ((div / ratio as c_ulong) - 1)) as u32;
        }
        i -= 1;
    }

    !0
}

unsafe fn rsnd_adg_ssi_ws_timing_gen2(io: *mut rsnd_dai_stream) -> u32 {
    let ssi_mod = rsnd_io_to_mod_ssi(io);
    let id = rsnd_mod_id(ssi_mod);
    let mut ws = id;

    if rsnd_ssi_is_pin_sharing(io) {
        match id {
            1 | 2 | 9 => ws = 0,
            4 => ws = 3,
            8 => ws = 7,
            _ => {}
        }
    } else {
        /*
         * SSI8 is not connected to ADG.
         * Thus SSI9 is using ws = 8
         */
        if id == 9 {
            ws = 8;
        }
    }

    ((0x6 + ws) << 8) as u32
}

unsafe fn __rsnd_adg_get_timesel_ratio(
    priv_: *mut rsnd_priv,
    _io: *mut rsnd_dai_stream,
    target_rate: c_uint,
    target_val: *mut c_uint,
    target_en: *mut c_uint,
) {
    let adg = rsnd_priv_to_adg(priv_);
    let dev = rsnd_priv_to_dev(priv_);
    let mut val: c_uint;
    let mut en: c_uint;
    let mut min: c_uint;
    let mut diff: c_uint;
    let sel_rate: [c_int; 5] = [
        (*adg).clkin_rate[CLKA],          /* 0000: CLKA */
        (*adg).clkin_rate[CLKB],          /* 0001: CLKB */
        (*adg).clkin_rate[CLKC],          /* 0010: CLKC */
        (*adg).brg_rate[ADG_HZ_441],      /* 0011: BRGA */
        (*adg).brg_rate[ADG_HZ_48],       /* 0100: BRGB */
    ];

    min = !0;
    val = 0;
    en = 0;
    for sel in 0..sel_rate.len() {
        let mut idx: c_int = 0;
        let mut step: c_int = 2;
        let mut div: c_int = 2;

        if sel_rate[sel] == 0 {
            continue;
        }

        while div <= 98304 {
            diff = (target_rate as c_int - sel_rate[sel] / div).abs() as c_uint;
            if min > diff {
                val = ((sel as c_uint) << 8) | idx as c_uint;
                min = diff;
                en = 1 << (sel + 1); /* fixme */
            }

            /*
             * step of 0_0000 / 0_0001 / 0_1101
             * are out of order
             */
            if (idx > 2) && ((idx % 2) != 0) {
                step *= 2;
            }
            if idx == 0x1c {
                div += step;
                step *= 2;
            }
            idx += 1;
            div += step;
        }
    }

    if min == !0 {
        dev_err(dev, b"no Input clock\n\0".as_ptr() as *const c_char);
        return;
    }

    *target_val = val;
    if !target_en.is_null() {
        *target_en = en;
    }
}

unsafe fn rsnd_adg_get_timesel_ratio(
    priv_: *mut rsnd_priv,
    io: *mut rsnd_dai_stream,
    in_rate: c_uint,
    out_rate: c_uint,
    in_: *mut u32,
    out: *mut u32,
    en: *mut u32,
) {
    let runtime = rsnd_io_to_runtime(io);
    let mut target_rate: c_uint;
    let mut target_val: *mut u32;
    let mut _in: u32;
    let mut _out: u32;
    let mut _en: u32;

    /* default = SSI WS */
    _out = rsnd_adg_ssi_ws_timing_gen2(io);
    _in = _out;

    target_rate = 0;
    target_val = ptr::null_mut();
    _en = 0;
    if (*runtime).rate != in_rate {
        target_rate = out_rate;
        target_val = &mut _out;
    } else if (*runtime).rate != out_rate {
        target_rate = in_rate;
        target_val = &mut _in;
    }

    if target_rate != 0 {
        __rsnd_adg_get_timesel_ratio(priv_, io, target_rate, target_val, &mut _en);
    }

    if !in_.is_null() {
        *in_ = _in;
    }
    if !out.is_null() {
        *out = _out;
    }
    if !en.is_null() {
        *en = _en;
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_adg_set_cmd_timsel_gen2(
    cmd_mod: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
) -> c_int {
    let priv_ = rsnd_mod_to_priv(cmd_mod);
    let adg = rsnd_priv_to_adg(priv_);
    let adg_mod = rsnd_mod_get(adg);
    let id = rsnd_mod_id(cmd_mod);
    let shift = if (id % 2) != 0 { 16 } else { 0 };
    let mut val: u32 = 0;

    rsnd_adg_get_timesel_ratio(
        priv_,
        io,
        rsnd_src_get_in_rate(priv_, io),
        rsnd_src_get_out_rate(priv_, io),
        ptr::null_mut(),
        &mut val,
        ptr::null_mut(),
    );

    val <<= shift;
    let mask: u32 = 0x0f1f << shift;

    rsnd_mod_bset(adg_mod, CMDOUT_TIMSEL(), mask, val);

    0
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_adg_set_src_timesel_gen2(
    src_mod: *mut rsnd_mod,
    io: *mut rsnd_dai_stream,
    in_rate: c_uint,
    out_rate: c_uint,
) -> c_int {
    let priv_ = rsnd_mod_to_priv(src_mod);
    let adg = rsnd_priv_to_adg(priv_);
    let adg_mod = rsnd_mod_get(adg);
    let mut in_: u32 = 0;
    let mut out: u32 = 0;
    let mut en: u32 = 0;
    let id = rsnd_mod_id(src_mod);
    let shift = if (id % 2) != 0 { 16 } else { 0 };

    rsnd_mod_make_sure(src_mod, RSND_MOD_SRC);

    rsnd_adg_get_timesel_ratio(priv_, io, in_rate, out_rate, &mut in_, &mut out, &mut en);

    in_ <<= shift;
    out <<= shift;
    let mask: u32 = 0x0f1f << shift;

    rsnd_mod_bset(adg_mod, SRCIN_TIMSEL(id / 2), mask, in_);
    rsnd_mod_bset(adg_mod, SRCOUT_TIMSEL(id / 2), mask, out);

    if en != 0 {
        rsnd_mod_bset(adg_mod, DIV_EN(), en, en);
    }

    0
}

unsafe fn rsnd_adg_set_ssi_clk(ssi_mod: *mut rsnd_mod, mut val: u32) {
    let priv_ = rsnd_mod_to_priv(ssi_mod);
    let adg = rsnd_priv_to_adg(priv_);
    let adg_mod = rsnd_mod_get(adg);
    let dev = rsnd_priv_to_dev(priv_);
    let id = rsnd_mod_id(ssi_mod);
    let shift = (id % 4) * 8;
    let mask: u32 = 0xff << shift;

    rsnd_mod_make_sure(ssi_mod, RSND_MOD_SSI);

    val <<= shift;

    /*
     * SSI 8 is not connected to ADG.
     * it works with SSI 7
     */
    if id == 8 {
        return;
    }

    rsnd_mod_bset(adg_mod, AUDIO_CLK_SEL(id / 4), mask, val);

    dev_dbg(dev, b"AUDIO_CLK_SEL is 0x%x\n\0".as_ptr() as *const c_char, val);
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_adg_clk_query(priv_: *mut rsnd_priv, rate: c_uint) -> c_int {
    let adg = rsnd_priv_to_adg(priv_);
    let sel_table: [c_int; CLKINMAX] = [0x1, 0x2, 0x3, 0x0];

    /*
     * find suitable clock from
     * AUDIO_CLKA/AUDIO_CLKB/AUDIO_CLKC/AUDIO_CLKI.
     */
    let mut i = 0;
    while i < (*adg).clkin_size as usize {
        let clk = (*adg).clkin[i];
        if clk.is_null() {
            break;
        }
        if rate as c_int == (*adg).clkin_rate[i] {
            return sel_table[i];
        }
        i += 1;
    }

    /*
     * find divided clock from BRGA/BRGB
     */
    if rate as c_int == (*adg).brg_rate[ADG_HZ_441] {
        return 0x10;
    }

    if rate as c_int == (*adg).brg_rate[ADG_HZ_48] {
        return 0x20;
    }

    -EIO
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_adg_ssi_clk_stop(ssi_mod: *mut rsnd_mod) -> c_int {
    let priv_ = rsnd_mod_to_priv(ssi_mod);
    let adg = rsnd_priv_to_adg(priv_);
    let id = rsnd_mod_id(ssi_mod) as usize;

    rsnd_adg_set_ssi_clk(ssi_mod, 0);

    /* RZ/G3E: only disable here, unprepare is done in hw_free */
    clk_disable((*adg).clk_adg_ssi[id]);
    clk_disable((*adg).clk_ssif_supply);

    0
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_adg_ssi_clk_try_start(
    ssi_mod: *mut rsnd_mod,
    rate: c_uint,
) -> c_int {
    let priv_ = rsnd_mod_to_priv(ssi_mod);
    let adg = rsnd_priv_to_adg(priv_);
    let dev = rsnd_priv_to_dev(priv_);
    let adg_mod = rsnd_mod_get(adg);
    let id = rsnd_mod_id(ssi_mod);
    let mut ret: c_int;
    let data: c_int;
    let mut ckr: u32;

    data = rsnd_adg_clk_query(priv_, rate);
    if data < 0 {
        return data;
    }

    rsnd_adg_set_ssi_clk(ssi_mod, data as u32);

    ckr = (*adg).ckr & !BRGCKR_31;
    if 0 == (rate % 8000) {
        ckr |= BRGCKR_31; /* use BRGB output = 48kHz */
    }
    if ckr != (*adg).ckr {
        rsnd_mod_bset(adg_mod, BRGCKR(), 0x80770000, (*adg).ckr);
        (*adg).ckr = ckr;
    }

    dev_dbg(
        dev,
        b"CLKOUT is based on BRG%c (= %dHz)\n\0".as_ptr() as *const c_char,
        if ckr != 0 { b'B' as c_int } else { b'A' as c_int },
        if ckr != 0 {
            (*adg).brg_rate[ADG_HZ_48]
        } else {
            (*adg).brg_rate[ADG_HZ_441]
        },
    );

    /*
     * RZ/G3E: enable per-SSI and supply clocks
     */
    ret = clk_enable((*adg).clk_adg_ssi[id as usize]);
    if ret != 0 {
        dev_err(dev, b"Cannot enable adg-ssi-%d ADG clock\n\0".as_ptr() as *const c_char, id);
        return ret;
    }

    ret = clk_enable((*adg).clk_ssif_supply);
    if ret != 0 {
        dev_err(dev, b"Cannot enable SSIF supply clock\n\0".as_ptr() as *const c_char);
        clk_disable((*adg).clk_adg_ssi[id as usize]);
        return ret;
    }

    0
}

unsafe fn rsnd_adg_ssi_clk_prepare(adg: *mut rsnd_adg) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;

    if (*adg).ssi_clk_prepared {
        return 0;
    }

    i = 0;
    while i < ADG_SSI_MAX as c_int {
        ret = clk_prepare((*adg).clk_adg_ssi[i as usize]);
        if ret != 0 {
            while {
                i -= 1;
                i >= 0
            } {
                clk_unprepare((*adg).clk_adg_ssi[i as usize]);
            }
            return ret;
        }
        i += 1;
    }
    ret = clk_prepare((*adg).clk_ssif_supply);
    if ret != 0 {
        while {
            i -= 1;
            i >= 0
        } {
            clk_unprepare((*adg).clk_adg_ssi[i as usize]);
        }
        return ret;
    }

    (*adg).ssi_clk_prepared = true;
    0
}

unsafe fn rsnd_adg_ssi_clk_unprepare(adg: *mut rsnd_adg) {
    if !(*adg).ssi_clk_prepared {
        return;
    }
    (*adg).ssi_clk_prepared = false;

    clk_unprepare((*adg).clk_ssif_supply);
    for i in 0..ADG_SSI_MAX {
        clk_unprepare((*adg).clk_adg_ssi[i]);
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_adg_clk_control(priv_: *mut rsnd_priv, enable: c_int) -> c_int {
    let adg = rsnd_priv_to_adg(priv_);
    let adg_mod = rsnd_mod_get(adg);
    let mut ret: c_int = 0;

    /*
     * rsnd_adg_clk_enable() and rsnd_adg_clk_disable() can be called
     * redundantly, for example when system suspend follows a resume
     * whose enable failed. Make this function idempotent so that the
     * "adg" clock, which has no clkin_rate[] style guard, is never
     * disabled twice.
     */
    if enable != 0 {
        if (*adg).clk_enabled {
            return 0;
        }
    } else {
        if !(*adg).clk_enabled {
            return 0;
        }
        (*adg).clk_enabled = false;
    }

    if enable != 0 {
        ret = clk_prepare_enable((*adg).adg);
        if ret < 0 {
            return ret;
        }

        rsnd_mod_bset(adg_mod, BRGCKR(), 0x80770000, (*adg).ckr);
        rsnd_mod_write(adg_mod, BRRA(), (*adg).brga);
        rsnd_mod_write(adg_mod, BRRB(), (*adg).brgb);
    }

    let mut i = 0usize;
    while i < (*adg).clkin_size as usize {
        let clk = (*adg).clkin[i];
        if clk.is_null() {
            break;
        }
        if enable != 0 {
            ret = clk_prepare_enable(clk);

            /*
             * We shouldn't use clk_get_rate() under
             * atomic context. Let's keep it when
             * rsnd_adg_clk_enable() was called
             */
            if ret < 0 {
                break;
            }

            (*adg).clkin_rate[i] = clk_get_rate(clk) as c_int;
        } else {
            if (*adg).clkin_rate[i] != 0 {
                clk_disable_unprepare(clk);
            }

            (*adg).clkin_rate[i] = 0;
        }
        i += 1;
    }

    /*
     * rsnd_adg_clk_enable() might return error (_disable() will not).
     * We need to rollback in such case
     */
    /*
     * RZ/G3E per-SSI ADG and SSIF supply clocks.
     *
     * Follow the same style as for_each_rsnd_clkin() above: on enable,
     * try to prepare every clock and accumulate the error. On disable,
     * unprepare every clock. Absent optional clocks are NULL, for
     * which clk_prepare() and clk_unprepare() are no-ops.
     */
    if enable != 0 {
        let sub_ret = rsnd_adg_ssi_clk_prepare(adg);

        /* Preserve the first error from the clkin loop above. */
        if sub_ret != 0 && ret == 0 {
            ret = sub_ret;
        }
    } else {
        rsnd_adg_ssi_clk_unprepare(adg);
    }

    /*
     * rsnd_adg_clk_enable() might return error (_disable() will not).
     * We need to rollback in such case
     */
    if ret < 0 {
        /*
         * Mark as enabled so that the rollback below is not
         * short-circuited by the idempotency guard. It clears
         * the flag again on its way through.
         */
        (*adg).clk_enabled = true;
        rsnd_adg_clk_disable(priv_);
        return ret;
    }

    /* disable adg */
    if enable == 0 {
        clk_disable_unprepare((*adg).adg);
    } else {
        (*adg).clk_enabled = true;
    }

    ret
}

unsafe fn rsnd_adg_create_null_clk(
    priv_: *mut rsnd_priv,
    name: *const c_char,
    parent: *const c_char,
) -> *mut clk {
    let dev = rsnd_priv_to_dev(priv_);
    let clk = clk_register_fixed_rate(dev, name, parent, 0, 0);
    if IS_ERR(clk as *const c_void) {
        dev_err(dev, b"create null clk error\n\0".as_ptr() as *const c_char);
        return ERR_CAST(clk as *const c_void);
    }

    clk
}

unsafe fn rsnd_adg_null_clk_get(priv_: *mut rsnd_priv) -> *mut clk {
    let adg = (*priv_).adg;

    if (*adg).null_clk.is_null() {
        static name: &[u8] = b"rsnd_adg_null\0";

        (*adg).null_clk = rsnd_adg_create_null_clk(priv_, name.as_ptr() as *const c_char, ptr::null());
    }

    (*adg).null_clk
}

unsafe fn rsnd_adg_null_clk_clean(priv_: *mut rsnd_priv) {
    let adg = (*priv_).adg;

    if !(*adg).null_clk.is_null() {
        clk_unregister_fixed_rate((*adg).null_clk);
    }
}

unsafe fn rsnd_adg_get_clkin(priv_: *mut rsnd_priv) -> c_int {
    let adg = (*priv_).adg;
    let dev = rsnd_priv_to_dev(priv_);
    let mut clk: *mut clk;
    let mut clkin_name: *const *const c_char;
    let mut clkin_size: c_int;

    clkin_name = clkin_name_gen2.as_ptr();
    clkin_size = clkin_name_gen2.len() as c_int;
    if rsnd_is_gen4(priv_) {
        clkin_name = clkin_name_gen4.as_ptr();
        clkin_size = clkin_name_gen4.len() as c_int;
    } else if rsnd_is_rzg3e(priv_) {
        clkin_name = clkin_name_rzg3e.as_ptr();
        clkin_size = clkin_name_rzg3e.len() as c_int;
    }

    /*
     * get adg
     * No "adg" is not error
     */
    clk = devm_clk_get(dev, b"adg\0".as_ptr() as *const c_char);
    if IS_ERR(clk as *const c_void) {
        clk = rsnd_adg_null_clk_get(priv_);
    }
    (*adg).adg = clk;

    /* get clkin */
    for i in 0..clkin_size as usize {
        clk = devm_clk_get(dev, *clkin_name.add(i));

        if IS_ERR(clk as *const c_void) {
            clk = rsnd_adg_null_clk_get(priv_);
        }
        if IS_ERR(clk as *const c_void) {
            dev_err(dev, b"adg clock IN get failed\n\0".as_ptr() as *const c_char);

            rsnd_adg_null_clk_clean(priv_);

            return -EIO;
        }

        (*adg).clkin[i] = clk;
    }

    (*adg).clkin_size = clkin_size;

    0
}

unsafe fn rsnd_adg_unregister_clkout(priv_: *mut rsnd_priv) {
    let adg = (*priv_).adg;

    let mut i = 0usize;
    while i < (*adg).clkout_size as usize {
        let clk = (*adg).clkout[i];
        if clk.is_null() {
            break;
        }
        clk_unregister_fixed_rate(clk);
        i += 1;
    }
}

unsafe fn rsnd_adg_get_clkout(priv_: *mut rsnd_priv) -> c_int {
    let adg = (*priv_).adg;
    let mut clk: *mut clk;
    let dev = rsnd_priv_to_dev(priv_);
    let np = (*dev).of_node;
    let mut prop: *mut property;
    let mut ckr: u32;
    let mut brgx: u32;
    let mut brga: u32;
    let mut brgb: u32;
    let mut req_rate: [u32; ADG_HZ_SIZE] = [0; ADG_HZ_SIZE];
    let mut count: uint32_t = 0;
    let mut req_Hz: [c_ulong; ADG_HZ_SIZE] = [0; ADG_HZ_SIZE];
    let mut clkout_size: c_int;
    let mut req_size: c_int;
    let mut approximate: c_int = 0;
    let mut parent_clk_name: *const c_char = ptr::null();
    let mut clkout_name: *const *const c_char;
    let brg_table: [c_int; CLKINMAX] = [0x0, 0x1, 0x4, 0x2];

    ckr = 0;
    brga = 0xff; /* default */
    brgb = 0xff; /* default */

    /*
     * ADG supports BRRA/BRRB output only
     * this means all clkout0/1/2/3 will be same rate
     */
    prop = of_find_property(np, b"clock-frequency\0".as_ptr() as *const c_char, ptr::null_mut());
    if prop.is_null() {
        if 0 == (req_rate[0] % 8000) {
            ckr |= BRGCKR_31; /* use BRGB output = 48kHz */
        }

        (*adg).ckr = ckr;
        (*adg).brga = brga;
        (*adg).brgb = brgb;

        return 0;
    }

    req_size = (*prop).length / core::mem::size_of::<u32>() as c_int;
    if req_size > ADG_HZ_SIZE as c_int {
        dev_err(dev, b"too many clock-frequency\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    of_property_read_u32_array(np, b"clock-frequency\0".as_ptr() as *const c_char, req_rate.as_mut_ptr(), req_size);
    req_Hz[ADG_HZ_48] = 0;
    req_Hz[ADG_HZ_441] = 0;
    for i in 0..req_size as usize {
        if 0 == (req_rate[i] % 44100) {
            req_Hz[ADG_HZ_441] = req_rate[i] as c_ulong;
        }
        if 0 == (req_rate[i] % 48000) {
            req_Hz[ADG_HZ_48] = req_rate[i] as c_ulong;
        }
    }

    /*
     * This driver is assuming that AUDIO_CLKA/AUDIO_CLKB/AUDIO_CLKC
     * have 44.1kHz or 48kHz base clocks for now.
     *
     * SSI itself can divide parent clock by 1/1 - 1/16
     * see
     *	rsnd_adg_ssi_clk_try_start()
     *	rsnd_ssi_master_clk_start()
     */

    /*
     * [APPROXIMATE]
     *
     * clk_i (internal clock) can't create accurate rate, it will be approximate rate.
     *
     * <Note>
     *
     * clk_i needs x2 of required maximum rate.
     * see
     *	- Minimum division of BRRA/BRRB
     *	- rsnd_ssi_clk_query()
     *
     * Sample Settings for TDM 8ch, 32bit width
     *
     *	8(ch) x 32(bit) x 44100(Hz) x 2<Note> = 22579200
     *	8(ch) x 32(bit) x 48000(Hz) x 2<Note> = 24576000
     *
     *	clock-frequency = <22579200 24576000>;
     */
    let mut i = 0usize;
    while i < (*adg).clkin_size as usize {
        clk = (*adg).clkin[i];
        if clk.is_null() {
            break;
        }
        let mut rate: u32;
        let div: u32;

        rate = clk_get_rate(clk) as u32;

        if 0 == rate {
            /* not used */
            i += 1;
            continue;
        }

        /* BRGA */

        if i == CLKI {
            /* see [APPROXIMATE] */
            rate = ((clk_get_rate(clk) / req_Hz[ADG_HZ_441]) * req_Hz[ADG_HZ_441]) as u32;
        }
        if (*adg).brg_rate[ADG_HZ_441] == 0 && req_Hz[ADG_HZ_441] != 0 && (0 == rate % 44100) {
            div = rate / req_Hz[ADG_HZ_441] as u32;
            brgx = rsnd_adg_calculate_brgx(div as c_ulong);
            if BRRx_MASK(brgx) == brgx {
                brga = brgx;
                (*adg).brg_rate[ADG_HZ_441] = (rate / div) as c_int;
                ckr |= (brg_table[i] as u32) << 20;
                if req_Hz[ADG_HZ_441] != 0 {
                    parent_clk_name = __clk_get_name(clk);
                }
                if i == CLKI {
                    approximate = 1;
                }
            }
        }

        /* BRGB */

        if i == CLKI {
            /* see [APPROXIMATE] */
            rate = ((clk_get_rate(clk) / req_Hz[ADG_HZ_48]) * req_Hz[ADG_HZ_48]) as u32;
        }
        if (*adg).brg_rate[ADG_HZ_48] == 0 && req_Hz[ADG_HZ_48] != 0 && (0 == rate % 48000) {
            div = rate / req_Hz[ADG_HZ_48] as u32;
            brgx = rsnd_adg_calculate_brgx(div as c_ulong);
            if BRRx_MASK(brgx) == brgx {
                brgb = brgx;
                (*adg).brg_rate[ADG_HZ_48] = (rate / div) as c_int;
                ckr |= (brg_table[i] as u32) << 16;
                if req_Hz[ADG_HZ_48] != 0 {
                    parent_clk_name = __clk_get_name(clk);
                }
                if i == CLKI {
                    approximate = 1;
                }
            }
        }
        i += 1;
    }

    if !(((*adg).brg_rate[ADG_HZ_48] != 0 && req_Hz[ADG_HZ_48] != 0)
        || ((*adg).brg_rate[ADG_HZ_441] != 0 && req_Hz[ADG_HZ_441] != 0))
    {
        if 0 == (req_rate[0] % 8000) {
            ckr |= BRGCKR_31; /* use BRGB output = 48kHz */
        }

        (*adg).ckr = ckr;
        (*adg).brga = brga;
        (*adg).brgb = brgb;

        return 0;
    }

    if approximate != 0 {
        dev_info(dev, b"It uses CLK_I as approximate rate\0".as_ptr() as *const c_char);
    }

    clkout_name = clkout_name_gen2.as_ptr();
    clkout_size = clkout_name_gen2.len() as c_int;
    if rsnd_is_gen4(priv_) {
        clkout_size = 1; /* reuse clkout_name_gen2[] */
    }

    /*
     * ADG supports BRRA/BRRB output only.
     * this means all clkout0/1/2/3 will be * same rate
     */

    of_property_read_u32(np, b"#clock-cells\0".as_ptr() as *const c_char, &mut count);
    /*
     * for clkout
     */
    if count == 0 {
        clk = clk_register_fixed_rate(
            dev,
            *clkout_name.add(CLKOUT),
            parent_clk_name,
            0,
            req_rate[0] as c_ulong,
        );
        if IS_ERR_OR_NULL(clk as *const c_void) {
            dev_err(dev, b"adg clock OUT get failed\n\0".as_ptr() as *const c_char);

            rsnd_adg_unregister_clkout(priv_);

            return -EIO;
        }

        (*adg).clkout[CLKOUT] = clk;
        (*adg).clkout_size = 1;
        of_clk_add_provider(np, &of_clk_src_simple_get as *const c_void, clk as *mut c_void);
    }
    /*
     * for clkout0/1/2/3
     */
    else {
        for i in 0..clkout_size as usize {
            clk = clk_register_fixed_rate(
                dev,
                *clkout_name.add(i),
                parent_clk_name,
                0,
                req_rate[0] as c_ulong,
            );
            if IS_ERR_OR_NULL(clk as *const c_void) {
                dev_err(dev, b"adg clock OUT get failed\n\0".as_ptr() as *const c_char);

                rsnd_adg_unregister_clkout(priv_);

                return -EIO;
            }

            (*adg).clkout[i] = clk;
        }
        (*adg).onecell.clks = (*adg).clkout.as_mut_ptr();
        (*adg).onecell.clk_num = clkout_size as c_uint;
        (*adg).clkout_size = clkout_size;
        of_clk_add_provider(
            np,
            &of_clk_src_onecell_get as *const c_void,
            &mut (*adg).onecell as *mut clk_onecell_data as *mut c_void,
        );
    }

    if 0 == (req_rate[0] % 8000) {
        ckr |= BRGCKR_31; /* use BRGB output = 48kHz */
    }

    (*adg).ckr = ckr;
    (*adg).brga = brga;
    (*adg).brgb = brgb;

    0
}

/* Original C condition: #if defined(DEBUG) || defined(CONFIG_DEBUG_FS) */
unsafe fn dbg_msg(dev: *mut device, m: *mut seq_file, fmt: *const c_char) {
    if !m.is_null() {
        seq_puts(m, fmt);
    } else {
        dev_dbg(dev, b"%s\0".as_ptr() as *const c_char, fmt);
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_adg_clk_dbg_info(priv_: *mut rsnd_priv, m: *mut seq_file) {
    let adg = rsnd_priv_to_adg(priv_);
    let dev = rsnd_priv_to_dev(priv_);

    let mut i = 0usize;
    while i < (*adg).clkin_size as usize {
        let clk = (*adg).clkin[i];
        if clk.is_null() {
            break;
        }
        dev_dbg(
            dev,
            b"%-18s : %pa : %ld\n\0".as_ptr() as *const c_char,
            __clk_get_name(clk),
            clk,
            clk_get_rate(clk),
        );
        i += 1;
    }

    dev_dbg(
        dev,
        b"BRGCKR = 0x%08x, BRRA/BRRB = 0x%x/0x%x\n\0".as_ptr() as *const c_char,
        (*adg).ckr,
        (*adg).brga,
        (*adg).brgb,
    );
    dev_dbg(
        dev,
        b"BRGA (for 44100 base) = %d\n\0".as_ptr() as *const c_char,
        (*adg).brg_rate[ADG_HZ_441],
    );
    dev_dbg(
        dev,
        b"BRGB (for 48000 base) = %d\n\0".as_ptr() as *const c_char,
        (*adg).brg_rate[ADG_HZ_48],
    );

    /*
     * Actual CLKOUT will be exchanged in rsnd_adg_ssi_clk_try_start()
     * by BRGCKR::BRGCKR_31
     */
    i = 0;
    while i < (*adg).clkout_size as usize {
        let clk = (*adg).clkout[i];
        if clk.is_null() {
            break;
        }
        dev_dbg(
            dev,
            b"%-18s : %pa : %ld\n\0".as_ptr() as *const c_char,
            __clk_get_name(clk),
            clk,
            clk_get_rate(clk),
        );
        i += 1;
    }

    let _ = m;
}

unsafe fn rsnd_adg_get_ssi_clks(priv_: *mut rsnd_priv) -> c_int {
    let adg = rsnd_priv_to_adg(priv_);
    let dev = rsnd_priv_to_dev(priv_);
    let mut name: [c_char; 16] = [0; 16];

    /* SSIF supply clock */
    (*adg).clk_ssif_supply = devm_clk_get_optional(dev, b"ssif_supply\0".as_ptr() as *const c_char);
    if IS_ERR((*adg).clk_ssif_supply as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*adg).clk_ssif_supply as *const c_void),
            b"failed to get ssif_supply clock\n\0".as_ptr() as *const c_char,
        );
    }

    /* Per-SSI ADG clocks (RZ/G3E-only; no legacy dotted form exists) */
    for i in 0..ADG_SSI_MAX {
        snprintf(
            name.as_mut_ptr(),
            name.len(),
            b"adg-ssi-%d\0".as_ptr() as *const c_char,
            i as c_int,
        );
        (*adg).clk_adg_ssi[i] = devm_clk_get_optional(dev, name.as_ptr());
        if IS_ERR((*adg).clk_adg_ssi[i] as *const c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR((*adg).clk_adg_ssi[i] as *const c_void),
                b"failed to get %s clock\n\0".as_ptr() as *const c_char,
                name.as_ptr(),
            );
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_adg_probe(priv_: *mut rsnd_priv) -> c_int {
    let mut rstc: *mut reset_control;
    let mut adg: *mut rsnd_adg;
    let dev = rsnd_priv_to_dev(priv_);
    let mut ret: c_int;

    adg = devm_kzalloc(dev, core::mem::size_of::<rsnd_adg>(), GFP_KERNEL) as *mut rsnd_adg;
    if adg.is_null() {
        return -ENOMEM;
    }

    rstc = devm_reset_control_get_optional_exclusive(dev, b"adg\0".as_ptr() as *const c_char);
    if IS_ERR(rstc as *const c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR(rstc as *const c_void),
            b"failed to get adg reset\n\0".as_ptr() as *const c_char,
        );
    }

    ret = rsnd_mod_init(
        priv_,
        &mut (*adg).mod_,
        &mut adg_ops,
        ptr::null_mut(),
        rstc,
        0,
        0,
    );
    if ret != 0 {
        return ret;
    }

    (*priv_).adg = adg;

    ret = rsnd_adg_get_clkin(priv_);
    if ret != 0 {
        return ret;
    }

    ret = rsnd_adg_get_clkout(priv_);
    if ret != 0 {
        return ret;
    }

    /* RZ/G3E-specific: per-SSI ADG and SSIF supply clocks */
    ret = rsnd_adg_get_ssi_clks(priv_);
    if ret != 0 {
        return ret;
    }

    ret = rsnd_adg_clk_enable(priv_);
    if ret != 0 {
        return ret;
    }

    rsnd_adg_clk_dbg_info(priv_, ptr::null_mut());

    0
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_adg_remove(priv_: *mut rsnd_priv) {
    let dev = rsnd_priv_to_dev(priv_);
    let np = (*dev).of_node;

    rsnd_adg_unregister_clkout(priv_);

    of_clk_del_provider(np);

    rsnd_adg_clk_disable(priv_);

    /* It should be called after rsnd_adg_clk_disable() */
    rsnd_adg_null_clk_clean(priv_);
}

unsafe fn rsnd_adg_mod_get(priv_: *mut rsnd_priv) -> *mut rsnd_mod {
    let adg = rsnd_priv_to_adg(priv_);

    if adg.is_null() {
        return ptr::null_mut();
    }

    rsnd_mod_get(adg)
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_adg_suspend(priv_: *mut rsnd_priv) {
    let mod_ = rsnd_adg_mod_get(priv_);

    if !mod_.is_null() {
        rsnd_suspend_clk_reset((*mod_).clk, (*mod_).rstc);
    }
}

#[no_mangle]
pub unsafe extern "C" fn rsnd_adg_resume(priv_: *mut rsnd_priv) {
    let mod_ = rsnd_adg_mod_get(priv_);

    if !mod_.is_null() {
        rsnd_resume_clk_reset((*mod_).clk, (*mod_).rstc);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
