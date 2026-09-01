// SPDX-License-Identifier: GPL-2.0-only
/*
 * Apple Onboard Audio Alsa helpers
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */
// C dependencies: <linux/module.h>, "alsa.h"

use core::ffi::{c_char, c_int, c_void};

const EBUSY: c_int = 16;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_device_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

pub type snd_device_type = c_int;

#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub driver: *mut c_char,
    pub shortname: *mut c_char,
    pub longname: *mut c_char,
    pub mixername: *mut c_char,
}

#[repr(C)]
pub struct aoa_card {
    pub alsa_card: *mut snd_card,
}

unsafe extern "C" {
    fn snd_card_new(
        dev: *mut device,
        idx: c_int,
        xid: *mut c_char,
        module: *mut module,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn snd_device_new(
        card: *mut snd_card,
        type_: snd_device_type,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    fn snd_device_register(card: *mut snd_card, device_data: *mut c_void) -> c_int;
    fn snd_device_free(card: *mut snd_card, device_data: *mut c_void);
    fn snd_ctl_add(card: *mut snd_card, control: *mut snd_kcontrol) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn printk(fmt: *const c_char, ...) -> c_int;
}

static mut INDEX: c_int = -1;
// module_param(index, int, 0444);
// MODULE_PARM_DESC(index, "index for AOA sound card.");

static mut AOA_CARD: *mut aoa_card = core::ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn aoa_alsa_init(
    name: *mut c_char,
    mod_: *mut module,
    dev: *mut device,
) -> c_int {
    let mut alsa_card: *mut snd_card = core::ptr::null_mut();
    let mut err: c_int;

    if !AOA_CARD.is_null() {
        /* cannot be EEXIST due to usage in aoa_fabric_register */
        return -EBUSY;
    }

    err = snd_card_new(
        dev,
        INDEX,
        name,
        mod_,
        core::mem::size_of::<aoa_card>(),
        &mut alsa_card,
    );
    if err < 0 {
        return err;
    }
    AOA_CARD = (*alsa_card).private_data as *mut aoa_card;
    (*AOA_CARD).alsa_card = alsa_card;
    strscpy((*alsa_card).driver, c"AppleOnbdAudio".as_ptr());
    strscpy((*alsa_card).shortname, name);
    strscpy((*alsa_card).longname, name);
    strscpy((*alsa_card).mixername, name);
    err = snd_card_register((*AOA_CARD).alsa_card);
    if err < 0 {
        printk(c"snd-aoa: couldn't register alsa card\n".as_ptr());
        snd_card_free((*AOA_CARD).alsa_card);
        AOA_CARD = core::ptr::null_mut();
        return err;
    }
    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn aoa_get_card() -> *mut snd_card {
    if !AOA_CARD.is_null() {
        return (*AOA_CARD).alsa_card;
    }
    return core::ptr::null_mut();
}
// EXPORT_SYMBOL_GPL(aoa_get_card);

#[no_mangle]
pub unsafe extern "C" fn aoa_alsa_cleanup() {
    if !AOA_CARD.is_null() {
        snd_card_free((*AOA_CARD).alsa_card);
        AOA_CARD = core::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn aoa_snd_device_new(
    type_: snd_device_type,
    device_data: *mut c_void,
    ops: *const snd_device_ops,
) -> c_int {
    let card: *mut snd_card = aoa_get_card();
    let mut err: c_int;

    if card.is_null() {
        return -ENOMEM;
    }

    err = snd_device_new(card, type_, device_data, ops);
    if err != 0 {
        printk(
            c"snd-aoa: failed to create snd device (%d)\n".as_ptr(),
            err,
        );
        return err;
    }
    err = snd_device_register(card, device_data);
    if err != 0 {
        printk(
            c"snd-aoa: failed to register snd device (%d)\n".as_ptr(),
            err,
        );
        printk(c"snd-aoa: have you forgotten the dev_register callback?\n".as_ptr());
        snd_device_free(card, device_data);
    }
    return err;
}
// EXPORT_SYMBOL_GPL(aoa_snd_device_new);

#[no_mangle]
pub unsafe extern "C" fn aoa_snd_ctl_add(control: *mut snd_kcontrol) -> c_int {
    let err: c_int;

    if AOA_CARD.is_null() {
        return -ENODEV;
    }

    err = snd_ctl_add((*AOA_CARD).alsa_card, control);
    if err != 0 {
        printk(
            c"snd-aoa: failed to add alsa control (%d)\n".as_ptr(),
            err,
        );
    }
    return err;
}
// EXPORT_SYMBOL_GPL(aoa_snd_ctl_add);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
