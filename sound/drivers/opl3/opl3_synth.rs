// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Copyright (c) by Uros Bizjak <uros@kss-loka.si>
 *
 *  Routines for OPL2/OPL3/OPL4 control
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, size_of_val, zeroed};
use core::ptr;

/*
 * CONFIG_SND_SEQUENCER enables OPL3_SUPPORT_SYNTH in the original C source.
 * The synth-gated code below is translated and kept under the corresponding
 * Rust cfg name for the build system to map.
 */

/*
 *    There is 18 possible 2 OP voices
 *      (9 in the left and 9 in the right).
 *      The first OP is the modulator and 2nd is the carrier.
 *
 *      The first three voices in the both sides may be connected
 *      with another voice to a 4 OP voice. For example voice 0
 *      can be connected with voice 3. The operators of voice 3 are
 *      used as operators 3 and 4 of the new 4 OP voice.
 *      In this case the 2 OP voice number 0 is the 'first half' and
 *      voice 3 is the second.
 */

/*
 *    Register offset table for OPL2/3 voices,
 *    OPL2 / one OPL3 register array side only
 */
#[no_mangle]
pub static mut snd_opl3_regmap: [[c_char; 4]; MAX_OPL2_VOICES as usize] = [
    /*    OP1   OP2   OP3   OP4         */
    /*   ------------------------       */
    [0x00, 0x03, 0x08, 0x0b],
    [0x01, 0x04, 0x09, 0x0c],
    [0x02, 0x05, 0x0a, 0x0d],
    [0x08, 0x0b, 0x00, 0x00],
    [0x09, 0x0c, 0x00, 0x00],
    [0x0a, 0x0d, 0x00, 0x00],
    [0x10, 0x13, 0x00, 0x00], /* used by percussive voices */
    [0x11, 0x14, 0x00, 0x00], /* if the percussive mode */
    [0x12, 0x15, 0x00, 0x00], /* is selected (only left reg block) */
];

/* EXPORT_SYMBOL(snd_opl3_regmap); */

unsafe extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> c_ulong;
    fn array_index_nospec(index: c_uint, size: c_uint) -> c_uint;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *const c_void);
    fn snd_BUG_ON(condition: bool) -> bool;
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
}

/*
 * open the device exclusively
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_open(_hw: *mut snd_hwdep, _file: *mut file) -> c_int {
    0
}

/*
 * ioctl for hwdep device:
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_ioctl(
    hw: *mut snd_hwdep,
    _file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_int {
    let opl3: *mut snd_opl3 = (*hw).private_data as *mut snd_opl3;
    let argp: *mut c_void = arg as *mut c_void;

    if snd_BUG_ON(opl3.is_null()) {
        return -EINVAL;
    }

    match cmd {
        SNDRV_DM_FM_IOCTL_INFO => {
            let mut info: snd_dm_fm_info = zeroed();

            info.fm_mode = (*opl3).fm_mode;
            info.rhythm = (*opl3).rhythm;
            if copy_to_user(
                argp,
                &info as *const snd_dm_fm_info as *const c_void,
                size_of::<snd_dm_fm_info>(),
            ) != 0
            {
                return -EFAULT;
            }
            0
        }

        SNDRV_DM_FM_IOCTL_RESET
        /* CONFIG_SND_OSSEMUL: SNDRV_DM_FM_OSS_IOCTL_RESET */ => {
            snd_opl3_reset(opl3);
            0
        }

        SNDRV_DM_FM_IOCTL_PLAY_NOTE
        /* CONFIG_SND_OSSEMUL: SNDRV_DM_FM_OSS_IOCTL_PLAY_NOTE */ => {
            let mut note: snd_dm_fm_note = zeroed();
            if copy_from_user(
                &mut note as *mut snd_dm_fm_note as *mut c_void,
                argp,
                size_of::<snd_dm_fm_note>(),
            ) != 0
            {
                return -EFAULT;
            }
            snd_opl3_play_note(opl3, &mut note)
        }

        SNDRV_DM_FM_IOCTL_SET_VOICE
        /* CONFIG_SND_OSSEMUL: SNDRV_DM_FM_OSS_IOCTL_SET_VOICE */ => {
            let mut voice: snd_dm_fm_voice = zeroed();
            if copy_from_user(
                &mut voice as *mut snd_dm_fm_voice as *mut c_void,
                argp,
                size_of::<snd_dm_fm_voice>(),
            ) != 0
            {
                return -EFAULT;
            }
            snd_opl3_set_voice(opl3, &mut voice)
        }

        SNDRV_DM_FM_IOCTL_SET_PARAMS
        /* CONFIG_SND_OSSEMUL: SNDRV_DM_FM_OSS_IOCTL_SET_PARAMS */ => {
            let mut params: snd_dm_fm_params = zeroed();
            if copy_from_user(
                &mut params as *mut snd_dm_fm_params as *mut c_void,
                argp,
                size_of::<snd_dm_fm_params>(),
            ) != 0
            {
                return -EFAULT;
            }
            snd_opl3_set_params(opl3, &mut params)
        }

        SNDRV_DM_FM_IOCTL_SET_MODE
        /* CONFIG_SND_OSSEMUL: SNDRV_DM_FM_OSS_IOCTL_SET_MODE */ => {
            snd_opl3_set_mode(opl3, arg as c_int)
        }

        SNDRV_DM_FM_IOCTL_SET_CONNECTION
        /* CONFIG_SND_OSSEMUL: SNDRV_DM_FM_OSS_IOCTL_SET_OPL */ => {
            snd_opl3_set_connection(opl3, arg as c_int)
        }

        #[cfg(OPL3_SUPPORT_SYNTH)]
        SNDRV_DM_FM_IOCTL_CLEAR_PATCHES => {
            snd_opl3_clear_patches(opl3);
            0
        }

        _ => {
            dev_dbg(
                (*(*opl3).card).dev,
                b"unknown IOCTL: 0x%x\n\0".as_ptr() as *const c_char,
                cmd,
            );
            -ENOTTY
        }
    }
}

/*
 * close the device
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_release(hw: *mut snd_hwdep, _file: *mut file) -> c_int {
    let opl3: *mut snd_opl3 = (*hw).private_data as *mut snd_opl3;

    snd_opl3_reset(opl3);
    0
}

#[cfg(OPL3_SUPPORT_SYNTH)]
/*
 * write the device - load patches
 */
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_write(
    hw: *mut snd_hwdep,
    mut buf: *const c_char,
    mut count: c_long,
    _offset: *mut loff_t,
) -> c_long {
    let opl3: *mut snd_opl3 = (*hw).private_data as *mut snd_opl3;
    let mut result: c_long = 0;
    let mut err: c_int = 0;
    let mut inst: sbi_patch = zeroed();

    while count >= size_of::<sbi_patch>() as c_long {
        let type_: u8;
        if copy_from_user(
            &mut inst as *mut sbi_patch as *mut c_void,
            buf as *const c_void,
            size_of::<sbi_patch>(),
        ) != 0
        {
            return -EFAULT as c_long;
        }
        if memcmp(inst.key.as_ptr() as *const c_void, FM_KEY_SBI.as_ptr() as *const c_void, 4) == 0
            || memcmp(inst.key.as_ptr() as *const c_void, FM_KEY_2OP.as_ptr() as *const c_void, 4) == 0
        {
            type_ = FM_PATCH_OPL2 as u8;
        } else if memcmp(inst.key.as_ptr() as *const c_void, FM_KEY_4OP.as_ptr() as *const c_void, 4) == 0 {
            type_ = FM_PATCH_OPL3 as u8;
        } else {
            /* invalid type */
            break;
        }
        err = snd_opl3_load_patch(
            opl3,
            inst.prog as c_int,
            inst.bank as c_int,
            type_ as c_int,
            inst.name.as_ptr(),
            inst.extension.as_ptr(),
            inst.data.as_ptr(),
        );
        if err < 0 {
            break;
        }
        result += size_of::<sbi_patch>() as c_long;
        count -= size_of::<sbi_patch>() as c_long;
        buf = buf.add(size_of::<sbi_patch>());
    }
    if result > 0 { result } else { err as c_long }
}

/*
 * Patch management
 */

/* offsets for SBI params */
const AM_VIB: usize = 0;
const KSL_LEVEL: usize = 2;
const ATTACK_DECAY: usize = 4;
const SUSTAIN_RELEASE: usize = 6;
const WAVE_SELECT: usize = 8;

/* offset for SBI instrument */
const CONNECTION: usize = 10;
const OFFSET_4OP: usize = 11;

/*
 * load a patch, obviously.
 *
 * loaded on the given program and bank numbers with the given type
 * (FM_PATCH_OPLx).
 * data is the pointer of SBI record _without_ header (key and name).
 * name is the name string of the patch.
 * ext is the extension data of 7 bytes long (stored in name of SBI
 * data up to offset 25), or NULL to skip.
 * return 0 if successful or a negative error code.
 */
#[cfg(OPL3_SUPPORT_SYNTH)]
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_load_patch(
    opl3: *mut snd_opl3,
    prog: c_int,
    bank: c_int,
    type_: c_int,
    name: *const c_char,
    ext: *const u8,
    data: *const u8,
) -> c_int {
    let mut i: c_int;

    let patch: *mut fm_patch = snd_opl3_find_patch(opl3, prog, bank, 1);
    if patch.is_null() {
        return -ENOMEM;
    }

    (*patch).type_ = type_;

    i = 0;
    while i < 2 {
        (*patch).inst.op[i as usize].am_vib = *data.add(AM_VIB + i as usize);
        (*patch).inst.op[i as usize].ksl_level = *data.add(KSL_LEVEL + i as usize);
        (*patch).inst.op[i as usize].attack_decay = *data.add(ATTACK_DECAY + i as usize);
        (*patch).inst.op[i as usize].sustain_release = *data.add(SUSTAIN_RELEASE + i as usize);
        (*patch).inst.op[i as usize].wave_select = *data.add(WAVE_SELECT + i as usize);
        i += 1;
    }
    (*patch).inst.feedback_connection[0] = *data.add(CONNECTION);

    if type_ == FM_PATCH_OPL3 {
        i = 0;
        while i < 2 {
            (*patch).inst.op[(i + 2) as usize].am_vib = *data.add(OFFSET_4OP + AM_VIB + i as usize);
            (*patch).inst.op[(i + 2) as usize].ksl_level = *data.add(OFFSET_4OP + KSL_LEVEL + i as usize);
            (*patch).inst.op[(i + 2) as usize].attack_decay = *data.add(OFFSET_4OP + ATTACK_DECAY + i as usize);
            (*patch).inst.op[(i + 2) as usize].sustain_release = *data.add(OFFSET_4OP + SUSTAIN_RELEASE + i as usize);
            (*patch).inst.op[(i + 2) as usize].wave_select = *data.add(OFFSET_4OP + WAVE_SELECT + i as usize);
            i += 1;
        }
        (*patch).inst.feedback_connection[1] = *data.add(OFFSET_4OP + CONNECTION);
    }

    if !ext.is_null() {
        (*patch).inst.echo_delay = *ext.add(0);
        (*patch).inst.echo_atten = *ext.add(1);
        (*patch).inst.chorus_spread = *ext.add(2);
        (*patch).inst.trnsps = *ext.add(3);
        (*patch).inst.fix_dur = *ext.add(4);
        (*patch).inst.modes = *ext.add(5);
        (*patch).inst.fix_key = *ext.add(6);
    }

    if !name.is_null() {
        strscpy(
            (*patch).name.as_mut_ptr(),
            name,
            size_of_val(&(*patch).name),
        );
    }

    0
}
/* EXPORT_SYMBOL(snd_opl3_load_patch); */

/*
 * find a patch with the given program and bank numbers, returns its pointer
 * if no matching patch is found and create_patch is set, it creates a
 * new patch object.
 */
#[cfg(OPL3_SUPPORT_SYNTH)]
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_find_patch(
    opl3: *mut snd_opl3,
    prog: c_int,
    bank: c_int,
    create_patch: c_int,
) -> *mut fm_patch {
    /* pretty dumb hash key */
    let key: c_uint = ((prog + bank) as c_uint) % OPL3_PATCH_HASH_SIZE;
    let mut patch: *mut fm_patch;

    patch = (*opl3).patch_table[key as usize];
    while !patch.is_null() {
        if (*patch).prog == prog && (*patch).bank == bank {
            return patch;
        }
        patch = (*patch).next;
    }
    if create_patch == 0 {
        return ptr::null_mut();
    }

    patch = kzalloc(size_of::<fm_patch>(), GFP_KERNEL) as *mut fm_patch;
    if patch.is_null() {
        return ptr::null_mut();
    }
    (*patch).prog = prog;
    (*patch).bank = bank;
    (*patch).next = (*opl3).patch_table[key as usize];
    (*opl3).patch_table[key as usize] = patch;
    patch
}
/* EXPORT_SYMBOL(snd_opl3_find_patch); */

/*
 * Clear all patches of the given OPL3 instance
 */
#[cfg(OPL3_SUPPORT_SYNTH)]
#[no_mangle]
pub unsafe extern "C" fn snd_opl3_clear_patches(opl3: *mut snd_opl3) {
    let mut i: c_int = 0;
    while i < OPL3_PATCH_HASH_SIZE as c_int {
        let mut patch: *mut fm_patch = (*opl3).patch_table[i as usize];
        let mut next: *mut fm_patch;
        while !patch.is_null() {
            next = (*patch).next;
            kfree(patch as *const c_void);
            patch = next;
        }
        i += 1;
    }
    memset(
        (*opl3).patch_table.as_mut_ptr() as *mut c_void,
        0,
        size_of_val(&(*opl3).patch_table),
    );
}
/* endif OPL3_SUPPORT_SYNTH */

/* ------------------------------ */

#[no_mangle]
pub unsafe extern "C" fn snd_opl3_reset(opl3: *mut snd_opl3) {
    let mut opl3_reg: u16;

    let mut reg_side: u16;
    let mut voice_offset: u8;

    let max_voices: c_int;
    let mut i: c_int;

    max_voices = if (*opl3).hardware < OPL3_HW_OPL3 {
        MAX_OPL2_VOICES
    } else {
        MAX_OPL3_VOICES
    };

    i = 0;
    while i < max_voices {
        /* Get register array side and offset of voice */
        if i < MAX_OPL2_VOICES {
            /* Left register block for voices 0 .. 8 */
            reg_side = OPL3_LEFT;
            voice_offset = i as u8;
        } else {
            /* Right register block for voices 9 .. 17 */
            reg_side = OPL3_RIGHT;
            voice_offset = (i - MAX_OPL2_VOICES) as u8;
        }
        opl3_reg = reg_side | (OPL3_REG_KSL_LEVEL + snd_opl3_regmap[voice_offset as usize][0] as u16);
        ((*opl3).command)(opl3, opl3_reg, OPL3_TOTAL_LEVEL_MASK); /* Operator 1 volume */
        opl3_reg = reg_side | (OPL3_REG_KSL_LEVEL + snd_opl3_regmap[voice_offset as usize][1] as u16);
        ((*opl3).command)(opl3, opl3_reg, OPL3_TOTAL_LEVEL_MASK); /* Operator 2 volume */

        opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK + voice_offset as u16);
        ((*opl3).command)(opl3, opl3_reg, 0x00); /* Note off */
        i += 1;
    }

    (*opl3).max_voices = MAX_OPL2_VOICES;
    (*opl3).fm_mode = SNDRV_DM_FM_MODE_OPL2;

    ((*opl3).command)(opl3, OPL3_LEFT | OPL3_REG_TEST, OPL3_ENABLE_WAVE_SELECT);
    ((*opl3).command)(opl3, OPL3_LEFT | OPL3_REG_PERCUSSION, 0x00); /* Melodic mode */
    (*opl3).rhythm = 0;
}

/* EXPORT_SYMBOL(snd_opl3_reset); */

unsafe fn snd_opl3_play_note(opl3: *mut snd_opl3, note: *mut snd_dm_fm_note) -> c_int {
    let reg_side: u16;
    let voice_offset: u8;

    let mut opl3_reg: u16;
    let mut reg_val: u8;

    /* Voices 0 -  8 in OPL2 mode */
    /* Voices 0 - 17 in OPL3 mode */
    if (*note).voice
        >= if (*opl3).fm_mode == SNDRV_DM_FM_MODE_OPL3 {
            MAX_OPL3_VOICES
        } else {
            MAX_OPL2_VOICES
        }
    {
        return -EINVAL;
    }

    /* Get register array side and offset of voice */
    if (*note).voice < MAX_OPL2_VOICES {
        /* Left register block for voices 0 .. 8 */
        reg_side = OPL3_LEFT;
        voice_offset = (*note).voice as u8;
    } else {
        /* Right register block for voices 9 .. 17 */
        reg_side = OPL3_RIGHT;
        voice_offset = ((*note).voice - MAX_OPL2_VOICES) as u8;
    }

    /* Set lower 8 bits of note frequency */
    reg_val = (*note).fnum as u8;
    opl3_reg = reg_side | (OPL3_REG_FNUM_LOW + voice_offset as u16);
    ((*opl3).command)(opl3, opl3_reg, reg_val);

    reg_val = 0x00;
    /* Set output sound flag */
    if (*note).key_on != 0 {
        reg_val |= OPL3_KEYON_BIT;
    }
    /* Set octave */
    reg_val |= (((*note).octave << 2) & OPL3_BLOCKNUM_MASK as c_int) as u8;
    /* Set higher 2 bits of note frequency */
    reg_val |= (((*note).fnum >> 8) as u8) & OPL3_FNUM_HIGH_MASK;

    /* Set OPL3 KEYON_BLOCK register of requested voice */
    opl3_reg = reg_side | (OPL3_REG_KEYON_BLOCK + voice_offset as u16);
    ((*opl3).command)(opl3, opl3_reg, reg_val);

    0
}

unsafe fn snd_opl3_set_voice(opl3: *mut snd_opl3, voice: *mut snd_dm_fm_voice) -> c_int {
    let reg_side: u16;
    let mut op_offset: u8;
    let mut voice_offset: u8;
    let voice_op: u8;

    let mut opl3_reg: u16;
    let mut reg_val: u8;

    /* Only operators 1 and 2 */
    if (*voice).op > 1 {
        return -EINVAL;
    }
    /* Voices 0 -  8 in OPL2 mode */
    /* Voices 0 - 17 in OPL3 mode */
    if (*voice).voice
        >= if (*opl3).fm_mode == SNDRV_DM_FM_MODE_OPL3 {
            MAX_OPL3_VOICES
        } else {
            MAX_OPL2_VOICES
        }
    {
        return -EINVAL;
    }

    /* Get register array side and offset of voice */
    if (*voice).voice < MAX_OPL2_VOICES {
        /* Left register block for voices 0 .. 8 */
        reg_side = OPL3_LEFT;
        voice_offset = (*voice).voice as u8;
    } else {
        /* Right register block for voices 9 .. 17 */
        reg_side = OPL3_RIGHT;
        voice_offset = ((*voice).voice - MAX_OPL2_VOICES) as u8;
    }
    /* Get register offset of operator */
    voice_offset = array_index_nospec(voice_offset as c_uint, MAX_OPL2_VOICES as c_uint) as u8;
    voice_op = array_index_nospec((*voice).op as c_uint, 4) as u8;
    op_offset = snd_opl3_regmap[voice_offset as usize][voice_op as usize] as u8;

    reg_val = 0x00;
    /* Set amplitude modulation (tremolo) effect */
    if (*voice).am != 0 {
        reg_val |= OPL3_TREMOLO_ON;
    }
    /* Set vibrato effect */
    if (*voice).vibrato != 0 {
        reg_val |= OPL3_VIBRATO_ON;
    }
    /* Set sustaining sound phase */
    if (*voice).do_sustain != 0 {
        reg_val |= OPL3_SUSTAIN_ON;
    }
    /* Set keyboard scaling bit */
    if (*voice).kbd_scale != 0 {
        reg_val |= OPL3_KSR;
    }
    /* Set harmonic or frequency multiplier */
    reg_val |= ((*voice).harmonic as u8) & OPL3_MULTIPLE_MASK;

    /* Set OPL3 AM_VIB register of requested voice/operator */
    opl3_reg = reg_side | (OPL3_REG_AM_VIB + op_offset as u16);
    ((*opl3).command)(opl3, opl3_reg, reg_val);

    /* Set decreasing volume of higher notes */
    reg_val = (((*voice).scale_level << 6) as u8) & OPL3_KSL_MASK;
    /* Set output volume */
    reg_val |= (!((*voice).volume as u8)) & OPL3_TOTAL_LEVEL_MASK;

    /* Set OPL3 KSL_LEVEL register of requested voice/operator */
    opl3_reg = reg_side | (OPL3_REG_KSL_LEVEL + op_offset as u16);
    ((*opl3).command)(opl3, opl3_reg, reg_val);

    /* Set attack phase level */
    reg_val = (((*voice).attack << 4) as u8) & OPL3_ATTACK_MASK;
    /* Set decay phase level */
    reg_val |= ((*voice).decay as u8) & OPL3_DECAY_MASK;

    /* Set OPL3 ATTACK_DECAY register of requested voice/operator */
    opl3_reg = reg_side | (OPL3_REG_ATTACK_DECAY + op_offset as u16);
    ((*opl3).command)(opl3, opl3_reg, reg_val);

    /* Set sustain phase level */
    reg_val = (((*voice).sustain << 4) as u8) & OPL3_SUSTAIN_MASK;
    /* Set release phase level */
    reg_val |= ((*voice).release as u8) & OPL3_RELEASE_MASK;

    /* Set OPL3 SUSTAIN_RELEASE register of requested voice/operator */
    opl3_reg = reg_side | (OPL3_REG_SUSTAIN_RELEASE + op_offset as u16);
    ((*opl3).command)(opl3, opl3_reg, reg_val);

    /* Set inter-operator feedback */
    reg_val = (((*voice).feedback << 1) as u8) & OPL3_FEEDBACK_MASK;
    /* Set inter-operator connection */
    if (*voice).connection != 0 {
        reg_val |= OPL3_CONNECTION_BIT;
    }
    /* OPL-3 only */
    if (*opl3).fm_mode == SNDRV_DM_FM_MODE_OPL3 {
        if (*voice).left != 0 {
            reg_val |= OPL3_VOICE_TO_LEFT;
        }
        if (*voice).right != 0 {
            reg_val |= OPL3_VOICE_TO_RIGHT;
        }
    }
    /* Feedback/connection bits are applicable to voice */
    opl3_reg = reg_side | (OPL3_REG_FEEDBACK_CONNECTION + voice_offset as u16);
    ((*opl3).command)(opl3, opl3_reg, reg_val);

    /* Select waveform */
    reg_val = ((*voice).waveform as u8) & OPL3_WAVE_SELECT_MASK;
    opl3_reg = reg_side | (OPL3_REG_WAVE_SELECT + op_offset as u16);
    ((*opl3).command)(opl3, opl3_reg, reg_val);

    0
}

unsafe fn snd_opl3_set_params(opl3: *mut snd_opl3, params: *mut snd_dm_fm_params) -> c_int {
    let mut reg_val: u8;

    reg_val = 0x00;
    /* Set keyboard split method */
    if (*params).kbd_split != 0 {
        reg_val |= OPL3_KEYBOARD_SPLIT;
    }
    ((*opl3).command)(opl3, OPL3_LEFT | OPL3_REG_KBD_SPLIT, reg_val);

    reg_val = 0x00;
    /* Set amplitude modulation (tremolo) depth */
    if (*params).am_depth != 0 {
        reg_val |= OPL3_TREMOLO_DEPTH;
    }
    /* Set vibrato depth */
    if (*params).vib_depth != 0 {
        reg_val |= OPL3_VIBRATO_DEPTH;
    }
    /* Set percussion mode */
    if (*params).rhythm != 0 {
        reg_val |= OPL3_PERCUSSION_ENABLE;
        (*opl3).rhythm = 1;
    } else {
        (*opl3).rhythm = 0;
    }
    /* Play percussion instruments */
    if (*params).bass != 0 {
        reg_val |= OPL3_BASSDRUM_ON;
    }
    if (*params).snare != 0 {
        reg_val |= OPL3_SNAREDRUM_ON;
    }
    if (*params).tomtom != 0 {
        reg_val |= OPL3_TOMTOM_ON;
    }
    if (*params).cymbal != 0 {
        reg_val |= OPL3_CYMBAL_ON;
    }
    if (*params).hihat != 0 {
        reg_val |= OPL3_HIHAT_ON;
    }

    ((*opl3).command)(opl3, OPL3_LEFT | OPL3_REG_PERCUSSION, reg_val);
    0
}

unsafe fn snd_opl3_set_mode(opl3: *mut snd_opl3, mode: c_int) -> c_int {
    if mode == SNDRV_DM_FM_MODE_OPL3 && (*opl3).hardware < OPL3_HW_OPL3 {
        return -EINVAL;
    }

    (*opl3).fm_mode = mode;
    if (*opl3).hardware >= OPL3_HW_OPL3 {
        ((*opl3).command)(opl3, OPL3_RIGHT | OPL3_REG_CONNECTION_SELECT, 0x00); /* Clear 4-op connections */
    }

    0
}

unsafe fn snd_opl3_set_connection(opl3: *mut snd_opl3, connection: c_int) -> c_int {
    let reg_val: u8;

    /* OPL-3 only */
    if (*opl3).fm_mode != SNDRV_DM_FM_MODE_OPL3 {
        return -EINVAL;
    }

    reg_val = (connection
        & (OPL3_RIGHT_4OP_0
            | OPL3_RIGHT_4OP_1
            | OPL3_RIGHT_4OP_2
            | OPL3_LEFT_4OP_0
            | OPL3_LEFT_4OP_1
            | OPL3_LEFT_4OP_2)) as u8;
    /* Set 4-op connections */
    ((*opl3).command)(opl3, OPL3_RIGHT | OPL3_REG_CONNECTION_SELECT, reg_val);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
