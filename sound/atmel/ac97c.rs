// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for Atmel AC97C
 *
 * Copyright (C) 2005-2009 Atmel Corporation
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_ushort, c_void};
use core::mem::zeroed;
use core::ptr;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
    pub period_size: snd_pcm_uframes_t,
    pub periods: c_int,
    pub buffer_size: snd_pcm_uframes_t,
    pub channels: c_uint,
    pub format: u64,
    pub rate: c_uint,
    pub dma_addr: c_ulong,
}
#[repr(C)]
pub struct snd_card {
    pub private_data: *mut c_void,
    pub driver: [c_char; 80],
    pub shortname: [c_char; 80],
    pub longname: [c_char; 80],
}
#[repr(C)]
pub struct snd_pcm {
    pub private_data: *mut c_void,
    pub info_flags: c_uint,
    pub name: [c_char; 80],
}
#[repr(C)]
pub struct snd_ac97 {
    pub private_data: *mut c_void,
}
#[repr(C)]
pub struct snd_ac97_bus {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

pub type snd_pcm_uframes_t = c_ulong;
pub type irqreturn_t = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: c_ulong,
    pub period_bytes_min: c_ulong,
    pub period_bytes_max: c_ulong,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
}

#[repr(C)]
pub struct snd_pcm_ops {
    pub open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct ac97_pcm_region {
    pub slots: c_uint,
}

#[repr(C)]
pub union ac97_pcm_r {
    pub region: [ac97_pcm_region; 1],
}

#[repr(C)]
pub struct ac97_pcm {
    pub stream: c_uint,
    pub exclusive: c_uint,
    pub r: ac97_pcm_r,
}

#[repr(C)]
pub struct snd_ac97_template {
    pub private_data: *mut c_void,
}

#[repr(C)]
pub struct snd_ac97_bus_ops {
    pub write: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort, c_ushort)>,
    pub read: Option<unsafe extern "C" fn(*mut snd_ac97, c_ushort) -> c_ushort>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_driver,
}

#[repr(C)]
pub struct atmel_ac97c {
    pub pclk: *mut clk,
    pub pdev: *mut platform_device,
    pub playback_substream: *mut snd_pcm_substream,
    pub capture_substream: *mut snd_pcm_substream,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub ac97: *mut snd_ac97,
    pub ac97_bus: *mut snd_ac97_bus,
    pub cur_format: u64,
    pub cur_rate: c_uint,
    pub playback_period: c_int,
    pub capture_period: c_int,
    /* Serialize access to opened variable */
    pub lock: spinlock_t,
    pub regs: *mut c_void,
    pub irq: c_int,
    pub opened: c_int,
    pub reset_pin: *mut gpio_desc,
}

/* Serialize access to opened variable */
static mut opened_mutex: mutex = mutex { _private: [] };

const EINVAL: c_int = 22;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;

extern "C" {
    static THIS_MODULE: *mut c_void;
    static SNDRV_DEFAULT_STR1: *const c_char;
    static atmel_ac97c_pm: dev_pm_ops;

    fn __raw_writel(val: c_ulong, addr: *mut c_void);
    fn __raw_readl(addr: *mut c_void) -> c_ulong;
    fn writel(val: c_ulong, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> c_ulong;
    fn udelay(usecs: c_ulong);

    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn spin_lock_init(lock: *mut spinlock_t);

    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut atmel_ac97c;
    fn pcm_format_to_bits(format: u64) -> u64;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> u64;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_ulong) -> snd_pcm_uframes_t;
    fn snd_ac97_set_rate(ac97: *mut snd_ac97, reg: c_ushort, rate: c_uint) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_ac97_pcm_assign(
        bus: *mut snd_ac97_bus,
        pcms_count: c_uint,
        pcms: *const ac97_pcm,
    ) -> c_int;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        size: c_ulong,
        max: c_ulong,
    );
    fn strscpy(dest: *mut c_char, src: *const c_char) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn snd_ac97_mixer(
        bus: *mut snd_ac97_bus,
        template: *mut snd_ac97_template,
        rac97: *mut *mut snd_ac97,
    ) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn gpiod_set_value(desc: *mut gpio_desc, value: c_int);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn snd_devm_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *const c_char,
        module: *mut c_void,
        extra_size: usize,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_ulong,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn devm_gpiod_get_index(
        dev: *mut device,
        con_id: *const c_char,
        index: c_uint,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn snd_ac97_bus(
        card: *mut snd_card,
        num: c_int,
        ops: *const snd_ac97_bus_ops,
        private_data: *mut c_void,
        rbus: *mut *mut snd_ac97_bus,
    ) -> c_int;
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
    fn module_platform_driver(driver: *mut platform_driver);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

extern "C" {
    static AC97C_OCA: usize;
    static AC97C_ICA: usize;
    static AC97C_CAMR: usize;
    static AC97C_IMR: usize;
    static AC97C_IER: usize;
    static AC97C_MR: usize;
    static AC97C_SR: usize;
    static AC97C_CASR: usize;
    static AC97C_COSR: usize;
    static AC97C_COMR: usize;
    static AC97C_COTHR: usize;
    static AC97C_CORHR: usize;
    static ATMEL_PDC_TPR: usize;
    static ATMEL_PDC_TCR: usize;
    static ATMEL_PDC_TNPR: usize;
    static ATMEL_PDC_TNCR: usize;
    static ATMEL_PDC_RPR: usize;
    static ATMEL_PDC_RCR: usize;
    static ATMEL_PDC_RNPR: usize;
    static ATMEL_PDC_RNCR: usize;
    static ATMEL_PDC_PTCR: usize;
    static ATMEL_PDC_PTSR: usize;
}

extern "C" {
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint;
    static SNDRV_PCM_INFO_JOINT_DUPLEX: c_uint;
    static SNDRV_PCM_INFO_RESUME: c_uint;
    static SNDRV_PCM_INFO_PAUSE: c_uint;
    static SNDRV_PCM_FMTBIT_S16_BE: u64;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: u64;
    static SNDRV_PCM_FORMAT_S16_BE: u64;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_PCM_STREAM_CAPTURE: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static SNDRV_DEFAULT_IDX1: c_int;
    static GPIOD_OUT_HIGH: c_int;
}

extern "C" {
    static PCM_LEFT: c_uint;
    static PCM_RIGHT: c_uint;
    static A: c_uint;
    static AC97C_CMR_DMAEN: c_ulong;
    static AC97C_CMR_SIZE_16: c_ulong;
    static AC97C_CMR_CEM_LITTLE: c_ulong;
    static AC97C_CMR_CENA: c_ulong;
    static AC97C_CSR_UNRUN: c_ulong;
    static AC97C_CSR_OVRUN: c_ulong;
    static AC97C_CSR_ENDTX: c_ulong;
    static AC97C_CSR_ENDRX: c_ulong;
    static AC97C_CSR_RXRDY: c_ulong;
    static AC97C_CSR_TXEMPTY: c_ulong;
    static AC97C_CSR_TXRDY: c_ulong;
    static AC97C_SR_CAEVT: c_ulong;
    static AC97C_SR_COEVT: c_ulong;
    static AC97C_MR_VRA: c_ulong;
    static AC97C_MR_ENA: c_ulong;
    static AC97C_MR_WRST: c_ulong;
    static ATMEL_PDC_TXTEN: c_ulong;
    static ATMEL_PDC_TXTDIS: c_ulong;
    static ATMEL_PDC_RXTEN: c_ulong;
    static ATMEL_PDC_RXTDIS: c_ulong;
    static AC97_PCM_FRONT_DAC_RATE: c_ushort;
    static AC97_PCM_LR_ADC_RATE: c_ushort;
    static AC97_SLOT_PCM_LEFT: c_uint;
    static AC97_SLOT_PCM_RIGHT: c_uint;
    static AC97_SLOT_MIC: c_uint;
}

unsafe fn get_chip_from_card(card: *mut snd_card) -> *mut atmel_ac97c {
    (*card).private_data as *mut atmel_ac97c
}

unsafe fn get_chip(ac97: *mut snd_ac97) -> *mut atmel_ac97c {
    (*ac97).private_data as *mut atmel_ac97c
}

unsafe fn ac97c_writel(chip: *mut atmel_ac97c, reg: usize, val: c_ulong) {
    __raw_writel(val, ((*chip).regs as *mut u8).add(reg) as *mut c_void);
}

unsafe fn ac97c_readl(chip: *mut atmel_ac97c, reg: usize) -> c_ulong {
    __raw_readl(((*chip).regs as *mut u8).add(reg) as *mut c_void)
}

unsafe fn pdc_addr(chip: *mut atmel_ac97c, reg: usize) -> *mut c_void {
    ((*chip).regs as *mut u8).add(reg) as *mut c_void
}

unsafe fn AC97C_CH_MASK(ch: c_uint) -> c_ulong {
    unimplemented!("external macro AC97C_CH_MASK");
}

unsafe fn AC97C_CH_ASSIGN(ch: c_uint, channel: c_uint) -> c_ulong {
    unimplemented!("external macro AC97C_CH_ASSIGN");
}

unsafe fn atmel_ac97c_hw_init() -> snd_pcm_hardware {
    snd_pcm_hardware {
        info: SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_BLOCK_TRANSFER
            | SNDRV_PCM_INFO_JOINT_DUPLEX
            | SNDRV_PCM_INFO_RESUME
            | SNDRV_PCM_INFO_PAUSE,
        formats: SNDRV_PCM_FMTBIT_S16_BE | SNDRV_PCM_FMTBIT_S16_LE,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 4000,
        rate_max: 48000,
        channels_min: 1,
        channels_max: 2,
        buffer_bytes_max: 2 * 2 * 64 * 2048,
        period_bytes_min: 4096,
        period_bytes_max: 4096,
        periods_min: 6,
        periods_max: 64,
    }
}

unsafe extern "C" fn atmel_ac97c_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;

    mutex_lock(ptr::addr_of_mut!(opened_mutex));
    (*chip).opened += 1;
    (*runtime).hw = atmel_ac97c_hw_init();
    if (*chip).cur_rate != 0 {
        (*runtime).hw.rate_min = (*chip).cur_rate;
        (*runtime).hw.rate_max = (*chip).cur_rate;
    }
    if (*chip).cur_format != 0 {
        (*runtime).hw.formats = pcm_format_to_bits((*chip).cur_format);
    }
    (*chip).playback_substream = substream;
    mutex_unlock(ptr::addr_of_mut!(opened_mutex));
    0
}

unsafe extern "C" fn atmel_ac97c_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;

    mutex_lock(ptr::addr_of_mut!(opened_mutex));
    (*chip).opened += 1;
    (*runtime).hw = atmel_ac97c_hw_init();
    if (*chip).cur_rate != 0 {
        (*runtime).hw.rate_min = (*chip).cur_rate;
        (*runtime).hw.rate_max = (*chip).cur_rate;
    }
    if (*chip).cur_format != 0 {
        (*runtime).hw.formats = pcm_format_to_bits((*chip).cur_format);
    }
    (*chip).capture_substream = substream;
    mutex_unlock(ptr::addr_of_mut!(opened_mutex));
    0
}

unsafe extern "C" fn atmel_ac97c_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);

    mutex_lock(ptr::addr_of_mut!(opened_mutex));
    (*chip).opened -= 1;
    if (*chip).opened == 0 {
        (*chip).cur_rate = 0;
        (*chip).cur_format = 0;
    }
    (*chip).playback_substream = ptr::null_mut();
    mutex_unlock(ptr::addr_of_mut!(opened_mutex));

    0
}

unsafe extern "C" fn atmel_ac97c_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);

    mutex_lock(ptr::addr_of_mut!(opened_mutex));
    (*chip).opened -= 1;
    if (*chip).opened == 0 {
        (*chip).cur_rate = 0;
        (*chip).cur_format = 0;
    }
    (*chip).capture_substream = ptr::null_mut();
    mutex_unlock(ptr::addr_of_mut!(opened_mutex));

    0
}

unsafe extern "C" fn atmel_ac97c_playback_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);

    /* Set restrictions to params. */
    mutex_lock(ptr::addr_of_mut!(opened_mutex));
    (*chip).cur_rate = params_rate(hw_params);
    (*chip).cur_format = params_format(hw_params);
    mutex_unlock(ptr::addr_of_mut!(opened_mutex));

    0
}

unsafe extern "C" fn atmel_ac97c_capture_hw_params(
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);

    /* Set restrictions to params. */
    mutex_lock(ptr::addr_of_mut!(opened_mutex));
    (*chip).cur_rate = params_rate(hw_params);
    (*chip).cur_format = params_format(hw_params);
    mutex_unlock(ptr::addr_of_mut!(opened_mutex));

    0
}

unsafe extern "C" fn atmel_ac97c_playback_prepare(
    substream: *mut snd_pcm_substream,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let block_size = frames_to_bytes(runtime, (*runtime).period_size);
    let mut word = ac97c_readl(chip, AC97C_OCA);
    let retval: c_int;

    (*chip).playback_period = 0;
    word &= !(AC97C_CH_MASK(PCM_LEFT) | AC97C_CH_MASK(PCM_RIGHT));

    /* assign channels to AC97C channel A */
    match (*runtime).channels {
        1 => word |= AC97C_CH_ASSIGN(PCM_LEFT, A),
        2 => word |= AC97C_CH_ASSIGN(PCM_LEFT, A) | AC97C_CH_ASSIGN(PCM_RIGHT, A),
        _ => {
            /* TODO: support more than two channels */
            return -EINVAL;
        }
    }
    ac97c_writel(chip, AC97C_OCA, word);

    /* configure sample format and size */
    word = ac97c_readl(chip, AC97C_CAMR);
    if (*chip).opened <= 1 {
        word = AC97C_CMR_DMAEN | AC97C_CMR_SIZE_16;
    } else {
        word |= AC97C_CMR_DMAEN | AC97C_CMR_SIZE_16;
    }

    if (*runtime).format == SNDRV_PCM_FORMAT_S16_LE {
    } else if (*runtime).format == SNDRV_PCM_FORMAT_S16_BE {
        word &= !AC97C_CMR_CEM_LITTLE;
    } else {
        word = ac97c_readl(chip, AC97C_OCA);
        word &= !(AC97C_CH_MASK(PCM_LEFT) | AC97C_CH_MASK(PCM_RIGHT));
        ac97c_writel(chip, AC97C_OCA, word);
        return -EINVAL;
    }

    /* Enable underrun interrupt on channel A */
    word |= AC97C_CSR_UNRUN;

    ac97c_writel(chip, AC97C_CAMR, word);

    /* Enable channel A event interrupt */
    word = ac97c_readl(chip, AC97C_IMR);
    word |= AC97C_SR_CAEVT;
    ac97c_writel(chip, AC97C_IER, word);

    /* set variable rate if needed */
    word = ac97c_readl(chip, AC97C_MR);
    if (*runtime).rate != 48000 {
        word |= AC97C_MR_VRA;
    } else {
        word &= !AC97C_MR_VRA;
    }
    ac97c_writel(chip, AC97C_MR, word);

    retval = snd_ac97_set_rate((*chip).ac97, AC97_PCM_FRONT_DAC_RATE, (*runtime).rate);
    if retval != 0 {
        dev_dbg(
            ptr::addr_of_mut!((*(*chip).pdev).dev),
            b"could not set rate %d Hz\n\0".as_ptr() as *const c_char,
            (*runtime).rate,
        );
    }

    /* Initialize and start the PDC */
    writel((*runtime).dma_addr, pdc_addr(chip, ATMEL_PDC_TPR));
    writel((block_size / 2) as c_ulong, pdc_addr(chip, ATMEL_PDC_TCR));
    writel(
        (*runtime).dma_addr + block_size as c_ulong,
        pdc_addr(chip, ATMEL_PDC_TNPR),
    );
    writel((block_size / 2) as c_ulong, pdc_addr(chip, ATMEL_PDC_TNCR));

    retval
}

unsafe extern "C" fn atmel_ac97c_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let block_size = frames_to_bytes(runtime, (*runtime).period_size);
    let mut word = ac97c_readl(chip, AC97C_ICA);
    let retval: c_int;

    (*chip).capture_period = 0;
    word &= !(AC97C_CH_MASK(PCM_LEFT) | AC97C_CH_MASK(PCM_RIGHT));

    /* assign channels to AC97C channel A */
    match (*runtime).channels {
        1 => word |= AC97C_CH_ASSIGN(PCM_LEFT, A),
        2 => word |= AC97C_CH_ASSIGN(PCM_LEFT, A) | AC97C_CH_ASSIGN(PCM_RIGHT, A),
        _ => {
            /* TODO: support more than two channels */
            return -EINVAL;
        }
    }
    ac97c_writel(chip, AC97C_ICA, word);

    /* configure sample format and size */
    word = ac97c_readl(chip, AC97C_CAMR);
    if (*chip).opened <= 1 {
        word = AC97C_CMR_DMAEN | AC97C_CMR_SIZE_16;
    } else {
        word |= AC97C_CMR_DMAEN | AC97C_CMR_SIZE_16;
    }

    if (*runtime).format == SNDRV_PCM_FORMAT_S16_LE {
    } else if (*runtime).format == SNDRV_PCM_FORMAT_S16_BE {
        word &= !AC97C_CMR_CEM_LITTLE;
    } else {
        word = ac97c_readl(chip, AC97C_ICA);
        word &= !(AC97C_CH_MASK(PCM_LEFT) | AC97C_CH_MASK(PCM_RIGHT));
        ac97c_writel(chip, AC97C_ICA, word);
        return -EINVAL;
    }

    /* Enable overrun interrupt on channel A */
    word |= AC97C_CSR_OVRUN;

    ac97c_writel(chip, AC97C_CAMR, word);

    /* Enable channel A event interrupt */
    word = ac97c_readl(chip, AC97C_IMR);
    word |= AC97C_SR_CAEVT;
    ac97c_writel(chip, AC97C_IER, word);

    /* set variable rate if needed */
    word = ac97c_readl(chip, AC97C_MR);
    if (*runtime).rate != 48000 {
        word |= AC97C_MR_VRA;
    } else {
        word &= !AC97C_MR_VRA;
    }
    ac97c_writel(chip, AC97C_MR, word);

    retval = snd_ac97_set_rate((*chip).ac97, AC97_PCM_LR_ADC_RATE, (*runtime).rate);
    if retval != 0 {
        dev_dbg(
            ptr::addr_of_mut!((*(*chip).pdev).dev),
            b"could not set rate %d Hz\n\0".as_ptr() as *const c_char,
            (*runtime).rate,
        );
    }

    /* Initialize and start the PDC */
    writel((*runtime).dma_addr, pdc_addr(chip, ATMEL_PDC_RPR));
    writel((block_size / 2) as c_ulong, pdc_addr(chip, ATMEL_PDC_RCR));
    writel(
        (*runtime).dma_addr + block_size as c_ulong,
        pdc_addr(chip, ATMEL_PDC_RNPR),
    );
    writel((block_size / 2) as c_ulong, pdc_addr(chip, ATMEL_PDC_RNCR));

    retval
}

unsafe extern "C" fn atmel_ac97c_playback_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut ptcr: c_ulong = 0;
    let mut camr = ac97c_readl(chip, AC97C_CAMR);

    if cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
        || cmd == SNDRV_PCM_TRIGGER_RESUME
        || cmd == SNDRV_PCM_TRIGGER_START
    {
        ptcr = ATMEL_PDC_TXTEN;
        camr |= AC97C_CMR_CENA | AC97C_CSR_ENDTX;
    } else if cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_STOP
    {
        ptcr |= ATMEL_PDC_TXTDIS;
        if (*chip).opened <= 1 {
            camr &= !AC97C_CMR_CENA;
        }
    } else {
        return -EINVAL;
    }

    ac97c_writel(chip, AC97C_CAMR, camr);
    writel(ptcr, pdc_addr(chip, ATMEL_PDC_PTCR));
    0
}

unsafe extern "C" fn atmel_ac97c_capture_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let chip = snd_pcm_substream_chip(substream);
    let mut camr = ac97c_readl(chip, AC97C_CAMR);
    let mut ptcr = readl(pdc_addr(chip, ATMEL_PDC_PTSR));

    if cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
        || cmd == SNDRV_PCM_TRIGGER_RESUME
        || cmd == SNDRV_PCM_TRIGGER_START
    {
        ptcr = ATMEL_PDC_RXTEN;
        camr |= AC97C_CMR_CENA | AC97C_CSR_ENDRX;
    } else if cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_STOP
    {
        ptcr |= ATMEL_PDC_RXTDIS;
        if (*chip).opened <= 1 {
            camr &= !AC97C_CMR_CENA;
        }
    } else {
        return -EINVAL;
    }

    ac97c_writel(chip, AC97C_CAMR, camr);
    writel(ptcr, pdc_addr(chip, ATMEL_PDC_PTCR));
    0
}

unsafe extern "C" fn atmel_ac97c_playback_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut bytes = readl(pdc_addr(chip, ATMEL_PDC_TPR));

    bytes -= (*runtime).dma_addr;

    let mut frames = bytes_to_frames(runtime, bytes);
    if frames >= (*runtime).buffer_size {
        frames -= (*runtime).buffer_size;
    }
    frames
}

unsafe extern "C" fn atmel_ac97c_capture_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let chip = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut bytes = readl(pdc_addr(chip, ATMEL_PDC_RPR));

    bytes -= (*runtime).dma_addr;

    let mut frames = bytes_to_frames(runtime, bytes);
    if frames >= (*runtime).buffer_size {
        frames -= (*runtime).buffer_size;
    }
    frames
}

static atmel_ac97_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(atmel_ac97c_playback_open),
    close: Some(atmel_ac97c_playback_close),
    hw_params: Some(atmel_ac97c_playback_hw_params),
    prepare: Some(atmel_ac97c_playback_prepare),
    trigger: Some(atmel_ac97c_playback_trigger),
    pointer: Some(atmel_ac97c_playback_pointer),
};

static atmel_ac97_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(atmel_ac97c_capture_open),
    close: Some(atmel_ac97c_capture_close),
    hw_params: Some(atmel_ac97c_capture_hw_params),
    prepare: Some(atmel_ac97c_capture_prepare),
    trigger: Some(atmel_ac97c_capture_trigger),
    pointer: Some(atmel_ac97c_capture_pointer),
};

unsafe extern "C" fn atmel_ac97c_interrupt(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    let chip = dev as *mut atmel_ac97c;
    let mut retval = IRQ_NONE;
    let sr = ac97c_readl(chip, AC97C_SR) as u32;
    let casr = ac97c_readl(chip, AC97C_CASR) as u32;
    let cosr = ac97c_readl(chip, AC97C_COSR) as u32;
    let camr = ac97c_readl(chip, AC97C_CAMR) as u32;

    if (sr as c_ulong & AC97C_SR_CAEVT) != 0 {
        let runtime: *mut snd_pcm_runtime;
        let mut offset: c_int;
        let mut next_period: c_int;
        let block_size: c_int;
        dev_dbg(
            ptr::addr_of_mut!((*(*chip).pdev).dev),
            b"channel A event%s%s%s%s%s%s\n\0".as_ptr() as *const c_char,
            if (casr as c_ulong & AC97C_CSR_OVRUN) != 0 { b" OVRUN\0".as_ptr() } else { b"\0".as_ptr() },
            if (casr as c_ulong & AC97C_CSR_RXRDY) != 0 { b" RXRDY\0".as_ptr() } else { b"\0".as_ptr() },
            if (casr as c_ulong & AC97C_CSR_UNRUN) != 0 { b" UNRUN\0".as_ptr() } else { b"\0".as_ptr() },
            if (casr as c_ulong & AC97C_CSR_TXEMPTY) != 0 { b" TXEMPTY\0".as_ptr() } else { b"\0".as_ptr() },
            if (casr as c_ulong & AC97C_CSR_TXRDY) != 0 { b" TXRDY\0".as_ptr() } else { b"\0".as_ptr() },
            if casr == 0 { b" NONE\0".as_ptr() } else { b"\0".as_ptr() },
        );
        if ((casr & camr) as c_ulong & AC97C_CSR_ENDTX) != 0 {
            runtime = (*(*chip).playback_substream).runtime;
            block_size = frames_to_bytes(runtime, (*runtime).period_size);
            (*chip).playback_period += 1;

            if (*chip).playback_period == (*runtime).periods {
                (*chip).playback_period = 0;
            }
            next_period = (*chip).playback_period + 1;
            if next_period == (*runtime).periods {
                next_period = 0;
            }

            offset = block_size * next_period;

            writel((*runtime).dma_addr + offset as c_ulong, pdc_addr(chip, ATMEL_PDC_TNPR));
            writel((block_size / 2) as c_ulong, pdc_addr(chip, ATMEL_PDC_TNCR));

            snd_pcm_period_elapsed((*chip).playback_substream);
        }
        if ((casr & camr) as c_ulong & AC97C_CSR_ENDRX) != 0 {
            runtime = (*(*chip).capture_substream).runtime;
            block_size = frames_to_bytes(runtime, (*runtime).period_size);
            (*chip).capture_period += 1;

            if (*chip).capture_period == (*runtime).periods {
                (*chip).capture_period = 0;
            }
            next_period = (*chip).capture_period + 1;
            if next_period == (*runtime).periods {
                next_period = 0;
            }

            offset = block_size * next_period;

            writel((*runtime).dma_addr + offset as c_ulong, pdc_addr(chip, ATMEL_PDC_RNPR));
            writel((block_size / 2) as c_ulong, pdc_addr(chip, ATMEL_PDC_RNCR));
            snd_pcm_period_elapsed((*chip).capture_substream);
        }
        retval = IRQ_HANDLED;
    }

    if (sr as c_ulong & AC97C_SR_COEVT) != 0 {
        dev_info(
            ptr::addr_of_mut!((*(*chip).pdev).dev),
            b"codec channel event%s%s%s%s%s\n\0".as_ptr() as *const c_char,
            if (cosr as c_ulong & AC97C_CSR_OVRUN) != 0 { b" OVRUN\0".as_ptr() } else { b"\0".as_ptr() },
            if (cosr as c_ulong & AC97C_CSR_RXRDY) != 0 { b" RXRDY\0".as_ptr() } else { b"\0".as_ptr() },
            if (cosr as c_ulong & AC97C_CSR_TXEMPTY) != 0 { b" TXEMPTY\0".as_ptr() } else { b"\0".as_ptr() },
            if (cosr as c_ulong & AC97C_CSR_TXRDY) != 0 { b" TXRDY\0".as_ptr() } else { b"\0".as_ptr() },
            if cosr == 0 { b" NONE\0".as_ptr() } else { b"\0".as_ptr() },
        );
        retval = IRQ_HANDLED;
    }

    if retval == IRQ_NONE {
        dev_err(
            ptr::addr_of_mut!((*(*chip).pdev).dev),
            b"spurious interrupt sr 0x%08x casr 0x%08x cosr 0x%08x\n\0".as_ptr()
                as *const c_char,
            sr,
            casr,
            cosr,
        );
    }

    retval
}

static at91_ac97_pcm_defs: [ac97_pcm; 3] = [
    /* Playback */
    ac97_pcm {
        stream: 0,
        exclusive: 1,
        r: ac97_pcm_r {
            region: [ac97_pcm_region {
                slots: unsafe { (1 << AC97_SLOT_PCM_LEFT) | (1 << AC97_SLOT_PCM_RIGHT) },
            }],
        },
    },
    /* PCM in */
    ac97_pcm {
        stream: 1,
        exclusive: 1,
        r: ac97_pcm_r {
            region: [ac97_pcm_region {
                slots: unsafe { (1 << AC97_SLOT_PCM_LEFT) | (1 << AC97_SLOT_PCM_RIGHT) },
            }],
        },
    },
    /* Mic in */
    ac97_pcm {
        stream: 1,
        exclusive: 1,
        r: ac97_pcm_r {
            region: [ac97_pcm_region {
                slots: unsafe { 1 << AC97_SLOT_MIC },
            }],
        },
    },
];

unsafe fn atmel_ac97c_pcm_new(chip: *mut atmel_ac97c) -> c_int {
    let mut pcm: *mut snd_pcm = ptr::null_mut();
    let hw = atmel_ac97c_hw_init();
    let mut retval: c_int;

    retval = snd_ac97_pcm_assign(
        (*chip).ac97_bus,
        at91_ac97_pcm_defs.len() as c_uint,
        at91_ac97_pcm_defs.as_ptr(),
    );
    if retval != 0 {
        return retval;
    }

    retval = snd_pcm_new((*chip).card, (*(*chip).card).shortname.as_ptr(), 0, 1, 1, &mut pcm);
    if retval != 0 {
        return retval;
    }

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &atmel_ac97_capture_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &atmel_ac97_playback_ops);

    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        ptr::addr_of_mut!((*(*chip).pdev).dev),
        (hw.periods_min as c_ulong) * hw.period_bytes_min,
        hw.buffer_bytes_max,
    );

    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = 0;
    strscpy((*pcm).name.as_mut_ptr(), (*(*chip).card).shortname.as_ptr());
    (*chip).pcm = pcm;

    0
}

unsafe fn atmel_ac97c_mixer_new(chip: *mut atmel_ac97c) -> c_int {
    let mut template: snd_ac97_template = zeroed();
    memset(
        &mut template as *mut snd_ac97_template as *mut c_void,
        0,
        core::mem::size_of::<snd_ac97_template>(),
    );
    template.private_data = chip as *mut c_void;
    snd_ac97_mixer((*chip).ac97_bus, &mut template, ptr::addr_of_mut!((*chip).ac97))
}

unsafe extern "C" fn atmel_ac97c_write(ac97: *mut snd_ac97, reg: c_ushort, val: c_ushort) {
    let chip = get_chip(ac97);
    let word = (((reg & 0x7f) as c_ulong) << 16) | val as c_ulong;
    let mut timeout: c_int = 40;

    loop {
        if (ac97c_readl(chip, AC97C_COSR) & AC97C_CSR_TXRDY) != 0 {
            ac97c_writel(chip, AC97C_COTHR, word);
            return;
        }
        udelay(1);
        timeout -= 1;
        if timeout == 0 {
            break;
        }
    }

    dev_dbg(ptr::addr_of_mut!((*(*chip).pdev).dev), b"codec write timeout\n\0".as_ptr() as *const c_char);
}

unsafe extern "C" fn atmel_ac97c_read(ac97: *mut snd_ac97, reg: c_ushort) -> c_ushort {
    let chip = get_chip(ac97);
    let word = (((0x80 | (reg & 0x7f)) as c_ulong) << 16) as c_ulong;
    let mut timeout: c_int;
    let mut write: c_int = 10;

    if (ac97c_readl(chip, AC97C_COSR) & AC97C_CSR_RXRDY) != 0 {
        ac97c_readl(chip, AC97C_CORHR);
    }

    loop {
        timeout = 40;
        loop {
            if (ac97c_readl(chip, AC97C_COSR) & AC97C_CSR_TXRDY) != 0 {
                ac97c_writel(chip, AC97C_COTHR, word);
                break;
            }
            udelay(10);
            timeout -= 1;
            if timeout == 0 {
                write -= 1;
                if write == 0 {
                    dev_dbg(ptr::addr_of_mut!((*(*chip).pdev).dev), b"codec read timeout\n\0".as_ptr() as *const c_char);
                    return 0xffff;
                }
                continue;
            }
        }

        loop {
            if (ac97c_readl(chip, AC97C_COSR) & AC97C_CSR_RXRDY) != 0 {
                let val = ac97c_readl(chip, AC97C_CORHR) as c_ushort;
                return val;
            }
            udelay(10);
            timeout -= 1;
            if timeout == 0 {
                break;
            }
        }

        write -= 1;
        if write == 0 {
            dev_dbg(ptr::addr_of_mut!((*(*chip).pdev).dev), b"codec read timeout\n\0".as_ptr() as *const c_char);
            return 0xffff;
        }
    }
}

unsafe fn atmel_ac97c_reset(chip: *mut atmel_ac97c) {
    ac97c_writel(chip, AC97C_MR, 0);
    ac97c_writel(chip, AC97C_MR, AC97C_MR_ENA);
    ac97c_writel(chip, AC97C_CAMR, 0);
    ac97c_writel(chip, AC97C_COMR, 0);

    if !IS_ERR((*chip).reset_pin as *const c_void) {
        gpiod_set_value((*chip).reset_pin, 0);
        /* AC97 v2.2 specifications says minimum 1 us. */
        udelay(2);
        gpiod_set_value((*chip).reset_pin, 1);
    } else {
        ac97c_writel(chip, AC97C_MR, AC97C_MR_WRST | AC97C_MR_ENA);
        udelay(2);
        ac97c_writel(chip, AC97C_MR, AC97C_MR_ENA);
    }
}

static atmel_ac97c_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: b"atmel,at91sam9263-ac97c\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, atmel_ac97c_dt_ids); */

unsafe extern "C" fn atmel_ac97c_probe(pdev: *mut platform_device) -> c_int {
    let dev = ptr::addr_of_mut!((*pdev).dev);
    let mut card: *mut snd_card = ptr::null_mut();
    let chip: *mut atmel_ac97c;
    let regs: *mut c_void;
    let pclk: *mut clk;
    static ops: snd_ac97_bus_ops = snd_ac97_bus_ops {
        write: Some(atmel_ac97c_write),
        read: Some(atmel_ac97c_read),
    };
    let mut retval: c_int;
    let irq: c_int;

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    pclk = devm_clk_get_enabled(ptr::addr_of_mut!((*pdev).dev), b"ac97_clk\0".as_ptr() as *const c_char);
    if IS_ERR(pclk as *const c_void) {
        dev_dbg(ptr::addr_of_mut!((*pdev).dev), b"no peripheral clock\n\0".as_ptr() as *const c_char);
        return PTR_ERR(pclk as *const c_void);
    }

    retval = snd_devm_card_new(
        ptr::addr_of_mut!((*pdev).dev),
        SNDRV_DEFAULT_IDX1,
        SNDRV_DEFAULT_STR1,
        THIS_MODULE,
        core::mem::size_of::<atmel_ac97c>(),
        &mut card,
    );
    if retval != 0 {
        dev_dbg(ptr::addr_of_mut!((*pdev).dev), b"could not create sound card device\n\0".as_ptr() as *const c_char);
        return retval;
    }

    chip = get_chip_from_card(card);

    retval = devm_request_irq(
        ptr::addr_of_mut!((*pdev).dev),
        irq,
        atmel_ac97c_interrupt,
        0,
        b"AC97C\0".as_ptr() as *const c_char,
        chip as *mut c_void,
    );
    if retval != 0 {
        return retval;
    }

    (*chip).irq = irq;

    spin_lock_init(ptr::addr_of_mut!((*chip).lock));

    strscpy((*card).driver.as_mut_ptr(), b"Atmel AC97C\0".as_ptr() as *const c_char);
    strscpy((*card).shortname.as_mut_ptr(), b"Atmel AC97C\0".as_ptr() as *const c_char);
    strscpy(
        (*card).longname.as_mut_ptr(),
        b"Atmel AC97 controller\0".as_ptr() as *const c_char,
    );

    (*chip).card = card;
    (*chip).pclk = pclk;
    (*chip).pdev = pdev;
    (*chip).regs = regs;

    (*chip).reset_pin = devm_gpiod_get_index(dev, b"ac97\0".as_ptr() as *const c_char, 2, GPIOD_OUT_HIGH);
    if IS_ERR((*chip).reset_pin as *const c_void) {
        dev_dbg(dev, b"reset pin not available\n\0".as_ptr() as *const c_char);
    }

    atmel_ac97c_reset(chip);

    /* Enable overrun interrupt from codec channel */
    ac97c_writel(chip, AC97C_COMR, AC97C_CSR_OVRUN);
    ac97c_writel(chip, AC97C_IER, ac97c_readl(chip, AC97C_IMR) | AC97C_SR_COEVT);

    retval = snd_ac97_bus(card, 0, &ops, chip as *mut c_void, ptr::addr_of_mut!((*chip).ac97_bus));
    if retval != 0 {
        dev_dbg(ptr::addr_of_mut!((*pdev).dev), b"could not register on ac97 bus\n\0".as_ptr() as *const c_char);
        return retval;
    }

    retval = atmel_ac97c_mixer_new(chip);
    if retval != 0 {
        dev_dbg(ptr::addr_of_mut!((*pdev).dev), b"could not register ac97 mixer\n\0".as_ptr() as *const c_char);
        return retval;
    }

    retval = atmel_ac97c_pcm_new(chip);
    if retval != 0 {
        dev_dbg(ptr::addr_of_mut!((*pdev).dev), b"could not register ac97 pcm device\n\0".as_ptr() as *const c_char);
        return retval;
    }

    retval = snd_card_register(card);
    if retval != 0 {
        dev_dbg(ptr::addr_of_mut!((*pdev).dev), b"could not register sound card\n\0".as_ptr() as *const c_char);
        return retval;
    }

    platform_set_drvdata(pdev, card as *mut c_void);

    dev_info(
        ptr::addr_of_mut!((*pdev).dev),
        b"Atmel AC97 controller at 0x%p, irq = %d\n\0".as_ptr() as *const c_char,
        (*chip).regs,
        irq,
    );

    0
}

unsafe extern "C" fn atmel_ac97c_suspend(pdev: *mut device) -> c_int {
    let card = dev_get_drvdata(pdev) as *mut snd_card;
    let chip = (*card).private_data as *mut atmel_ac97c;

    clk_disable_unprepare((*chip).pclk);
    0
}

unsafe extern "C" fn atmel_ac97c_resume(pdev: *mut device) -> c_int {
    let card = dev_get_drvdata(pdev) as *mut snd_card;
    let chip = (*card).private_data as *mut atmel_ac97c;
    let ret = clk_prepare_enable((*chip).pclk);

    ret
}

/* DEFINE_SIMPLE_DEV_PM_OPS(atmel_ac97c_pm, atmel_ac97c_suspend, atmel_ac97c_resume); */

unsafe extern "C" fn atmel_ac97c_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_card;
    let chip = get_chip_from_card(card);

    ac97c_writel(chip, AC97C_CAMR, 0);
    ac97c_writel(chip, AC97C_COMR, 0);
    ac97c_writel(chip, AC97C_MR, 0);
}

static mut atmel_ac97c_driver: platform_driver = platform_driver {
    probe: Some(atmel_ac97c_probe),
    remove: Some(atmel_ac97c_remove),
    driver: platform_driver_driver {
        name: b"atmel_ac97c\0".as_ptr() as *const c_char,
        pm: unsafe { pm_ptr(&atmel_ac97c_pm) },
        of_match_table: atmel_ac97c_dt_ids.as_ptr(),
    },
};

unsafe fn init_module_registration() {
    module_platform_driver(ptr::addr_of_mut!(atmel_ac97c_driver));
}

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("Driver for Atmel AC97 controller"); */
/* MODULE_AUTHOR("Hans-Christian Egtvedt <egtvedt@samfundet.no>"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
