// SPDX-License-Identifier: GPL-2.0+
// imx-pcm-fiq.c  --  ALSA Soc Audio Layer
//
// Copyright 2009 Sascha Hauer <s.hauer@pengutronix.de>
//
// This code is based on code copyrighted by Freescale,
// Liam Girdwood, Javier Martin and probably others.

// Dependencies from:
// <linux/clk.h>, <linux/delay.h>, <linux/device.h>, <linux/dma-mapping.h>,
// <linux/init.h>, <linux/interrupt.h>, <linux/module.h>,
// <linux/platform_device.h>, <linux/slab.h>, ALSA SoC headers, <asm/fiq.h>,
// <linux/platform_data/asoc-imx-ssi.h>, "imx-ssi.h", and "imx-pcm.h".

#[repr(C)]
pub struct imx_pcm_runtime_data {
    period: core::ffi::c_uint,
    periods: core::ffi::c_int,
    offset: core::ffi::c_ulong,
    hrt: hrtimer,
    poll_time_ns: core::ffi::c_int,
    substream: *mut snd_pcm_substream,
    playing: atomic_t,
    capturing: atomic_t,
}

unsafe extern "C" {
    static mut imx_ssi_fiq_tx_buffer: core::ffi::c_ulong;
    static mut imx_ssi_fiq_rx_buffer: core::ffi::c_ulong;
    static mut imx_ssi_fiq_base: core::ffi::c_ulong;
    static imx_ssi_fiq_start: core::ffi::c_uchar;
    static imx_ssi_fiq_end: core::ffi::c_uchar;

    fn get_fiq_regs(regs: *mut pt_regs);
    fn set_fiq_regs(regs: *mut pt_regs);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn hrtimer_forward_now(timer: *mut hrtimer, interval: ktime_t) -> u64;
    fn ns_to_ktime(ns: core::ffi::c_int) -> ktime_t;
    fn params_periods(params: *mut snd_pcm_hw_params) -> core::ffi::c_int;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> core::ffi::c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> core::ffi::c_uint;
    fn params_period_size(params: *mut snd_pcm_hw_params) -> core::ffi::c_uint;
    fn atomic_read(v: *const atomic_t) -> core::ffi::c_int;
    fn atomic_set(v: *mut atomic_t, i: core::ffi::c_int);
    fn hrtimer_start(timer: *mut hrtimer, tim: ktime_t, mode: hrtimer_mode) -> core::ffi::c_int;
    fn enable_fiq(fiq: core::ffi::c_int);
    fn disable_fiq(fiq: core::ffi::c_int);
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: core::ffi::c_ulong) -> snd_pcm_uframes_t;
    fn kzalloc(size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn kfree(objp: *const core::ffi::c_void);
    fn hrtimer_setup(
        timer: *mut hrtimer,
        function: Option<unsafe extern "C" fn(*mut hrtimer) -> hrtimer_restart>,
        clock_id: clockid_t,
        mode: hrtimer_mode,
    );
    fn snd_pcm_hw_constraint_integer(
        runtime: *mut snd_pcm_runtime,
        var: snd_pcm_hw_param,
    ) -> core::ffi::c_int;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    );
    fn hrtimer_cancel(timer: *mut hrtimer) -> core::ffi::c_int;
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> core::ffi::c_int;
    fn snd_pcm_set_fixed_buffer_all(
        pcm: *mut snd_pcm,
        ty: snd_dma_type,
        dev: *mut device,
        size: usize,
    ) -> core::ffi::c_int;
    fn set_fiq_handler(start: *const core::ffi::c_void, length: core::ffi::c_uint);
    fn mxc_set_irq_fiq(irq: core::ffi::c_uint, enable: core::ffi::c_uint);
    fn claim_fiq(fh: *mut fiq_handler) -> core::ffi::c_int;
    fn release_fiq(fh: *mut fiq_handler);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

unsafe extern "C" fn snd_hrtimer_callback(hrt: *mut hrtimer) -> hrtimer_restart {
    let iprtd = container_of!(hrt, imx_pcm_runtime_data, hrt);
    let substream = (*iprtd).substream;
    let mut regs: pt_regs = core::mem::zeroed();

    if atomic_read(&(*iprtd).playing) == 0 && atomic_read(&(*iprtd).capturing) == 0 {
        return HRTIMER_NORESTART;
    }

    get_fiq_regs(&mut regs);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*iprtd).offset = (regs.ARM_r8 & 0xffff) as core::ffi::c_ulong;
    } else {
        (*iprtd).offset = (regs.ARM_r9 & 0xffff) as core::ffi::c_ulong;
    }

    snd_pcm_period_elapsed(substream);

    hrtimer_forward_now(hrt, ns_to_ktime((*iprtd).poll_time_ns));

    HRTIMER_RESTART
}

static mut fh: fiq_handler = fiq_handler {
    name: DRV_NAME,
};

unsafe extern "C" fn snd_imx_pcm_hw_params(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> core::ffi::c_int {
    let runtime = (*substream).runtime;
    let iprtd = (*runtime).private_data as *mut imx_pcm_runtime_data;

    (*iprtd).periods = params_periods(params);
    (*iprtd).period = params_period_bytes(params);
    (*iprtd).offset = 0;
    (*iprtd).poll_time_ns =
        (1000000000u32 / params_rate(params) * params_period_size(params)) as core::ffi::c_int;

    0
}

unsafe extern "C" fn snd_imx_pcm_prepare(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let runtime = (*substream).runtime;
    let iprtd = (*runtime).private_data as *mut imx_pcm_runtime_data;
    let mut regs: pt_regs = core::mem::zeroed();

    get_fiq_regs(&mut regs);
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regs.ARM_r8 = (((*iprtd).period * (*iprtd).periods as core::ffi::c_uint - 1) << 16) as _;
    } else {
        regs.ARM_r9 = (((*iprtd).period * (*iprtd).periods as core::ffi::c_uint - 1) << 16) as _;
    }

    set_fiq_regs(&mut regs);

    0
}

static mut imx_pcm_fiq: core::ffi::c_int = 0;

unsafe extern "C" fn snd_imx_pcm_trigger(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: core::ffi::c_int,
) -> core::ffi::c_int {
    let runtime = (*substream).runtime;
    let iprtd = (*runtime).private_data as *mut imx_pcm_runtime_data;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                atomic_set(&mut (*iprtd).playing, 1);
            } else {
                atomic_set(&mut (*iprtd).capturing, 1);
            }
            hrtimer_start(
                &mut (*iprtd).hrt,
                ns_to_ktime((*iprtd).poll_time_ns),
                HRTIMER_MODE_REL,
            );
            enable_fiq(imx_pcm_fiq);
        }

        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
                atomic_set(&mut (*iprtd).playing, 0);
            } else {
                atomic_set(&mut (*iprtd).capturing, 0);
            }
            if atomic_read(&(*iprtd).playing) == 0 && atomic_read(&(*iprtd).capturing) == 0 {
                disable_fiq(imx_pcm_fiq);
            }
        }

        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn snd_imx_pcm_pointer(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let runtime = (*substream).runtime;
    let iprtd = (*runtime).private_data as *mut imx_pcm_runtime_data;

    bytes_to_frames((*substream).runtime, (*iprtd).offset)
}

static snd_imx_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_RESUME,
    formats: SNDRV_PCM_FMTBIT_S16_LE,
    buffer_bytes_max: IMX_SSI_DMABUF_SIZE,
    period_bytes_min: 128,
    period_bytes_max: 16 * 1024,
    periods_min: 4,
    periods_max: 255,
    fifo_size: 0,
};

unsafe extern "C" fn snd_imx_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let runtime = (*substream).runtime;
    let iprtd: *mut imx_pcm_runtime_data =
        kzalloc(core::mem::size_of::<imx_pcm_runtime_data>(), GFP_KERNEL) as *mut imx_pcm_runtime_data;
    let ret: core::ffi::c_int;

    if iprtd.is_null() {
        return -ENOMEM;
    }
    (*runtime).private_data = iprtd as *mut core::ffi::c_void;

    (*iprtd).substream = substream;

    atomic_set(&mut (*iprtd).playing, 0);
    atomic_set(&mut (*iprtd).capturing, 0);
    hrtimer_setup(
        &mut (*iprtd).hrt,
        Some(snd_hrtimer_callback),
        CLOCK_MONOTONIC,
        HRTIMER_MODE_REL,
    );

    ret = snd_pcm_hw_constraint_integer((*substream).runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        kfree(iprtd as *const core::ffi::c_void);
        return ret;
    }

    snd_soc_set_runtime_hwparams(substream, &snd_imx_hardware);
    0
}

unsafe extern "C" fn snd_imx_close(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> core::ffi::c_int {
    let runtime = (*substream).runtime;
    let iprtd = (*runtime).private_data as *mut imx_pcm_runtime_data;

    hrtimer_cancel(&mut (*iprtd).hrt);

    kfree(iprtd as *const core::ffi::c_void);

    0
}

unsafe extern "C" fn imx_pcm_new(rtd: *mut snd_soc_pcm_runtime) -> core::ffi::c_int {
    let card = (*(*rtd).card).snd_card;
    let pcm = (*rtd).pcm;
    let ret: core::ffi::c_int;

    ret = dma_coerce_mask_and_coherent((*card).dev, DMA_BIT_MASK(32));
    if ret != 0 {
        return ret;
    }

    snd_pcm_set_fixed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV_WC,
        (*(*pcm).card).dev,
        IMX_SSI_DMABUF_SIZE,
    )
}

static mut ssi_irq: core::ffi::c_int = 0;

unsafe extern "C" fn snd_imx_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> core::ffi::c_int {
    let pcm = (*rtd).pcm;
    let mut substream: *mut snd_pcm_substream;
    let ret: core::ffi::c_int;

    ret = imx_pcm_new(rtd);
    if ret != 0 {
        return ret;
    }

    substream = (*pcm).streams[SNDRV_PCM_STREAM_PLAYBACK as usize].substream;
    if !substream.is_null() {
        let buf = &mut (*substream).dma_buffer as *mut snd_dma_buffer;

        imx_ssi_fiq_tx_buffer = (*buf).area as core::ffi::c_ulong;
    }

    substream = (*pcm).streams[SNDRV_PCM_STREAM_CAPTURE as usize].substream;
    if !substream.is_null() {
        let buf = &mut (*substream).dma_buffer as *mut snd_dma_buffer;

        imx_ssi_fiq_rx_buffer = (*buf).area as core::ffi::c_ulong;
    }

    set_fiq_handler(
        &imx_ssi_fiq_start as *const _ as *const core::ffi::c_void,
        (&imx_ssi_fiq_end as *const _ as usize - &imx_ssi_fiq_start as *const _ as usize)
            as core::ffi::c_uint,
    );

    0
}

unsafe extern "C" fn snd_imx_pcm_free(
    _component: *mut snd_soc_component,
    _pcm: *mut snd_pcm,
) {
    mxc_set_irq_fiq(ssi_irq as core::ffi::c_uint, 0);
    release_fiq(&mut fh);
}

static imx_soc_component_fiq: snd_soc_component_driver = snd_soc_component_driver {
    open: Some(snd_imx_open),
    close: Some(snd_imx_close),
    hw_params: Some(snd_imx_pcm_hw_params),
    prepare: Some(snd_imx_pcm_prepare),
    trigger: Some(snd_imx_pcm_trigger),
    pointer: Some(snd_imx_pcm_pointer),
    pcm_new: Some(snd_imx_pcm_new),
    pcm_free: Some(snd_imx_pcm_free),
};

#[no_mangle]
pub unsafe extern "C" fn imx_pcm_fiq_init(
    pdev: *mut platform_device,
    params: *mut imx_pcm_fiq_params,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;

    ret = claim_fiq(&mut fh);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"failed to claim fiq: %d\0".as_ptr() as *const core::ffi::c_char,
            ret,
        );
        return ret;
    }

    mxc_set_irq_fiq((*params).irq as core::ffi::c_uint, 1);
    ssi_irq = (*params).irq;

    imx_pcm_fiq = (*params).irq;

    imx_ssi_fiq_base = (*params).base as core::ffi::c_ulong;

    (*(*params).dma_params_tx).maxburst = 4;
    (*(*params).dma_params_rx).maxburst = 6;

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &imx_soc_component_fiq,
        core::ptr::null_mut(),
        0,
    );
    if ret != 0 {
        mxc_set_irq_fiq(ssi_irq as core::ffi::c_uint, 0);
        release_fiq(&mut fh);

        return ret;
    }

    0
}

// EXPORT_SYMBOL_GPL(imx_pcm_fiq_init);

#[no_mangle]
pub unsafe extern "C" fn imx_pcm_fiq_exit(_pdev: *mut platform_device) {}

// EXPORT_SYMBOL_GPL(imx_pcm_fiq_exit);

// MODULE_DESCRIPTION("Freescale i.MX PCM FIQ handler");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
