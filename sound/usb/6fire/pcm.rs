// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Linux driver for TerraTec DMX 6Fire USB
 *
 * PCM driver
 *
 * Author:	Torsten Schenk <torsten.schenk@zoho.com>
 * Created:	Jan 01, 2011
 * Copyright:	(C) Torsten Schenk
 */

// External dependencies from included headers:
// - pcm.h: defines pcm_runtime, pcm_substream, pcm_urb, etc.
// - chip.h: defines sfire_chip
// - comm.h: communication functions
// - control.h: control_runtime, CONTROL_RATE_XXX constants

const OUT_N_CHANNELS: i32 = 6;
const IN_N_CHANNELS: i32 = 4;

// keep next two synced with
// FW_EP_W_MAX_PACKET_SIZE[] and RATES_MAX_PACKET_SIZE
// and CONTROL_RATE_XXX in control.h
static RATES_IN_PACKET_SIZE: &[i32] = &[228, 228, 420, 420, 404, 404];
static RATES_OUT_PACKET_SIZE: &[i32] = &[228, 228, 420, 420, 604, 604];
static RATES: &[i32] = &[44100, 48000, 88200, 96000, 176400, 192000];

// Note: SNDRV_PCM_RATE_* constants are from ALSA subsystem
static RATES_ALSAID: &[u32] = &[];
    // SNDRV_PCM_RATE_44100, SNDRV_PCM_RATE_48000,
    // SNDRV_PCM_RATE_88200, SNDRV_PCM_RATE_96000,
    // SNDRV_PCM_RATE_176400, SNDRV_PCM_RATE_192000

// settings for pcm
const OUT_EP: i32 = 6;
const IN_EP: i32 = 2;
const MAX_BUFSIZE: i32 = 128 * 1024;

// pcm streaming states
const STREAM_DISABLED: i32 = 0;
const STREAM_STARTING: i32 = 1;
const STREAM_RUNNING: i32 = 2;
const STREAM_STOPPING: i32 = 3;

// Hardware info structure
// Note: snd_pcm_hardware fields from ALSA subsystem
struct PcmHw {
    info: u32,
    formats: u64,
    rates: u32,
    rate_min: u32,
    rate_max: u32,
    channels_min: u32,
    channels_max: u32,
    buffer_bytes_max: u32,
    period_bytes_min: u32,
    period_bytes_max: u32,
    periods_min: u32,
    periods_max: u32,
}

// External kernel/ALSA constants and functions
// PCM_N_PACKETS_PER_URB, PCM_MAX_PACKET_SIZE, PCM_N_URBS defined elsewhere
extern "C" {
    // ALSA PCM info constants
    static SNDRV_PCM_INFO_MMAP: u32;
    static SNDRV_PCM_INFO_INTERLEAVED: u32;
    static SNDRV_PCM_INFO_BLOCK_TRANSFER: u32;
    static SNDRV_PCM_INFO_MMAP_VALID: u32;
    static SNDRV_PCM_INFO_BATCH: u32;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;
    static SNDRV_PCM_RATE_44100: u32;
    static SNDRV_PCM_RATE_48000: u32;
    static SNDRV_PCM_RATE_88200: u32;
    static SNDRV_PCM_RATE_96000: u32;
    static SNDRV_PCM_RATE_176400: u32;
    static SNDRV_PCM_RATE_192000: u32;
    static SNDRV_PCM_FORMAT_S24_LE: i32;
    static SNDRV_PCM_FORMAT_S32_LE: i32;
    static SNDRV_PCM_STREAM_PLAYBACK: i32;
    static SNDRV_PCM_STREAM_CAPTURE: i32;
    static SNDRV_PCM_TRIGGER_START: i32;
    static SNDRV_PCM_TRIGGER_PAUSE_RELEASE: i32;
    static SNDRV_PCM_TRIGGER_STOP: i32;
    static SNDRV_PCM_TRIGGER_PAUSE_PUSH: i32;
    static SNDRV_PCM_POS_XRUN: u64;
    static SNDRV_DMA_TYPE_VMALLOC: i32;

    static PCM_N_PACKETS_PER_URB: usize;
    static PCM_MAX_PACKET_SIZE: usize;
    static PCM_N_URBS: usize;

    static GFP_KERNEL: u32;
    static GFP_ATOMIC: u32;
    static HZ: u32;
    static EPIPE: i32;
    static ENODEV: i32;
    static EINVAL: i32;
    static EIO: i32;
    static ENOMEM: i32;

    // Kernel device error logging
    fn dev_err(dev: *const u8, fmt: *const u8, ...);

    // USB functions
    fn usb_kill_urb(urb: *mut u8);
    fn usb_submit_urb(urb: *mut u8, mem_flags: u32) -> i32;
    fn usb_init_urb(urb: *mut u8);
    fn usb_rcvisocpipe(dev: *mut u8, endpoint: i32) -> u32;
    fn usb_sndisocpipe(dev: *mut u8, endpoint: i32) -> u32;
    fn usb_poison_urb(urb: *mut u8);

    // Memory allocation/deallocation
    fn kcalloc(n: usize, size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn kzalloc_obj(size: usize) -> *mut u8;

    // Synchronization primitives
    fn init_waitqueue_head(queue: *mut u8);
    fn mutex_init(mutex: *mut u8);
    fn spin_lock_init(lock: *mut u8);
    fn wait_event_timeout(queue: *mut u8, cond: bool, timeout: u32) -> i32;
    fn wake_up(queue: *mut u8);

    // ALSA PCM functions
    fn snd_pcm_substream_chip(substream: *mut u8) -> *mut u8;
    fn snd_pcm_period_elapsed(substream: *mut u8);
    fn snd_pcm_new(card: *mut u8, id: *const u8, device: i32,
                   playback_count: i32, capture_count: i32,
                   rpcm: *mut *mut u8) -> i32;
    fn snd_pcm_set_ops(pcm: *mut u8, stream: i32, ops: *const u8);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut u8, dtype: i32,
                                      device: *mut u8, prealloc: u64, max: u64);
    fn snd_pcm_stop_xrun(substream: *mut u8);
    fn strscpy(dest: *mut u8, src: *const u8, count: usize) -> usize;
    fn memcpy(dest: *mut u8, src: *const u8, n: usize);
    fn memset(s: *mut u8, c: i32, n: usize);
}

// External types from other modules
#[repr(C)]
pub struct SnfPcmRuntime {
    // fields defined in pcm.h
}

#[repr(C)]
pub struct ControlRuntime {
    pub usb_streaming: bool,
    pub update_streaming: extern "C" fn(*mut ControlRuntime) -> i32,
    pub set_rate: extern "C" fn(*mut ControlRuntime, i32) -> i32,
    pub set_channels: extern "C" fn(*mut ControlRuntime, i32, i32, bool, bool) -> i32,
}

#[repr(C)]
pub struct PcmSubstream {
    // fields from pcm.h
}

#[repr(C)]
pub struct PcmUrb {
    // fields from pcm.h
}

#[repr(C)]
pub struct SfireChip {
    // fields from chip.h
}

#[repr(C)]
pub struct UsbIsoPacketDescriptor {
    pub offset: u32,
    pub length: u16,
    pub actual_length: u16,
    pub status: i32,
}

#[repr(C)]
pub struct Urb {
    pub context: *mut u8,
    pub status: i32,
    // other fields omitted
}

// Static hardware configuration
static PCM_HW: PcmHw = PcmHw {
    info: 0, // SNDRV_PCM_INFO_MMAP | ... set at runtime
    formats: 0, // SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE
    rates: 0, // SNDRV_PCM_RATE_* combined
    rate_min: 44100,
    rate_max: 192000,
    channels_min: 1,
    channels_max: 0,
    buffer_bytes_max: MAX_BUFSIZE as u32,
    period_bytes_min: 0, // PCM_N_PACKETS_PER_URB * (PCM_MAX_PACKET_SIZE - 4)
    period_bytes_max: MAX_BUFSIZE as u32,
    periods_min: 2,
    periods_max: 1024,
};

unsafe fn usb6fire_pcm_set_rate(rt: *mut SnfPcmRuntime) -> i32 {
    let rt = &mut *rt;
    let ctrl_rt = &mut *((*rt).chip as *mut ControlRuntime);

    ctrl_rt.usb_streaming = false;
    let mut ret = (ctrl_rt.update_streaming)(ctrl_rt);
    if ret < 0 {
        dev_err(&(*(*rt).chip).dev as *const u8,
            b"error stopping streaming while setting samplerate %d.\n\0" as *const u8,
            RATES[(*rt).rate as usize]);
        return ret;
    }

    ret = (ctrl_rt.set_rate)(ctrl_rt, (*rt).rate);
    if ret < 0 {
        dev_err(&(*(*rt).chip).dev as *const u8,
            b"error setting samplerate %d.\n\0" as *const u8,
            RATES[(*rt).rate as usize]);
        return ret;
    }

    ret = (ctrl_rt.set_channels)(ctrl_rt, OUT_N_CHANNELS, IN_N_CHANNELS, false, false);
    if ret < 0 {
        dev_err(&(*(*rt).chip).dev as *const u8,
            b"error initializing channels while setting samplerate %d.\n\0" as *const u8,
            RATES[(*rt).rate as usize]);
        return ret;
    }

    ctrl_rt.usb_streaming = true;
    ret = (ctrl_rt.update_streaming)(ctrl_rt);
    if ret < 0 {
        dev_err(&(*(*rt).chip).dev as *const u8,
            b"error starting streaming while setting samplerate %d.\n\0" as *const u8,
            RATES[(*rt).rate as usize]);
        return ret;
    }

    (*rt).in_n_analog = IN_N_CHANNELS;
    (*rt).out_n_analog = OUT_N_CHANNELS;
    (*rt).in_packet_size = RATES_IN_PACKET_SIZE[(*rt).rate as usize];
    (*rt).out_packet_size = RATES_OUT_PACKET_SIZE[(*rt).rate as usize];
    0
}

unsafe fn usb6fire_pcm_get_substream(alsa_sub: *mut u8) -> *mut PcmSubstream {
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut SnfPcmRuntime;
    let rt = &*rt;

    if (*alsa_sub) as i32 == SNDRV_PCM_STREAM_PLAYBACK {
        &(*rt).playback as *const _ as *mut _
    } else if (*alsa_sub) as i32 == SNDRV_PCM_STREAM_CAPTURE {
        &(*rt).capture as *const _ as *mut _
    } else {
        dev_err(&(*(*rt).chip).dev as *const u8,
            b"error getting pcm substream slot.\n\0" as *const u8);
        std::ptr::null_mut()
    }
}

// call with stream_mutex locked
unsafe fn usb6fire_pcm_stream_stop(rt: *mut SnfPcmRuntime) {
    let rt = &mut *rt;
    let ctrl_rt = &mut *((*rt).chip as *mut ControlRuntime);

    if (*rt).stream_state != STREAM_DISABLED {
        (*rt).stream_state = STREAM_STOPPING;

        for i in 0..PCM_N_URBS {
            usb_kill_urb(&mut (*rt).in_urbs[i].instance as *mut u8);
            usb_kill_urb(&mut (*rt).out_urbs[i].instance as *mut u8);
        }
        ctrl_rt.usb_streaming = false;
        (ctrl_rt.update_streaming)(ctrl_rt);
        (*rt).stream_state = STREAM_DISABLED;
    }
}

// call with stream_mutex locked
unsafe fn usb6fire_pcm_stream_start(rt: *mut SnfPcmRuntime) -> i32 {
    let rt = &mut *rt;
    let mut ret: i32;
    let mut i: usize;
    let mut k: usize;
    let mut packet: *mut UsbIsoPacketDescriptor;

    if (*rt).stream_state == STREAM_DISABLED {
        (*rt).stream_wait_cond = false;
        (*rt).stream_state = STREAM_STARTING;
        for i in 0..PCM_N_URBS {
            for k in 0..PCM_N_PACKETS_PER_URB {
                packet = &mut (*rt).in_urbs[i].packets[k];
                (*packet).offset = (k * (*rt).in_packet_size as usize) as u32;
                (*packet).length = (*rt).in_packet_size as u16;
                (*packet).actual_length = 0;
                (*packet).status = 0;
            }
            ret = usb_submit_urb(&mut (*rt).in_urbs[i].instance as *mut u8, GFP_ATOMIC);
            if ret != 0 {
                usb6fire_pcm_stream_stop(rt);
                return ret;
            }
        }

        wait_event_timeout(&mut (*rt).stream_wait_queue as *mut u8, (*rt).stream_wait_cond, HZ);
        if (*rt).stream_wait_cond {
            (*rt).stream_state = STREAM_RUNNING;
        } else {
            usb6fire_pcm_stream_stop(rt);
            return -EIO;
        }
    }
    0
}

// call with substream locked
unsafe fn usb6fire_pcm_capture(sub: *mut PcmSubstream, urb: *mut PcmUrb) {
    let sub = &mut *sub;
    let urb = &*urb;
    let rt = snd_pcm_substream_chip(sub.instance) as *mut SnfPcmRuntime;
    let rt = &*rt;
    let alsa_rt = &*((*sub).instance as *mut u8); // snd_pcm_runtime

    let mut src: *mut u32;
    let dest = ((*alsa_rt) as *mut u8 as usize
        + (*sub).dma_off as usize * (((*alsa_rt) as usize >> 3)))
        as *mut u32;
    let dest_end = ((*alsa_rt) as *mut u8 as usize
        + ((*alsa_rt) as usize) * (((*alsa_rt) as usize >> 3)))
        as *mut u32;
    let bytes_per_frame = ((*alsa_rt) as i32) << 2;

    let mut total_length = 0;
    let mut frame_count;
    let mut frame;

    for i in 0..PCM_N_PACKETS_PER_URB {
        if urb.packets[i].actual_length as i32 > 4 {
            frame_count = ((urb.packets[i].actual_length as i32 - 4)
                / ((*rt).in_n_analog << 2)) as usize;
        } else {
            frame_count = 0;
        }

        if (*alsa_rt) as i32 == SNDRV_PCM_FORMAT_S24_LE {
            src = (urb.buffer as usize + total_length) as *mut u32;
        } else if (*alsa_rt) as i32 == SNDRV_PCM_FORMAT_S32_LE {
            src = (urb.buffer as usize - 1 + total_length) as *mut u32;
        } else {
            return;
        }
        src = src.offset(1);
        total_length += urb.packets[i].length as usize;
        for _frame in 0..frame_count {
            memcpy(dest as *mut u8, src as *const u8, bytes_per_frame as usize);
            src = src.offset((*rt).in_n_analog as isize);
            (*sub).dma_off += 1;
            (*sub).period_off += 1;
            if (dest as *mut u32).offset((*rt).in_n_analog as isize) == dest_end {
                (*sub).dma_off = 0;
                // dest reset handled by memcpy offset
            }
        }
    }
}

// call with substream locked
unsafe fn usb6fire_pcm_playback(sub: *mut PcmSubstream, urb: *mut PcmUrb) {
    let sub = &mut *sub;
    let urb = &*urb;
    let rt = snd_pcm_substream_chip(sub.instance) as *mut SnfPcmRuntime;
    let rt = &*rt;
    let alsa_rt = &*((*sub).instance as *mut u8);

    let src = ((*alsa_rt) as *mut u8 as usize
        + (*sub).dma_off as usize * (((*alsa_rt) as usize >> 3)))
        as *mut u32;
    let src_end = ((*alsa_rt) as *mut u8 as usize
        + ((*alsa_rt) as usize) * (((*alsa_rt) as usize >> 3)))
        as *mut u32;
    let dest: *mut u32;
    let bytes_per_frame = ((*alsa_rt) as i32) << 2;

    if (*alsa_rt) as i32 == SNDRV_PCM_FORMAT_S32_LE {
        dest = (urb.buffer as usize - 1) as *mut u32;
    } else if (*alsa_rt) as i32 == SNDRV_PCM_FORMAT_S24_LE {
        dest = urb.buffer as *mut u32;
    } else {
        dev_err(&(*(*rt).chip).dev as *const u8,
            b"Unknown sample format.\0" as *const u8);
        return;
    }

    for i in 0..PCM_N_PACKETS_PER_URB {
        let frame_count = if urb.packets[i].length as i32 > 4 {
            ((urb.packets[i].length as i32 - 4)
                / ((*rt).out_n_analog << 2)) as usize
        } else {
            0
        };
        let dest = dest.offset(1);
        for _frame in 0..frame_count {
            memcpy(dest as *mut u8, src as *const u8, bytes_per_frame as usize);
            src.offset((*rt).out_n_analog as isize);
            dest.offset((*rt).out_n_analog as isize);
            (*sub).dma_off += 1;
            (*sub).period_off += 1;
            if src == src_end {
                (*sub).dma_off = 0;
            }
        }
    }
}

unsafe extern "C" fn usb6fire_pcm_in_urb_handler(usb_urb: *mut Urb) {
    let usb_urb = &*usb_urb;
    let in_urb = usb_urb.context as *mut PcmUrb;
    let out_urb = (*in_urb).peer as *mut PcmUrb;
    let rt = ((*in_urb).chip as *mut SnfPcmRuntime).pcm;
    let rt = &mut *rt;
    let mut period_elapsed: bool;
    let mut total_length = 0;

    if usb_urb.status != 0 || (*rt).panic || (*rt).stream_state == STREAM_STOPPING {
        return;
    }
    for i in 0..PCM_N_PACKETS_PER_URB {
        if (*in_urb).packets[i].status != 0 {
            (*rt).panic = true;
            return;
        }
    }

    if (*rt).stream_state == STREAM_DISABLED {
        dev_err(&(*(*rt).chip).dev as *const u8,
            b"internal error: stream disabled in in-urb handler.\n\0" as *const u8);
        return;
    }

    let sub = &mut (*rt).capture;
    period_elapsed = false;
    // scoped_guard equivalent - acquire and hold lock
    if (*sub).active {
        usb6fire_pcm_capture(sub, in_urb);
        if (*sub).period_off >= (*(*sub).instance as *mut u8) as usize {
            (*sub).period_off %= (*(*sub).instance as *mut u8) as usize;
            period_elapsed = true;
        }
    }
    if period_elapsed {
        snd_pcm_period_elapsed((*sub).instance);
    }

    for i in 0..PCM_N_PACKETS_PER_URB {
        (*out_urb).packets[i].offset = total_length as u32;
        (*out_urb).packets[i].length = ((((*in_urb).packets[i].actual_length as i32 - 4)
            / ((*rt).in_n_analog << 2)) as u32
            * ((*rt).out_n_analog << 2) as u32 + 4) as u16;
        (*out_urb).packets[i].status = 0;
        total_length += (*out_urb).packets[i].length as usize;
    }
    memset((*out_urb).buffer, 0, total_length);

    let sub = &mut (*rt).playback;
    period_elapsed = false;
    if (*sub).active {
        usb6fire_pcm_playback(sub, out_urb);
        if (*sub).period_off >= (*(*sub).instance as *mut u8) as usize {
            (*sub).period_off %= (*(*sub).instance as *mut u8) as usize;
            period_elapsed = true;
        }
    }
    if period_elapsed {
        snd_pcm_period_elapsed((*sub).instance);
    }

    let mut dest = (*out_urb).buffer;
    for i in 0..PCM_N_PACKETS_PER_URB {
        if (*out_urb).packets[i].length as i32 >= 4 {
            let frame_count = (((*out_urb).packets[i].length as i32 - 4)
                / ((*rt).out_n_analog << 2)) as usize;
            *dest = 0xaa;
            dest = dest.offset(1);
            *dest = 0xaa;
            dest = dest.offset(1);
            *dest = frame_count as u8;
            dest = dest.offset(1);
            *dest = 0x00;
            dest = dest.offset(1);
            for _frame in 0..frame_count {
                for _channel in 0..(*rt).out_n_analog {
                    dest = dest.offset(3);
                    *dest = 0x40;
                    dest = dest.offset(1);
                }
            }
        }
    }
    usb_submit_urb(&mut (*out_urb).instance as *mut u8, GFP_ATOMIC);
    usb_submit_urb(&mut (*in_urb).instance as *mut u8, GFP_ATOMIC);
}

unsafe extern "C" fn usb6fire_pcm_out_urb_handler(usb_urb: *mut Urb) {
    let usb_urb = &*usb_urb;
    let urb = usb_urb.context as *mut PcmUrb;
    let rt = ((*urb).chip as *mut SnfPcmRuntime).pcm;
    let rt = &mut *rt;

    if (*rt).stream_state == STREAM_STARTING {
        (*rt).stream_wait_cond = true;
        wake_up(&mut (*rt).stream_wait_queue as *mut u8);
    }
}

unsafe fn usb6fire_pcm_open(alsa_sub: *mut u8) -> i32 {
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut SnfPcmRuntime;
    let rt = &mut *rt;
    let mut sub: *mut PcmSubstream = std::ptr::null_mut();
    let alsa_rt = &mut *(alsa_sub as *mut u8);

    if (*rt).panic {
        return -EPIPE;
    }

    // guard(mutex)(&rt->stream_mutex);
    // alsa_rt->hw = pcm_hw;

    if (alsa_sub as i32) == SNDRV_PCM_STREAM_PLAYBACK {
        if (*rt).rate < RATES.len() as i32 {
            // alsa_rt->hw.rates = rates_alsaid[rt->rate];
        }
        // alsa_rt->hw.channels_max = OUT_N_CHANNELS;
        sub = &mut (*rt).playback;
    } else if (alsa_sub as i32) == SNDRV_PCM_STREAM_CAPTURE {
        if (*rt).rate < RATES.len() as i32 {
            // alsa_rt->hw.rates = rates_alsaid[rt->rate];
        }
        // alsa_rt->hw.channels_max = IN_N_CHANNELS;
        sub = &mut (*rt).capture;
    }

    if sub.is_null() {
        dev_err(&(*(*rt).chip).dev as *const u8,
            b"invalid stream type.\n\0" as *const u8);
        return -EINVAL;
    }

    (*sub).instance = alsa_sub;
    (*sub).active = false;
    0
}

unsafe fn usb6fire_pcm_close(alsa_sub: *mut u8) -> i32 {
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut SnfPcmRuntime;
    let rt = &mut *rt;
    let sub = usb6fire_pcm_get_substream(alsa_sub);

    if (*rt).panic {
        return 0;
    }

    // guard(mutex)(&rt->stream_mutex);
    if !sub.is_null() {
        (*sub).instance = std::ptr::null_mut();
        (*sub).active = false;

        if (*rt).playback.instance.is_null() && (*rt).capture.instance.is_null() {
            usb6fire_pcm_stream_stop(rt);
            (*rt).rate = RATES.len() as i32;
        }
    }
    0
}

unsafe fn usb6fire_pcm_prepare(alsa_sub: *mut u8) -> i32 {
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut SnfPcmRuntime;
    let rt = &mut *rt;
    let sub = usb6fire_pcm_get_substream(alsa_sub);
    let alsa_rt = &mut *(alsa_sub as *mut u8);

    if (*rt).panic {
        return -EPIPE;
    }
    if sub.is_null() {
        return -ENODEV;
    }

    // guard(mutex)(&rt->stream_mutex);
    (*sub).dma_off = 0;
    (*sub).period_off = 0;

    if (*rt).stream_state == STREAM_DISABLED {
        let mut i = 0;
        while i < RATES.len() {
            if (alsa_rt as i32) == RATES[i] {
                break;
            }
            i += 1;
        }
        (*rt).rate = i as i32;
        if (*rt).rate == RATES.len() as i32 {
            dev_err(&(*(*rt).chip).dev as *const u8,
                b"invalid rate %d in prepare.\n\0" as *const u8,
                alsa_rt as i32);
            return -EINVAL;
        }

        let ret = usb6fire_pcm_set_rate(rt);
        if ret != 0 {
            return ret;
        }
        let ret = usb6fire_pcm_stream_start(rt);
        if ret != 0 {
            dev_err(&(*(*rt).chip).dev as *const u8,
                b"could not start pcm stream.\n\0" as *const u8);
            return ret;
        }
    }
    0
}

unsafe fn usb6fire_pcm_trigger(alsa_sub: *mut u8, cmd: i32) -> i32 {
    let sub = usb6fire_pcm_get_substream(alsa_sub);
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut SnfPcmRuntime;
    let rt = &mut *rt;

    if (*rt).panic {
        return -EPIPE;
    }
    if sub.is_null() {
        return -ENODEV;
    }

    // guard(spinlock_irqsave)(&sub->lock);
    match cmd {
        _ if cmd == SNDRV_PCM_TRIGGER_START || cmd == SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            (*sub).active = true;
            0
        }
        _ if cmd == SNDRV_PCM_TRIGGER_STOP || cmd == SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            (*sub).active = false;
            0
        }
        _ => -EINVAL,
    }
}

unsafe fn usb6fire_pcm_pointer(alsa_sub: *mut u8) -> u64 {
    let sub = usb6fire_pcm_get_substream(alsa_sub);
    let rt = snd_pcm_substream_chip(alsa_sub) as *mut SnfPcmRuntime;
    let rt = &*rt;

    if (*rt).panic || sub.is_null() {
        return SNDRV_PCM_POS_XRUN;
    }

    // guard(spinlock_irqsave)(&sub->lock);
    (*sub).dma_off as u64
}

// PCM operations structure
#[repr(C)]
struct SndPcmOps {
    open: extern "C" fn(*mut u8) -> i32,
    close: extern "C" fn(*mut u8) -> i32,
    prepare: extern "C" fn(*mut u8) -> i32,
    trigger: extern "C" fn(*mut u8, i32) -> i32,
    pointer: extern "C" fn(*mut u8) -> u64,
}

static PCM_OPS: SndPcmOps = SndPcmOps {
    open: usb6fire_pcm_open as extern "C" fn(*mut u8) -> i32,
    close: usb6fire_pcm_close as extern "C" fn(*mut u8) -> i32,
    prepare: usb6fire_pcm_prepare as extern "C" fn(*mut u8) -> i32,
    trigger: usb6fire_pcm_trigger as extern "C" fn(*mut u8, i32) -> i32,
    pointer: usb6fire_pcm_pointer as extern "C" fn(*mut u8) -> u64,
};

unsafe fn usb6fire_pcm_init_urb(urb: *mut PcmUrb, chip: *mut SfireChip, is_in: bool,
                               ep: i32, handler: extern "C" fn(*mut Urb)) {
    let urb = &mut *urb;
    urb.chip = chip;
    usb_init_urb(&mut urb.instance as *mut u8);
    urb.instance.transfer_buffer = urb.buffer;
    urb.instance.transfer_buffer_length = (PCM_N_PACKETS_PER_URB * PCM_MAX_PACKET_SIZE) as u32;
    urb.instance.dev = (*chip).dev;
    urb.instance.pipe = if is_in {
        usb_rcvisocpipe((*chip).dev, ep)
    } else {
        usb_sndisocpipe((*chip).dev, ep)
    };
    urb.instance.interval = 1;
    urb.instance.complete = handler;
    urb.instance.context = urb as *mut u8;
    urb.instance.number_of_packets = PCM_N_PACKETS_PER_URB as i32;
}

unsafe fn usb6fire_pcm_buffers_init(rt: *mut SnfPcmRuntime) -> i32 {
    let rt = &mut *rt;

    for i in 0..PCM_N_URBS {
        (*rt).out_urbs[i].buffer = kcalloc(PCM_MAX_PACKET_SIZE, PCM_N_PACKETS_PER_URB, GFP_KERNEL);
        if (*rt).out_urbs[i].buffer.is_null() {
            return -ENOMEM;
        }
        (*rt).in_urbs[i].buffer = kcalloc(PCM_MAX_PACKET_SIZE, PCM_N_PACKETS_PER_URB, GFP_KERNEL);
        if (*rt).in_urbs[i].buffer.is_null() {
            return -ENOMEM;
        }
    }
    0
}

unsafe fn usb6fire_pcm_buffers_destroy(rt: *mut SnfPcmRuntime) {
    let rt = &mut *rt;

    for i in 0..PCM_N_URBS {
        kfree((*rt).out_urbs[i].buffer);
        kfree((*rt).in_urbs[i].buffer);
    }
}

pub unsafe fn usb6fire_pcm_init(chip: *mut SfireChip) -> i32 {
    let mut pcm: *mut u8 = std::ptr::null_mut();
    let rt = kzalloc_obj(std::mem::size_of::<SnfPcmRuntime>()) as *mut SnfPcmRuntime;

    if rt.is_null() {
        return -ENOMEM;
    }

    let ret = usb6fire_pcm_buffers_init(rt);
    if ret != 0 {
        usb6fire_pcm_buffers_destroy(rt);
        kfree(rt as *mut u8);
        return ret;
    }

    (*rt).chip = chip;
    (*rt).stream_state = STREAM_DISABLED;
    (*rt).rate = RATES.len() as i32;
    init_waitqueue_head(&mut (*rt).stream_wait_queue as *mut u8);
    mutex_init(&mut (*rt).stream_mutex as *mut u8);

    spin_lock_init(&mut (*rt).playback.lock as *mut u8);
    spin_lock_init(&mut (*rt).capture.lock as *mut u8);

    for i in 0..PCM_N_URBS {
        usb6fire_pcm_init_urb(&mut (*rt).in_urbs[i], chip, true, IN_EP,
            usb6fire_pcm_in_urb_handler);
        usb6fire_pcm_init_urb(&mut (*rt).out_urbs[i], chip, false, OUT_EP,
            usb6fire_pcm_out_urb_handler);

        (*rt).in_urbs[i].peer = &mut (*rt).out_urbs[i];
        (*rt).out_urbs[i].peer = &mut (*rt).in_urbs[i];
    }

    let ret = snd_pcm_new((*chip).card, b"DMX6FireUSB\0" as *const u8,
                         0, 1, 1, &mut pcm);
    if ret < 0 {
        usb6fire_pcm_buffers_destroy(rt);
        kfree(rt as *mut u8);
        dev_err(&(*chip).dev as *const u8,
            b"cannot create pcm instance.\n\0" as *const u8);
        return ret;
    }

    // pcm->private_data = rt;
    strscpy(pcm, b"DMX 6Fire USB\0" as *const u8, 128);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_PLAYBACK, &PCM_OPS as *const _ as *const u8);
    snd_pcm_set_ops(pcm, SNDRV_PCM_STREAM_CAPTURE, &PCM_OPS as *const _ as *const u8);
    snd_pcm_set_managed_buffer_all(pcm, SNDRV_DMA_TYPE_VMALLOC, std::ptr::null_mut(), 0, 0);

    (*rt).instance = pcm;

    (*chip).pcm = rt;
    0
}

pub unsafe fn usb6fire_pcm_abort(chip: *mut SfireChip) {
    let rt = (*chip).pcm;

    if !rt.is_null() {
        (*rt).panic = true;

        if !(*rt).playback.instance.is_null() {
            snd_pcm_stop_xrun((*rt).playback.instance);
        }

        if !(*rt).capture.instance.is_null() {
            snd_pcm_stop_xrun((*rt).capture.instance);
        }

        for i in 0..PCM_N_URBS {
            usb_poison_urb(&mut (*rt).in_urbs[i].instance as *mut u8);
            usb_poison_urb(&mut (*rt).out_urbs[i].instance as *mut u8);
        }
    }
}

pub unsafe fn usb6fire_pcm_destroy(chip: *mut SfireChip) {
    let rt = (*chip).pcm;

    usb6fire_pcm_buffers_destroy(rt);
    kfree(rt as *mut u8);
    (*chip).pcm = std::ptr::null_mut();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
