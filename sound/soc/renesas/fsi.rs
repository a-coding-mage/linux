// SPDX-License-Identifier: GPL-2.0
//
// Fifo-attached Serial Interface (FSI) support for SH7724
//
// Copyright (C) 2009 Renesas Solutions Corp.
// Kuninori Morimoto <morimoto.kuninori@renesas.com>
//
// Based on ssi.c
// Copyright (c) 2007 Manuel Lauss <mano@roarinelk.homelinux.net>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type phys_addr_t = c_ulong;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_int;

/* PortA/PortB register */
const REG_DO_FMT: u32 = 0x0000;
const REG_DOFF_CTL: u32 = 0x0004;
const REG_DOFF_ST: u32 = 0x0008;
const REG_DI_FMT: u32 = 0x000C;
const REG_DIFF_CTL: u32 = 0x0010;
const REG_DIFF_ST: u32 = 0x0014;
const REG_CKG1: u32 = 0x0018;
const REG_CKG2: u32 = 0x001C;
const REG_DIDT: u32 = 0x0020;
const REG_DODT: u32 = 0x0024;
const REG_MUTE_ST: u32 = 0x0028;
const REG_OUT_DMAC: u32 = 0x002C;
const REG_OUT_SEL: u32 = 0x0030;
const REG_IN_DMAC: u32 = 0x0038;

/* master register */
const MST_CLK_RST: u32 = 0x0210;
const MST_SOFT_RST: u32 = 0x0214;
const MST_FIFO_SZ: u32 = 0x0218;

/* core register (depend on FSI version) */
const A_MST_CTLR: u32 = 0x0180;
const B_MST_CTLR: u32 = 0x01A0;
const CPU_INT_ST: u32 = 0x01F4;
const CPU_IEMSK: u32 = 0x01F8;
const CPU_IMSK: u32 = 0x01FC;
const INT_ST: u32 = 0x0200;
const IEMSK: u32 = 0x0204;
const IMSK: u32 = 0x0208;

/* DO_FMT */
/* DI_FMT */
const CR_BWS_MASK: u32 = 0x3 << 20; /* FSI2 */
const CR_BWS_24: u32 = 0x0 << 20; /* FSI2 */
const CR_BWS_16: u32 = 0x1 << 20; /* FSI2 */
const CR_BWS_20: u32 = 0x2 << 20; /* FSI2 */

const CR_DTMD_PCM: u32 = 0x0 << 8; /* FSI2 */
const CR_DTMD_SPDIF_PCM: u32 = 0x1 << 8; /* FSI2 */
const CR_DTMD_SPDIF_STREAM: u32 = 0x2 << 8; /* FSI2 */

const CR_MONO: u32 = 0x0 << 4;
const CR_MONO_D: u32 = 0x1 << 4;
const CR_PCM: u32 = 0x2 << 4;
const CR_I2S: u32 = 0x3 << 4;
const CR_TDM: u32 = 0x4 << 4;
const CR_TDM_D: u32 = 0x5 << 4;

/* OUT_DMAC */
/* IN_DMAC */
const VDMD_MASK: u32 = 0x3 << 4;
const VDMD_FRONT: u32 = 0x0 << 4; /* Package in front */
const VDMD_BACK: u32 = 0x1 << 4; /* Package in back */
const VDMD_STREAM: u32 = 0x2 << 4; /* Stream mode(16bit * 2) */

const DMA_ON: u32 = 0x1 << 0;

/* DOFF_CTL */
/* DIFF_CTL */
const IRQ_HALF: u32 = 0x00100000;
const FIFO_CLR: u32 = 0x00000001;

/* DOFF_ST */
const ERR_OVER: u32 = 0x00000010;
const ERR_UNDER: u32 = 0x00000001;
const ST_ERR: u32 = ERR_OVER | ERR_UNDER;

/* CKG1 */
const ACKMD_MASK: u32 = 0x00007000;
const BPFMD_MASK: u32 = 0x00000700;
const DIMD: u32 = 1 << 4;
const DOMD: u32 = 1 << 0;

/* A/B MST_CTLR */
const BP: u32 = 1 << 4; /* Fix the signal of Biphase output */
const SE: u32 = 1 << 0; /* Fix the master clock */

/* CLK_RST */
const CRB: u32 = 1 << 4;
const CRA: u32 = 1 << 0;

/* IO SHIFT / MACRO */
const BI_SHIFT: u32 = 12;
const BO_SHIFT: u32 = 8;
const AI_SHIFT: u32 = 4;
const AO_SHIFT: u32 = 0;
const fn AB_IO(param: u32, shift: u32) -> u32 { param << shift }

/* SOFT_RST */
const PBSR: u32 = 1 << 12; /* Port B Software Reset */
const PASR: u32 = 1 << 8; /* Port A Software Reset */
const IR: u32 = 1 << 4; /* Interrupt Reset */
const FSISR: u32 = 1 << 0; /* Software Reset */

/* OUT_SEL (FSI2) */
const DMMD: u32 = 1 << 4; /* SPDIF output timing 0: Biphase only */
                         /*                       1: Biphase and serial */

/* FIFO_SZ */
const FIFO_SZ_MASK: u32 = 0x7;

const FSI_RATES: u32 = SNDRV_PCM_RATE_8000_96000;
const FSI_FMTS: u64 = SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S16_LE;

/*
 * bus options
 *
 * 0x000000BA
 *
 * A : sample widtht 16bit setting
 * B : sample widtht 24bit setting
 */
const SHIFT_16DATA: u32 = 0;
const SHIFT_24DATA: u32 = 4;

const PACKAGE_24BITBUS_BACK: u32 = 0;
const PACKAGE_24BITBUS_FRONT: u32 = 1;
const PACKAGE_16BITBUS_STREAM: u32 = 2;

const fn BUSOP_SET_16(a: u32) -> u32 { a << SHIFT_16DATA }
const fn BUSOP_SET_24(a: u32) -> u32 { a << SHIFT_24DATA }
const fn BUSOP_GET_16(a: u32) -> u32 { (a >> SHIFT_16DATA) & 0xF }
const fn BUSOP_GET_24(a: u32) -> u32 { (a >> SHIFT_24DATA) & 0xF }

const EINVAL: c_int = 22;
const EIO: c_int = 5;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const GFP_KERNEL: c_uint = 0;
const IORESOURCE_MEM: c_uint = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const DMA_PREP_INTERRUPT: c_uint = 1;
const DMA_CTRL_ACK: c_uint = 2;
const DMA_SLAVE_BUSWIDTH_4_BYTES: c_uint = 4;
const DMA_MEM_TO_DEV: dma_transfer_direction = 0;
const DMA_DEV_TO_MEM: dma_transfer_direction = 1;

/*
 * FSI driver use below type name for variable
 *
 * xxx_num  : number of data
 * xxx_pos  : position of data
 * xxx_capa : capacity of data
 */

#[repr(C)] pub struct snd_pcm_substream { pub stream: c_int, pub runtime: *mut snd_pcm_runtime }
#[repr(C)] pub struct snd_pcm_runtime { pub buffer_size: c_int, pub period_size: c_int, pub periods: c_int, pub dma_area: *mut u8, pub dma_addr: phys_addr_t }
#[repr(C)] pub struct snd_soc_dai { pub id: c_int, pub dev: *mut device }
#[repr(C)] pub struct snd_soc_pcm_runtime { pub pcm: *mut snd_pcm, pub card: *mut snd_soc_card }
#[repr(C)] pub struct snd_soc_card { pub snd_card: *mut snd_card }
#[repr(C)] pub struct snd_card { pub dev: *mut device }
#[repr(C)] pub struct snd_pcm;
#[repr(C)] pub struct snd_soc_component;
#[repr(C)] pub struct snd_pcm_hw_params;
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct clk;
#[repr(C)] pub struct dma_chan;
#[repr(C)] pub struct resource { pub start: phys_addr_t }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }

#[repr(C)]
pub struct dma_async_tx_descriptor {
    pub callback: Option<unsafe extern "C" fn(*mut c_void)>,
    pub callback_param: *mut c_void,
}

type dma_transfer_direction = c_int;

#[repr(C)]
pub struct dma_slave_config {
    pub dst_addr: phys_addr_t,
    pub dst_addr_width: c_uint,
    pub direction: dma_transfer_direction,
    pub src_addr: phys_addr_t,
    pub src_addr_width: c_uint,
}

#[repr(C)]
pub struct sh_fsi_port_info { pub flags: c_ulong, pub tx_id: c_int }

#[repr(C)]
pub struct sh_fsi_platform_info {
    pub port_a: sh_fsi_port_info,
    pub port_b: sh_fsi_port_info,
}

#[repr(C)]
pub struct fsi_stream {
    /*
     * these are initialized by fsi_stream_init()
     */
    pub substream: *mut snd_pcm_substream,
    pub fifo_sample_capa: c_int,
    pub buff_sample_capa: c_int,
    pub buff_sample_pos: c_int,
    pub period_samples: c_int,
    pub period_pos: c_int,
    pub sample_width: c_int,
    pub uerr_num: c_int,
    pub oerr_num: c_int,

    /*
     * bus options
     */
    pub bus_option: u32,

    /*
     * these are initialized by fsi_handler_init()
     */
    pub handler: *const fsi_stream_handler,
    pub priv_: *mut fsi_priv,

    /*
     * these are for DMAEngine
     */
    pub chan: *mut dma_chan,
    pub dma_id: c_int,
}

#[repr(C)]
pub struct fsi_clk {
    /* see [FSI clock] */
    pub own: *mut clk,
    pub xck: *mut clk,
    pub ick: *mut clk,
    pub div: *mut clk,
    pub set_rate: Option<unsafe extern "C" fn(*mut device, *mut fsi_priv) -> c_int>,
    pub rate: c_ulong,
    pub count: c_uint,
}

#[repr(C)]
pub struct fsi_priv {
    pub base: *mut u8,
    pub phys: phys_addr_t,
    pub master: *mut fsi_master,
    pub playback: fsi_stream,
    pub capture: fsi_stream,
    pub clock: fsi_clk,
    pub fmt: u32,
    /* C bitfields translated as storage preserving integer intent. */
    pub chan_num: c_int,
    pub clk_master: c_uint,
    pub clk_cpg: c_uint,
    pub spdif: c_uint,
    pub enable_stream: c_uint,
    pub bit_clk_inv: c_uint,
    pub lr_clk_inv: c_uint,
}

#[repr(C)]
pub struct fsi_stream_handler {
    pub init: Option<unsafe extern "C" fn(*mut fsi_priv, *mut fsi_stream) -> c_int>,
    pub quit: Option<unsafe extern "C" fn(*mut fsi_priv, *mut fsi_stream) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut fsi_priv, *mut fsi_stream, *mut device) -> c_int>,
    pub transfer: Option<unsafe extern "C" fn(*mut fsi_priv, *mut fsi_stream) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut fsi_priv, *mut fsi_stream) -> c_int>,
    pub start_stop: Option<unsafe extern "C" fn(*mut fsi_priv, *mut fsi_stream, c_int) -> c_int>,
}

unsafe fn fsi_stream_handler_call_init(io: *mut fsi_stream, fsi: *mut fsi_priv) -> c_int {
    if io.is_null() { -ENODEV } else { (*(*io).handler).init.map_or(0, |f| f(fsi, io)) }
}
unsafe fn fsi_stream_handler_call_quit(io: *mut fsi_stream, fsi: *mut fsi_priv) -> c_int {
    if io.is_null() { -ENODEV } else { (*(*io).handler).quit.map_or(0, |f| f(fsi, io)) }
}
unsafe fn fsi_stream_handler_call_probe(io: *mut fsi_stream, fsi: *mut fsi_priv, dev: *mut device) -> c_int {
    if io.is_null() { -ENODEV } else { (*(*io).handler).probe.map_or(0, |f| f(fsi, io, dev)) }
}
unsafe fn fsi_stream_handler_call_transfer(io: *mut fsi_stream, fsi: *mut fsi_priv) -> c_int {
    if io.is_null() { -ENODEV } else { (*(*io).handler).transfer.map_or(0, |f| f(fsi, io)) }
}
unsafe fn fsi_stream_handler_call_remove(io: *mut fsi_stream, fsi: *mut fsi_priv) -> c_int {
    if io.is_null() { -ENODEV } else { (*(*io).handler).remove.map_or(0, |f| f(fsi, io)) }
}
unsafe fn fsi_stream_handler_call_start_stop(io: *mut fsi_stream, fsi: *mut fsi_priv, enable: c_int) -> c_int {
    if io.is_null() { -ENODEV } else { (*(*io).handler).start_stop.map_or(0, |f| f(fsi, io, enable)) }
}

#[repr(C)]
pub struct fsi_core {
    pub ver: c_int,
    pub int_st: u32,
    pub iemsk: u32,
    pub imsk: u32,
    pub a_mclk: u32,
    pub b_mclk: u32,
}

#[repr(C)]
pub struct fsi_master {
    pub base: *mut u8,
    pub fsia: fsi_priv,
    pub fsib: fsi_priv,
    pub clk_spu: *mut clk,
    pub core: *const fsi_core,
    pub lock: spinlock_t,
}

#[inline]
unsafe fn fsi_stream_is_play(fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int {
    (ptr::addr_of_mut!((*fsi).playback) == io) as c_int
}

/*
 * basic read write function
 */
unsafe fn __fsi_reg_write(reg: *mut u32, mut data: u32) {
    /* valid data area is 24bit */
    data &= 0x00ff_ffff;
    __raw_writel(data, reg);
}

unsafe fn __fsi_reg_read(reg: *mut u32) -> u32 {
    __raw_readl(reg)
}

unsafe fn __fsi_reg_mask_set(reg: *mut u32, mask: u32, data: u32) {
    let mut val = __fsi_reg_read(reg);
    val &= !mask;
    val |= data & mask;
    __fsi_reg_write(reg, val);
}

unsafe fn fsi_reg_write(p: *mut fsi_priv, r: u32, d: u32) { __fsi_reg_write((*p).base.add(r as usize) as *mut u32, d) }
unsafe fn fsi_reg_read(p: *mut fsi_priv, r: u32) -> u32 { __fsi_reg_read((*p).base.add(r as usize) as *mut u32) }
unsafe fn fsi_reg_mask_set(p: *mut fsi_priv, r: u32, m: u32, d: u32) { __fsi_reg_mask_set((*p).base.add(r as usize) as *mut u32, m, d) }
unsafe fn fsi_master_read(p: *mut fsi_master, r: u32) -> u32 { _fsi_master_read(p, r) }
unsafe fn fsi_core_read(p: *mut fsi_master, r: u32) -> u32 { _fsi_master_read(p, r) }

unsafe fn _fsi_master_read(master: *mut fsi_master, reg: u32) -> u32 {
    spin_lock_irqsave(ptr::addr_of_mut!((*master).lock));
    let ret = __fsi_reg_read((*master).base.add(reg as usize) as *mut u32);
    spin_unlock_irqrestore(ptr::addr_of_mut!((*master).lock));
    ret
}

unsafe fn fsi_master_mask_set(p: *mut fsi_master, r: u32, m: u32, d: u32) { _fsi_master_mask_set(p, r, m, d) }
unsafe fn fsi_core_mask_set(p: *mut fsi_master, r: u32, m: u32, d: u32) { _fsi_master_mask_set(p, r, m, d) }

unsafe fn _fsi_master_mask_set(master: *mut fsi_master, reg: u32, mask: u32, data: u32) {
    spin_lock_irqsave(ptr::addr_of_mut!((*master).lock));
    __fsi_reg_mask_set((*master).base.add(reg as usize) as *mut u32, mask, data);
    spin_unlock_irqrestore(ptr::addr_of_mut!((*master).lock));
}

/*
 * basic function
 */
unsafe fn fsi_version(master: *mut fsi_master) -> c_int { (*(*master).core).ver }
unsafe fn fsi_get_master(fsi: *mut fsi_priv) -> *mut fsi_master { (*fsi).master }
unsafe fn fsi_is_clk_master(fsi: *mut fsi_priv) -> c_int { (*fsi).clk_master as c_int }
unsafe fn fsi_is_port_a(fsi: *mut fsi_priv) -> c_int { ((*(*fsi).master).base == (*fsi).base) as c_int }
unsafe fn fsi_is_spdif(fsi: *mut fsi_priv) -> c_int { (*fsi).spdif as c_int }
unsafe fn fsi_is_enable_stream(fsi: *mut fsi_priv) -> c_int { (*fsi).enable_stream as c_int }
unsafe fn fsi_is_play(substream: *mut snd_pcm_substream) -> c_int { ((*substream).stream == SNDRV_PCM_STREAM_PLAYBACK) as c_int }
unsafe fn fsi_get_dai(substream: *mut snd_pcm_substream) -> *mut snd_soc_dai {
    let rtd = snd_soc_substream_to_rtd(substream);
    snd_soc_rtd_to_cpu(rtd, 0)
}
unsafe fn fsi_get_priv_frm_dai(dai: *mut snd_soc_dai) -> *mut fsi_priv {
    let master = snd_soc_dai_get_drvdata(dai) as *mut fsi_master;
    if (*dai).id == 0 { ptr::addr_of_mut!((*master).fsia) } else { ptr::addr_of_mut!((*master).fsib) }
}
unsafe fn fsi_get_priv(substream: *mut snd_pcm_substream) -> *mut fsi_priv { fsi_get_priv_frm_dai(fsi_get_dai(substream)) }
unsafe fn fsi_get_port_shift(fsi: *mut fsi_priv, io: *mut fsi_stream) -> u32 {
    let is_play = fsi_stream_is_play(fsi, io);
    let is_porta = fsi_is_port_a(fsi);
    if is_porta != 0 {
        if is_play != 0 { AO_SHIFT } else { AI_SHIFT }
    } else if is_play != 0 { BO_SHIFT } else { BI_SHIFT }
}
unsafe fn fsi_frame2sample(fsi: *mut fsi_priv, frames: c_int) -> c_int { frames * (*fsi).chan_num }
unsafe fn fsi_sample2frame(fsi: *mut fsi_priv, samples: c_int) -> c_int { samples / (*fsi).chan_num }
unsafe fn fsi_stream_is_working(fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int {
    let master = fsi_get_master(fsi);
    spin_lock_irqsave(ptr::addr_of_mut!((*master).lock));
    let ret = (!(*io).substream.is_null() && !(*(*io).substream).runtime.is_null()) as c_int;
    spin_unlock_irqrestore(ptr::addr_of_mut!((*master).lock));
    ret
}
unsafe fn fsi_get_current_fifo_samples(fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int {
    let is_play = fsi_stream_is_play(fsi, io);
    let status = if is_play != 0 { fsi_reg_read(fsi, REG_DOFF_ST) } else { fsi_reg_read(fsi, REG_DIFF_ST) };
    let frames = (0x1ff & (status >> 8)) as c_int;
    fsi_frame2sample(fsi, frames)
}
unsafe fn fsi_count_fifo_err(fsi: *mut fsi_priv) {
    if fsi_stream_is_working(fsi, ptr::addr_of_mut!((*fsi).playback)) == 0 &&
       fsi_stream_is_working(fsi, ptr::addr_of_mut!((*fsi).capture)) == 0 { return; }
    let ostatus = fsi_reg_read(fsi, REG_DOFF_ST);
    let istatus = fsi_reg_read(fsi, REG_DIFF_ST);
    if ostatus & ERR_OVER != 0 { (*fsi).playback.oerr_num += 1; }
    if ostatus & ERR_UNDER != 0 { (*fsi).playback.uerr_num += 1; }
    if istatus & ERR_OVER != 0 { (*fsi).capture.oerr_num += 1; }
    if istatus & ERR_UNDER != 0 { (*fsi).capture.uerr_num += 1; }
    fsi_reg_write(fsi, REG_DOFF_ST, 0);
    fsi_reg_write(fsi, REG_DIFF_ST, 0);
}

/*
 * fsi_stream_xx() function
 */
#[inline]
unsafe fn fsi_stream_get(fsi: *mut fsi_priv, substream: *mut snd_pcm_substream) -> *mut fsi_stream {
    if fsi_is_play(substream) != 0 { ptr::addr_of_mut!((*fsi).playback) } else { ptr::addr_of_mut!((*fsi).capture) }
}
unsafe fn fsi_stream_to_priv(io: *mut fsi_stream) -> *mut fsi_priv { (*io).priv_ }
unsafe fn fsi_stream_init(fsi: *mut fsi_priv, io: *mut fsi_stream, substream: *mut snd_pcm_substream) {
    let runtime = (*substream).runtime;
    let master = fsi_get_master(fsi);
    spin_lock_irqsave(ptr::addr_of_mut!((*master).lock));
    (*io).substream = substream;
    (*io).buff_sample_capa = fsi_frame2sample(fsi, (*runtime).buffer_size);
    (*io).buff_sample_pos = 0;
    (*io).period_samples = fsi_frame2sample(fsi, (*runtime).period_size);
    (*io).period_pos = 0;
    (*io).sample_width = samples_to_bytes(runtime, 1);
    (*io).bus_option = 0;
    (*io).oerr_num = -1; /* ignore 1st err */
    (*io).uerr_num = -1; /* ignore 1st err */
    fsi_stream_handler_call_init(io, fsi);
    spin_unlock_irqrestore(ptr::addr_of_mut!((*master).lock));
}
unsafe fn fsi_stream_quit(fsi: *mut fsi_priv, io: *mut fsi_stream) {
    let dai = fsi_get_dai((*io).substream);
    let master = fsi_get_master(fsi);
    spin_lock_irqsave(ptr::addr_of_mut!((*master).lock));
    if (*io).oerr_num > 0 { dev_err((*dai).dev, c"over_run = %d\n".as_ptr(), (*io).oerr_num); }
    if (*io).uerr_num > 0 { dev_err((*dai).dev, c"under_run = %d\n".as_ptr(), (*io).uerr_num); }
    fsi_stream_handler_call_quit(io, fsi);
    (*io).substream = ptr::null_mut();
    (*io).buff_sample_capa = 0;
    (*io).buff_sample_pos = 0;
    (*io).period_samples = 0;
    (*io).period_pos = 0;
    (*io).sample_width = 0;
    (*io).bus_option = 0;
    (*io).oerr_num = 0;
    (*io).uerr_num = 0;
    spin_unlock_irqrestore(ptr::addr_of_mut!((*master).lock));
}
unsafe fn fsi_stream_transfer(io: *mut fsi_stream) -> c_int {
    let fsi = fsi_stream_to_priv(io);
    if fsi.is_null() { return -EIO; }
    fsi_stream_handler_call_transfer(io, fsi)
}
unsafe fn fsi_stream_start(fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int { fsi_stream_handler_call_start_stop(io, fsi, 1) }
unsafe fn fsi_stream_stop(fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int { fsi_stream_handler_call_start_stop(io, fsi, 0) }
unsafe fn fsi_stream_probe(fsi: *mut fsi_priv, dev: *mut device) -> c_int {
    let mut io = ptr::addr_of_mut!((*fsi).playback);
    let ret1 = fsi_stream_handler_call_probe(io, fsi, dev);
    io = ptr::addr_of_mut!((*fsi).capture);
    let ret2 = fsi_stream_handler_call_probe(io, fsi, dev);
    if ret1 < 0 { return ret1; }
    if ret2 < 0 { return ret2; }
    0
}
unsafe fn fsi_stream_remove(fsi: *mut fsi_priv) -> c_int {
    let mut io = ptr::addr_of_mut!((*fsi).playback);
    let ret1 = fsi_stream_handler_call_remove(io, fsi);
    io = ptr::addr_of_mut!((*fsi).capture);
    let ret2 = fsi_stream_handler_call_remove(io, fsi);
    if ret1 < 0 { return ret1; }
    if ret2 < 0 { return ret2; }
    0
}

/*
 * format/bus/dma setting
 */
unsafe fn fsi_format_bus_setup(fsi: *mut fsi_priv, io: *mut fsi_stream, bus: u32, dev: *mut device) {
    let master = fsi_get_master(fsi);
    let is_play = fsi_stream_is_play(fsi, io);
    let mut fmt = (*fsi).fmt;
    if fsi_version(master) >= 2 {
        let mut dma = 0;
        match bus {
            PACKAGE_24BITBUS_FRONT => { fmt |= CR_BWS_24; dma |= VDMD_FRONT; dev_dbg(dev, c"24bit bus / package in front\n".as_ptr()); }
            PACKAGE_16BITBUS_STREAM => { fmt |= CR_BWS_16; dma |= VDMD_STREAM; dev_dbg(dev, c"16bit bus / stream mode\n".as_ptr()); }
            _ => { fmt |= CR_BWS_24; dma |= VDMD_BACK; dev_dbg(dev, c"24bit bus / package in back\n".as_ptr()); }
        }
        if is_play != 0 { fsi_reg_write(fsi, REG_OUT_DMAC, dma); } else { fsi_reg_write(fsi, REG_IN_DMAC, dma); }
    }
    if is_play != 0 { fsi_reg_write(fsi, REG_DO_FMT, fmt); } else { fsi_reg_write(fsi, REG_DI_FMT, fmt); }
}

/*
 * irq function
 */
unsafe fn fsi_irq_enable(fsi: *mut fsi_priv, io: *mut fsi_stream) {
    let data = AB_IO(1, fsi_get_port_shift(fsi, io));
    let master = fsi_get_master(fsi);
    fsi_core_mask_set(master, (*(*master).core).imsk, data, data);
    fsi_core_mask_set(master, (*(*master).core).iemsk, data, data);
}
unsafe fn fsi_irq_disable(fsi: *mut fsi_priv, io: *mut fsi_stream) {
    let data = AB_IO(1, fsi_get_port_shift(fsi, io));
    let master = fsi_get_master(fsi);
    fsi_core_mask_set(master, (*(*master).core).imsk, data, 0);
    fsi_core_mask_set(master, (*(*master).core).iemsk, data, 0);
}
unsafe fn fsi_irq_get_status(master: *mut fsi_master) -> u32 { fsi_core_read(master, (*(*master).core).int_st) }
unsafe fn fsi_irq_clear_status(fsi: *mut fsi_priv) {
    let mut data = 0;
    let master = fsi_get_master(fsi);
    if fsi_stream_is_working(fsi, ptr::addr_of_mut!((*fsi).playback)) == 0 &&
       fsi_stream_is_working(fsi, ptr::addr_of_mut!((*fsi).capture)) == 0 { return; }
    data |= AB_IO(1, fsi_get_port_shift(fsi, ptr::addr_of_mut!((*fsi).playback)));
    data |= AB_IO(1, fsi_get_port_shift(fsi, ptr::addr_of_mut!((*fsi).capture)));
    /* clear interrupt factor */
    fsi_core_mask_set(master, (*(*master).core).int_st, data, 0);
}

/*
 * SPDIF master clock function
 *
 * These functions are used later FSI2
 */
unsafe fn fsi_spdif_clk_ctrl(fsi: *mut fsi_priv, enable: c_int) {
    let master = fsi_get_master(fsi);
    let mask = BP | SE;
    let val = if enable != 0 { mask } else { 0 };
    if fsi_is_port_a(fsi) != 0 {
        fsi_core_mask_set(master, (*(*master).core).a_mclk, mask, val);
    } else {
        fsi_core_mask_set(master, (*(*master).core).b_mclk, mask, val);
    }
}

/*
 * clock function
 */
unsafe fn fsi_clk_invalid(fsi: *mut fsi_priv) { fsi_clk_valid(fsi, 0) }
unsafe fn fsi_clk_valid(fsi: *mut fsi_priv, rate: c_ulong) { (*fsi).clock.rate = rate; }
unsafe fn fsi_clk_is_valid(fsi: *mut fsi_priv) -> c_int { ((*fsi).clock.set_rate.is_some() && (*fsi).clock.rate != 0) as c_int }

unsafe fn fsi_clk_prepare(fsi: *mut fsi_priv) -> c_int {
    let clock = ptr::addr_of_mut!((*fsi).clock);
    let spu = (*(*fsi).master).clk_spu;
    let xck = (*clock).xck;
    let ick = (*clock).ick;
    let div = (*clock).div;
    let mut ret = clk_prepare(spu);
    if ret != 0 { return ret; }
    ret = clk_prepare(xck);
    if ret != 0 { clk_unprepare(spu); return ret; }
    ret = clk_prepare(ick);
    if ret != 0 { clk_unprepare(xck); clk_unprepare(spu); return ret; }
    ret = clk_prepare(div);
    if ret != 0 { clk_unprepare(ick); clk_unprepare(xck); clk_unprepare(spu); return ret; }
    0
}
unsafe fn fsi_clk_unprepare(fsi: *mut fsi_priv) {
    let clock = ptr::addr_of_mut!((*fsi).clock);
    clk_unprepare((*clock).div);
    clk_unprepare((*clock).ick);
    clk_unprepare((*clock).xck);
    clk_unprepare((*(*fsi).master).clk_spu);
}
unsafe extern "C" fn fsi_clk_enable(dev: *mut device, fsi: *mut fsi_priv) -> c_int {
    let clock = ptr::addr_of_mut!((*fsi).clock);
    let mut ret = -EINVAL;
    if fsi_clk_is_valid(fsi) == 0 { return ret; }
    if (*clock).count == 0 {
        ret = (*clock).set_rate.unwrap()(dev, fsi);
        if ret < 0 { fsi_clk_invalid(fsi); return ret; }
        ret = clk_enable((*clock).xck);
        if ret != 0 { return ret; }
        ret = clk_enable((*clock).ick);
        if ret != 0 { clk_disable((*clock).xck); return ret; }
        ret = clk_enable((*clock).div);
        if ret != 0 { clk_disable((*clock).ick); clk_disable((*clock).xck); return ret; }
        (*clock).count += 1;
    }
    ret
}
unsafe fn fsi_clk_disable(_dev: *mut device, fsi: *mut fsi_priv) -> c_int {
    let clock = ptr::addr_of_mut!((*fsi).clock);
    if fsi_clk_is_valid(fsi) == 0 { return -EINVAL; }
    let old = (*clock).count;
    (*clock).count = (*clock).count.wrapping_sub(1);
    if old == 1 {
        clk_disable((*clock).xck);
        clk_disable((*clock).ick);
        clk_disable((*clock).div);
    }
    0
}

unsafe fn fsi_clk_set_ackbpf(dev: *mut device, fsi: *mut fsi_priv, ackmd: c_int, bpfmd: c_int) -> c_int {
    let mut data = 0;
    if bpfmd > ackmd {
        dev_err(dev, c"unsupported rate (%d/%d)\n".as_ptr(), ackmd, bpfmd);
        return -EINVAL;
    }
    match ackmd {
        512 => data |= 0x0 << 12,
        256 => data |= 0x1 << 12,
        128 => data |= 0x2 << 12,
        64 => data |= 0x3 << 12,
        32 => data |= 0x4 << 12,
        _ => { dev_err(dev, c"unsupported ackmd (%d)\n".as_ptr(), ackmd); return -EINVAL; }
    }
    match bpfmd {
        32 => data |= 0x0 << 8,
        64 => data |= 0x1 << 8,
        128 => data |= 0x2 << 8,
        256 => data |= 0x3 << 8,
        512 => data |= 0x4 << 8,
        16 => data |= 0x7 << 8,
        _ => { dev_err(dev, c"unsupported bpfmd (%d)\n".as_ptr(), bpfmd); return -EINVAL; }
    }
    dev_dbg(dev, c"ACKMD/BPFMD = %d/%d\n".as_ptr(), ackmd, bpfmd);
    fsi_reg_mask_set(fsi, REG_CKG1, ACKMD_MASK | BPFMD_MASK, data);
    udelay(10);
    0
}

unsafe extern "C" fn fsi_clk_set_rate_external(dev: *mut device, fsi: *mut fsi_priv) -> c_int {
    let xck = (*fsi).clock.xck;
    let ick = (*fsi).clock.ick;
    let rate = (*fsi).clock.rate;
    let ret;
    if xck.is_null() || ick.is_null() {
        dev_err(dev, c"xck clock or ick clock is missing\n".as_ptr());
        return -EINVAL;
    }
    let xrate = clk_get_rate(xck);
    if xrate % rate != 0 {
        dev_err(dev, c"unsupported clock rate\n".as_ptr());
        return -EINVAL;
    }
    clk_set_parent(ick, xck);
    clk_set_rate(ick, xrate);
    let bpfmd = (*fsi).chan_num * 32;
    let ackmd = (xrate / rate) as c_int;
    dev_dbg(dev, c"external/rate = %ld/%ld\n".as_ptr(), xrate, rate);
    ret = fsi_clk_set_ackbpf(dev, fsi, ackmd, bpfmd);
    if ret < 0 { dev_err(dev, c"%s failed".as_ptr(), c"fsi_clk_set_rate_external".as_ptr()); }
    ret
}

unsafe extern "C" fn fsi_clk_set_rate_cpg(dev: *mut device, fsi: *mut fsi_priv) -> c_int {
    let ick = (*fsi).clock.ick;
    let div = (*fsi).clock.div;
    let rate = (*fsi).clock.rate;
    let mut target = 0;
    let mut ret = -EINVAL;
    if ick.is_null() || div.is_null() {
        dev_err(dev, c"ick clock or div clock is missing\n".as_ptr());
        return -EINVAL;
    }
    if 12288000 % rate == 0 { target = 12288000; }
    if 11289600 % rate == 0 { target = 11289600; }
    if target == 0 {
        dev_err(dev, c"unsupported rate\n".as_ptr());
        return ret;
    }
    let bpfmd = (*fsi).chan_num * 32;
    let ackmd = (target / rate) as c_int;
    ret = fsi_clk_set_ackbpf(dev, fsi, ackmd, bpfmd);
    if ret < 0 {
        dev_err(dev, c"%s failed".as_ptr(), c"fsi_clk_set_rate_cpg".as_ptr());
        return ret;
    }
    /*
     * The clock flow is
     *
     * [CPG] = cout => [FSI_DIV] = audio => [FSI] => [codec]
     *
     * But, it needs to find best match of CPG and FSI_DIV
     * combination, since it is difficult to generate correct
     * frequency of audio clock from ick clock only.
     * Because ick is created from its parent clock.
     *
     * target = rate x [512/256/128/64]fs
     * cout   = round(target x adjustment)
     * actual = cout / adjustment (by FSI-DIV) ~= target
     * audio  = actual
     */
    let mut min = !0 as c_ulong;
    let mut best_cout = 0;
    let mut best_act = 0;
    let mut adj = 1;
    while adj < 0xffff {
        let mut cout = target * adj as c_ulong;
        if cout > 100000000 { break; } /* max clock = 100MHz */
        cout = clk_round_rate(ick, cout);
        let actual = cout / adj as c_ulong;
        let diff = if actual > target { actual - target } else { target - actual };
        if diff < min {
            min = diff;
            best_cout = cout;
            best_act = actual;
        }
        adj += 1;
    }
    ret = clk_set_rate(ick, best_cout);
    if ret < 0 { dev_err(dev, c"ick clock failed\n".as_ptr()); return -EIO; }
    ret = clk_set_rate(div, clk_round_rate(div, best_act));
    if ret < 0 { dev_err(dev, c"div clock failed\n".as_ptr()); return -EIO; }
    dev_dbg(dev, c"ick/div = %ld/%ld\n".as_ptr(), clk_get_rate(ick), clk_get_rate(div));
    ret
}

unsafe fn fsi_clk_init(dev: *mut device, fsi: *mut fsi_priv) -> c_int {
    let clock = ptr::addr_of_mut!((*fsi).clock);
    let master = (*fsi).master;
    let is_porta = fsi_is_port_a(fsi);
    let (xck, ick, div);
    if (*fsi).clk_cpg != 0 {
        xck = 0; ick = 1; div = 1;
        (*clock).set_rate = Some(fsi_clk_set_rate_cpg);
    } else {
        xck = 1; ick = 1; div = 0;
        (*clock).set_rate = Some(fsi_clk_set_rate_external);
    }
    (*clock).xck = ptr::null_mut();
    (*clock).ick = ptr::null_mut();
    (*clock).div = ptr::null_mut();
    (*clock).rate = 0;
    (*clock).count = 0;
    (*clock).own = devm_clk_get(dev, ptr::null());
    if IS_ERR((*clock).own as *const c_void) { return dev_err_probe(dev, PTR_ERR((*clock).own as *const c_void), c"Can't get fck clock\n".as_ptr()); }
    if (*master).clk_spu.is_null() {
        (*master).clk_spu = devm_clk_get_optional(dev, c"spu".as_ptr());
        if IS_ERR((*master).clk_spu as *const c_void) { return dev_err_probe(dev, PTR_ERR((*master).clk_spu as *const c_void), c"Can't get spu clock\n".as_ptr()); }
    }
    if xck != 0 {
        (*clock).xck = devm_clk_get_optional(dev, if is_porta != 0 { c"xcka".as_ptr() } else { c"xckb".as_ptr() });
        if IS_ERR((*clock).xck as *const c_void) { return dev_err_probe(dev, PTR_ERR((*clock).xck as *const c_void), c"Can't get xck clock\n".as_ptr()); }
        if (*clock).xck == (*clock).own { dev_err(dev, c"cpu doesn't support xck clock\n".as_ptr()); return -EINVAL; }
    }
    if ick != 0 {
        (*clock).ick = devm_clk_get_optional(dev, if is_porta != 0 { c"icka".as_ptr() } else { c"ickb".as_ptr() });
        if IS_ERR((*clock).ick as *const c_void) { return dev_err_probe(dev, PTR_ERR((*clock).ick as *const c_void), c"Can't get ick clock\n".as_ptr()); }
        if (*clock).ick == (*clock).own { dev_err(dev, c"cpu doesn't support ick clock\n".as_ptr()); return -EINVAL; }
    }
    if div != 0 {
        (*clock).div = devm_clk_get_optional(dev, if is_porta != 0 { c"diva".as_ptr() } else { c"divb".as_ptr() });
        if IS_ERR((*clock).div as *const c_void) { return dev_err_probe(dev, PTR_ERR((*clock).div as *const c_void), c"Can't get div clock\n".as_ptr()); }
        if (*clock).div == (*clock).own { dev_err(dev, c"cpu doesn't support div clock\n".as_ptr()); return -EINVAL; }
    }
    0
}

unsafe fn fsi_pointer_update(io: *mut fsi_stream, size: c_int) {
    (*io).buff_sample_pos += size;
    if (*io).buff_sample_pos >= (*io).period_samples * ((*io).period_pos + 1) {
        let substream = (*io).substream;
        let runtime = (*substream).runtime;
        (*io).period_pos += 1;
        if (*io).period_pos >= (*runtime).periods {
            (*io).buff_sample_pos = 0;
            (*io).period_pos = 0;
        }
        snd_pcm_period_elapsed(substream);
    }
}

/*
 * pio data transfer handler
 */
unsafe extern "C" fn fsi_pio_push16(fsi: *mut fsi_priv, _buf: *mut u8, samples: c_int) {
    if fsi_is_enable_stream(fsi) != 0 {
        let buf = _buf as *mut u32;
        for i in 0..(samples / 2) { fsi_reg_write(fsi, REG_DODT, *buf.add(i as usize)); }
    } else {
        let buf = _buf as *mut u16;
        for i in 0..samples { fsi_reg_write(fsi, REG_DODT, (*buf.add(i as usize) as u32) << 8); }
    }
}
unsafe extern "C" fn fsi_pio_pop16(fsi: *mut fsi_priv, _buf: *mut u8, samples: c_int) {
    let buf = _buf as *mut u16;
    for i in 0..samples { *buf.add(i as usize) = (fsi_reg_read(fsi, REG_DIDT) >> 8) as u16; }
}
unsafe extern "C" fn fsi_pio_push32(fsi: *mut fsi_priv, _buf: *mut u8, samples: c_int) {
    let buf = _buf as *mut u32;
    for i in 0..samples { fsi_reg_write(fsi, REG_DODT, *buf.add(i as usize)); }
}
unsafe extern "C" fn fsi_pio_pop32(fsi: *mut fsi_priv, _buf: *mut u8, samples: c_int) {
    let buf = _buf as *mut u32;
    for i in 0..samples { *buf.add(i as usize) = fsi_reg_read(fsi, REG_DIDT); }
}
unsafe fn fsi_pio_get_area(_fsi: *mut fsi_priv, io: *mut fsi_stream) -> *mut u8 {
    let runtime = (*(*io).substream).runtime;
    (*runtime).dma_area.add(samples_to_bytes(runtime, (*io).buff_sample_pos) as usize)
}
type pio_run = unsafe extern "C" fn(*mut fsi_priv, *mut u8, c_int);
unsafe fn fsi_pio_transfer(fsi: *mut fsi_priv, io: *mut fsi_stream, run16: pio_run, run32: pio_run, samples: c_int) -> c_int {
    if fsi_stream_is_working(fsi, io) == 0 { return -EINVAL; }
    let buf = fsi_pio_get_area(fsi, io);
    match (*io).sample_width {
        2 => run16(fsi, buf, samples),
        4 => run32(fsi, buf, samples),
        _ => return -EINVAL,
    }
    fsi_pointer_update(io, samples);
    0
}
unsafe extern "C" fn fsi_pio_pop(fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int {
    let sample_residues = fsi_get_current_fifo_samples(fsi, io);
    let sample_space = (*io).buff_sample_capa - (*io).buff_sample_pos;
    let samples = core::cmp::min(sample_residues, sample_space);
    fsi_pio_transfer(fsi, io, fsi_pio_pop16, fsi_pio_pop32, samples)
}
unsafe extern "C" fn fsi_pio_push(fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int {
    let sample_residues = (*io).buff_sample_capa - (*io).buff_sample_pos;
    let sample_space = (*io).fifo_sample_capa - fsi_get_current_fifo_samples(fsi, io);
    let samples = core::cmp::min(sample_residues, sample_space);
    fsi_pio_transfer(fsi, io, fsi_pio_push16, fsi_pio_push32, samples)
}
unsafe extern "C" fn fsi_pio_start_stop(fsi: *mut fsi_priv, io: *mut fsi_stream, enable: c_int) -> c_int {
    let master = fsi_get_master(fsi);
    let clk = if fsi_is_port_a(fsi) != 0 { CRA } else { CRB };
    if enable != 0 { fsi_irq_enable(fsi, io); } else { fsi_irq_disable(fsi, io); }
    if fsi_is_clk_master(fsi) != 0 { fsi_master_mask_set(master, MST_CLK_RST, clk, if enable != 0 { clk } else { 0 }); }
    0
}
unsafe extern "C" fn fsi_pio_push_init(fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int {
    if fsi_is_enable_stream(fsi) != 0 {
        (*io).bus_option = BUSOP_SET_24(PACKAGE_24BITBUS_BACK) | BUSOP_SET_16(PACKAGE_16BITBUS_STREAM);
    } else {
        (*io).bus_option = BUSOP_SET_24(PACKAGE_24BITBUS_BACK) | BUSOP_SET_16(PACKAGE_24BITBUS_BACK);
    }
    0
}
unsafe extern "C" fn fsi_pio_pop_init(_fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int {
    (*io).bus_option = BUSOP_SET_24(PACKAGE_24BITBUS_BACK) | BUSOP_SET_16(PACKAGE_24BITBUS_BACK);
    0
}

static fsi_pio_push_handler: fsi_stream_handler = fsi_stream_handler {
    init: Some(fsi_pio_push_init), quit: None, probe: None, transfer: Some(fsi_pio_push), remove: None, start_stop: Some(fsi_pio_start_stop),
};
static fsi_pio_pop_handler: fsi_stream_handler = fsi_stream_handler {
    init: Some(fsi_pio_pop_init), quit: None, probe: None, transfer: Some(fsi_pio_pop), remove: None, start_stop: Some(fsi_pio_start_stop),
};

unsafe extern "C" fn fsi_interrupt(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let master = data as *mut fsi_master;
    let int_st = fsi_irq_get_status(master);
    /* clear irq status */
    fsi_master_mask_set(master, MST_SOFT_RST, IR, 0);
    fsi_master_mask_set(master, MST_SOFT_RST, IR, IR);
    if int_st & AB_IO(1, AO_SHIFT) != 0 { fsi_stream_transfer(ptr::addr_of_mut!((*master).fsia.playback)); }
    if int_st & AB_IO(1, BO_SHIFT) != 0 { fsi_stream_transfer(ptr::addr_of_mut!((*master).fsib.playback)); }
    if int_st & AB_IO(1, AI_SHIFT) != 0 { fsi_stream_transfer(ptr::addr_of_mut!((*master).fsia.capture)); }
    if int_st & AB_IO(1, BI_SHIFT) != 0 { fsi_stream_transfer(ptr::addr_of_mut!((*master).fsib.capture)); }
    fsi_count_fifo_err(ptr::addr_of_mut!((*master).fsia));
    fsi_count_fifo_err(ptr::addr_of_mut!((*master).fsib));
    fsi_irq_clear_status(ptr::addr_of_mut!((*master).fsia));
    fsi_irq_clear_status(ptr::addr_of_mut!((*master).fsib));
    IRQ_HANDLED
}

/*
 * dma data transfer handler
 */
unsafe extern "C" fn fsi_dma_init(_fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int {
    (*io).bus_option = BUSOP_SET_24(PACKAGE_24BITBUS_BACK) | BUSOP_SET_16(PACKAGE_16BITBUS_STREAM);
    0
}
unsafe extern "C" fn fsi_dma_complete(data: *mut c_void) {
    let io = data as *mut fsi_stream;
    let fsi = fsi_stream_to_priv(io);
    fsi_pointer_update(io, (*io).period_samples);
    fsi_count_fifo_err(fsi);
}
unsafe extern "C" fn fsi_dma_transfer(fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int {
    let dai = fsi_get_dai((*io).substream);
    let substream = (*io).substream;
    let is_play = fsi_stream_is_play(fsi, io);
    let dir = if is_play != 0 { DMA_MEM_TO_DEV } else { DMA_DEV_TO_MEM };
    let mut ret = -EIO;
    let desc = dmaengine_prep_dma_cyclic((*io).chan, (*(*substream).runtime).dma_addr, snd_pcm_lib_buffer_bytes(substream), snd_pcm_lib_period_bytes(substream), dir, DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if desc.is_null() {
        dev_err((*dai).dev, c"dmaengine_prep_dma_cyclic() fail\n".as_ptr());
        return ret;
    }
    (*desc).callback = Some(fsi_dma_complete);
    (*desc).callback_param = io as *mut c_void;
    if dmaengine_submit(desc) < 0 {
        dev_err((*dai).dev, c"tx_submit() fail\n".as_ptr());
        return ret;
    }
    dma_async_issue_pending((*io).chan);
    /*
     * FIXME
     *
     * In DMAEngine case, codec and FSI cannot be started simultaneously
     * since FSI is using the scheduler work queue.
     * Therefore, in capture case, probably FSI FIFO will have got
     * overflow error in this point.
     * in that case, DMA cannot start transfer until error was cleared.
     */
    if is_play == 0 && (ERR_OVER & fsi_reg_read(fsi, REG_DIFF_ST)) != 0 {
        fsi_reg_mask_set(fsi, REG_DIFF_CTL, FIFO_CLR, FIFO_CLR);
        fsi_reg_write(fsi, REG_DIFF_ST, 0);
    }
    ret = 0;
    ret
}
unsafe extern "C" fn fsi_dma_push_start_stop(fsi: *mut fsi_priv, io: *mut fsi_stream, start: c_int) -> c_int {
    let master = fsi_get_master(fsi);
    let clk = if fsi_is_port_a(fsi) != 0 { CRA } else { CRB };
    let enable = if start != 0 { DMA_ON } else { 0 };
    fsi_reg_mask_set(fsi, REG_OUT_DMAC, DMA_ON, enable);
    dmaengine_terminate_all((*io).chan);
    if fsi_is_clk_master(fsi) != 0 { fsi_master_mask_set(master, MST_CLK_RST, clk, if enable != 0 { clk } else { 0 }); }
    0
}
unsafe extern "C" fn fsi_dma_probe(fsi: *mut fsi_priv, io: *mut fsi_stream, dev: *mut device) -> c_int {
    let is_play = fsi_stream_is_play(fsi, io);
    /* CONFIG_SUPERH path used dma_request_channel(mask, shdma_chan_filter, (void *)io->dma_id). */
    (*io).chan = dma_request_chan(dev, if is_play != 0 { c"tx".as_ptr() } else { c"rx".as_ptr() });
    if IS_ERR((*io).chan as *const c_void) { (*io).chan = ptr::null_mut(); }
    if !(*io).chan.is_null() {
        let mut cfg = dma_slave_config { dst_addr: 0, dst_addr_width: 0, direction: 0, src_addr: 0, src_addr_width: 0 };
        if is_play != 0 {
            cfg.dst_addr = (*fsi).phys + REG_DODT as c_ulong;
            cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
            cfg.direction = DMA_MEM_TO_DEV;
        } else {
            cfg.src_addr = (*fsi).phys + REG_DIDT as c_ulong;
            cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
            cfg.direction = DMA_DEV_TO_MEM;
        }
        let ret = dmaengine_slave_config((*io).chan, ptr::addr_of_mut!(cfg));
        if ret < 0 {
            dma_release_channel((*io).chan);
            (*io).chan = ptr::null_mut();
        }
    }
    if (*io).chan.is_null() {
        if is_play != 0 { (*fsi).playback.handler = ptr::addr_of!(fsi_pio_push_handler); } else { (*fsi).capture.handler = ptr::addr_of!(fsi_pio_pop_handler); }
        dev_info(dev, c"switch handler (dma => pio)\n".as_ptr());
        return fsi_stream_probe(fsi, dev);
    }
    0
}
unsafe extern "C" fn fsi_dma_remove(fsi: *mut fsi_priv, io: *mut fsi_stream) -> c_int {
    fsi_stream_stop(fsi, io);
    if !(*io).chan.is_null() { dma_release_channel((*io).chan); }
    (*io).chan = ptr::null_mut();
    0
}
static fsi_dma_push_handler: fsi_stream_handler = fsi_stream_handler {
    init: Some(fsi_dma_init), quit: None, probe: Some(fsi_dma_probe), transfer: Some(fsi_dma_transfer), remove: Some(fsi_dma_remove), start_stop: Some(fsi_dma_push_start_stop),
};

/*
 * dai ops
 */
unsafe fn fsi_fifo_init(fsi: *mut fsi_priv, io: *mut fsi_stream, dev: *mut device) {
    let master = fsi_get_master(fsi);
    let is_play = fsi_stream_is_play(fsi, io);
    let mut shift = fsi_master_read(master, MST_FIFO_SZ);
    shift >>= fsi_get_port_shift(fsi, io);
    shift &= FIFO_SZ_MASK;
    let mut frame_capa = 256 << shift;
    dev_dbg(dev, c"fifo = %d words\n".as_ptr(), frame_capa);
    let mut i = 1;
    while i < (*fsi).chan_num {
        frame_capa >>= 1;
        i <<= 1;
    }
    dev_dbg(dev, c"%d channel %d store\n".as_ptr(), (*fsi).chan_num, frame_capa);
    (*io).fifo_sample_capa = fsi_frame2sample(fsi, frame_capa);
    if is_play != 0 {
        fsi_reg_write(fsi, REG_DOFF_CTL, IRQ_HALF);
        fsi_reg_mask_set(fsi, REG_DOFF_CTL, FIFO_CLR, FIFO_CLR);
    } else {
        fsi_reg_write(fsi, REG_DIFF_CTL, IRQ_HALF);
        fsi_reg_mask_set(fsi, REG_DIFF_CTL, FIFO_CLR, FIFO_CLR);
    }
}

unsafe fn fsi_hw_startup(fsi: *mut fsi_priv, io: *mut fsi_stream, dev: *mut device) -> c_int {
    let mut data = 0;
    let mut ret = clk_enable((*(*fsi).master).clk_spu);
    if ret != 0 { return ret; }
    if fsi_is_clk_master(fsi) != 0 { data = DIMD | DOMD; }
    fsi_reg_mask_set(fsi, REG_CKG1, DIMD | DOMD, data);
    data = 0;
    if (*fsi).bit_clk_inv != 0 { data |= 1 << 0; }
    if (*fsi).lr_clk_inv != 0 { data |= 1 << 4; }
    if fsi_is_clk_master(fsi) != 0 { data <<= 8; }
    fsi_reg_write(fsi, REG_CKG2, data);
    if fsi_is_spdif(fsi) != 0 {
        fsi_spdif_clk_ctrl(fsi, 1);
        fsi_reg_mask_set(fsi, REG_OUT_SEL, DMMD, DMMD);
    }
    data = 0;
    match (*io).sample_width {
        2 => data = BUSOP_GET_16((*io).bus_option),
        4 => data = BUSOP_GET_24((*io).bus_option),
        _ => {}
    }
    fsi_format_bus_setup(fsi, io, data, dev);
    fsi_irq_disable(fsi, io);
    fsi_irq_clear_status(fsi);
    fsi_fifo_init(fsi, io, dev);
    if fsi_is_clk_master(fsi) != 0 {
        ret = fsi_clk_enable(dev, fsi);
        if ret != 0 { clk_disable((*(*fsi).master).clk_spu); return ret; }
    }
    0
}
unsafe fn fsi_hw_shutdown(fsi: *mut fsi_priv, dev: *mut device) -> c_int {
    if fsi_is_clk_master(fsi) != 0 {
        let ret = fsi_clk_disable(dev, fsi);
        if ret != 0 { return ret; }
    }
    clk_disable((*(*fsi).master).clk_spu);
    0
}

unsafe extern "C" fn fsi_dai_startup(substream: *mut snd_pcm_substream, _dai: *mut snd_soc_dai) -> c_int {
    let fsi = fsi_get_priv(substream);
    fsi_clk_invalid(fsi);
    fsi_clk_prepare(fsi)
}
unsafe extern "C" fn fsi_dai_shutdown(substream: *mut snd_pcm_substream, _dai: *mut snd_soc_dai) {
    let fsi = fsi_get_priv(substream);
    fsi_clk_unprepare(fsi);
    fsi_clk_invalid(fsi);
}
unsafe extern "C" fn fsi_dai_trigger(substream: *mut snd_pcm_substream, cmd: c_int, dai: *mut snd_soc_dai) -> c_int {
    let fsi = fsi_get_priv(substream);
    let io = fsi_stream_get(fsi, substream);
    let mut ret = 0;
    match cmd {
        SNDRV_PCM_TRIGGER_START => {
            fsi_stream_init(fsi, io, substream);
            if ret == 0 { ret = fsi_hw_startup(fsi, io, (*dai).dev); }
            if ret == 0 { ret = fsi_stream_start(fsi, io); }
            if ret == 0 { ret = fsi_stream_transfer(io); }
        }
        SNDRV_PCM_TRIGGER_STOP => {
            fsi_stream_stop(fsi, io);
            fsi_stream_quit(fsi, io);
            if ret == 0 { ret = fsi_hw_shutdown(fsi, (*dai).dev); }
        }
        _ => {}
    }
    ret
}
unsafe fn fsi_set_fmt_dai(fsi: *mut fsi_priv, fmt: c_uint) -> c_int {
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => { (*fsi).fmt = CR_I2S; (*fsi).chan_num = 2; }
        SND_SOC_DAIFMT_LEFT_J => { (*fsi).fmt = CR_PCM; (*fsi).chan_num = 2; }
        _ => return -EINVAL,
    }
    0
}
unsafe fn fsi_set_fmt_spdif(fsi: *mut fsi_priv) -> c_int {
    let master = fsi_get_master(fsi);
    if fsi_version(master) < 2 { return -EINVAL; }
    (*fsi).fmt = CR_DTMD_SPDIF_PCM | CR_PCM;
    (*fsi).chan_num = 2;
    0
}
unsafe extern "C" fn fsi_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let fsi = fsi_get_priv_frm_dai(dai);
    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FC => {}
        SND_SOC_DAIFMT_BP_FP => (*fsi).clk_master = 1,
        _ => return -EINVAL,
    }
    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_IF => { (*fsi).bit_clk_inv = 0; (*fsi).lr_clk_inv = 1; }
        SND_SOC_DAIFMT_IB_NF => { (*fsi).bit_clk_inv = 1; (*fsi).lr_clk_inv = 0; }
        SND_SOC_DAIFMT_IB_IF => { (*fsi).bit_clk_inv = 1; (*fsi).lr_clk_inv = 1; }
        _ => { (*fsi).bit_clk_inv = 0; (*fsi).lr_clk_inv = 0; }
    }
    if fsi_is_spdif(fsi) != 0 { fsi_set_fmt_spdif(fsi) } else { fsi_set_fmt_dai(fsi, fmt & SND_SOC_DAIFMT_FORMAT_MASK) }
}
unsafe extern "C" fn fsi_dai_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, _dai: *mut snd_soc_dai) -> c_int {
    let fsi = fsi_get_priv(substream);
    if fsi_is_clk_master(fsi) != 0 { fsi_clk_valid(fsi, params_rate(params)); }
    0
}

static fsi_dai_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_I2S |
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J |
    SND_SOC_POSSIBLE_DAIFMT_NB_NF |
    SND_SOC_POSSIBLE_DAIFMT_NB_IF |
    SND_SOC_POSSIBLE_DAIFMT_IB_NF |
    SND_SOC_POSSIBLE_DAIFMT_IB_IF;

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_int,
}
static fsi_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(fsi_dai_startup),
    shutdown: Some(fsi_dai_shutdown),
    trigger: Some(fsi_dai_trigger),
    set_fmt: Some(fsi_dai_set_fmt),
    hw_params: Some(fsi_dai_hw_params),
    auto_selectable_formats: ptr::addr_of!(fsi_dai_formats),
    num_auto_selectable_formats: 1,
};

/*
 * pcm ops
 */
#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: u64,
    pub buffer_bytes_max: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub fifo_size: c_uint,
}
static fsi_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID,
    buffer_bytes_max: 64 * 1024,
    period_bytes_min: 32,
    period_bytes_max: 8192,
    periods_min: 1,
    periods_max: 32,
    fifo_size: 256,
};

unsafe extern "C" fn fsi_pcm_open(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;
    snd_soc_set_runtime_hwparams(substream, ptr::addr_of!(fsi_pcm_hardware));
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS)
}
unsafe extern "C" fn fsi_pointer(_component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t {
    let fsi = fsi_get_priv(substream);
    let io = fsi_stream_get(fsi, substream);
    fsi_sample2frame(fsi, (*io).buff_sample_pos) as snd_pcm_uframes_t
}

/*
 * snd_soc_component
 */
const PREALLOC_BUFFER: c_uint = 32 * 1024;
const PREALLOC_BUFFER_MAX: c_uint = 32 * 1024;

unsafe extern "C" fn fsi_pcm_new(_component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime) -> c_int {
    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_DEV, (*(*(*rtd).card).snd_card).dev, PREALLOC_BUFFER, PREALLOC_BUFFER_MAX);
    0
}

/*
 * alsa struct
 */
#[repr(C)] pub struct snd_soc_pcm_stream { pub rates: u32, pub formats: u64, pub channels_min: c_uint, pub channels_max: c_uint }
#[repr(C)] pub struct snd_soc_dai_driver { pub name: *const c_char, pub playback: snd_soc_pcm_stream, pub capture: snd_soc_pcm_stream, pub ops: *const snd_soc_dai_ops }
static mut fsi_soc_dai: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: c"fsia-dai".as_ptr(),
        playback: snd_soc_pcm_stream { rates: FSI_RATES, formats: FSI_FMTS, channels_min: 2, channels_max: 2 },
        capture: snd_soc_pcm_stream { rates: FSI_RATES, formats: FSI_FMTS, channels_min: 2, channels_max: 2 },
        ops: ptr::addr_of!(fsi_dai_ops),
    },
    snd_soc_dai_driver {
        name: c"fsib-dai".as_ptr(),
        playback: snd_soc_pcm_stream { rates: FSI_RATES, formats: FSI_FMTS, channels_min: 2, channels_max: 2 },
        capture: snd_soc_pcm_stream { rates: FSI_RATES, formats: FSI_FMTS, channels_min: 2, channels_max: 2 },
        ops: ptr::addr_of!(fsi_dai_ops),
    },
];

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
}
static fsi_soc_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"fsi".as_ptr(),
    open: Some(fsi_pcm_open),
    pointer: Some(fsi_pointer),
    pcm_new: Some(fsi_pcm_new),
};

/*
 * platform function
 */
#[repr(C)] struct fsi_of_parse_property { name: *const c_char, val: c_uint }
unsafe fn fsi_of_parse(name: *mut c_char, np: *mut device_node, info: *mut sh_fsi_port_info, dev: *mut device) {
    let mut i = 0;
    let mut prop = [0 as c_char; 128];
    let mut flags: c_ulong = 0;
    let of_parse_property = [
        fsi_of_parse_property { name: c"spdif-connection".as_ptr(), val: SH_FSI_FMT_SPDIF },
        fsi_of_parse_property { name: c"stream-mode-support".as_ptr(), val: SH_FSI_ENABLE_STREAM_MODE },
        fsi_of_parse_property { name: c"use-internal-clock".as_ptr(), val: SH_FSI_CLK_CPG },
    ];
    while i < of_parse_property.len() {
        sprintf(prop.as_mut_ptr(), c"%s,%s".as_ptr(), name, of_parse_property[i].name);
        if of_property_present(np, prop.as_ptr()) != 0 { flags |= of_parse_property[i].val as c_ulong; }
        i += 1;
    }
    (*info).flags = flags;
    dev_dbg(dev, c"%s flags : %lx\n".as_ptr(), name, (*info).flags);
}
unsafe fn fsi_port_info_init(fsi: *mut fsi_priv, info: *mut sh_fsi_port_info) {
    if (*info).flags & SH_FSI_FMT_SPDIF as c_ulong != 0 { (*fsi).spdif = 1; }
    if (*info).flags & SH_FSI_CLK_CPG as c_ulong != 0 { (*fsi).clk_cpg = 1; }
    if (*info).flags & SH_FSI_ENABLE_STREAM_MODE as c_ulong != 0 { (*fsi).enable_stream = 1; }
}
unsafe fn fsi_handler_init(fsi: *mut fsi_priv, info: *mut sh_fsi_port_info) {
    (*fsi).playback.handler = ptr::addr_of!(fsi_pio_push_handler); /* default PIO */
    (*fsi).playback.priv_ = fsi;
    (*fsi).capture.handler = ptr::addr_of!(fsi_pio_pop_handler); /* default PIO */
    (*fsi).capture.priv_ = fsi;
    if (*info).tx_id != 0 {
        (*fsi).playback.dma_id = (*info).tx_id;
        (*fsi).playback.handler = ptr::addr_of!(fsi_dma_push_handler);
    }
}

static fsi1_core: fsi_core = fsi_core {
    ver: 1,
    /* Interrupt */
    int_st: INT_ST,
    iemsk: IEMSK,
    imsk: IMSK,
    a_mclk: 0,
    b_mclk: 0,
};
static fsi2_core: fsi_core = fsi_core {
    ver: 2,
    /* Interrupt */
    int_st: CPU_INT_ST,
    iemsk: CPU_IEMSK,
    imsk: CPU_IMSK,
    a_mclk: A_MST_CTLR,
    b_mclk: B_MST_CTLR,
};

#[repr(C)] pub struct of_device_id { pub compatible: *const c_char, pub data: *const c_void }
static fsi_of_match: [of_device_id; 3] = [
    of_device_id { compatible: c"renesas,sh_fsi".as_ptr(), data: ptr::addr_of!(fsi1_core) as *const c_void },
    of_device_id { compatible: c"renesas,sh_fsi2".as_ptr(), data: ptr::addr_of!(fsi2_core) as *const c_void },
    of_device_id { compatible: ptr::null(), data: ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, fsi_of_match); */

#[repr(C)] pub struct platform_device_id { _dummy: c_int }
static fsi_id_table: [platform_device_id; 1] = [
    /* an array with no valid entry prevents matching on driver name */
    platform_device_id { _dummy: 0 },
];
/* MODULE_DEVICE_TABLE(platform, fsi_id_table); */

unsafe extern "C" fn fsi_probe(pdev: *mut platform_device) -> c_int {
    let mut info: sh_fsi_platform_info = core::mem::zeroed();
    fsi_of_parse(c"fsia".as_ptr() as *mut c_char, (*pdev).dev.of_node, ptr::addr_of_mut!(info.port_a), ptr::addr_of_mut!((*pdev).dev));
    fsi_of_parse(c"fsib".as_ptr() as *mut c_char, (*pdev).dev.of_node, ptr::addr_of_mut!(info.port_b), ptr::addr_of_mut!((*pdev).dev));
    let core = of_device_get_match_data(ptr::addr_of_mut!((*pdev).dev)) as *const fsi_core;
    if core.is_null() {
        dev_err(ptr::addr_of_mut!((*pdev).dev), c"unknown fsi device\n".as_ptr());
        return -ENODEV;
    }
    let res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        dev_err(ptr::addr_of_mut!((*pdev).dev), c"Not enough FSI platform resources.\n".as_ptr());
        return -ENODEV;
    }
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    let master = devm_kzalloc(ptr::addr_of_mut!((*pdev).dev), size_of::<fsi_master>(), GFP_KERNEL) as *mut fsi_master;
    if master.is_null() { return -ENOMEM; }
    (*master).base = devm_ioremap(ptr::addr_of_mut!((*pdev).dev), (*res).start, resource_size(res)) as *mut u8;
    if (*master).base.is_null() {
        dev_err(ptr::addr_of_mut!((*pdev).dev), c"Unable to ioremap FSI registers.\n".as_ptr());
        return -ENXIO;
    }
    (*master).core = core;
    spin_lock_init(ptr::addr_of_mut!((*master).lock));
    let mut fsi = ptr::addr_of_mut!((*master).fsia);
    (*fsi).base = (*master).base;
    (*fsi).phys = (*res).start;
    (*fsi).master = master;
    fsi_port_info_init(fsi, ptr::addr_of_mut!(info.port_a));
    fsi_handler_init(fsi, ptr::addr_of_mut!(info.port_a));
    let mut ret = fsi_clk_init(ptr::addr_of_mut!((*pdev).dev), fsi);
    if ret != 0 { dev_err(ptr::addr_of_mut!((*pdev).dev), c"FSIA clk init failed\n".as_ptr()); return ret; }
    ret = fsi_stream_probe(fsi, ptr::addr_of_mut!((*pdev).dev));
    if ret < 0 { dev_err(ptr::addr_of_mut!((*pdev).dev), c"FSIA stream probe failed\n".as_ptr()); return ret; }
    fsi = ptr::addr_of_mut!((*master).fsib);
    (*fsi).base = (*master).base.add(0x40);
    (*fsi).phys = (*res).start + 0x40;
    (*fsi).master = master;
    fsi_port_info_init(fsi, ptr::addr_of_mut!(info.port_b));
    fsi_handler_init(fsi, ptr::addr_of_mut!(info.port_b));
    ret = fsi_clk_init(ptr::addr_of_mut!((*pdev).dev), fsi);
    if ret != 0 {
        dev_err(ptr::addr_of_mut!((*pdev).dev), c"FSIB clk init failed\n".as_ptr());
        fsi_stream_remove(ptr::addr_of_mut!((*master).fsia));
        return ret;
    }
    ret = fsi_stream_probe(fsi, ptr::addr_of_mut!((*pdev).dev));
    if ret < 0 {
        dev_err(ptr::addr_of_mut!((*pdev).dev), c"FSIB stream probe failed\n".as_ptr());
        fsi_stream_remove(ptr::addr_of_mut!((*master).fsia));
        return ret;
    }
    pm_runtime_enable(ptr::addr_of_mut!((*pdev).dev));
    dev_set_drvdata(ptr::addr_of_mut!((*pdev).dev), master as *mut c_void);
    ret = devm_request_irq(ptr::addr_of_mut!((*pdev).dev), irq, Some(fsi_interrupt), 0, dev_name(ptr::addr_of_mut!((*pdev).dev)), master as *mut c_void);
    if ret != 0 {
        dev_err(ptr::addr_of_mut!((*pdev).dev), c"irq request err\n".as_ptr());
        pm_runtime_disable(ptr::addr_of_mut!((*pdev).dev));
        fsi_stream_remove(ptr::addr_of_mut!((*master).fsib));
        fsi_stream_remove(ptr::addr_of_mut!((*master).fsia));
        return ret;
    }
    ret = devm_snd_soc_register_component(ptr::addr_of_mut!((*pdev).dev), ptr::addr_of!(fsi_soc_component), ptr::addr_of_mut!(fsi_soc_dai) as *mut snd_soc_dai_driver, fsi_soc_dai.len());
    if ret < 0 {
        dev_err(ptr::addr_of_mut!((*pdev).dev), c"cannot snd component register\n".as_ptr());
        pm_runtime_disable(ptr::addr_of_mut!((*pdev).dev));
        fsi_stream_remove(ptr::addr_of_mut!((*master).fsib));
        fsi_stream_remove(ptr::addr_of_mut!((*master).fsia));
        return ret;
    }
    ret
}

unsafe extern "C" fn fsi_remove(pdev: *mut platform_device) {
    let master = dev_get_drvdata(ptr::addr_of_mut!((*pdev).dev)) as *mut fsi_master;
    pm_runtime_disable(ptr::addr_of_mut!((*pdev).dev));
    fsi_stream_remove(ptr::addr_of_mut!((*master).fsia));
    fsi_stream_remove(ptr::addr_of_mut!((*master).fsib));
}

unsafe fn __fsi_suspend(fsi: *mut fsi_priv, io: *mut fsi_stream, dev: *mut device) {
    if fsi_stream_is_working(fsi, io) == 0 { return; }
    fsi_stream_stop(fsi, io);
    fsi_hw_shutdown(fsi, dev);
}
unsafe fn __fsi_resume(fsi: *mut fsi_priv, io: *mut fsi_stream, dev: *mut device) {
    if fsi_stream_is_working(fsi, io) == 0 { return; }
    fsi_hw_startup(fsi, io, dev);
    fsi_stream_start(fsi, io);
}
unsafe extern "C" fn fsi_suspend(dev: *mut device) -> c_int {
    let master = dev_get_drvdata(dev) as *mut fsi_master;
    let fsia = ptr::addr_of_mut!((*master).fsia);
    let fsib = ptr::addr_of_mut!((*master).fsib);
    __fsi_suspend(fsia, ptr::addr_of_mut!((*fsia).playback), dev);
    __fsi_suspend(fsia, ptr::addr_of_mut!((*fsia).capture), dev);
    __fsi_suspend(fsib, ptr::addr_of_mut!((*fsib).playback), dev);
    __fsi_suspend(fsib, ptr::addr_of_mut!((*fsib).capture), dev);
    0
}
unsafe extern "C" fn fsi_resume(dev: *mut device) -> c_int {
    let master = dev_get_drvdata(dev) as *mut fsi_master;
    let fsia = ptr::addr_of_mut!((*master).fsia);
    let fsib = ptr::addr_of_mut!((*master).fsib);
    __fsi_resume(fsia, ptr::addr_of_mut!((*fsia).playback), dev);
    __fsi_resume(fsia, ptr::addr_of_mut!((*fsia).capture), dev);
    __fsi_resume(fsib, ptr::addr_of_mut!((*fsib).playback), dev);
    __fsi_resume(fsib, ptr::addr_of_mut!((*fsib).capture), dev);
    0
}

#[repr(C)] pub struct dev_pm_ops { pub suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>, pub resume: Option<unsafe extern "C" fn(*mut device) -> c_int> }
static fsi_pm_ops: dev_pm_ops = dev_pm_ops { suspend: Some(fsi_suspend), resume: Some(fsi_resume) };

#[repr(C)] pub struct device_driver { pub name: *const c_char, pub pm: *const dev_pm_ops, pub of_match_table: *const of_device_id }
#[repr(C)] pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub id_table: *const platform_device_id,
}
static mut fsi_driver: platform_driver = platform_driver {
    driver: device_driver { name: c"fsi-pcm-audio".as_ptr(), pm: ptr::addr_of!(fsi_pm_ops), of_match_table: ptr::addr_of!(fsi_of_match) as *const of_device_id },
    probe: Some(fsi_probe),
    remove: Some(fsi_remove),
    id_table: ptr::addr_of!(fsi_id_table) as *const platform_device_id,
};
/* module_platform_driver(fsi_driver); */

/* MODULE_LICENSE("GPL v2"); */
/* MODULE_DESCRIPTION("SuperH onchip FSI audio driver"); */
/* MODULE_AUTHOR("Kuninori Morimoto <morimoto.kuninori@renesas.com>"); */
/* MODULE_ALIAS("platform:fsi-pcm-audio"); */

unsafe extern "C" {
    static SNDRV_PCM_RATE_8000_96000: u32;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_BC_FC: c_uint;
    static SND_SOC_DAIFMT_BP_FP: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_IF: c_uint;
    static SND_SOC_DAIFMT_IB_NF: c_uint;
    static SND_SOC_DAIFMT_IB_IF: c_uint;
    static SND_SOC_POSSIBLE_DAIFMT_I2S: u64;
    static SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64;
    static SND_SOC_POSSIBLE_DAIFMT_NB_NF: u64;
    static SND_SOC_POSSIBLE_DAIFMT_NB_IF: u64;
    static SND_SOC_POSSIBLE_DAIFMT_IB_NF: u64;
    static SND_SOC_POSSIBLE_DAIFMT_IB_IF: u64;
    static SNDRV_PCM_INFO_INTERLEAVED: u64;
    static SNDRV_PCM_INFO_MMAP: u64;
    static SNDRV_PCM_INFO_MMAP_VALID: u64;
    static SNDRV_PCM_HW_PARAM_PERIODS: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static SH_FSI_FMT_SPDIF: c_uint;
    static SH_FSI_ENABLE_STREAM_MODE: c_uint;
    static SH_FSI_CLK_CPG: c_uint;

    fn __raw_writel(data: u32, reg: *mut u32);
    fn __raw_readl(reg: *mut u32) -> u32;
    fn spin_lock_irqsave(lock: *mut spinlock_t);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn samples_to_bytes(runtime: *mut snd_pcm_runtime, samples: c_int) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn clk_prepare(clk: *mut clk) -> c_int;
    fn clk_unprepare(clk: *mut clk);
    fn clk_enable(clk: *mut clk) -> c_int;
    fn clk_disable(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_ulong;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_clk_get_optional(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn udelay(usecs: c_ulong);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn dmaengine_prep_dma_cyclic(chan: *mut dma_chan, buf_addr: phys_addr_t, buf_len: c_uint, period_len: c_uint, dir: dma_transfer_direction, flags: c_uint) -> *mut dma_async_tx_descriptor;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> c_uint;
    fn dmaengine_submit(desc: *mut dma_async_tx_descriptor) -> c_int;
    fn dma_async_issue_pending(chan: *mut dma_chan);
    fn dmaengine_terminate_all(chan: *mut dma_chan);
    fn dma_request_chan(dev: *mut device, name: *const c_char) -> *mut dma_chan;
    fn dmaengine_slave_config(chan: *mut dma_chan, config: *mut dma_slave_config) -> c_int;
    fn dma_release_channel(chan: *mut dma_chan);
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: c_int) -> c_int;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, ty: c_int, dev: *mut device, size: c_uint, max: c_uint);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_ulong;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn of_property_present(np: *mut device_node, propname: *const c_char) -> c_int;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn platform_get_resource(pdev: *mut platform_device, ty: c_uint, num: c_uint) -> *mut resource;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_ioremap(dev: *mut device, offset: phys_addr_t, size: c_ulong) -> *mut c_void;
    fn resource_size(res: *mut resource) -> c_ulong;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_uint, name: *const c_char, data: *mut c_void) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: usize) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
