// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * atmel_ssc_dai.c  --  ALSA SoC ATMEL SSC Audio Layer Platform driver
 *
 * Copyright (C) 2005 SAN People
 * Copyright (C) 2008 Atmel
 *
 * Author: Sedji Gaouaou <sedji.gaouaou@atmel.com>
 *         ATMEL CORP.
 *
 * Based on at91-ssc.c by
 * Frank Mandarino <fmandarino@endrelia.com>
 * Based on pxa2xx Platform drivers by
 * Liam Girdwood <lrg@slimlogic.co.uk>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u32 = u32;
type u64 = u64;
type irqreturn_t = c_int;

const NUM_SSC_DEVICES: usize = 3;

const IRQ_HANDLED: irqreturn_t = 1;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;

const ATMEL_SSC_CMR_DIV: c_int = 0;
const ATMEL_SSC_TCMR_PERIOD: c_int = 1;
const ATMEL_SSC_RCMR_PERIOD: c_int = 2;

const SSC_DIR_MASK_UNUSED: c_int = 0;
const SSC_DIR_MASK_PLAYBACK: c_int = 1 << 0;
const SSC_DIR_MASK_CAPTURE: c_int = 1 << 1;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_HW_PARAM_FRAME_BITS: c_int = 1;
const SNDRV_PCM_HW_PARAM_CHANNELS: c_int = 2;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;

const SNDRV_PCM_FORMAT_S8: c_int = 0;
const SNDRV_PCM_FORMAT_S16_LE: c_int = 1;
const SNDRV_PCM_FORMAT_S24_LE: c_int = 2;
const SNDRV_PCM_FORMAT_S32_LE: c_int = 3;

const SNDRV_PCM_FMTBIT_S8: u64 = 1 << SNDRV_PCM_FORMAT_S8;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << SNDRV_PCM_FORMAT_S16_LE;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << SNDRV_PCM_FORMAT_S32_LE;
const SNDRV_PCM_RATE_CONTINUOUS: u32 = 1 << 30;

const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0x000f;
const SND_SOC_DAIFMT_BC_FP: c_uint = 0x0001;
const SND_SOC_DAIFMT_BC_FC: c_uint = 0x0002;
const SND_SOC_DAIFMT_BP_FP: c_uint = 0x0003;
const SND_SOC_DAIFMT_BP_FC: c_uint = 0x0004;
const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x00f0;
const SND_SOC_DAIFMT_LEFT_J: c_uint = 0x0010;
const SND_SOC_DAIFMT_I2S: c_uint = 0x0020;
const SND_SOC_DAIFMT_DSP_A: c_uint = 0x0030;
const SND_SOC_POSSIBLE_DAIFMT_I2S: u64 = 1 << 0;
const SND_SOC_POSSIBLE_DAIFMT_LEFT_J: u64 = 1 << 1;
const SND_SOC_POSSIBLE_DAIFMT_DSP_A: u64 = 1 << 2;

const ATMEL_PDC_TPR: u32 = 0;
const ATMEL_PDC_TCR: u32 = 0;
const ATMEL_PDC_TNPR: u32 = 0;
const ATMEL_PDC_TNCR: u32 = 0;
const ATMEL_PDC_RPR: u32 = 0;
const ATMEL_PDC_RCR: u32 = 0;
const ATMEL_PDC_RNPR: u32 = 0;
const ATMEL_PDC_RNCR: u32 = 0;
const ATMEL_PDC_TXTEN: u32 = 0;
const ATMEL_PDC_TXTDIS: u32 = 0;
const ATMEL_PDC_RXTEN: u32 = 0;
const ATMEL_PDC_RXTDIS: u32 = 0;

const CR: c_int = 0;
const SR: c_int = 1;
const IMR: c_int = 2;
const IDR: c_int = 3;
const IER: c_int = 4;
const CMR: c_int = 5;
const RCMR: c_int = 6;
const RFMR: c_int = 7;
const TCMR: c_int = 8;
const TFMR: c_int = 9;
const PDC_RPR: c_int = 10;
const PDC_RCR: c_int = 11;
const PDC_RNPR: c_int = 12;
const PDC_RNCR: c_int = 13;
const PDC_TPR: c_int = 14;
const PDC_TCR: c_int = 15;
const PDC_TNPR: c_int = 16;
const PDC_TNCR: c_int = 17;

const CR_TXEN: c_int = 0;
const CR_TXDIS: c_int = 1;
const CR_RXEN: c_int = 2;
const CR_RXDIS: c_int = 3;
const CR_SWRST: c_int = 4;
const SR_ENDTX: c_int = 5;
const SR_TXBUFE: c_int = 6;
const SR_ENDRX: c_int = 7;
const SR_RXBUFF: c_int = 8;
const SR_OVRUN: c_int = 9;
const SR_RXEN: c_int = 10;
const SR_TXEN: c_int = 11;

const SSC_FSOS_POSITIVE: c_int = 0;
const SSC_FSOS_NEGATIVE: c_int = 1;
const SSC_FSOS_NONE: c_int = 2;
const SSC_START_RISING_RF: c_int = 0;
const SSC_START_FALLING_RF: c_int = 1;
const SSC_CKS_DIV: c_int = 0;
const SSC_CKS_PIN: c_int = 1;
const SSC_CKS_CLOCK: c_int = 2;
const SSC_CKO_NONE: c_int = 0;
const SSC_CKO_CONTINUOUS: c_int = 1;
const SSC_CKI_RISING: c_int = 0;
const SSC_CKI_FALLING: c_int = 1;
const SSC_FSEDGE_POSITIVE: c_int = 0;

const RCMR_STTDLY: c_int = 0;
const TCMR_STTDLY: c_int = 0;
const RCMR_START: c_int = 0;
const TCMR_START: c_int = 0;
const RCMR_CKS: c_int = 0;
const RCMR_CKO: c_int = 0;
const TCMR_CKS: c_int = 0;
const TCMR_CKO: c_int = 0;
const RCMR_PERIOD: c_int = 0;
const RCMR_CKI: c_int = 0;
const TCMR_PERIOD: c_int = 0;
const TCMR_CKI: c_int = 0;
const RFMR_FSLEN_EXT: c_int = 0;
const RFMR_FSEDGE: c_int = 0;
const RFMR_FSOS: c_int = 0;
const RFMR_FSLEN: c_int = 0;
const RFMR_DATNB: c_int = 0;
const RFMR_MSBF: c_int = 0;
const RFMR_LOOP: c_int = 0;
const RFMR_DATLEN: c_int = 0;
const TFMR_FSLEN_EXT: c_int = 0;
const TFMR_FSEDGE: c_int = 0;
const TFMR_FSDEN: c_int = 0;
const TFMR_FSOS: c_int = 0;
const TFMR_FSLEN: c_int = 0;
const TFMR_DATNB: c_int = 0;
const TFMR_MSBF: c_int = 0;
const TFMR_DATDEF: c_int = 0;
const TFMR_DATLEN: c_int = 0;

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

const fn BIT(nr: c_int) -> u32 {
    1u32 << nr
}

const fn SSC_BIT(nr: c_int) -> u32 {
    BIT(nr)
}

const fn SSC_BF(_field: c_int, value: c_int) -> u32 {
    value as u32
}

fn DIV_ROUND_CLOSEST(x: u32, divisor: c_int) -> u32 {
    ((x as c_int + divisor / 2) / divisor) as u32
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    pub rate_num: c_uint,
    pub rate_den: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_rule {
    pub private: *mut c_void,
    pub var: c_int,
}

#[repr(C)]
pub struct snd_interval {
    pub min: c_uint,
    pub max: c_uint,
    pub openmin: c_uint,
    pub openmax: c_uint,
    pub integer: c_uint,
}

#[repr(C)]
pub struct snd_ratnum {
    pub num: c_uint,
    pub den_min: c_uint,
    pub den_max: c_uint,
    pub den_step: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct platform_device {
    pub id: c_int,
    pub dev: device,
}

#[repr(C)]
pub struct ssc_platform_data {
    pub use_dma: bool,
    pub has_fslen_ext: bool,
}

#[repr(C)]
pub struct ssc_device {
    pub regs: *mut c_void,
    pub clk: *mut clk,
    pub irq: c_int,
    pub pdata: *mut ssc_platform_data,
    pub pdev: *mut platform_device,
    pub clk_from_rk_pin: bool,
}

#[repr(C)]
pub struct atmel_pdc_regs {
    pub xpr: u32,
    pub xcr: u32,
    pub xnpr: u32,
    pub xncr: u32,
}

#[repr(C)]
pub struct atmel_ssc_mask {
    pub ssc_enable: u32,
    pub ssc_disable: u32,
    pub ssc_endx: u32,
    pub ssc_endbuf: u32,
    pub ssc_error: u32,
    pub pdc_enable: u32,
    pub pdc_disable: u32,
}

#[repr(C)]
pub struct atmel_pcm_dma_params {
    pub name: *const c_char,
    pub pdc: *mut atmel_pdc_regs,
    pub mask: *mut atmel_ssc_mask,
    pub dma_intr_handler: Option<unsafe extern "C" fn(u32, *mut snd_pcm_substream)>,
    pub substream: *mut snd_pcm_substream,
    pub ssc: *mut ssc_device,
    pub pdc_xfer_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ssc_state {
    pub ssc_sr: u32,
    pub ssc_imr: u32,
    pub ssc_cmr: u32,
    pub ssc_rcmr: u32,
    pub ssc_rfmr: u32,
    pub ssc_tcmr: u32,
    pub ssc_tfmr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_ssc_info {
    pub name: *const c_char,
    pub dir_mask: c_int,
    pub initialized: c_int,
    pub ssc: *mut ssc_device,
    pub dma_params: [*mut atmel_pcm_dma_params; 2],
    pub mck_rate: u32,
    pub daifmt: c_uint,
    pub cmr_div: u32,
    pub tcmr_period: u32,
    pub rcmr_period: u32,
    pub forced_divider: u32,
    pub ssc_state: atmel_ssc_state,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub set_clkdiv: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: u32,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub suspend: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub legacy_dai_naming: c_int,
}

unsafe extern "C" {
    fn ssc_readl(regs: *mut c_void, reg: c_int) -> u32;
    fn ssc_writel(regs: *mut c_void, reg: c_int, value: u32);
    fn hw_param_interval(params: *mut snd_pcm_hw_params, var: c_int) -> *mut snd_interval;
    fn snd_soc_params_to_frame_size(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_soc_params_to_bclk(params: *mut snd_pcm_hw_params) -> c_int;
    fn snd_interval_ratnum(
        i: *mut snd_interval,
        rats_count: c_int,
        rats: *mut snd_ratnum,
        num: *mut c_uint,
        den: *mut c_uint,
    ) -> c_int;
    fn snd_interval_refine(i: *mut snd_interval, v: *mut snd_interval) -> c_int;
    fn clk_enable(clk: *mut clk) -> c_int;
    fn clk_disable(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> u32;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
        private: *mut c_void,
        arg1: c_int,
        arg2: c_int,
        terminator: c_int,
    ) -> c_int;
    fn snd_soc_dai_set_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut c_void,
    );
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn request_irq(
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_int, dev: *mut c_void);
    fn snd_soc_component_active(component: *mut snd_soc_component) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn atmel_pcm_dma_platform_register(dev: *mut device) -> c_int;
    fn atmel_pcm_pdc_platform_register(dev: *mut device) -> c_int;
    fn ssc_request(ssc_id: c_int) -> *mut ssc_device;
    fn ssc_free(ssc: *mut ssc_device);
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
}

type c_long = isize;

unsafe fn to_platform_device(dev: *mut device) -> *mut platform_device {
    dev as *mut platform_device
}

/*
 * SSC PDC registers required by the PCM DMA engine.
 */
static mut pdc_tx_reg: atmel_pdc_regs = atmel_pdc_regs {
    xpr: ATMEL_PDC_TPR,
    xcr: ATMEL_PDC_TCR,
    xnpr: ATMEL_PDC_TNPR,
    xncr: ATMEL_PDC_TNCR,
};

static mut pdc_rx_reg: atmel_pdc_regs = atmel_pdc_regs {
    xpr: ATMEL_PDC_RPR,
    xcr: ATMEL_PDC_RCR,
    xnpr: ATMEL_PDC_RNPR,
    xncr: ATMEL_PDC_RNCR,
};

/*
 * SSC & PDC status bits for transmit and receive.
 */
static mut ssc_tx_mask: atmel_ssc_mask = atmel_ssc_mask {
    ssc_enable: SSC_BIT(CR_TXEN),
    ssc_disable: SSC_BIT(CR_TXDIS),
    ssc_endx: SSC_BIT(SR_ENDTX),
    ssc_endbuf: SSC_BIT(SR_TXBUFE),
    ssc_error: SSC_BIT(SR_OVRUN),
    pdc_enable: ATMEL_PDC_TXTEN,
    pdc_disable: ATMEL_PDC_TXTDIS,
};

static mut ssc_rx_mask: atmel_ssc_mask = atmel_ssc_mask {
    ssc_enable: SSC_BIT(CR_RXEN),
    ssc_disable: SSC_BIT(CR_RXDIS),
    ssc_endx: SSC_BIT(SR_ENDRX),
    ssc_endbuf: SSC_BIT(SR_RXBUFF),
    ssc_error: SSC_BIT(SR_OVRUN),
    pdc_enable: ATMEL_PDC_RXTEN,
    pdc_disable: ATMEL_PDC_RXTDIS,
};

/*
 * DMA parameters.
 */
static mut ssc_dma_params: [[atmel_pcm_dma_params; 2]; NUM_SSC_DEVICES] = [
    [
        atmel_pcm_dma_params {
            name: c_str!("SSC0 PCM out"),
            pdc: core::ptr::addr_of_mut!(pdc_tx_reg),
            mask: core::ptr::addr_of_mut!(ssc_tx_mask),
            dma_intr_handler: None,
            substream: core::ptr::null_mut(),
            ssc: core::ptr::null_mut(),
            pdc_xfer_size: 0,
        },
        atmel_pcm_dma_params {
            name: c_str!("SSC0 PCM in"),
            pdc: core::ptr::addr_of_mut!(pdc_rx_reg),
            mask: core::ptr::addr_of_mut!(ssc_rx_mask),
            dma_intr_handler: None,
            substream: core::ptr::null_mut(),
            ssc: core::ptr::null_mut(),
            pdc_xfer_size: 0,
        },
    ],
    [
        atmel_pcm_dma_params {
            name: c_str!("SSC1 PCM out"),
            pdc: core::ptr::addr_of_mut!(pdc_tx_reg),
            mask: core::ptr::addr_of_mut!(ssc_tx_mask),
            dma_intr_handler: None,
            substream: core::ptr::null_mut(),
            ssc: core::ptr::null_mut(),
            pdc_xfer_size: 0,
        },
        atmel_pcm_dma_params {
            name: c_str!("SSC1 PCM in"),
            pdc: core::ptr::addr_of_mut!(pdc_rx_reg),
            mask: core::ptr::addr_of_mut!(ssc_rx_mask),
            dma_intr_handler: None,
            substream: core::ptr::null_mut(),
            ssc: core::ptr::null_mut(),
            pdc_xfer_size: 0,
        },
    ],
    [
        atmel_pcm_dma_params {
            name: c_str!("SSC2 PCM out"),
            pdc: core::ptr::addr_of_mut!(pdc_tx_reg),
            mask: core::ptr::addr_of_mut!(ssc_tx_mask),
            dma_intr_handler: None,
            substream: core::ptr::null_mut(),
            ssc: core::ptr::null_mut(),
            pdc_xfer_size: 0,
        },
        atmel_pcm_dma_params {
            name: c_str!("SSC2 PCM in"),
            pdc: core::ptr::addr_of_mut!(pdc_rx_reg),
            mask: core::ptr::addr_of_mut!(ssc_rx_mask),
            dma_intr_handler: None,
            substream: core::ptr::null_mut(),
            ssc: core::ptr::null_mut(),
            pdc_xfer_size: 0,
        },
    ],
];

const EMPTY_STATE: atmel_ssc_state = atmel_ssc_state {
    ssc_sr: 0,
    ssc_imr: 0,
    ssc_cmr: 0,
    ssc_rcmr: 0,
    ssc_rfmr: 0,
    ssc_tcmr: 0,
    ssc_tfmr: 0,
};

static mut ssc_info: [atmel_ssc_info; NUM_SSC_DEVICES] = [
    atmel_ssc_info {
        name: c_str!("ssc0"),
        dir_mask: SSC_DIR_MASK_UNUSED,
        initialized: 0,
        ssc: core::ptr::null_mut(),
        dma_params: [core::ptr::null_mut(); 2],
        mck_rate: 0,
        daifmt: 0,
        cmr_div: 0,
        tcmr_period: 0,
        rcmr_period: 0,
        forced_divider: 0,
        ssc_state: EMPTY_STATE,
    },
    atmel_ssc_info {
        name: c_str!("ssc1"),
        dir_mask: SSC_DIR_MASK_UNUSED,
        initialized: 0,
        ssc: core::ptr::null_mut(),
        dma_params: [core::ptr::null_mut(); 2],
        mck_rate: 0,
        daifmt: 0,
        cmr_div: 0,
        tcmr_period: 0,
        rcmr_period: 0,
        forced_divider: 0,
        ssc_state: EMPTY_STATE,
    },
    atmel_ssc_info {
        name: c_str!("ssc2"),
        dir_mask: SSC_DIR_MASK_UNUSED,
        initialized: 0,
        ssc: core::ptr::null_mut(),
        dma_params: [core::ptr::null_mut(); 2],
        mck_rate: 0,
        daifmt: 0,
        cmr_div: 0,
        tcmr_period: 0,
        rcmr_period: 0,
        forced_divider: 0,
        ssc_state: EMPTY_STATE,
    },
];

/*
 * SSC interrupt handler.  Passes PDC interrupts to the DMA
 * interrupt handler in the PCM driver.
 */
unsafe extern "C" fn atmel_ssc_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let ssc_p = dev_id as *mut atmel_ssc_info;
    let ssc_sr: u32;
    let mut ssc_substream_mask: u32;
    let mut i: usize;

    ssc_sr = (ssc_readl((*(*ssc_p).ssc).regs, SR) as c_ulong
        & ssc_readl((*(*ssc_p).ssc).regs, IMR) as c_ulong) as u32;

    /*
     * Loop through the substreams attached to this SSC.  If
     * a DMA-related interrupt occurred on that substream, call
     * the DMA interrupt handler function, if one has been
     * registered in the dma_params structure by the PCM driver.
     */
    i = 0;
    while i < (*ssc_p).dma_params.len() {
        let dma_params = (*ssc_p).dma_params[i];

        if !dma_params.is_null() && (*dma_params).dma_intr_handler.is_some() {
            ssc_substream_mask = (*(*dma_params).mask).ssc_endx | (*(*dma_params).mask).ssc_endbuf;
            if ssc_sr & ssc_substream_mask != 0 {
                ((*dma_params).dma_intr_handler.unwrap())(ssc_sr, (*dma_params).substream);
            }
        }
        i += 1;
    }

    IRQ_HANDLED
}

/*
 * When the bit clock is input, limit the maximum rate according to the
 * Serial Clock Ratio Considerations section from the SSC documentation:
 *
 *   The Transmitter and the Receiver can be programmed to operate
 *   with the clock signals provided on either the TK or RK pins.
 *   This allows the SSC to support many slave-mode data transfers.
 *   In this case, the maximum clock speed allowed on the RK pin is:
 *   - Peripheral clock divided by 2 if Receiver Frame Synchro is input
 *   - Peripheral clock divided by 3 if Receiver Frame Synchro is output
 *   In addition, the maximum clock speed allowed on the TK pin is:
 *   - Peripheral clock divided by 6 if Transmit Frame Synchro is input
 *   - Peripheral clock divided by 2 if Transmit Frame Synchro is output
 *
 * When the bit clock is output, limit the rate according to the
 * SSC divider restrictions.
 */
unsafe extern "C" fn atmel_ssc_hw_rule_rate(
    params: *mut snd_pcm_hw_params,
    rule: *mut snd_pcm_hw_rule,
) -> c_int {
    let ssc_p = (*rule).private as *mut atmel_ssc_info;
    let ssc = (*ssc_p).ssc;
    let i = hw_param_interval(params, (*rule).var);
    let mut t: snd_interval = core::mem::zeroed();
    let mut r = snd_ratnum {
        num: 0,
        den_min: 1,
        den_max: 4095,
        den_step: 1,
    };
    let mut num: c_uint = 0;
    let mut den: c_uint = 0;
    let frame_size: c_int;
    let mut mck_div: c_int = 2;
    let ret: c_int;

    frame_size = snd_soc_params_to_frame_size(params);
    if frame_size < 0 {
        return frame_size;
    }

    match (*ssc_p).daifmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FP => {
            if ((*ssc_p).dir_mask & SSC_DIR_MASK_CAPTURE) != 0 && (*ssc).clk_from_rk_pin {
                /*
                 * Receiver Frame Synchro (i.e. capture)
                 * is output (format is _CFS) and the RK pin
                 * is used for input (format is _CBM_).
                 */
                mck_div = 3;
            }
        }
        SND_SOC_DAIFMT_BC_FC => {
            if ((*ssc_p).dir_mask & SSC_DIR_MASK_PLAYBACK) != 0 && !(*ssc).clk_from_rk_pin {
                /*
                 * Transmit Frame Synchro (i.e. playback)
                 * is input (format is _CFM) and the TK pin
                 * is used for input (format _CBM_ but not
                 * using the RK pin).
                 */
                mck_div = 6;
            }
        }
        _ => {}
    }

    match (*ssc_p).daifmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FP => {
            r.num = (*ssc_p).mck_rate / mck_div as u32 / frame_size as u32;

            ret = snd_interval_ratnum(i, 1, &mut r, &mut num, &mut den);
            if ret >= 0 && den != 0 && (*rule).var == SNDRV_PCM_HW_PARAM_RATE {
                (*params).rate_num = num;
                (*params).rate_den = den;
            }
        }
        SND_SOC_DAIFMT_BC_FP | SND_SOC_DAIFMT_BC_FC => {
            t.min = 8000;
            t.max = (*ssc_p).mck_rate / mck_div as u32 / frame_size as u32;
            t.openmin = 0;
            t.openmax = 0;
            t.integer = 0;
            ret = snd_interval_refine(i, &mut t);
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

/*-------------------------------------------------------------------------*\
 * DAI functions
\*-------------------------------------------------------------------------*/
/*
 * Startup.  Only that one substream allowed in each direction.
 */
unsafe extern "C" fn atmel_ssc_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pdev = to_platform_device((*dai).dev);
    let ssc_p = core::ptr::addr_of_mut!(ssc_info[(*pdev).id as usize]);
    let dma_params: *mut atmel_pcm_dma_params;
    let dir: c_int;
    let dir_mask: c_int;
    let mut ret: c_int;

    pr_debug(
        c_str!("atmel_ssc_startup: SSC_SR=0x%x\n"),
        ssc_readl((*(*ssc_p).ssc).regs, SR),
    );

    /* Enable PMC peripheral clock for this SSC */
    pr_debug(c_str!("atmel_ssc_dai: Starting clock\n"));
    ret = clk_enable((*(*ssc_p).ssc).clk);
    if ret != 0 {
        return ret;
    }

    (*ssc_p).mck_rate = clk_get_rate((*(*ssc_p).ssc).clk);

    /* Reset the SSC unless initialized to keep it in a clean state */
    if (*ssc_p).initialized == 0 {
        ssc_writel((*(*ssc_p).ssc).regs, CR, SSC_BIT(CR_SWRST));
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        dir = 0;
        dir_mask = SSC_DIR_MASK_PLAYBACK;
    } else {
        dir = 1;
        dir_mask = SSC_DIR_MASK_CAPTURE;
    }

    ret = snd_pcm_hw_rule_add(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        atmel_ssc_hw_rule_rate,
        ssc_p as *mut c_void,
        SNDRV_PCM_HW_PARAM_FRAME_BITS,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        -1,
    );
    if ret < 0 {
        dev_err((*dai).dev, c_str!("Failed to specify rate rule: %d\n"), ret);
        return ret;
    }

    dma_params = core::ptr::addr_of_mut!(ssc_dma_params[(*pdev).id as usize][dir as usize]);
    (*dma_params).ssc = (*ssc_p).ssc;
    (*dma_params).substream = substream;

    (*ssc_p).dma_params[dir as usize] = dma_params;

    snd_soc_dai_set_dma_data(dai, substream, dma_params as *mut c_void);

    if (*ssc_p).dir_mask & dir_mask != 0 {
        return -EBUSY;
    }

    (*ssc_p).dir_mask |= dir_mask;

    0
}

/*
 * Shutdown.  Clear DMA parameters and shutdown the SSC if there
 * are no other substreams open.
 */
unsafe extern "C" fn atmel_ssc_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let pdev = to_platform_device((*dai).dev);
    let ssc_p = core::ptr::addr_of_mut!(ssc_info[(*pdev).id as usize]);
    let dma_params: *mut atmel_pcm_dma_params;
    let dir: c_int;
    let dir_mask: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        dir = 0;
    } else {
        dir = 1;
    }

    dma_params = (*ssc_p).dma_params[dir as usize];

    if !dma_params.is_null() {
        (*dma_params).ssc = core::ptr::null_mut();
        (*dma_params).substream = core::ptr::null_mut();
        (*ssc_p).dma_params[dir as usize] = core::ptr::null_mut();
    }

    dir_mask = 1 << dir;

    (*ssc_p).dir_mask &= !dir_mask;
    if (*ssc_p).dir_mask == 0 {
        if (*ssc_p).initialized != 0 {
            free_irq((*(*ssc_p).ssc).irq, ssc_p as *mut c_void);
            (*ssc_p).initialized = 0;
        }

        /* Reset the SSC */
        ssc_writel((*(*ssc_p).ssc).regs, CR, SSC_BIT(CR_SWRST));
        /* Clear the SSC dividers */
        (*ssc_p).rcmr_period = 0;
        (*ssc_p).tcmr_period = (*ssc_p).rcmr_period;
        (*ssc_p).cmr_div = (*ssc_p).tcmr_period;
        (*ssc_p).forced_divider = 0;
    }

    /* Shutdown the SSC clock. */
    pr_debug(c_str!("atmel_ssc_dai: Stopping clock\n"));
    clk_disable((*(*ssc_p).ssc).clk);
}

/*
 * Record the DAI format for use in hw_params().
 */
unsafe extern "C" fn atmel_ssc_set_dai_fmt(cpu_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let pdev = to_platform_device((*cpu_dai).dev);
    let ssc_p = core::ptr::addr_of_mut!(ssc_info[(*pdev).id as usize]);

    (*ssc_p).daifmt = fmt;
    0
}

/*
 * Record SSC clock dividers for use in hw_params().
 */
unsafe extern "C" fn atmel_ssc_set_dai_clkdiv(
    cpu_dai: *mut snd_soc_dai,
    div_id: c_int,
    div: c_int,
) -> c_int {
    let pdev = to_platform_device((*cpu_dai).dev);
    let ssc_p = core::ptr::addr_of_mut!(ssc_info[(*pdev).id as usize]);

    match div_id {
        ATMEL_SSC_CMR_DIV => {
            /*
             * The same master clock divider is used for both
             * transmit and receive, so if a value has already
             * been set, it must match this value.
             */
            if (*ssc_p).dir_mask != (SSC_DIR_MASK_PLAYBACK | SSC_DIR_MASK_CAPTURE) {
                (*ssc_p).cmr_div = div as u32;
            } else if (*ssc_p).cmr_div == 0 {
                (*ssc_p).cmr_div = div as u32;
            } else if div as u32 != (*ssc_p).cmr_div {
                return -EBUSY;
            }
            (*ssc_p).forced_divider |= BIT(ATMEL_SSC_CMR_DIV);
        }
        ATMEL_SSC_TCMR_PERIOD => {
            (*ssc_p).tcmr_period = div as u32;
            (*ssc_p).forced_divider |= BIT(ATMEL_SSC_TCMR_PERIOD);
        }
        ATMEL_SSC_RCMR_PERIOD => {
            (*ssc_p).rcmr_period = div as u32;
            (*ssc_p).forced_divider |= BIT(ATMEL_SSC_RCMR_PERIOD);
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

/* Is the cpu-dai master of the frame clock? */
unsafe fn atmel_ssc_cfs(ssc_p: *mut atmel_ssc_info) -> c_int {
    match (*ssc_p).daifmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BC_FP | SND_SOC_DAIFMT_BP_FP => return 1,
        _ => {}
    }
    0
}

/* Is the cpu-dai master of the bit clock? */
unsafe fn atmel_ssc_cbs(ssc_p: *mut atmel_ssc_info) -> c_int {
    match (*ssc_p).daifmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_BP_FC | SND_SOC_DAIFMT_BP_FP => return 1,
        _ => {}
    }
    0
}

/*
 * Configure the SSC.
 */
unsafe extern "C" fn atmel_ssc_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pdev = to_platform_device((*dai).dev);
    let id = (*pdev).id;
    let ssc_p = core::ptr::addr_of_mut!(ssc_info[id as usize]);
    let ssc = (*ssc_p).ssc;
    let dma_params: *mut atmel_pcm_dma_params;
    let dir: c_int;
    let channels: c_int;
    let bits: c_int;
    let mut tfmr: u32;
    let mut rfmr: u32;
    let mut tcmr: u32;
    let mut rcmr: u32;
    let mut ret: c_int;
    let mut fslen: c_int;
    let mut fslen_ext: c_int;
    let mut fs_osync: c_int;
    let fs_edge: c_int;
    let mut cmr_div: u32;
    let mut tcmr_period: u32;
    let mut rcmr_period: u32;

    /*
     * Currently, there is only one set of dma params for
     * each direction.  If more are added, this code will
     * have to be changed to select the proper set.
     */
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        dir = 0;
    } else {
        dir = 1;
    }

    /*
     * If the cpu dai should provide BCLK, but noone has provided the
     * divider needed for that to work, fall back to something sensible.
     */
    cmr_div = (*ssc_p).cmr_div;
    if ((*ssc_p).forced_divider & BIT(ATMEL_SSC_CMR_DIV)) == 0 && atmel_ssc_cbs(ssc_p) != 0 {
        let bclk_rate = snd_soc_params_to_bclk(params);

        if bclk_rate < 0 {
            dev_err(
                (*dai).dev,
                c_str!("unable to calculate cmr_div: %d\n"),
                bclk_rate,
            );
            return bclk_rate;
        }

        cmr_div = DIV_ROUND_CLOSEST((*ssc_p).mck_rate, 2 * bclk_rate);
    }

    /*
     * If the cpu dai should provide LRCLK, but noone has provided the
     * dividers needed for that to work, fall back to something sensible.
     */
    tcmr_period = (*ssc_p).tcmr_period;
    rcmr_period = (*ssc_p).rcmr_period;
    if atmel_ssc_cfs(ssc_p) != 0 {
        let frame_size = snd_soc_params_to_frame_size(params);

        if frame_size < 0 {
            dev_err(
                (*dai).dev,
                c_str!("unable to calculate tx/rx cmr_period: %d\n"),
                frame_size,
            );
            return frame_size;
        }

        if ((*ssc_p).forced_divider & BIT(ATMEL_SSC_TCMR_PERIOD)) == 0 {
            tcmr_period = (frame_size / 2 - 1) as u32;
        }
        if ((*ssc_p).forced_divider & BIT(ATMEL_SSC_RCMR_PERIOD)) == 0 {
            rcmr_period = (frame_size / 2 - 1) as u32;
        }
    }

    dma_params = (*ssc_p).dma_params[dir as usize];

    channels = params_channels(params);

    /*
     * Determine sample size in bits and the PDC increment.
     */
    match params_format(params) {
        SNDRV_PCM_FORMAT_S8 => {
            bits = 8;
            (*dma_params).pdc_xfer_size = 1;
        }
        SNDRV_PCM_FORMAT_S16_LE => {
            bits = 16;
            (*dma_params).pdc_xfer_size = 2;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            bits = 24;
            (*dma_params).pdc_xfer_size = 4;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            bits = 32;
            (*dma_params).pdc_xfer_size = 4;
        }
        _ => {
            printk(c_str!("atmel_ssc_dai: unsupported PCM format"));
            return -EINVAL;
        }
    }

    /*
     * Compute SSC register settings.
     */

    fslen_ext = (bits - 1) / 16;
    fslen = (bits - 1) % 16;

    match (*ssc_p).daifmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_LEFT_J => {
            fs_osync = SSC_FSOS_POSITIVE;
            fs_edge = SSC_START_RISING_RF;

            rcmr = SSC_BF(RCMR_STTDLY, 0);
            tcmr = SSC_BF(TCMR_STTDLY, 0);
        }

        SND_SOC_DAIFMT_I2S => {
            fs_osync = SSC_FSOS_NEGATIVE;
            fs_edge = SSC_START_FALLING_RF;

            rcmr = SSC_BF(RCMR_STTDLY, 1);
            tcmr = SSC_BF(TCMR_STTDLY, 1);
        }

        SND_SOC_DAIFMT_DSP_A => {
            /*
             * DSP/PCM Mode A format
             *
             * Data is transferred on first BCLK after LRC pulse rising
             * edge.If stereo, the right channel data is contiguous with
             * the left channel data.
             */
            fs_osync = SSC_FSOS_POSITIVE;
            fs_edge = SSC_START_RISING_RF;
            fslen_ext = 0;
            fslen = fslen_ext;

            rcmr = SSC_BF(RCMR_STTDLY, 1);
            tcmr = SSC_BF(TCMR_STTDLY, 1);
        }

        _ => {
            printk(
                c_str!("atmel_ssc_dai: unsupported DAI format 0x%x\n"),
                (*ssc_p).daifmt,
            );
            return -EINVAL;
        }
    }

    if atmel_ssc_cfs(ssc_p) == 0 {
        fslen_ext = 0;
        fslen = fslen_ext;
        rcmr_period = 0;
        tcmr_period = rcmr_period;
        fs_osync = SSC_FSOS_NONE;
    }

    rcmr |= SSC_BF(RCMR_START, fs_edge);
    tcmr |= SSC_BF(TCMR_START, fs_edge);

    if atmel_ssc_cbs(ssc_p) != 0 {
        /*
         * SSC provides BCLK
         *
         * The SSC transmit and receive clocks are generated from the
         * MCK divider, and the BCLK signal is output
         * on the SSC TK line.
         */
        rcmr |= SSC_BF(RCMR_CKS, SSC_CKS_DIV) | SSC_BF(RCMR_CKO, SSC_CKO_NONE);

        tcmr |= SSC_BF(TCMR_CKS, SSC_CKS_DIV) | SSC_BF(TCMR_CKO, SSC_CKO_CONTINUOUS);
    } else {
        rcmr |= SSC_BF(
            RCMR_CKS,
            if (*ssc).clk_from_rk_pin {
                SSC_CKS_PIN
            } else {
                SSC_CKS_CLOCK
            },
        ) | SSC_BF(RCMR_CKO, SSC_CKO_NONE);

        tcmr |= SSC_BF(
            TCMR_CKS,
            if (*ssc).clk_from_rk_pin {
                SSC_CKS_CLOCK
            } else {
                SSC_CKS_PIN
            },
        ) | SSC_BF(TCMR_CKO, SSC_CKO_NONE);
    }

    rcmr |= SSC_BF(RCMR_PERIOD, rcmr_period as c_int) | SSC_BF(RCMR_CKI, SSC_CKI_RISING);

    tcmr |= SSC_BF(TCMR_PERIOD, tcmr_period as c_int) | SSC_BF(TCMR_CKI, SSC_CKI_FALLING);

    rfmr = SSC_BF(RFMR_FSLEN_EXT, fslen_ext)
        | SSC_BF(RFMR_FSEDGE, SSC_FSEDGE_POSITIVE)
        | SSC_BF(RFMR_FSOS, fs_osync)
        | SSC_BF(RFMR_FSLEN, fslen)
        | SSC_BF(RFMR_DATNB, channels - 1)
        | SSC_BIT(RFMR_MSBF)
        | SSC_BF(RFMR_LOOP, 0)
        | SSC_BF(RFMR_DATLEN, bits - 1);

    tfmr = SSC_BF(TFMR_FSLEN_EXT, fslen_ext)
        | SSC_BF(TFMR_FSEDGE, SSC_FSEDGE_POSITIVE)
        | SSC_BF(TFMR_FSDEN, 0)
        | SSC_BF(TFMR_FSOS, fs_osync)
        | SSC_BF(TFMR_FSLEN, fslen)
        | SSC_BF(TFMR_DATNB, channels - 1)
        | SSC_BIT(TFMR_MSBF)
        | SSC_BF(TFMR_DATDEF, 0)
        | SSC_BF(TFMR_DATLEN, bits - 1);

    if fslen_ext != 0 && !(*(*ssc).pdata).has_fslen_ext {
        dev_err(
            (*dai).dev,
            c_str!("sample size %d is too large for SSC device\n"),
            bits,
        );
        return -EINVAL;
    }

    pr_debug(
        c_str!("atmel_ssc_hw_params: RCMR=%08x RFMR=%08x TCMR=%08x TFMR=%08x\n"),
        rcmr,
        rfmr,
        tcmr,
        tfmr,
    );

    if (*ssc_p).initialized == 0 {
        if !(*(*(*ssc_p).ssc).pdata).use_dma {
            ssc_writel((*(*ssc_p).ssc).regs, PDC_RPR, 0);
            ssc_writel((*(*ssc_p).ssc).regs, PDC_RCR, 0);
            ssc_writel((*(*ssc_p).ssc).regs, PDC_RNPR, 0);
            ssc_writel((*(*ssc_p).ssc).regs, PDC_RNCR, 0);

            ssc_writel((*(*ssc_p).ssc).regs, PDC_TPR, 0);
            ssc_writel((*(*ssc_p).ssc).regs, PDC_TCR, 0);
            ssc_writel((*(*ssc_p).ssc).regs, PDC_TNPR, 0);
            ssc_writel((*(*ssc_p).ssc).regs, PDC_TNCR, 0);
        }

        ret = request_irq(
            (*(*ssc_p).ssc).irq,
            atmel_ssc_interrupt,
            0,
            (*ssc_p).name,
            ssc_p as *mut c_void,
        );
        if ret < 0 {
            printk(c_str!("atmel_ssc_dai: request_irq failure\n"));
            pr_debug(c_str!("Atmel_ssc_dai: Stopping clock\n"));
            clk_disable((*(*ssc_p).ssc).clk);
            return ret;
        }

        (*ssc_p).initialized = 1;
    }

    /* set SSC clock mode register */
    ssc_writel((*(*ssc_p).ssc).regs, CMR, cmr_div);

    /* set receive clock mode and format */
    ssc_writel((*(*ssc_p).ssc).regs, RCMR, rcmr);
    ssc_writel((*(*ssc_p).ssc).regs, RFMR, rfmr);

    /* set transmit clock mode and format */
    ssc_writel((*(*ssc_p).ssc).regs, TCMR, tcmr);
    ssc_writel((*(*ssc_p).ssc).regs, TFMR, tfmr);

    pr_debug(c_str!("atmel_ssc_dai,hw_params: SSC initialized\n"));
    0
}

unsafe extern "C" fn atmel_ssc_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pdev = to_platform_device((*dai).dev);
    let ssc_p = core::ptr::addr_of_mut!(ssc_info[(*pdev).id as usize]);
    let dma_params: *mut atmel_pcm_dma_params;
    let dir: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        dir = 0;
    } else {
        dir = 1;
    }

    dma_params = (*ssc_p).dma_params[dir as usize];

    ssc_writel((*(*ssc_p).ssc).regs, CR, (*(*dma_params).mask).ssc_disable);
    ssc_writel((*(*ssc_p).ssc).regs, IDR, (*(*dma_params).mask).ssc_error);

    pr_debug(
        c_str!("%s enabled SSC_SR=0x%08x\n"),
        if dir != 0 {
            c_str!("receive")
        } else {
            c_str!("transmit")
        },
        ssc_readl((*(*ssc_p).ssc).regs, SR),
    );
    0
}

unsafe extern "C" fn atmel_ssc_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let pdev = to_platform_device((*dai).dev);
    let ssc_p = core::ptr::addr_of_mut!(ssc_info[(*pdev).id as usize]);
    let dma_params: *mut atmel_pcm_dma_params;
    let dir: c_int;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        dir = 0;
    } else {
        dir = 1;
    }

    dma_params = (*ssc_p).dma_params[dir as usize];

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ssc_writel((*(*ssc_p).ssc).regs, CR, (*(*dma_params).mask).ssc_enable);
        }
        _ => {
            ssc_writel((*(*ssc_p).ssc).regs, CR, (*(*dma_params).mask).ssc_disable);
        }
    }

    0
}

unsafe extern "C" fn atmel_ssc_suspend(component: *mut snd_soc_component) -> c_int {
    let ssc_p: *mut atmel_ssc_info;
    let pdev = to_platform_device((*component).dev);

    if snd_soc_component_active(component) == 0 {
        return 0;
    }

    ssc_p = core::ptr::addr_of_mut!(ssc_info[(*pdev).id as usize]);

    /* Save the status register before disabling transmit and receive */
    (*ssc_p).ssc_state.ssc_sr = ssc_readl((*(*ssc_p).ssc).regs, SR);
    ssc_writel(
        (*(*ssc_p).ssc).regs,
        CR,
        SSC_BIT(CR_TXDIS) | SSC_BIT(CR_RXDIS),
    );

    /* Save the current interrupt mask, then disable unmasked interrupts */
    (*ssc_p).ssc_state.ssc_imr = ssc_readl((*(*ssc_p).ssc).regs, IMR);
    ssc_writel((*(*ssc_p).ssc).regs, IDR, (*ssc_p).ssc_state.ssc_imr);

    (*ssc_p).ssc_state.ssc_cmr = ssc_readl((*(*ssc_p).ssc).regs, CMR);
    (*ssc_p).ssc_state.ssc_rcmr = ssc_readl((*(*ssc_p).ssc).regs, RCMR);
    (*ssc_p).ssc_state.ssc_rfmr = ssc_readl((*(*ssc_p).ssc).regs, RFMR);
    (*ssc_p).ssc_state.ssc_tcmr = ssc_readl((*(*ssc_p).ssc).regs, TCMR);
    (*ssc_p).ssc_state.ssc_tfmr = ssc_readl((*(*ssc_p).ssc).regs, TFMR);

    0
}

unsafe extern "C" fn atmel_ssc_resume(component: *mut snd_soc_component) -> c_int {
    let ssc_p: *mut atmel_ssc_info;
    let pdev = to_platform_device((*component).dev);
    let mut cr: u32;

    if snd_soc_component_active(component) == 0 {
        return 0;
    }

    ssc_p = core::ptr::addr_of_mut!(ssc_info[(*pdev).id as usize]);

    /* restore SSC register settings */
    ssc_writel((*(*ssc_p).ssc).regs, TFMR, (*ssc_p).ssc_state.ssc_tfmr);
    ssc_writel((*(*ssc_p).ssc).regs, TCMR, (*ssc_p).ssc_state.ssc_tcmr);
    ssc_writel((*(*ssc_p).ssc).regs, RFMR, (*ssc_p).ssc_state.ssc_rfmr);
    ssc_writel((*(*ssc_p).ssc).regs, RCMR, (*ssc_p).ssc_state.ssc_rcmr);
    ssc_writel((*(*ssc_p).ssc).regs, CMR, (*ssc_p).ssc_state.ssc_cmr);

    /* re-enable interrupts */
    ssc_writel((*(*ssc_p).ssc).regs, IER, (*ssc_p).ssc_state.ssc_imr);

    /* Re-enable receive and transmit as appropriate */
    cr = 0;
    cr |= if ((*ssc_p).ssc_state.ssc_sr & SSC_BIT(SR_RXEN)) != 0 {
        SSC_BIT(CR_RXEN)
    } else {
        0
    };
    cr |= if ((*ssc_p).ssc_state.ssc_sr & SSC_BIT(SR_TXEN)) != 0 {
        SSC_BIT(CR_TXEN)
    } else {
        0
    };
    ssc_writel((*(*ssc_p).ssc).regs, CR, cr);

    0
}

/* S24_LE is not supported if more than 2 channels (of TDM slots) are used. */
const ATMEL_SSC_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

static atmel_ssc_selectable_formats: u64 = SND_SOC_POSSIBLE_DAIFMT_I2S
    | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
    | SND_SOC_POSSIBLE_DAIFMT_DSP_A;

static atmel_ssc_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(atmel_ssc_startup),
    shutdown: Some(atmel_ssc_shutdown),
    prepare: Some(atmel_ssc_prepare),
    trigger: Some(atmel_ssc_trigger),
    hw_params: Some(atmel_ssc_hw_params),
    set_fmt: Some(atmel_ssc_set_dai_fmt),
    set_clkdiv: Some(atmel_ssc_set_dai_clkdiv),
    auto_selectable_formats: &atmel_ssc_selectable_formats,
    num_auto_selectable_formats: 1,
};

static mut atmel_ssc_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Playback"),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 8000,
        rate_max: 384000,
        formats: ATMEL_SSC_FORMATS,
    },
    capture: snd_soc_pcm_stream {
        stream_name: c_str!("Capture"),
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 8000,
        rate_max: 384000,
        formats: ATMEL_SSC_FORMATS,
    },
    ops: &atmel_ssc_dai_ops,
};

static atmel_ssc_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c_str!("atmel-ssc"),
    suspend: Some(atmel_ssc_suspend),
    resume: Some(atmel_ssc_resume),
    legacy_dai_naming: 1,
};

unsafe fn asoc_ssc_init(dev: *mut device) -> c_int {
    let ssc = dev_get_drvdata(dev) as *mut ssc_device;
    let mut ret: c_int;

    ret = devm_snd_soc_register_component(
        dev,
        &atmel_ssc_component,
        core::ptr::addr_of_mut!(atmel_ssc_dai),
        1,
    );
    if ret != 0 {
        dev_err(dev, c_str!("Could not register DAI: %d\n"), ret);
        return ret;
    }

    if (*(*ssc).pdata).use_dma {
        ret = atmel_pcm_dma_platform_register(dev);
    } else {
        ret = atmel_pcm_pdc_platform_register(dev);
    }

    if ret != 0 {
        dev_err(dev, c_str!("Could not register PCM: %d\n"), ret);
        return ret;
    }

    0
}

/**
 * atmel_ssc_set_audio - Allocate the specified SSC for audio use.
 * @ssc_id: SSD ID in [0, NUM_SSC_DEVICES[
 */
#[no_mangle]
pub unsafe extern "C" fn atmel_ssc_set_audio(ssc_id: c_int) -> c_int {
    let ssc: *mut ssc_device;

    /* If we can grab the SSC briefly to parent the DAI device off it */
    ssc = ssc_request(ssc_id);
    if IS_ERR(ssc as *const c_void) {
        pr_err(
            c_str!("Unable to parent ASoC SSC DAI on SSC: %ld\n"),
            PTR_ERR(ssc as *const c_void),
        );
        return PTR_ERR(ssc as *const c_void) as c_int;
    } else {
        ssc_info[ssc_id as usize].ssc = ssc;
    }

    asoc_ssc_init(&mut (*(*ssc).pdev).dev)
}
/* EXPORT_SYMBOL_GPL(atmel_ssc_set_audio); */

#[no_mangle]
pub unsafe extern "C" fn atmel_ssc_put_audio(ssc_id: c_int) {
    let ssc = ssc_info[ssc_id as usize].ssc;

    ssc_free(ssc);
}
/* EXPORT_SYMBOL_GPL(atmel_ssc_put_audio); */

/* Module information */
/* MODULE_AUTHOR("Sedji Gaouaou, sedji.gaouaou@atmel.com, www.atmel.com"); */
/* MODULE_DESCRIPTION("ATMEL SSC ASoC Interface"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
