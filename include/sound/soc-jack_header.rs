/* SPDX-License-Identifier: GPL-2.0
 *
 * soc-jack.h
 *
 * Copyright (C) 2019 Renesas Electronics Corp.
 * Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
 */

/* Types supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct snd_soc_jack_pin {
    pub list: list_head,
    pub pin: *const ::core::ffi::c_char,
    pub mask: ::core::ffi::c_int,
    pub invert: bool,
}

#[repr(C)]
pub struct snd_soc_jack_zone {
    pub min_mv: ::core::ffi::c_uint,
    pub max_mv: ::core::ffi::c_uint,
    pub jack_type: ::core::ffi::c_uint,
    pub debounce_time: ::core::ffi::c_uint,
    pub list: list_head,
}

#[repr(C)]
pub struct snd_soc_jack_gpio {
    pub idx: ::core::ffi::c_uint,
    pub gpiod_dev: *mut device,
    pub name: *const ::core::ffi::c_char,
    pub report: ::core::ffi::c_int,
    pub invert: ::core::ffi::c_int,
    pub debounce_time: ::core::ffi::c_int,
    pub wake: bool,

    /* private: */
    pub jack: *mut snd_soc_jack,
    pub work: delayed_work,
    pub pm_notifier: notifier_block,
    pub desc: *mut gpio_desc,

    pub data: *mut ::core::ffi::c_void,
    /* public: */
    pub jack_status_check:
        Option<unsafe extern "C" fn(data: *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct snd_soc_jack {
    pub mutex: mutex,
    pub jack: *mut snd_jack,
    pub card: *mut snd_soc_card,
    pub pins: list_head,
    pub status: ::core::ffi::c_int,
    pub notifier: blocking_notifier_head,
    pub jack_zones: list_head,
}

extern "C" {
    pub fn snd_soc_jack_report(
        jack: *mut snd_soc_jack,
        status: ::core::ffi::c_int,
        mask: ::core::ffi::c_int,
    );
    pub fn snd_soc_jack_add_pins(
        jack: *mut snd_soc_jack,
        count: ::core::ffi::c_int,
        pins: *mut snd_soc_jack_pin,
    ) -> ::core::ffi::c_int;
    pub fn snd_soc_jack_notifier_register(
        jack: *mut snd_soc_jack,
        nb: *mut notifier_block,
    );
    pub fn snd_soc_jack_notifier_unregister(
        jack: *mut snd_soc_jack,
        nb: *mut notifier_block,
    );
    pub fn snd_soc_jack_add_zones(
        jack: *mut snd_soc_jack,
        count: ::core::ffi::c_int,
        zones: *mut snd_soc_jack_zone,
    ) -> ::core::ffi::c_int;
    pub fn snd_soc_jack_get_type(
        jack: *mut snd_soc_jack,
        micbias_voltage: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

/* CONFIG_GPIOLIB declarations; when disabled, the C header supplies inline stubs. */
#[cfg(feature = "CONFIG_GPIOLIB")]
extern "C" {
    pub fn snd_soc_jack_add_gpios(
        jack: *mut snd_soc_jack,
        count: ::core::ffi::c_int,
        gpios: *mut snd_soc_jack_gpio,
    ) -> ::core::ffi::c_int;
    pub fn snd_soc_jack_add_gpiods(
        gpiod_dev: *mut device,
        jack: *mut snd_soc_jack,
        count: ::core::ffi::c_int,
        gpios: *mut snd_soc_jack_gpio,
    ) -> ::core::ffi::c_int;
    pub fn snd_soc_jack_free_gpios(
        jack: *mut snd_soc_jack,
        count: ::core::ffi::c_int,
        gpios: *mut snd_soc_jack_gpio,
    );
}

#[cfg(not(feature = "CONFIG_GPIOLIB"))]
pub unsafe fn snd_soc_jack_add_gpios(
    _jack: *mut snd_soc_jack,
    _count: ::core::ffi::c_int,
    _gpios: *mut snd_soc_jack_gpio,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_GPIOLIB"))]
pub unsafe fn snd_soc_jack_add_gpiods(
    _gpiod_dev: *mut device,
    _jack: *mut snd_soc_jack,
    _count: ::core::ffi::c_int,
    _gpios: *mut snd_soc_jack_gpio,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_GPIOLIB"))]
pub unsafe fn snd_soc_jack_free_gpios(
    _jack: *mut snd_soc_jack,
    _count: ::core::ffi::c_int,
    _gpios: *mut snd_soc_jack_gpio,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
