/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * tascam.h - a part of driver for TASCAM FireWire series
 *
 * Copyright (c) 2015 Takashi Sakamoto
 */

// C header dependencies:
// linux/device.h, linux/firewire.h, linux/firewire-constants.h, linux/module.h,
// linux/mutex.h, linux/slab.h, linux/compat.h, linux/sched/signal.h,
// sound/core.h, sound/initval.h, sound/info.h, sound/pcm.h,
// sound/pcm_params.h, sound/firewire.h, sound/hwdep.h, sound/rawmidi.h,
// ../lib.h, ../amdtp-stream.h, ../iso-resources.h.

use core::ffi::{c_char, c_int, c_uint};

#[repr(C)]
pub struct snd_tscm_spec {
    pub name: *const c_char,
    pub has_adat: bool,
    pub has_spdif: bool,
    pub pcm_capture_analog_channels: c_uint,
    pub pcm_playback_analog_channels: c_uint,
    pub midi_capture_ports: c_uint,
    pub midi_playback_ports: c_uint,
}

pub const TSCM_MIDI_IN_PORT_MAX: usize = 4;
pub const TSCM_MIDI_OUT_PORT_MAX: usize = 4;

#[repr(C)]
pub struct snd_fw_async_midi_port {
    pub parent: *mut fw_device,
    pub work: work_struct,
    pub idling: bool,
    pub next_ktime: ktime_t,
    pub error: bool,

    pub transaction: fw_transaction,

    pub buf: [u8; 4],
    pub running_status: u8,
    pub on_sysex: bool,

    pub substream: *mut snd_rawmidi_substream,
    pub consume_bytes: c_int,
}

pub const SND_TSCM_QUEUE_COUNT: usize = 16;

#[repr(C)]
pub struct snd_tscm {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,

    pub mutex: mutex,
    pub lock: spinlock_t,

    pub spec: *const snd_tscm_spec,

    pub tx_resources: fw_iso_resources,
    pub rx_resources: fw_iso_resources,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub substreams_counter: c_uint,

    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,

    /* For MIDI message incoming transactions. */
    pub async_handler: fw_address_handler,
    pub tx_midi_substreams: [*mut snd_rawmidi_substream; TSCM_MIDI_IN_PORT_MAX],

    /* For MIDI message outgoing transactions. */
    pub out_ports: [snd_fw_async_midi_port; TSCM_MIDI_OUT_PORT_MAX],

    // A cache of status information in tx isoc packets.
    pub state: [__be32; SNDRV_FIREWIRE_TASCAM_STATE_COUNT],
    pub hwdep: *mut snd_hwdep,
    pub queue: [snd_firewire_tascam_change; SND_TSCM_QUEUE_COUNT],
    pub pull_pos: c_uint,
    pub push_pos: c_uint,

    pub domain: amdtp_domain,
    pub need_long_tx_init_skip: bool,
}

pub const TSCM_ADDR_BASE: u64 = 0xffff00000000u64;

pub const TSCM_OFFSET_FIRMWARE_REGISTER: u32 = 0x0000;
pub const TSCM_OFFSET_FIRMWARE_FPGA: u32 = 0x0004;
pub const TSCM_OFFSET_FIRMWARE_ARM: u32 = 0x0008;
pub const TSCM_OFFSET_FIRMWARE_HW: u32 = 0x000c;

pub const TSCM_OFFSET_ISOC_TX_CH: u32 = 0x0200;
pub const TSCM_OFFSET_UNKNOWN: u32 = 0x0204;
pub const TSCM_OFFSET_START_STREAMING: u32 = 0x0208;
pub const TSCM_OFFSET_ISOC_RX_CH: u32 = 0x020c;
pub const TSCM_OFFSET_ISOC_RX_ON: u32 = 0x0210; /* Little conviction. */
pub const TSCM_OFFSET_TX_PCM_CHANNELS: u32 = 0x0214;
pub const TSCM_OFFSET_RX_PCM_CHANNELS: u32 = 0x0218;
pub const TSCM_OFFSET_MULTIPLEX_MODE: u32 = 0x021c;
pub const TSCM_OFFSET_ISOC_TX_ON: u32 = 0x0220;
/* Unknown				0x0224 */
pub const TSCM_OFFSET_CLOCK_STATUS: u32 = 0x0228;
pub const TSCM_OFFSET_SET_OPTION: u32 = 0x022c;

pub const TSCM_OFFSET_MIDI_TX_ON: u32 = 0x0300;
pub const TSCM_OFFSET_MIDI_TX_ADDR_HI: u32 = 0x0304;
pub const TSCM_OFFSET_MIDI_TX_ADDR_LO: u32 = 0x0308;

pub const TSCM_OFFSET_LED_POWER: u32 = 0x0404;

pub const TSCM_OFFSET_MIDI_RX_QUAD: u32 = 0x4000;

// Although FE-8 supports the above registers, it has no I/O interfaces for
// audio samples and music messages. Otherwise it supports another notification
// for status and control message as well as LED brightening. The message
// consists of quadlet-aligned data up to 32 quadlets. The first byte of message
// is fixed to 0x40. The second byte is between 0x00 to 0x1f and represent each
// control:
//   fader:	0x00-0x07
//   button:	0x0d, 0x0e
//   knob:	0x14-0x1b
//   sensing:	0x0b
//
// The rest two bytes represent state of the controls; e.g. current value for
// fader and knob, bitmasks for button and sensing.
// Just after turning on, 32 quadlets messages with 0x00-0x1f are immediately
// sent in one transaction. After, several quadlets are sent in one transaction.
//
// TSCM_OFFSET_FE8_CTL_TX_ON		0x0310
// TSCM_OFFSET_FE8_CTL_TX_ADDR_HI	0x0314
// TSCM_OFFSET_FE8_CTL_TX_ADDR_LO	0x0318

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_tscm_clock {
    SND_TSCM_CLOCK_INTERNAL = 0,
    SND_TSCM_CLOCK_WORD = 1,
    SND_TSCM_CLOCK_SPDIF = 2,
    SND_TSCM_CLOCK_ADAT = 3,
}

unsafe extern "C" {
    pub fn amdtp_tscm_init(
        s: *mut amdtp_stream,
        unit: *mut fw_unit,
        dir: amdtp_stream_direction,
        pcm_channels: c_uint,
    ) -> c_int;
    pub fn amdtp_tscm_set_parameters(s: *mut amdtp_stream, rate: c_uint) -> c_int;
    pub fn amdtp_tscm_add_pcm_hw_constraints(
        s: *mut amdtp_stream,
        runtime: *mut snd_pcm_runtime,
    ) -> c_int;

    pub fn snd_tscm_stream_get_rate(tscm: *mut snd_tscm, rate: *mut c_uint) -> c_int;
    pub fn snd_tscm_stream_get_clock(
        tscm: *mut snd_tscm,
        clock: *mut snd_tscm_clock,
    ) -> c_int;
    pub fn snd_tscm_stream_init_duplex(tscm: *mut snd_tscm) -> c_int;
    pub fn snd_tscm_stream_update_duplex(tscm: *mut snd_tscm);
    pub fn snd_tscm_stream_destroy_duplex(tscm: *mut snd_tscm);
    pub fn snd_tscm_stream_reserve_duplex(
        tscm: *mut snd_tscm,
        rate: c_uint,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    pub fn snd_tscm_stream_start_duplex(tscm: *mut snd_tscm, rate: c_uint) -> c_int;
    pub fn snd_tscm_stream_stop_duplex(tscm: *mut snd_tscm);

    pub fn snd_tscm_stream_lock_changed(tscm: *mut snd_tscm);
    pub fn snd_tscm_stream_lock_try(tscm: *mut snd_tscm) -> c_int;
    pub fn snd_tscm_stream_lock_release(tscm: *mut snd_tscm);

    pub fn snd_fw_async_midi_port_init(port: *mut snd_fw_async_midi_port);
}

#[inline]
pub unsafe fn snd_fw_async_midi_port_run(
    port: *mut snd_fw_async_midi_port,
    substream: *mut snd_rawmidi_substream,
) {
    if unsafe { !(*port).error } {
        unsafe {
            (*port).substream = substream;
            schedule_work(&mut (*port).work);
        }
    }
}

#[inline]
pub unsafe fn snd_fw_async_midi_port_finish(port: *mut snd_fw_async_midi_port) {
    unsafe {
        (*port).substream = core::ptr::null_mut();
        cancel_work_sync(&mut (*port).work);
        (*port).error = false;
    }
}

unsafe extern "C" {
    pub fn snd_tscm_transaction_register(tscm: *mut snd_tscm) -> c_int;
    pub fn snd_tscm_transaction_reregister(tscm: *mut snd_tscm) -> c_int;
    pub fn snd_tscm_transaction_unregister(tscm: *mut snd_tscm);

    pub fn snd_tscm_proc_init(tscm: *mut snd_tscm);

    pub fn snd_tscm_create_pcm_devices(tscm: *mut snd_tscm) -> c_int;

    pub fn snd_tscm_create_midi_devices(tscm: *mut snd_tscm) -> c_int;

    pub fn snd_tscm_create_hwdep_device(tscm: *mut snd_tscm) -> c_int;

    fn schedule_work(work: *mut work_struct);
    fn cancel_work_sync(work: *mut work_struct) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
