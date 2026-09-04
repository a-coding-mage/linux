// SPDX-License-Identifier: GPL-2.0-or-later
//
// Linux driver for M2Tech hiFace compatible devices
//
// Copyright 2012-2013 (C) M2TECH S.r.l and Amarula Solutions B.V.
//
// Authors:  Michael Trimarchi <michael@amarulasolutions.com>
//           Antonio Ospite <ao2@amarulasolutions.com>
//
// The driver is based on the work done in TerraTec DMX 6Fire USB

// Depends on: linux/slab.h, sound/pcm.h, pcm.h, chip.h

const OUT_EP: u32 = 0x2;
const PCM_N_URBS: usize = 8;
const PCM_PACKET_SIZE: usize = 4096;
const PCM_BUFFER_SIZE: usize = 2 * PCM_N_URBS * PCM_PACKET_SIZE;

#[repr(C)]
pub struct PcmUrb {
    pub chip: *mut HifaceChip,
    pub instance: Urb,
    pub submitted: UsbAnchor,
    pub buffer: *mut u8,
}

#[repr(C)]
pub struct PcmSubstream {
    pub lock: SpinlockT,
    pub instance: *mut SndPcmSubstream,
    pub active: bool,
    pub dma_off: SndPcmUframes,
    pub period_off: SndPcmUframes,
}

pub const STREAM_DISABLED: u8 = 0;
pub const STREAM_STARTING: u8 = 1;
pub const STREAM_RUNNING: u8 = 2;
pub const STREAM_STOPPING: u8 = 3;

#[repr(C)]
pub struct PcmRuntime {
    pub chip: *mut HifaceChip,
    pub instance: *mut SndPcm,
    pub playback: PcmSubstream,
    pub panic: bool,
    pub out_urbs: [PcmUrb; PCM_N_URBS],
    pub stream_mutex: MutexT,
    pub stream_state: u8,
    pub extra_freq: u8,
    pub stream_wait_queue: WaitQueueHeadT,
    pub stream_wait_cond: bool,
}

static RATES: &[u32] = &[44100, 48000, 88200, 96000, 176400, 192000, 352800, 384000];

#[repr(C)]
pub struct SndPcmHwConstraintList {
    pub count: u32,
    pub list: *const u32,
    pub mask: u32,
}

static CONSTRAINTS_EXTRA_RATES: SndPcmHwConstraintList = SndPcmHwConstraintList {
    count: 8,
    list: RATES.as_ptr(),
    mask: 0,
};

#[repr(C)]
pub struct SndPcmHardware {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u16,
    pub channels_max: u16,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
}

static PCM_HW: SndPcmHardware = SndPcmHardware {
    info: 0, /* SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_BLOCK_TRANSFER | SNDRV_PCM_INFO_PAUSE | SNDRV_PCM_INFO_MMAP_VALID | SNDRV_PCM_INFO_BATCH */
    formats: 0, /* SNDRV_PCM_FMTBIT_S32_LE */
    rates: 0, /* SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_88200 | SNDRV_PCM_RATE_96000 | SNDRV_PCM_RATE_176400 | SNDRV_PCM_RATE_192000 */
    rate_min: 44100,
    rate_max: 192000,
    channels_min: 2,
    channels_max: 2,
    buffer_bytes_max: PCM_BUFFER_SIZE,
    period_bytes_min: PCM_PACKET_SIZE,
    period_bytes_max: PCM_BUFFER_SIZE,
    periods_min: 2,
    periods_max: 1024,
};

const HIFACE_SET_RATE_REQUEST: u8 = 0xb0;
const HIFACE_RATE_44100: u16 = 0x43;
const HIFACE_RATE_48000: u16 = 0x4b;
const HIFACE_RATE_88200: u16 = 0x42;
const HIFACE_RATE_96000: u16 = 0x4a;
const HIFACE_RATE_176400: u16 = 0x40;
const HIFACE_RATE_192000: u16 = 0x48;
const HIFACE_RATE_352800: u16 = 0x58;
const HIFACE_RATE_384000: u16 = 0x68;

pub unsafe fn hiface_pcm_set_rate(rt: *mut PcmRuntime, rate: u32) -> i32 {
    let device = (*(*rt).chip).dev;
    let rate_value: u16;

    match rate {
        44100 => rate_value = HIFACE_RATE_44100,
        48000 => rate_value = HIFACE_RATE_48000,
        88200 => rate_value = HIFACE_RATE_88200,
        96000 => rate_value = HIFACE_RATE_96000,
        176400 => rate_value = HIFACE_RATE_176400,
        192000 => rate_value = HIFACE_RATE_192000,
        352800 => rate_value = HIFACE_RATE_352800,
        384000 => rate_value = HIFACE_RATE_384000,
        _ => {
            dev_err_external!((*device).dev, "Unsupported rate {}\n", rate);
            return -22; /* EINVAL */
        }
    }

    let ret = usb_control_msg_send(
        device,
        0,
        HIFACE_SET_RATE_REQUEST,
        0xc0, /* USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_OTHER */
        rate_value as u32,
        0,
        std::ptr::null_mut() as *mut u8,
        0,
        100,
        0x10, /* GFP_KERNEL */
    );
    if ret != 0 {
        dev_err_external!((*device).dev, "Error setting samplerate {}.\n", rate);
    }

    ret
}

pub unsafe fn hiface_pcm_get_substream(alsa_sub: *mut SndPcmSubstream) -> *mut PcmSubstream {
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut PcmRuntime;
    let device = &(*(*rt).chip).dev;

    if (*alsa_sub).stream == 0 { /* SNDRV_PCM_STREAM_PLAYBACK */
        return &mut (*rt).playback as *mut PcmSubstream;
    }

    dev_err_external!(*device, "Error getting pcm substream slot.\n");
    std::ptr::null_mut()
}

pub unsafe fn hiface_pcm_stream_stop(rt: *mut PcmRuntime) {
    let mut time: i32;

    if (*rt).stream_state != STREAM_DISABLED {
        (*rt).stream_state = STREAM_STOPPING;

        for i in 0..PCM_N_URBS {
            time = usb_wait_anchor_empty_timeout(&mut (*rt).out_urbs[i].submitted, 100);
            if time == 0 {
                usb_kill_anchored_urbs(&mut (*rt).out_urbs[i].submitted);
            }
            usb_kill_urb(&mut (*rt).out_urbs[i].instance);
        }

        (*rt).stream_state = STREAM_DISABLED;
    }
}

pub unsafe fn hiface_pcm_stream_start(rt: *mut PcmRuntime) -> i32 {
    let mut ret = 0;

    if (*rt).stream_state == STREAM_DISABLED {
        (*rt).panic = false;

        (*rt).stream_state = STREAM_STARTING;
        for i in 0..PCM_N_URBS {
            memset((*rt).out_urbs[i].buffer, 0, PCM_PACKET_SIZE);
            usb_anchor_urb(
                &mut (*rt).out_urbs[i].instance,
                &mut (*rt).out_urbs[i].submitted,
            );
            ret = usb_submit_urb(&mut (*rt).out_urbs[i].instance, 0x10); /* GFP_ATOMIC */
            if ret != 0 {
                hiface_pcm_stream_stop(rt);
                return ret;
            }
        }

        wait_event_timeout(
            &mut (*rt).stream_wait_queue,
            &mut (*rt).stream_wait_cond,
            1, /* HZ */
        );
        if (*rt).stream_wait_cond {
            let device = &(*(*rt).chip).dev;
            dev_dbg_external!(*device, "{}: Stream is running wakeup event\n", "hiface_pcm_stream_start");
            (*rt).stream_state = STREAM_RUNNING;
        } else {
            hiface_pcm_stream_stop(rt);
            return -5; /* EIO */
        }
    }
    ret
}

pub unsafe fn memcpy_swahw32(dest: *mut u8, src: *mut u8, n: usize) {
    for i in 0..(n / 4) {
        let dest_u32 = dest.cast::<u32>();
        let src_u32 = src.cast::<u32>();
        *dest_u32.add(i) = swahw32(*src_u32.add(i));
    }
}

pub unsafe fn hiface_pcm_playback(sub: *mut PcmSubstream, urb: *mut PcmUrb) -> bool {
    let alsa_rt = (*(*sub).instance).runtime;
    let device = &(*(*urb).chip).dev;
    let source: *mut u8;
    let pcm_buffer_size: usize;

    debug_assert_eq!((*alsa_rt).format, 2); /* SNDRV_PCM_FORMAT_S32_LE */

    pcm_buffer_size = snd_pcm_lib_buffer_bytes((*sub).instance);

    if (*sub).dma_off + PCM_PACKET_SIZE <= pcm_buffer_size {
        dev_dbg_external!(
            *device,
            "{}: (1) buffer_size {:#x} dma_offset {:#x}\n",
            "hiface_pcm_playback",
            pcm_buffer_size as u32,
            (*sub).dma_off as u32
        );

        source = (*alsa_rt).dma_area.add((*sub).dma_off);
        memcpy_swahw32((*urb).buffer, source, PCM_PACKET_SIZE);
    } else {
        let len: usize;

        dev_dbg_external!(
            *device,
            "{}: (2) buffer_size {:#x} dma_offset {:#x}\n",
            "hiface_pcm_playback",
            pcm_buffer_size as u32,
            (*sub).dma_off as u32
        );

        len = pcm_buffer_size - (*sub).dma_off;

        source = (*alsa_rt).dma_area.add((*sub).dma_off);
        memcpy_swahw32((*urb).buffer, source, len);

        source = (*alsa_rt).dma_area;
        memcpy_swahw32(
            (*urb).buffer.add(len),
            source,
            PCM_PACKET_SIZE - len,
        );
    }
    (*sub).dma_off += PCM_PACKET_SIZE;
    if (*sub).dma_off >= pcm_buffer_size {
        (*sub).dma_off -= pcm_buffer_size;
    }

    (*sub).period_off += PCM_PACKET_SIZE;
    if (*sub).period_off >= (*alsa_rt).period_size {
        (*sub).period_off %= (*alsa_rt).period_size;
        return true;
    }
    false
}

pub unsafe extern "C" fn hiface_pcm_out_urb_handler(usb_urb: *mut Urb) {
    let out_urb = (*usb_urb).context as *mut PcmUrb;
    let rt = (*(*out_urb).chip).pcm as *mut PcmRuntime;
    let sub: *mut PcmSubstream;
    let mut do_period_elapsed = false;
    let ret: i32;

    if (*rt).panic || (*rt).stream_state == STREAM_STOPPING {
        return;
    }

    if (*usb_urb).status == -2 ||    /* ENOENT - unlinked */
       (*usb_urb).status == -19 ||   /* ENODEV - device removed */
       (*usb_urb).status == -104 ||  /* ECONNRESET - unlinked */
       (*usb_urb).status == -108     /* ESHUTDOWN - device disabled */
    {
        (*rt).panic = true;
        return;
    }

    if (*rt).stream_state == STREAM_STARTING {
        (*rt).stream_wait_cond = true;
        wake_up(&mut (*rt).stream_wait_queue);
    }

    sub = &mut (*rt).playback;
    spinlock_irqsave_lock(&mut (*sub).lock);
    if (*sub).active {
        do_period_elapsed = hiface_pcm_playback(sub, out_urb);
    } else {
        memset((*out_urb).buffer, 0, PCM_PACKET_SIZE);
    }
    spinlock_irqsave_unlock(&mut (*sub).lock);

    if do_period_elapsed {
        snd_pcm_period_elapsed((*sub).instance);
    }

    ret = usb_submit_urb(&mut (*out_urb).instance, 0x10); /* GFP_ATOMIC */
    if ret < 0 {
        (*rt).panic = true;
    }
}

pub unsafe fn hiface_pcm_open(alsa_sub: *mut SndPcmSubstream) -> i32 {
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut PcmRuntime;
    let mut sub: *mut PcmSubstream = std::ptr::null_mut();
    let alsa_rt = (*alsa_sub).runtime;

    if (*rt).panic {
        return -32; /* EPIPE */
    }

    mutex_lock(&mut (*rt).stream_mutex);
    (*alsa_rt).hw = PCM_HW;

    if (*alsa_sub).stream == 0 { /* SNDRV_PCM_STREAM_PLAYBACK */
        sub = &mut (*rt).playback;
    }

    if sub.is_null() {
        let device = &(*(*rt).chip).dev;
        dev_err_external!(*device, "Invalid stream type\n");
        mutex_unlock(&mut (*rt).stream_mutex);
        return -22; /* EINVAL */
    }

    if (*rt).extra_freq != 0 {
        (*alsa_rt).hw.rates |= 0x80000000; /* SNDRV_PCM_RATE_KNOT */
        (*alsa_rt).hw.rate_max = 384000;

        let ret = snd_pcm_hw_constraint_list(
            (*alsa_sub).runtime,
            0,
            0, /* SNDRV_PCM_HW_PARAM_RATE */
            &CONSTRAINTS_EXTRA_RATES,
        );
        if ret < 0 {
            mutex_unlock(&mut (*rt).stream_mutex);
            return ret;
        }
    }

    (*sub).instance = alsa_sub;
    (*sub).active = false;
    mutex_unlock(&mut (*rt).stream_mutex);
    0
}

pub unsafe fn hiface_pcm_close(alsa_sub: *mut SndPcmSubstream) -> i32 {
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut PcmRuntime;
    let sub = hiface_pcm_get_substream(alsa_sub);

    if (*rt).panic {
        return 0;
    }

    mutex_lock(&mut (*rt).stream_mutex);
    if !sub.is_null() {
        hiface_pcm_stream_stop(rt);

        spinlock_irqsave_lock(&mut (*sub).lock);
        (*sub).instance = std::ptr::null_mut();
        (*sub).active = false;
        spinlock_irqsave_unlock(&mut (*sub).lock);
    }
    mutex_unlock(&mut (*rt).stream_mutex);
    0
}

pub unsafe fn hiface_pcm_prepare(alsa_sub: *mut SndPcmSubstream) -> i32 {
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut PcmRuntime;
    let sub = hiface_pcm_get_substream(alsa_sub);
    let alsa_rt = (*alsa_sub).runtime;

    if (*rt).panic {
        return -32; /* EPIPE */
    }
    if sub.is_null() {
        return -19; /* ENODEV */
    }

    mutex_lock(&mut (*rt).stream_mutex);

    hiface_pcm_stream_stop(rt);

    (*sub).dma_off = 0;
    (*sub).period_off = 0;

    if (*rt).stream_state == STREAM_DISABLED {
        let mut ret = hiface_pcm_set_rate(rt, (*alsa_rt).rate);
        if ret != 0 {
            mutex_unlock(&mut (*rt).stream_mutex);
            return ret;
        }
        ret = hiface_pcm_stream_start(rt);
        if ret != 0 {
            mutex_unlock(&mut (*rt).stream_mutex);
            return ret;
        }
    }
    mutex_unlock(&mut (*rt).stream_mutex);
    0
}

pub unsafe fn hiface_pcm_trigger(alsa_sub: *mut SndPcmSubstream, cmd: i32) -> i32 {
    let sub = hiface_pcm_get_substream(alsa_sub);
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut PcmRuntime;

    if (*rt).panic {
        return -32; /* EPIPE */
    }
    if sub.is_null() {
        return -19; /* ENODEV */
    }

    match cmd {
        0 | 3 => { /* SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE */
            spinlock_irq_lock(&mut (*sub).lock);
            (*sub).active = true;
            spinlock_irq_unlock(&mut (*sub).lock);
            0
        }
        1 | 4 => { /* SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH */
            spinlock_irq_lock(&mut (*sub).lock);
            (*sub).active = false;
            spinlock_irq_unlock(&mut (*sub).lock);
            0
        }
        _ => -22, /* EINVAL */
    }
}

pub unsafe fn hiface_pcm_pointer(alsa_sub: *mut SndPcmSubstream) -> u64 {
    let sub = hiface_pcm_get_substream(alsa_sub);
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut PcmRuntime;
    let dma_offset: u64;

    if (*rt).panic || sub.is_null() {
        return 0xffffffffffffffff; /* SNDRV_PCM_POS_XRUN */
    }

    spinlock_irqsave_lock(&mut (*sub).lock);
    dma_offset = (*sub).dma_off;
    spinlock_irqsave_unlock(&mut (*sub).lock);
    bytes_to_frames((*alsa_sub).runtime, dma_offset)
}

#[repr(C)]
pub struct SndPcmOps {
    pub open: unsafe extern "C" fn(*mut SndPcmSubstream) -> i32,
    pub close: unsafe extern "C" fn(*mut SndPcmSubstream) -> i32,
    pub prepare: unsafe extern "C" fn(*mut SndPcmSubstream) -> i32,
    pub trigger: unsafe extern "C" fn(*mut SndPcmSubstream, i32) -> i32,
    pub pointer: unsafe extern "C" fn(*mut SndPcmSubstream) -> u64,
}

static PCM_OPS: SndPcmOps = SndPcmOps {
    open: hiface_pcm_open,
    close: hiface_pcm_close,
    prepare: hiface_pcm_prepare,
    trigger: hiface_pcm_trigger,
    pointer: hiface_pcm_pointer,
};

pub unsafe fn hiface_pcm_init_urb(
    urb: *mut PcmUrb,
    chip: *mut HifaceChip,
    ep: u32,
    handler: unsafe extern "C" fn(*mut Urb),
) -> i32 {
    (*urb).chip = chip;
    usb_init_urb(&mut (*urb).instance);

    (*urb).buffer = kzalloc(PCM_PACKET_SIZE, 0x10); /* GFP_KERNEL */
    if (*urb).buffer.is_null() {
        return -12; /* ENOMEM */
    }

    usb_fill_bulk_urb(
        &mut (*urb).instance,
        (*chip).dev,
        usb_sndbulkpipe((*chip).dev, ep),
        (*urb).buffer as *mut libc::c_void,
        PCM_PACKET_SIZE,
        handler,
        urb as *mut libc::c_void,
    );
    if usb_urb_ep_type_check(&mut (*urb).instance) != 0 {
        return -22; /* EINVAL */
    }
    init_usb_anchor(&mut (*urb).submitted);

    0
}

pub unsafe fn hiface_pcm_abort(chip: *mut HifaceChip) {
    let rt = (*chip).pcm;

    if !rt.is_null() {
        (*rt).panic = true;

        mutex_lock(&mut (*rt).stream_mutex);
        hiface_pcm_stream_stop(rt);
        mutex_unlock(&mut (*rt).stream_mutex);
    }
}

unsafe fn hiface_pcm_destroy(chip: *mut HifaceChip) {
    let rt = (*chip).pcm;

    for i in 0..PCM_N_URBS {
        kfree((*rt).out_urbs[i].buffer as *mut libc::c_void);
    }

    kfree(rt as *mut libc::c_void);
    (*chip).pcm = std::ptr::null_mut();
}

unsafe extern "C" fn hiface_pcm_free(pcm: *mut SndPcm) {
    let rt = (*pcm).private_data as *mut PcmRuntime;

    if !rt.is_null() {
        hiface_pcm_destroy((*rt).chip);
    }
}

pub unsafe fn hiface_pcm_init(chip: *mut HifaceChip, extra_freq: u8) -> i32 {
    let mut ret: i32;
    let pcm: *mut SndPcm;
    let rt: *mut PcmRuntime;

    rt = kzalloc(std::mem::size_of::<PcmRuntime>(), 0x10) as *mut PcmRuntime; /* GFP_KERNEL */
    if rt.is_null() {
        return -12; /* ENOMEM */
    }

    (*rt).chip = chip;
    (*rt).stream_state = STREAM_DISABLED;
    if extra_freq != 0 {
        (*rt).extra_freq = 1;
    }

    init_waitqueue_head(&mut (*rt).stream_wait_queue);
    mutex_init(&mut (*rt).stream_mutex);
    spin_lock_init(&mut (*rt).playback.lock);

    for i in 0..PCM_N_URBS {
        ret = hiface_pcm_init_urb(
            &mut (*rt).out_urbs[i],
            chip,
            OUT_EP,
            hiface_pcm_out_urb_handler,
        );
        if ret < 0 {
            goto_error(chip, rt, 0);
            return ret;
        }
    }

    ret = snd_pcm_new((*chip).card, "USB-SPDIF Audio\0".as_ptr() as *const i8, 0, 1, 0, &mut (pcm as *mut *mut SndPcm));
    if ret < 0 {
        dev_err_external!((*chip).dev.dev, "Cannot create pcm instance\n");
        goto_error(chip, rt, PCM_N_URBS);
        return ret;
    }

    (*pcm).private_data = rt as *mut libc::c_void;
    (*pcm).private_free = Some(hiface_pcm_free);

    strscpy((*pcm).name.as_mut_ptr(), "USB-SPDIF Audio\0".as_ptr() as *const i8, 32);
    snd_pcm_set_ops(pcm, 0, &PCM_OPS); /* SNDRV_PCM_STREAM_PLAYBACK */
    snd_pcm_set_managed_buffer_all(pcm, 3, std::ptr::null_mut(), 0, 0); /* SNDRV_DMA_TYPE_VMALLOC */

    (*rt).instance = pcm;

    (*chip).pcm = rt;
    0
}

fn goto_error(chip: *mut HifaceChip, rt: *mut PcmRuntime, start_i: usize) {
    unsafe {
        for i in start_i..PCM_N_URBS {
            kfree((*rt).out_urbs[i].buffer as *mut libc::c_void);
        }
        kfree(rt as *mut libc::c_void);
    }
}

// External declarations for dependencies
extern "C" {
    pub type HifaceChip;
    pub type SndPcmSubstream;
    pub type SndPcm;
    pub type SndPcmRuntime;
    pub type Urb;
    pub type UsbAnchor;
    pub type UsbDevice;
    pub type SpinlockT;
    pub type MutexT;
    pub type WaitQueueHeadT;
    pub type SndPcmUframes;

    fn snd_pcm_substream_chip(substream: *mut SndPcmSubstream) -> *mut libc::c_void;
    fn usb_wait_anchor_empty_timeout(anchor: *mut UsbAnchor, timeout: i32) -> i32;
    fn usb_kill_anchored_urbs(anchor: *mut UsbAnchor);
    fn usb_kill_urb(urb: *mut Urb);
    fn memset(s: *mut libc::c_void, c: i32, n: usize) -> *mut libc::c_void;
    fn usb_anchor_urb(urb: *mut Urb, anchor: *mut UsbAnchor);
    fn usb_submit_urb(urb: *mut Urb, mem_flags: u32) -> i32;
    fn wait_event_timeout(wq: *mut WaitQueueHeadT, cond: *mut bool, timeout: u32) -> i32;
    fn dev_dbg_external(device: *mut libc::c_void, fmt: *const i8, ...);
    fn dev_err_external(device: *mut libc::c_void, fmt: *const i8, ...);
    fn snd_pcm_lib_buffer_bytes(substream: *mut SndPcmSubstream) -> usize;
    fn swahw32(value: u32) -> u32;
    fn snd_pcm_period_elapsed(substream: *mut SndPcmSubstream);
    fn wake_up(wq: *mut WaitQueueHeadT);
    fn spinlock_irqsave_lock(lock: *mut SpinlockT);
    fn spinlock_irqsave_unlock(lock: *mut SpinlockT);
    fn spinlock_irq_lock(lock: *mut SpinlockT);
    fn spinlock_irq_unlock(lock: *mut SpinlockT);
    fn mutex_lock(mutex: *mut MutexT);
    fn mutex_unlock(mutex: *mut MutexT);
    fn snd_pcm_hw_constraint_list(
        runtime: *mut SndPcmRuntime,
        rules: u32,
        var: u32,
        list: *const SndPcmHwConstraintList,
    ) -> i32;
    fn usb_control_msg_send(
        dev: *mut UsbDevice,
        pipe: u32,
        request: u8,
        requesttype: u32,
        value: u32,
        index: u32,
        data: *mut u8,
        size: u32,
        timeout: u32,
        flags: u32,
    ) -> i32;
    fn usb_init_urb(urb: *mut Urb);
    fn kzalloc(size: usize, flags: u32) -> *mut libc::c_void;
    fn kfree(p: *mut libc::c_void);
    fn usb_sndbulkpipe(dev: *mut UsbDevice, endpoint: u32) -> u32;
    fn usb_fill_bulk_urb(
        urb: *mut Urb,
        dev: *mut UsbDevice,
        pipe: u32,
        transfer_buffer: *mut libc::c_void,
        buffer_length: usize,
        complete: unsafe extern "C" fn(*mut Urb),
        context: *mut libc::c_void,
    );
    fn usb_urb_ep_type_check(urb: *mut Urb) -> i32;
    fn init_usb_anchor(anchor: *mut UsbAnchor);
    fn init_waitqueue_head(wq: *mut WaitQueueHeadT);
    fn mutex_init(mutex: *mut MutexT);
    fn spin_lock_init(lock: *mut SpinlockT);
    fn snd_pcm_new(
        card: *mut libc::c_void,
        id: *const i8,
        device: i32,
        playback_count: i32,
        capture_count: i32,
        rpcm: *mut *mut SndPcm,
    ) -> i32;
    fn strscpy(dest: *mut i8, src: *const i8, count: usize) -> usize;
    fn snd_pcm_set_ops(pcm: *mut SndPcm, stream: i32, ops: *const SndPcmOps);
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut SndPcm,
        type_: i32,
        device: *mut libc::c_void,
        size: usize,
        max_size: usize,
    );
    fn bytes_to_frames(runtime: *mut SndPcmRuntime, bytes: u64) -> u64;
}

macro_rules! dev_dbg_external {
    ($dev:expr, $fmt:expr, $($arg:expr),*) => {
        unsafe {
            dev_dbg_external($dev as *mut libc::c_void, concat!($fmt, "\0").as_ptr() as *const i8, $($arg),*);
        }
    };
}

macro_rules! dev_err_external {
    ($dev:expr, $fmt:expr, $($arg:expr),*) => {
        unsafe {
            dev_err_external($dev as *mut libc::c_void, concat!($fmt, "\0").as_ptr() as *const i8, $($arg),*);
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
