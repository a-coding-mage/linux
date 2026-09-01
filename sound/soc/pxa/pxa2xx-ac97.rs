// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/sound/pxa2xx-ac97.c -- AC97 support for the Intel PXA2xx chip.
 *
 * Author:	Nicolas Pitre
 * Created:	Dec 02, 2004
 * Copyright:	MontaVista Software Inc.
 */

// C dependencies:
// linux/init.h, linux/io.h, linux/module.h, linux/platform_device.h,
// linux/dmaengine.h, linux/dma/pxa-dma.h, sound/ac97/controller.h,
// sound/core.h, sound/ac97_codec.h, sound/soc.h, sound/dmaengine_pcm.h,
// linux/platform_data/asoc-pxa.h, and "pxa2xx-lib.h".

const PCDR: u64 = 0x0040; /* PCM FIFO Data Register */
const MODR: u64 = 0x0140; /* Modem FIFO Data Register */
const MCDR: u64 = 0x0060; /* Mic-in FIFO Data Register */

const PXA2XX_AC97_RATES: u32 = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_11025
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000;

extern "C" {
    fn pxa2xx_ac97_try_warm_reset();
    fn pxa2xx_ac97_finish_reset();
    fn pxa2xx_ac97_try_cold_reset();
    fn pxa2xx_ac97_read(slot: core::ffi::c_int, reg: core::ffi::c_ushort) -> core::ffi::c_int;
    fn pxa2xx_ac97_write(
        slot: core::ffi::c_int,
        reg: core::ffi::c_ushort,
        val: core::ffi::c_ushort,
    ) -> core::ffi::c_int;
    fn snd_soc_dai_set_dma_data(
        cpu_dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
        data: *mut snd_dmaengine_dai_dma_data,
    );
    fn pxa2xx_soc_pcm_new(component: *mut snd_soc_component, rtd: *mut snd_soc_pcm_runtime)
        -> core::ffi::c_int;
    fn pxa2xx_soc_pcm_open(substream: *mut snd_pcm_substream) -> core::ffi::c_int;
    fn pxa2xx_soc_pcm_close(substream: *mut snd_pcm_substream) -> core::ffi::c_int;
    fn pxa2xx_soc_pcm_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
    ) -> core::ffi::c_int;
    fn pxa2xx_soc_pcm_prepare(substream: *mut snd_pcm_substream) -> core::ffi::c_int;
    fn pxa2xx_soc_pcm_trigger(
        substream: *mut snd_pcm_substream,
        cmd: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn pxa2xx_soc_pcm_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn platform_get_resource(
        pdev: *mut platform_device,
        ty: core::ffi::c_uint,
        num: core::ffi::c_uint,
    ) -> *mut resource;
    fn pxa2xx_ac97_hw_probe(pdev: *mut platform_device) -> core::ffi::c_int;
    fn snd_ac97_controller_register(
        ops: *mut ac97_controller_ops,
        dev: *mut device,
        slots_available: core::ffi::c_ulong,
    ) -> *mut ac97_controller;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> core::ffi::c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut core::ffi::c_void;
    fn snd_ac97_controller_unregister(ctrl: *mut ac97_controller);
    fn pxa2xx_ac97_hw_remove(pdev: *mut platform_device);
    fn pxa2xx_ac97_hw_suspend() -> core::ffi::c_int;
    fn pxa2xx_ac97_hw_resume() -> core::ffi::c_int;
}

#[repr(C)]
pub struct ac97_controller {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ac97_controller_ops {
    pub read: Option<
        unsafe extern "C" fn(
            adrv: *mut ac97_controller,
            slot: core::ffi::c_int,
            reg: core::ffi::c_ushort,
        ) -> core::ffi::c_int,
    >,
    pub write: Option<
        unsafe extern "C" fn(
            adrv: *mut ac97_controller,
            slot: core::ffi::c_int,
            reg: core::ffi::c_ushort,
            val: core::ffi::c_ushort,
        ) -> core::ffi::c_int,
    >,
    pub warm_reset: Option<unsafe extern "C" fn(adrv: *mut ac97_controller)>,
    pub reset: Option<unsafe extern "C" fn(adrv: *mut ac97_controller)>,
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: u64,
    pub addr_width: core::ffi::c_uint,
    pub chan_name: *const core::ffi::c_char,
    pub maxburst: core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: core::ffi::c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            cpu_dai: *mut snd_soc_dai,
        ) -> core::ffi::c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const core::ffi::c_char,
    pub channels_min: core::ffi::c_uint,
    pub channels_max: core::ffi::c_uint,
    pub rates: u32,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const core::ffi::c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const core::ffi::c_char,
    pub pcm_new: Option<
        unsafe extern "C" fn(
            component: *mut snd_soc_component,
            rtd: *mut snd_soc_pcm_runtime,
        ) -> core::ffi::c_int,
    >,
    pub open: Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> core::ffi::c_int>,
    pub close: Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> core::ffi::c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
        ) -> core::ffi::c_int,
    >,
    pub prepare: Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> core::ffi::c_int>,
    pub trigger: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            cmd: core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    pub pointer: Option<unsafe extern "C" fn(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub id: core::ffi::c_int,
}

#[repr(C)]
pub struct resource {
    pub start: u64,
}

#[repr(C)]
pub struct dev_pm_ops {
    pub suspend: Option<unsafe extern "C" fn(dev: *mut device) -> core::ffi::c_int>,
    pub resume: Option<unsafe extern "C" fn(dev: *mut device) -> core::ffi::c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> core::ffi::c_int>,
    pub remove: Option<unsafe extern "C" fn(pdev: *mut platform_device)>,
    pub driver: device_driver,
}

pub type snd_pcm_uframes_t = core::ffi::c_ulong;

unsafe extern "C" {
    static DMA_SLAVE_BUSWIDTH_4_BYTES: core::ffi::c_uint;
    static DMA_SLAVE_BUSWIDTH_2_BYTES: core::ffi::c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: core::ffi::c_int;
    static SNDRV_PCM_RATE_8000: u32;
    static SNDRV_PCM_RATE_11025: u32;
    static SNDRV_PCM_RATE_16000: u32;
    static SNDRV_PCM_RATE_22050: u32;
    static SNDRV_PCM_RATE_44100: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static ENODEV: core::ffi::c_int;
    static ENXIO: core::ffi::c_int;
    static IORESOURCE_MEM: core::ffi::c_uint;
    static AC97_SLOTS_AVAILABLE_ALL: core::ffi::c_ulong;
}

unsafe extern "C" fn pxa2xx_ac97_warm_reset(_adrv: *mut ac97_controller) {
    unsafe {
        pxa2xx_ac97_try_warm_reset();
        pxa2xx_ac97_finish_reset();
    }
}

unsafe extern "C" fn pxa2xx_ac97_cold_reset(_adrv: *mut ac97_controller) {
    unsafe {
        pxa2xx_ac97_try_cold_reset();
        pxa2xx_ac97_finish_reset();
    }
}

unsafe extern "C" fn pxa2xx_ac97_read_actrl(
    _adrv: *mut ac97_controller,
    slot: core::ffi::c_int,
    reg: core::ffi::c_ushort,
) -> core::ffi::c_int {
    unsafe { pxa2xx_ac97_read(slot, reg) }
}

unsafe extern "C" fn pxa2xx_ac97_write_actrl(
    _adrv: *mut ac97_controller,
    slot: core::ffi::c_int,
    reg: core::ffi::c_ushort,
    val: core::ffi::c_ushort,
) -> core::ffi::c_int {
    unsafe { pxa2xx_ac97_write(slot, reg, val) }
}

static mut pxa2xx_ac97_ops: ac97_controller_ops = ac97_controller_ops {
    read: Some(pxa2xx_ac97_read_actrl),
    write: Some(pxa2xx_ac97_write_actrl),
    warm_reset: Some(pxa2xx_ac97_warm_reset),
    reset: Some(pxa2xx_ac97_cold_reset),
};

static mut pxa2xx_ac97_pcm_stereo_in: snd_dmaengine_dai_dma_data = snd_dmaengine_dai_dma_data {
    addr: 0,
    addr_width: unsafe { DMA_SLAVE_BUSWIDTH_4_BYTES },
    chan_name: c"pcm_pcm_stereo_in".as_ptr(),
    maxburst: 32,
};

static mut pxa2xx_ac97_pcm_stereo_out: snd_dmaengine_dai_dma_data = snd_dmaengine_dai_dma_data {
    addr: 0,
    addr_width: unsafe { DMA_SLAVE_BUSWIDTH_4_BYTES },
    chan_name: c"pcm_pcm_stereo_out".as_ptr(),
    maxburst: 32,
};

static mut pxa2xx_ac97_pcm_aux_mono_out: snd_dmaengine_dai_dma_data =
    snd_dmaengine_dai_dma_data {
        addr: 0,
        addr_width: unsafe { DMA_SLAVE_BUSWIDTH_2_BYTES },
        chan_name: c"pcm_aux_mono_out".as_ptr(),
        maxburst: 16,
    };

static mut pxa2xx_ac97_pcm_aux_mono_in: snd_dmaengine_dai_dma_data = snd_dmaengine_dai_dma_data {
    addr: 0,
    addr_width: unsafe { DMA_SLAVE_BUSWIDTH_2_BYTES },
    chan_name: c"pcm_aux_mono_in".as_ptr(),
    maxburst: 16,
};

static mut pxa2xx_ac97_pcm_mic_mono_in: snd_dmaengine_dai_dma_data = snd_dmaengine_dai_dma_data {
    addr: 0,
    addr_width: unsafe { DMA_SLAVE_BUSWIDTH_2_BYTES },
    chan_name: c"pcm_aux_mic_mono".as_ptr(),
    maxburst: 16,
};

unsafe extern "C" fn pxa2xx_ac97_hifi_startup(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let dma_data: *mut snd_dmaengine_dai_dma_data;

    unsafe {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            dma_data = &mut pxa2xx_ac97_pcm_stereo_out;
        } else {
            dma_data = &mut pxa2xx_ac97_pcm_stereo_in;
        }

        snd_soc_dai_set_dma_data(cpu_dai, substream, dma_data);
    }

    0
}

unsafe extern "C" fn pxa2xx_ac97_aux_startup(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let dma_data: *mut snd_dmaengine_dai_dma_data;

    unsafe {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            dma_data = &mut pxa2xx_ac97_pcm_aux_mono_out;
        } else {
            dma_data = &mut pxa2xx_ac97_pcm_aux_mono_in;
        }

        snd_soc_dai_set_dma_data(cpu_dai, substream, dma_data);
    }

    0
}

unsafe extern "C" fn pxa2xx_ac97_mic_startup(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    unsafe {
        if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
            return -ENODEV;
        }
        snd_soc_dai_set_dma_data(
            cpu_dai,
            substream,
            &mut pxa2xx_ac97_pcm_mic_mono_in,
        );
    }

    0
}

static pxa_ac97_hifi_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(pxa2xx_ac97_hifi_startup),
};

static pxa_ac97_aux_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(pxa2xx_ac97_aux_startup),
};

static pxa_ac97_mic_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(pxa2xx_ac97_mic_startup),
};

/*
 * There is only 1 physical AC97 interface for pxa2xx, but it
 * has extra fifo's that can be used for aux DACs and ADCs.
 */
static mut pxa_ac97_dai_driver: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        name: c"pxa2xx-ac97".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"AC97 Playback".as_ptr(),
            channels_min: 2,
            channels_max: 2,
            rates: PXA2XX_AC97_RATES,
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"AC97 Capture".as_ptr(),
            channels_min: 2,
            channels_max: 2,
            rates: PXA2XX_AC97_RATES,
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
        },
        ops: &pxa_ac97_hifi_dai_ops,
    },
    snd_soc_dai_driver {
        name: c"pxa2xx-ac97-aux".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: c"AC97 Aux Playback".as_ptr(),
            channels_min: 1,
            channels_max: 1,
            rates: PXA2XX_AC97_RATES,
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"AC97 Aux Capture".as_ptr(),
            channels_min: 1,
            channels_max: 1,
            rates: PXA2XX_AC97_RATES,
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
        },
        ops: &pxa_ac97_aux_dai_ops,
    },
    snd_soc_dai_driver {
        name: c"pxa2xx-ac97-mic".as_ptr(),
        playback: snd_soc_pcm_stream {
            stream_name: core::ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        capture: snd_soc_pcm_stream {
            stream_name: c"AC97 Mic Capture".as_ptr(),
            channels_min: 1,
            channels_max: 1,
            rates: PXA2XX_AC97_RATES,
            formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE },
        },
        ops: &pxa_ac97_mic_dai_ops,
    },
];

static pxa_ac97_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"pxa-ac97".as_ptr(),
    pcm_new: Some(pxa2xx_soc_pcm_new),
    open: Some(pxa2xx_soc_pcm_open),
    close: Some(pxa2xx_soc_pcm_close),
    hw_params: Some(pxa2xx_soc_pcm_hw_params),
    prepare: Some(pxa2xx_soc_pcm_prepare),
    trigger: Some(pxa2xx_soc_pcm_trigger),
    pointer: Some(pxa2xx_soc_pcm_pointer),
};

// CONFIG_OF:
static pxa2xx_ac97_dt_ids: [of_device_id; 4] = [
    of_device_id {
        compatible: c"marvell,pxa250-ac97".as_ptr(),
    },
    of_device_id {
        compatible: c"marvell,pxa270-ac97".as_ptr(),
    },
    of_device_id {
        compatible: c"marvell,pxa300-ac97".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, pxa2xx_ac97_dt_ids);

unsafe extern "C" fn pxa2xx_ac97_dev_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let ret: core::ffi::c_int;
    let ctrl: *mut ac97_controller;
    let regs: *mut resource;

    unsafe {
        if (*pdev).id != -1 {
            dev_err(
                &mut (*pdev).dev,
                c"PXA2xx has only one AC97 port.\n".as_ptr(),
            );
            return -ENXIO;
        }

        regs = platform_get_resource(pdev, IORESOURCE_MEM, 0);
        if regs.is_null() {
            return -ENXIO;
        }

        pxa2xx_ac97_pcm_stereo_in.addr = (*regs).start.wrapping_add(PCDR);
        pxa2xx_ac97_pcm_stereo_out.addr = (*regs).start.wrapping_add(PCDR);
        pxa2xx_ac97_pcm_aux_mono_out.addr = (*regs).start.wrapping_add(MODR);
        pxa2xx_ac97_pcm_aux_mono_in.addr = (*regs).start.wrapping_add(MODR);
        pxa2xx_ac97_pcm_mic_mono_in.addr = (*regs).start.wrapping_add(MCDR);

        ret = pxa2xx_ac97_hw_probe(pdev);
        if ret != 0 {
            dev_err(
                &mut (*pdev).dev,
                c"PXA2xx AC97 hw probe error (%d)\n".as_ptr(),
                ret,
            );
            return ret;
        }

        ctrl = snd_ac97_controller_register(
            &mut pxa2xx_ac97_ops,
            &mut (*pdev).dev,
            AC97_SLOTS_AVAILABLE_ALL,
        );
        if IS_ERR(ctrl as *const core::ffi::c_void) {
            return PTR_ERR(ctrl as *const core::ffi::c_void);
        }

        platform_set_drvdata(pdev, ctrl as *mut core::ffi::c_void);
        /* Punt most of the init to the SoC probe; we may need the machine
         * driver to do interesting things with the clocking to get us up
         * and running.
         */
        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &pxa_ac97_component,
            pxa_ac97_dai_driver.as_mut_ptr(),
            pxa_ac97_dai_driver.len() as core::ffi::c_int,
        )
    }
}

unsafe extern "C" fn pxa2xx_ac97_dev_remove(pdev: *mut platform_device) {
    let ctrl: *mut ac97_controller =
        unsafe { platform_get_drvdata(pdev) as *mut ac97_controller };

    unsafe {
        snd_ac97_controller_unregister(ctrl);
        pxa2xx_ac97_hw_remove(pdev);
    }
}

unsafe extern "C" fn pxa2xx_ac97_dev_suspend(_dev: *mut device) -> core::ffi::c_int {
    unsafe { pxa2xx_ac97_hw_suspend() }
}

unsafe extern "C" fn pxa2xx_ac97_dev_resume(_dev: *mut device) -> core::ffi::c_int {
    unsafe { pxa2xx_ac97_hw_resume() }
}

static pxa2xx_ac97_pm_ops: dev_pm_ops = dev_pm_ops {
    suspend: Some(pxa2xx_ac97_dev_suspend),
    resume: Some(pxa2xx_ac97_dev_resume),
};

static mut pxa2xx_ac97_driver: platform_driver = platform_driver {
    probe: Some(pxa2xx_ac97_dev_probe),
    remove: Some(pxa2xx_ac97_dev_remove),
    driver: device_driver {
        name: c"pxa2xx-ac97".as_ptr(),
        pm: &pxa2xx_ac97_pm_ops,
        of_match_table: pxa2xx_ac97_dt_ids.as_ptr(),
    },
};

// module_platform_driver(pxa2xx_ac97_driver);
// MODULE_AUTHOR("Nicolas Pitre");
// MODULE_DESCRIPTION("AC97 driver for the Intel PXA2xx chip");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:pxa2xx-ac97");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
