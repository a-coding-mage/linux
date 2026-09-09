/* SPDX-License-Identifier: GPL-2.0-only */
/* Driver for the TXx9 SoC DMA Controller */

// C dependencies: linux::dmaengine and asm::txx9::dmac provide the referenced types/constants.

#[inline]
pub fn txx9_dma_have_SMPCHN() -> bool {
    cfg!(feature = "CONFIG_MACH_TX49XX")
}

#[cfg(feature = "CONFIG_MACH_TX49XX")]
pub const TXX9_DMA_USE_SIMPLE_CHAIN: bool = true;

#[cfg(all(feature = "__LITTLE_ENDIAN", feature = "CONFIG_MACH_TX49XX"))]
pub const CCR_LE: u32 = TXX9_DMA_CCR_LE;
#[cfg(all(feature = "__LITTLE_ENDIAN", feature = "CONFIG_MACH_TX49XX"))]
pub const MCR_LE: u32 = 0;
#[cfg(all(feature = "__LITTLE_ENDIAN", not(feature = "CONFIG_MACH_TX49XX")))]
pub const CCR_LE: u32 = 0;
#[cfg(all(feature = "__LITTLE_ENDIAN", not(feature = "CONFIG_MACH_TX49XX")))]
pub const MCR_LE: u32 = TXX9_DMA_MCR_LE;
#[cfg(not(feature = "__LITTLE_ENDIAN"))]
pub const CCR_LE: u32 = 0;
#[cfg(not(feature = "__LITTLE_ENDIAN"))]
pub const MCR_LE: u32 = 0;

#[repr(C)]
pub struct txx9dmac_cregs {
    pub CHAR: u64,
    pub SAR: u64,
    pub DAR: u64,
    pub CNTR: u32,
    pub __pad_CNTR: u32,
    pub SAIR: u32,
    pub __pad_SAIR: u32,
    pub DAIR: u32,
    pub __pad_DAIR: u32,
    pub CCR: u32,
    pub __pad_CCR: u32,
    pub CSR: u32,
    pub __pad_CSR: u32,
}

#[repr(C)]
pub struct txx9dmac_cregs32 { pub CHAR: u32, pub SAR: u32, pub DAR: u32, pub CNTR: u32, pub SAIR: u32, pub DAIR: u32, pub CCR: u32, pub CSR: u32 }

#[repr(C)]
pub struct txx9dmac_regs {
    pub CHAN: [txx9dmac_cregs; TXX9_DMA_MAX_NR_CHANNELS as usize],
    pub __pad: [u64; 9],
    pub MFDR: u64,
    pub MCR: u32,
    pub __pad_MCR: u32,
}
#[repr(C)]
pub struct txx9dmac_regs32 {
    pub CHAN: [txx9dmac_cregs32; TXX9_DMA_MAX_NR_CHANNELS as usize],
    pub __pad: [u32; 9], pub MFDR: u32, pub MCR: u32,
}

pub const fn TXX9_DMA_MCR_EIS(ch: u32) -> u32 { 0x10000000u32 << ch }
pub const fn TXX9_DMA_MCR_DIS(ch: u32) -> u32 { 0x01000000u32 << ch }
pub const TXX9_DMA_MCR_RSFIF: u32 = 0x00000080;
pub const fn TXX9_DMA_MCR_FIFUM(ch: u32) -> u32 { 0x00000008u32 << ch }
pub const TXX9_DMA_MCR_LE: u32 = 0x00000004;
pub const TXX9_DMA_MCR_RPRT: u32 = 0x00000002;
pub const TXX9_DMA_MCR_MSTEN: u32 = 0x00000001;

pub const TXX9_DMA_CCR_IMMCHN: u32 = 0x20000000;
pub const TXX9_DMA_CCR_USEXFSZ: u32 = 0x10000000;
pub const TXX9_DMA_CCR_LE: u32 = 0x08000000;
pub const TXX9_DMA_CCR_DBINH: u32 = 0x04000000;
pub const TXX9_DMA_CCR_SBINH: u32 = 0x02000000;
pub const TXX9_DMA_CCR_CHRST: u32 = 0x01000000;
pub const TXX9_DMA_CCR_RVBYTE: u32 = 0x00800000;
pub const TXX9_DMA_CCR_ACKPOL: u32 = 0x00400000;
pub const TXX9_DMA_CCR_REQPL: u32 = 0x00200000;
pub const TXX9_DMA_CCR_EGREQ: u32 = 0x00100000;
pub const TXX9_DMA_CCR_CHDN: u32 = 0x00080000;
pub const TXX9_DMA_CCR_DNCTL: u32 = 0x00060000;
pub const TXX9_DMA_CCR_EXTRQ: u32 = 0x00010000;
pub const TXX9_DMA_CCR_INTRQD: u32 = 0x0000e000;
pub const TXX9_DMA_CCR_INTENE: u32 = 0x00001000;
pub const TXX9_DMA_CCR_INTENC: u32 = 0x00000800;
pub const TXX9_DMA_CCR_INTENT: u32 = 0x00000400;
pub const TXX9_DMA_CCR_CHNEN: u32 = 0x00000200;
pub const TXX9_DMA_CCR_XFACT: u32 = 0x00000100;
pub const TXX9_DMA_CCR_SMPCHN: u32 = 0x00000020;
pub const fn TXX9_DMA_CCR_XFSZ(order: u32) -> u32 { (order << 2) & 0x0000001c }
pub const TXX9_DMA_CCR_XFSZ_1: u32 = 0;
pub const TXX9_DMA_CCR_XFSZ_2: u32 = 4;
pub const TXX9_DMA_CCR_XFSZ_4: u32 = 8;
pub const TXX9_DMA_CCR_XFSZ_8: u32 = 12;
pub const TXX9_DMA_CCR_XFSZ_X4: u32 = 16;
pub const TXX9_DMA_CCR_XFSZ_X8: u32 = 20;
pub const TXX9_DMA_CCR_XFSZ_X16: u32 = 24;
pub const TXX9_DMA_CCR_XFSZ_X32: u32 = 28;
pub const TXX9_DMA_CCR_MEMIO: u32 = 2;
pub const TXX9_DMA_CCR_SNGAD: u32 = 1;

pub const TXX9_DMA_CSR_CHNEN: u32 = 0x400;
pub const TXX9_DMA_CSR_STLXFER: u32 = 0x200;
pub const TXX9_DMA_CSR_XFACT: u32 = 0x100;
pub const TXX9_DMA_CSR_ABCHC: u32 = 0x80;
pub const TXX9_DMA_CSR_NCHNC: u32 = 0x40;
pub const TXX9_DMA_CSR_NTRNFC: u32 = 0x20;
pub const TXX9_DMA_CSR_EXTDN: u32 = 0x10;
pub const TXX9_DMA_CSR_CFERR: u32 = 8;
pub const TXX9_DMA_CSR_CHERR: u32 = 4;
pub const TXX9_DMA_CSR_DESERR: u32 = 2;
pub const TXX9_DMA_CSR_SORERR: u32 = 1;

#[repr(C)]
pub struct txx9dmac_chan {
    pub chan: dma_chan, pub dma: dma_device, pub ddev: *mut txx9dmac_dev,
    pub ch_regs: *mut core::ffi::c_void, pub tasklet: tasklet_struct, pub irq: i32, pub ccr: u32,
    pub lock: spinlock_t, pub active_list: list_head, pub queue: list_head, pub free_list: list_head,
    pub descs_allocated: u32,
}
#[repr(C)]
pub struct txx9dmac_dev {
    pub regs: *mut core::ffi::c_void, pub tasklet: tasklet_struct, pub irq: i32,
    pub chan: [*mut txx9dmac_chan; TXX9_DMA_MAX_NR_CHANNELS as usize],
    pub have_64bit_regs: bool, pub descsize: u32,
}
#[inline] pub unsafe fn __is_dmac64(ddev: *const txx9dmac_dev) -> bool { (*ddev).have_64bit_regs }
#[inline] pub unsafe fn is_dmac64(dc: *const txx9dmac_chan) -> bool { __is_dmac64((*dc).ddev) }

#[cfg(feature = "CONFIG_MACH_TX49XX")]
#[repr(C)] pub struct txx9dmac_hwdesc { pub CHAR: u64, pub SAR: u64, pub DAR: u64, pub CNTR: u32, pub __pad_CNTR: u32 }
#[cfg(not(feature = "CONFIG_MACH_TX49XX"))]
pub type txx9dmac_hwdesc = txx9dmac_cregs;
#[cfg(feature = "CONFIG_MACH_TX49XX")]
pub type txx9dmac_hwdesc32 = txx9dmac_cregs32;
#[cfg(not(feature = "CONFIG_MACH_TX49XX"))]
pub type txx9dmac_hwdesc32 = txx9dmac_cregs32;

#[repr(C)] pub union txx9dmac_desc_hw { pub hwdesc: txx9dmac_hwdesc, pub hwdesc32: txx9dmac_hwdesc32 }
#[repr(C)] pub struct txx9dmac_desc {
    pub first: txx9dmac_desc_hw, pub desc_node: list_head, pub tx_list: list_head,
    pub txd: dma_async_tx_descriptor, pub len: usize,
}

#[cfg(feature = "CONFIG_MACH_TX49XX")]
#[inline] pub unsafe fn txx9dmac_chan_INTENT(dc: *const txx9dmac_chan) -> bool { ((*dc).ccr & TXX9_DMA_CCR_INTENT) != 0 }
#[cfg(feature = "CONFIG_MACH_TX49XX")]
#[inline] pub unsafe fn txx9dmac_chan_set_INTENT(dc: *mut txx9dmac_chan) { (*dc).ccr |= TXX9_DMA_CCR_INTENT; }
#[cfg(feature = "CONFIG_MACH_TX49XX")]
#[inline] pub unsafe fn txx9dmac_desc_set_INTENT(_ddev: *mut txx9dmac_dev, _desc: *mut txx9dmac_desc) {}
#[cfg(feature = "CONFIG_MACH_TX49XX")]
#[inline] pub unsafe fn txx9dmac_chan_set_SMPCHN(dc: *mut txx9dmac_chan) { (*dc).ccr |= TXX9_DMA_CCR_SMPCHN; }
#[cfg(feature = "CONFIG_MACH_TX49XX")]
#[inline] pub unsafe fn txx9dmac_desc_set_nosimple(_ddev: *mut txx9dmac_dev, _desc: *mut txx9dmac_desc, _sair: u32, _dair: u32, _ccr: u32) {}

#[cfg(not(feature = "CONFIG_MACH_TX49XX"))]
#[inline] pub unsafe fn txx9dmac_chan_INTENT(_dc: *const txx9dmac_chan) -> bool { true }
#[cfg(not(feature = "CONFIG_MACH_TX49XX"))]
#[inline] pub unsafe fn txx9dmac_chan_set_INTENT(_dc: *mut txx9dmac_chan) {}
#[cfg(not(feature = "CONFIG_MACH_TX49XX"))]
#[inline] pub unsafe fn txx9dmac_desc_set_INTENT(ddev: *mut txx9dmac_dev, desc: *mut txx9dmac_desc) {
    if __is_dmac64(ddev) { (*desc).first.hwdesc.CCR |= TXX9_DMA_CCR_INTENT; }
    else { (*desc).first.hwdesc32.CCR |= TXX9_DMA_CCR_INTENT; }
}
#[cfg(not(feature = "CONFIG_MACH_TX49XX"))]
#[inline] pub unsafe fn txx9dmac_chan_set_SMPCHN(_dc: *mut txx9dmac_chan) {}
#[cfg(not(feature = "CONFIG_MACH_TX49XX"))]
#[inline] pub unsafe fn txx9dmac_desc_set_nosimple(ddev: *mut txx9dmac_dev, desc: *mut txx9dmac_desc, sai: u32, dai: u32, ccr: u32) {
    if __is_dmac64(ddev) { (*desc).first.hwdesc.SAIR = sai; (*desc).first.hwdesc.DAIR = dai; (*desc).first.hwdesc.CCR = ccr; }
    else { (*desc).first.hwdesc32.SAIR = sai; (*desc).first.hwdesc32.DAIR = dai; (*desc).first.hwdesc32.CCR = ccr; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
