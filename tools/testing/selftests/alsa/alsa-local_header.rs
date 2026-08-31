// SPDX-License-Identifier: GPL-2.0
//
// kselftest configuration helpers for the hw specific configuration
//
// Original author: Jaroslav Kysela <perex@perex.cz>
// Copyright (c) 2022 Red Hat Inc.

use core::ffi::{c_char, c_int, c_long};

// Dependency intent from C header:
// #include <alsa/asoundlib.h>
#[repr(C)]
pub struct snd_config_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn get_alsalib_config() -> *mut snd_config_t;

    pub fn conf_load_from_file(filename: *const c_char) -> *mut snd_config_t;
    pub fn conf_load();
    pub fn conf_free();
    pub fn conf_by_card(card: c_int) -> *mut snd_config_t;
    pub fn conf_get_subtree(
        root: *mut snd_config_t,
        key1: *const c_char,
        key2: *const c_char,
    ) -> *mut snd_config_t;
    pub fn conf_get_count(
        root: *mut snd_config_t,
        key1: *const c_char,
        key2: *const c_char,
    ) -> c_int;
    pub fn conf_get_string(
        root: *mut snd_config_t,
        key1: *const c_char,
        key2: *const c_char,
        def: *const c_char,
    ) -> *const c_char;
    pub fn conf_get_long(
        root: *mut snd_config_t,
        key1: *const c_char,
        key2: *const c_char,
        def: c_long,
    ) -> c_long;
    pub fn conf_get_bool(
        root: *mut snd_config_t,
        key1: *const c_char,
        key2: *const c_char,
        def: c_int,
    ) -> c_int;
    pub fn conf_get_string_array(
        root: *mut snd_config_t,
        key1: *const c_char,
        key2: *const c_char,
        array: *mut *const c_char,
        array_size: c_int,
        def: *const c_char,
    );
}

#[repr(C)]
pub struct card_cfg_data {
    pub card: c_int,
    pub config: *mut snd_config_t,
    pub filename: *const c_char,
    pub config_id: *const c_char,
    pub next: *mut card_cfg_data,
}

unsafe extern "C" {
    pub static mut conf_cards: *mut card_cfg_data;
}
