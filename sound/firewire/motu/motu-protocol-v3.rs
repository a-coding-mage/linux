// SPDX-License-Identifier: GPL-2.0-only
/*
 * motu-protocol-v3.c - a part of driver for MOTU FireWire series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// Original C dependencies:
// #include <linux/delay.h>
// #include "motu.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type __be32 = u32;
type u32 = core::ffi::c_uint;

const V3_CLOCK_STATUS_OFFSET: c_uint = 0x0b14;
const V3_FETCH_PCM_FRAMES: u32 = 0x02000000;
const V3_CLOCK_RATE_MASK: u32 = 0x0000ff00;
const V3_CLOCK_RATE_SHIFT: u32 = 8;
const V3_CLOCK_SOURCE_MASK: u32 = 0x000000ff;
const V3_CLOCK_SRC_INTERNAL: u32 = 0x00;
const V3_CLOCK_SRC_WORD_ON_BNC: u32 = 0x01;
const V3_CLOCK_SRC_SPH: u32 = 0x02;
const V3_CLOCK_SRC_AESEBU_ON_XLR: u32 = 0x08;
const V3_CLOCK_SRC_SPDIF_ON_COAX: u32 = 0x10;
const V3_CLOCK_SRC_OPT_IFACE_A: u32 = 0x18;
const V3_CLOCK_SRC_OPT_IFACE_B: u32 = 0x19;

const V3_OPT_IFACE_MODE_OFFSET: c_uint = 0x0c94;
const V3_ENABLE_OPT_IN_IFACE_A: u32 = 0x00000001;
const V3_ENABLE_OPT_IN_IFACE_B: u32 = 0x00000002;
const V3_ENABLE_OPT_OUT_IFACE_A: u32 = 0x00000100;
const V3_ENABLE_OPT_OUT_IFACE_B: u32 = 0x00000200;
const V3_NO_ADAT_OPT_IN_IFACE_A: u32 = 0x00010000;
const V3_NO_ADAT_OPT_IN_IFACE_B: u32 = 0x00100000;
const V3_NO_ADAT_OPT_OUT_IFACE_A: u32 = 0x00040000;
const V3_NO_ADAT_OPT_OUT_IFACE_B: u32 = 0x00400000;

const V3_MSG_FLAG_CLK_CHANGED: u32 = 0x00000002;
const V3_CLK_WAIT_MSEC: c_uint = 4000;

extern "C" {
    static snd_motu_clock_rates: [c_uint; 0];

    fn snd_motu_transaction_read(
        motu: *mut snd_motu,
        offset: c_uint,
        buf: *mut c_void,
        size: usize,
    ) -> c_int;
    fn snd_motu_transaction_write(
        motu: *mut snd_motu,
        offset: c_uint,
        buf: *const c_void,
        size: usize,
    ) -> c_int;
    fn wait_event_interruptible_timeout(
        wq_head: *mut c_void,
        condition: c_int,
        timeout: c_long,
    ) -> c_long;
    fn msecs_to_jiffies(msecs: c_uint) -> c_long;
}

type c_long = core::ffi::c_long;

extern "C" {
    static EIO: c_int;
    static EINVAL: c_int;
    static ETIMEDOUT: c_int;
}

#[repr(C)]
pub struct snd_motu {
    pub msg: u32,
    pub hwdep_wait: *mut c_void,
    pub tx_packet_formats: snd_motu_packet_format,
    pub rx_packet_formats: snd_motu_packet_format,
    pub spec: *const snd_motu_spec,
}

#[repr(C)]
pub struct snd_motu_packet_format {
    pub pcm_byte_offset: c_uint,
    pub msg_chunks: c_uint,
    pub pcm_chunks: [c_uint; 3],
}

#[repr(C)]
pub struct snd_motu_spec {
    pub name: *const c_char,
    pub protocol_version: c_uint,
    pub flags: c_uint,
    pub tx_fixed_pcm_chunks: [c_uint; 3],
    pub rx_fixed_pcm_chunks: [c_uint; 3],
}

#[repr(C)]
pub enum snd_motu_clock_source {
    SND_MOTU_CLOCK_SOURCE_INTERNAL,
    SND_MOTU_CLOCK_SOURCE_WORD_ON_BNC,
    SND_MOTU_CLOCK_SOURCE_SPH,
    SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_A,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_A,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_B,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_B,
    SND_MOTU_CLOCK_SOURCE_UNKNOWN,
}

extern "C" {
    static SND_MOTU_PROTOCOL_V3: c_uint;
    static SND_MOTU_SPEC_RX_MIDI_3RD_Q: c_uint;
    static SND_MOTU_SPEC_TX_MIDI_3RD_Q: c_uint;
    static SND_MOTU_SPEC_COMMAND_DSP: c_uint;
    static SND_MOTU_SPEC_RX_MIDI_2ND_Q: c_uint;
    static SND_MOTU_SPEC_REGISTER_DSP: c_uint;
}

#[inline]
unsafe fn be32_to_cpu(v: __be32) -> u32 {
    u32::from_be(v)
}

#[inline]
unsafe fn cpu_to_be32(v: u32) -> __be32 {
    v.to_be()
}

#[inline]
unsafe fn array_size_snd_motu_clock_rates() -> usize {
    (&snd_motu_clock_rates as *const [c_uint; 0] as *const c_uint).len()
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v3_get_clock_rate(
    motu: *mut snd_motu,
    rate: *mut c_uint,
) -> c_int {
    let mut reg: __be32 = 0;
    let mut data: u32;
    let mut err: c_int;

    err = snd_motu_transaction_read(
        motu,
        V3_CLOCK_STATUS_OFFSET,
        &mut reg as *mut __be32 as *mut c_void,
        size_of::<__be32>(),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg);

    data = (data & V3_CLOCK_RATE_MASK) >> V3_CLOCK_RATE_SHIFT;
    if data as usize >= array_size_snd_motu_clock_rates() {
        return -EIO;
    }

    *rate = *(&snd_motu_clock_rates as *const [c_uint; 0] as *const c_uint).add(data as usize);

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v3_set_clock_rate(
    motu: *mut snd_motu,
    rate: c_uint,
) -> c_int {
    let mut reg: __be32 = 0;
    let mut data: u32;
    let mut need_to_wait: bool;
    let mut i: c_int = 0;
    let mut err: c_int;

    while (i as usize) < array_size_snd_motu_clock_rates() {
        if *(&snd_motu_clock_rates as *const [c_uint; 0] as *const c_uint).add(i as usize) == rate {
            break;
        }
        i += 1;
    }
    if i as usize == array_size_snd_motu_clock_rates() {
        return -EINVAL;
    }

    err = snd_motu_transaction_read(
        motu,
        V3_CLOCK_STATUS_OFFSET,
        &mut reg as *mut __be32 as *mut c_void,
        size_of::<__be32>(),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg);

    data &= !(V3_CLOCK_RATE_MASK | V3_FETCH_PCM_FRAMES);
    data |= (i as u32) << V3_CLOCK_RATE_SHIFT;

    need_to_wait = data != be32_to_cpu(reg);

    reg = cpu_to_be32(data);
    err = snd_motu_transaction_write(
        motu,
        V3_CLOCK_STATUS_OFFSET,
        &reg as *const __be32 as *const c_void,
        size_of::<__be32>(),
    );
    if err < 0 {
        return err;
    }

    if need_to_wait {
        let mut result: c_long;

        (*motu).msg = 0;
        result = wait_event_interruptible_timeout(
            (*motu).hwdep_wait,
            ((*motu).msg & V3_MSG_FLAG_CLK_CHANGED) as c_int,
            msecs_to_jiffies(V3_CLK_WAIT_MSEC),
        );
        if result < 0 {
            return result as c_int;
        }
        if result == 0 {
            return -ETIMEDOUT;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v3_get_clock_source(
    motu: *mut snd_motu,
    src: *mut snd_motu_clock_source,
) -> c_int {
    let mut reg: __be32 = 0;
    let mut data: u32;
    let mut err: c_int;

    err = snd_motu_transaction_read(
        motu,
        V3_CLOCK_STATUS_OFFSET,
        &mut reg as *mut __be32 as *mut c_void,
        size_of::<__be32>(),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg) & V3_CLOCK_SOURCE_MASK;

    match data {
        V3_CLOCK_SRC_INTERNAL => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_INTERNAL;
        }
        V3_CLOCK_SRC_WORD_ON_BNC => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_WORD_ON_BNC;
        }
        V3_CLOCK_SRC_SPH => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_SPH;
        }
        V3_CLOCK_SRC_AESEBU_ON_XLR => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR;
        }
        V3_CLOCK_SRC_SPDIF_ON_COAX => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX;
        }
        V3_CLOCK_SRC_OPT_IFACE_A | V3_CLOCK_SRC_OPT_IFACE_B => {
            let mut reg: __be32 = 0;
            let options: u32;

            err = snd_motu_transaction_read(
                motu,
                V3_OPT_IFACE_MODE_OFFSET,
                &mut reg as *mut __be32 as *mut c_void,
                size_of::<__be32>(),
            );
            if err < 0 {
                return err;
            }
            options = be32_to_cpu(reg);

            if data == V3_CLOCK_SRC_OPT_IFACE_A {
                if options & V3_NO_ADAT_OPT_IN_IFACE_A != 0 {
                    *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_A;
                } else {
                    *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_A;
                }
            } else {
                if options & V3_NO_ADAT_OPT_IN_IFACE_B != 0 {
                    *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT_B;
                } else {
                    *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT_B;
                }
            }
        }
        _ => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_UNKNOWN;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v3_switch_fetching_mode(
    motu: *mut snd_motu,
    enable: bool,
) -> c_int {
    let mut reg: __be32 = 0;
    let mut data: u32;
    let err: c_int;

    err = snd_motu_transaction_read(
        motu,
        V3_CLOCK_STATUS_OFFSET,
        &mut reg as *mut __be32 as *mut c_void,
        size_of::<__be32>(),
    );
    if err < 0 {
        return 0;
    }
    data = be32_to_cpu(reg);

    if enable {
        data |= V3_FETCH_PCM_FRAMES;
    } else {
        data &= !V3_FETCH_PCM_FRAMES;
    }

    reg = cpu_to_be32(data);
    snd_motu_transaction_write(
        motu,
        V3_CLOCK_STATUS_OFFSET,
        &reg as *const __be32 as *const c_void,
        size_of::<__be32>(),
    )
}

unsafe fn detect_packet_formats_with_opt_ifaces(motu: *mut snd_motu, data: u32) -> c_int {
    if data & V3_ENABLE_OPT_IN_IFACE_A != 0 {
        if data & V3_NO_ADAT_OPT_IN_IFACE_A != 0 {
            (*motu).tx_packet_formats.pcm_chunks[0] += 4;
            (*motu).tx_packet_formats.pcm_chunks[1] += 4;
        } else {
            (*motu).tx_packet_formats.pcm_chunks[0] += 8;
            (*motu).tx_packet_formats.pcm_chunks[1] += 4;
        }
    }

    if data & V3_ENABLE_OPT_IN_IFACE_B != 0 {
        if data & V3_NO_ADAT_OPT_IN_IFACE_B != 0 {
            (*motu).tx_packet_formats.pcm_chunks[0] += 4;
            (*motu).tx_packet_formats.pcm_chunks[1] += 4;
        } else {
            (*motu).tx_packet_formats.pcm_chunks[0] += 8;
            (*motu).tx_packet_formats.pcm_chunks[1] += 4;
        }
    }

    if data & V3_ENABLE_OPT_OUT_IFACE_A != 0 {
        if data & V3_NO_ADAT_OPT_OUT_IFACE_A != 0 {
            (*motu).rx_packet_formats.pcm_chunks[0] += 4;
            (*motu).rx_packet_formats.pcm_chunks[1] += 4;
        } else {
            (*motu).rx_packet_formats.pcm_chunks[0] += 8;
            (*motu).rx_packet_formats.pcm_chunks[1] += 4;
        }
    }

    if data & V3_ENABLE_OPT_OUT_IFACE_B != 0 {
        if data & V3_NO_ADAT_OPT_OUT_IFACE_B != 0 {
            (*motu).rx_packet_formats.pcm_chunks[0] += 4;
            (*motu).rx_packet_formats.pcm_chunks[1] += 4;
        } else {
            (*motu).rx_packet_formats.pcm_chunks[0] += 8;
            (*motu).rx_packet_formats.pcm_chunks[1] += 4;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v3_cache_packet_formats(motu: *mut snd_motu) -> c_int {
    let mut reg: __be32 = 0;
    let data: u32;
    let mut err: c_int;

    (*motu).tx_packet_formats.pcm_byte_offset = 10;
    (*motu).rx_packet_formats.pcm_byte_offset = 10;

    (*motu).tx_packet_formats.msg_chunks = 2;
    (*motu).rx_packet_formats.msg_chunks = 2;

    err = snd_motu_transaction_read(
        motu,
        V3_OPT_IFACE_MODE_OFFSET,
        &mut reg as *mut __be32 as *mut c_void,
        size_of::<__be32>(),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg);

    ptr::copy_nonoverlapping(
        (*(*motu).spec).tx_fixed_pcm_chunks.as_ptr(),
        (*motu).tx_packet_formats.pcm_chunks.as_mut_ptr(),
        (*motu).tx_packet_formats.pcm_chunks.len(),
    );
    ptr::copy_nonoverlapping(
        (*(*motu).spec).rx_fixed_pcm_chunks.as_ptr(),
        (*motu).rx_packet_formats.pcm_chunks.as_mut_ptr(),
        (*motu).rx_packet_formats.pcm_chunks.len(),
    );

    if (*motu).spec == &snd_motu_spec_828mk3_fw as *const snd_motu_spec
        || (*motu).spec == &snd_motu_spec_828mk3_hybrid as *const snd_motu_spec
        || (*motu).spec == &snd_motu_spec_896mk3 as *const snd_motu_spec
        || (*motu).spec == &snd_motu_spec_traveler_mk3 as *const snd_motu_spec
        || (*motu).spec == &snd_motu_spec_track16 as *const snd_motu_spec
    {
        detect_packet_formats_with_opt_ifaces(motu, data)
    } else {
        0
    }
}

#[no_mangle]
pub static snd_motu_spec_828mk3_fw: snd_motu_spec = snd_motu_spec {
    name: b"828mk3\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V3,
    flags: SND_MOTU_SPEC_RX_MIDI_3RD_Q | SND_MOTU_SPEC_TX_MIDI_3RD_Q | SND_MOTU_SPEC_COMMAND_DSP,
    tx_fixed_pcm_chunks: [18, 18, 14],
    rx_fixed_pcm_chunks: [14, 14, 10],
};

#[no_mangle]
pub static snd_motu_spec_828mk3_hybrid: snd_motu_spec = snd_motu_spec {
    name: b"828mk3\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V3,
    flags: SND_MOTU_SPEC_RX_MIDI_3RD_Q | SND_MOTU_SPEC_TX_MIDI_3RD_Q | SND_MOTU_SPEC_COMMAND_DSP,
    tx_fixed_pcm_chunks: [18, 18, 14],
    rx_fixed_pcm_chunks: [14, 14, 14], // Additional 4 dummy chunks at higher rate.
};

#[no_mangle]
pub static snd_motu_spec_896mk3: snd_motu_spec = snd_motu_spec {
    name: b"896mk3\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V3,
    flags: SND_MOTU_SPEC_COMMAND_DSP,
    tx_fixed_pcm_chunks: [18, 14, 10],
    rx_fixed_pcm_chunks: [18, 14, 10],
};

#[no_mangle]
pub static snd_motu_spec_traveler_mk3: snd_motu_spec = snd_motu_spec {
    name: b"TravelerMk3\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V3,
    flags: SND_MOTU_SPEC_RX_MIDI_3RD_Q | SND_MOTU_SPEC_TX_MIDI_3RD_Q | SND_MOTU_SPEC_COMMAND_DSP,
    tx_fixed_pcm_chunks: [18, 14, 10],
    rx_fixed_pcm_chunks: [14, 14, 10],
};

#[no_mangle]
pub static snd_motu_spec_ultralite_mk3: snd_motu_spec = snd_motu_spec {
    name: b"UltraLiteMk3\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V3,
    flags: SND_MOTU_SPEC_RX_MIDI_3RD_Q | SND_MOTU_SPEC_TX_MIDI_3RD_Q | SND_MOTU_SPEC_COMMAND_DSP,
    tx_fixed_pcm_chunks: [18, 14, 10],
    rx_fixed_pcm_chunks: [14, 14, 14],
};

#[no_mangle]
pub static snd_motu_spec_audio_express: snd_motu_spec = snd_motu_spec {
    name: b"AudioExpress\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V3,
    flags: SND_MOTU_SPEC_RX_MIDI_2ND_Q | SND_MOTU_SPEC_TX_MIDI_3RD_Q | SND_MOTU_SPEC_REGISTER_DSP,
    tx_fixed_pcm_chunks: [10, 10, 0],
    rx_fixed_pcm_chunks: [10, 10, 0],
};

#[no_mangle]
pub static snd_motu_spec_track16: snd_motu_spec = snd_motu_spec {
    name: b"Track16\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V3,
    flags: SND_MOTU_SPEC_RX_MIDI_3RD_Q | SND_MOTU_SPEC_TX_MIDI_3RD_Q | SND_MOTU_SPEC_COMMAND_DSP,
    tx_fixed_pcm_chunks: [14, 14, 14],
    rx_fixed_pcm_chunks: [6, 6, 6],
};

#[no_mangle]
pub static snd_motu_spec_4pre: snd_motu_spec = snd_motu_spec {
    name: b"4pre\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V3,
    flags: SND_MOTU_SPEC_REGISTER_DSP,
    tx_fixed_pcm_chunks: [10, 10, 0],
    rx_fixed_pcm_chunks: [10, 10, 0],
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
