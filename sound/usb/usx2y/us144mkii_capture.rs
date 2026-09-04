// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025 Šerif Rami <ramiserifpersia@gmail.com>

// Translated from us144mkii.h include and kernel ALSA/USB APIs.
// External dependencies are declared as stubs; their implementations
// are supplied by the kernel driver framework.

use core::ptr;
use core::mem;

// External types - declared but not defined in this file
#[repr(C)]
pub struct snd_pcm_substream {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct tascam_card {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_ops {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct urb {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _opaque: [u8; 0],
}

// External function declarations
extern "C" {
    fn snd_pcm_substream_chip(substream: *mut snd_pcm_substream) -> *mut tascam_card;
    fn atomic_read(v: *const i32) -> i32;
    fn atomic_set(v: *mut i32, i: i32);
    fn atomic_dec(v: *mut i32);
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn dev_err_ratelimited(dev: *mut core::ffi::c_void, fmt: *const u8, ...);
    fn snd_pcm_lib_ioctl(substream: *mut snd_pcm_substream, cmd: u32, arg: *mut core::ffi::c_void) -> i32;
    fn tascam_pcm_hw_params(substream: *mut snd_pcm_substream) -> i32;
    fn tascam_pcm_hw_free(substream: *mut snd_pcm_substream) -> i32;
    fn tascam_pcm_trigger(substream: *mut snd_pcm_substream) -> i32;
    fn container_of(ptr: *const core::ffi::c_void, container_type: *mut core::ffi::c_void, member: *const u8) -> *mut core::ffi::c_void;
    fn process_capture_routing_us144mkii(tascam: *mut tascam_card, decoded_block: *mut i32, routed_block: *mut i32);
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: usize) -> usize;
    fn do_div(n: *mut u64, base: usize) -> u64;
    fn schedule_work(work: *mut work_struct);
    fn usb_get_urb(urb: *mut urb);
    fn usb_anchor_urb(urb: *mut urb, anchor: *mut core::ffi::c_void);
    fn usb_submit_urb(urb: *mut urb, mem_flags: u32) -> i32;
    fn usb_unanchor_urb(urb: *mut urb);
    fn usb_put_urb(urb: *mut urb);
}

// External constants and macros (defined elsewhere)
const FRAMES_PER_DECODE_BLOCK: usize = 8;
const DECODED_CHANNELS_PER_FRAME: usize = 4;
const DECODED_SAMPLE_SIZE: usize = 4;
const RAW_BYTES_PER_DECODE_BLOCK: usize = 512;
const CAPTURE_RING_BUFFER_SIZE: usize = 0; // Set by caller
const NUM_CHANNELS: i32 = 4;
const BYTES_PER_SAMPLE: i32 = 3;
const GFP_ATOMIC: u32 = 0x20; // Kernel constant
const ENOENT: i32 = -2;
const ECONNRESET: i32 = -104;
const ESHUTDOWN: i32 = -108;
const ENODEV: i32 = -19;
const EPROTO: i32 = -71;

extern "C" {
    static tascam_pcm_hw: core::ffi::c_void;
}

/// tascam_capture_open() - Opens the PCM capture substream.
/// @substream: The ALSA PCM substream to open.
///
/// This function sets the hardware parameters for the capture substream
/// and stores a reference to the substream in the driver's private data.
///
/// Return: 0 on success.
#[no_mangle]
pub unsafe extern "C" fn tascam_capture_open(substream: *mut snd_pcm_substream) -> i32 {
    let tascam = snd_pcm_substream_chip(substream);

    // substream->runtime->hw = tascam_pcm_hw;
    let runtime_ptr = (*substream).runtime;
    // Simplified field access - exact field layout depends on kernel structures
    let hw_field_offset = 0; // This would need actual struct offset
    *(runtime_ptr as *mut *const core::ffi::c_void).add(hw_field_offset) = &tascam_pcm_hw as *const _ as *const core::ffi::c_void;

    // tascam->capture_substream = substream;
    // Assuming capture_substream is at a known offset in tascam_card
    let capture_substream_offset = 0; // Placeholder - actual offset from tascam_card definition
    *(tascam as *mut *mut snd_pcm_substream).add(capture_substream_offset) = substream;

    // atomic_set(&tascam->capture_active, 0);
    // Assuming capture_active is at a known offset
    let capture_active_offset = 0; // Placeholder
    atomic_set((tascam as *mut i32).add(capture_active_offset), 0);

    0
}

/// tascam_capture_close() - Closes the PCM capture substream.
/// @substream: The ALSA PCM substream to close.
///
/// This function clears the reference to the capture substream in the
/// driver's private data.
///
/// Return: 0 on success.
#[no_mangle]
pub unsafe extern "C" fn tascam_capture_close(substream: *mut snd_pcm_substream) -> i32 {
    let tascam = snd_pcm_substream_chip(substream);

    // tascam->capture_substream = NULL;
    let capture_substream_offset = 0;
    *(tascam as *mut *mut snd_pcm_substream).add(capture_substream_offset) = ptr::null_mut();

    0
}

/// tascam_capture_prepare() - Prepares the PCM capture substream for use.
/// @substream: The ALSA PCM substream to prepare.
///
/// This function initializes capture-related counters and ring buffer pointers.
///
/// Return: 0 on success.
#[no_mangle]
pub unsafe extern "C" fn tascam_capture_prepare(substream: *mut snd_pcm_substream) -> i32 {
    let tascam = snd_pcm_substream_chip(substream);

    // tascam->driver_capture_pos = 0;
    // Assuming specific field offsets in tascam_card
    let driver_capture_pos_offset = 0;
    *(tascam as *mut usize).add(driver_capture_pos_offset) = 0;

    // tascam->capture_frames_processed = 0;
    let capture_frames_processed_offset = 1;
    *(tascam as *mut u64).add(capture_frames_processed_offset) = 0;

    // tascam->last_capture_period_pos = 0;
    let last_capture_period_pos_offset = 2;
    *(tascam as *mut usize).add(last_capture_period_pos_offset) = 0;

    // tascam->capture_ring_buffer_read_ptr = 0;
    let capture_ring_buffer_read_ptr_offset = 3;
    *(tascam as *mut usize).add(capture_ring_buffer_read_ptr_offset) = 0;

    // tascam->capture_ring_buffer_write_ptr = 0;
    let capture_ring_buffer_write_ptr_offset = 4;
    *(tascam as *mut usize).add(capture_ring_buffer_write_ptr_offset) = 0;

    0
}

/// tascam_capture_pointer() - Returns the current capture pointer position.
/// @substream: The ALSA PCM substream.
///
/// This function returns the current position of the capture pointer within
/// the ALSA ring buffer, in frames.
///
/// Return: The current capture pointer position in frames.
#[no_mangle]
pub unsafe extern "C" fn tascam_capture_pointer(substream: *mut snd_pcm_substream) -> usize {
    let tascam = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut pos: u64;

    // if (!atomic_read(&tascam->capture_active))
    let capture_active_offset = 0;
    if atomic_read((tascam as *const i32).add(capture_active_offset)) == 0 {
        return 0;
    }

    // scoped_guard(spinlock_irqsave, &tascam->lock) {
    //     pos = tascam->capture_frames_processed;
    // }
    // Simplified - actual spinlock acquisition would be needed
    let capture_frames_processed_offset = 1;
    pos = *(tascam as *const u64).add(capture_frames_processed_offset);

    // if (runtime->buffer_size == 0)
    let buffer_size_offset = 0;
    if *(runtime as *const usize).add(buffer_size_offset) == 0 {
        return 0;
    }

    // return do_div(pos, runtime->buffer_size);
    let buffer_size = *(runtime as *const usize).add(buffer_size_offset);
    do_div(&mut pos, buffer_size)
}

/// tascam_capture_ops - ALSA PCM operations for capture.
///
/// This structure defines the callback functions for capture stream operations,
/// including open, close, ioctl, hardware parameters, hardware free, prepare,
/// trigger, and pointer.
#[no_mangle]
pub static tascam_capture_ops: snd_pcm_ops = unsafe { core::mem::zeroed() };

/// decode_tascam_capture_block() - Decodes a raw 512-byte block from the device.
/// @src_block: Pointer to the 512-byte raw source block.
/// @dst_block: Pointer to the destination buffer for decoded audio frames.
///
/// The device sends audio data in a complex, multiplexed format. This function
/// demultiplexes the bits from the raw block into 8 frames of 4-channel,
/// 24-bit audio (stored in 32-bit containers).
unsafe fn decode_tascam_capture_block(src_block: *const u8, dst_block: *mut i32) {
    // memset(dst_block, 0, FRAMES_PER_DECODE_BLOCK * DECODED_CHANNELS_PER_FRAME * DECODED_SAMPLE_SIZE);
    ptr::write_bytes(
        dst_block,
        0,
        FRAMES_PER_DECODE_BLOCK * DECODED_CHANNELS_PER_FRAME * DECODED_SAMPLE_SIZE,
    );

    for frame in 0..FRAMES_PER_DECODE_BLOCK {
        let p_src_frame_base = src_block.add(frame * 64);
        let p_dst_frame = dst_block.add(frame * 4);

        let mut ch: [i32; 4] = [0; 4];

        for bit in 0..24 {
            let byte1 = *p_src_frame_base.add(bit);
            let byte2 = *p_src_frame_base.add(bit + 32);

            ch[0] = (ch[0] << 1) | ((byte1 & 1) as i32);
            ch[2] = (ch[2] << 1) | (((byte1 >> 1) & 1) as i32);

            ch[1] = (ch[1] << 1) | ((byte2 & 1) as i32);
            ch[3] = (ch[3] << 1) | (((byte2 >> 1) & 1) as i32);
        }

        // The result is a 24-bit sample. Shift left by 8 to align it to
        // the most significant bits of a 32-bit integer (S32_LE format).
        *p_dst_frame.add(0) = ch[0] << 8;
        *p_dst_frame.add(1) = ch[1] << 8;
        *p_dst_frame.add(2) = ch[2] << 8;
        *p_dst_frame.add(3) = ch[3] << 8;
    }
}

#[no_mangle]
pub unsafe extern "C" fn tascam_capture_work_handler(work: *mut work_struct) {
    // struct tascam_card *tascam = container_of(work, struct tascam_card, capture_work);
    let tascam = container_of(work as *const _, ptr::null_mut::<tascam_card>(), ptr::null());
    let tascam = tascam as *mut tascam_card;

    // Simplified field access - these offsets would need to match actual struct layout
    let capture_substream_offset = 0;
    let substream = *(tascam as *const *mut snd_pcm_substream).add(capture_substream_offset);

    let runtime: *mut snd_pcm_runtime;

    let capture_decode_raw_block_offset = 0;
    let raw_block = *(tascam as *const *mut u8).add(capture_decode_raw_block_offset);

    let capture_decode_dst_block_offset = 1;
    let decoded_block = *(tascam as *const *mut i32).add(capture_decode_dst_block_offset);

    let capture_routing_buffer_offset = 2;
    let routed_block = *(tascam as *const *mut i32).add(capture_routing_buffer_offset);

    if substream.is_null() || (*substream).runtime.is_null() {
        return;
    }
    runtime = (*substream).runtime;

    if raw_block.is_null() || decoded_block.is_null() || routed_block.is_null() {
        dev_err(
            (*(*tascam).card).dev,
            b"Capture decode/routing buffers not allocated!\n" as *const u8,
        );
        return;
    }

    let capture_active_offset = 0;
    while atomic_read((tascam as *const i32).add(capture_active_offset)) != 0 {
        let write_ptr: usize;
        let read_ptr: usize;
        let available_data: usize;
        let can_process: bool;

        // scoped_guard(spinlock_irqsave, &tascam->lock) {
        let capture_ring_buffer_write_ptr_offset = 4;
        let capture_ring_buffer_read_ptr_offset = 3;
        write_ptr = *(tascam as *const usize).add(capture_ring_buffer_write_ptr_offset);
        read_ptr = *(tascam as *const usize).add(capture_ring_buffer_read_ptr_offset);

        available_data = if write_ptr >= read_ptr {
            write_ptr - read_ptr
        } else {
            CAPTURE_RING_BUFFER_SIZE - read_ptr + write_ptr
        };

        can_process = available_data >= RAW_BYTES_PER_DECODE_BLOCK;

        if can_process {
            let bytes_to_end = CAPTURE_RING_BUFFER_SIZE - read_ptr;
            let capture_ring_buffer_offset = 0;
            let capture_ring_buffer = *(tascam as *const *mut u8).add(capture_ring_buffer_offset);

            if bytes_to_end >= RAW_BYTES_PER_DECODE_BLOCK {
                ptr::copy_nonoverlapping(
                    capture_ring_buffer.add(read_ptr),
                    raw_block,
                    RAW_BYTES_PER_DECODE_BLOCK,
                );
            } else {
                ptr::copy_nonoverlapping(
                    capture_ring_buffer.add(read_ptr),
                    raw_block,
                    bytes_to_end,
                );
                ptr::copy_nonoverlapping(
                    capture_ring_buffer,
                    raw_block.add(bytes_to_end),
                    RAW_BYTES_PER_DECODE_BLOCK - bytes_to_end,
                );
            }
            *(tascam as *mut usize).add(capture_ring_buffer_read_ptr_offset) =
                (read_ptr + RAW_BYTES_PER_DECODE_BLOCK) % CAPTURE_RING_BUFFER_SIZE;
        }
        // }

        if !can_process {
            break;
        }

        decode_tascam_capture_block(raw_block, decoded_block);
        process_capture_routing_us144mkii(tascam, decoded_block, routed_block);

        // scoped_guard(spinlock_irqsave, &tascam->lock) {
        if atomic_read((tascam as *const i32).add(capture_active_offset)) != 0 {
            for f in 0..FRAMES_PER_DECODE_BLOCK {
                let driver_capture_pos_offset = 0;
                let driver_capture_pos = *(tascam as *const usize).add(driver_capture_pos_offset);

                let dst_frame_start = (*runtime).dma_area.add(frames_to_bytes(runtime, driver_capture_pos));
                let routed_frame_start = routed_block.add(f * (NUM_CHANNELS as usize));

                for c in 0..(NUM_CHANNELS as usize) {
                    let dst_channel = dst_frame_start.add(c * (BYTES_PER_SAMPLE as usize));
                    let src_channel_s32 = routed_frame_start.add(c);

                    ptr::copy_nonoverlapping(
                        (src_channel_s32 as *const u8).add(1),
                        dst_channel as *mut u8,
                        3,
                    );
                }

                let new_pos = (driver_capture_pos + 1) % (*runtime).buffer_size;
                *(tascam as *mut usize).add(driver_capture_pos_offset) = new_pos;
            }
        }
        // }
    }
}

#[no_mangle]
pub unsafe extern "C" fn capture_urb_complete(urb: *mut urb) {
    let tascam = (*urb).context as *mut tascam_card;

    let status_offset = 0;
    let urb_status = *(urb as *const i32).add(status_offset);

    if urb_status != 0 {
        if urb_status != ENOENT && urb_status != ECONNRESET && urb_status != ESHUTDOWN
            && urb_status != ENODEV && urb_status != EPROTO
        {
            dev_err_ratelimited(
                (*(*tascam).card).dev,
                b"Capture URB failed: %d\n" as *const u8,
                urb_status,
            );
        }
        usb_put_urb(urb);
        return;
    }

    let capture_active_offset = 0;
    if tascam.is_null() || atomic_read((tascam as *const i32).add(capture_active_offset)) == 0 {
        usb_put_urb(urb);
        return;
    }

    let actual_length_offset = 1;
    let actual_length = *(urb as *const usize).add(actual_length_offset);

    if actual_length > 0 {
        // scoped_guard(spinlock_irqsave, &tascam->lock) {
        let capture_ring_buffer_write_ptr_offset = 4;
        let write_ptr = *(tascam as *const usize).add(capture_ring_buffer_write_ptr_offset);
        let bytes_to_end = CAPTURE_RING_BUFFER_SIZE - write_ptr;

        let capture_ring_buffer_offset = 0;
        let capture_ring_buffer = *(tascam as *const *mut u8).add(capture_ring_buffer_offset);

        let transfer_buffer_offset = 2;
        let transfer_buffer = *(urb as *const *mut u8).add(transfer_buffer_offset);

        if actual_length > bytes_to_end {
            ptr::copy_nonoverlapping(transfer_buffer, capture_ring_buffer.add(write_ptr), bytes_to_end);
            ptr::copy_nonoverlapping(
                transfer_buffer.add(bytes_to_end),
                capture_ring_buffer,
                actual_length - bytes_to_end,
            );
        } else {
            ptr::copy_nonoverlapping(
                transfer_buffer,
                capture_ring_buffer.add(write_ptr),
                actual_length,
            );
        }

        *(tascam as *mut usize).add(capture_ring_buffer_write_ptr_offset) =
            (write_ptr + actual_length) % CAPTURE_RING_BUFFER_SIZE;
        // }

        schedule_work(&mut (*tascam).capture_work);
    }

    usb_get_urb(urb);
    usb_anchor_urb(urb, &mut (*tascam).capture_anchor);
    let ret = usb_submit_urb(urb, GFP_ATOMIC);
    if ret < 0 {
        dev_err_ratelimited(
            (*(*tascam).card).dev,
            b"Failed to resubmit capture URB: %d\n" as *const u8,
            ret,
        );
        usb_unanchor_urb(urb);
        usb_put_urb(urb);
        let active_urbs_offset = 0;
        atomic_dec((tascam as *mut i32).add(active_urbs_offset));
        return;
    }

    usb_put_urb(urb);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
