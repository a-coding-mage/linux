// SPDX-License-Identifier: GPL-2.0
// ff-protocol-latter.c - a part of driver for RME Fireface series
//
// Copyright (c) 2019 Takashi Sakamoto

// C dependencies: <linux/delay.h>, "ff.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;

type u8 = u8;
type u32 = u32;
type u64 = u64;
type size_t = usize;
type __le32 = u32;

const LATTER_STF: u64 = 0xffff00000004;
const LATTER_ISOC_CHANNELS: u64 = 0xffff00000008;
const LATTER_ISOC_START: u64 = 0xffff0000000c;
const LATTER_FETCH_MODE: u64 = 0xffff00000010;
const LATTER_SYNC_STATUS: u64 = 0x0000801c0000;

const EIO: c_int = 5;
const EINVAL: c_int = 22;
const ETIMEDOUT: c_int = 110;
const TCODE_READ_QUADLET_REQUEST: c_int = 0;
const TCODE_WRITE_QUADLET_REQUEST: c_int = 1;

#[repr(C)]
pub struct snd_ff {
    pub unit: *mut fw_unit,
    pub unit_version: snd_ff_unit_version,
    pub tx_resources: fw_iso_resources,
    pub rx_resources: fw_iso_resources,
    pub tx_stream: amdtp_stream,
    pub rx_stream: amdtp_stream,
    pub spec: *mut snd_ff_spec,
    pub tx_midi_substreams: [*mut snd_rawmidi_substream; 4],
    pub on_sysex: [bool; 4],
    pub msg_buf: [[__le32; 1]; 4],
    pub rx_bytes: [c_int; 4],
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_device {
    pub max_speed: c_uint,
    pub card: *mut fw_card,
}

#[repr(C)]
pub struct fw_card {
    pub generation: c_uint,
}

#[repr(C)]
pub struct fw_iso_resources {
    pub channels_mask: u64,
    pub generation: c_uint,
    pub channel: c_uint,
}

#[repr(C)]
pub struct amdtp_stream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ff_spec {
    pub midi_in_ports: c_uint,
}

#[repr(C)]
pub struct snd_rawmidi_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum snd_ff_unit_version {
    SND_FF_UNIT_VERSION_UCX,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum snd_ff_clock_src {
    SND_FF_CLOCK_SRC_SPDIF,
    SND_FF_CLOCK_SRC_ADAT1,
    SND_FF_CLOCK_SRC_WORD,
    SND_FF_CLOCK_SRC_INTERNAL,
    SND_FF_CLOCK_SRC_ADAT2,
}

#[repr(C)]
pub enum snd_ff_stream_mode {
    _Dummy,
}

#[repr(C)]
pub struct snd_ff_protocol {
    pub handle_msg: Option<
        unsafe extern "C" fn(*mut snd_ff, c_uint, *const __le32, size_t, u32),
    >,
    pub fill_midi_msg:
        Option<unsafe extern "C" fn(*mut snd_ff, *mut snd_rawmidi_substream, c_uint) -> c_int>,
    pub get_clock:
        Option<unsafe extern "C" fn(*mut snd_ff, *mut c_uint, *mut snd_ff_clock_src) -> c_int>,
    pub switch_fetching_mode: Option<unsafe extern "C" fn(*mut snd_ff, bool) -> c_int>,
    pub allocate_resources: Option<unsafe extern "C" fn(*mut snd_ff, c_uint) -> c_int>,
    pub begin_session: Option<unsafe extern "C" fn(*mut snd_ff, c_uint) -> c_int>,
    pub finish_session: Option<unsafe extern "C" fn(*mut snd_ff)>,
    pub dump_status: Option<unsafe extern "C" fn(*mut snd_ff, *mut snd_info_buffer)>,
}

unsafe extern "C" {
    static amdtp_rate_table: [c_uint; 0];

    fn snd_fw_transaction(
        unit: *mut fw_unit,
        tcode: c_int,
        offset: u64,
        buffer: *mut c_void,
        length: size_t,
        flags: c_int,
    ) -> c_int;
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn fw_iso_resources_allocate(
        resources: *mut fw_iso_resources,
        max_payload: c_uint,
        speed: c_uint,
    ) -> c_int;
    fn fw_iso_resources_free(resources: *mut fw_iso_resources);
    fn fw_iso_resources_update(resources: *mut fw_iso_resources) -> c_int;
    fn amdtp_stream_get_max_payload(stream: *mut amdtp_stream) -> c_uint;
    fn snd_ff_stream_get_multiplier_mode(index: c_int, mode: *mut snd_ff_stream_mode) -> c_int;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_ff_proc_get_clk_label(src: snd_ff_clock_src) -> *const c_char;
    fn snd_rawmidi_receive(substream: *mut snd_rawmidi_substream, buffer: *mut u8, count: c_uint);
    fn snd_rawmidi_transmit_peek(
        substream: *mut snd_rawmidi_substream,
        buffer: *mut u8,
        count: c_uint,
    ) -> c_int;
}

#[inline]
fn cpu_to_le32(data: u32) -> __le32 {
    data.to_le()
}

#[inline]
fn le32_to_cpu(data: __le32) -> u32 {
    u32::from_le(data)
}

#[inline]
unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    core::ptr::read_volatile(p)
}

// The content of sync status register differs between models.
//
// Fireface UCX:
//  0xf0000000: (unidentified)
//  0x0f000000: effective rate of sampling clock
//  0x00f00000: detected rate of word clock on BNC interface
//  0x000f0000: detected rate of ADAT or S/PDIF on optical interface
//  0x0000f000: detected rate of S/PDIF on coaxial interface
//  0x00000e00: effective source of sampling clock
//    0x00000e00: Internal
//    0x00000800: (unidentified)
//    0x00000600: Word clock on BNC interface
//    0x00000400: ADAT on optical interface
//    0x00000200: S/PDIF on coaxial or optical interface
//  0x00000100: Optical interface is used for ADAT signal
//  0x00000080: (unidentified)
//  0x00000040: Synchronized to word clock on BNC interface
//  0x00000020: Synchronized to ADAT or S/PDIF on optical interface
//  0x00000010: Synchronized to S/PDIF on coaxial interface
//  0x00000008: (unidentified)
//  0x00000004: Lock word clock on BNC interface
//  0x00000002: Lock ADAT or S/PDIF on optical interface
//  0x00000001: Lock S/PDIF on coaxial interface
//
// Fireface 802 (and perhaps UFX):
//   0xf0000000: effective rate of sampling clock
//   0x0f000000: detected rate of ADAT-B on 2nd optical interface
//   0x00f00000: detected rate of ADAT-A on 1st optical interface
//   0x000f0000: detected rate of AES/EBU on XLR or coaxial interface
//   0x0000f000: detected rate of word clock on BNC interface
//   0x00000e00: effective source of sampling clock
//     0x00000e00: internal
//     0x00000800: ADAT-B
//     0x00000600: ADAT-A
//     0x00000400: AES/EBU
//     0x00000200: Word clock
//   0x00000080: Synchronized to ADAT-B on 2nd optical interface
//   0x00000040: Synchronized to ADAT-A on 1st optical interface
//   0x00000020: Synchronized to AES/EBU on XLR or 2nd optical interface
//   0x00000010: Synchronized to word clock on BNC interface
//   0x00000008: Lock ADAT-B on 2nd optical interface
//   0x00000004: Lock ADAT-A on 1st optical interface
//   0x00000002: Lock AES/EBU on XLR or 2nd optical interface
//   0x00000001: Lock word clock on BNC interface
//
// The pattern for rate bits:
//   0x00: 32.0 kHz
//   0x01: 44.1 kHz
//   0x02: 48.0 kHz
//   0x04: 64.0 kHz
//   0x05: 88.2 kHz
//   0x06: 96.0 kHz
//   0x08: 128.0 kHz
//   0x09: 176.4 kHz
//   0x0a: 192.0 kHz
unsafe extern "C" fn parse_clock_bits(
    data: u32,
    rate: *mut c_uint,
    src: *mut snd_ff_clock_src,
    unit_version: snd_ff_unit_version,
) -> c_int {
    #[repr(C)]
    struct RateEntry {
        rate: c_uint,
        flag: u32,
    }
    #[repr(C)]
    struct ClkEntry {
        src: snd_ff_clock_src,
        flag: u32,
    }

    static RATE_ENTRIES: [RateEntry; 9] = [
        RateEntry { rate: 32000, flag: 0x00 },
        RateEntry { rate: 44100, flag: 0x01 },
        RateEntry { rate: 48000, flag: 0x02 },
        RateEntry { rate: 64000, flag: 0x04 },
        RateEntry { rate: 88200, flag: 0x05 },
        RateEntry { rate: 96000, flag: 0x06 },
        RateEntry { rate: 128000, flag: 0x08 },
        RateEntry { rate: 176400, flag: 0x09 },
        RateEntry { rate: 192000, flag: 0x0a },
    ];
    static UCX_CLK_ENTRIES: [ClkEntry; 4] = [
        ClkEntry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_SPDIF, flag: 0x00000200 },
        ClkEntry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_ADAT1, flag: 0x00000400 },
        ClkEntry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_WORD, flag: 0x00000600 },
        ClkEntry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_INTERNAL, flag: 0x00000e00 },
    ];
    static UFX_FF802_CLK_ENTRIES: [ClkEntry; 5] = [
        ClkEntry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_WORD, flag: 0x00000200 },
        ClkEntry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_SPDIF, flag: 0x00000400 },
        ClkEntry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_ADAT1, flag: 0x00000600 },
        ClkEntry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_ADAT2, flag: 0x00000800 },
        ClkEntry { src: snd_ff_clock_src::SND_FF_CLOCK_SRC_INTERNAL, flag: 0x00000e00 },
    ];

    let rate_bits: u32;
    let clk_entries: *const ClkEntry;
    let clk_entry_count: c_uint;

    if unit_version == snd_ff_unit_version::SND_FF_UNIT_VERSION_UCX {
        rate_bits = (data & 0x0f000000) >> 24;
        clk_entries = UCX_CLK_ENTRIES.as_ptr();
        clk_entry_count = UCX_CLK_ENTRIES.len() as c_uint;
    } else {
        rate_bits = (data & 0xf0000000) >> 28;
        clk_entries = UFX_FF802_CLK_ENTRIES.as_ptr();
        clk_entry_count = UFX_FF802_CLK_ENTRIES.len() as c_uint;
    }

    let mut i: c_int = 0;
    while (i as usize) < RATE_ENTRIES.len() {
        let rate_entry = &RATE_ENTRIES[i as usize];
        if rate_bits == rate_entry.flag {
            *rate = rate_entry.rate;
            break;
        }
        i += 1;
    }
    if (i as usize) == RATE_ENTRIES.len() {
        return -EIO;
    }

    i = 0;
    while (i as c_uint) < clk_entry_count {
        let clk_entry = &*clk_entries.add(i as usize);
        if (data & 0x000e00) == clk_entry.flag {
            *src = clk_entry.src;
            break;
        }
        i += 1;
    }
    if (i as c_uint) == clk_entry_count {
        return -EIO;
    }

    0
}

unsafe extern "C" fn latter_get_clock(
    ff: *mut snd_ff,
    rate: *mut c_uint,
    src: *mut snd_ff_clock_src,
) -> c_int {
    let mut reg: __le32 = 0;
    let data: u32;
    let err: c_int;

    err = snd_fw_transaction(
        (*ff).unit,
        TCODE_READ_QUADLET_REQUEST,
        LATTER_SYNC_STATUS,
        &mut reg as *mut __le32 as *mut c_void,
        size_of::<__le32>(),
        0,
    );
    if err < 0 {
        return err;
    }
    data = le32_to_cpu(reg);

    parse_clock_bits(data, rate, src, (*ff).unit_version)
}

unsafe extern "C" fn latter_switch_fetching_mode(ff: *mut snd_ff, enable: bool) -> c_int {
    let data: u32;
    let mut reg: __le32;

    if enable {
        data = 0x00000000;
    } else {
        data = 0xffffffff;
    }
    reg = cpu_to_le32(data);

    snd_fw_transaction(
        (*ff).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        LATTER_FETCH_MODE,
        &mut reg as *mut __le32 as *mut c_void,
        size_of::<__le32>(),
        0,
    )
}

unsafe extern "C" fn latter_allocate_resources(ff: *mut snd_ff, rate: c_uint) -> c_int {
    let mut mode: snd_ff_stream_mode = core::mem::zeroed();
    let mut code: c_uint;
    let mut reg: __le32;
    let mut count: c_uint;
    let mut i: c_int;
    let mut err: c_int;

    // Set the number of data blocks transferred in a second.
    if rate % 48000 == 0 {
        code = 0x04;
    } else if rate % 44100 == 0 {
        code = 0x02;
    } else if rate % 32000 == 0 {
        code = 0x00;
    } else {
        return -EINVAL;
    }

    if rate >= 64000 && rate < 128000 {
        code |= 0x08;
    } else if rate >= 128000 {
        code |= 0x10;
    }

    reg = cpu_to_le32(code);
    err = snd_fw_transaction(
        (*ff).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        LATTER_STF,
        &mut reg as *mut __le32 as *mut c_void,
        size_of::<__le32>(),
        0,
    );
    if err < 0 {
        return err;
    }

    // Confirm to shift transmission clock.
    count = 0;
    while {
        count = count.wrapping_add(1);
        count < 10
    } {
        let mut curr_rate: c_uint = 0;
        let mut src: snd_ff_clock_src = core::mem::zeroed();

        err = latter_get_clock(ff, &mut curr_rate, &mut src);
        if err < 0 {
            return err;
        }

        if curr_rate == rate {
            break;
        }
    }
    if count > 10 {
        return -ETIMEDOUT;
    }

    i = 0;
    while (i as usize) < amdtp_rate_table.len() {
        if rate == amdtp_rate_table[i as usize] {
            break;
        }
        i += 1;
    }
    if (i as usize) == amdtp_rate_table.len() {
        return -EINVAL;
    }

    err = snd_ff_stream_get_multiplier_mode(i, &mut mode);
    if err < 0 {
        return err;
    }

    // Keep resources for in-stream.
    (*ff).tx_resources.channels_mask = 0x00000000000000ffu64;
    err = fw_iso_resources_allocate(
        &mut (*ff).tx_resources,
        amdtp_stream_get_max_payload(&mut (*ff).tx_stream),
        (*fw_parent_device((*ff).unit)).max_speed,
    );
    if err < 0 {
        return err;
    }

    // Keep resources for out-stream.
    (*ff).rx_resources.channels_mask = 0x00000000000000ffu64;
    err = fw_iso_resources_allocate(
        &mut (*ff).rx_resources,
        amdtp_stream_get_max_payload(&mut (*ff).rx_stream),
        (*fw_parent_device((*ff).unit)).max_speed,
    );
    if err < 0 {
        fw_iso_resources_free(&mut (*ff).tx_resources);
    }

    err
}

unsafe extern "C" fn latter_begin_session(ff: *mut snd_ff, rate: c_uint) -> c_int {
    let generation: c_uint = (*ff).rx_resources.generation;
    let flag: c_uint;
    let data: u32;
    let mut reg: __le32;
    let mut err: c_int;

    if (*ff).unit_version == snd_ff_unit_version::SND_FF_UNIT_VERSION_UCX {
        // For Fireface UCX. Always use the maximum number of data
        // channels in data block of packet.
        if rate >= 32000 && rate <= 48000 {
            flag = 0x92;
        } else if rate >= 64000 && rate <= 96000 {
            flag = 0x8e;
        } else if rate >= 128000 && rate <= 192000 {
            flag = 0x8c;
        } else {
            return -EINVAL;
        }
    } else {
        // For Fireface UFX and 802. Due to bandwidth limitation on
        // IEEE 1394a (400 Mbps), Analog 1-12 and AES are available
        // without any ADAT at quadruple speed.
        if rate >= 32000 && rate <= 48000 {
            flag = 0x9e;
        } else if rate >= 64000 && rate <= 96000 {
            flag = 0x96;
        } else if rate >= 128000 && rate <= 192000 {
            flag = 0x8e;
        } else {
            return -EINVAL;
        }
    }

    if generation != (*(*fw_parent_device((*ff).unit)).card).generation {
        err = fw_iso_resources_update(&mut (*ff).tx_resources);
        if err < 0 {
            return err;
        }

        err = fw_iso_resources_update(&mut (*ff).rx_resources);
        if err < 0 {
            return err;
        }
    }

    data = ((*ff).tx_resources.channel << 8) | (*ff).rx_resources.channel;
    reg = cpu_to_le32(data);
    err = snd_fw_transaction(
        (*ff).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        LATTER_ISOC_CHANNELS,
        &mut reg as *mut __le32 as *mut c_void,
        size_of::<__le32>(),
        0,
    );
    if err < 0 {
        return err;
    }

    reg = cpu_to_le32(flag);
    snd_fw_transaction(
        (*ff).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        LATTER_ISOC_START,
        &mut reg as *mut __le32 as *mut c_void,
        size_of::<__le32>(),
        0,
    )
}

unsafe extern "C" fn latter_finish_session(ff: *mut snd_ff) {
    let mut reg: __le32;

    reg = cpu_to_le32(0x00000000);
    snd_fw_transaction(
        (*ff).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        LATTER_ISOC_START,
        &mut reg as *mut __le32 as *mut c_void,
        size_of::<__le32>(),
        0,
    );
}

unsafe extern "C" fn latter_dump_status(ff: *mut snd_ff, buffer: *mut snd_info_buffer) {
    #[repr(C)]
    struct ClkEntry {
        label: *const c_char,
        locked_mask: u32,
        synced_mask: u32,
    }

    static UCX_CLK_ENTRIES: [ClkEntry; 3] = [
        ClkEntry { label: b"S/PDIF\0".as_ptr() as *const c_char, locked_mask: 0x00000001, synced_mask: 0x00000010 },
        ClkEntry { label: b"ADAT\0".as_ptr() as *const c_char, locked_mask: 0x00000002, synced_mask: 0x00000020 },
        ClkEntry { label: b"WDClk\0".as_ptr() as *const c_char, locked_mask: 0x00000004, synced_mask: 0x00000040 },
    ];
    static UFX_FF802_CLK_ENTRIES: [ClkEntry; 4] = [
        ClkEntry { label: b"WDClk\0".as_ptr() as *const c_char, locked_mask: 0x00000001, synced_mask: 0x00000010 },
        ClkEntry { label: b"AES/EBU\0".as_ptr() as *const c_char, locked_mask: 0x00000002, synced_mask: 0x00000020 },
        ClkEntry { label: b"ADAT-A\0".as_ptr() as *const c_char, locked_mask: 0x00000004, synced_mask: 0x00000040 },
        ClkEntry { label: b"ADAT-B\0".as_ptr() as *const c_char, locked_mask: 0x00000008, synced_mask: 0x00000080 },
    ];
    let clk_entries: *const ClkEntry;
    let mut reg: __le32 = 0;
    let data: u32;
    let mut rate: c_uint = 0;
    let mut src: snd_ff_clock_src = core::mem::zeroed();
    let label: *const c_char;
    let clk_entry_count: c_uint;
    let mut i: c_int;
    let mut err: c_int;

    err = snd_fw_transaction(
        (*ff).unit,
        TCODE_READ_QUADLET_REQUEST,
        LATTER_SYNC_STATUS,
        &mut reg as *mut __le32 as *mut c_void,
        size_of::<__le32>(),
        0,
    );
    if err < 0 {
        return;
    }
    data = le32_to_cpu(reg);

    snd_iprintf(buffer, b"External source detection:\n\0".as_ptr() as *const c_char);

    if (*ff).unit_version == snd_ff_unit_version::SND_FF_UNIT_VERSION_UCX {
        clk_entries = UCX_CLK_ENTRIES.as_ptr();
        clk_entry_count = UCX_CLK_ENTRIES.len() as c_uint;
    } else {
        clk_entries = UFX_FF802_CLK_ENTRIES.as_ptr();
        clk_entry_count = UFX_FF802_CLK_ENTRIES.len() as c_uint;
    }

    i = 0;
    while (i as c_uint) < clk_entry_count {
        let clk_entry = &*clk_entries.add(i as usize);
        snd_iprintf(buffer, b"%s: \0".as_ptr() as *const c_char, clk_entry.label);
        if data & clk_entry.locked_mask != 0 {
            if data & clk_entry.synced_mask != 0 {
                snd_iprintf(buffer, b"sync\n\0".as_ptr() as *const c_char);
            } else {
                snd_iprintf(buffer, b"lock\n\0".as_ptr() as *const c_char);
            }
        } else {
            snd_iprintf(buffer, b"none\n\0".as_ptr() as *const c_char);
        }
        i += 1;
    }

    err = parse_clock_bits(data, &mut rate, &mut src, (*ff).unit_version);
    if err < 0 {
        return;
    }
    label = snd_ff_proc_get_clk_label(src);
    if label.is_null() {
        return;
    }

    snd_iprintf(
        buffer,
        b"Referred clock: %s %d\n\0".as_ptr() as *const c_char,
        label,
        rate,
    );
}

// NOTE: transactions are transferred within 0x00-0x7f in allocated range of
// address. This seems to be for check of discontinuity in receiver side.
//
// Like Fireface 400, drivers can select one of 4 options for lower 4 bytes of
// destination address by bit flags in quadlet register (little endian) at
// 0x'ffff'0000'0014:
//
// bit flags: offset of destination address
// - 0x00002000: 0x'....'....'0000'0000
// - 0x00004000: 0x'....'....'0000'0080
// - 0x00008000: 0x'....'....'0000'0100
// - 0x00010000: 0x'....'....'0000'0180
//
// Drivers can suppress the device to transfer asynchronous transactions by
// clear these bit flags.
//
// Actually, the register is write-only and includes the other settings such as
// input attenuation. This driver allocates for the first option
// (0x'....'....'0000'0000) and expects userspace application to configure the
// register for it.
unsafe extern "C" fn latter_handle_midi_msg(
    ff: *mut snd_ff,
    _offset: c_uint,
    buf: *const __le32,
    _length: size_t,
    _tstamp: u32,
) {
    let data: u32 = le32_to_cpu(*buf);
    let index: c_uint = (data & 0x000000f0) >> 4;
    let mut byte: [u8; 3] = [0; 3];
    let substream: *mut snd_rawmidi_substream;
    let len: c_uint;

    if index >= (*(*ff).spec).midi_in_ports {
        return;
    }

    match data & 0x0000000f {
        0x00000008 | 0x00000009 | 0x0000000a | 0x0000000b | 0x0000000e => {
            len = 3;
        }
        0x0000000c | 0x0000000d => {
            len = 2;
        }
        _ => {
            len = {
                let mut tmp = data & 0x00000003;
                if tmp == 0 {
                    tmp = 3;
                }
                tmp
            };
        }
    }

    byte[0] = ((data & 0x0000ff00) >> 8) as u8;
    byte[1] = ((data & 0x00ff0000) >> 16) as u8;
    byte[2] = ((data & 0xff000000) >> 24) as u8;

    substream = READ_ONCE((*ff).tx_midi_substreams.as_ptr().add(index as usize));
    if !substream.is_null() {
        snd_rawmidi_receive(substream, byte.as_mut_ptr(), len);
    }
}

/*
 * When return minus value, given argument is not MIDI status.
 * When return 0, given argument is a beginning of system exclusive.
 * When return the others, given argument is MIDI data.
 */
#[inline]
unsafe extern "C" fn calculate_message_bytes(status: u8) -> c_int {
    match status {
        0xf6 => return 1, /* Tune request. */
        0xf8 => return 1, /* Timing clock. */
        0xfa => return 1, /* Start. */
        0xfb => return 1, /* Continue. */
        0xfc => return 1, /* Stop. */
        0xfe => return 1, /* Active sensing. */
        0xff => return 1, /* System reset. */
        0xf1 => return 2, /* MIDI time code quarter frame. */
        0xf3 => return 2, /* Song select. */
        0xf2 => return 3, /* Song position pointer. */
        0xf0 => return 0, /* Exclusive. */
        0xf7 => {}
        0xf4 => {}
        0xf5 => {}
        0xf9 => {}
        0xfd => {}
        _ => match status & 0xf0 {
            0x80 => return 3, /* Note on. */
            0x90 => return 3, /* Note off. */
            0xa0 => return 3, /* Polyphonic key pressure. */
            0xb0 => return 3, /* Control change and Mode change. */
            0xe0 => return 3, /* Pitch bend change. */
            0xc0 => return 2, /* Program change. */
            0xd0 => return 2, /* Channel pressure. */
            _ => {}
        },
    }

    -EINVAL
}

unsafe extern "C" fn latter_fill_midi_msg(
    ff: *mut snd_ff,
    substream: *mut snd_rawmidi_substream,
    port: c_uint,
) -> c_int {
    let mut data: u32 = 0;
    let buf: *mut u8 = &mut data as *mut u32 as *mut u8;
    let mut consumed: c_int;

    *buf.add(0) = (port << 4) as u8;
    consumed = snd_rawmidi_transmit_peek(substream, buf.add(1), 3);
    if consumed <= 0 {
        return consumed;
    }

    if !(*ff).on_sysex[port as usize] {
        if *buf.add(1) != 0xf0 {
            if consumed < calculate_message_bytes(*buf.add(1)) {
                return 0;
            }
        } else {
            // The beginning of exclusives.
            (*ff).on_sysex[port as usize] = true;
        }

        *buf.add(0) |= consumed as u8;
    } else {
        if *buf.add(1) != 0xf7 {
            if *buf.add(2) == 0xf7 || *buf.add(3) == 0xf7 {
                // Transfer end code at next time.
                consumed -= 1;
            }

            *buf.add(0) |= consumed as u8;
        } else {
            // The end of exclusives.
            (*ff).on_sysex[port as usize] = false;
            consumed = 1;
            *buf.add(0) |= 0x0f;
        }
    }

    (*ff).msg_buf[port as usize][0] = cpu_to_le32(data);
    (*ff).rx_bytes[port as usize] = consumed;

    1
}

#[no_mangle]
pub static snd_ff_protocol_latter: snd_ff_protocol = snd_ff_protocol {
    handle_msg: Some(latter_handle_midi_msg),
    fill_midi_msg: Some(latter_fill_midi_msg),
    get_clock: Some(latter_get_clock),
    switch_fetching_mode: Some(latter_switch_fetching_mode),
    allocate_resources: Some(latter_allocate_resources),
    begin_session: Some(latter_begin_session),
    finish_session: Some(latter_finish_session),
    dump_status: Some(latter_dump_status),
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
