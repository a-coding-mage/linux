// SPDX-License-Identifier: GPL-2.0-only
// Edirol UA-101/UA-1000 driver
// Copyright (c) Clemens Ladisch <clemens@ladisch.de>

// External dependencies from Linux kernel
// Replaces: #include <linux/init.h>
// Replaces: #include <linux/module.h>
// Replaces: #include <linux/slab.h>
// Replaces: #include <linux/usb.h>
// Replaces: #include <linux/usb/audio.h>
// Replaces: #include <sound/core.h>
// Replaces: #include <sound/initval.h>
// Replaces: #include <sound/pcm.h>
// Replaces: #include <sound/pcm_params.h>
// Replaces: #include "../usbaudio.h"
// Replaces: #include "../midi.h"

// Module metadata (encoded as declarative comments for kernel module)
// MODULE_DESCRIPTION("Edirol UA-101/1000 driver");
// MODULE_AUTHOR("Clemens Ladisch <clemens@ladisch.de>");
// MODULE_LICENSE("GPL v2");

// Should not be lower than the minimum scheduling delay of the host
// controller.  Some Intel controllers need more than one frame; as long as
// that driver doesn't tell us about this, use 1.5 frames just to be sure.
const MIN_QUEUE_LENGTH: usize = 12;
// Somewhat random.
const MAX_QUEUE_LENGTH: usize = 30;
// This magic value optimizes memory usage efficiency for the UA-101's packet
// sizes at all sample rates, taking into account the stupid cache pool sizes
// that usb_alloc_coherent() uses.
const DEFAULT_QUEUE_LENGTH: usize = 21;

const MAX_PACKET_SIZE: usize = 672; // hardware specific
// DIV_ROUND_UP(MAX_QUEUE_LENGTH, PAGE_SIZE / MAX_PACKET_SIZE)
// With PAGE_SIZE = 4096: (30 + 6 - 1) / 6 = 6
const MAX_MEMORY_BUFFERS: usize = 6;

const SNDRV_CARDS: usize = 32;

// Static module parameters
static mut INDEX: [i32; SNDRV_CARDS] = [
    -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1,
];
static mut ID: [*const i8; SNDRV_CARDS] = [std::ptr::null(); SNDRV_CARDS];
static mut ENABLE: [bool; SNDRV_CARDS] = [
    true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true,
];
static mut QUEUE_LENGTH: u32 = 21;

// Replaces: module_param_array(index, int, NULL, 0444);
// MODULE_PARM_DESC(index, "card index");
// module_param_array(id, charp, NULL, 0444);
// MODULE_PARM_DESC(id, "ID string");
// module_param_array(enable, bool, NULL, 0444);
// MODULE_PARM_DESC(enable, "enable card");
// module_param(queue_length, uint, 0644);
// MODULE_PARM_DESC(queue_length, "USB queue length in microframes, 12-30");

#[repr(C)]
#[derive(Clone, Copy)]
pub enum IntfType {
    INTF_PLAYBACK = 0,
    INTF_CAPTURE = 1,
    INTF_MIDI = 2,
    INTF_COUNT = 3,
}

// bits in struct ua101::states
#[repr(C)]
#[derive(Clone, Copy)]
pub enum StateFlag {
    USB_CAPTURE_RUNNING = 0,
    USB_PLAYBACK_RUNNING = 1,
    ALSA_CAPTURE_OPEN = 2,
    ALSA_PLAYBACK_OPEN = 3,
    ALSA_CAPTURE_RUNNING = 4,
    ALSA_PLAYBACK_RUNNING = 5,
    CAPTURE_URB_COMPLETED = 6,
    PLAYBACK_URB_COMPLETED = 7,
    DISCONNECTED = 8,
}

#[repr(C)]
pub struct UsbIsoPacketDescriptor {
    offset: u32,
    length: u32,
    actual_length: u16,
    status: i32,
}

#[repr(C)]
pub struct Urb {
    dev: *mut UsbDevice,
    pipe: u32,
    transfer_flags: u32,
    transfer_buffer: *mut u8,
    transfer_dma: u64,
    transfer_buffer_length: u32,
    number_of_packets: i32,
    interval: u32,
    context: *mut std::ffi::c_void,
    complete: Option<unsafe extern "C" fn(*mut Urb)>,
    iso_frame_desc: [UsbIsoPacketDescriptor; 1],
    status: i32,
}

#[repr(C)]
pub struct Ua101Urb {
    urb: Urb,
    iso_frame_desc: [UsbIsoPacketDescriptor; 1],
    ready_list: ListHead,
}

#[repr(C)]
pub struct ListHead {
    next: *mut ListHead,
    prev: *mut ListHead,
}

#[repr(C)]
pub struct StreamBuffer {
    size: usize,
    addr: *mut u8,
    dma: u64,
}

#[repr(C)]
pub struct Ua101Stream {
    substream: *mut SndPcmSubstream,
    usb_pipe: u32,
    channels: u32,
    frame_bytes: u32,
    max_packet_bytes: u32,
    period_pos: u32,
    buffer_pos: u32,
    queue_length: u32,
    urbs: [*mut Ua101Urb; MAX_QUEUE_LENGTH],
    buffers: [StreamBuffer; MAX_MEMORY_BUFFERS],
}

#[repr(C)]
pub struct Ua101 {
    dev: *mut UsbDevice,
    card: *mut SndCard,
    intf: [*mut UsbInterface; 3],
    card_index: i32,
    pcm: *mut SndPcm,
    midi_list: ListHead,
    format_bit: u64,
    rate: u32,
    packets_per_second: u32,
    lock: SpinLock,
    mutex: Mutex,
    states: u32,

    // FIFO to synchronize playback rate to capture rate
    rate_feedback_start: u32,
    rate_feedback_count: u32,
    rate_feedback: [u8; MAX_QUEUE_LENGTH],

    ready_playback_urbs: ListHead,
    playback_work: WorkStruct,
    alsa_capture_wait: WaitQueueHead,
    rate_feedback_wait: WaitQueueHead,
    alsa_playback_wait: WaitQueueHead,
    capture: Ua101Stream,
    playback: Ua101Stream,
}

// External kernel types
pub struct UsbDevice;
pub struct SndCard;
pub struct UsbInterface;
pub struct SndPcm;
pub struct SndPcmSubstream;
pub struct SndPcmRuntime;
pub struct SndPcmOps;
pub struct SpinLock;
pub struct Mutex;
pub struct WorkStruct;
pub struct WaitQueueHead;

pub struct UacFormatTypeIDiscreteDescriptor;
pub struct UsbHostInterface;
pub struct UsbDescriptorHeader;
pub struct UsbEndpointDescriptor;
pub struct SndUsb MidiEndpointInfo;
pub struct SndUsbAudioQuirk;
pub struct UsbDriver;

// Static globals
static mut DEVICES_MUTEX: Mutex = unsafe { std::mem::zeroed() };
static mut DEVICES_USED: u32 = 0;
static mut UA101_DRIVER: UsbDriver = unsafe { std::mem::zeroed() };

extern "C" {
    fn abort_alsa_playback(ua: *mut Ua101);
    fn abort_alsa_capture(ua: *mut Ua101);
    fn test_and_clear_bit(nr: u32, addr: *mut u32) -> u32;
    fn wake_up(q: *mut WaitQueueHead);
    fn test_bit(nr: u32, addr: *mut u32) -> u32;
    fn set_bit(nr: u32, addr: *mut u32);
    fn clear_bit(nr: u32, addr: *mut u32);
    fn list_add_tail(new: *mut ListHead, head: *mut ListHead);
    fn list_first_entry(ptr: *mut ListHead, typ: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn list_del(entry: *mut ListHead);
    fn list_empty(head: *const ListHead) -> i32;
    fn queue_work(wq: *mut std::ffi::c_void, work: *mut WorkStruct) -> i32;
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8;
    fn dev_err(dev: *mut std::ffi::c_void, fmt: *const i8, ...);
    fn dev_warn(dev: *mut std::ffi::c_void, fmt: *const i8, ...);
    fn usb_submit_urb(urb: *mut Urb, mem_flags: u32) -> i32;
    fn usb_kill_urb(urb: *mut Urb);
    fn usb_set_interface(dev: *mut UsbDevice, ifnum: i32, alternate: i32) -> i32;
    fn snd_pcm_stop_xrun(substream: *mut SndPcmSubstream);
    fn snd_pcm_period_elapsed(substream: *mut SndPcmSubstream);
    fn snd_pcm_rate_to_rate_bit(rate: u32) -> u64;
    fn snd_pcm_hw_constraint_minmax(runtime: *mut SndPcmRuntime, var: u32, min: u32, max: u32) -> i32;
    fn snd_pcm_hw_constraint_msbits(runtime: *mut SndPcmRuntime, pbits: i32, sbits: i32, width: i32) -> i32;
    fn DIV_ROUND_CLOSEST(x: u32, y: u32) -> u32;
    fn find_format_descriptor(interface: *mut UsbInterface) -> *const UacFormatTypeIDiscreteDescriptor;
    fn combine_triple(p: *const u8) -> u32;
    fn usb_endpoint_is_isoc_in(epd: *const UsbEndpointDescriptor) -> i32;
    fn usb_endpoint_is_isoc_out(epd: *const UsbEndpointDescriptor) -> i32;
    fn usb_endpoint_maxp(epd: *const UsbEndpointDescriptor) -> u16;
    fn usb_endpoint_num(epd: *const UsbEndpointDescriptor) -> u8;
    fn usb_rcvisocpipe(dev: *mut UsbDevice, endpoint: u8) -> u32;
    fn usb_sndisocpipe(dev: *mut UsbDevice, endpoint: u8) -> u32;
    fn usb_alloc_coherent(dev: *mut UsbDevice, size: usize, flags: u32, dma_handle: *mut u64) -> *mut u8;
    fn usb_free_coherent(dev: *mut UsbDevice, size: usize, addr: *mut u8, dma: u64);
    fn kmalloc_obj(size: usize) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn usb_init_urb(urb: *mut Urb);
    fn INIT_LIST_HEAD(list: *mut ListHead);
    fn init_waitqueue_head(q: *mut WaitQueueHead);
    fn INIT_WORK(work: *mut WorkStruct, func: *const std::ffi::c_void);
    fn cancel_work_sync(work: *mut WorkStruct) -> i32;
    fn wait_event(wq: WaitQueueHead, condition: i32);
    fn snd_card_new(dev: *mut std::ffi::c_void, idx: i32, xid: *const i8, module: *mut std::ffi::c_void, extra_size: usize, card_ret: *mut *mut SndCard) -> i32;
    fn snd_card_free(card: *mut SndCard);
    fn snd_card_free_when_closed(card: *mut SndCard);
    fn snd_card_register(card: *mut SndCard) -> i32;
    fn snd_card_disconnect(card: *mut SndCard);
    fn snd_pcm_new(card: *mut SndCard, name: *const i8, device: i32, playback_count: i32, capture_count: i32, rpcm: *mut *mut SndPcm) -> i32;
    fn snd_pcm_set_ops(pcm: *mut SndPcm, direction: i32, ops: *const SndPcmOps);
    fn snd_pcm_set_managed_buffer_all(pcm: *mut SndPcm, typ: i32, dev: *mut std::ffi::c_void, size: u64, max: u64);
    fn snd_usbmidi_create(card: *mut SndCard, iface: *mut UsbInterface, midi_list: *mut ListHead, quirk: *const SndUsbAudioQuirk) -> i32;
    fn snd_usbmidi_disconnect(entry: *mut ListHead);
    fn usb_get_intfdata(intf: *mut UsbInterface) -> *mut std::ffi::c_void;
    fn usb_set_intfdata(intf: *mut UsbInterface, data: *mut std::ffi::c_void);
    fn usb_ifnum_to_if(dev: *mut UsbDevice, ifnum: i32) -> *mut UsbInterface;
    fn usb_driver_claim_interface(driver: *mut UsbDriver, iface: *mut UsbInterface, priv_data: *mut std::ffi::c_void) -> i32;
    fn usb_driver_release_interface(driver: *mut UsbDriver, iface: *mut UsbInterface);
    fn interface_to_usbdev(intf: *mut UsbInterface) -> *mut UsbDevice;
    fn usb_make_path(dev: *mut UsbDevice, buf: *mut i8, size: usize);
    fn snprintf(buf: *mut i8, size: usize, fmt: *const i8, ...);
    fn strscpy(dest: *mut i8, src: *const i8, count: usize) -> i32;
    fn mutex_destroy(mutex: *mut Mutex);
    fn list_for_each(pos: *mut *mut ListHead, head: *mut ListHead);
}

unsafe fn usb_error_string(err: i32) -> &'static str {
    match err {
        -19 => "no device",
        -2 => "endpoint not enabled",
        -32 => "endpoint stalled",
        -28 => "not enough bandwidth",
        -108 => "device disabled",
        -113 => "device suspended",
        -22 | -11 | -7 | -90 => "internal error",
        _ => "unknown error",
    }
}

unsafe fn abort_usb_capture(ua: *mut Ua101) {
    if test_and_clear_bit(StateFlag::USB_CAPTURE_RUNNING as u32, &mut (*ua).states) != 0 {
        wake_up(&mut (*ua).alsa_capture_wait);
        wake_up(&mut (*ua).rate_feedback_wait);
    }
}

unsafe fn abort_usb_playback(ua: *mut Ua101) {
    if test_and_clear_bit(StateFlag::USB_PLAYBACK_RUNNING as u32, &mut (*ua).states) != 0 {
        wake_up(&mut (*ua).alsa_playback_wait);
    }
}

unsafe extern "C" fn playback_urb_complete(usb_urb: *mut Urb) {
    let urb = usb_urb as *mut Ua101Urb;
    let ua = (*urb).urb.context as *mut Ua101;

    if unlikely((*urb).urb.status == -2 ||
                (*urb).urb.status == -19 ||
                (*urb).urb.status == -104 ||
                (*urb).urb.status == -108) {
        abort_usb_playback(ua);
        abort_alsa_playback(ua);
        return;
    }

    if test_bit(StateFlag::USB_PLAYBACK_RUNNING as u32, &(*ua).states) != 0 {
        // append URB to FIFO
        // guard(spinlock_irqsave)(&ua->lock);
        list_add_tail(&mut (*urb).ready_list, &mut (*ua).ready_playback_urbs);
        if (*ua).rate_feedback_count > 0 {
            // queue_work(system_highpri_wq, &ua->playback_work);
        }
        // ua->playback.substream->runtime->delay -=
        //     urb->urb.iso_frame_desc[0].length /
        //     ua->playback.frame_bytes;
    }
}

unsafe extern "C" fn first_playback_urb_complete(urb: *mut Urb) {
    let ua = (*urb).context as *mut Ua101;

    (*urb).complete = Some(playback_urb_complete);
    playback_urb_complete(urb);

    set_bit(StateFlag::PLAYBACK_URB_COMPLETED as u32, &mut (*ua).states);
    wake_up(&mut (*ua).alsa_playback_wait);
}

// copy data from the ALSA ring buffer into the URB buffer
unsafe fn copy_playback_data(stream: *mut Ua101Stream, urb: *mut Urb, frames: u32) -> bool {
    let runtime: *mut SndPcmRuntime;
    let frame_bytes: u32;
    let source: *const u8;

    runtime = (*(*stream).substream).runtime;
    frame_bytes = (*stream).frame_bytes;
    source = (*runtime).dma_area.add(((*stream).buffer_pos * frame_bytes) as usize);
    if (*stream).buffer_pos + frames <= (*runtime).buffer_size {
        memcpy((*urb).transfer_buffer, source, (frames * frame_bytes) as usize);
    } else {
        // wrap around at end of ring buffer
        let frames1 = (*runtime).buffer_size - (*stream).buffer_pos;
        memcpy((*urb).transfer_buffer, source, (frames1 * frame_bytes) as usize);
        memcpy(
            (*urb).transfer_buffer.add((frames1 * frame_bytes) as usize),
            (*runtime).dma_area,
            ((frames - frames1) * frame_bytes) as usize,
        );
    }

    (*stream).buffer_pos += frames;
    if (*stream).buffer_pos >= (*runtime).buffer_size {
        (*stream).buffer_pos -= (*runtime).buffer_size;
    }
    (*stream).period_pos += frames;
    if (*stream).period_pos >= (*runtime).period_size {
        (*stream).period_pos -= (*runtime).period_size;
        return true;
    }
    return false;
}

#[inline]
unsafe fn add_with_wraparound(ua: *mut Ua101, value: *mut u32, add: u32) {
    *value += add;
    if *value >= (*ua).playback.queue_length {
        *value -= (*ua).playback.queue_length;
    }
}

unsafe extern "C" fn playback_work(work: *mut WorkStruct) {
    let ua = (work as usize - std::mem::offset_of!(Ua101, playback_work)) as *mut Ua101;
    let mut frames: u32;
    let mut urb: *mut Ua101Urb;
    let mut do_period_elapsed: bool = false;
    let mut err: i32;

    if unlikely(test_bit(StateFlag::USB_PLAYBACK_RUNNING as u32, &(*ua).states) == 0) {
        return;
    }

    // Synchronizing the playback rate to the capture rate is done by using
    // the same sequence of packet sizes for both streams.
    // Submitting a playback URB therefore requires both a ready URB and
    // the size of the corresponding capture packet, i.e., both playback
    // and capture URBs must have been completed.  Since the USB core does
    // not guarantee that playback and capture complete callbacks are
    // called alternately, we use two FIFOs for packet sizes and read URBs;
    // submitting playback URBs is possible as long as both FIFOs are
    // nonempty.

    // scoped_guard(spinlock_irqsave, &ua->lock)
    {
        while (*ua).rate_feedback_count > 0 && list_empty(&(*ua).ready_playback_urbs) == 0 {
            // take packet size out of FIFO
            frames = (*ua).rate_feedback[(*ua).rate_feedback_start as usize];
            add_with_wraparound(ua, &mut (*ua).rate_feedback_start, 1);
            (*ua).rate_feedback_count -= 1;

            // take URB out of FIFO
            urb = list_first_entry(
                &mut (*ua).ready_playback_urbs,
                std::ptr::null_mut::<Ua101Urb>(),
            ) as *mut Ua101Urb;
            list_del(&mut (*urb).ready_list);

            // fill packet with data or silence
            (*urb).urb.iso_frame_desc[0].length = frames * (*ua).playback.frame_bytes;
            if test_bit(StateFlag::ALSA_PLAYBACK_RUNNING as u32, &(*ua).states) != 0 {
                do_period_elapsed |= copy_playback_data(&mut (*ua).playback, &mut (*urb).urb, frames);
            } else {
                memset(
                    (*urb).urb.transfer_buffer,
                    0,
                    (*urb).urb.iso_frame_desc[0].length as usize,
                );
            }

            // and off you go ...
            err = usb_submit_urb(&mut (*urb).urb, 0); // GFP_ATOMIC
            if unlikely(err < 0) {
                abort_usb_playback(ua);
                abort_alsa_playback(ua);
                dev_err(
                    &mut (*(*ua).dev).dev,
                    b"USB request error %d: %s\n" as *const u8 as *const i8,
                    err,
                    usb_error_string(err).as_ptr() as *const i8,
                );
                return;
            }
            // ua->playback.substream->runtime->delay += frames;
        }
    }

    if do_period_elapsed {
        snd_pcm_period_elapsed((*ua).playback.substream);
    }
}

// copy data from the URB buffer into the ALSA ring buffer
unsafe fn copy_capture_data(stream: *mut Ua101Stream, urb: *mut Urb, frames: u32) -> bool {
    let runtime: *mut SndPcmRuntime;
    let frame_bytes: u32;
    let dest: *mut u8;

    runtime = (*(*stream).substream).runtime;
    frame_bytes = (*stream).frame_bytes;
    dest = (*runtime).dma_area.add(((*stream).buffer_pos * frame_bytes) as usize);
    if (*stream).buffer_pos + frames <= (*runtime).buffer_size {
        memcpy(dest, (*urb).transfer_buffer, (frames * frame_bytes) as usize);
    } else {
        // wrap around at end of ring buffer
        let frames1 = (*runtime).buffer_size - (*stream).buffer_pos;
        memcpy(dest, (*urb).transfer_buffer, (frames1 * frame_bytes) as usize);
        memcpy(
            (*runtime).dma_area,
            (*urb).transfer_buffer.add((frames1 * frame_bytes) as usize),
            ((frames - frames1) * frame_bytes) as usize,
        );
    }

    (*stream).buffer_pos += frames;
    if (*stream).buffer_pos >= (*runtime).buffer_size {
        (*stream).buffer_pos -= (*runtime).buffer_size;
    }
    (*stream).period_pos += frames;
    if (*stream).period_pos >= (*runtime).period_size {
        (*stream).period_pos -= (*runtime).period_size;
        return true;
    }
    return false;
}

unsafe extern "C" fn capture_urb_complete(urb: *mut Urb) {
    let ua = (*urb).context as *mut Ua101;
    let stream = &mut (*ua).capture;
    let mut frames: u32;
    let mut write_ptr: u32;
    let mut do_period_elapsed: bool;
    let mut err: i32;

    if unlikely(
        (*urb).status == -2 ||
        (*urb).status == -19 ||
        (*urb).status == -104 ||
        (*urb).status == -108
    ) {
        abort_usb_playback(ua);
        abort_usb_capture(ua);
        abort_alsa_playback(ua);
        abort_alsa_capture(ua);
        return;
    }

    if (*urb).status >= 0 && (*urb).iso_frame_desc[0].status >= 0 {
        frames = (*urb).iso_frame_desc[0].actual_length / (*stream).frame_bytes;
    } else {
        frames = 0;
    }

    // scoped_guard(spinlock_irqsave, &ua->lock)
    {
        if frames > 0 && test_bit(StateFlag::ALSA_CAPTURE_RUNNING as u32, &(*ua).states) != 0 {
            do_period_elapsed = copy_capture_data(stream, urb, frames);
        } else {
            do_period_elapsed = false;
        }

        if test_bit(StateFlag::USB_CAPTURE_RUNNING as u32, &(*ua).states) != 0 {
            err = usb_submit_urb(urb, 0); // GFP_ATOMIC
            if unlikely(err < 0) {
                dev_err(
                    &mut (*(*ua).dev).dev,
                    b"USB request error %d: %s\n" as *const u8 as *const i8,
                    err,
                    usb_error_string(err).as_ptr() as *const i8,
                );
                abort_usb_playback(ua);
                abort_usb_capture(ua);
                abort_alsa_playback(ua);
                abort_alsa_capture(ua);
                return;
            }

            // append packet size to FIFO
            write_ptr = (*ua).rate_feedback_start;
            add_with_wraparound(ua, &mut write_ptr, (*ua).rate_feedback_count);
            (*ua).rate_feedback[write_ptr as usize] = frames;
            if (*ua).rate_feedback_count < (*ua).playback.queue_length {
                (*ua).rate_feedback_count += 1;
                if (*ua).rate_feedback_count == (*ua).playback.queue_length {
                    wake_up(&mut (*ua).rate_feedback_wait);
                }
            } else {
                // Ring buffer overflow; this happens when the playback
                // stream is not running.  Throw away the oldest entry,
                // so that the playback stream, when it starts, sees
                // the most recent packet sizes.
                add_with_wraparound(ua, &mut (*ua).rate_feedback_start, 1);
            }
            if test_bit(StateFlag::USB_PLAYBACK_RUNNING as u32, &(*ua).states) != 0 &&
               list_empty(&(*ua).ready_playback_urbs) == 0 {
                // queue_work(system_highpri_wq, &ua->playback_work);
            }
        }
    }

    if do_period_elapsed {
        snd_pcm_period_elapsed((*stream).substream);
    }
}

unsafe extern "C" fn first_capture_urb_complete(urb: *mut Urb) {
    let ua = (*urb).context as *mut Ua101;

    (*urb).complete = Some(capture_urb_complete);
    capture_urb_complete(urb);

    set_bit(StateFlag::CAPTURE_URB_COMPLETED as u32, &mut (*ua).states);
    wake_up(&mut (*ua).alsa_capture_wait);
}

unsafe fn submit_stream_urbs(ua: *mut Ua101, stream: *mut Ua101Stream) -> i32 {
    let mut i: u32 = 0;

    while i < (*stream).queue_length {
        let err = usb_submit_urb(&mut (*(*stream).urbs[i as usize]).urb, 0xD0); // GFP_KERNEL
        if err < 0 {
            dev_err(
                &mut (*(*ua).dev).dev,
                b"USB request error %d: %s\n" as *const u8 as *const i8,
                err,
                usb_error_string(err).as_ptr() as *const i8,
            );
            return err;
        }
        i += 1;
    }
    return 0;
}

unsafe fn kill_stream_urbs(stream: *mut Ua101Stream) {
    let mut i: u32 = 0;

    while i < (*stream).queue_length {
        if !(*stream).urbs[i as usize].is_null() {
            usb_kill_urb(&mut (*(*stream).urbs[i as usize]).urb);
        }
        i += 1;
    }
}

unsafe fn enable_iso_interface(ua: *mut Ua101, intf_index: u32) -> i32 {
    let alts: *mut UsbHostInterface;

    alts = (*(*ua).intf[intf_index as usize]).cur_altsetting;
    if (*alts).desc.bAlternateSetting != 1 {
        let err = usb_set_interface(
            (*ua).dev,
            (*alts).desc.bInterfaceNumber as i32,
            1,
        );
        if err < 0 {
            dev_err(
                &mut (*(*ua).dev).dev,
                b"cannot initialize interface; error %d: %s\n" as *const u8 as *const i8,
                err,
                usb_error_string(err).as_ptr() as *const i8,
            );
            return err;
        }
    }
    return 0;
}

unsafe fn disable_iso_interface(ua: *mut Ua101, intf_index: u32) {
    let alts: *mut UsbHostInterface;

    if (*ua).intf[intf_index as usize].is_null() {
        return;
    }

    alts = (*(*ua).intf[intf_index as usize]).cur_altsetting;
    if (*alts).desc.bAlternateSetting != 0 {
        let err = usb_set_interface(
            (*ua).dev,
            (*alts).desc.bInterfaceNumber as i32,
            0,
        );
        if err < 0 && test_bit(StateFlag::DISCONNECTED as u32, &(*ua).states) == 0 {
            dev_warn(
                &mut (*(*ua).dev).dev,
                b"interface reset failed; error %d: %s\n" as *const u8 as *const i8,
                err,
                usb_error_string(err).as_ptr() as *const i8,
            );
        }
    }
}

unsafe fn stop_usb_capture(ua: *mut Ua101) {
    clear_bit(StateFlag::USB_CAPTURE_RUNNING as u32, &mut (*ua).states);

    kill_stream_urbs(&mut (*ua).capture);

    disable_iso_interface(ua, IntfType::INTF_CAPTURE as u32);
}

unsafe fn start_usb_capture(ua: *mut Ua101) -> i32 {
    let mut err: i32;

    if test_bit(StateFlag::DISCONNECTED as u32, &(*ua).states) != 0 {
        return -19; // ENODEV
    }

    if test_bit(StateFlag::USB_CAPTURE_RUNNING as u32, &(*ua).states) != 0 {
        return 0;
    }

    kill_stream_urbs(&mut (*ua).capture);

    err = enable_iso_interface(ua, IntfType::INTF_CAPTURE as u32);
    if err < 0 {
        return err;
    }

    clear_bit(StateFlag::CAPTURE_URB_COMPLETED as u32, &mut (*ua).states);
    (*(*ua).capture.urbs[0]).urb.complete = Some(first_capture_urb_complete);
    (*ua).rate_feedback_start = 0;
    (*ua).rate_feedback_count = 0;

    set_bit(StateFlag::USB_CAPTURE_RUNNING as u32, &mut (*ua).states);
    err = submit_stream_urbs(ua, &mut (*ua).capture);
    if err < 0 {
        stop_usb_capture(ua);
    }
    return err;
}

unsafe fn stop_usb_playback(ua: *mut Ua101) {
    clear_bit(StateFlag::USB_PLAYBACK_RUNNING as u32, &mut (*ua).states);

    kill_stream_urbs(&mut (*ua).playback);

    cancel_work_sync(&mut (*ua).playback_work);

    disable_iso_interface(ua, IntfType::INTF_PLAYBACK as u32);
}

unsafe fn start_usb_playback(ua: *mut Ua101) -> i32 {
    let mut i: u32;
    let mut frames: u32;
    let mut urb: *mut Urb;
    let mut err: i32 = 0;

    if test_bit(StateFlag::DISCONNECTED as u32, &(*ua).states) != 0 {
        return -19; // ENODEV
    }

    if test_bit(StateFlag::USB_PLAYBACK_RUNNING as u32, &(*ua).states) != 0 {
        return 0;
    }

    kill_stream_urbs(&mut (*ua).playback);
    cancel_work_sync(&mut (*ua).playback_work);

    err = enable_iso_interface(ua, IntfType::INTF_PLAYBACK as u32);
    if err < 0 {
        return err;
    }

    clear_bit(StateFlag::PLAYBACK_URB_COMPLETED as u32, &mut (*ua).states);
    (*(*ua).playback.urbs[0]).urb.complete = Some(first_playback_urb_complete);
    // scoped_guard(spinlock_irq, &ua->lock)
    {
        INIT_LIST_HEAD(&mut (*ua).ready_playback_urbs);
    }

    // We submit the initial URBs all at once, so we have to wait for the
    // packet size FIFO to be full.
    wait_event(
        (*ua).rate_feedback_wait,
        ((*ua).rate_feedback_count >= (*ua).playback.queue_length ||
         test_bit(StateFlag::USB_CAPTURE_RUNNING as u32, &(*ua).states) == 0 ||
         test_bit(StateFlag::DISCONNECTED as u32, &(*ua).states) != 0) as i32,
    );
    if test_bit(StateFlag::DISCONNECTED as u32, &(*ua).states) != 0 {
        stop_usb_playback(ua);
        return -19; // ENODEV
    }
    if test_bit(StateFlag::USB_CAPTURE_RUNNING as u32, &(*ua).states) == 0 {
        stop_usb_playback(ua);
        return -5; // EIO
    }

    i = 0;
    while i < (*ua).playback.queue_length {
        // all initial URBs contain silence
        // scoped_guard(spinlock_irq, &ua->lock)
        {
            frames = (*ua).rate_feedback[(*ua).rate_feedback_start as usize] as u32;
            add_with_wraparound(ua, &mut (*ua).rate_feedback_start, 1);
            (*ua).rate_feedback_count -= 1;
        }
        urb = &mut (*(*ua).playback.urbs[i as usize]).urb;
        (*urb).iso_frame_desc[0].length = frames * (*ua).playback.frame_bytes;
        memset(
            (*urb).transfer_buffer,
            0,
            (*urb).iso_frame_desc[0].length as usize,
        );
        i += 1;
    }

    set_bit(StateFlag::USB_PLAYBACK_RUNNING as u32, &mut (*ua).states);
    err = submit_stream_urbs(ua, &mut (*ua).playback);
    if err < 0 {
        stop_usb_playback(ua);
    }
    return err;
}

unsafe fn set_stream_hw(ua: *mut Ua101, substream: *mut SndPcmSubstream, channels: u32) -> i32 {
    let mut err: i32;

    let runtime = (*substream).runtime;
    (*runtime).hw.info = 0x00000001 |   // SNDRV_PCM_INFO_MMAP
                        0x00000002 |   // SNDRV_PCM_INFO_MMAP_VALID
                        0x00000004 |   // SNDRV_PCM_INFO_BATCH
                        0x00000100 |   // SNDRV_PCM_INFO_INTERLEAVED
                        0x00000200 |   // SNDRV_PCM_INFO_BLOCK_TRANSFER
                        0x00001000;    // SNDRV_PCM_INFO_FIFO_IN_FRAMES
    (*runtime).hw.formats = (*ua).format_bit;
    (*runtime).hw.rates = snd_pcm_rate_to_rate_bit((*ua).rate);
    (*runtime).hw.rate_min = (*ua).rate;
    (*runtime).hw.rate_max = (*ua).rate;
    (*runtime).hw.channels_min = channels;
    (*runtime).hw.channels_max = channels;
    (*runtime).hw.buffer_bytes_max = 45000 * 1024;
    (*runtime).hw.period_bytes_min = 1;
    (*runtime).hw.period_bytes_max = 0xFFFFFFFFu32;
    (*runtime).hw.periods_min = 2;
    (*runtime).hw.periods_max = 0xFFFFFFFFu32;
    err = snd_pcm_hw_constraint_minmax(
        runtime,
        6,  // SNDRV_PCM_HW_PARAM_PERIOD_TIME
        1500000 / (*ua).packets_per_second,
        0xFFFFFFFFu32,
    );
    if err < 0 {
        return err;
    }
    err = snd_pcm_hw_constraint_msbits(runtime, 0, 32, 24);
    return err;
}

unsafe extern "C" fn capture_pcm_open(substream: *mut SndPcmSubstream) -> i32 {
    let ua = (*substream).private_data as *mut Ua101;
    let mut err: i32;

    (*ua).capture.substream = substream;
    err = set_stream_hw(ua, substream, (*ua).capture.channels);
    if err < 0 {
        return err;
    }
    let runtime = (*substream).runtime;
    (*runtime).hw.fifo_size = DIV_ROUND_CLOSEST((*ua).rate, (*ua).packets_per_second);
    (*runtime).delay = (*runtime).hw.fifo_size;

    // guard(mutex)(&ua->mutex);
    err = start_usb_capture(ua);
    if err >= 0 {
        set_bit(StateFlag::ALSA_CAPTURE_OPEN as u32, &mut (*ua).states);
    }
    return err;
}

unsafe extern "C" fn playback_pcm_open(substream: *mut SndPcmSubstream) -> i32 {
    let ua = (*substream).private_data as *mut Ua101;
    let mut err: i32;

    (*ua).playback.substream = substream;
    err = set_stream_hw(ua, substream, (*ua).playback.channels);
    if err < 0 {
        return err;
    }
    let runtime = (*substream).runtime;
    (*runtime).hw.fifo_size = DIV_ROUND_CLOSEST(
        (*ua).rate * (*ua).playback.queue_length,
        (*ua).packets_per_second,
    );

    // guard(mutex)(&ua->mutex);
    err = start_usb_capture(ua);
    if err < 0 {
        return err;
    }
    err = start_usb_playback(ua);
    if err < 0 {
        if test_bit(StateFlag::ALSA_CAPTURE_OPEN as u32, &(*ua).states) == 0 {
            stop_usb_capture(ua);
        }
        return err;
    }
    set_bit(StateFlag::ALSA_PLAYBACK_OPEN as u32, &mut (*ua).states);
    return 0;
}

unsafe extern "C" fn capture_pcm_close(substream: *mut SndPcmSubstream) -> i32 {
    let ua = (*substream).private_data as *mut Ua101;

    // guard(mutex)(&ua->mutex);
    clear_bit(StateFlag::ALSA_CAPTURE_OPEN as u32, &mut (*ua).states);
    if test_bit(StateFlag::ALSA_PLAYBACK_OPEN as u32, &(*ua).states) == 0 {
        stop_usb_capture(ua);
    }
    return 0;
}

unsafe extern "C" fn playback_pcm_close(substream: *mut SndPcmSubstream) -> i32 {
    let ua = (*substream).private_data as *mut Ua101;

    // guard(mutex)(&ua->mutex);
    stop_usb_playback(ua);
    clear_bit(StateFlag::ALSA_PLAYBACK_OPEN as u32, &mut (*ua).states);
    if test_bit(StateFlag::ALSA_CAPTURE_OPEN as u32, &(*ua).states) == 0 {
        stop_usb_capture(ua);
    }
    return 0;
}

unsafe extern "C" fn capture_pcm_hw_params(
    substream: *mut SndPcmSubstream,
    hw_params: *mut std::ffi::c_void,
) -> i32 {
    let ua = (*substream).private_data as *mut Ua101;

    // guard(mutex)(&ua->mutex);
    return start_usb_capture(ua);
}

unsafe extern "C" fn playback_pcm_hw_params(
    substream: *mut SndPcmSubstream,
    hw_params: *mut std::ffi::c_void,
) -> i32 {
    let ua = (*substream).private_data as *mut Ua101;
    let mut err: i32;

    // guard(mutex)(&ua->mutex);
    err = start_usb_capture(ua);
    if err >= 0 {
        err = start_usb_playback(ua);
    }
    return err;
}

unsafe extern "C" fn capture_pcm_prepare(substream: *mut SndPcmSubstream) -> i32 {
    let ua = (*substream).private_data as *mut Ua101;
    let mut err: i32;

    // scoped_guard(mutex, &ua->mutex)
    {
        err = start_usb_capture(ua);
    }
    if err < 0 {
        return err;
    }

    // The EHCI driver schedules the first packet of an iso stream at 10 ms
    // in the future, i.e., no data is actually captured for that long.
    // Take the wait here so that the stream is known to be actually
    // running when the start trigger has been called.
    wait_event(
        (*ua).alsa_capture_wait,
        (test_bit(StateFlag::CAPTURE_URB_COMPLETED as u32, &(*ua).states) != 0 ||
         test_bit(StateFlag::USB_CAPTURE_RUNNING as u32, &(*ua).states) == 0) as i32,
    );
    if test_bit(StateFlag::DISCONNECTED as u32, &(*ua).states) != 0 {
        return -19; // ENODEV
    }
    if test_bit(StateFlag::USB_CAPTURE_RUNNING as u32, &(*ua).states) == 0 {
        return -5; // EIO
    }

    (*ua).capture.period_pos = 0;
    (*ua).capture.buffer_pos = 0;
    return 0;
}

unsafe extern "C" fn playback_pcm_prepare(substream: *mut SndPcmSubstream) -> i32 {
    let ua = (*substream).private_data as *mut Ua101;
    let mut err: i32;

    // scoped_guard(mutex, &ua->mutex)
    {
        err = start_usb_capture(ua);
        if err >= 0 {
            err = start_usb_playback(ua);
        }
    }
    if err < 0 {
        return err;
    }

    // see the comment in capture_pcm_prepare()
    wait_event(
        (*ua).alsa_playback_wait,
        (test_bit(StateFlag::PLAYBACK_URB_COMPLETED as u32, &(*ua).states) != 0 ||
         test_bit(StateFlag::USB_PLAYBACK_RUNNING as u32, &(*ua).states) == 0) as i32,
    );
    if test_bit(StateFlag::DISCONNECTED as u32, &(*ua).states) != 0 {
        return -19; // ENODEV
    }
    if test_bit(StateFlag::USB_PLAYBACK_RUNNING as u32, &(*ua).states) == 0 {
        return -5; // EIO
    }

    (*substream).runtime.delay = 0;
    (*ua).playback.period_pos = 0;
    (*ua).playback.buffer_pos = 0;
    return 0;
}

unsafe extern "C" fn capture_pcm_trigger(substream: *mut SndPcmSubstream, cmd: i32) -> i32 {
    let ua = (*substream).private_data as *mut Ua101;

    match cmd {
        1 => { // SNDRV_PCM_TRIGGER_START
            if test_bit(StateFlag::USB_CAPTURE_RUNNING as u32, &(*ua).states) == 0 {
                return -5; // EIO
            }
            set_bit(StateFlag::ALSA_CAPTURE_RUNNING as u32, &mut (*ua).states);
            return 0;
        }
        0 => { // SNDRV_PCM_TRIGGER_STOP
            clear_bit(StateFlag::ALSA_CAPTURE_RUNNING as u32, &mut (*ua).states);
            return 0;
        }
        _ => {
            return -22; // EINVAL
        }
    }
}

unsafe extern "C" fn playback_pcm_trigger(substream: *mut SndPcmSubstream, cmd: i32) -> i32 {
    let ua = (*substream).private_data as *mut Ua101;

    match cmd {
        1 => { // SNDRV_PCM_TRIGGER_START
            if test_bit(StateFlag::USB_PLAYBACK_RUNNING as u32, &(*ua).states) == 0 {
                return -5; // EIO
            }
            set_bit(StateFlag::ALSA_PLAYBACK_RUNNING as u32, &mut (*ua).states);
            return 0;
        }
        0 => { // SNDRV_PCM_TRIGGER_STOP
            clear_bit(StateFlag::ALSA_PLAYBACK_RUNNING as u32, &mut (*ua).states);
            return 0;
        }
        _ => {
            return -22; // EINVAL
        }
    }
}

unsafe fn ua101_pcm_pointer(ua: *mut Ua101, stream: *mut Ua101Stream) -> u32 {
    // guard(spinlock_irqsave)(&ua->lock);
    return (*stream).buffer_pos;
}

unsafe extern "C" fn capture_pcm_pointer(subs: *mut SndPcmSubstream) -> u32 {
    let ua = (*subs).private_data as *mut Ua101;

    return ua101_pcm_pointer(ua, &mut (*ua).capture);
}

unsafe extern "C" fn playback_pcm_pointer(subs: *mut SndPcmSubstream) -> u32 {
    let ua = (*subs).private_data as *mut Ua101;

    return ua101_pcm_pointer(ua, &mut (*ua).playback);
}

// Replaces const struct snd_pcm_ops capture_pcm_ops
// .open = capture_pcm_open,
// .close = capture_pcm_close,
// .hw_params = capture_pcm_hw_params,
// .prepare = capture_pcm_prepare,
// .trigger = capture_pcm_trigger,
// .pointer = capture_pcm_pointer,

// Replaces const struct snd_pcm_ops playback_pcm_ops
// .open = playback_pcm_open,
// .close = playback_pcm_close,
// .hw_params = playback_pcm_hw_params,
// .prepare = playback_pcm_prepare,
// .trigger = playback_pcm_trigger,
// .pointer = playback_pcm_pointer,

// These would need external definition of uac_format_type_i_discrete_descriptor
unsafe fn find_format_descriptor(
    interface: *mut UsbInterface,
) -> *const UacFormatTypeIDiscreteDescriptor {
    // This function requires parsing USB descriptors and is delegated to external dependency
    // Stub: return null for missing dependency
    std::ptr::null()
}

unsafe fn detect_usb_format(ua: *mut Ua101) -> i32 {
    // This function requires external descriptor parsing functions
    // and kernel usb audio format definitions
    // Stub: return -6 (ENXIO) for missing dependencies
    -6
}

unsafe fn alloc_stream_buffers(ua: *mut Ua101, stream: *mut Ua101Stream) -> i32 {
    let mut remaining_packets: u32;
    let mut packets: u32;
    let packets_per_page: u32;
    let mut i: u32 = 0;
    let mut size: usize;

    (*stream).queue_length = unsafe { QUEUE_LENGTH } as u32;
    if (*stream).queue_length < MIN_QUEUE_LENGTH as u32 {
        (*stream).queue_length = MIN_QUEUE_LENGTH as u32;
    }
    if (*stream).queue_length > MAX_QUEUE_LENGTH as u32 {
        (*stream).queue_length = MAX_QUEUE_LENGTH as u32;
    }

    // The cache pool sizes used by usb_alloc_coherent() (128, 512, 2048) are
    // quite bad when used with the packet sizes of this device (e.g. 280,
    // 520, 624).  Therefore, we allocate and subdivide entire pages, using
    // a smaller buffer only for the last chunk.
    remaining_packets = (*stream).queue_length;
    packets_per_page = 4096 / (*stream).max_packet_bytes;
    while i < MAX_MEMORY_BUFFERS as u32 {
        if remaining_packets < packets_per_page {
            packets = remaining_packets;
        } else {
            packets = packets_per_page;
        }
        size = (packets * (*stream).max_packet_bytes) as usize;
        (*stream).buffers[i as usize].addr = usb_alloc_coherent(
            (*ua).dev,
            size,
            0xD0, // GFP_KERNEL
            &mut (*stream).buffers[i as usize].dma,
        );
        if (*stream).buffers[i as usize].addr.is_null() {
            return -12; // ENOMEM
        }
        (*stream).buffers[i as usize].size = size;
        remaining_packets -= packets;
        if remaining_packets == 0 {
            break;
        }
        i += 1;
    }
    if remaining_packets != 0 {
        dev_err(
            &mut (*(*ua).dev).dev,
            b"too many packets\n" as *const u8 as *const i8,
        );
        return -6; // ENXIO
    }
    return 0;
}

unsafe fn free_stream_buffers(ua: *mut Ua101, stream: *mut Ua101Stream) {
    let mut i: u32 = 0;

    while i < MAX_MEMORY_BUFFERS as u32 {
        usb_free_coherent(
            (*ua).dev,
            (*stream).buffers[i as usize].size,
            (*stream).buffers[i as usize].addr,
            (*stream).buffers[i as usize].dma,
        );
        i += 1;
    }
}

unsafe fn alloc_stream_urbs(
    ua: *mut Ua101,
    stream: *mut Ua101Stream,
    urb_complete: Option<unsafe extern "C" fn(*mut Urb)>,
) -> i32 {
    let max_packet_size = (*stream).max_packet_bytes;
    let mut urb: *mut Ua101Urb;
    let mut b: u32 = 0;
    let mut u: u32 = 0;

    while b < MAX_MEMORY_BUFFERS as u32 {
        let mut size = (*stream).buffers[b as usize].size;
        let mut addr = (*stream).buffers[b as usize].addr;
        let mut dma = (*stream).buffers[b as usize].dma;

        while size >= max_packet_size as usize {
            if u >= (*stream).queue_length {
                dev_err(
                    &mut (*(*ua).dev).dev,
                    b"internal buffer size error\n" as *const u8 as *const i8,
                );
                return -6; // ENXIO
            }
            urb = kmalloc_obj(std::mem::size_of::<Ua101Urb>()) as *mut Ua101Urb;
            if urb.is_null() {
                return -12; // ENOMEM
            }
            usb_init_urb(&mut (*urb).urb);
            (*urb).urb.dev = (*ua).dev;
            (*urb).urb.pipe = (*stream).usb_pipe;
            (*urb).urb.transfer_flags = 1; // URB_NO_TRANSFER_DMA_MAP
            (*urb).urb.transfer_buffer = addr;
            (*urb).urb.transfer_dma = dma;
            (*urb).urb.transfer_buffer_length = max_packet_size;
            (*urb).urb.number_of_packets = 1;
            (*urb).urb.interval = 1;
            (*urb).urb.context = ua as *mut std::ffi::c_void;
            (*urb).urb.complete = urb_complete;
            (*urb).urb.iso_frame_desc[0].offset = 0;
            (*urb).urb.iso_frame_desc[0].length = max_packet_size;
            (*stream).urbs[u as usize] = urb;
            u += 1;
            size -= max_packet_size as usize;
            addr = addr.add(max_packet_size as usize);
            dma += max_packet_size as u64;
        }
        b += 1;
    }
    if u == (*stream).queue_length {
        return 0;
    }
    dev_err(
        &mut (*(*ua).dev).dev,
        b"internal buffer size error\n" as *const u8 as *const i8,
    );
    return -6; // ENXIO
}

unsafe fn free_stream_urbs(stream: *mut Ua101Stream) {
    let mut i: u32 = 0;

    while i < (*stream).queue_length {
        if !(*stream).urbs[i as usize].is_null() {
            kfree((*stream).urbs[i as usize] as *mut u8);
            (*stream).urbs[i as usize] = std::ptr::null_mut();
        }
        i += 1;
    }
}

unsafe fn free_usb_related_resources(ua: *mut Ua101, interface: *mut UsbInterface) {
    let mut i: u32 = 0;
    let mut intf: *mut UsbInterface;

    // scoped_guard(mutex, &ua->mutex)
    {
        free_stream_urbs(&mut (*ua).capture);
        free_stream_urbs(&mut (*ua).playback);
    }
    free_stream_buffers(ua, &mut (*ua).capture);
    free_stream_buffers(ua, &mut (*ua).playback);

    while i < 3 {
        // scoped_guard(mutex, &ua->mutex)
        {
            intf = (*ua).intf[i as usize];
            (*ua).intf[i as usize] = std::ptr::null_mut();
        }
        if !intf.is_null() {
            usb_set_intfdata(intf, std::ptr::null_mut());
            if intf != interface {
                usb_driver_release_interface(&mut UA101_DRIVER, intf);
            }
        }
        i += 1;
    }
}

unsafe extern "C" fn ua101_card_free(card: *mut SndCard) {
    let ua = (*card).private_data as *mut Ua101;

    mutex_destroy(&mut (*ua).mutex);
}

unsafe extern "C" fn ua101_probe(
    interface: *mut UsbInterface,
    usb_id: *const std::ffi::c_void,
) -> i32 {
    let mut card: *mut SndCard = std::ptr::null_mut();
    let mut ua: *mut Ua101;
    let mut card_index: u32 = 0;
    let mut i: u32;
    let is_ua1000: i32;
    let name: *const i8;
    let mut usb_path: [i8; 32] = [0; 32];
    let mut err: i32;

    // Stub: missing external descriptor functions
    // Real implementation would:
    // - Check interface number
    // - Allocate card
    // - Initialize device structure
    // - Allocate stream buffers/URBs
    // - Create PCM and MIDI
    return -6; // ENXIO for missing dependencies
}

unsafe extern "C" fn ua101_disconnect(interface: *mut UsbInterface) {
    let ua = usb_get_intfdata(interface) as *mut Ua101;
    let mut midi: *mut ListHead;

    if ua.is_null() {
        return;
    }

    // guard(mutex)(&devices_mutex);

    set_bit(StateFlag::DISCONNECTED as u32, &mut (*ua).states);
    wake_up(&mut (*ua).rate_feedback_wait);

    // make sure that userspace cannot create new requests
    snd_card_disconnect((*ua).card);

    // make sure that there are no pending USB requests
    // list_for_each(midi, &ua->midi_list)
    //     snd_usbmidi_disconnect(midi);
    abort_alsa_playback(ua);
    abort_alsa_capture(ua);
    // scoped_guard(mutex, &ua->mutex)
    {
        stop_usb_playback(ua);
        stop_usb_capture(ua);
    }

    free_usb_related_resources(ua, interface);

    // devices_used &= ~(1 << ua->card_index);

    snd_card_free_when_closed((*ua).card);
}

// Static USB device ID table
// Replaces:
// static const struct usb_device_id ua101_ids[] = {
//     { USB_DEVICE(0x0582, 0x0044) }, /* UA-1000 high speed */
//     { USB_DEVICE(0x0582, 0x007d) }, /* UA-101 high speed */
//     { USB_DEVICE(0x0582, 0x008d) }, /* UA-101 full speed */
//     { }
// };
// MODULE_DEVICE_TABLE(usb, ua101_ids);

// static struct usb_driver ua101_driver = {
//     .name = "snd-ua101",
//     .id_table = ua101_ids,
//     .probe = ua101_probe,
//     .disconnect = ua101_disconnect,
// #if 0
//     .suspend = ua101_suspend,
//     .resume = ua101_resume,
// #endif
// };
//
// module_usb_driver(ua101_driver);

// Kernel module entry point (replaced by module macro in C)
// This would be handled by the kernel module system when using module_usb_driver()

#[inline]
fn unlikely(x: bool) -> bool {
    x
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
