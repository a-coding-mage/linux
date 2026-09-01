/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * motu.h - a part of driver for MOTU FireWire series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// C header dependencies:
// linux/device.h, linux/firewire.h, linux/firewire-constants.h, linux/module.h,
// linux/mutex.h, linux/slab.h, linux/compat.h, linux/sched/signal.h,
// sound/control.h, sound/core.h, sound/pcm.h, sound/info.h,
// sound/rawmidi.h, sound/firewire.h, sound/hwdep.h,
// ../lib.h, ../amdtp-stream.h, ../iso-resources.h

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct snd_motu_packet_format {
    pub midi_flag_offset: u8,
    pub midi_byte_offset: u8,
    pub pcm_byte_offset: u8,

    pub msg_chunks: u8,
    pub pcm_chunks: [u8; 3],
}

#[repr(C)]
pub struct amdtp_motu_cache {
    pub event_offsets: *mut c_uint,
    pub size: c_uint,
    pub tail: c_uint,
    pub tx_cycle_count: c_uint,
    pub head: c_uint,
    pub rx_cycle_count: c_uint,
}

#[repr(C)]
pub struct snd_motu {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub mutex: mutex,
    pub lock: spinlock_t,

    /* Model dependent information. */
    pub spec: *const snd_motu_spec,

    /* For packet streaming */
    pub tx_packet_formats: snd_motu_packet_format,
    pub rx_packet_formats: snd_motu_packet_format,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub tx_resources: fw_iso_resources,
    pub rx_resources: fw_iso_resources,
    pub substreams_counter: c_uint,

    /* For notification. */
    pub async_handler: fw_address_handler,
    pub msg: u32,

    /* For uapi */
    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,
    pub hwdep: *mut snd_hwdep,

    pub domain: amdtp_domain,

    pub cache: amdtp_motu_cache,

    pub message_parser: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_motu_spec_flags {
    SND_MOTU_SPEC_RX_MIDI_2ND_Q = 0x0001,
    SND_MOTU_SPEC_RX_MIDI_3RD_Q = 0x0002,
    SND_MOTU_SPEC_TX_MIDI_2ND_Q = 0x0004,
    SND_MOTU_SPEC_TX_MIDI_3RD_Q = 0x0008,
    SND_MOTU_SPEC_REGISTER_DSP = 0x0010,
    SND_MOTU_SPEC_COMMAND_DSP = 0x0020,
}

pub const SND_MOTU_CLOCK_RATE_COUNT: usize = 6;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_motu_clock_source {
    SND_MOTU_CLOCK_SOURCE_INTERNAL,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_DSUB,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_A,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_B,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_A,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_B,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX,
    SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR,
    SND_MOTU_CLOCK_SOURCE_WORD_ON_BNC,
    SND_MOTU_CLOCK_SOURCE_SPH,
    SND_MOTU_CLOCK_SOURCE_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_motu_protocol_version {
    SND_MOTU_PROTOCOL_V1,
    SND_MOTU_PROTOCOL_V2,
    SND_MOTU_PROTOCOL_V3,
}

#[repr(C)]
pub struct snd_motu_spec {
    pub name: *const c_char,
    pub protocol_version: snd_motu_protocol_version,
    // The combination of snd_motu_spec_flags enumeration-constants.
    pub flags: c_uint,

    pub tx_fixed_pcm_chunks: [u8; 3],
    pub rx_fixed_pcm_chunks: [u8; 3],
}

unsafe extern "C" {
    pub static snd_motu_clock_rates: [c_uint; SND_MOTU_CLOCK_RATE_COUNT];

    pub static snd_motu_spec_828: snd_motu_spec;
    pub static snd_motu_spec_896: snd_motu_spec;

    pub static snd_motu_spec_828mk2: snd_motu_spec;
    pub static snd_motu_spec_896hd: snd_motu_spec;
    pub static snd_motu_spec_traveler: snd_motu_spec;
    pub static snd_motu_spec_ultralite: snd_motu_spec;
    pub static snd_motu_spec_8pre: snd_motu_spec;

    pub static snd_motu_spec_828mk3_fw: snd_motu_spec;
    pub static snd_motu_spec_828mk3_hybrid: snd_motu_spec;
    pub static snd_motu_spec_896mk3: snd_motu_spec;
    pub static snd_motu_spec_traveler_mk3: snd_motu_spec;
    pub static snd_motu_spec_ultralite_mk3: snd_motu_spec;
    pub static snd_motu_spec_audio_express: snd_motu_spec;
    pub static snd_motu_spec_track16: snd_motu_spec;
    pub static snd_motu_spec_4pre: snd_motu_spec;

    pub fn amdtp_motu_init(
        s: *mut amdtp_stream,
        unit: *mut fw_unit,
        dir: amdtp_stream_direction,
        spec: *const snd_motu_spec,
        cache: *mut amdtp_motu_cache,
    ) -> c_int;
    pub fn amdtp_motu_set_parameters(
        s: *mut amdtp_stream,
        rate: c_uint,
        midi_ports: c_uint,
        formats: *mut snd_motu_packet_format,
    ) -> c_int;
    pub fn amdtp_motu_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> c_int;
    pub fn amdtp_motu_midi_trigger(
        s: *mut amdtp_stream,
        port: c_uint,
        midi: *mut snd_rawmidi_substream,
    );

    pub fn snd_motu_transaction_read(
        motu: *mut snd_motu,
        offset: u32,
        reg: *mut __be32,
        size: size_t,
    ) -> c_int;
    pub fn snd_motu_transaction_write(
        motu: *mut snd_motu,
        offset: u32,
        reg: *mut __be32,
        size: size_t,
    ) -> c_int;
    pub fn snd_motu_transaction_register(motu: *mut snd_motu) -> c_int;
    pub fn snd_motu_transaction_reregister(motu: *mut snd_motu) -> c_int;
    pub fn snd_motu_transaction_unregister(motu: *mut snd_motu);

    pub fn snd_motu_stream_init_duplex(motu: *mut snd_motu) -> c_int;
    pub fn snd_motu_stream_destroy_duplex(motu: *mut snd_motu);
    pub fn snd_motu_stream_cache_packet_formats(motu: *mut snd_motu) -> c_int;
    pub fn snd_motu_stream_reserve_duplex(
        motu: *mut snd_motu,
        rate: c_uint,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    pub fn snd_motu_stream_start_duplex(motu: *mut snd_motu) -> c_int;
    pub fn snd_motu_stream_stop_duplex(motu: *mut snd_motu);
    pub fn snd_motu_stream_lock_try(motu: *mut snd_motu) -> c_int;
    pub fn snd_motu_stream_lock_release(motu: *mut snd_motu);

    pub fn snd_motu_proc_init(motu: *mut snd_motu);

    pub fn snd_motu_create_pcm_devices(motu: *mut snd_motu) -> c_int;

    pub fn snd_motu_create_midi_devices(motu: *mut snd_motu) -> c_int;

    pub fn snd_motu_create_hwdep_device(motu: *mut snd_motu) -> c_int;

    pub fn snd_motu_protocol_v1_get_clock_rate(motu: *mut snd_motu, rate: *mut c_uint) -> c_int;
    pub fn snd_motu_protocol_v1_set_clock_rate(motu: *mut snd_motu, rate: c_uint) -> c_int;
    pub fn snd_motu_protocol_v1_get_clock_source(
        motu: *mut snd_motu,
        src: *mut snd_motu_clock_source,
    ) -> c_int;
    pub fn snd_motu_protocol_v1_switch_fetching_mode(motu: *mut snd_motu, enable: bool) -> c_int;
    pub fn snd_motu_protocol_v1_cache_packet_formats(motu: *mut snd_motu) -> c_int;

    pub fn snd_motu_protocol_v2_get_clock_rate(motu: *mut snd_motu, rate: *mut c_uint) -> c_int;
    pub fn snd_motu_protocol_v2_set_clock_rate(motu: *mut snd_motu, rate: c_uint) -> c_int;
    pub fn snd_motu_protocol_v2_get_clock_source(
        motu: *mut snd_motu,
        src: *mut snd_motu_clock_source,
    ) -> c_int;
    pub fn snd_motu_protocol_v2_switch_fetching_mode(motu: *mut snd_motu, enable: bool) -> c_int;
    pub fn snd_motu_protocol_v2_cache_packet_formats(motu: *mut snd_motu) -> c_int;

    pub fn snd_motu_protocol_v3_get_clock_rate(motu: *mut snd_motu, rate: *mut c_uint) -> c_int;
    pub fn snd_motu_protocol_v3_set_clock_rate(motu: *mut snd_motu, rate: c_uint) -> c_int;
    pub fn snd_motu_protocol_v3_get_clock_source(
        motu: *mut snd_motu,
        src: *mut snd_motu_clock_source,
    ) -> c_int;
    pub fn snd_motu_protocol_v3_switch_fetching_mode(motu: *mut snd_motu, enable: bool) -> c_int;
    pub fn snd_motu_protocol_v3_cache_packet_formats(motu: *mut snd_motu) -> c_int;
}

#[inline]
pub unsafe fn snd_motu_protocol_get_clock_rate(motu: *mut snd_motu, rate: *mut c_uint) -> c_int {
    if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V2 {
        unsafe { snd_motu_protocol_v2_get_clock_rate(motu, rate) }
    } else if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V3 {
        unsafe { snd_motu_protocol_v3_get_clock_rate(motu, rate) }
    } else if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V1 {
        unsafe { snd_motu_protocol_v1_get_clock_rate(motu, rate) }
    } else {
        -(ENXIO as c_int)
    }
}

#[inline]
pub unsafe fn snd_motu_protocol_set_clock_rate(motu: *mut snd_motu, rate: c_uint) -> c_int {
    if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V2 {
        unsafe { snd_motu_protocol_v2_set_clock_rate(motu, rate) }
    } else if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V3 {
        unsafe { snd_motu_protocol_v3_set_clock_rate(motu, rate) }
    } else if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V1 {
        unsafe { snd_motu_protocol_v1_set_clock_rate(motu, rate) }
    } else {
        -(ENXIO as c_int)
    }
}

#[inline]
pub unsafe fn snd_motu_protocol_get_clock_source(
    motu: *mut snd_motu,
    source: *mut snd_motu_clock_source,
) -> c_int {
    if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V2 {
        unsafe { snd_motu_protocol_v2_get_clock_source(motu, source) }
    } else if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V3 {
        unsafe { snd_motu_protocol_v3_get_clock_source(motu, source) }
    } else if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V1 {
        unsafe { snd_motu_protocol_v1_get_clock_source(motu, source) }
    } else {
        -(ENXIO as c_int)
    }
}

#[inline]
pub unsafe fn snd_motu_protocol_switch_fetching_mode(
    motu: *mut snd_motu,
    enable: bool,
) -> c_int {
    if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V2 {
        unsafe { snd_motu_protocol_v2_switch_fetching_mode(motu, enable) }
    } else if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V3 {
        unsafe { snd_motu_protocol_v3_switch_fetching_mode(motu, enable) }
    } else if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V1 {
        unsafe { snd_motu_protocol_v1_switch_fetching_mode(motu, enable) }
    } else {
        -(ENXIO as c_int)
    }
}

#[inline]
pub unsafe fn snd_motu_protocol_cache_packet_formats(motu: *mut snd_motu) -> c_int {
    if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V2 {
        unsafe { snd_motu_protocol_v2_cache_packet_formats(motu) }
    } else if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V3 {
        unsafe { snd_motu_protocol_v3_cache_packet_formats(motu) }
    } else if (*(*motu).spec).protocol_version == snd_motu_protocol_version::SND_MOTU_PROTOCOL_V1 {
        unsafe { snd_motu_protocol_v1_cache_packet_formats(motu) }
    } else {
        -(ENXIO as c_int)
    }
}

unsafe extern "C" {
    pub fn snd_motu_register_dsp_message_parser_new(motu: *mut snd_motu) -> c_int;
    pub fn snd_motu_register_dsp_message_parser_init(motu: *mut snd_motu) -> c_int;
    pub fn snd_motu_register_dsp_message_parser_parse(
        s: *const amdtp_stream,
        descs: *const pkt_desc,
        count: c_uint,
    );
    pub fn snd_motu_register_dsp_message_parser_copy_meter(
        motu: *mut snd_motu,
        meter: *mut snd_firewire_motu_register_dsp_meter,
    );
    pub fn snd_motu_register_dsp_message_parser_copy_parameter(
        motu: *mut snd_motu,
        params: *mut snd_firewire_motu_register_dsp_parameter,
    );
    pub fn snd_motu_register_dsp_message_parser_count_event(motu: *mut snd_motu) -> c_uint;
    pub fn snd_motu_register_dsp_message_parser_copy_event(
        motu: *mut snd_motu,
        event: *mut u32,
    ) -> bool;

    pub fn snd_motu_command_dsp_message_parser_new(motu: *mut snd_motu) -> c_int;
    pub fn snd_motu_command_dsp_message_parser_init(motu: *mut snd_motu, sfc: cip_sfc) -> c_int;
    pub fn snd_motu_command_dsp_message_parser_parse(
        s: *const amdtp_stream,
        descs: *const pkt_desc,
        count: c_uint,
    );
    pub fn snd_motu_command_dsp_message_parser_copy_meter(
        motu: *mut snd_motu,
        meter: *mut snd_firewire_motu_command_dsp_meter,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
