// SPDX-License-Identifier: GPL-2.0
//
// Renesas R-Car
//
// Copyright (C) 2013 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type phys_addr_t = usize;
pub type snd_pcm_uframes_t = usize;
pub type bool_ = bool;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_chan {
    _private: [u8; 0],
}
#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut c_void,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_constraint_list {
    _private: [u8; 0],
}
#[repr(C)]
pub struct of_phandle_args {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn clk_enable(clk: *mut clk) -> c_int;
    pub fn clk_disable(clk: *mut clk);
    pub fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

pub const RSND_BASE_ADG: c_int = 0;
pub const RSND_BASE_SSI: c_int = 1;
pub const RSND_BASE_SSIU: c_int = 2;
pub const RSND_BASE_SCU: c_int = 3; // for Gen2/Gen3
pub const RSND_BASE_SDMC: c_int = 3; // for Gen4 reuse
pub const RSND_BASE_MAX: c_int = 4;

/*
 *	pseudo register
 *
 * The register address offsets SRU/SCU/SSIU on Gen1/Gen2 are very different.
 * This driver uses pseudo register in order to hide it.
 * see gen1/gen2 for detail
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rsnd_reg {
    SRC_I_BUSIF_MODE,
    SRC_O_BUSIF_MODE,
    SRC_ROUTE_MODE0,
    SRC_SWRSR,
    SRC_SRCIR,
    SRC_ADINR,
    SRC_IFSCR,
    SRC_IFSVR,
    SRC_SRCCR,
    SRC_CTRL,
    SRC_BSDSR,
    SRC_BSISR,
    SRC_INT_ENABLE0,
    SRC_BUSIF_DALIGN,
    SRCIN_TIMSEL0,
    SRCIN_TIMSEL1,
    SRCIN_TIMSEL2,
    SRCIN_TIMSEL3,
    SRCIN_TIMSEL4,
    SRCOUT_TIMSEL0,
    SRCOUT_TIMSEL1,
    SRCOUT_TIMSEL2,
    SRCOUT_TIMSEL3,
    SRCOUT_TIMSEL4,
    SCU_SYS_STATUS0,
    SCU_SYS_STATUS1,
    SCU_SYS_INT_EN0,
    SCU_SYS_INT_EN1,
    CMD_CTRL,
    CMD_BUSIF_MODE,
    CMD_BUSIF_DALIGN,
    CMD_ROUTE_SLCT,
    CMDOUT_TIMSEL,
    CTU_SWRSR,
    CTU_CTUIR,
    CTU_ADINR,
    CTU_CPMDR,
    CTU_SCMDR,
    CTU_SV00R,
    CTU_SV01R,
    CTU_SV02R,
    CTU_SV03R,
    CTU_SV04R,
    CTU_SV05R,
    CTU_SV06R,
    CTU_SV07R,
    CTU_SV10R,
    CTU_SV11R,
    CTU_SV12R,
    CTU_SV13R,
    CTU_SV14R,
    CTU_SV15R,
    CTU_SV16R,
    CTU_SV17R,
    CTU_SV20R,
    CTU_SV21R,
    CTU_SV22R,
    CTU_SV23R,
    CTU_SV24R,
    CTU_SV25R,
    CTU_SV26R,
    CTU_SV27R,
    CTU_SV30R,
    CTU_SV31R,
    CTU_SV32R,
    CTU_SV33R,
    CTU_SV34R,
    CTU_SV35R,
    CTU_SV36R,
    CTU_SV37R,
    MIX_SWRSR,
    MIX_MIXIR,
    MIX_ADINR,
    MIX_MIXMR,
    MIX_MVPDR,
    MIX_MDBAR,
    MIX_MDBBR,
    MIX_MDBCR,
    MIX_MDBDR,
    MIX_MDBER,
    DVC_SWRSR,
    DVC_DVUIR,
    DVC_ADINR,
    DVC_DVUCR,
    DVC_ZCMCR,
    DVC_VOL0R,
    DVC_VOL1R,
    DVC_VOL2R,
    DVC_VOL3R,
    DVC_VOL4R,
    DVC_VOL5R,
    DVC_VOL6R,
    DVC_VOL7R,
    DVC_DVUER,
    DVC_VRCTR,
    DVC_VRPDR,
    DVC_VRDBR,
    BRRA,
    BRRB,
    BRGCKR,
    DIV_EN,
    AUDIO_CLK_SEL0,
    AUDIO_CLK_SEL1,
    AUDIO_CLK_SEL2,
    AUDIO_CLK_SEL3,
    SSI_MODE,
    SSI_MODE0,
    SSI_MODE1,
    SSI_MODE2,
    SSI_MODE3,
    SSI_CONTROL,
    SSI_CONTROL2,
    SSI_CTRL,
    SSI_BUSIF0_MODE,
    SSI_BUSIF1_MODE,
    SSI_BUSIF2_MODE,
    SSI_BUSIF3_MODE,
    SSI_BUSIF4_MODE,
    SSI_BUSIF5_MODE,
    SSI_BUSIF6_MODE,
    SSI_BUSIF7_MODE,
    SSI_BUSIF0_ADINR,
    SSI_BUSIF1_ADINR,
    SSI_BUSIF2_ADINR,
    SSI_BUSIF3_ADINR,
    SSI_BUSIF4_ADINR,
    SSI_BUSIF5_ADINR,
    SSI_BUSIF6_ADINR,
    SSI_BUSIF7_ADINR,
    SSI_BUSIF0_DALIGN,
    SSI_BUSIF1_DALIGN,
    SSI_BUSIF2_DALIGN,
    SSI_BUSIF3_DALIGN,
    SSI_BUSIF4_DALIGN,
    SSI_BUSIF5_DALIGN,
    SSI_BUSIF6_DALIGN,
    SSI_BUSIF7_DALIGN,
    SSI_INT_ENABLE,
    SSI_SYS_STATUS0,
    SSI_SYS_STATUS1,
    SSI_SYS_STATUS2,
    SSI_SYS_STATUS3,
    SSI_SYS_STATUS4,
    SSI_SYS_STATUS5,
    SSI_SYS_STATUS6,
    SSI_SYS_STATUS7,
    SSI_SYS_INT_ENABLE0,
    SSI_SYS_INT_ENABLE1,
    SSI_SYS_INT_ENABLE2,
    SSI_SYS_INT_ENABLE3,
    SSI_SYS_INT_ENABLE4,
    SSI_SYS_INT_ENABLE5,
    SSI_SYS_INT_ENABLE6,
    SSI_SYS_INT_ENABLE7,
    HDMI0_SEL,
    HDMI1_SEL,
    SSI9_BUSIF0_MODE,
    SSI9_BUSIF1_MODE,
    SSI9_BUSIF2_MODE,
    SSI9_BUSIF3_MODE,
    SSI9_BUSIF4_MODE,
    SSI9_BUSIF5_MODE,
    SSI9_BUSIF6_MODE,
    SSI9_BUSIF7_MODE,
    SSI9_BUSIF0_ADINR,
    SSI9_BUSIF1_ADINR,
    SSI9_BUSIF2_ADINR,
    SSI9_BUSIF3_ADINR,
    SSI9_BUSIF4_ADINR,
    SSI9_BUSIF5_ADINR,
    SSI9_BUSIF6_ADINR,
    SSI9_BUSIF7_ADINR,
    SSI9_BUSIF0_DALIGN,
    SSI9_BUSIF1_DALIGN,
    SSI9_BUSIF2_DALIGN,
    SSI9_BUSIF3_DALIGN,
    SSI9_BUSIF4_DALIGN,
    SSI9_BUSIF5_DALIGN,
    SSI9_BUSIF6_DALIGN,
    SSI9_BUSIF7_DALIGN,
    SSICR,
    SSISR,
    SSITDR,
    SSIRDR,
    SSIWSR,
    REG_MAX,
}

const fn reg_add(reg: rsnd_reg, offset: c_int) -> rsnd_reg {
    unsafe { core::mem::transmute::<c_int, rsnd_reg>(reg as c_int + offset) }
}
pub const fn SRCIN_TIMSEL(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::SRCIN_TIMSEL0, i) }
pub const fn SRCOUT_TIMSEL(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::SRCOUT_TIMSEL0, i) }
pub const fn CTU_SVxxR(i: c_int, j: c_int) -> rsnd_reg { reg_add(rsnd_reg::CTU_SV00R, i * 8 + j) }
pub const fn DVC_VOLxR(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::DVC_VOL0R, i) }
pub const fn AUDIO_CLK_SEL(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::AUDIO_CLK_SEL0, i) }
pub const fn SSI_BUSIF_MODE(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::SSI_BUSIF0_MODE, i) }
pub const fn SSI_BUSIF_ADINR(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::SSI_BUSIF0_ADINR, i) }
pub const fn SSI_BUSIF_DALIGN(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::SSI_BUSIF0_DALIGN, i) }
pub const fn SSI9_BUSIF_MODE(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::SSI9_BUSIF0_MODE, i) }
pub const fn SSI9_BUSIF_ADINR(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::SSI9_BUSIF0_ADINR, i) }
pub const fn SSI9_BUSIF_DALIGN(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::SSI9_BUSIF0_DALIGN, i) }
pub const fn SSI_SYS_STATUS(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::SSI_SYS_STATUS0, i) }
pub const fn SSI_SYS_INT_ENABLE(i: c_int) -> rsnd_reg { reg_add(rsnd_reg::SSI_SYS_INT_ENABLE0, i) }

#[repr(C)]
pub struct rsnd_mod_ops {
    pub name: *mut c_char,
    pub dma_req: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod) -> *mut dma_chan>,
    pub probe: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub init: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub quit: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub start: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub stop: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub irq: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv, c_int) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_soc_pcm_runtime) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_pcm_uframes_t) -> c_int>,
    pub fallback: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub cleanup: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut rsnd_priv) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, *mut snd_pcm_substream) -> c_int>,
    pub get_status: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream, rsnd_mod_type) -> *mut u32>,
    pub id: Option<unsafe extern "C" fn(*mut rsnd_mod) -> c_int>,
    pub id_sub: Option<unsafe extern "C" fn(*mut rsnd_mod) -> c_int>,
    pub id_cmd: Option<unsafe extern "C" fn(*mut rsnd_mod) -> c_int>,
    // CONFIG_DEBUG_FS: void (*debug_info)(struct seq_file *m, struct rsnd_dai_stream *io, struct rsnd_mod *mod);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum rsnd_mod_type {
    RSND_MOD_AUDMAPP,
    RSND_MOD_AUDMA,
    RSND_MOD_DVC,
    RSND_MOD_MIX,
    RSND_MOD_CTU,
    RSND_MOD_CMD,
    RSND_MOD_SRC,
    RSND_MOD_SSIM3,
    RSND_MOD_SSIM2,
    RSND_MOD_SSIM1,
    RSND_MOD_SSIP,
    RSND_MOD_SSI,
    RSND_MOD_SSIU,
    RSND_MOD_MAX,
}

#[repr(C)]
pub struct rsnd_mod {
    pub id: c_int,
    pub type_: rsnd_mod_type,
    pub ops: *mut rsnd_mod_ops,
    pub priv_: *mut rsnd_priv,
    pub clk: *mut clk,
    pub rstc: *mut reset_control,
    pub status: u32,
}

pub const __rsnd_mod_shift_init: c_int = 4;
pub const __rsnd_mod_shift_quit: c_int = 4;
pub const __rsnd_mod_shift_start: c_int = 8;
pub const __rsnd_mod_shift_stop: c_int = 8;
pub const __rsnd_mod_shift_hw_params: c_int = 12;
pub const __rsnd_mod_shift_hw_free: c_int = 12;
pub const __rsnd_mod_shift_probe: c_int = 28;
pub const __rsnd_mod_shift_remove: c_int = 28;
pub const __rsnd_mod_shift_irq: c_int = 28;
pub const __rsnd_mod_shift_pcm_new: c_int = 28;
pub const __rsnd_mod_shift_fallback: c_int = 28;
pub const __rsnd_mod_shift_pointer: c_int = 28;
pub const __rsnd_mod_shift_prepare: c_int = 28;
pub const __rsnd_mod_shift_cleanup: c_int = 28;

pub const __rsnd_mod_add_probe: c_int = 0;
pub const __rsnd_mod_add_remove: c_int = 0;
pub const __rsnd_mod_add_prepare: c_int = 0;
pub const __rsnd_mod_add_cleanup: c_int = 0;
pub const __rsnd_mod_add_init: c_int = 1;
pub const __rsnd_mod_add_quit: c_int = -1;
pub const __rsnd_mod_add_start: c_int = 1;
pub const __rsnd_mod_add_stop: c_int = -1;
pub const __rsnd_mod_add_hw_params: c_int = 1;
pub const __rsnd_mod_add_hw_free: c_int = -1;
pub const __rsnd_mod_add_irq: c_int = 0;
pub const __rsnd_mod_add_pcm_new: c_int = 0;
pub const __rsnd_mod_add_fallback: c_int = 0;
pub const __rsnd_mod_add_pointer: c_int = 0;

pub const __rsnd_mod_call_probe: c_int = 0;
pub const __rsnd_mod_call_remove: c_int = 0;
pub const __rsnd_mod_call_prepare: c_int = 0;
pub const __rsnd_mod_call_cleanup: c_int = 0;
pub const __rsnd_mod_call_init: c_int = 0;
pub const __rsnd_mod_call_quit: c_int = 1;
pub const __rsnd_mod_call_start: c_int = 0;
pub const __rsnd_mod_call_stop: c_int = 1;
pub const __rsnd_mod_call_hw_params: c_int = 0;
pub const __rsnd_mod_call_hw_free: c_int = 1;
pub const __rsnd_mod_call_irq: c_int = 0;
pub const __rsnd_mod_call_pcm_new: c_int = 0;
pub const __rsnd_mod_call_fallback: c_int = 0;
pub const __rsnd_mod_call_pointer: c_int = 0;

pub unsafe fn rsnd_mod_to_priv(mod_: *mut rsnd_mod) -> *mut rsnd_priv { unsafe { (*mod_).priv_ } }
pub unsafe fn rsnd_mod_power_on(mod_: *mut rsnd_mod) -> c_int { unsafe { clk_enable((*mod_).clk) } }
pub unsafe fn rsnd_mod_power_off(mod_: *mut rsnd_mod) { unsafe { clk_disable((*mod_).clk) } }
pub unsafe fn rsnd_mod_get(ip: *mut rsnd_mod) -> *mut rsnd_mod { ip }

unsafe extern "C" {
    pub fn rsnd_mod_read(mod_: *mut rsnd_mod, reg: rsnd_reg) -> u32;
    pub fn rsnd_mod_write(mod_: *mut rsnd_mod, reg: rsnd_reg, data: u32);
    pub fn rsnd_mod_bset(mod_: *mut rsnd_mod, reg: rsnd_reg, mask: u32, data: u32);
    pub fn rsnd_get_adinr_bit(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> u32;
    pub fn rsnd_get_dalign(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> u32;
    pub fn rsnd_get_busif_shift(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) -> u32;

    pub fn rsnd_dma_attach(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod, dma_mod: *mut *mut rsnd_mod) -> c_int;
    pub fn rsnd_dma_probe(priv_: *mut rsnd_priv) -> c_int;
    pub fn rsnd_dma_suspend(priv_: *mut rsnd_priv);
    pub fn rsnd_dma_resume(priv_: *mut rsnd_priv);
    pub fn rsnd_dma_request_channel(of_node: *mut device_node, name: *mut c_char, mod_: *mut rsnd_mod, x: *mut c_char) -> *mut dma_chan;

    pub fn rsnd_mod_init(priv_: *mut rsnd_priv, mod_: *mut rsnd_mod, ops: *mut rsnd_mod_ops, clk: *mut clk, rstc: *mut reset_control, type_: rsnd_mod_type, id: c_int) -> c_int;
    pub fn rsnd_mod_quit(mod_: *mut rsnd_mod);
    pub fn rsnd_suspend_clk_reset(clk: *mut clk, rstc: *mut reset_control);
    pub fn rsnd_resume_clk_reset(clk: *mut clk, rstc: *mut reset_control);
    pub fn rsnd_mod_dma_req(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) -> *mut dma_chan;
    pub fn rsnd_mod_interrupt(mod_: *mut rsnd_mod, callback: Option<unsafe extern "C" fn(*mut rsnd_mod, *mut rsnd_dai_stream)>);
    pub fn rsnd_mod_get_status(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, type_: rsnd_mod_type) -> *mut u32;
    pub fn rsnd_mod_id(mod_: *mut rsnd_mod) -> c_int;
    pub fn rsnd_mod_id_raw(mod_: *mut rsnd_mod) -> c_int;
    pub fn rsnd_mod_id_sub(mod_: *mut rsnd_mod) -> c_int;
    pub fn rsnd_mod_name(mod_: *mut rsnd_mod) -> *mut c_char;
    pub fn rsnd_mod_next(iterator: *mut c_int, io: *mut rsnd_dai_stream, array: *mut rsnd_mod_type, array_size: c_int) -> *mut rsnd_mod;

    pub fn rsnd_parse_connect_common(rdai: *mut rsnd_dai, name: *mut c_char, mod_get: Option<unsafe extern "C" fn(*mut rsnd_priv, c_int) -> *mut rsnd_mod>, node: *mut device_node, playback: *mut device_node, capture: *mut device_node);
    pub fn rsnd_node_count(priv_: *mut rsnd_priv, node: *mut device_node, name: *mut c_char) -> c_int;
    pub fn rsnd_node_fixed_index(dev: *mut device, node: *mut device_node, name: *mut c_char, idx: c_int) -> c_int;
    pub fn rsnd_channel_normalization(chan: c_int) -> c_int;
    pub fn rsnd_runtime_channel_original_with_params(io: *mut rsnd_dai_stream, params: *mut snd_pcm_hw_params) -> c_int;
    pub fn rsnd_runtime_channel_after_ctu_with_params(io: *mut rsnd_dai_stream, params: *mut snd_pcm_hw_params) -> c_int;
    pub fn rsnd_runtime_channel_for_ssi_with_params(io: *mut rsnd_dai_stream, params: *mut snd_pcm_hw_params) -> c_int;
    pub fn rsnd_runtime_is_multi_ssi(io: *mut rsnd_dai_stream) -> c_int;
    pub fn rsnd_runtime_is_tdm(io: *mut rsnd_dai_stream) -> c_int;
    pub fn rsnd_runtime_is_tdm_split(io: *mut rsnd_dai_stream) -> c_int;
    pub fn rsnd_devm_clk_get_indexed(dev: *mut device, base: *const c_char, index: c_int) -> *mut clk;
    pub fn rsnd_devm_clk_get_optional_indexed(dev: *mut device, base: *const c_char, index: c_int) -> *mut clk;
    pub fn rsnd_devm_reset_control_get_optional_indexed(dev: *mut device, base: *const c_char, index: c_int) -> *mut reset_control;
    pub fn rsnd_parse_of_node(priv_: *mut rsnd_priv, name: *const c_char) -> *mut device_node;
}

pub unsafe fn rsnd_runtime_channel_original(io: *mut rsnd_dai_stream) -> c_int { unsafe { rsnd_runtime_channel_original_with_params(io, core::ptr::null_mut()) } }
pub unsafe fn rsnd_runtime_channel_after_ctu(io: *mut rsnd_dai_stream) -> c_int { unsafe { rsnd_runtime_channel_after_ctu_with_params(io, core::ptr::null_mut()) } }
pub unsafe fn rsnd_runtime_channel_for_ssi(io: *mut rsnd_dai_stream) -> c_int { unsafe { rsnd_runtime_channel_for_ssi_with_params(io, core::ptr::null_mut()) } }

pub const RSND_NODE_DAI: &[u8] = b"rcar_sound,dai\0";
pub const RSND_NODE_SSI: &[u8] = b"rcar_sound,ssi\0";
pub const RSND_NODE_SSIU: &[u8] = b"rcar_sound,ssiu\0";
pub const RSND_NODE_SRC: &[u8] = b"rcar_sound,src\0";
pub const RSND_NODE_CTU: &[u8] = b"rcar_sound,ctu\0";
pub const RSND_NODE_MIX: &[u8] = b"rcar_sound,mix\0";
pub const RSND_NODE_DVC: &[u8] = b"rcar_sound,dvc\0";

pub const RSND_DAI_NAME_SIZE: usize = 16;
#[repr(C)]
pub struct rsnd_dai_stream {
    pub name: [c_char; RSND_DAI_NAME_SIZE],
    pub substream: *mut snd_pcm_substream,
    pub mod_: [*mut rsnd_mod; rsnd_mod_type::RSND_MOD_MAX as usize],
    pub dma: *mut rsnd_mod,
    pub rdai: *mut rsnd_dai,
    pub dmac_dev: *mut device, /* for IPMMU */
    pub converted_rate: u32,   /* converted sampling rate */
    pub converted_chan: c_int, /* converted channels */
    pub parent_ssi_status: u32,
    pub flags: u32,
}

pub const RSND_STREAM_HDMI0: u32 = 1 << 0;
pub const RSND_STREAM_HDMI1: u32 = 1 << 1;
pub const RSND_STREAM_TDM_SPLIT: u32 = 1 << 2;
pub const RSND_HW_RULE_ERR: u32 = 1 << 3;

pub unsafe fn rsnd_io_to_mod(io: *mut rsnd_dai_stream, i: usize) -> *mut rsnd_mod {
    if i < rsnd_mod_type::RSND_MOD_MAX as usize { unsafe { (*io).mod_[i] } } else { core::ptr::null_mut() }
}
pub unsafe fn rsnd_io_to_mod_ssi(io: *mut rsnd_dai_stream) -> *mut rsnd_mod { unsafe { rsnd_io_to_mod(io, rsnd_mod_type::RSND_MOD_SSI as usize) } }
pub unsafe fn rsnd_io_to_mod_ssiu(io: *mut rsnd_dai_stream) -> *mut rsnd_mod { unsafe { rsnd_io_to_mod(io, rsnd_mod_type::RSND_MOD_SSIU as usize) } }
pub unsafe fn rsnd_io_to_mod_ssip(io: *mut rsnd_dai_stream) -> *mut rsnd_mod { unsafe { rsnd_io_to_mod(io, rsnd_mod_type::RSND_MOD_SSIP as usize) } }
pub unsafe fn rsnd_io_to_mod_src(io: *mut rsnd_dai_stream) -> *mut rsnd_mod { unsafe { rsnd_io_to_mod(io, rsnd_mod_type::RSND_MOD_SRC as usize) } }
pub unsafe fn rsnd_io_to_mod_ctu(io: *mut rsnd_dai_stream) -> *mut rsnd_mod { unsafe { rsnd_io_to_mod(io, rsnd_mod_type::RSND_MOD_CTU as usize) } }
pub unsafe fn rsnd_io_to_mod_mix(io: *mut rsnd_dai_stream) -> *mut rsnd_mod { unsafe { rsnd_io_to_mod(io, rsnd_mod_type::RSND_MOD_MIX as usize) } }
pub unsafe fn rsnd_io_to_mod_dvc(io: *mut rsnd_dai_stream) -> *mut rsnd_mod { unsafe { rsnd_io_to_mod(io, rsnd_mod_type::RSND_MOD_DVC as usize) } }
pub unsafe fn rsnd_io_to_mod_cmd(io: *mut rsnd_dai_stream) -> *mut rsnd_mod { unsafe { rsnd_io_to_mod(io, rsnd_mod_type::RSND_MOD_CMD as usize) } }
pub unsafe fn rsnd_io_to_rdai(io: *mut rsnd_dai_stream) -> *mut rsnd_dai { unsafe { (*io).rdai } }
pub unsafe fn rsnd_io_to_priv(io: *mut rsnd_dai_stream) -> *mut rsnd_priv { unsafe { rsnd_rdai_to_priv(rsnd_io_to_rdai(io)) } }
pub unsafe fn rsnd_io_is_play(io: *mut rsnd_dai_stream) -> bool { unsafe { core::ptr::addr_of_mut!((*rsnd_io_to_rdai(io)).playback) == io } }
pub unsafe fn rsnd_io_to_runtime(io: *mut rsnd_dai_stream) -> *mut c_void { unsafe { if !(*io).substream.is_null() { (*(*io).substream).runtime } else { core::ptr::null_mut() } } }
pub unsafe fn rsnd_io_converted_rate(io: *mut rsnd_dai_stream) -> u32 { unsafe { (*io).converted_rate } }
pub unsafe fn rsnd_io_converted_chan(io: *mut rsnd_dai_stream) -> c_int { unsafe { (*io).converted_chan } }

unsafe extern "C" {
    pub fn rsnd_io_is_working(io: *mut rsnd_dai_stream) -> c_int;
}

#[repr(C)]
pub struct rsnd_dai {
    pub name: [c_char; RSND_DAI_NAME_SIZE],
    pub playback: rsnd_dai_stream,
    pub capture: rsnd_dai_stream,
    pub priv_: *mut rsnd_priv,
    pub constraint: snd_pcm_hw_constraint_list,
    pub dai_args: of_phandle_args,
    pub max_channels: c_int, /* 2ch - 16ch */
    pub ssi_lane: c_int,     /* 1lane - 4lane */
    pub chan_width: c_int,   /* 16/24/32 bit width */
    // C bitfields packed in unsigned int:
    // unsigned int clk_master:1, bit_clk_inv:1, frm_clk_inv:1, sys_delay:1, data_alignment:1;
    pub bitfield_flags: c_uint,
}

pub unsafe fn rsnd_rdai_nr(priv_: *mut rsnd_priv) -> c_int { unsafe { (*priv_).rdai_nr } }
pub unsafe fn rsnd_rdai_is_clk_master(rdai: *mut rsnd_dai) -> c_uint { unsafe { (*rdai).bitfield_flags & 1 } }
pub unsafe fn rsnd_rdai_to_priv(rdai: *mut rsnd_dai) -> *mut rsnd_priv { unsafe { (*rdai).priv_ } }

unsafe extern "C" {
    pub fn rsnd_rdai_get(priv_: *mut rsnd_priv, id: c_int) -> *mut rsnd_dai;
    pub fn rsnd_rdai_channels_ctrl(rdai: *mut rsnd_dai, max_channels: c_int) -> c_int;
    pub fn rsnd_rdai_ssi_lane_ctrl(rdai: *mut rsnd_dai, ssi_lane: c_int) -> c_int;
    pub fn rsnd_rdai_width_ctrl(rdai: *mut rsnd_dai, width: c_int) -> c_int;
    pub fn rsnd_dai_connect(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, type_: rsnd_mod_type) -> c_int;
    pub fn rsnd_gen_probe(priv_: *mut rsnd_priv) -> c_int;
    pub fn rsnd_gen_reg_get(priv_: *mut rsnd_priv, mod_: *mut rsnd_mod, reg: rsnd_reg) -> *mut c_void;
    pub fn rsnd_gen_get_phy_addr(priv_: *mut rsnd_priv, reg_id: c_int) -> phys_addr_t;
    pub fn rsnd_gen_get_base_addr(priv_: *mut rsnd_priv, reg_id: c_int) -> *mut c_void;
    pub fn rsnd_adg_clk_query(priv_: *mut rsnd_priv, rate: c_uint) -> c_int;
    pub fn rsnd_adg_ssi_clk_stop(ssi_mod: *mut rsnd_mod) -> c_int;
    pub fn rsnd_adg_ssi_clk_try_start(ssi_mod: *mut rsnd_mod, rate: c_uint) -> c_int;
    pub fn rsnd_adg_probe(priv_: *mut rsnd_priv) -> c_int;
    pub fn rsnd_adg_remove(priv_: *mut rsnd_priv);
    pub fn rsnd_adg_suspend(priv_: *mut rsnd_priv);
    pub fn rsnd_adg_resume(priv_: *mut rsnd_priv);
    pub fn rsnd_adg_set_src_timesel_gen2(src_mod: *mut rsnd_mod, io: *mut rsnd_dai_stream, in_rate: c_uint, out_rate: c_uint) -> c_int;
    pub fn rsnd_adg_set_cmd_timsel_gen2(cmd_mod: *mut rsnd_mod, io: *mut rsnd_dai_stream) -> c_int;
    pub fn rsnd_adg_clk_control(priv_: *mut rsnd_priv, enable: c_int) -> c_int;
    pub fn rsnd_adg_clk_dbg_info(priv_: *mut rsnd_priv, m: *mut seq_file);
}

pub unsafe fn rsnd_rdai_channels_set(rdai: *mut rsnd_dai, max_channels: c_int) -> c_int { unsafe { rsnd_rdai_channels_ctrl(rdai, max_channels) } }
pub unsafe fn rsnd_rdai_channels_get(rdai: *mut rsnd_dai) -> c_int { unsafe { rsnd_rdai_channels_ctrl(rdai, 0) } }
pub unsafe fn rsnd_rdai_ssi_lane_set(rdai: *mut rsnd_dai, ssi_lane: c_int) -> c_int { unsafe { rsnd_rdai_ssi_lane_ctrl(rdai, ssi_lane) } }
pub unsafe fn rsnd_rdai_ssi_lane_get(rdai: *mut rsnd_dai) -> c_int { unsafe { rsnd_rdai_ssi_lane_ctrl(rdai, 0) } }
pub unsafe fn rsnd_rdai_width_set(rdai: *mut rsnd_dai, width: c_int) -> c_int { unsafe { rsnd_rdai_width_ctrl(rdai, width) } }
pub unsafe fn rsnd_rdai_width_get(rdai: *mut rsnd_dai) -> c_int { unsafe { rsnd_rdai_width_ctrl(rdai, 0) } }
pub unsafe fn rsnd_adg_clk_enable(priv_: *mut rsnd_priv) -> c_int { unsafe { rsnd_adg_clk_control(priv_, 1) } }
pub unsafe fn rsnd_adg_clk_disable(priv_: *mut rsnd_priv) -> c_int { unsafe { rsnd_adg_clk_control(priv_, 0) } }

#[repr(C)]
pub struct rsnd_priv {
    pub pdev: *mut platform_device,
    pub lock: spinlock_t,
    pub flags: c_ulong,
    pub gen: *mut c_void,
    pub adg: *mut c_void,
    pub dma: *mut c_void,
    pub ssi: *mut c_void,
    pub ssi_nr: c_int,
    pub ssiu_ctrl: *mut c_void,
    pub ssiu: *mut c_void,
    pub ssiu_nr: c_int,
    pub src_ctrl: *mut c_void,
    pub src: *mut c_void,
    pub src_nr: c_int,
    pub ctu: *mut c_void,
    pub ctu_nr: c_int,
    pub mix: *mut c_void,
    pub mix_nr: c_int,
    pub dvc: *mut c_void,
    pub dvc_nr: c_int,
    pub cmd: *mut c_void,
    pub cmd_nr: c_int,
    pub daidrv: *mut snd_soc_dai_driver,
    pub rdai: *mut rsnd_dai,
    pub rdai_nr: c_int,
    pub component_dais: [c_int; RSND_MAX_COMPONENT],
}

pub const RSND_GEN_MASK: c_ulong = 0xF << 0;
pub const RSND_GEN1: c_ulong = 1 << 0;
pub const RSND_GEN2: c_ulong = 2 << 0;
pub const RSND_GEN3: c_ulong = 3 << 0;
pub const RSND_GEN4: c_ulong = 4 << 0;
pub const RSND_SOC_MASK: c_ulong = 0xF << 4;
pub const RSND_SOC_E: c_ulong = 1 << 4;
pub const RSND_RZ_MASK: c_ulong = 0xF << 8;
pub const RSND_RZ3: c_ulong = 3 << 8;
pub const RSND_RZ_ID_MASK: c_ulong = 0xF << 12;
pub const RSND_RZG3E: c_ulong = 1 << 12;
pub const RSND_SSIU_BUSIF_STATUS_COUNT_2: c_ulong = 1 << 16;
pub const RSND_MAX_COMPONENT: usize = 3;

pub unsafe fn rsnd_priv_to_pdev(priv_: *mut rsnd_priv) -> *mut platform_device { unsafe { (*priv_).pdev } }
pub unsafe fn rsnd_priv_to_dev(priv_: *mut rsnd_priv) -> *mut device { unsafe { core::ptr::addr_of_mut!((*rsnd_priv_to_pdev(priv_)).dev) } }
pub unsafe fn rsnd_is_gen1(priv_: *mut rsnd_priv) -> bool { unsafe { ((*priv_).flags & RSND_GEN_MASK) == RSND_GEN1 } }
pub unsafe fn rsnd_is_gen2(priv_: *mut rsnd_priv) -> bool { unsafe { ((*priv_).flags & RSND_GEN_MASK) == RSND_GEN2 } }
pub unsafe fn rsnd_is_gen3(priv_: *mut rsnd_priv) -> bool { unsafe { ((*priv_).flags & RSND_GEN_MASK) == RSND_GEN3 } }
pub unsafe fn rsnd_is_gen4(priv_: *mut rsnd_priv) -> bool { unsafe { ((*priv_).flags & RSND_GEN_MASK) == RSND_GEN4 } }
pub unsafe fn rsnd_is_gen3_e3(priv_: *mut rsnd_priv) -> bool { unsafe { ((*priv_).flags & (RSND_GEN_MASK | RSND_SOC_MASK)) == (RSND_GEN3 | RSND_SOC_E) } }
pub unsafe fn rsnd_is_rzg3e(priv_: *mut rsnd_priv) -> bool { unsafe { ((*priv_).flags & (RSND_RZ_MASK | RSND_RZ_ID_MASK)) == (RSND_RZ3 | RSND_RZG3E) } }
pub unsafe fn rsnd_flags_has(p: *mut rsnd_priv, f: c_ulong) -> c_ulong { unsafe { (*p).flags & f } }
pub unsafe fn rsnd_flags_set(p: *mut rsnd_priv, f: c_ulong) { unsafe { (*p).flags |= f } }
pub unsafe fn rsnd_flags_del(p: *mut rsnd_priv, f: c_ulong) { unsafe { (*p).flags &= !f } }

#[repr(C)]
pub struct rsnd_kctrl_cfg {
    pub max: c_uint,
    pub size: c_uint,
    pub val: *mut u32,
    pub texts: *const *const c_char,
    pub accept: Option<unsafe extern "C" fn(*mut rsnd_dai_stream) -> c_int>,
    pub update: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod)>,
    pub io: *mut rsnd_dai_stream,
    pub card: *mut snd_card,
    pub kctrl: *mut snd_kcontrol,
    pub mod_: *mut rsnd_mod,
}

pub const RSND_MAX_CHANNELS: usize = 8;
#[repr(C)]
pub struct rsnd_kctrl_cfg_m {
    pub cfg: rsnd_kctrl_cfg,
    pub val: [u32; RSND_MAX_CHANNELS],
}
#[repr(C)]
pub struct rsnd_kctrl_cfg_s {
    pub cfg: rsnd_kctrl_cfg,
    pub val: u32,
}
pub fn rsnd_kctrl_size(x: &rsnd_kctrl_cfg_m) -> c_uint { x.cfg.size }
pub fn rsnd_kctrl_max(x: &rsnd_kctrl_cfg_m) -> c_uint { x.cfg.max }
pub fn rsnd_kctrl_valm(x: &rsnd_kctrl_cfg_m, i: usize) -> u32 { x.val[i] }
pub fn rsnd_kctrl_vals(x: &rsnd_kctrl_cfg_s) -> u32 { x.val }

unsafe extern "C" {
    pub fn rsnd_kctrl_accept_anytime(io: *mut rsnd_dai_stream) -> c_int;
    pub fn rsnd_kctrl_init_m(cfg: *mut rsnd_kctrl_cfg_m) -> *mut rsnd_kctrl_cfg;
    pub fn rsnd_kctrl_init_s(cfg: *mut rsnd_kctrl_cfg_s) -> *mut rsnd_kctrl_cfg;
    pub fn rsnd_kctrl_new(mod_: *mut rsnd_mod, io: *mut rsnd_dai_stream, rtd: *mut snd_soc_pcm_runtime, name: *const u8, accept: Option<unsafe extern "C" fn(*mut rsnd_dai_stream) -> c_int>, update: Option<unsafe extern "C" fn(*mut rsnd_dai_stream, *mut rsnd_mod)>, cfg: *mut rsnd_kctrl_cfg, texts: *const *const c_char, size: c_int, max: u32) -> c_int;
    pub static volume_ramp_rate: *const *const c_char;
}

pub const VOLUME_RAMP_MAX_DVC: c_int = 0x17 + 1;
pub const VOLUME_RAMP_MAX_MIX: c_int = 0x0a + 1;

unsafe extern "C" {
    pub fn rsnd_ssi_probe(priv_: *mut rsnd_priv) -> c_int;
    pub fn rsnd_ssi_remove(priv_: *mut rsnd_priv);
    pub fn rsnd_ssi_suspend(priv_: *mut rsnd_priv);
    pub fn rsnd_ssi_resume(priv_: *mut rsnd_priv);
    pub fn rsnd_ssi_mod_get(priv_: *mut rsnd_priv, id: c_int) -> *mut rsnd_mod;
    pub fn rsnd_ssi_use_busif(io: *mut rsnd_dai_stream) -> c_int;
    pub fn rsnd_ssi_multi_secondaries_runtime(io: *mut rsnd_dai_stream) -> u32;
    pub fn rsnd_ssi_is_dma_mode(mod_: *mut rsnd_mod) -> c_int;
    pub fn __rsnd_ssi_is_pin_sharing(mod_: *mut rsnd_mod) -> c_int;
    pub fn rsnd_parse_connect_ssi(rdai: *mut rsnd_dai, playback: *mut device_node, capture: *mut device_node);
    pub fn rsnd_ssi_clk_query(rdai: *mut rsnd_dai, param1: c_int, param2: c_int, idx: *mut c_int) -> c_uint;
    pub fn rsnd_ssiu_attach(io: *mut rsnd_dai_stream, mod_: *mut rsnd_mod) -> c_int;
    pub fn rsnd_ssiu_probe(priv_: *mut rsnd_priv) -> c_int;
    pub fn rsnd_ssiu_remove(priv_: *mut rsnd_priv);
    pub fn rsnd_ssiu_suspend(priv_: *mut rsnd_priv);
    pub fn rsnd_ssiu_resume(priv_: *mut rsnd_priv);
    pub fn rsnd_parse_connect_ssiu(rdai: *mut rsnd_dai, playback: *mut device_node, capture: *mut device_node);
    pub fn rsnd_ssiu_busif_err_status_clear(mod_: *mut rsnd_mod) -> bool;
    pub fn rsnd_src_probe(priv_: *mut rsnd_priv) -> c_int;
    pub fn rsnd_src_remove(priv_: *mut rsnd_priv);
    pub fn rsnd_src_suspend(priv_: *mut rsnd_priv);
    pub fn rsnd_src_resume(priv_: *mut rsnd_priv);
    pub fn rsnd_src_mod_get(priv_: *mut rsnd_priv, id: c_int) -> *mut rsnd_mod;
    pub fn rsnd_src_get_rate(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream, is_in: c_int) -> c_uint;
    pub fn rsnd_ctu_probe(priv_: *mut rsnd_priv) -> c_int;
    pub fn rsnd_ctu_remove(priv_: *mut rsnd_priv);
    pub fn rsnd_ctu_suspend(priv_: *mut rsnd_priv);
    pub fn rsnd_ctu_resume(priv_: *mut rsnd_priv);
    pub fn rsnd_ctu_mod_get(priv_: *mut rsnd_priv, id: c_int) -> *mut rsnd_mod;
    pub fn rsnd_mix_probe(priv_: *mut rsnd_priv) -> c_int;
    pub fn rsnd_mix_remove(priv_: *mut rsnd_priv);
    pub fn rsnd_mix_suspend(priv_: *mut rsnd_priv);
    pub fn rsnd_mix_resume(priv_: *mut rsnd_priv);
    pub fn rsnd_mix_mod_get(priv_: *mut rsnd_priv, id: c_int) -> *mut rsnd_mod;
    pub fn rsnd_dvc_probe(priv_: *mut rsnd_priv) -> c_int;
    pub fn rsnd_dvc_remove(priv_: *mut rsnd_priv);
    pub fn rsnd_dvc_suspend(priv_: *mut rsnd_priv);
    pub fn rsnd_dvc_resume(priv_: *mut rsnd_priv);
    pub fn rsnd_dvc_mod_get(priv_: *mut rsnd_priv, id: c_int) -> *mut rsnd_mod;
    pub fn rsnd_cmd_probe(priv_: *mut rsnd_priv) -> c_int;
    pub fn rsnd_cmd_remove(priv_: *mut rsnd_priv);
    pub fn rsnd_cmd_attach(io: *mut rsnd_dai_stream, id: c_int) -> c_int;
    pub fn rsnd_mod_make_sure(mod_: *mut rsnd_mod, type_: rsnd_mod_type);
    pub fn rsnd_debugfs_probe(component: *mut snd_soc_component) -> c_int;
    pub fn rsnd_debugfs_reg_show(m: *mut seq_file, _addr: phys_addr_t, base: *mut c_void, offset: c_int, size: c_int);
    pub fn rsnd_debugfs_mod_reg_show(m: *mut seq_file, mod_: *mut rsnd_mod, reg_id: c_int, offset: c_int, size: c_int);
}

pub unsafe fn rsnd_ssi_is_pin_sharing(io: *mut rsnd_dai_stream) -> c_int { unsafe { __rsnd_ssi_is_pin_sharing(rsnd_io_to_mod_ssi(io)) } }
pub unsafe fn rsnd_ssi_of_node(priv_: *mut rsnd_priv) -> *mut device_node { unsafe { rsnd_parse_of_node(priv_, RSND_NODE_SSI.as_ptr() as *const c_char) } }
pub unsafe fn rsnd_ssiu_of_node(priv_: *mut rsnd_priv) -> *mut device_node { unsafe { rsnd_parse_of_node(priv_, RSND_NODE_SSIU.as_ptr() as *const c_char) } }
pub unsafe fn rsnd_src_get_in_rate(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream) -> c_uint { unsafe { rsnd_src_get_rate(priv_, io, 1) } }
pub unsafe fn rsnd_src_get_out_rate(priv_: *mut rsnd_priv, io: *mut rsnd_dai_stream) -> c_uint { unsafe { rsnd_src_get_rate(priv_, io, 0) } }
pub unsafe fn rsnd_src_of_node(priv_: *mut rsnd_priv) -> *mut device_node { unsafe { rsnd_parse_of_node(priv_, RSND_NODE_SRC.as_ptr() as *const c_char) } }
pub unsafe fn rsnd_ctu_of_node(priv_: *mut rsnd_priv) -> *mut device_node { unsafe { rsnd_parse_of_node(priv_, RSND_NODE_CTU.as_ptr() as *const c_char) } }
pub unsafe fn rsnd_mix_of_node(priv_: *mut rsnd_priv) -> *mut device_node { unsafe { rsnd_parse_of_node(priv_, RSND_NODE_MIX.as_ptr() as *const c_char) } }
pub unsafe fn rsnd_dvc_of_node(priv_: *mut rsnd_priv) -> *mut device_node { unsafe { rsnd_parse_of_node(priv_, RSND_NODE_DVC.as_ptr() as *const c_char) } }

// rsnd_parse_connect_src/ctu/mix/dvc are C macros expanding to rsnd_parse_connect_common()
// with the corresponding module getter and OF node helper.
// rsnd_print_irq_status(dev, param...) calls dev_info() unless RSND_DEBUG_NO_IRQ_STATUS is built in.
// CONFIG_DEBUG_FS disabled C branch defines rsnd_debugfs_probe as NULL.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
