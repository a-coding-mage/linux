// SPDX-License-Identifier: GPL-2.0
/*
 * ALSA SoC Synopsys PIO PCM for I2S driver
 *
 * sound/soc/dwc/designware_pcm.c
 *
 * Copyright (C) 2016 Synopsys
 * Jose Abreu <joabreu@synopsys.com>
 */

// C dependencies: <linux/io.h>, <linux/rcupdate.h>, <sound/pcm.h>,
// <sound/pcm_params.h>, and "local.h".

pub const PERIOD_BYTES_MIN: u32 = 4096;
pub const BUFFER_BYTES_MAX: u32 = 3 * 2 * 8 * PERIOD_BYTES_MIN;
pub const PERIODS_MIN: u32 = 2;

type bool_ = bool;
type size_t = usize;
type snd_pcm_uframes_t = libc::c_ulong;

extern "C" {
    static SNDRV_PCM_INFO_INTERLEAVED: libc::c_uint;
    static SNDRV_PCM_INFO_MMAP: libc::c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: libc::c_uint;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: libc::c_uint;
    static SNDRV_PCM_RATE_32000: libc::c_uint;
    static SNDRV_PCM_RATE_44100: libc::c_uint;
    static SNDRV_PCM_RATE_48000: libc::c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;

    static SNDRV_PCM_FORMAT_S16_LE: libc::c_int;
    static SNDRV_PCM_FORMAT_S24_LE: libc::c_int;
    static SNDRV_PCM_FORMAT_S32_LE: libc::c_int;
    static SNDRV_PCM_HW_PARAM_PERIODS: libc::c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: libc::c_int;
    static SNDRV_PCM_TRIGGER_START: libc::c_int;
    static SNDRV_PCM_TRIGGER_RESUME: libc::c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: libc::c_int;
    static SNDRV_PCM_TRIGGER_STOP: libc::c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: libc::c_int;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: libc::c_int;
    static SNDRV_DMA_TYPE_CONTINUOUS: libc::c_int;
    static EINVAL: libc::c_int;

    fn iowrite32(value: u32, addr: *mut libc::c_void);
    fn ioread32(addr: *mut libc::c_void) -> u32;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn synchronize_rcu();
    fn snd_pcm_running(substream: *mut snd_pcm_substream) -> libc::c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_substream_to_rtd(
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(
        rtd: *mut snd_soc_pcm_runtime,
        num: libc::c_int,
    ) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut libc::c_void;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    );
    fn snd_pcm_hw_constraint_integer(
        runtime: *mut snd_pcm_runtime,
        var: libc::c_int,
    ) -> libc::c_int;
    fn params_channels(hw_params: *mut snd_pcm_hw_params) -> libc::c_uint;
    fn params_format(hw_params: *mut snd_pcm_hw_params) -> libc::c_int;
    fn dev_err(dev: *mut device, fmt: *const libc::c_char, ...);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: libc::c_int,
        data: *mut libc::c_void,
        size: size_t,
        max: size_t,
    );
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut libc::c_void,
        num_dai: libc::c_int,
    ) -> libc::c_int;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_area: *mut libc::c_void,
    pub period_size: snd_pcm_uframes_t,
    pub buffer_size: snd_pcm_uframes_t,
    pub private_data: *mut libc::c_void,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: libc::c_int,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: libc::c_uint,
    pub rates: libc::c_uint,
    pub rate_min: libc::c_uint,
    pub rate_max: libc::c_uint,
    pub formats: u64,
    pub channels_min: libc::c_uint,
    pub channels_max: libc::c_uint,
    pub buffer_bytes_max: size_t,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: libc::c_uint,
    pub periods_max: libc::c_uint,
    pub fifo_size: size_t,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub open: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            substream: *mut snd_pcm_substream,
        ) -> libc::c_int,
    >,
    pub close: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            substream: *mut snd_pcm_substream,
        ) -> libc::c_int,
    >,
    pub hw_params: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            substream: *mut snd_pcm_substream,
            hw_params: *mut snd_pcm_hw_params,
        ) -> libc::c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            substream: *mut snd_pcm_substream,
            cmd: libc::c_int,
        ) -> libc::c_int,
    >,
    pub pointer: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            substream: *mut snd_pcm_substream,
        ) -> snd_pcm_uframes_t,
    >,
    pub pcm_new: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            rtd: *mut snd_soc_pcm_runtime,
        ) -> libc::c_int,
    >,
}

pub type dw_pcm_tx_fn_t = unsafe extern "C" fn(
    dev: *mut dw_i2s_dev,
    runtime: *mut snd_pcm_runtime,
    tx_ptr: libc::c_uint,
    period_elapsed: *mut bool_,
) -> libc::c_uint;

pub type dw_pcm_rx_fn_t = unsafe extern "C" fn(
    dev: *mut dw_i2s_dev,
    runtime: *mut snd_pcm_runtime,
    rx_ptr: libc::c_uint,
    period_elapsed: *mut bool_,
) -> libc::c_uint;

#[repr(C)]
pub struct dw_i2s_dev {
    pub fifo_th: libc::c_int,
    pub i2s_base: *mut u8,
    pub l_reg: usize,
    pub r_reg: usize,
    pub tx_substream: *mut snd_pcm_substream,
    pub rx_substream: *mut snd_pcm_substream,
    pub tx_ptr: libc::c_uint,
    pub rx_ptr: libc::c_uint,
    pub tx_fn: dw_pcm_tx_fn_t,
    pub rx_fn: dw_pcm_rx_fn_t,
    pub dev: *mut device,
}

unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    core::ptr::read_volatile(p)
}

unsafe fn WRITE_ONCE<T>(p: *mut T, v: T) {
    core::ptr::write_volatile(p, v);
}

unsafe fn rcu_dereference<T>(p: *mut T) -> *mut T {
    READ_ONCE(&p)
}

unsafe fn rcu_assign_pointer<T>(p: *mut *mut T, v: *mut T) {
    WRITE_ONCE(p, v);
}

unsafe fn cmpxchg<T: PartialEq + Copy>(p: *mut T, old: T, new: T) -> T {
    let prev = READ_ONCE(p);
    if prev == old {
        WRITE_ONCE(p, new);
    }
    prev
}

unsafe extern "C" fn dw_pcm_tx_16(
    dev: *mut dw_i2s_dev,
    runtime: *mut snd_pcm_runtime,
    mut tx_ptr: libc::c_uint,
    period_elapsed: *mut bool_,
) -> libc::c_uint {
    let p = (*runtime).dma_area as *const [u16; 2];
    let mut period_pos = tx_ptr as snd_pcm_uframes_t % (*runtime).period_size;
    let mut i: libc::c_int = 0;

    while i < (*dev).fifo_th {
        iowrite32((*p.add(tx_ptr as usize))[0] as u32, (*dev).i2s_base.add((*dev).l_reg) as *mut libc::c_void);
        iowrite32((*p.add(tx_ptr as usize))[1] as u32, (*dev).i2s_base.add((*dev).r_reg) as *mut libc::c_void);
        period_pos = period_pos.wrapping_add(1);
        tx_ptr = tx_ptr.wrapping_add(1);
        if tx_ptr as snd_pcm_uframes_t >= (*runtime).buffer_size {
            tx_ptr = 0;
        }
        i += 1;
    }
    *period_elapsed = period_pos >= (*runtime).period_size;
    tx_ptr
}

unsafe extern "C" fn dw_pcm_tx_32(
    dev: *mut dw_i2s_dev,
    runtime: *mut snd_pcm_runtime,
    mut tx_ptr: libc::c_uint,
    period_elapsed: *mut bool_,
) -> libc::c_uint {
    let p = (*runtime).dma_area as *const [u32; 2];
    let mut period_pos = tx_ptr as snd_pcm_uframes_t % (*runtime).period_size;
    let mut i: libc::c_int = 0;

    while i < (*dev).fifo_th {
        iowrite32((*p.add(tx_ptr as usize))[0], (*dev).i2s_base.add((*dev).l_reg) as *mut libc::c_void);
        iowrite32((*p.add(tx_ptr as usize))[1], (*dev).i2s_base.add((*dev).r_reg) as *mut libc::c_void);
        period_pos = period_pos.wrapping_add(1);
        tx_ptr = tx_ptr.wrapping_add(1);
        if tx_ptr as snd_pcm_uframes_t >= (*runtime).buffer_size {
            tx_ptr = 0;
        }
        i += 1;
    }
    *period_elapsed = period_pos >= (*runtime).period_size;
    tx_ptr
}

unsafe extern "C" fn dw_pcm_rx_16(
    dev: *mut dw_i2s_dev,
    runtime: *mut snd_pcm_runtime,
    mut rx_ptr: libc::c_uint,
    period_elapsed: *mut bool_,
) -> libc::c_uint {
    let p = (*runtime).dma_area as *mut [u16; 2];
    let mut period_pos = rx_ptr as snd_pcm_uframes_t % (*runtime).period_size;
    let mut i: libc::c_int = 0;

    while i < (*dev).fifo_th {
        (*p.add(rx_ptr as usize))[0] = ioread32((*dev).i2s_base.add((*dev).l_reg) as *mut libc::c_void) as u16;
        (*p.add(rx_ptr as usize))[1] = ioread32((*dev).i2s_base.add((*dev).r_reg) as *mut libc::c_void) as u16;
        period_pos = period_pos.wrapping_add(1);
        rx_ptr = rx_ptr.wrapping_add(1);
        if rx_ptr as snd_pcm_uframes_t >= (*runtime).buffer_size {
            rx_ptr = 0;
        }
        i += 1;
    }
    *period_elapsed = period_pos >= (*runtime).period_size;
    rx_ptr
}

unsafe extern "C" fn dw_pcm_rx_32(
    dev: *mut dw_i2s_dev,
    runtime: *mut snd_pcm_runtime,
    mut rx_ptr: libc::c_uint,
    period_elapsed: *mut bool_,
) -> libc::c_uint {
    let p = (*runtime).dma_area as *mut [u32; 2];
    let mut period_pos = rx_ptr as snd_pcm_uframes_t % (*runtime).period_size;
    let mut i: libc::c_int = 0;

    while i < (*dev).fifo_th {
        (*p.add(rx_ptr as usize))[0] = ioread32((*dev).i2s_base.add((*dev).l_reg) as *mut libc::c_void);
        (*p.add(rx_ptr as usize))[1] = ioread32((*dev).i2s_base.add((*dev).r_reg) as *mut libc::c_void);
        period_pos = period_pos.wrapping_add(1);
        rx_ptr = rx_ptr.wrapping_add(1);
        if rx_ptr as snd_pcm_uframes_t >= (*runtime).buffer_size {
            rx_ptr = 0;
        }
        i += 1;
    }
    *period_elapsed = period_pos >= (*runtime).period_size;
    rx_ptr
}

static dw_pcm_hardware: snd_pcm_hardware = unsafe {
    snd_pcm_hardware {
        info: SNDRV_PCM_INFO_INTERLEAVED
            | SNDRV_PCM_INFO_MMAP
            | SNDRV_PCM_INFO_MMAP_VALID
            | SNDRV_PCM_INFO_BLOCK_TRANSFER,
        rates: SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000,
        rate_min: 32000,
        rate_max: 48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
        channels_min: 2,
        channels_max: 2,
        buffer_bytes_max: BUFFER_BYTES_MAX as size_t,
        period_bytes_min: PERIOD_BYTES_MIN as size_t,
        period_bytes_max: (BUFFER_BYTES_MAX / PERIODS_MIN) as size_t,
        periods_min: PERIODS_MIN,
        periods_max: BUFFER_BYTES_MAX / PERIOD_BYTES_MIN,
        fifo_size: 16,
    }
};

unsafe extern "C" fn dw_pcm_transfer(dev: *mut dw_i2s_dev, push: bool_) {
    let substream: *mut snd_pcm_substream;
    let active: bool_;
    let mut period_elapsed: bool_ = false;

    rcu_read_lock();
    if push {
        substream = rcu_dereference((*dev).tx_substream);
    } else {
        substream = rcu_dereference((*dev).rx_substream);
    }
    active = !substream.is_null() && snd_pcm_running(substream) != 0;
    if active {
        let ptr: libc::c_uint;
        let new_ptr: libc::c_uint;

        if push {
            ptr = READ_ONCE(&(*dev).tx_ptr);
            new_ptr = ((*dev).tx_fn)(dev, (*substream).runtime, ptr, &mut period_elapsed);
            cmpxchg(&mut (*dev).tx_ptr, ptr, new_ptr);
        } else {
            ptr = READ_ONCE(&(*dev).rx_ptr);
            new_ptr = ((*dev).rx_fn)(dev, (*substream).runtime, ptr, &mut period_elapsed);
            cmpxchg(&mut (*dev).rx_ptr, ptr, new_ptr);
        }

        if period_elapsed {
            snd_pcm_period_elapsed(substream);
        }
    }
    rcu_read_unlock();
}

#[no_mangle]
pub unsafe extern "C" fn dw_pcm_push_tx(dev: *mut dw_i2s_dev) {
    dw_pcm_transfer(dev, true);
}

#[no_mangle]
pub unsafe extern "C" fn dw_pcm_pop_rx(dev: *mut dw_i2s_dev) {
    dw_pcm_transfer(dev, false);
}

unsafe extern "C" fn dw_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> libc::c_int {
    let runtime = (*substream).runtime;
    let rtd = snd_soc_substream_to_rtd(substream);
    let dev = snd_soc_dai_get_drvdata(snd_soc_rtd_to_cpu(rtd, 0)) as *mut dw_i2s_dev;

    snd_soc_set_runtime_hwparams(substream, &dw_pcm_hardware);
    snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    (*runtime).private_data = dev as *mut libc::c_void;

    0
}

unsafe extern "C" fn dw_pcm_close(
    _component: *mut snd_soc_component,
    _substream: *mut snd_pcm_substream,
) -> libc::c_int {
    synchronize_rcu();
    0
}

unsafe extern "C" fn dw_pcm_hw_params(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> libc::c_int {
    let runtime = (*substream).runtime;
    let dev = (*runtime).private_data as *mut dw_i2s_dev;

    match params_channels(hw_params) {
        2 => {}
        _ => {
            dev_err((*dev).dev, b"invalid channels number\n\0".as_ptr() as *const libc::c_char);
            return -EINVAL;
        }
    }

    if params_format(hw_params) == SNDRV_PCM_FORMAT_S16_LE {
        (*dev).tx_fn = dw_pcm_tx_16;
        (*dev).rx_fn = dw_pcm_rx_16;
    } else if params_format(hw_params) == SNDRV_PCM_FORMAT_S24_LE
        || params_format(hw_params) == SNDRV_PCM_FORMAT_S32_LE
    {
        (*dev).tx_fn = dw_pcm_tx_32;
        (*dev).rx_fn = dw_pcm_rx_32;
    } else {
        dev_err((*dev).dev, b"invalid format\n\0".as_ptr() as *const libc::c_char);
        return -EINVAL;
    }

    0
}

unsafe extern "C" fn dw_pcm_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: libc::c_int,
) -> libc::c_int {
    let runtime = (*substream).runtime;
    let dev = (*runtime).private_data as *mut dw_i2s_dev;
    let mut ret: libc::c_int = 0;

    if cmd == SNDRV_PCM_TRIGGER_START
        || cmd == SNDRV_PCM_TRIGGER_RESUME
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE
    {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            WRITE_ONCE(&mut (*dev).tx_ptr, 0);
            rcu_assign_pointer(&mut (*dev).tx_substream, substream);
        } else {
            WRITE_ONCE(&mut (*dev).rx_ptr, 0);
            rcu_assign_pointer(&mut (*dev).rx_substream, substream);
        }
    } else if cmd == SNDRV_PCM_TRIGGER_STOP
        || cmd == SNDRV_PCM_TRIGGER_SUSPEND
        || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH
    {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            rcu_assign_pointer(&mut (*dev).tx_substream, core::ptr::null_mut());
        } else {
            rcu_assign_pointer(&mut (*dev).rx_substream, core::ptr::null_mut());
        }
    } else {
        ret = -EINVAL;
    }

    ret
}

unsafe extern "C" fn dw_pcm_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let dev = (*runtime).private_data as *mut dw_i2s_dev;
    let pos: snd_pcm_uframes_t;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        pos = READ_ONCE(&(*dev).tx_ptr) as snd_pcm_uframes_t;
    } else {
        pos = READ_ONCE(&(*dev).rx_ptr) as snd_pcm_uframes_t;
    }

    if pos < (*runtime).buffer_size {
        pos
    } else {
        0
    }
}

unsafe extern "C" fn dw_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> libc::c_int {
    let size: size_t = dw_pcm_hardware.buffer_bytes_max;

    snd_pcm_set_managed_buffer_all(
        (*rtd).pcm,
        SNDRV_DMA_TYPE_CONTINUOUS,
        core::ptr::null_mut(),
        size,
        size,
    );
    0
}

static dw_pcm_component: snd_soc_component_driver = snd_soc_component_driver {
    open: Some(dw_pcm_open),
    close: Some(dw_pcm_close),
    hw_params: Some(dw_pcm_hw_params),
    trigger: Some(dw_pcm_trigger),
    pointer: Some(dw_pcm_pointer),
    pcm_new: Some(dw_pcm_new),
};

#[no_mangle]
pub unsafe extern "C" fn dw_pcm_register(pdev: *mut platform_device) -> libc::c_int {
    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &dw_pcm_component,
        core::ptr::null_mut(),
        0,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
