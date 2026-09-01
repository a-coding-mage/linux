// SPDX-License-Identifier: GPL-2.0-only
// motu-protocol-v1.c - a part of driver for MOTU FireWire series
//
// Copyright (c) 2021 Takashi Sakamoto <o-takashi@sakamocchi.jp>

// C dependencies: "motu.h" and <linux/delay.h>.

type u32 = u32;
type __be32 = u32;

const ENXIO: i32 = 6;
const EINVAL: i32 = 22;

const SND_MOTU_PROTOCOL_V1: u32 = 1;

#[repr(C)]
pub struct snd_motu_packet_format {
    pub pcm_byte_offset: u32,
    pub msg_chunks: u32,
    pub pcm_chunks: [u32; 3],
}

#[repr(C)]
pub struct snd_motu {
    pub spec: *const snd_motu_spec,
    pub tx_packet_formats: snd_motu_packet_format,
    pub rx_packet_formats: snd_motu_packet_format,
}

#[repr(C)]
pub struct snd_motu_spec {
    pub name: *const i8,
    pub protocol_version: u32,
    pub tx_fixed_pcm_chunks: [u32; 3],
    pub rx_fixed_pcm_chunks: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum snd_motu_clock_source {
    SND_MOTU_CLOCK_SOURCE_INTERNAL,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT,
    SND_MOTU_CLOCK_SOURCE_SPH,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX,
    SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT,
    SND_MOTU_CLOCK_SOURCE_ADAT_ON_DSUB,
    SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR,
    SND_MOTU_CLOCK_SOURCE_WORD_ON_BNC,
}

extern "C" {
    fn snd_motu_transaction_read(
        motu: *mut snd_motu,
        offset: u32,
        buf: *mut __be32,
        size: usize,
    ) -> i32;
    fn snd_motu_transaction_write(
        motu: *mut snd_motu,
        offset: u32,
        buf: *const __be32,
        size: usize,
    ) -> i32;
    fn msleep(msecs: u32);
}

#[inline]
fn be32_to_cpu(v: __be32) -> u32 {
    u32::from_be(v)
}

#[inline]
fn cpu_to_be32(v: u32) -> __be32 {
    v.to_be()
}

// Status register for MOTU 828 (0x'ffff'f000'0b00).
//
// 0xffff0000: ISOC_COMM_CONTROL_MASK in motu-stream.c.
// 0x00008000: mode of optical input interface.
//   0x00008000: for S/PDIF signal.
//   0x00000000: disabled or for ADAT signal.
// 0x00004000: mode of optical output interface.
//   0x00004000: for S/PDIF signal.
//   0x00000000: disabled or for ADAT signal.
// 0x00003f00: monitor input mode.
//   0x00000800: analog-1/2
//   0x00001a00: analog-3/4
//   0x00002c00: analog-5/6
//   0x00003e00: analog-7/8
//   0x00000000: analog-1
//   0x00000900: analog-2
//   0x00001200: analog-3
//   0x00001b00: analog-4
//   0x00002400: analog-5
//   0x00002d00: analog-6
//   0x00003600: analog-7
//   0x00003f00: analog-8
// 0x00000080: enable stream input.
// 0x00000040: disable monitor input.
// 0x00000008: enable main out.
// 0x00000004: rate of sampling clock.
//   0x00000004: 48.0 kHz
//   0x00000000: 44.1 kHz
// 0x00000023: source of sampling clock.
//   0x00000003: source packet header (SPH)
//   0x00000002: S/PDIF on optical/coaxial interface.
//   0x00000021: ADAT on optical interface
//   0x00000001: ADAT on Dsub 9pin
//   0x00000000: internal

const CLK_828_STATUS_OFFSET: u32 = 0x0b00;
const CLK_828_STATUS_MASK: u32 = 0x0000ffff;
const CLK_828_STATUS_FLAG_OPT_IN_IFACE_IS_SPDIF: u32 = 0x00008000;
const CLK_828_STATUS_FLAG_OPT_OUT_IFACE_IS_SPDIF: u32 = 0x00004000;
const CLK_828_STATUS_FLAG_FETCH_PCM_FRAMES: u32 = 0x00000080;
const CLK_828_STATUS_FLAG_ENABLE_OUTPUT: u32 = 0x00000008;
const CLK_828_STATUS_FLAG_RATE_48000: u32 = 0x00000004;
const CLK_828_STATUS_MASK_SRC: u32 = 0x00000023;
const CLK_828_STATUS_FLAG_SRC_ADAT_ON_OPT: u32 = 0x00000021;
const CLK_828_STATUS_FLAG_SRC_SPH: u32 = 0x00000003;
const CLK_828_STATUS_FLAG_SRC_SPDIF: u32 = 0x00000002;
const CLK_828_STATUS_FLAG_SRC_ADAT_ON_DSUB: u32 = 0x00000001;
const CLK_828_STATUS_FLAG_SRC_INTERNAL: u32 = 0x00000000;

// Status register for MOTU 896 (0x'ffff'f000'0b14).
//
// 0xf0000000: enable physical and stream input to DAC.
//   0x80000000: disable
//   0x40000000: disable
//   0x20000000: enable (prior to the other bits)
//   0x10000000: disable
//   0x00000000: disable
// 0x08000000: speed of word clock signal output on BNC interface.
//   0x00000000: force to low rate (44.1/48.0 kHz).
//   0x08000000: follow to system clock.
// 0x04000000: something relevant to clock.
// 0x03000000: enable output.
//  0x02000000: enabled irreversibly once standing unless the device voluntarily disables it.
//  0x01000000: enabled irreversibly once standing unless the device voluntarily disables it.
// 0x00ffff00: monitor input mode.
//   0x00000000: disabled
//   0x00004800: analog-1/2
//   0x00005a00: analog-3/4
//   0x00006c00: analog-5/6
//   0x00007e00: analog-7/8
//   0x00104800: AES/EBU-1/2
//   0x00004000: analog-1
//   0x00004900: analog-2
//   0x00005200: analog-3
//   0x00005b00: analog-4
//   0x00006400: analog-5
//   0x00006d00: analog-6
//   0x00007600: analog-7
//   0x00007f00: analog-8
//   0x00104000: AES/EBU-1
//   0x00104900: AES/EBU-2
// 0x00000060: sample rate conversion for AES/EBU input/output.
//   0x00000000: None
//   0x00000020: input signal is converted to system rate
//   0x00000040: output is slave to input, ignoring system rate
//   0x00000060: output is double rate than system rate
// 0x00000018: nominal rate of sampling clock.
//   0x00000000: 44.1 kHz
//   0x00000008: 48.0 kHz
//   0x00000010: 88.2 kHz
//   0x00000018: 96.0 kHz
// 0x00000007: source of sampling clock.
//   0x00000000: internal
//   0x00000001: ADAT on optical interface
//   0x00000002: AES/EBU on XLR
//   0x00000003: source packet header (SPH)
//   0x00000004: word clock on BNC
//   0x00000005: ADAT on Dsub 9pin

const CLK_896_STATUS_OFFSET: u32 = 0x0b14;
const CLK_896_STATUS_FLAG_FETCH_ENABLE: u32 = 0x20000000;
const CLK_896_STATUS_FLAG_OUTPUT_ON: u32 = 0x03000000;
const CLK_896_STATUS_MASK_SRC: u32 = 0x00000007;
const CLK_896_STATUS_FLAG_SRC_INTERNAL: u32 = 0x00000000;
const CLK_896_STATUS_FLAG_SRC_ADAT_ON_OPT: u32 = 0x00000001;
const CLK_896_STATUS_FLAG_SRC_AESEBU: u32 = 0x00000002;
const CLK_896_STATUS_FLAG_SRC_SPH: u32 = 0x00000003;
const CLK_896_STATUS_FLAG_SRC_WORD: u32 = 0x00000004;
const CLK_896_STATUS_FLAG_SRC_ADAT_ON_DSUB: u32 = 0x00000005;
const CLK_896_STATUS_MASK_RATE: u32 = 0x00000018;
const CLK_896_STATUS_FLAG_RATE_44100: u32 = 0x00000000;
const CLK_896_STATUS_FLAG_RATE_48000: u32 = 0x00000008;
const CLK_896_STATUS_FLAG_RATE_88200: u32 = 0x00000010;
const CLK_896_STATUS_FLAG_RATE_96000: u32 = 0x00000018;

unsafe fn parse_clock_rate_828(data: u32, rate: *mut u32) {
    if data & CLK_828_STATUS_FLAG_RATE_48000 != 0 {
        *rate = 48000;
    } else {
        *rate = 44100;
    }
}

unsafe fn get_clock_rate_828(motu: *mut snd_motu, rate: *mut u32) -> i32 {
    let mut reg: __be32 = 0;
    let err: i32;

    err = snd_motu_transaction_read(
        motu,
        CLK_828_STATUS_OFFSET,
        &mut reg,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    parse_clock_rate_828(be32_to_cpu(reg), rate);

    0
}

unsafe fn parse_clock_rate_896(data: u32, rate: *mut u32) -> i32 {
    match data & CLK_896_STATUS_MASK_RATE {
        CLK_896_STATUS_FLAG_RATE_44100 => {
            *rate = 44100;
        }
        CLK_896_STATUS_FLAG_RATE_48000 => {
            *rate = 48000;
        }
        CLK_896_STATUS_FLAG_RATE_88200 => {
            *rate = 88200;
        }
        CLK_896_STATUS_FLAG_RATE_96000 => {
            *rate = 96000;
        }
        _ => return -ENXIO,
    }

    0
}

unsafe fn get_clock_rate_896(motu: *mut snd_motu, rate: *mut u32) -> i32 {
    let mut reg: __be32 = 0;
    let err: i32;

    err = snd_motu_transaction_read(
        motu,
        CLK_896_STATUS_OFFSET,
        &mut reg,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    parse_clock_rate_896(be32_to_cpu(reg), rate)
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v1_get_clock_rate(
    motu: *mut snd_motu,
    rate: *mut u32,
) -> i32 {
    if (*motu).spec == &snd_motu_spec_828 as *const snd_motu_spec {
        get_clock_rate_828(motu, rate)
    } else if (*motu).spec == &snd_motu_spec_896 as *const snd_motu_spec {
        get_clock_rate_896(motu, rate)
    } else {
        -ENXIO
    }
}

unsafe fn set_clock_rate_828(motu: *mut snd_motu, rate: u32) -> i32 {
    let mut reg: __be32 = 0;
    let mut data: u32;
    let err: i32;

    err = snd_motu_transaction_read(
        motu,
        CLK_828_STATUS_OFFSET,
        &mut reg,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg) & CLK_828_STATUS_MASK;

    data &= !CLK_828_STATUS_FLAG_RATE_48000;
    if rate == 48000 {
        data |= CLK_828_STATUS_FLAG_RATE_48000;
    }

    reg = cpu_to_be32(data);
    snd_motu_transaction_write(
        motu,
        CLK_828_STATUS_OFFSET,
        &reg,
        core::mem::size_of_val(&reg),
    )
}

unsafe fn set_clock_rate_896(motu: *mut snd_motu, rate: u32) -> i32 {
    let flag: u32;
    let mut reg: __be32 = 0;
    let mut data: u32;
    let err: i32;

    err = snd_motu_transaction_read(
        motu,
        CLK_896_STATUS_OFFSET,
        &mut reg,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg);

    match rate {
        44100 => {
            flag = CLK_896_STATUS_FLAG_RATE_44100;
        }
        48000 => {
            flag = CLK_896_STATUS_FLAG_RATE_48000;
        }
        88200 => {
            flag = CLK_896_STATUS_FLAG_RATE_88200;
        }
        96000 => {
            flag = CLK_896_STATUS_FLAG_RATE_96000;
        }
        _ => return -EINVAL,
    }

    data &= !CLK_896_STATUS_MASK_RATE;
    data |= flag;

    reg = cpu_to_be32(data);
    snd_motu_transaction_write(
        motu,
        CLK_896_STATUS_OFFSET,
        &reg,
        core::mem::size_of_val(&reg),
    )
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v1_set_clock_rate(
    motu: *mut snd_motu,
    rate: u32,
) -> i32 {
    if (*motu).spec == &snd_motu_spec_828 as *const snd_motu_spec {
        set_clock_rate_828(motu, rate)
    } else if (*motu).spec == &snd_motu_spec_896 as *const snd_motu_spec {
        set_clock_rate_896(motu, rate)
    } else {
        -ENXIO
    }
}

unsafe fn get_clock_source_828(
    motu: *mut snd_motu,
    src: *mut snd_motu_clock_source,
) -> i32 {
    let mut reg: __be32 = 0;
    let data: u32;
    let err: i32;

    err = snd_motu_transaction_read(
        motu,
        CLK_828_STATUS_OFFSET,
        &mut reg,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg) & CLK_828_STATUS_MASK;

    match data & CLK_828_STATUS_MASK_SRC {
        CLK_828_STATUS_FLAG_SRC_ADAT_ON_OPT => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT;
        }
        CLK_828_STATUS_FLAG_SRC_SPH => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_SPH;
        }
        CLK_828_STATUS_FLAG_SRC_SPDIF => {
            if data & CLK_828_STATUS_FLAG_OPT_IN_IFACE_IS_SPDIF != 0 {
                *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_SPDIF_ON_COAX;
            } else {
                *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_SPDIF_ON_OPT;
            }
        }
        CLK_828_STATUS_FLAG_SRC_ADAT_ON_DSUB => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_ADAT_ON_DSUB;
        }
        CLK_828_STATUS_FLAG_SRC_INTERNAL => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_INTERNAL;
        }
        _ => return -ENXIO,
    }

    0
}

unsafe fn get_clock_source_896(
    motu: *mut snd_motu,
    src: *mut snd_motu_clock_source,
) -> i32 {
    let mut reg: __be32 = 0;
    let data: u32;
    let err: i32;

    err = snd_motu_transaction_read(
        motu,
        CLK_896_STATUS_OFFSET,
        &mut reg,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg);

    match data & CLK_896_STATUS_MASK_SRC {
        CLK_896_STATUS_FLAG_SRC_INTERNAL => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_INTERNAL;
        }
        CLK_896_STATUS_FLAG_SRC_ADAT_ON_OPT => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_ADAT_ON_OPT;
        }
        CLK_896_STATUS_FLAG_SRC_AESEBU => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_AESEBU_ON_XLR;
        }
        CLK_896_STATUS_FLAG_SRC_SPH => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_SPH;
        }
        CLK_896_STATUS_FLAG_SRC_WORD => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_WORD_ON_BNC;
        }
        CLK_896_STATUS_FLAG_SRC_ADAT_ON_DSUB => {
            *src = snd_motu_clock_source::SND_MOTU_CLOCK_SOURCE_ADAT_ON_DSUB;
        }
        _ => return -ENXIO,
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v1_get_clock_source(
    motu: *mut snd_motu,
    src: *mut snd_motu_clock_source,
) -> i32 {
    if (*motu).spec == &snd_motu_spec_828 as *const snd_motu_spec {
        get_clock_source_828(motu, src)
    } else if (*motu).spec == &snd_motu_spec_896 as *const snd_motu_spec {
        get_clock_source_896(motu, src)
    } else {
        -ENXIO
    }
}

unsafe fn switch_fetching_mode_828(motu: *mut snd_motu, enable: bool) -> i32 {
    let mut reg: __be32 = 0;
    let mut data: u32;
    let err: i32;

    err = snd_motu_transaction_read(
        motu,
        CLK_828_STATUS_OFFSET,
        &mut reg,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg) & CLK_828_STATUS_MASK;

    data &= !(CLK_828_STATUS_FLAG_FETCH_PCM_FRAMES | CLK_828_STATUS_FLAG_ENABLE_OUTPUT);
    if enable {
        // This transaction should be initiated after the device receives batch of packets
        // since the device voluntarily mutes outputs. As a workaround, yield processor over
        // 100 msec.
        msleep(100);
        data |= CLK_828_STATUS_FLAG_FETCH_PCM_FRAMES | CLK_828_STATUS_FLAG_ENABLE_OUTPUT;
    }

    reg = cpu_to_be32(data);
    snd_motu_transaction_write(
        motu,
        CLK_828_STATUS_OFFSET,
        &reg,
        core::mem::size_of_val(&reg),
    )
}

unsafe fn switch_fetching_mode_896(motu: *mut snd_motu, enable: bool) -> i32 {
    let mut reg: __be32 = 0;
    let mut data: u32;
    let err: i32;

    err = snd_motu_transaction_read(
        motu,
        CLK_896_STATUS_OFFSET,
        &mut reg,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg);

    data &= !CLK_896_STATUS_FLAG_FETCH_ENABLE;
    if enable {
        data |= CLK_896_STATUS_FLAG_FETCH_ENABLE | CLK_896_STATUS_FLAG_OUTPUT_ON;
    }

    reg = cpu_to_be32(data);
    snd_motu_transaction_write(
        motu,
        CLK_896_STATUS_OFFSET,
        &reg,
        core::mem::size_of_val(&reg),
    )
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v1_switch_fetching_mode(
    motu: *mut snd_motu,
    enable: bool,
) -> i32 {
    if (*motu).spec == &snd_motu_spec_828 as *const snd_motu_spec {
        switch_fetching_mode_828(motu, enable)
    } else if (*motu).spec == &snd_motu_spec_896 as *const snd_motu_spec {
        switch_fetching_mode_896(motu, enable)
    } else {
        -ENXIO
    }
}

unsafe fn detect_packet_formats_828(motu: *mut snd_motu) -> i32 {
    let mut reg: __be32 = 0;
    let data: u32;
    let err: i32;

    (*motu).tx_packet_formats.pcm_byte_offset = 4;
    (*motu).tx_packet_formats.msg_chunks = 2;

    (*motu).rx_packet_formats.pcm_byte_offset = 4;
    (*motu).rx_packet_formats.msg_chunks = 0;

    err = snd_motu_transaction_read(
        motu,
        CLK_828_STATUS_OFFSET,
        &mut reg,
        core::mem::size_of_val(&reg),
    );
    if err < 0 {
        return err;
    }
    data = be32_to_cpu(reg) & CLK_828_STATUS_MASK;

    // The number of chunks is just reduced when SPDIF is activated.
    if !(data & CLK_828_STATUS_FLAG_OPT_IN_IFACE_IS_SPDIF != 0) {
        (*motu).tx_packet_formats.pcm_chunks[0] += 8;
    }

    if !(data & CLK_828_STATUS_FLAG_OPT_OUT_IFACE_IS_SPDIF != 0) {
        (*motu).rx_packet_formats.pcm_chunks[0] += 8;
    }

    0
}

unsafe fn detect_packet_formats_896(motu: *mut snd_motu) -> i32 {
    // 24bit PCM frames follow to source packet header without message chunk.
    (*motu).tx_packet_formats.pcm_byte_offset = 4;
    (*motu).rx_packet_formats.pcm_byte_offset = 4;

    // No message chunk in data block.
    (*motu).tx_packet_formats.msg_chunks = 0;
    (*motu).rx_packet_formats.msg_chunks = 0;

    // Always enable optical interface for ADAT signal since the device have no registers
    // to refer to current configuration.
    (*motu).tx_packet_formats.pcm_chunks[0] += 8;
    (*motu).tx_packet_formats.pcm_chunks[1] += 8;

    (*motu).rx_packet_formats.pcm_chunks[0] += 8;
    (*motu).rx_packet_formats.pcm_chunks[1] += 8;

    0
}

#[no_mangle]
pub unsafe extern "C" fn snd_motu_protocol_v1_cache_packet_formats(
    motu: *mut snd_motu,
) -> i32 {
    (*motu).tx_packet_formats.pcm_chunks = (*(*motu).spec).tx_fixed_pcm_chunks;
    (*motu).rx_packet_formats.pcm_chunks = (*(*motu).spec).rx_fixed_pcm_chunks;

    if (*motu).spec == &snd_motu_spec_828 as *const snd_motu_spec {
        detect_packet_formats_828(motu)
    } else if (*motu).spec == &snd_motu_spec_896 as *const snd_motu_spec {
        detect_packet_formats_896(motu)
    } else {
        0
    }
}

#[no_mangle]
pub static snd_motu_spec_828: snd_motu_spec = snd_motu_spec {
    name: b"828\0".as_ptr() as *const i8,
    protocol_version: SND_MOTU_PROTOCOL_V1,
    tx_fixed_pcm_chunks: [10, 0, 0],
    rx_fixed_pcm_chunks: [10, 0, 0],
};

#[no_mangle]
pub static snd_motu_spec_896: snd_motu_spec = snd_motu_spec {
    name: b"896\0".as_ptr() as *const i8,
    protocol_version: 0,
    tx_fixed_pcm_chunks: [10, 10, 0],
    rx_fixed_pcm_chunks: [10, 10, 0],
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
