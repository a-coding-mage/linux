// SPDX-License-Identifier: GPL-2.0-only
/*
 * amdtp-tascam.c - a part of driver for TASCAM FireWire series
 *
 * Copyright (c) 2015 Takashi Sakamoto
 */

// Dependencies from the original C includes:
//   #include <sound/pcm.h>
//   #include "tascam.h"

use core::ffi::{c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const AMDTP_FMT_TSCM_TX: c_uint = 0x1e;
const AMDTP_FMT_TSCM_RX: c_uint = 0x3e;

#[repr(C)]
struct amdtp_tscm {
    pcm_channels: c_uint,
}

type u32 = c_uint;
type __be32 = u32;
type bool_ = bool;

#[repr(C)]
struct amdtp_stream {
    protocol: *mut c_void,
    direction: amdtp_stream_direction,
    pcm_buffer_pointer: c_uint,
    data_block_quadlets: c_uint,
    ctx_data: amdtp_stream_ctx_data,
}

#[repr(C)]
union amdtp_stream_ctx_data {
    rx: amdtp_stream_rx_ctx_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct amdtp_stream_rx_ctx_data {
    fdf: c_uint,
}

type amdtp_stream_direction = c_uint;

#[repr(C)]
struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
struct snd_pcm_runtime {
    buffer_size: c_uint,
    dma_area: *mut c_void,
}

#[repr(C)]
struct pkt_desc {
    ctx_payload: *mut __be32,
    data_blocks: c_uint,
}

#[repr(C)]
struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_tscm {
    tx_stream: amdtp_stream,
    hwdep: *mut snd_tscm_hwdep,
    state: [__be32; SNDRV_FIREWIRE_TASCAM_STATE_COUNT],
    queue: [snd_firewire_tascam_change; SND_TSCM_QUEUE_COUNT],
    push_pos: c_uint,
    lock: spinlock_t,
    hwdep_wait: wait_queue_head_t,
}

#[repr(C)]
struct snd_tscm_hwdep {
    used: bool_,
}

#[repr(C)]
struct snd_firewire_tascam_change {
    index: c_uint,
    before: __be32,
    after: __be32,
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
struct wait_queue_head_t {
    _private: [u8; 0],
}

type amdtp_stream_process_ctx_payloads_t = Option<
    unsafe extern "C" fn(
        *mut amdtp_stream,
        *const pkt_desc,
        c_uint,
        *mut snd_pcm_substream,
    ),
>;

extern "C" {
    static AMDTP_IN_STREAM: amdtp_stream_direction;
    static AMDTP_OUT_STREAM: amdtp_stream_direction;
    static CIP_NONBLOCKING: c_uint;
    static CIP_SKIP_DBC_ZERO_CHECK: c_uint;
    static CIP_UNAWARE_SYT: c_uint;
    static SNDRV_FIREWIRE_TASCAM_STATE_COUNT: usize;
    static SND_TSCM_QUEUE_COUNT: usize;
    static EBUSY: c_int;

    fn amdtp_stream_running(s: *mut amdtp_stream) -> bool_;
    fn amdtp_stream_set_parameters(
        s: *mut amdtp_stream,
        rate: c_uint,
        data_channels: c_uint,
        pcm_channels: c_uint,
    ) -> c_int;
    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: c_uint) -> usize;
    fn cpu_to_be32(value: u32) -> __be32;
    fn be32_to_cpu(value: __be32) -> u32;
    fn snd_pcm_hw_constraint_msbits(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        width: c_uint,
        msbits: c_uint,
    ) -> c_int;
    fn amdtp_stream_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> c_int;
    fn amdtp_stream_next_packet_desc(
        s: *mut amdtp_stream,
        desc: *const pkt_desc,
    ) -> *const pkt_desc;
    fn amdtp_stream_init(
        s: *mut amdtp_stream,
        unit: *mut fw_unit,
        dir: amdtp_stream_direction,
        flags: c_uint,
        fmt: c_uint,
        process_ctx_payloads: amdtp_stream_process_ctx_payloads_t,
        protocol_size: usize,
    ) -> c_int;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn wake_up(wait: *mut wait_queue_head_t);
}

type c_ulong = core::ffi::c_ulong;

unsafe fn READ_ONCE_bool(p: *const bool_) -> bool_ {
    ptr::read_volatile(p)
}

unsafe fn container_of_tx_stream(s: *mut amdtp_stream) -> *mut snd_tscm {
    s as *mut snd_tscm
}

#[no_mangle]
pub unsafe extern "C" fn amdtp_tscm_set_parameters(
    s: *mut amdtp_stream,
    rate: c_uint,
) -> c_int {
    let p = (*s).protocol as *mut amdtp_tscm;
    let mut data_channels: c_uint;

    if amdtp_stream_running(s) {
        return -EBUSY;
    }

    data_channels = (*p).pcm_channels;

    /* Packets in in-stream have extra 2 data channels. */
    if (*s).direction == AMDTP_IN_STREAM {
        data_channels = data_channels.wrapping_add(2);
    }

    amdtp_stream_set_parameters(s, rate, data_channels, 1)
}

unsafe extern "C" fn write_pcm_s32(
    s: *mut amdtp_stream,
    pcm: *mut snd_pcm_substream,
    mut buffer: *mut __be32,
    frames: c_uint,
    pcm_frames: c_uint,
) {
    let p = (*s).protocol as *mut amdtp_tscm;
    let channels = (*p).pcm_channels;
    let runtime = (*pcm).runtime;
    let mut pcm_buffer_pointer: c_uint;
    let mut remaining_frames: c_int;
    let mut src: *const u32;
    let mut i: c_int;
    let mut c: c_int;

    pcm_buffer_pointer = (*s).pcm_buffer_pointer.wrapping_add(pcm_frames);
    pcm_buffer_pointer %= (*runtime).buffer_size;

    src = ((*runtime).dma_area as *mut u8).add(frames_to_bytes(runtime, pcm_buffer_pointer))
        as *const u32;
    remaining_frames = ((*runtime).buffer_size - pcm_buffer_pointer) as c_int;

    i = 0;
    while i < frames as c_int {
        c = 0;
        while c < channels as c_int {
            *buffer.add(c as usize) = cpu_to_be32(*src);
            src = src.add(1);
            c += 1;
        }
        buffer = buffer.add((*s).data_block_quadlets as usize);
        remaining_frames -= 1;
        if remaining_frames == 0 {
            src = (*runtime).dma_area as *const u32;
        }
        i += 1;
    }
}

unsafe extern "C" fn read_pcm_s32(
    s: *mut amdtp_stream,
    pcm: *mut snd_pcm_substream,
    mut buffer: *mut __be32,
    frames: c_uint,
    pcm_frames: c_uint,
) {
    let p = (*s).protocol as *mut amdtp_tscm;
    let channels = (*p).pcm_channels;
    let runtime = (*pcm).runtime;
    let mut pcm_buffer_pointer: c_uint;
    let mut remaining_frames: c_int;
    let mut dst: *mut u32;
    let mut i: c_int;
    let mut c: c_int;

    pcm_buffer_pointer = (*s).pcm_buffer_pointer.wrapping_add(pcm_frames);
    pcm_buffer_pointer %= (*runtime).buffer_size;

    dst = ((*runtime).dma_area as *mut u8).add(frames_to_bytes(runtime, pcm_buffer_pointer))
        as *mut u32;
    remaining_frames = ((*runtime).buffer_size - pcm_buffer_pointer) as c_int;

    /* The first data channel is for event counter. */
    buffer = buffer.add(1);

    i = 0;
    while i < frames as c_int {
        c = 0;
        while c < channels as c_int {
            *dst = be32_to_cpu(*buffer.add(c as usize));
            dst = dst.add(1);
            c += 1;
        }
        buffer = buffer.add((*s).data_block_quadlets as usize);
        remaining_frames -= 1;
        if remaining_frames == 0 {
            dst = (*runtime).dma_area as *mut u32;
        }
        i += 1;
    }
}

unsafe extern "C" fn write_pcm_silence(
    s: *mut amdtp_stream,
    mut buffer: *mut __be32,
    data_blocks: c_uint,
) {
    let p = (*s).protocol as *mut amdtp_tscm;
    let channels: c_uint;
    let mut i: c_uint;
    let mut c: c_uint;

    channels = (*p).pcm_channels;

    i = 0;
    while i < data_blocks {
        c = 0;
        while c < channels {
            *buffer.add(c as usize) = 0x00000000;
            c += 1;
        }
        buffer = buffer.add((*s).data_block_quadlets as usize);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn amdtp_tscm_add_pcm_hw_constraints(
    s: *mut amdtp_stream,
    runtime: *mut snd_pcm_runtime,
) -> c_int {
    let mut err: c_int;

    /*
     * Our implementation allows this protocol to deliver 24 bit sample in
     * 32bit data channel.
     */
    err = snd_pcm_hw_constraint_msbits(runtime, 0, 32, 24);
    if err < 0 {
        return err;
    }

    amdtp_stream_add_pcm_hw_constraints(s, runtime)
}

unsafe extern "C" fn read_status_messages(
    s: *mut amdtp_stream,
    mut buffer: *mut __be32,
    data_blocks: c_uint,
) {
    let tscm = container_of_tx_stream(s);
    let used = READ_ONCE_bool(&(*(*tscm).hwdep).used);
    let mut i: c_int;

    i = 0;
    while i < data_blocks as c_int {
        let index: c_uint;
        let before: __be32;
        let after: __be32;

        index = be32_to_cpu(*buffer.add(0)) % SNDRV_FIREWIRE_TASCAM_STATE_COUNT as c_uint;
        before = (*tscm).state[index as usize];
        after = *buffer.add((*s).data_block_quadlets as usize - 1);

        if used && index > 4 && index < 16 {
            let mask: __be32;

            if index == 5 {
                mask = cpu_to_be32(!0x0000ffffu32);
            } else if index == 6 {
                mask = cpu_to_be32(!0x0000ffffu32);
            } else if index == 8 {
                mask = cpu_to_be32(!0x000f0f00u32);
            } else {
                mask = cpu_to_be32(!0x00000000u32);
            }

            if ((before ^ after) & mask) != 0 {
                let entry = &mut (*tscm).queue[(*tscm).push_pos as usize]
                    as *mut snd_firewire_tascam_change;
                let mut flags: c_ulong = 0;

                spin_lock_irqsave(&mut (*tscm).lock, &mut flags);
                (*entry).index = index;
                (*entry).before = before;
                (*entry).after = after;
                (*tscm).push_pos = (*tscm).push_pos.wrapping_add(1);
                if (*tscm).push_pos >= SND_TSCM_QUEUE_COUNT as c_uint {
                    (*tscm).push_pos = 0;
                }
                spin_unlock_irqrestore(&mut (*tscm).lock, flags);

                wake_up(&mut (*tscm).hwdep_wait);
            }
        }

        (*tscm).state[index as usize] = after;
        buffer = buffer.add((*s).data_block_quadlets as usize);
        i += 1;
    }
}

unsafe extern "C" fn process_ir_ctx_payloads(
    s: *mut amdtp_stream,
    mut desc: *const pkt_desc,
    count: c_uint,
    pcm: *mut snd_pcm_substream,
) {
    let mut pcm_frames: c_uint = 0;
    let mut i: c_int;

    i = 0;
    while i < count as c_int {
        let buf = (*desc).ctx_payload;
        let data_blocks = (*desc).data_blocks;

        if !pcm.is_null() {
            read_pcm_s32(s, pcm, buf, data_blocks, pcm_frames);
            pcm_frames = pcm_frames.wrapping_add(data_blocks);
        }

        read_status_messages(s, buf, data_blocks);

        desc = amdtp_stream_next_packet_desc(s, desc);
        i += 1;
    }
}

unsafe extern "C" fn process_it_ctx_payloads(
    s: *mut amdtp_stream,
    mut desc: *const pkt_desc,
    count: c_uint,
    pcm: *mut snd_pcm_substream,
) {
    let mut pcm_frames: c_uint = 0;
    let mut i: c_int;

    i = 0;
    while i < count as c_int {
        let buf = (*desc).ctx_payload;
        let data_blocks = (*desc).data_blocks;

        if !pcm.is_null() {
            write_pcm_s32(s, pcm, buf, data_blocks, pcm_frames);
            pcm_frames = pcm_frames.wrapping_add(data_blocks);
        } else {
            write_pcm_silence(s, buf, data_blocks);
        }

        desc = amdtp_stream_next_packet_desc(s, desc);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn amdtp_tscm_init(
    s: *mut amdtp_stream,
    unit: *mut fw_unit,
    dir: amdtp_stream_direction,
    pcm_channels: c_uint,
) -> c_int {
    let process_ctx_payloads: amdtp_stream_process_ctx_payloads_t;
    let flags: c_uint = CIP_NONBLOCKING | CIP_SKIP_DBC_ZERO_CHECK | CIP_UNAWARE_SYT;
    let p: *mut amdtp_tscm;
    let fmt: c_uint;
    let mut err: c_int;

    if dir == AMDTP_IN_STREAM {
        fmt = AMDTP_FMT_TSCM_TX;
        process_ctx_payloads = Some(process_ir_ctx_payloads);
    } else {
        fmt = AMDTP_FMT_TSCM_RX;
        process_ctx_payloads = Some(process_it_ctx_payloads);
    }

    err = amdtp_stream_init(
        s,
        unit,
        dir,
        flags,
        fmt,
        process_ctx_payloads,
        size_of::<amdtp_tscm>(),
    );
    if err < 0 {
        return err;
    }

    if dir == AMDTP_OUT_STREAM {
        // Use fixed value for FDF field.
        (*s).ctx_data.rx.fdf = 0x00;
    }

    /* This protocol uses fixed number of data channels for PCM samples. */
    p = (*s).protocol as *mut amdtp_tscm;
    (*p).pcm_channels = pcm_channels;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
