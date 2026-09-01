// SPDX-License-Identifier: GPL-2.0
//
// Freescale S/PDIF ALSA SoC Digital Audio Interface (DAI) driver
//
// Copyright (C) 2013 Freescale Semiconductor, Inc.
//
// Based on stmp3xxx_spdif_dai.c
// Vladimir Barinov <vbarinov@embeddedalley.com>
// Copyright 2008 SigmaTel, Inc
// Copyright 2008 Embedded Alley Solutions, Inc

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type bool_t = bool;
type irqreturn_t = c_int;

const FSL_SPDIF_TXFIFO_WML: u32 = 0x8;
const FSL_SPDIF_RXFIFO_WML: u32 = 0x8;

const INTR_FOR_PLAYBACK: u32 = INT_TXFIFO_RESYNC;
const INTR_FOR_CAPTURE: u32 = INT_SYM_ERR | INT_BIT_ERR | INT_URX_FUL |
    INT_URX_OV | INT_QRX_FUL | INT_QRX_OV |
    INT_UQ_SYNC | INT_UQ_ERR | INT_RXFIFO_RESYNC |
    INT_LOSS_LOCK | INT_DPLL_LOCKED;

#[inline]
const fn SIE_INTR_FOR(tx: bool) -> u32 {
    if tx { INTR_FOR_PLAYBACK } else { INTR_FOR_CAPTURE }
}

/* Index list for the values that has if (DPLL Locked) condition */
static mut srpc_dpll_locked: [u8; 7] = [0x0, 0x1, 0x2, 0x3, 0x4, 0xa, 0xb];
const SRPC_NODPLL_START1: u32 = 0x5;
const SRPC_NODPLL_START2: u32 = 0xc;

const DEFAULT_RXCLK_SRC: u8 = 1;

const RX_SAMPLE_RATE_KCONTROL: *const c_char = b"RX Sample Rate\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: u32,
    pub formats: u64,
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
}
#[repr(C)]
pub struct snd_card { _private: [u8; 0] }
#[repr(C)]
pub struct snd_kcontrol {
    pub id: snd_ctl_elem_id,
}
#[repr(C)]
pub struct snd_ctl_elem_id { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: u64,
    pub maxburst: u32,
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}
#[repr(C)]
pub struct snd_pcm_hw_params { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub pcm: *mut snd_pcm,
}
#[repr(C)]
pub struct snd_pcm {
    pub streams: [snd_pcm_stream; 2],
}
#[repr(C)]
pub struct snd_pcm_stream {
    pub substream_count: c_int,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub driver: *mut snd_soc_dai_driver,
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_component {
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
    pub dai_link: *mut c_void,
}
#[repr(C)]
pub struct snd_ctl_elem_info {
    pub type_: c_uint,
    pub count: c_uint,
    pub value: snd_ctl_elem_info_value,
}
#[repr(C)]
pub union snd_ctl_elem_info_value {
    pub integer: snd_ctl_elem_info_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info_integer {
    pub min: i64,
    pub max: i64,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_union,
}
#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub iec958: snd_aes_iec958,
    pub bytes: snd_ctl_elem_value_bytes,
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_aes_iec958 {
    pub status: [u8; 24],
    pub subcode: [u8; SPDIF_UBITS_SIZE as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_bytes {
    pub data: [u8; SPDIF_UBITS_SIZE as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}
#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub access: c_uint,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}
#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}
#[repr(C)]
pub struct resource {
    pub start: u64,
}
#[repr(C)]
pub struct dev_pm_ops { _private: [u8; 0] }
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}
#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

/**
 * struct fsl_spdif_soc_data: soc specific data
 *
 * @imx: for imx platform
 * @shared_root_clock: flag of sharing a clock source with others;
 *                     so the driver shouldn't set root clock rate
 * @raw_capture_mode: if raw capture mode support
 * @cchannel_192b: if there are registers for 192bits C channel data
 * @interrupts: interrupt number
 * @tx_burst: tx maxburst size
 * @rx_burst: rx maxburst size
 * @tx_formats: tx supported data format
 */
#[repr(C)]
struct fsl_spdif_soc_data {
    imx: bool,
    shared_root_clock: bool,
    raw_capture_mode: bool,
    cchannel_192b: bool,
    interrupts: u32,
    tx_burst: u32,
    rx_burst: u32,
    tx_formats: u64,
}

/*
 * SPDIF control structure
 * Defines channel status, subcode and Q sub
 */
#[repr(C)]
struct spdif_mixer_control {
    /* spinlock to access control data */
    ctl_lock: spinlock_t,

    /* IEC958 channel tx status bit */
    ch_status: [u8; 4],

    /* User bits */
    subcode: [u8; (2 * SPDIF_UBITS_SIZE) as usize],

    /* Q subcode part of user bits */
    qsub: [u8; (2 * SPDIF_QSUB_SIZE) as usize],

    /* Buffer offset for U/Q */
    upos: u32,
    qpos: u32,

    /* Ready buffer index of the two buffers */
    ready_buf: u32,
}

/**
 * struct fsl_spdif_priv - Freescale SPDIF private data
 */
#[repr(C)]
struct fsl_spdif_priv {
    soc: *const fsl_spdif_soc_data,
    fsl_spdif_control: spdif_mixer_control,
    cpu_dai_drv: snd_soc_dai_driver,
    snd_card: *mut snd_card,
    rxrate_kcontrol: *mut snd_kcontrol,
    pdev: *mut platform_device,
    regmap: *mut regmap,
    dpll_locked: bool,
    txrate: [u32; SPDIF_TXRATE_MAX as usize],
    txclk_df: [u8; SPDIF_TXRATE_MAX as usize],
    sysclk_df: [u16; SPDIF_TXRATE_MAX as usize],
    txclk_src: [u8; SPDIF_TXRATE_MAX as usize],
    rxclk_src: u8,
    txclk: [*mut clk; STC_TXCLK_SRC_MAX as usize],
    rxclk: *mut clk,
    coreclk: *mut clk,
    sysclk: *mut clk,
    spbaclk: *mut clk,
    dma_params_tx: snd_dmaengine_dai_dma_data,
    dma_params_rx: snd_dmaengine_dai_dma_data,
    /* regcache for SRPC */
    regcache_srpc: u32,
    bypass: bool,
    pll8k_clk: *mut clk,
    pll11k_clk: *mut clk,
}

static fsl_spdif_vf610: fsl_spdif_soc_data = fsl_spdif_soc_data {
    imx: false, shared_root_clock: false, raw_capture_mode: false, cchannel_192b: false,
    interrupts: 1, tx_burst: FSL_SPDIF_TXFIFO_WML, rx_burst: FSL_SPDIF_RXFIFO_WML,
    tx_formats: FSL_SPDIF_FORMATS_PLAYBACK,
};

static fsl_spdif_imx35: fsl_spdif_soc_data = fsl_spdif_soc_data {
    imx: true, shared_root_clock: false, raw_capture_mode: false, cchannel_192b: false,
    interrupts: 1, tx_burst: FSL_SPDIF_TXFIFO_WML, rx_burst: FSL_SPDIF_RXFIFO_WML,
    tx_formats: FSL_SPDIF_FORMATS_PLAYBACK,
};

static fsl_spdif_imx6sx: fsl_spdif_soc_data = fsl_spdif_soc_data {
    imx: true, shared_root_clock: true, raw_capture_mode: false, cchannel_192b: false,
    interrupts: 1, tx_burst: FSL_SPDIF_TXFIFO_WML, rx_burst: FSL_SPDIF_RXFIFO_WML,
    tx_formats: FSL_SPDIF_FORMATS_PLAYBACK,
};

static fsl_spdif_imx8qm: fsl_spdif_soc_data = fsl_spdif_soc_data {
    imx: true, shared_root_clock: true, raw_capture_mode: false, cchannel_192b: false,
    interrupts: 2, tx_burst: 2, rx_burst: 2, tx_formats: SNDRV_PCM_FMTBIT_S24_LE,
};

static fsl_spdif_imx8mm: fsl_spdif_soc_data = fsl_spdif_soc_data {
    imx: true, shared_root_clock: false, raw_capture_mode: true, cchannel_192b: false,
    interrupts: 1, tx_burst: FSL_SPDIF_TXFIFO_WML, rx_burst: FSL_SPDIF_RXFIFO_WML,
    tx_formats: FSL_SPDIF_FORMATS_PLAYBACK,
};

static fsl_spdif_imx8ulp: fsl_spdif_soc_data = fsl_spdif_soc_data {
    imx: true, shared_root_clock: true, raw_capture_mode: false, cchannel_192b: true,
    interrupts: 1, tx_burst: 2, rx_burst: 2, tx_formats: SNDRV_PCM_FMTBIT_S24_LE,
};

/* Check if clk is a root clock that does not share clock source with others */
#[inline]
unsafe fn fsl_spdif_can_set_clk_rate(spdif: *mut fsl_spdif_priv, clk_id: c_int) -> bool {
    clk_id == STC_TXCLK_SPDIF_ROOT && !(*(*spdif).soc).shared_root_clock
}

/* DPLL locked and lock loss interrupt handler */
unsafe fn spdif_irq_dpll_lock(spdif_priv: *mut fsl_spdif_priv) {
    let regmap = (*spdif_priv).regmap;
    let pdev = (*spdif_priv).pdev;
    let mut locked: u32 = 0;

    regmap_read(regmap, REG_SPDIF_SRPC, &mut locked);
    locked &= SRPC_DPLL_LOCKED;

    dev_dbg(&mut (*pdev).dev, b"isr: Rx dpll %s \n\0".as_ptr() as *const c_char,
            if locked != 0 { b"locked\0".as_ptr() } else { b"loss lock\0".as_ptr() });

    (*spdif_priv).dpll_locked = locked != 0;

    if !(*spdif_priv).snd_card.is_null() && !(*spdif_priv).rxrate_kcontrol.is_null() {
        snd_ctl_notify((*spdif_priv).snd_card,
                       SNDRV_CTL_EVENT_MASK_VALUE,
                       &mut (*(*spdif_priv).rxrate_kcontrol).id);
    }
}

/* Receiver found illegal symbol interrupt handler */
unsafe fn spdif_irq_sym_error(spdif_priv: *mut fsl_spdif_priv) {
    let regmap = (*spdif_priv).regmap;
    let pdev = (*spdif_priv).pdev;

    dev_dbg(&mut (*pdev).dev, b"isr: receiver found illegal symbol\n\0".as_ptr() as *const c_char);

    /* Clear illegal symbol if DPLL unlocked since no audio stream */
    if !(*spdif_priv).dpll_locked {
        regmap_update_bits(regmap, REG_SPDIF_SIE, INT_SYM_ERR, 0);
    }
}

/* U/Q Channel receive register full */
unsafe fn spdif_irq_uqrx_full(spdif_priv: *mut fsl_spdif_priv, name: c_char) {
    let ctrl = &mut (*spdif_priv).fsl_spdif_control as *mut spdif_mixer_control;
    let regmap = (*spdif_priv).regmap;
    let pdev = (*spdif_priv).pdev;
    let mut size: u32 = 0;
    let mut val: u32 = 0;
    let mut reg: u32 = 0;
    let pos: *mut u32;

    match name as u8 as char {
        'U' => {
            pos = &mut (*ctrl).upos;
            size = SPDIF_UBITS_SIZE;
            reg = REG_SPDIF_SRU;
        }
        'Q' => {
            pos = &mut (*ctrl).qpos;
            size = SPDIF_QSUB_SIZE;
            reg = REG_SPDIF_SRQ;
        }
        _ => {
            dev_err(&mut (*pdev).dev, b"unsupported channel name\n\0".as_ptr() as *const c_char);
            return;
        }
    }

    dev_dbg(&mut (*pdev).dev, b"isr: %c Channel receive register full\n\0".as_ptr() as *const c_char, name as c_int);

    if *pos >= size * 2 {
        *pos = 0;
    } else if unlikely(((*pos % size) + 3 > size) as c_int) != 0 {
        dev_err(&mut (*pdev).dev, b"User bit receive buffer overflow\n\0".as_ptr() as *const c_char);
        return;
    }

    regmap_read(regmap, reg, &mut val);
    (*ctrl).subcode[*pos as usize] = (val >> 16) as u8; *pos += 1;
    (*ctrl).subcode[*pos as usize] = (val >> 8) as u8; *pos += 1;
    (*ctrl).subcode[*pos as usize] = val as u8; *pos += 1;
}

/* U/Q Channel sync found */
unsafe fn spdif_irq_uq_sync(spdif_priv: *mut fsl_spdif_priv) {
    let ctrl = &mut (*spdif_priv).fsl_spdif_control as *mut spdif_mixer_control;
    let pdev = (*spdif_priv).pdev;

    dev_dbg(&mut (*pdev).dev, b"isr: U/Q Channel sync found\n\0".as_ptr() as *const c_char);

    /* U/Q buffer reset */
    if (*ctrl).qpos == 0 {
        return;
    }

    /* Set ready to this buffer */
    (*ctrl).ready_buf = ((*ctrl).qpos - 1) / SPDIF_QSUB_SIZE + 1;
}

/* U/Q Channel framing error */
unsafe fn spdif_irq_uq_err(spdif_priv: *mut fsl_spdif_priv) {
    let ctrl = &mut (*spdif_priv).fsl_spdif_control as *mut spdif_mixer_control;
    let regmap = (*spdif_priv).regmap;
    let pdev = (*spdif_priv).pdev;
    let mut val: u32 = 0;

    dev_dbg(&mut (*pdev).dev, b"isr: U/Q Channel framing error\n\0".as_ptr() as *const c_char);

    /* Read U/Q data to clear the irq and do buffer reset */
    regmap_read(regmap, REG_SPDIF_SRU, &mut val);
    regmap_read(regmap, REG_SPDIF_SRQ, &mut val);

    /* Drop this U/Q buffer */
    (*ctrl).ready_buf = 0;
    (*ctrl).upos = 0;
    (*ctrl).qpos = 0;
}

/* Get spdif interrupt status and clear the interrupt */
unsafe fn spdif_intr_status_clear(spdif_priv: *mut fsl_spdif_priv) -> u32 {
    let regmap = (*spdif_priv).regmap;
    let mut val: u32 = 0;
    let mut val2: u32 = 0;

    regmap_read(regmap, REG_SPDIF_SIS, &mut val);
    regmap_read(regmap, REG_SPDIF_SIE, &mut val2);

    regmap_write(regmap, REG_SPDIF_SIC, val & val2);

    val
}

unsafe extern "C" fn spdif_isr(_irq: c_int, devid: *mut c_void) -> irqreturn_t {
    let spdif_priv = devid as *mut fsl_spdif_priv;
    let pdev = (*spdif_priv).pdev;
    let sis = spdif_intr_status_clear(spdif_priv);

    if sis & INT_DPLL_LOCKED != 0 { spdif_irq_dpll_lock(spdif_priv); }
    if sis & INT_TXFIFO_UNOV != 0 { dev_dbg(&mut (*pdev).dev, b"isr: Tx FIFO under/overrun\n\0".as_ptr() as *const c_char); }
    if sis & INT_TXFIFO_RESYNC != 0 { dev_dbg(&mut (*pdev).dev, b"isr: Tx FIFO resync\n\0".as_ptr() as *const c_char); }
    if sis & INT_CNEW != 0 { dev_dbg(&mut (*pdev).dev, b"isr: cstatus new\n\0".as_ptr() as *const c_char); }
    if sis & INT_VAL_NOGOOD != 0 { dev_dbg(&mut (*pdev).dev, b"isr: validity flag no good\n\0".as_ptr() as *const c_char); }
    if sis & INT_SYM_ERR != 0 { spdif_irq_sym_error(spdif_priv); }
    if sis & INT_BIT_ERR != 0 { dev_dbg(&mut (*pdev).dev, b"isr: receiver found parity bit error\n\0".as_ptr() as *const c_char); }
    if sis & INT_URX_FUL != 0 { spdif_irq_uqrx_full(spdif_priv, b'U' as c_char); }
    if sis & INT_URX_OV != 0 { dev_dbg(&mut (*pdev).dev, b"isr: U Channel receive register overrun\n\0".as_ptr() as *const c_char); }
    if sis & INT_QRX_FUL != 0 { spdif_irq_uqrx_full(spdif_priv, b'Q' as c_char); }
    if sis & INT_QRX_OV != 0 { dev_dbg(&mut (*pdev).dev, b"isr: Q Channel receive register overrun\n\0".as_ptr() as *const c_char); }
    if sis & INT_UQ_SYNC != 0 { spdif_irq_uq_sync(spdif_priv); }
    if sis & INT_UQ_ERR != 0 { spdif_irq_uq_err(spdif_priv); }
    if sis & INT_RXFIFO_UNOV != 0 { dev_dbg(&mut (*pdev).dev, b"isr: Rx FIFO under/overrun\n\0".as_ptr() as *const c_char); }
    if sis & INT_RXFIFO_RESYNC != 0 { dev_dbg(&mut (*pdev).dev, b"isr: Rx FIFO resync\n\0".as_ptr() as *const c_char); }
    if sis & INT_LOSS_LOCK != 0 { spdif_irq_dpll_lock(spdif_priv); }
    /* FIXME: Write Tx FIFO to clear TxEm */
    if sis & INT_TX_EM != 0 { dev_dbg(&mut (*pdev).dev, b"isr: Tx FIFO empty\n\0".as_ptr() as *const c_char); }
    /* FIXME: Read Rx FIFO to clear RxFIFOFul */
    if sis & INT_RXFIFO_FUL != 0 { dev_dbg(&mut (*pdev).dev, b"isr: Rx FIFO full\n\0".as_ptr() as *const c_char); }

    IRQ_HANDLED
}

unsafe fn spdif_softreset(spdif_priv: *mut fsl_spdif_priv) -> c_int {
    let regmap = (*spdif_priv).regmap;
    let mut val: u32 = 0;
    let mut cycle: u32 = 1000;

    regcache_cache_bypass(regmap, true);
    regmap_write(regmap, REG_SPDIF_SCR, SCR_SOFT_RESET);

    /*
     * RESET bit would be cleared after finishing its reset procedure,
     * which typically lasts 8 cycles. 1000 cycles will keep it safe.
     */
    loop {
        regmap_read(regmap, REG_SPDIF_SCR, &mut val);
        if !((val & SCR_SOFT_RESET) != 0 && { let old = cycle; cycle = cycle.wrapping_sub(1); old != 0 }) {
            break;
        }
    }

    regcache_cache_bypass(regmap, false);
    regcache_mark_dirty(regmap);
    regcache_sync(regmap);

    if cycle != 0 { 0 } else { -EBUSY }
}

unsafe fn spdif_set_cstatus(ctrl: *mut spdif_mixer_control, mask: u8, cstatus: u8) {
    (*ctrl).ch_status[3] &= !mask;
    (*ctrl).ch_status[3] |= cstatus & mask;
}

unsafe fn spdif_write_channel_status(spdif_priv: *mut fsl_spdif_priv) {
    let ctrl = &mut (*spdif_priv).fsl_spdif_control as *mut spdif_mixer_control;
    let regmap = (*spdif_priv).regmap;
    let pdev = (*spdif_priv).pdev;
    let mut ch_status: u32;

    ch_status = ((bitrev8((*ctrl).ch_status[0]) as u32) << 16) |
        ((bitrev8((*ctrl).ch_status[1]) as u32) << 8) |
        bitrev8((*ctrl).ch_status[2]) as u32;
    regmap_write(regmap, REG_SPDIF_STCSCH, ch_status);
    dev_dbg(&mut (*pdev).dev, b"STCSCH: 0x%06x\n\0".as_ptr() as *const c_char, ch_status);

    ch_status = (bitrev8((*ctrl).ch_status[3]) as u32) << 16;
    regmap_write(regmap, REG_SPDIF_STCSCL, ch_status);
    dev_dbg(&mut (*pdev).dev, b"STCSCL: 0x%06x\n\0".as_ptr() as *const c_char, ch_status);

    if (*(*spdif_priv).soc).cchannel_192b {
        ch_status = ((bitrev8((*ctrl).ch_status[0]) as u32) << 24) |
            ((bitrev8((*ctrl).ch_status[1]) as u32) << 16) |
            ((bitrev8((*ctrl).ch_status[2]) as u32) << 8) |
            bitrev8((*ctrl).ch_status[3]) as u32;

        regmap_update_bits(regmap, REG_SPDIF_SCR, 0x1000000, 0x1000000);

        /*
         * The first 32bit should be in REG_SPDIF_STCCA_31_0 register,
         * but here we need to set REG_SPDIF_STCCA_191_160 on 8ULP
         * then can get correct result with HDMI analyzer capture.
         * There is a hardware bug here.
         */
        regmap_write(regmap, REG_SPDIF_STCCA_191_160, ch_status);
    }
}

/* Set SPDIF PhaseConfig register for rx clock */
unsafe fn spdif_set_rx_clksrc(spdif_priv: *mut fsl_spdif_priv,
                              gainsel: spdif_gainsel, _dpll_locked: c_int) -> c_int {
    let regmap = (*spdif_priv).regmap;
    let clksrc = (*spdif_priv).rxclk_src;

    if clksrc as c_uint >= SRPC_CLKSRC_MAX || gainsel as c_uint >= GAINSEL_MULTI_MAX {
        return -EINVAL;
    }

    regmap_update_bits(regmap, REG_SPDIF_SRPC,
                       SRPC_CLKSRC_SEL_MASK | SRPC_GAINSEL_MASK,
                       SRPC_CLKSRC_SEL_SET(clksrc as u32) | SRPC_GAINSEL_SET(gainsel as u32));

    0
}

unsafe fn fsl_spdif_probe_txclk(spdif_priv: *mut fsl_spdif_priv, index: spdif_txrate) -> c_int;

unsafe fn spdif_set_sample_rate(substream: *mut snd_pcm_substream, sample_rate: c_int) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let spdif_priv = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut fsl_spdif_priv;
    let ctrl = &mut (*spdif_priv).fsl_spdif_control as *mut spdif_mixer_control;
    let regmap = (*spdif_priv).regmap;
    let pdev = (*spdif_priv).pdev;
    let mut csfs: c_ulong = 0;
    let stc: u32;
    let mask: u32;
    let rate: u32;
    let sysclk_df: u16;
    let clk_id: u8;
    let txclk_df: u8;
    let mut ret: c_int;

    match sample_rate {
        22050 => { rate = SPDIF_TXRATE_22050; csfs = IEC958_AES3_CON_FS_22050 as c_ulong; }
        32000 => { rate = SPDIF_TXRATE_32000; csfs = IEC958_AES3_CON_FS_32000 as c_ulong; }
        44100 => { rate = SPDIF_TXRATE_44100; csfs = IEC958_AES3_CON_FS_44100 as c_ulong; }
        48000 => { rate = SPDIF_TXRATE_48000; csfs = IEC958_AES3_CON_FS_48000 as c_ulong; }
        88200 => { rate = SPDIF_TXRATE_88200; csfs = IEC958_AES3_CON_FS_88200 as c_ulong; }
        96000 => { rate = SPDIF_TXRATE_96000; csfs = IEC958_AES3_CON_FS_96000 as c_ulong; }
        176400 => { rate = SPDIF_TXRATE_176400; csfs = IEC958_AES3_CON_FS_176400 as c_ulong; }
        192000 => { rate = SPDIF_TXRATE_192000; csfs = IEC958_AES3_CON_FS_192000 as c_ulong; }
        _ => {
            dev_err(&mut (*pdev).dev, b"unsupported sample rate %d\n\0".as_ptr() as *const c_char, sample_rate);
            return -EINVAL;
        }
    }

    ret = fsl_spdif_probe_txclk(spdif_priv, rate as spdif_txrate);
    if ret != 0 { return ret; }

    clk_id = (*spdif_priv).txclk_src[rate as usize];
    if clk_id as c_uint >= STC_TXCLK_SRC_MAX {
        dev_err(&mut (*pdev).dev, b"tx clock source is out of range\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    txclk_df = (*spdif_priv).txclk_df[rate as usize];
    if txclk_df == 0 {
        dev_err(&mut (*pdev).dev, b"the txclk_df can't be zero\n\0".as_ptr() as *const c_char);
        return -EINVAL;
    }

    sysclk_df = (*spdif_priv).sysclk_df[rate as usize];

    if fsl_spdif_can_set_clk_rate(spdif_priv, clk_id as c_int) {
        /* The S/PDIF block needs a clock of 64 * fs * txclk_df */
        ret = clk_set_rate((*spdif_priv).txclk[clk_id as usize],
                           (64 * sample_rate * txclk_df as c_int) as c_ulong);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"failed to set tx clock rate\n\0".as_ptr() as *const c_char);
            return ret;
        }
    }

    dev_dbg(&mut (*pdev).dev, b"expected clock rate = %d\n\0".as_ptr() as *const c_char,
            64 * sample_rate * txclk_df as c_int * sysclk_df as c_int);
    dev_dbg(&mut (*pdev).dev, b"actual clock rate = %ld\n\0".as_ptr() as *const c_char,
            clk_get_rate((*spdif_priv).txclk[clk_id as usize]));

    /* set fs field in consumer channel status */
    spdif_set_cstatus(ctrl, IEC958_AES3_CON_FS as u8, csfs as u8);

    /* select clock source and divisor */
    stc = STC_TXCLK_ALL_EN | STC_TXCLK_SRC_SET(clk_id as u32) |
        STC_TXCLK_DF(txclk_df as u32) | STC_SYSCLK_DF(sysclk_df as u32);
    mask = STC_TXCLK_ALL_EN_MASK | STC_TXCLK_SRC_MASK |
        STC_TXCLK_DF_MASK | STC_SYSCLK_DF_MASK;
    regmap_update_bits(regmap, REG_SPDIF_STC, mask, stc);

    dev_dbg(&mut (*pdev).dev, b"set sample rate to %dHz for %dHz playback\n\0".as_ptr() as *const c_char,
            (*spdif_priv).txrate[rate as usize], sample_rate);

    0
}

unsafe extern "C" fn fsl_spdif_startup(substream: *mut snd_pcm_substream,
                                       cpu_dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let spdif_priv = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut fsl_spdif_priv;
    let pdev = (*spdif_priv).pdev;
    let regmap = (*spdif_priv).regmap;
    let scr: u32;
    let mask: u32;
    let ret: c_int;

    /* Reset module and interrupts only for first initialization */
    if snd_soc_dai_active(cpu_dai) == 0 {
        ret = spdif_softreset(spdif_priv);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"failed to soft reset\n\0".as_ptr() as *const c_char);
            return ret;
        }

        /* Disable all the interrupts */
        regmap_update_bits(regmap, REG_SPDIF_SIE, 0xffffff, 0);
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        scr = SCR_TXFIFO_AUTOSYNC | SCR_TXFIFO_CTRL_NORMAL |
            SCR_TXSEL_NORMAL | SCR_USRC_SEL_CHIP |
            SCR_TXFIFO_FSEL_IF8;
        mask = SCR_TXFIFO_AUTOSYNC_MASK | SCR_TXFIFO_CTRL_MASK |
            SCR_TXSEL_MASK | SCR_USRC_SEL_MASK |
            SCR_TXFIFO_FSEL_MASK;
    } else {
        scr = SCR_RXFIFO_FSEL_IF8 | SCR_RXFIFO_AUTOSYNC;
        mask = SCR_RXFIFO_FSEL_MASK | SCR_RXFIFO_AUTOSYNC_MASK |
            SCR_RXFIFO_CTL_MASK | SCR_RXFIFO_OFF_MASK;
    }
    regmap_update_bits(regmap, REG_SPDIF_SCR, mask, scr);

    /* Power up SPDIF module */
    regmap_update_bits(regmap, REG_SPDIF_SCR, SCR_LOW_POWER, 0);

    0
}

unsafe extern "C" fn fsl_spdif_shutdown(substream: *mut snd_pcm_substream,
                                        cpu_dai: *mut snd_soc_dai) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let spdif_priv = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut fsl_spdif_priv;
    let regmap = (*spdif_priv).regmap;
    let scr: u32;
    let mask: u32;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        scr = 0;
        mask = SCR_TXFIFO_AUTOSYNC_MASK | SCR_TXFIFO_CTRL_MASK |
            SCR_TXSEL_MASK | SCR_USRC_SEL_MASK |
            SCR_TXFIFO_FSEL_MASK;
        /* Disable TX clock */
        regmap_update_bits(regmap, REG_SPDIF_STC, STC_TXCLK_ALL_EN_MASK, 0);
    } else {
        scr = SCR_RXFIFO_OFF | SCR_RXFIFO_CTL_ZERO;
        mask = SCR_RXFIFO_FSEL_MASK | SCR_RXFIFO_AUTOSYNC_MASK |
            SCR_RXFIFO_CTL_MASK | SCR_RXFIFO_OFF_MASK;
    }
    regmap_update_bits(regmap, REG_SPDIF_SCR, mask, scr);

    /* Power down SPDIF module only if tx&rx are both inactive */
    if snd_soc_dai_active(cpu_dai) == 0 {
        spdif_intr_status_clear(spdif_priv);
        regmap_update_bits(regmap, REG_SPDIF_SCR, SCR_LOW_POWER, SCR_LOW_POWER);
    }
}

unsafe fn spdif_reparent_rootclk(spdif_priv: *mut fsl_spdif_priv, sample_rate: c_uint) -> c_int {
    let pdev = (*spdif_priv).pdev;
    let clk_ptr: *mut clk;
    let ret: c_int;

    /* Reparent clock if required condition is true */
    if !fsl_spdif_can_set_clk_rate(spdif_priv, STC_TXCLK_SPDIF_ROOT) {
        return 0;
    }

    /* Get root clock */
    clk_ptr = (*spdif_priv).txclk[STC_TXCLK_SPDIF_ROOT as usize];

    /* Disable clock first, for it was enabled by pm_runtime */
    clk_disable_unprepare(clk_ptr);
    fsl_asoc_reparent_pll_clocks(&mut (*pdev).dev, clk_ptr, (*spdif_priv).pll8k_clk,
                                 (*spdif_priv).pll11k_clk, sample_rate);
    ret = clk_prepare_enable(clk_ptr);
    if ret != 0 { return ret; }

    0
}

unsafe extern "C" fn fsl_spdif_hw_params(substream: *mut snd_pcm_substream,
                                         params: *mut snd_pcm_hw_params,
                                         _dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let spdif_priv = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut fsl_spdif_priv;
    let ctrl = &mut (*spdif_priv).fsl_spdif_control as *mut spdif_mixer_control;
    let pdev = (*spdif_priv).pdev;
    let sample_rate = params_rate(params);
    let mut ret: c_int = 0;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        ret = spdif_reparent_rootclk(spdif_priv, sample_rate);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"%s: reparent root clk failed: %d\n\0".as_ptr() as *const c_char,
                    b"fsl_spdif_hw_params\0".as_ptr(), sample_rate);
            return ret;
        }

        ret = spdif_set_sample_rate(substream, sample_rate as c_int);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"%s: set sample rate failed: %d\n\0".as_ptr() as *const c_char,
                    b"fsl_spdif_hw_params\0".as_ptr(), sample_rate);
            return ret;
        }
        spdif_set_cstatus(ctrl, IEC958_AES3_CON_CLOCK as u8,
                          IEC958_AES3_CON_CLOCK_1000PPM as u8);
        spdif_write_channel_status(spdif_priv);
    } else {
        /* Setup rx clock source */
        ret = spdif_set_rx_clksrc(spdif_priv, SPDIF_DEFAULT_GAINSEL as spdif_gainsel, 1);
    }

    ret
}

unsafe extern "C" fn fsl_spdif_trigger(substream: *mut snd_pcm_substream,
                                       cmd: c_int, _dai: *mut snd_soc_dai) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let spdif_priv = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut fsl_spdif_priv;
    let regmap = (*spdif_priv).regmap;
    let tx = (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK;
    let intr = SIE_INTR_FOR(tx);
    let dmaen = SCR_DMA_xX_EN(tx);

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            regmap_update_bits(regmap, REG_SPDIF_SIE, intr, intr);
            regmap_update_bits(regmap, REG_SPDIF_SCR, dmaen, dmaen);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            regmap_update_bits(regmap, REG_SPDIF_SCR, dmaen, 0);
            regmap_update_bits(regmap, REG_SPDIF_SIE, intr, 0);
            regmap_write(regmap, REG_SPDIF_STL, 0x0);
            regmap_write(regmap, REG_SPDIF_STR, 0x0);
        }
        _ => return -EINVAL,
    }

    0
}

/*
 * FSL SPDIF IEC958 controller(mixer) functions
 *
 *	Channel status get/put control
 *	User bit value get/put control
 *	Valid bit value get control
 *	DPLL lock status get control
 *	User bit sync mode selection control
 */

unsafe extern "C" fn fsl_spdif_info(_kcontrol: *mut snd_kcontrol,
                                    uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_IEC958;
    (*uinfo).count = 1;
    0
}

unsafe extern "C" fn fsl_spdif_pb_get(kcontrol: *mut snd_kcontrol,
                                      uvalue: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let ctrl = &mut (*spdif_priv).fsl_spdif_control as *mut spdif_mixer_control;

    (*uvalue).value.iec958.status[0] = (*ctrl).ch_status[0];
    (*uvalue).value.iec958.status[1] = (*ctrl).ch_status[1];
    (*uvalue).value.iec958.status[2] = (*ctrl).ch_status[2];
    (*uvalue).value.iec958.status[3] = (*ctrl).ch_status[3];

    0
}

unsafe extern "C" fn fsl_spdif_pb_put(kcontrol: *mut snd_kcontrol,
                                      uvalue: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let ctrl = &mut (*spdif_priv).fsl_spdif_control as *mut spdif_mixer_control;

    (*ctrl).ch_status[0] = (*uvalue).value.iec958.status[0];
    (*ctrl).ch_status[1] = (*uvalue).value.iec958.status[1];
    (*ctrl).ch_status[2] = (*uvalue).value.iec958.status[2];
    (*ctrl).ch_status[3] = (*uvalue).value.iec958.status[3];

    spdif_write_channel_status(spdif_priv);

    0
}

/* Get channel status from SPDIF_RX_CCHAN register */
unsafe extern "C" fn fsl_spdif_capture_get(kcontrol: *mut snd_kcontrol,
                                           ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let regmap = (*spdif_priv).regmap;
    let mut cstatus: u32 = 0;
    let mut val: u32 = 0;

    regmap_read(regmap, REG_SPDIF_SIS, &mut val);
    if val & INT_CNEW == 0 { return -EAGAIN; }

    regmap_read(regmap, REG_SPDIF_SRCSH, &mut cstatus);
    (*ucontrol).value.iec958.status[0] = ((cstatus >> 16) & 0xFF) as u8;
    (*ucontrol).value.iec958.status[1] = ((cstatus >> 8) & 0xFF) as u8;
    (*ucontrol).value.iec958.status[2] = (cstatus & 0xFF) as u8;

    regmap_read(regmap, REG_SPDIF_SRCSL, &mut cstatus);
    (*ucontrol).value.iec958.status[3] = ((cstatus >> 16) & 0xFF) as u8;
    (*ucontrol).value.iec958.status[4] = ((cstatus >> 8) & 0xFF) as u8;
    (*ucontrol).value.iec958.status[5] = (cstatus & 0xFF) as u8;

    /* Clear intr */
    regmap_write(regmap, REG_SPDIF_SIC, INT_CNEW);

    0
}

/*
 * Get User bits (subcode) from chip value which readed out
 * in UChannel register.
 */
unsafe extern "C" fn fsl_spdif_subcode_get(kcontrol: *mut snd_kcontrol,
                                           ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let ctrl = &mut (*spdif_priv).fsl_spdif_control as *mut spdif_mixer_control;
    let mut ret = -EAGAIN;

    /* guard(spinlock_irqsave)(&ctrl->ctl_lock); */
    if (*ctrl).ready_buf != 0 {
        let idx = ((*ctrl).ready_buf - 1) * SPDIF_UBITS_SIZE;
        memcpy((*ucontrol).value.iec958.subcode.as_mut_ptr() as *mut c_void,
               (*ctrl).subcode.as_ptr().add(idx as usize) as *const c_void,
               SPDIF_UBITS_SIZE as usize);
        ret = 0;
    }

    ret
}

/* Q-subcode information. The byte size is SPDIF_UBITS_SIZE/8 */
unsafe extern "C" fn fsl_spdif_qinfo(_kcontrol: *mut snd_kcontrol,
                                     uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_BYTES;
    (*uinfo).count = SPDIF_QSUB_SIZE;
    0
}

/* Get Q subcode from chip value which readed out in QChannel register */
unsafe extern "C" fn fsl_spdif_qget(kcontrol: *mut snd_kcontrol,
                                    ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let ctrl = &mut (*spdif_priv).fsl_spdif_control as *mut spdif_mixer_control;
    let mut ret = -EAGAIN;

    /* guard(spinlock_irqsave)(&ctrl->ctl_lock); */
    if (*ctrl).ready_buf != 0 {
        let idx = ((*ctrl).ready_buf - 1) * SPDIF_QSUB_SIZE;
        memcpy((*ucontrol).value.bytes.data.as_mut_ptr() as *mut c_void,
               (*ctrl).qsub.as_ptr().add(idx as usize) as *const c_void,
               SPDIF_QSUB_SIZE as usize);
        ret = 0;
    }

    ret
}

/* Get valid good bit from interrupt status register */
unsafe extern "C" fn fsl_spdif_rx_vbit_get(kcontrol: *mut snd_kcontrol,
                                           ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let regmap = (*spdif_priv).regmap;
    let mut val: u32 = 0;

    regmap_read(regmap, REG_SPDIF_SIS, &mut val);
    (*ucontrol).value.integer.value[0] = ((val & INT_VAL_NOGOOD) != 0) as i64;
    regmap_write(regmap, REG_SPDIF_SIC, INT_VAL_NOGOOD);

    0
}

unsafe extern "C" fn fsl_spdif_tx_vbit_get(kcontrol: *mut snd_kcontrol,
                                           ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let regmap = (*spdif_priv).regmap;
    let mut val: u32 = 0;

    regmap_read(regmap, REG_SPDIF_SCR, &mut val);
    val = (val & SCR_VAL_MASK) >> SCR_VAL_OFFSET;
    val = 1 - val;
    (*ucontrol).value.integer.value[0] = val as i64;

    0
}

unsafe extern "C" fn fsl_spdif_tx_vbit_put(kcontrol: *mut snd_kcontrol,
                                           ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let regmap = (*spdif_priv).regmap;
    let val = ((1 - (*ucontrol).value.integer.value[0]) as u32) << SCR_VAL_OFFSET;

    regmap_update_bits(regmap, REG_SPDIF_SCR, SCR_VAL_MASK, val);

    0
}

unsafe extern "C" fn fsl_spdif_rx_rcm_get(kcontrol: *mut snd_kcontrol,
                                          ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let regmap = (*spdif_priv).regmap;
    let mut val: u32 = 0;

    regmap_read(regmap, REG_SPDIF_SCR, &mut val);
    val = if val & SCR_RAW_CAPTURE_MODE != 0 { 1 } else { 0 };
    (*ucontrol).value.integer.value[0] = val as i64;

    0
}

unsafe extern "C" fn fsl_spdif_rx_rcm_put(kcontrol: *mut snd_kcontrol,
                                          ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let regmap = (*spdif_priv).regmap;
    let val = if (*ucontrol).value.integer.value[0] != 0 { SCR_RAW_CAPTURE_MODE } else { 0 };

    if val != 0 {
        (*(*cpu_dai).driver).capture.formats |= SNDRV_PCM_FMTBIT_S32_LE;
    } else {
        (*(*cpu_dai).driver).capture.formats &= !SNDRV_PCM_FMTBIT_S32_LE;
    }

    regmap_update_bits(regmap, REG_SPDIF_SCR, SCR_RAW_CAPTURE_MODE, val);

    0
}

unsafe extern "C" fn fsl_spdif_bypass_get(kcontrol: *mut snd_kcontrol,
                                          ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut fsl_spdif_priv;

    (*ucontrol).value.integer.value[0] = if (*priv_).bypass { 1 } else { 0 };

    0
}

unsafe extern "C" fn fsl_spdif_bypass_put(kcontrol: *mut snd_kcontrol,
                                          ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let priv_ = snd_soc_dai_get_drvdata(dai) as *mut fsl_spdif_priv;
    let card = (*(*dai).component).card;
    let set = (*ucontrol).value.integer.value[0] != 0;
    let regmap = (*priv_).regmap;
    let rtd: *mut snd_soc_pcm_runtime;
    let scr: u32;
    let mut mask: u32;

    rtd = snd_soc_get_pcm_runtime(card, (*card).dai_link);

    if (*priv_).bypass == set {
        return 0; /* nothing to do */
    }

    if snd_soc_dai_active(dai) != 0 {
        dev_err((*dai).dev, b"Cannot change BYPASS mode while stream is running.\n\0".as_ptr() as *const c_char);
        return -EBUSY;
    }

    pm_runtime_get_sync((*dai).dev);

    if set {
        /* Disable interrupts */
        regmap_update_bits(regmap, REG_SPDIF_SIE, 0xffffff, 0);

        /* Configure BYPASS mode */
        scr = SCR_TXSEL_RX | SCR_RXFIFO_OFF;
        mask = SCR_RXFIFO_FSEL_MASK | SCR_RXFIFO_AUTOSYNC_MASK |
            SCR_RXFIFO_CTL_MASK | SCR_RXFIFO_OFF_MASK | SCR_TXSEL_MASK;
        /* Power up SPDIF module */
        mask |= SCR_LOW_POWER;
    } else {
        /* Power down SPDIF module, disable TX */
        scr = SCR_LOW_POWER | SCR_TXSEL_OFF;
        mask = SCR_LOW_POWER | SCR_TXSEL_MASK;
    }

    regmap_update_bits(regmap, REG_SPDIF_SCR, mask, scr);

    /* Disable playback & capture if BYPASS mode is enabled, enable otherwise */
    for stream in 0..2 {
        (*(*rtd).pcm).streams[stream].substream_count = if set { 0 } else { 1 };
    }

    (*priv_).bypass = set;
    pm_runtime_put_sync((*dai).dev);

    0
}

/* DPLL lock information */
unsafe extern "C" fn fsl_spdif_rxrate_info(_kcontrol: *mut snd_kcontrol,
                                           uinfo: *mut snd_ctl_elem_info) -> c_int {
    (*uinfo).type_ = SNDRV_CTL_ELEM_TYPE_INTEGER;
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 16000;
    (*uinfo).value.integer.max = 192000;

    0
}

static mut gainsel_multi: [u32; GAINSEL_MULTI_MAX as usize] = [
    24, 16, 12, 8, 6, 4, 3,
];

/* Get RX data clock rate given the SPDIF bus_clk */
unsafe fn spdif_get_rxclk_rate(spdif_priv: *mut fsl_spdif_priv,
                               gainsel: spdif_gainsel) -> c_int {
    let regmap = (*spdif_priv).regmap;
    let pdev = (*spdif_priv).pdev;
    let mut tmpval64: u64;
    let mut busclk_freq: u64 = 0;
    let mut freqmeas: u32 = 0;
    let mut phaseconf: u32 = 0;
    let clksrc: u8;

    regmap_read(regmap, REG_SPDIF_SRFM, &mut freqmeas);
    regmap_read(regmap, REG_SPDIF_SRPC, &mut phaseconf);

    clksrc = ((phaseconf >> SRPC_CLKSRC_SEL_OFFSET) & 0xf) as u8;

    /* Get bus clock from system */
    if srpc_dpll_locked[clksrc as usize] != 0 && (phaseconf & SRPC_DPLL_LOCKED) != 0 {
        busclk_freq = clk_get_rate((*spdif_priv).sysclk) as u64;
    }

    /* FreqMeas_CLK = (BUS_CLK * FreqMeas) / 2 ^ 10 / GAINSEL / 128 */
    tmpval64 = busclk_freq * freqmeas as u64;
    tmpval64 /= (gainsel_multi[gainsel as usize] * 1024) as u64;
    tmpval64 /= (128 * 1024) as u64;

    dev_dbg(&mut (*pdev).dev, b"FreqMeas: %d\n\0".as_ptr() as *const c_char, freqmeas);
    dev_dbg(&mut (*pdev).dev, b"BusclkFreq: %lld\n\0".as_ptr() as *const c_char, busclk_freq);
    dev_dbg(&mut (*pdev).dev, b"RxRate: %lld\n\0".as_ptr() as *const c_char, tmpval64);

    tmpval64 as c_int
}

/*
 * Get DPLL lock or not info from stable interrupt status register.
 * User application must use this control to get locked,
 * then can do next PCM operation
 */
unsafe extern "C" fn fsl_spdif_rxrate_get(kcontrol: *mut snd_kcontrol,
                                          ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let mut rate: c_int = 0;

    if (*spdif_priv).dpll_locked {
        rate = spdif_get_rxclk_rate(spdif_priv, SPDIF_DEFAULT_GAINSEL as spdif_gainsel);
    }

    (*ucontrol).value.integer.value[0] = rate as i64;

    0
}

/*
 * User bit sync mode:
 * 1 CD User channel subcode
 * 0 Non-CD data
 */
unsafe extern "C" fn fsl_spdif_usync_get(kcontrol: *mut snd_kcontrol,
                                         ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let regmap = (*spdif_priv).regmap;
    let mut val: u32 = 0;

    regmap_read(regmap, REG_SPDIF_SRCD, &mut val);
    (*ucontrol).value.integer.value[0] = ((val & SRCD_CD_USER) != 0) as i64;

    0
}

/*
 * User bit sync mode:
 * 1 CD User channel subcode
 * 0 Non-CD data
 */
unsafe extern "C" fn fsl_spdif_usync_put(kcontrol: *mut snd_kcontrol,
                                         ucontrol: *mut snd_ctl_elem_value) -> c_int {
    let cpu_dai = snd_kcontrol_chip(kcontrol) as *mut snd_soc_dai;
    let spdif_priv = snd_soc_dai_get_drvdata(cpu_dai) as *mut fsl_spdif_priv;
    let regmap = (*spdif_priv).regmap;
    let val = ((*ucontrol).value.integer.value[0] as u32) << SRCD_CD_USER_OFFSET;

    regmap_update_bits(regmap, REG_SPDIF_SRCD, SRCD_CD_USER, val);

    0
}

/* FSL SPDIF IEC958 controller defines */
static fsl_spdif_ctrls: [snd_kcontrol_new; 9] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: b"IEC958 Playback Default\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(fsl_spdif_info), get: Some(fsl_spdif_pb_get), put: Some(fsl_spdif_pb_put) },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Capture Default\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(fsl_spdif_info), get: Some(fsl_spdif_capture_get), put: None },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Subcode Capture Default\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(fsl_spdif_info), get: Some(fsl_spdif_subcode_get), put: None },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Q-subcode Capture Default\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(fsl_spdif_qinfo), get: Some(fsl_spdif_qget), put: None },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 RX V-Bit Errors\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(snd_ctl_boolean_mono_info), get: Some(fsl_spdif_rx_vbit_get), put: None },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 TX V-Bit\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(snd_ctl_boolean_mono_info), get: Some(fsl_spdif_tx_vbit_get), put: Some(fsl_spdif_tx_vbit_put) },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_MIXER, name: RX_SAMPLE_RATE_KCONTROL, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(fsl_spdif_rxrate_info), get: Some(fsl_spdif_rxrate_get), put: None },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"Bypass Mode\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READWRITE, info: Some(snd_ctl_boolean_mono_info), get: Some(fsl_spdif_bypass_get), put: Some(fsl_spdif_bypass_put) },
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 USyncMode CDText\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(snd_ctl_boolean_mono_info), get: Some(fsl_spdif_usync_get), put: Some(fsl_spdif_usync_put) },
];

static fsl_spdif_ctrls_rcm: [snd_kcontrol_new; 1] = [
    snd_kcontrol_new { iface: SNDRV_CTL_ELEM_IFACE_PCM, name: b"IEC958 Raw Capture Mode\0".as_ptr() as *const c_char, access: SNDRV_CTL_ELEM_ACCESS_READ | SNDRV_CTL_ELEM_ACCESS_WRITE | SNDRV_CTL_ELEM_ACCESS_VOLATILE, info: Some(snd_ctl_boolean_mono_info), get: Some(fsl_spdif_rx_rcm_get), put: Some(fsl_spdif_rx_rcm_put) },
];

unsafe extern "C" fn fsl_spdif_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let spdif_private = snd_soc_dai_get_drvdata(dai) as *mut fsl_spdif_priv;

    snd_soc_dai_init_dma_data(dai, &mut (*spdif_private).dma_params_tx,
                              &mut (*spdif_private).dma_params_rx);

    snd_soc_add_dai_controls(dai, fsl_spdif_ctrls.as_ptr(), fsl_spdif_ctrls.len() as c_uint);

    if (*(*spdif_private).soc).raw_capture_mode {
        snd_soc_add_dai_controls(dai, fsl_spdif_ctrls_rcm.as_ptr(),
                                 fsl_spdif_ctrls_rcm.len() as c_uint);
    }

    (*spdif_private).snd_card = (*(*(*dai).component).card).snd_card;
    (*spdif_private).rxrate_kcontrol = snd_soc_card_get_kcontrol((*(*dai).component).card,
                                                                 RX_SAMPLE_RATE_KCONTROL);
    if (*spdif_private).rxrate_kcontrol.is_null() {
        dev_err(&mut (*(*spdif_private).pdev).dev, b"failed to get %s kcontrol\n\0".as_ptr() as *const c_char,
                RX_SAMPLE_RATE_KCONTROL);
    }

    /*Clear the val bit for Tx*/
    regmap_update_bits((*spdif_private).regmap, REG_SPDIF_SCR,
                       SCR_VAL_MASK, SCR_VAL_CLEAR);

    0
}

static fsl_spdif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(fsl_spdif_dai_probe),
    startup: Some(fsl_spdif_startup),
    hw_params: Some(fsl_spdif_hw_params),
    trigger: Some(fsl_spdif_trigger),
    shutdown: Some(fsl_spdif_shutdown),
};

static mut fsl_spdif_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: b"CPU-Playback\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: FSL_SPDIF_RATES_PLAYBACK,
        formats: FSL_SPDIF_FORMATS_PLAYBACK,
    },
    capture: snd_soc_pcm_stream {
        stream_name: b"CPU-Capture\0".as_ptr() as *const c_char,
        channels_min: 2,
        channels_max: 2,
        rates: FSL_SPDIF_RATES_CAPTURE,
        formats: FSL_SPDIF_FORMATS_CAPTURE,
    },
    ops: &fsl_spdif_dai_ops,
    name: core::ptr::null(),
};

static fsl_spdif_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"fsl-spdif\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

/* FSL SPDIF REGMAP */
static fsl_spdif_reg_defaults: [reg_default; 10] = [
    reg_default { reg: REG_SPDIF_SCR,    def: 0x00000400 },
    reg_default { reg: REG_SPDIF_SRCD,   def: 0x00000000 },
    reg_default { reg: REG_SPDIF_SIE,    def: 0x00000000 },
    reg_default { reg: REG_SPDIF_STL,    def: 0x00000000 },
    reg_default { reg: REG_SPDIF_STR,    def: 0x00000000 },
    reg_default { reg: REG_SPDIF_STCSCH, def: 0x00000000 },
    reg_default { reg: REG_SPDIF_STCSCL, def: 0x00000000 },
    reg_default { reg: REG_SPDIF_STCSPH, def: 0x00000000 },
    reg_default { reg: REG_SPDIF_STCSPL, def: 0x00000000 },
    reg_default { reg: REG_SPDIF_STC,    def: 0x00020f00 },
];

unsafe extern "C" fn fsl_spdif_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        REG_SPDIF_SCR | REG_SPDIF_SRCD | REG_SPDIF_SRPC | REG_SPDIF_SIE |
        REG_SPDIF_SIS | REG_SPDIF_SRL | REG_SPDIF_SRR | REG_SPDIF_SRCSH |
        REG_SPDIF_SRCSL | REG_SPDIF_SRU | REG_SPDIF_SRQ | REG_SPDIF_STCSCH |
        REG_SPDIF_STCSCL | REG_SPDIF_STCSPH | REG_SPDIF_STCSPL | REG_SPDIF_SRFM |
        REG_SPDIF_STC | REG_SPDIF_SRCCA_31_0 | REG_SPDIF_SRCCA_63_32 |
        REG_SPDIF_SRCCA_95_64 | REG_SPDIF_SRCCA_127_96 | REG_SPDIF_SRCCA_159_128 |
        REG_SPDIF_SRCCA_191_160 | REG_SPDIF_STCCA_31_0 | REG_SPDIF_STCCA_63_32 |
        REG_SPDIF_STCCA_95_64 | REG_SPDIF_STCCA_127_96 | REG_SPDIF_STCCA_159_128 |
        REG_SPDIF_STCCA_191_160 => true,
        _ => false,
    }
}

unsafe extern "C" fn fsl_spdif_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        REG_SPDIF_SRPC | REG_SPDIF_SIS | REG_SPDIF_SRL | REG_SPDIF_SRR |
        REG_SPDIF_SRCSH | REG_SPDIF_SRCSL | REG_SPDIF_SRU | REG_SPDIF_SRQ |
        REG_SPDIF_SRFM | REG_SPDIF_SRCCA_31_0 | REG_SPDIF_SRCCA_63_32 |
        REG_SPDIF_SRCCA_95_64 | REG_SPDIF_SRCCA_127_96 | REG_SPDIF_SRCCA_159_128 |
        REG_SPDIF_SRCCA_191_160 => true,
        _ => false,
    }
}

unsafe extern "C" fn fsl_spdif_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        REG_SPDIF_SCR | REG_SPDIF_SRCD | REG_SPDIF_SRPC | REG_SPDIF_SIE |
        REG_SPDIF_SIC | REG_SPDIF_STL | REG_SPDIF_STR | REG_SPDIF_STCSCH |
        REG_SPDIF_STCSCL | REG_SPDIF_STCSPH | REG_SPDIF_STCSPL | REG_SPDIF_STC |
        REG_SPDIF_STCCA_31_0 | REG_SPDIF_STCCA_63_32 | REG_SPDIF_STCCA_95_64 |
        REG_SPDIF_STCCA_127_96 | REG_SPDIF_STCCA_159_128 | REG_SPDIF_STCCA_191_160 => true,
        _ => false,
    }
}

static fsl_spdif_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: REG_SPDIF_STCCA_191_160,
    reg_defaults: fsl_spdif_reg_defaults.as_ptr(),
    num_reg_defaults: fsl_spdif_reg_defaults.len() as c_uint,
    readable_reg: Some(fsl_spdif_readable_reg),
    volatile_reg: Some(fsl_spdif_volatile_reg),
    writeable_reg: Some(fsl_spdif_writeable_reg),
    cache_type: REGCACHE_FLAT,
};

unsafe fn fsl_spdif_txclk_caldiv(spdif_priv: *mut fsl_spdif_priv,
                                 clk_ptr: *mut clk, mut savesub: u64,
                                 index: spdif_txrate, round: bool) -> u32 {
    static rate: [u32; 8] = [22050, 32000, 44100, 48000, 88200, 96000, 176400, 192000];
    let is_sysclk = clk_is_match(clk_ptr, (*spdif_priv).sysclk) != 0;
    let mut rate_ideal: u64;
    let mut rate_actual: u64;
    let mut sub: u64;
    let mut arate: u32;
    let sysclk_dfmin: u16;
    let sysclk_dfmax: u16;
    let mut sysclk_df: u16;
    let mut txclk_df: u8;

    /* The sysclk has an extra divisor [2, 512] */
    sysclk_dfmin = if is_sysclk { 2 } else { 1 };
    sysclk_dfmax = if is_sysclk { 512 } else { 1 };

    sysclk_df = sysclk_dfmin;
    while sysclk_df <= sysclk_dfmax {
        txclk_df = 1;
        while txclk_df <= 128 {
            rate_ideal = rate[index as usize] as u64 * txclk_df as u64 * 64u64;
            if round {
                rate_actual = clk_round_rate(clk_ptr, rate_ideal as c_ulong) as u64;
            } else {
                rate_actual = clk_get_rate(clk_ptr) as u64;
            }

            arate = (rate_actual / 64) as u32;
            arate /= txclk_df as u32 * sysclk_df as u32;

            if arate == rate[index as usize] {
                /* We are lucky */
                savesub = 0;
                (*spdif_priv).txclk_df[index as usize] = txclk_df;
                (*spdif_priv).sysclk_df[index as usize] = sysclk_df;
                (*spdif_priv).txrate[index as usize] = arate;
                return savesub as u32;
            } else if arate / rate[index as usize] == 1 {
                /* A little bigger than expect */
                sub = (arate - rate[index as usize]) as u64 * 100000;
                sub /= rate[index as usize] as u64;
                if sub < savesub {
                    savesub = sub;
                    (*spdif_priv).txclk_df[index as usize] = txclk_df;
                    (*spdif_priv).sysclk_df[index as usize] = sysclk_df;
                    (*spdif_priv).txrate[index as usize] = arate;
                }
            } else if rate[index as usize] / arate == 1 {
                /* A little smaller than expect */
                sub = (rate[index as usize] - arate) as u64 * 100000;
                sub /= rate[index as usize] as u64;
                if sub < savesub {
                    savesub = sub;
                    (*spdif_priv).txclk_df[index as usize] = txclk_df;
                    (*spdif_priv).sysclk_df[index as usize] = sysclk_df;
                    (*spdif_priv).txrate[index as usize] = arate;
                }
            }
            txclk_df = txclk_df.wrapping_add(1);
        }
        sysclk_df = sysclk_df.wrapping_add(1);
    }

    savesub as u32
}

unsafe fn fsl_spdif_probe_txclk(spdif_priv: *mut fsl_spdif_priv,
                                index: spdif_txrate) -> c_int {
    static rate: [u32; 8] = [22050, 32000, 44100, 48000, 88200, 96000, 176400, 192000];
    let pdev = (*spdif_priv).pdev;
    let dev = &mut (*pdev).dev as *mut device;
    let mut savesub: u64 = 100000;
    let mut ret: u64;
    let mut clk_ptr: *mut clk;
    let mut i: c_int = 0;

    while i < STC_TXCLK_SRC_MAX {
        clk_ptr = (*spdif_priv).txclk[i as usize];
        if IS_ERR(clk_ptr as *const c_void) != 0 {
            dev_err(dev, b"no rxtx%d clock in devicetree\n\0".as_ptr() as *const c_char, i);
            return PTR_ERR(clk_ptr as *const c_void);
        }
        if clk_get_rate(clk_ptr) == 0 {
            i += 1;
            continue;
        }

        ret = fsl_spdif_txclk_caldiv(spdif_priv, clk_ptr, savesub, index,
                                     fsl_spdif_can_set_clk_rate(spdif_priv, i)) as u64;
        if savesub == ret {
            i += 1;
            continue;
        }

        savesub = ret;
        (*spdif_priv).txclk_src[index as usize] = i as u8;

        /* To quick catch a divisor, we allow a 0.1% deviation */
        if savesub < 100 {
            break;
        }
        i += 1;
    }

    dev_dbg(dev, b"use rxtx%d as tx clock source for %dHz sample rate\n\0".as_ptr() as *const c_char,
            (*spdif_priv).txclk_src[index as usize] as c_int, rate[index as usize]);
    dev_dbg(dev, b"use txclk df %d for %dHz sample rate\n\0".as_ptr() as *const c_char,
            (*spdif_priv).txclk_df[index as usize] as c_int, rate[index as usize]);
    if clk_is_match((*spdif_priv).txclk[(*spdif_priv).txclk_src[index as usize] as usize], (*spdif_priv).sysclk) != 0 {
        dev_dbg(dev, b"use sysclk df %d for %dHz sample rate\n\0".as_ptr() as *const c_char,
                (*spdif_priv).sysclk_df[index as usize] as c_int, rate[index as usize]);
    }
    dev_dbg(dev, b"the best rate for %dHz sample rate is %dHz\n\0".as_ptr() as *const c_char,
            rate[index as usize], (*spdif_priv).txrate[index as usize]);

    0
}

unsafe extern "C" fn fsl_spdif_probe(pdev: *mut platform_device) -> c_int {
    let mut spdif_priv: *mut fsl_spdif_priv;
    let mut ctrl: *mut spdif_mixer_control;
    let mut res: *mut resource = core::ptr::null_mut();
    let regs: *mut c_void;
    let mut irq: c_int;
    let mut ret: c_int;
    let mut i: c_int;
    let mut tmp: [c_char; 16] = [0; 16];

    spdif_priv = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<fsl_spdif_priv>(), GFP_KERNEL) as *mut fsl_spdif_priv;
    if spdif_priv.is_null() {
        return -ENOMEM;
    }

    (*spdif_priv).pdev = pdev;
    (*spdif_priv).soc = of_device_get_match_data(&mut (*pdev).dev) as *const fsl_spdif_soc_data;

    /* Initialize this copy of the CPU DAI driver structure */
    memcpy(&mut (*spdif_priv).cpu_dai_drv as *mut _ as *mut c_void,
           &raw const fsl_spdif_dai as *const _ as *const c_void,
           core::mem::size_of_val(&fsl_spdif_dai));
    (*spdif_priv).cpu_dai_drv.name = dev_name(&mut (*pdev).dev);
    (*spdif_priv).cpu_dai_drv.playback.formats = (*(*spdif_priv).soc).tx_formats;

    /* Get the addresses and IRQ */
    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(regs) != 0 {
        return PTR_ERR(regs);
    }

    (*spdif_priv).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, regs, &fsl_spdif_regmap_config);
    if IS_ERR((*spdif_priv).regmap as *const c_void) != 0 {
        dev_err(&mut (*pdev).dev, b"regmap init failed\n\0".as_ptr() as *const c_char);
        return PTR_ERR((*spdif_priv).regmap as *const c_void);
    }

    i = 0;
    while i < (*(*spdif_priv).soc).interrupts as c_int {
        irq = platform_get_irq(pdev, i as c_uint);
        if irq < 0 { return irq; }

        ret = devm_request_irq(&mut (*pdev).dev, irq, Some(spdif_isr), 0,
                               dev_name(&mut (*pdev).dev), spdif_priv as *mut c_void);
        if ret != 0 {
            dev_err(&mut (*pdev).dev, b"could not claim irq %u\n\0".as_ptr() as *const c_char, irq);
            return ret;
        }
        i += 1;
    }

    i = 0;
    while i < STC_TXCLK_SRC_MAX {
        sprintf(tmp.as_mut_ptr(), b"rxtx%d\0".as_ptr() as *const c_char, i);
        (*spdif_priv).txclk[i as usize] = devm_clk_get(&mut (*pdev).dev, tmp.as_ptr());
        if IS_ERR((*spdif_priv).txclk[i as usize] as *const c_void) != 0 {
            dev_err(&mut (*pdev).dev, b"no rxtx%d clock in devicetree\n\0".as_ptr() as *const c_char, i);
            return PTR_ERR((*spdif_priv).txclk[i as usize] as *const c_void);
        }
        i += 1;
    }

    /* Get system clock for rx clock rate calculation */
    (*spdif_priv).sysclk = (*spdif_priv).txclk[5];
    if IS_ERR((*spdif_priv).sysclk as *const c_void) != 0 {
        dev_err(&mut (*pdev).dev, b"no sys clock (rxtx5) in devicetree\n\0".as_ptr() as *const c_char);
        return PTR_ERR((*spdif_priv).sysclk as *const c_void);
    }

    /* Get core clock for data register access via DMA */
    (*spdif_priv).coreclk = devm_clk_get(&mut (*pdev).dev, b"core\0".as_ptr() as *const c_char);
    if IS_ERR((*spdif_priv).coreclk as *const c_void) != 0 {
        dev_err(&mut (*pdev).dev, b"no core clock in devicetree\n\0".as_ptr() as *const c_char);
        return PTR_ERR((*spdif_priv).coreclk as *const c_void);
    }

    (*spdif_priv).spbaclk = devm_clk_get(&mut (*pdev).dev, b"spba\0".as_ptr() as *const c_char);
    if IS_ERR((*spdif_priv).spbaclk as *const c_void) != 0 {
        dev_warn(&mut (*pdev).dev, b"no spba clock in devicetree\n\0".as_ptr() as *const c_char);
    }

    /* Select clock source for rx/tx clock */
    (*spdif_priv).rxclk = (*spdif_priv).txclk[1];
    if IS_ERR((*spdif_priv).rxclk as *const c_void) != 0 {
        dev_err(&mut (*pdev).dev, b"no rxtx1 clock in devicetree\n\0".as_ptr() as *const c_char);
        return PTR_ERR((*spdif_priv).rxclk as *const c_void);
    }
    (*spdif_priv).rxclk_src = DEFAULT_RXCLK_SRC;

    fsl_asoc_get_pll_clocks(&mut (*pdev).dev, &mut (*spdif_priv).pll8k_clk,
                            &mut (*spdif_priv).pll11k_clk);

    /* Initial spinlock for control data */
    ctrl = &mut (*spdif_priv).fsl_spdif_control;
    spin_lock_init(&mut (*ctrl).ctl_lock);

    /* Init tx channel status default value */
    (*ctrl).ch_status[0] = (IEC958_AES0_CON_NOT_COPYRIGHT | IEC958_AES0_CON_EMPHASIS_5015) as u8;
    (*ctrl).ch_status[1] = IEC958_AES1_CON_DIGDIGCONV_ID as u8;
    (*ctrl).ch_status[2] = 0x00;
    (*ctrl).ch_status[3] = (IEC958_AES3_CON_FS_44100 | IEC958_AES3_CON_CLOCK_1000PPM) as u8;

    (*spdif_priv).dpll_locked = false;

    (*spdif_priv).dma_params_tx.maxburst = (*(*spdif_priv).soc).tx_burst;
    (*spdif_priv).dma_params_rx.maxburst = (*(*spdif_priv).soc).rx_burst;
    (*spdif_priv).dma_params_tx.addr = (*res).start + REG_SPDIF_STL as u64;
    (*spdif_priv).dma_params_rx.addr = (*res).start + REG_SPDIF_SRL as u64;

    /* Register with ASoC */
    dev_set_drvdata(&mut (*pdev).dev, spdif_priv as *mut c_void);
    pm_runtime_enable(&mut (*pdev).dev);
    regcache_cache_only((*spdif_priv).regmap, true);

    /*
     * Register platform component before registering cpu dai for there
     * is not defer probe for platform component in snd_soc_add_pcm_runtime().
     */
    ret = imx_pcm_dma_init(pdev);
    if ret != 0 {
        dev_err_probe(&mut (*pdev).dev, ret, b"imx_pcm_dma_init failed\n\0".as_ptr() as *const c_char);
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &fsl_spdif_component,
                                          &mut (*spdif_priv).cpu_dai_drv, 1);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"failed to register DAI: %d\n\0".as_ptr() as *const c_char, ret);
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret
}

unsafe extern "C" fn fsl_spdif_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn fsl_spdif_runtime_suspend(dev: *mut device) -> c_int {
    let spdif_priv = dev_get_drvdata(dev) as *mut fsl_spdif_priv;
    let mut i: c_int;

    /* Disable all the interrupts */
    regmap_update_bits((*spdif_priv).regmap, REG_SPDIF_SIE, 0xffffff, 0);

    regmap_read((*spdif_priv).regmap, REG_SPDIF_SRPC,
                &mut (*spdif_priv).regcache_srpc);
    regcache_cache_only((*spdif_priv).regmap, true);

    i = 0;
    while i < STC_TXCLK_SRC_MAX {
        clk_disable_unprepare((*spdif_priv).txclk[i as usize]);
        i += 1;
    }

    if IS_ERR((*spdif_priv).spbaclk as *const c_void) == 0 {
        clk_disable_unprepare((*spdif_priv).spbaclk);
    }
    clk_disable_unprepare((*spdif_priv).coreclk);

    0
}

unsafe extern "C" fn fsl_spdif_runtime_resume(dev: *mut device) -> c_int {
    let spdif_priv = dev_get_drvdata(dev) as *mut fsl_spdif_priv;
    let mut ret: c_int;
    let mut i: c_int;

    ret = clk_prepare_enable((*spdif_priv).coreclk);
    if ret != 0 {
        dev_err(dev, b"failed to enable core clock\n\0".as_ptr() as *const c_char);
        return ret;
    }

    if IS_ERR((*spdif_priv).spbaclk as *const c_void) == 0 {
        ret = clk_prepare_enable((*spdif_priv).spbaclk);
        if ret != 0 {
            dev_err(dev, b"failed to enable spba clock\n\0".as_ptr() as *const c_char);
            clk_disable_unprepare((*spdif_priv).coreclk);
            return ret;
        }
    }

    i = 0;
    while i < STC_TXCLK_SRC_MAX {
        ret = clk_prepare_enable((*spdif_priv).txclk[i as usize]);
        if ret != 0 {
            i -= 1;
            while i >= 0 {
                clk_disable_unprepare((*spdif_priv).txclk[i as usize]);
                i -= 1;
            }
            if IS_ERR((*spdif_priv).spbaclk as *const c_void) == 0 {
                clk_disable_unprepare((*spdif_priv).spbaclk);
            }
            clk_disable_unprepare((*spdif_priv).coreclk);
            return ret;
        }
        i += 1;
    }

    regcache_cache_only((*spdif_priv).regmap, false);
    regcache_mark_dirty((*spdif_priv).regmap);

    regmap_update_bits((*spdif_priv).regmap, REG_SPDIF_SRPC,
                       SRPC_CLKSRC_SEL_MASK | SRPC_GAINSEL_MASK,
                       (*spdif_priv).regcache_srpc);

    ret = regcache_sync((*spdif_priv).regmap);
    if ret != 0 {
        i -= 1;
        while i >= 0 {
            clk_disable_unprepare((*spdif_priv).txclk[i as usize]);
            i -= 1;
        }
        if IS_ERR((*spdif_priv).spbaclk as *const c_void) == 0 {
            clk_disable_unprepare((*spdif_priv).spbaclk);
        }
        clk_disable_unprepare((*spdif_priv).coreclk);
        return ret;
    }

    0
}

static fsl_spdif_pm: dev_pm_ops = dev_pm_ops { _private: [] };
/* SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
 * RUNTIME_PM_OPS(fsl_spdif_runtime_suspend, fsl_spdif_runtime_resume, NULL)
 */

static fsl_spdif_dt_ids: [of_device_id; 7] = [
    of_device_id { compatible: b"fsl,imx35-spdif\0".as_ptr() as *const c_char, data: &fsl_spdif_imx35 as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,vf610-spdif\0".as_ptr() as *const c_char, data: &fsl_spdif_vf610 as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx6sx-spdif\0".as_ptr() as *const c_char, data: &fsl_spdif_imx6sx as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx8qm-spdif\0".as_ptr() as *const c_char, data: &fsl_spdif_imx8qm as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx8mm-spdif\0".as_ptr() as *const c_char, data: &fsl_spdif_imx8mm as *const _ as *const c_void },
    of_device_id { compatible: b"fsl,imx8ulp-spdif\0".as_ptr() as *const c_char, data: &fsl_spdif_imx8ulp as *const _ as *const c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];
/* MODULE_DEVICE_TABLE(of, fsl_spdif_dt_ids); */

static mut fsl_spdif_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"fsl-spdif-dai\0".as_ptr() as *const c_char,
        of_match_table: fsl_spdif_dt_ids.as_ptr(),
        pm: &fsl_spdif_pm,
    },
    probe: Some(fsl_spdif_probe),
    remove: Some(fsl_spdif_remove),
};

/* module_platform_driver(fsl_spdif_driver); */

/* MODULE_AUTHOR("Freescale Semiconductor, Inc."); */
/* MODULE_DESCRIPTION("Freescale S/PDIF CPU DAI Driver"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:fsl-spdif-dai"); */

type spdif_gainsel = c_uint;
type spdif_txrate = c_uint;

extern "C" {
    static SPDIF_UBITS_SIZE: u32;
    static SPDIF_QSUB_SIZE: u32;
    static SPDIF_TXRATE_MAX: u32;
    static STC_TXCLK_SRC_MAX: c_int;
    static STC_TXCLK_SPDIF_ROOT: c_int;
    static SRPC_CLKSRC_MAX: c_uint;
    static GAINSEL_MULTI_MAX: c_uint;
    static SPDIF_DEFAULT_GAINSEL: c_uint;

    static INT_TXFIFO_RESYNC: u32;
    static INT_SYM_ERR: u32;
    static INT_BIT_ERR: u32;
    static INT_URX_FUL: u32;
    static INT_URX_OV: u32;
    static INT_QRX_FUL: u32;
    static INT_QRX_OV: u32;
    static INT_UQ_SYNC: u32;
    static INT_UQ_ERR: u32;
    static INT_RXFIFO_RESYNC: u32;
    static INT_LOSS_LOCK: u32;
    static INT_DPLL_LOCKED: u32;
    static INT_TXFIFO_UNOV: u32;
    static INT_CNEW: u32;
    static INT_VAL_NOGOOD: u32;
    static INT_RXFIFO_UNOV: u32;
    static INT_TX_EM: u32;
    static INT_RXFIFO_FUL: u32;

    static REG_SPDIF_SRPC: u32;
    static REG_SPDIF_SIE: u32;
    static REG_SPDIF_SRU: u32;
    static REG_SPDIF_SRQ: u32;
    static REG_SPDIF_SIS: u32;
    static REG_SPDIF_SIC: u32;
    static REG_SPDIF_SCR: u32;
    static REG_SPDIF_STCSCH: u32;
    static REG_SPDIF_STCSCL: u32;
    static REG_SPDIF_STCCA_191_160: u32;
    static REG_SPDIF_STC: u32;
    static REG_SPDIF_STL: u32;
    static REG_SPDIF_STR: u32;
    static REG_SPDIF_SRCSH: u32;
    static REG_SPDIF_SRCSL: u32;
    static REG_SPDIF_SRCD: u32;
    static REG_SPDIF_SRFM: u32;
    static REG_SPDIF_SRL: u32;
    static REG_SPDIF_SRR: u32;
    static REG_SPDIF_STCSPH: u32;
    static REG_SPDIF_STCSPL: u32;
    static REG_SPDIF_SRCCA_31_0: u32;
    static REG_SPDIF_SRCCA_63_32: u32;
    static REG_SPDIF_SRCCA_95_64: u32;
    static REG_SPDIF_SRCCA_127_96: u32;
    static REG_SPDIF_SRCCA_159_128: u32;
    static REG_SPDIF_SRCCA_191_160: u32;
    static REG_SPDIF_STCCA_31_0: u32;
    static REG_SPDIF_STCCA_63_32: u32;
    static REG_SPDIF_STCCA_95_64: u32;
    static REG_SPDIF_STCCA_127_96: u32;
    static REG_SPDIF_STCCA_159_128: u32;

    static SRPC_DPLL_LOCKED: u32;
    static SRPC_CLKSRC_SEL_MASK: u32;
    static SRPC_GAINSEL_MASK: u32;
    static SRPC_CLKSRC_SEL_OFFSET: u32;
    static SCR_SOFT_RESET: u32;
    static SCR_TXFIFO_AUTOSYNC: u32;
    static SCR_TXFIFO_CTRL_NORMAL: u32;
    static SCR_TXSEL_NORMAL: u32;
    static SCR_USRC_SEL_CHIP: u32;
    static SCR_TXFIFO_FSEL_IF8: u32;
    static SCR_TXFIFO_AUTOSYNC_MASK: u32;
    static SCR_TXFIFO_CTRL_MASK: u32;
    static SCR_TXSEL_MASK: u32;
    static SCR_USRC_SEL_MASK: u32;
    static SCR_TXFIFO_FSEL_MASK: u32;
    static SCR_RXFIFO_FSEL_IF8: u32;
    static SCR_RXFIFO_AUTOSYNC: u32;
    static SCR_RXFIFO_FSEL_MASK: u32;
    static SCR_RXFIFO_AUTOSYNC_MASK: u32;
    static SCR_RXFIFO_CTL_MASK: u32;
    static SCR_RXFIFO_OFF_MASK: u32;
    static SCR_LOW_POWER: u32;
    static SCR_RXFIFO_OFF: u32;
    static SCR_RXFIFO_CTL_ZERO: u32;
    static SCR_VAL_MASK: u32;
    static SCR_VAL_OFFSET: u32;
    static SCR_VAL_CLEAR: u32;
    static SCR_RAW_CAPTURE_MODE: u32;
    static SCR_TXSEL_RX: u32;
    static SCR_TXSEL_OFF: u32;
    static STC_TXCLK_ALL_EN: u32;
    static STC_TXCLK_ALL_EN_MASK: u32;
    static STC_TXCLK_SRC_MASK: u32;
    static STC_TXCLK_DF_MASK: u32;
    static STC_SYSCLK_DF_MASK: u32;
    static SRCD_CD_USER: u32;
    static SRCD_CD_USER_OFFSET: u32;

    static SPDIF_TXRATE_22050: u32;
    static SPDIF_TXRATE_32000: u32;
    static SPDIF_TXRATE_44100: u32;
    static SPDIF_TXRATE_48000: u32;
    static SPDIF_TXRATE_88200: u32;
    static SPDIF_TXRATE_96000: u32;
    static SPDIF_TXRATE_176400: u32;
    static SPDIF_TXRATE_192000: u32;

    static IEC958_AES3_CON_FS_22050: u32;
    static IEC958_AES3_CON_FS_32000: u32;
    static IEC958_AES3_CON_FS_44100: u32;
    static IEC958_AES3_CON_FS_48000: u32;
    static IEC958_AES3_CON_FS_88200: u32;
    static IEC958_AES3_CON_FS_96000: u32;
    static IEC958_AES3_CON_FS_176400: u32;
    static IEC958_AES3_CON_FS_192000: u32;
    static IEC958_AES3_CON_FS: u32;
    static IEC958_AES3_CON_CLOCK: u32;
    static IEC958_AES3_CON_CLOCK_1000PPM: u32;
    static IEC958_AES0_CON_NOT_COPYRIGHT: u32;
    static IEC958_AES0_CON_EMPHASIS_5015: u32;
    static IEC958_AES1_CON_DIGDIGCONV_ID: u32;

    static FSL_SPDIF_FORMATS_PLAYBACK: u64;
    static FSL_SPDIF_RATES_PLAYBACK: u32;
    static FSL_SPDIF_RATES_CAPTURE: u32;
    static FSL_SPDIF_FORMATS_CAPTURE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_CTL_ELEM_TYPE_IEC958: c_uint;
    static SNDRV_CTL_ELEM_TYPE_BYTES: c_uint;
    static SNDRV_CTL_ELEM_TYPE_INTEGER: c_uint;
    static SNDRV_CTL_ELEM_IFACE_MIXER: c_uint;
    static SNDRV_CTL_ELEM_IFACE_PCM: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_READ: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_WRITE: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_VOLATILE: c_uint;
    static SNDRV_CTL_ELEM_ACCESS_READWRITE: c_uint;
    static SNDRV_CTL_EVENT_MASK_VALUE: c_uint;
    static REGCACHE_FLAT: c_uint;
    static IRQ_HANDLED: irqreturn_t;
    static GFP_KERNEL: c_uint;
    static EBUSY: c_int;
    static EINVAL: c_int;
    static EAGAIN: c_int;
    static ENOMEM: c_int;

    fn SRPC_CLKSRC_SEL_SET(v: u32) -> u32;
    fn SRPC_GAINSEL_SET(v: u32) -> u32;
    fn STC_TXCLK_SRC_SET(v: u32) -> u32;
    fn STC_TXCLK_DF(v: u32) -> u32;
    fn STC_SYSCLK_DF(v: u32) -> u32;
    fn SCR_DMA_xX_EN(tx: bool) -> u32;

    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> c_int;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn regcache_cache_bypass(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn bitrev8(v: u8) -> u8;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn snd_ctl_notify(card: *mut snd_card, mask: c_uint, id: *mut snd_ctl_elem_id);
    fn unlikely(v: c_int) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, idx: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_active(dai: *mut snd_soc_dai) -> c_int;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_ulong;
    fn clk_is_match(a: *mut clk, b: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn fsl_asoc_reparent_pll_clocks(dev: *mut device, clk: *mut clk, pll8k: *mut clk, pll11k: *mut clk, sample_rate: c_uint);
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn snd_ctl_boolean_mono_info(kcontrol: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
    fn snd_soc_get_pcm_runtime(card: *mut snd_soc_card, dai_link: *mut c_void) -> *mut snd_soc_pcm_runtime;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn snd_soc_dai_init_dma_data(dai: *mut snd_soc_dai, tx: *mut snd_dmaengine_dai_dma_data, rx: *mut snd_dmaengine_dai_dma_data);
    fn snd_soc_add_dai_controls(dai: *mut snd_soc_dai, controls: *const snd_kcontrol_new, num: c_uint) -> c_int;
    fn snd_soc_card_get_kcontrol(card: *mut snd_soc_card, name: *const c_char) -> *mut snd_kcontrol;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_platform_get_and_ioremap_resource(pdev: *mut platform_device, index: c_uint, res: *mut *mut resource) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> c_int;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>, flags: c_uint, name: *const c_char, dev_id: *mut c_void) -> c_int;
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn fsl_asoc_get_pll_clocks(dev: *mut device, pll8k: *mut *mut clk, pll11k: *mut *mut clk);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn imx_pcm_dma_init(pdev: *mut platform_device) -> c_int;
    fn devm_snd_soc_register_component(dev: *mut device, component: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
