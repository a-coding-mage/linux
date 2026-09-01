// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Routines for control of the TEA6330T circuit via i2c bus
 *  Sound fader control circuit for car radios by Philips Semiconductors
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// C include dependencies:
// <linux/init.h>, <linux/slab.h>, <linux/module.h>,
// <sound/core.h>, <sound/control.h>, <sound/tea6330t.h>

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;

const SNDRV_CTL_ELEM_IFACE_MIXER: c_int = 2;
const SNDRV_CTL_ELEM_TYPE_INTEGER: c_int = 2;

const TEA6330T_ADDR: c_uchar = 0x80 >> 1; /* fixed address */

const TEA6330T_SADDR_VOLUME_LEFT: usize = 0x00; /* volume left */
const TEA6330T_SADDR_VOLUME_RIGHT: usize = 0x01; /* volume right */
const TEA6330T_SADDR_BASS: usize = 0x02; /* bass control */
const TEA6330T_SADDR_TREBLE: usize = 0x03; /* treble control */
const TEA6330T_SADDR_FADER: usize = 0x04; /* fader control */
const TEA6330T_MFN: c_uchar = 0x20; /* mute control for selected channels */
const TEA6330T_FCH: c_uchar = 0x10; /* select fader channels - front or rear */
const TEA6330T_SADDR_AUDIO_SWITCH: usize = 0x05; /* audio switch */
const TEA6330T_GMU: c_uchar = 0x80; /* mute control, general mute */
const TEA6330T_EQN: c_uchar = 0x40; /* equalizer switchover (0=equalizer-on) */

type c_uchar = u8;
type u8_ = u8;

#[repr(C)]
pub struct snd_i2c_bus {
    pub devices: list_head,
}

#[repr(C)]
pub struct snd_i2c_device {
    pub list: list_head,
    pub addr: c_uchar,
    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(*mut snd_i2c_device)>,
}

#[repr(C)]
pub struct snd_card {
    pub mixername: *mut c_char,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
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
    pub min: c_long,
    pub max: c_long,
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
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [c_long; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

type c_long = i64;

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_int,
    pub name: *const c_char,
    pub index: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
struct tea6330t {
    device: *mut snd_i2c_device,
    bus: *mut snd_i2c_bus,
    equalizer: c_int,
    fader: c_int,
    regs: [c_uchar; 8],
    mleft: c_uchar,
    mright: c_uchar,
    bass: c_uchar,
    treble: c_uchar,
    max_bass: c_uchar,
    max_treble: c_uchar,
}

unsafe extern "C" {
    fn snd_i2c_lock(bus: *mut snd_i2c_bus);
    fn snd_i2c_unlock(bus: *mut snd_i2c_bus);
    fn snd_i2c_probeaddr(bus: *mut snd_i2c_bus, addr: c_uchar) -> c_int;
    fn snd_i2c_sendbytes(device: *mut snd_i2c_device, bytes: *mut c_uchar, count: c_int) -> c_int;
    fn snd_i2c_device_create(
        bus: *mut snd_i2c_bus,
        name: *const c_char,
        addr: c_uchar,
        device: *mut *mut snd_i2c_device,
    ) -> c_int;
    fn snd_i2c_device_free(device: *mut snd_i2c_device);
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ctl_boolean_stereo_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut snd_ctl_elem_info,
    ) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(knew: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn snd_component_add(card: *mut snd_card, component: *const c_char) -> c_int;
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

unsafe fn kzalloc_obj_tea6330t() -> *mut tea6330t {
    kzalloc(core::mem::size_of::<tea6330t>(), 0) as *mut tea6330t
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_tea6330t_detect(bus: *mut snd_i2c_bus, equalizer: c_int) -> c_int {
    let mut res: c_int;

    let _ = equalizer;
    snd_i2c_lock(bus);
    res = snd_i2c_probeaddr(bus, TEA6330T_ADDR);
    snd_i2c_unlock(bus);
    res
}

/*
static void snd_tea6330t_set(struct tea6330t *tea,
			     unsigned char addr, unsigned char value)
{
	snd_i2c_write(tea->bus, TEA6330T_ADDR, addr, value, 1);
}
*/

// TEA6330T_MASTER_VOLUME(xname, xindex)
unsafe extern "C" fn snd_tea6330t_info_master_volume(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER as c_uint;
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 43;
    0
}

unsafe extern "C" fn snd_tea6330t_get_master_volume(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tea = snd_kcontrol_chip(kcontrol) as *mut tea6330t;

    snd_i2c_lock((*tea).bus);
    (*ucontrol).value.integer.value[0] = ((*tea).mleft as c_long) - 0x14;
    (*ucontrol).value.integer.value[1] = ((*tea).mright as c_long) - 0x14;
    snd_i2c_unlock((*tea).bus);
    0
}

unsafe extern "C" fn snd_tea6330t_put_master_volume(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tea = snd_kcontrol_chip(kcontrol) as *mut tea6330t;
    let mut change: c_int;
    let mut count: c_int;
    let mut err: c_int;
    let mut bytes = [0 as c_uchar; 3];
    let val1: c_uchar;
    let val2: c_uchar;

    val1 = (((*ucontrol).value.integer.value[0] % 44) + 0x14) as c_uchar;
    val2 = (((*ucontrol).value.integer.value[1] % 44) + 0x14) as c_uchar;
    snd_i2c_lock((*tea).bus);
    change = (val1 != (*tea).mleft || val2 != (*tea).mright) as c_int;
    (*tea).mleft = val1;
    (*tea).mright = val2;
    count = 0;
    if (*tea).regs[TEA6330T_SADDR_VOLUME_LEFT] != 0 {
        bytes[count as usize] = TEA6330T_SADDR_VOLUME_LEFT as c_uchar;
        count += 1;
        (*tea).regs[TEA6330T_SADDR_VOLUME_LEFT] = (*tea).mleft;
        bytes[count as usize] = (*tea).regs[TEA6330T_SADDR_VOLUME_LEFT];
        count += 1;
    }
    if (*tea).regs[TEA6330T_SADDR_VOLUME_RIGHT] != 0 {
        if count == 0 {
            bytes[count as usize] = TEA6330T_SADDR_VOLUME_RIGHT as c_uchar;
            count += 1;
        }
        (*tea).regs[TEA6330T_SADDR_VOLUME_RIGHT] = (*tea).mright;
        bytes[count as usize] = (*tea).regs[TEA6330T_SADDR_VOLUME_RIGHT];
        count += 1;
    }
    if count > 0 {
        err = snd_i2c_sendbytes((*tea).device, bytes.as_mut_ptr(), count);
        if err < 0 {
            change = err;
        }
    }
    snd_i2c_unlock((*tea).bus);
    change
}

// TEA6330T_MASTER_SWITCH(xname, xindex)
unsafe extern "C" fn snd_tea6330t_get_master_switch(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tea = snd_kcontrol_chip(kcontrol) as *mut tea6330t;

    snd_i2c_lock((*tea).bus);
    (*ucontrol).value.integer.value[0] =
        if (*tea).regs[TEA6330T_SADDR_VOLUME_LEFT] == 0 { 0 } else { 1 };
    (*ucontrol).value.integer.value[1] =
        if (*tea).regs[TEA6330T_SADDR_VOLUME_RIGHT] == 0 { 0 } else { 1 };
    snd_i2c_unlock((*tea).bus);
    0
}

unsafe extern "C" fn snd_tea6330t_put_master_switch(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tea = snd_kcontrol_chip(kcontrol) as *mut tea6330t;
    let mut change: c_int;
    let err: c_int;
    let mut bytes = [0 as c_uchar; 3];
    let oval1: c_uchar;
    let oval2: c_uchar;
    let val1: c_uchar;
    let val2: c_uchar;

    val1 = ((*ucontrol).value.integer.value[0] & 1) as c_uchar;
    val2 = ((*ucontrol).value.integer.value[1] & 1) as c_uchar;
    snd_i2c_lock((*tea).bus);
    oval1 = if (*tea).regs[TEA6330T_SADDR_VOLUME_LEFT] == 0 { 0 } else { 1 };
    oval2 = if (*tea).regs[TEA6330T_SADDR_VOLUME_RIGHT] == 0 { 0 } else { 1 };
    change = (val1 != oval1 || val2 != oval2) as c_int;
    (*tea).regs[TEA6330T_SADDR_VOLUME_LEFT] = if val1 != 0 { (*tea).mleft } else { 0 };
    (*tea).regs[TEA6330T_SADDR_VOLUME_RIGHT] = if val2 != 0 { (*tea).mright } else { 0 };
    bytes[0] = TEA6330T_SADDR_VOLUME_LEFT as c_uchar;
    bytes[1] = (*tea).regs[TEA6330T_SADDR_VOLUME_LEFT];
    bytes[2] = (*tea).regs[TEA6330T_SADDR_VOLUME_RIGHT];
    err = snd_i2c_sendbytes((*tea).device, bytes.as_mut_ptr(), 3);
    if err < 0 {
        change = err;
    }
    snd_i2c_unlock((*tea).bus);
    change
}

// TEA6330T_BASS(xname, xindex)
unsafe extern "C" fn snd_tea6330t_info_bass(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let tea = snd_kcontrol_chip(kcontrol) as *mut tea6330t;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER as c_uint;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*tea).max_bass as c_long;
    0
}

unsafe extern "C" fn snd_tea6330t_get_bass(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tea = snd_kcontrol_chip(kcontrol) as *mut tea6330t;

    (*ucontrol).value.integer.value[0] = (*tea).bass as c_long;
    0
}

unsafe extern "C" fn snd_tea6330t_put_bass(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tea = snd_kcontrol_chip(kcontrol) as *mut tea6330t;
    let mut change: c_int;
    let err: c_int;
    let mut bytes = [0 as c_uchar; 2];
    let mut val1: c_uchar;

    val1 = ((*ucontrol).value.integer.value[0] % (((*tea).max_bass as c_long) + 1)) as c_uchar;
    snd_i2c_lock((*tea).bus);
    (*tea).bass = val1;
    val1 = val1.wrapping_add(if (*tea).equalizer != 0 { 7 } else { 3 });
    change = ((*tea).regs[TEA6330T_SADDR_BASS] != val1) as c_int;
    bytes[0] = TEA6330T_SADDR_BASS as c_uchar;
    (*tea).regs[TEA6330T_SADDR_BASS] = val1;
    bytes[1] = (*tea).regs[TEA6330T_SADDR_BASS];
    err = snd_i2c_sendbytes((*tea).device, bytes.as_mut_ptr(), 2);
    if err < 0 {
        change = err;
    }
    snd_i2c_unlock((*tea).bus);
    change
}

// TEA6330T_TREBLE(xname, xindex)
unsafe extern "C" fn snd_tea6330t_info_treble(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let tea = snd_kcontrol_chip(kcontrol) as *mut tea6330t;

    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER as c_uint;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = (*tea).max_treble as c_long;
    0
}

unsafe extern "C" fn snd_tea6330t_get_treble(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tea = snd_kcontrol_chip(kcontrol) as *mut tea6330t;

    (*ucontrol).value.integer.value[0] = (*tea).treble as c_long;
    0
}

unsafe extern "C" fn snd_tea6330t_put_treble(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let tea = snd_kcontrol_chip(kcontrol) as *mut tea6330t;
    let mut change: c_int;
    let err: c_int;
    let mut bytes = [0 as c_uchar; 2];
    let mut val1: c_uchar;

    val1 = ((*ucontrol).value.integer.value[0] % (((*tea).max_treble as c_long) + 1)) as c_uchar;
    snd_i2c_lock((*tea).bus);
    (*tea).treble = val1;
    val1 = val1.wrapping_add(3);
    change = ((*tea).regs[TEA6330T_SADDR_TREBLE] != val1) as c_int;
    bytes[0] = TEA6330T_SADDR_TREBLE as c_uchar;
    (*tea).regs[TEA6330T_SADDR_TREBLE] = val1;
    bytes[1] = (*tea).regs[TEA6330T_SADDR_TREBLE];
    err = snd_i2c_sendbytes((*tea).device, bytes.as_mut_ptr(), 2);
    if err < 0 {
        change = err;
    }
    snd_i2c_unlock((*tea).bus);
    change
}

static MASTER_PLAYBACK_SWITCH_NAME: &[u8] = b"Master Playback Switch\0";
static MASTER_PLAYBACK_VOLUME_NAME: &[u8] = b"Master Playback Volume\0";
static TONE_CONTROL_BASS_NAME: &[u8] = b"Tone Control - Bass\0";
static TONE_CONTROL_TREBLE_NAME: &[u8] = b"Tone Control - Treble\0";
static TEA6330T_NAME: &[u8] = b"TEA6330T\0";
static MIXER_SUFFIX: &[u8] = b",TEA6330T\0";

static snd_tea6330t_controls: [snd_kcontrol_new; 4] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: MASTER_PLAYBACK_SWITCH_NAME.as_ptr() as *const c_char,
        index: 0,
        info: Some(snd_ctl_boolean_stereo_info),
        get: Some(snd_tea6330t_get_master_switch),
        put: Some(snd_tea6330t_put_master_switch),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: MASTER_PLAYBACK_VOLUME_NAME.as_ptr() as *const c_char,
        index: 0,
        info: Some(snd_tea6330t_info_master_volume),
        get: Some(snd_tea6330t_get_master_volume),
        put: Some(snd_tea6330t_put_master_volume),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: TONE_CONTROL_BASS_NAME.as_ptr() as *const c_char,
        index: 0,
        info: Some(snd_tea6330t_info_bass),
        get: Some(snd_tea6330t_get_bass),
        put: Some(snd_tea6330t_put_bass),
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: TONE_CONTROL_TREBLE_NAME.as_ptr() as *const c_char,
        index: 0,
        info: Some(snd_tea6330t_info_treble),
        get: Some(snd_tea6330t_get_treble),
        put: Some(snd_tea6330t_put_treble),
    },
];

unsafe extern "C" fn snd_tea6330_free(device: *mut snd_i2c_device) {
    kfree((*device).private_data);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_tea6330t_update_mixer(
    card: *mut snd_card,
    bus: *mut snd_i2c_bus,
    equalizer: c_int,
    fader: c_int,
) -> c_int {
    let mut device: *mut snd_i2c_device = core::ptr::null_mut();
    let tea: *mut tea6330t;
    let mut knew: *const snd_kcontrol_new;
    let mut idx: c_uint;
    let mut err: c_int;
    let default_treble: u8_;
    let default_bass: u8_;
    let mut bytes = [0 as c_uchar; 7];

    tea = kzalloc_obj_tea6330t();
    if tea.is_null() {
        return -ENOMEM;
    }
    err = snd_i2c_device_create(bus, TEA6330T_NAME.as_ptr() as *const c_char, TEA6330T_ADDR, &mut device);
    if err < 0 {
        kfree(tea as *mut c_void);
        return err;
    }
    (*tea).device = device;
    (*tea).bus = bus;
    (*tea).equalizer = equalizer;
    (*tea).fader = fader;
    (*device).private_data = tea as *mut c_void;
    (*device).private_free = Some(snd_tea6330_free);

    snd_i2c_lock(bus);

    /* turn fader off and handle equalizer */
    (*tea).regs[TEA6330T_SADDR_FADER] = 0x3f;
    (*tea).regs[TEA6330T_SADDR_AUDIO_SWITCH] = if equalizer != 0 { 0 } else { TEA6330T_EQN };
    /* initialize mixer */
    if (*tea).equalizer == 0 {
        (*tea).max_bass = 9;
        (*tea).max_treble = 8;
        default_bass = 3 + 4;
        (*tea).bass = 4;
        default_treble = 3 + 4;
        (*tea).treble = 4;
    } else {
        (*tea).max_bass = 5;
        (*tea).max_treble = 0;
        default_bass = 7 + 4;
        (*tea).bass = 4;
        default_treble = 3;
        (*tea).treble = 0;
    }
    (*tea).mright = 0x14;
    (*tea).mleft = (*tea).mright;
    (*tea).regs[TEA6330T_SADDR_BASS] = default_bass;
    (*tea).regs[TEA6330T_SADDR_TREBLE] = default_treble;

    /* compose I2C message and put the hardware to initial state */
    bytes[0] = TEA6330T_SADDR_VOLUME_LEFT as c_uchar;
    idx = 0;
    while idx < 6 {
        bytes[idx as usize + 1] = (*tea).regs[idx as usize];
        idx += 1;
    }
    err = snd_i2c_sendbytes(device, bytes.as_mut_ptr(), 7);
    if err < 0 {
        snd_i2c_unlock(bus);
        snd_i2c_device_free(device);
        return err;
    }

    strcat((*card).mixername, MIXER_SUFFIX.as_ptr() as *const c_char);
    err = snd_component_add(card, TEA6330T_NAME.as_ptr() as *const c_char);
    if err < 0 {
        snd_i2c_unlock(bus);
        snd_i2c_device_free(device);
        return err;
    }

    idx = 0;
    while (idx as usize) < snd_tea6330t_controls.len() {
        knew = &snd_tea6330t_controls[idx as usize];
        if (*tea).treble == 0
            && strcmp((*knew).name, TONE_CONTROL_TREBLE_NAME.as_ptr() as *const c_char) == 0
        {
            idx += 1;
            continue;
        }
        err = snd_ctl_add(card, snd_ctl_new1(knew, tea as *mut c_void));
        if err < 0 {
            snd_i2c_unlock(bus);
            snd_i2c_device_free(device);
            return err;
        }
        idx += 1;
    }

    snd_i2c_unlock(bus);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_tea6330t_restore_mixer(bus: *mut snd_i2c_bus) -> c_int {
    let mut device: *mut snd_i2c_device;
    let tea: *mut tea6330t;
    let mut bytes = [0 as c_uchar; 7];
    let mut idx: c_uint;
    let mut err: c_int;

    if bus.is_null() {
        return -EINVAL;
    }

    snd_i2c_lock(bus);
    /*
     * C source uses:
     * list_for_each_entry(device, &bus->devices, list)
     * The loop below preserves the Linux intrusive-list traversal intent.
     */
    let head = &mut (*bus).devices as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        device = (pos as *mut u8).sub(core::mem::offset_of!(snd_i2c_device, list)) as *mut snd_i2c_device;
        if (*device).addr != TEA6330T_ADDR {
            pos = (*pos).next;
            continue;
        }

        tea = (*device).private_data as *mut tea6330t;
        if tea.is_null() {
            err = -EINVAL;
            snd_i2c_unlock(bus);
            return err;
        }

        bytes[0] = TEA6330T_SADDR_VOLUME_LEFT as c_uchar;
        idx = 0;
        while idx < 6 {
            bytes[idx as usize + 1] = (*tea).regs[idx as usize];
            idx += 1;
        }
        err = snd_i2c_sendbytes(device, bytes.as_mut_ptr(), 7);
        err = if err < 0 { err } else { 0 };
        snd_i2c_unlock(bus);
        return err;
    }

    err = -ENODEV;

    snd_i2c_unlock(bus);
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
