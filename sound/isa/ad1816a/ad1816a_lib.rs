// SPDX-License-Identifier: GPL-2.0-or-later
/*
    ad1816a.c - lowlevel code for Analog Devices AD1816A chip.
    Copyright (C) 1999-2000 by Massimo Piccioni <dafastidio@libero.it>

*/

// Translated from the isolated C implementation source. C include dependencies
// are expected to be supplied by the surrounding Rust translation unit.
use crate::*;

unsafe fn snd_ad1816a_busy_wait(chip: *mut snd_ad1816a) -> c_int {
    let mut timeout: c_int;

    timeout = 1000;
    while {
        let old = timeout;
        timeout -= 1;
        old > 0
    } {
        if inb(AD1816A_REG(AD1816A_CHIP_STATUS)) & AD1816A_READY != 0 {
            return 0;
        }
        udelay(10);
    }

    dev_warn((*(*chip).card).dev, c"chip busy.\n".as_ptr());
    -EBUSY
}

unsafe fn snd_ad1816a_in(chip: *mut snd_ad1816a, reg: c_uchar) -> c_uchar {
    snd_ad1816a_busy_wait(chip);
    inb(AD1816A_REG(reg))
}

unsafe fn snd_ad1816a_out(chip: *mut snd_ad1816a, reg: c_uchar, value: c_uchar) {
    snd_ad1816a_busy_wait(chip);
    outb(value, AD1816A_REG(reg));
}

unsafe fn snd_ad1816a_out_mask(
    chip: *mut snd_ad1816a,
    reg: c_uchar,
    mask: c_uchar,
    value: c_uchar,
) {
    snd_ad1816a_out(
        chip,
        reg,
        (value & mask) | (snd_ad1816a_in(chip, reg) & !mask),
    );
}

unsafe fn snd_ad1816a_read(chip: *mut snd_ad1816a, reg: c_uchar) -> c_ushort {
    snd_ad1816a_out(chip, AD1816A_INDIR_ADDR, reg & 0x3f);
    (snd_ad1816a_in(chip, AD1816A_INDIR_DATA_LOW) as c_ushort)
        | ((snd_ad1816a_in(chip, AD1816A_INDIR_DATA_HIGH) as c_ushort) << 8)
}

unsafe fn snd_ad1816a_write(chip: *mut snd_ad1816a, reg: c_uchar, value: c_ushort) {
    snd_ad1816a_out(chip, AD1816A_INDIR_ADDR, reg & 0x3f);
    snd_ad1816a_out(chip, AD1816A_INDIR_DATA_LOW, (value & 0xff) as c_uchar);
    snd_ad1816a_out(
        chip,
        AD1816A_INDIR_DATA_HIGH,
        ((value >> 8) & 0xff) as c_uchar,
    );
}

unsafe fn snd_ad1816a_write_mask(
    chip: *mut snd_ad1816a,
    reg: c_uchar,
    mask: c_ushort,
    value: c_ushort,
) {
    snd_ad1816a_write(
        chip,
        reg,
        (value & mask) | (snd_ad1816a_read(chip, reg) & !mask),
    );
}

unsafe fn snd_ad1816a_get_format(
    _chip: *mut snd_ad1816a,
    format: snd_pcm_format_t,
    channels: c_int,
) -> c_uchar {
    let mut retval: c_uchar = AD1816A_FMT_LINEAR_8;

    match format {
        SNDRV_PCM_FORMAT_MU_LAW => retval = AD1816A_FMT_ULAW_8,
        SNDRV_PCM_FORMAT_A_LAW => retval = AD1816A_FMT_ALAW_8,
        SNDRV_PCM_FORMAT_S16_LE => retval = AD1816A_FMT_LINEAR_16_LIT,
        SNDRV_PCM_FORMAT_S16_BE => retval = AD1816A_FMT_LINEAR_16_BIG,
        _ => {}
    }
    if channels > 1 {
        retval | AD1816A_FMT_STEREO
    } else {
        retval
    }
}

unsafe fn snd_ad1816a_open(chip: *mut snd_ad1816a, mut mode: c_uint) -> c_int {
    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);

    if (*chip).mode & mode != 0 {
        return -EAGAIN;
    }

    mode &= AD1816A_MODE_OPEN;
    match mode {
        AD1816A_MODE_PLAYBACK => {
            snd_ad1816a_out_mask(
                chip,
                AD1816A_INTERRUPT_STATUS,
                AD1816A_PLAYBACK_IRQ_PENDING,
                0x00,
            );
            snd_ad1816a_write_mask(
                chip,
                AD1816A_INTERRUPT_ENABLE,
                AD1816A_PLAYBACK_IRQ_ENABLE,
                0xffff,
            );
        }
        AD1816A_MODE_CAPTURE => {
            snd_ad1816a_out_mask(
                chip,
                AD1816A_INTERRUPT_STATUS,
                AD1816A_CAPTURE_IRQ_PENDING,
                0x00,
            );
            snd_ad1816a_write_mask(
                chip,
                AD1816A_INTERRUPT_ENABLE,
                AD1816A_CAPTURE_IRQ_ENABLE,
                0xffff,
            );
        }
        AD1816A_MODE_TIMER => {
            snd_ad1816a_out_mask(
                chip,
                AD1816A_INTERRUPT_STATUS,
                AD1816A_TIMER_IRQ_PENDING,
                0x00,
            );
            snd_ad1816a_write_mask(
                chip,
                AD1816A_INTERRUPT_ENABLE,
                AD1816A_TIMER_IRQ_ENABLE,
                0xffff,
            );
        }
        _ => {}
    }
    (*chip).mode |= mode;

    0
}

unsafe fn snd_ad1816a_close(chip: *mut snd_ad1816a, mut mode: c_uint) {
    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);

    mode &= AD1816A_MODE_OPEN;
    match mode {
        AD1816A_MODE_PLAYBACK => {
            snd_ad1816a_out_mask(
                chip,
                AD1816A_INTERRUPT_STATUS,
                AD1816A_PLAYBACK_IRQ_PENDING,
                0x00,
            );
            snd_ad1816a_write_mask(
                chip,
                AD1816A_INTERRUPT_ENABLE,
                AD1816A_PLAYBACK_IRQ_ENABLE,
                0x0000,
            );
        }
        AD1816A_MODE_CAPTURE => {
            snd_ad1816a_out_mask(
                chip,
                AD1816A_INTERRUPT_STATUS,
                AD1816A_CAPTURE_IRQ_PENDING,
                0x00,
            );
            snd_ad1816a_write_mask(
                chip,
                AD1816A_INTERRUPT_ENABLE,
                AD1816A_CAPTURE_IRQ_ENABLE,
                0x0000,
            );
        }
        AD1816A_MODE_TIMER => {
            snd_ad1816a_out_mask(
                chip,
                AD1816A_INTERRUPT_STATUS,
                AD1816A_TIMER_IRQ_PENDING,
                0x00,
            );
            snd_ad1816a_write_mask(
                chip,
                AD1816A_INTERRUPT_ENABLE,
                AD1816A_TIMER_IRQ_ENABLE,
                0x0000,
            );
        }
        _ => {}
    }
    (*chip).mode &= !mode;
    if (*chip).mode & AD1816A_MODE_OPEN == 0 {
        (*chip).mode = 0;
    }
}

unsafe fn snd_ad1816a_trigger(
    chip: *mut snd_ad1816a,
    what: c_uchar,
    _channel: c_int,
    mut cmd: c_int,
    iscapture: c_int,
) -> c_int {
    let mut error: c_int = 0;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_STOP => {
            let _guard = spinlock_guard(&mut (*chip).lock);
            cmd = if cmd == SNDRV_PCM_TRIGGER_START { 0xff } else { 0x00 };
            /* if (what & AD1816A_PLAYBACK_ENABLE) */
            /* That is not valid, because playback and capture enable
             * are the same bit pattern, just to different addresses
             */
            if iscapture == 0 {
                snd_ad1816a_out_mask(
                    chip,
                    AD1816A_PLAYBACK_CONFIG,
                    AD1816A_PLAYBACK_ENABLE,
                    cmd as c_uchar,
                );
            } else {
                snd_ad1816a_out_mask(
                    chip,
                    AD1816A_CAPTURE_CONFIG,
                    AD1816A_CAPTURE_ENABLE,
                    cmd as c_uchar,
                );
            }
        }
        _ => {
            dev_warn(
                (*(*chip).card).dev,
                c"invalid trigger mode 0x%x.\n".as_ptr(),
                what as c_uint,
            );
            error = -EINVAL;
        }
    }

    error
}

unsafe extern "C" fn snd_ad1816a_playback_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let chip: *mut snd_ad1816a = snd_pcm_substream_chip(substream);
    snd_ad1816a_trigger(chip, AD1816A_PLAYBACK_ENABLE, SNDRV_PCM_STREAM_PLAYBACK, cmd, 0)
}

unsafe extern "C" fn snd_ad1816a_capture_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let chip: *mut snd_ad1816a = snd_pcm_substream_chip(substream);
    snd_ad1816a_trigger(chip, AD1816A_CAPTURE_ENABLE, SNDRV_PCM_STREAM_CAPTURE, cmd, 1)
}

unsafe extern "C" fn snd_ad1816a_playback_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1816a = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut size: c_uint;
    let mut rate: c_uint;

    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);

    size = snd_pcm_lib_buffer_bytes(substream) as c_uint;
    (*chip).p_dma_size = size;
    snd_ad1816a_out_mask(
        chip,
        AD1816A_PLAYBACK_CONFIG,
        AD1816A_PLAYBACK_ENABLE | AD1816A_PLAYBACK_PIO,
        0x00,
    );

    snd_dma_program(
        (*chip).dma1,
        (*runtime).dma_addr,
        size,
        DMA_MODE_WRITE | DMA_AUTOINIT,
    );

    rate = (*runtime).rate;
    if (*chip).clock_freq != 0 {
        rate = (rate * 33000) / (*chip).clock_freq;
    }
    snd_ad1816a_write(chip, AD1816A_PLAYBACK_SAMPLE_RATE, rate as c_ushort);
    snd_ad1816a_out_mask(
        chip,
        AD1816A_PLAYBACK_CONFIG,
        AD1816A_FMT_ALL | AD1816A_FMT_STEREO,
        snd_ad1816a_get_format(chip, (*runtime).format, (*runtime).channels),
    );

    snd_ad1816a_write(
        chip,
        AD1816A_PLAYBACK_BASE_COUNT,
        (snd_pcm_lib_period_bytes(substream) / 4 - 1) as c_ushort,
    );
    0
}

unsafe extern "C" fn snd_ad1816a_capture_prepare(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1816a = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut size: c_uint;
    let mut rate: c_uint;

    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);

    size = snd_pcm_lib_buffer_bytes(substream) as c_uint;
    (*chip).c_dma_size = size;
    snd_ad1816a_out_mask(
        chip,
        AD1816A_CAPTURE_CONFIG,
        AD1816A_CAPTURE_ENABLE | AD1816A_CAPTURE_PIO,
        0x00,
    );

    snd_dma_program(
        (*chip).dma2,
        (*runtime).dma_addr,
        size,
        DMA_MODE_READ | DMA_AUTOINIT,
    );

    rate = (*runtime).rate;
    if (*chip).clock_freq != 0 {
        rate = (rate * 33000) / (*chip).clock_freq;
    }
    snd_ad1816a_write(chip, AD1816A_CAPTURE_SAMPLE_RATE, rate as c_ushort);
    snd_ad1816a_out_mask(
        chip,
        AD1816A_CAPTURE_CONFIG,
        AD1816A_FMT_ALL | AD1816A_FMT_STEREO,
        snd_ad1816a_get_format(chip, (*runtime).format, (*runtime).channels),
    );

    snd_ad1816a_write(
        chip,
        AD1816A_CAPTURE_BASE_COUNT,
        (snd_pcm_lib_period_bytes(substream) / 4 - 1) as c_ushort,
    );
    0
}

unsafe extern "C" fn snd_ad1816a_playback_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let chip: *mut snd_ad1816a = snd_pcm_substream_chip(substream);
    let ptr: size_t;
    if (*chip).mode & AD1816A_MODE_PLAYBACK == 0 {
        return 0;
    }
    ptr = snd_dma_pointer((*chip).dma1, (*chip).p_dma_size);
    bytes_to_frames((*substream).runtime, ptr)
}

unsafe extern "C" fn snd_ad1816a_capture_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let chip: *mut snd_ad1816a = snd_pcm_substream_chip(substream);
    let ptr: size_t;
    if (*chip).mode & AD1816A_MODE_CAPTURE == 0 {
        return 0;
    }
    ptr = snd_dma_pointer((*chip).dma2, (*chip).c_dma_size);
    bytes_to_frames((*substream).runtime, ptr)
}

unsafe extern "C" fn snd_ad1816a_interrupt(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let chip: *mut snd_ad1816a = dev_id as *mut snd_ad1816a;
    let status: c_uchar;

    {
        let _guard = spinlock_guard(&mut (*chip).lock);
        status = snd_ad1816a_in(chip, AD1816A_INTERRUPT_STATUS);
    }

    if status & AD1816A_PLAYBACK_IRQ_PENDING != 0 && !(*chip).playback_substream.is_null() {
        snd_pcm_period_elapsed((*chip).playback_substream);
    }

    if status & AD1816A_CAPTURE_IRQ_PENDING != 0 && !(*chip).capture_substream.is_null() {
        snd_pcm_period_elapsed((*chip).capture_substream);
    }

    if status & AD1816A_TIMER_IRQ_PENDING != 0 && !(*chip).timer.is_null() {
        snd_timer_interrupt((*chip).timer, (*(*chip).timer).sticks);
    }

    {
        let _guard = spinlock_guard(&mut (*chip).lock);
        snd_ad1816a_out(chip, AD1816A_INTERRUPT_STATUS, 0x00);
    }
    IRQ_HANDLED
}

static snd_ad1816a_playback: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_MU_LAW
        | SNDRV_PCM_FMTBIT_A_LAW
        | SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S16_BE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 55200,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

static snd_ad1816a_capture: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_MU_LAW
        | SNDRV_PCM_FMTBIT_A_LAW
        | SNDRV_PCM_FMTBIT_U8
        | SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S16_BE,
    rates: SNDRV_PCM_RATE_CONTINUOUS | SNDRV_PCM_RATE_8000_48000,
    rate_min: 4000,
    rate_max: 55200,
    channels_min: 1,
    channels_max: 2,
    buffer_bytes_max: 128 * 1024,
    period_bytes_min: 64,
    period_bytes_max: 128 * 1024,
    periods_min: 1,
    periods_max: 1024,
    fifo_size: 0,
};

unsafe extern "C" fn snd_ad1816a_timer_close(timer: *mut snd_timer) -> c_int {
    let chip: *mut snd_ad1816a = snd_timer_chip(timer);
    snd_ad1816a_close(chip, AD1816A_MODE_TIMER);
    0
}

unsafe extern "C" fn snd_ad1816a_timer_open(timer: *mut snd_timer) -> c_int {
    let chip: *mut snd_ad1816a = snd_timer_chip(timer);
    snd_ad1816a_open(chip, AD1816A_MODE_TIMER);
    0
}

unsafe extern "C" fn snd_ad1816a_timer_resolution(timer: *mut snd_timer) -> c_ulong {
    if snd_BUG_ON(timer.is_null()) != 0 {
        return 0;
    }

    10000
}

unsafe extern "C" fn snd_ad1816a_timer_start(timer: *mut snd_timer) -> c_int {
    let bits: c_ushort;
    let chip: *mut snd_ad1816a = snd_timer_chip(timer);

    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);
    bits = snd_ad1816a_read(chip, AD1816A_INTERRUPT_ENABLE);

    if bits & AD1816A_TIMER_ENABLE == 0 {
        snd_ad1816a_write(chip, AD1816A_TIMER_BASE_COUNT, ((*timer).sticks & 0xffff) as c_ushort);

        snd_ad1816a_write_mask(chip, AD1816A_INTERRUPT_ENABLE, AD1816A_TIMER_ENABLE, 0xffff);
    }
    0
}

unsafe extern "C" fn snd_ad1816a_timer_stop(timer: *mut snd_timer) -> c_int {
    let chip: *mut snd_ad1816a = snd_timer_chip(timer);

    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);
    snd_ad1816a_write_mask(chip, AD1816A_INTERRUPT_ENABLE, AD1816A_TIMER_ENABLE, 0x0000);
    0
}

static snd_ad1816a_timer_table: snd_timer_hardware = snd_timer_hardware {
    flags: SNDRV_TIMER_HW_AUTO,
    resolution: 10000,
    ticks: 65535,
    open: Some(snd_ad1816a_timer_open),
    close: Some(snd_ad1816a_timer_close),
    c_resolution: Some(snd_ad1816a_timer_resolution),
    start: Some(snd_ad1816a_timer_start),
    stop: Some(snd_ad1816a_timer_stop),
};

unsafe extern "C" fn snd_ad1816a_playback_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1816a = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut error: c_int;

    error = snd_ad1816a_open(chip, AD1816A_MODE_PLAYBACK);
    if error < 0 {
        return error;
    }
    (*runtime).hw = snd_ad1816a_playback;
    snd_pcm_limit_isa_dma_size((*chip).dma1, &mut (*runtime).hw.buffer_bytes_max);
    snd_pcm_limit_isa_dma_size((*chip).dma1, &mut (*runtime).hw.period_bytes_max);
    (*chip).playback_substream = substream;
    0
}

unsafe extern "C" fn snd_ad1816a_capture_open(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1816a = snd_pcm_substream_chip(substream);
    let runtime: *mut snd_pcm_runtime = (*substream).runtime;
    let mut error: c_int;

    error = snd_ad1816a_open(chip, AD1816A_MODE_CAPTURE);
    if error < 0 {
        return error;
    }
    (*runtime).hw = snd_ad1816a_capture;
    snd_pcm_limit_isa_dma_size((*chip).dma2, &mut (*runtime).hw.buffer_bytes_max);
    snd_pcm_limit_isa_dma_size((*chip).dma2, &mut (*runtime).hw.period_bytes_max);
    (*chip).capture_substream = substream;
    0
}

unsafe extern "C" fn snd_ad1816a_playback_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1816a = snd_pcm_substream_chip(substream);

    (*chip).playback_substream = core::ptr::null_mut();
    snd_ad1816a_close(chip, AD1816A_MODE_PLAYBACK);
    0
}

unsafe extern "C" fn snd_ad1816a_capture_close(substream: *mut snd_pcm_substream) -> c_int {
    let chip: *mut snd_ad1816a = snd_pcm_substream_chip(substream);

    (*chip).capture_substream = core::ptr::null_mut();
    snd_ad1816a_close(chip, AD1816A_MODE_CAPTURE);
    0
}

unsafe fn snd_ad1816a_init(chip: *mut snd_ad1816a) {
    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);

    snd_ad1816a_out(chip, AD1816A_INTERRUPT_STATUS, 0x00);
    snd_ad1816a_out_mask(
        chip,
        AD1816A_PLAYBACK_CONFIG,
        AD1816A_PLAYBACK_ENABLE | AD1816A_PLAYBACK_PIO,
        0x00,
    );
    snd_ad1816a_out_mask(
        chip,
        AD1816A_CAPTURE_CONFIG,
        AD1816A_CAPTURE_ENABLE | AD1816A_CAPTURE_PIO,
        0x00,
    );
    snd_ad1816a_write(chip, AD1816A_INTERRUPT_ENABLE, 0x0000);
    snd_ad1816a_write_mask(
        chip,
        AD1816A_CHIP_CONFIG,
        AD1816A_CAPTURE_NOT_EQUAL | AD1816A_WSS_ENABLE,
        0xffff,
    );
    snd_ad1816a_write(chip, AD1816A_DSP_CONFIG, 0x0000);
    snd_ad1816a_write(chip, AD1816A_POWERDOWN_CTRL, 0x0000);
}

// C conditional: #ifdef CONFIG_PM
#[cfg(CONFIG_PM)]
pub unsafe extern "C" fn snd_ad1816a_suspend(chip: *mut snd_ad1816a) {
    let mut reg: c_int;

    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);
    reg = 0;
    while reg < 48 {
        (*chip).image[reg as usize] = snd_ad1816a_read(chip, reg as c_uchar);
        reg += 1;
    }
}

#[cfg(CONFIG_PM)]
pub unsafe extern "C" fn snd_ad1816a_resume(chip: *mut snd_ad1816a) {
    let mut reg: c_int;

    snd_ad1816a_init(chip);
    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);
    reg = 0;
    while reg < 48 {
        snd_ad1816a_write(chip, reg as c_uchar, (*chip).image[reg as usize]);
        reg += 1;
    }
}

unsafe fn snd_ad1816a_probe(chip: *mut snd_ad1816a) -> c_int {
    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);

    (*chip).version = snd_ad1816a_read(chip, AD1816A_VERSION_ID);
    match (*chip).version {
        0 => (*chip).hardware = AD1816A_HW_AD1815,
        1 => (*chip).hardware = AD1816A_HW_AD18MAX10,
        3 => (*chip).hardware = AD1816A_HW_AD1816A,
        _ => (*chip).hardware = AD1816A_HW_AUTO,
    }
    0
}

unsafe fn snd_ad1816a_chip_id(chip: *mut snd_ad1816a) -> *const c_char {
    match (*chip).hardware {
        AD1816A_HW_AD1816A => c"AD1816A".as_ptr(),
        AD1816A_HW_AD1815 => c"AD1815".as_ptr(),
        AD1816A_HW_AD18MAX10 => c"AD18max10".as_ptr(),
        _ => {
            dev_warn(
                (*(*chip).card).dev,
                c"Unknown chip version %d:%d.\n".as_ptr(),
                (*chip).version as c_int,
                (*chip).hardware as c_int,
            );
            c"AD1816A - unknown".as_ptr()
        }
    }
}

pub unsafe extern "C" fn snd_ad1816a_create(
    card: *mut snd_card,
    port: c_ulong,
    irq: c_int,
    dma1: c_int,
    dma2: c_int,
    chip: *mut snd_ad1816a,
) -> c_int {
    let mut error: c_int;

    (*chip).irq = -1;
    (*chip).dma1 = -1;
    (*chip).dma2 = -1;

    (*chip).res_port = devm_request_region((*card).dev, port, 16, c"AD1816A".as_ptr());
    if (*chip).res_port.is_null() {
        dev_err(
            (*card).dev,
            c"ad1816a: can't grab port 0x%lx\n".as_ptr(),
            port,
        );
        return -EBUSY;
    }
    if devm_request_irq(
        (*card).dev,
        irq,
        Some(snd_ad1816a_interrupt),
        0,
        c"AD1816A".as_ptr(),
        chip as *mut c_void,
    ) != 0
    {
        dev_err((*card).dev, c"ad1816a: can't grab IRQ %d\n".as_ptr(), irq);
        return -EBUSY;
    }
    (*chip).irq = irq;
    (*card).sync_irq = (*chip).irq;
    if snd_devm_request_dma((*card).dev, dma1, c"AD1816A - 1".as_ptr()) != 0 {
        dev_err((*card).dev, c"ad1816a: can't grab DMA1 %d\n".as_ptr(), dma1);
        return -EBUSY;
    }
    (*chip).dma1 = dma1;
    if snd_devm_request_dma((*card).dev, dma2, c"AD1816A - 2".as_ptr()) != 0 {
        dev_err((*card).dev, c"ad1816a: can't grab DMA2 %d\n".as_ptr(), dma2);
        return -EBUSY;
    }
    (*chip).dma2 = dma2;

    (*chip).card = card;
    (*chip).port = port;
    spin_lock_init(&mut (*chip).lock);

    error = snd_ad1816a_probe(chip);
    if error != 0 {
        return error;
    }

    snd_ad1816a_init(chip);

    0
}

static snd_ad1816a_playback_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_ad1816a_playback_open),
    close: Some(snd_ad1816a_playback_close),
    prepare: Some(snd_ad1816a_playback_prepare),
    trigger: Some(snd_ad1816a_playback_trigger),
    pointer: Some(snd_ad1816a_playback_pointer),
};

static snd_ad1816a_capture_ops: snd_pcm_ops = snd_pcm_ops {
    open: Some(snd_ad1816a_capture_open),
    close: Some(snd_ad1816a_capture_close),
    prepare: Some(snd_ad1816a_capture_prepare),
    trigger: Some(snd_ad1816a_capture_trigger),
    pointer: Some(snd_ad1816a_capture_pointer),
};

pub unsafe extern "C" fn snd_ad1816a_pcm(chip: *mut snd_ad1816a, device: c_int) -> c_int {
    let mut error: c_int;
    let mut pcm: *mut snd_pcm = core::ptr::null_mut();

    error = snd_pcm_new((*chip).card, c"AD1816A".as_ptr(), device, 1, 1, &mut pcm);
    if error != 0 {
        return error;
    }

    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &snd_ad1816a_playback_ops);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &snd_ad1816a_capture_ops);

    (*pcm).private_data = chip as *mut c_void;
    (*pcm).info_flags = if (*chip).dma1 == (*chip).dma2 {
        SNDRV_PCM_INFO_JOINT_DUPLEX
    } else {
        0
    };

    strscpy((*pcm).name.as_mut_ptr(), snd_ad1816a_chip_id(chip));
    snd_ad1816a_init(chip);

    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        (*(*chip).card).dev,
        64 * 1024,
        if (*chip).dma1 > 3 || (*chip).dma2 > 3 {
            128 * 1024
        } else {
            64 * 1024
        },
    );

    (*chip).pcm = pcm;
    0
}

pub unsafe extern "C" fn snd_ad1816a_timer(chip: *mut snd_ad1816a, device: c_int) -> c_int {
    let mut timer: *mut snd_timer = core::ptr::null_mut();
    let mut tid: snd_timer_id = core::mem::zeroed();
    let mut error: c_int;

    tid.dev_class = SNDRV_TIMER_CLASS_CARD;
    tid.dev_sclass = SNDRV_TIMER_SCLASS_NONE;
    tid.card = (*(*chip).card).number;
    tid.device = device;
    tid.subdevice = 0;
    error = snd_timer_new((*chip).card, c"AD1816A".as_ptr(), &mut tid, &mut timer);
    if error < 0 {
        return error;
    }
    strscpy((*timer).name.as_mut_ptr(), snd_ad1816a_chip_id(chip));
    (*timer).private_data = chip as *mut c_void;
    (*chip).timer = timer;
    (*timer).hw = snd_ad1816a_timer_table;
    0
}

/*
 *
 */

unsafe extern "C" fn snd_ad1816a_info_mux(
    _kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    static texts: [*const c_char; 8] = [
        c"Line".as_ptr(),
        c"Mix".as_ptr(),
        c"CD".as_ptr(),
        c"Synth".as_ptr(),
        c"Video".as_ptr(),
        c"Mic".as_ptr(),
        c"Phone".as_ptr(),
        core::ptr::null(),
    ];

    snd_ctl_enum_info(uinfo, 2, 7, texts.as_ptr())
}

unsafe extern "C" fn snd_ad1816a_get_mux(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_ad1816a = snd_kcontrol_chip(kcontrol);
    let val: c_ushort;

    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);
    val = snd_ad1816a_read(chip, AD1816A_ADC_SOURCE_SEL);
    (*ucontrol).value.enumerated.item[0] = ((val >> 12) & 7) as c_uint;
    (*ucontrol).value.enumerated.item[1] = ((val >> 4) & 7) as c_uint;
    0
}

unsafe extern "C" fn snd_ad1816a_put_mux(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_ad1816a = snd_kcontrol_chip(kcontrol);
    let val: c_ushort;
    let change: c_int;

    if (*ucontrol).value.enumerated.item[0] > 6 || (*ucontrol).value.enumerated.item[1] > 6 {
        return -EINVAL;
    }
    val = (((*ucontrol).value.enumerated.item[0] << 12)
        | ((*ucontrol).value.enumerated.item[1] << 4)) as c_ushort;
    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);
    change = (snd_ad1816a_read(chip, AD1816A_ADC_SOURCE_SEL) != val) as c_int;
    snd_ad1816a_write(chip, AD1816A_ADC_SOURCE_SEL, val);
    change
}

const fn AD1816A_SINGLE_TLV(
    xname: *const c_char,
    reg: c_ulong,
    shift: c_ulong,
    mask: c_ulong,
    invert: c_ulong,
    xtlv: *const c_uint,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        name: xname,
        info: Some(snd_ad1816a_info_single),
        get: Some(snd_ad1816a_get_single),
        put: Some(snd_ad1816a_put_single),
        private_value: reg | (shift << 8) | (mask << 16) | (invert << 24),
        tlv: snd_kcontrol_new_tlv { p: xtlv },
    }
}

const fn AD1816A_SINGLE(
    xname: *const c_char,
    reg: c_ulong,
    shift: c_ulong,
    mask: c_ulong,
    invert: c_ulong,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        info: Some(snd_ad1816a_info_single),
        get: Some(snd_ad1816a_get_single),
        put: Some(snd_ad1816a_put_single),
        private_value: reg | (shift << 8) | (mask << 16) | (invert << 24),
        ..unsafe { core::mem::zeroed() }
    }
}

unsafe extern "C" fn snd_ad1816a_info_single(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let mask: c_int = (((*kcontrol).private_value >> 16) & 0xff) as c_int;

    (*uinfo).type_ = if mask == 1 {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN
    } else {
        SNDRV_CTL_ELEM_TYPE_INTEGER
    };
    (*uinfo).count = 1;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_ad1816a_get_single(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_ad1816a = snd_kcontrol_chip(kcontrol);
    let reg: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let shift: c_int = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let mask: c_int = (((*kcontrol).private_value >> 16) & 0xff) as c_int;
    let invert: c_int = (((*kcontrol).private_value >> 24) & 0xff) as c_int;

    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);
    (*ucontrol).value.integer.value[0] =
        ((snd_ad1816a_read(chip, reg as c_uchar) >> shift) & mask as c_ushort) as c_long;
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = mask as c_long - (*ucontrol).value.integer.value[0];
    }
    0
}

unsafe extern "C" fn snd_ad1816a_put_single(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_ad1816a = snd_kcontrol_chip(kcontrol);
    let reg: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let shift: c_int = (((*kcontrol).private_value >> 8) & 0xff) as c_int;
    let mask: c_int = (((*kcontrol).private_value >> 16) & 0xff) as c_int;
    let invert: c_int = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let change: c_int;
    let old_val: c_ushort;
    let mut val: c_ushort;

    val = ((*ucontrol).value.integer.value[0] as c_ushort) & mask as c_ushort;
    if invert != 0 {
        val = mask as c_ushort - val;
    }
    val <<= shift;
    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);
    old_val = snd_ad1816a_read(chip, reg as c_uchar);
    val = (old_val & !((mask as c_ushort) << shift)) | val;
    change = (val != old_val) as c_int;
    snd_ad1816a_write(chip, reg as c_uchar, val);
    change
}

const fn AD1816A_DOUBLE_TLV(
    xname: *const c_char,
    reg: c_ulong,
    shift_left: c_ulong,
    shift_right: c_ulong,
    mask: c_ulong,
    invert: c_ulong,
    xtlv: *const c_uint,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        access: SNDRV_CTL_ELEM_ACCESS_READWRITE | SNDRV_CTL_ELEM_ACCESS_TLV_READ,
        name: xname,
        info: Some(snd_ad1816a_info_double),
        get: Some(snd_ad1816a_get_double),
        put: Some(snd_ad1816a_put_double),
        private_value: reg | (shift_left << 8) | (shift_right << 12) | (mask << 16) | (invert << 24),
        tlv: snd_kcontrol_new_tlv { p: xtlv },
    }
}

const fn AD1816A_DOUBLE(
    xname: *const c_char,
    reg: c_ulong,
    shift_left: c_ulong,
    shift_right: c_ulong,
    mask: c_ulong,
    invert: c_ulong,
) -> snd_kcontrol_new {
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: xname,
        info: Some(snd_ad1816a_info_double),
        get: Some(snd_ad1816a_get_double),
        put: Some(snd_ad1816a_put_double),
        private_value: reg | (shift_left << 8) | (shift_right << 12) | (mask << 16) | (invert << 24),
        ..unsafe { core::mem::zeroed() }
    }
}

unsafe extern "C" fn snd_ad1816a_info_double(
    kcontrol: *mut snd_kcontrol,
    uinfo: *mut snd_ctl_elem_info,
) -> c_int {
    let mask: c_int = (((*kcontrol).private_value >> 16) & 0xff) as c_int;

    (*uinfo).type_ = if mask == 1 {
        SNDRV_CTL_ELEM_TYPE_BOOLEAN
    } else {
        SNDRV_CTL_ELEM_TYPE_INTEGER
    };
    (*uinfo).count = 2;
    (*uinfo).value.integer.min = 0;
    (*uinfo).value.integer.max = mask as c_long;
    0
}

unsafe extern "C" fn snd_ad1816a_get_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_ad1816a = snd_kcontrol_chip(kcontrol);
    let reg: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let shift_left: c_int = (((*kcontrol).private_value >> 8) & 0x0f) as c_int;
    let shift_right: c_int = (((*kcontrol).private_value >> 12) & 0x0f) as c_int;
    let mask: c_int = (((*kcontrol).private_value >> 16) & 0xff) as c_int;
    let invert: c_int = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let val: c_ushort;

    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);
    val = snd_ad1816a_read(chip, reg as c_uchar);
    (*ucontrol).value.integer.value[0] = ((val >> shift_left) & mask as c_ushort) as c_long;
    (*ucontrol).value.integer.value[1] = ((val >> shift_right) & mask as c_ushort) as c_long;
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = mask as c_long - (*ucontrol).value.integer.value[0];
        (*ucontrol).value.integer.value[1] = mask as c_long - (*ucontrol).value.integer.value[1];
    }
    0
}

unsafe extern "C" fn snd_ad1816a_put_double(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let chip: *mut snd_ad1816a = snd_kcontrol_chip(kcontrol);
    let reg: c_int = ((*kcontrol).private_value & 0xff) as c_int;
    let shift_left: c_int = (((*kcontrol).private_value >> 8) & 0x0f) as c_int;
    let shift_right: c_int = (((*kcontrol).private_value >> 12) & 0x0f) as c_int;
    let mask: c_int = (((*kcontrol).private_value >> 16) & 0xff) as c_int;
    let invert: c_int = (((*kcontrol).private_value >> 24) & 0xff) as c_int;
    let change: c_int;
    let old_val: c_ushort;
    let mut val1: c_ushort;
    let mut val2: c_ushort;

    val1 = ((*ucontrol).value.integer.value[0] as c_ushort) & mask as c_ushort;
    val2 = ((*ucontrol).value.integer.value[1] as c_ushort) & mask as c_ushort;
    if invert != 0 {
        val1 = mask as c_ushort - val1;
        val2 = mask as c_ushort - val2;
    }
    val1 <<= shift_left;
    val2 <<= shift_right;
    let _guard = spinlock_irqsave_guard(&mut (*chip).lock);
    old_val = snd_ad1816a_read(chip, reg as c_uchar);
    val1 = (old_val & !(((mask as c_ushort) << shift_left) | ((mask as c_ushort) << shift_right)))
        | val1
        | val2;
    change = (val1 != old_val) as c_int;
    snd_ad1816a_write(chip, reg as c_uchar, val1);
    change
}

static db_scale_4bit: [c_uint; 4] = DECLARE_TLV_DB_SCALE(-4500, 300, 0);
static db_scale_5bit: [c_uint; 4] = DECLARE_TLV_DB_SCALE(-4650, 150, 0);
static db_scale_6bit: [c_uint; 4] = DECLARE_TLV_DB_SCALE(-9450, 150, 0);
static db_scale_5bit_12db_max: [c_uint; 4] = DECLARE_TLV_DB_SCALE(-3450, 150, 0);
static db_scale_rec_gain: [c_uint; 4] = DECLARE_TLV_DB_SCALE(0, 150, 0);

static snd_ad1816a_controls: [snd_kcontrol_new; 27] = [
    AD1816A_DOUBLE(c"Master Playback Switch".as_ptr(), AD1816A_MASTER_ATT, 15, 7, 1, 1),
    AD1816A_DOUBLE_TLV(c"Master Playback Volume".as_ptr(), AD1816A_MASTER_ATT, 8, 0, 31, 1, db_scale_5bit.as_ptr()),
    AD1816A_DOUBLE(c"PCM Playback Switch".as_ptr(), AD1816A_VOICE_ATT, 15, 7, 1, 1),
    AD1816A_DOUBLE_TLV(c"PCM Playback Volume".as_ptr(), AD1816A_VOICE_ATT, 8, 0, 63, 1, db_scale_6bit.as_ptr()),
    AD1816A_DOUBLE(c"Line Playback Switch".as_ptr(), AD1816A_LINE_GAIN_ATT, 15, 7, 1, 1),
    AD1816A_DOUBLE_TLV(c"Line Playback Volume".as_ptr(), AD1816A_LINE_GAIN_ATT, 8, 0, 31, 1, db_scale_5bit_12db_max.as_ptr()),
    AD1816A_DOUBLE(c"CD Playback Switch".as_ptr(), AD1816A_CD_GAIN_ATT, 15, 7, 1, 1),
    AD1816A_DOUBLE_TLV(c"CD Playback Volume".as_ptr(), AD1816A_CD_GAIN_ATT, 8, 0, 31, 1, db_scale_5bit_12db_max.as_ptr()),
    AD1816A_DOUBLE(c"Synth Playback Switch".as_ptr(), AD1816A_SYNTH_GAIN_ATT, 15, 7, 1, 1),
    AD1816A_DOUBLE_TLV(c"Synth Playback Volume".as_ptr(), AD1816A_SYNTH_GAIN_ATT, 8, 0, 31, 1, db_scale_5bit_12db_max.as_ptr()),
    AD1816A_DOUBLE(c"FM Playback Switch".as_ptr(), AD1816A_FM_ATT, 15, 7, 1, 1),
    AD1816A_DOUBLE_TLV(c"FM Playback Volume".as_ptr(), AD1816A_FM_ATT, 8, 0, 63, 1, db_scale_6bit.as_ptr()),
    AD1816A_SINGLE(c"Mic Playback Switch".as_ptr(), AD1816A_MIC_GAIN_ATT, 15, 1, 1),
    AD1816A_SINGLE_TLV(c"Mic Playback Volume".as_ptr(), AD1816A_MIC_GAIN_ATT, 8, 31, 1, db_scale_5bit_12db_max.as_ptr()),
    AD1816A_SINGLE(c"Mic Boost".as_ptr(), AD1816A_MIC_GAIN_ATT, 14, 1, 0),
    AD1816A_DOUBLE(c"Video Playback Switch".as_ptr(), AD1816A_VID_GAIN_ATT, 15, 7, 1, 1),
    AD1816A_DOUBLE_TLV(c"Video Playback Volume".as_ptr(), AD1816A_VID_GAIN_ATT, 8, 0, 31, 1, db_scale_5bit_12db_max.as_ptr()),
    AD1816A_SINGLE(c"Phone Capture Switch".as_ptr(), AD1816A_PHONE_IN_GAIN_ATT, 15, 1, 1),
    AD1816A_SINGLE_TLV(c"Phone Capture Volume".as_ptr(), AD1816A_PHONE_IN_GAIN_ATT, 0, 15, 1, db_scale_4bit.as_ptr()),
    AD1816A_SINGLE(c"Phone Playback Switch".as_ptr(), AD1816A_PHONE_OUT_ATT, 7, 1, 1),
    AD1816A_SINGLE_TLV(c"Phone Playback Volume".as_ptr(), AD1816A_PHONE_OUT_ATT, 0, 31, 1, db_scale_5bit.as_ptr()),
    snd_kcontrol_new {
        iface: SNDRV_CTL_ELEM_IFACE_MIXER,
        name: c"Capture Source".as_ptr(),
        info: Some(snd_ad1816a_info_mux),
        get: Some(snd_ad1816a_get_mux),
        put: Some(snd_ad1816a_put_mux),
        ..unsafe { core::mem::zeroed() }
    },
    AD1816A_DOUBLE(c"Capture Switch".as_ptr(), AD1816A_ADC_PGA, 15, 7, 1, 1),
    AD1816A_DOUBLE_TLV(c"Capture Volume".as_ptr(), AD1816A_ADC_PGA, 8, 0, 15, 0, db_scale_rec_gain.as_ptr()),
    AD1816A_SINGLE(c"3D Control - Switch".as_ptr(), AD1816A_3D_PHAT_CTRL, 15, 1, 1),
    AD1816A_SINGLE(c"3D Control - Level".as_ptr(), AD1816A_3D_PHAT_CTRL, 0, 15, 0),
];

pub unsafe extern "C" fn snd_ad1816a_mixer(chip: *mut snd_ad1816a) -> c_int {
    let card: *mut snd_card;
    let mut idx: c_uint;
    let mut err: c_int;

    if snd_BUG_ON(chip.is_null() || (*chip).card.is_null()) != 0 {
        return -EINVAL;
    }

    card = (*chip).card;

    strscpy((*card).mixername.as_mut_ptr(), snd_ad1816a_chip_id(chip));

    idx = 0;
    while (idx as usize) < snd_ad1816a_controls.len() {
        err = snd_ctl_add(
            card,
            snd_ctl_new1(&snd_ad1816a_controls[idx as usize], chip as *mut c_void),
        );
        if err < 0 {
            return err;
        }
        idx += 1;
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
