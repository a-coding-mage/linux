// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Support for Digigram Lola PCI-e boards
 *
 *  Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;

const PLAY: usize = 0;
const CAPT: usize = 1;
const BAR1: usize = 1;
const LOLA_PAR_AUDIO_WIDGET_CAP: c_uint = 0;
const LOLA_PAR_AMP_OUT_CAP: c_uint = 0;
const LOLA_PAR_AMP_IN_CAP: c_uint = 0;
const LOLA_VERB_GET_MAX_LEVEL: c_uint = 0;
const LOLA_VERB_SET_SOURCE_GAIN: c_uint = 0;
const LOLA_VERB_SET_MIX_GAIN: c_uint = 0;
const LOLA_VERB_SET_AMP_GAIN_MUTE: c_uint = 0;
const LOLA_VERB_SET_SRC: c_uint = 0;
const LOLA_VERB_SET_DESTINATION_GAIN: c_uint = 0;
const LOLA_BAR1_SOURCE_GAIN_ENABLE: usize = 0;
const MAX_AUDIO_INOUT_COUNT: c_int = 0;
const MAX_STREAM_IN_COUNT: c_int = 0;
const LOLA_MIXER_DIM: usize = 0;
const LOLA_PEAK_METER_CAN_AGC_MASK: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_TLV_READ: c_uint = 0;
const SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK: c_uint = 0;
const SNDRV_CTL_TLVT_DB_SCALE: c_uint = 0;
const TLV_DB_SCALE_MUTE: c_uint = 0;

#[repr(C)]
pub struct lola {
    pub card: *mut snd_card,
    pub pin: [lola_pin_array; 2],
    pub mixer: lola_mixer,
    pub pcm: [lola_pcm; 2],
    pub bar: [lola_bar; 2],
    pub input_src_caps_mask: c_uint,
    pub input_src_mask: c_uint,
}

#[repr(C)]
pub struct lola_pin_array {
    pub num_pins: c_int,
    pub num_analog_pins: c_int,
    pub pins: *mut lola_pin,
}

#[repr(C)]
pub struct lola_pin {
    pub nid: c_int,
    pub is_analog: bool,
    pub amp_mute: c_uint,
    pub amp_step_size: c_uint,
    pub amp_num_steps: c_uint,
    pub amp_offset: c_uint,
    pub max_level: c_uint,
    pub config_default_reg: c_uint,
    pub fixed_gain_list_len: c_uint,
    pub cur_gain_step: c_uint,
}

#[repr(C)]
pub struct lola_mixer {
    pub nid: c_int,
    pub caps: c_uint,
    pub array: *mut lola_mixer_array,
    pub array_saved: *mut lola_mixer_array,
    pub src_stream_outs: c_int,
    pub src_phys_ins: c_int,
    pub dest_stream_ins: c_int,
    pub dest_phys_outs: c_int,
    pub src_stream_out_ofs: c_int,
    pub dest_phys_out_ofs: c_int,
    pub src_mask: c_uint,
    pub dest_mask: c_uint,
}

#[repr(C)]
pub struct lola_mixer_array {
    pub src_gain_enable: c_uint,
    pub src_gain: [u16; LOLA_MIXER_DIM],
    pub dest_mix_gain_enable: [c_uint; LOLA_MIXER_DIM],
    pub dest_mix_gain: [[u16; LOLA_MIXER_DIM]; LOLA_MIXER_DIM],
}

#[repr(C)]
pub struct lola_pcm {
    pub num_streams: c_int,
    pub streams: *mut lola_stream,
}

#[repr(C)]
pub struct lola_stream {
    pub nid: c_int,
}

#[repr(C)]
pub struct lola_bar {
    pub remap_addr: usize,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut c_void,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}

#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_integer {
    pub min: i64,
    pub max: i64,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub id: snd_ctl_elem_id,
    pub value: snd_ctl_elem_value_union,
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    _private: [u8; 0],
}

#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_ulong; 128],
}

#[repr(C)]
pub union snd_kcontrol_tlv {
    pub c: Option<
        unsafe extern "C" fn(
            *mut snd_kcontrol,
            c_int,
            c_uint,
            *mut c_uint,
        ) -> c_int,
    >,
    pub p: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub access: c_uint,
    pub name: *mut c_char,
    pub count: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
    pub tlv: snd_kcontrol_tlv,
}

unsafe extern "C" {
    fn lola_read_param(chip: *mut lola, nid: c_int, param: c_uint, val: *mut c_uint) -> c_int;
    fn lola_codec_read(
        chip: *mut lola,
        nid: c_int,
        verb: c_uint,
        data: c_uint,
        extdata: c_uint,
        val: *mut c_uint,
        extval: *mut c_void,
    ) -> c_int;
    fn lola_codec_write(chip: *mut lola, nid: c_int, verb: c_uint, data: c_uint, extdata: c_uint)
        -> c_int;
    fn lola_codec_flush(chip: *mut lola) -> c_int;
    fn vfree(addr: *mut c_void);
    fn vmalloc(size: usize) -> *mut c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut lola;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(template: *const snd_kcontrol_new, chip: *mut c_void) -> *mut snd_kcontrol;
    fn snd_ctl_get_ioffidx(kcontrol: *mut snd_kcontrol, id: *const snd_ctl_elem_id) -> c_uint;
    fn put_user(value: c_uint, ptr: *mut c_uint) -> c_int;
}

macro_rules! dev_err {
    ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        let _ = ($dev, $fmt $(, $arg)*);
    }};
}

macro_rules! dev_dbg {
    ($dev:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        let _ = ($dev, $fmt $(, $arg)*);
    }};
}

const fn LOLA_AMP_MUTE_CAPABLE(val: c_uint) -> c_uint {
    val
}

const fn LOLA_AMP_STEP_SIZE(val: c_uint) -> c_uint {
    val
}

const fn LOLA_AMP_NUM_STEPS(val: c_uint) -> c_uint {
    val
}

const fn LOLA_AMP_OFFSET(val: c_uint) -> c_uint {
    val
}

const fn LOLA_MIXER_SRC_INPUT_PLAY_SEPARATION(val: c_uint) -> c_int {
    val as c_int
}

const fn LOLA_MIXER_DEST_REC_OUTPUT_SEPARATION(val: c_uint) -> c_int {
    val as c_int
}

unsafe fn readl(addr: *const c_uint) -> c_uint {
    unsafe { core::ptr::read_volatile(addr) }
}

unsafe fn readw(addr: *const u16) -> u16 {
    unsafe { core::ptr::read_volatile(addr) }
}

unsafe fn writel(val: c_uint, addr: *mut c_uint) {
    unsafe { core::ptr::write_volatile(addr, val) };
}

unsafe fn writew(val: u16, addr: *mut u16) {
    unsafe { core::ptr::write_volatile(addr, val) };
}

unsafe fn memset_io(dst: *mut lola_mixer_array, val: c_int, size: usize) {
    unsafe { core::ptr::write_bytes(dst as *mut u8, val as u8, size) };
}

unsafe fn lola_init_pin(chip: *mut lola, pin: *mut lola_pin, dir: c_int, nid: c_int) -> c_int {
    let mut val: c_uint = 0;
    let mut err: c_int;

    unsafe {
        (*pin).nid = nid;
        err = lola_read_param(chip, nid, LOLA_PAR_AUDIO_WIDGET_CAP, &mut val);
        if err < 0 {
            dev_err!((*(*chip).card).dev, "Can't read wcaps for 0x%x\n", nid);
            return err;
        }
        val &= 0x00f00fff; /* test TYPE and bits 0..11 */
        if val == 0x00400200 {
            /* Type = 4, Digital = 1 */
            (*pin).is_analog = false;
        } else if val == 0x0040000a && dir == CAPT as c_int {
            /* Dig=0, InAmp/ovrd */
            (*pin).is_analog = true;
        } else if val == 0x0040000c && dir == PLAY as c_int {
            /* Dig=0, OutAmp/ovrd */
            (*pin).is_analog = true;
        } else {
            dev_err!((*(*chip).card).dev, "Invalid wcaps 0x%x for 0x%x\n", val, nid);
            return -EINVAL;
        }

        /* analog parameters only following, so continue in case of Digital pin
         */
        if !(*pin).is_analog {
            return 0;
        }

        if dir == PLAY as c_int {
            err = lola_read_param(chip, nid, LOLA_PAR_AMP_OUT_CAP, &mut val);
        } else {
            err = lola_read_param(chip, nid, LOLA_PAR_AMP_IN_CAP, &mut val);
        }
        if err < 0 {
            dev_err!((*(*chip).card).dev, "Can't read AMP-caps for 0x%x\n", nid);
            return err;
        }

        (*pin).amp_mute = LOLA_AMP_MUTE_CAPABLE(val);
        (*pin).amp_step_size = LOLA_AMP_STEP_SIZE(val);
        (*pin).amp_num_steps = LOLA_AMP_NUM_STEPS(val);
        if (*pin).amp_num_steps != 0 {
            /* zero as mute state */
            (*pin).amp_num_steps = (*pin).amp_num_steps.wrapping_add(1);
            (*pin).amp_step_size = (*pin).amp_step_size.wrapping_add(1);
        }
        (*pin).amp_offset = LOLA_AMP_OFFSET(val);

        err = lola_codec_read(
            chip,
            nid,
            LOLA_VERB_GET_MAX_LEVEL,
            0,
            0,
            &mut val,
            core::ptr::null_mut(),
        );
        if err < 0 {
            dev_err!((*(*chip).card).dev, "Can't get MAX_LEVEL 0x%x\n", nid);
            return err;
        }
        (*pin).max_level = val & 0x3ff; /* 10 bits */

        (*pin).config_default_reg = 0;
        (*pin).fixed_gain_list_len = 0;
        (*pin).cur_gain_step = 0;
    }

    0
}

pub unsafe extern "C" fn lola_init_pins(chip: *mut lola, dir: c_int, nidp: *mut c_int) -> c_int {
    let mut i: c_int;
    let mut err: c_int;
    let mut nid: c_int;
    unsafe {
        nid = *nidp;
        i = 0;
        while i < (*chip).pin[dir as usize].num_pins {
            err = lola_init_pin(
                chip,
                (*chip).pin[dir as usize].pins.add(i as usize),
                dir,
                nid,
            );
            if err < 0 {
                return err;
            }
            if (*(*chip).pin[dir as usize].pins.add(i as usize)).is_analog {
                (*chip).pin[dir as usize].num_analog_pins += 1;
            }
            i += 1;
            nid += 1;
        }
        *nidp = nid;
    }
    0
}

pub unsafe extern "C" fn lola_free_mixer(chip: *mut lola) {
    unsafe { vfree((*chip).mixer.array_saved as *mut c_void) };
}

pub unsafe extern "C" fn lola_init_mixer_widget(chip: *mut lola, nid: c_int) -> c_int {
    let mut val: c_uint = 0;
    let err: c_int;

    unsafe {
        err = lola_read_param(chip, nid, LOLA_PAR_AUDIO_WIDGET_CAP, &mut val);
        if err < 0 {
            dev_err!((*(*chip).card).dev, "Can't read wcaps for 0x%x\n", nid);
            return err;
        }

        if (val & 0xfff00000) != 0x02f00000 {
            /* test SubType and Type */
            dev_dbg!((*(*chip).card).dev, "No valid mixer widget\n");
            return 0;
        }

        (*chip).mixer.nid = nid;
        (*chip).mixer.caps = val;
        (*chip).mixer.array =
            ((*chip).bar[BAR1].remap_addr + LOLA_BAR1_SOURCE_GAIN_ENABLE) as *mut lola_mixer_array;

        /* reserve memory to copy mixer data for sleep mode transitions */
        (*chip).mixer.array_saved =
            vmalloc(core::mem::size_of::<lola_mixer_array>()) as *mut lola_mixer_array;
        if (*chip).mixer.array_saved.is_null() {
            return -ENOMEM;
        }

        /* mixer matrix sources are physical input data and play streams */
        (*chip).mixer.src_stream_outs = (*chip).pcm[PLAY].num_streams;
        (*chip).mixer.src_phys_ins = (*chip).pin[CAPT].num_pins;

        /* mixer matrix destinations are record streams and physical output */
        (*chip).mixer.dest_stream_ins = (*chip).pcm[CAPT].num_streams;
        (*chip).mixer.dest_phys_outs = (*chip).pin[PLAY].num_pins;

        /* mixer matrix may have unused areas between PhysIn and
         * Play or Record and PhysOut zones
         */
        (*chip).mixer.src_stream_out_ofs =
            (*chip).mixer.src_phys_ins + LOLA_MIXER_SRC_INPUT_PLAY_SEPARATION(val);
        (*chip).mixer.dest_phys_out_ofs =
            (*chip).mixer.dest_stream_ins + LOLA_MIXER_DEST_REC_OUTPUT_SEPARATION(val);

        /* example : MixerMatrix of LoLa881 (LoLa16161 uses unused zones)
         * +-+  0-------8------16-------8------16
         * | |  |       |       |       |       |
         * |s|  | INPUT |       | INPUT |       |
         * | |->|  ->   |unused |  ->   |unused |
         * |r|  |CAPTURE|       | OUTPUT|       |
         * | |  |  MIX  |       |  MIX  |       |
         * |c|  8--------------------------------
         * | |  |       |       |       |       |
         * | |  |       |       |       |       |
         * |g|  |unused |unused |unused |unused |
         * | |  |       |       |       |       |
         * |a|  |       |       |       |       |
         * | |  16-------------------------------
         * |i|  |       |       |       |       |
         * | |  | PLAYBK|       | PLAYBK|       |
         * |n|->|  ->   |unused |  ->   |unused |
         * | |  |CAPTURE|       | OUTPUT|       |
         * | |  |  MIX  |       |  MIX  |       |
         * |a|  8--------------------------------
         * |r|  |       |       |       |       |
         * |r|  |       |       |       |       |
         * |a|  |unused |unused |unused |unused |
         * |y|  |       |       |       |       |
         * | |  |       |       |       |       |
         * +++  16--|---------------|------------
         *      +---V---------------V-----------+
         *      |  dest_mix_gain_enable array   |
         *      +-------------------------------+
         */
        /* example : MixerMatrix of LoLa280
         * +-+  0-------8-2
         * | |  |       | |
         * |s|  | INPUT | |     INPUT
         * |r|->|  ->   | |      ->
         * |c|  |CAPTURE| | <-  OUTPUT
         * | |  |  MIX  | |      MIX
         * |g|  8----------
         * |a|  |       | |
         * |i|  | PLAYBK| |     PLAYBACK
         * |n|->|  ->   | |      ->
         * | |  |CAPTURE| | <-  OUTPUT
         * |a|  |  MIX  | |      MIX
         * |r|  8---|----|-
         * |r|  +---V----V-------------------+
         * |a|  | dest_mix_gain_enable array |
         * |y|  +----------------------------+
         */
        if (*chip).mixer.src_stream_out_ofs > MAX_AUDIO_INOUT_COUNT
            || (*chip).mixer.dest_phys_out_ofs > MAX_STREAM_IN_COUNT
        {
            dev_err!((*(*chip).card).dev, "Invalid mixer widget size\n");
            return -EINVAL;
        }

        (*chip).mixer.src_mask = ((1_u32 << (*chip).mixer.src_phys_ins) - 1)
            | (((1_u32 << (*chip).mixer.src_stream_outs) - 1)
                << (*chip).mixer.src_stream_out_ofs);
        (*chip).mixer.dest_mask = ((1_u32 << (*chip).mixer.dest_stream_ins) - 1)
            | (((1_u32 << (*chip).mixer.dest_phys_outs) - 1)
                << (*chip).mixer.dest_phys_out_ofs);

        dev_dbg!(
            (*(*chip).card).dev,
            "Mixer src_mask=%x, dest_mask=%x\n",
            (*chip).mixer.src_mask,
            (*chip).mixer.dest_mask
        );
    }

    0
}

unsafe fn lola_mixer_set_src_gain(
    chip: *mut lola,
    id: c_uint,
    gain: u16,
    on: bool,
) -> c_int {
    let oldval: c_uint;
    let mut val: c_uint;

    unsafe {
        if ((*chip).mixer.src_mask & (1 << id)) == 0 {
            return -EINVAL;
        }
        val = readl(&(*(*chip).mixer.array).src_gain_enable);
        oldval = val;
        if on {
            val |= 1 << id;
        } else {
            val &= !(1 << id);
        }
        /* test if values unchanged */
        if val == oldval && gain == readw(&(*(*chip).mixer.array).src_gain[id as usize]) {
            return 0;
        }

        dev_dbg!(
            (*(*chip).card).dev,
            "lola_mixer_set_src_gain (id=%d, gain=%d) enable=%x\n",
            id,
            gain,
            val
        );
        writew(gain, &mut (*(*chip).mixer.array).src_gain[id as usize]);
        writel(val, &mut (*(*chip).mixer.array).src_gain_enable);
        lola_codec_flush(chip);
        /* inform micro-controller about the new source gain */
        lola_codec_write(chip, (*chip).mixer.nid, LOLA_VERB_SET_SOURCE_GAIN, id, 0)
    }
}

#[cfg(any())] /* not used */
unsafe fn lola_mixer_set_src_gains(chip: *mut lola, mask: c_uint, mut gains: *mut u16) -> c_int {
    let mut i: c_int;

    unsafe {
        if ((*chip).mixer.src_mask & mask) != mask {
            return -EINVAL;
        }
        i = 0;
        while i < LOLA_MIXER_DIM as c_int {
            if (mask & (1 << i)) != 0 {
                writew(*gains, &mut (*(*chip).mixer.array).src_gain[i as usize]);
                gains = gains.add(1);
            }
            i += 1;
        }
        writel(mask, &mut (*(*chip).mixer.array).src_gain_enable);
        lola_codec_flush(chip);
        if ((*chip).mixer.caps & LOLA_PEAK_METER_CAN_AGC_MASK) != 0 {
            /* update for all srcs at once */
            return lola_codec_write(chip, (*chip).mixer.nid, LOLA_VERB_SET_SOURCE_GAIN, 0x80, 0);
        }
        /* update manually */
        i = 0;
        while i < LOLA_MIXER_DIM as c_int {
            if (mask & (1 << i)) != 0 {
                lola_codec_write(
                    chip,
                    (*chip).mixer.nid,
                    LOLA_VERB_SET_SOURCE_GAIN,
                    i as c_uint,
                    0,
                );
            }
            i += 1;
        }
    }
    0
}

unsafe fn lola_mixer_set_mapping_gain(
    chip: *mut lola,
    src: c_uint,
    dest: c_uint,
    gain: u16,
    on: bool,
) -> c_int {
    let mut val: c_uint;

    unsafe {
        if ((*chip).mixer.src_mask & (1 << src)) == 0
            || ((*chip).mixer.dest_mask & (1 << dest)) == 0
        {
            return -EINVAL;
        }
        if on {
            writew(
                gain,
                &mut (*(*chip).mixer.array).dest_mix_gain[dest as usize][src as usize],
            );
        }
        val = readl(&(*(*chip).mixer.array).dest_mix_gain_enable[dest as usize]);
        if on {
            val |= 1 << src;
        } else {
            val &= !(1 << src);
        }
        writel(
            val,
            &mut (*(*chip).mixer.array).dest_mix_gain_enable[dest as usize],
        );
        lola_codec_flush(chip);
        lola_codec_write(
            chip,
            (*chip).mixer.nid,
            LOLA_VERB_SET_MIX_GAIN,
            src,
            dest,
        )
    }
}

#[cfg(any())] /* not used */
unsafe fn lola_mixer_set_dest_gains(
    chip: *mut lola,
    id: c_uint,
    mask: c_uint,
    mut gains: *mut u16,
) -> c_int {
    let mut i: c_int;

    unsafe {
        if ((*chip).mixer.dest_mask & (1 << id)) == 0 || ((*chip).mixer.src_mask & mask) != mask {
            return -EINVAL;
        }
        i = 0;
        while i < LOLA_MIXER_DIM as c_int {
            if (mask & (1 << i)) != 0 {
                writew(
                    *gains,
                    &mut (*(*chip).mixer.array).dest_mix_gain[id as usize][i as usize],
                );
                gains = gains.add(1);
            }
            i += 1;
        }
        writel(
            mask,
            &mut (*(*chip).mixer.array).dest_mix_gain_enable[id as usize],
        );
        lola_codec_flush(chip);
        /* update for all dests at once */
        lola_codec_write(
            chip,
            (*chip).mixer.nid,
            LOLA_VERB_SET_DESTINATION_GAIN,
            id,
            0,
        )
    }
}

/*
 */

unsafe fn set_analog_volume(
    chip: *mut lola,
    dir: c_int,
    idx: c_uint,
    val: c_uint,
    external_call: bool,
) -> c_int {
    let pin: *mut lola_pin;
    let err: c_int;

    unsafe {
        if idx >= (*chip).pin[dir as usize].num_pins as c_uint {
            return -EINVAL;
        }
        pin = (*chip).pin[dir as usize].pins.add(idx as usize);
        if !(*pin).is_analog || (*pin).amp_num_steps <= val {
            return -EINVAL;
        }
        if external_call && (*pin).cur_gain_step == val {
            return 0;
        }
        if external_call {
            lola_codec_flush(chip);
        }
        dev_dbg!(
            (*(*chip).card).dev,
            "set_analog_volume (dir=%d idx=%d, volume=%d)\n",
            dir,
            idx,
            val
        );
        err = lola_codec_write(chip, (*pin).nid, LOLA_VERB_SET_AMP_GAIN_MUTE, val, 0);
        if err < 0 {
            return err;
        }
        if external_call {
            (*pin).cur_gain_step = val;
        }
    }
    0
}

pub unsafe extern "C" fn lola_setup_all_analog_gains(
    chip: *mut lola,
    dir: c_int,
    mute: bool,
) -> c_int {
    let pin: *mut lola_pin;
    let mut idx: c_int;
    let max_idx: c_int;

    unsafe {
        pin = (*chip).pin[dir as usize].pins;
        max_idx = (*chip).pin[dir as usize].num_pins;
        idx = 0;
        while idx < max_idx {
            if (*pin.add(idx as usize)).is_analog {
                let val: c_uint = if mute {
                    0
                } else {
                    (*pin.add(idx as usize)).cur_gain_step
                };
                /* set volume and do not save the value */
                set_analog_volume(chip, dir, idx as c_uint, val, false);
            }
            idx += 1;
        }
        lola_codec_flush(chip)
    }
}

pub unsafe extern "C" fn lola_set_src_config(
    chip: *mut lola,
    src_mask: c_uint,
    update: bool,
) -> c_int {
    let mut ret: c_int = 0;
    let mut success: c_int = 0;
    let mut n: c_int;
    let err: c_int;

    unsafe {
        /* SRC can be activated and the dwInputSRCMask is valid? */
        if ((*chip).input_src_caps_mask & src_mask) != src_mask {
            return -EINVAL;
        }
        /* handle all even Inputs - SRC is a stereo setting !!! */
        n = 0;
        while n < (*chip).pin[CAPT].num_pins {
            let mask: c_uint = 3_u32 << n; /* handle the stereo case */
            let new_src: c_uint;
            let src_state: c_uint;
            if ((*chip).input_src_caps_mask & mask) == 0 {
                n += 2;
                continue;
            }
            /* if one IO needs SRC, both stereo IO will get SRC */
            new_src = if (src_mask & mask) != 0 { 1 } else { 0 };
            if update {
                src_state = if ((*chip).input_src_mask & mask) != 0 { 1 } else { 0 };
                if src_state == new_src {
                    n += 2;
                    continue; /* nothing to change for this IO */
                }
            }
            err = lola_codec_write(
                chip,
                (*(*chip).pcm[CAPT].streams.add(n as usize)).nid,
                LOLA_VERB_SET_SRC,
                new_src,
                0,
            );
            if err == 0 {
                success += 1;
            } else {
                ret = err;
            }
            n += 2;
        }
        if success != 0 {
            ret = lola_codec_flush(chip);
        }
        if ret == 0 {
            (*chip).input_src_mask = src_mask;
        }
    }
    ret
}

/*
 */
unsafe fn init_mixer_values(chip: *mut lola) -> c_int {
    let mut i: c_int;

    unsafe {
        /* all sample rate converters on */
        lola_set_src_config(chip, (1 << (*chip).pin[CAPT].num_pins) - 1, false);

        /* clear all mixer matrix settings */
        memset_io(
            (*chip).mixer.array,
            0,
            core::mem::size_of_val(&*(*chip).mixer.array),
        );
        /* inform firmware about all updated matrix columns - capture part */
        i = 0;
        while i < (*chip).mixer.dest_stream_ins {
            lola_codec_write(
                chip,
                (*chip).mixer.nid,
                LOLA_VERB_SET_DESTINATION_GAIN,
                i as c_uint,
                0,
            );
            i += 1;
        }
        /* inform firmware about all updated matrix columns - output part */
        i = 0;
        while i < (*chip).mixer.dest_phys_outs {
            lola_codec_write(
                chip,
                (*chip).mixer.nid,
                LOLA_VERB_SET_DESTINATION_GAIN,
                ((*chip).mixer.dest_phys_out_ofs + i) as c_uint,
                0,
            );
            i += 1;
        }

        /* set all digital input source (master) gains to 0dB */
        i = 0;
        while i < (*chip).mixer.src_phys_ins {
            lola_mixer_set_src_gain(chip, i as c_uint, 336, true); /* 0dB */
            i += 1;
        }

        /* set all digital playback source (master) gains to 0dB */
        i = 0;
        while i < (*chip).mixer.src_stream_outs {
            lola_mixer_set_src_gain(
                chip,
                (i + (*chip).mixer.src_stream_out_ofs) as c_uint,
                336,
                true,
            ); /* 0dB */
            i += 1;
        }
        /* set gain value 0dB diagonally in matrix - part INPUT -> CAPTURE */
        i = 0;
        while i < (*chip).mixer.dest_stream_ins {
            let src: c_int = i % (*chip).mixer.src_phys_ins;
            lola_mixer_set_mapping_gain(chip, src as c_uint, i as c_uint, 336, true);
            i += 1;
        }
        /* set gain value 0dB diagonally in matrix , part PLAYBACK -> OUTPUT
         * (LoLa280 : playback channel 0,2,4,6 linked to output channel 0)
         * (LoLa280 : playback channel 1,3,5,7 linked to output channel 1)
         */
        i = 0;
        while i < (*chip).mixer.src_stream_outs {
            let src: c_int = (*chip).mixer.src_stream_out_ofs + i;
            let dst: c_int =
                (*chip).mixer.dest_phys_out_ofs + i % (*chip).mixer.dest_phys_outs;
            lola_mixer_set_mapping_gain(chip, src as c_uint, dst as c_uint, 336, true);
            i += 1;
        }
    }
    0
}

/*
 * analog mixer control element
 */
unsafe extern "C" fn lola_analog_vol_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        let chip: *mut lola = snd_kcontrol_chip(kcontrol);
        let dir: c_int = (*kcontrol).private_value as c_int;

        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = (*chip).pin[dir as usize].num_pins as c_uint;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = (*(*chip).pin[dir as usize].pins).amp_num_steps as i64;
    }
    0
}

unsafe extern "C" fn lola_analog_vol_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut lola = snd_kcontrol_chip(kcontrol);
        let dir: c_int = (*kcontrol).private_value as c_int;
        let mut i: c_int = 0;

        while i < (*chip).pin[dir as usize].num_pins {
            (*ucontrol).value.integer.value[i as usize] =
                (*(*chip).pin[dir as usize].pins.add(i as usize)).cur_gain_step as c_ulong;
            i += 1;
        }
    }
    0
}

unsafe extern "C" fn lola_analog_vol_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut lola = snd_kcontrol_chip(kcontrol);
        let dir: c_int = (*kcontrol).private_value as c_int;
        let mut i: c_int = 0;

        while i < (*chip).pin[dir as usize].num_pins {
            let err = set_analog_volume(
                chip,
                dir,
                i as c_uint,
                (*ucontrol).value.integer.value[i as usize] as c_uint,
                true,
            );
            if err < 0 {
                return err;
            }
            i += 1;
        }
    }
    0
}

unsafe extern "C" fn lola_analog_vol_tlv(
    kcontrol: *mut snd_kcontrol,
    _op_flag: c_int,
    size: c_uint,
    tlv: *mut c_uint,
) -> c_int {
    unsafe {
        let chip: *mut lola = snd_kcontrol_chip(kcontrol);
        let dir: c_int = (*kcontrol).private_value as c_int;
        let mut val2: c_uint;
        let val1: c_uint;
        let pin: *mut lola_pin;

        if (size as usize) < 4 * core::mem::size_of::<c_uint>() {
            return -ENOMEM;
        }
        pin = (*chip).pin[dir as usize].pins;

        val2 = (*pin).amp_step_size * 25;
        val1 = ((-1_i32) * (*pin).amp_offset as i32 * val2 as i32) as c_uint;
        /* #ifdef TLV_DB_SCALE_MUTE */
        val2 |= TLV_DB_SCALE_MUTE;
        /* #endif */
        if put_user(SNDRV_CTL_TLVT_DB_SCALE, tlv) != 0 {
            return -EFAULT;
        }
        if put_user((2 * core::mem::size_of::<c_uint>()) as c_uint, tlv.add(1)) != 0 {
            return -EFAULT;
        }
        if put_user(val1, tlv.add(2)) != 0 {
            return -EFAULT;
        }
        if put_user(val2, tlv.add(3)) != 0 {
            return -EFAULT;
        }
    }
    0
}

static mut lola_analog_mixer: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE
        | SNDRV_CTL_ELEM_ACCESS_TLV_READ
        | SNDRV_CTL_ELEM_ACCESS_TLV_CALLBACK,
    name: core::ptr::null_mut(),
    count: 0,
    info: Some(lola_analog_vol_info),
    get: Some(lola_analog_vol_get),
    put: Some(lola_analog_vol_put),
    private_value: 0,
    tlv: snd_kcontrol_tlv {
        c: Some(lola_analog_vol_tlv),
    },
};

unsafe fn create_analog_mixer(chip: *mut lola, dir: c_int, name: *mut c_char) -> c_int {
    unsafe {
        if (*chip).pin[dir as usize].num_pins == 0 {
            return 0;
        }
        /* no analog volumes on digital only adapters */
        if (*chip).pin[dir as usize].num_pins != (*chip).pin[dir as usize].num_analog_pins {
            return 0;
        }
        lola_analog_mixer.name = name;
        lola_analog_mixer.private_value = dir as c_ulong;
        snd_ctl_add(
            (*chip).card,
            snd_ctl_new1(&raw const lola_analog_mixer, chip as *mut c_void),
        )
    }
}

/*
 * Hardware sample rate converter on digital input
 */
unsafe extern "C" fn lola_input_src_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        let chip: *mut lola = snd_kcontrol_chip(kcontrol);

        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
        (*uinfo).count = (*chip).pin[CAPT].num_pins as c_uint;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 1;
    }
    0
}

unsafe extern "C" fn lola_input_src_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut lola = snd_kcontrol_chip(kcontrol);
        let mut i: c_int = 0;

        while i < (*chip).pin[CAPT].num_pins {
            (*ucontrol).value.integer.value[i as usize] =
                (((*chip).input_src_mask & (1 << i)) != 0) as c_ulong;
            i += 1;
        }
    }
    0
}

unsafe extern "C" fn lola_input_src_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut lola = snd_kcontrol_chip(kcontrol);
        let mut i: c_int;
        let mut mask: c_uint;

        mask = 0;
        i = 0;
        while i < (*chip).pin[CAPT].num_pins {
            if (*ucontrol).value.integer.value[i as usize] != 0 {
                mask |= 1 << i;
            }
            i += 1;
        }
        lola_set_src_config(chip, mask, true)
    }
}

static lola_input_src_name: &[u8] = b"Digital SRC Capture Switch\0";

static lola_input_src_mixer: snd_kcontrol_new = snd_kcontrol_new {
    name: lola_input_src_name.as_ptr() as *mut c_char,
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: 0,
    count: 0,
    info: Some(lola_input_src_info),
    get: Some(lola_input_src_get),
    put: Some(lola_input_src_put),
    private_value: 0,
    tlv: snd_kcontrol_tlv {
        p: core::ptr::null(),
    },
};

/*
 * Lola16161 or Lola881 can have Hardware sample rate converters
 * on its digital input pins
 */
unsafe fn create_input_src_mixer(chip: *mut lola) -> c_int {
    unsafe {
        if (*chip).input_src_caps_mask == 0 {
            return 0;
        }

        snd_ctl_add(
            (*chip).card,
            snd_ctl_new1(&lola_input_src_mixer, chip as *mut c_void),
        )
    }
}

/*
 * src gain mixer
 */
unsafe extern "C" fn lola_src_gain_info(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    unsafe {
        let count: c_uint = ((*kcontrol).private_value >> 8) as c_uint & 0xff;

        (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
        (*uinfo).count = count;
        (*uinfo).value.integer.min = 0;
        (*uinfo).value.integer.max = 409;
    }
    0
}

unsafe extern "C" fn lola_src_gain_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut lola = snd_kcontrol_chip(kcontrol);
        let ofs: c_uint = (*kcontrol).private_value as c_uint & 0xff;
        let count: c_uint = ((*kcontrol).private_value >> 8) as c_uint & 0xff;
        let mask: c_uint;
        let mut i: c_uint;

        mask = readl(&(*(*chip).mixer.array).src_gain_enable);
        i = 0;
        while i < count {
            let idx: c_uint = ofs + i;
            let val: u16;
            if ((*chip).mixer.src_mask & (1 << idx)) == 0 {
                return -EINVAL;
            }
            if (mask & (1 << idx)) != 0 {
                val = readw(&(*(*chip).mixer.array).src_gain[idx as usize]).wrapping_add(1);
            } else {
                val = 0;
            }
            (*ucontrol).value.integer.value[i as usize] = val as c_ulong;
            i += 1;
        }
    }
    0
}

unsafe extern "C" fn lola_src_gain_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        let chip: *mut lola = snd_kcontrol_chip(kcontrol);
        let ofs: c_uint = (*kcontrol).private_value as c_uint & 0xff;
        let count: c_uint = ((*kcontrol).private_value >> 8) as c_uint & 0xff;
        let mut i: c_int = 0;

        while i < count as c_int {
            let idx: c_uint = ofs + i as c_uint;
            let mut val: u16 = (*ucontrol).value.integer.value[i as usize] as u16;
            if val != 0 {
                val = val.wrapping_sub(1);
            }
            let err = lola_mixer_set_src_gain(chip, idx, val, val != 0);
            if err < 0 {
                return err;
            }
            i += 1;
        }
    }
    0
}

/* raw value: 0 = -84dB, 336 = 0dB, 408=18dB, incremented 1 for mute */
static lola_src_gain_tlv: [c_uint; 4] = [SNDRV_CTL_TLVT_DB_SCALE, 2 * 4, (-8425_i32) as c_uint, 25 | 1];

static mut lola_src_gain_mixer: snd_kcontrol_new = snd_kcontrol_new {
    iface: SNDRV_CTL_ELEM_IFACE_MIXER,
    access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
    name: core::ptr::null_mut(),
    count: 0,
    info: Some(lola_src_gain_info),
    get: Some(lola_src_gain_get),
    put: Some(lola_src_gain_put),
    private_value: 0,
    tlv: snd_kcontrol_tlv {
        p: lola_src_gain_tlv.as_ptr(),
    },
};

unsafe fn create_src_gain_mixer(
    chip: *mut lola,
    num: c_int,
    ofs: c_int,
    name: *mut c_char,
) -> c_int {
    unsafe {
        lola_src_gain_mixer.name = name;
        lola_src_gain_mixer.private_value = (ofs + (num << 8)) as c_ulong;
        snd_ctl_add(
            (*chip).card,
            snd_ctl_new1(&raw const lola_src_gain_mixer, chip as *mut c_void),
        )
    }
}

#[cfg(any())] /* not used */
/*
 * destination gain (matrix-like) mixer
 */
mod unused_dest_gain_mixer {
    use super::*;

    unsafe extern "C" fn lola_dest_gain_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int {
        unsafe {
            let src_num: c_uint = ((*kcontrol).private_value >> 8) as c_uint & 0xff;

            (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
            (*uinfo).count = src_num;
            (*uinfo).value.integer.min = 0;
            (*uinfo).value.integer.max = 433;
        }
        0
    }

    unsafe extern "C" fn lola_dest_gain_get(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int {
        unsafe {
            let chip: *mut lola = snd_kcontrol_chip(kcontrol);
            let src_ofs: c_uint = (*kcontrol).private_value as c_uint & 0xff;
            let src_num: c_uint = ((*kcontrol).private_value >> 8) as c_uint & 0xff;
            let dst_ofs: c_uint = ((*kcontrol).private_value >> 16) as c_uint & 0xff;
            let dst: c_uint;
            let mask: c_uint;
            let mut i: c_uint;

            dst = snd_ctl_get_ioffidx(kcontrol, &(*ucontrol).id) + dst_ofs;
            mask = readl(&(*(*chip).mixer.array).dest_mix_gain_enable[dst as usize]);
            i = 0;
            while i < src_num {
                let src: c_uint = src_ofs + i;
                let val: u16;
                if ((*chip).mixer.src_mask & (1 << src)) == 0 {
                    return -EINVAL;
                }
                if (mask & (1 << dst)) != 0 {
                    val = readw(&(*(*chip).mixer.array).dest_mix_gain[dst as usize][src as usize])
                        .wrapping_add(1);
                } else {
                    val = 0;
                }
                (*ucontrol).value.integer.value[i as usize] = val as c_ulong;
                i += 1;
            }
        }
        0
    }

    unsafe extern "C" fn lola_dest_gain_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int {
        unsafe {
            let chip: *mut lola = snd_kcontrol_chip(kcontrol);
            let src_ofs: c_uint = (*kcontrol).private_value as c_uint & 0xff;
            let src_num: c_uint = ((*kcontrol).private_value >> 8) as c_uint & 0xff;
            let dst_ofs: c_uint = ((*kcontrol).private_value >> 16) as c_uint & 0xff;
            let dst: c_uint;
            let mut mask: c_uint;
            let mut gains: [u16; 0] = [];
            let mut i: c_int;
            let mut num: c_int;

            mask = 0;
            num = 0;
            i = 0;
            while i < src_num as c_int {
                let val: u16 = (*ucontrol).value.integer.value[i as usize] as u16;
                if val != 0 {
                    gains[num as usize] = val - 1;
                    num += 1;
                    mask |= 1 << i;
                }
                i += 1;
            }
            mask <<= src_ofs;
            dst = snd_ctl_get_ioffidx(kcontrol, &(*ucontrol).id) + dst_ofs;
            lola_mixer_set_dest_gains(chip, dst, mask, gains.as_mut_ptr())
        }
    }

    static lola_dest_gain_tlv: [c_uint; 4] =
        [SNDRV_CTL_TLVT_DB_SCALE, 2 * 4, (-8425_i32) as c_uint, 25 | 1];

    static mut lola_dest_gain_mixer: snd_kcontrol_new = snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        name: core::ptr::null_mut(),
        count: 0,
        info: Some(lola_dest_gain_info),
        get: Some(lola_dest_gain_get),
        put: Some(lola_dest_gain_put),
        private_value: 0,
        tlv: snd_kcontrol_tlv {
            p: lola_dest_gain_tlv.as_ptr(),
        },
    };

    unsafe fn create_dest_gain_mixer(
        chip: *mut lola,
        src_num: c_int,
        src_ofs: c_int,
        num: c_int,
        ofs: c_int,
        name: *mut c_char,
    ) -> c_int {
        unsafe {
            lola_dest_gain_mixer.count = num as c_uint;
            lola_dest_gain_mixer.name = name;
            lola_dest_gain_mixer.private_value =
                (src_ofs + (src_num << 8) + (ofs << 16) + (num << 24)) as c_ulong;
            snd_ctl_add(
                (*chip).card,
                snd_ctl_new1(&raw const lola_dest_gain_mixer, chip as *mut c_void),
            )
        }
    }
}

/*
 */
pub unsafe extern "C" fn lola_create_mixer(chip: *mut lola) -> c_int {
    let mut err: c_int;

    unsafe {
        err = create_analog_mixer(
            chip,
            PLAY as c_int,
            c"Analog Playback Volume".as_ptr() as *mut c_char,
        );
        if err < 0 {
            return err;
        }
        err = create_analog_mixer(
            chip,
            CAPT as c_int,
            c"Analog Capture Volume".as_ptr() as *mut c_char,
        );
        if err < 0 {
            return err;
        }
        err = create_input_src_mixer(chip);
        if err < 0 {
            return err;
        }
        err = create_src_gain_mixer(
            chip,
            (*chip).mixer.src_phys_ins,
            0,
            c"Digital Capture Volume".as_ptr() as *mut c_char,
        );
        if err < 0 {
            return err;
        }
        err = create_src_gain_mixer(
            chip,
            (*chip).mixer.src_stream_outs,
            (*chip).mixer.src_stream_out_ofs,
            c"Digital Playback Volume".as_ptr() as *mut c_char,
        );
        if err < 0 {
            return err;
        }
        #[cfg(any())]
        {
            /* FIXME: buggy mixer matrix handling */
            err = unused_dest_gain_mixer::create_dest_gain_mixer(
                chip,
                (*chip).mixer.src_phys_ins,
                0,
                (*chip).mixer.dest_stream_ins,
                0,
                c"Line Capture Volume".as_ptr() as *mut c_char,
            );
            if err < 0 {
                return err;
            }
            err = unused_dest_gain_mixer::create_dest_gain_mixer(
                chip,
                (*chip).mixer.src_stream_outs,
                (*chip).mixer.src_stream_out_ofs,
                (*chip).mixer.dest_stream_ins,
                0,
                c"Stream-Loopback Capture Volume".as_ptr() as *mut c_char,
            );
            if err < 0 {
                return err;
            }
            err = unused_dest_gain_mixer::create_dest_gain_mixer(
                chip,
                (*chip).mixer.src_phys_ins,
                0,
                (*chip).mixer.dest_phys_outs,
                (*chip).mixer.dest_phys_out_ofs,
                c"Line-Loopback Playback Volume".as_ptr() as *mut c_char,
            );
            if err < 0 {
                return err;
            }
            err = unused_dest_gain_mixer::create_dest_gain_mixer(
                chip,
                (*chip).mixer.src_stream_outs,
                (*chip).mixer.src_stream_out_ofs,
                (*chip).mixer.dest_phys_outs,
                (*chip).mixer.dest_phys_out_ofs,
                c"Stream Playback Volume".as_ptr() as *mut c_char,
            );
            if err < 0 {
                return err;
            }
        }
        init_mixer_values(chip)
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
