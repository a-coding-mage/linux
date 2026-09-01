// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for Sound Blaster mixer control
 */

use core::ffi::{c_char, c_int, c_ulong};

extern "C" {
    fn outb(value: u8, port: c_ulong);
    fn inb(port: c_ulong) -> u8;
    fn udelay(usecs: c_ulong);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_sb;
    fn snd_ctl_enum_info(
        uinfo: *mut snd_ctl_elem_info,
        channels: c_uint,
        items: c_uint,
        names: *const *const c_char,
    ) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut core::ffi::c_void)
        -> *mut snd_kcontrol;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_sbmixer_add_ctl_elem(chip: *mut snd_sb, elem: *const sbmix_elem) -> c_int;
    fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
    fn snd_BUG_ON(condition: bool) -> bool;
}

type c_uint = u32;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct snd_sb {
    pub card: *mut snd_card,
    pub mixer_lock: core::ffi::c_void,
    pub hardware: c_int,
    pub saved_regs: [u8; 64],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut core::ffi::c_void,
    pub mixername: [c_char; 80],
}

#[repr(C)]
pub struct snd_kcontrol_id {
    pub name: [c_char; 44],
    pub index: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_kcontrol_id,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
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
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_data,
}

#[repr(C)]
pub union snd_ctl_elem_value_data {
    pub integer: snd_ctl_elem_value_integer,
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbmix_elem {
    pub name: *const c_char,
    pub index: c_int,
    pub type_: c_int,
    pub private_value: c_ulong,
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! SB_SINGLE {
    ($name:literal, $reg:expr, $shift:expr, $mask:expr) => {
        sbmix_elem {
            name: c_str!($name),
            index: 0,
            type_: SB_MIX_SINGLE,
            private_value: (($reg as c_ulong) | (($shift as c_ulong) << 16) | (($mask as c_ulong) << 24)),
        }
    };
}

macro_rules! SB_DOUBLE {
    ($name:literal, $left_reg:expr, $right_reg:expr, $left_shift:expr, $right_shift:expr, $mask:expr) => {
        sbmix_elem {
            name: c_str!($name),
            index: 0,
            type_: SB_MIX_DOUBLE,
            private_value: (($left_reg as c_ulong)
                | (($right_reg as c_ulong) << 8)
                | (($left_shift as c_ulong) << 16)
                | (($right_shift as c_ulong) << 19)
                | (($mask as c_ulong) << 24)),
        }
    };
}

macro_rules! SB16_INPUT_SW {
    ($name:literal, $reg1:expr, $reg2:expr, $left_shift:expr, $right_shift:expr) => {
        sbmix_elem {
            name: c_str!($name),
            index: 0,
            type_: SB_MIX_INPUT_SW,
            private_value: (($reg1 as c_ulong)
                | (($reg2 as c_ulong) << 8)
                | (($left_shift as c_ulong) << 16)
                | (($right_shift as c_ulong) << 24)),
        }
    };
}

unsafe fn SBP(chip: *mut snd_sb, reg: c_int) -> c_ulong {
    crate::SBP(chip, reg)
}

pub unsafe extern "C" fn snd_sbmixer_write(chip: *mut snd_sb, reg: u8, data: u8) {
    outb(reg, SBP(chip, MIXER_ADDR));
    udelay(10);
    outb(data, SBP(chip, MIXER_DATA));
    udelay(10);
    /* IO_DEBUG: dev_dbg(chip->card->dev, "mixer_write 0x%x 0x%x\n", reg, data); */
}

pub unsafe extern "C" fn snd_sbmixer_read(chip: *mut snd_sb, reg: u8) -> u8 {
    let result: u8;

    outb(reg, SBP(chip, MIXER_ADDR));
    udelay(10);
    result = inb(SBP(chip, MIXER_DATA));
    udelay(10);
    /* IO_DEBUG: dev_dbg(chip->card->dev, "mixer_read 0x%x 0x%x\n", reg, result); */
    result
}

/*
 * Single channel mixer element
 */

unsafe extern "C" fn snd_sbmixer_info_single(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;

    (*uinfo).type_ = if mask == 1 {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN
    } else {
        SNDRV_CTL_ELEM_TYPE_INTEGER
    };
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as i64;
    0
}

unsafe extern "C" fn snd_sbmixer_get_single(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_int;
    let shift = (((*kcontrol).private_value >> 16) & 0xff) as c_int;
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let val: u8;

    /* guard(spinlock_irqsave)(&sb->mixer_lock); */
    val = ((snd_sbmixer_read(sb, reg as u8) as c_int >> shift) & mask) as u8;
    (*ucontrol).value.integer.value[0] = val as i64;
    0
}

unsafe extern "C" fn snd_sbmixer_put_single(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let reg = ((*kcontrol).private_value & 0xff) as c_int;
    let shift = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let change: c_int;
    let mut val: u8;
    let oval: u8;

    val = (((*ucontrol).value.integer.value[0] as c_int & mask) << shift) as u8;
    /* guard(spinlock_irqsave)(&sb->mixer_lock); */
    oval = snd_sbmixer_read(sb, reg as u8);
    val = (((oval as c_int) & !(mask << shift)) | val as c_int) as u8;
    change = (val != oval) as c_int;
    if change != 0 {
        snd_sbmixer_write(sb, reg as u8, val);
    }
    change
}

/*
 * Double channel mixer element
 */

unsafe extern "C" fn snd_sbmixer_info_double(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;

    (*uinfo).type_ = if mask == 1 {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN
    } else {
        SNDRV_CTL_ELEM_TYPE_INTEGER
    };
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as i64;
    0
}

unsafe extern "C" fn snd_sbmixer_get_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as c_int;
    let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let left_shift = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let right_shift = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let left: u8;
    let right: u8;

    /* guard(spinlock_irqsave)(&sb->mixer_lock); */
    left = ((snd_sbmixer_read(sb, left_reg as u8) as c_int >> left_shift) & mask) as u8;
    right = ((snd_sbmixer_read(sb, right_reg as u8) as c_int >> right_shift) & mask) as u8;
    (*ucontrol).value.integer.value[0] = left as i64;
    (*ucontrol).value.integer.value[1] = right as i64;
    0
}

unsafe extern "C" fn snd_sbmixer_put_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let left_reg = ((*kcontrol).private_value & 0xff) as c_int;
    let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let left_shift = (((*kcontrol).private_value >> 16) & 0x07) as c_int;
    let right_shift = (((*kcontrol).private_value >> 19) & 0x07) as c_int;
    let mask = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let change: c_int;
    let mut left: u8;
    let mut right: u8;
    let oleft: u8;
    let oright: u8;

    left = (((*ucontrol).value.integer.value[0] as c_int & mask) << left_shift) as u8;
    right = (((*ucontrol).value.integer.value[1] as c_int & mask) << right_shift) as u8;
    /* guard(spinlock_irqsave)(&sb->mixer_lock); */
    if left_reg == right_reg {
        oleft = snd_sbmixer_read(sb, left_reg as u8);
        left = (((oleft as c_int) & !((mask << left_shift) | (mask << right_shift)))
            | left as c_int
            | right as c_int) as u8;
        change = (left != oleft) as c_int;
        if change != 0 {
            snd_sbmixer_write(sb, left_reg as u8, left);
        }
    } else {
        oleft = snd_sbmixer_read(sb, left_reg as u8);
        oright = snd_sbmixer_read(sb, right_reg as u8);
        left = (((oleft as c_int) & !(mask << left_shift)) | left as c_int) as u8;
        right = (((oright as c_int) & !(mask << right_shift)) | right as c_int) as u8;
        change = (left != oleft || right != oright) as c_int;
        if change != 0 {
            snd_sbmixer_write(sb, left_reg as u8, left);
            snd_sbmixer_write(sb, right_reg as u8, right);
        }
    }
    change
}

/*
 * DT-019x / ALS-007 capture/input switch
 */

unsafe extern "C" fn snd_dt019x_input_sw_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 5] = [
        c_str!("CD"),
        c_str!("Mic"),
        c_str!("Line"),
        c_str!("Synth"),
        c_str!("Master"),
    ];

    snd_ctl_enum_info(uinfo, 1, 5, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_dt019x_input_sw_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let oval: u8;

    /* scoped_guard(spinlock_irqsave, &sb->mixer_lock) */
    oval = snd_sbmixer_read(sb, SB_DT019X_CAPTURE_SW);
    match oval & 0x07 {
        SB_DT019X_CAP_CD => (*ucontrol).value.enumerated.item[0] = 0,
        SB_DT019X_CAP_MIC => (*ucontrol).value.enumerated.item[0] = 1,
        SB_DT019X_CAP_LINE => (*ucontrol).value.enumerated.item[0] = 2,
        SB_DT019X_CAP_MAIN => (*ucontrol).value.enumerated.item[0] = 4,
        /*
         * To record the synth on these cards you must record the main.
         * Thus SB_DT019X_CAP_SYNTH == SB_DT019X_CAP_MAIN and would cause
         * duplicate case labels if left uncommented.
         */
        _ => (*ucontrol).value.enumerated.item[0] = 4,
    }
    0
}

unsafe extern "C" fn snd_dt019x_input_sw_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let change: c_int;
    let nval: u8;
    let oval: u8;

    if (*ucontrol).value.enumerated.item[0] > 4 {
        return -EINVAL;
    }
    match (*ucontrol).value.enumerated.item[0] {
        0 => nval = SB_DT019X_CAP_CD,
        1 => nval = SB_DT019X_CAP_MIC,
        2 => nval = SB_DT019X_CAP_LINE,
        3 => nval = SB_DT019X_CAP_SYNTH,
        4 => nval = SB_DT019X_CAP_MAIN,
        _ => nval = SB_DT019X_CAP_MAIN,
    }
    /* guard(spinlock_irqsave)(&sb->mixer_lock); */
    oval = snd_sbmixer_read(sb, SB_DT019X_CAPTURE_SW);
    change = (nval != oval) as c_int;
    if change != 0 {
        snd_sbmixer_write(sb, SB_DT019X_CAPTURE_SW, nval);
    }
    change
}

/*
 * ALS4000 mono recording control switch
 */

unsafe extern "C" fn snd_als4k_mono_capture_route_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 3] = [
        c_str!("L chan only"),
        c_str!("R chan only"),
        c_str!("L ch/2 + R ch/2"),
    ];

    snd_ctl_enum_info(uinfo, 1, 3, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_als4k_mono_capture_route_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let mut oval: u8;

    /* guard(spinlock_irqsave)(&sb->mixer_lock); */
    oval = snd_sbmixer_read(sb, SB_ALS4000_MONO_IO_CTRL);
    oval >>= 6;
    if oval > 2 {
        oval = 2;
    }

    (*ucontrol).value.enumerated.item[0] = oval as c_uint;
    0
}

unsafe extern "C" fn snd_als4k_mono_capture_route_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let change: c_int;
    let nval: u8;
    let oval: u8;

    if (*ucontrol).value.enumerated.item[0] > 2 {
        return -EINVAL;
    }
    /* guard(spinlock_irqsave)(&sb->mixer_lock); */
    oval = snd_sbmixer_read(sb, SB_ALS4000_MONO_IO_CTRL);

    nval = (((oval as c_int) & !(3 << 6))
        | (((*ucontrol).value.enumerated.item[0] as c_int) << 6)) as u8;
    change = (nval != oval) as c_int;
    if change != 0 {
        snd_sbmixer_write(sb, SB_ALS4000_MONO_IO_CTRL, nval);
    }
    change
}

/*
 * SBPRO input multiplexer
 */

unsafe extern "C" fn snd_sb8mixer_info_mux(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 3] = [c_str!("Mic"), c_str!("CD"), c_str!("Line")];

    snd_ctl_enum_info(uinfo, 1, 3, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_sb8mixer_get_mux(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let oval: u8;

    /* guard(spinlock_irqsave)(&sb->mixer_lock); */
    oval = snd_sbmixer_read(sb, SB_DSP_CAPTURE_SOURCE);
    match (oval >> 0x01) & 0x03 {
        SB_DSP_MIXS_CD => (*ucontrol).value.enumerated.item[0] = 1,
        SB_DSP_MIXS_LINE => (*ucontrol).value.enumerated.item[0] = 2,
        _ => (*ucontrol).value.enumerated.item[0] = 0,
    }
    0
}

unsafe extern "C" fn snd_sb8mixer_put_mux(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let change: c_int;
    let mut nval: u8;
    let oval: u8;

    if (*ucontrol).value.enumerated.item[0] > 2 {
        return -EINVAL;
    }
    match (*ucontrol).value.enumerated.item[0] {
        1 => nval = SB_DSP_MIXS_CD,
        2 => nval = SB_DSP_MIXS_LINE,
        _ => nval = SB_DSP_MIXS_MIC,
    }
    nval <<= 1;
    /* guard(spinlock_irqsave)(&sb->mixer_lock); */
    oval = snd_sbmixer_read(sb, SB_DSP_CAPTURE_SOURCE);
    nval |= oval & !0x06;
    change = (nval != oval) as c_int;
    if change != 0 {
        snd_sbmixer_write(sb, SB_DSP_CAPTURE_SOURCE, nval);
    }
    change
}

/*
 * SB16 input switch
 */

unsafe extern "C" fn snd_sb16mixer_info_input_sw(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 4;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe extern "C" fn snd_sb16mixer_get_input_sw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let reg1 = ((*kcontrol).private_value & 0xff) as c_int;
    let reg2 = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let left_shift = (((*kcontrol).private_value >> 16) & 0x0f) as c_int;
    let right_shift = (((*kcontrol).private_value >> 24) & 0x0f) as c_int;
    let val1: u8;
    let val2: u8;

    /* guard(spinlock_irqsave)(&sb->mixer_lock); */
    val1 = snd_sbmixer_read(sb, reg1 as u8);
    val2 = snd_sbmixer_read(sb, reg2 as u8);
    (*ucontrol).value.integer.value[0] = ((val1 as c_int >> left_shift) & 0x01) as i64;
    (*ucontrol).value.integer.value[1] = ((val2 as c_int >> left_shift) & 0x01) as i64;
    (*ucontrol).value.integer.value[2] = ((val1 as c_int >> right_shift) & 0x01) as i64;
    (*ucontrol).value.integer.value[3] = ((val2 as c_int >> right_shift) & 0x01) as i64;
    0
}

unsafe extern "C" fn snd_sb16mixer_put_input_sw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let sb = snd_kcontrol_chip(kcontrol);
    let reg1 = ((*kcontrol).private_value & 0xff) as c_int;
    let reg2 = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let left_shift = (((*kcontrol).private_value >> 16) & 0x0f) as c_int;
    let right_shift = (((*kcontrol).private_value >> 24) & 0x0f) as c_int;
    let change: c_int;
    let mut val1: u8;
    let mut val2: u8;
    let oval1: u8;
    let oval2: u8;

    /* guard(spinlock_irqsave)(&sb->mixer_lock); */
    oval1 = snd_sbmixer_read(sb, reg1 as u8);
    oval2 = snd_sbmixer_read(sb, reg2 as u8);
    val1 = ((oval1 as c_int) & !((1 << left_shift) | (1 << right_shift))) as u8;
    val2 = ((oval2 as c_int) & !((1 << left_shift) | (1 << right_shift))) as u8;
    val1 |= (((*ucontrol).value.integer.value[0] as c_int & 1) << left_shift) as u8;
    val2 |= (((*ucontrol).value.integer.value[1] as c_int & 1) << left_shift) as u8;
    val1 |= (((*ucontrol).value.integer.value[2] as c_int & 1) << right_shift) as u8;
    val2 |= (((*ucontrol).value.integer.value[3] as c_int & 1) << right_shift) as u8;
    change = (val1 != oval1 || val2 != oval2) as c_int;
    if change != 0 {
        snd_sbmixer_write(sb, reg1 as u8, val1);
        snd_sbmixer_write(sb, reg2 as u8, val2);
    }
    change
}

pub unsafe extern "C" fn snd_sbmixer_add_ctl(
    chip: *mut snd_sb,
    name: *const c_char,
    index: c_int,
    type_: c_int,
    value: c_ulong,
) -> c_int {
    static NEWCTLS: [snd_kcontrol_new; 6] = [
        snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, info: Some(snd_sbmixer_info_single), get: Some(snd_sbmixer_get_single), put: Some(snd_sbmixer_put_single) },
        snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, info: Some(snd_sbmixer_info_double), get: Some(snd_sbmixer_get_double), put: Some(snd_sbmixer_put_double) },
        snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, info: Some(snd_sb16mixer_info_input_sw), get: Some(snd_sb16mixer_get_input_sw), put: Some(snd_sb16mixer_put_input_sw) },
        snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, info: Some(snd_sb8mixer_info_mux), get: Some(snd_sb8mixer_get_mux), put: Some(snd_sb8mixer_put_mux) },
        snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, info: Some(snd_dt019x_input_sw_info), get: Some(snd_dt019x_input_sw_get), put: Some(snd_dt019x_input_sw_put) },
        snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, info: Some(snd_als4k_mono_capture_route_info), get: Some(snd_als4k_mono_capture_route_get), put: Some(snd_als4k_mono_capture_route_put) },
    ];
    let ctl: *mut snd_kcontrol;
    let err: c_int;

    ctl = snd_ctl_new1(&NEWCTLS[type_ as usize], chip as *mut core::ffi::c_void);
    if ctl.is_null() {
        return -ENOMEM;
    }
    strscpy((*ctl).id.name.as_mut_ptr(), name, (*ctl).id.name.len());
    (*ctl).id.index = index as c_uint;
    (*ctl).private_value = value;
    err = snd_ctl_add((*chip).card, ctl);
    if err < 0 {
        return err;
    }
    0
}

/*
 * SB 2.0 specific mixer elements
 */

static SND_SB20_CONTROLS: [sbmix_elem; 4] = [
    SB_SINGLE!("Master Playback Volume", SB_DSP20_MASTER_DEV, 1, 7),
    SB_SINGLE!("PCM Playback Volume", SB_DSP20_PCM_DEV, 1, 3),
    SB_SINGLE!("Synth Playback Volume", SB_DSP20_FM_DEV, 1, 7),
    SB_SINGLE!("CD Playback Volume", SB_DSP20_CD_DEV, 1, 7),
];

static SND_SB20_INIT_VALUES: [[u8; 2]; 2] = [[SB_DSP20_MASTER_DEV, 0], [SB_DSP20_FM_DEV, 0]];

/*
 * SB Pro specific mixer elements
 */
static SND_SBPRO_CONTROLS: [sbmix_elem; 10] = [
    SB_DOUBLE!("Master Playback Volume", SB_DSP_MASTER_DEV, SB_DSP_MASTER_DEV, 5, 1, 7),
    SB_DOUBLE!("PCM Playback Volume", SB_DSP_PCM_DEV, SB_DSP_PCM_DEV, 5, 1, 7),
    SB_SINGLE!("PCM Playback Filter", SB_DSP_PLAYBACK_FILT, 5, 1),
    SB_DOUBLE!("Synth Playback Volume", SB_DSP_FM_DEV, SB_DSP_FM_DEV, 5, 1, 7),
    SB_DOUBLE!("CD Playback Volume", SB_DSP_CD_DEV, SB_DSP_CD_DEV, 5, 1, 7),
    SB_DOUBLE!("Line Playback Volume", SB_DSP_LINE_DEV, SB_DSP_LINE_DEV, 5, 1, 7),
    SB_SINGLE!("Mic Playback Volume", SB_DSP_MIC_DEV, 1, 3),
    sbmix_elem { name: c_str!("Capture Source"), index: 0, type_: SB_MIX_CAPTURE_PRO, private_value: 0 },
    SB_SINGLE!("Capture Filter", SB_DSP_CAPTURE_FILT, 5, 1),
    SB_SINGLE!("Capture Low-Pass Filter", SB_DSP_CAPTURE_FILT, 3, 1),
];

static SND_SBPRO_INIT_VALUES: [[u8; 2]; 3] =
    [[SB_DSP_MASTER_DEV, 0], [SB_DSP_PCM_DEV, 0], [SB_DSP_FM_DEV, 0]];

/*
 * SB16 specific mixer elements
 */
static SND_SB16_CONTROLS: [sbmix_elem; 20] = [
    SB_DOUBLE!("Master Playback Volume", SB_DSP4_MASTER_DEV, SB_DSP4_MASTER_DEV + 1, 3, 3, 31),
    SB_DOUBLE!("PCM Playback Volume", SB_DSP4_PCM_DEV, SB_DSP4_PCM_DEV + 1, 3, 3, 31),
    SB16_INPUT_SW!("Synth Capture Route", SB_DSP4_INPUT_LEFT, SB_DSP4_INPUT_RIGHT, 6, 5),
    SB_DOUBLE!("Synth Playback Volume", SB_DSP4_SYNTH_DEV, SB_DSP4_SYNTH_DEV + 1, 3, 3, 31),
    SB16_INPUT_SW!("CD Capture Route", SB_DSP4_INPUT_LEFT, SB_DSP4_INPUT_RIGHT, 2, 1),
    SB_DOUBLE!("CD Playback Switch", SB_DSP4_OUTPUT_SW, SB_DSP4_OUTPUT_SW, 2, 1, 1),
    SB_DOUBLE!("CD Playback Volume", SB_DSP4_CD_DEV, SB_DSP4_CD_DEV + 1, 3, 3, 31),
    SB16_INPUT_SW!("Mic Capture Route", SB_DSP4_INPUT_LEFT, SB_DSP4_INPUT_RIGHT, 0, 0),
    SB_SINGLE!("Mic Playback Switch", SB_DSP4_OUTPUT_SW, 0, 1),
    SB_SINGLE!("Mic Playback Volume", SB_DSP4_MIC_DEV, 3, 31),
    SB_SINGLE!("Beep Volume", SB_DSP4_SPEAKER_DEV, 6, 3),
    SB_DOUBLE!("Capture Volume", SB_DSP4_IGAIN_DEV, SB_DSP4_IGAIN_DEV + 1, 6, 6, 3),
    SB_DOUBLE!("Playback Volume", SB_DSP4_OGAIN_DEV, SB_DSP4_OGAIN_DEV + 1, 6, 6, 3),
    SB16_INPUT_SW!("Line Capture Route", SB_DSP4_INPUT_LEFT, SB_DSP4_INPUT_RIGHT, 4, 3),
    SB_DOUBLE!("Line Playback Switch", SB_DSP4_OUTPUT_SW, SB_DSP4_OUTPUT_SW, 4, 3, 1),
    SB_DOUBLE!("Line Playback Volume", SB_DSP4_LINE_DEV, SB_DSP4_LINE_DEV + 1, 3, 3, 31),
    SB_SINGLE!("Mic Auto Gain", SB_DSP4_MIC_AGC, 0, 1),
    SB_SINGLE!("3D Enhancement Switch", SB_DSP4_3DSE, 0, 1),
    SB_DOUBLE!("Tone Control - Bass", SB_DSP4_BASS_DEV, SB_DSP4_BASS_DEV + 1, 4, 4, 15),
    SB_DOUBLE!("Tone Control - Treble", SB_DSP4_TREBLE_DEV, SB_DSP4_TREBLE_DEV + 1, 4, 4, 15),
];

static SND_SB16_INIT_VALUES: [[u8; 2]; 10] = [
    [SB_DSP4_MASTER_DEV + 0, 0], [SB_DSP4_MASTER_DEV + 1, 0],
    [SB_DSP4_PCM_DEV + 0, 0], [SB_DSP4_PCM_DEV + 1, 0],
    [SB_DSP4_SYNTH_DEV + 0, 0], [SB_DSP4_SYNTH_DEV + 1, 0],
    [SB_DSP4_INPUT_LEFT, 0], [SB_DSP4_INPUT_RIGHT, 0],
    [SB_DSP4_OUTPUT_SW, 0], [SB_DSP4_SPEAKER_DEV, 0],
];

/*
 * DT019x specific mixer elements
 */
static SND_DT019X_CONTROLS: [sbmix_elem; 13] = [
    /* ALS4000 below has some parts which we might be lacking,
     * e.g. snd_als4000_ctl_mono_playback_switch - check it! */
    SB_DOUBLE!("Master Playback Volume", SB_DT019X_MASTER_DEV, SB_DT019X_MASTER_DEV, 4, 0, 15),
    SB_DOUBLE!("PCM Playback Switch", SB_DT019X_OUTPUT_SW2, SB_DT019X_OUTPUT_SW2, 2, 1, 1),
    SB_DOUBLE!("PCM Playback Volume", SB_DT019X_PCM_DEV, SB_DT019X_PCM_DEV, 4, 0, 15),
    SB_DOUBLE!("Synth Playback Switch", SB_DT019X_OUTPUT_SW2, SB_DT019X_OUTPUT_SW2, 4, 3, 1),
    SB_DOUBLE!("Synth Playback Volume", SB_DT019X_SYNTH_DEV, SB_DT019X_SYNTH_DEV, 4, 0, 15),
    SB_DOUBLE!("CD Playback Switch", SB_DSP4_OUTPUT_SW, SB_DSP4_OUTPUT_SW, 2, 1, 1),
    SB_DOUBLE!("CD Playback Volume", SB_DT019X_CD_DEV, SB_DT019X_CD_DEV, 4, 0, 15),
    SB_SINGLE!("Mic Playback Switch", SB_DSP4_OUTPUT_SW, 0, 1),
    SB_SINGLE!("Mic Playback Volume", SB_DT019X_MIC_DEV, 4, 7),
    SB_SINGLE!("Beep Volume", SB_DT019X_SPKR_DEV, 0, 7),
    SB_DOUBLE!("Line Playback Switch", SB_DSP4_OUTPUT_SW, SB_DSP4_OUTPUT_SW, 4, 3, 1),
    SB_DOUBLE!("Line Playback Volume", SB_DT019X_LINE_DEV, SB_DT019X_LINE_DEV, 4, 0, 15),
    sbmix_elem { name: c_str!("Capture Source"), index: 0, type_: SB_MIX_CAPTURE_DT019X, private_value: 0 },
];

static SND_DT019X_INIT_VALUES: [[u8; 2]; 9] = [
    [SB_DT019X_MASTER_DEV, 0], [SB_DT019X_PCM_DEV, 0], [SB_DT019X_SYNTH_DEV, 0],
    [SB_DT019X_CD_DEV, 0], [SB_DT019X_MIC_DEV, 0], /* Includes PC-speaker in high nibble */
    [SB_DT019X_LINE_DEV, 0], [SB_DSP4_OUTPUT_SW, 0], [SB_DT019X_OUTPUT_SW2, 0],
    [SB_DT019X_CAPTURE_SW, 0x06],
];

/*
 * ALS4000 specific mixer elements
 */
static SND_ALS4000_CONTROLS: [sbmix_elem; 14] = [
    SB_DOUBLE!("PCM Playback Switch", SB_DT019X_OUTPUT_SW2, SB_DT019X_OUTPUT_SW2, 2, 1, 1),
    SB_DOUBLE!("Synth Playback Switch", SB_DT019X_OUTPUT_SW2, SB_DT019X_OUTPUT_SW2, 4, 3, 1),
    SB_SINGLE!("Mic Boost (+20dB)", SB_ALS4000_MIC_IN_GAIN, 0, 0x03),
    SB_SINGLE!("Master Mono Playback Switch", SB_ALS4000_MONO_IO_CTRL, 5, 1),
    sbmix_elem { name: c_str!("Master Mono Capture Route"), index: 0, type_: SB_MIX_MONO_CAPTURE_ALS4K, private_value: 0 },
    SB_SINGLE!("Mono Playback Switch", SB_DT019X_OUTPUT_SW2, 0, 1),
    SB_SINGLE!("Analog Loopback Switch", SB_ALS4000_MIC_IN_GAIN, 7, 0x01),
    SB_SINGLE!("3D Control - Switch", SB_ALS4000_3D_SND_FX, 6, 0x01),
    SB_SINGLE!("Digital Loopback Switch", SB_ALS4000_CR3_CONFIGURATION, 7, 0x01),
    /* FIXME: functionality of 3D controls might be swapped, I didn't find
     * a description of how to identify what is supposed to be what */
    SB_SINGLE!("3D Control - Level", SB_ALS4000_3D_SND_FX, 0, 0x07),
    /* FIXME: maybe there's actually some standard 3D ctrl name for it?? */
    SB_SINGLE!("3D Control - Freq", SB_ALS4000_3D_SND_FX, 4, 0x03),
    /* FIXME: ALS4000a.pdf mentions BBD (Bucket Brigade Device) time delay,
     * but what ALSA 3D attribute is that actually? "Center", "Depth",
     * "Wide" or "Space" or even "Level"? Assuming "Wide" for now... */
    SB_SINGLE!("3D Control - Wide", SB_ALS4000_3D_TIME_DELAY, 0, 0x0f),
    SB_SINGLE!("3D PowerOff Switch", SB_ALS4000_3D_TIME_DELAY, 4, 0x01),
    SB_SINGLE!("Master Playback 8kHz / 20kHz LPF Switch", SB_ALS4000_FMDAC, 5, 0x01),
    /* NOT_AVAILABLE:
     * SB_SINGLE("FMDAC Switch (Option ?)", SB_ALS4000_FMDAC, 0, 0x01),
     * SB_SINGLE("QSound Mode", SB_ALS4000_QSOUND, 1, 0x1f),
     */
];

static SND_ALS4000_INIT_VALUES: [[u8; 2]; 12] = [
    [SB_DSP4_MASTER_DEV + 0, 0], [SB_DSP4_MASTER_DEV + 1, 0],
    [SB_DSP4_PCM_DEV + 0, 0], [SB_DSP4_PCM_DEV + 1, 0],
    [SB_DSP4_SYNTH_DEV + 0, 0], [SB_DSP4_SYNTH_DEV + 1, 0],
    [SB_DSP4_SPEAKER_DEV, 0], [SB_DSP4_OUTPUT_SW, 0],
    [SB_DSP4_INPUT_LEFT, 0], [SB_DSP4_INPUT_RIGHT, 0],
    [SB_DT019X_OUTPUT_SW2, 0], [SB_ALS4000_MIC_IN_GAIN, 0],
];

unsafe extern "C" fn snd_sbmixer_init(
    chip: *mut snd_sb,
    controls: *const sbmix_elem,
    controls_count: c_int,
    map: *const [u8; 2],
    map_count: c_int,
    name: *mut c_char,
) -> c_int {
    let card = (*chip).card;
    let mut idx: c_int;
    let mut err: c_int;

    /* mixer reset */
    /* scoped_guard(spinlock_irqsave, &chip->mixer_lock) */
    snd_sbmixer_write(chip, 0x00, 0x00);

    /* mute and zero volume channels */
    idx = 0;
    while idx < map_count {
        /* guard(spinlock_irqsave)(&chip->mixer_lock); */
        snd_sbmixer_write(chip, (*map.add(idx as usize))[0], (*map.add(idx as usize))[1]);
        idx += 1;
    }

    idx = 0;
    while idx < controls_count {
        err = snd_sbmixer_add_ctl_elem(chip, controls.add(idx as usize));
        if err < 0 {
            return err;
        }
        idx += 1;
    }
    snd_component_add(card, name);
    strscpy((*card).mixername.as_mut_ptr(), name, (*card).mixername.len());
    0
}

pub unsafe extern "C" fn snd_sbmixer_new(chip: *mut snd_sb) -> c_int {
    let card: *mut snd_card;
    let mut err: c_int;

    if snd_BUG_ON(chip.is_null() || (*chip).card.is_null()) {
        return -EINVAL;
    }

    card = (*chip).card;

    match (*chip).hardware {
        SB_HW_10 => return 0, /* no mixer chip on SB1.x */
        SB_HW_20 | SB_HW_201 => {
            err = snd_sbmixer_init(chip, SND_SB20_CONTROLS.as_ptr(), SND_SB20_CONTROLS.len() as c_int, SND_SB20_INIT_VALUES.as_ptr(), SND_SB20_INIT_VALUES.len() as c_int, c_str!("CTL1335") as *mut c_char);
            if err < 0 { return err; }
        }
        SB_HW_PRO | SB_HW_JAZZ16 => {
            err = snd_sbmixer_init(chip, SND_SBPRO_CONTROLS.as_ptr(), SND_SBPRO_CONTROLS.len() as c_int, SND_SBPRO_INIT_VALUES.as_ptr(), SND_SBPRO_INIT_VALUES.len() as c_int, c_str!("CTL1345") as *mut c_char);
            if err < 0 { return err; }
        }
        SB_HW_16 | SB_HW_ALS100 | SB_HW_CS5530 => {
            err = snd_sbmixer_init(chip, SND_SB16_CONTROLS.as_ptr(), SND_SB16_CONTROLS.len() as c_int, SND_SB16_INIT_VALUES.as_ptr(), SND_SB16_INIT_VALUES.len() as c_int, c_str!("CTL1745") as *mut c_char);
            if err < 0 { return err; }
        }
        SB_HW_ALS4000 => {
            /* use only the first 16 controls from SB16 */
            err = snd_sbmixer_init(chip, SND_SB16_CONTROLS.as_ptr(), 16, SND_SB16_INIT_VALUES.as_ptr(), SND_SB16_INIT_VALUES.len() as c_int, c_str!("ALS4000") as *mut c_char);
            if err < 0 { return err; }
            err = snd_sbmixer_init(chip, SND_ALS4000_CONTROLS.as_ptr(), SND_ALS4000_CONTROLS.len() as c_int, SND_ALS4000_INIT_VALUES.as_ptr(), SND_ALS4000_INIT_VALUES.len() as c_int, c_str!("ALS4000") as *mut c_char);
            if err < 0 { return err; }
        }
        SB_HW_DT019X => {
            err = snd_sbmixer_init(chip, SND_DT019X_CONTROLS.as_ptr(), SND_DT019X_CONTROLS.len() as c_int, SND_DT019X_INIT_VALUES.as_ptr(), SND_DT019X_INIT_VALUES.len() as c_int, c_str!("DT019X") as *mut c_char);
            if err < 0 { return err; }
        }
        _ => {
            strscpy((*card).mixername.as_mut_ptr(), c_str!("???"), (*card).mixername.len());
        }
    }
    0
}

/* CONFIG_PM */
static SB20_SAVED_REGS: [u8; 4] = [SB_DSP20_MASTER_DEV, SB_DSP20_PCM_DEV, SB_DSP20_FM_DEV, SB_DSP20_CD_DEV];

static SBPRO_SAVED_REGS: [u8; 9] = [
    SB_DSP_MASTER_DEV, SB_DSP_PCM_DEV, SB_DSP_PLAYBACK_FILT, SB_DSP_FM_DEV,
    SB_DSP_CD_DEV, SB_DSP_LINE_DEV, SB_DSP_MIC_DEV, SB_DSP_CAPTURE_SOURCE,
    SB_DSP_CAPTURE_FILT,
];

static SB16_SAVED_REGS: [u8; 25] = [
    SB_DSP4_MASTER_DEV, SB_DSP4_MASTER_DEV + 1, SB_DSP4_3DSE,
    SB_DSP4_BASS_DEV, SB_DSP4_BASS_DEV + 1, SB_DSP4_TREBLE_DEV, SB_DSP4_TREBLE_DEV + 1,
    SB_DSP4_PCM_DEV, SB_DSP4_PCM_DEV + 1, SB_DSP4_INPUT_LEFT, SB_DSP4_INPUT_RIGHT,
    SB_DSP4_SYNTH_DEV, SB_DSP4_SYNTH_DEV + 1, SB_DSP4_OUTPUT_SW,
    SB_DSP4_CD_DEV, SB_DSP4_CD_DEV + 1, SB_DSP4_LINE_DEV, SB_DSP4_LINE_DEV + 1,
    SB_DSP4_MIC_DEV, SB_DSP4_SPEAKER_DEV, SB_DSP4_IGAIN_DEV, SB_DSP4_IGAIN_DEV + 1,
    SB_DSP4_OGAIN_DEV, SB_DSP4_OGAIN_DEV + 1, SB_DSP4_MIC_AGC,
];

static DT019X_SAVED_REGS: [u8; 10] = [
    SB_DT019X_MASTER_DEV, SB_DT019X_PCM_DEV, SB_DT019X_SYNTH_DEV, SB_DT019X_CD_DEV,
    SB_DT019X_MIC_DEV, SB_DT019X_SPKR_DEV, SB_DT019X_LINE_DEV, SB_DSP4_OUTPUT_SW,
    SB_DT019X_OUTPUT_SW2, SB_DT019X_CAPTURE_SW,
];

static ALS4000_SAVED_REGS: [u8; 24] = [
    /* please verify in dsheet whether regs to be added
       are actually real H/W or just dummy */
    SB_DSP4_MASTER_DEV, SB_DSP4_MASTER_DEV + 1, SB_DSP4_OUTPUT_SW,
    SB_DSP4_PCM_DEV, SB_DSP4_PCM_DEV + 1, SB_DSP4_INPUT_LEFT, SB_DSP4_INPUT_RIGHT,
    SB_DSP4_SYNTH_DEV, SB_DSP4_SYNTH_DEV + 1, SB_DSP4_CD_DEV, SB_DSP4_CD_DEV + 1,
    SB_DSP4_MIC_DEV, SB_DSP4_SPEAKER_DEV, SB_DSP4_IGAIN_DEV, SB_DSP4_IGAIN_DEV + 1,
    SB_DSP4_OGAIN_DEV, SB_DSP4_OGAIN_DEV + 1, SB_DT019X_OUTPUT_SW2,
    SB_ALS4000_MONO_IO_CTRL, SB_ALS4000_MIC_IN_GAIN, SB_ALS4000_FMDAC,
    SB_ALS4000_3D_SND_FX, SB_ALS4000_3D_TIME_DELAY, SB_ALS4000_CR3_CONFIGURATION,
];

unsafe extern "C" fn save_mixer(chip: *mut snd_sb, mut regs: *const u8, mut num_regs: c_int) {
    let mut val = (*chip).saved_regs.as_mut_ptr();
    if snd_BUG_ON(num_regs as usize > (*chip).saved_regs.len()) {
        return;
    }
    while num_regs != 0 {
        *val = snd_sbmixer_read(chip, *regs);
        val = val.add(1);
        regs = regs.add(1);
        num_regs -= 1;
    }
}

unsafe extern "C" fn restore_mixer(chip: *mut snd_sb, mut regs: *const u8, mut num_regs: c_int) {
    let mut val = (*chip).saved_regs.as_mut_ptr();
    if snd_BUG_ON(num_regs as usize > (*chip).saved_regs.len()) {
        return;
    }
    while num_regs != 0 {
        snd_sbmixer_write(chip, *regs, *val);
        regs = regs.add(1);
        val = val.add(1);
        num_regs -= 1;
    }
}

pub unsafe extern "C" fn snd_sbmixer_suspend(chip: *mut snd_sb) {
    match (*chip).hardware {
        SB_HW_20 | SB_HW_201 => save_mixer(chip, SB20_SAVED_REGS.as_ptr(), SB20_SAVED_REGS.len() as c_int),
        SB_HW_PRO | SB_HW_JAZZ16 => save_mixer(chip, SBPRO_SAVED_REGS.as_ptr(), SBPRO_SAVED_REGS.len() as c_int),
        SB_HW_16 | SB_HW_ALS100 | SB_HW_CS5530 => save_mixer(chip, SB16_SAVED_REGS.as_ptr(), SB16_SAVED_REGS.len() as c_int),
        SB_HW_ALS4000 => save_mixer(chip, ALS4000_SAVED_REGS.as_ptr(), ALS4000_SAVED_REGS.len() as c_int),
        SB_HW_DT019X => save_mixer(chip, DT019X_SAVED_REGS.as_ptr(), DT019X_SAVED_REGS.len() as c_int),
        _ => {}
    }
}

pub unsafe extern "C" fn snd_sbmixer_resume(chip: *mut snd_sb) {
    match (*chip).hardware {
        SB_HW_20 | SB_HW_201 => restore_mixer(chip, SB20_SAVED_REGS.as_ptr(), SB20_SAVED_REGS.len() as c_int),
        SB_HW_PRO | SB_HW_JAZZ16 => restore_mixer(chip, SBPRO_SAVED_REGS.as_ptr(), SBPRO_SAVED_REGS.len() as c_int),
        SB_HW_16 | SB_HW_ALS100 | SB_HW_CS5530 => restore_mixer(chip, SB16_SAVED_REGS.as_ptr(), SB16_SAVED_REGS.len() as c_int),
        SB_HW_ALS4000 => restore_mixer(chip, ALS4000_SAVED_REGS.as_ptr(), ALS4000_SAVED_REGS.len() as c_int),
        SB_HW_DT019X => restore_mixer(chip, DT019X_SAVED_REGS.as_ptr(), DT019X_SAVED_REGS.len() as c_int),
        _ => {}
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
