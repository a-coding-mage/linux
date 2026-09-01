// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PMac Burgundy lowlevel functions
 *
 * Copyright (c) by Takashi Iwai <tiwai@suse.de>
 * code based on dmasound.c.
 */

// Dependencies from the original C includes:
// <linux/io.h>, <linux/init.h>, <linux/delay.h>, <linux/of.h>,
// <sound/core.h>, "pmac.h", "burgundy.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

extern "C" {
    fn in_le32(addr: *const c_void) -> c_uint;
    fn out_le32(addr: *mut c_void, val: c_uint);
    fn udelay(usecs: c_uint);
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn of_machine_is_compatible(compat: *const c_char) -> c_int;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_pmac;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(
        kcontrol: *const snd_kcontrol_new,
        private_data: *mut c_void,
    ) -> *mut snd_kcontrol;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *const snd_ctl_elem_id);
    fn snd_pmac_add_automute(chip: *mut snd_pmac) -> c_int;
}

const fn BASE2ADDR(base: c_uint) -> c_uint {
    base << 12
}

const fn ADDR2BASE(addr: c_uint) -> c_uint {
    addr >> 12
}

/*
 * External types, constants, and struct layouts are supplied by translated
 * dependencies corresponding to pmac.h, burgundy.h, and ALSA/kernel headers.
 */

/* Waits for busy flag to clear */
unsafe fn snd_pmac_burgundy_busy_wait(chip: *mut snd_pmac) {
    let mut timeout: c_int = 50;

    while (in_le32(&mut (*(*chip).awacs).codec_ctrl as *mut _ as *const c_void) & MASK_NEWECMD) != 0
        && {
            let old = timeout;
            timeout -= 1;
            old != 0
        }
    {
        udelay(1);
    }
    if timeout < 0 {
        printk(c"burgundy_busy_wait: timeout\n".as_ptr());
    }
}

unsafe fn snd_pmac_burgundy_extend_wait(chip: *mut snd_pmac) {
    let mut timeout: c_int;

    timeout = 50;
    while (in_le32(&mut (*(*chip).awacs).codec_stat as *mut _ as *const c_void) & MASK_EXTEND) == 0
        && {
            let old = timeout;
            timeout -= 1;
            old != 0
        }
    {
        udelay(1);
    }
    if timeout < 0 {
        printk(c"burgundy_extend_wait: timeout #1\n".as_ptr());
    }
    timeout = 50;
    while (in_le32(&mut (*(*chip).awacs).codec_stat as *mut _ as *const c_void) & MASK_EXTEND) != 0
        && {
            let old = timeout;
            timeout -= 1;
            old != 0
        }
    {
        udelay(1);
    }
    if timeout < 0 {
        printk(c"burgundy_extend_wait: timeout #2\n".as_ptr());
    }
}

unsafe fn snd_pmac_burgundy_wcw(chip: *mut snd_pmac, addr: c_uint, val: c_uint) {
    out_le32(
        &mut (*(*chip).awacs).codec_ctrl as *mut _ as *mut c_void,
        addr.wrapping_add(0x200c00).wrapping_add(val & 0xff),
    );
    snd_pmac_burgundy_busy_wait(chip);
    out_le32(
        &mut (*(*chip).awacs).codec_ctrl as *mut _ as *mut c_void,
        addr.wrapping_add(0x200d00).wrapping_add((val >> 8) & 0xff),
    );
    snd_pmac_burgundy_busy_wait(chip);
    out_le32(
        &mut (*(*chip).awacs).codec_ctrl as *mut _ as *mut c_void,
        addr.wrapping_add(0x200e00).wrapping_add((val >> 16) & 0xff),
    );
    snd_pmac_burgundy_busy_wait(chip);
    out_le32(
        &mut (*(*chip).awacs).codec_ctrl as *mut _ as *mut c_void,
        addr.wrapping_add(0x200f00).wrapping_add((val >> 24) & 0xff),
    );
    snd_pmac_burgundy_busy_wait(chip);
}

unsafe fn snd_pmac_burgundy_rcw(chip: *mut snd_pmac, addr: c_uint) -> c_uint {
    let mut val: c_uint = 0;

    // C used guard(spinlock_irqsave)(&chip->reg_lock) for this scope.
    let _guard = spinlock_irqsave_guard(&mut (*chip).reg_lock);

    out_le32(
        &mut (*(*chip).awacs).codec_ctrl as *mut _ as *mut c_void,
        addr.wrapping_add(0x100000),
    );
    snd_pmac_burgundy_busy_wait(chip);
    snd_pmac_burgundy_extend_wait(chip);
    val = val.wrapping_add((in_le32(&mut (*(*chip).awacs).codec_stat as *mut _ as *const c_void) >> 4) & 0xff);

    out_le32(
        &mut (*(*chip).awacs).codec_ctrl as *mut _ as *mut c_void,
        addr.wrapping_add(0x100100),
    );
    snd_pmac_burgundy_busy_wait(chip);
    snd_pmac_burgundy_extend_wait(chip);
    val = val.wrapping_add(((in_le32(&mut (*(*chip).awacs).codec_stat as *mut _ as *const c_void) >> 4) & 0xff) << 8);

    out_le32(
        &mut (*(*chip).awacs).codec_ctrl as *mut _ as *mut c_void,
        addr.wrapping_add(0x100200),
    );
    snd_pmac_burgundy_busy_wait(chip);
    snd_pmac_burgundy_extend_wait(chip);
    val = val.wrapping_add(((in_le32(&mut (*(*chip).awacs).codec_stat as *mut _ as *const c_void) >> 4) & 0xff) << 16);

    out_le32(
        &mut (*(*chip).awacs).codec_ctrl as *mut _ as *mut c_void,
        addr.wrapping_add(0x100300),
    );
    snd_pmac_burgundy_busy_wait(chip);
    snd_pmac_burgundy_extend_wait(chip);
    val = val.wrapping_add(((in_le32(&mut (*(*chip).awacs).codec_stat as *mut _ as *const c_void) >> 4) & 0xff) << 24);

    val
}

unsafe fn snd_pmac_burgundy_wcb(chip: *mut snd_pmac, addr: c_uint, val: c_uint) {
    out_le32(
        &mut (*(*chip).awacs).codec_ctrl as *mut _ as *mut c_void,
        addr.wrapping_add(0x300000).wrapping_add(val & 0xff),
    );
    snd_pmac_burgundy_busy_wait(chip);
}

unsafe fn snd_pmac_burgundy_rcb(chip: *mut snd_pmac, addr: c_uint) -> c_uint {
    let mut val: c_uint = 0;

    // C used guard(spinlock_irqsave)(&chip->reg_lock) for this scope.
    let _guard = spinlock_irqsave_guard(&mut (*chip).reg_lock);

    out_le32(
        &mut (*(*chip).awacs).codec_ctrl as *mut _ as *mut c_void,
        addr.wrapping_add(0x100000),
    );
    snd_pmac_burgundy_busy_wait(chip);
    snd_pmac_burgundy_extend_wait(chip);
    val = val.wrapping_add((in_le32(&mut (*(*chip).awacs).codec_stat as *mut _ as *const c_void) >> 4) & 0xff);

    val
}

/*
 * Burgundy volume: 0 - 100, stereo, word reg
 */
unsafe fn snd_pmac_burgundy_write_volume(
    chip: *mut snd_pmac,
    address: c_uint,
    volume: *mut c_long,
    shift: c_int,
) {
    let hardvolume: c_int;
    let lvolume: c_int;
    let rvolume: c_int;

    if *volume.add(0) < 0
        || *volume.add(0) > 100
        || *volume.add(1) < 0
        || *volume.add(1) > 100
    {
        return; /* -EINVAL */
    }
    lvolume = if *volume.add(0) != 0 {
        (*volume.add(0) + BURGUNDY_VOLUME_OFFSET as c_long) as c_int
    } else {
        0
    };
    rvolume = if *volume.add(1) != 0 {
        (*volume.add(1) + BURGUNDY_VOLUME_OFFSET as c_long) as c_int
    } else {
        0
    };

    hardvolume = lvolume + (rvolume << shift);
    let mut hardvolume_mut = hardvolume;
    if shift == 8 {
        hardvolume_mut |= hardvolume_mut << 16;
    }

    snd_pmac_burgundy_wcw(chip, address, hardvolume_mut as c_uint);
}

unsafe fn snd_pmac_burgundy_read_volume(
    chip: *mut snd_pmac,
    address: c_uint,
    volume: *mut c_long,
    shift: c_int,
) {
    let wvolume: c_int;

    wvolume = snd_pmac_burgundy_rcw(chip, address) as c_int;

    *volume.add(0) = (wvolume & 0xff) as c_long;
    if *volume.add(0) >= BURGUNDY_VOLUME_OFFSET as c_long {
        *volume.add(0) -= BURGUNDY_VOLUME_OFFSET as c_long;
    } else {
        *volume.add(0) = 0;
    }
    *volume.add(1) = ((wvolume >> shift) & 0xff) as c_long;
    if *volume.add(1) >= BURGUNDY_VOLUME_OFFSET as c_long {
        *volume.add(1) -= BURGUNDY_VOLUME_OFFSET as c_long;
    } else {
        *volume.add(1) = 0;
    }
}

unsafe extern "C" fn snd_pmac_burgundy_info_volume(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 100;
    0
}

unsafe extern "C" fn snd_pmac_burgundy_get_volume(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let addr: c_uint = BASE2ADDR(((*kcontrol).private_value & 0xff) as c_uint);
    let shift: c_int = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    snd_pmac_burgundy_read_volume(chip, addr, (*ucontrol).value.integer.value.as_mut_ptr(), shift);
    0
}

unsafe extern "C" fn snd_pmac_burgundy_put_volume(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let addr: c_uint = BASE2ADDR(((*kcontrol).private_value & 0xff) as c_uint);
    let shift: c_int = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let mut nvoices: [c_long; 2] = [0; 2];

    snd_pmac_burgundy_write_volume(chip, addr, (*ucontrol).value.integer.value.as_mut_ptr(), shift);
    snd_pmac_burgundy_read_volume(chip, addr, nvoices.as_mut_ptr(), shift);
    (nvoices[0] != (*ucontrol).value.integer.value[0]
        || nvoices[1] != (*ucontrol).value.integer.value[1]) as c_int
}

const fn BURGUNDY_VOLUME_W(xname: *const c_char, xindex: c_uint, addr: c_uint, shift: c_uint) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        index: xindex,
        info: Some(snd_pmac_burgundy_info_volume),
        get: Some(snd_pmac_burgundy_get_volume),
        put: Some(snd_pmac_burgundy_put_volume),
        private_value: ((ADDR2BASE(addr) & 0xff) | (shift << 8)) as c_ulong,
    }
}

/*
 * Burgundy volume: 0 - 100, stereo, 2-byte reg
 */
unsafe fn snd_pmac_burgundy_write_volume_2b(
    chip: *mut snd_pmac,
    address: c_uint,
    volume: *mut c_long,
    mut off: c_int,
) {
    let lvolume: c_int;
    let rvolume: c_int;

    off |= off << 2;
    lvolume = if *volume.add(0) != 0 {
        (*volume.add(0) + BURGUNDY_VOLUME_OFFSET as c_long) as c_int
    } else {
        0
    };
    rvolume = if *volume.add(1) != 0 {
        (*volume.add(1) + BURGUNDY_VOLUME_OFFSET as c_long) as c_int
    } else {
        0
    };

    snd_pmac_burgundy_wcb(chip, address.wrapping_add(off as c_uint), lvolume as c_uint);
    snd_pmac_burgundy_wcb(chip, address.wrapping_add(off as c_uint).wrapping_add(0x500), rvolume as c_uint);
}

unsafe fn snd_pmac_burgundy_read_volume_2b(
    chip: *mut snd_pmac,
    address: c_uint,
    volume: *mut c_long,
    off: c_int,
) {
    *volume.add(0) = snd_pmac_burgundy_rcb(chip, address.wrapping_add(off as c_uint)) as c_long;
    if *volume.add(0) >= BURGUNDY_VOLUME_OFFSET as c_long {
        *volume.add(0) -= BURGUNDY_VOLUME_OFFSET as c_long;
    } else {
        *volume.add(0) = 0;
    }
    *volume.add(1) = snd_pmac_burgundy_rcb(chip, address.wrapping_add(off as c_uint).wrapping_add(0x100)) as c_long;
    if *volume.add(1) >= BURGUNDY_VOLUME_OFFSET as c_long {
        *volume.add(1) -= BURGUNDY_VOLUME_OFFSET as c_long;
    } else {
        *volume.add(1) = 0;
    }
}

unsafe extern "C" fn snd_pmac_burgundy_info_volume_2b(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 100;
    0
}

unsafe extern "C" fn snd_pmac_burgundy_get_volume_2b(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let addr: c_uint = BASE2ADDR(((*kcontrol).private_value & 0xff) as c_uint);
    let off: c_int = ((*kcontrol).private_value & 0x300) as c_int;
    snd_pmac_burgundy_read_volume_2b(chip, addr, (*ucontrol).value.integer.value.as_mut_ptr(), off);
    0
}

unsafe extern "C" fn snd_pmac_burgundy_put_volume_2b(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let addr: c_uint = BASE2ADDR(((*kcontrol).private_value & 0xff) as c_uint);
    let off: c_int = ((*kcontrol).private_value & 0x300) as c_int;
    let mut nvoices: [c_long; 2] = [0; 2];

    snd_pmac_burgundy_write_volume_2b(chip, addr, (*ucontrol).value.integer.value.as_mut_ptr(), off);
    snd_pmac_burgundy_read_volume_2b(chip, addr, nvoices.as_mut_ptr(), off);
    (nvoices[0] != (*ucontrol).value.integer.value[0]
        || nvoices[1] != (*ucontrol).value.integer.value[1]) as c_int
}

const fn BURGUNDY_VOLUME_2B(xname: *const c_char, xindex: c_uint, addr: c_uint, off: c_uint) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        index: xindex,
        info: Some(snd_pmac_burgundy_info_volume_2b),
        get: Some(snd_pmac_burgundy_get_volume_2b),
        put: Some(snd_pmac_burgundy_put_volume_2b),
        private_value: ((ADDR2BASE(addr) & 0xff) | (off << 8)) as c_ulong,
    }
}

/*
 * Burgundy gain/attenuation: 0 - 15, mono/stereo, byte reg
 */
unsafe extern "C" fn snd_pmac_burgundy_info_gain(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let stereo: c_int = (((*kcontrol).private_value >> 24) & 1) as c_int;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = (stereo + 1) as c_uint;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 15;
    0
}

unsafe extern "C" fn snd_pmac_burgundy_get_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let addr: c_uint = BASE2ADDR(((*kcontrol).private_value & 0xff) as c_uint);
    let stereo: c_int = (((*kcontrol).private_value >> 24) & 1) as c_int;
    let atten: c_int = (((*kcontrol).private_value >> 25) & 1) as c_int;
    let mut oval: c_int;

    oval = snd_pmac_burgundy_rcb(chip, addr) as c_int;
    if atten != 0 {
        oval = !oval & 0xff;
    }
    (*ucontrol).value.integer.value[0] = (oval & 0xf) as c_long;
    if stereo != 0 {
        (*ucontrol).value.integer.value[1] = ((oval >> 4) & 0xf) as c_long;
    }
    0
}

unsafe extern "C" fn snd_pmac_burgundy_put_gain(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let addr: c_uint = BASE2ADDR(((*kcontrol).private_value & 0xff) as c_uint);
    let stereo: c_int = (((*kcontrol).private_value >> 24) & 1) as c_int;
    let atten: c_int = (((*kcontrol).private_value >> 25) & 1) as c_int;
    let mut oval: c_int;
    let mut val: c_int;

    oval = snd_pmac_burgundy_rcb(chip, addr) as c_int;
    if atten != 0 {
        oval = !oval & 0xff;
    }
    val = (*ucontrol).value.integer.value[0] as c_int;
    if stereo != 0 {
        val |= ((*ucontrol).value.integer.value[1] as c_int) << 4;
    } else {
        val |= ((*ucontrol).value.integer.value[0] as c_int) << 4;
    }
    if atten != 0 {
        val = !val & 0xff;
    }
    snd_pmac_burgundy_wcb(chip, addr, val as c_uint);
    (val != oval) as c_int
}

const fn BURGUNDY_VOLUME_B(
    xname: *const c_char,
    xindex: c_uint,
    addr: c_uint,
    stereo: c_uint,
    atten: c_uint,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        index: xindex,
        info: Some(snd_pmac_burgundy_info_gain),
        get: Some(snd_pmac_burgundy_get_gain),
        put: Some(snd_pmac_burgundy_put_gain),
        private_value: (ADDR2BASE(addr) | (stereo << 24) | (atten << 25)) as c_ulong,
    }
}

/*
 * Burgundy switch: 0/1, mono/stereo, word reg
 */
unsafe extern "C" fn snd_pmac_burgundy_info_switch_w(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let stereo: c_int = (((*kcontrol).private_value >> 24) & 1) as c_int;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = (stereo + 1) as c_uint;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe extern "C" fn snd_pmac_burgundy_get_switch_w(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let addr: c_uint = BASE2ADDR((((*kcontrol).private_value >> 16) & 0xff) as c_uint);
    let lmask: c_int = 1 << ((*kcontrol).private_value & 0xff);
    let rmask: c_int = 1 << (((*kcontrol).private_value >> 8) & 0xff);
    let stereo: c_int = (((*kcontrol).private_value >> 24) & 1) as c_int;
    let val: c_int = snd_pmac_burgundy_rcw(chip, addr) as c_int;
    (*ucontrol).value.integer.value[0] = if (val & lmask) != 0 { 1 } else { 0 };
    if stereo != 0 {
        (*ucontrol).value.integer.value[1] = if (val & rmask) != 0 { 1 } else { 0 };
    }
    0
}

unsafe extern "C" fn snd_pmac_burgundy_put_switch_w(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let addr: c_uint = BASE2ADDR((((*kcontrol).private_value >> 16) & 0xff) as c_uint);
    let lmask: c_int = 1 << ((*kcontrol).private_value & 0xff);
    let rmask: c_int = 1 << (((*kcontrol).private_value >> 8) & 0xff);
    let stereo: c_int = (((*kcontrol).private_value >> 24) & 1) as c_int;
    let mut val: c_int;
    let oval: c_int;

    oval = snd_pmac_burgundy_rcw(chip, addr) as c_int;
    val = oval & !(lmask | if stereo != 0 { rmask } else { 0 });
    if (*ucontrol).value.integer.value[0] != 0 {
        val |= lmask;
    }
    if stereo != 0 && (*ucontrol).value.integer.value[1] != 0 {
        val |= rmask;
    }
    snd_pmac_burgundy_wcw(chip, addr, val as c_uint);
    (val != oval) as c_int
}

const fn BURGUNDY_SWITCH_W(
    xname: *const c_char,
    xindex: c_uint,
    addr: c_uint,
    lbit: c_uint,
    rbit: c_uint,
    stereo: c_uint,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        index: xindex,
        info: Some(snd_pmac_burgundy_info_switch_w),
        get: Some(snd_pmac_burgundy_get_switch_w),
        put: Some(snd_pmac_burgundy_put_switch_w),
        private_value: (lbit | (rbit << 8) | (ADDR2BASE(addr) << 16) | (stereo << 24)) as c_ulong,
    }
}

/*
 * Burgundy switch: 0/1, mono/stereo, byte reg, bit mask
 */
unsafe extern "C" fn snd_pmac_burgundy_info_switch_b(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let stereo: c_int = (((*kcontrol).private_value >> 24) & 1) as c_int;
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = (stereo + 1) as c_uint;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe extern "C" fn snd_pmac_burgundy_get_switch_b(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let addr: c_uint = BASE2ADDR((((*kcontrol).private_value >> 16) & 0xff) as c_uint);
    let lmask: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let rmask: c_int = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let stereo: c_int = (((*kcontrol).private_value >> 24) & 1) as c_int;
    let val: c_int = snd_pmac_burgundy_rcb(chip, addr) as c_int;
    (*ucontrol).value.integer.value[0] = if (val & lmask) != 0 { 1 } else { 0 };
    if stereo != 0 {
        (*ucontrol).value.integer.value[1] = if (val & rmask) != 0 { 1 } else { 0 };
    }
    0
}

unsafe extern "C" fn snd_pmac_burgundy_put_switch_b(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_pmac = snd_kcontrol_chip(kcontrol);
    let addr: c_uint = BASE2ADDR((((*kcontrol).private_value >> 16) & 0xff) as c_uint);
    let lmask: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let rmask: c_int = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let stereo: c_int = (((*kcontrol).private_value >> 24) & 1) as c_int;
    let mut val: c_int;
    let oval: c_int;

    oval = snd_pmac_burgundy_rcb(chip, addr) as c_int;
    val = oval & !(lmask | rmask);
    if (*ucontrol).value.integer.value[0] != 0 {
        val |= lmask;
    }
    if stereo != 0 && (*ucontrol).value.integer.value[1] != 0 {
        val |= rmask;
    }
    snd_pmac_burgundy_wcb(chip, addr, val as c_uint);
    (val != oval) as c_int
}

const fn BURGUNDY_SWITCH_B(
    xname: *const c_char,
    xindex: c_uint,
    addr: c_uint,
    lmask: c_uint,
    rmask: c_uint,
    stereo: c_uint,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        index: xindex,
        info: Some(snd_pmac_burgundy_info_switch_b),
        get: Some(snd_pmac_burgundy_get_switch_b),
        put: Some(snd_pmac_burgundy_put_switch_b),
        private_value: (lmask | (rmask << 8) | (ADDR2BASE(addr) << 16) | (stereo << 24)) as c_ulong,
    }
}

/*
 * Burgundy mixers
 */
static snd_pmac_burgundy_mixers: [snd_kcontrol_new; 9] = [
    BURGUNDY_VOLUME_W(c"Master Playback Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_MASTER_VOLUME, 8),
    BURGUNDY_VOLUME_W(c"CD Capture Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_VOLCD, 16),
    BURGUNDY_VOLUME_2B(c"Input Capture Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_VOLMIX01, 2),
    BURGUNDY_VOLUME_2B(c"Mixer Playback Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_VOLMIX23, 0),
    BURGUNDY_VOLUME_B(c"CD Gain Capture Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_GAINCD, 1, 0),
    BURGUNDY_SWITCH_W(c"Master Capture Switch".as_ptr(), 0, MASK_ADDR_BURGUNDY_OUTPUTENABLES, 24, 0, 0),
    BURGUNDY_SWITCH_W(c"CD Capture Switch".as_ptr(), 0, MASK_ADDR_BURGUNDY_CAPTURESELECTS, 0, 16, 1),
    BURGUNDY_SWITCH_W(c"CD Playback Switch".as_ptr(), 0, MASK_ADDR_BURGUNDY_OUTPUTSELECTS, 0, 16, 1),
    /*
     * BURGUNDY_SWITCH_W("Loop Capture Switch", 0,
     *      MASK_ADDR_BURGUNDY_CAPTURESELECTS, 8, 24, 1),
     * BURGUNDY_SWITCH_B("Mixer out Capture Switch", 0,
     *      MASK_ADDR_BURGUNDY_HOSTIFAD, 0x02, 0, 0),
     * BURGUNDY_SWITCH_B("Mixer Capture Switch", 0,
     *      MASK_ADDR_BURGUNDY_HOSTIFAD, 0x01, 0, 0),
     * BURGUNDY_SWITCH_B("PCM out Capture Switch", 0,
     *      MASK_ADDR_BURGUNDY_HOSTIFEH, 0x02, 0, 0),
     */
    BURGUNDY_SWITCH_B(c"PCM Capture Switch".as_ptr(), 0, MASK_ADDR_BURGUNDY_HOSTIFEH, 0x01, 0, 0),
];

static snd_pmac_burgundy_mixers_imac: [snd_kcontrol_new; 12] = [
    BURGUNDY_VOLUME_W(c"Line in Capture Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_VOLLINE, 16),
    BURGUNDY_VOLUME_W(c"Mic Capture Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_VOLMIC, 16),
    BURGUNDY_VOLUME_B(c"Line in Gain Capture Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_GAINLINE, 1, 0),
    BURGUNDY_VOLUME_B(c"Mic Gain Capture Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_GAINMIC, 1, 0),
    BURGUNDY_VOLUME_B(c"Speaker Playback Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_ATTENSPEAKER, 1, 1),
    BURGUNDY_VOLUME_B(c"Line out Playback Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_ATTENLINEOUT, 1, 1),
    BURGUNDY_VOLUME_B(c"Headphone Playback Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_ATTENHP, 1, 1),
    BURGUNDY_SWITCH_W(c"Line in Capture Switch".as_ptr(), 0, MASK_ADDR_BURGUNDY_CAPTURESELECTS, 1, 17, 1),
    BURGUNDY_SWITCH_W(c"Mic Capture Switch".as_ptr(), 0, MASK_ADDR_BURGUNDY_CAPTURESELECTS, 2, 18, 1),
    BURGUNDY_SWITCH_W(c"Line in Playback Switch".as_ptr(), 0, MASK_ADDR_BURGUNDY_OUTPUTSELECTS, 1, 17, 1),
    BURGUNDY_SWITCH_W(c"Mic Playback Switch".as_ptr(), 0, MASK_ADDR_BURGUNDY_OUTPUTSELECTS, 2, 18, 1),
    BURGUNDY_SWITCH_B(c"Mic Boost Capture Switch".as_ptr(), 0, MASK_ADDR_BURGUNDY_INPBOOST, 0x40, 0x80, 1),
];

static snd_pmac_burgundy_mixers_pmac: [snd_kcontrol_new; 6] = [
    BURGUNDY_VOLUME_W(c"Line in Capture Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_VOLMIC, 16),
    BURGUNDY_VOLUME_B(c"Line in Gain Capture Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_GAINMIC, 1, 0),
    BURGUNDY_VOLUME_B(c"Speaker Playback Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_ATTENMONO, 0, 1),
    BURGUNDY_VOLUME_B(c"Line out Playback Volume".as_ptr(), 0, MASK_ADDR_BURGUNDY_ATTENSPEAKER, 1, 1),
    BURGUNDY_SWITCH_W(c"Line in Capture Switch".as_ptr(), 0, MASK_ADDR_BURGUNDY_CAPTURESELECTS, 2, 18, 1),
    BURGUNDY_SWITCH_W(c"Line in Playback Switch".as_ptr(), 0, MASK_ADDR_BURGUNDY_OUTPUTSELECTS, 2, 18, 1),
    /*
     * BURGUNDY_SWITCH_B("Line in Boost Capture Switch", 0,
     *      MASK_ADDR_BURGUNDY_INPBOOST, 0x40, 0x80, 1)
     */
];

static snd_pmac_burgundy_master_sw_imac: snd_kcontrol_new = BURGUNDY_SWITCH_B(
    c"Master Playback Switch".as_ptr(),
    0,
    MASK_ADDR_BURGUNDY_MORE_OUTPUTENABLES,
    BURGUNDY_OUTPUT_LEFT | BURGUNDY_LINEOUT_LEFT | BURGUNDY_HP_LEFT,
    BURGUNDY_OUTPUT_RIGHT | BURGUNDY_LINEOUT_RIGHT | BURGUNDY_HP_RIGHT,
    1,
);
static snd_pmac_burgundy_master_sw_pmac: snd_kcontrol_new = BURGUNDY_SWITCH_B(
    c"Master Playback Switch".as_ptr(),
    0,
    MASK_ADDR_BURGUNDY_MORE_OUTPUTENABLES,
    BURGUNDY_OUTPUT_INTERN | BURGUNDY_OUTPUT_LEFT,
    BURGUNDY_OUTPUT_RIGHT,
    1,
);
static snd_pmac_burgundy_speaker_sw_imac: snd_kcontrol_new = BURGUNDY_SWITCH_B(
    c"Speaker Playback Switch".as_ptr(),
    0,
    MASK_ADDR_BURGUNDY_MORE_OUTPUTENABLES,
    BURGUNDY_OUTPUT_LEFT,
    BURGUNDY_OUTPUT_RIGHT,
    1,
);
static snd_pmac_burgundy_speaker_sw_pmac: snd_kcontrol_new = BURGUNDY_SWITCH_B(
    c"Speaker Playback Switch".as_ptr(),
    0,
    MASK_ADDR_BURGUNDY_MORE_OUTPUTENABLES,
    BURGUNDY_OUTPUT_INTERN,
    0,
    0,
);
static snd_pmac_burgundy_line_sw_imac: snd_kcontrol_new = BURGUNDY_SWITCH_B(
    c"Line out Playback Switch".as_ptr(),
    0,
    MASK_ADDR_BURGUNDY_MORE_OUTPUTENABLES,
    BURGUNDY_LINEOUT_LEFT,
    BURGUNDY_LINEOUT_RIGHT,
    1,
);
static snd_pmac_burgundy_line_sw_pmac: snd_kcontrol_new = BURGUNDY_SWITCH_B(
    c"Line out Playback Switch".as_ptr(),
    0,
    MASK_ADDR_BURGUNDY_MORE_OUTPUTENABLES,
    BURGUNDY_OUTPUT_LEFT,
    BURGUNDY_OUTPUT_RIGHT,
    1,
);
static snd_pmac_burgundy_hp_sw_imac: snd_kcontrol_new = BURGUNDY_SWITCH_B(
    c"Headphone Playback Switch".as_ptr(),
    0,
    MASK_ADDR_BURGUNDY_MORE_OUTPUTENABLES,
    BURGUNDY_HP_LEFT,
    BURGUNDY_HP_RIGHT,
    1,
);

// Original C conditional: #ifdef PMAC_SUPPORT_AUTOMUTE
/*
 * auto-mute stuffs
 */
unsafe extern "C" fn snd_pmac_burgundy_detect_headphone(chip: *mut snd_pmac) -> c_int {
    if (in_le32(&mut (*(*chip).awacs).codec_stat as *mut _ as *const c_void) & (*chip).hp_stat_mask) != 0 {
        1
    } else {
        0
    }
}

unsafe extern "C" fn snd_pmac_burgundy_update_automute(chip: *mut snd_pmac, do_notify: c_int) {
    if (*chip).auto_mute != 0 {
        let imac: c_int = of_machine_is_compatible(c"iMac".as_ptr());
        let mut reg: c_int;
        let oreg: c_int;

        reg = snd_pmac_burgundy_rcb(chip, MASK_ADDR_BURGUNDY_MORE_OUTPUTENABLES) as c_int;
        oreg = reg;
        reg &= if imac != 0 {
            !((BURGUNDY_OUTPUT_LEFT | BURGUNDY_OUTPUT_RIGHT | BURGUNDY_HP_LEFT | BURGUNDY_HP_RIGHT) as c_int)
        } else {
            !((BURGUNDY_OUTPUT_LEFT | BURGUNDY_OUTPUT_RIGHT | BURGUNDY_OUTPUT_INTERN) as c_int)
        };
        if snd_pmac_burgundy_detect_headphone(chip) != 0 {
            reg |= if imac != 0 {
                (BURGUNDY_HP_LEFT | BURGUNDY_HP_RIGHT) as c_int
            } else {
                (BURGUNDY_OUTPUT_LEFT | BURGUNDY_OUTPUT_RIGHT) as c_int
            };
        } else {
            reg |= if imac != 0 {
                (BURGUNDY_OUTPUT_LEFT | BURGUNDY_OUTPUT_RIGHT) as c_int
            } else {
                BURGUNDY_OUTPUT_INTERN as c_int
            };
        }
        if do_notify != 0 && reg == oreg {
            return;
        }
        snd_pmac_burgundy_wcb(chip, MASK_ADDR_BURGUNDY_MORE_OUTPUTENABLES, reg as c_uint);
        if do_notify != 0 {
            snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &(*(*chip).master_sw_ctl).id);
            snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &(*(*chip).speaker_sw_ctl).id);
            snd_ctl_notify((*chip).card, SNDRV_CTL_EVENT_MASK_VALUE, &(*(*chip).hp_detect_ctl).id);
        }
    }
}
// Original C conditional end: #endif /* PMAC_SUPPORT_AUTOMUTE */

/*
 * initialize burgundy
 */
#[no_mangle]
pub unsafe extern "C" fn snd_pmac_burgundy_init(chip: *mut snd_pmac) -> c_int {
    let imac: c_int = of_machine_is_compatible(c"iMac".as_ptr());
    let mut i: c_int;
    let mut err: c_int;

    /* Checks to see the chip is alive and kicking */
    if (in_le32(&mut (*(*chip).awacs).codec_ctrl as *mut _ as *const c_void) & MASK_ERRCODE) == 0xf0000 {
        printk(c"pmac burgundy: disabled by MacOS :-(\n".as_ptr());
        return 1;
    }

    snd_pmac_burgundy_wcw(chip, MASK_ADDR_BURGUNDY_OUTPUTENABLES, DEF_BURGUNDY_OUTPUTENABLES);
    snd_pmac_burgundy_wcb(chip, MASK_ADDR_BURGUNDY_MORE_OUTPUTENABLES, DEF_BURGUNDY_MORE_OUTPUTENABLES);
    snd_pmac_burgundy_wcw(chip, MASK_ADDR_BURGUNDY_OUTPUTSELECTS, DEF_BURGUNDY_OUTPUTSELECTS);

    snd_pmac_burgundy_wcb(chip, MASK_ADDR_BURGUNDY_INPSEL21, DEF_BURGUNDY_INPSEL21);
    snd_pmac_burgundy_wcb(
        chip,
        MASK_ADDR_BURGUNDY_INPSEL3,
        if imac != 0 {
            DEF_BURGUNDY_INPSEL3_IMAC
        } else {
            DEF_BURGUNDY_INPSEL3_PMAC
        },
    );
    snd_pmac_burgundy_wcb(chip, MASK_ADDR_BURGUNDY_GAINCD, DEF_BURGUNDY_GAINCD);
    snd_pmac_burgundy_wcb(chip, MASK_ADDR_BURGUNDY_GAINLINE, DEF_BURGUNDY_GAINLINE);
    snd_pmac_burgundy_wcb(chip, MASK_ADDR_BURGUNDY_GAINMIC, DEF_BURGUNDY_GAINMIC);
    snd_pmac_burgundy_wcb(chip, MASK_ADDR_BURGUNDY_GAINMODEM, DEF_BURGUNDY_GAINMODEM);

    snd_pmac_burgundy_wcb(chip, MASK_ADDR_BURGUNDY_ATTENSPEAKER, DEF_BURGUNDY_ATTENSPEAKER);
    snd_pmac_burgundy_wcb(chip, MASK_ADDR_BURGUNDY_ATTENLINEOUT, DEF_BURGUNDY_ATTENLINEOUT);
    snd_pmac_burgundy_wcb(chip, MASK_ADDR_BURGUNDY_ATTENHP, DEF_BURGUNDY_ATTENHP);

    snd_pmac_burgundy_wcw(chip, MASK_ADDR_BURGUNDY_MASTER_VOLUME, DEF_BURGUNDY_MASTER_VOLUME);
    snd_pmac_burgundy_wcw(chip, MASK_ADDR_BURGUNDY_VOLCD, DEF_BURGUNDY_VOLCD);
    snd_pmac_burgundy_wcw(chip, MASK_ADDR_BURGUNDY_VOLLINE, DEF_BURGUNDY_VOLLINE);
    snd_pmac_burgundy_wcw(chip, MASK_ADDR_BURGUNDY_VOLMIC, DEF_BURGUNDY_VOLMIC);

    if (*chip).hp_stat_mask == 0 {
        /* set headphone-jack detection bit */
        if imac != 0 {
            (*chip).hp_stat_mask =
                BURGUNDY_HPDETECT_IMAC_UPPER | BURGUNDY_HPDETECT_IMAC_LOWER | BURGUNDY_HPDETECT_IMAC_SIDE;
        } else {
            (*chip).hp_stat_mask = BURGUNDY_HPDETECT_PMAC_BACK;
        }
    }
    /*
     * build burgundy mixers
     */
    strscpy((*(*chip).card).mixername.as_mut_ptr(), c"PowerMac Burgundy".as_ptr());

    i = 0;
    while i < snd_pmac_burgundy_mixers.len() as c_int {
        err = snd_ctl_add(
            (*chip).card,
            snd_ctl_new1(&snd_pmac_burgundy_mixers[i as usize], chip as *mut c_void),
        );
        if err < 0 {
            return err;
        }
        i += 1;
    }
    i = 0;
    while i
        < if imac != 0 {
            snd_pmac_burgundy_mixers_imac.len() as c_int
        } else {
            snd_pmac_burgundy_mixers_pmac.len() as c_int
        }
    {
        err = snd_ctl_add(
            (*chip).card,
            snd_ctl_new1(
                if imac != 0 {
                    &snd_pmac_burgundy_mixers_imac[i as usize]
                } else {
                    &snd_pmac_burgundy_mixers_pmac[i as usize]
                },
                chip as *mut c_void,
            ),
        );
        if err < 0 {
            return err;
        }
        i += 1;
    }
    (*chip).master_sw_ctl = snd_ctl_new1(
        if imac != 0 {
            &snd_pmac_burgundy_master_sw_imac
        } else {
            &snd_pmac_burgundy_master_sw_pmac
        },
        chip as *mut c_void,
    );
    err = snd_ctl_add((*chip).card, (*chip).master_sw_ctl);
    if err < 0 {
        return err;
    }
    (*chip).master_sw_ctl = snd_ctl_new1(
        if imac != 0 {
            &snd_pmac_burgundy_line_sw_imac
        } else {
            &snd_pmac_burgundy_line_sw_pmac
        },
        chip as *mut c_void,
    );
    err = snd_ctl_add((*chip).card, (*chip).master_sw_ctl);
    if err < 0 {
        return err;
    }
    if imac != 0 {
        (*chip).master_sw_ctl = snd_ctl_new1(&snd_pmac_burgundy_hp_sw_imac, chip as *mut c_void);
        err = snd_ctl_add((*chip).card, (*chip).master_sw_ctl);
        if err < 0 {
            return err;
        }
    }
    (*chip).speaker_sw_ctl = snd_ctl_new1(
        if imac != 0 {
            &snd_pmac_burgundy_speaker_sw_imac
        } else {
            &snd_pmac_burgundy_speaker_sw_pmac
        },
        chip as *mut c_void,
    );
    err = snd_ctl_add((*chip).card, (*chip).speaker_sw_ctl);
    if err < 0 {
        return err;
    }

    // Original C conditional: #ifdef PMAC_SUPPORT_AUTOMUTE
    err = snd_pmac_add_automute(chip);
    if err < 0 {
        return err;
    }

    (*chip).detect_headphone = Some(snd_pmac_burgundy_detect_headphone);
    (*chip).update_automute = Some(snd_pmac_burgundy_update_automute);
    snd_pmac_burgundy_update_automute(chip, 0); /* update the status only */
    // Original C conditional end: #endif

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
