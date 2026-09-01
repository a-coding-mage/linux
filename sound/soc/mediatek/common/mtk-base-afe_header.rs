/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mtk-base-afe.h  --  Mediatek base afe structure
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Garlic Tseng <garlic.tseng@mediatek.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

/* Depends on linux/soc/mediatek/mtk_sip_svc.h. */
pub const MTK_STREAM_NUM: c_uint = SNDRV_PCM_STREAM_LAST + 1;
pub const MTK_SIP_AUDIO_CONTROL: c_uint = MTK_SIP_SMC_CMD(0x517);

/* SMC CALL Operations */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mtk_audio_smc_call_op {
    MTK_AUDIO_SMC_OP_INIT = 0,
    MTK_AUDIO_SMC_OP_DRAM_REQUEST,
    MTK_AUDIO_SMC_OP_DRAM_RELEASE,
    MTK_AUDIO_SMC_OP_SRAM_REQUEST,
    MTK_AUDIO_SMC_OP_SRAM_RELEASE,
    MTK_AUDIO_SMC_OP_ADSP_REQUEST,
    MTK_AUDIO_SMC_OP_ADSP_RELEASE,
    MTK_AUDIO_SMC_OP_DOMAIN_SIDEBANDS,
    MTK_AUDIO_SMC_OP_BTCVSD_WRITE,
    MTK_AUDIO_SMC_OP_BTCVSD_UPDATE_CTRL_CLEAR,
    MTK_AUDIO_SMC_OP_BTCVSD_UPDATE_CTRL_UNDERFLOW,
    MTK_AUDIO_SMC_OP_NUM,
}

#[repr(C)]
pub struct mtk_base_memif_data {
    pub id: c_int,
    pub name: *const c_char,
    pub reg_ofs_base: c_int,
    pub reg_ofs_cur: c_int,
    pub reg_ofs_end: c_int,
    pub reg_ofs_base_msb: c_int,
    pub reg_ofs_cur_msb: c_int,
    pub reg_ofs_end_msb: c_int,
    pub fs_reg: c_int,
    pub fs_shift: c_int,
    pub fs_maskbit: c_int,
    pub mono_reg: c_int,
    pub mono_shift: c_int,
    pub mono_invert: c_int,
    pub quad_ch_reg: c_int,
    pub quad_ch_mask: c_int,
    pub quad_ch_shift: c_int,
    pub int_odd_flag_reg: c_int,
    pub int_odd_flag_shift: c_int,
    pub enable_reg: c_int,
    pub enable_shift: c_int,
    pub hd_reg: c_int,
    pub hd_shift: c_int,
    pub hd_align_reg: c_int,
    pub hd_align_mshift: c_int,
    pub msb_reg: c_int,
    pub msb_shift: c_int,
    pub msb_end_reg: c_int,
    pub msb_end_shift: c_int,
    pub agent_disable_reg: c_int,
    pub agent_disable_shift: c_int,
    pub ch_num_reg: c_int,
    pub ch_num_shift: c_int,
    pub ch_num_maskbit: c_int,
    /* playback memif only */
    pub pbuf_reg: c_int,
    pub pbuf_mask: c_int,
    pub pbuf_shift: c_int,
    pub minlen_reg: c_int,
    pub minlen_mask: c_int,
    pub minlen_shift: c_int,
}

#[repr(C)]
pub struct mtk_base_irq_data {
    pub id: c_int,
    pub irq_cnt_reg: c_int,
    pub irq_cnt_shift: c_int,
    pub irq_cnt_maskbit: c_int,
    pub irq_fs_reg: c_int,
    pub irq_fs_shift: c_int,
    pub irq_fs_maskbit: c_int,
    pub irq_en_reg: c_int,
    pub irq_en_shift: c_int,
    pub irq_clr_reg: c_int,
    pub irq_clr_shift: c_int,
    pub irq_status_shift: c_int,
}

pub enum device {}
pub enum list_head {}
pub enum regmap {}
pub enum snd_pcm_substream {}
pub enum snd_soc_dai {}
pub enum snd_soc_dai_driver {}
pub enum snd_pcm_hardware {}
pub enum snd_kcontrol_new {}
pub enum snd_soc_dapm_widget {}
pub enum snd_soc_dapm_route {}
pub enum mutex {}

#[repr(C)]
pub struct mtk_base_afe {
    pub base_addr: *mut c_void,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq_alloc_lock: mutex, /* dynamic alloc irq lock */

    pub reg_back_up_list: *const c_uint,
    pub reg_back_up: *mut c_uint,
    pub reg_back_up_list_num: c_uint,

    pub runtime_suspend: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub suspended: bool,

    pub memif: *mut mtk_base_afe_memif,
    pub memif_size: c_int,
    pub irqs: *mut mtk_base_afe_irq,
    pub irqs_size: c_int,
    pub memif_32bit_supported: c_int,
    pub preallocate_buffers: bool,

    pub sub_dais: list_head,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_uint,

    pub mtk_afe_hardware: *const snd_pcm_hardware,
    pub memif_fs: Option<
        unsafe extern "C" fn(substream: *mut snd_pcm_substream, rate: c_uint) -> c_int,
    >,
    pub irq_fs: Option<
        unsafe extern "C" fn(substream: *mut snd_pcm_substream, rate: c_uint) -> c_int,
    >,
    pub get_dai_fs: Option<
        unsafe extern "C" fn(afe: *mut mtk_base_afe, dai_id: c_int, rate: c_uint) -> c_int,
    >,
    pub get_memif_pbuf_size:
        Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> c_int>,

    pub request_dram_resource: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,
    pub release_dram_resource: Option<unsafe extern "C" fn(dev: *mut device) -> c_int>,

    pub platform_priv: *mut c_void,
}

#[repr(C)]
pub struct mtk_base_afe_memif {
    pub phys_buf_addr: c_uint,
    pub buffer_size: c_int,
    pub substream: *mut snd_pcm_substream,
    pub data: *const mtk_base_memif_data,
    pub irq_usage: c_int,
    pub const_irq: c_int,
    pub dma_area: *mut u8,
    pub dma_addr: dma_addr_t,
    pub dma_bytes: size_t,
}

#[repr(C)]
pub struct mtk_base_afe_irq {
    pub irq_data: *const mtk_base_irq_data,
    pub irq_occupyed: c_int,
}

#[repr(C)]
pub struct mtk_base_afe_dai {
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_uint,

    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,

    pub list: list_head,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
