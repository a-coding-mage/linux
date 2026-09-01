// SPDX-License-Identifier: GPL-2.0-only
/*
 * amdtp-ff.c - a part of driver for RME Fireface series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto
 */

// C dependencies:
// #include <sound/pcm.h>
// #include "ff.h"

use core::ffi::c_void;

type __le32 = u32;
type u32 = core::ffi::c_uint;

const EBUSY: core::ffi::c_int = 16;
const CIP_BLOCKING: core::ffi::c_uint = 0x01;
const CIP_UNAWARE_SYT: core::ffi::c_uint = 0x02;
const CIP_NO_HEADER: core::ffi::c_uint = 0x04;
const AMDTP_IN_STREAM: amdtp_stream_direction = 0;

type amdtp_stream_direction = core::ffi::c_uint;
type amdtp_stream_process_ctx_payloads_t = Option<
    unsafe extern "C" fn(
        s: *mut amdtp_stream,
        desc: *const pkt_desc,
        count: core::ffi::c_uint,
        pcm: *mut snd_pcm_substream,
    ),
>;

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdtp_stream {
    pub protocol: *mut c_void,
    pub pcm_buffer_pointer: core::ffi::c_uint,
    pub data_block_quadlets: core::ffi::c_uint,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub buffer_size: core::ffi::c_uint,
    pub dma_area: *mut c_void,
}

#[repr(C)]
pub struct pkt_desc {
    pub ctx_payload: *mut c_void,
    pub data_blocks: core::ffi::c_uint,
}

#[repr(C)]
struct amdtp_ff {
    pcm_channels: core::ffi::c_uint,
}

unsafe extern "C" {
    fn amdtp_stream_running(s: *mut amdtp_stream) -> bool;
    fn amdtp_stream_set_parameters(
        s: *mut amdtp_stream,
        rate: core::ffi::c_uint,
        data_channels: core::ffi::c_uint,
        midi_ports: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn frames_to_bytes(
        runtime: *mut snd_pcm_runtime,
        frames: core::ffi::c_uint,
    ) -> core::ffi::c_ulong;
    fn snd_pcm_hw_constraint_msbits(
        runtime: *mut snd_pcm_runtime,
        cond: core::ffi::c_uint,
        width: core::ffi::c_uint,
        msbits: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    fn amdtp_stream_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> core::ffi::c_int;
    fn amdtp_stream_next_packet_desc(
        s: *mut amdtp_stream,
        desc: *const pkt_desc,
    ) -> *const pkt_desc;
    fn amdtp_stream_init(
        s: *mut amdtp_stream,
        unit: *mut fw_unit,
        dir: amdtp_stream_direction,
        flags: core::ffi::c_uint,
        fmt: core::ffi::c_uint,
        process_ctx_payloads: amdtp_stream_process_ctx_payloads_t,
        protocol_size: usize,
    ) -> core::ffi::c_int;
}

#[inline]
fn cpu_to_le32(value: u32) -> __le32 {
    value.to_le()
}

#[inline]
fn le32_to_cpu(value: __le32) -> u32 {
    u32::from_le(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn amdtp_ff_set_parameters(
    s: *mut amdtp_stream,
    rate: core::ffi::c_uint,
    pcm_channels: core::ffi::c_uint,
) -> core::ffi::c_int {
    let p = unsafe { (*s).protocol as *mut amdtp_ff };
    let data_channels: core::ffi::c_uint;

    if unsafe { amdtp_stream_running(s) } {
        return -EBUSY;
    }

    unsafe {
        (*p).pcm_channels = pcm_channels;
    }
    data_channels = pcm_channels;

    unsafe { amdtp_stream_set_parameters(s, rate, data_channels, 1) }
}

unsafe extern "C" fn write_pcm_s32(
    s: *mut amdtp_stream,
    pcm: *mut snd_pcm_substream,
    mut buffer: *mut __le32,
    frames: core::ffi::c_uint,
    pcm_frames: core::ffi::c_uint,
) {
    let p = unsafe { (*s).protocol as *mut amdtp_ff };
    let channels = unsafe { (*p).pcm_channels };
    let runtime = unsafe { (*pcm).runtime };
    let mut pcm_buffer_pointer: core::ffi::c_uint;
    let mut remaining_frames: core::ffi::c_int;
    let mut src: *const u32;
    let mut i: core::ffi::c_int;
    let mut c: core::ffi::c_int;

    pcm_buffer_pointer = unsafe { (*s).pcm_buffer_pointer }.wrapping_add(pcm_frames);
    pcm_buffer_pointer %= unsafe { (*runtime).buffer_size };

    src = unsafe {
        ((*runtime).dma_area as *const u8)
            .add(frames_to_bytes(runtime, pcm_buffer_pointer) as usize)
            as *const u32
    };
    remaining_frames = unsafe { (*runtime).buffer_size.wrapping_sub(pcm_buffer_pointer) as core::ffi::c_int };

    i = 0;
    while i < frames as core::ffi::c_int {
        c = 0;
        while c < channels as core::ffi::c_int {
            unsafe {
                *buffer.add(c as usize) = cpu_to_le32(*src);
                src = src.add(1);
            }
            c += 1;
        }
        unsafe {
            buffer = buffer.add((*s).data_block_quadlets as usize);
        }
        remaining_frames -= 1;
        if remaining_frames == 0 {
            src = unsafe { (*runtime).dma_area as *const u32 };
        }
        i += 1;
    }
}

unsafe extern "C" fn read_pcm_s32(
    s: *mut amdtp_stream,
    pcm: *mut snd_pcm_substream,
    mut buffer: *mut __le32,
    frames: core::ffi::c_uint,
    pcm_frames: core::ffi::c_uint,
) {
    let p = unsafe { (*s).protocol as *mut amdtp_ff };
    let channels = unsafe { (*p).pcm_channels };
    let runtime = unsafe { (*pcm).runtime };
    let mut pcm_buffer_pointer: core::ffi::c_uint;
    let mut remaining_frames: core::ffi::c_int;
    let mut dst: *mut u32;
    let mut i: core::ffi::c_int;
    let mut c: core::ffi::c_int;

    pcm_buffer_pointer = unsafe { (*s).pcm_buffer_pointer }.wrapping_add(pcm_frames);
    pcm_buffer_pointer %= unsafe { (*runtime).buffer_size };

    dst = unsafe {
        ((*runtime).dma_area as *mut u8)
            .add(frames_to_bytes(runtime, pcm_buffer_pointer) as usize)
            as *mut u32
    };
    remaining_frames = unsafe { (*runtime).buffer_size.wrapping_sub(pcm_buffer_pointer) as core::ffi::c_int };

    i = 0;
    while i < frames as core::ffi::c_int {
        c = 0;
        while c < channels as core::ffi::c_int {
            unsafe {
                *dst = le32_to_cpu(*buffer.add(c as usize)) & 0xffffff00;
                dst = dst.add(1);
            }
            c += 1;
        }
        unsafe {
            buffer = buffer.add((*s).data_block_quadlets as usize);
        }
        remaining_frames -= 1;
        if remaining_frames == 0 {
            dst = unsafe { (*runtime).dma_area as *mut u32 };
        }
        i += 1;
    }
}

unsafe extern "C" fn write_pcm_silence(
    s: *mut amdtp_stream,
    mut buffer: *mut __le32,
    frames: core::ffi::c_uint,
) {
    let p = unsafe { (*s).protocol as *mut amdtp_ff };
    let channels = unsafe { (*p).pcm_channels };
    let mut i: core::ffi::c_uint;
    let mut c: core::ffi::c_uint;

    i = 0;
    while i < frames {
        c = 0;
        while c < channels {
            unsafe {
                *buffer.add(c as usize) = cpu_to_le32(0x00000000);
            }
            c += 1;
        }
        unsafe {
            buffer = buffer.add((*s).data_block_quadlets as usize);
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn amdtp_ff_add_pcm_hw_constraints(
    s: *mut amdtp_stream,
    runtime: *mut snd_pcm_runtime,
) -> core::ffi::c_int {
    let mut err: core::ffi::c_int;

    err = unsafe { snd_pcm_hw_constraint_msbits(runtime, 0, 32, 24) };
    if err < 0 {
        return err;
    }

    unsafe { amdtp_stream_add_pcm_hw_constraints(s, runtime) }
}

unsafe extern "C" fn process_it_ctx_payloads(
    s: *mut amdtp_stream,
    mut desc: *const pkt_desc,
    count: core::ffi::c_uint,
    pcm: *mut snd_pcm_substream,
) {
    let mut pcm_frames: core::ffi::c_uint = 0;
    let mut i: core::ffi::c_int;

    i = 0;
    while i < count as core::ffi::c_int {
        let buf = unsafe { (*desc).ctx_payload as *mut __le32 };
        let data_blocks = unsafe { (*desc).data_blocks };

        if !pcm.is_null() {
            unsafe {
                write_pcm_s32(s, pcm, buf, data_blocks, pcm_frames);
            }
            pcm_frames = pcm_frames.wrapping_add(data_blocks);
        } else {
            unsafe {
                write_pcm_silence(s, buf, data_blocks);
            }
        }

        desc = unsafe { amdtp_stream_next_packet_desc(s, desc) };
        i += 1;
    }
}

unsafe extern "C" fn process_ir_ctx_payloads(
    s: *mut amdtp_stream,
    mut desc: *const pkt_desc,
    count: core::ffi::c_uint,
    pcm: *mut snd_pcm_substream,
) {
    let mut pcm_frames: core::ffi::c_uint = 0;
    let mut i: core::ffi::c_int;

    i = 0;
    while i < count as core::ffi::c_int {
        let buf = unsafe { (*desc).ctx_payload as *mut __le32 };
        let data_blocks = unsafe { (*desc).data_blocks };

        if !pcm.is_null() {
            unsafe {
                read_pcm_s32(s, pcm, buf, data_blocks, pcm_frames);
            }
            pcm_frames = pcm_frames.wrapping_add(data_blocks);
        }

        desc = unsafe { amdtp_stream_next_packet_desc(s, desc) };
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn amdtp_ff_init(
    s: *mut amdtp_stream,
    unit: *mut fw_unit,
    dir: amdtp_stream_direction,
) -> core::ffi::c_int {
    let process_ctx_payloads: amdtp_stream_process_ctx_payloads_t;

    if dir == AMDTP_IN_STREAM {
        process_ctx_payloads = Some(process_ir_ctx_payloads);
    } else {
        process_ctx_payloads = Some(process_it_ctx_payloads);
    }

    unsafe {
        amdtp_stream_init(
            s,
            unit,
            dir,
            CIP_BLOCKING | CIP_UNAWARE_SYT | CIP_NO_HEADER,
            0,
            process_ctx_payloads,
            core::mem::size_of::<amdtp_ff>(),
        )
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
