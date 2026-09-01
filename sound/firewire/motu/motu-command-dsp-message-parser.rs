// SPDX-License-Identifier: GPL-2.0-only
//
// motu-command-dsp-message-parser.c - a part of driver for MOTU FireWire series
//
// Copyright (c) 2021 Takashi Sakamoto <o-takashi@sakamocchi.jp>

// Below models allow software to configure their DSP function by command transferred in
// asynchronous transaction:
//  * 828 mk3 (FireWire only and Hybrid)
//  * 896 mk3 (FireWire only and Hybrid)
//  * Ultralite mk3 (FireWire only and Hybrid)
//  * Traveler mk3
//  * Track 16
//
// Isochronous packets from the above models includes messages to report state of hardware meter.

// Dependencies originally provided by "motu.h".

#[repr(C)]
enum msg_parser_state {
    INITIALIZED,
    FRAGMENT_DETECTED,
    AVAILABLE,
}

#[repr(C)]
struct msg_parser {
    lock: spinlock_t,
    state: msg_parser_state,
    interval: ::core::ffi::c_uint,
    message_count: ::core::ffi::c_uint,
    fragment_pos: ::core::ffi::c_uint,
    value_index: ::core::ffi::c_uint,
    value: u64,
    meter: snd_firewire_motu_command_dsp_meter,
}

extern "C" {
    fn devm_kzalloc(
        dev: *mut ::core::ffi::c_void,
        size: usize,
        flags: ::core::ffi::c_uint,
    ) -> *mut ::core::ffi::c_void;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn amdtp_stream_next_packet_desc(
        s: *const amdtp_stream,
        desc: *const pkt_desc,
    ) -> *const pkt_desc;
    fn memcpy(
        dest: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: usize,
    ) -> *mut ::core::ffi::c_void;
}

pub unsafe fn snd_motu_command_dsp_message_parser_new(motu: *mut snd_motu) -> ::core::ffi::c_int {
    let parser: *mut msg_parser;

    parser = devm_kzalloc(
        &mut (*(*motu).card).card_dev as *mut _ as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<msg_parser>(),
        GFP_KERNEL,
    ) as *mut msg_parser;
    if parser.is_null() {
        return -ENOMEM;
    }
    spin_lock_init(&mut (*parser).lock);
    (*motu).message_parser = parser as *mut ::core::ffi::c_void;

    0
}

pub unsafe fn snd_motu_command_dsp_message_parser_init(
    motu: *mut snd_motu,
    sfc: cip_sfc,
) -> ::core::ffi::c_int {
    let parser = (*motu).message_parser as *mut msg_parser;

    (*parser).state = msg_parser_state::INITIALIZED;

    // All of data blocks don't have messages with meaningful information.
    match sfc {
        cip_sfc::CIP_SFC_176400 | cip_sfc::CIP_SFC_192000 => {
            (*parser).interval = 4;
        }
        cip_sfc::CIP_SFC_88200 | cip_sfc::CIP_SFC_96000 => {
            (*parser).interval = 2;
        }
        cip_sfc::CIP_SFC_32000 | cip_sfc::CIP_SFC_44100 | cip_sfc::CIP_SFC_48000 => {
            (*parser).interval = 1;
        }
        _ => {
            (*parser).interval = 1;
        }
    }

    0
}

const FRAGMENT_POS: usize = 6;
const MIDI_BYTE_POS: usize = 7;
const MIDI_FLAG_POS: usize = 8;
// One value of hardware meter consists of 4 messages.
const FRAGMENTS_PER_VALUE: ::core::ffi::c_uint = 4;
const VALUES_AT_IMAGE_END: u64 = 0xffffffffffffffff;

pub unsafe fn snd_motu_command_dsp_message_parser_parse(
    s: *const amdtp_stream,
    mut desc: *const pkt_desc,
    count: ::core::ffi::c_uint,
) {
    let motu = container_of!(s, snd_motu, tx_stream);
    let data_block_quadlets = (*s).data_block_quadlets;
    let parser = (*motu).message_parser as *mut msg_parser;
    let interval = (*parser).interval;
    let mut i: ::core::ffi::c_int;

    guard_spinlock_irqsave!(&mut (*parser).lock);

    i = 0;
    while i < count as ::core::ffi::c_int {
        let mut buffer = (*desc).ctx_payload as *mut __be32;
        let data_blocks = (*desc).data_blocks;
        let mut j: ::core::ffi::c_int;

        desc = amdtp_stream_next_packet_desc(s, desc);

        j = 0;
        while j < data_blocks as ::core::ffi::c_int {
            let b = buffer as *mut u8;
            buffer = buffer.add(data_block_quadlets as usize);

            match (*parser).state {
                msg_parser_state::INITIALIZED => {
                    let fragment = *b.add(FRAGMENT_POS);

                    if fragment > 0 {
                        (*parser).value = fragment as u64;
                        (*parser).message_count = 1;
                        (*parser).state = msg_parser_state::FRAGMENT_DETECTED;
                    }
                }
                msg_parser_state::FRAGMENT_DETECTED => {
                    if (*parser).message_count % interval == 0 {
                        let fragment = *b.add(FRAGMENT_POS);

                        (*parser).value >>= 8;
                        (*parser).value |= (fragment as u64) << 56;

                        if (*parser).value == VALUES_AT_IMAGE_END {
                            (*parser).state = msg_parser_state::AVAILABLE;
                            (*parser).fragment_pos = 0;
                            (*parser).value_index = 0;
                            (*parser).message_count = 0;
                        }
                    }
                    (*parser).message_count += 1;
                }
                msg_parser_state::AVAILABLE => {
                    if (*parser).message_count % interval == 0 {
                        let fragment = *b.add(FRAGMENT_POS);

                        (*parser).value >>= 8;
                        (*parser).value |= (fragment as u64) << 56;
                        (*parser).fragment_pos += 1;

                        if (*parser).fragment_pos == 4 {
                            // Skip the last two quadlets since they could be
                            // invalid value (0xffffffff) as floating point
                            // number.
                            if (*parser).value_index
                                < SNDRV_FIREWIRE_MOTU_COMMAND_DSP_METER_COUNT - 2
                            {
                                let val = ((*parser).value >> 32) as u32;
                                (*parser).meter.data[(*parser).value_index as usize] = val;
                            }
                            (*parser).value_index += 1;
                            (*parser).fragment_pos = 0;
                        }

                        if (*parser).value == VALUES_AT_IMAGE_END {
                            (*parser).value_index = 0;
                            (*parser).fragment_pos = 0;
                            (*parser).message_count = 0;
                        }
                    }
                    (*parser).message_count += 1;
                }
            }

            j += 1;
        }

        i += 1;
    }
}

pub unsafe fn snd_motu_command_dsp_message_parser_copy_meter(
    motu: *mut snd_motu,
    meter: *mut snd_firewire_motu_command_dsp_meter,
) {
    let parser = (*motu).message_parser as *mut msg_parser;

    guard_spinlock_irqsave!(&mut (*parser).lock);
    memcpy(
        meter as *mut ::core::ffi::c_void,
        &mut (*parser).meter as *mut _ as *const ::core::ffi::c_void,
        ::core::mem::size_of_val(&*meter),
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
