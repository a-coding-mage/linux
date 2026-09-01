// SPDX-License-Identifier: GPL-2.0-only
/*
 * motu-protocol-v2.c - a part of driver for MOTU FireWire series
 *
 * Copyright (c) 2015-2017 Takashi Sakamoto <o-takashi@sakamocchi.jp>
 */

// C dependency intent: #include "motu.h"

use core::mem::size_of_val;
use core::ptr;

const V2_CLOCK_STATUS_OFFSET: u32 = 0x0b14;
const V2_CLOCK_RATE_MASK: u32 = 0x00000038;
const V2_CLOCK_RATE_SHIFT: u32 = 3;
const V2_CLOCK_SRC_MASK: u32 = 0x00000007;
const V2_CLOCK_SRC_SHIFT: u32 = 0;
const V2_CLOCK_SRC_AESEBU_ON_XLR: u32 = 0x07; // In Traveler.
const V2_CLOCK_SRC_ADAT_ON_DSUB: u32 = 0x05;
const V2_CLOCK_SRC_WORD_ON_BNC: u32 = 0x04;
const V2_CLOCK_SRC_SPH: u32 = 0x03;
const V2_CLOCK_SRC_SPDIF: u32 = 0x02; // on either coaxial or optical. AES/EBU in 896HD.
const V2_CLOCK_SRC_ADAT_ON_OPT: u32 = 0x01;
const V2_CLOCK_SRC_INTERNAL: u32 = 0x00;
const V2_CLOCK_FETCH_ENABLE: u32 = 0x02000000;
const V2_CLOCK_MODEL_SPECIFIC: u32 = 0x04000000;

const V2_IN_OUT_CONF_OFFSET: u32 = 0x0c04;
const V2_OPT_OUT_IFACE_MASK: u32 = 0x00000c00;
const V2_OPT_OUT_IFACE_SHIFT: u32 = 10;
const V2_OPT_IN_IFACE_MASK: u32 = 0x00000300;
const V2_OPT_IN_IFACE_SHIFT: u32 = 8;
const V2_OPT_IFACE_MODE_NONE: u32 = 0;
const V2_OPT_IFACE_MODE_ADAT: u32 = 1;
const V2_OPT_IFACE_MODE_SPDIF: u32 = 2;

unsafe fn get_clock_rate(data: u32, rate: *mut c_uint) -> c_int {
    let index = ((data & V2_CLOCK_RATE_MASK) >> V2_CLOCK_RATE_SHIFT) as usize;
    if index >= snd_motu_clock_rates.len() {
        return -EIO;
    }

    unsafe {
        *rate = snd_motu_clock_rates[index];
    }

    0
}

pub unsafe extern "C" fn snd_motu_protocol_v2_get_clock_rate(
    motu: *mut snd_motu,
    rate: *mut c_uint,
) -> c_int {
    let mut reg: __be32 = 0;
    let mut err: c_int;

    err = unsafe {
        snd_motu_transaction_read(
            motu,
            V2_CLOCK_STATUS_OFFSET,
            &mut reg as *mut __be32 as *mut _,
            size_of_val(&reg),
        )
    };
    if err < 0 {
        return err;
    }

    unsafe { get_clock_rate(be32_to_cpu(reg), rate) }
}

pub unsafe extern "C" fn snd_motu_protocol_v2_set_clock_rate(
    motu: *mut snd_motu,
    rate: c_uint,
) -> c_int {
    let mut reg: __be32 = 0;
    let mut data: u32;
    let mut i: usize;
    let mut err: c_int;

    i = 0;
    while i < snd_motu_clock_rates.len() {
        if snd_motu_clock_rates[i] == rate {
            break;
        }
        i += 1;
    }
    if i == snd_motu_clock_rates.len() {
        return -EINVAL;
    }

    err = unsafe {
        snd_motu_transaction_read(
            motu,
            V2_CLOCK_STATUS_OFFSET,
            &mut reg as *mut __be32 as *mut _,
            size_of_val(&reg),
        )
    };
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg);

    data &= !V2_CLOCK_RATE_MASK;
    data |= (i as u32) << V2_CLOCK_RATE_SHIFT;

    reg = cpu_to_be32(data);
    unsafe {
        snd_motu_transaction_write(
            motu,
            V2_CLOCK_STATUS_OFFSET,
            &mut reg as *mut __be32 as *mut _,
            size_of_val(&reg),
        )
    }
}

unsafe fn get_clock_source(
    motu: *mut snd_motu,
    data: u32,
    src: *mut snd_motu_clock_source,
) -> c_int {
    match data & V2_CLOCK_SRC_MASK {
        V2_CLOCK_SRC_INTERNAL => unsafe {
            *src = SND_MOTU_CLOCK_SOURCE_INTERNAL;
        },
        V2_CLOCK_SRC_ADAT_ON_OPT => unsafe {
            *src = SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT;
        },
        V2_CLOCK_SRC_SPDIF => {
            let support_iec60958_on_opt: bool = unsafe {
                (*motu).spec == &snd_motu_spec_828mk2 as *const snd_motu_spec
                    || (*motu).spec == &snd_motu_spec_traveler as *const snd_motu_spec
            };

            unsafe {
                if (*motu).spec == &snd_motu_spec_896hd as *const snd_motu_spec {
                    *src = SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR;
                } else if !support_iec60958_on_opt {
                    *src = SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX;
                } else {
                    let mut reg: __be32 = 0;

                    // To check the configuration of optical interface.
                    let err = snd_motu_transaction_read(
                        motu,
                        V2_IN_OUT_CONF_OFFSET,
                        &mut reg as *mut __be32 as *mut _,
                        size_of_val(&reg),
                    );
                    if err < 0 {
                        return err;
                    }

                    if ((data & V2_OPT_IN_IFACE_MASK) >> V2_OPT_IN_IFACE_SHIFT)
                        == V2_OPT_IFACE_MODE_SPDIF
                    {
                        *src = SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT;
                    } else {
                        *src = SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX;
                    }
                }
            }
        }
        V2_CLOCK_SRC_SPH => unsafe {
            *src = SND_MOTU_CLOCK_SOURCE_SPH;
        },
        V2_CLOCK_SRC_WORD_ON_BNC => unsafe {
            *src = SND_MOTU_CLOCK_SOURCE_WORD_ON_BNC;
        },
        V2_CLOCK_SRC_ADAT_ON_DSUB => unsafe {
            *src = SND_MOTU_CLOCK_SOURCE_ADAT_ON_DSUB;
        },
        V2_CLOCK_SRC_AESEBU_ON_XLR => unsafe {
            // For Traveler.
            *src = SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR;
        },
        _ => unsafe {
            *src = SND_MOTU_CLOCK_SOURCE_UNKNOWN;
        },
    }

    0
}

pub unsafe extern "C" fn snd_motu_protocol_v2_get_clock_source(
    motu: *mut snd_motu,
    src: *mut snd_motu_clock_source,
) -> c_int {
    let mut reg: __be32 = 0;
    let mut err: c_int;

    err = unsafe {
        snd_motu_transaction_read(
            motu,
            V2_CLOCK_STATUS_OFFSET,
            &mut reg as *mut __be32 as *mut _,
            size_of_val(&reg),
        )
    };
    if err < 0 {
        return err;
    }

    unsafe { get_clock_source(motu, be32_to_cpu(reg), src) }
}

// Expected for Traveler, which implements Altera Cyclone EP1C3.
unsafe fn switch_fetching_mode_cyclone(
    _motu: *mut snd_motu,
    data: *mut u32,
    _enable: bool,
) -> c_int {
    unsafe {
        *data |= V2_CLOCK_MODEL_SPECIFIC;
    }

    0
}

// For UltraLite and 8pre, which implements Xilinx Spartan XC3S200.
unsafe fn switch_fetching_mode_spartan(
    motu: *mut snd_motu,
    data: *mut u32,
    _enable: bool,
) -> c_int {
    let mut rate: c_uint = 0;
    let mut src: snd_motu_clock_source = SND_MOTU_CLOCK_SOURCE_UNKNOWN;
    let mut err: c_int;

    err = unsafe { get_clock_source(motu, *data, &mut src as *mut snd_motu_clock_source) };
    if err < 0 {
        return err;
    }

    err = unsafe { get_clock_rate(*data, &mut rate as *mut c_uint) };
    if err < 0 {
        return err;
    }

    if src == SND_MOTU_CLOCK_SOURCE_SPH && rate > 48000 {
        unsafe {
            *data |= V2_CLOCK_MODEL_SPECIFIC;
        }
    }

    0
}

pub unsafe extern "C" fn snd_motu_protocol_v2_switch_fetching_mode(
    motu: *mut snd_motu,
    enable: bool,
) -> c_int {
    unsafe {
        if (*motu).spec == &snd_motu_spec_828mk2 as *const snd_motu_spec {
            // 828mkII implements Altera ACEX 1K EP1K30. Nothing to do.
            0
        } else if (*motu).spec == &snd_motu_spec_896hd as *const snd_motu_spec {
            // 896HD implements Altera Cyclone EP1C3 but nothing to do.
            0
        } else {
            let mut reg: __be32 = 0;
            let mut data: u32;
            let mut err: c_int;

            err = snd_motu_transaction_read(
                motu,
                V2_CLOCK_STATUS_OFFSET,
                &mut reg as *mut __be32 as *mut _,
                size_of_val(&reg),
            );
            if err < 0 {
                return err;
            }
            data = be32_to_cpu(reg);

            data &= !(V2_CLOCK_FETCH_ENABLE | V2_CLOCK_MODEL_SPECIFIC);
            if enable {
                data |= V2_CLOCK_FETCH_ENABLE;
            }

            if (*motu).spec == &snd_motu_spec_traveler as *const snd_motu_spec {
                err = switch_fetching_mode_cyclone(motu, &mut data as *mut u32, enable);
            } else {
                err = switch_fetching_mode_spartan(motu, &mut data as *mut u32, enable);
            }
            if err < 0 {
                return err;
            }

            reg = cpu_to_be32(data);
            snd_motu_transaction_write(
                motu,
                V2_CLOCK_STATUS_OFFSET,
                &mut reg as *mut __be32 as *mut _,
                size_of_val(&reg),
            )
        }
    }
}

pub unsafe extern "C" fn snd_motu_protocol_v2_cache_packet_formats(
    motu: *mut snd_motu,
) -> c_int {
    let has_two_opt_ifaces: bool =
        unsafe { (*motu).spec == &snd_motu_spec_8pre as *const snd_motu_spec };
    let mut reg: __be32 = 0;
    let mut data: u32;
    let mut err: c_int;

    unsafe {
        (*motu).tx_packet_formats.pcm_byte_offset = 10;
        (*motu).rx_packet_formats.pcm_byte_offset = 10;

        (*motu).tx_packet_formats.msg_chunks = 2;
        (*motu).rx_packet_formats.msg_chunks = 2;

        err = snd_motu_transaction_read(
            motu,
            V2_IN_OUT_CONF_OFFSET,
            &mut reg as *mut __be32 as *mut _,
            size_of_val(&reg),
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

        if ((data & V2_OPT_IN_IFACE_MASK) >> V2_OPT_IN_IFACE_SHIFT) == V2_OPT_IFACE_MODE_ADAT {
            (*motu).tx_packet_formats.pcm_chunks[0] += 8;

            if !has_two_opt_ifaces {
                (*motu).tx_packet_formats.pcm_chunks[1] += 4;
            } else {
                (*motu).tx_packet_formats.pcm_chunks[1] += 8;
            }
        }

        if ((data & V2_OPT_OUT_IFACE_MASK) >> V2_OPT_OUT_IFACE_SHIFT) == V2_OPT_IFACE_MODE_ADAT {
            (*motu).rx_packet_formats.pcm_chunks[0] += 8;

            if !has_two_opt_ifaces {
                (*motu).rx_packet_formats.pcm_chunks[1] += 4;
            } else {
                (*motu).rx_packet_formats.pcm_chunks[1] += 8;
            }
        }
    }

    0
}

pub static snd_motu_spec_828mk2: snd_motu_spec = snd_motu_spec {
    name: b"828mk2\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V2,
    flags: SND_MOTU_SPEC_RX_MIDI_2ND_Q
        | SND_MOTU_SPEC_TX_MIDI_2ND_Q
        | SND_MOTU_SPEC_REGISTER_DSP,
    tx_fixed_pcm_chunks: [14, 14, 0],
    rx_fixed_pcm_chunks: [14, 14, 0],
};

pub static snd_motu_spec_896hd: snd_motu_spec = snd_motu_spec {
    name: b"896HD\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V2,
    flags: SND_MOTU_SPEC_REGISTER_DSP,
    tx_fixed_pcm_chunks: [14, 14, 8],
    rx_fixed_pcm_chunks: [14, 14, 8],
};

pub static snd_motu_spec_traveler: snd_motu_spec = snd_motu_spec {
    name: b"Traveler\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V2,
    flags: SND_MOTU_SPEC_RX_MIDI_2ND_Q
        | SND_MOTU_SPEC_TX_MIDI_2ND_Q
        | SND_MOTU_SPEC_REGISTER_DSP,
    tx_fixed_pcm_chunks: [14, 14, 8],
    rx_fixed_pcm_chunks: [14, 14, 8],
};

pub static snd_motu_spec_ultralite: snd_motu_spec = snd_motu_spec {
    name: b"UltraLite\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V2,
    flags: SND_MOTU_SPEC_RX_MIDI_2ND_Q
        | SND_MOTU_SPEC_TX_MIDI_2ND_Q
        | SND_MOTU_SPEC_REGISTER_DSP,
    tx_fixed_pcm_chunks: [14, 14, 0],
    rx_fixed_pcm_chunks: [14, 14, 0],
};

pub static snd_motu_spec_8pre: snd_motu_spec = snd_motu_spec {
    name: b"8pre\0".as_ptr() as *const c_char,
    protocol_version: SND_MOTU_PROTOCOL_V2,
    flags: SND_MOTU_SPEC_RX_MIDI_2ND_Q
        | SND_MOTU_SPEC_TX_MIDI_2ND_Q
        | SND_MOTU_SPEC_REGISTER_DSP,
    // Two dummy chunks always in the end of data block.
    tx_fixed_pcm_chunks: [10, 10, 0],
    rx_fixed_pcm_chunks: [6, 6, 0],
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
