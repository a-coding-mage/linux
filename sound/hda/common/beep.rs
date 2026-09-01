// SPDX-License-Identifier: GPL-2.0+
/*
 * Digital Beep Input Interface for HD-audio codec
 *
 * Author: Matt Ranostay <matt.ranostay@konsulko.com>
 * Copyright (c) 2008 Embedded Alley Solutions Inc
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::offset_of;
use core::ptr;

const DIGBEEP_HZ_STEP: c_int = 46875; /* 46.875 Hz */
const DIGBEEP_HZ_MIN: c_int = 93750; /* 93.750 Hz */
const DIGBEEP_HZ_MAX: c_int = 12000000; /* 12 KHz */

const AC_VERB_SET_BEEP_CONTROL: c_uint = 0;
const AC_VERB_SET_DIGI_CONVERT_2: c_uint = 0;
const AC_AMPCAP_MUTE: c_uint = 0;
const SND_BELL: c_uint = 0;
const SND_TONE: c_uint = 0;
const EV_SND: c_uint = 0;
const BUS_PCI: c_uint = 0;
const SNDRV_DEV_JACK: c_uint = 0;
const HDA_BEEP_MODE_OFF: c_int = 0;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    pub parent: *mut device,
}

#[repr(C)]
pub struct input_id {
    pub bustype: c_uint,
    pub vendor: c_uint,
    pub product: c_uint,
    pub version: c_uint,
}

#[repr(C)]
pub struct input_dev {
    pub name: *const c_char,
    pub phys: *mut c_char,
    pub id: input_id,
    pub dev: device,
    pub evbit: [c_ulong; 1],
    pub sndbit: [c_ulong; 1],
    pub event: Option<
        unsafe extern "C" fn(*mut input_dev, c_uint, c_uint, c_int) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
    pub card_dev: device,
}

#[repr(C)]
pub struct hda_bus {
    pub shutdown: bool,
}

#[repr(C)]
pub struct hda_core {
    pub vendor_id: c_uint,
}

#[repr(C)]
pub struct hda_codec {
    pub beep: *mut hda_beep,
    pub beep_just_power_on: bool,
    pub beep_mode: c_int,
    pub card: *mut snd_card,
    pub addr: c_int,
    pub bus: *mut hda_bus,
    pub core: hda_core,
}

#[repr(C)]
pub struct hda_beep {
    pub codec: *mut hda_codec,
    pub power_hook: Option<unsafe extern "C" fn(*mut hda_beep, bool)>,
    pub playing: c_int,
    pub nid: c_int,
    pub beep_work: work_struct,
    pub enabled: c_int,
    pub tone: c_int,
    pub linear_tone: bool,
    pub keep_power_at_enable: bool,
    pub registered: bool,
    pub dev: *mut input_dev,
    pub phys: [c_char; 32],
}

#[repr(C)]
pub struct snd_device {
    pub device_data: *mut c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    pub dev_register: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_disconnect: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
    pub dev_free: Option<unsafe extern "C" fn(*mut snd_device) -> c_int>,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 2],
}

type c_ulong = usize;

unsafe extern "C" {
    fn snd_hda_power_up(codec: *mut hda_codec);
    fn snd_hda_power_down(codec: *mut hda_codec);
    fn snd_hda_power_up_pm(codec: *mut hda_codec);
    fn snd_hda_power_down_pm(codec: *mut hda_codec);
    fn snd_hda_codec_write(
        codec: *mut hda_codec,
        nid: c_int,
        flags: c_int,
        verb: c_uint,
        parm: c_int,
    );
    fn snd_hda_codec_write_cache(
        codec: *mut hda_codec,
        nid: c_int,
        flags: c_int,
        verb: c_uint,
        parm: c_int,
    );
    fn input_get_drvdata(dev: *mut input_dev) -> *mut c_void;
    fn input_set_drvdata(dev: *mut input_dev, data: *mut c_void);
    fn schedule_work(work: *mut work_struct) -> bool;
    fn cancel_work_sync(work: *mut work_struct) -> bool;
    fn input_register_device(dev: *mut input_dev) -> c_int;
    fn input_unregister_device(dev: *mut input_dev);
    fn input_free_device(dev: *mut input_dev);
    fn input_allocate_device() -> *mut input_dev;
    fn snd_device_new(
        card: *mut snd_card,
        type_: c_int,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn snd_device_free(card: *mut snd_card, device_data: *mut c_void) -> c_int;
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn snd_hda_get_bool_hint(codec: *mut hda_codec, key: *const c_char) -> bool;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut hda_codec;
    fn query_amp_caps(codec: *mut hda_codec, nid: c_int, direction: c_int) -> c_uint;
    fn get_amp_nid(kcontrol: *mut snd_kcontrol) -> c_int;
    fn get_amp_direction(kcontrol: *mut snd_kcontrol) -> c_int;
    fn get_amp_channels(kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_hda_mixer_amp_switch_get(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn snd_hda_mixer_amp_switch_put(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
}

const fn bit_mask(nr: c_uint) -> c_ulong {
    1usize << (nr as usize)
}

/* generate or stop tone */
unsafe fn generate_tone(beep: *mut hda_beep, tone: c_int) {
    let codec = (*beep).codec;

    if tone != 0 && (*beep).playing == 0 {
        snd_hda_power_up(codec);
        if let Some(power_hook) = (*beep).power_hook {
            power_hook(beep, true);
        }
        (*beep).playing = 1;
    }
    if !(*codec).beep_just_power_on {
        snd_hda_codec_write(codec, (*beep).nid, 0, AC_VERB_SET_BEEP_CONTROL, tone);
    }
    if tone == 0 && (*beep).playing != 0 {
        (*beep).playing = 0;
        if let Some(power_hook) = (*beep).power_hook {
            power_hook(beep, false);
        }
        snd_hda_power_down(codec);
    }
}

unsafe extern "C" fn snd_hda_generate_beep(work: *mut work_struct) {
    let beep = (work as *mut u8).sub(offset_of!(hda_beep, beep_work)) as *mut hda_beep;

    if (*beep).enabled != 0 {
        generate_tone(beep, (*beep).tone);
    }
}

/* (non-standard) Linear beep tone calculation for IDT/STAC codecs
 *
 * The tone frequency of beep generator on IDT/STAC codecs is
 * defined from the 8bit tone parameter, in Hz,
 *    freq = 48000 * (257 - tone) / 1024
 * that is from 12kHz to 93.75Hz in steps of 46.875 Hz
 */
unsafe fn beep_linear_tone(_beep: *mut hda_beep, mut hz: c_int) -> c_int {
    if hz <= 0 {
        return 0;
    }
    hz *= 1000; /* fixed point */
    hz = hz - DIGBEEP_HZ_MIN + DIGBEEP_HZ_STEP / 2; /* round to nearest step */
    if hz < 0 {
        hz = 0; /* turn off PC beep*/
    } else if hz >= DIGBEEP_HZ_MAX - DIGBEEP_HZ_MIN {
        hz = 1; /* max frequency */
    } else {
        hz /= DIGBEEP_HZ_STEP;
        hz = 255 - hz;
    }
    hz
}

/* HD-audio standard beep tone parameter calculation
 *
 * The tone frequency in Hz is calculated as
 *   freq = 48000 / (tone * 4)
 * from 47Hz to 12kHz
 */
unsafe fn beep_standard_tone(_beep: *mut hda_beep, mut hz: c_int) -> c_int {
    if hz <= 0 {
        return 0; /* disabled */
    }
    hz = 12000 / hz;
    if hz > 0xff {
        return 0xff;
    }
    if hz <= 0 {
        return 1;
    }
    hz
}

unsafe extern "C" fn snd_hda_beep_event(
    dev: *mut input_dev,
    _type: c_uint,
    code: c_uint,
    mut hz: c_int,
) -> c_int {
    let beep = input_get_drvdata(dev) as *mut hda_beep;

    match code {
        SND_BELL => {
            if hz != 0 {
                hz = 1000;
            }
            if (*beep).linear_tone {
                (*beep).tone = beep_linear_tone(beep, hz);
            } else {
                (*beep).tone = beep_standard_tone(beep, hz);
            }
        }
        SND_TONE => {
            if (*beep).linear_tone {
                (*beep).tone = beep_linear_tone(beep, hz);
            } else {
                (*beep).tone = beep_standard_tone(beep, hz);
            }
        }
        _ => {
            return -1;
        }
    }

    /* schedule beep event */
    schedule_work(&mut (*beep).beep_work);
    0
}

unsafe fn turn_on_beep(beep: *mut hda_beep) {
    if (*beep).keep_power_at_enable {
        snd_hda_power_up_pm((*beep).codec);
    }
}

unsafe fn turn_off_beep(beep: *mut hda_beep) {
    cancel_work_sync(&mut (*beep).beep_work);
    if (*beep).playing != 0 {
        /* turn off beep */
        generate_tone(beep, 0);
    }
    if (*beep).keep_power_at_enable {
        snd_hda_power_down_pm((*beep).codec);
    }
}

/**
 * snd_hda_enable_beep_device - Turn on/off beep sound
 * @codec: the HDA codec
 * @enable: flag to turn on/off
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_enable_beep_device(
    codec: *mut hda_codec,
    mut enable: c_int,
) -> c_int {
    let beep = (*codec).beep;
    if beep.is_null() {
        return 0;
    }
    enable = (enable != 0) as c_int;
    if (*beep).enabled != enable {
        (*beep).enabled = enable;
        if enable != 0 {
            turn_on_beep(beep);
        } else {
            turn_off_beep(beep);
        }
        return 1;
    }
    0
}

unsafe extern "C" fn beep_dev_register(device: *mut snd_device) -> c_int {
    let beep = (*device).device_data as *mut hda_beep;
    let err: c_int;

    err = input_register_device((*beep).dev);
    if err == 0 {
        (*beep).registered = true;
    }
    err
}

unsafe extern "C" fn beep_dev_disconnect(device: *mut snd_device) -> c_int {
    let beep = (*device).device_data as *mut hda_beep;

    if (*beep).registered {
        input_unregister_device((*beep).dev);
    } else {
        input_free_device((*beep).dev);
    }
    if (*beep).enabled != 0 {
        turn_off_beep(beep);
    }
    0
}

unsafe extern "C" fn beep_dev_free(device: *mut snd_device) -> c_int {
    let beep = (*device).device_data as *mut hda_beep;

    (*(*beep).codec).beep = ptr::null_mut();
    kfree(beep as *mut c_void);
    0
}

/**
 * snd_hda_attach_beep_device - Attach a beep input device
 * @codec: the HDA codec
 * @nid: beep NID
 *
 * Attach a beep object to the given widget.  If beep hint is turned off
 * explicitly or beep_mode of the codec is turned off, this doesn't nothing.
 *
 * Currently, only one beep device is allowed to each codec.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_attach_beep_device(
    codec: *mut hda_codec,
    nid: c_int,
) -> c_int {
    static OPS: snd_device_ops = snd_device_ops {
        dev_register: Some(beep_dev_register),
        dev_disconnect: Some(beep_dev_disconnect),
        dev_free: Some(beep_dev_free),
    };
    let input_dev: *mut input_dev;
    let beep: *mut hda_beep;
    let mut err: c_int;

    if !(*codec).beep_just_power_on {
        if !snd_hda_get_bool_hint(codec, c"beep".as_ptr()) {
            return 0; /* disabled explicitly by hints */
        }
        if (*codec).beep_mode == HDA_BEEP_MODE_OFF {
            return 0; /* disabled by module option */
        }
    }

    beep = kzalloc(core::mem::size_of::<hda_beep>(), 0) as *mut hda_beep;
    if beep.is_null() {
        return -ENOMEM;
    }
    snprintf(
        (*beep).phys.as_mut_ptr(),
        (*beep).phys.len(),
        c"card%d/codec#%d/beep0".as_ptr(),
        (*(*codec).card).number,
        (*codec).addr,
    );
    /* enable linear scale */
    snd_hda_codec_write_cache(codec, nid, 0, AC_VERB_SET_DIGI_CONVERT_2, 0x01);

    (*beep).nid = nid;
    (*beep).codec = codec;
    (*codec).beep = beep;

    /* INIT_WORK(&beep->beep_work, &snd_hda_generate_beep); */

    input_dev = input_allocate_device();
    if input_dev.is_null() {
        err = -ENOMEM;
        kfree(beep as *mut c_void);
        (*codec).beep = ptr::null_mut();
        return err;
    }

    /* setup digital beep device */
    (*input_dev).name = c"HDA Digital PCBeep".as_ptr();
    (*input_dev).phys = (*beep).phys.as_mut_ptr();
    (*input_dev).id.bustype = BUS_PCI;
    (*input_dev).dev.parent = &mut (*(*codec).card).card_dev;

    (*input_dev).id.vendor = (*codec).core.vendor_id >> 16;
    (*input_dev).id.product = (*codec).core.vendor_id & 0xffff;
    (*input_dev).id.version = 0x01;

    (*input_dev).evbit[0] = bit_mask(EV_SND);
    (*input_dev).sndbit[0] = bit_mask(SND_BELL) | bit_mask(SND_TONE);
    (*input_dev).event = Some(snd_hda_beep_event);
    input_set_drvdata(input_dev, beep as *mut c_void);

    (*beep).dev = input_dev;

    err = snd_device_new((*codec).card, SNDRV_DEV_JACK as c_int, beep as *mut c_void, &OPS);
    if err < 0 {
        input_free_device((*beep).dev);
        kfree(beep as *mut c_void);
        (*codec).beep = ptr::null_mut();
        return err;
    }

    0
}

/**
 * snd_hda_detach_beep_device - Detach the beep device
 * @codec: the HDA codec
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_detach_beep_device(codec: *mut hda_codec) {
    if !(*(*codec).bus).shutdown && !(*codec).beep.is_null() {
        snd_device_free((*codec).card, (*codec).beep as *mut c_void);
    }
}

unsafe fn ctl_has_mute(kcontrol: *mut snd_kcontrol) -> bool {
    let codec = snd_kcontrol_chip(kcontrol);
    (query_amp_caps(codec, get_amp_nid(kcontrol), get_amp_direction(kcontrol)) & AC_AMPCAP_MUTE)
        != 0
}

/* get/put callbacks for beep mute mixer switches */

/**
 * snd_hda_mixer_amp_switch_get_beep - Get callback for beep controls
 * @kcontrol: ctl element
 * @ucontrol: pointer to get/store the data
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_mixer_amp_switch_get_beep(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let beep = (*codec).beep;
    let chs = get_amp_channels(kcontrol);

    if !beep.is_null() && ((*beep).enabled == 0 || !ctl_has_mute(kcontrol)) {
        if chs & 1 != 0 {
            (*ucontrol).value.integer.value[0] = (*beep).enabled as c_long;
        }
        if chs & 2 != 0 {
            (*ucontrol).value.integer.value[1] = (*beep).enabled as c_long;
        }
        return 0;
    }
    snd_hda_mixer_amp_switch_get(kcontrol, ucontrol)
}

/**
 * snd_hda_mixer_amp_switch_put_beep - Put callback for beep controls
 * @kcontrol: ctl element
 * @ucontrol: pointer to get/store the data
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_hda_mixer_amp_switch_put_beep(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let codec = snd_kcontrol_chip(kcontrol);
    let beep = (*codec).beep;
    if !beep.is_null() {
        let chs = get_amp_channels(kcontrol) as u8;
        let mut enable: c_int = 0;
        let mut valp = (*ucontrol).value.integer.value.as_mut_ptr();
        if chs & 1 != 0 {
            enable |= *valp as c_int;
            valp = valp.add(1);
        }
        if chs & 2 != 0 {
            enable |= *valp as c_int;
        }
        snd_hda_enable_beep_device(codec, enable);
    }
    if !ctl_has_mute(kcontrol) {
        return 0;
    }
    snd_hda_mixer_amp_switch_put(kcontrol, ucontrol)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
