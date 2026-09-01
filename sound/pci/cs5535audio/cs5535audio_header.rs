/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_ulong, c_uint};

pub type u32 = u32;
pub type __le32 = u32;
pub type __le16 = u16;
pub type spinlock_t = c_ulong;

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ac97 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dma_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ac97_template {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn outl(value: c_uint, port: c_ulong);
    fn outb(value: u8, port: c_ulong);
    fn inl(port: c_ulong) -> c_uint;
    fn inw(port: c_ulong) -> u16;
    fn inb(port: c_ulong) -> u8;
}

#[inline]
pub unsafe fn cs_writel(cs5535au: *mut cs5535audio, reg: c_ulong, val: c_uint) {
    unsafe { outl(val, (*cs5535au).port.wrapping_add(reg)) };
}

#[inline]
pub unsafe fn cs_writeb(cs5535au: *mut cs5535audio, reg: c_ulong, val: u8) {
    unsafe { outb(val, (*cs5535au).port.wrapping_add(reg)) };
}

#[inline]
pub unsafe fn cs_readl(cs5535au: *mut cs5535audio, reg: c_ulong) -> c_uint {
    unsafe { inl((*cs5535au).port.wrapping_add(reg)) }
}

#[inline]
pub unsafe fn cs_readw(cs5535au: *mut cs5535audio, reg: c_ulong) -> u16 {
    unsafe { inw((*cs5535au).port.wrapping_add(reg)) }
}

#[inline]
pub unsafe fn cs_readb(cs5535au: *mut cs5535audio, reg: c_ulong) -> u8 {
    unsafe { inb((*cs5535au).port.wrapping_add(reg)) }
}

pub const CS5535AUDIO_MAX_DESCRIPTORS: usize = 128;

/* acc_codec bar0 reg addrs */
pub const ACC_GPIO_STATUS: c_ulong = 0x00;
pub const ACC_CODEC_STATUS: c_ulong = 0x08;
pub const ACC_CODEC_CNTL: c_ulong = 0x0C;
pub const ACC_IRQ_STATUS: c_ulong = 0x12;
pub const ACC_BM0_CMD: c_ulong = 0x20;
pub const ACC_BM1_CMD: c_ulong = 0x28;
pub const ACC_BM0_PRD: c_ulong = 0x24;
pub const ACC_BM1_PRD: c_ulong = 0x2C;
pub const ACC_BM0_STATUS: c_ulong = 0x21;
pub const ACC_BM1_STATUS: c_ulong = 0x29;
pub const ACC_BM0_PNTR: c_ulong = 0x60;
pub const ACC_BM1_PNTR: c_ulong = 0x64;

/* acc_codec bar0 reg bits */
/* ACC_IRQ_STATUS */
pub const IRQ_STS: c_int = 0;
pub const WU_IRQ_STS: c_int = 1;
pub const BM0_IRQ_STS: c_int = 2;
pub const BM1_IRQ_STS: c_int = 3;
/* ACC_BMX_STATUS */
pub const EOP: c_uint = 1 << 0;
pub const BM_EOP_ERR: c_uint = 1 << 1;
/* ACC_BMX_CTL */
pub const BM_CTL_EN: c_uint = 0x01;
pub const BM_CTL_PAUSE: c_uint = 0x03;
pub const BM_CTL_DIS: c_uint = 0x00;
pub const BM_CTL_BYTE_ORD_LE: c_uint = 0x00;
pub const BM_CTL_BYTE_ORD_BE: c_uint = 0x04;
/* cs5535 specific ac97 codec register defines */
pub const CMD_MASK: c_uint = 0xFF00FFFF;
pub const CMD_NEW: c_uint = 0x00010000;
pub const STS_NEW: c_uint = 0x00020000;
pub const PRM_RDY_STS: c_uint = 0x00800000;
pub const ACC_CODEC_CNTL_WR_CMD: c_uint = !0x80000000u32;
pub const ACC_CODEC_CNTL_RD_CMD: c_uint = 0x80000000;
pub const ACC_CODEC_CNTL_LNK_SHUTDOWN: c_uint = 0x00040000;
pub const ACC_CODEC_CNTL_LNK_WRM_RST: c_uint = 0x00020000;
pub const PRD_JMP: c_uint = 0x2000;
pub const PRD_EOP: c_uint = 0x4000;
pub const PRD_EOT: c_uint = 0x8000;

pub const CS5535AUDIO_DMA_PLAYBACK: c_int = 0;
pub const CS5535AUDIO_DMA_CAPTURE: c_int = 1;
pub const NUM_CS5535AUDIO_DMAS: usize = 2;

#[repr(C)]
pub struct cs5535audio_dma_ops {
    pub type_: c_int,
    pub enable_dma: Option<unsafe extern "C" fn(cs5535au: *mut cs5535audio)>,
    pub disable_dma: Option<unsafe extern "C" fn(cs5535au: *mut cs5535audio)>,
    pub pause_dma: Option<unsafe extern "C" fn(cs5535au: *mut cs5535audio)>,
    pub setup_prd: Option<unsafe extern "C" fn(cs5535au: *mut cs5535audio, prd_addr: u32)>,
    pub read_prd: Option<unsafe extern "C" fn(cs5535au: *mut cs5535audio) -> u32>,
    pub read_dma_pntr: Option<unsafe extern "C" fn(cs5535au: *mut cs5535audio) -> u32>,
}

#[repr(C)]
pub struct cs5535audio_dma_desc {
    pub addr: __le32,
    pub size: __le16,
    pub ctlreserved: __le16,
}

#[repr(C)]
pub struct cs5535audio_dma {
    pub ops: *const cs5535audio_dma_ops,
    pub desc_buf: snd_dma_buffer,
    pub substream: *mut snd_pcm_substream,
    pub buf_addr: c_uint,
    pub buf_bytes: c_uint,
    pub period_bytes: c_uint,
    pub periods: c_uint,
    pub saved_prd: u32,
    pub pcm_open_flag: c_int,
}

#[repr(C)]
pub struct cs5535audio {
    pub card: *mut snd_card,
    pub ac97: *mut snd_ac97,
    pub pcm: *mut snd_pcm,
    pub irq: c_int,
    pub pci: *mut pci_dev,
    pub port: c_ulong,
    pub reg_lock: spinlock_t,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub dmas: [cs5535audio_dma; NUM_CS5535AUDIO_DMAS],
}

unsafe extern "C" {
    pub static snd_cs5535audio_pm: dev_pm_ops;
}

/*
 * CONFIG_OLPC conditional:
 * when enabled, these functions are provided externally and capture open/close
 * call olpc_analog_input and olpc_mic_bias as below; otherwise they are inline
 * no-op helpers except olpc_quirks, which returns 0.
 */
unsafe extern "C" {
    pub fn olpc_prequirks(card: *mut snd_card, ac97: *mut snd_ac97_template);
    pub fn olpc_quirks(card: *mut snd_card, ac97: *mut snd_ac97) -> c_int;
    pub fn olpc_quirks_cleanup();
    pub fn olpc_analog_input(ac97: *mut snd_ac97, on: c_int);
    pub fn olpc_mic_bias(ac97: *mut snd_ac97, on: c_int);
}

#[inline]
pub unsafe fn olpc_capture_open(ac97: *mut snd_ac97) {
    /* default to Analog Input off */
    unsafe { olpc_analog_input(ac97, 0) };
    /* enable MIC Bias for recording */
    unsafe { olpc_mic_bias(ac97, 1) };
}

#[inline]
pub unsafe fn olpc_capture_close(ac97: *mut snd_ac97) {
    /* disable Analog Input */
    unsafe { olpc_analog_input(ac97, 0) };
    /* disable the MIC Bias (so the recording LED turns off) */
    unsafe { olpc_mic_bias(ac97, 0) };
}

unsafe extern "C" {
    pub fn snd_cs5535audio_pcm(cs5535audio: *mut cs5535audio) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
