// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025 Šerif Rami <ramiserifpersia@gmail.com>

// Depends on: us144mkii.h and associated kernel types/macros

/// fpo_init_pattern() - Generates a packet distribution pattern.
/// @size: The number of elements in the pattern array (e.g., 8).
/// @pattern_array: Pointer to the array to be populated.
/// @initial_value: The base value to initialize each element with.
/// @target_sum: The desired sum of all elements in the final array.
///
/// This function initializes an array with a base value and then iteratively
/// adjusts the elements to match a target sum, distributing the difference
/// as evenly as possible.
unsafe fn fpo_init_pattern(
    size: u32,
    pattern_array: *mut u32,
    initial_value: u32,
    target_sum: i32,
) {
    if size == 0 {
        return;
    }

    for i in 0..(size as usize) {
        *pattern_array.add(i) = initial_value;
    }

    let diff = target_sum - (size as i32 * initial_value as i32);
    for i in 0..(diff.abs() as usize) {
        if diff > 0 {
            *pattern_array.add(i) = (*pattern_array.add(i)).wrapping_add(1);
        } else {
            *pattern_array.add(i) = (*pattern_array.add(i)).wrapping_sub(1);
        }
    }
}

#[repr(C)]
pub struct SndPcmHardware {
    pub info: u32,
    pub formats: u64,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub channels_min: u32,
    pub channels_max: u32,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
}

// TODO: Define NUM_CHANNELS, BYTES_PER_FRAME, and SNDRV_PCM_INFO_* constants from us144mkii.h
pub const TASCAM_PCM_HW: SndPcmHardware = SndPcmHardware {
    info: (0x00000001 | 0x00000002 | 0x00000004 | 0x00000008 | 0x00000010 | 0x00000020),
    formats: 0x0000000040000000,
    rates: (0x00000001 | 0x00000002 | 0x00000004 | 0x00000008),
    rate_min: 44100,
    rate_max: 96000,
    channels_min: 4,
    channels_max: 4,
    buffer_bytes_max: 1024 * 1024,
    period_bytes_min: 48 * 3,
    period_bytes_max: 1024 * 3,
    periods_min: 2,
    periods_max: 1024,
};

// Forward declarations for external types
#[repr(C)]
pub struct TascamCard {
    pub line_out_source: i32,
    pub digital_out_source: i32,
    pub capture_12_source: i32,
    pub capture_34_source: i32,
    pub dev: *mut UsbDevice,
    pub current_rate: i32,
    pub card: *mut SndCard,
    pub lock: SpinlockT,
    pub playback_active: AtomicT,
    pub capture_active: AtomicT,
    pub active_urbs: AtomicT,
    pub feedback_urbs: [*mut UrbT; 4],
    pub playback_urbs: [*mut UrbT; 4],
    pub capture_urbs: [*mut UrbT; 8],
    pub feedback_anchor: UsbAnchorT,
    pub playback_anchor: UsbAnchorT,
    pub capture_anchor: UsbAnchorT,
    pub stop_work: WorkStructT,
    pub fpo: FpoState,
}

#[repr(C)]
pub struct FpoState {
    pub sample_rate_khz: u32,
    pub base_feedback_value: u32,
    pub feedback_offset: u32,
    pub current_index: u32,
    pub previous_index: u32,
    pub sync_locked: bool,
    pub full_frame_patterns: [[u32; 8]; 5],
}

pub struct UsbDevice;
pub struct SndCard;
pub struct SpinlockT;
pub struct AtomicT;
pub struct UrbT;
pub struct UsbAnchorT;
pub struct WorkStructT;

pub fn process_playback_routing_us144mkii(
    tascam: *mut TascamCard,
    src_buffer: *const u8,
    dst_buffer: *mut u8,
    frames: usize,
) {
    unsafe {
        for f in 0..frames {
            let src_12 = src_buffer.add(f * 3);
            let src_34 = src_12.add(2 * 1);
            let dst_line = dst_buffer.add(f * 3);
            let dst_digital = dst_line.add(2 * 1);

            if (*tascam).line_out_source == 0 {
                core::ptr::copy_nonoverlapping(src_12, dst_line, 2 * 1);
            } else {
                core::ptr::copy_nonoverlapping(src_34, dst_line, 2 * 1);
            }

            if (*tascam).digital_out_source == 0 {
                core::ptr::copy_nonoverlapping(src_12, dst_digital, 2 * 1);
            } else {
                core::ptr::copy_nonoverlapping(src_34, dst_digital, 2 * 1);
            }
        }
    }
}

pub fn process_capture_routing_us144mkii(
    tascam: *mut TascamCard,
    decoded_block: *const i32,
    routed_block: *mut i32,
) {
    unsafe {
        for f in 0..64 {
            let src_frame = decoded_block.add(f * 4);
            let dst_frame = routed_block.add(f * 4);

            if (*tascam).capture_12_source == 0 {
                *dst_frame.add(0) = *src_frame.add(0);
                *dst_frame.add(1) = *src_frame.add(1);
            } else {
                *dst_frame.add(0) = *src_frame.add(2);
                *dst_frame.add(1) = *src_frame.add(3);
            }

            if (*tascam).capture_34_source == 0 {
                *dst_frame.add(2) = *src_frame.add(0);
                *dst_frame.add(3) = *src_frame.add(1);
            } else {
                *dst_frame.add(2) = *src_frame.add(2);
                *dst_frame.add(3) = *src_frame.add(3);
            }
        }
    }
}

// TODO: Define REG_ADDR_RATE_* constants from us144mkii.h
pub fn us144mkii_configure_device_for_rate(tascam: *mut TascamCard, rate: i32) -> i32 {
    unsafe {
        let dev = (*tascam).dev;

        let payload_44100 = [0x44u8, 0xac, 0x00];
        let payload_48000 = [0x80u8, 0xbb, 0x00];
        let payload_88200 = [0x88u8, 0x58, 0x01];
        let payload_96000 = [0x00u8, 0x77, 0x01];

        let (current_payload_src, rate_vendor_wvalue) = match rate {
            44100 => (&payload_44100[..], 0u16),
            48000 => (&payload_48000[..], 1u16),
            88200 => (&payload_88200[..], 2u16),
            96000 => (&payload_96000[..], 3u16),
            _ => {
                return -22;
            }
        };

        let mut rate_payload_buf = [0u8; 3];
        core::ptr::copy_nonoverlapping(current_payload_src.as_ptr(), rate_payload_buf.as_mut_ptr(), 3);

        let mut err = usb_control_msg(
            dev,
            usb_sndctrlpipe(dev, 0),
            0x01,
            0xc0,
            0x01,
            0x0000,
            core::ptr::null_mut(),
            0,
            1000,
        );
        if err < 0 {
            return err;
        }

        err = usb_control_msg(
            dev,
            usb_sndctrlpipe(dev, 0),
            0x01,
            0xa1,
            0x0100,
            0x0200,
            rate_payload_buf.as_mut_ptr() as *mut _,
            3,
            1000,
        );
        if err < 0 {
            return err;
        }

        err = usb_control_msg(
            dev,
            usb_sndctrlpipe(dev, 0),
            0x01,
            0xa1,
            0x0100,
            0x0300,
            rate_payload_buf.as_mut_ptr() as *mut _,
            3,
            1000,
        );
        if err < 0 {
            return err;
        }

        err = usb_control_msg(
            dev,
            usb_sndctrlpipe(dev, 0),
            0x04,
            0xc0,
            0x0d,
            0x0001,
            core::ptr::null_mut(),
            0,
            1000,
        );
        if err < 0 {
            return err;
        }

        err = usb_control_msg(
            dev,
            usb_sndctrlpipe(dev, 0),
            0x04,
            0xc0,
            0x0e,
            0x0001,
            core::ptr::null_mut(),
            0,
            1000,
        );
        if err < 0 {
            return err;
        }

        err = usb_control_msg(
            dev,
            usb_sndctrlpipe(dev, 0),
            0x04,
            0xc0,
            0x0f,
            0x0001,
            core::ptr::null_mut(),
            0,
            1000,
        );
        if err < 0 {
            return err;
        }

        err = usb_control_msg(
            dev,
            usb_sndctrlpipe(dev, 0),
            0x04,
            0xc0,
            rate_vendor_wvalue,
            0x0001,
            core::ptr::null_mut(),
            0,
            1000,
        );
        if err < 0 {
            return err;
        }

        err = usb_control_msg(
            dev,
            usb_sndctrlpipe(dev, 0),
            0x04,
            0xc0,
            0x11,
            0x0001,
            core::ptr::null_mut(),
            0,
            1000,
        );
        if err < 0 {
            return err;
        }

        err = usb_control_msg(
            dev,
            usb_sndctrlpipe(dev, 0),
            0x01,
            0xc0,
            0x02,
            0x0000,
            core::ptr::null_mut(),
            0,
            1000,
        );
        if err < 0 {
            return err;
        }

        0
    }
}

// TODO: Define SndPcmSubstream, SndPcmHwParams types from kernel
pub struct SndPcmSubstream {
    pub stream: i32,
}

pub struct SndPcmHwParams {
    pub intervals: [u32; 32],
}

pub fn tascam_pcm_hw_params(
    substream: *mut SndPcmSubstream,
    params: *mut SndPcmHwParams,
) -> i32 {
    unsafe {
        let tascam = snd_pcm_substream_chip(substream);
        let rate = params_rate(params);

        if (*substream).stream == 0 {
            (*tascam).fpo.sample_rate_khz = rate as u32 / 1000;
            (*tascam).fpo.base_feedback_value = (*tascam).fpo.sample_rate_khz;
            (*tascam).fpo.feedback_offset = 2;
            (*tascam).fpo.current_index = 0;
            (*tascam).fpo.previous_index = 0;
            (*tascam).fpo.sync_locked = false;

            let initial_value = (*tascam).fpo.sample_rate_khz / 8;

            for i in 0..5 {
                let target_sum = (*tascam).fpo.sample_rate_khz as i32
                    - (*tascam).fpo.feedback_offset as i32
                    + i as i32;
                fpo_init_pattern(
                    8,
                    (*tascam).fpo.full_frame_patterns[i as usize].as_mut_ptr(),
                    initial_value,
                    target_sum,
                );
            }
        }

        if (*tascam).current_rate != rate {
            let err = us144mkii_configure_device_for_rate(tascam, rate);
            if err < 0 {
                (*tascam).current_rate = 0;
                return err;
            }
            (*tascam).current_rate = rate;
        }

        0
    }
}

pub fn tascam_pcm_hw_free(_substream: *mut SndPcmSubstream) -> i32 {
    0
}

pub fn tascam_pcm_trigger(substream: *mut SndPcmSubstream, cmd: i32) -> i32 {
    unsafe {
        let tascam = snd_pcm_substream_chip(substream);
        let mut err = 0;
        let mut do_start = false;
        let mut do_stop = false;

        spinlock_irqsave_lock(&(*tascam).lock);

        match cmd {
            0x0 | 0x2 => {
                if atomic_read(&(*tascam).playback_active) == 0 {
                    atomic_set(&(*tascam).playback_active, 1);
                    atomic_set(&(*tascam).capture_active, 1);
                    do_start = true;
                }
            }
            0x1 | 0x3 | 0x4 => {
                if atomic_read(&(*tascam).playback_active) != 0 {
                    atomic_set(&(*tascam).playback_active, 0);
                    atomic_set(&(*tascam).capture_active, 0);
                    do_stop = true;
                }
            }
            _ => {
                err = -22;
            }
        }

        spinlock_irqsave_unlock(&(*tascam).lock);

        if do_start {
            if atomic_read(&(*tascam).active_urbs) > 0 {
                return -11;
            }

            for i in 0..4 {
                usb_get_urb((*tascam).feedback_urbs[i]);
                usb_anchor_urb(
                    (*tascam).feedback_urbs[i],
                    &mut (*tascam).feedback_anchor,
                );
                err = usb_submit_urb((*tascam).feedback_urbs[i], 0x120);
                if err < 0 {
                    usb_unanchor_urb((*tascam).feedback_urbs[i]);
                    usb_put_urb((*tascam).feedback_urbs[i]);
                    atomic_dec(&(*tascam).active_urbs);
                    do_stop = true;
                    break;
                }
                atomic_inc(&(*tascam).active_urbs);
            }

            if !do_stop {
                for i in 0..4 {
                    usb_get_urb((*tascam).playback_urbs[i]);
                    usb_anchor_urb(
                        (*tascam).playback_urbs[i],
                        &mut (*tascam).playback_anchor,
                    );
                    err = usb_submit_urb((*tascam).playback_urbs[i], 0x120);
                    if err < 0 {
                        usb_unanchor_urb((*tascam).playback_urbs[i]);
                        usb_put_urb((*tascam).playback_urbs[i]);
                        atomic_dec(&(*tascam).active_urbs);
                        do_stop = true;
                        break;
                    }
                    atomic_inc(&(*tascam).active_urbs);
                }
            }

            if !do_stop {
                for i in 0..8 {
                    usb_get_urb((*tascam).capture_urbs[i]);
                    usb_anchor_urb(
                        (*tascam).capture_urbs[i],
                        &mut (*tascam).capture_anchor,
                    );
                    err = usb_submit_urb((*tascam).capture_urbs[i], 0x120);
                    if err < 0 {
                        usb_unanchor_urb((*tascam).capture_urbs[i]);
                        usb_put_urb((*tascam).capture_urbs[i]);
                        atomic_dec(&(*tascam).active_urbs);
                        do_stop = true;
                        break;
                    }
                    atomic_inc(&(*tascam).active_urbs);
                }
            }

            if !do_stop {
                return 0;
            }
        }

        if do_stop {
            schedule_work(&(*tascam).stop_work);
        }

        err
    }
}

pub struct SndPcm {
    pub private_data: *mut TascamCard,
}

pub fn tascam_init_pcm(pcm: *mut SndPcm) -> i32 {
    unsafe {
        let tascam = (*pcm).private_data;

        snd_pcm_set_ops(pcm, 0, &TASCAM_PLAYBACK_OPS);
        snd_pcm_set_ops(pcm, 1, &TASCAM_CAPTURE_OPS);

        snd_pcm_set_managed_buffer_all(
            pcm,
            0x02,
            &(*(*tascam).dev).dev.parent,
            64 * 1024,
            TASCAM_PCM_HW.buffer_bytes_max,
        );

        0
    }
}

// Stub function declarations for kernel USB/ALSA functions
// TODO: These are external dependencies from the Linux kernel
pub unsafe fn snd_pcm_substream_chip(substream: *mut SndPcmSubstream) -> *mut TascamCard {
    core::ptr::null_mut()
}

pub unsafe fn params_rate(params: *mut SndPcmHwParams) -> i32 {
    0
}

pub unsafe fn usb_sndctrlpipe(dev: *mut UsbDevice, ep: u32) -> u32 {
    0
}

pub unsafe fn usb_control_msg(
    dev: *mut UsbDevice,
    pipe: u32,
    request: u32,
    requesttype: u32,
    value: u16,
    index: u16,
    data: *mut u8,
    size: u32,
    timeout: u32,
) -> i32 {
    0
}

pub unsafe fn usb_get_urb(urb: *mut UrbT) {}

pub unsafe fn usb_anchor_urb(urb: *mut UrbT, anchor: *mut UsbAnchorT) {}

pub unsafe fn usb_submit_urb(urb: *mut UrbT, mem_flags: u32) -> i32 {
    0
}

pub unsafe fn usb_unanchor_urb(urb: *mut UrbT) {}

pub unsafe fn usb_put_urb(urb: *mut UrbT) {}

pub unsafe fn spinlock_irqsave_lock(lock: *mut SpinlockT) {}

pub unsafe fn spinlock_irqsave_unlock(lock: *mut SpinlockT) {}

pub unsafe fn atomic_read(v: *const AtomicT) -> i32 {
    0
}

pub unsafe fn atomic_set(v: *mut AtomicT, i: i32) {}

pub unsafe fn atomic_inc(v: *mut AtomicT) {}

pub unsafe fn atomic_dec(v: *mut AtomicT) {}

pub unsafe fn schedule_work(work: *mut WorkStructT) {}

pub unsafe fn snd_pcm_set_ops(
    pcm: *mut SndPcm,
    stream: i32,
    ops: *const (),
) {}

pub unsafe fn snd_pcm_set_managed_buffer_all(
    pcm: *mut SndPcm,
    type_: i32,
    dev: *const (),
    prealloc_size: usize,
    max_size: usize,
) {}

pub const TASCAM_PLAYBACK_OPS: () = ();
pub const TASCAM_CAPTURE_OPS: () = ();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
