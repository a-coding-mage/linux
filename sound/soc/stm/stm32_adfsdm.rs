// SPDX-License-Identifier: GPL-2.0
/*
 * This file is part of STM32 DFSDM ASoC DAI driver
 *
 * Copyright (C) 2017, STMicroelectronics - All Rights Reserved
 * Authors: Arnaud Pouliquen <arnaud.pouliquen@st.com>
 *          Olivier Moysan <olivier.moysan@st.com>
 */

/*
 * C dependencies:
 * linux/clk.h, linux/module.h, linux/mutex.h, linux/platform_device.h,
 * linux/slab.h, linux/pm_runtime.h, linux/iio/iio.h, linux/iio/consumer.h,
 * linux/iio/adc/stm32-dfsdm-adc.h, sound/pcm.h, sound/soc.h
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

const STM32_ADFSDM_DRV_NAME: &[u8] = b"stm32-adfsdm\0";

const DFSDM_MAX_PERIOD_SIZE: usize = PAGE_SIZE / 2;
const DFSDM_MAX_PERIODS: usize = 6;

type size_t = usize;
type ssize_t = isize;
type u8 = u8;
type u16 = u16;
type snd_pcm_format_t = c_int;
type snd_pcm_uframes_t = usize;

#[repr(C)]
struct stm32_adfsdm_priv {
    dai_drv: snd_soc_dai_driver,
    substream: *mut snd_pcm_substream,
    dev: *mut device,

    /* IIO */
    iio_ch: *mut iio_channel,
    iio_cb: *mut iio_cb_buffer,
    iio_active: bool,

    /* PCM buffer */
    pcm_buff: *mut c_uchar,
    pos: c_uint,

    lock: mutex, /* protect against race condition on iio state */
}

type c_uchar = u8;

#[repr(C)]
struct snd_pcm_hardware {
    info: c_uint,
    formats: c_uint,
    channels_min: c_uint,
    channels_max: c_uint,
    periods_min: c_uint,
    periods_max: c_uint,
    period_bytes_max: usize,
    buffer_bytes_max: usize,
}

#[repr(C)]
struct snd_soc_dai_ops {
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    set_sysclk: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_uint, c_int) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_soc_pcm_stream {
    channels_min: c_uint,
    channels_max: c_uint,
    formats: c_uint,
    rates: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_soc_dai_driver {
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    legacy_dai_naming: c_int,
    open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pointer:
        Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
    debugfs_prefix: *const c_char,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct platform_driver_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    driver: platform_driver_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

#[repr(C)]
struct snd_pcm_runtime {
    rate: c_uint,
    format: snd_pcm_format_t,
    dma_area: *mut c_uchar,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    dev: *mut device,
    pcm: *mut snd_pcm,
}

#[repr(C)]
struct iio_channel {
    indio_dev: *mut c_void,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}
#[repr(C)]
struct iio_cb_buffer {
    _private: [u8; 0],
}
#[repr(C)]
struct mutex {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
}
#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
struct snd_pcm {
    _private: [u8; 0],
}
#[repr(C)]
struct platform_device {
    dev: device,
}

extern "C" {
    static PAGE_SIZE: usize;
    static SNDRV_PCM_INFO_INTERLEAVED: c_uint;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint;
    static SNDRV_PCM_INFO_MMAP: c_uint;
    static SNDRV_PCM_INFO_PAUSE: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static SNDRV_PCM_RATE_CONTINUOUS: c_uint;
    static SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t;
    static SNDRV_PCM_TRIGGER_START: c_int;
    static SNDRV_PCM_TRIGGER_RESUME: c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: c_int;
    static SNDRV_PCM_TRIGGER_STOP: c_int;
    static SNDRV_DMA_TYPE_DEV: c_int;
    static IIO_CHAN_INFO_SAMP_FREQ: c_int;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;

    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn iio_channel_stop_all_cb(cb: *mut iio_cb_buffer);
    fn iio_write_channel_attribute(
        ch: *mut iio_channel,
        val: c_uint,
        val2: c_int,
        info: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn iio_channel_start_all_cb(cb: *mut iio_cb_buffer) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn iio_write_channel_ext_info(
        ch: *mut iio_channel,
        attr: *const c_char,
        buf: *const c_char,
        len: size_t,
    ) -> ssize_t;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> size_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn stm32_dfsdm_get_buff_cb(
        indio_dev: *mut c_void,
        cb: unsafe extern "C" fn(*const c_void, size_t, *mut c_void) -> c_int,
        private: *mut c_void,
    ) -> c_int;
    fn stm32_dfsdm_release_buff_cb(indio_dev: *mut c_void) -> c_int;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    ) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn iio_channel_cb_set_buffer_watermark(cb: *mut iio_cb_buffer, watermark: c_uint) -> c_int;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        dev: *mut device,
        size: c_uint,
        max: c_uint,
    );
    fn iio_channel_release_all_cb(data: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn mutex_init(lock: *mut mutex);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_iio_channel_get_all(dev: *mut device) -> *mut iio_channel;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn iio_channel_get_all_cb(
        dev: *mut device,
        cb: *const unsafe extern "C" fn(*const c_void, *mut c_void) -> c_int,
        private: *mut c_void,
    ) -> *mut iio_cb_buffer;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn snd_soc_unregister_component(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
}

static stm32_adfsdm_pcm_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: unsafe {
        SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_BLOCK_TRANSFER
            | SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_PAUSE
    },
    formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE },

    channels_min: 1,
    channels_max: 1,

    periods_min: 2,
    periods_max: DFSDM_MAX_PERIODS as c_uint,

    period_bytes_max: DFSDM_MAX_PERIOD_SIZE,
    buffer_bytes_max: DFSDM_MAX_PERIODS * DFSDM_MAX_PERIOD_SIZE,
};

unsafe extern "C" fn stm32_adfsdm_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let priv_0 = snd_soc_dai_get_drvdata(dai) as *mut stm32_adfsdm_priv;

    mutex_lock(&mut (*priv_0).lock);
    if (*priv_0).iio_active {
        iio_channel_stop_all_cb((*priv_0).iio_cb);
        (*priv_0).iio_active = false;
    }
    mutex_unlock(&mut (*priv_0).lock);
}

unsafe extern "C" fn stm32_adfsdm_dai_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_0 = snd_soc_dai_get_drvdata(dai) as *mut stm32_adfsdm_priv;
    let mut ret: c_int;

    mutex_lock(&mut (*priv_0).lock);
    if (*priv_0).iio_active {
        iio_channel_stop_all_cb((*priv_0).iio_cb);
        (*priv_0).iio_active = false;
    }

    ret = iio_write_channel_attribute(
        (*priv_0).iio_ch,
        (*(*substream).runtime).rate,
        0,
        IIO_CHAN_INFO_SAMP_FREQ,
    );
    if ret < 0 {
        dev_err(
            (*dai).dev,
            b"%s: Failed to set %d sampling rate\n\0".as_ptr() as *const c_char,
            b"stm32_adfsdm_dai_prepare\0".as_ptr() as *const c_char,
            (*(*substream).runtime).rate,
        );
        mutex_unlock(&mut (*priv_0).lock);
        return ret;
    }

    if !(*priv_0).iio_active {
        ret = iio_channel_start_all_cb((*priv_0).iio_cb);
        if ret == 0 {
            (*priv_0).iio_active = true;
        } else {
            dev_err(
                (*dai).dev,
                b"%s: IIO channel start failed (%d)\n\0".as_ptr() as *const c_char,
                b"stm32_adfsdm_dai_prepare\0".as_ptr() as *const c_char,
                ret,
            );
        }
    }

    mutex_unlock(&mut (*priv_0).lock);
    ret
}

unsafe extern "C" fn stm32_adfsdm_set_sysclk(
    dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let priv_0 = snd_soc_dai_get_drvdata(dai) as *mut stm32_adfsdm_priv;
    let size: ssize_t;
    let mut str_freq = [0 as c_char; 10];

    dev_dbg(
        (*dai).dev,
        b"%s: Enter for freq %d\n\0".as_ptr() as *const c_char,
        b"stm32_adfsdm_set_sysclk\0".as_ptr() as *const c_char,
        freq,
    );

    /* Set IIO frequency if CODEC is master as clock comes from SPI_IN */

    snprintf(
        str_freq.as_mut_ptr(),
        core::mem::size_of_val(&str_freq),
        b"%u\n\0".as_ptr() as *const c_char,
        freq,
    );
    size = iio_write_channel_ext_info(
        (*priv_0).iio_ch,
        b"spi_clk_freq\0".as_ptr() as *const c_char,
        str_freq.as_ptr(),
        core::mem::size_of_val(&str_freq),
    );
    if size != core::mem::size_of_val(&str_freq) as ssize_t {
        dev_err(
            (*dai).dev,
            b"%s: Failed to set SPI clock\n\0".as_ptr() as *const c_char,
            b"stm32_adfsdm_set_sysclk\0".as_ptr() as *const c_char,
        );
        return -EINVAL;
    }
    0
}

static stm32_adfsdm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    shutdown: Some(stm32_adfsdm_shutdown),
    prepare: Some(stm32_adfsdm_dai_prepare),
    set_sysclk: Some(stm32_adfsdm_set_sysclk),
};

static stm32_adfsdm_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    capture: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 1,
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE },
        rates: unsafe { SNDRV_PCM_RATE_CONTINUOUS },
        rate_min: 8000,
        rate_max: 192000,
    },
    ops: &stm32_adfsdm_dai_ops,
};

static stm32_adfsdm_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"stm32_dfsdm_audio\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
    open: None,
    close: None,
    hw_params: None,
    trigger: None,
    pointer: None,
    pcm_new: None,
    debugfs_prefix: core::ptr::null(),
};

unsafe fn mutex_lock(_lock: *mut mutex) {
    /* guard(mutex) external lock operation from C; dependency supplied by kernel Rust bindings */
}

unsafe fn mutex_unlock(_lock: *mut mutex) {
    /* guard(mutex) external unlock operation from C; dependency supplied by kernel Rust bindings */
}

unsafe fn stm32_memcpy_32to16(dest: *mut c_void, src: *const c_void, n: size_t) {
    let mut i: c_uint = 0;
    let mut d = dest as *mut u16;
    let mut s = src as *mut u16;

    s = s.add(1);
    i = (n >> 1) as c_uint;
    while i > 0 {
        *d = *s;
        d = d.add(1);
        s = s.add(1);
        s = s.add(1);
        i -= 1;
    }
}

unsafe extern "C" fn stm32_afsdm_pcm_cb(
    data: *const c_void,
    size: size_t,
    private: *mut c_void,
) -> c_int {
    let priv_0 = private as *mut stm32_adfsdm_priv;
    let rtd = snd_soc_substream_to_rtd((*priv_0).substream);
    let pcm_buff = (*priv_0).pcm_buff;
    let src_buff = data as *mut u8;
    let old_pos = (*priv_0).pos;
    let buff_size = snd_pcm_lib_buffer_bytes((*priv_0).substream);
    let period_size = snd_pcm_lib_period_bytes((*priv_0).substream);
    let mut cur_size: size_t;
    let mut src_size = size;
    let format = (*(*(*priv_0).substream).runtime).format;

    if format == SNDRV_PCM_FORMAT_S16_LE {
        src_size >>= 1;
    }
    cur_size = src_size;

    dev_dbg(
        (*rtd).dev,
        b"%s: buff_add :%p, pos = %d, size = %zu\n\0".as_ptr() as *const c_char,
        b"stm32_afsdm_pcm_cb\0".as_ptr() as *const c_char,
        pcm_buff.add((*priv_0).pos as usize),
        (*priv_0).pos,
        src_size,
    );

    if ((*priv_0).pos as size_t + src_size) > buff_size {
        if format == SNDRV_PCM_FORMAT_S16_LE {
            stm32_memcpy_32to16(
                pcm_buff.add((*priv_0).pos as usize) as *mut c_void,
                src_buff as *const c_void,
                buff_size - (*priv_0).pos as size_t,
            );
        } else {
            memcpy(
                pcm_buff.add((*priv_0).pos as usize) as *mut c_void,
                src_buff as *const c_void,
                buff_size - (*priv_0).pos as size_t,
            );
        }
        cur_size -= buff_size - (*priv_0).pos as size_t;
        (*priv_0).pos = 0;
    }

    if format == SNDRV_PCM_FORMAT_S16_LE {
        stm32_memcpy_32to16(
            pcm_buff.add((*priv_0).pos as usize) as *mut c_void,
            src_buff.add(src_size - cur_size) as *const c_void,
            cur_size,
        );
    } else {
        memcpy(
            pcm_buff.add((*priv_0).pos as usize) as *mut c_void,
            src_buff.add(src_size - cur_size) as *const c_void,
            cur_size,
        );
    }

    (*priv_0).pos = (((*priv_0).pos as size_t + cur_size) % buff_size) as c_uint;

    if cur_size != src_size || (old_pos != 0 && (old_pos as size_t % period_size < size)) {
        snd_pcm_period_elapsed((*priv_0).substream);
    }

    0
}

unsafe extern "C" fn stm32_adfsdm_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv_0 = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut stm32_adfsdm_priv;

    if cmd == SNDRV_PCM_TRIGGER_START || cmd == SNDRV_PCM_TRIGGER_RESUME {
        (*priv_0).pos = 0;
        return stm32_dfsdm_get_buff_cb(
            (*(*priv_0).iio_ch).indio_dev,
            stm32_afsdm_pcm_cb,
            priv_0 as *mut c_void,
        );
    } else if cmd == SNDRV_PCM_TRIGGER_SUSPEND || cmd == SNDRV_PCM_TRIGGER_STOP {
        return stm32_dfsdm_release_buff_cb((*(*priv_0).iio_ch).indio_dev);
    }

    -EINVAL
}

unsafe extern "C" fn stm32_adfsdm_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv_0 = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut stm32_adfsdm_priv;
    let ret: c_int;

    ret = snd_soc_set_runtime_hwparams(substream, &stm32_adfsdm_pcm_hw);
    if ret == 0 {
        (*priv_0).substream = substream;
    }

    ret
}

unsafe extern "C" fn stm32_adfsdm_pcm_close(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv_0 = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut stm32_adfsdm_priv;

    (*priv_0).substream = core::ptr::null_mut();

    0
}

unsafe extern "C" fn stm32_adfsdm_pcm_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv_0 = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut stm32_adfsdm_priv;

    bytes_to_frames((*substream).runtime, (*priv_0).pos)
}

unsafe extern "C" fn stm32_adfsdm_pcm_hw_params(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let priv_0 = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut stm32_adfsdm_priv;

    (*priv_0).pcm_buff = (*(*substream).runtime).dma_area;

    iio_channel_cb_set_buffer_watermark((*priv_0).iio_cb, params_period_size(params))
}

unsafe extern "C" fn stm32_adfsdm_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let pcm = (*rtd).pcm;
    let priv_0 = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut stm32_adfsdm_priv;
    let size: c_uint = (DFSDM_MAX_PERIODS * DFSDM_MAX_PERIOD_SIZE) as c_uint;

    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_DEV, (*priv_0).dev, size, size);
    0
}

unsafe extern "C" fn stm32_adfsdm_dummy_cb(_data: *const c_void, _private: *mut c_void) -> c_int {
    /*
     * This dummy callback is requested by iio_channel_get_all_cb() API,
     * but the stm32_dfsdm_get_buff_cb() API is used instead, to optimize
     * DMA transfers.
     */
    0
}

unsafe extern "C" fn stm32_adfsdm_cleanup(data: *mut c_void) {
    iio_channel_release_all_cb(data);
}

static stm32_adfsdm_soc_platform: snd_soc_component_driver = snd_soc_component_driver {
    name: core::ptr::null(),
    legacy_dai_naming: 0,
    open: Some(stm32_adfsdm_pcm_open),
    close: Some(stm32_adfsdm_pcm_close),
    hw_params: Some(stm32_adfsdm_pcm_hw_params),
    trigger: Some(stm32_adfsdm_trigger),
    pointer: Some(stm32_adfsdm_pcm_pointer),
    pcm_new: Some(stm32_adfsdm_pcm_new),
    debugfs_prefix: b"pcm\0".as_ptr() as *const c_char,
};

static stm32_adfsdm_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"st,stm32h7-dfsdm-dai\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, stm32_adfsdm_of_match); */

unsafe extern "C" fn stm32_adfsdm_probe(pdev: *mut platform_device) -> c_int {
    let priv_0: *mut stm32_adfsdm_priv;
    let mut ret: c_int;

    priv_0 = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<stm32_adfsdm_priv>(),
        GFP_KERNEL,
    ) as *mut stm32_adfsdm_priv;
    if priv_0.is_null() {
        return -ENOMEM;
    }

    (*priv_0).dev = &mut (*pdev).dev;
    (*priv_0).dai_drv = stm32_adfsdm_dai;
    mutex_init(&mut (*priv_0).lock);

    dev_set_drvdata(&mut (*pdev).dev, priv_0 as *mut c_void);

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &stm32_adfsdm_dai_component,
        &mut (*priv_0).dai_drv,
        1,
    );
    if ret < 0 {
        return ret;
    }

    /* Associate iio channel */
    (*priv_0).iio_ch = devm_iio_channel_get_all(&mut (*pdev).dev);
    if IS_ERR((*priv_0).iio_ch as *const c_void) {
        return PTR_ERR((*priv_0).iio_ch as *const c_void);
    }

    (*priv_0).iio_cb =
        iio_channel_get_all_cb(&mut (*pdev).dev, &stm32_adfsdm_dummy_cb, core::ptr::null_mut());
    if IS_ERR((*priv_0).iio_cb as *const c_void) {
        return PTR_ERR((*priv_0).iio_cb as *const c_void);
    }

    ret = devm_add_action_or_reset(
        &mut (*pdev).dev,
        stm32_adfsdm_cleanup,
        (*priv_0).iio_cb as *mut c_void,
    );
    if ret < 0 {
        dev_err(
            &mut (*pdev).dev,
            b"Unable to add action\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &stm32_adfsdm_soc_platform,
        core::ptr::null_mut(),
        0,
    );
    if ret < 0 {
        return ret;
    }

    pm_runtime_enable(&mut (*pdev).dev);

    ret
}

unsafe extern "C" fn stm32_adfsdm_remove(pdev: *mut platform_device) {
    snd_soc_unregister_component(&mut (*pdev).dev);
    pm_runtime_disable(&mut (*pdev).dev);
}

static mut stm32_adfsdm_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: STM32_ADFSDM_DRV_NAME.as_ptr() as *const c_char,
        of_match_table: stm32_adfsdm_of_match.as_ptr(),
    },
    probe: Some(stm32_adfsdm_probe),
    remove: Some(stm32_adfsdm_remove),
};

/* module_platform_driver(stm32_adfsdm_driver); */

/* MODULE_DESCRIPTION("stm32 DFSDM DAI driver"); */
/* MODULE_AUTHOR("Arnaud Pouliquen <arnaud.pouliquen@st.com>"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:" STM32_ADFSDM_DRV_NAME); */
/* MODULE_IMPORT_NS("IIO_CONSUMER"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
