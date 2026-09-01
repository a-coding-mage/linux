// SPDX-License-Identifier: GPL-2.0-only
/*
 * amdtp-dot.c - a part of driver for Digidesign Digi 002/003 family
 *
 * Copyright (c) 2014-2015 Takashi Sakamoto
 * Copyright (C) 2012 Robin Gareus <robin@gareus.org>
 * Copyright (C) 2012 Damien Zammit <damien@zamaudio.com>
 */

// Dependencies from <sound/pcm.h> and "digi00x.h" are declared here as
// external C-compatible items and are expected to be supplied by surrounding
// translated sources.

use core::ffi::c_void;

pub type u8 = ::core::ffi::c_uchar;
pub type u32 = ::core::ffi::c_uint;
pub type __be32 = u32;

pub const EBUSY: ::core::ffi::c_int = 16;

pub const CIP_FMT_AM: ::core::ffi::c_uint = 0x10;

/* 'Clock-based rate control mode' is just supported. */
pub const AMDTP_FDF_AM824: ::core::ffi::c_uint = 0x00;

/*
 * Nominally 3125 bytes/second, but the MIDI port's clock might be
 * 1% too slow, and the bus clock 100 ppm too fast.
 */
pub const MIDI_BYTES_PER_SECOND: ::core::ffi::c_int = 3093;

/*
 * Several devices look only at the first eight data blocks.
 * In any case, this is more than enough for the MIDI data rate.
 */
pub const MAX_MIDI_RX_BLOCKS: usize = 8;

/* 3 = MAX(DOT_MIDI_IN_PORTS, DOT_MIDI_OUT_PORTS) + 1. */
pub const MAX_MIDI_PORTS: usize = 3;

pub const BYTE_PER_SAMPLE: usize = 4;
pub const MAGIC_DOT_BYTE: usize = 2;

pub const CIP_NONBLOCKING: ::core::ffi::c_uint = 0;
pub const CIP_UNAWARE_SYT: ::core::ffi::c_uint = 0;
pub const AMDTP_IN_STREAM: amdtp_stream_direction = 0;

#[repr(C)]
pub struct snd_rawmidi_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_area: *mut c_void,
    pub buffer_size: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

pub type amdtp_stream_direction = ::core::ffi::c_uint;

#[repr(C)]
pub struct amdtp_stream_rx_ctx_data {
    pub fdf: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct amdtp_stream_ctx_data_rx_wrapper {
    pub rx: amdtp_stream_rx_ctx_data,
}

#[repr(C)]
pub struct amdtp_stream {
    pub protocol: *mut amdtp_dot,
    pub ctx_data: amdtp_stream_ctx_data_rx_wrapper,
    pub sfc: ::core::ffi::c_uint,
    pub syt_interval: ::core::ffi::c_uint,
    pub data_block_quadlets: ::core::ffi::c_uint,
    pub pcm_buffer_pointer: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct pkt_desc {
    pub ctx_payload: *mut __be32,
    pub data_blocks: ::core::ffi::c_uint,
    pub data_block_counter: ::core::ffi::c_uint,
}

pub type amdtp_stream_process_ctx_payloads_t = Option<
    unsafe extern "C" fn(
        s: *mut amdtp_stream,
        desc: *const pkt_desc,
        count: ::core::ffi::c_uint,
        pcm: *mut snd_pcm_substream,
    ),
>;

unsafe extern "C" {
    static amdtp_rate_table: [::core::ffi::c_uint; 0];

    fn amdtp_stream_running(s: *mut amdtp_stream) -> bool;
    fn amdtp_stream_set_parameters(
        s: *mut amdtp_stream,
        rate: ::core::ffi::c_uint,
        data_channels: ::core::ffi::c_uint,
        pcm_channels: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn frames_to_bytes(
        runtime: *mut snd_pcm_runtime,
        frames: ::core::ffi::c_uint,
    ) -> usize;
    fn cpu_to_be32(value: u32) -> __be32;
    fn be32_to_cpu(value: __be32) -> u32;
    fn snd_rawmidi_transmit(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut u8,
        count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn snd_rawmidi_receive(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut u8,
        count: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn snd_pcm_hw_constraint_msbits(
        runtime: *mut snd_pcm_runtime,
        cond: ::core::ffi::c_uint,
        width: ::core::ffi::c_uint,
        msbits: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn amdtp_stream_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> ::core::ffi::c_int;
    fn amdtp_stream_next_packet_desc(
        s: *mut amdtp_stream,
        desc: *const pkt_desc,
    ) -> *const pkt_desc;
    fn amdtp_stream_init(
        s: *mut amdtp_stream,
        unit: *mut fw_unit,
        dir: amdtp_stream_direction,
        flags: ::core::ffi::c_uint,
        fmt: ::core::ffi::c_uint,
        process_ctx_payloads: amdtp_stream_process_ctx_payloads_t,
        protocol_size: usize,
    ) -> ::core::ffi::c_int;
}

/*
 * The double-oh-three algorithm was discovered by Robin Gareus and Damien
 * Zammit in 2012, with reverse-engineering for Digi 003 Rack.
 */
#[repr(C)]
pub struct dot_state {
    pub carry: u8,
    pub idx: u8,
    pub off: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct amdtp_dot {
    pub pcm_channels: ::core::ffi::c_uint,
    pub state: dot_state,

    pub midi: [*mut snd_rawmidi_substream; MAX_MIDI_PORTS],
    pub midi_fifo_used: [::core::ffi::c_int; MAX_MIDI_PORTS],
    pub midi_fifo_limit: ::core::ffi::c_int,
}

/*
 * double-oh-three look up table
 *
 * @param idx index byte (audio-sample data) 0x00..0xff
 * @param off channel offset shift
 * @return salt to XOR with given data
 */
pub const fn MAGIC_BYTE_OFF(x: usize) -> usize {
    x * BYTE_PER_SAMPLE + MAGIC_DOT_BYTE
}

unsafe fn dot_scrt(idx: u8, off: ::core::ffi::c_uint) -> u8 {
    /*
     * the length of the added pattern only depends on the lower nibble
     * of the last non-zero data
     */
    static LEN: [u8; 16] = [0, 1, 3, 5, 7, 9, 11, 13, 14, 12, 10, 8, 6, 4, 2, 0];

    /*
     * the lower nibble of the salt. Interleaved sequence.
     * this is walked backwards according to len[]
     */
    static NIB: [u8; 15] = [
        0x8, 0x7, 0x9, 0x6, 0xa, 0x5, 0xb, 0x4, 0xc, 0x3, 0xd, 0x2, 0xe, 0x1, 0xf,
    ];

    /* circular list for the salt's hi nibble. */
    static HIR: [u8; 15] = [
        0x0, 0x6, 0xf, 0x8, 0x7, 0x5, 0x3, 0x4, 0xc, 0xd, 0xe, 0x1, 0x2, 0xb, 0xa,
    ];

    /*
     * start offset for upper nibble mapping.
     * note: 9 is /special/. In the case where the high nibble == 0x9,
     * hir[] is not used and - coincidentally - the salt's hi nibble is
     * 0x09 regardless of the offset.
     */
    static HIO: [u8; 16] = [0, 11, 12, 6, 7, 5, 1, 4, 3, 0x00, 14, 13, 8, 9, 10, 2];

    let ln: u8 = idx & 0xf;
    let hn: u8 = (idx >> 4) & 0xf;
    let hr: u8 = if hn == 0x9 {
        0x9
    } else {
        HIR[((HIO[hn as usize] as ::core::ffi::c_uint + off) % 15) as usize]
    };

    if (LEN[ln as usize] as ::core::ffi::c_uint) < off {
        return 0x00;
    }

    NIB[(14_i32 + off as i32 - LEN[ln as usize] as i32) as usize] | (hr << 4)
}

unsafe fn dot_encode_step(state: *mut dot_state, buffer: *mut __be32) {
    let data: *mut u8 = buffer as *mut u8;

    if *data.add(MAGIC_DOT_BYTE) != 0x00 {
        (*state).off = 0;
        (*state).idx = *data.add(MAGIC_DOT_BYTE) ^ (*state).carry;
    }
    *data.add(MAGIC_DOT_BYTE) ^= (*state).carry;
    (*state).off = (*state).off.wrapping_add(1);
    (*state).carry = dot_scrt((*state).idx, (*state).off);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn amdtp_dot_set_parameters(
    s: *mut amdtp_stream,
    rate: ::core::ffi::c_uint,
    pcm_channels: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let p: *mut amdtp_dot = (*s).protocol;
    let err: ::core::ffi::c_int;

    if amdtp_stream_running(s) {
        return -EBUSY;
    }

    /*
     * A first data channel is for MIDI messages, the rest is Multi Bit
     * Linear Audio data channel.
     */
    err = amdtp_stream_set_parameters(s, rate, pcm_channels.wrapping_add(1), 1);
    if err < 0 {
        return err;
    }

    (*s).ctx_data.rx.fdf = AMDTP_FDF_AM824 | (*s).sfc;

    (*p).pcm_channels = pcm_channels;

    /*
     * We do not know the actual MIDI FIFO size of most devices.  Just
     * assume two bytes, i.e., one byte can be received over the bus while
     * the previous one is transmitted over MIDI.
     * (The value here is adjusted for midi_ratelimit_per_packet().)
     */
    (*p).midi_fifo_limit =
        rate as ::core::ffi::c_int - MIDI_BYTES_PER_SECOND * (*s).syt_interval as ::core::ffi::c_int + 1;

    0
}

unsafe fn write_pcm_s32(
    s: *mut amdtp_stream,
    pcm: *mut snd_pcm_substream,
    mut buffer: *mut __be32,
    frames: ::core::ffi::c_uint,
    pcm_frames: ::core::ffi::c_uint,
) {
    let p: *mut amdtp_dot = (*s).protocol;
    let channels: ::core::ffi::c_uint = (*p).pcm_channels;
    let runtime: *mut snd_pcm_runtime = (*pcm).runtime;
    let mut pcm_buffer_pointer: ::core::ffi::c_uint;
    let mut remaining_frames: ::core::ffi::c_int;
    let mut src: *const u32;
    let mut i: ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int;

    pcm_buffer_pointer = (*s).pcm_buffer_pointer.wrapping_add(pcm_frames);
    pcm_buffer_pointer %= (*runtime).buffer_size;

    src = ((*runtime).dma_area as *mut u8).add(frames_to_bytes(runtime, pcm_buffer_pointer)) as *const u32;
    remaining_frames = ((*runtime).buffer_size - pcm_buffer_pointer) as ::core::ffi::c_int;

    buffer = buffer.add(1);
    i = 0;
    while i < frames as ::core::ffi::c_int {
        c = 0;
        while c < channels as ::core::ffi::c_int {
            *buffer.add(c as usize) = cpu_to_be32(((*src >> 8) | 0x40000000) as u32);
            dot_encode_step(&mut (*p).state, buffer.add(c as usize));
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

unsafe fn read_pcm_s32(
    s: *mut amdtp_stream,
    pcm: *mut snd_pcm_substream,
    mut buffer: *mut __be32,
    frames: ::core::ffi::c_uint,
    pcm_frames: ::core::ffi::c_uint,
) {
    let p: *mut amdtp_dot = (*s).protocol;
    let channels: ::core::ffi::c_uint = (*p).pcm_channels;
    let runtime: *mut snd_pcm_runtime = (*pcm).runtime;
    let mut pcm_buffer_pointer: ::core::ffi::c_uint;
    let mut remaining_frames: ::core::ffi::c_int;
    let mut dst: *mut u32;
    let mut i: ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int;

    pcm_buffer_pointer = (*s).pcm_buffer_pointer.wrapping_add(pcm_frames);
    pcm_buffer_pointer %= (*runtime).buffer_size;

    dst = ((*runtime).dma_area as *mut u8).add(frames_to_bytes(runtime, pcm_buffer_pointer)) as *mut u32;
    remaining_frames = ((*runtime).buffer_size - pcm_buffer_pointer) as ::core::ffi::c_int;

    buffer = buffer.add(1);
    i = 0;
    while i < frames as ::core::ffi::c_int {
        c = 0;
        while c < channels as ::core::ffi::c_int {
            *dst = be32_to_cpu(*buffer.add(c as usize)) << 8;
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

unsafe fn write_pcm_silence(
    s: *mut amdtp_stream,
    mut buffer: *mut __be32,
    data_blocks: ::core::ffi::c_uint,
) {
    let p: *mut amdtp_dot = (*s).protocol;
    let channels: ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int;

    channels = (*p).pcm_channels;

    buffer = buffer.add(1);
    i = 0;
    while i < data_blocks as ::core::ffi::c_int {
        c = 0;
        while c < channels as ::core::ffi::c_int {
            *buffer.add(c as usize) = cpu_to_be32(0x40000000);
            c += 1;
        }
        buffer = buffer.add((*s).data_block_quadlets as usize);
        i += 1;
    }
}

unsafe fn midi_ratelimit_per_packet(s: *mut amdtp_stream, port: ::core::ffi::c_uint) -> bool {
    let p: *mut amdtp_dot = (*s).protocol;
    let mut used: ::core::ffi::c_int;

    used = (*p).midi_fifo_used[port as usize];
    if used == 0 {
        return true;
    }

    used -= MIDI_BYTES_PER_SECOND * (*s).syt_interval as ::core::ffi::c_int;
    used = core::cmp::max(used, 0);
    (*p).midi_fifo_used[port as usize] = used;

    used < (*p).midi_fifo_limit
}

unsafe fn midi_use_bytes(
    s: *mut amdtp_stream,
    port: ::core::ffi::c_uint,
    count: ::core::ffi::c_uint,
) {
    let p: *mut amdtp_dot = (*s).protocol;

    (*p).midi_fifo_used[port as usize] += amdtp_rate_table[(*s).sfc as usize] as ::core::ffi::c_int
        * count as ::core::ffi::c_int;
}

unsafe fn write_midi_messages(
    s: *mut amdtp_stream,
    mut buffer: *mut __be32,
    data_blocks: ::core::ffi::c_uint,
    data_block_counter: ::core::ffi::c_uint,
) {
    let p: *mut amdtp_dot = (*s).protocol;
    let mut f: ::core::ffi::c_uint;
    let mut port: ::core::ffi::c_uint;
    let mut len: ::core::ffi::c_int;
    let mut b: *mut u8;

    f = 0;
    while f < data_blocks {
        port = (data_block_counter + f) % 8;
        b = &mut *buffer.add(0) as *mut __be32 as *mut u8;

        len = 0;
        if port < MAX_MIDI_PORTS as ::core::ffi::c_uint
            && midi_ratelimit_per_packet(s, port)
            && !(*p).midi[port as usize].is_null()
        {
            len = snd_rawmidi_transmit((*p).midi[port as usize], b.add(1), 2);
        }

        if len > 0 {
            /*
             * Upper 4 bits of LSB represent port number.
             * - 0000b: physical MIDI port 1.
             * - 0010b: physical MIDI port 2.
             * - 1110b: console MIDI port.
             */
            if port == 2 {
                *b.add(3) = 0xe0;
            } else if port == 1 {
                *b.add(3) = 0x20;
            } else {
                *b.add(3) = 0x00;
            }
            *b.add(3) |= len as u8;
            midi_use_bytes(s, port, len as ::core::ffi::c_uint);
        } else {
            *b.add(1) = 0;
            *b.add(2) = 0;
            *b.add(3) = 0;
        }
        *b.add(0) = 0x80;

        buffer = buffer.add((*s).data_block_quadlets as usize);
        f += 1;
    }
}

unsafe fn read_midi_messages(
    s: *mut amdtp_stream,
    mut buffer: *mut __be32,
    data_blocks: ::core::ffi::c_uint,
) {
    let p: *mut amdtp_dot = (*s).protocol;
    let mut f: ::core::ffi::c_uint;
    let mut port: ::core::ffi::c_uint;
    let mut len: ::core::ffi::c_uint;
    let mut b: *mut u8;

    f = 0;
    while f < data_blocks {
        b = &mut *buffer.add(0) as *mut __be32 as *mut u8;

        len = (*b.add(3) & 0x0f) as ::core::ffi::c_uint;
        if len > 0 {
            /*
             * Upper 4 bits of LSB represent port number.
             * - 0000b: physical MIDI port 1. Use port 0.
             * - 1110b: console MIDI port. Use port 2.
             */
            if *b.add(3) >> 4 > 0 {
                port = 2;
            } else {
                port = 0;
            }

            if port < MAX_MIDI_PORTS as ::core::ffi::c_uint && !(*p).midi[port as usize].is_null() {
                snd_rawmidi_receive((*p).midi[port as usize], b.add(1), len);
            }
        }

        buffer = buffer.add((*s).data_block_quadlets as usize);
        f += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn amdtp_dot_add_pcm_hw_constraints(
    s: *mut amdtp_stream,
    runtime: *mut snd_pcm_runtime,
) -> ::core::ffi::c_int {
    let err: ::core::ffi::c_int;

    /* This protocol delivers 24 bit data in 32bit data channel. */
    err = snd_pcm_hw_constraint_msbits(runtime, 0, 32, 24);
    if err < 0 {
        return err;
    }

    amdtp_stream_add_pcm_hw_constraints(s, runtime)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn amdtp_dot_midi_trigger(
    s: *mut amdtp_stream,
    port: ::core::ffi::c_uint,
    midi: *mut snd_rawmidi_substream,
) {
    let p: *mut amdtp_dot = (*s).protocol;

    if port < MAX_MIDI_PORTS as ::core::ffi::c_uint {
        core::ptr::write_volatile(&mut (*p).midi[port as usize], midi);
    }
}

unsafe extern "C" fn process_ir_ctx_payloads(
    s: *mut amdtp_stream,
    mut desc: *const pkt_desc,
    count: ::core::ffi::c_uint,
    pcm: *mut snd_pcm_substream,
) {
    let mut pcm_frames: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_int;

    i = 0;
    while i < count as ::core::ffi::c_int {
        let buf: *mut __be32 = (*desc).ctx_payload;
        let data_blocks: ::core::ffi::c_uint = (*desc).data_blocks;

        if !pcm.is_null() {
            read_pcm_s32(s, pcm, buf, data_blocks, pcm_frames);
            pcm_frames += data_blocks;
        }

        read_midi_messages(s, buf, data_blocks);

        desc = amdtp_stream_next_packet_desc(s, desc);
        i += 1;
    }
}

unsafe extern "C" fn process_it_ctx_payloads(
    s: *mut amdtp_stream,
    mut desc: *const pkt_desc,
    count: ::core::ffi::c_uint,
    pcm: *mut snd_pcm_substream,
) {
    let mut pcm_frames: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_int;

    i = 0;
    while i < count as ::core::ffi::c_int {
        let buf: *mut __be32 = (*desc).ctx_payload;
        let data_blocks: ::core::ffi::c_uint = (*desc).data_blocks;

        if !pcm.is_null() {
            write_pcm_s32(s, pcm, buf, data_blocks, pcm_frames);
            pcm_frames += data_blocks;
        } else {
            write_pcm_silence(s, buf, data_blocks);
        }

        write_midi_messages(s, buf, data_blocks, (*desc).data_block_counter);

        desc = amdtp_stream_next_packet_desc(s, desc);
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn amdtp_dot_init(
    s: *mut amdtp_stream,
    unit: *mut fw_unit,
    dir: amdtp_stream_direction,
) -> ::core::ffi::c_int {
    let process_ctx_payloads: amdtp_stream_process_ctx_payloads_t;
    let flags: ::core::ffi::c_uint = CIP_NONBLOCKING | CIP_UNAWARE_SYT;

    // Use different mode between incoming/outgoing.
    if dir == AMDTP_IN_STREAM {
        process_ctx_payloads = Some(process_ir_ctx_payloads);
    } else {
        process_ctx_payloads = Some(process_it_ctx_payloads);
    }

    amdtp_stream_init(
        s,
        unit,
        dir,
        flags,
        CIP_FMT_AM,
        process_ctx_payloads,
        core::mem::size_of::<amdtp_dot>(),
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn amdtp_dot_reset(s: *mut amdtp_stream) {
    let p: *mut amdtp_dot = (*s).protocol;

    (*p).state.carry = 0x00;
    (*p).state.idx = 0x00;
    (*p).state.off = 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
