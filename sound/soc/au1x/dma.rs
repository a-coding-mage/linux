// SPDX-License-Identifier: GPL-2.0-only
/*
 * Au1000/Au1500/Au1100 Audio DMA support.
 *
 * (c) 2011 Manuel Lauss <manuel.lauss@googlemail.com>
 *
 * copied almost verbatim from the old ALSA driver, written by
 *			Charles Eidsness <charles@cooper-street.com>
 */

// Rust translation of dependencies originally provided by:
// linux/module.h, linux/init.h, linux/platform_device.h, linux/slab.h,
// linux/dma-mapping.h, sound/core.h, sound/pcm.h, sound/pcm_params.h,
// sound/soc.h, asm/mach-au1x00/au1000.h, asm/mach-au1x00/au1000_dma.h,
// and "psc.h".

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const DRV_NAME: &[u8] = b"au1x_dma\0";

type u32 = u32;
type irqreturn_t = c_int;
type snd_pcm_uframes_t = c_ulong;

const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const GFP_KERNEL: c_uint = 0;
const IRQ_HANDLED: irqreturn_t = 1;

const DMA_D0: c_int = 1;
const DMA_D1: c_int = 2;
const DMA_NC: c_int = 0;

const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 2;
const SNDRV_PCM_INFO_BATCH: c_uint = 1 << 3;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_DMA_TYPE_CONTINUOUS: c_int = 0;

type c_uint = u32;

#[repr(C)]
struct snd_pcm_runtime {
    dma_area: *mut c_void,
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    stream: c_int,
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
struct snd_soc_pcm_runtime {
    pcm: *mut snd_pcm,
}

#[repr(C)]
struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
}

#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct snd_pcm_hardware {
    info: c_uint,
    period_bytes_min: c_uint,
    period_bytes_max: c_uint,
    periods_min: c_uint,
    periods_max: c_uint,
    buffer_bytes_max: c_uint,
    fifo_size: c_uint,
}

#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
    open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    close: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
        ) -> c_int,
    >,
    hw_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_pcm_runtime) -> c_int>,
}

#[repr(C)]
struct pcm_period {
    start: u32,
    relative_end: u32, /* relative to start of buffer */
    next: *mut pcm_period,
}

#[repr(C)]
struct audio_stream {
    substream: *mut snd_pcm_substream,
    dma: c_int,
    buffer: *mut pcm_period,
    period_size: c_uint,
    periods: c_uint,
}

#[repr(C)]
struct alchemy_pcm_ctx {
    stream: [audio_stream; 2], /* playback & capture */
}

unsafe extern "C" {
    fn kfree(ptr: *mut c_void);
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn virt_to_phys(address: *mut c_void) -> c_ulong;

    fn disable_dma(dma: c_int);
    fn init_dma(dma: c_int);
    fn get_dma_active_buffer(dma: c_int) -> c_int;
    fn clear_dma_done0(dma: c_int);
    fn clear_dma_done1(dma: c_int);
    fn set_dma_addr0(dma: c_int, addr: u32);
    fn set_dma_addr1(dma: c_int, addr: u32);
    fn set_dma_count0(dma: c_int, count: c_uint);
    fn set_dma_count1(dma: c_int, count: c_uint);
    fn enable_dma_buffers(dma: c_int);
    fn start_dma(dma: c_int);
    fn get_dma_buffer_done(dma: c_int) -> c_int;
    fn enable_dma_buffer0(dma: c_int);
    fn enable_dma_buffer1(dma: c_int);
    fn request_au1000_dma(
        dma: c_int,
        name: *const c_char,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_ulong,
        dev_id: *mut c_void,
    ) -> c_int;
    fn free_au1000_dma(dma: c_int);
    fn set_dma_mode(dma: c_int, mode: c_int);
    fn get_dma_mode(dma: c_int) -> c_int;
    fn get_dma_residue(dma: c_int) -> c_long;

    fn pr_debug(fmt: *const c_char, ...);
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_dma_data(dai: *mut snd_soc_dai, substream: *mut snd_pcm_substream) -> *mut c_int;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_periods(params: *mut snd_pcm_hw_params) -> c_uint;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_long) -> snd_pcm_uframes_t;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        ty: c_int,
        data: *mut c_void,
        size: usize,
        max: usize,
    );
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut c_void,
        num_dai: c_int,
    ) -> c_int;
}

type c_long = isize;

unsafe fn kmalloc_obj<T>() -> *mut T {
    unsafe { kmalloc(size_of::<T>(), GFP_KERNEL) as *mut T }
}

unsafe fn au1000_release_dma_link(stream: *mut audio_stream) {
    let mut pointer: *mut pcm_period;
    let mut pointer_next: *mut pcm_period;

    unsafe {
        (*stream).period_size = 0;
        (*stream).periods = 0;
        pointer = (*stream).buffer;
        if pointer.is_null() {
            return;
        }
        loop {
            pointer_next = (*pointer).next;
            kfree(pointer as *mut c_void);
            pointer = pointer_next;
            if pointer == (*stream).buffer {
                break;
            }
        }
        (*stream).buffer = ptr::null_mut();
    }
}

unsafe fn au1000_setup_dma_link(
    stream: *mut audio_stream,
    period_bytes: c_uint,
    periods: c_uint,
) -> c_int {
    let substream: *mut snd_pcm_substream;
    let runtime: *mut snd_pcm_runtime;
    let mut pointer: *mut pcm_period;
    let dma_start: c_ulong;
    let mut i: c_int;

    unsafe {
        substream = (*stream).substream;
        runtime = (*substream).runtime;
        dma_start = virt_to_phys((*runtime).dma_area);

        if (*stream).period_size == period_bytes && (*stream).periods == periods {
            return 0; /* not changed */
        }

        au1000_release_dma_link(stream);

        (*stream).period_size = period_bytes;
        (*stream).periods = periods;

        (*stream).buffer = kmalloc_obj::<pcm_period>();
        if (*stream).buffer.is_null() {
            return -ENOMEM;
        }
        pointer = (*stream).buffer;
        i = 0;
        while i < periods as c_int {
            (*pointer).start = dma_start.wrapping_add((i as c_ulong).wrapping_mul(period_bytes as c_ulong)) as u32;
            (*pointer).relative_end = (((i + 1) as c_uint).wrapping_mul(period_bytes)).wrapping_sub(0x1) as u32;
            if i < periods as c_int - 1 {
                (*pointer).next = kmalloc_obj::<pcm_period>();
                if (*pointer).next.is_null() {
                    au1000_release_dma_link(stream);
                    return -ENOMEM;
                }
                pointer = (*pointer).next;
            }
            i += 1;
        }
        (*pointer).next = (*stream).buffer;
        0
    }
}

unsafe fn au1000_dma_stop(stream: *mut audio_stream) {
    unsafe {
        if !(*stream).buffer.is_null() {
            disable_dma((*stream).dma);
        }
    }
}

unsafe fn au1000_dma_start(stream: *mut audio_stream) {
    unsafe {
        if (*stream).buffer.is_null() {
            return;
        }

        init_dma((*stream).dma);
        if get_dma_active_buffer((*stream).dma) == 0 {
            clear_dma_done0((*stream).dma);
            set_dma_addr0((*stream).dma, (*(*stream).buffer).start);
            set_dma_count0((*stream).dma, (*stream).period_size >> 1);
            set_dma_addr1((*stream).dma, (*(*(*stream).buffer).next).start);
            set_dma_count1((*stream).dma, (*stream).period_size >> 1);
        } else {
            clear_dma_done1((*stream).dma);
            set_dma_addr1((*stream).dma, (*(*stream).buffer).start);
            set_dma_count1((*stream).dma, (*stream).period_size >> 1);
            set_dma_addr0((*stream).dma, (*(*(*stream).buffer).next).start);
            set_dma_count0((*stream).dma, (*stream).period_size >> 1);
        }
        enable_dma_buffers((*stream).dma);
        start_dma((*stream).dma);
    }
}

unsafe extern "C" fn au1000_dma_interrupt(irq: c_int, ptr: *mut c_void) -> irqreturn_t {
    let stream = ptr as *mut audio_stream;
    let substream: *mut snd_pcm_substream;

    unsafe {
        substream = (*stream).substream;

        match get_dma_buffer_done((*stream).dma) {
            DMA_D0 => {
                (*stream).buffer = (*(*stream).buffer).next;
                clear_dma_done0((*stream).dma);
                set_dma_addr0((*stream).dma, (*(*(*stream).buffer).next).start);
                set_dma_count0((*stream).dma, (*stream).period_size >> 1);
                enable_dma_buffer0((*stream).dma);
            }
            DMA_D1 => {
                (*stream).buffer = (*(*stream).buffer).next;
                clear_dma_done1((*stream).dma);
                set_dma_addr1((*stream).dma, (*(*(*stream).buffer).next).start);
                set_dma_count1((*stream).dma, (*stream).period_size >> 1);
                enable_dma_buffer1((*stream).dma);
            }
            x if x == (DMA_D0 | DMA_D1) => {
                pr_debug(c"DMA %d missed interrupt.\n".as_ptr(), (*stream).dma);
                au1000_dma_stop(stream);
                au1000_dma_start(stream);
            }
            x if x == (!DMA_D0 & !DMA_D1) => {
                pr_debug(c"DMA %d empty irq.\n".as_ptr(), (*stream).dma);
            }
            _ => {}
        }
        snd_pcm_period_elapsed(substream);
        IRQ_HANDLED
    }
}

static alchemy_pcm_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BATCH,
    period_bytes_min: 1024,
    period_bytes_max: 16 * 1024 - 1,
    periods_min: 4,
    periods_max: 255,
    buffer_bytes_max: 128 * 1024,
    fifo_size: 16,
};

unsafe fn ss_to_ctx(
    ss: *mut snd_pcm_substream,
    component: *mut snd_soc_component,
) -> *mut alchemy_pcm_ctx {
    unsafe { snd_soc_component_get_drvdata(component) as *mut alchemy_pcm_ctx }
}

unsafe fn ss_to_as(
    ss: *mut snd_pcm_substream,
    component: *mut snd_soc_component,
) -> *mut audio_stream {
    let ctx: *mut alchemy_pcm_ctx = unsafe { ss_to_ctx(ss, component) };
    unsafe { &mut (*ctx).stream[(*ss).stream as usize] as *mut audio_stream }
}

unsafe extern "C" fn alchemy_pcm_open(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let ctx: *mut alchemy_pcm_ctx;
    let rtd: *mut snd_soc_pcm_runtime;
    let mut dmaids: *mut c_int;
    let s: c_int;
    let name: *const c_char;

    unsafe {
        ctx = ss_to_ctx(substream, component);
        rtd = snd_soc_substream_to_rtd(substream);
        s = (*substream).stream;

        dmaids = snd_soc_dai_get_dma_data(snd_soc_rtd_to_cpu(rtd, 0), substream);
        if dmaids.is_null() {
            return -ENODEV; /* whoa, has ordering changed? */
        }

        /* DMA setup */
        name = if s == SNDRV_PCM_STREAM_PLAYBACK {
            c"audio-tx".as_ptr()
        } else {
            c"audio-rx".as_ptr()
        };
        (*ctx).stream[s as usize].dma = request_au1000_dma(
            *dmaids.add(s as usize),
            name,
            Some(au1000_dma_interrupt),
            0,
            &mut (*ctx).stream[s as usize] as *mut audio_stream as *mut c_void,
        );
        set_dma_mode(
            (*ctx).stream[s as usize].dma,
            get_dma_mode((*ctx).stream[s as usize].dma) & !DMA_NC,
        );

        (*ctx).stream[s as usize].substream = substream;
        (*ctx).stream[s as usize].buffer = ptr::null_mut();
        snd_soc_set_runtime_hwparams(substream, &alchemy_pcm_hardware);

        0
    }
}

unsafe extern "C" fn alchemy_pcm_close(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let ctx: *mut alchemy_pcm_ctx;
    let stype: c_int;

    unsafe {
        ctx = ss_to_ctx(substream, component);
        stype = (*substream).stream;

        (*ctx).stream[stype as usize].substream = ptr::null_mut();
        free_au1000_dma((*ctx).stream[stype as usize].dma);

        0
    }
}

unsafe extern "C" fn alchemy_pcm_hw_params(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    hw_params: *mut snd_pcm_hw_params,
) -> c_int {
    let stream: *mut audio_stream = unsafe { ss_to_as(substream, component) };

    unsafe {
        au1000_setup_dma_link(
            stream,
            params_period_bytes(hw_params),
            params_periods(hw_params),
        )
    }
}

unsafe extern "C" fn alchemy_pcm_hw_free(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    let stream: *mut audio_stream = unsafe { ss_to_as(substream, component) };
    unsafe {
        au1000_release_dma_link(stream);
    }
    0
}

unsafe extern "C" fn alchemy_pcm_trigger(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let stream: *mut audio_stream = unsafe { ss_to_as(substream, component) };
    let mut err: c_int = 0;

    unsafe {
        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                au1000_dma_start(stream);
            }
            SNDRV_PCM_TRIGGER_STOP => {
                au1000_dma_stop(stream);
            }
            _ => {
                err = -EINVAL;
            }
        }
    }
    err
}

unsafe extern "C" fn alchemy_pcm_pointer(
    component: *mut snd_soc_component,
    ss: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let stream: *mut audio_stream = unsafe { ss_to_as(ss, component) };
    let mut location: c_long;

    unsafe {
        location = get_dma_residue((*stream).dma);
        location = (*(*stream).buffer).relative_end as c_long - location;
        if location == -1 {
            location = 0;
        }
        bytes_to_frames((*ss).runtime, location)
    }
}

unsafe extern "C" fn alchemy_pcm_new(
    component: *mut snd_soc_component,
    rtd: *mut snd_soc_pcm_runtime,
) -> c_int {
    let pcm: *mut snd_pcm;

    unsafe {
        pcm = (*rtd).pcm;

        snd_pcm_set_managed_buffer_all(
            pcm,
            SNDRV_DMA_TYPE_CONTINUOUS,
            ptr::null_mut(),
            65536,
            (4096 * 1024) - 1,
        );

        0
    }
}

static alchemy_pcm_soc_component: snd_soc_component_driver = snd_soc_component_driver {
    name: DRV_NAME.as_ptr() as *const c_char,
    open: Some(alchemy_pcm_open),
    close: Some(alchemy_pcm_close),
    hw_params: Some(alchemy_pcm_hw_params),
    hw_free: Some(alchemy_pcm_hw_free),
    trigger: Some(alchemy_pcm_trigger),
    pointer: Some(alchemy_pcm_pointer),
    pcm_new: Some(alchemy_pcm_new),
};

unsafe extern "C" fn alchemy_pcm_drvprobe(pdev: *mut platform_device) -> c_int {
    let ctx: *mut alchemy_pcm_ctx;

    unsafe {
        ctx = devm_kzalloc(
            &mut (*pdev).dev,
            size_of::<alchemy_pcm_ctx>(),
            GFP_KERNEL,
        ) as *mut alchemy_pcm_ctx;
        if ctx.is_null() {
            return -ENOMEM;
        }

        platform_set_drvdata(pdev, ctx as *mut c_void);

        devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &alchemy_pcm_soc_component,
            ptr::null_mut(),
            0,
        )
    }
}

static mut alchemy_pcmdma_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"alchemy-pcm-dma".as_ptr(),
    },
    probe: Some(alchemy_pcm_drvprobe),
};

// module_platform_driver(alchemy_pcmdma_driver);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Au1000/Au1500/Au1100 Audio DMA driver");
// MODULE_AUTHOR("Manuel Lauss");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
