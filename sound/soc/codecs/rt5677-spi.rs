// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt5677-spi.c  --  RT5677 ALSA SoC audio codec driver
 *
 * Copyright 2013 Realtek Semiconductor Corp.
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

// C dependencies removed from executable Rust:
// linux/cleanup.h, linux/module.h, linux/input.h, linux/spi/spi.h,
// linux/device.h, linux/init.h, linux/delay.h, linux/interrupt.h,
// linux/irq.h, linux/slab.h, linux/sched.h, linux/uaccess.h,
// linux/regulator/consumer.h, linux/pm_qos.h, linux/sysfs.h, linux/clk.h,
// linux/firmware.h, linux/acpi.h, sound/soc.h, rt5677.h, rt5677-spi.h.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type u8 = ::core::primitive::u8;
type u32 = ::core::primitive::u32;
type size_t = usize;
type bool_ = bool;
type snd_pcm_uframes_t = usize;

const DRV_NAME: &[u8] = b"rt5677spi\0";

const RT5677_SPI_BURST_LEN: usize = 240;
const RT5677_SPI_HEADER: usize = 5;
const RT5677_SPI_FREQ: u32 = 6000000;

/* The AddressPhase and DataPhase of SPI commands are MSB first on the wire.
 * DataPhase word size of 16-bit commands is 2 bytes.
 * DataPhase word size of 32-bit commands is 4 bytes.
 * DataPhase word size of burst commands is 8 bytes.
 * The DSP CPU is little-endian.
 */
const RT5677_SPI_WRITE_BURST: u8 = 0x5;
const RT5677_SPI_READ_BURST: u8 = 0x4;
const RT5677_SPI_WRITE_32: u8 = 0x3;
const RT5677_SPI_READ_32: u8 = 0x2;
const RT5677_SPI_WRITE_16: u8 = 0x1;
const RT5677_SPI_READ_16: u8 = 0x0;

const RT5677_BUF_BYTES_TOTAL: u32 = 0x20000;
const RT5677_MIC_BUF_ADDR: u32 = 0x60030000;
const RT5677_MODEL_ADDR: u32 = 0x5FFC9800;
const RT5677_MIC_BUF_BYTES: u32 = RT5677_BUF_BYTES_TOTAL - size_of::<u32>() as u32;
const RT5677_MIC_BUF_FIRST_READ_SIZE: u32 = 0x10000;

#[repr(C)]
pub struct spi_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    pub work: work_struct,
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
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_area: *mut u8,
    pub dma_bytes: size_t,
    pub rate: c_uint,
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub pcm: *mut c_void,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rt5677_priv {
    pub set_dsp_vad: Option<unsafe extern "C" fn(*mut snd_soc_component, bool_)>,
}

#[repr(C)]
pub struct firmware {
    pub size: size_t,
    pub data: *const u8,
}

#[repr(C)]
pub struct spi_transfer {
    pub tx_buf: *const c_void,
    pub rx_buf: *mut c_void,
    pub len: c_uint,
    pub speed_hz: u32,
}

#[repr(C)]
pub struct spi_message {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub buffer_bytes_max: size_t,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub capture: snd_soc_pcm_stream,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, *mut snd_pcm_hw_params) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [c_char; 16],
}

#[repr(C)]
pub struct spi_device_id {
    pub name: [c_char; 32],
    pub driver_data: usize,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct spi_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut spi_device) -> c_int>,
    pub id_table: *const spi_device_id,
}

#[repr(C)]
pub struct rt5677_dsp {
    pub dev: *mut device,
    pub copy_work: delayed_work,
    pub dma_lock: mutex,
    pub substream: *mut snd_pcm_substream,
    pub dma_offset: size_t,      /* zero-based offset into runtime->dma_area */
    pub avail_bytes: size_t,     /* number of new bytes since last period */
    pub mic_read_offset: u32,    /* zero-based offset into DSP's mic buffer */
    pub new_hotword: bool_,      /* a new hotword is fired */
}

unsafe extern "C" {
    static mut spi_mutex: mutex;

    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_RATE_16000: c_uint;
    static PAGE_SIZE: size_t;
    static GFP_KERNEL: c_uint;
    static SNDRV_DMA_TYPE_VMALLOC: c_uint;

    static ENODEV: c_int;
    static EACCES: c_int;
    static EFAULT: c_int;
    static EINVAL: c_int;
    static ENOMEM: c_int;

    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtdcom_lookup(rtd: *mut snd_soc_pcm_runtime, name: *const c_char) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_set_drvdata(component: *mut snd_soc_component, data: *mut c_void);
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool_;
    fn mutex_init(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn INIT_DELAYED_WORK(work: *mut delayed_work, func: unsafe extern "C" fn(*mut work_struct));
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: size_t) -> snd_pcm_uframes_t;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: snd_pcm_uframes_t) -> size_t;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn secs_to_jiffies(secs: c_uint) -> c_uint;
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_uint) -> bool_;
    fn snd_pcm_set_managed_buffer_all(pcm: *mut c_void, ty: c_uint, data: *mut c_void, min: size_t, max: size_t);
    fn devm_snd_soc_register_component(dev: *mut device, component_driver: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn spi_message_init_with_transfers(m: *mut spi_message, t: *mut spi_transfer, num_xfers: c_uint);
    fn spi_sync(spi: *mut spi_device, message: *mut spi_message) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
}

static mut g_spi: *mut spi_device = ptr::null_mut();

static rt5677_spi_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: 0, /* SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_INTERLEAVED */
    formats: 0, /* SNDRV_PCM_FMTBIT_S16_LE */
    period_bytes_min: 0, /* PAGE_SIZE */
    period_bytes_max: RT5677_BUF_BYTES_TOTAL as size_t / 8,
    periods_min: 8,
    periods_max: 8,
    channels_min: 1,
    channels_max: 1,
    buffer_bytes_max: RT5677_BUF_BYTES_TOTAL as size_t,
};

static mut rt5677_spi_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    /* The DAI name "rt5677-dsp-cpu-dai" is not used. The actual DAI name
     * registered with ASoC is the name of the device "spi-RT5677AA:00",
     * because we only have one DAI. See snd_soc_register_dais().
     */
    name: b"rt5677-dsp-cpu-dai\0".as_ptr() as *const c_char,
    id: 0,
    capture: snd_soc_pcm_stream {
        stream_name: b"DSP Capture\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 1,
        rates: 0, /* SNDRV_PCM_RATE_16000 */
        formats: 0, /* SNDRV_PCM_FMTBIT_S16_LE */
    },
};

/* PCM for streaming audio from the DSP buffer */
unsafe extern "C" fn rt5677_spi_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    snd_soc_set_runtime_hwparams(substream, &rt5677_spi_pcm_hardware);
    0
}

unsafe extern "C" fn rt5677_spi_pcm_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let codec_component = snd_soc_rtdcom_lookup(rtd, b"rt5677\0".as_ptr() as *const c_char);
    let rt5677 = snd_soc_component_get_drvdata(codec_component) as *mut rt5677_priv;
    let rt5677_dsp = snd_soc_component_get_drvdata(component) as *mut rt5677_dsp;

    cancel_delayed_work_sync(&mut (*rt5677_dsp).copy_work);
    if let Some(set_dsp_vad) = (*rt5677).set_dsp_vad {
        set_dsp_vad(codec_component, false);
    }
    0
}

unsafe extern "C" fn rt5677_spi_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    _hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let rt5677_dsp = snd_soc_component_get_drvdata(component) as *mut rt5677_dsp;

    mutex_lock(&mut (*rt5677_dsp).dma_lock);
    (*rt5677_dsp).substream = substream;
    mutex_unlock(&mut (*rt5677_dsp).dma_lock);

    0
}

unsafe extern "C" fn rt5677_spi_hw_free(
    component: *mut snd_soc_component,
    _substream: *mut snd_pcm_substream,
) -> c_int {
    let rt5677_dsp = snd_soc_component_get_drvdata(component) as *mut rt5677_dsp;

    mutex_lock(&mut (*rt5677_dsp).dma_lock);
    (*rt5677_dsp).substream = ptr::null_mut();
    mutex_unlock(&mut (*rt5677_dsp).dma_lock);

    0
}

unsafe extern "C" fn rt5677_spi_prepare(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let rt5677_component = snd_soc_rtdcom_lookup(rtd, b"rt5677\0".as_ptr() as *const c_char);
    let rt5677 = snd_soc_component_get_drvdata(rt5677_component) as *mut rt5677_priv;
    let rt5677_dsp = snd_soc_component_get_drvdata(component) as *mut rt5677_dsp;

    if let Some(set_dsp_vad) = (*rt5677).set_dsp_vad {
        set_dsp_vad(rt5677_component, true);
    }
    (*rt5677_dsp).dma_offset = 0;
    (*rt5677_dsp).avail_bytes = 0;
    0
}

unsafe extern "C" fn rt5677_spi_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let rt5677_dsp = snd_soc_component_get_drvdata(component) as *mut rt5677_dsp;

    bytes_to_frames(runtime, (*rt5677_dsp).dma_offset)
}

unsafe fn rt5677_spi_mic_write_offset(mic_write_offset: *mut u32) -> c_int {
    let mut ret: c_int;
    /* Grab the first 4 bytes that hold the write pointer on the
     * dsp, and check to make sure that it points somewhere inside the
     * buffer.
     */
    ret = rt5677_spi_read(RT5677_MIC_BUF_ADDR, mic_write_offset as *mut c_void, size_of::<u32>());
    if ret != 0 {
        return ret;
    }
    /* Adjust the offset so that it's zero-based */
    *mic_write_offset = (*mic_write_offset).wrapping_sub(size_of::<u32>() as u32);
    if *mic_write_offset < RT5677_MIC_BUF_BYTES { 0 } else { -EFAULT }
}

/*
 * Copy one contiguous block of audio samples from the DSP mic buffer to the
 * dma_area of the pcm runtime. The receiving buffer may wrap around.
 * @begin: start offset of the block to copy, in bytes.
 * @end:   offset of the first byte after the block to copy, must be greater
 *         than or equal to begin.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
unsafe fn rt5677_spi_copy_block(rt5677_dsp: *mut rt5677_dsp, mut begin: u32, end: u32) -> c_int {
    let runtime = (*(*rt5677_dsp).substream).runtime;
    let bytes_per_frame = frames_to_bytes(runtime, 1);
    let mut first_chunk_len: size_t;
    let mut second_chunk_len: size_t;
    let mut ret: c_int;

    if begin > end || (*runtime).dma_bytes < 2 * bytes_per_frame {
        dev_err(
            (*rt5677_dsp).dev,
            b"Invalid copy from (%u, %u), dma_area size %zu\n\0".as_ptr() as *const c_char,
            begin,
            end,
            (*runtime).dma_bytes,
        );
        return -EINVAL;
    }

    /* The block to copy is empty */
    if begin == end {
        return 0;
    }

    /* If the incoming chunk is too big for the receiving buffer, only the
     * last "receiving buffer size - one frame" bytes are copied.
     */
    if (end - begin) as size_t > (*runtime).dma_bytes - bytes_per_frame {
        begin = end - ((*runtime).dma_bytes - bytes_per_frame) as u32;
    }

    /* May need to split to two chunks, calculate the size of each */
    first_chunk_len = (end - begin) as size_t;
    second_chunk_len = 0;
    if (*rt5677_dsp).dma_offset + first_chunk_len > (*runtime).dma_bytes {
        /* Receiving buffer wrapped around */
        second_chunk_len = first_chunk_len;
        first_chunk_len = (*runtime).dma_bytes - (*rt5677_dsp).dma_offset;
        second_chunk_len -= first_chunk_len;
    }

    /* Copy first chunk */
    ret = rt5677_spi_read(
        RT5677_MIC_BUF_ADDR + size_of::<u32>() as u32 + begin,
        (*runtime).dma_area.add((*rt5677_dsp).dma_offset) as *mut c_void,
        first_chunk_len,
    );
    if ret != 0 {
        return ret;
    }
    (*rt5677_dsp).dma_offset += first_chunk_len;
    if (*rt5677_dsp).dma_offset == (*runtime).dma_bytes {
        (*rt5677_dsp).dma_offset = 0;
    }

    /* Copy second chunk */
    if second_chunk_len != 0 {
        ret = rt5677_spi_read(
            RT5677_MIC_BUF_ADDR + size_of::<u32>() as u32 + begin + first_chunk_len as u32,
            (*runtime).dma_area as *mut c_void,
            second_chunk_len,
        );
        if ret == 0 {
            (*rt5677_dsp).dma_offset = second_chunk_len;
        }
    }
    ret
}

/*
 * Copy a given amount of audio samples from the DSP mic buffer starting at
 * mic_read_offset, to the dma_area of the pcm runtime. The source buffer may
 * wrap around. mic_read_offset is updated after successful copy.
 * @amount: amount of samples to copy, in bytes.
 *
 * Return: Zero if successful, or a negative error code on failure.
 */
unsafe fn rt5677_spi_copy(rt5677_dsp: *mut rt5677_dsp, amount: u32) -> c_int {
    let mut ret: c_int = 0;
    let mut target: u32;

    if amount == 0 {
        return ret;
    }

    target = (*rt5677_dsp).mic_read_offset + amount;
    /* Copy the first chunk in DSP's mic buffer */
    ret |= rt5677_spi_copy_block(
        rt5677_dsp,
        (*rt5677_dsp).mic_read_offset,
        core::cmp::min(target, RT5677_MIC_BUF_BYTES),
    );

    if target >= RT5677_MIC_BUF_BYTES {
        /* Wrap around, copy the second chunk */
        target -= RT5677_MIC_BUF_BYTES;
        ret |= rt5677_spi_copy_block(rt5677_dsp, 0, target);
    }

    if ret == 0 {
        (*rt5677_dsp).mic_read_offset = target;
    }
    ret
}

/*
 * A delayed work that streams audio samples from the DSP mic buffer to the
 * dma_area of the pcm runtime via SPI.
 */
unsafe extern "C" fn rt5677_spi_copy_work(work: *mut work_struct) {
    let rt5677_dsp = (work as *mut u8).sub(offset_of_rt5677_dsp_copy_work_work()) as *mut rt5677_dsp;
    let runtime: *mut snd_pcm_runtime;
    let mut mic_write_offset: u32 = 0;
    let mut new_bytes: size_t;
    let mut copy_bytes: size_t;
    let period_bytes: size_t;
    let delay: c_uint;
    let mut ret: c_int;

    /* Ensure runtime->dma_area buffer does not go away while copying. */
    mutex_lock(&mut (*rt5677_dsp).dma_lock);
    if (*rt5677_dsp).substream.is_null() {
        dev_err((*rt5677_dsp).dev, b"No pcm substream\n\0".as_ptr() as *const c_char);
        mutex_unlock(&mut (*rt5677_dsp).dma_lock);
        return;
    }

    runtime = (*(*rt5677_dsp).substream).runtime;

    if rt5677_spi_mic_write_offset(&mut mic_write_offset) != 0 {
        dev_err((*rt5677_dsp).dev, b"No mic_write_offset\n\0".as_ptr() as *const c_char);
        mutex_unlock(&mut (*rt5677_dsp).dma_lock);
        return;
    }

    /* If this is the first time that we've asked for streaming data after
     * a hotword is fired, we should start reading from the previous 2
     * seconds of audio from wherever the mic_write_offset is currently.
     */
    if (*rt5677_dsp).new_hotword {
        (*rt5677_dsp).new_hotword = false;
        /* See if buffer wraparound happens */
        if mic_write_offset < RT5677_MIC_BUF_FIRST_READ_SIZE {
            (*rt5677_dsp).mic_read_offset =
                RT5677_MIC_BUF_BYTES - (RT5677_MIC_BUF_FIRST_READ_SIZE - mic_write_offset);
        } else {
            (*rt5677_dsp).mic_read_offset = mic_write_offset - RT5677_MIC_BUF_FIRST_READ_SIZE;
        }
    }

    /* Calculate the amount of new samples in bytes */
    if (*rt5677_dsp).mic_read_offset <= mic_write_offset {
        new_bytes = (mic_write_offset - (*rt5677_dsp).mic_read_offset) as size_t;
    } else {
        new_bytes = (RT5677_MIC_BUF_BYTES + mic_write_offset - (*rt5677_dsp).mic_read_offset) as size_t;
    }

    /* Copy all new samples from DSP mic buffer, one period at a time */
    period_bytes = snd_pcm_lib_period_bytes((*rt5677_dsp).substream);
    while new_bytes != 0 {
        copy_bytes = core::cmp::min(new_bytes, period_bytes - (*rt5677_dsp).avail_bytes);
        ret = rt5677_spi_copy(rt5677_dsp, copy_bytes as u32);
        if ret != 0 {
            dev_err((*rt5677_dsp).dev, b"Copy failed %d\n\0".as_ptr() as *const c_char, ret);
            mutex_unlock(&mut (*rt5677_dsp).dma_lock);
            return;
        }
        (*rt5677_dsp).avail_bytes += copy_bytes;
        if (*rt5677_dsp).avail_bytes >= period_bytes {
            snd_pcm_period_elapsed((*rt5677_dsp).substream);
            (*rt5677_dsp).avail_bytes = 0;
        }
        new_bytes -= copy_bytes;
    }

    delay = (bytes_to_frames(runtime, period_bytes) as c_uint) / (*runtime).rate;
    schedule_delayed_work(&mut (*rt5677_dsp).copy_work, secs_to_jiffies(delay));
    mutex_unlock(&mut (*rt5677_dsp).dma_lock);
}

unsafe extern "C" fn rt5677_spi_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_VMALLOC, ptr::null_mut(), 0, 0);
    0
}

unsafe extern "C" fn rt5677_spi_pcm_probe(component: *mut snd_soc_component) -> c_int {
    let rt5677_dsp: *mut rt5677_dsp;

    rt5677_dsp = devm_kzalloc((*component).dev, size_of::<rt5677_dsp>(), GFP_KERNEL) as *mut rt5677_dsp;
    if rt5677_dsp.is_null() {
        return -ENOMEM;
    }
    (*rt5677_dsp).dev = &mut (*g_spi).dev;
    mutex_init(&mut (*rt5677_dsp).dma_lock);
    INIT_DELAYED_WORK(&mut (*rt5677_dsp).copy_work, rt5677_spi_copy_work);

    snd_soc_component_set_drvdata(component, rt5677_dsp as *mut c_void);
    0
}

static rt5677_spi_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const c_char,
    probe: Some(rt5677_spi_pcm_probe),
    open: Some(rt5677_spi_pcm_open),
    close: Some(rt5677_spi_pcm_close),
    hw_params: Some(rt5677_spi_hw_params),
    hw_free: Some(rt5677_spi_hw_free),
    prepare: Some(rt5677_spi_prepare),
    pointer: Some(rt5677_spi_pcm_pointer),
    pcm_new: Some(rt5677_spi_pcm_new),
    legacy_dai_naming: 1,
};

/* Select a suitable transfer command for the next transfer to ensure
 * the transfer address is always naturally aligned while minimizing
 * the total number of transfers required.
 *
 * 3 transfer commands are available:
 * RT5677_SPI_READ/WRITE_16:	Transfer 2 bytes
 * RT5677_SPI_READ/WRITE_32:	Transfer 4 bytes
 * RT5677_SPI_READ/WRITE_BURST:	Transfer any multiples of 8 bytes
 *
 * Note:
 * 16 Bit writes and reads are restricted to the address range
 * 0x18020000 ~ 0x18021000
 *
 * For example, reading 256 bytes at 0x60030004 uses the following commands:
 * 0x60030004 RT5677_SPI_READ_32	4 bytes
 * 0x60030008 RT5677_SPI_READ_BURST	240 bytes
 * 0x600300F8 RT5677_SPI_READ_BURST	8 bytes
 * 0x60030100 RT5677_SPI_READ_32	4 bytes
 *
 * Input:
 * @read: true for read commands; false for write commands
 * @align: alignment of the next transfer address
 * @remain: number of bytes remaining to transfer
 *
 * Output:
 * @len: number of bytes to transfer with the selected command
 * Returns the selected command
 */
unsafe fn rt5677_spi_select_cmd(read: bool_, align: u32, remain: u32, len: *mut u32) -> u8 {
    let cmd: u8;

    if align == 4 || remain <= 4 {
        cmd = RT5677_SPI_READ_32;
        *len = 4;
    } else {
        cmd = RT5677_SPI_READ_BURST;
        *len = (((remain - 1) >> 3) + 1) << 3;
        *len = core::cmp::min(*len, RT5677_SPI_BURST_LEN as u32);
    }
    if read { cmd } else { cmd + 1 }
}

/* Copy dstlen bytes from src to dst, while reversing byte order for each word.
 * If srclen < dstlen, zeros are padded.
 */
unsafe fn rt5677_spi_reverse(dst: *mut u8, dstlen: u32, src: *const u8, srclen: u32) {
    let mut w: u32;
    let mut i: u32;
    let mut si: u32;
    let word_size: u32 = core::cmp::min(dstlen, 8);

    w = 0;
    while w < dstlen {
        i = 0;
        while i < word_size && i + w < dstlen {
            si = w + word_size - i - 1;
            *dst.add((w + i) as usize) = if si < srclen { *src.add(si as usize) } else { 0 };
            i += 1;
        }
        w += word_size;
    }
}

/* Read DSP address space using SPI. addr and len have to be 4-byte aligned. */
#[no_mangle]
pub unsafe extern "C" fn rt5677_spi_read(addr: u32, rxbuf: *mut c_void, len: size_t) -> c_int {
    let mut offset: u32;
    let mut status: c_int = 0;
    let mut t: [spi_transfer; 2] = core::mem::zeroed();
    let mut m: spi_message = core::mem::zeroed();
    /* +4 bytes is for the DummyPhase following the AddressPhase */
    let mut header: [u8; RT5677_SPI_HEADER + 4] = [0; RT5677_SPI_HEADER + 4];
    let mut body: [u8; RT5677_SPI_BURST_LEN] = [0; RT5677_SPI_BURST_LEN];
    let mut spi_cmd: u8;
    let cb = rxbuf as *mut u8;

    if g_spi.is_null() {
        return -ENODEV;
    }

    if (addr & 3) != 0 || (len & 3) != 0 {
        dev_err(&mut (*g_spi).dev, b"Bad read align 0x%x(%zu)\n\0".as_ptr() as *const c_char, addr, len);
        return -EACCES;
    }

    t[0].tx_buf = header.as_ptr() as *const c_void;
    t[0].len = header.len() as c_uint;
    t[0].speed_hz = RT5677_SPI_FREQ;
    t[1].rx_buf = body.as_mut_ptr() as *mut c_void;
    t[1].speed_hz = RT5677_SPI_FREQ;
    spi_message_init_with_transfers(&mut m, t.as_mut_ptr(), t.len() as c_uint);

    offset = 0;
    while (offset as size_t) < len {
        spi_cmd = rt5677_spi_select_cmd(
            true,
            (addr + offset) & 7,
            (len - offset as size_t) as u32,
            &mut t[1].len,
        );

        /* Construct SPI message header */
        header[0] = spi_cmd;
        header[1] = (((addr + offset) & 0xff000000) >> 24) as u8;
        header[2] = (((addr + offset) & 0x00ff0000) >> 16) as u8;
        header[3] = (((addr + offset) & 0x0000ff00) >> 8) as u8;
        header[4] = (((addr + offset) & 0x000000ff) >> 0) as u8;

        mutex_lock(&mut spi_mutex);
        status |= spi_sync(g_spi, &mut m);
        mutex_unlock(&mut spi_mutex);

        /* Copy data back to caller buffer */
        rt5677_spi_reverse(cb.add(offset as usize), (len - offset as size_t) as u32, body.as_ptr(), t[1].len);
        offset += t[1].len;
    }
    status
}
/* EXPORT_SYMBOL_GPL(rt5677_spi_read); */

/* Write DSP address space using SPI. addr has to be 4-byte aligned.
 * If len is not 4-byte aligned, then extra zeros are written at the end
 * as padding.
 */
#[no_mangle]
pub unsafe extern "C" fn rt5677_spi_write(addr: u32, txbuf: *const c_void, len: size_t) -> c_int {
    let mut offset: u32;
    let mut status: c_int = 0;
    let mut t: spi_transfer = core::mem::zeroed();
    let mut m: spi_message = core::mem::zeroed();
    /* +1 byte is for the DummyPhase following the DataPhase */
    let mut buf: [u8; RT5677_SPI_HEADER + RT5677_SPI_BURST_LEN + 1] =
        [0; RT5677_SPI_HEADER + RT5677_SPI_BURST_LEN + 1];
    let body = buf.as_mut_ptr().add(RT5677_SPI_HEADER);
    let mut spi_cmd: u8;
    let cb = txbuf as *const u8;

    if g_spi.is_null() {
        return -ENODEV;
    }

    if (addr & 3) != 0 {
        dev_err(&mut (*g_spi).dev, b"Bad write align 0x%x(%zu)\n\0".as_ptr() as *const c_char, addr, len);
        return -EACCES;
    }

    t.tx_buf = buf.as_ptr() as *const c_void;
    t.speed_hz = RT5677_SPI_FREQ;
    spi_message_init_with_transfers(&mut m, &mut t, 1);

    offset = 0;
    while (offset as size_t) < len {
        spi_cmd = rt5677_spi_select_cmd(
            false,
            (addr + offset) & 7,
            (len - offset as size_t) as u32,
            &mut t.len,
        );

        /* Construct SPI message header */
        buf[0] = spi_cmd;
        buf[1] = (((addr + offset) & 0xff000000) >> 24) as u8;
        buf[2] = (((addr + offset) & 0x00ff0000) >> 16) as u8;
        buf[3] = (((addr + offset) & 0x0000ff00) >> 8) as u8;
        buf[4] = (((addr + offset) & 0x000000ff) >> 0) as u8;

        /* Fetch data from caller buffer */
        rt5677_spi_reverse(body, t.len, cb.add(offset as usize), (len - offset as size_t) as u32);
        offset += t.len;
        t.len += (RT5677_SPI_HEADER + 1) as c_uint;

        mutex_lock(&mut spi_mutex);
        status |= spi_sync(g_spi, &mut m);
        mutex_unlock(&mut spi_mutex);
    }
    status
}
/* EXPORT_SYMBOL_GPL(rt5677_spi_write); */

#[no_mangle]
pub unsafe extern "C" fn rt5677_spi_write_firmware(addr: u32, fw: *const firmware) -> c_int {
    rt5677_spi_write(addr, (*fw).data as *const c_void, (*fw).size)
}
/* EXPORT_SYMBOL_GPL(rt5677_spi_write_firmware); */

#[no_mangle]
pub unsafe extern "C" fn rt5677_spi_hotword_detected() {
    let rt5677_dsp: *mut rt5677_dsp;

    if g_spi.is_null() {
        return;
    }

    rt5677_dsp = dev_get_drvdata(&mut (*g_spi).dev) as *mut rt5677_dsp;
    if rt5677_dsp.is_null() {
        dev_err(&mut (*g_spi).dev, b"Can't get rt5677_dsp\n\0".as_ptr() as *const c_char);
        return;
    }

    mutex_lock(&mut (*rt5677_dsp).dma_lock);
    dev_info((*rt5677_dsp).dev, b"Hotword detected\n\0".as_ptr() as *const c_char);
    (*rt5677_dsp).new_hotword = true;
    mutex_unlock(&mut (*rt5677_dsp).dma_lock);

    schedule_delayed_work(&mut (*rt5677_dsp).copy_work, 0);
}
/* EXPORT_SYMBOL_GPL(rt5677_spi_hotword_detected); */

unsafe extern "C" fn rt5677_spi_probe(spi: *mut spi_device) -> c_int {
    let ret: c_int;

    g_spi = spi;

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &rt5677_spi_dai_component,
        &mut rt5677_spi_dai,
        1,
    );
    if ret < 0 {
        dev_err(&mut (*spi).dev, b"Failed to register component.\n\0".as_ptr() as *const c_char);
    }

    ret
}

// #ifdef CONFIG_ACPI
static rt5677_spi_acpi_id: [acpi_device_id; 3] = [
    acpi_device_id { id: [
        b'1' as c_char, b'0' as c_char, b'E' as c_char, b'C' as c_char,
        b'5' as c_char, b'6' as c_char, b'7' as c_char, b'7' as c_char,
        0, 0, 0, 0, 0, 0, 0, 0,
    ] },
    acpi_device_id { id: [
        b'R' as c_char, b'T' as c_char, b'5' as c_char, b'6' as c_char,
        b'7' as c_char, b'7' as c_char, b'A' as c_char, b'A' as c_char,
        0, 0, 0, 0, 0, 0, 0, 0,
    ] },
    acpi_device_id { id: [0; 16] },
];
/* MODULE_DEVICE_TABLE(acpi, rt5677_spi_acpi_id); */
// #endif

static rt5677_spi_ids: [spi_device_id; 2] = [
    spi_device_id { name: [
        b'r' as c_char, b't' as c_char, b'5' as c_char, b'6' as c_char,
        b'7' as c_char, b'7' as c_char, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], driver_data: 0 },
    spi_device_id { name: [0; 32], driver_data: 0 },
];
/* MODULE_DEVICE_TABLE(spi, rt5677_spi_ids); */

static mut rt5677_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: DRV_NAME.as_ptr() as *const c_char,
        acpi_match_table: rt5677_spi_acpi_id.as_ptr(),
    },
    probe: Some(rt5677_spi_probe),
    id_table: rt5677_spi_ids.as_ptr(),
};
/* module_spi_driver(rt5677_spi_driver); */

/* MODULE_DESCRIPTION("ASoC RT5677 SPI driver"); */
/* MODULE_AUTHOR("Oder Chiou <oder_chiou@realtek.com>"); */
/* MODULE_LICENSE("GPL v2"); */

const fn offset_of_rt5677_dsp_copy_work_work() -> usize {
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
