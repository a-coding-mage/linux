// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) STMicroelectronics SA 2015
 * Authors: Arnaud Pouliquen <arnaud.pouliquen@st.com>
 *          for STMicroelectronics.
 */

use core::ffi::{c_int, c_uint, c_ulong, c_void};

/* Dependencies from <linux/regmap.h> and <sound/dmaengine_pcm.h> are external. */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap_field {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_ulong,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub buffer_bytes_max: usize,
}
#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_aes_iec958 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_driver {
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
pub struct snd_pcm_hw_rule {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub channels: c_uint,
    pub format: c_int,
}

pub type spinlock_t = c_ulong;

unsafe extern "C" {
    pub fn snd_pcm_format_width(format: c_int) -> c_int;
}

pub const PAGE_SIZE: usize = 4096;
pub const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 0;
pub const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 1;
pub const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 2;
pub const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 3;
pub const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 4;
pub const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << 10;
pub const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 2;
pub const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;

#[inline]
pub const fn BIT(n: c_int) -> u32 {
    if n < 0 {
        0
    } else {
        1u32 << (n as u32)
    }
}

#[inline]
pub unsafe fn readl_relaxed(addr: *mut c_void) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

#[inline]
pub unsafe fn writel_relaxed(value: u32, addr: *mut c_void) {
    unsafe { core::ptr::write_volatile(addr as *mut u32, value) }
}

#[inline]
unsafe fn reg_addr(ip: *mut uniperif, offset: c_int) -> *mut c_void {
    unsafe { ((*ip).base as *mut u8).offset(offset as isize) as *mut c_void }
}

/*
 * Register access macros
 */
#[inline]
pub unsafe fn GET_UNIPERIF_REG(ip: *mut uniperif, offset: c_int, shift: c_int, mask: u32) -> u32 {
    unsafe { (readl_relaxed(reg_addr(ip, offset)) >> (shift as u32)) & mask }
}

#[inline]
pub unsafe fn SET_UNIPERIF_REG(ip: *mut uniperif, offset: c_int, shift: c_int, mask: u32, value: u32) {
    unsafe {
        let addr = reg_addr(ip, offset);
        writel_relaxed(
            (readl_relaxed(addr) & !(mask << (shift as u32))) | ((value & mask) << (shift as u32)),
            addr,
        );
    }
}

#[inline]
pub unsafe fn SET_UNIPERIF_BIT_REG(ip: *mut uniperif, offset: c_int, shift: c_int, mask: u32, value: u32) {
    unsafe { writel_relaxed((value & mask) << (shift as u32), reg_addr(ip, offset)) }
}

/*
 * UNIPERIF_SOFT_RST reg
 */
pub const fn UNIPERIF_SOFT_RST_OFFSET(_: *const uniperif) -> c_int { 0x0000 }
pub unsafe fn GET_UNIPERIF_SOFT_RST(ip: *mut uniperif) -> u32 {
    unsafe {
        if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int {
            readl_relaxed(reg_addr(ip, UNIPERIF_SOFT_RST_OFFSET(ip)))
        } else { 0 }
    }
}
pub unsafe fn SET_UNIPERIF_SOFT_RST(ip: *mut uniperif, value: u32) { unsafe { writel_relaxed(value, reg_addr(ip, UNIPERIF_SOFT_RST_OFFSET(ip))) } }
pub const fn UNIPERIF_SOFT_RST_SOFT_RST_SHIFT(_: *const uniperif) -> c_int { 0x0 }
pub const fn UNIPERIF_SOFT_RST_SOFT_RST_MASK(_: *const uniperif) -> u32 { 0x1 }
pub unsafe fn SET_UNIPERIF_SOFT_RST_SOFT_RST(ip: *mut uniperif) { unsafe { SET_UNIPERIF_BIT_REG(ip, UNIPERIF_SOFT_RST_OFFSET(ip), UNIPERIF_SOFT_RST_SOFT_RST_SHIFT(ip), UNIPERIF_SOFT_RST_SOFT_RST_MASK(ip), 1) } }
pub unsafe fn GET_UNIPERIF_SOFT_RST_SOFT_RST(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_SOFT_RST_OFFSET(ip), UNIPERIF_SOFT_RST_SOFT_RST_SHIFT(ip), UNIPERIF_SOFT_RST_SOFT_RST_MASK(ip)) } }

/*
 * UNIPERIF_FIFO_DATA reg
 */
pub const fn UNIPERIF_FIFO_DATA_OFFSET(_: *const uniperif) -> c_int { 0x0004 }
pub unsafe fn SET_UNIPERIF_DATA(ip: *mut uniperif, value: u32) { unsafe { writel_relaxed(value, reg_addr(ip, UNIPERIF_FIFO_DATA_OFFSET(ip))) } }

/*
 * UNIPERIF_CHANNEL_STA_REGN reg
 */
pub const fn UNIPERIF_CHANNEL_STA_REGN(_: *const uniperif, n: c_int) -> c_int { 0x0060 + (4 * n) }
/* C macro GET_UNIPERIF_CHANNEL_STA_REGN(ip) references n without declaring it. */
pub unsafe fn GET_UNIPERIF_CHANNEL_STA_REGN(ip: *mut uniperif, n: c_int) -> u32 { unsafe { readl_relaxed(reg_addr(ip, UNIPERIF_CHANNEL_STA_REGN(ip, n))) } }
pub unsafe fn SET_UNIPERIF_CHANNEL_STA_REGN(ip: *mut uniperif, n: c_int, value: u32) { unsafe { writel_relaxed(value, reg_addr(ip, UNIPERIF_CHANNEL_STA_REGN(ip, n))) } }

macro_rules! channel_sta_reg {
    ($off:ident, $get:ident, $set:ident, $value:expr) => {
        pub const fn $off(_: *const uniperif) -> c_int { $value }
        pub unsafe fn $get(ip: *mut uniperif) -> u32 { unsafe { readl_relaxed(reg_addr(ip, $off(ip))) } }
        pub unsafe fn $set(ip: *mut uniperif, value: u32) { unsafe { writel_relaxed(value, reg_addr(ip, $off(ip))) } }
    };
}
channel_sta_reg!(UNIPERIF_CHANNEL_STA_REG0_OFFSET, GET_UNIPERIF_CHANNEL_STA_REG0, SET_UNIPERIF_CHANNEL_STA_REG0, 0x0060);
channel_sta_reg!(UNIPERIF_CHANNEL_STA_REG1_OFFSET, GET_UNIPERIF_CHANNEL_STA_REG1, SET_UNIPERIF_CHANNEL_STA_REG1, 0x0064);
channel_sta_reg!(UNIPERIF_CHANNEL_STA_REG2_OFFSET, GET_UNIPERIF_CHANNEL_STA_REG2, SET_UNIPERIF_CHANNEL_STA_REG2, 0x0068);
channel_sta_reg!(UNIPERIF_CHANNEL_STA_REG3_OFFSET, GET_UNIPERIF_CHANNEL_STA_REG3, SET_UNIPERIF_CHANNEL_STA_REG3, 0x006C);
channel_sta_reg!(UNIPERIF_CHANNEL_STA_REG4_OFFSET, GET_UNIPERIF_CHANNEL_STA_REG4, SET_UNIPERIF_CHANNEL_STA_REG4, 0x0070);
channel_sta_reg!(UNIPERIF_CHANNEL_STA_REG5_OFFSET, GET_UNIPERIF_CHANNEL_STA_REG5, SET_UNIPERIF_CHANNEL_STA_REG5, 0x0074);

macro_rules! simple_reg {
    ($off:ident, $get:ident, $set:ident, $value:expr) => {
        pub const fn $off(_: *const uniperif) -> c_int { $value }
        pub unsafe fn $get(ip: *mut uniperif) -> u32 { unsafe { readl_relaxed(reg_addr(ip, $off(ip))) } }
        pub unsafe fn $set(ip: *mut uniperif, value: u32) { unsafe { writel_relaxed(value, reg_addr(ip, $off(ip))) } }
    };
}
macro_rules! ro_reg {
    ($off:ident, $get:ident, $value:expr) => {
        pub const fn $off(_: *const uniperif) -> c_int { $value }
        pub unsafe fn $get(ip: *mut uniperif) -> u32 { unsafe { readl_relaxed(reg_addr(ip, $off(ip))) } }
    };
}
macro_rules! wo_reg {
    ($off:ident, $set:ident, $value:expr) => {
        pub const fn $off(_: *const uniperif) -> c_int { $value }
        pub unsafe fn $set(ip: *mut uniperif, value: u32) { unsafe { writel_relaxed(value, reg_addr(ip, $off(ip))) } }
    };
}

/*
 *  UNIPERIF_ITS reg
 */
ro_reg!(UNIPERIF_ITS_OFFSET, GET_UNIPERIF_ITS, 0x000C);
pub const fn UNIPERIF_ITS_MEM_BLK_READ_SHIFT(_: *const uniperif) -> c_int { 5 }
pub const fn UNIPERIF_ITS_MEM_BLK_READ_MASK(ip: *const uniperif) -> u32 { BIT(UNIPERIF_ITS_MEM_BLK_READ_SHIFT(ip)) }
pub unsafe fn UNIPERIF_ITS_FIFO_ERROR_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { 8 } } }
pub unsafe fn UNIPERIF_ITS_FIFO_ERROR_MASK(ip: *const uniperif) -> u32 { unsafe { BIT(UNIPERIF_ITS_FIFO_ERROR_SHIFT(ip)) } }
pub const fn UNIPERIF_ITS_DMA_ERROR_SHIFT(_: *const uniperif) -> c_int { 9 }
pub const fn UNIPERIF_ITS_DMA_ERROR_MASK(ip: *const uniperif) -> u32 { BIT(UNIPERIF_ITS_DMA_ERROR_SHIFT(ip)) }
pub unsafe fn UNIPERIF_ITS_UNDERFLOW_REC_DONE_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { -1 } else { 12 } } }
pub unsafe fn UNIPERIF_ITS_UNDERFLOW_REC_DONE_MASK(ip: *const uniperif) -> u32 { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { BIT(UNIPERIF_ITS_UNDERFLOW_REC_DONE_SHIFT(ip)) } } }
pub unsafe fn UNIPERIF_ITS_UNDERFLOW_REC_FAILED_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { -1 } else { 13 } } }
pub unsafe fn UNIPERIF_ITS_UNDERFLOW_REC_FAILED_MASK(ip: *const uniperif) -> u32 { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { BIT(UNIPERIF_ITS_UNDERFLOW_REC_FAILED_SHIFT(ip)) } } }

/*
 *  UNIPERIF_ITS_BCLR reg
 */
wo_reg!(UNIPERIF_ITS_BCLR_OFFSET, SET_UNIPERIF_ITS_BCLR, 0x0010);
pub unsafe fn UNIPERIF_ITS_BCLR_FIFO_ERROR_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { 8 } } }
pub unsafe fn UNIPERIF_ITS_BCLR_FIFO_ERROR_MASK(ip: *const uniperif) -> u32 { unsafe { BIT(UNIPERIF_ITS_BCLR_FIFO_ERROR_SHIFT(ip)) } }
pub unsafe fn SET_UNIPERIF_ITS_BCLR_FIFO_ERROR(ip: *mut uniperif) { unsafe { SET_UNIPERIF_ITS_BCLR(ip, UNIPERIF_ITS_BCLR_FIFO_ERROR_MASK(ip)) } }

/*
 *  UNIPERIF_ITM reg
 */
ro_reg!(UNIPERIF_ITM_OFFSET, GET_UNIPERIF_ITM, 0x0018);
pub unsafe fn UNIPERIF_ITM_FIFO_ERROR_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { 8 } } }
pub unsafe fn UNIPERIF_ITM_FIFO_ERROR_MASK(ip: *const uniperif) -> u32 { unsafe { BIT(UNIPERIF_ITM_FIFO_ERROR_SHIFT(ip)) } }
pub unsafe fn UNIPERIF_ITM_UNDERFLOW_REC_DONE_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { -1 } else { 12 } } }
pub unsafe fn UNIPERIF_ITM_UNDERFLOW_REC_DONE_MASK(ip: *const uniperif) -> u32 { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { BIT(UNIPERIF_ITM_UNDERFLOW_REC_DONE_SHIFT(ip)) } } }
pub unsafe fn UNIPERIF_ITM_UNDERFLOW_REC_FAILED_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { -1 } else { 13 } } }
pub unsafe fn UNIPERIF_ITM_UNDERFLOW_REC_FAILED_MASK(ip: *const uniperif) -> u32 { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { BIT(UNIPERIF_ITM_UNDERFLOW_REC_FAILED_SHIFT(ip)) } } }

wo_reg!(UNIPERIF_ITM_BCLR_OFFSET, SET_UNIPERIF_ITM_BCLR, 0x001c);
pub unsafe fn UNIPERIF_ITM_BCLR_FIFO_ERROR_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { 8 } } }
pub unsafe fn UNIPERIF_ITM_BCLR_FIFO_ERROR_MASK(ip: *const uniperif) -> u32 { unsafe { BIT(UNIPERIF_ITM_BCLR_FIFO_ERROR_SHIFT(ip)) } }
pub unsafe fn SET_UNIPERIF_ITM_BCLR_FIFO_ERROR(ip: *mut uniperif) { unsafe { SET_UNIPERIF_ITM_BCLR(ip, UNIPERIF_ITM_BCLR_FIFO_ERROR_MASK(ip)) } }
pub const fn UNIPERIF_ITM_BCLR_DMA_ERROR_SHIFT(_: *const uniperif) -> c_int { 9 }
pub const fn UNIPERIF_ITM_BCLR_DMA_ERROR_MASK(ip: *const uniperif) -> u32 { BIT(UNIPERIF_ITM_BCLR_DMA_ERROR_SHIFT(ip)) }
pub unsafe fn SET_UNIPERIF_ITM_BCLR_DMA_ERROR(ip: *mut uniperif) { unsafe { SET_UNIPERIF_ITM_BCLR(ip, UNIPERIF_ITM_BCLR_DMA_ERROR_MASK(ip)) } }

wo_reg!(UNIPERIF_ITM_BSET_OFFSET, SET_UNIPERIF_ITM_BSET, 0x0020);
pub unsafe fn UNIPERIF_ITM_BSET_FIFO_ERROR_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { 8 } } }
pub unsafe fn UNIPERIF_ITM_BSET_FIFO_ERROR_MASK(ip: *const uniperif) -> u32 { unsafe { BIT(UNIPERIF_ITM_BSET_FIFO_ERROR_SHIFT(ip)) } }
pub unsafe fn SET_UNIPERIF_ITM_BSET_FIFO_ERROR(ip: *mut uniperif) { unsafe { SET_UNIPERIF_ITM_BSET(ip, UNIPERIF_ITM_BSET_FIFO_ERROR_MASK(ip)) } }
pub const fn UNIPERIF_ITM_BSET_MEM_BLK_READ_SHIFT(_: *const uniperif) -> c_int { 5 }
pub const fn UNIPERIF_ITM_BSET_MEM_BLK_READ_MASK(ip: *const uniperif) -> u32 { BIT(UNIPERIF_ITM_BSET_MEM_BLK_READ_SHIFT(ip)) }
pub unsafe fn SET_UNIPERIF_ITM_BSET_MEM_BLK_READ(ip: *mut uniperif) { unsafe { SET_UNIPERIF_ITM_BSET(ip, UNIPERIF_ITM_BSET_MEM_BLK_READ_MASK(ip)) } }
pub const fn UNIPERIF_ITM_BSET_DMA_ERROR_SHIFT(_: *const uniperif) -> c_int { 9 }
pub const fn UNIPERIF_ITM_BSET_DMA_ERROR_MASK(ip: *const uniperif) -> u32 { BIT(UNIPERIF_ITM_BSET_DMA_ERROR_SHIFT(ip)) }
pub unsafe fn SET_UNIPERIF_ITM_BSET_DMA_ERROR(ip: *mut uniperif) { unsafe { SET_UNIPERIF_ITM_BSET(ip, UNIPERIF_ITM_BSET_DMA_ERROR_MASK(ip)) } }
pub unsafe fn UNIPERIF_ITM_BSET_UNDERFLOW_REC_DONE_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { -1 } else { 12 } } }
pub unsafe fn UNIPERIF_ITM_BSET_UNDERFLOW_REC_DONE_MASK(ip: *const uniperif) -> u32 { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { BIT(UNIPERIF_ITM_BSET_UNDERFLOW_REC_DONE_SHIFT(ip)) } } }
pub unsafe fn SET_UNIPERIF_ITM_BSET_UNDERFLOW_REC_DONE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_ITM_BSET(ip, UNIPERIF_ITM_BSET_UNDERFLOW_REC_DONE_MASK(ip)) } }
pub unsafe fn UNIPERIF_ITM_BSET_UNDERFLOW_REC_FAILED_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { -1 } else { 13 } } }
pub unsafe fn UNIPERIF_ITM_BSET_UNDERFLOW_REC_FAILED_MASK(ip: *const uniperif) -> u32 { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { BIT(UNIPERIF_ITM_BSET_UNDERFLOW_REC_FAILED_SHIFT(ip)) } } }
pub unsafe fn SET_UNIPERIF_ITM_BSET_UNDERFLOW_REC_FAILED(ip: *mut uniperif) { unsafe { SET_UNIPERIF_ITM_BSET(ip, UNIPERIF_ITM_BSET_UNDERFLOW_REC_FAILED_MASK(ip)) } }

/*
 * UNIPERIF_CONFIG reg
 */
simple_reg!(UNIPERIF_CONFIG_OFFSET, GET_UNIPERIF_CONFIG, SET_UNIPERIF_CONFIG, 0x0040);
macro_rules! field {
    ($s:ident, $m:ident, $get:ident, $set:ident, $shift:expr, $mask:expr) => {
        pub const fn $s(_: *const uniperif) -> c_int { $shift }
        pub const fn $m(_: *const uniperif) -> u32 { $mask }
        pub unsafe fn $get(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_CONFIG_OFFSET(ip), $s(ip), $m(ip)) } }
        pub unsafe fn $set(ip: *mut uniperif, value: u32) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CONFIG_OFFSET(ip), $s(ip), $m(ip), value) } }
    };
}
field!(UNIPERIF_CONFIG_PARITY_CNTR_SHIFT, UNIPERIF_CONFIG_PARITY_CNTR_MASK, GET_UNIPERIF_CONFIG_PARITY_CNTR, SET_UNIPERIF_CONFIG_PARITY_CNTR, 0, 0x1);
pub unsafe fn SET_UNIPERIF_CONFIG_PARITY_CNTR_BY_HW(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_PARITY_CNTR(ip, 0) } }
pub unsafe fn SET_UNIPERIF_CONFIG_PARITY_CNTR_BY_SW(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_PARITY_CNTR(ip, 1) } }
field!(UNIPERIF_CONFIG_CHANNEL_STA_CNTR_SHIFT, UNIPERIF_CONFIG_CHANNEL_STA_CNTR_MASK, GET_UNIPERIF_CONFIG_CHANNEL_STA_CNTR, SET_UNIPERIF_CONFIG_CHANNEL_STA_CNTR, 1, 0x1);
pub unsafe fn SET_UNIPERIF_CONFIG_CHANNEL_STA_CNTR_BY_SW(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_CHANNEL_STA_CNTR(ip, 0) } }
pub unsafe fn SET_UNIPERIF_CONFIG_CHANNEL_STA_CNTR_BY_HW(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_CHANNEL_STA_CNTR(ip, 1) } }
field!(UNIPERIF_CONFIG_USER_DAT_CNTR_SHIFT, UNIPERIF_CONFIG_USER_DAT_CNTR_MASK, GET_UNIPERIF_CONFIG_USER_DAT_CNTR, SET_UNIPERIF_CONFIG_USER_DAT_CNTR, 2, 0x1);
pub unsafe fn SET_UNIPERIF_CONFIG_USER_DAT_CNTR_BY_HW(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_USER_DAT_CNTR(ip, 1) } }
pub unsafe fn SET_UNIPERIF_CONFIG_USER_DAT_CNTR_BY_SW(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_USER_DAT_CNTR(ip, 0) } }
field!(UNIPERIF_CONFIG_VALIDITY_DAT_CNTR_SHIFT, UNIPERIF_CONFIG_VALIDITY_DAT_CNTR_MASK, GET_UNIPERIF_CONFIG_VALIDITY_DAT_CNTR, SET_UNIPERIF_CONFIG_VALIDITY_DAT_CNTR, 3, 0x1);
pub unsafe fn SET_UNIPERIF_CONFIG_VALIDITY_DAT_CNTR_BY_SW(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_VALIDITY_DAT_CNTR(ip, 0) } }
pub unsafe fn SET_UNIPERIF_CONFIG_VALIDITY_DAT_CNTR_BY_HW(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_VALIDITY_DAT_CNTR(ip, 1) } }
field!(UNIPERIF_CONFIG_ONE_BIT_AUD_SHIFT, UNIPERIF_CONFIG_ONE_BIT_AUD_MASK, GET_UNIPERIF_CONFIG_ONE_BIT_AUD, SET_UNIPERIF_CONFIG_ONE_BIT_AUD, 4, 0x1);
pub unsafe fn SET_UNIPERIF_CONFIG_ONE_BIT_AUD_DISABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_ONE_BIT_AUD(ip, 0) } }
pub unsafe fn SET_UNIPERIF_CONFIG_ONE_BIT_AUD_ENABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_ONE_BIT_AUD(ip, 1) } }
field!(UNIPERIF_CONFIG_MEM_FMT_SHIFT, UNIPERIF_CONFIG_MEM_FMT_MASK, GET_UNIPERIF_CONFIG_MEM_FMT, SET_UNIPERIF_CONFIG_MEM_FMT, 5, 0x1);
pub const fn VALUE_UNIPERIF_CONFIG_MEM_FMT_16_0(_: *const uniperif) -> u32 { 0 }
pub const fn VALUE_UNIPERIF_CONFIG_MEM_FMT_16_16(_: *const uniperif) -> u32 { 1 }
pub unsafe fn SET_UNIPERIF_CONFIG_MEM_FMT_16_0(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_MEM_FMT(ip, VALUE_UNIPERIF_CONFIG_MEM_FMT_16_0(ip)) } }
pub unsafe fn SET_UNIPERIF_CONFIG_MEM_FMT_16_16(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_MEM_FMT(ip, VALUE_UNIPERIF_CONFIG_MEM_FMT_16_16(ip)) } }
field!(UNIPERIF_CONFIG_REPEAT_CHL_STS_SHIFT, UNIPERIF_CONFIG_REPEAT_CHL_STS_MASK, GET_UNIPERIF_CONFIG_REPEAT_CHL_STS, SET_UNIPERIF_CONFIG_REPEAT_CHL_STS, 6, 0x1);
pub unsafe fn SET_UNIPERIF_CONFIG_REPEAT_CHL_STS_ENABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_REPEAT_CHL_STS(ip, 0) } }
pub unsafe fn SET_UNIPERIF_CONFIG_REPEAT_CHL_STS_DISABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_REPEAT_CHL_STS(ip, 1) } }
pub unsafe fn UNIPERIF_CONFIG_BACK_STALL_REQ_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 7 } else { -1 } } }
pub const fn UNIPERIF_CONFIG_BACK_STALL_REQ_MASK(_: *const uniperif) -> u32 { 0x1 }
pub unsafe fn GET_UNIPERIF_CONFIG_BACK_STALL_REQ(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_CONFIG_OFFSET(ip), UNIPERIF_CONFIG_BACK_STALL_REQ_SHIFT(ip), UNIPERIF_CONFIG_BACK_STALL_REQ_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_CONFIG_BACK_STALL_REQ_DISABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CONFIG_OFFSET(ip), UNIPERIF_CONFIG_BACK_STALL_REQ_SHIFT(ip), UNIPERIF_CONFIG_BACK_STALL_REQ_MASK(ip), 0) } }
pub unsafe fn SET_UNIPERIF_CONFIG_BACK_STALL_REQ_ENABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CONFIG_OFFSET(ip), UNIPERIF_CONFIG_BACK_STALL_REQ_SHIFT(ip), UNIPERIF_CONFIG_BACK_STALL_REQ_MASK(ip), 1) } }
field!(UNIPERIF_CONFIG_DMA_TRIG_LIMIT_SHIFT, UNIPERIF_CONFIG_DMA_TRIG_LIMIT_MASK, GET_UNIPERIF_CONFIG_DMA_TRIG_LIMIT, SET_UNIPERIF_CONFIG_DMA_TRIG_LIMIT, 8, 0x7F);
pub unsafe fn UNIPERIF_CONFIG_CHL_STS_UPDATE_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 16 } else { -1 } } }
pub const fn UNIPERIF_CONFIG_CHL_STS_UPDATE_MASK(_: *const uniperif) -> u32 { 0x1 }
pub unsafe fn GET_UNIPERIF_CONFIG_CHL_STS_UPDATE(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_CONFIG_OFFSET(ip), UNIPERIF_CONFIG_CHL_STS_UPDATE_SHIFT(ip), UNIPERIF_CONFIG_CHL_STS_UPDATE_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_CONFIG_CHL_STS_UPDATE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CONFIG_OFFSET(ip), UNIPERIF_CONFIG_CHL_STS_UPDATE_SHIFT(ip), UNIPERIF_CONFIG_CHL_STS_UPDATE_MASK(ip), 1) } }
field!(UNIPERIF_CONFIG_IDLE_MOD_SHIFT, UNIPERIF_CONFIG_IDLE_MOD_MASK, GET_UNIPERIF_CONFIG_IDLE_MOD, SET_UNIPERIF_CONFIG_IDLE_MOD, 18, 0x1);
pub unsafe fn SET_UNIPERIF_CONFIG_IDLE_MOD_DISABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_IDLE_MOD(ip, 0) } }
pub unsafe fn SET_UNIPERIF_CONFIG_IDLE_MOD_ENABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_IDLE_MOD(ip, 1) } }
field!(UNIPERIF_CONFIG_SUBFRAME_SEL_SHIFT, UNIPERIF_CONFIG_SUBFRAME_SEL_MASK, GET_UNIPERIF_CONFIG_SUBFRAME_SEL, SET_UNIPERIF_CONFIG_SUBFRAME_SEL, 19, 0x1);
pub unsafe fn SET_UNIPERIF_CONFIG_SUBFRAME_SEL_SUBF1_SUBF0(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_SUBFRAME_SEL(ip, 1) } }
pub unsafe fn SET_UNIPERIF_CONFIG_SUBFRAME_SEL_SUBF0_SUBF1(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_SUBFRAME_SEL(ip, 0) } }
field!(UNIPERIF_CONFIG_SPDIF_SW_CTRL_SHIFT, UNIPERIF_CONFIG_SPDIF_SW_CTRL_MASK, GET_UNIPERIF_CONFIG_SPDIF_SW_CTRL, SET_UNIPERIF_CONFIG_SPDIF_SW_CTRL, 20, 0x1);
pub unsafe fn SET_UNIPERIF_CONFIG_SPDIF_SW_CTRL_ENABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_SPDIF_SW_CTRL(ip, 1) } }
pub unsafe fn SET_UNIPERIF_CONFIG_SPDIF_SW_CTRL_DISABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CONFIG_SPDIF_SW_CTRL(ip, 0) } }
pub unsafe fn UNIPERIF_CONFIG_MSTR_CLKEDGE_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 24 } else { -1 } } }
pub const fn UNIPERIF_CONFIG_MSTR_CLKEDGE_MASK(_: *const uniperif) -> u32 { 0x1 }
pub unsafe fn GET_UNIPERIF_CONFIG_MSTR_CLKEDGE(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_CONFIG_OFFSET(ip), UNIPERIF_CONFIG_MSTR_CLKEDGE_SHIFT(ip), UNIPERIF_CONFIG_MSTR_CLKEDGE_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_CONFIG_MSTR_CLKEDGE_FALLING(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CONFIG_OFFSET(ip), UNIPERIF_CONFIG_MSTR_CLKEDGE_SHIFT(ip), UNIPERIF_CONFIG_MSTR_CLKEDGE_MASK(ip), 1) } }
pub unsafe fn SET_UNIPERIF_CONFIG_MSTR_CLKEDGE_RISING(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CONFIG_OFFSET(ip), UNIPERIF_CONFIG_MSTR_CLKEDGE_SHIFT(ip), UNIPERIF_CONFIG_MSTR_CLKEDGE_MASK(ip), 0) } }

/*
 * UNIPERIF_CTRL reg
 */
simple_reg!(UNIPERIF_CTRL_OFFSET, GET_UNIPERIF_CTRL, SET_UNIPERIF_CTRL, 0x0044);
macro_rules! ctrl_field {
    ($s:ident, $m:ident, $get:ident, $set:ident, $shift:expr, $mask:expr) => {
        pub const fn $s(_: *const uniperif) -> c_int { $shift }
        pub const fn $m(_: *const uniperif) -> u32 { $mask }
        pub unsafe fn $get(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), $s(ip), $m(ip)) } }
        pub unsafe fn $set(ip: *mut uniperif, value: u32) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), $s(ip), $m(ip), value) } }
    };
}
ctrl_field!(UNIPERIF_CTRL_OPERATION_SHIFT, UNIPERIF_CTRL_OPERATION_MASK, GET_UNIPERIF_CTRL_OPERATION, SET_UNIPERIF_CTRL_OPERATION, 0, 0x7);
pub const fn VALUE_UNIPERIF_CTRL_OPERATION_OFF(_: *const uniperif) -> u32 { 0 }
pub unsafe fn VALUE_UNIPERIF_CTRL_OPERATION_MUTE_PCM_NULL(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 1 } else { -1 } } }
pub unsafe fn VALUE_UNIPERIF_CTRL_OPERATION_MUTE_PAUSE_BURST(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 2 } else { -1 } } }
pub const fn VALUE_UNIPERIF_CTRL_OPERATION_PCM_DATA(_: *const uniperif) -> u32 { 3 }
/* This is the same as above! */
pub const fn VALUE_UNIPERIF_CTRL_OPERATION_AUDIO_DATA(_: *const uniperif) -> u32 { 3 }
pub const fn VALUE_UNIPERIF_CTRL_OPERATION_ENC_DATA(_: *const uniperif) -> u32 { 4 }
pub unsafe fn VALUE_UNIPERIF_CTRL_OPERATION_CD_DATA(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 5 } else { -1 } } }
pub unsafe fn VALUE_UNIPERIF_CTRL_OPERATION_STANDBY(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { -1 } else { 7 } } }
pub unsafe fn SET_UNIPERIF_CTRL_OPERATION_OFF(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_OPERATION(ip, VALUE_UNIPERIF_CTRL_OPERATION_OFF(ip)) } }
pub unsafe fn SET_UNIPERIF_CTRL_OPERATION_MUTE_PCM_NULL(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_OPERATION(ip, VALUE_UNIPERIF_CTRL_OPERATION_MUTE_PCM_NULL(ip) as u32) } }
pub unsafe fn SET_UNIPERIF_CTRL_OPERATION_MUTE_PAUSE_BURST(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_OPERATION(ip, VALUE_UNIPERIF_CTRL_OPERATION_MUTE_PAUSE_BURST(ip) as u32) } }
pub unsafe fn SET_UNIPERIF_CTRL_OPERATION_PCM_DATA(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_OPERATION(ip, VALUE_UNIPERIF_CTRL_OPERATION_PCM_DATA(ip)) } }
pub unsafe fn SET_UNIPERIF_CTRL_OPERATION_AUDIO_DATA(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_OPERATION(ip, VALUE_UNIPERIF_CTRL_OPERATION_AUDIO_DATA(ip)) } }
pub unsafe fn SET_UNIPERIF_CTRL_OPERATION_ENC_DATA(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_OPERATION(ip, VALUE_UNIPERIF_CTRL_OPERATION_ENC_DATA(ip)) } }
pub unsafe fn SET_UNIPERIF_CTRL_OPERATION_CD_DATA(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_OPERATION(ip, VALUE_UNIPERIF_CTRL_OPERATION_CD_DATA(ip) as u32) } }
pub unsafe fn SET_UNIPERIF_CTRL_OPERATION_STANDBY(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_OPERATION(ip, VALUE_UNIPERIF_CTRL_OPERATION_STANDBY(ip) as u32) } }

pub unsafe fn versioned_shift(ip: *const uniperif, old: c_int, new_: c_int) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { old } else { new_ } } }
pub unsafe fn UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_SHIFT(ip: *const uniperif) -> c_int { unsafe { versioned_shift(ip, -1, 3) } }
pub const fn UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_MASK(_: *const uniperif) -> u32 { 0x1 }
pub unsafe fn GET_UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_SHIFT(ip), UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_OFF(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_SHIFT(ip), UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_MASK(ip), 0) } }
pub unsafe fn SET_UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_ON(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_SHIFT(ip), UNIPERIF_CTRL_EXIT_STBY_ON_EOBLOCK_MASK(ip), 1) } }
ctrl_field!(UNIPERIF_CTRL_ROUNDING_SHIFT, UNIPERIF_CTRL_ROUNDING_MASK, GET_UNIPERIF_CTRL_ROUNDING, SET_UNIPERIF_CTRL_ROUNDING, 4, 0x1);
pub unsafe fn SET_UNIPERIF_CTRL_ROUNDING_OFF(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_ROUNDING(ip, 0) } }
pub unsafe fn SET_UNIPERIF_CTRL_ROUNDING_ON(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_ROUNDING(ip, 1) } }
ctrl_field!(UNIPERIF_CTRL_DIVIDER_SHIFT, UNIPERIF_CTRL_DIVIDER_MASK, GET_UNIPERIF_CTRL_DIVIDER, SET_UNIPERIF_CTRL_DIVIDER, 5, 0xff);
macro_rules! ctrl_versioned_bit {
    ($shift:ident, $mask:ident, $get:ident, $off:expr, $old:expr, $new:expr) => {
        pub unsafe fn $shift(ip: *const uniperif) -> c_int { unsafe { versioned_shift(ip, $old, $new) } }
        pub const fn $mask(_: *const uniperif) -> u32 { 0x1 }
        pub unsafe fn $get(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), $shift(ip), $mask(ip)) } }
    };
}
ctrl_versioned_bit!(UNIPERIF_CTRL_BYTE_SWP_SHIFT, UNIPERIF_CTRL_BYTE_SWP_MASK, GET_UNIPERIF_CTRL_BYTE_SWP, 0, 13, -1);
pub unsafe fn SET_UNIPERIF_CTRL_BYTE_SWP_OFF(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), UNIPERIF_CTRL_BYTE_SWP_SHIFT(ip), UNIPERIF_CTRL_BYTE_SWP_MASK(ip), 0) } }
pub unsafe fn SET_UNIPERIF_CTRL_BYTE_SWP_ON(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), UNIPERIF_CTRL_BYTE_SWP_SHIFT(ip), UNIPERIF_CTRL_BYTE_SWP_MASK(ip), 1) } }
ctrl_versioned_bit!(UNIPERIF_CTRL_ZERO_STUFF_SHIFT, UNIPERIF_CTRL_ZERO_STUFF_MASK, GET_UNIPERIF_CTRL_ZERO_STUFF, 0, 14, -1);
pub unsafe fn SET_UNIPERIF_CTRL_ZERO_STUFF_HW(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), UNIPERIF_CTRL_ZERO_STUFF_SHIFT(ip), UNIPERIF_CTRL_ZERO_STUFF_MASK(ip), 1) } }
pub unsafe fn SET_UNIPERIF_CTRL_ZERO_STUFF_SW(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), UNIPERIF_CTRL_ZERO_STUFF_SHIFT(ip), UNIPERIF_CTRL_ZERO_STUFF_MASK(ip), 0) } }
ctrl_versioned_bit!(UNIPERIF_CTRL_SPDIF_LAT_SHIFT, UNIPERIF_CTRL_SPDIF_LAT_MASK, GET_UNIPERIF_CTRL_SPDIF_LAT, 0, 16, -1);
pub unsafe fn SET_UNIPERIF_CTRL_SPDIF_LAT_ON(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), UNIPERIF_CTRL_SPDIF_LAT_SHIFT(ip), UNIPERIF_CTRL_SPDIF_LAT_MASK(ip), 1) } }
pub unsafe fn SET_UNIPERIF_CTRL_SPDIF_LAT_OFF(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), UNIPERIF_CTRL_SPDIF_LAT_SHIFT(ip), UNIPERIF_CTRL_SPDIF_LAT_MASK(ip), 0) } }
ctrl_field!(UNIPERIF_CTRL_SPDIF_FMT_SHIFT, UNIPERIF_CTRL_SPDIF_FMT_MASK, GET_UNIPERIF_CTRL_SPDIF_FMT, SET_UNIPERIF_CTRL_SPDIF_FMT, 17, 0x1);
pub unsafe fn SET_UNIPERIF_CTRL_SPDIF_FMT_ON(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_SPDIF_FMT(ip, 1) } }
pub unsafe fn SET_UNIPERIF_CTRL_SPDIF_FMT_OFF(ip: *mut uniperif) { unsafe { SET_UNIPERIF_CTRL_SPDIF_FMT(ip, 0) } }
ctrl_versioned_bit!(UNIPERIF_CTRL_READER_OUT_SEL_SHIFT, UNIPERIF_CTRL_READER_OUT_SEL_MASK, GET_UNIPERIF_CTRL_READER_OUT_SEL, 0, 18, -1);
pub unsafe fn SET_UNIPERIF_CTRL_READER_OUT_SEL_IN_MEM(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), UNIPERIF_CTRL_READER_OUT_SEL_SHIFT(ip), UNIPERIF_CTRL_READER_OUT_SEL_MASK(ip), 0) } }
pub unsafe fn SET_UNIPERIF_CTRL_READER_OUT_SEL_ON_I2S_LINE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_CTRL_OFFSET(ip), UNIPERIF_CTRL_READER_OUT_SEL_SHIFT(ip), UNIPERIF_CTRL_READER_OUT_SEL_MASK(ip), 1) } }
ctrl_field!(UNIPERIF_CTRL_UNDERFLOW_REC_WINDOW_SHIFT, UNIPERIF_CTRL_UNDERFLOW_REC_WINDOW_MASK, GET_UNIPERIF_CTRL_UNDERFLOW_REC_WINDOW, SET_UNIPERIF_CTRL_UNDERFLOW_REC_WINDOW, 20, 0xff);

/*
 * UNIPERIF_I2S_FMT a.k.a UNIPERIF_FORMAT reg
 */
simple_reg!(UNIPERIF_I2S_FMT_OFFSET, GET_UNIPERIF_I2S_FMT, SET_UNIPERIF_I2S_FMT, 0x0048);
macro_rules! i2s_field {
    ($s:ident, $m:ident, $get:ident, $set:ident, $shift:expr, $mask:expr) => {
        pub const fn $s(_: *const uniperif) -> c_int { $shift }
        pub const fn $m(_: *const uniperif) -> u32 { $mask }
        pub unsafe fn $get(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_I2S_FMT_OFFSET(ip), $s(ip), $m(ip)) } }
        pub unsafe fn $set(ip: *mut uniperif, value: u32) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_I2S_FMT_OFFSET(ip), $s(ip), $m(ip), value) } }
    };
}
i2s_field!(UNIPERIF_I2S_FMT_NBIT_SHIFT, UNIPERIF_I2S_FMT_NBIT_MASK, GET_UNIPERIF_I2S_FMT_NBIT, SET_UNIPERIF_I2S_FMT_NBIT, 0, 0x1);
pub unsafe fn SET_UNIPERIF_I2S_FMT_NBIT_32(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_NBIT(ip, 0) } }
pub unsafe fn SET_UNIPERIF_I2S_FMT_NBIT_16(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_NBIT(ip, 1) } }
i2s_field!(UNIPERIF_I2S_FMT_DATA_SIZE_SHIFT, UNIPERIF_I2S_FMT_DATA_SIZE_MASK, GET_UNIPERIF_I2S_FMT_DATA_SIZE, SET_UNIPERIF_I2S_FMT_DATA_SIZE, 1, 0x7);
pub unsafe fn SET_UNIPERIF_I2S_FMT_DATA_SIZE_16(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_DATA_SIZE(ip, 0) } }
pub unsafe fn SET_UNIPERIF_I2S_FMT_DATA_SIZE_18(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_DATA_SIZE(ip, 1) } }
pub unsafe fn SET_UNIPERIF_I2S_FMT_DATA_SIZE_20(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_DATA_SIZE(ip, 2) } }
pub unsafe fn SET_UNIPERIF_I2S_FMT_DATA_SIZE_24(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_DATA_SIZE(ip, 3) } }
pub unsafe fn SET_UNIPERIF_I2S_FMTL_DATA_SIZE_28(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_DATA_SIZE(ip, 4) } }
pub unsafe fn SET_UNIPERIF_I2S_FMT_DATA_SIZE_32(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_DATA_SIZE(ip, 5) } }
i2s_field!(UNIPERIF_I2S_FMT_LR_POL_SHIFT, UNIPERIF_I2S_FMT_LR_POL_MASK, GET_UNIPERIF_I2S_FMT_LR_POL, SET_UNIPERIF_I2S_FMT_LR_POL, 4, 0x1);
pub const fn VALUE_UNIPERIF_I2S_FMT_LR_POL_LOW(_: *const uniperif) -> u32 { 0x0 }
pub const fn VALUE_UNIPERIF_I2S_FMT_LR_POL_HIG(_: *const uniperif) -> u32 { 0x1 }
pub unsafe fn SET_UNIPERIF_I2S_FMT_LR_POL_LOW(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_LR_POL(ip, VALUE_UNIPERIF_I2S_FMT_LR_POL_LOW(ip)) } }
pub unsafe fn SET_UNIPERIF_I2S_FMT_LR_POL_HIG(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_LR_POL(ip, VALUE_UNIPERIF_I2S_FMT_LR_POL_HIG(ip)) } }
i2s_field!(UNIPERIF_I2S_FMT_SCLK_EDGE_SHIFT, UNIPERIF_I2S_FMT_SCLK_EDGE_MASK, GET_UNIPERIF_I2S_FMT_SCLK_EDGE, SET_UNIPERIF_I2S_FMT_SCLK_EDGE, 5, 0x1);
pub unsafe fn SET_UNIPERIF_I2S_FMT_SCLK_EDGE_RISING(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_SCLK_EDGE(ip, 0) } }
pub unsafe fn SET_UNIPERIF_I2S_FMT_SCLK_EDGE_FALLING(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_SCLK_EDGE(ip, 1) } }
i2s_field!(UNIPERIF_I2S_FMT_PADDING_SHIFT, UNIPERIF_I2S_FMT_PADDING_MASK, GET_UNIPERIF_I2S_FMT_PADDING, SET_UNIPERIF_I2S_FMT_PADDING, 6, 0x1);
pub const fn VALUE_UNIPERIF_I2S_FMT_PADDING_I2S_MODE(_: *const uniperif) -> u32 { 0x0 }
pub const fn VALUE_UNIPERIF_I2S_FMT_PADDING_SONY_MODE(_: *const uniperif) -> u32 { 0x1 }
pub unsafe fn SET_UNIPERIF_I2S_FMT_PADDING_I2S_MODE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_PADDING(ip, VALUE_UNIPERIF_I2S_FMT_PADDING_I2S_MODE(ip)) } }
pub unsafe fn SET_UNIPERIF_I2S_FMT_PADDING_SONY_MODE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_PADDING(ip, VALUE_UNIPERIF_I2S_FMT_PADDING_SONY_MODE(ip)) } }
i2s_field!(UNIPERIF_I2S_FMT_ALIGN_SHIFT, UNIPERIF_I2S_FMT_ALIGN_MASK, GET_UNIPERIF_I2S_FMT_ALIGN, SET_UNIPERIF_I2S_FMT_ALIGN, 7, 0x1);
pub unsafe fn SET_UNIPERIF_I2S_FMT_ALIGN_LEFT(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_ALIGN(ip, 0) } }
pub unsafe fn SET_UNIPERIF_I2S_FMT_ALIGN_RIGHT(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_ALIGN(ip, 1) } }
i2s_field!(UNIPERIF_I2S_FMT_ORDER_SHIFT, UNIPERIF_I2S_FMT_ORDER_MASK, GET_UNIPERIF_I2S_FMT_ORDER, SET_UNIPERIF_I2S_FMT_ORDER, 8, 0x1);
pub unsafe fn SET_UNIPERIF_I2S_FMT_ORDER_LSB(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_ORDER(ip, 0) } }
pub unsafe fn SET_UNIPERIF_I2S_FMT_ORDER_MSB(ip: *mut uniperif) { unsafe { SET_UNIPERIF_I2S_FMT_ORDER(ip, 1) } }
i2s_field!(UNIPERIF_I2S_FMT_NUM_CH_SHIFT, UNIPERIF_I2S_FMT_NUM_CH_MASK, GET_UNIPERIF_I2S_FMT_NUM_CH, SET_UNIPERIF_I2S_FMT_NUM_CH, 9, 0x7);
i2s_field!(UNIPERIF_I2S_FMT_NO_OF_SAMPLES_TO_READ_SHIFT, UNIPERIF_I2S_FMT_NO_OF_SAMPLES_TO_READ_MASK, GET_UNIPERIF_I2S_FMT_NO_OF_SAMPLES_TO_READ, SET_UNIPERIF_I2S_FMT_NO_OF_SAMPLES_TO_READ, 12, 0xfffff);

/*
 * UNIPERIF_BIT_CONTROL reg
 */
pub unsafe fn UNIPERIF_BIT_CONTROL_OFFSET(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { -1 } else { 0x004c } } }
pub unsafe fn GET_UNIPERIF_BIT_CONTROL(ip: *mut uniperif) -> u32 { unsafe { readl_relaxed(reg_addr(ip, UNIPERIF_BIT_CONTROL_OFFSET(ip))) } }
pub unsafe fn SET_UNIPERIF_BIT_CONTROL(ip: *mut uniperif, value: u32) { unsafe { writel_relaxed(value, reg_addr(ip, UNIPERIF_BIT_CONTROL_OFFSET(ip))) } }
pub const fn UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION_SHIFT(_: *const uniperif) -> c_int { 0 }
pub const fn UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION_MASK(_: *const uniperif) -> u32 { 0x1 }
pub unsafe fn GET_UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_BIT_CONTROL_OFFSET(ip), UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION_SHIFT(ip), UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_BIT_CONTROL_OFFSET(ip), UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION_SHIFT(ip), UNIPERIF_BIT_CONTROL_CLR_UNDERFLOW_DURATION_MASK(ip), 1) } }
pub const fn UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE_SHIFT(_: *const uniperif) -> c_int { 1 }
pub const fn UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE_MASK(_: *const uniperif) -> u32 { 0x1 }
pub unsafe fn GET_UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_BIT_CONTROL_OFFSET(ip), UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE_SHIFT(ip), UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_BIT_REG(ip, UNIPERIF_BIT_CONTROL_OFFSET(ip), UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE_SHIFT(ip), UNIPERIF_BIT_CONTROL_CHL_STS_UPDATE_MASK(ip), 1) } }

/*
 * UNIPERIF_STATUS_1 reg
 */
simple_reg!(UNIPERIF_STATUS_1_OFFSET, GET_UNIPERIF_STATUS_1, SET_UNIPERIF_STATUS_1, 0x0050);
pub unsafe fn UNIPERIF_STATUS_1_UNDERFLOW_DURATION_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { -1 } else { 0 } } }
pub const fn UNIPERIF_STATUS_1_UNDERFLOW_DURATION_MASK(_: *const uniperif) -> u32 { 0xff }
pub unsafe fn GET_UNIPERIF_STATUS_1_UNDERFLOW_DURATION(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_STATUS_1_OFFSET(ip), UNIPERIF_STATUS_1_UNDERFLOW_DURATION_SHIFT(ip), UNIPERIF_STATUS_1_UNDERFLOW_DURATION_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_STATUS_1_UNDERFLOW_DURATION(ip: *mut uniperif, value: u32) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_STATUS_1_OFFSET(ip), UNIPERIF_STATUS_1_UNDERFLOW_DURATION_SHIFT(ip), UNIPERIF_STATUS_1_UNDERFLOW_DURATION_MASK(ip), value) } }

/*
 * UNIPERIF_USER_VALIDITY reg
 */
simple_reg!(UNIPERIF_USER_VALIDITY_OFFSET, GET_UNIPERIF_USER_VALIDITY, SET_UNIPERIF_USER_VALIDITY, 0x0090);
pub const fn UNIPERIF_USER_VALIDITY_VALIDITY_LR_SHIFT(_: *const uniperif) -> c_int { 0 }
pub const fn UNIPERIF_USER_VALIDITY_VALIDITY_LR_MASK(_: *const uniperif) -> u32 { 0x3 }
pub unsafe fn GET_UNIPERIF_USER_VALIDITY_VALIDITY_LR(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_USER_VALIDITY_OFFSET(ip), UNIPERIF_USER_VALIDITY_VALIDITY_LR_SHIFT(ip), UNIPERIF_USER_VALIDITY_VALIDITY_LR_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_USER_VALIDITY_VALIDITY_LR(ip: *mut uniperif, value: c_int) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_USER_VALIDITY_OFFSET(ip), UNIPERIF_USER_VALIDITY_VALIDITY_LR_SHIFT(ip), UNIPERIF_USER_VALIDITY_VALIDITY_LR_MASK(ip), if value != 0 { 0x3 } else { 0 }) } }

/*
 * UNIPERIF_DBG_STANDBY_LEFT_SP reg
 */
pub const fn UNIPERIF_DBG_STANDBY_LEFT_SP_OFFSET(_: *const uniperif) -> c_int { 0x0150 }
pub unsafe fn UNIPERIF_DBG_STANDBY_LEFT_SP_SHIFT(ip: *const uniperif) -> c_int { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { -1 } else { 0 } } }
pub unsafe fn UNIPERIF_DBG_STANDBY_LEFT_SP_MASK(ip: *const uniperif) -> u32 { unsafe { if (*ip).ver < uniperif_version::SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0 as c_int { 0 } else { 0xFFFFFF } } }
pub unsafe fn GET_UNIPERIF_DBG_STANDBY_LEFT_SP(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_DBG_STANDBY_LEFT_SP_OFFSET(ip), UNIPERIF_DBG_STANDBY_LEFT_SP_SHIFT(ip), UNIPERIF_DBG_STANDBY_LEFT_SP_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_DBG_STANDBY_LEFT_SP(ip: *mut uniperif, value: u32) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_DBG_STANDBY_LEFT_SP_OFFSET(ip), UNIPERIF_DBG_STANDBY_LEFT_SP_SHIFT(ip), UNIPERIF_DBG_STANDBY_LEFT_SP_MASK(ip), value) } }

/*
 * UNIPERIF_TDM_ENABLE
 */
simple_reg!(UNIPERIF_TDM_ENABLE_OFFSET, GET_UNIPERIF_TDM_ENABLE, SET_UNIPERIF_TDM_ENABLE, 0x0118);
pub const fn UNIPERIF_TDM_ENABLE_EN_TDM_SHIFT(_: *const uniperif) -> c_int { 0x0 }
pub const fn UNIPERIF_TDM_ENABLE_EN_TDM_MASK(_: *const uniperif) -> u32 { 0x1 }
pub unsafe fn GET_UNIPERIF_TDM_ENABLE_EN_TDM(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_TDM_ENABLE_OFFSET(ip), UNIPERIF_TDM_ENABLE_EN_TDM_SHIFT(ip), UNIPERIF_TDM_ENABLE_EN_TDM_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_TDM_ENABLE_TDM_ENABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_TDM_ENABLE_OFFSET(ip), UNIPERIF_TDM_ENABLE_EN_TDM_SHIFT(ip), UNIPERIF_TDM_ENABLE_EN_TDM_MASK(ip), 1) } }
pub unsafe fn SET_UNIPERIF_TDM_ENABLE_TDM_DISABLE(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_TDM_ENABLE_OFFSET(ip), UNIPERIF_TDM_ENABLE_EN_TDM_SHIFT(ip), UNIPERIF_TDM_ENABLE_EN_TDM_MASK(ip), 0) } }

/*
 * UNIPERIF_TDM_FS_REF_FREQ
 */
simple_reg!(UNIPERIF_TDM_FS_REF_FREQ_OFFSET, GET_UNIPERIF_TDM_FS_REF_FREQ, SET_UNIPERIF_TDM_FS_REF_FREQ, 0x011c);
pub const fn UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_SHIFT(_: *const uniperif) -> c_int { 0x0 }
pub const fn VALUE_UNIPERIF_TDM_FS_REF_FREQ_8KHZ(_: *const uniperif) -> u32 { 0 }
pub const fn VALUE_UNIPERIF_TDM_FS_REF_FREQ_16KHZ(_: *const uniperif) -> u32 { 1 }
pub const fn VALUE_UNIPERIF_TDM_FS_REF_FREQ_32KHZ(_: *const uniperif) -> u32 { 2 }
pub const fn VALUE_UNIPERIF_TDM_FS_REF_FREQ_48KHZ(_: *const uniperif) -> u32 { 3 }
pub const fn UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_MASK(_: *const uniperif) -> u32 { 0x3 }
pub unsafe fn GET_UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_TDM_FS_REF_FREQ_OFFSET(ip), UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_SHIFT(ip), UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_TDM_FS_REF_FREQ_8KHZ(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_TDM_FS_REF_FREQ_OFFSET(ip), UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_SHIFT(ip), UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_MASK(ip), VALUE_UNIPERIF_TDM_FS_REF_FREQ_8KHZ(ip)) } }
pub unsafe fn SET_UNIPERIF_TDM_FS_REF_FREQ_16KHZ(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_TDM_FS_REF_FREQ_OFFSET(ip), UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_SHIFT(ip), UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_MASK(ip), VALUE_UNIPERIF_TDM_FS_REF_FREQ_16KHZ(ip)) } }
pub unsafe fn SET_UNIPERIF_TDM_FS_REF_FREQ_32KHZ(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_TDM_FS_REF_FREQ_OFFSET(ip), UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_SHIFT(ip), UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_MASK(ip), VALUE_UNIPERIF_TDM_FS_REF_FREQ_32KHZ(ip)) } }
pub unsafe fn SET_UNIPERIF_TDM_FS_REF_FREQ_48KHZ(ip: *mut uniperif) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_TDM_FS_REF_FREQ_OFFSET(ip), UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_SHIFT(ip), UNIPERIF_TDM_FS_REF_FREQ_REF_FREQ_MASK(ip), VALUE_UNIPERIF_TDM_FS_REF_FREQ_48KHZ(ip)) } }

/*
 * UNIPERIF_TDM_FS_REF_DIV
 */
simple_reg!(UNIPERIF_TDM_FS_REF_DIV_OFFSET, GET_UNIPERIF_TDM_FS_REF_DIV, SET_UNIPERIF_TDM_FS_REF_DIV, 0x0120);
pub const fn UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT_SHIFT(_: *const uniperif) -> c_int { 0x0 }
pub const fn UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT_MASK(_: *const uniperif) -> u32 { 0xff }
pub unsafe fn GET_UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT(ip: *mut uniperif) -> u32 { unsafe { GET_UNIPERIF_REG(ip, UNIPERIF_TDM_FS_REF_DIV_OFFSET(ip), UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT_SHIFT(ip), UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT_MASK(ip)) } }
pub unsafe fn SET_UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT(ip: *mut uniperif, value: u32) { unsafe { SET_UNIPERIF_REG(ip, UNIPERIF_TDM_FS_REF_DIV_OFFSET(ip), UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT_SHIFT(ip), UNIPERIF_TDM_FS_REF_DIV_NUM_TIMESLOT_MASK(ip), value) } }

/*
 * UNIPERIF_TDM_WORD_POS_X_Y
 * 32 bits of UNIPERIF_TDM_WORD_POS_X_Y register shall be set in 1 shot
 */
pub const fn UNIPERIF_TDM_WORD_POS_1_2_OFFSET(_: *const uniperif) -> c_int { 0x013c }
pub const fn UNIPERIF_TDM_WORD_POS_3_4_OFFSET(_: *const uniperif) -> c_int { 0x0140 }
pub const fn UNIPERIF_TDM_WORD_POS_5_6_OFFSET(_: *const uniperif) -> c_int { 0x0144 }
pub const fn UNIPERIF_TDM_WORD_POS_7_8_OFFSET(_: *const uniperif) -> c_int { 0x0148 }
pub unsafe fn GET_UNIPERIF_TDM_WORD_POS(ip: *mut uniperif, words: uniperif_word_pos) -> u32 {
    unsafe {
        let offset = match words {
            uniperif_word_pos::WORD_1_2 => UNIPERIF_TDM_WORD_POS_1_2_OFFSET(ip),
            uniperif_word_pos::WORD_3_4 => UNIPERIF_TDM_WORD_POS_3_4_OFFSET(ip),
            uniperif_word_pos::WORD_5_6 => UNIPERIF_TDM_WORD_POS_5_6_OFFSET(ip),
            uniperif_word_pos::WORD_7_8 => UNIPERIF_TDM_WORD_POS_7_8_OFFSET(ip),
            uniperif_word_pos::WORD_MAX => UNIPERIF_TDM_WORD_POS_7_8_OFFSET(ip),
        };
        readl_relaxed(reg_addr(ip, offset))
    }
}
pub unsafe fn SET_UNIPERIF_TDM_WORD_POS(ip: *mut uniperif, words: uniperif_word_pos, value: u32) {
    unsafe {
        let offset = match words {
            uniperif_word_pos::WORD_1_2 => UNIPERIF_TDM_WORD_POS_1_2_OFFSET(ip),
            uniperif_word_pos::WORD_3_4 => UNIPERIF_TDM_WORD_POS_3_4_OFFSET(ip),
            uniperif_word_pos::WORD_5_6 => UNIPERIF_TDM_WORD_POS_5_6_OFFSET(ip),
            uniperif_word_pos::WORD_7_8 => UNIPERIF_TDM_WORD_POS_7_8_OFFSET(ip),
            uniperif_word_pos::WORD_MAX => UNIPERIF_TDM_WORD_POS_7_8_OFFSET(ip),
        };
        writel_relaxed(value, reg_addr(ip, offset));
    }
}

/*
 * uniperipheral IP capabilities
 */
pub const UNIPERIF_FIFO_SIZE: c_int = 70; /* FIFO is 70 cells deep */
pub const UNIPERIF_FIFO_FRAMES: c_int = 4; /* FDMA trigger limit in frames */

pub unsafe fn UNIPERIF_TYPE_IS_HDMI(p: *const uniperif) -> bool { unsafe { (*p).type_ == uniperif_type::SND_ST_UNIPERIF_TYPE_HDMI } }
pub unsafe fn UNIPERIF_TYPE_IS_PCM(p: *const uniperif) -> bool { unsafe { (*p).type_ == uniperif_type::SND_ST_UNIPERIF_TYPE_PCM } }
pub unsafe fn UNIPERIF_TYPE_IS_SPDIF(p: *const uniperif) -> bool { unsafe { (*p).type_ == uniperif_type::SND_ST_UNIPERIF_TYPE_SPDIF } }
pub unsafe fn UNIPERIF_TYPE_IS_IEC958(p: *const uniperif) -> bool { unsafe { UNIPERIF_TYPE_IS_HDMI(p) || UNIPERIF_TYPE_IS_SPDIF(p) } }
pub unsafe fn UNIPERIF_TYPE_IS_TDM(p: *const uniperif) -> bool { unsafe { (*p).type_ == uniperif_type::SND_ST_UNIPERIF_TYPE_TDM } }

/*
 * Uniperipheral IP revisions
 */
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum uniperif_version {
    SND_ST_UNIPERIF_VERSION_UNKNOWN,
    /* SASG1 (Orly), Newman */
    SND_ST_UNIPERIF_VERSION_C6AUD0_UNI_1_0,
    /* SASC1, SASG2 (Orly2) */
    SND_ST_UNIPERIF_VERSION_UNI_PLR_1_0,
    /* SASC1, SASG2 (Orly2), TELSS, Cannes */
    SND_ST_UNIPERIF_VERSION_UNI_RDR_1_0,
    /* TELSS (SASC1) */
    SND_ST_UNIPERIF_VERSION_TDM_PLR_1_0,
    /* Cannes/Monaco */
    SND_ST_UNIPERIF_VERSION_UNI_PLR_TOP_1_0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum uniperif_type {
    SND_ST_UNIPERIF_TYPE_NONE = 0x00,
    SND_ST_UNIPERIF_TYPE_HDMI = 0x01,
    SND_ST_UNIPERIF_TYPE_PCM = 0x02,
    SND_ST_UNIPERIF_TYPE_SPDIF = 0x04,
    SND_ST_UNIPERIF_TYPE_TDM = 0x08,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum uniperif_state {
    UNIPERIF_STATE_STOPPED,
    UNIPERIF_STATE_STARTED,
    UNIPERIF_STATE_STANDBY,
    UNIPERIF_STATE_UNDERFLOW,
    UNIPERIF_STATE_XRUN = 4,
}
pub const UNIPERIF_STATE_OVERFLOW: uniperif_state = uniperif_state::UNIPERIF_STATE_UNDERFLOW;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum uniperif_iec958_encoding_mode {
    UNIPERIF_IEC958_ENCODING_MODE_PCM,
    UNIPERIF_IEC958_ENCODING_MODE_ENCODED,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum uniperif_word_pos {
    WORD_1_2,
    WORD_3_4,
    WORD_5_6,
    WORD_7_8,
    WORD_MAX,
}

#[repr(C)]
pub struct uniperif_iec958_settings {
    pub encoding_mode: uniperif_iec958_encoding_mode,
    pub iec958: snd_aes_iec958,
}

#[repr(C)]
pub struct dai_tdm_slot {
    pub mask: c_uint,
    pub slots: c_int,
    pub slot_width: c_int,
    pub avail_slots: c_uint,
}

#[repr(C)]
pub struct uniperif {
    /* System information */
    pub type_: uniperif_type,
    pub underflow_enabled: c_int, /* Underflow recovery mode */
    pub dev: *mut device,
    pub id: c_int,  /* instance value of the uniperipheral IP */
    pub ver: c_int, /* IP version, used by register access macros */
    pub clk_sel: *mut regmap_field,
    pub valid_sel: *mut regmap_field,
    pub irq_lock: spinlock_t, /* use to prevent race condition with IRQ */

    /* capabilities */
    pub hw: *const snd_pcm_hardware,

    /* Resources */
    pub mem_region: *mut resource,
    pub base: *mut c_void,
    pub fifo_phys_address: c_ulong,
    pub irq: c_int,

    /* Clocks */
    pub clk: *mut clk,
    pub mclk: c_int,
    pub clk_adj: c_int,

    /* Runtime data */
    pub state: uniperif_state,
    pub substream: *mut snd_pcm_substream,

    /* Specific to IEC958 player */
    pub stream_settings: uniperif_iec958_settings,
    pub ctrl_lock: mutex, /* For resource updated by stream and controls*/

    /*alsa ctrl*/
    pub snd_ctrls: *mut snd_kcontrol_new,
    pub num_ctrls: c_int,

    /* dai properties */
    pub daifmt: c_uint,
    pub tdm_slot: dai_tdm_slot,

    /* DAI callbacks */
    pub dai_ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct sti_uniperiph_dai {
    pub stream: c_int,
    pub uni: *mut uniperif,
    pub dma_data: snd_dmaengine_dai_dma_data,
}

#[repr(C)]
pub struct sti_uniperiph_data {
    pub pdev: *mut platform_device,
    pub dai: *mut snd_soc_dai_driver,
    pub dai_data: sti_uniperiph_dai,
}

pub static uni_tdm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S32_LE | SNDRV_PCM_FMTBIT_S16_LE,
    rates: SNDRV_PCM_RATE_CONTINUOUS,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 1,
    channels_max: 32,
    periods_min: 2,
    periods_max: 10,
    period_bytes_min: 128,
    period_bytes_max: 64 * PAGE_SIZE,
    buffer_bytes_max: 256 * PAGE_SIZE,
};

unsafe extern "C" {
    /* uniperiph player*/
    pub fn uni_player_init(pdev: *mut platform_device, player: *mut uniperif) -> c_int;
    pub fn uni_player_resume(player: *mut uniperif) -> c_int;

    /* uniperiph reader */
    pub fn uni_reader_init(pdev: *mut platform_device, reader: *mut uniperif) -> c_int;

    /* common */
    pub fn sti_uniperiph_dai_probe(dai: *mut snd_soc_dai) -> c_int;
    pub fn sti_uniperiph_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    pub fn sti_uniperiph_dai_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        dai: *mut snd_soc_dai,
    ) -> c_int;
}

#[inline]
pub unsafe fn sti_uniperiph_get_user_frame_size(runtime: *mut snd_pcm_runtime) -> c_int {
    unsafe { ((*runtime).channels as c_int * snd_pcm_format_width((*runtime).format)) / 8 }
}

#[inline]
pub unsafe fn sti_uniperiph_get_unip_tdm_frame_size(uni: *mut uniperif) -> c_int {
    unsafe { ((*uni).tdm_slot.slots * (*uni).tdm_slot.slot_width) / 8 }
}

unsafe extern "C" {
    pub fn sti_uniperiph_reset(uni: *mut uniperif) -> c_int;
    pub fn sti_uniperiph_set_tdm_slot(
        dai: *mut snd_soc_dai,
        tx_mask: c_uint,
        rx_mask: c_uint,
        slots: c_int,
        slot_width: c_int,
    ) -> c_int;
    pub fn sti_uniperiph_get_tdm_word_pos(uni: *mut uniperif, word_pos: *mut c_uint) -> c_int;
    pub fn sti_uniperiph_fix_tdm_chan(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int;
    pub fn sti_uniperiph_fix_tdm_format(params: *mut snd_pcm_hw_params, rule: *mut snd_pcm_hw_rule) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
