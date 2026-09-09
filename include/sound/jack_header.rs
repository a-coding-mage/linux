/* SPDX-License-Identifier: GPL-2.0-or-later */

/*
 *  Jack abstraction layer
 *
 *  Copyright 2008 Wolfson Microelectronics plc
 */

/* C dependency: <sound/core.h> */

/* C forward declaration: struct input_dev; */

/**
 * enum snd_jack_types - Jack types which can be reported
 * @SND_JACK_HEADPHONE: Headphone
 * @SND_JACK_MICROPHONE: Microphone
 * @SND_JACK_HEADSET: Headset
 * @SND_JACK_LINEOUT: Line out
 * @SND_JACK_MECHANICAL: Mechanical switch
 * @SND_JACK_VIDEOOUT: Video out
 * @SND_JACK_AVOUT: AV (Audio Video) out
 * @SND_JACK_LINEIN:  Line in
 * @SND_JACK_USB: USB audio device
 * @SND_JACK_BTN_0: Button 0
 * @SND_JACK_BTN_1: Button 1
 * @SND_JACK_BTN_2: Button 2
 * @SND_JACK_BTN_3: Button 3
 * @SND_JACK_BTN_4: Button 4
 * @SND_JACK_BTN_5: Button 5
 *
 * These values are used as a bitmask.
 *
 * Note that this must be kept in sync with the lookup table in
 * sound/core/jack.c.
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_jack_types {
    SND_JACK_HEADPHONE = 0x0001,
    SND_JACK_MICROPHONE = 0x0002,
    SND_JACK_HEADSET = 0x0001 | 0x0002,
    SND_JACK_LINEOUT = 0x0004,
    SND_JACK_MECHANICAL = 0x0008, /* If detected separately */
    SND_JACK_VIDEOOUT = 0x0010,
    SND_JACK_AVOUT = 0x0004 | 0x0010,
    SND_JACK_LINEIN = 0x0020,
    SND_JACK_USB = 0x0040,

    /* Kept separate from switches to facilitate implementation */
    SND_JACK_BTN_0 = 0x4000,
    SND_JACK_BTN_1 = 0x2000,
    SND_JACK_BTN_2 = 0x1000,
    SND_JACK_BTN_3 = 0x0800,
    SND_JACK_BTN_4 = 0x0400,
    SND_JACK_BTN_5 = 0x0200,
}

/* Keep in sync with definitions above */
pub const SND_JACK_SWITCH_TYPES: i32 = 7;

#[repr(C)]
pub struct snd_jack {
    pub kctl_list: list_head,
    pub card: *mut snd_card,
    pub id: *const core::ffi::c_char,
    /* CONFIG_SND_JACK_INPUT_DEV conditionally includes the following fields. */
    #[cfg(CONFIG_SND_JACK_INPUT_DEV)]
    pub input_dev: *mut input_dev,
    #[cfg(CONFIG_SND_JACK_INPUT_DEV)]
    pub input_dev_lock: mutex,
    #[cfg(CONFIG_SND_JACK_INPUT_DEV)]
    pub registered: i32,
    #[cfg(CONFIG_SND_JACK_INPUT_DEV)]
    pub type_: i32,
    #[cfg(CONFIG_SND_JACK_INPUT_DEV)]
    pub name: [core::ffi::c_char; 100],
    #[cfg(CONFIG_SND_JACK_INPUT_DEV)]
    pub key: [u32; 6], /* Keep in sync with definitions above */
    pub hw_status_cache: i32,
    pub private_data: *mut core::ffi::c_void,
    pub private_free: Option<unsafe extern "C" fn(jack: *mut snd_jack)>,
}

/* CONFIG_SND_JACK conditionally declares the external implementations. */
#[cfg(CONFIG_SND_JACK)]
extern "C" {
    pub fn snd_jack_new(
        card: *mut snd_card,
        id: *const core::ffi::c_char,
        type_: i32,
        jack: *mut *mut snd_jack,
        initial_kctl: bool,
        phantom_jack: bool,
    ) -> i32;
    pub fn snd_jack_add_new_kctl(
        jack: *mut snd_jack,
        name: *const core::ffi::c_char,
        mask: i32,
    ) -> i32;
    #[cfg(CONFIG_SND_JACK_INPUT_DEV)]
    pub fn snd_jack_set_key(
        jack: *mut snd_jack,
        type_: snd_jack_types,
        keytype: i32,
    ) -> i32;
    pub fn snd_jack_report(jack: *mut snd_jack, status: i32);
}

/* CONFIG_SND_JACK disabled: the C inline stubs return zero or do nothing. */
#[cfg(not(CONFIG_SND_JACK))]
pub unsafe fn snd_jack_new(
    _card: *mut snd_card,
    _id: *const core::ffi::c_char,
    _type: i32,
    _jack: *mut *mut snd_jack,
    _initial_kctl: bool,
    _phantom_jack: bool,
) -> i32 {
    0
}

#[cfg(not(CONFIG_SND_JACK))]
pub unsafe fn snd_jack_add_new_kctl(
    _jack: *mut snd_jack,
    _name: *const core::ffi::c_char,
    _mask: i32,
) -> i32 {
    0
}

#[cfg(not(CONFIG_SND_JACK))]
pub unsafe fn snd_jack_report(_jack: *mut snd_jack, _status: i32) {}

/* !CONFIG_SND_JACK || !CONFIG_SND_JACK_INPUT_DEV */
#[cfg(any(not(CONFIG_SND_JACK), not(CONFIG_SND_JACK_INPUT_DEV)))]
pub unsafe fn snd_jack_set_key(
    _jack: *mut snd_jack,
    _type_: snd_jack_types,
    _keytype: i32,
) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
