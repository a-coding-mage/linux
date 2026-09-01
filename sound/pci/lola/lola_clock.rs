// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Support for Digigram Lola PCI-e boards
 *
 *  Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
 */

/* Translated from implementation source pci/lola/lola_clock.c.
 * Kernel, ALSA, and local lola.h dependencies are declared here only as
 * external items or constants expected from the surrounding driver.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

pub const EBUSY: c_int = 16;
pub const EINVAL: c_int = 22;

pub const LOLA_MAXFREQ_AT_GRANULARITY_MIN: c_uint = 48000;
pub const LOLA_MAXFREQ_AT_GRANULARITY_BELOW_MAX: c_uint = 96000;

extern "C" {
    static LOLA_GRANULARITY_MIN: c_uint;
    static LOLA_GRANULARITY_MAX: c_uint;
    static LOLA_GRANULARITY_STEP: c_uint;
    static LOLA_VERB_SET_GRANULARITY_STEPS: c_uint;
    static LOLA_PAR_AUDIO_WIDGET_CAP: c_uint;
    static MAX_SAMPLE_CLOCK_COUNT: c_uint;
    static LOLA_VERB_GET_CLOCK_LIST: c_uint;
    static LOLA_CLOCK_FORMAT_NONE: c_int;
    static LOLA_CLOCK_TYPE_INTERNAL: c_uint;
    static LOLA_CLOCK_TYPE_VIDEO: c_uint;
    static LOLA_CLOCK_FORMAT_NTSC: c_int;
    static LOLA_CLOCK_FORMAT_PAL: c_int;
    static LOLA_VERB_SET_UNSOLICITED_ENABLE: c_uint;
    static LOLA_UNSOLICITED_ENABLE: c_uint;
    static LOLA_UNSOLICITED_TAG: c_uint;
    static LOLA_VERB_SET_CLOCK_SELECT: c_uint;
    static LOLA_UNSOL_RESP_TAG_OFFSET: c_uint;
    static LOLA_UNSOLICITED_TAG_MASK: c_uint;

    fn lola_codec_write(
        chip: *mut lola,
        nid: c_uint,
        verb: c_uint,
        data: c_uint,
        extdata: c_uint,
    ) -> c_int;
    fn lola_codec_flush(chip: *mut lola) -> c_int;
    fn usleep_range(min: c_uint, max: c_uint);
    fn lola_read_param(chip: *mut lola, nid: c_int, param: c_uint, val: *mut c_uint) -> c_int;
    fn lola_codec_read(
        chip: *mut lola,
        nid: c_int,
        verb: c_uint,
        data: c_uint,
        extdata: c_uint,
        res: *mut c_uint,
        res_ex: *mut c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut c_void, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut c_void, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct lola_sample_clock {
    pub type_: c_uint,
    pub format: c_int,
    pub freq: c_uint,
}

#[repr(C)]
pub struct lola_clock {
    pub nid: c_int,
    pub items: c_int,
    pub cur_index: c_int,
    pub cur_freq: c_int,
    pub cur_valid: bool,
    pub sample_clock: *mut lola_sample_clock,
    pub idx_lookup: *mut c_uint,
}

#[repr(C)]
pub struct lola {
    pub granularity: c_uint,
    pub clock: lola_clock,
    pub card: *mut snd_card,
    pub sample_rate_min: c_uint,
    pub audio_in_alloc_mask: c_uint,
    pub audio_out_alloc_mask: c_uint,
}

#[inline]
fn div_round_up(n: c_int, d: c_int) -> c_int {
    (n + d - 1) / d
}

#[no_mangle]
pub unsafe extern "C" fn lola_sample_rate_convert(coded: c_uint) -> c_uint {
    let mut freq: c_uint;

    /* base frequency */
    match coded & 0x3 {
        0 => freq = 48000,
        1 => freq = 44100,
        2 => freq = 32000,
        _ => return 0, /* error */
    }

    /* multiplier / devisor */
    match coded & 0x1c {
        x if x == (0 << 2) => {}
        x if x == (4 << 2) => {}
        x if x == (1 << 2) => freq = freq.wrapping_mul(2),
        x if x == (2 << 2) => freq = freq.wrapping_mul(4),
        x if x == (5 << 2) => freq /= 2,
        x if x == (6 << 2) => freq /= 4,
        _ => return 0, /* error */
    }

    /* adjustement */
    match coded & 0x60 {
        x if x == (0 << 5) => {}
        x if x == (1 << 5) => freq = freq.wrapping_mul(999) / 1000,
        x if x == (2 << 5) => freq = freq.wrapping_mul(1001) / 1000,
        _ => return 0, /* error */
    }
    freq
}

/*
 * Granualrity
 */

unsafe fn check_gran_clock_compatibility(
    chip: *mut lola,
    val: c_uint,
    freq: c_uint,
) -> bool {
    if (*chip).granularity == 0 {
        return true;
    }

    if val < LOLA_GRANULARITY_MIN
        || val > LOLA_GRANULARITY_MAX
        || (val % LOLA_GRANULARITY_STEP) != 0
    {
        return false;
    }

    if val == LOLA_GRANULARITY_MIN {
        if freq > LOLA_MAXFREQ_AT_GRANULARITY_MIN {
            return false;
        }
    } else if val < LOLA_GRANULARITY_MAX {
        if freq > LOLA_MAXFREQ_AT_GRANULARITY_BELOW_MAX {
            return false;
        }
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn lola_set_granularity(
    chip: *mut lola,
    mut val: c_uint,
    force: bool,
) -> c_int {
    let err: c_int;

    if !force {
        if val == (*chip).granularity {
            return 0;
        }
        /*
         * Disabled C block:
         * change Gran only if there are no streams allocated !
         * if (chip->audio_in_alloc_mask || chip->audio_out_alloc_mask)
         *     return -EBUSY;
         */
        if !check_gran_clock_compatibility(chip, val, (*chip).clock.cur_freq as c_uint) {
            return -EINVAL;
        }
    }

    (*chip).granularity = val;
    val /= LOLA_GRANULARITY_STEP;

    /* audio function group */
    err = lola_codec_write(chip, 1, LOLA_VERB_SET_GRANULARITY_STEPS, val, 0);
    if err < 0 {
        return err;
    }
    /* this can be a very slow function !!! */
    usleep_range(400u32.wrapping_mul(val), 20000);
    lola_codec_flush(chip)
}

/*
 * Clock widget handling
 */

#[no_mangle]
pub unsafe extern "C" fn lola_init_clock_widget(chip: *mut lola, nid: c_int) -> c_int {
    let mut val: c_uint = 0;
    let mut i: c_int;
    let mut j: c_int;
    let nitems: c_int;
    let nb_verbs: c_int;
    let mut idx: c_int;
    let mut idx_list: c_int;
    let mut err: c_int;

    err = lola_read_param(chip, nid, LOLA_PAR_AUDIO_WIDGET_CAP, &mut val);
    if err < 0 {
        dev_err(
            (*(*chip).card).dev,
            b"Can't read wcaps for 0x%x\n\0".as_ptr() as *const c_char,
            nid,
        );
        return err;
    }

    if (val & 0xfff00000) != 0x01f00000 {
        /* test SubType and Type */
        dev_dbg(
            (*(*chip).card).dev,
            b"No valid clock widget\n\0".as_ptr() as *const c_char,
        );
        return 0;
    }

    (*chip).clock.nid = nid;
    (*chip).clock.items = (val & 0xff) as c_int;
    dev_dbg(
        (*(*chip).card).dev,
        b"clock_list nid=%x, entries=%d\n\0".as_ptr() as *const c_char,
        nid,
        (*chip).clock.items,
    );
    if (*chip).clock.items as c_uint > MAX_SAMPLE_CLOCK_COUNT {
        dev_err(
            (*(*chip).card).dev,
            b"CLOCK_LIST too big: %d\n\0".as_ptr() as *const c_char,
            (*chip).clock.items,
        );
        return -EINVAL;
    }

    nitems = (*chip).clock.items;
    nb_verbs = div_round_up(nitems, 4);
    idx = 0;
    idx_list = 0;
    i = 0;
    while i < nb_verbs {
        let mut res_ex: c_uint = 0;
        let mut items: [u16; 4] = [0; 4];

        err = lola_codec_read(
            chip,
            nid,
            LOLA_VERB_GET_CLOCK_LIST,
            idx as c_uint,
            0,
            &mut val,
            &mut res_ex,
        );
        if err < 0 {
            dev_err(
                (*(*chip).card).dev,
                b"Can't read CLOCK_LIST\n\0".as_ptr() as *const c_char,
            );
            return -EINVAL;
        }

        items[0] = (val & 0xfff) as u16;
        items[1] = ((val >> 16) & 0xfff) as u16;
        items[2] = (res_ex & 0xfff) as u16;
        items[3] = ((res_ex >> 16) & 0xfff) as u16;

        j = 0;
        while j < 4 {
            let type_: u8 = (items[j as usize] >> 8) as u8;
            let mut freq: c_uint = (items[j as usize] & 0xff) as c_uint;
            let mut format: c_int = LOLA_CLOCK_FORMAT_NONE;
            let mut add_clock: bool = true;
            if type_ as c_uint == LOLA_CLOCK_TYPE_INTERNAL {
                freq = lola_sample_rate_convert(freq);
                if freq < (*chip).sample_rate_min {
                    add_clock = false;
                } else if freq == 48000 {
                    (*chip).clock.cur_index = idx_list;
                    (*chip).clock.cur_freq = 48000;
                    (*chip).clock.cur_valid = true;
                }
            } else if type_ as c_uint == LOLA_CLOCK_TYPE_VIDEO {
                freq = lola_sample_rate_convert(freq);
                if freq < (*chip).sample_rate_min {
                    add_clock = false;
                }
                /* video clock has a format (0:NTSC, 1:PAL)*/
                if (items[j as usize] & 0x80) != 0 {
                    format = LOLA_CLOCK_FORMAT_NTSC;
                } else {
                    format = LOLA_CLOCK_FORMAT_PAL;
                }
            }
            if add_clock {
                let sc: *mut lola_sample_clock;
                sc = (*chip).clock.sample_clock.offset(idx_list as isize);
                (*sc).type_ = type_ as c_uint;
                (*sc).format = format;
                (*sc).freq = freq;
                /* keep the index used with the board */
                *(*chip).clock.idx_lookup.offset(idx_list as isize) = idx as c_uint;
                idx_list += 1;
            } else {
                (*chip).clock.items -= 1;
            }
            idx += 1;
            if idx >= nitems {
                break;
            }
            j += 1;
        }
        i += 1;
    }
    0
}

/* enable unsolicited events of the clock widget */
#[no_mangle]
pub unsafe extern "C" fn lola_enable_clock_events(chip: *mut lola) -> c_int {
    let mut res: c_uint = 0;
    let mut err: c_int;

    err = lola_codec_read(
        chip,
        (*chip).clock.nid,
        LOLA_VERB_SET_UNSOLICITED_ENABLE,
        LOLA_UNSOLICITED_ENABLE | LOLA_UNSOLICITED_TAG,
        0,
        &mut res,
        ptr::null_mut(),
    );
    if err < 0 {
        return err;
    }
    if res != 0 {
        dev_warn(
            (*(*chip).card).dev,
            b"error in enable_clock_events %d\n\0".as_ptr() as *const c_char,
            res,
        );
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn lola_set_clock_index(chip: *mut lola, idx: c_uint) -> c_int {
    let mut res: c_uint = 0;
    let mut err: c_int;

    err = lola_codec_read(
        chip,
        (*chip).clock.nid,
        LOLA_VERB_SET_CLOCK_SELECT,
        *(*chip).clock.idx_lookup.offset(idx as isize),
        0,
        &mut res,
        ptr::null_mut(),
    );
    if err < 0 {
        return err;
    }
    if res != 0 {
        dev_warn(
            (*(*chip).card).dev,
            b"error in set_clock %d\n\0".as_ptr() as *const c_char,
            res,
        );
        return -EINVAL;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn lola_update_ext_clock_freq(chip: *mut lola, val: c_uint) -> bool {
    let tag: c_uint;

    /*
     * the current EXTERNAL clock information gets updated by interrupt
     * with an unsolicited response
     */
    if val == 0 {
        return false;
    }
    tag = (val >> LOLA_UNSOL_RESP_TAG_OFFSET) & LOLA_UNSOLICITED_TAG_MASK;
    if tag != LOLA_UNSOLICITED_TAG {
        return false;
    }

    /* only for current = external clocks */
    if (*(*chip)
        .clock
        .sample_clock
        .offset((*chip).clock.cur_index as isize))
    .type_
        != LOLA_CLOCK_TYPE_INTERNAL
    {
        (*chip).clock.cur_freq = lola_sample_rate_convert(val & 0x7f) as c_int;
        (*chip).clock.cur_valid = (val & 0x100) != 0;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn lola_set_clock(chip: *mut lola, idx: c_int) -> c_int {
    let mut freq: c_int = 0;
    let mut valid: bool = false;

    if idx == (*chip).clock.cur_index {
        /* current clock is allowed */
        freq = (*chip).clock.cur_freq;
        valid = (*chip).clock.cur_valid;
    } else if (*(*chip).clock.sample_clock.offset(idx as isize)).type_ == LOLA_CLOCK_TYPE_INTERNAL {
        /* internal clocks allowed */
        freq = (*(*chip).clock.sample_clock.offset(idx as isize)).freq as c_int;
        valid = true;
    }

    if freq == 0 || !valid {
        return -EINVAL;
    }

    if !check_gran_clock_compatibility(chip, (*chip).granularity, freq as c_uint) {
        return -EINVAL;
    }

    if idx != (*chip).clock.cur_index {
        let err: c_int = lola_set_clock_index(chip, idx as c_uint);
        if err < 0 {
            return err;
        }
        /* update new settings */
        (*chip).clock.cur_index = idx;
        (*chip).clock.cur_freq = freq;
        (*chip).clock.cur_valid = true;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn lola_set_sample_rate(chip: *mut lola, rate: c_int) -> c_int {
    let mut i: c_int;

    if (*chip).clock.cur_freq == rate && (*chip).clock.cur_valid {
        return 0;
    }
    /* search for new dwClockIndex */
    i = 0;
    while i < (*chip).clock.items {
        if (*(*chip).clock.sample_clock.offset(i as isize)).type_ == LOLA_CLOCK_TYPE_INTERNAL
            && (*(*chip).clock.sample_clock.offset(i as isize)).freq == rate as c_uint
        {
            break;
        }
        i += 1;
    }
    if i >= (*chip).clock.items {
        return -EINVAL;
    }
    lola_set_clock(chip, i)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
