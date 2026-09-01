// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   ALSA driver for ICEnsemble VT17xx
 *
 *   Lowlevel functions for WM8766 codec
 *
 *	Copyright (c) 2012 Ondrej Zary <linux@rainbow-software.org>
 */

/* C dependencies:
 * <linux/delay.h>
 * <sound/core.h>
 * <sound/control.h>
 * <sound/tlv.h>
 * "wm8766.h"
 */

use core::ffi::{c_int, c_uint};
use core::mem::{size_of, zeroed};
use core::ptr::{copy_nonoverlapping, null};

/* low-level access */

unsafe fn snd_wm8766_write(wm: *mut snd_wm8766, addr: u16, data: u16) {
    if addr < WM8766_REG_COUNT as u16 {
        (*wm).regs[addr as usize] = data;
    }
    ((*wm).ops.write)(wm, addr, data);
}

/* mixer controls */

static wm8766_tlv: [c_uint; 4] = [
    SNDRV_CTL_TLVT_DB_SCALE,
    (2 * size_of::<c_uint>()) as c_uint,
    (-12750i32) as c_uint,
    50 | TLV_DB_SCALE_MUTE,
];

static snd_wm8766_default_ctl: [snd_wm8766_ctl; WM8766_CTL_COUNT as usize] = {
    let mut ctl: [snd_wm8766_ctl; WM8766_CTL_COUNT as usize] =
        [snd_wm8766_ctl::ZERO; WM8766_CTL_COUNT as usize];

    ctl[WM8766_CTL_CH1_VOL as usize] = snd_wm8766_ctl {
        name: c"Channel 1 Playback Volume".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_INTEGER,
        tlv: wm8766_tlv.as_ptr(),
        reg1: WM8766_REG_DACL1,
        reg2: WM8766_REG_DACR1,
        mask1: WM8766_VOL_MASK,
        mask2: WM8766_VOL_MASK,
        max: 0xff,
        flags: WM8766_FLAG_STEREO | WM8766_FLAG_VOL_UPDATE,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_CH2_VOL as usize] = snd_wm8766_ctl {
        name: c"Channel 2 Playback Volume".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_INTEGER,
        tlv: wm8766_tlv.as_ptr(),
        reg1: WM8766_REG_DACL2,
        reg2: WM8766_REG_DACR2,
        mask1: WM8766_VOL_MASK,
        mask2: WM8766_VOL_MASK,
        max: 0xff,
        flags: WM8766_FLAG_STEREO | WM8766_FLAG_VOL_UPDATE,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_CH3_VOL as usize] = snd_wm8766_ctl {
        name: c"Channel 3 Playback Volume".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_INTEGER,
        tlv: wm8766_tlv.as_ptr(),
        reg1: WM8766_REG_DACL3,
        reg2: WM8766_REG_DACR3,
        mask1: WM8766_VOL_MASK,
        mask2: WM8766_VOL_MASK,
        max: 0xff,
        flags: WM8766_FLAG_STEREO | WM8766_FLAG_VOL_UPDATE,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_CH1_SW as usize] = snd_wm8766_ctl {
        name: c"Channel 1 Playback Switch".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN,
        reg1: WM8766_REG_DACCTRL2,
        mask1: WM8766_DAC2_MUTE1,
        flags: WM8766_FLAG_INVERT,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_CH2_SW as usize] = snd_wm8766_ctl {
        name: c"Channel 2 Playback Switch".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN,
        reg1: WM8766_REG_DACCTRL2,
        mask1: WM8766_DAC2_MUTE2,
        flags: WM8766_FLAG_INVERT,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_CH3_SW as usize] = snd_wm8766_ctl {
        name: c"Channel 3 Playback Switch".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN,
        reg1: WM8766_REG_DACCTRL2,
        mask1: WM8766_DAC2_MUTE3,
        flags: WM8766_FLAG_INVERT,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_PHASE1_SW as usize] = snd_wm8766_ctl {
        name: c"Channel 1 Phase Invert Playback Switch".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN,
        reg1: WM8766_REG_IFCTRL,
        mask1: WM8766_PHASE_INVERT1,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_PHASE2_SW as usize] = snd_wm8766_ctl {
        name: c"Channel 2 Phase Invert Playback Switch".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN,
        reg1: WM8766_REG_IFCTRL,
        mask1: WM8766_PHASE_INVERT2,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_PHASE3_SW as usize] = snd_wm8766_ctl {
        name: c"Channel 3 Phase Invert Playback Switch".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN,
        reg1: WM8766_REG_IFCTRL,
        mask1: WM8766_PHASE_INVERT3,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_DEEMPH1_SW as usize] = snd_wm8766_ctl {
        name: c"Channel 1 Deemphasis Playback Switch".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN,
        reg1: WM8766_REG_DACCTRL2,
        mask1: WM8766_DAC2_DEEMP1,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_DEEMPH2_SW as usize] = snd_wm8766_ctl {
        name: c"Channel 2 Deemphasis Playback Switch".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN,
        reg1: WM8766_REG_DACCTRL2,
        mask1: WM8766_DAC2_DEEMP2,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_DEEMPH3_SW as usize] = snd_wm8766_ctl {
        name: c"Channel 3 Deemphasis Playback Switch".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN,
        reg1: WM8766_REG_DACCTRL2,
        mask1: WM8766_DAC2_DEEMP3,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_IZD_SW as usize] = snd_wm8766_ctl {
        name: c"Infinite Zero Detect Playback Switch".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN,
        reg1: WM8766_REG_DACCTRL1,
        mask1: WM8766_DAC_IZD,
        ..snd_wm8766_ctl::ZERO
    };
    ctl[WM8766_CTL_ZC_SW as usize] = snd_wm8766_ctl {
        name: c"Zero Cross Detect Playback Switch".as_ptr(),
        type_: SNDRV_CTL_ELEM_TYPE_BOOLEAN,
        reg1: WM8766_REG_DACCTRL2,
        mask1: WM8766_DAC2_ZCD,
        flags: WM8766_FLAG_INVERT,
        ..snd_wm8766_ctl::ZERO
    };

    ctl
};

/* exported functions */

pub unsafe extern "C" fn snd_wm8766_init(wm: *mut snd_wm8766) {
    static default_values: [u16; 11] = [
        0x000, 0x100,
        0x120, 0x000,
        0x000, 0x100, 0x000, 0x100, 0x000,
        0x000, 0x080,
    ];

    copy_nonoverlapping(
        snd_wm8766_default_ctl.as_ptr(),
        (*wm).ctl.as_mut_ptr(),
        (*wm).ctl.len(),
    );

    snd_wm8766_write(wm, WM8766_REG_RESET, 0x00); /* reset */
    udelay(10);
    /* load defaults */
    let mut i = 0usize;
    while i < default_values.len() {
        snd_wm8766_write(wm, i as u16, default_values[i]);
        i += 1;
    }
}

pub unsafe extern "C" fn snd_wm8766_resume(wm: *mut snd_wm8766) {
    let mut i = 0;

    while i < WM8766_REG_COUNT {
        snd_wm8766_write(wm, i as u16, (*wm).regs[i as usize]);
        i += 1;
    }
}

pub unsafe extern "C" fn snd_wm8766_set_if(wm: *mut snd_wm8766, mut dac: u16) {
    let val: u16 = (*wm).regs[WM8766_REG_IFCTRL as usize] & !WM8766_IF_MASK;

    dac &= WM8766_IF_MASK;
    snd_wm8766_write(wm, WM8766_REG_IFCTRL, val | dac);
}

pub unsafe extern "C" fn snd_wm8766_volume_restore(wm: *mut snd_wm8766) {
    let val: u16 = (*wm).regs[WM8766_REG_DACR1 as usize];
    /* restore volume after MCLK stopped */
    snd_wm8766_write(wm, WM8766_REG_DACR1, val | WM8766_VOL_UPDATE);
}

/* mixer callbacks */

unsafe extern "C" fn snd_wm8766_volume_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let wm: *mut snd_wm8766 = snd_kcontrol_chip(kcontrol);
    let n: c_int = (*kcontrol).private_value as c_int;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = if ((*wm).ctl[n as usize].flags & WM8766_FLAG_STEREO) != 0 {
        2
    } else {
        1
    };
    (*uinfo).value.integer.min = (*wm).ctl[n as usize].min;
    (*uinfo).value.integer.max = (*wm).ctl[n as usize].max;

    0
}

unsafe extern "C" fn snd_wm8766_enum_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let wm: *mut snd_wm8766 = snd_kcontrol_chip(kcontrol);
    let n: c_int = (*kcontrol).private_value as c_int;

    snd_ctl_enum_info(
        uinfo,
        1,
        (*wm).ctl[n as usize].max,
        (*wm).ctl[n as usize].enum_names,
    )
}

unsafe extern "C" fn snd_wm8766_ctl_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let wm: *mut snd_wm8766 = snd_kcontrol_chip(kcontrol);
    let n: c_int = (*kcontrol).private_value as c_int;
    let mut val1: u16 = 0;
    let mut val2: u16 = 0;

    if let Some(get) = (*wm).ctl[n as usize].get {
        get(wm, &mut val1, &mut val2);
    } else {
        val1 = (*wm).regs[(*wm).ctl[n as usize].reg1 as usize] & (*wm).ctl[n as usize].mask1;
        val1 >>= __ffs((*wm).ctl[n as usize].mask1) as u16;
        if ((*wm).ctl[n as usize].flags & WM8766_FLAG_STEREO) != 0 {
            val2 = (*wm).regs[(*wm).ctl[n as usize].reg2 as usize] & (*wm).ctl[n as usize].mask2;
            val2 >>= __ffs((*wm).ctl[n as usize].mask2) as u16;
            if ((*wm).ctl[n as usize].flags & WM8766_FLAG_VOL_UPDATE) != 0 {
                val2 &= !WM8766_VOL_UPDATE;
            }
        }
    }
    if ((*wm).ctl[n as usize].flags & WM8766_FLAG_INVERT) != 0 {
        val1 = (*wm).ctl[n as usize].max - (val1 - (*wm).ctl[n as usize].min);
        if ((*wm).ctl[n as usize].flags & WM8766_FLAG_STEREO) != 0 {
            val2 = (*wm).ctl[n as usize].max - (val2 - (*wm).ctl[n as usize].min);
        }
    }
    (*ucontrol).value.integer.value[0] = val1 as _;
    if ((*wm).ctl[n as usize].flags & WM8766_FLAG_STEREO) != 0 {
        (*ucontrol).value.integer.value[1] = val2 as _;
    }

    0
}

unsafe extern "C" fn snd_wm8766_ctl_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let wm: *mut snd_wm8766 = snd_kcontrol_chip(kcontrol);
    let n: c_int = (*kcontrol).private_value as c_int;
    let mut val: u16;
    let mut regval1: u16;
    let mut regval2: u16;

    /* this also works for enum because value is a union */
    regval1 = (*ucontrol).value.integer.value[0] as u16;
    regval2 = (*ucontrol).value.integer.value[1] as u16;
    if ((*wm).ctl[n as usize].flags & WM8766_FLAG_INVERT) != 0 {
        regval1 = (*wm).ctl[n as usize].max - (regval1 - (*wm).ctl[n as usize].min);
        regval2 = (*wm).ctl[n as usize].max - (regval2 - (*wm).ctl[n as usize].min);
    }
    if let Some(set) = (*wm).ctl[n as usize].set {
        set(wm, regval1, regval2);
    } else {
        val = (*wm).regs[(*wm).ctl[n as usize].reg1 as usize] & !(*wm).ctl[n as usize].mask1;
        val |= regval1 << __ffs((*wm).ctl[n as usize].mask1);
        /* both stereo controls in one register */
        if ((*wm).ctl[n as usize].flags & WM8766_FLAG_STEREO) != 0
            && (*wm).ctl[n as usize].reg1 == (*wm).ctl[n as usize].reg2
        {
            val &= !(*wm).ctl[n as usize].mask2;
            val |= regval2 << __ffs((*wm).ctl[n as usize].mask2);
        }
        snd_wm8766_write(wm, (*wm).ctl[n as usize].reg1, val);
        /* stereo controls in different registers */
        if ((*wm).ctl[n as usize].flags & WM8766_FLAG_STEREO) != 0
            && (*wm).ctl[n as usize].reg1 != (*wm).ctl[n as usize].reg2
        {
            val = (*wm).regs[(*wm).ctl[n as usize].reg2 as usize] & !(*wm).ctl[n as usize].mask2;
            val |= regval2 << __ffs((*wm).ctl[n as usize].mask2);
            if ((*wm).ctl[n as usize].flags & WM8766_FLAG_VOL_UPDATE) != 0 {
                val |= WM8766_VOL_UPDATE;
            }
            snd_wm8766_write(wm, (*wm).ctl[n as usize].reg2, val);
        }
    }

    0
}

unsafe fn snd_wm8766_add_control(wm: *mut snd_wm8766, num: c_int) -> c_int {
    let mut cont: snd_kcontrol_new = zeroed();
    let mut ctl: *mut snd_kcontrol;

    cont.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    cont.private_value = num as _;
    cont.name = (*wm).ctl[num as usize].name;
    cont.access = SNDRV_CTL_ELEM_ACCESS_READWRITE;
    if ((*wm).ctl[num as usize].flags & WM8766_FLAG_LIM) != 0
        || ((*wm).ctl[num as usize].flags & WM8766_FLAG_ALC) != 0
    {
        cont.access |= SNDRV_CTL_ELEM_ACCESS_INACTIVE;
    }
    cont.tlv.p = null();
    cont.get = Some(snd_wm8766_ctl_get);
    cont.put = Some(snd_wm8766_ctl_put);

    match (*wm).ctl[num as usize].type_ {
        SNDRV_CTL_ELEM_TYPE_INTEGER => {
            cont.info = Some(snd_wm8766_volume_info);
            cont.access |= SNDRV_CTL_ELEM_ACCESS_TLV_READ;
            cont.tlv.p = (*wm).ctl[num as usize].tlv;
        }
        SNDRV_CTL_ELEM_TYPE_BOOLEAN => {
            (*wm).ctl[num as usize].max = 1;
            if ((*wm).ctl[num as usize].flags & WM8766_FLAG_STEREO) != 0 {
                cont.info = Some(snd_ctl_boolean_stereo_info);
            } else {
                cont.info = Some(snd_ctl_boolean_mono_info);
            }
        }
        SNDRV_CTL_ELEM_TYPE_ENUMERATED => {
            cont.info = Some(snd_wm8766_enum_info);
        }
        _ => {
            return -EINVAL;
        }
    }
    ctl = snd_ctl_new1(&mut cont, wm as *mut _);
    if ctl.is_null() {
        return -ENOMEM;
    }
    (*wm).ctl[num as usize].kctl = ctl;

    snd_ctl_add((*wm).card, ctl)
}

pub unsafe extern "C" fn snd_wm8766_build_controls(wm: *mut snd_wm8766) -> c_int {
    let mut err: c_int;
    let mut i: c_int = 0;

    while i < WM8766_CTL_COUNT {
        if !(*wm).ctl[i as usize].name.is_null() {
            err = snd_wm8766_add_control(wm, i);
            if err < 0 {
                return err;
            }
        }
        i += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
