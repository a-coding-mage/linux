// SPDX-License-Identifier: GPL-2.0-only
/*
 * AM824 format in Audio and Music Data Transmission Protocol (IEC 61883-6)
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 * Copyright (c) 2015 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// Dependencies from linux/slab.h and "amdtp-am824.h" are supplied externally.

pub type u8 = ::core::ffi::c_uchar;
pub type u32 = ::core::ffi::c_uint;
pub type __be32 = u32;

const CIP_FMT_AM: ::core::ffi::c_uint = 0x10;

/* "Clock-based rate control mode" is just supported. */
const AMDTP_FDF_AM824: ::core::ffi::c_uint = 0x00;

/*
 * Nominally 3125 bytes/second, but the MIDI port's clock might be
 * 1% too slow, and the bus clock 100 ppm too fast.
 */
const MIDI_BYTES_PER_SECOND: ::core::ffi::c_int = 3093;

/*
 * Several devices look only at the first eight data blocks.
 * In any case, this is more than enough for the MIDI data rate.
 */
const MAX_MIDI_RX_BLOCKS: ::core::ffi::c_uint = 8;

extern "C" {
    static amdtp_rate_table: [::core::ffi::c_uint; 0];

    fn amdtp_stream_running(s: *mut amdtp_stream) -> bool;
    fn amdtp_stream_set_parameters(
        s: *mut amdtp_stream,
        rate: ::core::ffi::c_uint,
        data_block_quadlets: ::core::ffi::c_uint,
        pcm_frame_multiplier: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn amdtp_stream_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> ::core::ffi::c_int;
    fn snd_pcm_hw_constraint_msbits(
        runtime: *mut snd_pcm_runtime,
        cond: ::core::ffi::c_uint,
        width: ::core::ffi::c_uint,
        msbits: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn snd_rawmidi_transmit(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut u8,
        count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn snd_rawmidi_receive(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut u8,
        count: ::core::ffi::c_int,
    );
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
    fn frames_to_bytes(
        runtime: *mut snd_pcm_runtime,
        frames: ::core::ffi::c_uint,
    ) -> usize;
}

// External constants from included headers.
extern "C" {
    static AM824_MAX_CHANNELS_FOR_MIDI: usize;
    static AM824_MAX_CHANNELS_FOR_PCM: usize;
    static AMDTP_OUT_STREAM: amdtp_stream_direction;
    static AMDTP_IN_STREAM: amdtp_stream_direction;
    static CIP_UNALIGHED_DBC: ::core::ffi::c_uint;
}

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
    pub buffer_size: ::core::ffi::c_uint,
    pub dma_area: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

pub type amdtp_stream_direction = ::core::ffi::c_uint;

#[repr(C)]
pub struct amdtp_stream_rx {
    pub fdf: ::core::ffi::c_uint,
}

#[repr(C)]
pub union amdtp_stream_ctx_data {
    pub rx: ::core::mem::ManuallyDrop<amdtp_stream_rx>,
}

#[repr(C)]
pub struct amdtp_stream {
    pub protocol: *mut ::core::ffi::c_void,
    pub direction: amdtp_stream_direction,
    pub ctx_data: amdtp_stream_ctx_data,
    pub sfc: ::core::ffi::c_uint,
    pub syt_interval: ::core::ffi::c_uint,
    pub pcm_buffer_pointer: ::core::ffi::c_uint,
    pub data_block_quadlets: ::core::ffi::c_uint,
    pub pcm_frame_multiplier: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_uint,
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

#[repr(C)]
pub struct amdtp_am824 {
    pub midi: [*mut snd_rawmidi_substream; 8 * 8],
    pub midi_fifo_limit: ::core::ffi::c_int,
    pub midi_fifo_used: [::core::ffi::c_int; 8 * 8],
    pub pcm_channels: ::core::ffi::c_uint,
    pub midi_ports: ::core::ffi::c_uint,

    pub pcm_positions: [u8; 64],
    pub midi_position: u8,
}

const EINVAL: ::core::ffi::c_int = 22;

#[inline]
unsafe fn div_round_up(n: ::core::ffi::c_uint, d: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    (n.wrapping_add(d).wrapping_sub(1)) / d
}

#[inline]
unsafe fn warn_on(condition: bool) -> bool {
    condition
}

#[inline]
unsafe fn cpu_to_be32(v: u32) -> __be32 {
    v.to_be()
}

#[inline]
unsafe fn be32_to_cpu(v: __be32) -> u32 {
    u32::from_be(v)
}

/**
 * amdtp_am824_set_parameters - set stream parameters
 * @s: the AMDTP stream to configure
 * @rate: the sample rate
 * @pcm_channels: the number of PCM samples in each data block, to be encoded
 *                as AM824 multi-bit linear audio
 * @midi_ports: the number of MIDI ports (i.e., MPX-MIDI Data Channels)
 * @double_pcm_frames: one data block transfers two PCM frames
 *
 * The parameters must be set before the stream is started, and must not be
 * changed while the stream is running.
 */
#[no_mangle]
pub unsafe extern "C" fn amdtp_am824_set_parameters(
    s: *mut amdtp_stream,
    rate: ::core::ffi::c_uint,
    pcm_channels: ::core::ffi::c_uint,
    midi_ports: ::core::ffi::c_uint,
    double_pcm_frames: bool,
) -> ::core::ffi::c_int {
    let p = (*s).protocol as *mut amdtp_am824;
    let midi_channels: ::core::ffi::c_uint;
    let pcm_frame_multiplier: ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_int;
    let err: ::core::ffi::c_int;

    if amdtp_stream_running(s) {
        return -EINVAL;
    }

    if pcm_channels > AM824_MAX_CHANNELS_FOR_PCM as ::core::ffi::c_uint {
        return -EINVAL;
    }

    midi_channels = div_round_up(midi_ports, 8);
    if midi_channels > AM824_MAX_CHANNELS_FOR_MIDI as ::core::ffi::c_uint {
        return -EINVAL;
    }

    if warn_on(amdtp_stream_running(s))
        || warn_on(pcm_channels > AM824_MAX_CHANNELS_FOR_PCM as ::core::ffi::c_uint)
        || warn_on(midi_channels > AM824_MAX_CHANNELS_FOR_MIDI as ::core::ffi::c_uint)
    {
        return -EINVAL;
    }

    /*
     * In IEC 61883-6, one data block represents one event. In ALSA, one
     * event equals to one PCM frame. But Dice has a quirk at higher
     * sampling rate to transfer two PCM frames in one data block.
     */
    if double_pcm_frames {
        pcm_frame_multiplier = 2;
    } else {
        pcm_frame_multiplier = 1;
    }

    err = amdtp_stream_set_parameters(
        s,
        rate,
        pcm_channels.wrapping_add(midi_channels),
        pcm_frame_multiplier,
    );
    if err < 0 {
        return err;
    }

    if (*s).direction == AMDTP_OUT_STREAM {
        (*s).ctx_data.rx.fdf = AMDTP_FDF_AM824 | (*s).sfc;
    }

    (*p).pcm_channels = pcm_channels;
    (*p).midi_ports = midi_ports;

    /* init the position map for PCM and MIDI channels */
    i = 0;
    while i < pcm_channels as ::core::ffi::c_int {
        (*p).pcm_positions[i as usize] = i as u8;
        i += 1;
    }
    (*p).midi_position = (*p).pcm_channels as u8;

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

/**
 * amdtp_am824_set_pcm_position - set an index of data channel for a channel
 *				  of PCM frame
 * @s: the AMDTP stream
 * @index: the index of data channel in an data block
 * @position: the channel of PCM frame
 */
#[no_mangle]
pub unsafe extern "C" fn amdtp_am824_set_pcm_position(
    s: *mut amdtp_stream,
    index: ::core::ffi::c_uint,
    position: ::core::ffi::c_uint,
) {
    let p = (*s).protocol as *mut amdtp_am824;

    if index < (*p).pcm_channels {
        (*p).pcm_positions[index as usize] = position as u8;
    }
}

/**
 * amdtp_am824_set_midi_position - set a index of data channel for MIDI
 *				   conformant data channel
 * @s: the AMDTP stream
 * @position: the index of data channel in an data block
 */
#[no_mangle]
pub unsafe extern "C" fn amdtp_am824_set_midi_position(
    s: *mut amdtp_stream,
    position: ::core::ffi::c_uint,
) {
    let p = (*s).protocol as *mut amdtp_am824;

    (*p).midi_position = position as u8;
}

unsafe extern "C" fn write_pcm_s32(
    s: *mut amdtp_stream,
    pcm: *mut snd_pcm_substream,
    mut buffer: *mut __be32,
    frames: ::core::ffi::c_uint,
    pcm_frames: ::core::ffi::c_uint,
) {
    let p = (*s).protocol as *mut amdtp_am824;
    let channels = (*p).pcm_channels;
    let runtime = (*pcm).runtime;
    let mut pcm_buffer_pointer: ::core::ffi::c_uint;
    let mut remaining_frames: ::core::ffi::c_int;
    let mut src: *const u32;
    let mut i: ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int;

    pcm_buffer_pointer = (*s).pcm_buffer_pointer.wrapping_add(pcm_frames);
    pcm_buffer_pointer %= (*runtime).buffer_size;

    src = ((*runtime).dma_area as *mut u8).add(frames_to_bytes(runtime, pcm_buffer_pointer)) as *const u32;
    remaining_frames = ((*runtime).buffer_size - pcm_buffer_pointer) as ::core::ffi::c_int;

    i = 0;
    while i < frames as ::core::ffi::c_int {
        c = 0;
        while c < channels as ::core::ffi::c_int {
            *buffer.add((*p).pcm_positions[c as usize] as usize) =
                cpu_to_be32((*src >> 8) | 0x40000000);
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
    frames: ::core::ffi::c_uint,
    pcm_frames: ::core::ffi::c_uint,
) {
    let p = (*s).protocol as *mut amdtp_am824;
    let channels = (*p).pcm_channels;
    let runtime = (*pcm).runtime;
    let mut pcm_buffer_pointer: ::core::ffi::c_uint;
    let mut remaining_frames: ::core::ffi::c_int;
    let mut dst: *mut u32;
    let mut i: ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int;

    pcm_buffer_pointer = (*s).pcm_buffer_pointer.wrapping_add(pcm_frames);
    pcm_buffer_pointer %= (*runtime).buffer_size;

    dst = ((*runtime).dma_area as *mut u8).add(frames_to_bytes(runtime, pcm_buffer_pointer)) as *mut u32;
    remaining_frames = ((*runtime).buffer_size - pcm_buffer_pointer) as ::core::ffi::c_int;

    i = 0;
    while i < frames as ::core::ffi::c_int {
        c = 0;
        while c < channels as ::core::ffi::c_int {
            *dst = be32_to_cpu(*buffer.add((*p).pcm_positions[c as usize] as usize)) << 8;
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
    frames: ::core::ffi::c_uint,
) {
    let p = (*s).protocol as *mut amdtp_am824;
    let mut i: ::core::ffi::c_uint;
    let mut c: ::core::ffi::c_uint;
    let channels = (*p).pcm_channels;

    i = 0;
    while i < frames {
        c = 0;
        while c < channels {
            *buffer.add((*p).pcm_positions[c as usize] as usize) = cpu_to_be32(0x40000000);
            c += 1;
        }
        buffer = buffer.add((*s).data_block_quadlets as usize);
        i += 1;
    }
}

/**
 * amdtp_am824_add_pcm_hw_constraints - add hw constraints for PCM substream
 * @s:		the AMDTP stream for AM824 data block, must be initialized.
 * @runtime:	the PCM substream runtime
 *
 */
#[no_mangle]
pub unsafe extern "C" fn amdtp_am824_add_pcm_hw_constraints(
    s: *mut amdtp_stream,
    runtime: *mut snd_pcm_runtime,
) -> ::core::ffi::c_int {
    let err: ::core::ffi::c_int;

    err = amdtp_stream_add_pcm_hw_constraints(s, runtime);
    if err < 0 {
        return err;
    }

    /* AM824 in IEC 61883-6 can deliver 24bit data. */
    snd_pcm_hw_constraint_msbits(runtime, 0, 32, 24)
}

/**
 * amdtp_am824_midi_trigger - start/stop playback/capture with a MIDI device
 * @s: the AMDTP stream
 * @port: index of MIDI port
 * @midi: the MIDI device to be started, or %NULL to stop the current device
 *
 * Call this function on a running isochronous stream to enable the actual
 * transmission of MIDI data.  This function should be called from the MIDI
 * device's .trigger callback.
 */
#[no_mangle]
pub unsafe extern "C" fn amdtp_am824_midi_trigger(
    s: *mut amdtp_stream,
    port: ::core::ffi::c_uint,
    midi: *mut snd_rawmidi_substream,
) {
    let p = (*s).protocol as *mut amdtp_am824;

    if port < (*p).midi_ports {
        ::core::ptr::write_volatile(&mut (*p).midi[port as usize], midi);
    }
}

/*
 * To avoid sending MIDI bytes at too high a rate, assume that the receiving
 * device has a FIFO, and track how much it is filled.  This values increases
 * by one whenever we send one byte in a packet, but the FIFO empties at
 * a constant rate independent of our packet rate.  One packet has syt_interval
 * samples, so the number of bytes that empty out of the FIFO, per packet(!),
 * is MIDI_BYTES_PER_SECOND * syt_interval / sample_rate.  To avoid storing
 * fractional values, the values in midi_fifo_used[] are measured in bytes
 * multiplied by the sample rate.
 */
unsafe extern "C" fn midi_ratelimit_per_packet(
    s: *mut amdtp_stream,
    port: ::core::ffi::c_uint,
) -> bool {
    let p = (*s).protocol as *mut amdtp_am824;
    let mut used: ::core::ffi::c_int;

    used = (*p).midi_fifo_used[port as usize];
    if used == 0 {
        /* common shortcut */
        return true;
    }

    used -= MIDI_BYTES_PER_SECOND * (*s).syt_interval as ::core::ffi::c_int;
    used = if used > 0 { used } else { 0 };
    (*p).midi_fifo_used[port as usize] = used;

    used < (*p).midi_fifo_limit
}

unsafe extern "C" fn midi_rate_use_one_byte(s: *mut amdtp_stream, port: ::core::ffi::c_uint) {
    let p = (*s).protocol as *mut amdtp_am824;

    (*p).midi_fifo_used[port as usize] += amdtp_rate_table[(*s).sfc as usize] as ::core::ffi::c_int;
}

unsafe extern "C" fn write_midi_messages(
    s: *mut amdtp_stream,
    mut buffer: *mut __be32,
    frames: ::core::ffi::c_uint,
    data_block_counter: ::core::ffi::c_uint,
) {
    let p = (*s).protocol as *mut amdtp_am824;
    let mut f: ::core::ffi::c_uint;
    let mut port: ::core::ffi::c_uint;
    let mut b: *mut u8;

    f = 0;
    while f < frames {
        b = buffer.add((*p).midi_position as usize) as *mut u8;

        port = data_block_counter.wrapping_add(f) % 8;
        if f < MAX_MIDI_RX_BLOCKS
            && midi_ratelimit_per_packet(s, port)
            && !(*p).midi[port as usize].is_null()
            && snd_rawmidi_transmit((*p).midi[port as usize], b.add(1), 1) == 1
        {
            midi_rate_use_one_byte(s, port);
            *b.add(0) = 0x81;
        } else {
            *b.add(0) = 0x80;
            *b.add(1) = 0;
        }
        *b.add(2) = 0;
        *b.add(3) = 0;

        buffer = buffer.add((*s).data_block_quadlets as usize);
        f += 1;
    }
}

unsafe extern "C" fn read_midi_messages(
    s: *mut amdtp_stream,
    mut buffer: *mut __be32,
    frames: ::core::ffi::c_uint,
    data_block_counter: ::core::ffi::c_uint,
) {
    let p = (*s).protocol as *mut amdtp_am824;
    let mut len: ::core::ffi::c_int;
    let mut b: *mut u8;
    let mut f: ::core::ffi::c_int;

    f = 0;
    while f < frames as ::core::ffi::c_int {
        let mut port: ::core::ffi::c_uint = f as ::core::ffi::c_uint;

        if !(((*s).flags & CIP_UNALIGHED_DBC) != 0) {
            port = port.wrapping_add(data_block_counter);
        }
        port %= 8;
        b = buffer.add((*p).midi_position as usize) as *mut u8;

        len = *b.add(0) as ::core::ffi::c_int - 0x80;
        if (1 <= len) && (len <= 3) && !(*p).midi[port as usize].is_null() {
            snd_rawmidi_receive((*p).midi[port as usize], b.add(1), len);
        }

        buffer = buffer.add((*s).data_block_quadlets as usize);
        f += 1;
    }
}

unsafe extern "C" fn process_it_ctx_payloads(
    s: *mut amdtp_stream,
    mut desc: *const pkt_desc,
    count: ::core::ffi::c_uint,
    pcm: *mut snd_pcm_substream,
) {
    let p = (*s).protocol as *mut amdtp_am824;
    let mut pcm_frames: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_int;

    i = 0;
    while i < count as ::core::ffi::c_int {
        let buf: *mut __be32 = (*desc).ctx_payload;
        let data_blocks: ::core::ffi::c_uint = (*desc).data_blocks;

        if !pcm.is_null() {
            write_pcm_s32(s, pcm, buf, data_blocks, pcm_frames);
            pcm_frames = pcm_frames.wrapping_add(data_blocks.wrapping_mul((*s).pcm_frame_multiplier));
        } else {
            write_pcm_silence(s, buf, data_blocks);
        }

        if (*p).midi_ports != 0 {
            write_midi_messages(s, buf, data_blocks, (*desc).data_block_counter);
        }

        desc = amdtp_stream_next_packet_desc(s, desc);
        i += 1;
    }
}

unsafe extern "C" fn process_ir_ctx_payloads(
    s: *mut amdtp_stream,
    mut desc: *const pkt_desc,
    count: ::core::ffi::c_uint,
    pcm: *mut snd_pcm_substream,
) {
    let p = (*s).protocol as *mut amdtp_am824;
    let mut pcm_frames: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_int;

    i = 0;
    while i < count as ::core::ffi::c_int {
        let buf: *mut __be32 = (*desc).ctx_payload;
        let data_blocks: ::core::ffi::c_uint = (*desc).data_blocks;

        if !pcm.is_null() {
            read_pcm_s32(s, pcm, buf, data_blocks, pcm_frames);
            pcm_frames = pcm_frames.wrapping_add(data_blocks.wrapping_mul((*s).pcm_frame_multiplier));
        }

        if (*p).midi_ports != 0 {
            read_midi_messages(s, buf, data_blocks, (*desc).data_block_counter);
        }

        desc = amdtp_stream_next_packet_desc(s, desc);
        i += 1;
    }
}

/**
 * amdtp_am824_init - initialize an AMDTP stream structure to handle AM824
 *		      data block
 * @s: the AMDTP stream to initialize
 * @unit: the target of the stream
 * @dir: the direction of stream
 * @flags: the details of the streaming protocol consist of cip_flags enumeration-constants.
 */
#[no_mangle]
pub unsafe extern "C" fn amdtp_am824_init(
    s: *mut amdtp_stream,
    unit: *mut fw_unit,
    dir: amdtp_stream_direction,
    flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let process_ctx_payloads: amdtp_stream_process_ctx_payloads_t;

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
        ::core::mem::size_of::<amdtp_am824>(),
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
