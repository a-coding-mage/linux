// SPDX-License-Identifier: GPL-2.0-only
/*
 * Au12x0/Au1550 PSC ALSA ASoC audio support.
 *
 * (c) 2007-2008 MSC Vertriebsges.m.b.H.,
 *	Manuel Lauss <manuel.lauss@gmail.com>
 *
 * DMA glue for Au1x-PSC audio.
 */

// C dependencies:
// linux/module.h, linux/init.h, linux/platform_device.h, linux/slab.h,
// linux/dma-mapping.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
// sound/soc.h, asm/mach-au1x00/au1000.h,
// asm/mach-au1x00/au1xxx_dbdma.h, asm/mach-au1x00/au1xxx_psc.h, "psc.h"

/*#define PCM_DEBUG*/

const DRV_NAME: *const ::core::ffi::c_char = b"dbdma2\0".as_ptr() as *const ::core::ffi::c_char;

// #define MSG(x...) printk(KERN_INFO "au1xpsc_pcm: " x)
// #ifdef PCM_DEBUG
// #define DBG MSG
// #else
// #define DBG(x...) do {} while (0)
// #endif

type u32 = u32;
type dma_addr_t = usize;
type snd_pcm_uframes_t = usize;

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: ::core::ffi::c_int,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_area: *mut ::core::ffi::c_void,
    pub dma_addr: dma_addr_t,
    pub dma_bytes: usize,
    pub min_align: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    pub msbits: ::core::ffi::c_int,
}

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> ::core::ffi::c_int>,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const ::core::ffi::c_char,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: ::core::ffi::c_uint,
    pub period_bytes_min: ::core::ffi::c_uint,
    pub period_bytes_max: ::core::ffi::c_uint,
    pub periods_min: ::core::ffi::c_uint,
    pub periods_max: ::core::ffi::c_uint,
    pub buffer_bytes_max: ::core::ffi::c_uint,
    pub fifo_size: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const ::core::ffi::c_char,
    pub open: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> ::core::ffi::c_int,
    >,
    pub close: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> ::core::ffi::c_int,
    >,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> ::core::ffi::c_int,
    >,
    pub prepare: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> ::core::ffi::c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub pointer: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t,
    >,
    pub pcm_new: Option<
        unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> ::core::ffi::c_int,
    >,
}

#[repr(C)]
struct au1xpsc_audio_dmadata {
    /* DDMA control data */
    ddma_id: ::core::ffi::c_uint, /* DDMA direction ID for this PSC */
    ddma_chan: u32,               /* DDMA context */

    /* PCM context (for irq handlers) */
    substream: *mut snd_pcm_substream,
    curr_period: ::core::ffi::c_ulong, /* current segment DDMA is working on */
    q_period: ::core::ffi::c_ulong,    /* queue period(s) */
    dma_area: dma_addr_t,              /* address of queued DMA area */
    dma_area_s: dma_addr_t,            /* start address of DMA area */
    pos: ::core::ffi::c_ulong,         /* current byte position being played */
    periods: ::core::ffi::c_ulong,     /* number of SG segments in total */
    period_bytes: ::core::ffi::c_ulong, /* size in bytes of one SG segment */

    /* runtime data */
    msbits: ::core::ffi::c_int,
}

/*
 * These settings are somewhat okay, at least on my machine audio plays
 * almost skip-free. Especially the 64kB buffer seems to help a LOT.
 */
const AU1XPSC_PERIOD_MIN_BYTES: ::core::ffi::c_uint = 1024;
const AU1XPSC_BUFFER_MIN_BYTES: usize = 65536;

extern "C" {
    static SNDRV_PCM_INFO_MMAP: ::core::ffi::c_uint;
    static SNDRV_PCM_INFO_MMAP_VALID: ::core::ffi::c_uint;
    static SNDRV_PCM_INFO_INTERLEAVED: ::core::ffi::c_uint;
    static SNDRV_PCM_INFO_BATCH: ::core::ffi::c_uint;
    static SNDRV_PCM_STREAM_CAPTURE: ::core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_START: ::core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_RESUME: ::core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_STOP: ::core::ffi::c_int;
    static SNDRV_PCM_TRIGGER_SUSPEND: ::core::ffi::c_int;
    static SNDRV_DMA_TYPE_DEV: ::core::ffi::c_int;
    static DSCR_CMD0_ALWAYS: ::core::ffi::c_uint;
    static DDMA_FLAGS_IE: ::core::ffi::c_uint;
    static GFP_KERNEL: ::core::ffi::c_uint;
    static ENOMEM: ::core::ffi::c_int;
    static EINVAL: ::core::ffi::c_int;
    static ENODEV: ::core::ffi::c_int;

    fn printk(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn au1xxx_dbdma_put_source(
        chan: u32,
        buf: dma_addr_t,
        nbytes: ::core::ffi::c_ulong,
        flags: ::core::ffi::c_uint,
    );
    fn au1xxx_dbdma_put_dest(
        chan: u32,
        buf: dma_addr_t,
        nbytes: ::core::ffi::c_ulong,
        flags: ::core::ffi::c_uint,
    );
    fn au1xxx_dbdma_stop(chan: u32);
    fn au1xxx_dbdma_reset(chan: u32);
    fn au1xxx_dbdma_chan_free(chan: u32);
    fn au1xxx_dbdma_chan_alloc(
        srcid: ::core::ffi::c_uint,
        destid: ::core::ffi::c_uint,
        callback: Option<unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_void)>,
        dev_id: *mut ::core::ffi::c_void,
    ) -> u32;
    fn au1xxx_dbdma_set_devwidth(chan: u32, bits: ::core::ffi::c_int);
    fn au1xxx_dbdma_ring_alloc(chan: u32, entries: ::core::ffi::c_int);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut ::core::ffi::c_void;
    fn params_periods(params: *mut snd_pcm_hw_params) -> ::core::ffi::c_uint;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> ::core::ffi::c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: ::core::ffi::c_ulong) -> snd_pcm_uframes_t;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: ::core::ffi::c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(
        dai: *mut snd_soc_dai,
        substream: *mut snd_pcm_substream,
    ) -> *mut ::core::ffi::c_int;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    );
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: ::core::ffi::c_int,
        data: *mut device,
        size: usize,
        max: usize,
    );
    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut ::core::ffi::c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut ::core::ffi::c_void,
        num_dai: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

/* PCM hardware DMA capabilities - platform specific */
static mut au1xpsc_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: 0, /* initialized in au1xpsc_pcm_hardware_init from external bit constants */
    period_bytes_min: AU1XPSC_PERIOD_MIN_BYTES,
    period_bytes_max: 4096 * 1024 - 1,
    periods_min: 2,
    periods_max: 4096, /* 2 to as-much-as-you-like */
    buffer_bytes_max: 4096 * 1024 - 1,
    fifo_size: 16, /* fifo entries of AC97/I2S PSC */
};

unsafe fn au1xpsc_pcm_hardware_init() {
    au1xpsc_pcm_hardware.info = SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BATCH;
}

unsafe fn au1x_pcm_queue_tx(cd: *mut au1xpsc_audio_dmadata) {
    au1xxx_dbdma_put_source(
        (*cd).ddma_chan,
        (*cd).dma_area,
        (*cd).period_bytes,
        DDMA_FLAGS_IE,
    );

    /* update next-to-queue period */
    (*cd).q_period = (*cd).q_period.wrapping_add(1);
    (*cd).dma_area = (*cd).dma_area.wrapping_add((*cd).period_bytes as usize);
    if (*cd).q_period >= (*cd).periods {
        (*cd).q_period = 0;
        (*cd).dma_area = (*cd).dma_area_s;
    }
}

unsafe fn au1x_pcm_queue_rx(cd: *mut au1xpsc_audio_dmadata) {
    au1xxx_dbdma_put_dest(
        (*cd).ddma_chan,
        (*cd).dma_area,
        (*cd).period_bytes,
        DDMA_FLAGS_IE,
    );

    /* update next-to-queue period */
    (*cd).q_period = (*cd).q_period.wrapping_add(1);
    (*cd).dma_area = (*cd).dma_area.wrapping_add((*cd).period_bytes as usize);
    if (*cd).q_period >= (*cd).periods {
        (*cd).q_period = 0;
        (*cd).dma_area = (*cd).dma_area_s;
    }
}

unsafe extern "C" fn au1x_pcm_dmatx_cb(
    _irq: ::core::ffi::c_int,
    dev_id: *mut ::core::ffi::c_void,
) {
    let cd = dev_id as *mut au1xpsc_audio_dmadata;

    (*cd).pos = (*cd).pos.wrapping_add((*cd).period_bytes);
    (*cd).curr_period = (*cd).curr_period.wrapping_add(1);
    if (*cd).curr_period >= (*cd).periods {
        (*cd).pos = 0;
        (*cd).curr_period = 0;
    }
    snd_pcm_period_elapsed((*cd).substream);
    au1x_pcm_queue_tx(cd);
}

unsafe extern "C" fn au1x_pcm_dmarx_cb(
    _irq: ::core::ffi::c_int,
    dev_id: *mut ::core::ffi::c_void,
) {
    let cd = dev_id as *mut au1xpsc_audio_dmadata;

    (*cd).pos = (*cd).pos.wrapping_add((*cd).period_bytes);
    (*cd).curr_period = (*cd).curr_period.wrapping_add(1);
    if (*cd).curr_period >= (*cd).periods {
        (*cd).pos = 0;
        (*cd).curr_period = 0;
    }
    snd_pcm_period_elapsed((*cd).substream);
    au1x_pcm_queue_rx(cd);
}

unsafe fn au1x_pcm_dbdma_free(pcd: *mut au1xpsc_audio_dmadata) {
    if (*pcd).ddma_chan != 0 {
        au1xxx_dbdma_stop((*pcd).ddma_chan);
        au1xxx_dbdma_reset((*pcd).ddma_chan);
        au1xxx_dbdma_chan_free((*pcd).ddma_chan);
        (*pcd).ddma_chan = 0;
        (*pcd).msbits = 0;
    }
}

/* in case of missing DMA ring or changed TX-source / RX-dest bit widths,
 * allocate (or reallocate) a 2-descriptor DMA ring with bit depth according
 * to ALSA-supplied sample depth.  This is due to limitations in the dbdma api
 * (cannot adjust source/dest widths of already allocated descriptor ring).
 */
unsafe fn au1x_pcm_dbdma_realloc(
    pcd: *mut au1xpsc_audio_dmadata,
    stype: ::core::ffi::c_int,
    mut msbits: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    /* DMA only in 8/16/32 bit widths */
    if msbits == 24 {
        msbits = 32;
    }

    /* check current config: correct bits and descriptors allocated? */
    if ((*pcd).ddma_chan != 0) && (msbits == (*pcd).msbits) {
        return 0; /* all ok! */
    }

    au1x_pcm_dbdma_free(pcd);

    if stype == SNDRV_PCM_STREAM_CAPTURE {
        (*pcd).ddma_chan = au1xxx_dbdma_chan_alloc(
            (*pcd).ddma_id,
            DSCR_CMD0_ALWAYS,
            Some(au1x_pcm_dmarx_cb),
            pcd as *mut ::core::ffi::c_void,
        );
    } else {
        (*pcd).ddma_chan = au1xxx_dbdma_chan_alloc(
            DSCR_CMD0_ALWAYS,
            (*pcd).ddma_id,
            Some(au1x_pcm_dmatx_cb),
            pcd as *mut ::core::ffi::c_void,
        );
    }

    if (*pcd).ddma_chan == 0 {
        return -ENOMEM;
    }

    au1xxx_dbdma_set_devwidth((*pcd).ddma_chan, msbits);
    au1xxx_dbdma_ring_alloc((*pcd).ddma_chan, 2);

    (*pcd).msbits = msbits;

    au1xxx_dbdma_stop((*pcd).ddma_chan);
    au1xxx_dbdma_reset((*pcd).ddma_chan);

    0
}

unsafe fn to_dmadata(
    ss: *mut snd_pcm_substream,
    component: *mut snd_soc_component,
) -> *mut au1xpsc_audio_dmadata {
    let pcd = snd_soc_component_get_drvdata(component) as *mut au1xpsc_audio_dmadata;
    pcd.add((*ss).stream as usize)
}

unsafe extern "C" fn au1xpsc_pcm_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> ::core::ffi::c_int {
    let runtime = (*substream).runtime;
    let pcd: *mut au1xpsc_audio_dmadata;
    let stype: ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int;

    stype = (*substream).stream;
    pcd = to_dmadata(substream, component);

    // DBG("runtime->dma_area = 0x%08lx dma_addr_t = 0x%08lx dma_size = %zu "
    //     "runtime->min_align %lu\n",
    //     (unsigned long)runtime->dma_area,
    //     (unsigned long)runtime->dma_addr, runtime->dma_bytes,
    //     runtime->min_align);
    //
    // DBG("bits %d  frags %d  frag_bytes %d  is_rx %d\n", params->msbits,
    //     params_periods(params), params_period_bytes(params), stype);

    ret = au1x_pcm_dbdma_realloc(pcd, stype, (*params).msbits);
    if ret != 0 {
        printk(b"au1xpsc_pcm: DDMA channel (re)alloc failed!\n\0".as_ptr() as *const _);
        return ret;
    }

    (*pcd).substream = substream;
    (*pcd).period_bytes = params_period_bytes(params) as ::core::ffi::c_ulong;
    (*pcd).periods = params_periods(params) as ::core::ffi::c_ulong;
    (*pcd).dma_area = (*runtime).dma_addr;
    (*pcd).dma_area_s = (*pcd).dma_area;
    (*pcd).q_period = 0;
    (*pcd).curr_period = 0;
    (*pcd).pos = 0;

    ret = 0;
    ret
}

unsafe extern "C" fn au1xpsc_pcm_prepare(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> ::core::ffi::c_int {
    let pcd = to_dmadata(substream, component);

    au1xxx_dbdma_reset((*pcd).ddma_chan);

    if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
        au1x_pcm_queue_rx(pcd);
        au1x_pcm_queue_rx(pcd);
    } else {
        au1x_pcm_queue_tx(pcd);
        au1x_pcm_queue_tx(pcd);
    }

    0
}

unsafe extern "C" fn au1xpsc_pcm_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let c: u32 = (*to_dmadata(substream, component)).ddma_chan;

    if cmd == SNDRV_PCM_TRIGGER_START || cmd == SNDRV_PCM_TRIGGER_RESUME {
        au1xxx_dbdma_start(c);
    } else if cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_SUSPEND {
        au1xxx_dbdma_stop(c);
    } else {
        return -EINVAL;
    }
    0
}

extern "C" {
    fn au1xxx_dbdma_start(chan: u32);
}

unsafe extern "C" fn au1xpsc_pcm_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    bytes_to_frames(
        (*substream).runtime,
        (*to_dmadata(substream, component)).pos,
    )
}

unsafe extern "C" fn au1xpsc_pcm_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> ::core::ffi::c_int {
    let pcd = to_dmadata(substream, component);
    let rtd = snd_soc_substream_to_rtd(substream);
    let stype = (*substream).stream;
    let dmaids: *mut ::core::ffi::c_int;

    dmaids = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
    if dmaids.is_null() {
        return -ENODEV; /* whoa, has ordering changed? */
    }

    (*pcd).ddma_id = *dmaids.add(stype as usize) as ::core::ffi::c_uint;

    au1xpsc_pcm_hardware_init();
    snd_soc_set_runtime_hwparams(substream, &au1xpsc_pcm_hardware as *const snd_pcm_hardware);
    0
}

unsafe extern "C" fn au1xpsc_pcm_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> ::core::ffi::c_int {
    au1x_pcm_dbdma_free(to_dmadata(substream, component));
    0
}

unsafe extern "C" fn au1xpsc_pcm_new(
    _component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> ::core::ffi::c_int {
    let card = (*(*rtd).card).snd_card;
    let pcm = (*rtd).pcm;

    snd_pcm_set_managed_buffer_all(
        pcm,
        SNDRV_DMA_TYPE_DEV,
        (*card).dev,
        AU1XPSC_BUFFER_MIN_BYTES,
        (4096 * 1024) - 1,
    );

    0
}

/* au1xpsc audio platform */
static au1xpsc_soc_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME,
    open: Some(au1xpsc_pcm_open),
    close: Some(au1xpsc_pcm_close),
    hw_params: Some(au1xpsc_pcm_hw_params),
    prepare: Some(au1xpsc_pcm_prepare),
    trigger: Some(au1xpsc_pcm_trigger),
    pointer: Some(au1xpsc_pcm_pointer),
    pcm_new: Some(au1xpsc_pcm_new),
};

unsafe extern "C" fn au1xpsc_pcm_drvprobe(pdev: *mut platform_device) -> ::core::ffi::c_int {
    let dmadata: *mut au1xpsc_audio_dmadata;

    dmadata = devm_kcalloc(
        &mut (*pdev).dev,
        2,
        ::core::mem::size_of::<au1xpsc_audio_dmadata>(),
        GFP_KERNEL,
    ) as *mut au1xpsc_audio_dmadata;
    if dmadata.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, dmadata as *mut ::core::ffi::c_void);

    devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &au1xpsc_soc_component,
        ::core::ptr::null_mut(),
        0,
    )
}

static mut au1xpsc_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"au1xpsc-pcm\0".as_ptr() as *const ::core::ffi::c_char,
    },
    probe: Some(au1xpsc_pcm_drvprobe),
};

// module_platform_driver(au1xpsc_pcm_driver);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Au12x0/Au1550 PSC Audio DMA driver");
// MODULE_AUTHOR("Manuel Lauss");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
