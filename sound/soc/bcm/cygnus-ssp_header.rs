/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2014-2015 Broadcom Corporation */

pub const CYGNUS_TDM_DAI_MAX_SLOTS: u32 = 16;

pub const CYGNUS_MAX_PLAYBACK_PORTS: usize = 4;
pub const CYGNUS_MAX_CAPTURE_PORTS: usize = 3;
pub const CYGNUS_MAX_I2S_PORTS: usize = 3;
pub const CYGNUS_MAX_PORTS: usize = CYGNUS_MAX_PLAYBACK_PORTS;
pub const CYGNUS_AUIDO_MAX_NUM_CLKS: usize = 3;

pub const CYGNUS_SSP_FRAMEBITS_DIV: u32 = 1;

pub const CYGNUS_SSPMODE_I2S: i32 = 0;
pub const CYGNUS_SSPMODE_TDM: i32 = 1;
pub const CYGNUS_SSPMODE_UNKNOWN: i32 = -1;

pub const CYGNUS_SSP_CLKSRC_PLL: u32 = 0;

/* Max string length of our dt property names */
pub const PROP_LEN_MAX: u32 = 40;

#[repr(C)]
pub struct ringbuf_regs {
    pub rdaddr: ::core::ffi::c_uint,
    pub wraddr: ::core::ffi::c_uint,
    pub baseaddr: ::core::ffi::c_uint,
    pub endaddr: ::core::ffi::c_uint,
    pub fmark: ::core::ffi::c_uint, /* freemark for play, fullmark for caputure */
    pub period_bytes: ::core::ffi::c_uint,
    pub buf_size: ::core::ffi::c_uint,
}

/*
 * C token-pasting macros translated as intent-preserving Rust macros.
 * Stable Rust cannot concatenate `$num` into external identifier names here;
 * callers should provide the five already-resolved register offset constants.
 */
#[macro_export]
macro_rules! RINGBUF_REG_PLAYBACK {
    ($rdaddr:expr, $wraddr:expr, $baseaddr:expr, $endaddr:expr, $free_mark:expr) => {
        ringbuf_regs {
            rdaddr: $rdaddr,
            wraddr: $wraddr,
            baseaddr: $baseaddr,
            endaddr: $endaddr,
            fmark: $free_mark,
            period_bytes: 0,
            buf_size: 0,
        }
    };
}

#[macro_export]
macro_rules! RINGBUF_REG_CAPTURE {
    ($rdaddr:expr, $wraddr:expr, $baseaddr:expr, $endaddr:expr, $full_mark:expr) => {
        ringbuf_regs {
            rdaddr: $rdaddr,
            wraddr: $wraddr,
            baseaddr: $baseaddr,
            endaddr: $endaddr,
            fmark: $full_mark,
            period_bytes: 0,
            buf_size: 0,
        }
    };
}

pub const PORT_TDM: u32 = 0;
pub const PORT_SPDIF: u32 = 1;

#[repr(C)]
pub struct cygnus_ssp_regs {
    pub i2s_stream_cfg: u32,
    pub i2s_cfg: u32,
    pub i2s_cap_stream_cfg: u32,
    pub i2s_cap_cfg: u32,
    pub i2s_mclk_cfg: u32,

    pub bf_destch_ctrl: u32,
    pub bf_destch_cfg: u32,
    pub bf_sourcech_ctrl: u32,
    pub bf_sourcech_cfg: u32,
    pub bf_sourcech_grp: u32,
}

#[repr(C)]
pub struct cygnus_track_clk {
    pub cap_en: bool,
    pub play_en: bool,
    pub cap_clk_en: bool,
    pub play_clk_en: bool,
}

#[repr(C)]
pub struct cygnus_aio_port {
    pub portnum: ::core::ffi::c_int,
    pub mode: ::core::ffi::c_int,
    pub is_slave: bool,
    pub streams_on: ::core::ffi::c_int, /* will be 0 if both capture and play are off */
    pub fsync_width: ::core::ffi::c_int,
    pub port_type: ::core::ffi::c_int,

    pub mclk: u32,
    pub lrclk: u32,
    pub bit_per_frame: u32,
    pub pll_clk_num: u32,

    pub cygaud: *mut cygnus_audio,
    pub regs: cygnus_ssp_regs,

    pub play_rb_regs: ringbuf_regs,
    pub capture_rb_regs: ringbuf_regs,

    pub play_stream: *mut snd_pcm_substream,
    pub capture_stream: *mut snd_pcm_substream,

    pub clk_trace: cygnus_track_clk,
}

#[repr(C)]
pub struct cygnus_audio {
    pub portinfo: [cygnus_aio_port; CYGNUS_MAX_PORTS],

    pub irq_num: ::core::ffi::c_int,
    pub audio: *mut ::core::ffi::c_void,
    pub dev: *mut device,
    pub i2s_in: *mut ::core::ffi::c_void,

    pub audio_clk: [*mut clk; CYGNUS_AUIDO_MAX_NUM_CLKS],
    pub active_ports: ::core::ffi::c_int,
    pub vco_rate: ::core::ffi::c_ulong,
}

unsafe extern "C" {
    pub type snd_pcm_substream;
    pub type snd_soc_dai;
    pub type device;
    pub type clk;

    pub fn cygnus_ssp_set_custom_fsync_width(
        cpu_dai: *mut snd_soc_dai,
        len: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn cygnus_soc_platform_register(
        dev: *mut device,
        cygaud: *mut cygnus_audio,
    ) -> ::core::ffi::c_int;
    pub fn cygnus_soc_platform_unregister(dev: *mut device) -> ::core::ffi::c_int;
}

/*
 * Duplicate C declaration preserved in this comment:
 * extern int cygnus_ssp_set_custom_fsync_width(struct snd_soc_dai *cpu_dai,
 *     int len);
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
