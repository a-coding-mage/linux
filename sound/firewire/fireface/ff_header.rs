/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ff.h - a part of driver for RME Fireface series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto
 */

// C header dependencies:
// <linux/device.h>, <linux/firewire.h>, <linux/firewire-constants.h>,
// <linux/module.h>, <linux/mutex.h>, <linux/slab.h>, <linux/compat.h>,
// <linux/sched/signal.h>, <sound/core.h>, <sound/info.h>,
// <sound/rawmidi.h>, <sound/pcm.h>, <sound/pcm_params.h>,
// <sound/hwdep.h>, <sound/firewire.h>, "../lib.h",
// "../amdtp-stream.h", and "../iso-resources.h".

pub type size_t = usize;
pub type u64 = u64;
pub type u32 = u32;
pub type u8 = u8;
pub type __le32 = u32;
pub type ktime_t = i64;
pub type spinlock_t = core::ffi::c_void;
pub type wait_queue_head_t = core::ffi::c_void;

#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_address_handler {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_transaction {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdtp_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_iso_resources {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdtp_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cip_sfc {
    // Supplied by "../lib.h".
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdtp_stream_direction {
    // Supplied by "../amdtp-stream.h".
}

pub const SND_FF_MAXIMIM_MIDI_QUADS: usize = 9;
pub const SND_FF_IN_MIDI_PORTS: usize = 2;
pub const SND_FF_OUT_MIDI_PORTS: usize = 2;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_ff_unit_version {
    SND_FF_UNIT_VERSION_FF800 = 0x000001,
    SND_FF_UNIT_VERSION_FF400 = 0x000002,
    SND_FF_UNIT_VERSION_UFX = 0x000003,
    SND_FF_UNIT_VERSION_UCX = 0x000004,
    SND_FF_UNIT_VERSION_802 = 0x000005,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_ff_stream_mode {
    SND_FF_STREAM_MODE_LOW = 0,
    SND_FF_STREAM_MODE_MID,
    SND_FF_STREAM_MODE_HIGH,
    SND_FF_STREAM_MODE_COUNT,
}

pub const SND_FF_STREAM_MODE_COUNT: usize =
    snd_ff_stream_mode::SND_FF_STREAM_MODE_COUNT as usize;

#[repr(C)]
pub struct snd_ff_spec {
    pub pcm_capture_channels: [core::ffi::c_uint; SND_FF_STREAM_MODE_COUNT],
    pub pcm_playback_channels: [core::ffi::c_uint; SND_FF_STREAM_MODE_COUNT],

    pub midi_in_ports: core::ffi::c_uint,
    pub midi_out_ports: core::ffi::c_uint,

    pub protocol: *const snd_ff_protocol,
    pub midi_high_addr: u64,
    pub midi_addr_range: u8,
    pub midi_rx_addrs: [u64; SND_FF_OUT_MIDI_PORTS],
}

#[repr(C)]
pub struct snd_ff {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub mutex: mutex,
    pub lock: spinlock_t,

    pub unit_version: snd_ff_unit_version,
    pub spec: *const snd_ff_spec,

    /* To handle MIDI tx. */
    pub tx_midi_substreams: [*mut snd_rawmidi_substream; SND_FF_IN_MIDI_PORTS],
    pub async_handler: fw_address_handler,

    /* TO handle MIDI rx. */
    pub rx_midi_substreams: [*mut snd_rawmidi_substream; SND_FF_OUT_MIDI_PORTS],
    pub on_sysex: [bool; SND_FF_OUT_MIDI_PORTS],
    pub msg_buf: [[__le32; SND_FF_MAXIMIM_MIDI_QUADS]; SND_FF_OUT_MIDI_PORTS],
    pub rx_midi_work: [work_struct; SND_FF_OUT_MIDI_PORTS],
    pub transactions: [fw_transaction; SND_FF_OUT_MIDI_PORTS],
    pub next_ktime: [ktime_t; SND_FF_OUT_MIDI_PORTS],
    pub rx_midi_error: [bool; SND_FF_OUT_MIDI_PORTS],
    pub rx_bytes: [core::ffi::c_uint; SND_FF_OUT_MIDI_PORTS],

    pub substreams_counter: core::ffi::c_uint,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub tx_resources: fw_iso_resources,
    pub rx_resources: fw_iso_resources,

    pub dev_lock_count: core::ffi::c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,

    pub domain: amdtp_domain,

    pub msg_parser: *mut core::ffi::c_void,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_ff_clock_src {
    SND_FF_CLOCK_SRC_INTERNAL,
    SND_FF_CLOCK_SRC_SPDIF,
    SND_FF_CLOCK_SRC_ADAT1,
    SND_FF_CLOCK_SRC_ADAT2,
    SND_FF_CLOCK_SRC_WORD,
    SND_FF_CLOCK_SRC_LTC,
    /* TODO: perhaps TCO exists. */
}

#[repr(C)]
pub struct snd_ff_protocol {
    pub msg_parser_size: size_t,
    pub has_msg: Option<unsafe extern "C" fn(ff: *mut snd_ff) -> bool>,
    pub copy_msg_to_user: Option<
        unsafe extern "C" fn(
            ff: *mut snd_ff,
            buf: *mut core::ffi::c_char,
            count: core::ffi::c_long,
        ) -> core::ffi::c_long,
    >,
    pub handle_msg: Option<
        unsafe extern "C" fn(
            ff: *mut snd_ff,
            offset: core::ffi::c_uint,
            buf: *const __le32,
            length: size_t,
            tstamp: u32,
        ),
    >,
    pub fill_midi_msg: Option<
        unsafe extern "C" fn(
            ff: *mut snd_ff,
            substream: *mut snd_rawmidi_substream,
            port: core::ffi::c_uint,
        ) -> core::ffi::c_int,
    >,
    pub get_clock: Option<
        unsafe extern "C" fn(
            ff: *mut snd_ff,
            rate: *mut core::ffi::c_uint,
            src: *mut snd_ff_clock_src,
        ) -> core::ffi::c_int,
    >,
    pub switch_fetching_mode:
        Option<unsafe extern "C" fn(ff: *mut snd_ff, enable: bool) -> core::ffi::c_int>,
    pub allocate_resources:
        Option<unsafe extern "C" fn(ff: *mut snd_ff, rate: core::ffi::c_uint) -> core::ffi::c_int>,
    pub begin_session:
        Option<unsafe extern "C" fn(ff: *mut snd_ff, rate: core::ffi::c_uint) -> core::ffi::c_int>,
    pub finish_session: Option<unsafe extern "C" fn(ff: *mut snd_ff)>,
    pub dump_status:
        Option<unsafe extern "C" fn(ff: *mut snd_ff, buffer: *mut snd_info_buffer)>,
}

unsafe extern "C" {
    pub static snd_ff_protocol_ff800: snd_ff_protocol;
    pub static snd_ff_protocol_ff400: snd_ff_protocol;
    pub static snd_ff_protocol_latter: snd_ff_protocol;

    pub fn snd_ff_transaction_register(ff: *mut snd_ff) -> core::ffi::c_int;
    pub fn snd_ff_transaction_reregister(ff: *mut snd_ff) -> core::ffi::c_int;
    pub fn snd_ff_transaction_unregister(ff: *mut snd_ff);

    pub fn amdtp_ff_set_parameters(
        s: *mut amdtp_stream,
        rate: core::ffi::c_uint,
        pcm_channels: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn amdtp_ff_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> core::ffi::c_int;
    pub fn amdtp_ff_init(
        s: *mut amdtp_stream,
        unit: *mut fw_unit,
        dir: amdtp_stream_direction,
    ) -> core::ffi::c_int;

    pub fn snd_ff_stream_get_multiplier_mode(
        sfc: cip_sfc,
        mode: *mut snd_ff_stream_mode,
    ) -> core::ffi::c_int;
    pub fn snd_ff_stream_init_duplex(ff: *mut snd_ff) -> core::ffi::c_int;
    pub fn snd_ff_stream_destroy_duplex(ff: *mut snd_ff);
    pub fn snd_ff_stream_reserve_duplex(
        ff: *mut snd_ff,
        rate: core::ffi::c_uint,
        frames_per_period: core::ffi::c_uint,
        frames_per_buffer: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn snd_ff_stream_start_duplex(
        ff: *mut snd_ff,
        rate: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn snd_ff_stream_stop_duplex(ff: *mut snd_ff);
    pub fn snd_ff_stream_update_duplex(ff: *mut snd_ff);

    pub fn snd_ff_stream_lock_changed(ff: *mut snd_ff);
    pub fn snd_ff_stream_lock_try(ff: *mut snd_ff) -> core::ffi::c_int;
    pub fn snd_ff_stream_lock_release(ff: *mut snd_ff);

    pub fn snd_ff_proc_init(ff: *mut snd_ff);
    pub fn snd_ff_proc_get_clk_label(src: snd_ff_clock_src) -> *const core::ffi::c_char;

    pub fn snd_ff_create_midi_devices(ff: *mut snd_ff) -> core::ffi::c_int;

    pub fn snd_ff_create_pcm_devices(ff: *mut snd_ff) -> core::ffi::c_int;

    pub fn snd_ff_create_hwdep_devices(ff: *mut snd_ff) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
