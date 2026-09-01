// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt5514-spi.c  --  RT5514 SPI driver
 *
 * Copyright 2015 Realtek Semiconductor Corp.
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

// C include dependencies translated as external Rust dependencies:
// linux/cleanup.h, linux/module.h, linux/input.h, linux/spi/spi.h,
// linux/device.h, linux/init.h, linux/delay.h, linux/interrupt.h,
// linux/irq.h, linux/slab.h, linux/sched.h, linux/uaccess.h,
// linux/regulator/consumer.h, linux/pm_qos.h, linux/sysfs.h, linux/clk.h,
// sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// sound/soc-dapm.h, sound/initval.h, sound/tlv.h, and rt5514-spi.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u8 = u8;
type u32 = u32;
type size_t = usize;
type bool_t = c_int;
type snd_pcm_uframes_t = usize;
type irqreturn_t = c_int;

const true_: bool_t = 1;
const false_: bool_t = 0;

const DRV_NAME: &[u8] = b"rt5514-spi\0";

extern "C" {
    static mut PAGE_SIZE: size_t;
    static mut GFP_KERNEL: c_uint;
    static mut ENOMEM: c_int;
    static mut SNDRV_PCM_INFO_MMAP: c_uint;
    static mut SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static mut SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static mut SNDRV_PCM_FMTBIT_S16_LE: u64;
    static mut SNDRV_PCM_RATE_16000: c_uint;
    static mut SNDRV_DMA_TYPE_VMALLOC: c_int;
    static mut RT5514_BUFFER_VOICE_WP: c_uint;
    static mut RT5514_BUFFER_VOICE_BASE: c_uint;
    static mut RT5514_BUFFER_VOICE_LIMIT: c_uint;
    static mut RT5514_IRQ_CTRL: c_uint;
    static mut RT5514_IRQ_STATUS_BIT: u8;
    static mut RT5514_SPI_CMD_BURST_READ: u8;
    static mut RT5514_SPI_CMD_BURST_WRITE: u8;
    static mut RT5514_SPI_BUF_LEN: c_uint;
    static mut IRQ_HANDLED: irqreturn_t;
    static mut IRQF_TRIGGER_RISING: c_uint;
    static mut IRQF_ONESHOT: c_uint;

    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_uint) -> bool_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: Option<unsafe extern "C" fn(*mut work_struct)>);
    fn devm_request_threaded_irq(
        dev: *mut device,
        irq: c_int,
        handler: *const c_void,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_uint,
        name: *const c_char,
        data: *mut c_void,
    ) -> c_int;
    fn device_init_wakeup(dev: *mut device, enable: bool);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool_t;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: size_t) -> snd_pcm_uframes_t;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        data: *mut c_void,
        size: size_t,
        max: size_t,
    ) -> c_int;
    fn kmalloc(size: size_t, flags: c_uint) -> *mut u8;
    fn kfree(ptr: *mut c_void);
    fn memset(ptr: *mut c_void, value: c_int, size: size_t) -> *mut c_void;
    fn spi_message_init(message: *mut spi_message);
    fn spi_message_add_tail(transfer: *mut spi_transfer, message: *mut spi_message);
    fn spi_sync(spi: *mut spi_device, message: *mut spi_message) -> c_int;
    fn spi_write(spi: *mut spi_device, buf: *const c_void, len: size_t) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn device_may_wakeup(dev: *mut device) -> bool;
    fn enable_irq_wake(irq: c_int) -> c_int;
    fn disable_irq_wake(irq: c_int) -> c_int;
    fn to_spi_device(dev: *mut device) -> *mut spi_device;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    dma_area: *mut u8,
    dma_bytes: size_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct delayed_work {
    work: work_struct,
}

#[repr(C)]
pub struct spi_device {
    dev: device,
    irq: c_int,
}

#[repr(C)]
pub struct spi_message {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_transfer {
    tx_buf: *const c_void,
    rx_buf: *mut c_void,
    len: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    info: c_uint,
    formats: u64,
    period_bytes_min: size_t,
    period_bytes_max: size_t,
    periods_min: c_uint,
    periods_max: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    buffer_bytes_max: size_t,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    name: *const c_char,
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pointer: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t,
    >,
    pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct dev_pm_ops {
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
    of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct spi_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
}

#[repr(C)]
struct rt5514_dsp {
    dev: *mut device,
    copy_work: delayed_work,
    dma_lock: mutex,
    substream: *mut snd_pcm_substream,
    buf_base: c_uint,
    buf_limit: c_uint,
    buf_rp: c_uint,
    buf_size: size_t,
    get_size: size_t,
    dma_offset: size_t,
}

static mut rt5514_spi: *mut spi_device = null_mut();

static mut rt5514_spi_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: 0,
    formats: 0,
    period_bytes_min: 0,
    period_bytes_max: 0x20000 / 8,
    periods_min: 8,
    periods_max: 8,
    channels_min: 1,
    channels_max: 1,
    buffer_bytes_max: 0x20000,
};

static mut rt5514_spi_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"rt5514-dsp-cpu-dai\0".as_ptr() as *const c_char,
    id: 0,
    capture: snd_soc_pcm_stream {
        stream_name: b"DSP Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 1,
        rates: 0,
        formats: 0,
    },
};

unsafe fn init_static_constants() {
    rt5514_spi_pcm_hardware.info =
        SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED;
    rt5514_spi_pcm_hardware.formats = SNDRV_PCM_FMTBIT_S16_LE;
    rt5514_spi_pcm_hardware.period_bytes_min = PAGE_SIZE;
    rt5514_spi_dai.capture.rates = SNDRV_PCM_RATE_16000;
    rt5514_spi_dai.capture.formats = SNDRV_PCM_FMTBIT_S16_LE;
}

unsafe extern "C" fn rt5514_spi_copy_work(work: *mut work_struct) {
    let rt5514_dsp = (work as *mut u8).sub(core::mem::offset_of!(rt5514_dsp, copy_work)
        + core::mem::offset_of!(delayed_work, work)) as *mut rt5514_dsp;
    let mut runtime: *mut snd_pcm_runtime;
    let mut period_bytes: size_t;
    let mut truncated_bytes: size_t = 0;
    let mut cur_wp: c_uint;
    let mut remain_data: c_uint;
    let mut buf: [u8; 8] = [0; 8];

    mutex_lock(&mut (*rt5514_dsp).dma_lock);
    if (*rt5514_dsp).substream.is_null() {
        dev_err((*rt5514_dsp).dev, b"No pcm substream\n\0".as_ptr() as *const c_char);
        mutex_unlock(&mut (*rt5514_dsp).dma_lock);
        return;
    }

    runtime = (*(*rt5514_dsp).substream).runtime;
    period_bytes = snd_pcm_lib_period_bytes((*rt5514_dsp).substream);
    if period_bytes == 0 {
        schedule_delayed_work(&mut (*rt5514_dsp).copy_work, 5);
        mutex_unlock(&mut (*rt5514_dsp).dma_lock);
        return;
    }

    if (*rt5514_dsp).buf_size % period_bytes != 0 {
        (*rt5514_dsp).buf_size = ((*rt5514_dsp).buf_size / period_bytes) * period_bytes;
    }

    if (*rt5514_dsp).get_size >= (*rt5514_dsp).buf_size {
        rt5514_spi_burst_read(RT5514_BUFFER_VOICE_WP, buf.as_mut_ptr(), size_of::<[u8; 8]>());
        cur_wp = (buf[0] as c_uint)
            | ((buf[1] as c_uint) << 8)
            | ((buf[2] as c_uint) << 16)
            | ((buf[3] as c_uint) << 24);

        if cur_wp >= (*rt5514_dsp).buf_rp {
            remain_data = cur_wp - (*rt5514_dsp).buf_rp;
        } else {
            remain_data = ((*rt5514_dsp).buf_limit - (*rt5514_dsp).buf_rp)
                + (cur_wp - (*rt5514_dsp).buf_base);
        }

        if (remain_data as size_t) < period_bytes {
            schedule_delayed_work(&mut (*rt5514_dsp).copy_work, 5);
            mutex_unlock(&mut (*rt5514_dsp).dma_lock);
            return;
        }
    }

    if ((*rt5514_dsp).buf_rp as size_t + period_bytes) <= (*rt5514_dsp).buf_limit as size_t {
        rt5514_spi_burst_read(
            (*rt5514_dsp).buf_rp,
            (*runtime).dma_area.add((*rt5514_dsp).dma_offset),
            period_bytes,
        );

        if ((*rt5514_dsp).buf_rp as size_t + period_bytes) == (*rt5514_dsp).buf_limit as size_t {
            (*rt5514_dsp).buf_rp = (*rt5514_dsp).buf_base;
        } else {
            (*rt5514_dsp).buf_rp = (*rt5514_dsp).buf_rp.wrapping_add(period_bytes as c_uint);
        }
    } else {
        truncated_bytes = ((*rt5514_dsp).buf_limit - (*rt5514_dsp).buf_rp) as size_t;
        rt5514_spi_burst_read(
            (*rt5514_dsp).buf_rp,
            (*runtime).dma_area.add((*rt5514_dsp).dma_offset),
            truncated_bytes,
        );

        rt5514_spi_burst_read(
            (*rt5514_dsp).buf_base,
            (*runtime)
                .dma_area
                .add((*rt5514_dsp).dma_offset)
                .add(truncated_bytes),
            period_bytes - truncated_bytes,
        );

        (*rt5514_dsp).buf_rp =
            (*rt5514_dsp).buf_base + (period_bytes - truncated_bytes) as c_uint;
    }

    (*rt5514_dsp).get_size += period_bytes;
    (*rt5514_dsp).dma_offset += period_bytes;
    if (*rt5514_dsp).dma_offset >= (*runtime).dma_bytes {
        (*rt5514_dsp).dma_offset = 0;
    }

    snd_pcm_period_elapsed((*rt5514_dsp).substream);

    schedule_delayed_work(&mut (*rt5514_dsp).copy_work, 5);
    mutex_unlock(&mut (*rt5514_dsp).dma_lock);
}

unsafe fn rt5514_schedule_copy(rt5514_dsp: *mut rt5514_dsp) {
    let mut buf: [u8; 8] = [0; 8];

    if (*rt5514_dsp).substream.is_null() {
        return;
    }

    (*rt5514_dsp).get_size = 0;

    /**
     * The address area x1800XXXX is the register address, and it cannot
     * support spi burst read perfectly. So we use the spi burst read
     * individually to make sure the data correctly.
     */
    rt5514_spi_burst_read(RT5514_BUFFER_VOICE_BASE, buf.as_mut_ptr(), size_of::<[u8; 8]>());
    (*rt5514_dsp).buf_base = (buf[0] as c_uint)
        | ((buf[1] as c_uint) << 8)
        | ((buf[2] as c_uint) << 16)
        | ((buf[3] as c_uint) << 24);

    rt5514_spi_burst_read(RT5514_BUFFER_VOICE_LIMIT, buf.as_mut_ptr(), size_of::<[u8; 8]>());
    (*rt5514_dsp).buf_limit = (buf[0] as c_uint)
        | ((buf[1] as c_uint) << 8)
        | ((buf[2] as c_uint) << 16)
        | ((buf[3] as c_uint) << 24);

    rt5514_spi_burst_read(RT5514_BUFFER_VOICE_WP, buf.as_mut_ptr(), size_of::<[u8; 8]>());
    (*rt5514_dsp).buf_rp = (buf[0] as c_uint)
        | ((buf[1] as c_uint) << 8)
        | ((buf[2] as c_uint) << 16)
        | ((buf[3] as c_uint) << 24);

    if (*rt5514_dsp).buf_rp % 8 != 0 {
        (*rt5514_dsp).buf_rp = ((*rt5514_dsp).buf_rp / 8) * 8;
    }

    (*rt5514_dsp).buf_size = ((*rt5514_dsp).buf_limit - (*rt5514_dsp).buf_base) as size_t;

    if (*rt5514_dsp).buf_base != 0
        && (*rt5514_dsp).buf_limit != 0
        && (*rt5514_dsp).buf_rp != 0
        && (*rt5514_dsp).buf_size != 0
    {
        schedule_delayed_work(&mut (*rt5514_dsp).copy_work, 0);
    }
}

unsafe extern "C" fn rt5514_spi_irq(_irq: c_int, data: *mut c_void) -> irqreturn_t {
    let rt5514_dsp = data as *mut rt5514_dsp;

    rt5514_schedule_copy(rt5514_dsp);

    IRQ_HANDLED
}

/* PCM for streaming audio from the DSP buffer */
unsafe extern "C" fn rt5514_spi_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    snd_soc_set_runtime_hwparams(substream, &rt5514_spi_pcm_hardware);

    0
}

unsafe extern "C" fn rt5514_spi_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    _hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let rt5514_dsp = snd_soc_component_get_drvdata(component) as *mut rt5514_dsp;
    let mut buf: [u8; 8] = [0; 8];

    mutex_lock(&mut (*rt5514_dsp).dma_lock);
    (*rt5514_dsp).substream = substream;
    (*rt5514_dsp).dma_offset = 0;

    /* Read IRQ status and schedule copy accordingly. */
    rt5514_spi_burst_read(RT5514_IRQ_CTRL, buf.as_mut_ptr(), size_of::<[u8; 8]>());
    if (buf[0] & RT5514_IRQ_STATUS_BIT) != 0 {
        rt5514_schedule_copy(rt5514_dsp);
    }

    mutex_unlock(&mut (*rt5514_dsp).dma_lock);
    0
}

unsafe extern "C" fn rt5514_spi_hw_free(
    component: *mut snd_soc_component,
    _substream: *mut snd_pcm_substream,
) -> c_int {
    let rt5514_dsp = snd_soc_component_get_drvdata(component) as *mut rt5514_dsp;

    mutex_lock(&mut (*rt5514_dsp).dma_lock);
    (*rt5514_dsp).substream = null_mut();
    mutex_unlock(&mut (*rt5514_dsp).dma_lock);

    cancel_delayed_work_sync(&mut (*rt5514_dsp).copy_work);

    0
}

unsafe extern "C" fn rt5514_spi_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let rt5514_dsp = snd_soc_component_get_drvdata(component) as *mut rt5514_dsp;

    bytes_to_frames(runtime, (*rt5514_dsp).dma_offset)
}

unsafe extern "C" fn rt5514_spi_pcm_probe(component: *mut snd_soc_component) -> c_int {
    let rt5514_dsp: *mut rt5514_dsp;
    let mut ret: c_int;

    rt5514_dsp = devm_kzalloc(
        (*component).dev,
        size_of::<rt5514_dsp>(),
        GFP_KERNEL,
    ) as *mut rt5514_dsp;
    if rt5514_dsp.is_null() {
        return -ENOMEM;
    }

    (*rt5514_dsp).dev = &mut (*rt5514_spi).dev;
    mutex_init(&mut (*rt5514_dsp).dma_lock);
    INIT_DELAYED_WORK(&mut (*rt5514_dsp).copy_work, Some(rt5514_spi_copy_work));
    snd_soc_component_set_drvdata(component, rt5514_dsp as *mut c_void);

    if (*rt5514_spi).irq != 0 {
        ret = devm_request_threaded_irq(
            &mut (*rt5514_spi).dev,
            (*rt5514_spi).irq,
            null(),
            Some(rt5514_spi_irq),
            IRQF_TRIGGER_RISING | IRQF_ONESHOT,
            b"rt5514-spi\0".as_ptr() as *const c_char,
            rt5514_dsp as *mut c_void,
        );
        if ret != 0 {
            dev_err(
                &mut (*rt5514_spi).dev,
                b"%s Failed to request IRQ: %d\n\0".as_ptr() as *const c_char,
                b"rt5514_spi_pcm_probe\0".as_ptr() as *const c_char,
                ret,
            );
        } else {
            device_init_wakeup((*rt5514_dsp).dev, true);
        }
    }

    0
}

unsafe extern "C" fn rt5514_spi_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_VMALLOC, null_mut(), 0, 0);
    0
}

static rt5514_spi_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const c_char,
    probe: Some(rt5514_spi_pcm_probe),
    open: Some(rt5514_spi_pcm_open),
    hw_params: Some(rt5514_spi_hw_params),
    hw_free: Some(rt5514_spi_hw_free),
    pointer: Some(rt5514_spi_pcm_pointer),
    pcm_new: Some(rt5514_spi_pcm_new),
    legacy_dai_naming: 1,
};

/**
 * rt5514_spi_burst_read - Read data from SPI by rt5514 address.
 * @addr: Start address.
 * @rxbuf: Data Buffer for reading.
 * @len: Data length, it must be a multiple of 8.
 *
 *
 * Returns true for success.
 */
#[no_mangle]
pub unsafe extern "C" fn rt5514_spi_burst_read(
    addr: c_uint,
    rxbuf: *mut u8,
    len: size_t,
) -> c_int {
    let spi_cmd: u8 = RT5514_SPI_CMD_BURST_READ;
    let mut status: c_int;
    let mut write_buf: [u8; 8] = [0; 8];
    let mut i: c_uint;
    let mut end: c_uint;
    let mut offset: c_uint = 0;

    let mut message: spi_message = zeroed();
    let mut x: [spi_transfer; 3] = zeroed();

    while (offset as size_t) < len {
        if offset + RT5514_SPI_BUF_LEN <= len as c_uint {
            end = RT5514_SPI_BUF_LEN;
        } else {
            end = (len as c_uint) % RT5514_SPI_BUF_LEN;
        }

        write_buf[0] = spi_cmd;
        write_buf[1] = (((addr + offset) & 0xff000000) >> 24) as u8;
        write_buf[2] = (((addr + offset) & 0x00ff0000) >> 16) as u8;
        write_buf[3] = (((addr + offset) & 0x0000ff00) >> 8) as u8;
        write_buf[4] = (((addr + offset) & 0x000000ff) >> 0) as u8;

        spi_message_init(&mut message);
        memset(
            x.as_mut_ptr() as *mut c_void,
            0,
            size_of::<[spi_transfer; 3]>(),
        );

        x[0].len = 5;
        x[0].tx_buf = write_buf.as_ptr() as *const c_void;
        spi_message_add_tail(&mut x[0], &mut message);

        x[1].len = 4;
        x[1].tx_buf = write_buf.as_ptr() as *const c_void;
        spi_message_add_tail(&mut x[1], &mut message);

        x[2].len = end;
        x[2].rx_buf = rxbuf.add(offset as size_t) as *mut c_void;
        spi_message_add_tail(&mut x[2], &mut message);

        status = spi_sync(rt5514_spi, &mut message);

        if status != 0 {
            return false_;
        }

        offset += RT5514_SPI_BUF_LEN;
    }

    i = 0;
    while (i as size_t) < len {
        write_buf[0] = *rxbuf.add(i as size_t + 0);
        write_buf[1] = *rxbuf.add(i as size_t + 1);
        write_buf[2] = *rxbuf.add(i as size_t + 2);
        write_buf[3] = *rxbuf.add(i as size_t + 3);
        write_buf[4] = *rxbuf.add(i as size_t + 4);
        write_buf[5] = *rxbuf.add(i as size_t + 5);
        write_buf[6] = *rxbuf.add(i as size_t + 6);
        write_buf[7] = *rxbuf.add(i as size_t + 7);

        *rxbuf.add(i as size_t + 0) = write_buf[7];
        *rxbuf.add(i as size_t + 1) = write_buf[6];
        *rxbuf.add(i as size_t + 2) = write_buf[5];
        *rxbuf.add(i as size_t + 3) = write_buf[4];
        *rxbuf.add(i as size_t + 4) = write_buf[3];
        *rxbuf.add(i as size_t + 5) = write_buf[2];
        *rxbuf.add(i as size_t + 6) = write_buf[1];
        *rxbuf.add(i as size_t + 7) = write_buf[0];

        i += 8;
    }

    true_
}
// EXPORT_SYMBOL_GPL(rt5514_spi_burst_read);

/**
 * rt5514_spi_burst_write - Write data to SPI by rt5514 address.
 * @addr: Start address.
 * @txbuf: Data Buffer for writng.
 * @len: Data length, it must be a multiple of 8.
 *
 *
 * Returns true for success.
 */
#[no_mangle]
pub unsafe extern "C" fn rt5514_spi_burst_write(
    addr: u32,
    txbuf: *const u8,
    len: size_t,
) -> c_int {
    let spi_cmd: u8 = RT5514_SPI_CMD_BURST_WRITE;
    let mut write_buf: *mut u8;
    let mut i: c_uint;
    let mut end: c_uint;
    let mut offset: c_uint = 0;

    write_buf = kmalloc((RT5514_SPI_BUF_LEN + 6) as size_t, GFP_KERNEL);

    if write_buf.is_null() {
        return -ENOMEM;
    }

    while (offset as size_t) < len {
        if offset + RT5514_SPI_BUF_LEN <= len as c_uint {
            end = RT5514_SPI_BUF_LEN;
        } else {
            end = (len as c_uint) % RT5514_SPI_BUF_LEN;
        }

        *write_buf.add(0) = spi_cmd;
        *write_buf.add(1) = (((addr + offset) & 0xff000000) >> 24) as u8;
        *write_buf.add(2) = (((addr + offset) & 0x00ff0000) >> 16) as u8;
        *write_buf.add(3) = (((addr + offset) & 0x0000ff00) >> 8) as u8;
        *write_buf.add(4) = (((addr + offset) & 0x000000ff) >> 0) as u8;

        i = 0;
        while i < end {
            *write_buf.add(i as size_t + 12) = *txbuf.add((offset + i + 0) as size_t);
            *write_buf.add(i as size_t + 11) = *txbuf.add((offset + i + 1) as size_t);
            *write_buf.add(i as size_t + 10) = *txbuf.add((offset + i + 2) as size_t);
            *write_buf.add(i as size_t + 9) = *txbuf.add((offset + i + 3) as size_t);
            *write_buf.add(i as size_t + 8) = *txbuf.add((offset + i + 4) as size_t);
            *write_buf.add(i as size_t + 7) = *txbuf.add((offset + i + 5) as size_t);
            *write_buf.add(i as size_t + 6) = *txbuf.add((offset + i + 6) as size_t);
            *write_buf.add(i as size_t + 5) = *txbuf.add((offset + i + 7) as size_t);
            i += 8;
        }

        *write_buf.add(end as size_t + 5) = spi_cmd;

        spi_write(rt5514_spi, write_buf as *const c_void, (end + 6) as size_t);

        offset += RT5514_SPI_BUF_LEN;
    }

    kfree(write_buf as *mut c_void);

    0
}
// EXPORT_SYMBOL_GPL(rt5514_spi_burst_write);

unsafe extern "C" fn rt5514_spi_probe(spi: *mut spi_device) -> c_int {
    let mut ret: c_int;

    init_static_constants();
    rt5514_spi = spi;

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &rt5514_spi_component,
        &mut rt5514_spi_dai,
        1,
    );
    if ret < 0 {
        dev_err(
            &mut (*spi).dev,
            b"Failed to register component.\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    0
}

unsafe extern "C" fn rt5514_suspend(dev: *mut device) -> c_int {
    let irq: c_int = (*to_spi_device(dev)).irq;

    if device_may_wakeup(dev) {
        enable_irq_wake(irq);
    }

    0
}

unsafe extern "C" fn rt5514_resume(dev: *mut device) -> c_int {
    let rt5514_dsp = dev_get_drvdata(dev) as *mut rt5514_dsp;
    let irq: c_int = (*to_spi_device(dev)).irq;
    let mut buf: [u8; 8] = [0; 8];

    if device_may_wakeup(dev) {
        disable_irq_wake(irq);
    }

    if !rt5514_dsp.is_null() {
        if !(*rt5514_dsp).substream.is_null() {
            rt5514_spi_burst_read(RT5514_IRQ_CTRL, buf.as_mut_ptr(), size_of::<[u8; 8]>());
            if (buf[0] & RT5514_IRQ_STATUS_BIT) != 0 {
                rt5514_schedule_copy(rt5514_dsp);
            }
        }
    }

    0
}

static rt5514_pm_ops: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(rt5514_suspend, rt5514_resume)
    suspend: Some(rt5514_suspend),
    resume: Some(rt5514_resume),
};

static rt5514_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"realtek,rt5514\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: null(),
    },
];
// MODULE_DEVICE_TABLE(of, rt5514_of_match);

static mut rt5514_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: b"rt5514\0".as_ptr() as *const c_char,
        pm: &rt5514_pm_ops,
        of_match_table: rt5514_of_match.as_ptr(),
    },
    probe: Some(rt5514_spi_probe),
};
// module_spi_driver(rt5514_spi_driver);

// MODULE_DESCRIPTION("RT5514 SPI driver");
// MODULE_AUTHOR("Oder Chiou <oder_chiou@realtek.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
