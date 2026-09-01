// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob.h - a part of driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Dependencies from Linux, ALSA, and sibling firewire driver headers are external
// to this translated header.

use core::ffi::{c_char, c_int, c_void};

pub type U8 = u8;
pub type U32 = u32;
pub type U64 = u64;

pub enum SndCard {}
pub enum FwUnit {}
pub enum Mutex {}
pub enum SpinlockT {}
pub enum AmdtpStream {}
pub enum CmpConnection {}
pub enum WaitQueueHeadT {}
pub enum AmdtpDomain {}

pub const TCODE_READ_BLOCK_REQUEST: c_int = 1;
pub const TCODE_READ_QUADLET_REQUEST: c_int = 4;

/* basic register addresses on DM1000/DM1100/DM1500 */
pub const BEBOB_ADDR_REG_INFO: U64 = 0xffffc8020000u64;
pub const BEBOB_ADDR_REG_REQ: U64 = 0xffffc8021000u64;

pub const SND_BEBOB_STRM_FMT_ENTRIES: usize = 7;

#[repr(C)]
pub struct SndBebobStreamFormation {
    pub pcm: u32,
    pub midi: u32,
}

/* this is a lookup table for index of stream formations */
unsafe extern "C" {
    pub static snd_bebob_rate_table: [u32; SND_BEBOB_STRM_FMT_ENTRIES];
}

/* device specific operations */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SndBebobClockType {
    SND_BEBOB_CLOCK_TYPE_INTERNAL = 0,
    SND_BEBOB_CLOCK_TYPE_EXTERNAL = 1,
    SND_BEBOB_CLOCK_TYPE_SYT = 2,
}

#[repr(C)]
pub struct SndBebobClockSpec {
    pub num: u32,
    pub labels: *const *const c_char,
    pub types: *const SndBebobClockType,
    pub get: Option<unsafe extern "C" fn(bebob: *mut SndBebob, id: *mut u32) -> c_int>,
}

#[repr(C)]
pub struct SndBebobRateSpec {
    pub get: Option<unsafe extern "C" fn(bebob: *mut SndBebob, rate: *mut u32) -> c_int>,
    pub set: Option<unsafe extern "C" fn(bebob: *mut SndBebob, rate: u32) -> c_int>,
}

#[repr(C)]
pub struct SndBebobMeterSpec {
    pub num: u32,
    pub labels: *const *const c_char,
    pub get: Option<unsafe extern "C" fn(bebob: *mut SndBebob, target: *mut U32, size: u32) -> c_int>,
}

#[repr(C)]
pub struct SndBebobSpec {
    pub clock: *const SndBebobClockSpec,
    pub rate: *const SndBebobRateSpec,
    pub meter: *const SndBebobMeterSpec,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SndBebobQuirk {
    SND_BEBOB_QUIRK_INITIAL_DISCONTINUOUS_DBC = 1 << 0,
    SND_BEBOB_QUIRK_WRONG_DBC = 1 << 1,
}

#[repr(C)]
pub struct SndBebob {
    pub card: *mut SndCard,
    pub unit: *mut FwUnit,
    pub card_index: c_int,

    pub mutex: Mutex,
    pub lock: SpinlockT,

    pub spec: *const SndBebobSpec,
    pub quirks: u32, // Combination of snd_bebob_quirk enumerations.

    pub midi_input_ports: u32,
    pub midi_output_ports: u32,

    pub tx_stream: AmdtpStream,
    pub rx_stream: AmdtpStream,
    pub out_conn: CmpConnection,
    pub in_conn: CmpConnection,
    pub substreams_counter: u32,

    pub tx_stream_formations: [SndBebobStreamFormation; SND_BEBOB_STRM_FMT_ENTRIES],
    pub rx_stream_formations: [SndBebobStreamFormation; SND_BEBOB_STRM_FMT_ENTRIES],

    pub sync_input_plug: c_int,

    /* for uapi */
    pub dev_lock_count: c_int,
    pub dev_lock_changed: bool,
    pub hwdep_wait: WaitQueueHeadT,

    /* for M-Audio special devices */
    pub maudio_special_quirk: *mut c_void,

    pub domain: AmdtpDomain,
}

unsafe extern "C" {
    pub fn snd_fw_transaction(
        unit: *mut FwUnit,
        tcode: c_int,
        offset: U64,
        buffer: *mut c_void,
        length: c_int,
        flags: c_int,
    ) -> c_int;
}

pub unsafe fn snd_bebob_read_block(
    unit: *mut FwUnit,
    addr: U64,
    buf: *mut c_void,
    size: c_int,
) -> c_int {
    unsafe {
        snd_fw_transaction(
            unit,
            TCODE_READ_BLOCK_REQUEST,
            BEBOB_ADDR_REG_INFO.wrapping_add(addr),
            buf,
            size,
            0,
        )
    }
}

pub unsafe fn snd_bebob_read_quad(unit: *mut FwUnit, addr: U64, buf: *mut U32) -> c_int {
    unsafe {
        snd_fw_transaction(
            unit,
            TCODE_READ_QUADLET_REQUEST,
            BEBOB_ADDR_REG_INFO.wrapping_add(addr),
            buf.cast::<c_void>(),
            core::mem::size_of::<U32>() as c_int,
            0,
        )
    }
}

/* AV/C Audio Subunit Specification 1.0 (Oct 2000, 1394TA) */
unsafe extern "C" {
    pub fn avc_audio_set_selector(
        unit: *mut FwUnit,
        subunit_id: u32,
        fb_id: u32,
        num: u32,
    ) -> c_int;
    pub fn avc_audio_get_selector(
        unit: *mut FwUnit,
        subunit_id: u32,
        fb_id: u32,
        num: *mut u32,
    ) -> c_int;
}

/*
 * AVC command extensions, AV/C Unit and Subunit, Revision 17
 * (Nov 2003, BridgeCo)
 */
pub const AVC_BRIDGECO_ADDR_BYTES: usize = 6;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AvcBridgecoPlugDir {
    AVC_BRIDGECO_PLUG_DIR_IN = 0x00,
    AVC_BRIDGECO_PLUG_DIR_OUT = 0x01,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AvcBridgecoPlugMode {
    AVC_BRIDGECO_PLUG_MODE_UNIT = 0x00,
    AVC_BRIDGECO_PLUG_MODE_SUBUNIT = 0x01,
    AVC_BRIDGECO_PLUG_MODE_FUNCTION_BLOCK = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AvcBridgecoPlugUnit {
    AVC_BRIDGECO_PLUG_UNIT_ISOC = 0x00,
    AVC_BRIDGECO_PLUG_UNIT_EXT = 0x01,
    AVC_BRIDGECO_PLUG_UNIT_ASYNC = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AvcBridgecoPlugType {
    AVC_BRIDGECO_PLUG_TYPE_ISOC = 0x00,
    AVC_BRIDGECO_PLUG_TYPE_ASYNC = 0x01,
    AVC_BRIDGECO_PLUG_TYPE_MIDI = 0x02,
    AVC_BRIDGECO_PLUG_TYPE_SYNC = 0x03,
    AVC_BRIDGECO_PLUG_TYPE_ANA = 0x04,
    AVC_BRIDGECO_PLUG_TYPE_DIG = 0x05,
    AVC_BRIDGECO_PLUG_TYPE_ADDITION = 0x06,
}

pub unsafe fn avc_bridgeco_fill_unit_addr(
    buf: *mut U8,
    dir: AvcBridgecoPlugDir,
    unit: AvcBridgecoPlugUnit,
    pid: u32,
) {
    unsafe {
        *buf.add(0) = 0xff; /* Unit */
        *buf.add(1) = dir as U8;
        *buf.add(2) = AvcBridgecoPlugMode::AVC_BRIDGECO_PLUG_MODE_UNIT as U8;
        *buf.add(3) = unit as U8;
        *buf.add(4) = (0xff & pid) as U8;
        *buf.add(5) = 0xff; /* reserved */
    }
}

pub unsafe fn avc_bridgeco_fill_msu_addr(
    buf: *mut U8,
    dir: AvcBridgecoPlugDir,
    pid: u32,
) {
    unsafe {
        *buf.add(0) = 0x60; /* Music subunit */
        *buf.add(1) = dir as U8;
        *buf.add(2) = AvcBridgecoPlugMode::AVC_BRIDGECO_PLUG_MODE_SUBUNIT as U8;
        *buf.add(3) = (0xff & pid) as U8;
        *buf.add(4) = 0xff; /* reserved */
        *buf.add(5) = 0xff; /* reserved */
    }
}

unsafe extern "C" {
    pub fn avc_bridgeco_get_plug_ch_pos(
        unit: *mut FwUnit,
        addr: *mut U8,
        buf: *mut U8,
        len: u32,
    ) -> c_int;
    pub fn avc_bridgeco_get_plug_type(
        unit: *mut FwUnit,
        addr: *mut U8,
        type_: *mut AvcBridgecoPlugType,
    ) -> c_int;
    pub fn avc_bridgeco_get_plug_ch_count(
        unit: *mut FwUnit,
        addr: *mut U8,
        ch_count: *mut u32,
    ) -> c_int;
    pub fn avc_bridgeco_get_plug_section_type(
        unit: *mut FwUnit,
        addr: *mut U8,
        id: u32,
        type_: *mut U8,
    ) -> c_int;
    pub fn avc_bridgeco_get_plug_input(unit: *mut FwUnit, addr: *mut U8, input: *mut U8)
        -> c_int;
    pub fn avc_bridgeco_get_plug_strm_fmt(
        unit: *mut FwUnit,
        addr: *mut U8,
        buf: *mut U8,
        len: *mut u32,
        eid: u32,
    ) -> c_int;

    /* for AMDTP streaming */
    pub fn snd_bebob_stream_get_rate(bebob: *mut SndBebob, rate: *mut u32) -> c_int;
    pub fn snd_bebob_stream_set_rate(bebob: *mut SndBebob, rate: u32) -> c_int;
    pub fn snd_bebob_stream_get_clock_src(
        bebob: *mut SndBebob,
        src: *mut SndBebobClockType,
    ) -> c_int;
    pub fn snd_bebob_stream_discover(bebob: *mut SndBebob) -> c_int;
    pub fn snd_bebob_stream_init_duplex(bebob: *mut SndBebob) -> c_int;
    pub fn snd_bebob_stream_reserve_duplex(
        bebob: *mut SndBebob,
        rate: u32,
        frames_per_period: u32,
        frames_per_buffer: u32,
    ) -> c_int;
    pub fn snd_bebob_stream_start_duplex(bebob: *mut SndBebob) -> c_int;
    pub fn snd_bebob_stream_stop_duplex(bebob: *mut SndBebob);
    pub fn snd_bebob_stream_destroy_duplex(bebob: *mut SndBebob);

    pub fn snd_bebob_stream_lock_changed(bebob: *mut SndBebob);
    pub fn snd_bebob_stream_lock_try(bebob: *mut SndBebob) -> c_int;
    pub fn snd_bebob_stream_lock_release(bebob: *mut SndBebob);

    pub fn snd_bebob_proc_init(bebob: *mut SndBebob);

    pub fn snd_bebob_create_midi_devices(bebob: *mut SndBebob) -> c_int;

    pub fn snd_bebob_create_pcm_devices(bebob: *mut SndBebob) -> c_int;

    pub fn snd_bebob_create_hwdep_device(bebob: *mut SndBebob) -> c_int;

    /* model specific operations */
    pub static phase88_rack_spec: SndBebobSpec;
    pub static yamaha_terratec_spec: SndBebobSpec;
    pub static saffirepro_26_spec: SndBebobSpec;
    pub static saffirepro_10_spec: SndBebobSpec;
    pub static saffire_le_spec: SndBebobSpec;
    pub static saffire_spec: SndBebobSpec;
    pub static maudio_fw410_spec: SndBebobSpec;
    pub static maudio_audiophile_spec: SndBebobSpec;
    pub static maudio_solo_spec: SndBebobSpec;
    pub static maudio_ozonic_spec: SndBebobSpec;
    pub static maudio_nrv10_spec: SndBebobSpec;
    pub static maudio_special_spec: SndBebobSpec;
    pub fn snd_bebob_maudio_special_discover(bebob: *mut SndBebob, is1814: bool) -> c_int;
    pub fn snd_bebob_maudio_load_firmware(unit: *mut FwUnit) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
