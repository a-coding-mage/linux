// SPDX-License-Identifier: GPL-2.0-only
/*
 * amdtp-motu.c - a part of driver for MOTU FireWire series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// Translated from C. Original dependencies:
// <linux/slab.h>, <sound/pcm.h>, "motu.h", and "amdtp-motu-trace.h".

use core::ffi::c_void;
use core::mem::size_of;
use core::ptr;

type U8 = u8;
type U32 = u32;
type U64 = u64;
type Be32 = u32;

const EBUSY: i32 = 16;
const EINVAL: i32 = 22;
const UINT_MAX: u32 = u32::MAX;

const CIP_FMT_MOTU: i32 = 0x02;
const CIP_FMT_MOTU_TX_V3: i32 = 0x22;
const MOTU_FDF_AM824: u8 = 0x22;

const TICKS_PER_CYCLE: u32 = 3072;
const CYCLES_PER_SECOND: u32 = 8000;
const TICKS_PER_SECOND: u32 = TICKS_PER_CYCLE * CYCLES_PER_SECOND;

const CIP_SPH_CYCLE_SHIFT: u32 = 12;
const CIP_SPH_CYCLE_MASK: u32 = 0x01fff000;
const CIP_SPH_OFFSET_MASK: u32 = 0x00000fff;

/*
 * Nominally 3125 bytes/second, but the MIDI port's clock might be
 * 1% too slow, and the bus clock 100 ppm too fast.
 */
const MIDI_BYTES_PER_SECOND: u32 = 3093;

const CIP_BLOCKING: u32 = 0;
const CIP_UNAWARE_SYT: u32 = 0;
const CIP_WRONG_DBS: u32 = 0;
const CIP_SKIP_DBC_ZERO_CHECK: u32 = 0;
const CIP_HEADER_WITHOUT_EOH: u32 = 0;
const CIP_DBC_IS_END_EVENT: u32 = 0;

const AMDTP_IN_STREAM: amdtp_stream_direction = 0;
const AMDTP_OUT_STREAM: amdtp_stream_direction = 1;

const SND_MOTU_PROTOCOL_V3: u32 = 0;
const SND_MOTU_SPEC_REGISTER_DSP: u32 = 0;
const SND_MOTU_SPEC_COMMAND_DSP: u32 = 0;

type amdtp_stream_direction = i32;
type amdtp_stream_process_ctx_payloads_t = Option<
    unsafe extern "C" fn(
        s: *mut amdtp_stream,
        desc: *const pkt_desc,
        count: u32,
        pcm: *mut snd_pcm_substream,
    ),
>;

#[repr(C)]
pub struct amdtp_motu {
    pcm_chunks: u32,
    pcm_byte_offset: u32,

    midi: *mut snd_rawmidi_substream,
    midi_ports: u32,
    midi_flag_offset: u32,
    midi_byte_offset: u32,

    midi_db_count: i32,
    midi_db_interval: u32,

    cache: *mut amdtp_motu_cache,
}

#[repr(C)]
pub struct amdtp_stream {
    protocol: *mut amdtp_motu,
    pcm_buffer_pointer: u32,
    data_block_quadlets: u32,
    domain: *mut amdtp_domain,
    sph: i32,
    ctx_data: amdtp_ctx_data,
}

#[repr(C)]
pub struct amdtp_ctx_data {
    rx: amdtp_rx,
}

#[repr(C)]
pub struct amdtp_rx {
    fdf: u8,
}

#[repr(C)]
pub struct amdtp_domain {
    processing_cycle: amdtp_processing_cycle,
}

#[repr(C)]
pub struct amdtp_processing_cycle {
    tx_start: u32,
    rx_start: u32,
}

#[repr(C)]
pub struct amdtp_motu_cache {
    event_offsets: *mut u32,
    size: u32,
    tail: u32,
    head: u32,
    tx_cycle_count: u32,
    rx_cycle_count: u32,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    buffer_size: u32,
    dma_area: *mut U8,
}

#[repr(C)]
pub struct snd_motu_packet_format {
    pcm_chunks: *mut u32,
    msg_chunks: u32,
    pcm_byte_offset: u32,
    midi_flag_offset: u32,
    midi_byte_offset: u32,
}

#[repr(C)]
pub struct pkt_desc {
    ctx_payload: *mut Be32,
    data_blocks: u32,
}

#[repr(C)]
pub struct snd_motu {
    tx_stream: amdtp_stream,
    spec: *const snd_motu_spec,
}

#[repr(C)]
pub struct snd_motu_spec {
    flags: u32,
    protocol_version: u32,
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

extern "C" {
    static snd_motu_clock_rates: [u32; 0];
    static snd_motu_spec_8pre: snd_motu_spec;
    static snd_motu_spec_ultralite: snd_motu_spec;

    fn amdtp_stream_running(s: *mut amdtp_stream) -> bool;
    fn amdtp_stream_set_parameters(
        s: *mut amdtp_stream,
        rate: u32,
        data_block_quadlets: u32,
        events_per_period: u32,
    ) -> i32;
    fn amdtp_stream_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> i32;
    fn amdtp_stream_init(
        s: *mut amdtp_stream,
        unit: *mut fw_unit,
        dir: amdtp_stream_direction,
        flags: u32,
        fmt: i32,
        process_ctx_payloads: amdtp_stream_process_ctx_payloads_t,
        protocol_size: usize,
    ) -> i32;
    fn amdtp_stream_next_packet_desc(
        s: *mut amdtp_stream,
        desc: *const pkt_desc,
    ) -> *const pkt_desc;

    fn frames_to_bytes(runtime: *mut snd_pcm_runtime, frames: u32) -> usize;
    fn snd_pcm_hw_constraint_msbits(
        runtime: *mut snd_pcm_runtime,
        cond: u32,
        width: u32,
        msbits: u32,
    ) -> i32;
    fn snd_rawmidi_transmit(
        midi: *mut snd_rawmidi_substream,
        buf: *mut U8,
        count: usize,
    ) -> i32;
    fn snd_rawmidi_receive(
        midi: *mut snd_rawmidi_substream,
        buf: *mut U8,
        count: usize,
    ) -> i32;

    fn trace_data_block_sph(s: *mut amdtp_stream, data_blocks: u32, buf: *mut Be32);
    fn trace_data_block_message(s: *mut amdtp_stream, data_blocks: u32, buf: *mut Be32);
    fn trace_data_block_sph_enabled() -> bool;
    fn trace_data_block_message_enabled() -> bool;

    fn snd_motu_register_dsp_message_parser_parse(
        s: *mut amdtp_stream,
        desc: *const pkt_desc,
        count: u32,
    );
    fn snd_motu_command_dsp_message_parser_parse(
        s: *mut amdtp_stream,
        desc: *const pkt_desc,
        count: u32,
    );
}

const fn div_round_up(n: u32, d: u32) -> u32 {
    (n + d - 1) / d
}

fn be32_to_cpu(v: Be32) -> U32 {
    u32::from_be(v)
}

fn cpu_to_be32(v: U32) -> Be32 {
    v.to_be()
}

unsafe fn read_once<T: Copy>(p: *const T) -> T {
    ptr::read_volatile(p)
}

unsafe fn write_once<T>(p: *mut T, v: T) {
    ptr::write_volatile(p, v);
}

unsafe fn container_of_tx_stream(s: *mut amdtp_stream) -> *mut snd_motu {
    s.cast::<U8>()
        .sub(core::mem::offset_of!(snd_motu, tx_stream))
        .cast::<snd_motu>()
}

#[no_mangle]
pub unsafe extern "C" fn amdtp_motu_set_parameters(
    s: *mut amdtp_stream,
    rate: u32,
    midi_ports: u32,
    formats: *mut snd_motu_packet_format,
) -> i32 {
    let p = (*s).protocol;
    let pcm_chunks: u32;
    let data_chunks: u32;
    let data_block_quadlets: u32;
    let mut mode: u32 = 0;
    let mut i: i32;
    let err: i32;

    if amdtp_stream_running(s) {
        return -EBUSY;
    }

    i = 0;
    while (i as usize) < snd_motu_clock_rates.len() {
        if snd_motu_clock_rates[i as usize] == rate {
            mode = (i as u32) >> 1;
            break;
        }
        i += 1;
    }
    if (i as usize) == snd_motu_clock_rates.len() {
        return -EINVAL;
    }

    // Each data block includes SPH in its head. Data chunks follow with
    // 3 byte alignment. Padding follows with zero to conform to quadlet
    // alignment.
    pcm_chunks = *(*formats).pcm_chunks.add(mode as usize);
    data_chunks = (*formats).msg_chunks + pcm_chunks;
    data_block_quadlets = 1 + div_round_up(data_chunks * 3, 4);

    err = amdtp_stream_set_parameters(s, rate, data_block_quadlets, 1);
    if err < 0 {
        return err;
    }

    (*p).pcm_chunks = pcm_chunks;
    (*p).pcm_byte_offset = (*formats).pcm_byte_offset;

    (*p).midi_ports = midi_ports;
    (*p).midi_flag_offset = (*formats).midi_flag_offset;
    (*p).midi_byte_offset = (*formats).midi_byte_offset;

    (*p).midi_db_count = 0;
    (*p).midi_db_interval = rate / MIDI_BYTES_PER_SECOND;

    0
}

unsafe fn read_pcm_s32(
    s: *mut amdtp_stream,
    pcm: *mut snd_pcm_substream,
    mut buffer: *mut Be32,
    data_blocks: u32,
    pcm_frames: u32,
) {
    let p = (*s).protocol;
    let channels = (*p).pcm_chunks;
    let runtime = (*pcm).runtime;
    let mut pcm_buffer_pointer: u32;
    let mut remaining_frames: i32;
    let mut byte: *mut U8;
    let mut dst: *mut U32;
    let mut i: i32;
    let mut c: i32;

    pcm_buffer_pointer = (*s).pcm_buffer_pointer + pcm_frames;
    pcm_buffer_pointer %= (*runtime).buffer_size;

    dst = (*runtime)
        .dma_area
        .add(frames_to_bytes(runtime, pcm_buffer_pointer))
        .cast::<U32>();
    remaining_frames = ((*runtime).buffer_size - pcm_buffer_pointer) as i32;

    i = 0;
    while (i as u32) < data_blocks {
        byte = buffer.cast::<U8>().add((*p).pcm_byte_offset as usize);

        c = 0;
        while (c as u32) < channels {
            *dst = ((*byte.add(0) as U32) << 24)
                | ((*byte.add(1) as U32) << 16)
                | ((*byte.add(2) as U32) << 8);
            byte = byte.add(3);
            dst = dst.add(1);
            c += 1;
        }
        buffer = buffer.add((*s).data_block_quadlets as usize);
        remaining_frames -= 1;
        if remaining_frames == 0 {
            dst = (*runtime).dma_area.cast::<U32>();
        }
        i += 1;
    }
}

unsafe fn write_pcm_s32(
    s: *mut amdtp_stream,
    pcm: *mut snd_pcm_substream,
    mut buffer: *mut Be32,
    data_blocks: u32,
    pcm_frames: u32,
) {
    let p = (*s).protocol;
    let channels = (*p).pcm_chunks;
    let runtime = (*pcm).runtime;
    let mut pcm_buffer_pointer: u32;
    let mut remaining_frames: i32;
    let mut byte: *mut U8;
    let mut src: *const U32;
    let mut i: i32;
    let mut c: i32;

    pcm_buffer_pointer = (*s).pcm_buffer_pointer + pcm_frames;
    pcm_buffer_pointer %= (*runtime).buffer_size;

    src = (*runtime)
        .dma_area
        .add(frames_to_bytes(runtime, pcm_buffer_pointer))
        .cast::<U32>();
    remaining_frames = ((*runtime).buffer_size - pcm_buffer_pointer) as i32;

    i = 0;
    while (i as u32) < data_blocks {
        byte = buffer.cast::<U8>().add((*p).pcm_byte_offset as usize);

        c = 0;
        while (c as u32) < channels {
            *byte.add(0) = ((*src >> 24) & 0xff) as U8;
            *byte.add(1) = ((*src >> 16) & 0xff) as U8;
            *byte.add(2) = ((*src >> 8) & 0xff) as U8;
            byte = byte.add(3);
            src = src.add(1);
            c += 1;
        }

        buffer = buffer.add((*s).data_block_quadlets as usize);
        remaining_frames -= 1;
        if remaining_frames == 0 {
            src = (*runtime).dma_area.cast::<U32>();
        }
        i += 1;
    }
}

unsafe fn write_pcm_silence(s: *mut amdtp_stream, mut buffer: *mut Be32, data_blocks: u32) {
    let p = (*s).protocol;
    let channels: u32;
    let mut i: u32;
    let mut c: u32;
    let mut byte: *mut U8;

    channels = (*p).pcm_chunks;

    i = 0;
    while i < data_blocks {
        byte = buffer.cast::<U8>().add((*p).pcm_byte_offset as usize);

        c = 0;
        while c < channels {
            *byte.add(0) = 0;
            *byte.add(1) = 0;
            *byte.add(2) = 0;
            byte = byte.add(3);
            c += 1;
        }

        buffer = buffer.add((*s).data_block_quadlets as usize);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn amdtp_motu_add_pcm_hw_constraints(
    s: *mut amdtp_stream,
    runtime: *mut snd_pcm_runtime,
) -> i32 {
    let mut err: i32;

    /* TODO: how to set an constraint for exactly 24bit PCM sample? */
    err = snd_pcm_hw_constraint_msbits(runtime, 0, 32, 24);
    if err < 0 {
        return err;
    }

    amdtp_stream_add_pcm_hw_constraints(s, runtime)
}

#[no_mangle]
pub unsafe extern "C" fn amdtp_motu_midi_trigger(
    s: *mut amdtp_stream,
    port: u32,
    midi: *mut snd_rawmidi_substream,
) {
    let p = (*s).protocol;

    if port < (*p).midi_ports {
        write_once(&mut (*p).midi, midi);
    }
}

unsafe fn write_midi_messages(s: *mut amdtp_stream, mut buffer: *mut Be32, data_blocks: u32) {
    let p = (*s).protocol;
    let midi = read_once(&(*p).midi);
    let mut b: *mut U8;
    let mut i: i32;

    i = 0;
    while (i as u32) < data_blocks {
        b = buffer.cast::<U8>();

        if !midi.is_null()
            && (*p).midi_db_count == 0
            && snd_rawmidi_transmit(midi, b.add((*p).midi_byte_offset as usize), 1) == 1
        {
            *b.add((*p).midi_flag_offset as usize) = 0x01;
        } else {
            *b.add((*p).midi_byte_offset as usize) = 0x00;
            *b.add((*p).midi_flag_offset as usize) = 0x00;
        }

        buffer = buffer.add((*s).data_block_quadlets as usize);

        (*p).midi_db_count -= 1;
        if (*p).midi_db_count < 0 {
            (*p).midi_db_count = (*p).midi_db_interval as i32;
        }
        i += 1;
    }
}

unsafe fn read_midi_messages(s: *mut amdtp_stream, mut buffer: *mut Be32, data_blocks: u32) {
    let p = (*s).protocol;
    let mut midi: *mut snd_rawmidi_substream;
    let mut b: *mut U8;
    let mut i: i32;

    i = 0;
    while (i as u32) < data_blocks {
        b = buffer.cast::<U8>();
        midi = read_once(&(*p).midi);

        if !midi.is_null() && (*b.add((*p).midi_flag_offset as usize) & 0x01) != 0 {
            snd_rawmidi_receive(midi, b.add((*p).midi_byte_offset as usize), 1);
        }

        buffer = buffer.add((*s).data_block_quadlets as usize);
        i += 1;
    }
}

/* For tracepoints. */
#[allow(dead_code)]
unsafe fn copy_sph(
    mut frames: *mut U32,
    mut buffer: *mut Be32,
    data_blocks: u32,
    data_block_quadlets: u32,
) {
    let mut i: u32;

    i = 0;
    while i < data_blocks {
        *frames = be32_to_cpu(*buffer);
        buffer = buffer.add(data_block_quadlets as usize);
        frames = frames.add(1);
        i += 1;
    }
}

/* For tracepoints. */
#[allow(dead_code)]
unsafe fn copy_message(
    mut frames: *mut U64,
    mut buffer: *mut Be32,
    data_blocks: u32,
    data_block_quadlets: u32,
) {
    let mut i: u32;

    /* This is just for v2/v3 protocol. */
    i = 0;
    while i < data_blocks {
        *frames = be32_to_cpu(*buffer.add(1)) as U64;
        *frames <<= 16;
        *frames |= (be32_to_cpu(*buffer.add(2)) >> 16) as U64;
        frames = frames.add(1);
        buffer = buffer.add(data_block_quadlets as usize);
        i += 1;
    }
}

unsafe fn probe_tracepoints_events(
    s: *mut amdtp_stream,
    mut desc: *const pkt_desc,
    count: u32,
) {
    let mut i: i32;

    i = 0;
    while (i as u32) < count {
        let buf = (*desc).ctx_payload;
        let data_blocks = (*desc).data_blocks;

        trace_data_block_sph(s, data_blocks, buf);
        trace_data_block_message(s, data_blocks, buf);

        desc = amdtp_stream_next_packet_desc(s, desc);
        i += 1;
    }
}

unsafe fn cache_event_offsets(
    cache: *mut amdtp_motu_cache,
    mut buf: *const Be32,
    data_blocks: u32,
    data_block_quadlets: u32,
) {
    let event_offsets = (*cache).event_offsets;
    let cache_size = (*cache).size;
    let mut cache_tail = (*cache).tail;
    let base_tick = (*cache).tx_cycle_count * TICKS_PER_CYCLE;
    let mut i: i32;

    i = 0;
    while (i as u32) < data_blocks {
        let sph: U32 = be32_to_cpu(*buf);
        let mut tick: u32;

        tick = ((sph & CIP_SPH_CYCLE_MASK) >> CIP_SPH_CYCLE_SHIFT) * TICKS_PER_CYCLE
            + (sph & CIP_SPH_OFFSET_MASK);

        if tick < base_tick {
            tick += TICKS_PER_SECOND;
        }
        *event_offsets.add(cache_tail as usize) = tick - base_tick;

        cache_tail = (cache_tail + 1) % cache_size;
        buf = buf.add(data_block_quadlets as usize);
        i += 1;
    }

    (*cache).tail = cache_tail;
    (*cache).tx_cycle_count = ((*cache).tx_cycle_count + 1) % CYCLES_PER_SECOND;
}

unsafe extern "C" fn process_ir_ctx_payloads(
    s: *mut amdtp_stream,
    mut desc: *const pkt_desc,
    count: u32,
    pcm: *mut snd_pcm_substream,
) {
    let motu = container_of_tx_stream(s);
    let p = (*s).protocol;
    let cursor = desc;
    let mut pcm_frames: u32 = 0;
    let mut i: i32;

    if (*(*p).cache).tx_cycle_count == UINT_MAX {
        (*(*p).cache).tx_cycle_count =
            (*(*s).domain).processing_cycle.tx_start % CYCLES_PER_SECOND;
    }

    // For data block processing.
    i = 0;
    while (i as u32) < count {
        let buf = (*desc).ctx_payload;
        let data_blocks = (*desc).data_blocks;

        cache_event_offsets((*p).cache, buf, data_blocks, (*s).data_block_quadlets);

        if !pcm.is_null() {
            read_pcm_s32(s, pcm, buf, data_blocks, pcm_frames);
            pcm_frames += data_blocks;
        }

        if (*p).midi_ports != 0 {
            read_midi_messages(s, buf, data_blocks);
        }

        desc = amdtp_stream_next_packet_desc(s, desc);
        i += 1;
    }

    desc = cursor;
    if ((*(*motu).spec).flags & SND_MOTU_SPEC_REGISTER_DSP) != 0 {
        snd_motu_register_dsp_message_parser_parse(s, desc, count);
    } else if ((*(*motu).spec).flags & SND_MOTU_SPEC_COMMAND_DSP) != 0 {
        snd_motu_command_dsp_message_parser_parse(s, desc, count);
    }

    // For tracepoints.
    if trace_data_block_sph_enabled() || trace_data_block_message_enabled() {
        probe_tracepoints_events(s, desc, count);
    }
}

unsafe fn write_sph(
    cache: *mut amdtp_motu_cache,
    mut buffer: *mut Be32,
    data_blocks: u32,
    data_block_quadlets: u32,
) {
    let event_offsets = (*cache).event_offsets;
    let cache_size = (*cache).size;
    let mut cache_head = (*cache).head;
    let base_tick = (*cache).rx_cycle_count * TICKS_PER_CYCLE;
    let mut i: i32;

    i = 0;
    while (i as u32) < data_blocks {
        let tick = (base_tick + *event_offsets.add(cache_head as usize)) % TICKS_PER_SECOND;
        let sph: U32 =
            ((tick / TICKS_PER_CYCLE) << CIP_SPH_CYCLE_SHIFT) | (tick % TICKS_PER_CYCLE);
        *buffer = cpu_to_be32(sph);

        cache_head = (cache_head + 1) % cache_size;
        buffer = buffer.add(data_block_quadlets as usize);
        i += 1;
    }

    (*cache).head = cache_head;
    (*cache).rx_cycle_count = ((*cache).rx_cycle_count + 1) % CYCLES_PER_SECOND;
}

unsafe extern "C" fn process_it_ctx_payloads(
    s: *mut amdtp_stream,
    mut desc: *const pkt_desc,
    count: u32,
    pcm: *mut snd_pcm_substream,
) {
    let p = (*s).protocol;
    let cursor = desc;
    let mut pcm_frames: u32 = 0;
    let mut i: i32;

    if (*(*p).cache).rx_cycle_count == UINT_MAX {
        (*(*p).cache).rx_cycle_count =
            (*(*s).domain).processing_cycle.rx_start % CYCLES_PER_SECOND;
    }

    // For data block processing.
    i = 0;
    while (i as u32) < count {
        let buf = (*desc).ctx_payload;
        let data_blocks = (*desc).data_blocks;

        if !pcm.is_null() {
            write_pcm_s32(s, pcm, buf, data_blocks, pcm_frames);
            pcm_frames += data_blocks;
        } else {
            write_pcm_silence(s, buf, data_blocks);
        }

        if (*p).midi_ports != 0 {
            write_midi_messages(s, buf, data_blocks);
        }

        write_sph((*p).cache, buf, data_blocks, (*s).data_block_quadlets);

        desc = amdtp_stream_next_packet_desc(s, desc);
        i += 1;
    }

    desc = cursor;

    // For tracepoints.
    if trace_data_block_sph_enabled() || trace_data_block_message_enabled() {
        probe_tracepoints_events(s, desc, count);
    }
}

#[no_mangle]
pub unsafe extern "C" fn amdtp_motu_init(
    s: *mut amdtp_stream,
    unit: *mut fw_unit,
    dir: amdtp_stream_direction,
    spec: *const snd_motu_spec,
    cache: *mut amdtp_motu_cache,
) -> i32 {
    let process_ctx_payloads: amdtp_stream_process_ctx_payloads_t;
    let mut fmt: i32 = CIP_FMT_MOTU;
    let mut flags: u32 = CIP_BLOCKING | CIP_UNAWARE_SYT;
    let p: *mut amdtp_motu;
    let err: i32;

    if dir == AMDTP_IN_STREAM {
        process_ctx_payloads = Some(process_ir_ctx_payloads);

        /*
         * Units of version 3 transmits packets with invalid CIP header
         * against IEC 61883-1.
         */
        if (*spec).protocol_version == SND_MOTU_PROTOCOL_V3 {
            flags |= CIP_WRONG_DBS | CIP_SKIP_DBC_ZERO_CHECK | CIP_HEADER_WITHOUT_EOH;
            fmt = CIP_FMT_MOTU_TX_V3;
        }

        if ptr::eq(spec, &snd_motu_spec_8pre) || ptr::eq(spec, &snd_motu_spec_ultralite) {
            // 8pre has some quirks.
            flags |= CIP_WRONG_DBS | CIP_SKIP_DBC_ZERO_CHECK;
        }
    } else {
        process_ctx_payloads = Some(process_it_ctx_payloads);
        flags |= CIP_DBC_IS_END_EVENT;
    }

    err = amdtp_stream_init(
        s,
        unit,
        dir,
        flags,
        fmt,
        process_ctx_payloads,
        size_of::<amdtp_motu>(),
    );
    if err < 0 {
        return err;
    }

    (*s).sph = 1;

    if dir == AMDTP_OUT_STREAM {
        // Use fixed value for FDF field.
        (*s).ctx_data.rx.fdf = MOTU_FDF_AM824;
    }

    p = (*s).protocol;
    (*p).cache = cache;

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
