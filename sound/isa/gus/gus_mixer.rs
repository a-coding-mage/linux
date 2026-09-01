// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Routines for control of ICS 2101 chip and "mixer" in GF1 chip
 */

/*
 * Original C dependencies:
 * <linux/time.h>
 * <linux/wait.h>
 * <sound/core.h>
 * <sound/control.h>
 * <sound/gus.h>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_uchar, c_void};

const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;
const EINVAL: c_int = 22;

const SNDRV_ICS_MASTER_DEV: c_int = 0;
const SNDRV_ICS_GF1_DEV: c_int = 1;
const SNDRV_ICS_LINE_DEV: c_int = 2;
const SNDRV_ICS_MIC_DEV: c_int = 3;
const SNDRV_ICS_CD_DEV: c_int = 4;

const MIXCNTRLREG: c_int = 0;
const GF1PAGE: c_int = 0;
const MIXCNTRLPORT: c_int = 0;
const MIXDATAPORT: c_int = 0;

#[repr(C)]
pub struct snd_card {
    pub mixername: [c_char; 80],
}

#[repr(C)]
pub struct snd_gf1 {
    pub active_voice: c_uchar,
    pub ics_regs: [[c_uchar; 2]; 256],
}

#[repr(C)]
pub struct snd_gus_card {
    pub card: *mut snd_card,
    pub reg_lock: c_void,
    pub mix_cntrl_reg: c_uchar,
    pub gf1: snd_gf1,
    pub ics_flag: c_int,
    pub ics_flipped: c_int,
    pub ess_flag: c_int,
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
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub index: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

unsafe extern "C" {
    fn snd_ctl_boolean_mono_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_gus_card;
    fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn outb(value: c_uchar, port: c_ulong);
    fn GUSP(gus: *mut snd_gus_card, reg: c_int) -> c_ulong;
}

unsafe fn snd_BUG_ON(cond: bool) -> bool {
    cond
}

/*
#define GF1_SINGLE(xname, xindex, shift, invert) \
{ .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, .index = xindex, \
  .info = snd_gf1_info_single, \
  .get = snd_gf1_get_single, .put = snd_gf1_put_single, \
  .private_value = shift | (invert << 8) }
*/
macro_rules! GF1_SINGLE {
    ($xname:expr, $xindex:expr, $shift:expr, $invert:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            index: $xindex,
            info: Some(snd_gf1_info_single),
            get: Some(snd_gf1_get_single),
            put: Some(snd_gf1_put_single),
            private_value: ($shift | ($invert << 8)) as c_ulong,
        }
    };
}

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        $array.len()
    };
}

unsafe extern "C" fn snd_gf1_info_single(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe { snd_ctl_boolean_mono_info(kcontrol, uinfo) }
}

unsafe extern "C" fn snd_gf1_get_single(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let gus: *mut snd_gus_card = unsafe { snd_kcontrol_chip(kcontrol) };
    let shift: c_int = unsafe { ((*kcontrol).private_value & 0xff) as c_int };
    let invert: c_int = unsafe { (((*kcontrol).private_value >> 8) & 1) as c_int };

    unsafe {
        (*ucontrol).value.integer.value[0] = (((*gus).mix_cntrl_reg >> shift) & 1) as c_long;
        if invert != 0 {
            (*ucontrol).value.integer.value[0] ^= 1;
        }
    }
    0
}

unsafe extern "C" fn snd_gf1_put_single(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let gus: *mut snd_gus_card = unsafe { snd_kcontrol_chip(kcontrol) };
    let shift: c_int = unsafe { ((*kcontrol).private_value & 0xff) as c_int };
    let invert: c_int = unsafe { (((*kcontrol).private_value >> 8) & 1) as c_int };
    let change: c_int;
    let oval: c_uchar;
    let mut nval: c_uchar;

    unsafe {
        nval = ((*ucontrol).value.integer.value[0] & 1) as c_uchar;
    }
    if invert != 0 {
        nval ^= 1;
    }
    nval = nval.wrapping_shl(shift as u32);
    /* Original C uses guard(spinlock_irqsave)(&gus->reg_lock). */
    unsafe {
        oval = (*gus).mix_cntrl_reg;
        nval = (oval & !(1u8.wrapping_shl(shift as u32))) | nval;
        change = (nval != oval) as c_int;
        (*gus).mix_cntrl_reg = nval;
        outb((*gus).mix_cntrl_reg, GUSP(gus, MIXCNTRLREG));
        (*gus).gf1.active_voice = 0;
        outb((*gus).gf1.active_voice, GUSP(gus, GF1PAGE));
    }
    change
}

/*
#define ICS_DOUBLE(xname, xindex, addr) \
{ .iface = SNDRV_CTL_ELEM_IFACE_MIXER, .name = xname, .index = xindex, \
  .info = snd_ics_info_double, \
  .get = snd_ics_get_double, .put = snd_ics_put_double, \
  .private_value = addr }
*/
macro_rules! ICS_DOUBLE {
    ($xname:expr, $xindex:expr, $addr:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: concat!($xname, "\0").as_ptr() as *const c_char,
            index: $xindex,
            info: Some(snd_ics_info_double),
            get: Some(snd_ics_get_double),
            put: Some(snd_ics_put_double),
            private_value: $addr as c_ulong,
        }
    };
}

unsafe extern "C" fn snd_ics_info_double(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER as c_uint;
        (*uinfo).count = 2;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 127;
    }
    0
}

unsafe extern "C" fn snd_ics_get_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let gus: *mut snd_gus_card = unsafe { snd_kcontrol_chip(kcontrol) };
    let addr: c_int = unsafe { ((*kcontrol).private_value & 0xff) as c_int };
    let left: c_uchar;
    let right: c_uchar;

    /* Original C uses guard(spinlock_irqsave)(&gus->reg_lock). */
    unsafe {
        left = (*gus).gf1.ics_regs[addr as usize][0];
        right = (*gus).gf1.ics_regs[addr as usize][1];
        (*ucontrol).value.integer.value[0] = (left & 127) as c_long;
        (*ucontrol).value.integer.value[1] = (right & 127) as c_long;
    }
    0
}

unsafe extern "C" fn snd_ics_put_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let gus: *mut snd_gus_card = unsafe { snd_kcontrol_chip(kcontrol) };
    let mut addr: c_int = unsafe { ((*kcontrol).private_value & 0xff) as c_int };
    let change: c_int;
    let mut val1: c_uchar;
    let mut val2: c_uchar;
    let oval1: c_uchar;
    let oval2: c_uchar;

    unsafe {
        val1 = ((*ucontrol).value.integer.value[0] & 127) as c_uchar;
        val2 = ((*ucontrol).value.integer.value[1] & 127) as c_uchar;
    }
    /* Original C uses guard(spinlock_irqsave)(&gus->reg_lock). */
    unsafe {
        oval1 = (*gus).gf1.ics_regs[addr as usize][0];
        oval2 = (*gus).gf1.ics_regs[addr as usize][1];
        change = (val1 != oval1 || val2 != oval2) as c_int;
        (*gus).gf1.ics_regs[addr as usize][0] = val1;
        (*gus).gf1.ics_regs[addr as usize][1] = val2;
        if (*gus).ics_flag != 0
            && (*gus).ics_flipped != 0
            && (addr == SNDRV_ICS_GF1_DEV || addr == SNDRV_ICS_MASTER_DEV)
        {
            core::mem::swap(&mut val1, &mut val2);
        }
        addr <<= 3;
        outb((addr | 0) as c_uchar, GUSP(gus, MIXCNTRLPORT));
        outb(1, GUSP(gus, MIXDATAPORT));
        outb((addr | 2) as c_uchar, GUSP(gus, MIXCNTRLPORT));
        outb(val1 as c_uchar, GUSP(gus, MIXDATAPORT));
        outb((addr | 1) as c_uchar, GUSP(gus, MIXCNTRLPORT));
        outb(2, GUSP(gus, MIXDATAPORT));
        outb((addr | 3) as c_uchar, GUSP(gus, MIXCNTRLPORT));
        outb(val2 as c_uchar, GUSP(gus, MIXDATAPORT));
    }
    change
}

static snd_gf1_controls: [snd_kcontrol_new; 3] = [
    GF1_SINGLE!("Master Playback Switch", 0, 1, 1),
    GF1_SINGLE!("Line Switch", 0, 0, 1),
    GF1_SINGLE!("Mic Switch", 0, 2, 0),
];

static snd_ics_controls: [snd_kcontrol_new; 8] = [
    GF1_SINGLE!("Master Playback Switch", 0, 1, 1),
    ICS_DOUBLE!("Master Playback Volume", 0, SNDRV_ICS_MASTER_DEV),
    ICS_DOUBLE!("Synth Playback Volume", 0, SNDRV_ICS_GF1_DEV),
    GF1_SINGLE!("Line Switch", 0, 0, 1),
    ICS_DOUBLE!("Line Playback Volume", 0, SNDRV_ICS_LINE_DEV),
    GF1_SINGLE!("Mic Switch", 0, 2, 0),
    ICS_DOUBLE!("Mic Playback Volume", 0, SNDRV_ICS_MIC_DEV),
    ICS_DOUBLE!("CD Playback Volume", 0, SNDRV_ICS_CD_DEV),
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_gf1_new_mixer(gus: *mut snd_gus_card) -> c_int {
    let card: *mut snd_card;
    let mut idx: c_uint;
    let max: c_uint;
    let mut err: c_int;

    if unsafe { snd_BUG_ON(gus.is_null()) } {
        return -EINVAL;
    }
    unsafe {
        card = (*gus).card;
    }
    if unsafe { snd_BUG_ON(card.is_null()) } {
        return -EINVAL;
    }

    unsafe {
        if (*gus).ics_flag != 0 {
            snd_component_add(card, c"ICS2101".as_ptr());
        }
        if (*card).mixername[0] == b'\0' as c_char {
            strscpy(
                (*card).mixername.as_mut_ptr(),
                if (*gus).ics_flag != 0 {
                    c"GF1,ICS2101".as_ptr()
                } else {
                    c"GF1".as_ptr()
                },
            );
        } else {
            if (*gus).ics_flag != 0 {
                strcat((*card).mixername.as_mut_ptr(), c",ICS2101".as_ptr());
            }
            strcat((*card).mixername.as_mut_ptr(), c",GF1".as_ptr());
        }
    }

    unsafe {
        if (*gus).ics_flag == 0 {
            max = if (*gus).ess_flag != 0 {
                1
            } else {
                ARRAY_SIZE!(snd_gf1_controls) as c_uint
            };
            idx = 0;
            while idx < max {
                err = snd_ctl_add(
                    card,
                    snd_ctl_new1(
                        &snd_gf1_controls[idx as usize],
                        gus as *mut c_void,
                    ),
                );
                if err < 0 {
                    return err;
                }
                idx += 1;
            }
        } else {
            idx = 0;
            while idx < ARRAY_SIZE!(snd_ics_controls) as c_uint {
                err = snd_ctl_add(
                    card,
                    snd_ctl_new1(
                        &snd_ics_controls[idx as usize],
                        gus as *mut c_void,
                    ),
                );
                if err < 0 {
                    return err;
                }
                idx += 1;
            }
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
