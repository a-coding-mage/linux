/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * oxfw.h - a part of driver for OXFW970/971 based devices
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// C header dependencies:
// linux/device.h, linux/firewire.h, linux/firewire-constants.h, linux/module.h,
// linux/mutex.h, linux/slab.h, linux/compat.h, linux/sched/signal.h,
// sound/control.h, sound/core.h, sound/initval.h, sound/pcm.h,
// sound/pcm_params.h, sound/info.h, sound/rawmidi.h, sound/firewire.h,
// sound/hwdep.h, ../lib.h, ../fcp.h, ../packets-buffer.h,
// ../iso-resources.h, ../amdtp-am824.h, ../cmp.h.

use core::ffi::{c_int, c_uint, c_void};

pub type u8 = core::ffi::c_uchar;
pub type bool = core::ffi::c_bool;
pub type spinlock_t = c_void;
pub type wait_queue_head_t = c_void;
pub type avc_general_plug_dir = c_uint;

#[repr(C)]
pub struct snd_card {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct fw_unit {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct cmp_connection {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct amdtp_stream {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct amdtp_domain {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_oxfw_quirk {
    // Postpone transferring packets during handling asynchronous transaction. As a result,
    // next isochronous packet includes more events than one packet can include.
    SND_OXFW_QUIRK_JUMBO_PAYLOAD = 0x01,
    // The dbs field of CIP header in tx packet is wrong.
    SND_OXFW_QUIRK_WRONG_DBS = 0x02,
    // Blocking transmission mode is used.
    SND_OXFW_QUIRK_BLOCKING_TRANSMISSION = 0x04,
    // Stanton SCS1.d and SCS1.m support unique transaction.
    SND_OXFW_QUIRK_SCS_TRANSACTION = 0x08,
    // Apogee Duet FireWire ignores data blocks in packet with NO_INFO for audio data
    // processing, while output level meter moves. Any value in syt field of packet takes
    // the device to process audio data even if the value is invalid in a point of
    // IEC 61883-1/6.
    SND_OXFW_QUIRK_IGNORE_NO_INFO_PACKET = 0x10,
    // Loud Technologies Mackie Onyx 1640i seems to configure OXFW971 ASIC so that it decides
    // event frequency according to events in received isochronous packets. The device looks to
    // performs media clock recovery voluntarily. In the recovery, the packets with NO_INFO
    // are ignored, thus driver should transfer packets with timestamp.
    SND_OXFW_QUIRK_VOLUNTARY_RECOVERY = 0x20,
    // Miglia Harmony Audio does not support AV/C Stream Format Information command.
    SND_OXFW_QUIRK_STREAM_FORMAT_INFO_UNSUPPORTED = 0x40,
    // Miglia Harmony Audio transmits CIP in which the value of dbc field expresses the number
    // of accumulated payload quadlets including the packet.
    SND_OXFW_QUIRK_DBC_IS_TOTAL_PAYLOAD_QUADLETS = 0x80,
}

/* This is an arbitrary number for convinience. */
pub const SND_OXFW_STREAM_FORMAT_ENTRIES: usize = 10;

#[repr(C)]
pub struct snd_oxfw {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
    pub mutex: mutex,
    pub lock: spinlock_t,

    // The combination of snd_oxfw_quirk enumeration-constants.
    pub quirks: c_uint,
    pub has_output: bool,
    pub has_input: bool,
    pub tx_stream_formats: [*mut u8; SND_OXFW_STREAM_FORMAT_ENTRIES],
    pub rx_stream_formats: [*mut u8; SND_OXFW_STREAM_FORMAT_ENTRIES],
    pub assumed: bool,
    pub out_conn: cmp_connection,
    pub in_conn: cmp_connection,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub substreams_count: c_uint,

    pub midi_input_ports: c_uint,
    pub midi_output_ports: c_uint,

    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: wait_queue_head_t,

    pub spec: *mut c_void,

    pub domain: amdtp_domain,
}

/*
 * AV/C Stream Format Information Specification 1.1 Working Draft
 * (Apr 2005, 1394TA)
 */
unsafe extern "C" {
    pub fn avc_stream_set_format(
        unit: *mut fw_unit,
        dir: avc_general_plug_dir,
        pid: c_uint,
        format: *mut u8,
        len: c_uint,
    ) -> c_int;
    pub fn avc_stream_get_format(
        unit: *mut fw_unit,
        dir: avc_general_plug_dir,
        pid: c_uint,
        buf: *mut u8,
        len: *mut c_uint,
        eid: c_uint,
    ) -> c_int;
}

#[inline]
pub unsafe fn avc_stream_get_format_single(
    unit: *mut fw_unit,
    dir: avc_general_plug_dir,
    pid: c_uint,
    buf: *mut u8,
    len: *mut c_uint,
) -> c_int {
    unsafe { avc_stream_get_format(unit, dir, pid, buf, len, 0xff) }
}

#[inline]
pub unsafe fn avc_stream_get_format_list(
    unit: *mut fw_unit,
    dir: avc_general_plug_dir,
    pid: c_uint,
    buf: *mut u8,
    len: *mut c_uint,
    eid: c_uint,
) -> c_int {
    unsafe { avc_stream_get_format(unit, dir, pid, buf, len, eid) }
}

/*
 * AV/C Digital Interface Command Set General Specification 4.2
 * (Sep 2004, 1394TA)
 */
unsafe extern "C" {
    pub fn avc_general_inquiry_sig_fmt(
        unit: *mut fw_unit,
        rate: c_uint,
        dir: avc_general_plug_dir,
        pid: core::ffi::c_ushort,
    ) -> c_int;

    pub fn snd_oxfw_stream_init_duplex(oxfw: *mut snd_oxfw) -> c_int;
    pub fn snd_oxfw_stream_reserve_duplex(
        oxfw: *mut snd_oxfw,
        stream: *mut amdtp_stream,
        rate: c_uint,
        pcm_channels: c_uint,
        frames_per_period: c_uint,
        frames_per_buffer: c_uint,
    ) -> c_int;
    pub fn snd_oxfw_stream_start_duplex(oxfw: *mut snd_oxfw) -> c_int;
    pub fn snd_oxfw_stream_stop_duplex(oxfw: *mut snd_oxfw);
    pub fn snd_oxfw_stream_destroy_duplex(oxfw: *mut snd_oxfw);
    pub fn snd_oxfw_stream_update_duplex(oxfw: *mut snd_oxfw);
}

#[repr(C)]
pub struct snd_oxfw_stream_formation {
    pub rate: c_uint,
    pub pcm: c_uint,
    pub midi: c_uint,
}

unsafe extern "C" {
    pub fn snd_oxfw_stream_parse_format(
        format: *const u8,
        formation: *mut snd_oxfw_stream_formation,
    ) -> c_int;
    pub fn snd_oxfw_stream_get_current_formation(
        oxfw: *mut snd_oxfw,
        dir: avc_general_plug_dir,
        formation: *mut snd_oxfw_stream_formation,
    ) -> c_int;

    pub fn snd_oxfw_stream_discover(oxfw: *mut snd_oxfw) -> c_int;

    pub fn snd_oxfw_stream_lock_changed(oxfw: *mut snd_oxfw);
    pub fn snd_oxfw_stream_lock_try(oxfw: *mut snd_oxfw) -> c_int;
    pub fn snd_oxfw_stream_lock_release(oxfw: *mut snd_oxfw);

    pub fn snd_oxfw_create_pcm(oxfw: *mut snd_oxfw) -> c_int;

    pub fn snd_oxfw_proc_init(oxfw: *mut snd_oxfw);

    pub fn snd_oxfw_create_midi(oxfw: *mut snd_oxfw) -> c_int;

    pub fn snd_oxfw_create_hwdep(oxfw: *mut snd_oxfw) -> c_int;

    pub fn snd_oxfw_add_spkr(oxfw: *mut snd_oxfw, is_lacie: bool) -> c_int;
    pub fn snd_oxfw_scs1x_add(oxfw: *mut snd_oxfw) -> c_int;
    pub fn snd_oxfw_scs1x_update(oxfw: *mut snd_oxfw);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
