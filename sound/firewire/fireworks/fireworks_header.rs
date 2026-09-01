// SPDX-License-Identifier: GPL-2.0-only
/*
 * fireworks.h - a part of driver for Fireworks based devices
 *
 * Copyright (c) 2009-2010 Clemens Ladisch
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Dependencies from the original C header include Linux firewire, ALSA, and
// local FireWire audio driver headers.
use core::ffi::{c_char, c_int, c_uint, c_void};

use crate::{
    amdtp_domain, amdtp_stream, cmp_connection, fw_unit, mutex, snd_card, spinlock_t,
    wait_queue_head_t,
};

pub const SND_EFW_MAX_MIDI_OUT_PORTS: c_uint = 2;
pub const SND_EFW_MAX_MIDI_IN_PORTS: c_uint = 2;

pub const SND_EFW_MULTIPLIER_MODES: usize = 3;
pub const HWINFO_NAME_SIZE_BYTES: usize = 32;
pub const HWINFO_MAX_CAPS_GROUPS: usize = 8;

/*
 * This should be greater than maximum bytes for EFW response content.
 * Currently response against command for isochronous channel mapping is
 * confirmed to be the maximum one. But for flexibility, use maximum data
 * payload for asynchronous primary packets at S100 (Cable base rate) in
 * IEEE Std 1394-1995.
 */
pub const SND_EFW_RESPONSE_MAXIMUM_BYTES: c_uint = 0x200;

unsafe extern "C" {
    pub static mut snd_efw_resp_buf_size: c_uint;
    pub static mut snd_efw_resp_buf_debug: bool;
}

#[repr(C, packed)]
pub struct snd_efw_phys_grp {
    pub type_: u8, /* see enum snd_efw_grp_type */
    pub count: u8,
}

#[repr(C)]
pub struct snd_efw {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub card_index: c_int,

    pub mutex: mutex,
    pub lock: spinlock_t,

    /* for transaction */
    pub seqnum: u32,
    pub resp_addr_changable: bool,

    /* for quirks */
    pub is_af9: bool,
    pub is_fireworks3: bool,
    pub firmware_version: u32,

    pub midi_in_ports: c_uint,
    pub midi_out_ports: c_uint,

    pub supported_sampling_rate: c_uint,
    pub pcm_capture_channels: [c_uint; SND_EFW_MULTIPLIER_MODES],
    pub pcm_playback_channels: [c_uint; SND_EFW_MULTIPLIER_MODES],

    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub out_conn: cmp_connection,
    pub in_conn: cmp_connection,
    pub substreams_counter: c_uint,

    /* hardware metering parameters */
    pub phys_out: c_uint,
    pub phys_in: c_uint,
    pub phys_out_grp_count: c_uint,
    pub phys_in_grp_count: c_uint,
    pub phys_out_grps: [snd_efw_phys_grp; HWINFO_MAX_CAPS_GROUPS],
    pub phys_in_grps: [snd_efw_phys_grp; HWINFO_MAX_CAPS_GROUPS],

    /* for uapi */
    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,

    /* response queue */
    pub resp_buf: *mut u8,
    pub pull_ptr: *mut u8,
    pub push_ptr: *mut u8,

    pub domain: amdtp_domain,
}

unsafe extern "C" {
    pub fn snd_efw_transaction_cmd(
        unit: *mut fw_unit,
        cmd: *const c_void,
        size: c_uint,
    ) -> c_int;
    pub fn snd_efw_transaction_run(
        unit: *mut fw_unit,
        cmd: *const c_void,
        cmd_size: c_uint,
        resp: *mut c_void,
        resp_size: c_uint,
    ) -> c_int;
    pub fn snd_efw_transaction_register() -> c_int;
    pub fn snd_efw_transaction_unregister();
    pub fn snd_efw_transaction_bus_reset(unit: *mut fw_unit);
    pub fn snd_efw_transaction_add_instance(efw: *mut snd_efw);
    pub fn snd_efw_transaction_remove_instance(efw: *mut snd_efw);
}

#[repr(C, packed)]
pub struct snd_efw_hwinfo {
    pub flags: u32,
    pub guid_hi: u32,
    pub guid_lo: u32,
    pub type_: u32,
    pub version: u32,
    pub vendor_name: [c_char; HWINFO_NAME_SIZE_BYTES],
    pub model_name: [c_char; HWINFO_NAME_SIZE_BYTES],
    pub supported_clocks: u32,
    pub amdtp_rx_pcm_channels: u32,
    pub amdtp_tx_pcm_channels: u32,
    pub phys_out: u32,
    pub phys_in: u32,
    pub phys_out_grp_count: u32,
    pub phys_out_grps: [snd_efw_phys_grp; HWINFO_MAX_CAPS_GROUPS],
    pub phys_in_grp_count: u32,
    pub phys_in_grps: [snd_efw_phys_grp; HWINFO_MAX_CAPS_GROUPS],
    pub midi_out_ports: u32,
    pub midi_in_ports: u32,
    pub max_sample_rate: u32,
    pub min_sample_rate: u32,
    pub dsp_version: u32,
    pub arm_version: u32,
    pub mixer_playback_channels: u32,
    pub mixer_capture_channels: u32,
    pub fpga_version: u32,
    pub amdtp_rx_pcm_channels_2x: u32,
    pub amdtp_tx_pcm_channels_2x: u32,
    pub amdtp_rx_pcm_channels_4x: u32,
    pub amdtp_tx_pcm_channels_4x: u32,
    pub reserved: [u32; 16],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum snd_efw_grp_type {
    SND_EFW_CH_TYPE_ANALOG = 0,
    SND_EFW_CH_TYPE_SPDIF = 1,
    SND_EFW_CH_TYPE_ADAT = 2,
    SND_EFW_CH_TYPE_SPDIF_OR_ADAT = 3,
    SND_EFW_CH_TYPE_ANALOG_MIRRORING = 4,
    SND_EFW_CH_TYPE_HEADPHONES = 5,
    SND_EFW_CH_TYPE_I2S = 6,
    SND_EFW_CH_TYPE_GUITAR = 7,
    SND_EFW_CH_TYPE_PIEZO_GUITAR = 8,
    SND_EFW_CH_TYPE_GUITAR_STRING = 9,
    SND_EFW_CH_TYPE_DUMMY = 10,
}

#[repr(C, packed)]
pub struct snd_efw_phys_meters {
    pub status: u32, /* guitar state/midi signal/clock input detect */
    pub reserved0: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
    pub out_meters: u32,
    pub in_meters: u32,
    pub reserved4: u32,
    pub reserved5: u32,
    pub values: [u32; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum snd_efw_clock_source {
    SND_EFW_CLOCK_SOURCE_INTERNAL = 0,
    // Unused.
    SND_EFW_CLOCK_SOURCE_WORDCLOCK = 2,
    SND_EFW_CLOCK_SOURCE_SPDIF = 3,
    SND_EFW_CLOCK_SOURCE_ADAT_1 = 4,
    SND_EFW_CLOCK_SOURCE_ADAT_2 = 5,
    SND_EFW_CLOCK_SOURCE_CONTINUOUS = 6, /* internal variable clock */
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum snd_efw_transport_mode {
    SND_EFW_TRANSPORT_MODE_WINDOWS = 0,
    SND_EFW_TRANSPORT_MODE_IEC61883 = 1,
}

unsafe extern "C" {
    pub fn snd_efw_command_set_resp_addr(
        efw: *mut snd_efw,
        addr_high: u16,
        addr_low: u32,
    ) -> c_int;
    pub fn snd_efw_command_set_tx_mode(
        efw: *mut snd_efw,
        mode: snd_efw_transport_mode,
    ) -> c_int;
    pub fn snd_efw_command_get_hwinfo(
        efw: *mut snd_efw,
        hwinfo: *mut snd_efw_hwinfo,
    ) -> c_int;
    pub fn snd_efw_command_get_phys_meters(
        efw: *mut snd_efw,
        meters: *mut snd_efw_phys_meters,
        len: c_uint,
    ) -> c_int;
    pub fn snd_efw_command_get_clock_source(
        efw: *mut snd_efw,
        source: *mut snd_efw_clock_source,
    ) -> c_int;
    pub fn snd_efw_command_get_sampling_rate(efw: *mut snd_efw, rate: *mut c_uint) -> c_int;
    pub fn snd_efw_command_set_sampling_rate(efw: *mut snd_efw, rate: c_uint) -> c_int;

    pub fn snd_efw_stream_init_duplex(efw: *mut snd_efw) -> c_int;
    pub fn snd_efw_stream_reserve_duplex(
        efw: *mut snd_efw,
        rate: c_uint,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    pub fn snd_efw_stream_start_duplex(efw: *mut snd_efw) -> c_int;
    pub fn snd_efw_stream_stop_duplex(efw: *mut snd_efw);
    pub fn snd_efw_stream_update_duplex(efw: *mut snd_efw);
    pub fn snd_efw_stream_destroy_duplex(efw: *mut snd_efw);
    pub fn snd_efw_stream_lock_changed(efw: *mut snd_efw);
    pub fn snd_efw_stream_lock_try(efw: *mut snd_efw) -> c_int;
    pub fn snd_efw_stream_lock_release(efw: *mut snd_efw);

    pub fn snd_efw_proc_init(efw: *mut snd_efw);

    pub fn snd_efw_create_midi_devices(efw: *mut snd_efw) -> c_int;

    pub fn snd_efw_create_pcm_devices(efw: *mut snd_efw) -> c_int;
    pub fn snd_efw_get_multiplier_mode(sampling_rate: c_uint, mode: *mut c_uint) -> c_int;

    pub fn snd_efw_create_hwdep_device(efw: *mut snd_efw) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
