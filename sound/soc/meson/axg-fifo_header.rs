/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (c) 2018 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

/* C header guard and include directives are intentionally omitted. */

#[repr(C)]
pub struct clk {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct reg_field {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct regmap_field {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _unused: [u8; 0],
}

pub type snd_pcm_uframes_t = ::std::os::raw::c_ulong;

#[allow(non_snake_case)]
pub const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

#[allow(non_snake_case)]
pub const fn GENMASK(h: u32, l: u32) -> u32 {
    u32::MAX.wrapping_shl(l) & u32::MAX.wrapping_shr(31 - h)
}

pub const AXG_FIFO_CH_MAX: u32 = 128;

/* Depends on SNDRV_PCM_FMTBIT_* constants supplied by ALSA headers. */
pub const AXG_FIFO_FORMATS: u64 = SNDRV_PCM_FMTBIT_S8
    | SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE
    | SNDRV_PCM_FMTBIT_IEC958_SUBFRAME_LE;

pub const AXG_FIFO_BURST: u32 = 8;

pub const FIFO_INT_ADDR_FINISH: u32 = BIT(0);
pub const FIFO_INT_ADDR_INT: u32 = BIT(1);
pub const FIFO_INT_COUNT_REPEAT: u32 = BIT(2);
pub const FIFO_INT_COUNT_ONCE: u32 = BIT(3);
pub const FIFO_INT_FIFO_ZERO: u32 = BIT(4);
pub const FIFO_INT_FIFO_DEPTH: u32 = BIT(5);
pub const FIFO_INT_MASK: u32 = GENMASK(7, 0);

pub const FIFO_CTRL0: u32 = 0x00;
pub const CTRL0_DMA_EN: u32 = BIT(31);
pub const CTRL0_INT_EN: u32 = GENMASK(23, 16);
pub const CTRL0_SEL_MASK: u32 = GENMASK(2, 0);
pub const CTRL0_SEL_SHIFT: u32 = 0;
pub const FIFO_CTRL1: u32 = 0x04;
pub const CTRL1_INT_CLR: u32 = GENMASK(7, 0);
pub const CTRL1_STATUS2_SEL: u32 = GENMASK(11, 8);
pub const STATUS2_SEL_DDR_READ: u32 = 0;
pub const CTRL1_FRDDR_DEPTH: u32 = GENMASK(31, 24);
pub const FIFO_START_ADDR: u32 = 0x08;
pub const FIFO_FINISH_ADDR: u32 = 0x0c;
pub const FIFO_INT_ADDR: u32 = 0x10;
pub const FIFO_STATUS1: u32 = 0x14;
pub const STATUS1_INT_STS: u32 = GENMASK(7, 0);
pub const FIFO_STATUS2: u32 = 0x18;
pub const FIFO_INIT_ADDR: u32 = 0x24;
pub const FIFO_CTRL2: u32 = 0x28;

#[repr(C)]
pub struct axg_fifo {
    pub map: *mut regmap,
    pub pclk: *mut clk,
    pub arb: *mut reset_control,
    pub field_threshold: *mut regmap_field,
    pub depth: ::std::os::raw::c_uint,
    pub irq: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct axg_fifo_match_data {
    pub component_drv: *const snd_soc_component_driver,
    pub dai_drv: *mut snd_soc_dai_driver,
    pub field_threshold: reg_field,
}

unsafe extern "C" {
    pub fn axg_fifo_pcm_open(
        component: *mut snd_soc_component,
        ss: *mut snd_pcm_substream,
    ) -> ::std::os::raw::c_int;
    pub fn axg_fifo_pcm_close(
        component: *mut snd_soc_component,
        ss: *mut snd_pcm_substream,
    ) -> ::std::os::raw::c_int;
    pub fn axg_fifo_pcm_hw_params(
        component: *mut snd_soc_component,
        ss: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
    ) -> ::std::os::raw::c_int;
    pub fn g12a_fifo_pcm_hw_params(
        component: *mut snd_soc_component,
        ss: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
    ) -> ::std::os::raw::c_int;
    pub fn axg_fifo_pcm_hw_free(
        component: *mut snd_soc_component,
        ss: *mut snd_pcm_substream,
    ) -> ::std::os::raw::c_int;
    pub fn axg_fifo_pcm_pointer(
        component: *mut snd_soc_component,
        ss: *mut snd_pcm_substream,
    ) -> snd_pcm_uframes_t;
    pub fn axg_fifo_pcm_trigger(
        component: *mut snd_soc_component,
        ss: *mut snd_pcm_substream,
        cmd: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn axg_fifo_pcm_new(
        rtd: *mut snd_soc_pcm_runtime,
        type_: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn axg_fifo_probe(pdev: *mut platform_device) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
