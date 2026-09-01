// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Helper functions for jack-detection kcontrols
 *
 * Copyright (c) 2011 Takashi Iwai <tiwai@suse.de>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// C dependency intent:
// #include <linux/kernel.h>
// #include <linux/export.h>
// #include <linux/string.h>
// #include <sound/core.h>
// #include <sound/control.h>

// Missing external constants from <sound/control.h>; values are supplied by the
// surrounding translated repository bindings.
const SNDRV_CTL_ELEM_IFACE_CARD: c_uint = 0; // TODO: external dependency
const SNDRV_CTL_ELEM_ACCESS_READ: c_uint = 0; // TODO: external dependency
const SNDRV_CTL_EVENT_MASK_VALUE: c_uint = 0; // TODO: external dependency

const SND_CTL_ELEM_ID_NAME_LEN: usize = 44;

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_id {
    pub numid: c_uint,
    pub iface: c_uint,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; SND_CTL_ELEM_ID_NAME_LEN],
    pub index: c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_id {
    pub numid: c_uint,
    pub iface: c_uint,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: [c_char; SND_CTL_ELEM_ID_NAME_LEN],
    pub index: c_uint,
}

#[repr(C)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: core::mem::ManuallyDrop<snd_ctl_elem_value_integer>,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_kcontrol_id,
    pub private_value: isize,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub device: c_uint,
    pub subdevice: c_uint,
    pub name: *const c_char,
    pub index: c_uint,
    pub access: c_uint,
    pub info: Option<
        unsafe extern "C" fn(
            kcontrol: *mut snd_kcontrol,
            uinfo: *mut c_void,
        ) -> c_int,
    >,
    pub get: Option<
        unsafe extern "C" fn(
            kcontrol: *mut snd_kcontrol,
            ucontrol: *mut snd_ctl_elem_value,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    fn snd_ctl_boolean_mono_info(
        kcontrol: *mut snd_kcontrol,
        uinfo: *mut c_void,
    ) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn strscpy(dest: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(cs: *const c_char, ct: *const c_char, count: usize) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn snd_ctl_find_id(card: *mut snd_card, id: *mut snd_ctl_elem_id) -> *mut c_void;
    fn snd_ctl_new1(
        ncontrol: *const snd_kcontrol_new,
        private_data: *mut c_void,
    ) -> *mut snd_kcontrol;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_kcontrol_id);
}

// #define jack_detect_kctl_info snd_ctl_boolean_mono_info

unsafe extern "C" fn jack_detect_kctl_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    unsafe {
        (*ucontrol).value.integer.value[0] = (*kcontrol).private_value as i64;
    }
    0
}

static jack_detect_kctl: snd_kcontrol_new = snd_kcontrol_new {
    /* name is filled later */
    iface: SNDRV_CTL_ELEM_IFACE_CARD,
    device: 0,
    subdevice: 0,
    name: core::ptr::null(),
    index: 0,
    access: SNDRV_CTL_ELEM_ACCESS_READ,
    info: Some(snd_ctl_boolean_mono_info),
    get: Some(jack_detect_kctl_get),
};

unsafe fn get_available_index(card: *mut snd_card, name: *const c_char) -> c_int {
    let mut sid: snd_ctl_elem_id = unsafe { core::mem::zeroed() };

    unsafe {
        memset(
            &mut sid as *mut snd_ctl_elem_id as *mut c_void,
            0,
            core::mem::size_of::<snd_ctl_elem_id>(),
        );
    }

    sid.index = 0;
    sid.iface = SNDRV_CTL_ELEM_IFACE_CARD;
    unsafe {
        strscpy(sid.name.as_mut_ptr(), name, sid.name.len());
    }

    unsafe {
        while !snd_ctl_find_id(card, &mut sid).is_null() {
            sid.index = sid.index.wrapping_add(1);
            /* reset numid; otherwise snd_ctl_find_id() hits this again */
            sid.numid = 0;
        }
    }

    sid.index as c_int
}

unsafe fn jack_kctl_name_gen(name: *mut c_char, src_name: *const c_char, size: usize) {
    let count: usize = unsafe { strlen(src_name) };
    let suf = b" Jack\0";
    let suf_ptr = suf.as_ptr() as *const c_char;
    let suf_len: usize = unsafe { strlen(suf_ptr) };
    let mut append_suf: bool = true;

    if count >= suf_len {
        append_suf = unsafe { strncmp(src_name.add(count - suf_len), suf_ptr, suf_len) != 0 };
    }

    if append_suf {
        let fmt = b"%s%s\0";
        unsafe {
            snprintf(name, size, fmt.as_ptr() as *const c_char, src_name, suf_ptr);
        }
    } else {
        unsafe {
            strscpy(name, src_name, size);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_kctl_jack_new(
    name: *const c_char,
    card: *mut snd_card,
) -> *mut snd_kcontrol {
    let kctl: *mut snd_kcontrol;

    unsafe {
        kctl = snd_ctl_new1(&jack_detect_kctl, core::ptr::null_mut());
    }
    if kctl.is_null() {
        return core::ptr::null_mut();
    }

    unsafe {
        jack_kctl_name_gen(
            (*kctl).id.name.as_mut_ptr(),
            name,
            (*kctl).id.name.len(),
        );
        (*kctl).id.index = get_available_index(card, (*kctl).id.name.as_ptr()) as c_uint;
        (*kctl).private_value = 0;
    }
    kctl
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn snd_kctl_jack_report(
    card: *mut snd_card,
    kctl: *mut snd_kcontrol,
    status: bool,
) {
    unsafe {
        if (*kctl).private_value == status as isize {
            return;
        }
        (*kctl).private_value = status as isize;
        snd_ctl_notify(card, SNDRV_CTL_EVENT_MASK_VALUE, &mut (*kctl).id);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
