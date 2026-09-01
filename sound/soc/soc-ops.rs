// SPDX-License-Identifier: GPL-2.0+
//
// soc-ops.c  --  Generic ASoC operations
//
// Copyright 2005 Wolfson Microelectronics PLC.
// Copyright 2005 Openedhand Ltd.
// Copyright (C) 2010 Slimlogic Ltd.
// Copyright (C) 2010 Texas Instruments Inc.
//
// Author: Liam Girdwood <lrg@slimlogic.co.uk>
//         with code, comments and ideas from :-
//         Richard Purdie <richard@openedhand.com>

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const GFP_KERNEL: c_uint = 0;
const GFP_DMA: c_uint = 0;
const BITS_PER_BYTE: c_uint = 8;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_int = 1;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const SNDRV_CTL_ELEM_TYPE_BYTES: c_int = 4;
const SNDRV_CTL_TLV_OP_READ: c_int = 0;
const SNDRV_CTL_TLV_OP_WRITE: c_int = 1;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    pub regmap: *mut regmap,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub name: [c_char; 44],
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_ctl_elem_id,
    pub private_value: c_ulong,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_int,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: c_long,
    pub max: c_long,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
    pub bytes: snd_ctl_elem_value_bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_bytes {
    pub data: [u8; 512],
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub shift_r: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
    pub values: *const c_uint,
    pub mask: c_uint,
}

#[repr(C)]
pub struct soc_mixer_control {
    pub reg: c_uint,
    pub rreg: c_uint,
    pub shift: c_uint,
    pub rshift: c_uint,
    pub max: c_int,
    pub min: c_int,
    pub platform_max: c_int,
    pub invert: c_uint,
    pub sign_bit: c_uint,
}

#[repr(C)]
pub struct soc_bytes {
    pub base: c_uint,
    pub num_regs: c_int,
    pub mask: c_uint,
}

#[repr(C)]
pub struct soc_bytes_ext {
    pub max: c_uint,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut c_uint, c_uint) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut c_uint, c_uint) -> c_int>,
}

#[repr(C)]
pub struct soc_mreg_control {
    pub regbase: c_uint,
    pub regcount: c_uint,
    pub nbits: c_uint,
    pub min: c_long,
    pub max: c_long,
    pub invert: c_uint,
}

unsafe extern "C" {
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        names: *const *const c_char,
    ) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_enum_val_to_item(e: *mut soc_enum, val: c_uint) -> c_uint;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_component_regmap_val_bytes(component: *mut snd_soc_component) -> c_int;
    fn snd_soc_card_get_kcontrol(card: *mut snd_soc_card, name: *const c_char) -> *mut snd_kcontrol;
    fn regmap_raw_read(map: *mut regmap, reg: c_uint, val: *mut c_void, val_len: c_int) -> c_int;
    fn regmap_raw_write(map: *mut regmap, reg: c_uint, val: *const c_void, val_len: c_int) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_parse_val(map: *mut regmap, val: *mut c_void, rval: *mut c_void) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kmemdup(src: *const c_void, len: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

const fn bit(nr: c_uint) -> c_uint {
    1u32 << nr
}

fn genmask(h: c_uint, l: c_uint) -> c_uint {
    if h >= 31 {
        (!0u32) << l
    } else {
        ((!0u32) << l) & (!0u32 >> (31 - h))
    }
}

fn genmask_ulong(h: c_uint, l: c_uint) -> c_ulong {
    let bits = c_ulong::BITS;
    if h >= bits - 1 {
        (!0 as c_ulong) << l
    } else {
        ((!0 as c_ulong) << l) & ((!0 as c_ulong) >> (bits - 1 - h))
    }
}

fn fls(x: c_int) -> c_int {
    if x == 0 {
        0
    } else {
        c_int::BITS as c_int - (x as c_uint).leading_zeros() as c_int
    }
}

fn sign_extend32(value: c_int, index: c_uint) -> c_int {
    let shift = 31 - index;
    ((value as c_uint) << shift) as c_int >> shift
}

fn clamp(val: c_int, lo: c_int, hi: c_int) -> c_int {
    if val < lo {
        lo
    } else if val > hi {
        hi
    } else {
        val
    }
}

unsafe fn snd_soc_volsw_is_stereo(mc: *mut soc_mixer_control) -> bool {
    unsafe { (*mc).reg != (*mc).rreg || (*mc).shift != (*mc).rshift }
}

/**
 * snd_soc_info_enum_double - enumerated double mixer info callback
 * @kcontrol: mixer control
 * @uinfo: control element information
 *
 * Callback to provide information about a double enumerated
 * mixer control.
 *
 * Returns 0 for success.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_info_enum_double(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let e = unsafe { (*kcontrol).private_value as *mut soc_enum };

    unsafe {
        snd_ctl_enum_info(
            uinfo,
            if (*e).shift_l == (*e).shift_r { 1 } else { 2 },
            (*e).items,
            (*e).texts,
        )
    }
}

/**
 * snd_soc_get_enum_double - enumerated double mixer get callback
 * @kcontrol: mixer control
 * @ucontrol: control element information
 *
 * Callback to get the value of a double enumerated mixer.
 *
 * Returns 0 for success.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_get_enum_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    let e = unsafe { (*kcontrol).private_value as *mut soc_enum };
    let reg_val = unsafe { snd_soc_component_read(component, (*e).reg) };
    let mut val = unsafe { (reg_val >> (*e).shift_l) & (*e).mask };
    let mut item = unsafe { snd_soc_enum_val_to_item(e, val) };
    unsafe {
        (*ucontrol).value.enumerated.item[0] = item;
        if (*e).shift_l != (*e).shift_r {
            val = (reg_val >> (*e).shift_r) & (*e).mask;
            item = snd_soc_enum_val_to_item(e, val);
            (*ucontrol).value.enumerated.item[1] = item;
        }
    }

    0
}

/**
 * snd_soc_put_enum_double - enumerated double mixer put callback
 * @kcontrol: mixer control
 * @ucontrol: control element information
 *
 * Callback to set the value of a double enumerated mixer.
 *
 * Returns 0 for success.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_put_enum_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    let e = unsafe { (*kcontrol).private_value as *mut soc_enum };
    let item = unsafe { (*ucontrol).value.enumerated.item.as_mut_ptr() };
    let mut val: c_uint;
    let mut mask: c_uint;

    unsafe {
        if *item.add(0) >= (*e).items {
            return -EINVAL;
        }
        val = snd_soc_enum_item_to_val(e, *item.add(0)) << (*e).shift_l;
        mask = (*e).mask << (*e).shift_l;
        if (*e).shift_l != (*e).shift_r {
            if *item.add(1) >= (*e).items {
                return -EINVAL;
            }
            val |= snd_soc_enum_item_to_val(e, *item.add(1)) << (*e).shift_r;
            mask |= (*e).mask << (*e).shift_r;
        }

        snd_soc_component_update_bits(component, (*e).reg, mask, val)
    }
}

unsafe fn soc_mixer_reg_to_ctl(
    mc: *mut soc_mixer_control,
    reg_val: c_uint,
    mask: c_uint,
    shift: c_uint,
    max: c_int,
    sx: bool,
) -> c_int {
    let mut val = ((reg_val >> shift) & mask) as c_int;

    unsafe {
        if (*mc).sign_bit != 0 {
            val = sign_extend32(val, (*mc).sign_bit);
        }

        if sx {
            val = val.wrapping_sub((*mc).min);
            val = core::cmp::min((val as c_uint) & mask, max as c_uint) as c_int;
        } else {
            val = clamp(val, (*mc).min, (*mc).max);
            val -= (*mc).min;
        }

        if (*mc).invert != 0 {
            val = max - val;
        }
    }

    val
}

unsafe fn soc_mixer_ctl_to_reg(
    mc: *mut soc_mixer_control,
    mut val: c_int,
    mask: c_uint,
    shift: c_uint,
    max: c_int,
) -> c_uint {
    unsafe {
        if (*mc).invert != 0 {
            val = max - val;
        }

        let reg_val = val + (*mc).min;
        ((reg_val as c_uint) & mask) << shift
    }
}

unsafe fn soc_mixer_valid_ctl(mc: *mut soc_mixer_control, val: c_long, max: c_int) -> c_int {
    unsafe {
        if val < 0 {
            return -EINVAL;
        }

        if (*mc).platform_max != 0 && val > (*mc).platform_max as c_long {
            return -EINVAL;
        }

        if val > max as c_long {
            return -EINVAL;
        }
    }

    0
}

unsafe fn soc_mixer_mask(mc: *mut soc_mixer_control) -> c_int {
    unsafe {
        if (*mc).sign_bit != 0 {
            genmask((*mc).sign_bit, 0) as c_int
        } else {
            genmask((fls((*mc).max) - 1) as c_uint, 0) as c_int
        }
    }
}

unsafe fn soc_mixer_sx_mask(mc: *mut soc_mixer_control) -> c_int {
    unsafe {
        // min + max will take us 1-bit over the size of the mask
        genmask((fls((*mc).min + (*mc).max) - 2) as c_uint, 0) as c_int
    }
}

unsafe fn soc_info_volsw(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
    mc: *mut soc_mixer_control,
    mut max: c_int,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;

        if max == 1 {
            /* Even two value controls ending in Volume should be integer */
            let vol_string = strstr((*kcontrol).id.name.as_ptr(), c" Volume".as_ptr());

            if vol_string.is_null() || strcmp(vol_string, c" Volume".as_ptr()) != 0 {
                (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
            }
        }

        if (*mc).platform_max != 0 && (*mc).platform_max < max {
            max = (*mc).platform_max;
        }

        (*uinfo).count = if snd_soc_volsw_is_stereo(mc) { 2 } else { 1 };
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = max as c_long;
    }

    0
}

unsafe fn soc_put_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
    mc: *mut soc_mixer_control,
    mask: c_int,
    max: c_int,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut val2: c_uint = 0;
    let mut double_r = false;
    let mut ret: c_int;

    unsafe {
        ret = soc_mixer_valid_ctl(mc, (*ucontrol).value.integer.value[0], max);
        if ret != 0 {
            return ret;
        }

        let mut val1 = soc_mixer_ctl_to_reg(
            mc,
            (*ucontrol).value.integer.value[0] as c_int,
            mask as c_uint,
            (*mc).shift,
            max,
        );
        let mut val_mask = (mask as c_uint) << (*mc).shift;

        if snd_soc_volsw_is_stereo(mc) {
            ret = soc_mixer_valid_ctl(mc, (*ucontrol).value.integer.value[1], max);
            if ret != 0 {
                return ret;
            }

            if (*mc).reg == (*mc).rreg {
                val1 |= soc_mixer_ctl_to_reg(
                    mc,
                    (*ucontrol).value.integer.value[1] as c_int,
                    mask as c_uint,
                    (*mc).rshift,
                    max,
                );
                val_mask |= (mask as c_uint) << (*mc).rshift;
            } else {
                val2 = soc_mixer_ctl_to_reg(
                    mc,
                    (*ucontrol).value.integer.value[1] as c_int,
                    mask as c_uint,
                    (*mc).shift,
                    max,
                );
                double_r = true;
            }
        }

        ret = snd_soc_component_update_bits(component, (*mc).reg, val_mask, val1);
        if ret < 0 {
            return ret;
        }

        if double_r {
            let err = snd_soc_component_update_bits(component, (*mc).rreg, val_mask, val2);
            /* Don't drop change flag */
            if err != 0 {
                return err;
            }
        }
    }

    ret
}

unsafe fn soc_get_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
    mc: *mut soc_mixer_control,
    mask: c_int,
    max: c_int,
    sx: bool,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    unsafe {
        let mut reg_val = snd_soc_component_read(component, (*mc).reg);
        let mut val = soc_mixer_reg_to_ctl(mc, reg_val, mask as c_uint, (*mc).shift, max, sx);

        (*ucontrol).value.integer.value[0] = val as c_long;

        if snd_soc_volsw_is_stereo(mc) {
            if (*mc).reg == (*mc).rreg {
                val = soc_mixer_reg_to_ctl(mc, reg_val, mask as c_uint, (*mc).rshift, max, sx);
            } else {
                reg_val = snd_soc_component_read(component, (*mc).rreg);
                val = soc_mixer_reg_to_ctl(mc, reg_val, mask as c_uint, (*mc).shift, max, sx);
            }

            (*ucontrol).value.integer.value[1] = val as c_long;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_info_volsw(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let mc = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };

    unsafe { soc_info_volsw(kcontrol, uinfo, mc, (*mc).max - (*mc).min) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_info_volsw_sx(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let mc = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };

    unsafe { soc_info_volsw(kcontrol, uinfo, mc, (*mc).max) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_get_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };
    let mask = unsafe { soc_mixer_mask(mc) };

    unsafe { soc_get_volsw(kcontrol, ucontrol, mc, mask, (*mc).max - (*mc).min, false) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_put_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };
    let mask = unsafe { soc_mixer_mask(mc) };

    unsafe { soc_put_volsw(kcontrol, ucontrol, mc, mask, (*mc).max - (*mc).min) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_get_volsw_sx(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };
    let mask = unsafe { soc_mixer_sx_mask(mc) };

    unsafe { soc_get_volsw(kcontrol, ucontrol, mc, mask, (*mc).max, true) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_put_volsw_sx(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let mc = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };
    let mask = unsafe { soc_mixer_sx_mask(mc) };

    unsafe { soc_put_volsw(kcontrol, ucontrol, mc, mask, (*mc).max) }
}

unsafe fn snd_soc_clip_to_platform_max(kctl: *mut snd_kcontrol) -> c_int {
    let mc = unsafe { (*kctl).private_value as *mut soc_mixer_control };
    let mut ret: c_int;

    unsafe {
        if (*mc).platform_max == 0 {
            return 0;
        }

        let uctl = kzalloc(core::mem::size_of::<snd_ctl_elem_value>(), GFP_KERNEL)
            as *mut snd_ctl_elem_value;
        if uctl.is_null() {
            return -ENOMEM;
        }

        ret = if let Some(get) = (*kctl).get {
            get(kctl, uctl)
        } else {
            -EINVAL
        };
        if ret < 0 {
            kfree(uctl as *const c_void);
            return ret;
        }

        if (*uctl).value.integer.value[0] > (*mc).platform_max as c_long {
            (*uctl).value.integer.value[0] = (*mc).platform_max as c_long;
        }

        if snd_soc_volsw_is_stereo(mc)
            && (*uctl).value.integer.value[1] > (*mc).platform_max as c_long
        {
            (*uctl).value.integer.value[1] = (*mc).platform_max as c_long;
        }

        ret = if let Some(put) = (*kctl).put {
            put(kctl, uctl)
        } else {
            -EINVAL
        };

        kfree(uctl as *const c_void);
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_limit_volume(
    card: *mut snd_soc_card,
    name: *const c_char,
    max: c_int,
) -> c_int {
    let mut ret = -EINVAL;

    unsafe {
        /* Sanity check for name and max */
        if name.is_null() || max <= 0 {
            return -EINVAL;
        }

        let kctl = snd_soc_card_get_kcontrol(card, name);
        if !kctl.is_null() {
            let mc = (*kctl).private_value as *mut soc_mixer_control;

            if max <= (*mc).max - (*mc).min {
                (*mc).platform_max = max;
                ret = snd_soc_clip_to_platform_max(kctl);
            }
        } else {
            /* Some cards blindly add limits for multiple variants. */
            dev_dbg(
                (*card).dev,
                c"Volume limit for unknown control '%s'\n".as_ptr(),
                name,
            );
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_bytes_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    let params = unsafe { (*kcontrol).private_value as *mut soc_bytes };
    let val_bytes = unsafe { snd_soc_component_regmap_val_bytes(component) };

    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
        (*uinfo).count = ((*params).num_regs * val_bytes) as c_uint;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_bytes_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    let params = unsafe { (*kcontrol).private_value as *mut soc_bytes };
    let val_bytes = unsafe { snd_soc_component_regmap_val_bytes(component) };
    let ret: c_int;

    unsafe {
        if !(*component).regmap.is_null() {
            ret = regmap_raw_read(
                (*component).regmap,
                (*params).base,
                (*ucontrol).value.bytes.data.as_mut_ptr() as *mut c_void,
                (*params).num_regs * val_bytes,
            );
        } else {
            ret = -EINVAL;
        }

        /* Hide any masked bytes to ensure consistent data reporting */
        if ret == 0 && (*params).mask != 0 {
            match val_bytes {
                1 => {
                    (*ucontrol).value.bytes.data[0] &= !((*params).mask as u8);
                }
                2 => {
                    let p = (*ucontrol).value.bytes.data.as_mut_ptr() as *mut u16;
                    *p &= (!(*params).mask as u16).to_be();
                }
                4 => {
                    let p = (*ucontrol).value.bytes.data.as_mut_ptr() as *mut u32;
                    *p &= (!(*params).mask as u32).to_be();
                }
                _ => return -EINVAL,
            }
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_bytes_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    let params = unsafe { (*kcontrol).private_value as *mut soc_bytes };
    let val_bytes = unsafe { snd_soc_component_regmap_val_bytes(component) };
    let mut val: c_uint = 0;
    let mut mask: c_uint;
    let mut ret: c_int;

    unsafe {
        if (*component).regmap.is_null() || (*params).num_regs == 0 {
            return -EINVAL;
        }

        let len = (*params).num_regs * val_bytes;

        let data = kmemdup(
            (*ucontrol).value.bytes.data.as_ptr() as *const c_void,
            len as usize,
            GFP_KERNEL | GFP_DMA,
        );
        if data.is_null() {
            return -ENOMEM;
        }

        /*
         * If we've got a mask then we need to preserve the register
         * bits.  We shouldn't modify the incoming data so take a
         * copy.
         */
        if (*params).mask != 0 {
            ret = regmap_read((*component).regmap, (*params).base, &mut val);
            if ret != 0 {
                kfree(data);
                return ret;
            }

            val &= (*params).mask;

            match val_bytes {
                1 => {
                    let p = data as *mut u8;
                    *p &= !((*params).mask as u8);
                    *p |= val as u8;
                }
                2 => {
                    mask = !(*params).mask;
                    ret = regmap_parse_val(
                        (*component).regmap,
                        &mut mask as *mut c_uint as *mut c_void,
                        &mut mask as *mut c_uint as *mut c_void,
                    );
                    if ret != 0 {
                        kfree(data);
                        return ret;
                    }

                    let p = data as *mut u16;
                    *p &= mask as u16;

                    ret = regmap_parse_val(
                        (*component).regmap,
                        &mut val as *mut c_uint as *mut c_void,
                        &mut val as *mut c_uint as *mut c_void,
                    );
                    if ret != 0 {
                        kfree(data);
                        return ret;
                    }

                    *p |= val as u16;
                }
                4 => {
                    mask = !(*params).mask;
                    ret = regmap_parse_val(
                        (*component).regmap,
                        &mut mask as *mut c_uint as *mut c_void,
                        &mut mask as *mut c_uint as *mut c_void,
                    );
                    if ret != 0 {
                        kfree(data);
                        return ret;
                    }

                    let p = data as *mut u32;
                    *p &= mask as u32;

                    ret = regmap_parse_val(
                        (*component).regmap,
                        &mut val as *mut c_uint as *mut c_void,
                        &mut val as *mut c_uint as *mut c_void,
                    );
                    if ret != 0 {
                        kfree(data);
                        return ret;
                    }

                    *p |= val as u32;
                }
                _ => {
                    kfree(data);
                    return -EINVAL;
                }
            }
        }

        ret = regmap_raw_write((*component).regmap, (*params).base, data, len);
        kfree(data);
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_bytes_info_ext(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_info,
) -> c_int {
    let params = unsafe { (*kcontrol).private_value as *mut soc_bytes_ext };

    unsafe {
        (*ucontrol).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
        (*ucontrol).count = (*params).max;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_bytes_tlv_callback(
    kcontrol: *mut snd_kcontrol,
    op_flag: c_int,
    size: c_uint,
    tlv: *mut c_uint,
) -> c_int {
    let params = unsafe { (*kcontrol).private_value as *mut soc_bytes_ext };
    let count = unsafe { if size < (*params).max { size } else { (*params).max } };
    let mut ret = -ENXIO;

    unsafe {
        match op_flag {
            SNDRV_CTL_TLV_OP_READ => {
                if let Some(get) = (*params).get {
                    ret = get(kcontrol, tlv, count);
                }
            }
            SNDRV_CTL_TLV_OP_WRITE => {
                if let Some(put) = (*params).put {
                    ret = put(kcontrol, tlv, count);
                }
            }
            _ => {}
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_info_xr_sx(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let mc = unsafe { (*kcontrol).private_value as *mut soc_mreg_control };

    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = 1;
        (*uinfo).value.integer.min = (*mc).min;
        (*uinfo).value.integer.max = (*mc).max;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_get_xr_sx(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    let mc = unsafe { (*kcontrol).private_value as *mut soc_mreg_control };
    let val_bytes = unsafe { snd_soc_component_regmap_val_bytes(component) };
    unsafe {
        let regbase = (*mc).regbase;
        let regcount = (*mc).regcount;
        let regwshift = val_bytes as c_uint * BITS_PER_BYTE;
        let regwmask = genmask(regwshift - 1, 0);
        let mask = genmask_ulong((*mc).nbits - 1, 0);
        let mut val: c_long = 0;
        let mut i: c_uint = 0;

        while i < regcount {
            let regval = snd_soc_component_read(component, regbase + i);

            val |= (((regval & regwmask) as c_ulong) << (regwshift * (regcount - i - 1)))
                as c_long;
            i += 1;
        }
        val &= mask as c_long;
        if (*mc).min < 0 && val > (*mc).max {
            val |= !(mask as c_long);
        }
        if (*mc).invert != 0 {
            val = (*mc).max - val;
        }
        (*ucontrol).value.integer.value[0] = val;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_put_xr_sx(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    let mc = unsafe { (*kcontrol).private_value as *mut soc_mreg_control };
    let val_bytes = unsafe { snd_soc_component_regmap_val_bytes(component) };
    let mut ret = 0;

    unsafe {
        let regbase = (*mc).regbase;
        let regcount = (*mc).regcount;
        let regwshift = val_bytes as c_uint * BITS_PER_BYTE;
        let regwmask = genmask(regwshift - 1, 0);
        let mask = genmask_ulong((*mc).nbits - 1, 0);
        let mut val = (*ucontrol).value.integer.value[0];
        let mut i: c_uint = 0;

        if val < (*mc).min || val > (*mc).max {
            return -EINVAL;
        }
        if (*mc).invert != 0 {
            val = (*mc).max - val;
        }
        val &= mask as c_long;
        while i < regcount {
            let regval = (((val as c_ulong) >> (regwshift * (regcount - i - 1))) as c_uint)
                & regwmask;
            let regmask = ((mask >> (regwshift * (regcount - i - 1))) as c_uint) & regwmask;
            let err = snd_soc_component_update_bits(component, regbase + i, regmask, regval);

            if err < 0 {
                return err;
            }
            if err > 0 {
                ret = err;
            }
            i += 1;
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_get_strobe(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    let mc = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };

    unsafe {
        let invert = ((*mc).invert != 0) as c_uint;
        let mask = bit((*mc).shift);
        let mut val = snd_soc_component_read(component, (*mc).reg);
        val &= mask;

        if (*mc).shift != 0 && val != 0 {
            val >>= (*mc).shift;
        }

        (*ucontrol).value.enumerated.item[0] = val ^ invert;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_soc_put_strobe(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = unsafe { snd_kcontrol_chip(kcontrol) };
    let mc = unsafe { (*kcontrol).private_value as *mut soc_mixer_control };

    unsafe {
        let strobe = ((*ucontrol).value.enumerated.item[0] != 0) as c_uint;
        let invert = ((*mc).invert != 0) as c_uint;
        let mask = bit((*mc).shift);
        let val1 = if (strobe ^ invert) != 0 { mask } else { 0 };
        let val2 = if (strobe ^ invert) != 0 { 0 } else { mask };

        let ret = snd_soc_component_update_bits(component, (*mc).reg, mask, val1);
        if ret < 0 {
            return ret;
        }

        snd_soc_component_update_bits(component, (*mc).reg, mask, val2)
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
