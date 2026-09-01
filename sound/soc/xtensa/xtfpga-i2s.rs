// SPDX-License-Identifier: GPL-2.0-only
/*
 * Xtfpga I2S controller driver
 *
 * Copyright (c) 2014 Cadence Design Systems Inc.
 */

// Dependencies on Linux kernel modules: linux/clk.h, linux/io.h, linux/module.h,
// linux/of.h, linux/platform_device.h, linux/pm_runtime.h, sound/pcm_params.h, sound/soc.h

const DRV_NAME: &str = "xtfpga-i2s";

const XTFPGA_I2S_VERSION: u32 = 0x00;
const XTFPGA_I2S_CONFIG: u32 = 0x04;
const XTFPGA_I2S_INT_MASK: u32 = 0x08;
const XTFPGA_I2S_INT_STATUS: u32 = 0x0c;
const XTFPGA_I2S_CHAN0_DATA: u32 = 0x10;
const XTFPGA_I2S_CHAN1_DATA: u32 = 0x14;
const XTFPGA_I2S_CHAN2_DATA: u32 = 0x18;
const XTFPGA_I2S_CHAN3_DATA: u32 = 0x1c;

const XTFPGA_I2S_CONFIG_TX_ENABLE: u32 = 0x1;
const XTFPGA_I2S_CONFIG_INT_ENABLE: u32 = 0x2;
const XTFPGA_I2S_CONFIG_LEFT: u32 = 0x4;
const XTFPGA_I2S_CONFIG_RATIO_BASE: u32 = 8;
const XTFPGA_I2S_CONFIG_RATIO_MASK: u32 = 0x0000ff00;
const XTFPGA_I2S_CONFIG_RES_BASE: u32 = 16;
const XTFPGA_I2S_CONFIG_RES_MASK: u32 = 0x003f0000;
const XTFPGA_I2S_CONFIG_LEVEL_BASE: u32 = 24;
const XTFPGA_I2S_CONFIG_LEVEL_MASK: u32 = 0x0f000000;
const XTFPGA_I2S_CONFIG_CHANNEL_BASE: u32 = 28;

const XTFPGA_I2S_INT_UNDERRUN: u32 = 0x1;
const XTFPGA_I2S_INT_LEVEL: u32 = 0x2;
const XTFPGA_I2S_INT_VALID: u32 = 0x3;

const XTFPGA_I2S_FIFO_SIZE: u32 = 8192;

// I2S controller operation:
//
// Enabling TX: output 1 period of zeros (starting with left channel)
// and then queued data.
//
// Level status and interrupt: whenever FIFO level is below FIFO trigger,
// level status is 1 and an IRQ is asserted (if enabled).
//
// Underrun status and interrupt: whenever FIFO is empty, underrun status
// is 1 and an IRQ is asserted (if enabled).

#[repr(C)]
pub struct xtfpga_i2s {
    pub dev: *mut core::ffi::c_void,
    pub clk: *mut core::ffi::c_void,
    pub regmap: *mut core::ffi::c_void,
    pub regs: *mut core::ffi::c_void,

    // current playback substream. NULL if not playing.
    //
    // Access to that field is synchronized between the interrupt handler
    // and userspace through RCU.
    //
    // Interrupt handler (threaded part) does PIO on substream data in RCU
    // read-side critical section. Trigger callback sets and clears the
    // pointer when the playback is started and stopped with
    // rcu_assign_pointer. When userspace is about to free the playback
    // stream in the pcm_close callback it synchronizes with the interrupt
    // handler by means of synchronize_rcu call.
    pub tx_substream: *mut core::ffi::c_void,
    pub tx_fn: Option<unsafe extern "C" fn(
        *mut xtfpga_i2s,
        *mut core::ffi::c_void,
        u32
    ) -> u32>,
    pub tx_ptr: u32,

    // current fifo level estimate.
    // Doesn't have to be perfectly accurate, but must be not less than
    // the actual FIFO level in order to avoid stall on push attempt.
    pub tx_fifo_level: u32,

    // FIFO level at which level interrupt occurs
    pub tx_fifo_low: u32,

    // maximal FIFO level
    pub tx_fifo_high: u32,
}

fn xtfpga_i2s_wr_reg(_dev: *mut core::ffi::c_void, reg: u32) -> bool {
    reg >= XTFPGA_I2S_CONFIG
}

fn xtfpga_i2s_rd_reg(_dev: *mut core::ffi::c_void, reg: u32) -> bool {
    reg < XTFPGA_I2S_CHAN0_DATA
}

fn xtfpga_i2s_volatile_reg(_dev: *mut core::ffi::c_void, reg: u32) -> bool {
    reg == XTFPGA_I2S_INT_STATUS
}

// regmap_config structure: external type reference
// const xtfpga_i2s_regmap_config: regmap_config = ...

// Generate functions that do PIO from TX DMA area to FIFO for all supported
// stream formats.
// Functions will be called xtfpga_pcm_tx_<channels>x<sample bits>, e.g.
// xtfpga_pcm_tx_2x16 for 16-bit stereo.
//
// FIFO consists of 32-bit words, one word per channel, always 2 channels.
// If I2S interface is configured with smaller sample resolution, only
// the LSB of each word is used.

unsafe extern "C" fn xtfpga_pcm_tx_1x16(
    i2s: *mut xtfpga_i2s,
    runtime: *mut core::ffi::c_void,
    mut tx_ptr: u32
) -> u32 {
    let p = (*runtime as *const u16) as *const [[u16; 1]];
    let buffer_size = (*(runtime as *const _ as *const u32)).offset(8);

    while (*i2s).tx_fifo_level < (*i2s).tx_fifo_high {
        core::ptr::write_bytes(
            (*i2s).regs.add(XTFPGA_I2S_CHAN0_DATA as usize) as *mut u32,
            (*p.add(tx_ptr as usize))[0] as u32,
            1
        );
        core::ptr::write_bytes(
            (*i2s).regs.add(XTFPGA_I2S_CHAN0_DATA as usize) as *mut u32,
            (*p.add(tx_ptr as usize))[0] as u32,
            1
        );
        tx_ptr += 1;
        if tx_ptr >= *buffer_size {
            tx_ptr = 0;
        }
        (*i2s).tx_fifo_level += 2;
    }
    tx_ptr
}

unsafe extern "C" fn xtfpga_pcm_tx_2x16(
    i2s: *mut xtfpga_i2s,
    runtime: *mut core::ffi::c_void,
    mut tx_ptr: u32
) -> u32 {
    let p = (*runtime as *const u16) as *const [[u16; 2]];
    let buffer_size = (*(runtime as *const _ as *const u32)).offset(8);

    while (*i2s).tx_fifo_level < (*i2s).tx_fifo_high {
        core::ptr::write_bytes(
            (*i2s).regs.add(XTFPGA_I2S_CHAN0_DATA as usize) as *mut u32,
            (*p.add(tx_ptr as usize))[0] as u32,
            1
        );
        core::ptr::write_bytes(
            (*i2s).regs.add(XTFPGA_I2S_CHAN0_DATA as usize) as *mut u32,
            (*p.add(tx_ptr as usize))[2 - 1] as u32,
            1
        );
        tx_ptr += 1;
        if tx_ptr >= *buffer_size {
            tx_ptr = 0;
        }
        (*i2s).tx_fifo_level += 2;
    }
    tx_ptr
}

unsafe extern "C" fn xtfpga_pcm_tx_1x32(
    i2s: *mut xtfpga_i2s,
    runtime: *mut core::ffi::c_void,
    mut tx_ptr: u32
) -> u32 {
    let p = (*runtime as *const u32) as *const [[u32; 1]];
    let buffer_size = (*(runtime as *const _ as *const u32)).offset(8);

    while (*i2s).tx_fifo_level < (*i2s).tx_fifo_high {
        core::ptr::write_bytes(
            (*i2s).regs.add(XTFPGA_I2S_CHAN0_DATA as usize) as *mut u32,
            (*p.add(tx_ptr as usize))[0],
            1
        );
        core::ptr::write_bytes(
            (*i2s).regs.add(XTFPGA_I2S_CHAN0_DATA as usize) as *mut u32,
            (*p.add(tx_ptr as usize))[0],
            1
        );
        tx_ptr += 1;
        if tx_ptr >= *buffer_size {
            tx_ptr = 0;
        }
        (*i2s).tx_fifo_level += 2;
    }
    tx_ptr
}

unsafe extern "C" fn xtfpga_pcm_tx_2x32(
    i2s: *mut xtfpga_i2s,
    runtime: *mut core::ffi::c_void,
    mut tx_ptr: u32
) -> u32 {
    let p = (*runtime as *const u32) as *const [[u32; 2]];
    let buffer_size = (*(runtime as *const _ as *const u32)).offset(8);

    while (*i2s).tx_fifo_level < (*i2s).tx_fifo_high {
        core::ptr::write_bytes(
            (*i2s).regs.add(XTFPGA_I2S_CHAN0_DATA as usize) as *mut u32,
            (*p.add(tx_ptr as usize))[0],
            1
        );
        core::ptr::write_bytes(
            (*i2s).regs.add(XTFPGA_I2S_CHAN0_DATA as usize) as *mut u32,
            (*p.add(tx_ptr as usize))[2 - 1],
            1
        );
        tx_ptr += 1;
        if tx_ptr >= *buffer_size {
            tx_ptr = 0;
        }
        (*i2s).tx_fifo_level += 2;
    }
    tx_ptr
}

// External kernel function declarations and types
extern "C" {
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference(p: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn snd_pcm_running(substream: *mut core::ffi::c_void) -> bool;
    fn READ_ONCE(x: *const u32) -> u32;
    fn cmpxchg(ptr: *mut u32, old: u32, new: u32) -> u32;
    fn regmap_read(regmap: *mut core::ffi::c_void, reg: u32, val: *mut u32) -> i32;
    fn regmap_write(regmap: *mut core::ffi::c_void, reg: u32, val: u32) -> i32;
    fn regmap_update_bits(
        regmap: *mut core::ffi::c_void,
        reg: u32,
        mask: u32,
        val: u32
    ) -> i32;
    fn snd_pcm_period_elapsed(substream: *mut core::ffi::c_void);
    fn dev_dbg_ratelimited(dev: *mut core::ffi::c_void, fmt: *const i8, ...);
    fn dev_dbg(dev: *mut core::ffi::c_void, fmt: *const i8, ...);
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const i8, ...);
    fn snd_soc_dai_get_drvdata(dai: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn snd_soc_dai_set_dma_data(
        dai: *mut core::ffi::c_void,
        substream: *mut core::ffi::c_void,
        data: *mut core::ffi::c_void
    );
    fn clk_set_rate(clk: *mut core::ffi::c_void, rate: u32) -> i32;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut core::ffi::c_void,
        hw: *const core::ffi::c_void
    );
    fn snd_soc_rtd_to_cpu(rtd: *mut core::ffi::c_void, idx: i32) -> *mut core::ffi::c_void;
    fn snd_soc_substream_to_rtd(substream: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn snd_soc_dai_get_dma_data(
        dai: *mut core::ffi::c_void,
        substream: *mut core::ffi::c_void
    ) -> *mut core::ffi::c_void;
    fn synchronize_rcu();
    fn WRITE_ONCE(ptr: *mut u32, val: u32);
    fn rcu_assign_pointer(ptr: *mut *mut core::ffi::c_void, val: *mut core::ffi::c_void);
    fn clk_disable_unprepare(clk: *mut core::ffi::c_void);
    fn clk_prepare_enable(clk: *mut core::ffi::c_void) -> i32;
    fn devm_kzalloc(dev: *mut core::ffi::c_void, size: usize, gfp: u32) -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut core::ffi::c_void, data: *mut core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut core::ffi::c_void,
        index: u32
    ) -> *mut core::ffi::c_void;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
    fn devm_regmap_init_mmio(
        dev: *mut core::ffi::c_void,
        regs: *mut core::ffi::c_void,
        config: *const core::ffi::c_void
    ) -> *mut core::ffi::c_void;
    fn dev_err_probe(
        dev: *mut core::ffi::c_void,
        err: i32,
        fmt: *const i8, ...
    ) -> i32;
    fn devm_clk_get(dev: *mut core::ffi::c_void, id: *const i8) -> *mut core::ffi::c_void;
    fn platform_get_irq(pdev: *mut core::ffi::c_void, num: u32) -> i32;
    fn devm_request_threaded_irq(
        dev: *mut core::ffi::c_void,
        irq: u32,
        handler: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> u32>,
        thread_fn: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> u32>,
        irqflags: u32,
        devname: *const i8,
        dev_id: *mut core::ffi::c_void
    ) -> i32;
    fn devm_snd_soc_register_component(
        dev: *mut core::ffi::c_void,
        cmpnt_drv: *const core::ffi::c_void,
        dai_drv: *const core::ffi::c_void,
        num_dai: u32
    ) -> i32;
    fn pm_runtime_enable(dev: *mut core::ffi::c_void);
    fn pm_runtime_enabled(dev: *mut core::ffi::c_void) -> bool;
    fn pm_runtime_disable(dev: *mut core::ffi::c_void);
    fn pm_runtime_status_suspended(dev: *mut core::ffi::c_void) -> bool;
}

fn xtfpga_pcm_push_tx(i2s: *mut xtfpga_i2s) -> bool {
    let mut tx_substream: *mut core::ffi::c_void;
    let tx_active: bool;

    unsafe {
        rcu_read_lock();
        tx_substream = rcu_dereference((*i2s).tx_substream);
        tx_active = !tx_substream.is_null() && snd_pcm_running(tx_substream);
        if tx_active {
            let tx_ptr = READ_ONCE(&(*i2s).tx_ptr);
            let new_tx_ptr = if let Some(tx_fn) = (*i2s).tx_fn {
                tx_fn(i2s, (*tx_substream as *const _ as *mut core::ffi::c_void), tx_ptr)
            } else {
                tx_ptr
            };

            cmpxchg(&mut (*i2s).tx_ptr, tx_ptr, new_tx_ptr);
        }
        rcu_read_unlock();
    }

    tx_active
}

fn xtfpga_pcm_refill_fifo(i2s: *mut xtfpga_i2s) {
    let mut int_status: u32 = 0;

    unsafe {
        regmap_read((*i2s).regmap, XTFPGA_I2S_INT_STATUS, &mut int_status);

        for _i in 0..2 {
            let tx_active = xtfpga_pcm_push_tx(i2s);

            regmap_write((*i2s).regmap, XTFPGA_I2S_INT_STATUS, XTFPGA_I2S_INT_VALID);
            if tx_active {
                regmap_read((*i2s).regmap, XTFPGA_I2S_INT_STATUS, &mut int_status);
            }

            if !tx_active || (int_status & XTFPGA_I2S_INT_LEVEL) == 0 {
                break;
            }

            (*i2s).tx_fifo_level = (*i2s).tx_fifo_low;
        }

        if (int_status & XTFPGA_I2S_INT_LEVEL) == 0 {
            regmap_write(
                (*i2s).regmap,
                XTFPGA_I2S_INT_MASK,
                XTFPGA_I2S_INT_VALID
            );
        } else if (int_status & XTFPGA_I2S_INT_UNDERRUN) == 0 {
            regmap_write(
                (*i2s).regmap,
                XTFPGA_I2S_INT_MASK,
                XTFPGA_I2S_INT_UNDERRUN
            );
        }

        if (int_status & XTFPGA_I2S_INT_UNDERRUN) == 0 {
            regmap_update_bits(
                (*i2s).regmap,
                XTFPGA_I2S_CONFIG,
                XTFPGA_I2S_CONFIG_INT_ENABLE | XTFPGA_I2S_CONFIG_TX_ENABLE,
                XTFPGA_I2S_CONFIG_INT_ENABLE | XTFPGA_I2S_CONFIG_TX_ENABLE
            );
        } else {
            regmap_update_bits(
                (*i2s).regmap,
                XTFPGA_I2S_CONFIG,
                XTFPGA_I2S_CONFIG_INT_ENABLE | XTFPGA_I2S_CONFIG_TX_ENABLE,
                0
            );
        }
    }
}

unsafe extern "C" fn xtfpga_i2s_threaded_irq_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> u32 {
    let i2s = dev_id as *mut xtfpga_i2s;
    let mut config: u32 = 0;
    let mut int_mask: u32 = 0;
    let mut int_status: u32 = 0;

    regmap_read((*i2s).regmap, XTFPGA_I2S_CONFIG, &mut config);
    regmap_read((*i2s).regmap, XTFPGA_I2S_INT_MASK, &mut int_mask);
    regmap_read((*i2s).regmap, XTFPGA_I2S_INT_STATUS, &mut int_status);

    if (config & XTFPGA_I2S_CONFIG_INT_ENABLE) == 0
        || (int_status & int_mask & XTFPGA_I2S_INT_VALID) == 0
    {
        return 0; // IRQ_NONE
    }

    if (int_status & XTFPGA_I2S_INT_UNDERRUN) != 0 {
        (*i2s).tx_fifo_level = 0;
        regmap_update_bits(
            (*i2s).regmap,
            XTFPGA_I2S_CONFIG,
            XTFPGA_I2S_CONFIG_TX_ENABLE,
            0
        );
    } else {
        (*i2s).tx_fifo_level = (*i2s).tx_fifo_low;
    }

    rcu_read_lock();
    let tx_substream = rcu_dereference((*i2s).tx_substream);

    if !tx_substream.is_null() && snd_pcm_running(tx_substream) {
        snd_pcm_period_elapsed(tx_substream);
        if (int_status & XTFPGA_I2S_INT_UNDERRUN) != 0 {
            dev_dbg_ratelimited(
                (*i2s).dev,
                b"%s: underrun\n\0".as_ptr() as *const i8,
                b"xtfpga_i2s_threaded_irq_handler\0".as_ptr() as *const i8
            );
        }
    }
    rcu_read_unlock();

    xtfpga_pcm_refill_fifo(i2s);

    1 // IRQ_HANDLED
}

fn xtfpga_i2s_startup(
    _substream: *mut core::ffi::c_void,
    dai: *mut core::ffi::c_void
) -> i32 {
    unsafe {
        let i2s = snd_soc_dai_get_drvdata(dai);
        snd_soc_dai_set_dma_data(dai, _substream, i2s);
    }
    0
}

fn xtfpga_i2s_hw_params(
    _substream: *mut core::ffi::c_void,
    params: *mut core::ffi::c_void,
    dai: *mut core::ffi::c_void
) -> i32 {
    unsafe {
        let i2s = snd_soc_dai_get_drvdata(dai) as *mut xtfpga_i2s;

        // External function calls to extract params
        // srate = params_rate(params);
        // channels = params_channels(params);
        // period_size = params_period_size(params);
        // sample_size = snd_pcm_format_width(params_format(params));
        let srate: u32 = *(params as *const u32);
        let channels: u32 = *(params.add(1) as *const u32);
        let period_size: u32 = *(params.add(2) as *const u32);
        let sample_size: u32 = *(params.add(3) as *const u32);

        regmap_update_bits(
            (*i2s).regmap,
            XTFPGA_I2S_CONFIG,
            XTFPGA_I2S_CONFIG_RES_MASK,
            sample_size << XTFPGA_I2S_CONFIG_RES_BASE
        );

        let freq = 256 * srate;
        let err = clk_set_rate((*i2s).clk, freq);
        if err < 0 {
            return err;
        }

        let ratio = (freq - (srate * sample_size * 8)) / (srate * sample_size * 4);

        regmap_update_bits(
            (*i2s).regmap,
            XTFPGA_I2S_CONFIG,
            XTFPGA_I2S_CONFIG_RATIO_MASK,
            ratio << XTFPGA_I2S_CONFIG_RATIO_BASE
        );

        (*i2s).tx_fifo_low = XTFPGA_I2S_FIFO_SIZE / 2;

        let mut level: u32 = 1;
        while (*i2s).tx_fifo_low / 2 >= period_size * 2
            && level < (XTFPGA_I2S_CONFIG_LEVEL_MASK >> XTFPGA_I2S_CONFIG_LEVEL_BASE)
        {
            (*i2s).tx_fifo_low /= 2;
            level += 1;
        }

        (*i2s).tx_fifo_high = 2 * (*i2s).tx_fifo_low;

        regmap_update_bits(
            (*i2s).regmap,
            XTFPGA_I2S_CONFIG,
            XTFPGA_I2S_CONFIG_LEVEL_MASK,
            level << XTFPGA_I2S_CONFIG_LEVEL_BASE
        );

        dev_dbg(
            (*i2s).dev,
            b"%s srate: %u, channels: %u, sample_size: %u, period_size: %u\n\0".as_ptr() as *const i8,
            b"xtfpga_i2s_hw_params\0".as_ptr() as *const i8,
            srate,
            channels,
            sample_size,
            period_size
        );
        dev_dbg(
            (*i2s).dev,
            b"%s freq: %u, ratio: %u, level: %u\n\0".as_ptr() as *const i8,
            b"xtfpga_i2s_hw_params\0".as_ptr() as *const i8,
            freq,
            ratio,
            level
        );
    }
    0
}

fn xtfpga_i2s_set_fmt(_cpu_dai: *mut core::ffi::c_void, fmt: u32) -> i32 {
    // External constants for format validation
    // SND_SOC_DAIFMT_INV_MASK, SND_SOC_DAIFMT_NB_NF, SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK,
    // SND_SOC_DAIFMT_BP_FP, SND_SOC_DAIFMT_FORMAT_MASK, SND_SOC_DAIFMT_I2S
    const SND_SOC_DAIFMT_INV_MASK: u32 = 0xf;
    const SND_SOC_DAIFMT_NB_NF: u32 = 0;
    const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: u32 = 0xf00;
    const SND_SOC_DAIFMT_BP_FP: u32 = 0x100;
    const SND_SOC_DAIFMT_FORMAT_MASK: u32 = 0xf0000;
    const SND_SOC_DAIFMT_I2S: u32 = 0;

    if (fmt & SND_SOC_DAIFMT_INV_MASK) != SND_SOC_DAIFMT_NB_NF {
        return -22; // EINVAL
    }
    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_BP_FP {
        return -22; // EINVAL
    }
    if (fmt & SND_SOC_DAIFMT_FORMAT_MASK) != SND_SOC_DAIFMT_I2S {
        return -22; // EINVAL
    }

    0
}

// PCM hardware capabilities struct: external type reference
// const xtfpga_pcm_hardware: snd_pcm_hardware = ...

fn xtfpga_pcm_open(
    _component: *mut core::ffi::c_void,
    substream: *mut core::ffi::c_void
) -> i32 {
    unsafe {
        let runtime = *(substream as *const *mut core::ffi::c_void);
        let rtd = snd_soc_substream_to_rtd(substream);

        // snd_soc_set_runtime_hwparams(substream, &xtfpga_pcm_hardware);
        let p = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
        *(runtime.add(1) as *mut *mut core::ffi::c_void) = p;
    }
    0
}

fn xtfpga_pcm_close(_component: *mut core::ffi::c_void, _substream: *mut core::ffi::c_void) -> i32 {
    unsafe {
        synchronize_rcu();
    }
    0
}

fn xtfpga_pcm_hw_params(
    _component: *mut core::ffi::c_void,
    substream: *mut core::ffi::c_void,
    hw_params: *mut core::ffi::c_void
) -> i32 {
    unsafe {
        let runtime = *(substream as *const *mut core::ffi::c_void);
        let i2s = *(runtime.add(1) as *const *mut xtfpga_i2s);

        // channels = params_channels(hw_params);
        let channels: u32 = *(hw_params as *const u32);

        match channels {
            1 | 2 => {}
            _ => return -22, // EINVAL
        }

        // params_format(hw_params)
        let format: u32 = *(hw_params.add(1) as *const u32);

        match format {
            0 => {
                // SNDRV_PCM_FORMAT_S16_LE
                (*i2s).tx_fn = if channels == 1 {
                    Some(xtfpga_pcm_tx_1x16)
                } else {
                    Some(xtfpga_pcm_tx_2x16)
                };
            }
            1 => {
                // SNDRV_PCM_FORMAT_S32_LE
                (*i2s).tx_fn = if channels == 1 {
                    Some(xtfpga_pcm_tx_1x32)
                } else {
                    Some(xtfpga_pcm_tx_2x32)
                };
            }
            _ => return -22, // EINVAL
        }
    }
    0
}

fn xtfpga_pcm_trigger(
    _component: *mut core::ffi::c_void,
    substream: *mut core::ffi::c_void,
    cmd: i32
) -> i32 {
    unsafe {
        let runtime = *(substream as *const *mut core::ffi::c_void);
        let i2s = *(runtime.add(1) as *const *mut xtfpga_i2s);

        const SNDRV_PCM_TRIGGER_START: i32 = 0;
        const SNDRV_PCM_TRIGGER_STOP: i32 = 1;
        const SNDRV_PCM_TRIGGER_PAUSE_PUSH: i32 = 3;
        const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: i32 = 4;
        const SNDRV_PCM_TRIGGER_SUSPEND: i32 = 5;
        const SNDRV_PCM_TRIGGER_RESUME: i32 = 6;

        match cmd {
            SNDRV_PCM_TRIGGER_START
            | SNDRV_PCM_TRIGGER_RESUME
            | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                WRITE_ONCE(&mut (*i2s).tx_ptr, 0);
                rcu_assign_pointer(&mut (*i2s).tx_substream, substream);
                xtfpga_pcm_refill_fifo(i2s);
            }

            SNDRV_PCM_TRIGGER_STOP
            | SNDRV_PCM_TRIGGER_SUSPEND
            | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                rcu_assign_pointer(&mut (*i2s).tx_substream, core::ptr::null_mut());
            }

            _ => return -22, // EINVAL
        }
    }
    0
}

fn xtfpga_pcm_pointer(
    _component: *mut core::ffi::c_void,
    substream: *mut core::ffi::c_void
) -> u32 {
    unsafe {
        let runtime = *(substream as *const *mut core::ffi::c_void);
        let i2s = *(runtime.add(1) as *const *mut xtfpga_i2s);
        let pos = READ_ONCE(&(*i2s).tx_ptr);

        let buffer_size = *(runtime.add(8) as *const u32);
        if pos < buffer_size {
            pos
        } else {
            0
        }
    }
}

fn xtfpga_pcm_new(
    _component: *mut core::ffi::c_void,
    rtd: *mut core::ffi::c_void
) -> i32 {
    unsafe {
        let card = *(rtd as *const *mut core::ffi::c_void);
        let size = XTFPGA_I2S_FIFO_SIZE * 8;

        // snd_pcm_set_managed_buffer_all(rtd->pcm, SNDRV_DMA_TYPE_DEV, card->dev, size, size);
    }
    0
}

// External driver structures
// const xtfpga_i2s_component: snd_soc_component_driver = ...
// const xtfpga_i2s_dai_ops: snd_soc_dai_ops = ...
// const xtfpga_i2s_dai: [snd_soc_dai_driver; 1] = ...

fn xtfpga_i2s_runtime_suspend(dev: *mut core::ffi::c_void) -> i32 {
    unsafe {
        let i2s = dev_get_drvdata(dev) as *mut xtfpga_i2s;
        clk_disable_unprepare((*i2s).clk);
    }
    0
}

fn xtfpga_i2s_runtime_resume(dev: *mut core::ffi::c_void) -> i32 {
    unsafe {
        let i2s = dev_get_drvdata(dev) as *mut xtfpga_i2s;
        let ret = clk_prepare_enable((*i2s).clk);
        if ret != 0 {
            dev_err(dev, b"clk_prepare_enable failed: %d\n\0".as_ptr() as *const i8, ret);
            return ret;
        }
    }
    0
}

fn xtfpga_i2s_probe(pdev: *mut core::ffi::c_void) -> i32 {
    unsafe {
        let i2s = devm_kzalloc(pdev, core::mem::size_of::<xtfpga_i2s>(), 0xd0) as *mut xtfpga_i2s;
        if i2s.is_null() {
            return -12; // ENOMEM
        }

        platform_set_drvdata(pdev, i2s as *mut core::ffi::c_void);
        (*i2s).dev = pdev;
        dev_dbg(pdev, b"dev: %p, i2s: %p\n\0".as_ptr() as *const i8, pdev, i2s);

        (*i2s).regs = devm_platform_ioremap_resource(pdev, 0);
        if IS_ERR((*i2s).regs as *const core::ffi::c_void) {
            return PTR_ERR((*i2s).regs as *const core::ffi::c_void);
        }

        (*i2s).regmap = devm_regmap_init_mmio(
            pdev,
            (*i2s).regs,
            core::ptr::null() as *const core::ffi::c_void
        );
        if IS_ERR((*i2s).regmap as *const core::ffi::c_void) {
            return dev_err_probe(
                pdev,
                PTR_ERR((*i2s).regmap as *const core::ffi::c_void),
                b"regmap init failed\n\0".as_ptr() as *const i8
            );
        }

        (*i2s).clk = devm_clk_get(pdev, core::ptr::null());
        if IS_ERR((*i2s).clk as *const core::ffi::c_void) {
            return dev_err_probe(
                pdev,
                PTR_ERR((*i2s).clk as *const core::ffi::c_void),
                b"couldn't get clock\n\0".as_ptr() as *const i8
            );
        }

        regmap_write(
            (*i2s).regmap,
            XTFPGA_I2S_CONFIG,
            0x1 << XTFPGA_I2S_CONFIG_CHANNEL_BASE
        );
        regmap_write(
            (*i2s).regmap,
            XTFPGA_I2S_INT_STATUS,
            XTFPGA_I2S_INT_VALID
        );
        regmap_write(
            (*i2s).regmap,
            XTFPGA_I2S_INT_MASK,
            XTFPGA_I2S_INT_UNDERRUN
        );

        let irq = platform_get_irq(pdev, 0);
        if irq < 0 {
            return irq;
        }

        let err = devm_request_threaded_irq(
            pdev,
            irq as u32,
            None,
            Some(xtfpga_i2s_threaded_irq_handler),
            0x02000020 | 0x00000002, // IRQF_SHARED | IRQF_ONESHOT
            pdev as *const i8,
            i2s as *mut core::ffi::c_void
        );
        if err < 0 {
            return err;
        }

        let err = devm_snd_soc_register_component(
            pdev,
            core::ptr::null(),
            core::ptr::null(),
            1
        );
        if err < 0 {
            return err;
        }

        pm_runtime_enable(pdev);
        if !pm_runtime_enabled(pdev) {
            let err = xtfpga_i2s_runtime_resume(pdev);
            if err != 0 {
                pm_runtime_disable(pdev);
                return err;
            }
        }

        0
    }
}

fn xtfpga_i2s_remove(pdev: *mut core::ffi::c_void) {
    unsafe {
        let i2s = dev_get_drvdata(pdev) as *mut xtfpga_i2s;

        if !(*i2s).regmap.is_null() && !IS_ERR((*i2s).regmap as *const core::ffi::c_void) {
            regmap_write((*i2s).regmap, XTFPGA_I2S_CONFIG, 0);
            regmap_write((*i2s).regmap, XTFPGA_I2S_INT_MASK, 0);
            regmap_write(
                (*i2s).regmap,
                XTFPGA_I2S_INT_STATUS,
                XTFPGA_I2S_INT_VALID
            );
        }
        pm_runtime_disable(pdev);
        if !pm_runtime_status_suspended(pdev) {
            xtfpga_i2s_runtime_suspend(pdev);
        }
    }
}

// Platform driver registration
// #ifdef CONFIG_OF
// const xtfpga_i2s_of_match: [of_device_id; 2] = ...
// MODULE_DEVICE_TABLE(of, xtfpga_i2s_of_match);
// #endif
//
// const xtfpga_i2s_pm_ops: dev_pm_ops = ...
//
// const xtfpga_i2s_driver: platform_driver = ...
//
// module_platform_driver(xtfpga_i2s_driver);
//
// MODULE_AUTHOR("Max Filippov <jcmvbkbc@gmail.com>");
// MODULE_DESCRIPTION("xtfpga I2S controller driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
