/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mtk-afe-fe-dais.h  --  Mediatek afe fe dai operator definition
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Garlic Tseng <garlic.tseng@mediatek.com>
 */

/* Header guard _MTK_AFE_FE_DAI_H_ omitted in Rust. */

#[repr(C)]
pub struct snd_soc_dai_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe_memif {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

pub type dma_addr_t = u64;
pub type size_t = usize;
pub type snd_pcm_format_t = i32;

unsafe extern "C" {
    pub fn mtk_afe_fe_startup(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    ) -> i32;

    pub fn mtk_afe_fe_shutdown(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    );

    pub fn mtk_afe_fe_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        dai: *mut snd_soc_dai,
    ) -> i32;

    pub fn mtk_afe_fe_hw_free(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    ) -> i32;

    pub fn mtk_afe_fe_prepare(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    ) -> i32;

    pub fn mtk_afe_fe_trigger(
        substream: *mut snd_pcm_substream,
        cmd: i32,
        dai: *mut snd_soc_dai,
    ) -> i32;

    pub static mtk_afe_fe_ops: snd_soc_dai_ops;

    pub fn mtk_dynamic_irq_acquire(afe: *mut mtk_base_afe) -> i32;

    pub fn mtk_dynamic_irq_release(
        afe: *mut mtk_base_afe,
        irq_id: i32,
    ) -> i32;

    pub fn mtk_afe_suspend(component: *mut snd_soc_component) -> i32;

    pub fn mtk_afe_resume(component: *mut snd_soc_component) -> i32;

    pub fn mtk_memif_set_enable(
        afe: *mut mtk_base_afe,
        id: i32,
    ) -> i32;

    pub fn mtk_memif_set_disable(
        afe: *mut mtk_base_afe,
        id: i32,
    ) -> i32;

    pub fn mtk_memif_set_addr(
        afe: *mut mtk_base_afe,
        id: i32,
        dma_area: *mut u8,
        dma_addr: dma_addr_t,
        dma_bytes: size_t,
    ) -> i32;

    pub fn mtk_memif_set_channel(
        afe: *mut mtk_base_afe,
        id: i32,
        channel: u32,
    ) -> i32;

    pub fn mtk_memif_set_rate_substream(
        substream: *mut snd_pcm_substream,
        id: i32,
        rate: u32,
    ) -> i32;

    pub fn mtk_memif_set_format(
        afe: *mut mtk_base_afe,
        id: i32,
        format: snd_pcm_format_t,
    ) -> i32;

    pub fn mtk_memif_set_pbuf_size(
        afe: *mut mtk_base_afe,
        id: i32,
        pbuf_size: i32,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
