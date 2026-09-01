// SPDX-License-Identifier: GPL-2.0
/*
 *   Sound driver for Nintendo 64.
 *
 *   Copyright 2021 Lauri Kasanen
 */

// Rust translation of the C implementation source. Kernel/ALSA declarations
// supplied by included headers in C are represented here as external items.

type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type c_int = core::ffi::c_int;
type c_char = core::ffi::c_char;
type c_void = core::ffi::c_void;
type size_t = usize;
type dma_addr_t = usize;
type snd_pcm_uframes_t = usize;
type irqreturn_t = c_int;

const AI_NTSC_DACRATE: u32 = 48681812;
const AI_STATUS_BUSY: u32 = 1 << 30;
const AI_STATUS_FULL: u32 = 1 << 31;

const AI_ADDR_REG: u8 = 0;
const AI_LEN_REG: u8 = 1;
const AI_CONTROL_REG: u8 = 2;
const AI_STATUS_REG: u8 = 3;
const AI_RATE_REG: u8 = 4;
const AI_BITCLOCK_REG: u8 = 5;

const MI_INTR_REG: u8 = 2;
const MI_MASK_REG: u8 = 3;

const MI_INTR_AI: u32 = 0x04;

const MI_MASK_CLR_AI: u32 = 0x0010;
const MI_MASK_SET_AI: u32 = 0x0020;

const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EBUSY: c_int = 16;

extern "C" {
    static THIS_MODULE: *mut c_void;

    static mut n64audio_driver: platform_driver;

    static SNDRV_DEFAULT_IDX1: c_int;
    static SNDRV_DEFAULT_STR1: *const c_char;

    static n64audio_pcm_hw: snd_pcm_hardware;
    static n64audio_pcm_ops: snd_pcm_ops;

    fn writel(value: u32, addr: *mut u32);
    fn readl(addr: *mut u32) -> u32;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn barrier();

    fn spin_lock_init(lock: *mut spinlock_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);

    fn snd_pcm_running(substream: *mut snd_pcm_substream) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn hw_param_interval(
        params: *mut snd_pcm_hw_params,
        var: c_int,
    ) -> *mut snd_interval;
    fn is_power_of_2(n: u32) -> bool;
    fn snd_interval_checkempty(i: *mut snd_interval) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        step: c_ulong,
    ) -> c_int;
    fn snd_pcm_hw_rule_add(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        func: Option<
            unsafe extern "C" fn(*mut snd_pcm_hw_params, *mut snd_pcm_hw_rule) -> c_int,
        >,
        private: *mut c_void,
        dep: c_int,
        sentinel: c_int,
    ) -> c_int;
    fn snd_pcm_lib_period_bytes(substream: *mut snd_pcm_substream) -> u32;
    fn snd_pcm_lib_buffer_bytes(substream: *mut snd_pcm_substream) -> u32;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, size: c_int) -> snd_pcm_uframes_t;
    fn snd_card_new(
        parent: *mut device,
        idx: c_int,
        xid: *const c_char,
        module: *mut c_void,
        extra_size: size_t,
        card_ret: *mut *mut snd_card,
    ) -> c_int;
    fn snd_card_free(card: *mut snd_card);
    fn snd_card_register(card: *mut snd_card) -> c_int;
    fn dma_alloc_coherent(
        dev: *mut device,
        size: size_t,
        dma_handle: *mut dma_addr_t,
        flag: gfp_t,
    ) -> *mut c_void;
    fn dma_free_coherent(dev: *mut device, size: size_t, cpu_addr: *mut c_void, dma_handle: dma_addr_t);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut u32;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn snd_pcm_new(
        card: *mut snd_card,
        id: *const c_char,
        device: c_int,
        playback_count: c_int,
        capture_count: c_int,
        rpcm: *mut *mut snd_pcm,
    ) -> c_int;
    fn strscpy(dest: *mut c_char, src: *const c_char) -> ssize_t;
    fn snd_pcm_set_ops(pcm: *mut snd_pcm, direction: c_int, ops: *const snd_pcm_ops);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        data: *mut device,
        size: size_t,
        max: size_t,
    );
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_uint,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_ulong,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn platform_driver_probe(
        driver: *mut platform_driver,
        probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    ) -> c_int;
}

type c_ulong = core::ffi::c_ulong;
type gfp_t = c_uint;
type ssize_t = isize;

const SNDRV_PCM_INFO_MMAP: u32 = 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 0;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 0;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: u32 = 0;
const SNDRV_PCM_FMTBIT_S16_BE: u64 = 0;
const SNDRV_PCM_RATE_8000_48000: u32 = 0;
const SNDRV_PCM_HW_PARAM_PERIOD_SIZE: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_DMA_TYPE_VMALLOC: c_int = 0;
const GFP_DMA: gfp_t = 0;
const GFP_KERNEL: gfp_t = 0;
const IRQF_SHARED: c_ulong = 0;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_card {
    dev: *mut device,
    private_data: *mut c_void,
    driver: [c_char; 80],
    shortname: [c_char; 80],
    longname: [c_char; 80],
}

#[repr(C)]
struct snd_pcm {
    private_data: *mut c_void,
    name: [c_char; 80],
}

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
    pcm: *mut snd_pcm,
}

#[repr(C)]
struct snd_pcm_runtime {
    hw: snd_pcm_hardware,
    dma_area: *mut c_void,
    rate: u32,
    period_size: snd_pcm_uframes_t,
    delay: snd_pcm_uframes_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_pcm_hardware {
    info: u32,
    formats: u64,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: size_t,
    period_bytes_min: size_t,
    period_bytes_max: size_t,
    periods_min: u32,
    periods_max: u32,
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_rule {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_interval {
    min: u32,
    max: u32,
    empty: c_uint,
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct platform_driver_inner {
    name: *const c_char,
}

#[repr(C)]
struct platform_driver {
    driver: platform_driver_inner,
}

#[repr(C)]
struct snd_pcm_ops {
    open: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int) -> c_int>,
    pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    close: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
}

#[repr(C)]
struct n64audio_chan {
    substream: *mut snd_pcm_substream,
    pos: c_int,
    nextpos: c_int,
    writesize: u32,
    bufsize: u32,
    lock: spinlock_t,
}

#[repr(C)]
struct n64audio {
    ai_reg_base: *mut u32,
    mi_reg_base: *mut u32,

    ring_base: *mut c_void,
    ring_base_dma: dma_addr_t,

    card: *mut snd_card,

    chan: n64audio_chan,
}

unsafe fn c_str(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe extern "C" fn n64audio_write_reg(priv_: *mut n64audio, reg: u8, value: u32) {
    unsafe {
        writel(value, (*priv_).ai_reg_base.add(reg as usize));
    }
}

unsafe extern "C" fn n64mi_write_reg(priv_: *mut n64audio, reg: u8, value: u32) {
    unsafe {
        writel(value, (*priv_).mi_reg_base.add(reg as usize));
    }
}

unsafe extern "C" fn n64mi_read_reg(priv_: *mut n64audio, reg: u8) -> u32 {
    unsafe { readl((*priv_).mi_reg_base.add(reg as usize)) }
}

unsafe extern "C" fn n64audio_push(priv_: *mut n64audio) {
    unsafe {
        let runtime: *mut snd_pcm_runtime = (*(*priv_).chan.substream).runtime;
        let mut flags: c_ulong = 0;

        spin_lock_irqsave(&mut (*priv_).chan.lock, &mut flags);

        let count: u32 = (*priv_).chan.writesize;

        memcpy(
            ((*priv_).ring_base as *mut u8).add((*priv_).chan.nextpos as usize) as *mut c_void,
            ((*runtime).dma_area as *mut u8).add((*priv_).chan.nextpos as usize) as *const c_void,
            count as size_t,
        );

        /*
         * The hw registers are double-buffered, and the IRQ fires essentially
         * one period behind. The core only allows one period's distance, so we
         * keep a private DMA buffer to afford two.
         */
        n64audio_write_reg(
            priv_,
            AI_ADDR_REG,
            ((*priv_).ring_base_dma + (*priv_).chan.nextpos as usize) as u32,
        );
        barrier();
        n64audio_write_reg(priv_, AI_LEN_REG, count);

        (*priv_).chan.nextpos += count as c_int;
        (*priv_).chan.nextpos %= (*priv_).chan.bufsize as c_int;

        (*runtime).delay = (*runtime).period_size;

        spin_unlock_irqrestore(&mut (*priv_).chan.lock, flags);
    }
}

unsafe extern "C" fn n64audio_isr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    unsafe {
        let priv_: *mut n64audio = dev_id as *mut n64audio;
        let intrs: u32 = n64mi_read_reg(priv_, MI_INTR_REG);

        // Check it's ours
        if (intrs & MI_INTR_AI) == 0 {
            return IRQ_NONE;
        }

        n64audio_write_reg(priv_, AI_STATUS_REG, 1);

        if !(*priv_).chan.substream.is_null() && snd_pcm_running((*priv_).chan.substream) != 0 {
            let mut flags: c_ulong = 0;
            spin_lock_irqsave(&mut (*priv_).chan.lock, &mut flags);
            (*priv_).chan.pos = (*priv_).chan.nextpos;
            spin_unlock_irqrestore(&mut (*priv_).chan.lock, flags);

            snd_pcm_period_elapsed((*priv_).chan.substream);
            if !(*priv_).chan.substream.is_null()
                && snd_pcm_running((*priv_).chan.substream) != 0
            {
                n64audio_push(priv_);
            }
        }

        IRQ_HANDLED
    }
}

static N64AUDIO_PCM_HW: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_BLOCK_TRANSFER,
    formats: SNDRV_PCM_FMTBIT_S16_BE,
    rates: SNDRV_PCM_RATE_8000_48000,
    rate_min: 8000,
    rate_max: 48000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: 32768,
    period_bytes_min: 1024,
    period_bytes_max: 32768,
    periods_min: 3,
    // 3 periods lets the double-buffering hw read one buffer behind safely
    periods_max: 128,
};

unsafe extern "C" fn hw_rule_period_size(
    params: *mut snd_pcm_hw_params,
    _rule: *mut snd_pcm_hw_rule,
) -> c_int {
    unsafe {
        let c: *mut snd_interval =
            hw_param_interval(params, SNDRV_PCM_HW_PARAM_PERIOD_SIZE);
        let mut changed: c_int = 0;

        /*
         * The DMA unit has errata on (start + len) & 0x3fff == 0x2000.
         * This constraint makes sure that the period size is not a power of two,
         * which combined with dma_alloc_coherent aligning the buffer to the largest
         * PoT <= size guarantees it won't be hit.
         */

        if is_power_of_2((*c).min) {
            (*c).min = (*c).min.wrapping_add(2);
            changed = 1;
        }
        if is_power_of_2((*c).max) {
            (*c).max = (*c).max.wrapping_sub(2);
            changed = 1;
        }
        if snd_interval_checkempty(c) != 0 {
            (*c).empty = 1;
            return -EINVAL;
        }

        changed
    }
}

unsafe extern "C" fn n64audio_pcm_open(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;
        let mut err: c_int;

        (*runtime).hw = N64AUDIO_PCM_HW;
        err = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
        if err < 0 {
            return err;
        }

        err = snd_pcm_hw_constraint_step(runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_SIZE, 2);
        if err < 0 {
            return err;
        }

        err = snd_pcm_hw_rule_add(
            runtime,
            0,
            SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
            Some(hw_rule_period_size),
            core::ptr::null_mut(),
            SNDRV_PCM_HW_PARAM_PERIOD_SIZE,
            -1,
        );
        if err < 0 {
            return err;
        }

        0
    }
}

unsafe extern "C" fn n64audio_pcm_prepare(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let runtime: *mut snd_pcm_runtime = (*substream).runtime;
        let priv_: *mut n64audio = (*(*substream).pcm).private_data as *mut n64audio;
        let mut rate: u32;

        rate = ((2 * AI_NTSC_DACRATE / (*runtime).rate) + 1) / 2 - 1;

        n64audio_write_reg(priv_, AI_RATE_REG, rate);

        rate /= 66;
        if rate > 16 {
            rate = 16;
        }
        n64audio_write_reg(priv_, AI_BITCLOCK_REG, rate - 1);

        spin_lock_irq(&mut (*priv_).chan.lock);

        /* Setup the pseudo-dma transfer pointers.  */
        (*priv_).chan.pos = 0;
        (*priv_).chan.nextpos = 0;
        (*priv_).chan.substream = substream;
        (*priv_).chan.writesize = snd_pcm_lib_period_bytes(substream);
        (*priv_).chan.bufsize = snd_pcm_lib_buffer_bytes(substream);

        spin_unlock_irq(&mut (*priv_).chan.lock);

        0
    }
}

unsafe extern "C" fn n64audio_pcm_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    unsafe {
        let priv_: *mut n64audio = (*(*substream).pcm).private_data as *mut n64audio;

        match cmd {
            SNDRV_PCM_TRIGGER_START => {
                n64audio_push((*(*substream).pcm).private_data as *mut n64audio);
                n64audio_write_reg(priv_, AI_CONTROL_REG, 1);
                n64mi_write_reg(priv_, MI_MASK_REG, MI_MASK_SET_AI);
            }
            SNDRV_PCM_TRIGGER_STOP => {
                n64audio_write_reg(priv_, AI_CONTROL_REG, 0);
                n64mi_write_reg(priv_, MI_MASK_REG, MI_MASK_CLR_AI);
            }
            _ => {
                return -EINVAL;
            }
        }
        0
    }
}

unsafe extern "C" fn n64audio_pcm_pointer(
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    unsafe {
        let priv_: *mut n64audio = (*(*substream).pcm).private_data as *mut n64audio;

        bytes_to_frames((*substream).runtime, (*priv_).chan.pos)
    }
}

unsafe extern "C" fn n64audio_pcm_close(substream: *mut snd_pcm_substream) -> c_int {
    unsafe {
        let priv_: *mut n64audio = (*(*substream).pcm).private_data as *mut n64audio;

        (*priv_).chan.substream = core::ptr::null_mut();

        0
    }
}

static N64AUDIO_PCM_OPS: snd_pcm_ops = snd_pcm_ops {
    open: Some(n64audio_pcm_open),
    prepare: Some(n64audio_pcm_prepare),
    trigger: Some(n64audio_pcm_trigger),
    pointer: Some(n64audio_pcm_pointer),
    close: Some(n64audio_pcm_close),
};

/*
 * The target device is embedded and RAM-constrained. We save RAM
 * by initializing in __init code that gets dropped late in boot.
 * For the same reason there is no module or unloading support.
 */
unsafe extern "C" fn n64audio_probe(pdev: *mut platform_device) -> c_int {
    unsafe {
        let mut card: *mut snd_card = core::ptr::null_mut();
        let mut pcm: *mut snd_pcm = core::ptr::null_mut();
        let priv_: *mut n64audio;
        let mut err: c_int;
        let irq: c_int;

        err = snd_card_new(
            &mut (*pdev).dev,
            SNDRV_DEFAULT_IDX1,
            SNDRV_DEFAULT_STR1,
            THIS_MODULE,
            core::mem::size_of::<n64audio>(),
            &mut card,
        );
        if err < 0 {
            return err;
        }

        priv_ = (*card).private_data as *mut n64audio;

        spin_lock_init(&mut (*priv_).chan.lock);

        (*priv_).card = card;

        (*priv_).ring_base = dma_alloc_coherent(
            (*card).dev,
            32 * 1024,
            &mut (*priv_).ring_base_dma,
            GFP_DMA | GFP_KERNEL,
        );
        if (*priv_).ring_base.is_null() {
            err = -ENOMEM;
            snd_card_free(card);
            return err;
        }

        (*priv_).mi_reg_base = devm_platform_ioremap_resource(pdev, 0);
        if IS_ERR((*priv_).mi_reg_base as *const c_void) {
            err = PTR_ERR((*priv_).mi_reg_base as *const c_void);
            dma_free_coherent(
                (*card).dev,
                32 * 1024,
                (*priv_).ring_base,
                (*priv_).ring_base_dma,
            );
            snd_card_free(card);
            return err;
        }

        (*priv_).ai_reg_base = devm_platform_ioremap_resource(pdev, 1);
        if IS_ERR((*priv_).ai_reg_base as *const c_void) {
            err = PTR_ERR((*priv_).ai_reg_base as *const c_void);
            dma_free_coherent(
                (*card).dev,
                32 * 1024,
                (*priv_).ring_base,
                (*priv_).ring_base_dma,
            );
            snd_card_free(card);
            return err;
        }

        err = snd_pcm_new(card, c_str(b"N64 Audio\0"), 0, 1, 0, &mut pcm);
        if err < 0 {
            dma_free_coherent(
                (*card).dev,
                32 * 1024,
                (*priv_).ring_base,
                (*priv_).ring_base_dma,
            );
            snd_card_free(card);
            return err;
        }

        (*pcm).private_data = priv_ as *mut c_void;
        strscpy((*pcm).name.as_mut_ptr(), c_str(b"N64 Audio\0"));

        snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &N64AUDIO_PCM_OPS);
        snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, (*card).dev, 0, 0);

        strscpy((*card).driver.as_mut_ptr(), c_str(b"N64 Audio\0"));
        strscpy((*card).shortname.as_mut_ptr(), c_str(b"N64 Audio\0"));
        strscpy((*card).longname.as_mut_ptr(), c_str(b"N64 Audio\0"));

        irq = platform_get_irq(pdev, 0);
        if irq < 0 {
            err = -EINVAL;
            dma_free_coherent(
                (*card).dev,
                32 * 1024,
                (*priv_).ring_base,
                (*priv_).ring_base_dma,
            );
            snd_card_free(card);
            return err;
        }
        if devm_request_irq(
            &mut (*pdev).dev,
            irq as c_uint,
            Some(n64audio_isr),
            IRQF_SHARED,
            c_str(b"N64 Audio\0"),
            priv_ as *mut c_void,
        ) != 0
        {
            err = -EBUSY;
            dma_free_coherent(
                (*card).dev,
                32 * 1024,
                (*priv_).ring_base,
                (*priv_).ring_base_dma,
            );
            snd_card_free(card);
            return err;
        }

        err = snd_card_register(card);
        if err < 0 {
            dma_free_coherent(
                (*card).dev,
                32 * 1024,
                (*priv_).ring_base,
                (*priv_).ring_base_dma,
            );
            snd_card_free(card);
            return err;
        }

        0
    }
}

static mut N64AUDIO_DRIVER: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"n64audio\0".as_ptr() as *const c_char,
    },
};

unsafe extern "C" fn n64audio_init() -> c_int {
    unsafe { platform_driver_probe(&mut N64AUDIO_DRIVER, Some(n64audio_probe)) }
}

// MODULE_AUTHOR("Lauri Kasanen <cand@gmx.com>");
// MODULE_DESCRIPTION("N64 Audio");
// MODULE_LICENSE("GPL");
// module_init(n64audio_init);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
