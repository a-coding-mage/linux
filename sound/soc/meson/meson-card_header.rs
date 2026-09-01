/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

use core::ffi::{c_int, c_uint, c_void};

/* Header guard _MESON_SND_CARD_H omitted in Rust. */

pub type bool = u8;

/* External dependency declarations from included kernel headers. */
extern "C" {
    pub type device_node;
    pub type platform_device;
    pub type snd_soc_card;
    pub type snd_pcm_substream;
    pub type snd_pcm_hw_params;
    pub type snd_soc_dai_link_component;
    pub type snd_soc_dai_link;
}

pub const DT_PREFIX: &[u8; 9] = b"amlogic,\0";

#[repr(C)]
pub struct meson_card_match_data {
    pub add_link: Option<
        unsafe extern "C" fn(
            card: *mut snd_soc_card,
            node: *mut device_node,
            index: *mut c_int,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct meson_card {
    pub match_data: *const meson_card_match_data,
    pub card: snd_soc_card,
    pub link_data: *mut *mut c_void,
}

extern "C" {
    pub fn meson_card_parse_daifmt(
        node: *mut device_node,
        cpu_node: *mut device_node,
    ) -> c_uint;

    pub fn meson_card_i2s_set_sysclk(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        mclk_fs: c_uint,
    ) -> c_int;

    pub fn meson_card_reallocate_links(card: *mut snd_soc_card, num_links: c_uint) -> c_int;

    pub fn meson_card_parse_dai(
        card: *mut snd_soc_card,
        node: *mut device_node,
        dlc: *mut snd_soc_dai_link_component,
    ) -> c_int;

    pub fn meson_card_set_be_link(
        card: *mut snd_soc_card,
        link: *mut snd_soc_dai_link,
        node: *mut device_node,
    ) -> c_int;

    pub fn meson_card_set_fe_link(
        card: *mut snd_soc_card,
        link: *mut snd_soc_dai_link,
        node: *mut device_node,
        is_playback: bool,
    ) -> c_int;

    pub fn meson_card_probe(pdev: *mut platform_device) -> c_int;

    pub fn meson_card_remove(pdev: *mut platform_device);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
