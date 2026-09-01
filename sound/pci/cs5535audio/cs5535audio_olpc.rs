// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OLPC XO-1 additional sound features
 *
 * Copyright © 2006  Jaya Kumar <jayakumar.lkml@gmail.com>
 * Copyright © 2007-2008  Andres Salomon <dilinger@debian.org>
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;

const DRV_NAME: &[u8] = b"cs5535audio-olpc\0";

const AC97_AD_TEST2: c_uint = 0;
const AC97_AD_HPFD_SHIFT: c_uint = 0;
const AC97_AD_MISC: c_uint = 0;
const AC97_AD_VREFD_SHIFT: c_uint = 0;
const OLPC_GPIO_MIC_AC: c_uint = 0;
const SNDRV_CTL_ELEM_TYPE_BOOLEAN: c_uint = 0;
const SNDRV_CTL_ELEM_IFACE_MIXER: c_uint = 0;
const AC97_SCAP_INV_EAPD: c_uint = 0;
const EIO: c_int = 5;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_ac97_bus {
    pub card: *mut snd_card,
}

#[repr(C)]
pub struct snd_ac97 {
    pub bus: *mut snd_ac97_bus,
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_ac97_template {
    pub scaps: c_uint,
}

#[repr(C)]
pub struct cs5535audio {
    pub ac97: *mut snd_ac97,
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
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
pub struct snd_ctl_elem_id {
    pub iface: c_uint,
    pub name: [c_char; 44],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

unsafe extern "C" {
    fn machine_is_olpc() -> c_int;
    fn snd_ac97_update_bits(
        ac97: *mut snd_ac97,
        reg: c_uint,
        mask: c_uint,
        value: c_uint,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn gpio_set_value(gpio: c_uint, value: c_int);
    fn gpio_get_value(gpio: c_uint) -> c_int;
    fn snd_kcontrol_chip(kctl: *mut snd_kcontrol) -> *mut c_void;
    fn snd_ac97_read(ac97: *mut snd_ac97, reg: c_uint) -> c_uint;
    fn olpc_board_pre(id: c_uint) -> c_uint;
    fn olpc_board_at_least(id: c_uint) -> c_int;
    fn gpio_request(gpio: c_uint, label: *const c_char) -> c_int;
    fn gpio_direction_output(gpio: c_uint, value: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn snd_ctl_remove_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> c_int;
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> c_int;
    fn snd_ctl_new1(ncontrol: *const snd_kcontrol_new, private_data: *mut c_void) -> *mut snd_kcontrol;
    fn gpio_free(gpio: c_uint);
}

/*
 * OLPC has an additional feature on top of the regular AD1888 codec features.
 * It has an Analog Input mode that is switched into (after disabling the
 * High Pass Filter) via GPIO.  It is supported on B2 and later models.
 */
#[no_mangle]
pub unsafe extern "C" fn olpc_analog_input(ac97: *mut snd_ac97, on: c_int) {
    let err: c_int;

    if machine_is_olpc() == 0 {
        return;
    }

    /* update the High Pass Filter (via AC97_AD_TEST2) */
    err = snd_ac97_update_bits(
        ac97,
        AC97_AD_TEST2,
        1u32 << AC97_AD_HPFD_SHIFT,
        (on as c_uint) << AC97_AD_HPFD_SHIFT,
    );
    if err < 0 {
        dev_err(
            (*(*(*ac97).bus).card).dev,
            b"setting High Pass Filter - %d\n\0".as_ptr() as *const c_char,
            err,
        );
        return;
    }

    /* set Analog Input through GPIO */
    gpio_set_value(OLPC_GPIO_MIC_AC, on);
}

/*
 * OLPC XO-1's V_REFOUT is a mic bias enable.
 */
#[no_mangle]
pub unsafe extern "C" fn olpc_mic_bias(ac97: *mut snd_ac97, mut on: c_int) {
    let err: c_int;

    if machine_is_olpc() == 0 {
        return;
    }

    on = if on != 0 { 0 } else { 1 };
    err = snd_ac97_update_bits(
        ac97,
        AC97_AD_MISC,
        1u32 << AC97_AD_VREFD_SHIFT,
        (on as c_uint) << AC97_AD_VREFD_SHIFT,
    );
    if err < 0 {
        dev_err(
            (*(*(*ac97).bus).card).dev,
            b"setting MIC Bias - %d\n\0".as_ptr() as *const c_char,
            err,
        );
    }
}

unsafe extern "C" fn olpc_dc_info(
    _kctl: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe extern "C" fn olpc_dc_get(
    _kctl: *mut snd_kcontrol,
    v: *mut snd_ctl_elem_value,
) -> c_int {
    (*v).value.integer.value[0] = gpio_get_value(OLPC_GPIO_MIC_AC) as c_long;
    0
}

unsafe extern "C" fn olpc_dc_put(
    kctl: *mut snd_kcontrol,
    v: *mut snd_ctl_elem_value,
) -> c_int {
    let cs5535au = snd_kcontrol_chip(kctl) as *mut cs5535audio;

    olpc_analog_input((*cs5535au).ac97, (*v).value.integer.value[0] as c_int);
    1
}

unsafe extern "C" fn olpc_mic_info(
    _kctl: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BOOLEAN;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = 1;
    0
}

unsafe extern "C" fn olpc_mic_get(
    kctl: *mut snd_kcontrol,
    v: *mut snd_ctl_elem_value,
) -> c_int {
    let cs5535au = snd_kcontrol_chip(kctl) as *mut cs5535audio;
    let ac97 = (*cs5535au).ac97;
    let i: c_int;

    i = ((snd_ac97_read(ac97, AC97_AD_MISC) >> AC97_AD_VREFD_SHIFT) & 0x1) as c_int;
    (*v).value.integer.value[0] = if i != 0 { 0 } else { 1 };
    0
}

unsafe extern "C" fn olpc_mic_put(
    kctl: *mut snd_kcontrol,
    v: *mut snd_ctl_elem_value,
) -> c_int {
    let cs5535au = snd_kcontrol_chip(kctl) as *mut cs5535audio;

    olpc_mic_bias((*cs5535au).ac97, (*v).value.integer.value[0] as c_int);
    1
}

static OLPC_CS5535AUDIO_CTLS: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"DC Mode Enable\0".as_ptr() as *const c_char,
        info: Some(olpc_dc_info),
        get: Some(olpc_dc_get),
        put: Some(olpc_dc_put),
        private_value: 0,
    },
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: b"MIC Bias Enable\0".as_ptr() as *const c_char,
        info: Some(olpc_mic_info),
        get: Some(olpc_mic_get),
        put: Some(olpc_mic_put),
        private_value: 0,
    },
];

#[no_mangle]
pub unsafe extern "C" fn olpc_prequirks(
    _card: *mut snd_card,
    ac97: *mut snd_ac97_template,
) {
    if machine_is_olpc() == 0 {
        return;
    }

    /* invert EAPD if on an OLPC B3 or higher */
    if olpc_board_at_least(olpc_board_pre(0xb3)) != 0 {
        (*ac97).scaps |= AC97_SCAP_INV_EAPD;
    }
}

#[no_mangle]
pub unsafe extern "C" fn olpc_quirks(card: *mut snd_card, ac97: *mut snd_ac97) -> c_int {
    let mut elem: snd_ctl_elem_id = core::mem::zeroed();
    let mut i: usize;
    let mut err: c_int;

    if machine_is_olpc() == 0 {
        return 0;
    }

    if gpio_request(OLPC_GPIO_MIC_AC, DRV_NAME.as_ptr() as *const c_char) != 0 {
        dev_err((*card).dev, b"unable to allocate MIC GPIO\n\0".as_ptr() as *const c_char);
        return -EIO;
    }
    gpio_direction_output(OLPC_GPIO_MIC_AC, 0);

    /* drop the original AD1888 HPF control */
    memset(
        &mut elem as *mut snd_ctl_elem_id as *mut c_void,
        0,
        size_of::<snd_ctl_elem_id>(),
    );
    elem.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    strscpy(
        elem.name.as_mut_ptr(),
        b"High Pass Filter Enable\0".as_ptr() as *const c_char,
        elem.name.len(),
    );
    snd_ctl_remove_id(card, &mut elem);

    /* drop the original V_REFOUT control */
    memset(
        &mut elem as *mut snd_ctl_elem_id as *mut c_void,
        0,
        size_of::<snd_ctl_elem_id>(),
    );
    elem.iface = SNDRV_CTL_ELEM_IFACE_MIXER;
    strscpy(
        elem.name.as_mut_ptr(),
        b"V_REFOUT Enable\0".as_ptr() as *const c_char,
        elem.name.len(),
    );
    snd_ctl_remove_id(card, &mut elem);

    /* add the OLPC-specific controls */
    i = 0;
    while i < OLPC_CS5535AUDIO_CTLS.len() {
        err = snd_ctl_add(
            card,
            snd_ctl_new1(&OLPC_CS5535AUDIO_CTLS[i], (*ac97).private_data),
        );
        if err < 0 {
            return err;
        }
        i += 1;
    }

    /* turn off the mic by default */
    olpc_mic_bias(ac97, 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn olpc_quirks_cleanup() {
    if machine_is_olpc() != 0 {
        gpio_free(OLPC_GPIO_MIC_AC);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
