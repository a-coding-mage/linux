// SPDX-License-Identifier: GPL-2.0
/*
 * mt8186-mt6366-common.h
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Jiaxin Yu <jiaxin.yu@mediatek.com>
 */

unsafe extern "C" {
    pub fn mt8186_mt6366_init(rtd: *mut snd_soc_pcm_runtime) -> core::ffi::c_int;
    pub fn mt8186_mt6366_card_set_be_link(
        dev: *mut device,
        link: *mut snd_soc_dai_link,
        node: *mut device_node,
        link_name: *mut core::ffi::c_char,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
