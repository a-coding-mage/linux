// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Universal routines for AK4531 codec
 */

/*
MODULE_AUTHOR("Jaroslav Kysela <perex@perex.cz>");
MODULE_DESCRIPTION("Universal routines for AK4531 codec");
MODULE_LICENSE("GPL");
*/

use crate::*;

/*
 *
 */

/* #if 0 */

unsafe extern "C" fn snd_ak4531_dump(ak4531: *mut snd_ak4531) {
    let mut idx: c_int;

    idx = 0;
    while idx < 0x19 {
        printk(
            KERN_DEBUG b"ak4531 0x%x: 0x%x\n\0".as_ptr() as *const c_char,
            idx,
            (*ak4531).regs[idx as usize] as c_int,
        );
        idx += 1;
    }
}

/* #endif */

/*
 *
 */

macro_rules! AK4531_SINGLE {
    ($xname:expr, $xindex:expr, $reg:expr, $shift:expr, $mask:expr, $invert:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            index: $xindex,
            info: Some(snd_ak4531_info_single),
            get: Some(snd_ak4531_get_single),
            put: Some(snd_ak4531_put_single),
            private_value: ($reg
                | ($shift << 16)
                | ($mask << 24)
                | ($invert << 22)) as c_ulong,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

macro_rules! AK4531_SINGLE_TLV {
    ($xname:expr, $xindex:expr, $reg:expr, $shift:expr, $mask:expr, $invert:expr, $xtlv:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            index: $xindex,
            info: Some(snd_ak4531_info_single),
            get: Some(snd_ak4531_get_single),
            put: Some(snd_ak4531_put_single),
            private_value: ($reg
                | ($shift << 16)
                | ($mask << 24)
                | ($invert << 22)) as c_ulong,
            tlv: snd_kcontrol_new_tlv { p: $xtlv.as_ptr() },
            ..unsafe { core::mem::zeroed() }
        }
    };
}

unsafe extern "C" fn snd_ak4531_info_single(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let mask: c_int = (((*kcontrol).private_value >> 24) & 0xff) as c_int;

    (*uinfo).type_ = if mask == 1 {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN
    } else {
        SNDRV_CTL_ELEM_TYPE_INTEGER
    };
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_ak4531_get_single(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak4531: *mut snd_ak4531 = snd_kcontrol_chip(kcontrol) as *mut snd_ak4531;
    let reg: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let shift: c_int = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let mask: c_int = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let invert: c_int = (((*kcontrol).private_value >> 22) & 1) as c_int;
    let mut val: c_int;

    let _guard = guard_mutex(&mut (*ak4531).reg_mutex);
    val = (((*ak4531).regs[reg as usize] as c_int) >> shift) & mask;
    if invert != 0 {
        val = mask - val;
    }
    (*ucontrol).value.integer.value[0] = val as c_long;
    0
}

unsafe extern "C" fn snd_ak4531_put_single(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak4531: *mut snd_ak4531 = snd_kcontrol_chip(kcontrol) as *mut snd_ak4531;
    let reg: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let shift: c_int = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let mask: c_int = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let invert: c_int = (((*kcontrol).private_value >> 22) & 1) as c_int;
    let change: c_int;
    let mut val: c_int;

    val = ((*ucontrol).value.integer.value[0] as c_int) & mask;
    if invert != 0 {
        val = mask - val;
    }
    val <<= shift;
    let _guard = guard_mutex(&mut (*ak4531).reg_mutex);
    val = (((*ak4531).regs[reg as usize] as c_int) & !(mask << shift)) | val;
    change = (val != (*ak4531).regs[reg as usize] as c_int) as c_int;
    (*ak4531).regs[reg as usize] = val as u8;
    ((*ak4531).write.unwrap())(ak4531, reg as c_uint, (*ak4531).regs[reg as usize] as c_uint);
    change
}

macro_rules! AK4531_DOUBLE {
    ($xname:expr, $xindex:expr, $left_reg:expr, $right_reg:expr, $left_shift:expr, $right_shift:expr, $mask:expr, $invert:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            index: $xindex,
            info: Some(snd_ak4531_info_double),
            get: Some(snd_ak4531_get_double),
            put: Some(snd_ak4531_put_double),
            private_value: ($left_reg
                | ($right_reg << 8)
                | ($left_shift << 16)
                | ($right_shift << 19)
                | ($mask << 24)
                | ($invert << 22)) as c_ulong,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

macro_rules! AK4531_DOUBLE_TLV {
    ($xname:expr, $xindex:expr, $left_reg:expr, $right_reg:expr, $left_shift:expr, $right_shift:expr, $mask:expr, $invert:expr, $xtlv:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            index: $xindex,
            info: Some(snd_ak4531_info_double),
            get: Some(snd_ak4531_get_double),
            put: Some(snd_ak4531_put_double),
            private_value: ($left_reg
                | ($right_reg << 8)
                | ($left_shift << 16)
                | ($right_shift << 19)
                | ($mask << 24)
                | ($invert << 22)) as c_ulong,
            tlv: snd_kcontrol_new_tlv { p: $xtlv.as_ptr() },
            ..unsafe { core::mem::zeroed() }
        }
    };
}

unsafe extern "C" fn snd_ak4531_info_double(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let mask: c_int = (((*kcontrol).private_value >> 24) & 0xff) as c_int;

    (*uinfo).type_ = if mask == 1 {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN
    } else {
        SNDRV_CTL_ELEM_TYPE_INTEGER
    };
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_ak4531_get_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak4531: *mut snd_ak4531 = snd_kcontrol_chip(kcontrol) as *mut snd_ak4531;
    let left_reg: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let right_reg: c_int = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let left_shift: c_int = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let right_shift: c_int = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
    let mask: c_int = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let invert: c_int = (((*kcontrol).private_value >> 22) & 1) as c_int;
    let mut left: c_int;
    let mut right: c_int;

    let _guard = guard_mutex(&mut (*ak4531).reg_mutex);
    left = (((*ak4531).regs[left_reg as usize] as c_int) >> left_shift) & mask;
    right = (((*ak4531).regs[right_reg as usize] as c_int) >> right_shift) & mask;
    if invert != 0 {
        left = mask - left;
        right = mask - right;
    }
    (*ucontrol).value.integer.value[0] = left as c_long;
    (*ucontrol).value.integer.value[1] = right as c_long;
    0
}

unsafe extern "C" fn snd_ak4531_put_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak4531: *mut snd_ak4531 = snd_kcontrol_chip(kcontrol) as *mut snd_ak4531;
    let left_reg: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let right_reg: c_int = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let left_shift: c_int = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let right_shift: c_int = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
    let mask: c_int = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let invert: c_int = (((*kcontrol).private_value >> 22) & 1) as c_int;
    let change: c_int;
    let mut left: c_int;
    let mut right: c_int;

    left = ((*ucontrol).value.integer.value[0] as c_int) & mask;
    right = ((*ucontrol).value.integer.value[1] as c_int) & mask;
    if invert != 0 {
        left = mask - left;
        right = mask - right;
    }
    left <<= left_shift;
    right <<= right_shift;
    let _guard = guard_mutex(&mut (*ak4531).reg_mutex);
    if left_reg == right_reg {
        left = (((*ak4531).regs[left_reg as usize] as c_int)
            & !((mask << left_shift) | (mask << right_shift)))
            | left
            | right;
        change = (left != (*ak4531).regs[left_reg as usize] as c_int) as c_int;
        (*ak4531).regs[left_reg as usize] = left as u8;
        ((*ak4531).write.unwrap())(
            ak4531,
            left_reg as c_uint,
            (*ak4531).regs[left_reg as usize] as c_uint,
        );
    } else {
        left = (((*ak4531).regs[left_reg as usize] as c_int) & !(mask << left_shift)) | left;
        right = (((*ak4531).regs[right_reg as usize] as c_int) & !(mask << right_shift)) | right;
        change = (left != (*ak4531).regs[left_reg as usize] as c_int
            || right != (*ak4531).regs[right_reg as usize] as c_int) as c_int;
        (*ak4531).regs[left_reg as usize] = left as u8;
        ((*ak4531).write.unwrap())(
            ak4531,
            left_reg as c_uint,
            (*ak4531).regs[left_reg as usize] as c_uint,
        );
        (*ak4531).regs[right_reg as usize] = right as u8;
        ((*ak4531).write.unwrap())(
            ak4531,
            right_reg as c_uint,
            (*ak4531).regs[right_reg as usize] as c_uint,
        );
    }
    change
}

macro_rules! AK4531_INPUT_SW {
    ($xname:expr, $xindex:expr, $reg1:expr, $reg2:expr, $left_shift:expr, $right_shift:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            index: $xindex,
            info: Some(snd_ak4531_info_input_sw),
            get: Some(snd_ak4531_get_input_sw),
            put: Some(snd_ak4531_put_input_sw),
            private_value: ($reg1 | ($reg2 << 8) | ($left_shift << 16) | ($right_shift << 24))
                as c_ulong,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

unsafe extern "C" fn snd_ak4531_info_input_sw(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 4;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe extern "C" fn snd_ak4531_get_input_sw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak4531: *mut snd_ak4531 = snd_kcontrol_chip(kcontrol) as *mut snd_ak4531;
    let reg1: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let reg2: c_int = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let left_shift: c_int = (((*kcontrol).private_value >> 16) & 0x0f) as c_int;
    let right_shift: c_int = (((*kcontrol).private_value >> 24) & 0x0f) as c_int;

    let _guard = guard_mutex(&mut (*ak4531).reg_mutex);
    (*ucontrol).value.integer.value[0] =
        (((*ak4531).regs[reg1 as usize] as c_int >> left_shift) & 1) as c_long;
    (*ucontrol).value.integer.value[1] =
        (((*ak4531).regs[reg2 as usize] as c_int >> left_shift) & 1) as c_long;
    (*ucontrol).value.integer.value[2] =
        (((*ak4531).regs[reg1 as usize] as c_int >> right_shift) & 1) as c_long;
    (*ucontrol).value.integer.value[3] =
        (((*ak4531).regs[reg2 as usize] as c_int >> right_shift) & 1) as c_long;
    0
}

unsafe extern "C" fn snd_ak4531_put_input_sw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let ak4531: *mut snd_ak4531 = snd_kcontrol_chip(kcontrol) as *mut snd_ak4531;
    let reg1: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let reg2: c_int = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let left_shift: c_int = (((*kcontrol).private_value >> 16) & 0x0f) as c_int;
    let right_shift: c_int = (((*kcontrol).private_value >> 24) & 0x0f) as c_int;
    let change: c_int;
    let mut val1: c_int;
    let mut val2: c_int;

    let _guard = guard_mutex(&mut (*ak4531).reg_mutex);
    val1 = ((*ak4531).regs[reg1 as usize] as c_int) & !((1 << left_shift) | (1 << right_shift));
    val2 = ((*ak4531).regs[reg2 as usize] as c_int) & !((1 << left_shift) | (1 << right_shift));
    val1 |= (((*ucontrol).value.integer.value[0] as c_int) & 1) << left_shift;
    val2 |= (((*ucontrol).value.integer.value[1] as c_int) & 1) << left_shift;
    val1 |= (((*ucontrol).value.integer.value[2] as c_int) & 1) << right_shift;
    val2 |= (((*ucontrol).value.integer.value[3] as c_int) & 1) << right_shift;
    change = (val1 != (*ak4531).regs[reg1 as usize] as c_int
        || val2 != (*ak4531).regs[reg2 as usize] as c_int) as c_int;
    (*ak4531).regs[reg1 as usize] = val1 as u8;
    ((*ak4531).write.unwrap())(ak4531, reg1 as c_uint, (*ak4531).regs[reg1 as usize] as c_uint);
    (*ak4531).regs[reg2 as usize] = val2 as u8;
    ((*ak4531).write.unwrap())(ak4531, reg2 as c_uint, (*ak4531).regs[reg2 as usize] as c_uint);
    change
}

static db_scale_master: [c_uint; 4] = [
    SNDRV_CTL_TLVT_DB_SCALE,
    2 * core::mem::size_of::<c_uint>() as c_uint,
    (-6200i32) as c_uint,
    200,
];
static db_scale_mono: [c_uint; 4] = [
    SNDRV_CTL_TLVT_DB_SCALE,
    2 * core::mem::size_of::<c_uint>() as c_uint,
    (-2800i32) as c_uint,
    400,
];
static db_scale_input: [c_uint; 4] = [
    SNDRV_CTL_TLVT_DB_SCALE,
    2 * core::mem::size_of::<c_uint>() as c_uint,
    (-5000i32) as c_uint,
    200,
];

static snd_ak4531_controls: [snd_kcontrol_new; 41] = [
    AK4531_DOUBLE_TLV!("Master Playback Switch", 0, AK4531_LMASTER, AK4531_RMASTER, 7, 7, 1, 1, db_scale_master),
    AK4531_DOUBLE!("Master Playback Volume", 0, AK4531_LMASTER, AK4531_RMASTER, 0, 0, 0x1f, 1),
    AK4531_SINGLE_TLV!("Master Mono Playback Switch", 0, AK4531_MONO_OUT, 7, 1, 1, db_scale_mono),
    AK4531_SINGLE!("Master Mono Playback Volume", 0, AK4531_MONO_OUT, 0, 0x07, 1),
    AK4531_DOUBLE!("PCM Switch", 0, AK4531_LVOICE, AK4531_RVOICE, 7, 7, 1, 1),
    AK4531_DOUBLE_TLV!("PCM Volume", 0, AK4531_LVOICE, AK4531_RVOICE, 0, 0, 0x1f, 1, db_scale_input),
    AK4531_DOUBLE!("PCM Playback Switch", 0, AK4531_OUT_SW2, AK4531_OUT_SW2, 3, 2, 1, 0),
    AK4531_DOUBLE!("PCM Capture Switch", 0, AK4531_LIN_SW2, AK4531_RIN_SW2, 2, 2, 1, 0),
    AK4531_DOUBLE!("PCM Switch", 1, AK4531_LFM, AK4531_RFM, 7, 7, 1, 1),
    AK4531_DOUBLE_TLV!("PCM Volume", 1, AK4531_LFM, AK4531_RFM, 0, 0, 0x1f, 1, db_scale_input),
    AK4531_DOUBLE!("PCM Playback Switch", 1, AK4531_OUT_SW1, AK4531_OUT_SW1, 6, 5, 1, 0),
    AK4531_INPUT_SW!("PCM Capture Route", 1, AK4531_LIN_SW1, AK4531_RIN_SW1, 6, 5),
    AK4531_DOUBLE!("CD Switch", 0, AK4531_LCD, AK4531_RCD, 7, 7, 1, 1),
    AK4531_DOUBLE_TLV!("CD Volume", 0, AK4531_LCD, AK4531_RCD, 0, 0, 0x1f, 1, db_scale_input),
    AK4531_DOUBLE!("CD Playback Switch", 0, AK4531_OUT_SW1, AK4531_OUT_SW1, 2, 1, 1, 0),
    AK4531_INPUT_SW!("CD Capture Route", 0, AK4531_LIN_SW1, AK4531_RIN_SW1, 2, 1),
    AK4531_DOUBLE!("Line Switch", 0, AK4531_LLINE, AK4531_RLINE, 7, 7, 1, 1),
    AK4531_DOUBLE_TLV!("Line Volume", 0, AK4531_LLINE, AK4531_RLINE, 0, 0, 0x1f, 1, db_scale_input),
    AK4531_DOUBLE!("Line Playback Switch", 0, AK4531_OUT_SW1, AK4531_OUT_SW1, 4, 3, 1, 0),
    AK4531_INPUT_SW!("Line Capture Route", 0, AK4531_LIN_SW1, AK4531_RIN_SW1, 4, 3),
    AK4531_DOUBLE!("Aux Switch", 0, AK4531_LAUXA, AK4531_RAUXA, 7, 7, 1, 1),
    AK4531_DOUBLE_TLV!("Aux Volume", 0, AK4531_LAUXA, AK4531_RAUXA, 0, 0, 0x1f, 1, db_scale_input),
    AK4531_DOUBLE!("Aux Playback Switch", 0, AK4531_OUT_SW2, AK4531_OUT_SW2, 5, 4, 1, 0),
    AK4531_INPUT_SW!("Aux Capture Route", 0, AK4531_LIN_SW2, AK4531_RIN_SW2, 4, 3),
    AK4531_SINGLE!("Mono Switch", 0, AK4531_MONO1, 7, 1, 1),
    AK4531_SINGLE_TLV!("Mono Volume", 0, AK4531_MONO1, 0, 0x1f, 1, db_scale_input),
    AK4531_SINGLE!("Mono Playback Switch", 0, AK4531_OUT_SW2, 0, 1, 0),
    AK4531_DOUBLE!("Mono Capture Switch", 0, AK4531_LIN_SW2, AK4531_RIN_SW2, 0, 0, 1, 0),
    AK4531_SINGLE!("Mono Switch", 1, AK4531_MONO2, 7, 1, 1),
    AK4531_SINGLE_TLV!("Mono Volume", 1, AK4531_MONO2, 0, 0x1f, 1, db_scale_input),
    AK4531_SINGLE!("Mono Playback Switch", 1, AK4531_OUT_SW2, 1, 1, 0),
    AK4531_DOUBLE!("Mono Capture Switch", 1, AK4531_LIN_SW2, AK4531_RIN_SW2, 1, 1, 1, 0),
    AK4531_SINGLE_TLV!("Mic Volume", 0, AK4531_MIC, 0, 0x1f, 1, db_scale_input),
    AK4531_SINGLE!("Mic Switch", 0, AK4531_MIC, 7, 1, 1),
    AK4531_SINGLE!("Mic Playback Switch", 0, AK4531_OUT_SW1, 0, 1, 0),
    AK4531_DOUBLE!("Mic Capture Switch", 0, AK4531_LIN_SW1, AK4531_RIN_SW1, 0, 0, 1, 0),
    AK4531_DOUBLE!("Mic Bypass Capture Switch", 0, AK4531_LIN_SW2, AK4531_RIN_SW2, 7, 7, 1, 0),
    AK4531_DOUBLE!("Mono1 Bypass Capture Switch", 0, AK4531_LIN_SW2, AK4531_RIN_SW2, 6, 6, 1, 0),
    AK4531_DOUBLE!("Mono2 Bypass Capture Switch", 0, AK4531_LIN_SW2, AK4531_RIN_SW2, 5, 5, 1, 0),
    AK4531_SINGLE!("AD Input Select", 0, AK4531_AD_IN, 0, 1, 0),
    AK4531_SINGLE!("Mic Boost (+30dB)", 0, AK4531_MIC_GAIN, 0, 1, 0),
];

unsafe extern "C" fn snd_ak4531_free(ak4531: *mut snd_ak4531) -> c_int {
    if !ak4531.is_null() {
        if (*ak4531).private_free.is_some() {
            ((*ak4531).private_free.unwrap())(ak4531);
        }
        kfree(ak4531 as *const c_void);
    }
    0
}

unsafe extern "C" fn snd_ak4531_dev_free(device: *mut snd_device) -> c_int {
    let ak4531: *mut snd_ak4531 = (*device).device_data as *mut snd_ak4531;
    snd_ak4531_free(ak4531)
}

static snd_ak4531_initial_map: [u8; 0x19 + 1] = [
    0x9f, /* 00: Master Volume Lch */
    0x9f, /* 01: Master Volume Rch */
    0x9f, /* 02: Voice Volume Lch */
    0x9f, /* 03: Voice Volume Rch */
    0x9f, /* 04: FM Volume Lch */
    0x9f, /* 05: FM Volume Rch */
    0x9f, /* 06: CD Audio Volume Lch */
    0x9f, /* 07: CD Audio Volume Rch */
    0x9f, /* 08: Line Volume Lch */
    0x9f, /* 09: Line Volume Rch */
    0x9f, /* 0a: Aux Volume Lch */
    0x9f, /* 0b: Aux Volume Rch */
    0x9f, /* 0c: Mono1 Volume */
    0x9f, /* 0d: Mono2 Volume */
    0x9f, /* 0e: Mic Volume */
    0x87, /* 0f: Mono-out Volume */
    0x00, /* 10: Output Mixer SW1 */
    0x00, /* 11: Output Mixer SW2 */
    0x00, /* 12: Lch Input Mixer SW1 */
    0x00, /* 13: Rch Input Mixer SW1 */
    0x00, /* 14: Lch Input Mixer SW2 */
    0x00, /* 15: Rch Input Mixer SW2 */
    0x00, /* 16: Reset & Power Down */
    0x00, /* 17: Clock Select */
    0x00, /* 18: AD Input Select */
    0x01, /* 19: Mic Amp Setup */
];

#[no_mangle]
pub unsafe extern "C" fn snd_ak4531_mixer(
    card: *mut snd_card,
    _ak4531: *mut snd_ak4531,
    rak4531: *mut *mut snd_ak4531,
) -> c_int {
    let mut idx: c_uint;
    let mut err: c_int;
    let ak4531: *mut snd_ak4531;
    static ops: snd_device_ops = snd_device_ops {
        dev_free: Some(snd_ak4531_dev_free),
        ..unsafe { core::mem::zeroed() }
    };

    if snd_BUG_ON(card.is_null() || _ak4531.is_null()) != 0 {
        return -EINVAL;
    }
    if !rak4531.is_null() {
        *rak4531 = core::ptr::null_mut();
    }
    ak4531 = kzalloc_obj::<snd_ak4531>();
    if ak4531.is_null() {
        return -ENOMEM;
    }
    *ak4531 = *_ak4531;
    mutex_init(&mut (*ak4531).reg_mutex);
    err = snd_component_add(card, b"AK4531\0".as_ptr() as *const c_char);
    if err < 0 {
        snd_ak4531_free(ak4531);
        return err;
    }
    strscpy(
        (*card).mixername.as_mut_ptr(),
        b"Asahi Kasei AK4531\0".as_ptr() as *const c_char,
        (*card).mixername.len(),
    );
    ((*ak4531).write.unwrap())(ak4531, AK4531_RESET as c_uint, 0x03); /* no RST, PD */
    udelay(100);
    ((*ak4531).write.unwrap())(ak4531, AK4531_CLOCK as c_uint, 0x00); /* CODEC ADC and CODEC DAC use {LR,B}CLK2 and run off LRCLK2 PLL */
    idx = 0;
    while idx <= 0x19 {
        if idx == AK4531_RESET as c_uint || idx == AK4531_CLOCK as c_uint {
            idx += 1;
            continue;
        }
        (*ak4531).regs[idx as usize] = snd_ak4531_initial_map[idx as usize];
        ((*ak4531).write.unwrap())(
            ak4531,
            idx,
            (*ak4531).regs[idx as usize] as c_uint,
        ); /* recording source is mixer */
        idx += 1;
    }
    idx = 0;
    while (idx as usize) < snd_ak4531_controls.len() {
        err = snd_ctl_add(
            card,
            snd_ctl_new1(&snd_ak4531_controls[idx as usize], ak4531 as *mut c_void),
        );
        if err < 0 {
            snd_ak4531_free(ak4531);
            return err;
        }
        idx += 1;
    }
    snd_ak4531_proc_init(card, ak4531);
    err = snd_device_new(card, SNDRV_DEV_CODEC, ak4531 as *mut c_void, &ops);
    if err < 0 {
        snd_ak4531_free(ak4531);
        return err;
    }

    /* #if 0 */
    snd_ak4531_dump(ak4531);
    /* #endif */
    if !rak4531.is_null() {
        *rak4531 = ak4531;
    }
    0
}

/*
 * power management
 */
/* #ifdef CONFIG_PM */
#[no_mangle]
pub unsafe extern "C" fn snd_ak4531_suspend(ak4531: *mut snd_ak4531) {
    /* mute */
    ((*ak4531).write.unwrap())(ak4531, AK4531_LMASTER as c_uint, 0x9f);
    ((*ak4531).write.unwrap())(ak4531, AK4531_RMASTER as c_uint, 0x9f);
    /* powerdown */
    ((*ak4531).write.unwrap())(ak4531, AK4531_RESET as c_uint, 0x01);
}

#[no_mangle]
pub unsafe extern "C" fn snd_ak4531_resume(ak4531: *mut snd_ak4531) {
    let mut idx: c_int;

    /* initialize */
    ((*ak4531).write.unwrap())(ak4531, AK4531_RESET as c_uint, 0x03);
    udelay(100);
    ((*ak4531).write.unwrap())(ak4531, AK4531_CLOCK as c_uint, 0x00);
    /* restore mixer registers */
    idx = 0;
    while idx <= 0x19 {
        if idx == AK4531_RESET || idx == AK4531_CLOCK {
            idx += 1;
            continue;
        }
        ((*ak4531).write.unwrap())(
            ak4531,
            idx as c_uint,
            (*ak4531).regs[idx as usize] as c_uint,
        );
        idx += 1;
    }
}
/* #endif */

/*
 * /proc interface
 */

unsafe extern "C" fn snd_ak4531_proc_read(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let ak4531: *mut snd_ak4531 = (*entry).private_data as *mut snd_ak4531;

    snd_iprintf(buffer, b"Asahi Kasei AK4531\n\n\0".as_ptr() as *const c_char);
    snd_iprintf(
        buffer,
        b"Recording source   : %s\nMIC gain           : %s\n\0".as_ptr() as *const c_char,
        if ((*ak4531).regs[AK4531_AD_IN as usize] & 1) != 0 {
            b"external\0".as_ptr() as *const c_char
        } else {
            b"mixer\0".as_ptr() as *const c_char
        },
        if ((*ak4531).regs[AK4531_MIC_GAIN as usize] & 1) != 0 {
            b"+30dB\0".as_ptr() as *const c_char
        } else {
            b"+0dB\0".as_ptr() as *const c_char
        },
    );
}

unsafe extern "C" fn snd_ak4531_proc_init(card: *mut snd_card, ak4531: *mut snd_ak4531) {
    snd_card_ro_proc_new(
        card,
        b"ak4531\0".as_ptr() as *const c_char,
        ak4531 as *mut c_void,
        Some(snd_ak4531_proc_read),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
