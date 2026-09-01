// SPDX-License-Identifier: GPL-2.0
/*
 * mtk-soc-card.h  --  MediaTek soc card data definition
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Chunxu Li <chunxu.li@mediatek.com>
 */

// C header guard omitted in Rust.

#[repr(C)]
pub struct mtk_platform_card_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_sof_priv {
    _private: [u8; 0],
}

// External dependency types from included kernel headers:
// struct list_head;
// struct snd_soc_component;

#[repr(C)]
pub struct mtk_soc_card_data {
    pub sof_priv: *const mtk_sof_priv,
    pub sof_dai_link_list: list_head,
    pub card_data: *mut mtk_platform_card_data,
    pub accdet: *mut snd_soc_component,
    pub mach_priv: *mut core::ffi::c_void,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
