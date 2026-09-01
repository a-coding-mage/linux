// SPDX-License-Identifier: GPL-2.0-or-later
/***************************************************************************
			  msnd_pinnacle_mixer.c  -  description
			     -------------------
    begin		: Fre Jun 7 2002
    copyright 		: (C) 2002 by karsten wiese
    email		: annabellesgarden@yahoo.de
 ***************************************************************************/

/***************************************************************************
 *							      		   *
 *									   *
 ***************************************************************************/

// Translated from the C implementation source. Kernel/ALSA headers and the
// msnd device definitions are expected to be supplied by surrounding bindings.
use crate::*;

const MSND_MIXER_VOLUME: usize = 0;
const MSND_MIXER_PCM: usize = 1;
const MSND_MIXER_AUX: usize = 2; /* Input source 1  (aux1) */
const MSND_MIXER_IMIX: usize = 3; /*  Recording monitor  */
const MSND_MIXER_SYNTH: usize = 4;
const MSND_MIXER_SPEAKER: usize = 5;
const MSND_MIXER_LINE: usize = 6;
const MSND_MIXER_MIC: usize = 7;
const MSND_MIXER_RECLEV: usize = 11; /* Recording level */
const MSND_MIXER_IGAIN: usize = 12; /* Input gain */
const MSND_MIXER_OGAIN: usize = 13; /* Output gain */
const MSND_MIXER_DIGITAL: usize = 17; /* Digital (input) 1 */

/*	Device mask bits	*/

const MSND_MASK_VOLUME: u32 = 1u32 << MSND_MIXER_VOLUME;
const MSND_MASK_SYNTH: u32 = 1u32 << MSND_MIXER_SYNTH;
const MSND_MASK_PCM: u32 = 1u32 << MSND_MIXER_PCM;
const MSND_MASK_SPEAKER: u32 = 1u32 << MSND_MIXER_SPEAKER;
const MSND_MASK_LINE: u32 = 1u32 << MSND_MIXER_LINE;
const MSND_MASK_MIC: u32 = 1u32 << MSND_MIXER_MIC;
const MSND_MASK_IMIX: u32 = 1u32 << MSND_MIXER_IMIX;
const MSND_MASK_RECLEV: u32 = 1u32 << MSND_MIXER_RECLEV;
const MSND_MASK_IGAIN: u32 = 1u32 << MSND_MIXER_IGAIN;
const MSND_MASK_OGAIN: u32 = 1u32 << MSND_MIXER_OGAIN;
const MSND_MASK_AUX: u32 = 1u32 << MSND_MIXER_AUX;
const MSND_MASK_DIGITAL: u32 = 1u32 << MSND_MIXER_DIGITAL;

unsafe extern "C" fn snd_msndmix_info_mux(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static TEXTS: [*const c_char; 3] = [
        c"Analog".as_ptr(),
        c"MASS".as_ptr(),
        c"SPDIF".as_ptr(),
    ];
    let chip: *mut snd_msnd = snd_kcontrol_chip(kcontrol);
    let items: c_uint = if test_bit(F_HAVEDIGITAL, &mut (*chip).flags) {
        3
    } else {
        2
    };

    snd_ctl_enum_info(uinfo, 1, items, TEXTS.as_ptr())
}

unsafe extern "C" fn snd_msndmix_get_mux(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_msnd = snd_kcontrol_chip(kcontrol);
    /* MSND_MASK_IMIX is the default */
    (*ucontrol).value.enumerated.item[0] = 0;

    if (*chip).recsrc & MSND_MASK_SYNTH != 0 {
        (*ucontrol).value.enumerated.item[0] = 1;
    } else if ((*chip).recsrc & MSND_MASK_DIGITAL != 0)
        && test_bit(F_HAVEDIGITAL, &mut (*chip).flags)
    {
        (*ucontrol).value.enumerated.item[0] = 2;
    }

    0
}

unsafe fn snd_msndmix_set_mux(chip: *mut snd_msnd, val: c_int) -> c_int {
    let newrecsrc: c_uint;
    let mut change: c_int;
    let msndbyte: u8;

    match val {
        0 => {
            newrecsrc = MSND_MASK_IMIX;
            msndbyte = HDEXAR_SET_ANA_IN;
        }
        1 => {
            newrecsrc = MSND_MASK_SYNTH;
            msndbyte = HDEXAR_SET_SYNTH_IN;
        }
        2 => {
            newrecsrc = MSND_MASK_DIGITAL;
            msndbyte = HDEXAR_SET_DAT_IN;
        }
        _ => return -EINVAL,
    }
    change = (newrecsrc != (*chip).recsrc) as c_int;
    if change != 0 {
        change = 0;
        if snd_msnd_send_word(chip, 0, 0, msndbyte) == 0 {
            if snd_msnd_send_dsp_cmd(chip, HDEX_AUX_REQ) == 0 {
                (*chip).recsrc = newrecsrc;
                change = 1;
            }
        }
    }
    change
}

unsafe extern "C" fn snd_msndmix_put_mux(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let msnd: *mut snd_msnd = snd_kcontrol_chip(kcontrol);
    snd_msndmix_set_mux(msnd, (*ucontrol).value.enumerated.item[0] as c_int)
}

unsafe extern "C" fn snd_msndmix_volume_info(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 100;
    0
}

unsafe extern "C" fn snd_msndmix_volume_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let msnd: *mut snd_msnd = snd_kcontrol_chip(kcontrol);
    let addr: usize = (*kcontrol).private_value as usize;

    let _guard = spinlock_irqsave(&mut (*msnd).mixer_lock);
    (*ucontrol).value.integer.value[0] = ((*msnd).left_levels[addr] * 100 / 0xffff) as c_long;
    (*ucontrol).value.integer.value[1] = ((*msnd).right_levels[addr] * 100 / 0xffff) as c_long;
    0
}

unsafe fn update_volm(dev: *mut snd_msnd, a: usize, left: usize, right: usize) {
    writew(
        (((*dev).left_levels[a] >> 1) * readw((*dev).SMA.add(SMA_wCurrMastVolLeft)) / 0xffff) as u16,
        (*dev).SMA.add(left),
    );
    writew(
        (((*dev).right_levels[a] >> 1) * readw((*dev).SMA.add(SMA_wCurrMastVolRight)) / 0xffff)
            as u16,
        (*dev).SMA.add(right),
    );
}

unsafe fn update_potm(dev: *mut snd_msnd, d: usize, left: usize, right: usize, ar: u8) {
    writeb(
        (((*dev).left_levels[d] >> 8) * readw((*dev).SMA.add(SMA_wCurrMastVolLeft)) / 0xffff)
            as u8,
        (*dev).SMA.add(left),
    );
    writeb(
        (((*dev).right_levels[d] >> 8) * readw((*dev).SMA.add(SMA_wCurrMastVolRight)) / 0xffff)
            as u8,
        (*dev).SMA.add(right),
    );
    if snd_msnd_send_word(dev, 0, 0, ar) == 0 {
        snd_msnd_send_dsp_cmd(dev, HDEX_AUX_REQ);
    }
}

unsafe fn update_pot(dev: *mut snd_msnd, d: usize, left: usize, right: usize, ar: u8) {
    writeb(((*dev).left_levels[d] >> 8) as u8, (*dev).SMA.add(left));
    writeb(((*dev).right_levels[d] >> 8) as u8, (*dev).SMA.add(right));
    if snd_msnd_send_word(dev, 0, 0, ar) == 0 {
        snd_msnd_send_dsp_cmd(dev, HDEX_AUX_REQ);
    }
}

unsafe fn snd_msndmix_set(dev: *mut snd_msnd, d: c_int, left: c_int, right: c_int) -> c_int {
    let bLeft: c_int;
    let bRight: c_int;
    let wLeft: c_int;
    let wRight: c_int;
    let mut updatemaster: c_int = 0;
    let d_usize = d as usize;

    if d >= LEVEL_ENTRIES {
        return -EINVAL;
    }

    bLeft = left * 0xff / 100;
    wLeft = left * 0xffff / 100;

    bRight = right * 0xff / 100;
    wRight = right * 0xffff / 100;

    (*dev).left_levels[d_usize] = wLeft as _;
    (*dev).right_levels[d_usize] = wRight as _;

    match d_usize {
        /* master volume unscaled controls */
        MSND_MIXER_LINE => {
            /* line pot control */
            /* scaled by IMIX in digital mix */
            writeb(bLeft as u8, (*dev).SMA.add(SMA_bInPotPosLeft));
            writeb(bRight as u8, (*dev).SMA.add(SMA_bInPotPosRight));
            if snd_msnd_send_word(dev, 0, 0, HDEXAR_IN_SET_POTS) == 0 {
                snd_msnd_send_dsp_cmd(dev, HDEX_AUX_REQ);
            }
        }
        MSND_MIXER_MIC => {
            /* mic pot control */
            if (*dev).type_ == msndClassic {
                return -EINVAL;
            }
            /* scaled by IMIX in digital mix */
            writeb(bLeft as u8, (*dev).SMA.add(SMA_bMicPotPosLeft));
            writeb(bRight as u8, (*dev).SMA.add(SMA_bMicPotPosRight));
            if snd_msnd_send_word(dev, 0, 0, HDEXAR_MIC_SET_POTS) == 0 {
                snd_msnd_send_dsp_cmd(dev, HDEX_AUX_REQ);
            }
        }
        MSND_MIXER_VOLUME | MSND_MIXER_AUX | MSND_MIXER_SYNTH | MSND_MIXER_PCM
        | MSND_MIXER_IMIX => {
            if d_usize == MSND_MIXER_VOLUME {
                /* master volume */
                writew(wLeft as u16, (*dev).SMA.add(SMA_wCurrMastVolLeft));
                writew(wRight as u16, (*dev).SMA.add(SMA_wCurrMastVolRight));
            }
            /*
             * MSND_MIXER_AUX: aux pot control, scaled by master volume
             * MSND_MIXER_SYNTH: synth vol (dsp mix), scaled by master volume
             * MSND_MIXER_PCM: pcm vol (dsp mix), scaled by master volume
             * MSND_MIXER_IMIX: input monitor (dsp mix), scaled by master volume
             */
            updatemaster = 1;
        }
        _ => return -EINVAL,
    }

    if updatemaster != 0 {
        /* update master volume scaled controls */
        update_volm(dev, MSND_MIXER_PCM, SMA_wCurrPlayVolLeft, SMA_wCurrPlayVolRight);
        update_volm(dev, MSND_MIXER_IMIX, SMA_wCurrInVolLeft, SMA_wCurrInVolRight);
        if (*dev).type_ == msndPinnacle {
            update_volm(dev, MSND_MIXER_SYNTH, SMA_wCurrMHdrVolLeft, SMA_wCurrMHdrVolRight);
        }
        update_potm(
            dev,
            MSND_MIXER_AUX,
            SMA_bAuxPotPosLeft,
            SMA_bAuxPotPosRight,
            HDEXAR_AUX_SET_POTS,
        );
    }

    0
}

unsafe extern "C" fn snd_msndmix_volume_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let msnd: *mut snd_msnd = snd_kcontrol_chip(kcontrol);
    let addr: usize = (*kcontrol).private_value as usize;
    let change: c_int;
    let left: c_int;
    let right: c_int;

    left = ((*ucontrol).value.integer.value[0] % 101) as c_int;
    right = ((*ucontrol).value.integer.value[1] % 101) as c_int;
    let _guard = spinlock_irqsave(&mut (*msnd).mixer_lock);
    change = ((*msnd).left_levels[addr] != left as _
        || (*msnd).right_levels[addr] != right as _) as c_int;
    snd_msndmix_set(msnd, addr as c_int, left, right);
    change
}

macro_rules! DUMMY_VOLUME {
    ($xname:literal, $xindex:expr, $addr:expr) => {
        snd_kcontrol_new {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: c_str!($xname),
            index: $xindex,
            info: Some(snd_msndmix_volume_info),
            get: Some(snd_msndmix_volume_get),
            put: Some(snd_msndmix_volume_put),
            private_value: $addr as _,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

static snd_msnd_controls: [snd_kcontrol_new; 7] = [
    DUMMY_VOLUME!("Master Volume", 0, MSND_MIXER_VOLUME),
    DUMMY_VOLUME!("PCM Volume", 0, MSND_MIXER_PCM),
    DUMMY_VOLUME!("Aux Volume", 0, MSND_MIXER_AUX),
    DUMMY_VOLUME!("Line Volume", 0, MSND_MIXER_LINE),
    DUMMY_VOLUME!("Mic Volume", 0, MSND_MIXER_MIC),
    DUMMY_VOLUME!("Monitor", 0, MSND_MIXER_IMIX),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Capture Source".as_ptr(),
        info: Some(snd_msndmix_info_mux),
        get: Some(snd_msndmix_get_mux),
        put: Some(snd_msndmix_put_mux),
        ..unsafe { core::mem::zeroed() }
    },
];

#[no_mangle]
pub unsafe extern "C" fn snd_msndmix_new(card: *mut snd_card) -> c_int {
    let chip: *mut snd_msnd = (*card).private_data as *mut snd_msnd;
    let mut idx: c_uint;
    let mut err: c_int;

    if snd_BUG_ON(chip.is_null()) != 0 {
        return -EINVAL;
    }
    spin_lock_init(&mut (*chip).mixer_lock);
    strscpy((*card).mixername.as_mut_ptr(), c"MSND Pinnacle Mixer".as_ptr());

    idx = 0;
    while (idx as usize) < snd_msnd_controls.len() {
        err = snd_ctl_add(
            card,
            snd_ctl_new1(snd_msnd_controls.as_ptr().add(idx as usize), chip as *mut c_void),
        );
        if err < 0 {
            return err;
        }
        idx += 1;
    }

    0
}
// EXPORT_SYMBOL(snd_msndmix_new);

#[no_mangle]
pub unsafe extern "C" fn snd_msndmix_setup(dev: *mut snd_msnd) {
    writew(
        (*dev).left_levels[MSND_MIXER_VOLUME] as u16,
        (*dev).SMA.add(SMA_wCurrMastVolLeft),
    );
    writew(
        (*dev).right_levels[MSND_MIXER_VOLUME] as u16,
        (*dev).SMA.add(SMA_wCurrMastVolRight),
    );
    update_pot(
        dev,
        MSND_MIXER_LINE,
        SMA_bInPotPosLeft,
        SMA_bInPotPosRight,
        HDEXAR_IN_SET_POTS,
    );
    update_potm(
        dev,
        MSND_MIXER_AUX,
        SMA_bAuxPotPosLeft,
        SMA_bAuxPotPosRight,
        HDEXAR_AUX_SET_POTS,
    );
    update_volm(dev, MSND_MIXER_PCM, SMA_wCurrPlayVolLeft, SMA_wCurrPlayVolRight);
    update_volm(dev, MSND_MIXER_IMIX, SMA_wCurrInVolLeft, SMA_wCurrInVolRight);
    if (*dev).type_ == msndPinnacle {
        update_pot(
            dev,
            MSND_MIXER_MIC,
            SMA_bMicPotPosLeft,
            SMA_bMicPotPosRight,
            HDEXAR_MIC_SET_POTS,
        );
        update_volm(dev, MSND_MIXER_SYNTH, SMA_wCurrMHdrVolLeft, SMA_wCurrMHdrVolRight);
    }
}
// EXPORT_SYMBOL(snd_msndmix_setup);

#[no_mangle]
pub unsafe extern "C" fn snd_msndmix_force_recsrc(dev: *mut snd_msnd, recsrc: c_int) -> c_int {
    (*dev).recsrc = -1i32 as _;
    snd_msndmix_set_mux(dev, recsrc)
}
// EXPORT_SYMBOL(snd_msndmix_force_recsrc);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
