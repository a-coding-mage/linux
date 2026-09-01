/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Apple Onboard Audio definitions
 *
 * Copyright 2006 Johannes Berg <johannes@sipsolutions.net>
 */

/* Rust translation of aoa.h. C include dependencies:
 * linux/module.h, sound/core.h, sound/asound.h, sound/control.h,
 * aoa-gpio.h, soundbus/soundbus.h
 */

use core::ffi::{c_char, c_int, c_void};

pub const MAX_CODEC_NAME_LEN: usize = 32;

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soundbus_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gpio_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
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

#[repr(C)]
pub struct gpio_methods {
    _private: [u8; 0],
}

/* enum snd_device_type is supplied by sound/core.h. */
pub type snd_device_type = c_int;

#[repr(C)]
pub struct aoa_codec {
    pub name: [c_char; MAX_CODEC_NAME_LEN],

    pub owner: *mut module,

    /* called when the fabric wants to init this codec.
     * Do alsa card manipulations from here. */
    pub init: Option<unsafe extern "C" fn(codec: *mut aoa_codec) -> c_int>,

    /* called when the fabric is done with the codec.
     * The alsa card will be cleaned up so don't bother. */
    pub exit: Option<unsafe extern "C" fn(codec: *mut aoa_codec)>,

    /* May be NULL, but can be used by the fabric.
     * Refcounting is the codec driver's responsibility */
    pub node: *mut device_node,

    /* assigned by fabric before init() is called, points
     * to the soundbus device. Cannot be NULL. */
    pub soundbus_dev: *mut soundbus_dev,

    /* assigned by the fabric before init() is called, points
     * to the fabric's gpio runtime record for the relevant
     * device. */
    pub gpio: *mut gpio_runtime,

    /* assigned by the fabric before init() is called, contains
     * a codec specific bitmask of what outputs and inputs are
     * actually connected */
    pub connected: u32,

    /* data the fabric can associate with this structure */
    pub fabric_data: *const c_void,

    /* private! */
    pub list: list_head,
    pub fabric: *mut aoa_fabric,
}

pub const MAX_LAYOUT_NAME_LEN: usize = 32;

#[repr(C)]
pub struct aoa_fabric {
    pub name: [c_char; MAX_LAYOUT_NAME_LEN],

    pub owner: *mut module,

    /* once codecs register, they are passed here after.
     * They are of course not initialised, since the
     * fabric is responsible for initialising some fields
     * in the codec structure! */
    pub found_codec: Option<unsafe extern "C" fn(codec: *mut aoa_codec) -> c_int>,
    /* called for each codec when it is removed,
     * also in the case that aoa_fabric_unregister
     * is called and all codecs are removed
     * from this fabric.
     * Also called if found_codec returned 0 but
     * the codec couldn't initialise. */
    pub remove_codec: Option<unsafe extern "C" fn(codec: *mut aoa_codec)>,
    /* If found_codec returned 0, and the codec
     * could be initialised, this is called. */
    pub attached_codec: Option<unsafe extern "C" fn(codec: *mut aoa_codec)>,
}

/* alsa help methods */
#[repr(C)]
pub struct aoa_card {
    pub alsa_card: *mut snd_card,
}

unsafe extern "C" {
    /* return 0 on success */
    pub fn aoa_codec_register(codec: *mut aoa_codec) -> c_int;
    pub fn aoa_codec_unregister(codec: *mut aoa_codec);

    /* return 0 on success, -EEXIST if another fabric is
     * registered, -EALREADY if the same fabric is registered.
     * Passing NULL can be used to test for the presence
     * of another fabric, if -EALREADY is returned there is
     * no other fabric present.
     * In the case that the function returns -EALREADY
     * and the fabric passed is not NULL, all codecs
     * that are not assigned yet are passed to the fabric
     * again for reconsideration. */
    pub fn aoa_fabric_register(fabric: *mut aoa_fabric, dev: *mut device) -> c_int;

    /* it is vital to call this when the fabric exits!
     * When calling, the remove_codec will be called
     * for all codecs, unless it is NULL. */
    pub fn aoa_fabric_unregister(fabric: *mut aoa_fabric);

    /* if for some reason you want to get rid of a codec
     * before the fabric is removed, use this.
     * Note that remove_codec is called for it! */
    pub fn aoa_fabric_unlink_codec(codec: *mut aoa_codec);

    pub fn aoa_snd_device_new(
        type_: snd_device_type,
        device_data: *mut c_void,
        ops: *const snd_device_ops,
    ) -> c_int;
    pub fn aoa_get_card() -> *mut snd_card;
    pub fn aoa_snd_ctl_add(control: *mut snd_kcontrol) -> c_int;

    /* GPIO stuff */
    pub static mut pmf_gpio_methods: *mut gpio_methods;
    pub static mut ftr_gpio_methods: *mut gpio_methods;
    /* extern struct gpio_methods *map_gpio_methods; */
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
