/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * digi00x.h - a part of driver for Digidesign Digi 002/003 family
 *
 * Copyright (c) 2014-2015 Takashi Sakamoto
 */

// C header guard and include directives are omitted in Rust. The original
// header depends on Linux, ALSA, FireWire, and local firewire helper types.

#[repr(C)]
pub struct snd_dg00x {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,

    pub mutex: mutex,
    pub lock: spinlock_t,

    pub tx_stream: amdtp_stream,
    pub tx_resources: fw_iso_resources,

    pub rx_stream: amdtp_stream,
    pub rx_resources: fw_iso_resources,

    pub substreams_counter: ::core::ffi::c_uint,

    /* for uapi */
    pub dev_lock_count: ::core::ffi::c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,

    /* For asynchronous messages. */
    pub async_handler: fw_address_handler,
    pub msg: u32,

    /* Console models have additional MIDI ports for control surface. */
    pub is_console: bool,

    pub domain: amdtp_domain,
}

pub const DG00X_ADDR_BASE: u64 = 0xffffe0000000u64;

pub const DG00X_OFFSET_STREAMING_STATE: u32 = 0x0000;
pub const DG00X_OFFSET_STREAMING_SET: u32 = 0x0004;
/* unknown but address in host space      0x0008 */
/* For LSB of the address                 0x000c */
/* unknown                                0x0010 */
pub const DG00X_OFFSET_MESSAGE_ADDR: u32 = 0x0014;
/* For LSB of the address                 0x0018 */
/* unknown                                0x001c */
/* unknown                                0x0020 */
/* not used                       0x0024--0x00ff */
pub const DG00X_OFFSET_ISOC_CHANNELS: u32 = 0x0100;
/* unknown                                0x0104 */
/* unknown                                0x0108 */
/* unknown                                0x010c */
pub const DG00X_OFFSET_LOCAL_RATE: u32 = 0x0110;
pub const DG00X_OFFSET_EXTERNAL_RATE: u32 = 0x0114;
pub const DG00X_OFFSET_CLOCK_SOURCE: u32 = 0x0118;
pub const DG00X_OFFSET_OPT_IFACE_MODE: u32 = 0x011c;
/* unknown                                0x0120 */
/* Mixer control on/off                   0x0124 */
/* unknown                                0x0128 */
pub const DG00X_OFFSET_DETECT_EXTERNAL: u32 = 0x012c;
/* unknown                                0x0138 */
pub const DG00X_OFFSET_MMC: u32 = 0x0400;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_dg00x_rate {
    SND_DG00X_RATE_44100 = 0,
    SND_DG00X_RATE_48000 = 1,
    SND_DG00X_RATE_88200 = 2,
    SND_DG00X_RATE_96000 = 3,
    SND_DG00X_RATE_COUNT = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_dg00x_clock {
    SND_DG00X_CLOCK_INTERNAL = 0,
    SND_DG00X_CLOCK_SPDIF = 1,
    SND_DG00X_CLOCK_ADAT = 2,
    SND_DG00X_CLOCK_WORD = 3,
    SND_DG00X_CLOCK_COUNT = 4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_dg00x_optical_mode {
    SND_DG00X_OPT_IFACE_MODE_ADAT = 0,
    SND_DG00X_OPT_IFACE_MODE_SPDIF = 1,
    SND_DG00X_OPT_IFACE_MODE_COUNT = 2,
}

pub const DOT_MIDI_IN_PORTS: u32 = 1;
pub const DOT_MIDI_OUT_PORTS: u32 = 2;

unsafe extern "C" {
    pub fn amdtp_dot_init(
        s: *mut amdtp_stream,
        unit: *mut fw_unit,
        dir: amdtp_stream_direction,
    ) -> ::core::ffi::c_int;
    pub fn amdtp_dot_set_parameters(
        s: *mut amdtp_stream,
        rate: ::core::ffi::c_uint,
        pcm_channels: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn amdtp_dot_reset(s: *mut amdtp_stream);
    pub fn amdtp_dot_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> ::core::ffi::c_int;
    pub fn amdtp_dot_midi_trigger(
        s: *mut amdtp_stream,
        port: ::core::ffi::c_uint,
        midi: *mut snd_rawmidi_substream,
    );

    pub fn snd_dg00x_transaction_register(dg00x: *mut snd_dg00x) -> ::core::ffi::c_int;
    pub fn snd_dg00x_transaction_reregister(dg00x: *mut snd_dg00x) -> ::core::ffi::c_int;
    pub fn snd_dg00x_transaction_unregister(dg00x: *mut snd_dg00x);

    pub static snd_dg00x_stream_rates:
        [::core::ffi::c_uint; snd_dg00x_rate::SND_DG00X_RATE_COUNT as usize];
    pub static snd_dg00x_stream_pcm_channels:
        [::core::ffi::c_uint; snd_dg00x_rate::SND_DG00X_RATE_COUNT as usize];
    pub fn snd_dg00x_stream_get_external_rate(
        dg00x: *mut snd_dg00x,
        rate: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn snd_dg00x_stream_get_local_rate(
        dg00x: *mut snd_dg00x,
        rate: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn snd_dg00x_stream_set_local_rate(
        dg00x: *mut snd_dg00x,
        rate: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn snd_dg00x_stream_get_clock(
        dg00x: *mut snd_dg00x,
        clock: *mut snd_dg00x_clock,
    ) -> ::core::ffi::c_int;
    pub fn snd_dg00x_stream_check_external_clock(
        dg00x: *mut snd_dg00x,
        detect: *mut bool,
    ) -> ::core::ffi::c_int;
    pub fn snd_dg00x_stream_init_duplex(dg00x: *mut snd_dg00x) -> ::core::ffi::c_int;
    pub fn snd_dg00x_stream_reserve_duplex(
        dg00x: *mut snd_dg00x,
        rate: ::core::ffi::c_uint,
        frames_per_period: ::core::ffi::c_uint,
        frames_per_buffer: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn snd_dg00x_stream_start_duplex(dg00x: *mut snd_dg00x) -> ::core::ffi::c_int;
    pub fn snd_dg00x_stream_stop_duplex(dg00x: *mut snd_dg00x);
    pub fn snd_dg00x_stream_update_duplex(dg00x: *mut snd_dg00x);
    pub fn snd_dg00x_stream_destroy_duplex(dg00x: *mut snd_dg00x);

    pub fn snd_dg00x_stream_lock_changed(dg00x: *mut snd_dg00x);
    pub fn snd_dg00x_stream_lock_try(dg00x: *mut snd_dg00x) -> ::core::ffi::c_int;
    pub fn snd_dg00x_stream_lock_release(dg00x: *mut snd_dg00x);

    pub fn snd_dg00x_proc_init(dg00x: *mut snd_dg00x);

    pub fn snd_dg00x_create_pcm_devices(dg00x: *mut snd_dg00x) -> ::core::ffi::c_int;

    pub fn snd_dg00x_create_midi_devices(dg00x: *mut snd_dg00x) -> ::core::ffi::c_int;

    pub fn snd_dg00x_create_hwdep_device(dg00x: *mut snd_dg00x) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
