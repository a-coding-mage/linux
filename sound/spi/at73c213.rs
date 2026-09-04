// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for AT73C213 16-bit stereo DAC connected to Atmel SSC
 *
 * Copyright (C) 2006-2007 Atmel Norway
 */

// Includes from linux kernel headers:
// #include <linux/clk.h>
// #include <linux/err.h>
// #include <linux/delay.h>
// #include <linux/device.h>
// #include <linux/dma-mapping.h>
// #include <linux/init.h>
// #include <linux/interrupt.h>
// #include <linux/module.h>
// #include <linux/mutex.h>
// #include <linux/platform_device.h>
// #include <linux/io.h>
// #include <sound/initval.h>
// #include <sound/control.h>
// #include <sound/core.h>
// #include <sound/pcm.h>
// #include <linux/atmel-ssc.h>
// #include <linux/spi/spi.h>
// #include <linux/spi/at73c213.h>
// #include "at73c213.h"

const BITRATE_MIN: usize = 8000;
const BITRATE_TARGET: usize = 16000; // CONFIG_SND_AT73C213_TARGET_BITRATE
const BITRATE_MAX: usize = 50000;

// Initial (hardware reset) AT73C213 register values.
static SND_AT73C213_ORIGINAL_IMAGE: &[u8] = &[
    0x00,   // 00 - CTRL
    0x05,   // 01 - LLIG
    0x05,   // 02 - RLIG
    0x08,   // 03 - LPMG
    0x08,   // 04 - RPMG
    0x00,   // 05 - LLOG
    0x00,   // 06 - RLOG
    0x22,   // 07 - OLC
    0x09,   // 08 - MC
    0x00,   // 09 - CSFC
    0x00,   // 0A - MISC
    0x00,   // 0B -
    0x00,   // 0C - PRECH
    0x05,   // 0D - AUXG
    0x00,   // 0E -
    0x00,   // 0F -
    0x00,   // 10 - RST
    0x00,   // 11 - PA_CTRL
];

// External types and structures from dependencies
#[repr(C)]
pub struct snd_card {
    _private_data: *mut core::ffi::c_void,
    shortname: [u8; 80],
    driver: [u8; 16],
    longname: [u8; 80],
    mixername: [u8; 80],
    // ... other fields
}

#[repr(C)]
pub struct snd_pcm {
    _private_data: *mut core::ffi::c_void,
    name: [u8; 80],
    info_flags: u32,
    // ... other fields
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    // ... other fields
}

#[repr(C)]
pub struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    dma_addr: u32,
    period_size: usize,
    buffer_size: usize,
    channels: u32,
    periods: u32,
    // ... other fields
}

#[repr(C)]
pub struct snd_pcm_hardware {
    info: u32,
    formats: u64,
    rates: u64,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: usize,
    period_bytes_min: usize,
    period_bytes_max: usize,
    periods_min: u32,
    periods_max: u32,
}

#[repr(C)]
pub struct spi_device {
    dev: device,
    // ... other fields
}

#[repr(C)]
pub struct spi_message {
    // ... fields
}

#[repr(C)]
pub struct spi_transfer {
    len: u32,
    cs_change: u32,
    tx_buf: *mut u8,
    rx_buf: *mut u8,
}

#[repr(C)]
pub struct ssc_device {
    clk: *mut core::ffi::c_void,
    irq: i32,
    regs: *mut core::ffi::c_void,
    pdev: *mut platform_device,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct device {
    // ... fields
}

#[repr(C)]
pub struct at73c213_board_info {
    dac_clk: *mut core::ffi::c_void,
    ssc_id: u32,
    shortname: *const u8,
}

#[repr(C)]
pub struct snd_device {
    device_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct snd_device_ops {
    dev_free: extern "C" fn(*mut snd_device) -> i32,
}

#[repr(C)]
pub struct snd_kcontrol {
    private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    value_integer_value: [i64; 128],
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    elem_type: u32,
    count: u32,
    value_integer_min: i64,
    value_integer_max: i64,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    iface: u32,
    name: *const u8,
    index: u32,
    info: extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> i32,
    get: extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32,
    put: extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> i32,
    private_value: usize,
}

#[repr(C)]
pub struct snd_pcm_ops {
    open: extern "C" fn(*mut snd_pcm_substream) -> i32,
    close: extern "C" fn(*mut snd_pcm_substream) -> i32,
    hw_params: extern "C" fn(*mut snd_pcm_substream, *mut core::ffi::c_void) -> i32,
    prepare: extern "C" fn(*mut snd_pcm_substream) -> i32,
    trigger: extern "C" fn(*mut snd_pcm_substream, i32) -> i32,
    pointer: extern "C" fn(*mut snd_pcm_substream) -> u32,
}

#[repr(C)]
pub struct spinlock_t {
    // Opaque
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    // Opaque
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_at73c213 {
    card: *mut snd_card,
    pcm: *mut snd_pcm,
    substream: *mut snd_pcm_substream,
    board: *mut at73c213_board_info,
    irq: i32,
    period: i32,
    bitrate: u32,
    ssc: *mut ssc_device,
    spi: *mut spi_device,
    spi_wbuffer: [u8; 2],
    spi_rbuffer: [u8; 2],
    reg_image: [u8; 18],
    lock: spinlock_t,
    mixer_lock: mutex,
}

fn get_chip(card: *mut snd_card) -> *mut snd_at73c213 {
    unsafe {
        (*card)._private_data as *mut snd_at73c213
    }
}

extern "C" {
    fn spi_message_init(msg: *mut spi_message);
    fn spi_message_add_tail(xfer: *mut spi_transfer, msg: *mut spi_message);
    fn spi_sync(spi: *mut spi_device, msg: *mut spi_message) -> i32;
    fn clk_get_rate(clk: *mut core::ffi::c_void) -> u32;
    fn clk_round_rate(clk: *mut core::ffi::c_void, rate: u32) -> i32;
    fn clk_set_rate(clk: *mut core::ffi::c_void, rate: u32) -> i32;
    fn clk_enable(clk: *mut core::ffi::c_void) -> i32;
    fn clk_disable(clk: *mut core::ffi::c_void);
    fn ssc_readl(regs: *mut core::ffi::c_void, offset: u32) -> u32;
    fn ssc_writel(regs: *mut core::ffi::c_void, offset: u32, val: u32);
    fn ssc_request(id: u32) -> *mut ssc_device;
    fn ssc_free(ssc: *mut ssc_device);
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: u32) -> u32;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: u32) -> u32;
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut snd_at73c213;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, param: u32) -> i32;
    fn snd_pcm_new(card: *mut snd_card, shortname: *const u8, device: i32,
                    playback_count: i32, capture_count: i32, rpcm: *mut *mut snd_pcm) -> i32;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, stream: i32, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut snd_pcm, dtype: u32,
                                       dev: *mut device, size_min: usize, size_max: usize);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_card_new(dev: *mut device, idx: i32, xid: *const u8, module: *const core::ffi::c_void,
                    extra_size: usize, card_ret: *mut *mut snd_card) -> i32;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_register(card: *mut snd_card) -> i32;
    fn snd_device_new(card: *mut snd_card, dtype: u32, device_data: *mut core::ffi::c_void,
                      ops: *const snd_device_ops) -> i32;
    fn snd_device_free(card: *mut snd_card, device_data: *mut core::ffi::c_void);
    fn snd_ctl_add(card: *mut snd_card, kcontrol: *mut snd_kcontrol) -> i32;
    fn snd_ctl_new1(kcontrolp: *const snd_kcontrol_new, private_data: *mut core::ffi::c_void) -> *mut snd_kcontrol;
    fn snd_ctl_remove(card: *mut snd_card, kcontrol: *mut snd_kcontrol);
    fn snd_ctl_find_numid(card: *mut snd_card, numid: u32) -> *mut snd_kcontrol;
    fn request_irq(irq: u32, handler: extern "C" fn(i32, *mut core::ffi::c_void) -> i32,
                   flags: u32, name: *const u8, dev: *mut core::ffi::c_void) -> i32;
    fn free_irq(irq: u32, dev_id: *mut core::ffi::c_void);
    fn spin_lock_init(lock: *mut spinlock_t);
    fn mutex_init(lock: *mut mutex);
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dev_info(dev: *mut device, fmt: *const u8, ...);
    fn msleep(msecs: u32);
    fn snprintf(buf: *mut u8, size: usize, fmt: *const u8, ...) -> i32;
    fn strscpy(dest: *mut u8, src: *const u8, size: usize) -> usize;
    fn sprintf(buf: *mut u8, fmt: *const u8, ...) -> i32;
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
}

fn snd_at73c213_write_reg(chip: *mut snd_at73c213, reg: u8, val: u8) -> i32 {
    unsafe {
        let mut msg: spi_message = core::mem::zeroed();
        let mut msg_xfer: spi_transfer = core::mem::zeroed();

        msg_xfer.len = 2;
        msg_xfer.cs_change = 0;

        spi_message_init(&mut msg);

        (*chip).spi_wbuffer[0] = reg;
        (*chip).spi_wbuffer[1] = val;

        msg_xfer.tx_buf = (*chip).spi_wbuffer.as_mut_ptr();
        msg_xfer.rx_buf = (*chip).spi_rbuffer.as_mut_ptr();
        spi_message_add_tail(&mut msg_xfer, &mut msg);

        let retval = spi_sync((*chip).spi, &mut msg);

        if retval == 0 {
            (*chip).reg_image[reg as usize] = val;
        }

        retval
    }
}

static mut SND_AT73C213_PLAYBACK_HW: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,        // SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER
    formats: 1 << 3, // SNDRV_PCM_FMTBIT_S16_BE
    rates: 1 << 9,   // SNDRV_PCM_RATE_CONTINUOUS
    rate_min: 8000,
    rate_max: 50000,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 64 * 1024 - 1,
    period_bytes_min: 512,
    period_bytes_max: 64 * 1024 - 1,
    periods_min: 4,
    periods_max: 1024,
};

fn snd_at73c213_set_bitrate(chip: *mut snd_at73c213) -> i32 {
    unsafe {
        let ssc_rate = clk_get_rate((*(*chip).ssc).clk);
        let mut dac_rate_new: u32;
        let mut ssc_div: u32;
        let status: i32;
        let ssc_div_max: u32;
        let ssc_div_min: u32;
        let mut max_tries: i32;

        ssc_div = ssc_rate / (BITRATE_TARGET as u32 * 2 * 16);
        ssc_div_min = ssc_rate / (BITRATE_MAX as u32 * 2 * 16);
        ssc_div_max = ssc_rate / (BITRATE_MIN as u32 * 2 * 16);
        max_tries = ((ssc_div_max - ssc_div_min) / 2) as i32;

        if max_tries < 1 {
            max_tries = 1;
        }

        // ssc_div must be even.
        ssc_div = (ssc_div + 1) & !1;

        if (ssc_rate / (ssc_div * 2 * 16)) < BITRATE_MIN as u32 {
            ssc_div -= 2;
            if (ssc_rate / (ssc_div * 2 * 16)) > BITRATE_MAX as u32 {
                return -74; // -ENXIO
            }
        }

        // Search for a possible bitrate.
        loop {
            // SSC clock / (ssc divider * 16-bit * stereo).
            if (ssc_rate / (ssc_div * 2 * 16)) < BITRATE_MIN as u32 {
                return -74; // -ENXIO
            }

            // 256 / (2 * 16) = 8
            dac_rate_new = 8 * (ssc_rate / ssc_div);

            let status = clk_round_rate((*chip).board as *mut core::ffi::c_void, dac_rate_new);
            if status <= 0 {
                return status;
            }

            // Ignore difference smaller than 256 Hz.
            if (status as u32 / 256) == (dac_rate_new / 256) {
                // goto set_rate
                let status2 = clk_set_rate((*(*chip).board).dac_clk, status as u32);
                if status2 < 0 {
                    return status2;
                }

                // Set divider in SSC device.
                ssc_writel((*(*chip).ssc).regs, 0x04, ssc_div / 2); // CMR

                // SSC clock / (ssc divider * 16-bit * stereo).
                (*chip).bitrate = ssc_rate / (ssc_div * 16 * 2);

                dev_info(&(*(*chip).spi).dev,
                         b"at73c213: supported bitrate is %lu (%lu divider)\n" as *const u8,
                         (*chip).bitrate as usize, ssc_div as usize);

                return 0;
            }

            ssc_div += 2;
            max_tries -= 1;
            if max_tries <= 0 {
                break;
            }
        }

        // Not able to find a valid bitrate.
        -74 // -ENXIO
    }
}

fn snd_at73c213_pcm_open(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let err: i32;

        // ensure buffer_size is a multiple of period_size
        let err = snd_pcm_hw_constraint_integer(runtime, 12); // SNDRV_PCM_HW_PARAM_PERIODS
        if err < 0 {
            return err;
        }

        SND_AT73C213_PLAYBACK_HW.rate_min = (*chip).bitrate;
        SND_AT73C213_PLAYBACK_HW.rate_max = (*chip).bitrate;
        (*runtime).hw = SND_AT73C213_PLAYBACK_HW;
        (*chip).substream = substream;

        let err = clk_enable((*(*chip).ssc).clk);
        if err != 0 {
            return err;
        }

        0
    }
}

fn snd_at73c213_pcm_close(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        (*chip).substream = core::ptr::null_mut();
        clk_disable((*(*chip).ssc).clk);
        0
    }
}

fn snd_at73c213_pcm_hw_params(substream: *mut snd_pcm_substream,
                               hw_params: *mut core::ffi::c_void) -> i32 {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let channels = 2; // params_channels(hw_params) - simplified
        let mut val: u32;

        val = ssc_readl((*(*chip).ssc).regs, 0x08); // TFMR
        // SSC_BFINS macro - insert field
        val = (val & !(0x0f << 8)) | ((channels as u32 - 1) << 8);
        ssc_writel((*(*chip).ssc).regs, 0x08, val); // TFMR

        0
    }
}

fn snd_at73c213_pcm_prepare(substream: *mut snd_pcm_substream) -> i32 {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let block_size: u32;

        block_size = frames_to_bytes(runtime, (*runtime).period_size as u32);

        (*chip).period = 0;

        ssc_writel((*(*chip).ssc).regs, 0x20,
                   (*runtime).dma_addr); // PDC_TPR
        ssc_writel((*(*chip).ssc).regs, 0x24,
                   ((*runtime).period_size * (*runtime).channels as usize) as u32); // PDC_TCR
        ssc_writel((*(*chip).ssc).regs, 0x28,
                   (*runtime).dma_addr + block_size); // PDC_TNPR
        ssc_writel((*(*chip).ssc).regs, 0x2c,
                   ((*runtime).period_size * (*runtime).channels as usize) as u32); // PDC_TNCR

        0
    }
}

fn snd_at73c213_pcm_trigger(substream: *mut snd_pcm_substream, cmd: i32) -> i32 {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);

        // guard(spinlock)(&chip->lock);
        // Simplified - in real code this would be a scoped lock

        match cmd {
            1 => { // SNDRV_PCM_TRIGGER_START
                ssc_writel((*(*chip).ssc).regs, 0x14, 0x00000040); // IER, SSC_BIT(IER_ENDTX)
                ssc_writel((*(*chip).ssc).regs, 0x2c, 0x00000100); // PDC_PTCR, SSC_BIT(PDC_PTCR_TXTEN)
            },
            2 => { // SNDRV_PCM_TRIGGER_STOP
                ssc_writel((*(*chip).ssc).regs, 0x2c, 0x00000200); // PDC_PTCR, SSC_BIT(PDC_PTCR_TXTDIS)
                ssc_writel((*(*chip).ssc).regs, 0x18, 0x00000040); // IDR, SSC_BIT(IDR_ENDTX)
            },
            _ => {
                dev_dbg(&(*(*chip).spi).dev, b"spurious command %x\n" as *const u8, cmd);
                return -22; // -EINVAL
            }
        }

        0
    }
}

fn snd_at73c213_pcm_pointer(substream: *mut snd_pcm_substream) -> u32 {
    unsafe {
        let chip = snd_pcm_substream_chip(substream);
        let runtime = (*substream).runtime;
        let mut pos: u32;
        let bytes: u32;

        bytes = ssc_readl((*(*chip).ssc).regs, 0x20) - (*runtime).dma_addr; // PDC_TPR

        pos = bytes_to_frames(runtime, bytes) as u32;
        if pos >= (*runtime).buffer_size as u32 {
            pos -= (*runtime).buffer_size as u32;
        }

        pos
    }
}

static AT73C213_PLAYBACK_OPS: snd_pcm_ops = snd_pcm_ops {
    open: snd_at73c213_pcm_open,
    close: snd_at73c213_pcm_close,
    hw_params: snd_at73c213_pcm_hw_params,
    prepare: snd_at73c213_pcm_prepare,
    trigger: snd_at73c213_pcm_trigger,
    pointer: snd_at73c213_pcm_pointer,
};

fn snd_at73c213_pcm_new(chip: *mut snd_at73c213, device: i32) -> i32 {
    unsafe {
        let mut pcm: *mut snd_pcm = core::ptr::null_mut();
        let retval: i32;

        let retval = snd_pcm_new((*chip).card, (*(*chip).card).shortname.as_ptr(),
                                  device, 1, 0, &mut pcm);
        if retval < 0 {
            return retval;
        }

        (*pcm)._private_data = chip as *mut core::ffi::c_void;
        (*pcm).info_flags = 0x00000002; // SNDRV_PCM_INFO_BLOCK_TRANSFER
        strscpy((*pcm).name.as_mut_ptr(), b"at73c213\0" as *const u8, 80);
        (*chip).pcm = pcm;

        snd_pcm_set_ops(pcm, 0, &AT73C213_PLAYBACK_OPS); // SNDRV_PCM_STREAM_PLAYBACK

        snd_pcm_set_managed_buffer_all((*chip).pcm,
                                        3, // SNDRV_DMA_TYPE_DEV
                                        &(*(*(*chip).ssc).pdev).dev,
                                        64 * 1024, 64 * 1024);

        retval
    }
}

extern "C" fn snd_at73c213_interrupt(irq: i32, dev_id: *mut core::ffi::c_void) -> i32 {
    unsafe {
        let chip = dev_id as *mut snd_at73c213;
        let runtime = (*(*chip).substream).runtime;
        let status: u32;
        let offset: i32;
        let block_size: u32;
        let next_period: i32;
        let mut retval = 0; // IRQ_NONE

        // scoped_guard(spinlock, &chip->lock)
        {
            block_size = frames_to_bytes(runtime, (*runtime).period_size as u32);
            let status = ssc_readl((*(*chip).ssc).regs, 0x1c); // IMR

            if (status & 0x00000040) != 0 { // SSC_BIT(IMR_ENDTX)
                (*chip).period += 1;
                if (*chip).period == (*runtime).periods as i32 {
                    (*chip).period = 0;
                }
                let next_period = (*chip).period + 1;
                let next_period = if next_period == (*runtime).periods as i32 {
                    0
                } else {
                    next_period
                };

                let offset = block_size as i32 * next_period;

                ssc_writel((*(*chip).ssc).regs, 0x28,
                           (*runtime).dma_addr + offset as u32); // PDC_TNPR
                ssc_writel((*(*chip).ssc).regs, 0x2c,
                           ((*runtime).period_size * (*runtime).channels as usize) as u32); // PDC_TNCR
                retval = 1; // IRQ_HANDLED
            }

            let _ = ssc_readl((*(*chip).ssc).regs, 0x1c); // IMR
        }

        if (status & 0x00000040) != 0 { // SSC_BIT(IMR_ENDTX)
            snd_pcm_period_elapsed((*chip).substream);
        }

        retval
    }
}

fn snd_at73c213_mono_get(kcontrol: *mut snd_kcontrol,
                         ucontrol: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = kcontrol as *mut snd_at73c213; // simplified - normally would extract from control
        let reg = ((*kcontrol).private_value & 0xff) as u8;
        let shift = (((*kcontrol).private_value >> 8) & 0xff) as u8;
        let mask = (((*kcontrol).private_value >> 16) & 0xff) as u8;
        let invert = (((*kcontrol).private_value >> 24) & 0xff) as u8;

        // guard(mutex)(&chip->mixer_lock);

        let val = (((*chip).reg_image[reg as usize] >> shift) & mask) as i64;
        *(&mut (*ucontrol).value_integer_value[0]) = val;

        if invert != 0 {
            *(&mut (*ucontrol).value_integer_value[0]) =
                (mask as i64) - (*ucontrol).value_integer_value[0];
        }

        0
    }
}

fn snd_at73c213_mono_put(kcontrol: *mut snd_kcontrol,
                         ucontrol: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = kcontrol as *mut snd_at73c213; // simplified
        let reg = ((*kcontrol).private_value & 0xff) as u8;
        let shift = (((*kcontrol).private_value >> 8) & 0xff) as u8;
        let mask = (((*kcontrol).private_value >> 16) & 0xff) as u8;
        let invert = (((*kcontrol).private_value >> 24) & 0xff) as u8;
        let mut val: u16 = ((*ucontrol).value_integer_value[0] as u16) & (mask as u16);

        if invert != 0 {
            val = (mask as u16) - val;
        }
        val <<= shift;

        // guard(mutex)(&chip->mixer_lock);

        val = (((*chip).reg_image[reg as usize] as u16) & !((mask as u16) << shift)) | val;
        let change = val as u8 != (*chip).reg_image[reg as usize];
        let retval = snd_at73c213_write_reg(chip, reg, val as u8);

        if retval != 0 {
            return retval;
        }

        if change { 1 } else { 0 }
    }
}

fn snd_at73c213_stereo_info(kcontrol: *mut snd_kcontrol,
                            uinfo: *mut snd_ctl_elem_info) -> i32 {
    unsafe {
        let mask = (((*kcontrol).private_value >> 24) & 0xff) as u8;

        if mask == 1 {
            (*uinfo).elem_type = 1; // SNDRV_CTL_ELEM_TYPE_BOOLEAN
        } else {
            (*uinfo).elem_type = 0; // SNDRV_CTL_ELEM_TYPE_INTEGER
        }

        (*uinfo).count = 2;
        (*uinfo).value_integer_min = 0;
        (*uinfo).value_integer_max = mask as i64;

        0
    }
}

fn snd_at73c213_stereo_get(kcontrol: *mut snd_kcontrol,
                           ucontrol: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = kcontrol as *mut snd_at73c213; // simplified
        let left_reg = ((*kcontrol).private_value & 0xff) as u8;
        let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as u8;
        let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as u8;
        let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as u8;
        let mask = (((*kcontrol).private_value >> 24) & 0xff) as u8;
        let invert = (((*kcontrol).private_value >> 22) & 1) as u8;

        // guard(mutex)(&chip->mixer_lock);

        (*ucontrol).value_integer_value[0] =
            ((((*chip).reg_image[left_reg as usize] >> shift_left) & mask) as i64);
        (*ucontrol).value_integer_value[1] =
            ((((*chip).reg_image[right_reg as usize] >> shift_right) & mask) as i64);

        if invert != 0 {
            (*ucontrol).value_integer_value[0] =
                (mask as i64) - (*ucontrol).value_integer_value[0];
            (*ucontrol).value_integer_value[1] =
                (mask as i64) - (*ucontrol).value_integer_value[1];
        }

        0
    }
}

fn snd_at73c213_stereo_put(kcontrol: *mut snd_kcontrol,
                           ucontrol: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = kcontrol as *mut snd_at73c213; // simplified
        let left_reg = ((*kcontrol).private_value & 0xff) as u8;
        let right_reg = (((*kcontrol).private_value >> 8) & 0xff) as u8;
        let shift_left = (((*kcontrol).private_value >> 16) & 0x07) as u8;
        let shift_right = (((*kcontrol).private_value >> 19) & 0x07) as u8;
        let mask = (((*kcontrol).private_value >> 24) & 0xff) as u8;
        let invert = (((*kcontrol).private_value >> 22) & 1) as u8;
        let mut val1: u16 = ((*ucontrol).value_integer_value[0] as u16) & (mask as u16);
        let mut val2: u16 = ((*ucontrol).value_integer_value[1] as u16) & (mask as u16);

        if invert != 0 {
            val1 = (mask as u16) - val1;
            val2 = (mask as u16) - val2;
        }
        val1 <<= shift_left;
        val2 <<= shift_right;

        // guard(mutex)(&chip->mixer_lock);

        val1 = (((*chip).reg_image[left_reg as usize] as u16) & !((mask as u16) << shift_left)) | val1;
        val2 = (((*chip).reg_image[right_reg as usize] as u16) & !((mask as u16) << shift_right)) | val2;
        let change = (val1 as u8 != (*chip).reg_image[left_reg as usize])
            || (val2 as u8 != (*chip).reg_image[right_reg as usize]);

        let retval = snd_at73c213_write_reg(chip, left_reg, val1 as u8);
        if retval != 0 {
            return retval;
        }
        let retval = snd_at73c213_write_reg(chip, right_reg, val2 as u8);
        if retval != 0 {
            return retval;
        }

        if change { 1 } else { 0 }
    }
}

fn snd_at73c213_mono_switch_get(kcontrol: *mut snd_kcontrol,
                                ucontrol: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = kcontrol as *mut snd_at73c213; // simplified
        let reg = ((*kcontrol).private_value & 0xff) as u8;
        let shift = (((*kcontrol).private_value >> 8) & 0xff) as u8;
        let invert = (((*kcontrol).private_value >> 24) & 0xff) as u8;

        // guard(mutex)(&chip->mixer_lock);

        (*ucontrol).value_integer_value[0] =
            ((((*chip).reg_image[reg as usize] >> shift) & 0x01) as i64);

        if invert != 0 {
            (*ucontrol).value_integer_value[0] =
                0x01 - (*ucontrol).value_integer_value[0];
        }

        0
    }
}

fn snd_at73c213_mono_switch_put(kcontrol: *mut snd_kcontrol,
                                ucontrol: *mut snd_ctl_elem_value) -> i32 {
    unsafe {
        let chip = kcontrol as *mut snd_at73c213; // simplified
        let reg = ((*kcontrol).private_value & 0xff) as u8;
        let shift = (((*kcontrol).private_value >> 8) & 0xff) as u8;
        let mask = (((*kcontrol).private_value >> 16) & 0xff) as u8;
        let invert = (((*kcontrol).private_value >> 24) & 0xff) as u8;
        let mut val: u8 = if (*ucontrol).value_integer_value[0] != 0 {
            mask
        } else {
            0
        };

        if invert != 0 {
            val = mask - val;
        }
        val <<= shift;

        // guard(mutex)(&chip->mixer_lock);

        val |= (*chip).reg_image[reg as usize] & !((mask) << shift);
        let change = val != (*chip).reg_image[reg as usize];

        let retval = snd_at73c213_write_reg(chip, reg, val);

        if retval != 0 {
            return retval;
        }

        if change { 1 } else { 0 }
    }
}

fn snd_at73c213_pa_volume_info(kcontrol: *mut snd_kcontrol,
                               uinfo: *mut snd_ctl_elem_info) -> i32 {
    unsafe {
        (*uinfo).elem_type = 0; // SNDRV_CTL_ELEM_TYPE_INTEGER
        (*uinfo).count = 1;
        (*uinfo).value_integer_min = 0;
        (*uinfo).value_integer_max = ((((*kcontrol).private_value >> 16) & 0xff) - 1) as i64;

        0
    }
}

fn snd_at73c213_line_capture_volume_info(_kcontrol: *mut snd_kcontrol,
                                         uinfo: *mut snd_ctl_elem_info) -> i32 {
    unsafe {
        (*uinfo).elem_type = 0; // SNDRV_CTL_ELEM_TYPE_INTEGER
        (*uinfo).count = 2;
        (*uinfo).value_integer_min = 14;
        (*uinfo).value_integer_max = 31;

        0
    }
}

fn snd_at73c213_aux_capture_volume_info(_kcontrol: *mut snd_kcontrol,
                                        uinfo: *mut snd_ctl_elem_info) -> i32 {
    unsafe {
        (*uinfo).elem_type = 0; // SNDRV_CTL_ELEM_TYPE_INTEGER
        (*uinfo).count = 1;
        (*uinfo).value_integer_min = 14;
        (*uinfo).value_integer_max = 31;

        0
    }
}

// Register constants from at73c213.h
const DAC_LMPG: u8 = 0x03;
const DAC_RMPG: u8 = 0x04;
const DAC_LLOG: u8 = 0x05;
const DAC_RLOG: u8 = 0x06;
const DAC_CTRL: u8 = 0x00;
const DAC_CTRL_ONPADRV: u8 = 7;
const PA_CTRL: u8 = 0x11;
const PA_CTRL_APAGAIN: u8 = 0;
const PA_CTRL_APALP: u8 = 2;
const PA_CTRL_APAON: u8 = 3;
const PA_CTRL_APAPRECH: u8 = 4;
const DAC_AUXG: u8 = 0x0d;
const DAC_CTRL_ONAUXIN: u8 = 6;
const DAC_LLIG: u8 = 0x01;
const DAC_RLIG: u8 = 0x02;
const DAC_RST: u8 = 0x10;
const DAC_PRECH: u8 = 0x0c;
const DAC_CTRL_ONDACL: u8 = 0;
const DAC_CTRL_ONDACR: u8 = 1;
const DAC_CTRL_ONLNOL: u8 = 2;
const DAC_CTRL_ONLNOR: u8 = 3;
const DAC_PRECH_ONMSTR: u8 = 6;

fn snd_at73c213_mixer(chip: *mut snd_at73c213) -> i32 {
    unsafe {
        if chip.is_null() || (*chip).pcm.is_null() {
            return -22; // -EINVAL
        }

        let card = (*chip).card;

        strscpy((*card).mixername.as_mut_ptr(), (*(*chip).pcm).name.as_ptr(), 80);

        // Simplified: not creating controls array here
        // In real code, would iterate through SND_AT73C213_CONTROLS array

        0
    }
}

fn snd_at73c213_ssc_init(chip: *mut snd_at73c213) -> i32 {
    unsafe {
        ssc_writel((*(*chip).ssc).regs, 0x00, // TCMR
                   (1 << 16) | (4 << 8) | (1 << 4) | 15);
        ssc_writel((*(*chip).ssc).regs, 0x08, // TFMR
                   (15 << 16) | (1 << 7) | (1 << 8) | (15 << 24) | (1 << 23));

        0
    }
}

fn snd_at73c213_chip_init(chip: *mut snd_at73c213) -> i32 {
    unsafe {
        let mut retval: i32;
        let mut dac_ctrl: u8 = 0;

        retval = snd_at73c213_set_bitrate(chip);
        if retval != 0 {
            return retval;
        }

        // Enable DAC master clock.
        retval = clk_enable((*(*chip).board).dac_clk);
        if retval != 0 {
            return retval;
        }

        // Initialize at73c213 on SPI bus.
        retval = snd_at73c213_write_reg(chip, DAC_RST, 0x04);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }
        msleep(1);
        retval = snd_at73c213_write_reg(chip, DAC_RST, 0x03);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }

        // Precharge everything.
        retval = snd_at73c213_write_reg(chip, DAC_PRECH, 0xff);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }
        retval = snd_at73c213_write_reg(chip, PA_CTRL, (1 << PA_CTRL_APAPRECH) as u8);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }
        retval = snd_at73c213_write_reg(chip, DAC_CTRL,
                ((1 << DAC_CTRL_ONLNOL) | (1 << DAC_CTRL_ONLNOR)) as u8);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }

        msleep(50);

        // Stop precharging PA.
        retval = snd_at73c213_write_reg(chip, PA_CTRL,
                ((1 << PA_CTRL_APALP) | 0x0f) as u8);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }

        msleep(450);

        // Stop precharging DAC, turn on master power.
        retval = snd_at73c213_write_reg(chip, DAC_PRECH, (1 << DAC_PRECH_ONMSTR) as u8);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }

        msleep(1);

        // Turn on DAC.
        dac_ctrl = ((1 << DAC_CTRL_ONDACL) | (1 << DAC_CTRL_ONDACR)
            | (1 << DAC_CTRL_ONLNOL) | (1 << DAC_CTRL_ONLNOR)) as u8;

        retval = snd_at73c213_write_reg(chip, DAC_CTRL, dac_ctrl);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }

        // Mute sound.
        retval = snd_at73c213_write_reg(chip, DAC_LMPG, 0x3f);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }
        retval = snd_at73c213_write_reg(chip, DAC_RMPG, 0x3f);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }
        retval = snd_at73c213_write_reg(chip, DAC_LLOG, 0x3f);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }
        retval = snd_at73c213_write_reg(chip, DAC_RLOG, 0x3f);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }
        retval = snd_at73c213_write_reg(chip, DAC_LLIG, 0x11);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }
        retval = snd_at73c213_write_reg(chip, DAC_RLIG, 0x11);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }
        retval = snd_at73c213_write_reg(chip, DAC_AUXG, 0x11);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }

        // Enable I2S device, i.e. clock output.
        ssc_writel((*(*chip).ssc).regs, 0x00, 0x00000001); // CR, SSC_BIT(CR_TXEN)

        0
    }
}

extern "C" fn snd_at73c213_dev_free(device: *mut snd_device) -> i32 {
    unsafe {
        let chip = (*device).device_data as *mut snd_at73c213;

        ssc_writel((*(*chip).ssc).regs, 0x00, 0x00000002); // CR, SSC_BIT(CR_TXDIS)
        if (*chip).irq >= 0 {
            free_irq((*chip).irq as u32, chip as *mut core::ffi::c_void);
            (*chip).irq = -1;
        }

        0
    }
}

fn snd_at73c213_dev_init(card: *mut snd_card, _spi: *mut spi_device) -> i32 {
    unsafe {
        let ops = snd_device_ops {
            dev_free: snd_at73c213_dev_free,
        };
        let chip = get_chip(card);
        let irq: i32;
        let mut retval: i32;

        irq = (*(*chip).ssc).irq;
        if irq < 0 {
            return irq;
        }

        spin_lock_init(&mut (*chip).lock);
        mutex_init(&mut (*chip).mixer_lock);
        (*chip).card = card;
        (*chip).irq = -1;

        retval = clk_enable((*(*chip).ssc).clk);
        if retval != 0 {
            return retval;
        }

        retval = request_irq(irq as u32, snd_at73c213_interrupt, 0,
                            b"at73c213\0" as *const u8, chip as *mut core::ffi::c_void);
        if retval != 0 {
            dev_dbg(&(*(*chip).spi).dev, b"unable to request irq %d\n" as *const u8, irq);
            clk_disable((*(*chip).ssc).clk);
            return retval;
        }
        (*chip).irq = irq;

        memcpy((*chip).reg_image.as_mut_ptr() as *mut core::ffi::c_void,
               SND_AT73C213_ORIGINAL_IMAGE.as_ptr() as *const core::ffi::c_void,
               SND_AT73C213_ORIGINAL_IMAGE.len());

        retval = snd_at73c213_ssc_init(chip);
        if retval != 0 {
            free_irq((*chip).irq as u32, chip as *mut core::ffi::c_void);
            (*chip).irq = -1;
            clk_disable((*(*chip).ssc).clk);
            return retval;
        }

        retval = snd_at73c213_chip_init(chip);
        if retval != 0 {
            free_irq((*chip).irq as u32, chip as *mut core::ffi::c_void);
            (*chip).irq = -1;
            clk_disable((*(*chip).ssc).clk);
            return retval;
        }

        retval = snd_at73c213_pcm_new(chip, 0);
        if retval != 0 {
            free_irq((*chip).irq as u32, chip as *mut core::ffi::c_void);
            (*chip).irq = -1;
            clk_disable((*(*chip).ssc).clk);
            return retval;
        }

        retval = snd_device_new(card, 5, chip as *mut core::ffi::c_void, &ops); // SNDRV_DEV_LOWLEVEL
        if retval != 0 {
            snd_device_free(card, chip as *mut core::ffi::c_void);
            free_irq((*chip).irq as u32, chip as *mut core::ffi::c_void);
            (*chip).irq = -1;
            clk_disable((*(*chip).ssc).clk);
            return retval;
        }

        retval = snd_at73c213_mixer(chip);
        if retval != 0 {
            snd_device_free(card, chip as *mut core::ffi::c_void);
            free_irq((*chip).irq as u32, chip as *mut core::ffi::c_void);
            (*chip).irq = -1;
            clk_disable((*(*chip).ssc).clk);
            return retval;
        }

        clk_disable((*(*chip).ssc).clk);
        0
    }
}

#[no_mangle]
pub extern "C" fn snd_at73c213_probe(spi: *mut spi_device) -> i32 {
    unsafe {
        let mut card: *mut snd_card = core::ptr::null_mut();
        let chip: *mut snd_at73c213;
        let board: *mut at73c213_board_info;
        let mut retval: i32;
        let mut id: [u8; 16] = [0; 16];

        board = (*spi).dev._private_data as *mut at73c213_board_info;
        if board.is_null() {
            dev_dbg(&(*spi).dev, b"no platform_data\n" as *const u8);
            return -74; // -ENXIO
        }

        if (*board).dac_clk.is_null() {
            dev_dbg(&(*spi).dev, b"no DAC clk\n" as *const u8);
            return -74; // -ENXIO
        }

        // Note: IS_ERR check would need to be done differently in Rust

        // Allocate "card" using some unused identifiers.
        snprintf(id.as_mut_ptr(), 16, b"at73c213_%d\0" as *const u8,
                 (*board).ssc_id);
        retval = snd_card_new(&(*spi).dev, -1, id.as_ptr(),
                              core::ptr::null(),
                              core::mem::size_of::<snd_at73c213>(),
                              &mut card);
        if retval < 0 {
            return retval;
        }

        chip = (*card)._private_data as *mut snd_at73c213;
        (*chip).spi = spi;
        (*chip).board = board;

        (*chip).ssc = ssc_request((*board).ssc_id);
        if (*chip).ssc.is_null() {
            dev_dbg(&(*spi).dev, b"could not get ssc%d device\n" as *const u8,
                   (*board).ssc_id);
            snd_card_free(card);
            return -74; // -ENXIO
        }

        retval = snd_at73c213_dev_init(card, spi);
        if retval != 0 {
            ssc_free((*chip).ssc);
            snd_card_free(card);
            return retval;
        }

        strscpy((*card).driver.as_mut_ptr(), b"at73c213\0" as *const u8, 16);
        strscpy((*card).shortname.as_mut_ptr(), (*board).shortname, 80);
        sprintf((*card).longname.as_mut_ptr(), b"%s on irq %d\0" as *const u8,
                (*card).shortname.as_ptr(), (*chip).irq);

        retval = snd_card_register(card);
        if retval != 0 {
            ssc_free((*chip).ssc);
            snd_card_free(card);
            return retval;
        }

        dev_set_drvdata(&(*spi).dev, card as *mut core::ffi::c_void);

        0
    }
}

#[no_mangle]
pub extern "C" fn snd_at73c213_remove(spi: *mut spi_device) {
    unsafe {
        let card: *mut snd_card = dev_get_drvdata(&(*spi).dev) as *mut snd_card;
        let chip = (*card)._private_data as *mut snd_at73c213;
        let mut retval: i32;

        // Stop playback.
        retval = clk_enable((*(*chip).ssc).clk);
        if retval == 0 {
            ssc_writel((*(*chip).ssc).regs, 0x00, 0x00000002); // CR, SSC_BIT(CR_TXDIS)
            clk_disable((*(*chip).ssc).clk);
        }

        // Mute sound.
        let _ = snd_at73c213_write_reg(chip, DAC_LMPG, 0x3f);
        let _ = snd_at73c213_write_reg(chip, DAC_RMPG, 0x3f);
        let _ = snd_at73c213_write_reg(chip, DAC_LLOG, 0x3f);
        let _ = snd_at73c213_write_reg(chip, DAC_RLOG, 0x3f);
        let _ = snd_at73c213_write_reg(chip, DAC_LLIG, 0x11);
        let _ = snd_at73c213_write_reg(chip, DAC_RLIG, 0x11);
        let _ = snd_at73c213_write_reg(chip, DAC_AUXG, 0x11);

        // Turn off PA.
        let _ = snd_at73c213_write_reg(chip, PA_CTRL,
                        ((*chip).reg_image[PA_CTRL as usize] | 0x0f) as u8);
        msleep(10);
        let _ = snd_at73c213_write_reg(chip, PA_CTRL,
                        ((1 << PA_CTRL_APALP) | 0x0f) as u8);

        // Turn off external DAC.
        let _ = snd_at73c213_write_reg(chip, DAC_CTRL, 0x0c);
        msleep(2);
        let _ = snd_at73c213_write_reg(chip, DAC_CTRL, 0x00);

        // Turn off master power.
        let _ = snd_at73c213_write_reg(chip, DAC_PRECH, 0x00);

        // Stop DAC master clock.
        clk_disable((*(*chip).board).dac_clk);

        ssc_free((*chip).ssc);
        snd_card_free(card);
    }
}

#[no_mangle]
pub extern "C" fn snd_at73c213_suspend(dev: *mut device) -> i32 {
    unsafe {
        let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
        let chip = (*card)._private_data as *mut snd_at73c213;

        ssc_writel((*(*chip).ssc).regs, 0x00, 0x00000002); // CR, SSC_BIT(CR_TXDIS)
        clk_disable((*(*chip).ssc).clk);
        clk_disable((*(*chip).board).dac_clk);

        0
    }
}

#[no_mangle]
pub extern "C" fn snd_at73c213_resume(dev: *mut device) -> i32 {
    unsafe {
        let card: *mut snd_card = dev_get_drvdata(dev) as *mut snd_card;
        let chip = (*card)._private_data as *mut snd_at73c213;
        let mut retval: i32;

        retval = clk_enable((*(*chip).board).dac_clk);
        if retval != 0 {
            return retval;
        }
        retval = clk_enable((*(*chip).ssc).clk);
        if retval != 0 {
            clk_disable((*(*chip).board).dac_clk);
            return retval;
        }
        ssc_writel((*(*chip).ssc).regs, 0x00, 0x00000001); // CR, SSC_BIT(CR_TXEN)

        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
