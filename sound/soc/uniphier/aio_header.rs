// SPDX-License-Identifier: GPL-2.0
//
// Socionext UniPhier AIO ALSA driver.
//
// Copyright (c) 2016-2018 Socionext Inc.

// Corresponds to included headers:
// #include <linux/spinlock.h>
// #include <linux/types.h>
// #include <sound/pcm.h>
// #include <sound/soc.h>
// #include <sound/soc-dai.h>

use core::ffi::{c_char, c_int};

// External opaque types from included headers
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

pub struct snd_pcm_substream;
pub struct snd_pcm_hw_params;
pub struct snd_compr_stream;
pub struct snd_compr_params;
pub struct snd_soc_dai;
pub struct snd_soc_dai_driver;
pub struct snd_soc_dai_ops;
pub struct snd_compress_ops;
pub struct platform_device;
pub struct clk;
pub struct reset_control;
pub struct regmap;

pub type dma_addr_t = u64;

#[repr(C)]
pub enum ID_PORT_TYPE {
    PORT_TYPE_UNKNOWN = 0,
    PORT_TYPE_I2S = 1,
    PORT_TYPE_SPDIF = 2,
    PORT_TYPE_EVE = 3,
    PORT_TYPE_CONV = 4,
}

#[repr(C)]
pub enum ID_PORT_DIR {
    PORT_DIR_OUTPUT = 0,
    PORT_DIR_INPUT = 1,
}

#[repr(C)]
pub enum IEC61937_PC {
    IEC61937_PC_AC3 = 0x0001,
    IEC61937_PC_PAUSE = 0x0003,
    IEC61937_PC_MPA = 0x0004,
    IEC61937_PC_MP3 = 0x0005,
    IEC61937_PC_DTS1 = 0x000b,
    IEC61937_PC_DTS2 = 0x000c,
    IEC61937_PC_DTS3 = 0x000d,
    IEC61937_PC_AAC = 0x0007,
}

// IEC61937 Repetition period of data-burst in IEC60958 frames
pub const IEC61937_FRM_STR_AC3: u32 = 1536;
pub const IEC61937_FRM_STR_MPA: u32 = 1152;
pub const IEC61937_FRM_STR_MP3: u32 = 1152;
pub const IEC61937_FRM_STR_DTS1: u32 = 512;
pub const IEC61937_FRM_STR_DTS2: u32 = 1024;
pub const IEC61937_FRM_STR_DTS3: u32 = 2048;
pub const IEC61937_FRM_STR_AAC: u32 = 1024;

// IEC61937 Repetition period of Pause data-burst in IEC60958 frames
pub const IEC61937_FRM_PAU_AC3: u32 = 3;
pub const IEC61937_FRM_PAU_MPA: u32 = 32;
pub const IEC61937_FRM_PAU_MP3: u32 = 32;
pub const IEC61937_FRM_PAU_DTS1: u32 = 3;
pub const IEC61937_FRM_PAU_DTS2: u32 = 3;
pub const IEC61937_FRM_PAU_DTS3: u32 = 3;
pub const IEC61937_FRM_PAU_AAC: u32 = 32;

// IEC61937 Pa and Pb
pub const IEC61937_HEADER_SIGN: u32 = 0x1f4e72f8;

pub const AUD_HW_PCMIN1: c_int = 0;
pub const AUD_HW_PCMIN2: c_int = 1;
pub const AUD_HW_PCMIN3: c_int = 2;
pub const AUD_HW_IECIN1: c_int = 3;
pub const AUD_HW_DIECIN1: c_int = 4;

pub const AUD_NAME_PCMIN1: &[u8] = b"aio-pcmin1\0";
pub const AUD_NAME_PCMIN2: &[u8] = b"aio-pcmin2\0";
pub const AUD_NAME_PCMIN3: &[u8] = b"aio-pcmin3\0";
pub const AUD_NAME_IECIN1: &[u8] = b"aio-iecin1\0";
pub const AUD_NAME_DIECIN1: &[u8] = b"aio-diecin1\0";

pub const AUD_HW_HPCMOUT1: c_int = 0;
pub const AUD_HW_PCMOUT1: c_int = 1;
pub const AUD_HW_PCMOUT2: c_int = 2;
pub const AUD_HW_PCMOUT3: c_int = 3;
pub const AUD_HW_EPCMOUT1: c_int = 4;
pub const AUD_HW_EPCMOUT2: c_int = 5;
pub const AUD_HW_EPCMOUT3: c_int = 6;
pub const AUD_HW_EPCMOUT6: c_int = 9;
pub const AUD_HW_HIECOUT1: c_int = 10;
pub const AUD_HW_IECOUT1: c_int = 11;
pub const AUD_HW_CMASTER: c_int = 31;

pub const AUD_NAME_HPCMOUT1: &[u8] = b"aio-hpcmout1\0";
pub const AUD_NAME_PCMOUT1: &[u8] = b"aio-pcmout1\0";
pub const AUD_NAME_PCMOUT2: &[u8] = b"aio-pcmout2\0";
pub const AUD_NAME_PCMOUT3: &[u8] = b"aio-pcmout3\0";
pub const AUD_NAME_EPCMOUT1: &[u8] = b"aio-epcmout1\0";
pub const AUD_NAME_EPCMOUT2: &[u8] = b"aio-epcmout2\0";
pub const AUD_NAME_EPCMOUT3: &[u8] = b"aio-epcmout3\0";
pub const AUD_NAME_EPCMOUT6: &[u8] = b"aio-epcmout6\0";
pub const AUD_NAME_HIECOUT1: &[u8] = b"aio-hiecout1\0";
pub const AUD_NAME_IECOUT1: &[u8] = b"aio-iecout1\0";
pub const AUD_NAME_CMASTER: &[u8] = b"aio-cmaster\0";
pub const AUD_NAME_HIECCOMPOUT1: &[u8] = b"aio-hieccompout1\0";
pub const AUD_NAME_IECCOMPOUT1: &[u8] = b"aio-ieccompout1\0";

pub const AUD_GNAME_HDMI: &[u8] = b"aio-hdmi\0";
pub const AUD_GNAME_LINE: &[u8] = b"aio-line\0";
pub const AUD_GNAME_AUX: &[u8] = b"aio-aux\0";
pub const AUD_GNAME_IEC: &[u8] = b"aio-iec\0";

pub const AUD_CLK_IO: c_int = 0;
pub const AUD_CLK_A1: c_int = 1;
pub const AUD_CLK_F1: c_int = 2;
pub const AUD_CLK_A2: c_int = 3;
pub const AUD_CLK_F2: c_int = 4;
pub const AUD_CLK_A: c_int = 5;
pub const AUD_CLK_F: c_int = 6;
pub const AUD_CLK_APLL: c_int = 7;
pub const AUD_CLK_RX0: c_int = 8;
pub const AUD_CLK_USB0: c_int = 9;
pub const AUD_CLK_HSC0: c_int = 10;

pub const AUD_PLL_A1: c_int = 0;
pub const AUD_PLL_F1: c_int = 1;
pub const AUD_PLL_A2: c_int = 2;
pub const AUD_PLL_F2: c_int = 3;
pub const AUD_PLL_APLL: c_int = 4;
pub const AUD_PLL_RX0: c_int = 5;
pub const AUD_PLL_USB0: c_int = 6;
pub const AUD_PLL_HSC0: c_int = 7;

pub const AUD_PLLDIV_1_2: c_int = 0;
pub const AUD_PLLDIV_1_3: c_int = 1;
pub const AUD_PLLDIV_1_1: c_int = 2;
pub const AUD_PLLDIV_2_3: c_int = 3;

pub const AUD_VOL_INIT: u32 = 0x4000;
pub const AUD_VOL_MAX: u32 = 0xffff;
pub const AUD_VOL_FADE_TIME: c_int = 20;

pub const AUD_RING_SIZE: usize = 128 * 1024;

pub const AUD_MIN_FRAGMENT: c_int = 4;
pub const AUD_MAX_FRAGMENT: c_int = 8;
pub const AUD_MIN_FRAGMENT_SIZE: usize = 4 * 1024;
pub const AUD_MAX_FRAGMENT_SIZE: usize = 16 * 1024;

pub const AUD_MAX_SLOTSEL: c_int = 5;

// This is a selector for virtual register map of AIO.
//
// map:  Specify the index of virtual register map.
// hw :  Specify the ID of real register map, selector uses this value.
//       A meaning of this value depends specification of SoC.
#[repr(C)]
pub struct uniphier_aio_selector {
    pub map: c_int,
    pub hw: c_int,
}

// 'SoftWare MAPping' setting of UniPhier AIO registers.
//
// We have to setup 'virtual' register maps to access 'real' registers of AIO.
// This feature is legacy and meaningless but AIO needs this to work.
//
// Each hardware blocks have own virtual register maps as following:
//
// Address Virtual                      Real
// ------- ---------                    ---------------
// 0x12000 DMAC map0 --> [selector] --> DMAC hardware 3
// 0x12080 DMAC map1 --> [selector] --> DMAC hardware 1
// ...
// 0x42000 Port map0 --> [selector] --> Port hardware 1
// 0x42400 Port map1 --> [selector] --> Port hardware 2
// ...
//
// ch   : Input or output channel of DMAC
// rb   : Ring buffer
// iport: PCM input port
// iif  : Input interface
// oport: PCM output port
// oif  : Output interface
// och  : Output channel of DMAC for sampling rate converter
//
// These are examples for sound data paths:
//
// For caputure device:
//   (outer of AIO) -> iport -> iif -> ch -> rb -> (CPU)
// For playback device:
//   (CPU) -> rb -> ch -> oif -> oport -> (outer of AIO)
// For sampling rate converter device:
//   (CPU) -> rb -> ch -> oif -> (HW SRC) -> iif -> och -> orb -> (CPU)
#[repr(C)]
pub struct uniphier_aio_swmap {
    pub r#type: c_int,
    pub dir: c_int,

    pub ch: uniphier_aio_selector,
    pub rb: uniphier_aio_selector,
    pub iport: uniphier_aio_selector,
    pub iif: uniphier_aio_selector,
    pub oport: uniphier_aio_selector,
    pub oif: uniphier_aio_selector,
    pub och: uniphier_aio_selector,
}

#[repr(C)]
pub struct uniphier_aio_spec {
    pub name: *const c_char,
    pub gname: *const c_char,
    pub swm: uniphier_aio_swmap,
}

#[repr(C)]
pub struct uniphier_aio_pll {
    pub enable: bool,
    pub freq: u32,
}

#[repr(C)]
pub struct uniphier_aio_chip_spec {
    pub specs: *const uniphier_aio_spec,
    pub num_specs: c_int,
    pub plls: *const uniphier_aio_pll,
    pub num_plls: c_int,
    pub dais: *mut snd_soc_dai_driver,
    pub num_dais: c_int,

    pub addr_ext: c_int,
}

#[repr(C)]
pub struct uniphier_aio_sub {
    pub aio: *mut uniphier_aio,

    pub lock: spinlock_t,

    pub swm: *const uniphier_aio_swmap,
    pub spec: *const uniphier_aio_spec,

    pub substream: *mut snd_pcm_substream,
    pub params: snd_pcm_hw_params,
    pub vol: c_int,

    pub cstream: *mut snd_compr_stream,
    pub cparams: snd_compr_params,
    pub compr_area: *mut u8,
    pub compr_addr: dma_addr_t,
    pub compr_bytes: usize,
    pub pass_through: c_int,
    pub iec_pc: IEC61937_PC,
    pub iec_header: bool,

    pub use_mmap: bool,
    pub setting: c_int,
    pub running: c_int,
    pub rd_offs: u64,
    pub wr_offs: u64,
    pub threshold: u32,
    pub rd_org: u64,
    pub wr_org: u64,
    pub rd_total: u64,
    pub wr_total: u64,
}

#[repr(C)]
pub struct uniphier_aio {
    pub chip: *mut uniphier_aio_chip,

    pub sub: [uniphier_aio_sub; 2],

    pub fmt: u32,
    pub clk_in: c_int,
    pub clk_out: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,
    pub plldiv: c_int,
}

#[repr(C)]
pub struct uniphier_aio_chip {
    pub pdev: *mut platform_device,
    pub chip_spec: *const uniphier_aio_chip_spec,

    pub aios: *mut uniphier_aio,
    pub num_aios: c_int,
    pub num_wup_aios: c_int,
    pub plls: *mut uniphier_aio_pll,
    pub num_plls: c_int,

    pub clk: *mut clk,
    pub rst: *mut reset_control,
    pub regmap: *mut regmap,
    pub regmap_sg: *mut regmap,
    pub active: c_int,
}

#[inline]
pub unsafe fn uniphier_priv(dai: *mut snd_soc_dai) -> *mut uniphier_aio {
    let chip = snd_soc_dai_get_drvdata(dai) as *mut uniphier_aio_chip;
    &mut (*chip).aios[(*dai).id as usize]
}

extern "C" {
    pub fn uniphier_aiodma_soc_register_platform(pdev: *mut platform_device) -> c_int;
    pub static uniphier_aio_compress_ops: snd_compress_ops;

    pub fn uniphier_aio_probe(pdev: *mut platform_device) -> c_int;
    pub fn uniphier_aio_remove(pdev: *mut platform_device);
    pub static uniphier_aio_i2s_ld11_ops: snd_soc_dai_ops;
    pub static uniphier_aio_i2s_pxs2_ops: snd_soc_dai_ops;
    pub static uniphier_aio_spdif_ld11_ops: snd_soc_dai_ops;
    pub static uniphier_aio_spdif_ld11_ops2: snd_soc_dai_ops;
    pub static uniphier_aio_spdif_pxs2_ops: snd_soc_dai_ops;
    pub static uniphier_aio_spdif_pxs2_ops2: snd_soc_dai_ops;

    pub fn aio_rb_cnt(sub: *mut uniphier_aio_sub) -> u64;
    pub fn aio_rbt_cnt_to_end(sub: *mut uniphier_aio_sub) -> u64;
    pub fn aio_rb_space(sub: *mut uniphier_aio_sub) -> u64;
    pub fn aio_rb_space_to_end(sub: *mut uniphier_aio_sub) -> u64;

    pub fn aio_iecout_set_enable(chip: *mut uniphier_aio_chip, enable: bool);
    pub fn aio_chip_set_pll(chip: *mut uniphier_aio_chip, pll_id: c_int, freq: u32) -> c_int;
    pub fn aio_chip_init(chip: *mut uniphier_aio_chip);
    pub fn aio_init(sub: *mut uniphier_aio_sub) -> c_int;
    pub fn aio_port_reset(sub: *mut uniphier_aio_sub);
    pub fn aio_port_set_param(
        sub: *mut uniphier_aio_sub,
        pass_through: c_int,
        params: *const snd_pcm_hw_params,
    ) -> c_int;
    pub fn aio_port_set_enable(sub: *mut uniphier_aio_sub, enable: c_int);
    pub fn aio_port_get_volume(sub: *mut uniphier_aio_sub) -> c_int;
    pub fn aio_port_set_volume(sub: *mut uniphier_aio_sub, vol: c_int);
    pub fn aio_if_set_param(sub: *mut uniphier_aio_sub, pass_through: c_int) -> c_int;
    pub fn aio_oport_set_stream_type(sub: *mut uniphier_aio_sub, pc: IEC61937_PC) -> c_int;
    pub fn aio_src_reset(sub: *mut uniphier_aio_sub);
    pub fn aio_src_set_param(
        sub: *mut uniphier_aio_sub,
        params: *const snd_pcm_hw_params,
    ) -> c_int;
    pub fn aio_srcif_set_param(sub: *mut uniphier_aio_sub) -> c_int;
    pub fn aio_srcch_set_param(sub: *mut uniphier_aio_sub) -> c_int;
    pub fn aio_srcch_set_enable(sub: *mut uniphier_aio_sub, enable: c_int);

    pub fn aiodma_ch_set_param(sub: *mut uniphier_aio_sub) -> c_int;
    pub fn aiodma_ch_set_enable(sub: *mut uniphier_aio_sub, enable: c_int);
    pub fn aiodma_rb_set_threshold(sub: *mut uniphier_aio_sub, size: u64, th: u32) -> c_int;
    pub fn aiodma_rb_set_buffer(
        sub: *mut uniphier_aio_sub,
        start: u64,
        end: u64,
        period: c_int,
    ) -> c_int;
    pub fn aiodma_rb_sync(sub: *mut uniphier_aio_sub, start: u64, size: u64, period: c_int);
    pub fn aiodma_rb_is_irq(sub: *mut uniphier_aio_sub) -> bool;
    pub fn aiodma_rb_clear_irq(sub: *mut uniphier_aio_sub);

    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
