// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025 Šerif Rami <ramiserifpersia@gmail.com>

use crate::us144mkii::{
    TascamCard, SndPcmSubstream, SndPcmRuntime, SndPcmOps, Urb, SndPcmUframes,
    tascam_pcm_hw, tascam_pcm_hw_params, tascam_pcm_hw_free, tascam_pcm_trigger,
    process_playback_routing_us144mkii, snd_pcm_lib_ioctl, snd_pcm_period_elapsed,
    snd_pcm_stop, usb_get_urb, usb_anchor_urb, usb_submit_urb, usb_unanchor_urb,
    usb_put_urb, bytes_to_frames, frames_to_bytes, div_u64, SNDRV_PCM_STATE_XRUN,
    GFP_ATOMIC, FEEDBACK_ACCUMULATOR_SIZE, FEEDBACK_URB_PACKETS, FEEDBACK_PACKET_SIZE,
    BYTES_PER_FRAME, PLAYBACK_URB_PACKETS, NUM_FEEDBACK_URBS, NUM_PLAYBACK_URBS,
    FEEDBACK_SYNC_LOSS_THRESHOLD,
};
use std::ptr;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};

/// tascam_playback_open() - Opens the PCM playback substream.
/// @substream: The ALSA PCM substream to open.
///
/// This function sets the hardware parameters for the playback substream
/// and stores a reference to the substream in the driver's private data.
///
/// Return: 0 on success.
unsafe extern "C" fn tascam_playback_open(substream: *mut SndPcmSubstream) -> i32 {
    let tascam = snd_pcm_substream_chip(substream);

    (*substream).runtime.as_mut().unwrap().hw = tascam_pcm_hw;
    (*tascam).playback_substream = substream;
    atomic_set(&(*tascam).playback_active, 0);

    0
}

/// tascam_playback_close() - Closes the PCM playback substream.
/// @substream: The ALSA PCM substream to close.
///
/// This function clears the reference to the playback substream in the
/// driver's private data.
///
/// Return: 0 on success.
unsafe extern "C" fn tascam_playback_close(substream: *mut SndPcmSubstream) -> i32 {
    let tascam = snd_pcm_substream_chip(substream);

    (*tascam).playback_substream = ptr::null_mut();

    0
}

/// tascam_playback_prepare() - Prepares the PCM playback substream for use.
/// @substream: The ALSA PCM substream to prepare.
///
/// This function initializes playback-related counters and flags, and configures
/// the playback URBs with appropriate packet sizes based on the nominal frame
/// rate.
///
/// Return: 0 on success.
unsafe extern "C" fn tascam_playback_prepare(substream: *mut SndPcmSubstream) -> i32 {
    let tascam = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut i: i32;
    let mut u: i32;
    let nominal_frames_per_packet: usize;
    let nominal_bytes_per_packet: usize;
    let total_bytes_in_urb: usize;

    (*tascam).driver_playback_pos = 0;
    (*tascam).playback_frames_consumed = 0;
    (*tascam).last_period_pos = 0;
    (*tascam).feedback_pattern_in_idx = 0;
    (*tascam).feedback_pattern_out_idx = 0;
    (*tascam).feedback_synced = false;
    (*tascam).feedback_consecutive_errors = 0;
    (*tascam).feedback_urb_skip_count = NUM_FEEDBACK_URBS;

    nominal_frames_per_packet = ((*runtime).rate as usize) / 8000;
    i = 0;
    while i < FEEDBACK_ACCUMULATOR_SIZE as i32 {
        (*tascam).feedback_accumulator_pattern[i as usize] = nominal_frames_per_packet;
        i += 1;
    }

    i = 0;
    while i < NUM_FEEDBACK_URBS as i32 {
        let f_urb = (*tascam).feedback_urbs[i as usize];
        let mut j: i32;

        (*f_urb).number_of_packets = FEEDBACK_URB_PACKETS as i32;
        (*f_urb).transfer_buffer_length =
            (FEEDBACK_URB_PACKETS as usize * FEEDBACK_PACKET_SIZE as usize) as u32;
        j = 0;
        while j < FEEDBACK_URB_PACKETS as i32 {
            (*f_urb).iso_frame_desc[j as usize].offset =
                (j as usize * FEEDBACK_PACKET_SIZE as usize) as u32;
            (*f_urb).iso_frame_desc[j as usize].length = FEEDBACK_PACKET_SIZE as u32;
            j += 1;
        }
        i += 1;
    }

    nominal_bytes_per_packet = nominal_frames_per_packet * BYTES_PER_FRAME as usize;
    total_bytes_in_urb = nominal_bytes_per_packet * PLAYBACK_URB_PACKETS as usize;

    u = 0;
    while u < NUM_PLAYBACK_URBS as i32 {
        let urb = (*tascam).playback_urbs[u as usize];

        ptr::write_bytes(
            (*urb).transfer_buffer as *mut u8,
            0,
            (*tascam).playback_urb_alloc_size,
        );
        (*urb).transfer_buffer_length = total_bytes_in_urb as u32;
        (*urb).number_of_packets = PLAYBACK_URB_PACKETS as i32;
        i = 0;
        while i < PLAYBACK_URB_PACKETS as i32 {
            (*urb).iso_frame_desc[i as usize].offset =
                (i as usize * nominal_bytes_per_packet) as u32;
            (*urb).iso_frame_desc[i as usize].length = nominal_bytes_per_packet as u32;
            i += 1;
        }
        u += 1;
    }

    0
}

/// tascam_playback_pointer() - Returns the current playback pointer position.
/// @substream: The ALSA PCM substream.
///
/// This function returns the current position of the playback pointer within
/// the ALSA ring buffer, in frames.
///
/// Return: The current playback pointer position in frames.
unsafe extern "C" fn tascam_playback_pointer(substream: *mut SndPcmSubstream) -> SndPcmUframes {
    let tascam = snd_pcm_substream_chip(substream);
    let runtime = (*substream).runtime;
    let mut pos: u64;

    if atomic_read(&(*tascam).playback_active) == 0 {
        return 0;
    }

    {
        let _guard = SpinlockIrqsaveGuard::new(&(*tascam).lock);
        pos = (*tascam).playback_frames_consumed;
    }

    if (*runtime).buffer_size == 0 {
        return 0;
    }

    (pos % (*runtime).buffer_size as u64) as SndPcmUframes
}

/// tascam_playback_ops - ALSA PCM operations for playback.
///
/// This structure defines the callback functions for playback stream operations,
/// including open, close, ioctl, hardware parameters, hardware free, prepare,
/// trigger, and pointer.
pub const TASCAM_PLAYBACK_OPS: SndPcmOps = SndPcmOps {
    open: Some(tascam_playback_open),
    close: Some(tascam_playback_close),
    ioctl: Some(snd_pcm_lib_ioctl),
    hw_params: Some(tascam_pcm_hw_params),
    hw_free: Some(tascam_pcm_hw_free),
    prepare: Some(tascam_playback_prepare),
    trigger: Some(tascam_pcm_trigger),
    pointer: Some(tascam_playback_pointer),
};

pub unsafe extern "C" fn playback_urb_complete(urb: *mut Urb) {
    let tascam = (*urb).context as *mut TascamCard;
    let mut substream: *mut SndPcmSubstream;
    let mut runtime: *mut SndPcmRuntime;
    let mut total_bytes_for_urb: usize = 0;
    let offset_frames: SndPcmUframes;
    let frames_to_copy: SndPcmUframes;
    let mut ret: i32;
    let mut i: i32;

    if (*urb).status != 0 {
        if (*urb).status != -libc::ENOENT
            && (*urb).status != -libc::ECONNRESET
            && (*urb).status != -libc::ESHUTDOWN
            && (*urb).status != -libc::ENODEV
        {
            dev_err_ratelimited(
                (*(*tascam).card).dev,
                "Playback URB failed: %d\n",
                (*urb).status,
            );
        }
        goto out;
    }
    if tascam.is_null() || atomic_read(&(*tascam).playback_active) == 0 {
        goto out;
    }

    substream = (*tascam).playback_substream;
    if substream.is_null() || (*substream).runtime.is_null() {
        goto out;
    }
    runtime = (*substream).runtime;

    {
        let _guard = SpinlockIrqsaveGuard::new(&(*tascam).lock);
        i = 0;
        while i < (*urb).number_of_packets {
            let frames_for_packet: u32;
            let bytes_for_packet: usize;

            if (*tascam).feedback_synced {
                frames_for_packet = (*tascam).feedback_accumulator_pattern
                    [(*tascam).feedback_pattern_out_idx as usize] as u32;
                (*tascam).feedback_pattern_out_idx =
                    ((*tascam).feedback_pattern_out_idx + 1) % FEEDBACK_ACCUMULATOR_SIZE;
            } else {
                frames_for_packet = ((*runtime).rate as u32) / 8000;
            }
            bytes_for_packet = (frames_for_packet as usize) * (BYTES_PER_FRAME as usize);

            (*urb).iso_frame_desc[i as usize].offset = total_bytes_for_urb as u32;
            (*urb).iso_frame_desc[i as usize].length = bytes_for_packet as u32;
            total_bytes_for_urb += bytes_for_packet;
            i += 1;
        }
        (*urb).transfer_buffer_length = total_bytes_for_urb as u32;

        offset_frames = (*tascam).driver_playback_pos;
        frames_to_copy = bytes_to_frames(runtime, total_bytes_for_urb);
        (*tascam).driver_playback_pos =
            (offset_frames + frames_to_copy) % (*runtime).buffer_size;
    }

    if total_bytes_for_urb > 0 {
        let dst_buf = (*urb).transfer_buffer as *mut u8;

        if (offset_frames as usize) + (frames_to_copy as usize) > (*runtime).buffer_size as usize {
            let first_chunk_bytes = frames_to_bytes(
                runtime,
                ((*runtime).buffer_size as usize) - (offset_frames as usize),
            );
            let second_chunk_bytes = total_bytes_for_urb - first_chunk_bytes;

            ptr::copy_nonoverlapping(
                ((*runtime).dma_area as *mut u8)
                    .add(frames_to_bytes(runtime, offset_frames as usize)),
                dst_buf,
                first_chunk_bytes,
            );
            ptr::copy_nonoverlapping(
                (*runtime).dma_area as *mut u8,
                dst_buf.add(first_chunk_bytes),
                second_chunk_bytes,
            );
        } else {
            ptr::copy_nonoverlapping(
                ((*runtime).dma_area as *mut u8)
                    .add(frames_to_bytes(runtime, offset_frames as usize)),
                dst_buf,
                total_bytes_for_urb,
            );
        }

        process_playback_routing_us144mkii(tascam, dst_buf, dst_buf, frames_to_copy);
    }

    (*urb).dev = (*tascam).dev;
    usb_get_urb(urb);
    usb_anchor_urb(urb, &(*tascam).playback_anchor);
    ret = usb_submit_urb(urb, GFP_ATOMIC);
    if ret < 0 {
        dev_err_ratelimited(
            (*(*tascam).card).dev,
            "Failed to resubmit playback URB: %d\n",
            ret,
        );
        usb_unanchor_urb(urb);
        usb_put_urb(urb);
        atomic_dec(&(*tascam).active_urbs);
    }
    out: {
        usb_put_urb(urb);
    }
}

pub unsafe extern "C" fn feedback_urb_complete(urb: *mut Urb) {
    let tascam = (*urb).context as *mut TascamCard;
    let mut playback_ss: *mut SndPcmSubstream;
    let mut capture_ss: *mut SndPcmSubstream;
    let mut playback_rt: *mut SndPcmRuntime;
    let mut capture_rt: *mut SndPcmRuntime;
    let mut total_frames_in_urb: u64 = 0;
    let mut ret: i32;
    let mut p: i32;
    let old_in_idx: u32;
    let new_in_idx: u32;
    let mut playback_period_elapsed: bool = false;
    let mut capture_period_elapsed: bool = false;

    if (*urb).status != 0 {
        if (*urb).status != -libc::ENOENT
            && (*urb).status != -libc::ECONNRESET
            && (*urb).status != -libc::ESHUTDOWN
            && (*urb).status != -libc::ENODEV
        {
            dev_err_ratelimited(
                (*(*tascam).card).dev,
                "Feedback URB failed: %d\n",
                (*urb).status,
            );
            atomic_dec(&(*tascam).active_urbs);
        }
        goto out;
    }
    if tascam.is_null() || atomic_read(&(*tascam).playback_active) == 0 {
        goto out;
    }

    playback_ss = (*tascam).playback_substream;
    if playback_ss.is_null() || (*playback_ss).runtime.is_null() {
        goto out;
    }
    playback_rt = (*playback_ss).runtime;

    capture_ss = (*tascam).capture_substream;
    capture_rt = if !capture_ss.is_null() {
        (*capture_ss).runtime
    } else {
        ptr::null_mut()
    };

    {
        let _guard = SpinlockIrqsaveGuard::new(&(*tascam).lock);

        if (*tascam).feedback_urb_skip_count > 0 {
            (*tascam).feedback_urb_skip_count -= 1;
            // This replaces the C 'break' which exits from the spinlock guard scope
        } else {
            old_in_idx = (*tascam).feedback_pattern_in_idx;

            p = 0;
            while p < (*urb).number_of_packets {
                let mut feedback_value: u8 = 0;
                let pattern: *const u32;
                let packet_ok: bool = (*urb).iso_frame_desc[p as usize].status == 0
                    && (*urb).iso_frame_desc[p as usize].actual_length >= 1;

                if packet_ok {
                    feedback_value = *(((*urb).transfer_buffer as *const u8)
                        .add((*urb).iso_frame_desc[p as usize].offset as usize));
                }

                if packet_ok {
                    let delta: i32 = (feedback_value as i32)
                        - ((*tascam).fpo.base_feedback_value as i32)
                        + (*tascam).fpo.feedback_offset;
                    let pattern_idx: i32;

                    if delta < 0 {
                        pattern_idx = 0;
                    } else if delta >= 5 {
                        pattern_idx = 4;
                    } else {
                        pattern_idx = delta;
                    }

                    pattern = (*tascam).fpo.full_frame_patterns[pattern_idx as usize];
                    (*tascam).feedback_consecutive_errors = 0;
                    let mut i: i32 = 0;

                    while i < 8 {
                        let in_idx: u32 =
                            (((*tascam).feedback_pattern_in_idx as i32 + i) % FEEDBACK_ACCUMULATOR_SIZE as i32)
                                as u32;

                        (*tascam).feedback_accumulator_pattern[in_idx as usize] =
                            *pattern.add(i as usize);
                        total_frames_in_urb += *pattern.add(i as usize) as u64;
                        i += 1;
                    }
                } else {
                    let nominal_frames: u32 = ((*playback_rt).rate as u32) / 8000;
                    let mut i: i32 = 0;

                    if (*tascam).feedback_synced {
                        (*tascam).feedback_consecutive_errors += 1;
                        if (*tascam).feedback_consecutive_errors > FEEDBACK_SYNC_LOSS_THRESHOLD {
                            dev_err(
                                (*(*tascam).card).dev,
                                "Fatal: Feedback sync lost. Stopping stream.\n",
                            );
                            schedule_work(&(*tascam).stop_pcm_work);
                            (*tascam).feedback_synced = false;
                            break;
                        }
                    }
                    while i < 8 {
                        let in_idx: u32 =
                            (((*tascam).feedback_pattern_in_idx as i32 + i) % FEEDBACK_ACCUMULATOR_SIZE as i32)
                                as u32;

                        (*tascam).feedback_accumulator_pattern[in_idx as usize] =
                            nominal_frames as usize;
                        total_frames_in_urb += nominal_frames as u64;
                        i += 1;
                    }
                }
                (*tascam).feedback_pattern_in_idx =
                    (((*tascam).feedback_pattern_in_idx as i32 + 8) % FEEDBACK_ACCUMULATOR_SIZE as i32)
                        as u32;
                p += 1;
            }

            new_in_idx = (*tascam).feedback_pattern_in_idx;

            if !(*tascam).feedback_synced {
                let out_idx: u32 = (*tascam).feedback_pattern_out_idx;
                let is_ahead: bool = ((new_in_idx as i32 - out_idx as i32) % FEEDBACK_ACCUMULATOR_SIZE as i32) as u32
                    < (FEEDBACK_ACCUMULATOR_SIZE / 2);
                let was_behind: bool = ((old_in_idx as i32 - out_idx as i32) % FEEDBACK_ACCUMULATOR_SIZE as i32) as u32
                    >= (FEEDBACK_ACCUMULATOR_SIZE / 2);

                if is_ahead && was_behind {
                    dev_dbg(
                        (*(*tascam).card).dev,
                        "Sync Acquired! (in: %u, out: %u)\n",
                        new_in_idx,
                        out_idx,
                    );
                    (*tascam).feedback_synced = true;
                    (*tascam).feedback_consecutive_errors = 0;
                }
            }

            if total_frames_in_urb > 0 {
                (*tascam).playback_frames_consumed += total_frames_in_urb;
                if atomic_read(&(*tascam).capture_active) != 0 {
                    (*tascam).capture_frames_processed += total_frames_in_urb;
                }
            }

            if (*playback_rt).period_size > 0 {
                let current_period: u64 = div_u64((*tascam).playback_frames_consumed, (*playback_rt).period_size);

                if current_period > (*tascam).last_period_pos {
                    (*tascam).last_period_pos = current_period;
                    playback_period_elapsed = true;
                }
            }

            if atomic_read(&(*tascam).capture_active) != 0
                && !capture_rt.is_null()
                && (*capture_rt).period_size > 0
            {
                let current_capture_period: u64 = div_u64((*tascam).capture_frames_processed, (*capture_rt).period_size);

                if current_capture_period > (*tascam).last_capture_period_pos {
                    (*tascam).last_capture_period_pos = current_capture_period;
                    capture_period_elapsed = true;
                }
            }
        }
    }

    if playback_period_elapsed {
        snd_pcm_period_elapsed(playback_ss);
    }
    if capture_period_elapsed {
        snd_pcm_period_elapsed(capture_ss);
    }

    (*urb).dev = (*tascam).dev;
    usb_get_urb(urb);
    usb_anchor_urb(urb, &(*tascam).feedback_anchor);
    ret = usb_submit_urb(urb, GFP_ATOMIC);
    if ret < 0 {
        dev_err_ratelimited(
            (*(*tascam).card).dev,
            "Failed to resubmit feedback URB: %d\n",
            ret,
        );
        usb_unanchor_urb(urb);
        usb_put_urb(urb);
    }
    out: {
        usb_put_urb(urb);
    }
}

pub unsafe extern "C" fn tascam_stop_pcm_work_handler(work: *mut crate::WorkStruct) {
    let tascam = container_of(work, TascamCard, stop_pcm_work) as *mut TascamCard;

    if !(*tascam).playback_substream.is_null() {
        snd_pcm_stop((*tascam).playback_substream, SNDRV_PCM_STATE_XRUN);
    }
    if !(*tascam).capture_substream.is_null() {
        snd_pcm_stop((*tascam).capture_substream, SNDRV_PCM_STATE_XRUN);
    }
}

// Helper functions and macros for Rust

unsafe fn snd_pcm_substream_chip(substream: *mut SndPcmSubstream) -> *mut TascamCard {
    // This is a kernel macro that gets the card from substream
    // Assuming it's defined in the external module, otherwise declare it here
    (*substream).private_data as *mut TascamCard
}

fn atomic_set(atomic: *const i32, val: i32) {
    unsafe {
        *(atomic as *mut i32) = val;
    }
}

fn atomic_read(atomic: *const i32) -> i32 {
    unsafe { *(atomic as *const i32) }
}

fn atomic_dec(atomic: *const i32) {
    unsafe {
        *(atomic as *mut i32) -= 1;
    }
}

struct SpinlockIrqsaveGuard {
    // Placeholder for spinlock guard - represents scoped_guard(spinlock_irqsave, ...)
}

impl SpinlockIrqsaveGuard {
    fn new(_lock: *const i32) -> Self {
        // In actual implementation, this would acquire the spinlock
        SpinlockIrqsaveGuard
    }
}

impl Drop for SpinlockIrqsaveGuard {
    fn drop(&mut self) {
        // In actual implementation, this would release the spinlock
    }
}

unsafe fn dev_err_ratelimited(dev: *const libc::c_void, fmt: *const i8, status: i32) {
    // Placeholder for dev_err_ratelimited kernel function
    // In actual use, this would call the kernel logging function
}

unsafe fn dev_err(dev: *const libc::c_void, fmt: *const i8) {
    // Placeholder for dev_err kernel function
    // In actual use, this would call the kernel logging function
}

unsafe fn dev_dbg(dev: *const libc::c_void, fmt: *const i8, new_in_idx: u32, out_idx: u32) {
    // Placeholder for dev_dbg kernel function
    // In actual use, this would call the kernel logging function
}

unsafe fn schedule_work(work: *const crate::WorkStruct) {
    // Placeholder for schedule_work kernel function
    // In actual use, this would schedule the work on a queue
}

unsafe fn container_of<T>(ptr: *mut crate::WorkStruct, _ty: std::marker::PhantomData<T>, _field: &str) -> *mut T {
    // Generic container_of implementation
    // This is a simplification; actual implementation would calculate offset
    ptr as *mut T
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
