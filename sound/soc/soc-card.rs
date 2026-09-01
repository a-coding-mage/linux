// SPDX-License-Identifier: GPL-2.0
//
// soc-card.c
//
// Copyright (C) 2019 Renesas Electronics Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type bool_ = bool;
type gfp_t = c_uint;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack_pin {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct blocking_notifier_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub enum snd_soc_bias_level {
    _Unused = 0,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub name: *const c_char,
    pub snd_card: *mut snd_card,
    pub suspend_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub suspend_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub resume_pre: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub resume_post: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub fixup_controls: Option<unsafe extern "C" fn(*mut snd_soc_card)>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub set_bias_level: Option<
        unsafe extern "C" fn(
            *mut snd_soc_card,
            *mut snd_soc_dapm_context,
            snd_soc_bias_level,
        ) -> c_int,
    >,
    pub set_bias_level_post: Option<
        unsafe extern "C" fn(
            *mut snd_soc_card,
            *mut snd_soc_dapm_context,
            snd_soc_bias_level,
        ) -> c_int,
    >,
    pub add_dai_link:
        Option<unsafe extern "C" fn(*mut snd_soc_card, *mut snd_soc_dai_link) -> c_int>,
    pub remove_dai_link: Option<unsafe extern "C" fn(*mut snd_soc_card, *mut snd_soc_dai_link)>,
    pub probed: c_int,
    pub topology_shortname: *const c_char,
}

#[repr(C)]
pub struct snd_soc_jack {
    pub mutex: mutex,
    pub card: *mut snd_soc_card,
    pub pins: list_head,
    pub jack_zones: list_head,
    pub notifier: blocking_notifier_head,
    pub jack: *mut snd_jack,
}

unsafe extern "C" {
    fn snd_soc_ret(dev: *mut device, ret: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_ctl_find_id_mixer(card: *mut snd_card, name: *const c_char) -> *mut snd_kcontrol;
    fn mutex_init(mutex: *mut mutex);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn BLOCKING_INIT_NOTIFIER_HEAD(notifier: *mut blocking_notifier_head);
    fn snd_jack_new(
        card: *mut snd_card,
        id: *const c_char,
        type_: c_int,
        jack: *mut *mut snd_jack,
        initial_kctl: bool_,
        phantom_jack: bool_,
    ) -> c_int;
    fn snd_soc_jack_add_pins(
        jack: *mut snd_soc_jack,
        count: c_uint,
        pins: *mut snd_soc_jack_pin,
    ) -> c_int;
    fn devm_kasprintf(dev: *mut device, gfp: gfp_t, fmt: *const c_char, ...) -> *mut c_char;

    // GFP_KERNEL is a Linux preprocessor constant supplied by headers outside this file.
    static GFP_KERNEL: gfp_t;
}

#[inline]
unsafe fn _soc_card_ret(card: *mut snd_soc_card, func: *const c_char, ret: c_int) -> c_int {
    unsafe {
        snd_soc_ret(
            (*card).dev,
            ret,
            b"at %s() on %s\n\0".as_ptr() as *const c_char,
            func,
            (*card).name,
        )
    }
}

#[inline]
unsafe fn soc_card_ret(card: *mut snd_soc_card, func: *const c_char, ret: c_int) -> c_int {
    unsafe { _soc_card_ret(card, func, ret) }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_get_kcontrol(
    soc_card: *mut snd_soc_card,
    name: *const c_char,
) -> *mut snd_kcontrol {
    unsafe {
        if name.is_null() {
            return core::ptr::null_mut();
        }

        snd_ctl_find_id_mixer((*soc_card).snd_card, name)
    }
}
// EXPORT_SYMBOL_GPL(snd_soc_card_get_kcontrol);

unsafe fn jack_new(
    card: *mut snd_soc_card,
    id: *const c_char,
    type_: c_int,
    jack: *mut snd_soc_jack,
    initial_kctl: bool_,
) -> c_int {
    unsafe {
        mutex_init(&mut (*jack).mutex);
        (*jack).card = card;
        INIT_LIST_HEAD(&mut (*jack).pins);
        INIT_LIST_HEAD(&mut (*jack).jack_zones);
        BLOCKING_INIT_NOTIFIER_HEAD(&mut (*jack).notifier);

        snd_jack_new(
            (*card).snd_card,
            id,
            type_,
            &mut (*jack).jack,
            initial_kctl,
            false,
        )
    }
}

/**
 * snd_soc_card_jack_new - Create a new jack without pins
 * @card:  ASoC card
 * @id:    an identifying string for this jack
 * @type:  a bitmask of enum snd_jack_type values that can be detected by
 *         this jack
 * @jack:  structure to use for the jack
 *
 * Creates a new jack object without pins. If adding pins later,
 * snd_soc_card_jack_new_pins() should be used instead with 0 as num_pins
 * argument.
 *
 * Returns zero if successful, or a negative error code on failure.
 * On success jack will be initialised.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_jack_new(
    card: *mut snd_soc_card,
    id: *const c_char,
    type_: c_int,
    jack: *mut snd_soc_jack,
) -> c_int {
    unsafe {
        soc_card_ret(
            card,
            b"snd_soc_card_jack_new\0".as_ptr() as *const c_char,
            jack_new(card, id, type_, jack, true),
        )
    }
}
// EXPORT_SYMBOL_GPL(snd_soc_card_jack_new);

/**
 * snd_soc_card_jack_new_pins - Create a new jack with pins
 * @card:  ASoC card
 * @id:    an identifying string for this jack
 * @type:  a bitmask of enum snd_jack_type values that can be detected by
 *         this jack
 * @jack:  structure to use for the jack
 * @pins:  Array of jack pins to be added to the jack or NULL
 * @num_pins: Number of elements in the @pins array
 *
 * Creates a new jack object with pins. If not adding pins,
 * snd_soc_card_jack_new() should be used instead.
 *
 * Returns zero if successful, or a negative error code on failure.
 * On success jack will be initialised.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_jack_new_pins(
    card: *mut snd_soc_card,
    id: *const c_char,
    type_: c_int,
    jack: *mut snd_soc_jack,
    pins: *mut snd_soc_jack_pin,
    num_pins: c_uint,
) -> c_int {
    unsafe {
        let mut ret: c_int;

        ret = jack_new(card, id, type_, jack, false);
        if ret != 0 {
            return soc_card_ret(
                card,
                b"snd_soc_card_jack_new_pins\0".as_ptr() as *const c_char,
                ret,
            );
        }

        if num_pins != 0 {
            ret = snd_soc_jack_add_pins(jack, num_pins, pins);
        }

        soc_card_ret(
            card,
            b"snd_soc_card_jack_new_pins\0".as_ptr() as *const c_char,
            ret,
        )
    }
}
// EXPORT_SYMBOL_GPL(snd_soc_card_jack_new_pins);

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_suspend_pre(card: *mut snd_soc_card) -> c_int {
    unsafe {
        let mut ret: c_int = 0;

        if let Some(suspend_pre) = (*card).suspend_pre {
            ret = suspend_pre(card);
        }

        soc_card_ret(
            card,
            b"snd_soc_card_suspend_pre\0".as_ptr() as *const c_char,
            ret,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_suspend_post(card: *mut snd_soc_card) -> c_int {
    unsafe {
        let mut ret: c_int = 0;

        if let Some(suspend_post) = (*card).suspend_post {
            ret = suspend_post(card);
        }

        soc_card_ret(
            card,
            b"snd_soc_card_suspend_post\0".as_ptr() as *const c_char,
            ret,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_resume_pre(card: *mut snd_soc_card) -> c_int {
    unsafe {
        let mut ret: c_int = 0;

        if let Some(resume_pre) = (*card).resume_pre {
            ret = resume_pre(card);
        }

        soc_card_ret(
            card,
            b"snd_soc_card_resume_pre\0".as_ptr() as *const c_char,
            ret,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_resume_post(card: *mut snd_soc_card) -> c_int {
    unsafe {
        let mut ret: c_int = 0;

        if let Some(resume_post) = (*card).resume_post {
            ret = resume_post(card);
        }

        soc_card_ret(
            card,
            b"snd_soc_card_resume_post\0".as_ptr() as *const c_char,
            ret,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_probe(card: *mut snd_soc_card) -> c_int {
    unsafe {
        if let Some(probe) = (*card).probe {
            let ret: c_int = probe(card);

            if ret < 0 {
                return soc_card_ret(
                    card,
                    b"snd_soc_card_probe\0".as_ptr() as *const c_char,
                    ret,
                );
            }

            /*
             * It has "card->probe" and "card->late_probe" callbacks.
             * So, set "probed" flag here, because it needs to care
             * about "late_probe".
             *
             * see
             *	snd_soc_bind_card()
             *	snd_soc_card_late_probe()
             */
            (*card).probed = 1;
        }

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_late_probe(card: *mut snd_soc_card) -> c_int {
    unsafe {
        if let Some(late_probe) = (*card).late_probe {
            let ret: c_int = late_probe(card);

            if ret < 0 {
                return soc_card_ret(
                    card,
                    b"snd_soc_card_late_probe\0".as_ptr() as *const c_char,
                    ret,
                );
            }
        }

        /*
         * It has "card->probe" and "card->late_probe" callbacks,
         * and "late_probe" callback is called after "probe".
         * This means, we can set "card->probed" flag afer "late_probe"
         * for all cases.
         *
         * see
         *	snd_soc_bind_card()
         *	snd_soc_card_probe()
         */
        (*card).probed = 1;

        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_fixup_controls(card: *mut snd_soc_card) {
    unsafe {
        if let Some(fixup_controls) = (*card).fixup_controls {
            fixup_controls(card);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_remove(card: *mut snd_soc_card) -> c_int {
    unsafe {
        let mut ret: c_int = 0;

        if (*card).probed != 0 {
            if let Some(remove) = (*card).remove {
                ret = remove(card);
            }
        }

        (*card).probed = 0;

        soc_card_ret(
            card,
            b"snd_soc_card_remove\0".as_ptr() as *const c_char,
            ret,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_set_bias_level(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    unsafe {
        let mut ret: c_int = 0;

        if let Some(set_bias_level) = (*card).set_bias_level {
            ret = set_bias_level(card, dapm, level);
        }

        soc_card_ret(
            card,
            b"snd_soc_card_set_bias_level\0".as_ptr() as *const c_char,
            ret,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_set_bias_level_post(
    card: *mut snd_soc_card,
    dapm: *mut snd_soc_dapm_context,
    level: snd_soc_bias_level,
) -> c_int {
    unsafe {
        let mut ret: c_int = 0;

        if let Some(set_bias_level_post) = (*card).set_bias_level_post {
            ret = set_bias_level_post(card, dapm, level);
        }

        soc_card_ret(
            card,
            b"snd_soc_card_set_bias_level_post\0".as_ptr() as *const c_char,
            ret,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_add_dai_link(
    card: *mut snd_soc_card,
    dai_link: *mut snd_soc_dai_link,
) -> c_int {
    unsafe {
        let mut ret: c_int = 0;

        if let Some(add_dai_link) = (*card).add_dai_link {
            ret = add_dai_link(card, dai_link);
        }

        soc_card_ret(
            card,
            b"snd_soc_card_add_dai_link\0".as_ptr() as *const c_char,
            ret,
        )
    }
}
// EXPORT_SYMBOL_GPL(snd_soc_card_add_dai_link);

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_remove_dai_link(
    card: *mut snd_soc_card,
    dai_link: *mut snd_soc_dai_link,
) {
    unsafe {
        if let Some(remove_dai_link) = (*card).remove_dai_link {
            remove_dai_link(card, dai_link);
        }
    }
}
// EXPORT_SYMBOL_GPL(snd_soc_card_remove_dai_link);

#[no_mangle]
pub unsafe extern "C" fn snd_soc_card_set_topology_name(
    card: *mut snd_soc_card,
    prefix: *const c_char,
) {
    unsafe {
        if prefix.is_null() || (*card).name.is_null() {
            return;
        }

        if (*card).topology_shortname.is_null() {
            (*card).topology_shortname = devm_kasprintf(
                (*card).dev,
                GFP_KERNEL,
                b"%s-%s\0".as_ptr() as *const c_char,
                prefix,
                (*card).name,
            );
        }

        (*card).name = (*card).topology_shortname;
    }
}
// EXPORT_SYMBOL_GPL(snd_soc_card_set_topology_name);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
