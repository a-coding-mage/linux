// SPDX-License-Identifier: GPL-2.0-only
//
// motu-register-dsp-message-parser.c - a part of driver for MOTU FireWire series
//
// Copyright (c) 2021 Takashi Sakamoto <o-takashi@sakamocchi.jp>

// Below models allow software to configure their DSP functions by asynchronous transaction
// to access their internal registers.
// * 828 mk2
// * 896hd
// * Traveler
// * 8 pre
// * Ultralite
// * 4 pre
// * Audio Express
//
// Additionally, isochronous packets from the above models include messages to notify state of
// DSP. The messages are two set of 3 byte data in 2nd and 3rd quadlet of data block. When user
// operates hardware components such as dial and switch, corresponding messages are transferred.
// The messages include Hardware metering and MIDI messages as well.

// C dependency intent: #include "motu.h"

const MSG_FLAG_POS: usize = 4;
const MSG_FLAG_TYPE_MASK: u8 = 0xf8;
const MSG_FLAG_MIDI_MASK: u8 = 0x01;
const MSG_FLAG_MODEL_SPECIFIC_MASK: u8 = 0x06;
const MSG_FLAG_8PRE: u8 = 0x00;
const MSG_FLAG_ULTRALITE: u8 = 0x04;
const MSG_FLAG_TRAVELER: u8 = 0x04;
const MSG_FLAG_828MK2: u8 = 0x04;
const MSG_FLAG_896HD: u8 = 0x04;
const MSG_FLAG_4PRE: u8 = 0x05; // MIDI mask is in 8th byte.
const MSG_FLAG_AUDIOEXPRESS: u8 = 0x05; // MIDI mask is in 8th byte.
const MSG_FLAG_TYPE_SHIFT: u8 = 3;
const MSG_VALUE_POS: usize = 5;
const MSG_MIDI_BYTE_POS: usize = 6;
const MSG_METER_IDX_POS: usize = 7;

// In 4 pre and Audio express, meter index is in 6th byte. MIDI flag is in 8th byte and MIDI byte
// is in 7th byte.
const MSG_METER_IDX_POS_4PRE_AE: usize = 6;
const MSG_MIDI_BYTE_POS_4PRE_AE: usize = 7;
const MSG_FLAG_MIDI_POS_4PRE_AE: usize = 8;

#[repr(u8)]
enum register_dsp_msg_type {
    // Used for messages with no information.
    INVALID = 0x00,
    MIXER_SELECT = 0x01,
    MIXER_SRC_GAIN = 0x02,
    MIXER_SRC_PAN = 0x03,
    MIXER_SRC_FLAG = 0x04,
    MIXER_OUTPUT_PAIRED_VOLUME = 0x05,
    MIXER_OUTPUT_PAIRED_FLAG = 0x06,
    MAIN_OUTPUT_PAIRED_VOLUME = 0x07,
    HP_OUTPUT_PAIRED_VOLUME = 0x08,
    HP_OUTPUT_PAIRED_ASSIGNMENT = 0x09,
    // Transferred by all models but the purpose is still unknown.
    UNKNOWN_0 = 0x0a,
    // Specific to 828mk2, 896hd, Traveler.
    UNKNOWN_2 = 0x0c,
    // Specific to 828mk2, Traveler, and 896hd (not functional).
    LINE_INPUT_BOOST = 0x0d,
    // Specific to 828mk2, Traveler, and 896hd (not functional).
    LINE_INPUT_NOMINAL_LEVEL = 0x0e,
    // Specific to Ultralite, 4 pre, Audio express, and 8 pre (not functional).
    INPUT_GAIN_AND_INVERT = 0x15,
    // Specific to 4 pre, and Audio express.
    INPUT_FLAG = 0x16,
    // Specific to 4 pre, and Audio express.
    MIXER_SRC_PAIRED_BALANCE = 0x17,
    // Specific to 4 pre, and Audio express.
    MIXER_SRC_PAIRED_WIDTH = 0x18,
    // Transferred by all models. This type of message interposes the series of the other
    // messages. The message delivers signal level up to 96.0 kHz. In 828mk2, 896hd, and
    // Traveler, one of physical outputs is selected for the message. The selection is done
    // by LSB one byte in asynchronous write quadlet transaction to 0x'ffff'f000'0b2c.
    METER = 0x1f,
}

const EVENT_QUEUE_SIZE: usize = 16;

#[repr(C)]
struct msg_parser {
    lock: spinlock_t,
    meter: snd_firewire_motu_register_dsp_meter,
    meter_pos_quirk: bool,

    param: snd_firewire_motu_register_dsp_parameter,
    prev_mixer_src_type: u8,
    mixer_ch: u8,
    mixer_src_ch: u8,

    input_ch: u8,
    prev_msg_type: u8,

    event_queue: [u32; EVENT_QUEUE_SIZE],
    push_pos: ::core::ffi::c_uint,
    pull_pos: ::core::ffi::c_uint,
}

pub unsafe fn snd_motu_register_dsp_message_parser_new(motu: *mut snd_motu) -> ::core::ffi::c_int {
    let parser: *mut msg_parser;

    parser = devm_kzalloc(
        &mut (*(*motu).card).card_dev,
        ::core::mem::size_of::<msg_parser>(),
        GFP_KERNEL,
    ) as *mut msg_parser;
    if parser.is_null() {
        return -ENOMEM;
    }

    spin_lock_init(&mut (*parser).lock);
    if (*motu).spec == &snd_motu_spec_4pre as *const _
        || (*motu).spec == &snd_motu_spec_audio_express as *const _
    {
        (*parser).meter_pos_quirk = true;
    }
    (*motu).message_parser = parser as *mut _;

    0
}

pub unsafe fn snd_motu_register_dsp_message_parser_init(motu: *mut snd_motu) -> ::core::ffi::c_int {
    let parser = (*motu).message_parser as *mut msg_parser;

    (*parser).prev_mixer_src_type = register_dsp_msg_type::INVALID as u8;
    (*parser).mixer_ch = 0xff;
    (*parser).mixer_src_ch = 0xff;
    (*parser).prev_msg_type = register_dsp_msg_type::INVALID as u8;

    0
}

// Rough implementaion of queue without overrun check.
unsafe fn queue_event(motu: *mut snd_motu, msg_type: u8, identifier0: u8, identifier1: u8, val: u8) {
    let parser = (*motu).message_parser as *mut msg_parser;
    let mut pos = (*parser).push_pos as usize;
    let entry: u32;

    if (*motu).hwdep.is_null() || (*(*motu).hwdep).used == 0 {
        return;
    }

    entry = ((msg_type as u32) << 24)
        | ((identifier0 as u32) << 16)
        | ((identifier1 as u32) << 8)
        | val as u32;
    (*parser).event_queue[pos] = entry;

    pos += 1;
    if pos >= EVENT_QUEUE_SIZE {
        pos = 0;
    }
    (*parser).push_pos = pos as ::core::ffi::c_uint;
}

pub unsafe fn snd_motu_register_dsp_message_parser_parse(
    s: *const amdtp_stream,
    mut desc: *const pkt_desc,
    count: ::core::ffi::c_uint,
) {
    let motu = container_of!(s, snd_motu, tx_stream);
    let data_block_quadlets = (*s).data_block_quadlets;
    let parser = (*motu).message_parser as *mut msg_parser;
    let meter_pos_quirk = (*parser).meter_pos_quirk;
    let pos = (*parser).push_pos;

    let _guard = spinlock_irqsave_guard(&mut (*parser).lock);

    for _i in 0..count {
        let mut buffer = (*desc).ctx_payload as *mut __be32;
        let data_blocks = (*desc).data_blocks;

        desc = amdtp_stream_next_packet_desc(s, desc);

        for _j in 0..data_blocks {
            let b = buffer as *mut u8;
            let msg_type =
                ((*b.add(MSG_FLAG_POS) & MSG_FLAG_TYPE_MASK) >> MSG_FLAG_TYPE_SHIFT) as u8;
            let val = *b.add(MSG_VALUE_POS);

            buffer = buffer.add(data_block_quadlets as usize);

            match msg_type {
                x if x == register_dsp_msg_type::MIXER_SELECT as u8 => {
                    let mixer_ch = val / 0x20;
                    if mixer_ch < SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT as u8 {
                        (*parser).mixer_src_ch = 0;
                        (*parser).mixer_ch = mixer_ch;
                    }
                }
                x if x == register_dsp_msg_type::MIXER_SRC_GAIN as u8
                    || x == register_dsp_msg_type::MIXER_SRC_PAN as u8
                    || x == register_dsp_msg_type::MIXER_SRC_FLAG as u8
                    || x == register_dsp_msg_type::MIXER_SRC_PAIRED_BALANCE as u8
                    || x == register_dsp_msg_type::MIXER_SRC_PAIRED_WIDTH as u8 =>
                {
                    let param = &mut (*parser).param as *mut snd_firewire_motu_register_dsp_parameter;
                    let mixer_ch = (*parser).mixer_ch;
                    let mut mixer_src_ch = (*parser).mixer_src_ch;

                    if msg_type != (*parser).prev_mixer_src_type {
                        mixer_src_ch = 0;
                    } else {
                        mixer_src_ch = mixer_src_ch.wrapping_add(1);
                    }
                    (*parser).prev_mixer_src_type = msg_type;

                    if mixer_ch < SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT as u8
                        && mixer_src_ch < SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_SRC_COUNT as u8
                    {
                        let mixer_ch = (*parser).mixer_ch as usize;
                        let mixer_src_ch_usize = mixer_src_ch as usize;

                        match msg_type {
                            x if x == register_dsp_msg_type::MIXER_SRC_GAIN as u8 => {
                                if (*param).mixer.source[mixer_ch].gain[mixer_src_ch_usize] != val {
                                    queue_event(motu, msg_type, mixer_ch as u8, mixer_src_ch, val);
                                    (*param).mixer.source[mixer_ch].gain[mixer_src_ch_usize] = val;
                                }
                            }
                            x if x == register_dsp_msg_type::MIXER_SRC_PAN as u8 => {
                                if (*param).mixer.source[mixer_ch].pan[mixer_src_ch_usize] != val {
                                    queue_event(motu, msg_type, mixer_ch as u8, mixer_src_ch, val);
                                    (*param).mixer.source[mixer_ch].pan[mixer_src_ch_usize] = val;
                                }
                            }
                            x if x == register_dsp_msg_type::MIXER_SRC_FLAG as u8 => {
                                if (*param).mixer.source[mixer_ch].flag[mixer_src_ch_usize] != val {
                                    queue_event(motu, msg_type, mixer_ch as u8, mixer_src_ch, val);
                                    (*param).mixer.source[mixer_ch].flag[mixer_src_ch_usize] = val;
                                }
                            }
                            x if x == register_dsp_msg_type::MIXER_SRC_PAIRED_BALANCE as u8 => {
                                if (*param).mixer.source[mixer_ch].paired_balance[mixer_src_ch_usize]
                                    != val
                                {
                                    queue_event(motu, msg_type, mixer_ch as u8, mixer_src_ch, val);
                                    (*param).mixer.source[mixer_ch].paired_balance
                                        [mixer_src_ch_usize] = val;
                                }
                            }
                            x if x == register_dsp_msg_type::MIXER_SRC_PAIRED_WIDTH as u8 => {
                                if (*param).mixer.source[mixer_ch].paired_width[mixer_src_ch_usize]
                                    != val
                                {
                                    queue_event(motu, msg_type, mixer_ch as u8, mixer_src_ch, val);
                                    (*param).mixer.source[mixer_ch].paired_width[mixer_src_ch_usize] =
                                        val;
                                }
                            }
                            _ => {}
                        }

                        (*parser).mixer_src_ch = mixer_src_ch;
                    }
                }
                x if x == register_dsp_msg_type::MIXER_OUTPUT_PAIRED_VOLUME as u8
                    || x == register_dsp_msg_type::MIXER_OUTPUT_PAIRED_FLAG as u8 =>
                {
                    let param = &mut (*parser).param as *mut snd_firewire_motu_register_dsp_parameter;
                    let mixer_ch = (*parser).mixer_ch;

                    if mixer_ch < SNDRV_FIREWIRE_MOTU_REGISTER_DSP_MIXER_COUNT as u8 {
                        let mixer_ch_usize = mixer_ch as usize;

                        match msg_type {
                            x if x == register_dsp_msg_type::MIXER_OUTPUT_PAIRED_VOLUME as u8 => {
                                if (*param).mixer.output.paired_volume[mixer_ch_usize] != val {
                                    queue_event(motu, msg_type, mixer_ch, 0, val);
                                    (*param).mixer.output.paired_volume[mixer_ch_usize] = val;
                                }
                            }
                            x if x == register_dsp_msg_type::MIXER_OUTPUT_PAIRED_FLAG as u8 => {
                                if (*param).mixer.output.paired_flag[mixer_ch_usize] != val {
                                    queue_event(motu, msg_type, mixer_ch, 0, val);
                                    (*param).mixer.output.paired_flag[mixer_ch_usize] = val;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                x if x == register_dsp_msg_type::MAIN_OUTPUT_PAIRED_VOLUME as u8 => {
                    if (*parser).param.output.main_paired_volume != val {
                        queue_event(motu, msg_type, 0, 0, val);
                        (*parser).param.output.main_paired_volume = val;
                    }
                }
                x if x == register_dsp_msg_type::HP_OUTPUT_PAIRED_VOLUME as u8 => {
                    if (*parser).param.output.hp_paired_volume != val {
                        queue_event(motu, msg_type, 0, 0, val);
                        (*parser).param.output.hp_paired_volume = val;
                    }
                }
                x if x == register_dsp_msg_type::HP_OUTPUT_PAIRED_ASSIGNMENT as u8 => {
                    if (*parser).param.output.hp_paired_assignment != val {
                        queue_event(motu, msg_type, 0, 0, val);
                        (*parser).param.output.hp_paired_assignment = val;
                    }
                }
                x if x == register_dsp_msg_type::LINE_INPUT_BOOST as u8 => {
                    if (*parser).param.line_input.boost_flag != val {
                        queue_event(motu, msg_type, 0, 0, val);
                        (*parser).param.line_input.boost_flag = val;
                    }
                }
                x if x == register_dsp_msg_type::LINE_INPUT_NOMINAL_LEVEL as u8 => {
                    if (*parser).param.line_input.nominal_level_flag != val {
                        queue_event(motu, msg_type, 0, 0, val);
                        (*parser).param.line_input.nominal_level_flag = val;
                    }
                }
                x if x == register_dsp_msg_type::INPUT_GAIN_AND_INVERT as u8
                    || x == register_dsp_msg_type::INPUT_FLAG as u8 =>
                {
                    let param = &mut (*parser).param as *mut snd_firewire_motu_register_dsp_parameter;
                    let mut input_ch = (*parser).input_ch;

                    if (*parser).prev_msg_type != msg_type {
                        input_ch = 0;
                    } else {
                        input_ch = input_ch.wrapping_add(1);
                    }

                    if input_ch < SNDRV_FIREWIRE_MOTU_REGISTER_DSP_INPUT_COUNT as u8 {
                        let input_ch_usize = input_ch as usize;

                        match msg_type {
                            x if x == register_dsp_msg_type::INPUT_GAIN_AND_INVERT as u8 => {
                                if (*param).input.gain_and_invert[input_ch_usize] != val {
                                    queue_event(motu, msg_type, input_ch, 0, val);
                                    (*param).input.gain_and_invert[input_ch_usize] = val;
                                }
                            }
                            x if x == register_dsp_msg_type::INPUT_FLAG as u8 => {
                                if (*param).input.flag[input_ch_usize] != val {
                                    queue_event(motu, msg_type, input_ch, 0, val);
                                    (*param).input.flag[input_ch_usize] = val;
                                }
                            }
                            _ => {}
                        }
                        (*parser).input_ch = input_ch;
                    }
                }
                x if x == register_dsp_msg_type::UNKNOWN_0 as u8
                    || x == register_dsp_msg_type::UNKNOWN_2 as u8 => {}
                x if x == register_dsp_msg_type::METER as u8 => {
                    let mut meter_pos: u8;

                    if !meter_pos_quirk {
                        meter_pos = *b.add(MSG_METER_IDX_POS);
                    } else {
                        meter_pos = *b.add(MSG_METER_IDX_POS_4PRE_AE);
                    }

                    if meter_pos < SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_INPUT_COUNT as u8 {
                        (*parser).meter.data[meter_pos as usize] = val;
                    } else if meter_pos >= 0x80 {
                        meter_pos = meter_pos.wrapping_sub(
                            0x80 - SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_INPUT_COUNT as u8,
                        );

                        if meter_pos < SNDRV_FIREWIRE_MOTU_REGISTER_DSP_METER_COUNT as u8 {
                            (*parser).meter.data[meter_pos as usize] = val;
                        }
                    }

                    // The message for meter is interruptible to the series of other
                    // types of messages. Don't cache it.
                    continue;
                }
                _ => {
                    // Don't cache it.
                    continue;
                }
            }

            (*parser).prev_msg_type = msg_type;
        }
    }

    if pos != (*parser).push_pos {
        wake_up(&mut (*motu).hwdep_wait);
    }
}

pub unsafe fn snd_motu_register_dsp_message_parser_copy_meter(
    motu: *mut snd_motu,
    meter: *mut snd_firewire_motu_register_dsp_meter,
) {
    let parser = (*motu).message_parser as *mut msg_parser;

    let _guard = spinlock_irqsave_guard(&mut (*parser).lock);
    memcpy(
        meter as *mut ::core::ffi::c_void,
        &mut (*parser).meter as *mut _ as *const ::core::ffi::c_void,
        ::core::mem::size_of::<snd_firewire_motu_register_dsp_meter>(),
    );
}

pub unsafe fn snd_motu_register_dsp_message_parser_copy_parameter(
    motu: *mut snd_motu,
    param: *mut snd_firewire_motu_register_dsp_parameter,
) {
    let parser = (*motu).message_parser as *mut msg_parser;

    let _guard = spinlock_irqsave_guard(&mut (*parser).lock);
    memcpy(
        param as *mut ::core::ffi::c_void,
        &mut (*parser).param as *mut _ as *const ::core::ffi::c_void,
        ::core::mem::size_of::<snd_firewire_motu_register_dsp_parameter>(),
    );
}

pub unsafe fn snd_motu_register_dsp_message_parser_count_event(
    motu: *mut snd_motu,
) -> ::core::ffi::c_uint {
    let parser = (*motu).message_parser as *mut msg_parser;

    let _guard = spinlock_irqsave_guard(&mut (*parser).lock);

    if (*parser).pull_pos > (*parser).push_pos {
        EVENT_QUEUE_SIZE as ::core::ffi::c_uint - (*parser).pull_pos + (*parser).push_pos
    } else {
        (*parser).push_pos - (*parser).pull_pos
    }
}

pub unsafe fn snd_motu_register_dsp_message_parser_copy_event(
    motu: *mut snd_motu,
    event: *mut u32,
) -> bool {
    let parser = (*motu).message_parser as *mut msg_parser;
    let mut pos: usize;

    let _guard = spinlock_irqsave_guard(&mut (*parser).lock);

    if (*parser).pull_pos == (*parser).push_pos {
        return false;
    }

    pos = (*parser).pull_pos as usize;
    *event = (*parser).event_queue[pos];

    pos += 1;
    if pos >= EVENT_QUEUE_SIZE {
        pos = 0;
    }
    (*parser).pull_pos = pos as ::core::ffi::c_uint;

    true
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
