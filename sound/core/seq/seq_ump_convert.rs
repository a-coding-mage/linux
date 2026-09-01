// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ALSA sequencer event conversion between UMP and legacy clients
 *
 * Translated from C. External types, constants, helpers, and delivery functions
 * are expected from the surrounding ALSA/kernel Rust bindings corresponding to:
 * linux/init.h, linux/errno.h, linux/string.h, sound/core.h, sound/ump.h,
 * sound/ump_msg.h, and seq_ump_convert.h.
 */

use crate::*;

/*
 * Upgrade / downgrade value bits
 */
fn downscale_32_to_7bit(src: u32) -> u8 {
    (src >> 25) as u8
}

fn downscale_32_to_14bit(src: u32) -> u16 {
    (src >> 18) as u16
}

fn downscale_16_to_7bit(src: u16) -> u8 {
    (src >> 9) as u8
}

fn upscale_7_to_16bit(src: u8) -> u16 {
    let val: u16 = (src as u16) << 9;
    if src <= 0x40 {
        return val;
    }
    let repeat: u16 = (src & 0x3f) as u16;
    val | (repeat << 3) | (repeat >> 3)
}

fn upscale_7_to_32bit(src: u8) -> u32 {
    let val: u32 = (src as u32) << 25;
    if src <= 0x40 {
        return val;
    }
    let repeat: u32 = (src & 0x3f) as u32;
    val | (repeat << 19) | (repeat << 13) | (repeat << 7) | (repeat << 1) | (repeat >> 5)
}

fn upscale_14_to_32bit(src: u16) -> u32 {
    let val: u32 = (src as u32) << 18;
    if src <= 0x2000 {
        return val;
    }
    let repeat: u32 = (src & 0x1fff) as u32;
    val | (repeat << 5) | (repeat >> 8)
}

unsafe fn get_ump_group(port: *mut snd_seq_client_port) -> u8 {
    if (*port).ump_group != 0 {
        ((*port).ump_group - 1) as u8
    } else {
        0
    }
}

/* create a UMP header */
unsafe fn make_raw_ump(port: *mut snd_seq_client_port, typ: u8) -> u32 {
    ump_compose(typ, get_ump_group(port), 0, 0)
}

/*
 * UMP -> MIDI1 sequencer event
 */

/* MIDI 1.0 CVM */

/* encode note event */
unsafe fn ump_midi1_to_note_ev(val: *const snd_ump_midi1_msg, ev: *mut snd_seq_event) {
    (*ev).data.note.channel = (*val).note.channel;
    (*ev).data.note.note = (*val).note.note;
    (*ev).data.note.velocity = (*val).note.velocity;
}

/* encode one parameter controls */
unsafe fn ump_midi1_to_ctrl_ev(val: *const snd_ump_midi1_msg, ev: *mut snd_seq_event) {
    (*ev).data.control.channel = (*val).caf.channel;
    (*ev).data.control.value = (*val).caf.data as _;
}

/* encode pitch wheel change */
unsafe fn ump_midi1_to_pitchbend_ev(val: *const snd_ump_midi1_msg, ev: *mut snd_seq_event) {
    (*ev).data.control.channel = (*val).pb.channel;
    (*ev).data.control.value = (((*val).pb.data_msb as i32) << 7) | ((*val).pb.data_lsb as i32);
    (*ev).data.control.value -= 8192;
}

/* encode midi control change */
unsafe fn ump_midi1_to_cc_ev(val: *const snd_ump_midi1_msg, ev: *mut snd_seq_event) {
    (*ev).data.control.channel = (*val).cc.channel;
    (*ev).data.control.param = (*val).cc.index as _;
    (*ev).data.control.value = (*val).cc.data as _;
}

/* Encoding MIDI 1.0 UMP packet */
struct seq_ump_midi1_to_ev {
    seq_type: i32,
    encode: Option<unsafe fn(*const snd_ump_midi1_msg, *mut snd_seq_event)>,
}

/* Encoders for MIDI1 status 0x80-0xe0 */
static midi1_msg_encoders: [seq_ump_midi1_to_ev; 7] = [
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_NOTEOFF, encode: Some(ump_midi1_to_note_ev) }, /* 0x80 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_NOTEON, encode: Some(ump_midi1_to_note_ev) }, /* 0x90 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_KEYPRESS, encode: Some(ump_midi1_to_note_ev) }, /* 0xa0 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_CONTROLLER, encode: Some(ump_midi1_to_cc_ev) }, /* 0xb0 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_PGMCHANGE, encode: Some(ump_midi1_to_ctrl_ev) }, /* 0xc0 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_CHANPRESS, encode: Some(ump_midi1_to_ctrl_ev) }, /* 0xd0 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_PITCHBEND, encode: Some(ump_midi1_to_pitchbend_ev) }, /* 0xe0 */
];

unsafe fn cvt_ump_midi1_to_event(val: *const snd_ump_midi1_msg, ev: *mut snd_seq_event) -> i32 {
    let mut status: u8 = (*val).note.status;

    if status < 0x8 || status > 0xe {
        return 0; /* invalid - skip */
    }
    status -= 8;
    (*ev).type_ = midi1_msg_encoders[status as usize].seq_type;
    (*ev).flags = SNDRV_SEQ_EVENT_LENGTH_FIXED;
    (midi1_msg_encoders[status as usize].encode.unwrap())(val, ev);
    1
}

/* MIDI System message */

/* encode one parameter value*/
unsafe fn ump_system_to_one_param_ev(val: *const snd_ump_midi1_msg, ev: *mut snd_seq_event) {
    (*ev).data.control.value = (*val).system.parm1 as _;
}

/* encode song position */
unsafe fn ump_system_to_songpos_ev(val: *const snd_ump_midi1_msg, ev: *mut snd_seq_event) {
    (*ev).data.control.value = (((*val).system.parm2 as i32) << 7) | ((*val).system.parm1 as i32);
}

/* Encoders for 0xf0 - 0xff */
static system_msg_encoders: [seq_ump_midi1_to_ev; 16] = [
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0xf0 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_QFRAME, encode: Some(ump_system_to_one_param_ev) }, /* 0xf1 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_SONGPOS, encode: Some(ump_system_to_songpos_ev) }, /* 0xf2 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_SONGSEL, encode: Some(ump_system_to_one_param_ev) }, /* 0xf3 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0xf4 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0xf5 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_TUNE_REQUEST, encode: None }, /* 0xf6 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0xf7 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_CLOCK, encode: None }, /* 0xf8 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0xf9 */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_START, encode: None }, /* 0xfa */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_CONTINUE, encode: None }, /* 0xfb */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_STOP, encode: None }, /* 0xfc */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0xfd */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_SENSING, encode: None }, /* 0xfe */
    seq_ump_midi1_to_ev { seq_type: SNDRV_SEQ_EVENT_RESET, encode: None }, /* 0xff */
];

unsafe fn cvt_ump_system_to_event(val: *const snd_ump_midi1_msg, ev: *mut snd_seq_event) -> i32 {
    let mut status: u8 = (*val).system.status;

    if (status & 0xf0) != UMP_MIDI1_MSG_REALTIME {
        return 0; /* invalid status - skip */
    }
    status &= 0x0f;
    (*ev).type_ = system_msg_encoders[status as usize].seq_type;
    (*ev).flags = SNDRV_SEQ_EVENT_LENGTH_FIXED;
    if (*ev).type_ == SNDRV_SEQ_EVENT_NONE {
        return 0;
    }
    if let Some(encode) = system_msg_encoders[status as usize].encode {
        encode(val, ev);
    }
    1
}

/* MIDI 2.0 CVM */

/* encode note event */
unsafe fn ump_midi2_to_note_ev(val: *const snd_ump_midi2_msg, ev: *mut snd_seq_event) -> i32 {
    (*ev).data.note.channel = (*val).note.channel;
    (*ev).data.note.note = (*val).note.note;
    (*ev).data.note.velocity = downscale_16_to_7bit((*val).note.velocity);
    /* correct note-on velocity 0 to 1;
     * it's no longer equivalent as not-off for MIDI 2.0
     */
    if (*ev).type_ == SNDRV_SEQ_EVENT_NOTEON && (*ev).data.note.velocity == 0 {
        (*ev).data.note.velocity = 1;
    }
    1
}

/* encode pitch wheel change */
unsafe fn ump_midi2_to_pitchbend_ev(val: *const snd_ump_midi2_msg, ev: *mut snd_seq_event) -> i32 {
    (*ev).data.control.channel = (*val).pb.channel;
    (*ev).data.control.value = downscale_32_to_14bit((*val).pb.data) as _;
    (*ev).data.control.value -= 8192;
    1
}

/* encode midi control change */
unsafe fn ump_midi2_to_cc_ev(val: *const snd_ump_midi2_msg, ev: *mut snd_seq_event) -> i32 {
    (*ev).data.control.channel = (*val).cc.channel;
    (*ev).data.control.param = (*val).cc.index as _;
    (*ev).data.control.value = downscale_32_to_7bit((*val).cc.data) as _;
    1
}

/* encode midi program change */
unsafe fn ump_midi2_to_pgm_ev(val: *const snd_ump_midi2_msg, mut ev: *mut snd_seq_event) -> i32 {
    let mut size = 1;

    (*ev).data.control.channel = (*val).pg.channel;
    if (*val).pg.bank_valid != 0 {
        (*ev).type_ = SNDRV_SEQ_EVENT_CONTROL14;
        (*ev).data.control.param = UMP_CC_BANK_SELECT as _;
        (*ev).data.control.value = (((*val).pg.bank_msb as i32) << 7) | ((*val).pg.bank_lsb as i32);
        *ev.add(1) = *ev;
        ev = ev.add(1);
        (*ev).type_ = SNDRV_SEQ_EVENT_PGMCHANGE;
        size = 2;
    }
    (*ev).data.control.value = (*val).pg.program as _;
    size
}

/* encode one parameter controls */
unsafe fn ump_midi2_to_ctrl_ev(val: *const snd_ump_midi2_msg, ev: *mut snd_seq_event) -> i32 {
    (*ev).data.control.channel = (*val).caf.channel;
    (*ev).data.control.value = downscale_32_to_7bit((*val).caf.data) as _;
    1
}

/* encode RPN/NRPN */
unsafe fn ump_midi2_to_rpn_ev(val: *const snd_ump_midi2_msg, ev: *mut snd_seq_event) -> i32 {
    (*ev).data.control.channel = (*val).rpn.channel;
    (*ev).data.control.param = (((*val).rpn.bank as i32) << 7) | ((*val).rpn.index as i32);
    (*ev).data.control.value = downscale_32_to_14bit((*val).rpn.data) as _;
    1
}

/* Encoding MIDI 2.0 UMP Packet */
struct seq_ump_midi2_to_ev {
    seq_type: i32,
    encode: Option<unsafe fn(*const snd_ump_midi2_msg, *mut snd_seq_event) -> i32>,
}

/* Encoders for MIDI2 status 0x00-0xf0 */
static midi2_msg_encoders: [seq_ump_midi2_to_ev; 16] = [
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0x00 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0x10 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_REGPARAM, encode: Some(ump_midi2_to_rpn_ev) }, /* 0x20 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_NONREGPARAM, encode: Some(ump_midi2_to_rpn_ev) }, /* 0x30 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0x40 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0x50 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0x60 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0x70 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_NOTEOFF, encode: Some(ump_midi2_to_note_ev) }, /* 0x80 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_NOTEON, encode: Some(ump_midi2_to_note_ev) }, /* 0x90 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_KEYPRESS, encode: Some(ump_midi2_to_note_ev) }, /* 0xa0 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_CONTROLLER, encode: Some(ump_midi2_to_cc_ev) }, /* 0xb0 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_PGMCHANGE, encode: Some(ump_midi2_to_pgm_ev) }, /* 0xc0 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_CHANPRESS, encode: Some(ump_midi2_to_ctrl_ev) }, /* 0xd0 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_PITCHBEND, encode: Some(ump_midi2_to_pitchbend_ev) }, /* 0xe0 */
    seq_ump_midi2_to_ev { seq_type: SNDRV_SEQ_EVENT_NONE, encode: None }, /* 0xf0 */
];

unsafe fn cvt_ump_midi2_to_event(val: *const snd_ump_midi2_msg, ev: *mut snd_seq_event) -> i32 {
    let status: u8 = (*val).note.status;

    (*ev).type_ = midi2_msg_encoders[status as usize].seq_type;
    if (*ev).type_ == SNDRV_SEQ_EVENT_NONE {
        return 0; /* skip */
    }
    (*ev).flags = SNDRV_SEQ_EVENT_LENGTH_FIXED;
    (midi2_msg_encoders[status as usize].encode.unwrap())(val, ev)
}

/* parse and compose for a sysex var-length event */
unsafe fn cvt_ump_sysex7_to_event(data: *const u32, buf: *mut u8, ev: *mut snd_seq_event) -> i32 {
    let status: u8;
    let bytes: u8;
    let mut val: u32;
    let mut size: i32 = 0;

    val = *data.add(0);
    status = ump_sysex_message_status(val);
    bytes = ump_sysex_message_length(val);
    if bytes > 6 {
        return 0; // skip
    }

    if status == UMP_SYSEX_STATUS_SINGLE || status == UMP_SYSEX_STATUS_START {
        *buf.add(0) = UMP_MIDI1_MSG_SYSEX_START;
        size = 1;
    }

    if bytes > 0 {
        *buf.add(size as usize) = ((val >> 8) & 0x7f) as u8;
        size += 1;
    }
    if bytes > 1 {
        *buf.add(size as usize) = (val & 0x7f) as u8;
        size += 1;
    }
    val = *data.add(1);
    if bytes > 2 {
        *buf.add(size as usize) = ((val >> 24) & 0x7f) as u8;
        size += 1;
    }
    if bytes > 3 {
        *buf.add(size as usize) = ((val >> 16) & 0x7f) as u8;
        size += 1;
    }
    if bytes > 4 {
        *buf.add(size as usize) = ((val >> 8) & 0x7f) as u8;
        size += 1;
    }
    if bytes > 5 {
        *buf.add(size as usize) = (val & 0x7f) as u8;
        size += 1;
    }

    if status == UMP_SYSEX_STATUS_SINGLE || status == UMP_SYSEX_STATUS_END {
        *buf.add(size as usize) = UMP_MIDI1_MSG_SYSEX_END;
        size += 1;
    }

    (*ev).type_ = SNDRV_SEQ_EVENT_SYSEX;
    (*ev).flags = SNDRV_SEQ_EVENT_LENGTH_VARIABLE;
    (*ev).data.ext.len = size as _;
    (*ev).data.ext.ptr = buf as _;
    1
}

/* convert UMP packet from MIDI 1.0 to MIDI 2.0 and deliver it */
unsafe fn cvt_ump_midi1_to_midi2(
    dest: *mut snd_seq_client,
    dest_port: *mut snd_seq_client_port,
    __event: *mut snd_seq_event,
    atomic: i32,
    hop: i32,
) -> i32 {
    let event = __event as *mut snd_seq_ump_event;
    let mut ev_cvt: snd_seq_ump_event = *event;
    let midi1 = (*event).ump.as_ptr() as *const snd_ump_midi1_msg;
    let midi2 = ev_cvt.ump.as_mut_ptr() as *mut snd_ump_midi2_msg;
    let mut cc: *mut ump_cvt_to_ump_bank;

    core::ptr::write_bytes(ev_cvt.ump.as_mut_ptr(), 0, ev_cvt.ump.len());

    (*midi2).note.type_ = UMP_MSG_TYPE_MIDI2_CHANNEL_VOICE;
    (*midi2).note.group = (*midi1).note.group;
    (*midi2).note.status = (*midi1).note.status;
    (*midi2).note.channel = (*midi1).note.channel;
    match (*midi1).note.status {
        UMP_MSG_STATUS_NOTE_ON | UMP_MSG_STATUS_NOTE_OFF => {
            (*midi2).note.note = (*midi1).note.note;
            (*midi2).note.velocity = upscale_7_to_16bit((*midi1).note.velocity);
        }
        UMP_MSG_STATUS_POLY_PRESSURE => {
            (*midi2).paf.note = (*midi1).paf.note;
            (*midi2).paf.data = upscale_7_to_32bit((*midi1).paf.data);
        }
        UMP_MSG_STATUS_CC => {
            cc = (*dest_port).midi2_bank.as_mut_ptr().add((*midi1).note.channel as usize);
            match (*midi1).cc.index {
                UMP_CC_BANK_SELECT => {
                    (*cc).bank_set = 1;
                    (*cc).cc_bank_msb = (*midi1).cc.data;
                    return 0; // skip
                }
                UMP_CC_BANK_SELECT_LSB => {
                    (*cc).bank_set = 1;
                    (*cc).cc_bank_lsb = (*midi1).cc.data;
                    return 0; // skip
                }
                _ => {}
            }
            (*midi2).cc.index = (*midi1).cc.index;
            (*midi2).cc.data = upscale_7_to_32bit((*midi1).cc.data);
        }
        UMP_MSG_STATUS_PROGRAM => {
            (*midi2).pg.program = (*midi1).pg.program;
            cc = (*dest_port).midi2_bank.as_mut_ptr().add((*midi1).note.channel as usize);
            if (*cc).bank_set != 0 {
                (*midi2).pg.bank_valid = 1;
                (*midi2).pg.bank_msb = (*cc).cc_bank_msb;
                (*midi2).pg.bank_lsb = (*cc).cc_bank_lsb;
                (*cc).bank_set = 0;
            }
        }
        UMP_MSG_STATUS_CHANNEL_PRESSURE => {
            (*midi2).caf.data = upscale_7_to_32bit((*midi1).caf.data);
        }
        UMP_MSG_STATUS_PITCH_BEND => {
            (*midi2).pb.data =
                upscale_14_to_32bit((((*midi1).pb.data_msb as u16) << 7) | ((*midi1).pb.data_lsb as u16));
        }
        _ => return 0,
    }

    __snd_seq_deliver_single_event(dest, dest_port, &mut ev_cvt as *mut _ as *mut snd_seq_event, atomic, hop)
}

/* convert UMP packet from MIDI 2.0 to MIDI 1.0 and deliver it */
unsafe fn cvt_ump_midi2_to_midi1(
    dest: *mut snd_seq_client,
    dest_port: *mut snd_seq_client_port,
    __event: *mut snd_seq_event,
    atomic: i32,
    hop: i32,
) -> i32 {
    let event = __event as *mut snd_seq_ump_event;
    let mut ev_cvt: snd_seq_ump_event = *event;
    let midi1 = ev_cvt.ump.as_mut_ptr() as *mut snd_ump_midi1_msg;
    let midi2 = (*event).ump.as_ptr() as *const snd_ump_midi2_msg;
    let mut err: i32;
    let v: u16;

    core::ptr::write_bytes(ev_cvt.ump.as_mut_ptr(), 0, ev_cvt.ump.len());

    (*midi1).note.type_ = UMP_MSG_TYPE_MIDI1_CHANNEL_VOICE;
    (*midi1).note.group = (*midi2).note.group;
    (*midi1).note.status = (*midi2).note.status;
    (*midi1).note.channel = (*midi2).note.channel;
    match (*midi2).note.status {
        UMP_MSG_STATUS_NOTE_ON | UMP_MSG_STATUS_NOTE_OFF => {
            (*midi1).note.note = (*midi2).note.note;
            (*midi1).note.velocity = downscale_16_to_7bit((*midi2).note.velocity);
        }
        UMP_MSG_STATUS_POLY_PRESSURE => {
            (*midi1).paf.note = (*midi2).paf.note;
            (*midi1).paf.data = downscale_32_to_7bit((*midi2).paf.data);
        }
        UMP_MSG_STATUS_CC => {
            (*midi1).cc.index = (*midi2).cc.index;
            (*midi1).cc.data = downscale_32_to_7bit((*midi2).cc.data);
        }
        UMP_MSG_STATUS_PROGRAM => {
            if (*midi2).pg.bank_valid != 0 {
                (*midi1).cc.status = UMP_MSG_STATUS_CC;
                (*midi1).cc.index = UMP_CC_BANK_SELECT;
                (*midi1).cc.data = (*midi2).pg.bank_msb;
                err = __snd_seq_deliver_single_event(dest, dest_port, &mut ev_cvt as *mut _ as *mut snd_seq_event, atomic, hop);
                if err < 0 {
                    return err;
                }
                (*midi1).cc.index = UMP_CC_BANK_SELECT_LSB;
                (*midi1).cc.data = (*midi2).pg.bank_lsb;
                err = __snd_seq_deliver_single_event(dest, dest_port, &mut ev_cvt as *mut _ as *mut snd_seq_event, atomic, hop);
                if err < 0 {
                    return err;
                }
                (*midi1).note.status = (*midi2).note.status;
            }
            (*midi1).pg.program = (*midi2).pg.program;
        }
        UMP_MSG_STATUS_CHANNEL_PRESSURE => {
            (*midi1).caf.data = downscale_32_to_7bit((*midi2).caf.data);
        }
        UMP_MSG_STATUS_PITCH_BEND => {
            v = downscale_32_to_14bit((*midi2).pb.data);
            (*midi1).pb.data_msb = (v >> 7) as u8;
            (*midi1).pb.data_lsb = (v & 0x7f) as u8;
        }
        _ => return 0,
    }

    __snd_seq_deliver_single_event(dest, dest_port, &mut ev_cvt as *mut _ as *mut snd_seq_event, atomic, hop)
}

/* convert UMP to a legacy ALSA seq event and deliver it */
unsafe fn cvt_ump_to_any(
    dest: *mut snd_seq_client,
    dest_port: *mut snd_seq_client_port,
    event: *mut snd_seq_event,
    typ: u8,
    atomic: i32,
    hop: i32,
) -> i32 {
    let mut ev_cvt: [snd_seq_event; 2] = [*event, *event]; /* up to two events */
    let ump_ev = event as *mut snd_seq_ump_event;
    /* use the second event as a temp buffer for saving stack usage */
    let sysex_buf = ev_cvt.as_mut_ptr().add(1) as *mut u8;
    let flags: u8 = (*event).flags & !SNDRV_SEQ_EVENT_UMP;
    let mut i: i32;
    let len: i32;
    let mut err: i32;

    ev_cvt[0].flags = flags;
    ev_cvt[1].flags = flags;
    match typ {
        UMP_MSG_TYPE_SYSTEM => {
            len = cvt_ump_system_to_event((*ump_ev).ump.as_ptr() as *const snd_ump_midi1_msg, ev_cvt.as_mut_ptr());
        }
        UMP_MSG_TYPE_MIDI1_CHANNEL_VOICE => {
            len = cvt_ump_midi1_to_event((*ump_ev).ump.as_ptr() as *const snd_ump_midi1_msg, ev_cvt.as_mut_ptr());
        }
        UMP_MSG_TYPE_MIDI2_CHANNEL_VOICE => {
            len = cvt_ump_midi2_to_event((*ump_ev).ump.as_ptr() as *const snd_ump_midi2_msg, ev_cvt.as_mut_ptr());
        }
        UMP_MSG_TYPE_DATA => {
            len = cvt_ump_sysex7_to_event((*ump_ev).ump.as_ptr(), sysex_buf, ev_cvt.as_mut_ptr());
        }
        _ => return 0,
    }

    i = 0;
    while i < len {
        err = __snd_seq_deliver_single_event(dest, dest_port, ev_cvt.as_mut_ptr().add(i as usize), atomic, hop);
        if err < 0 {
            return err;
        }
        i += 1;
    }

    0
}

/* Replace UMP group field with the destination and deliver */
unsafe fn deliver_with_group_convert(
    dest: *mut snd_seq_client,
    dest_port: *mut snd_seq_client_port,
    ump_ev: *mut snd_seq_ump_event,
    atomic: i32,
    hop: i32,
) -> i32 {
    let mut ev: snd_seq_ump_event = *ump_ev;

    /* rewrite the group to the destination port */
    ev.ump[0] &= !(0xf_u32 << 24);
    /* fill with the new group; the dest_port->ump_group field is 1-based */
    ev.ump[0] |= (((*dest_port).ump_group - 1) as u32) << 24;

    __snd_seq_deliver_single_event(dest, dest_port, &mut ev as *mut _ as *mut snd_seq_event, atomic, hop)
}

/* apply the UMP event filter; return true to skip the event */
unsafe fn ump_event_filtered(dest: *mut snd_seq_client, ev: *const snd_seq_ump_event) -> bool {
    let group: u8;

    group = ump_message_group((*ev).ump[0]);
    if ump_is_groupless_msg(ump_message_type((*ev).ump[0])) {
        return ((*dest).group_filter & (1_u32 << 0)) != 0;
    }
    /* check the bitmap for 1-based group number */
    ((*dest).group_filter & (1_u32 << (group + 1))) != 0
}

/* Convert from UMP packet and deliver */
pub unsafe fn snd_seq_deliver_from_ump(
    source: *mut snd_seq_client,
    dest: *mut snd_seq_client,
    dest_port: *mut snd_seq_client_port,
    event: *mut snd_seq_event,
    atomic: i32,
    hop: i32,
) -> i32 {
    let ump_ev = event as *mut snd_seq_ump_event;
    let typ: u8;

    if snd_seq_ev_is_variable(event) {
        return 0; // skip, no variable event for UMP, so far
    }
    if ump_event_filtered(dest, ump_ev) {
        return 0; // skip if group filter is set and matching
    }
    typ = ump_message_type((*ump_ev).ump[0]);

    if snd_seq_client_is_ump(dest) {
        let is_midi2: bool = snd_seq_client_is_midi2(dest) && (*dest_port).is_midi1 == 0;

        if is_midi2 && typ == UMP_MSG_TYPE_MIDI1_CHANNEL_VOICE {
            return cvt_ump_midi1_to_midi2(dest, dest_port, event, atomic, hop);
        } else if !is_midi2 && typ == UMP_MSG_TYPE_MIDI2_CHANNEL_VOICE {
            return cvt_ump_midi2_to_midi1(dest, dest_port, event, atomic, hop);
        }
        /* non-EP port and different group is set? */
        if (*dest_port).ump_group != 0
            && !ump_is_groupless_msg(typ)
            && ump_message_group((*ump_ev).ump[0]) + 1 != (*dest_port).ump_group
        {
            return deliver_with_group_convert(dest, dest_port, ump_ev, atomic, hop);
        }
        /* copy as-is */
        return __snd_seq_deliver_single_event(dest, dest_port, event, atomic, hop);
    }

    cvt_ump_to_any(dest, dest_port, event, typ, atomic, hop)
}

/*
 * MIDI1 sequencer event -> UMP conversion
 */

/* Conversion to UMP MIDI 1.0 */

/* convert note on/off event to MIDI 1.0 UMP */
unsafe fn note_ev_to_ump_midi1(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi1_msg,
    mut status: u8,
) -> i32 {
    if (*event).data.note.velocity == 0 {
        status = UMP_MSG_STATUS_NOTE_OFF;
    }
    (*data).note.status = status;
    (*data).note.channel = (*event).data.note.channel & 0x0f;
    (*data).note.velocity = (*event).data.note.velocity & 0x7f;
    (*data).note.note = (*event).data.note.note & 0x7f;
    1
}

/* convert CC event to MIDI 1.0 UMP */
unsafe fn cc_ev_to_ump_midi1(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi1_msg,
    status: u8,
) -> i32 {
    (*data).cc.status = status;
    (*data).cc.channel = (*event).data.control.channel & 0x0f;
    (*data).cc.index = (*event).data.control.param as u8;
    (*data).cc.data = (*event).data.control.value as u8;
    1
}

/* convert one-parameter control event to MIDI 1.0 UMP */
unsafe fn ctrl_ev_to_ump_midi1(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi1_msg,
    status: u8,
) -> i32 {
    (*data).caf.status = status;
    (*data).caf.channel = (*event).data.control.channel & 0x0f;
    (*data).caf.data = ((*event).data.control.value & 0x7f) as u8;
    1
}

/* convert pitchbend event to MIDI 1.0 UMP */
unsafe fn pitchbend_ev_to_ump_midi1(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi1_msg,
    status: u8,
) -> i32 {
    let mut val: i32 = (*event).data.control.value + 8192;

    val = clamp(val, 0, 0x3fff);
    (*data).pb.status = status;
    (*data).pb.channel = (*event).data.control.channel & 0x0f;
    (*data).pb.data_msb = ((val >> 7) & 0x7f) as u8;
    (*data).pb.data_lsb = (val & 0x7f) as u8;
    1
}

/* convert 14bit control event to MIDI 1.0 UMP; split to two events */
unsafe fn ctrl14_ev_to_ump_midi1(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi1_msg,
    status: u8,
) -> i32 {
    (*data).cc.status = UMP_MSG_STATUS_CC;
    (*data).cc.channel = (*event).data.control.channel & 0x0f;
    (*data).cc.index = ((*event).data.control.param & 0x7f) as u8;
    if (*event).data.control.param < 0x20 {
        (*data).cc.data = (((*event).data.control.value >> 7) & 0x7f) as u8;
        *data.add(1) = *data;
        (*data.add(1)).cc.index = ((*event).data.control.param | 0x20) as u8;
        (*data.add(1)).cc.data = ((*event).data.control.value & 0x7f) as u8;
        return 2;
    }

    (*data).cc.data = ((*event).data.control.value & 0x7f) as u8;
    1
}

/* convert RPN/NRPN event to MIDI 1.0 UMP; split to four events */
unsafe fn rpn_ev_to_ump_midi1(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi1_msg,
    status: u8,
) -> i32 {
    let is_rpn: bool = status == UMP_MSG_STATUS_RPN;

    (*data).cc.status = UMP_MSG_STATUS_CC;
    (*data).cc.channel = (*event).data.control.channel & 0x0f;
    *data.add(1) = *data;
    *data.add(2) = *data;
    *data.add(3) = *data;

    (*data.add(0)).cc.index = if is_rpn { UMP_CC_RPN_MSB } else { UMP_CC_NRPN_MSB };
    (*data.add(0)).cc.data = (((*event).data.control.param >> 7) & 0x7f) as u8;
    (*data.add(1)).cc.index = if is_rpn { UMP_CC_RPN_LSB } else { UMP_CC_NRPN_LSB };
    (*data.add(1)).cc.data = ((*event).data.control.param & 0x7f) as u8;
    (*data.add(2)).cc.index = UMP_CC_DATA;
    (*data.add(2)).cc.data = (((*event).data.control.value >> 7) & 0x7f) as u8;
    (*data.add(3)).cc.index = UMP_CC_DATA_LSB;
    (*data.add(3)).cc.data = ((*event).data.control.value & 0x7f) as u8;
    4
}

/* convert system / RT message to UMP */
unsafe fn system_ev_to_ump_midi1(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi1_msg,
    status: u8,
) -> i32 {
    (*data).system.type_ = UMP_MSG_TYPE_SYSTEM; // override
    (*data).system.status = status;
    1
}

/* convert system / RT message with 1 parameter to UMP */
unsafe fn system_1p_ev_to_ump_midi1(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi1_msg,
    status: u8,
) -> i32 {
    (*data).system.type_ = UMP_MSG_TYPE_SYSTEM; // override
    (*data).system.status = status;
    (*data).system.parm1 = ((*event).data.control.value & 0x7f) as u8;
    1
}

/* convert system / RT message with two parameters to UMP */
unsafe fn system_2p_ev_to_ump_midi1(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi1_msg,
    status: u8,
) -> i32 {
    (*data).system.type_ = UMP_MSG_TYPE_SYSTEM; // override
    (*data).system.status = status;
    (*data).system.parm1 = ((*event).data.control.value & 0x7f) as u8;
    (*data).system.parm2 = (((*event).data.control.value >> 7) & 0x7f) as u8;
    1
}

/* Conversion to UMP MIDI 2.0 */

/* convert note on/off event to MIDI 2.0 UMP */
unsafe fn note_ev_to_ump_midi2(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi2_msg,
    mut status: u8,
) -> i32 {
    if (*event).data.note.velocity == 0 {
        status = UMP_MSG_STATUS_NOTE_OFF;
    }
    (*data).note.status = status;
    (*data).note.channel = (*event).data.note.channel & 0x0f;
    (*data).note.note = (*event).data.note.note & 0x7f;
    (*data).note.velocity = upscale_7_to_16bit((*event).data.note.velocity & 0x7f);
    1
}

/* convert PAF event to MIDI 2.0 UMP */
unsafe fn paf_ev_to_ump_midi2(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi2_msg,
    status: u8,
) -> i32 {
    (*data).paf.status = status;
    (*data).paf.channel = (*event).data.note.channel & 0x0f;
    (*data).paf.note = (*event).data.note.note & 0x7f;
    (*data).paf.data = upscale_7_to_32bit((*event).data.note.velocity & 0x7f);
    1
}

unsafe fn reset_rpn(cc: *mut ump_cvt_to_ump_bank) {
    (*cc).rpn_set = 0;
    (*cc).nrpn_set = 0;
    (*cc).cc_rpn_lsb = 0;
    (*cc).cc_rpn_msb = (*cc).cc_rpn_lsb;
    (*cc).cc_data_lsb = 0;
    (*cc).cc_data_msb = (*cc).cc_data_lsb;
    (*cc).cc_data_lsb_set = 0;
    (*cc).cc_data_msb_set = (*cc).cc_data_lsb_set;
}

/* set up the MIDI2 RPN/NRPN packet data from the parsed info */
unsafe fn fill_rpn(
    cc: *mut ump_cvt_to_ump_bank,
    data: *mut snd_ump_midi2_msg,
    channel: u8,
    flush: bool,
) -> i32 {
    if !((*cc).cc_data_lsb_set != 0 || (*cc).cc_data_msb_set != 0) {
        return 0; // skip
    }
    /* when not flushing, wait for complete data set */
    if !flush && ((*cc).cc_data_lsb_set == 0 || (*cc).cc_data_msb_set == 0) {
        return 0; // skip
    }

    if (*cc).rpn_set != 0 {
        (*data).rpn.status = UMP_MSG_STATUS_RPN;
        (*data).rpn.bank = (*cc).cc_rpn_msb;
        (*data).rpn.index = (*cc).cc_rpn_lsb;
    } else if (*cc).nrpn_set != 0 {
        (*data).rpn.status = UMP_MSG_STATUS_NRPN;
        (*data).rpn.bank = (*cc).cc_nrpn_msb;
        (*data).rpn.index = (*cc).cc_nrpn_lsb;
    } else {
        return 0; // skip
    }

    (*data).rpn.data = upscale_14_to_32bit((((*cc).cc_data_msb as u16) << 7) | ((*cc).cc_data_lsb as u16));
    (*data).rpn.channel = channel;

    reset_rpn(cc);
    1
}

/* convert CC event to MIDI 2.0 UMP */
unsafe fn cc_ev_to_ump_midi2(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi2_msg,
    status: u8,
) -> i32 {
    let channel: u8 = (*event).data.control.channel & 0x0f;
    let index: u8 = ((*event).data.control.param & 0x7f) as u8;
    let val: u8 = ((*event).data.control.value & 0x7f) as u8;
    let cc: *mut ump_cvt_to_ump_bank = (*dest_port).midi2_bank.as_mut_ptr().add(channel as usize);
    let ret: i32;

    /* process special CC's (bank/rpn/nrpn) */
    match index {
        UMP_CC_RPN_MSB => {
            ret = fill_rpn(cc, data, channel, true);
            (*cc).rpn_set = 1;
            (*cc).cc_rpn_msb = val;
            if (*cc).cc_rpn_msb == 0x7f && (*cc).cc_rpn_lsb == 0x7f {
                reset_rpn(cc);
            }
        }
        UMP_CC_RPN_LSB => {
            ret = fill_rpn(cc, data, channel, true);
            (*cc).rpn_set = 1;
            (*cc).cc_rpn_lsb = val;
            if (*cc).cc_rpn_msb == 0x7f && (*cc).cc_rpn_lsb == 0x7f {
                reset_rpn(cc);
            }
        }
        UMP_CC_NRPN_MSB => {
            ret = fill_rpn(cc, data, channel, true);
            (*cc).nrpn_set = 1;
            (*cc).cc_nrpn_msb = val;
        }
        UMP_CC_NRPN_LSB => {
            ret = fill_rpn(cc, data, channel, true);
            (*cc).nrpn_set = 1;
            (*cc).cc_nrpn_lsb = val;
        }
        UMP_CC_DATA => {
            (*cc).cc_data_msb_set = 1;
            (*cc).cc_data_msb = val;
            ret = fill_rpn(cc, data, channel, false);
        }
        UMP_CC_BANK_SELECT => {
            (*cc).bank_set = 1;
            (*cc).cc_bank_msb = val;
            ret = 0; // skip
        }
        UMP_CC_BANK_SELECT_LSB => {
            (*cc).bank_set = 1;
            (*cc).cc_bank_lsb = val;
            ret = 0; // skip
        }
        UMP_CC_DATA_LSB => {
            (*cc).cc_data_lsb_set = 1;
            (*cc).cc_data_lsb = val;
            ret = fill_rpn(cc, data, channel, false);
        }
        _ => {
            (*data).cc.status = status;
            (*data).cc.channel = channel;
            (*data).cc.index = index;
            (*data).cc.data = upscale_7_to_32bit(((*event).data.control.value & 0x7f) as u8);
            ret = 1;
        }
    }

    ret
}

/* convert one-parameter control event to MIDI 2.0 UMP */
unsafe fn ctrl_ev_to_ump_midi2(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi2_msg,
    status: u8,
) -> i32 {
    (*data).caf.status = status;
    (*data).caf.channel = (*event).data.control.channel & 0x0f;
    (*data).caf.data = upscale_7_to_32bit(((*event).data.control.value & 0x7f) as u8);
    1
}

/* convert program change event to MIDI 2.0 UMP */
unsafe fn pgm_ev_to_ump_midi2(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi2_msg,
    status: u8,
) -> i32 {
    let channel: u8 = (*event).data.control.channel & 0x0f;
    let cc: *mut ump_cvt_to_ump_bank = (*dest_port).midi2_bank.as_mut_ptr().add(channel as usize);

    (*data).pg.status = status;
    (*data).pg.channel = channel;
    (*data).pg.program = ((*event).data.control.value & 0x7f) as u8;
    if (*cc).bank_set != 0 {
        (*data).pg.bank_valid = 1;
        (*data).pg.bank_msb = (*cc).cc_bank_msb;
        (*data).pg.bank_lsb = (*cc).cc_bank_lsb;
        (*cc).bank_set = 0;
    }
    1
}

/* convert pitchbend event to MIDI 2.0 UMP */
unsafe fn pitchbend_ev_to_ump_midi2(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi2_msg,
    status: u8,
) -> i32 {
    let mut val: i32 = (*event).data.control.value + 8192;

    val = clamp(val, 0, 0x3fff);
    (*data).pb.status = status;
    (*data).pb.channel = (*event).data.control.channel & 0x0f;
    (*data).pb.data = upscale_14_to_32bit(val as u16);
    1
}

/* convert 14bit control event to MIDI 2.0 UMP; split to two events */
unsafe fn ctrl14_ev_to_ump_midi2(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi2_msg,
    status: u8,
) -> i32 {
    let channel: u8 = (*event).data.control.channel & 0x0f;
    let index: u8 = ((*event).data.control.param & 0x7f) as u8;
    let cc: *mut ump_cvt_to_ump_bank = (*dest_port).midi2_bank.as_mut_ptr().add(channel as usize);
    let msb: u8;
    let lsb: u8;
    let ret: i32;

    msb = (((*event).data.control.value >> 7) & 0x7f) as u8;
    lsb = ((*event).data.control.value & 0x7f) as u8;
    /* process special CC's (bank/rpn/nrpn) */
    match index {
        UMP_CC_BANK_SELECT => {
            (*cc).cc_bank_msb = msb;
            (*cc).bank_set = 1;
            (*cc).cc_bank_lsb = lsb;
            return 0; // skip
        }
        UMP_CC_BANK_SELECT_LSB => {
            (*cc).bank_set = 1;
            (*cc).cc_bank_lsb = lsb;
            return 0; // skip
        }
        UMP_CC_RPN_MSB | UMP_CC_RPN_LSB => {
            ret = fill_rpn(cc, data, channel, true);
            (*cc).cc_rpn_msb = msb;
            (*cc).cc_rpn_lsb = lsb;
            (*cc).rpn_set = 1;
            if (*cc).cc_rpn_msb == 0x7f && (*cc).cc_rpn_lsb == 0x7f {
                reset_rpn(cc);
            }
            return ret;
        }
        UMP_CC_NRPN_MSB | UMP_CC_NRPN_LSB => {
            ret = fill_rpn(cc, data, channel, true);
            (*cc).cc_nrpn_msb = msb;
            (*cc).nrpn_set = 1;
            (*cc).cc_nrpn_lsb = lsb;
            return ret;
        }
        UMP_CC_DATA | UMP_CC_DATA_LSB => {
            (*cc).cc_data_lsb_set = 1;
            (*cc).cc_data_msb_set = (*cc).cc_data_lsb_set;
            (*cc).cc_data_msb = msb;
            (*cc).cc_data_lsb = lsb;
            return fill_rpn(cc, data, channel, false);
        }
        _ => {}
    }

    (*data).cc.status = UMP_MSG_STATUS_CC;
    (*data).cc.channel = channel;
    (*data).cc.index = index;
    if (*event).data.control.param < 0x20 {
        (*data).cc.data = upscale_7_to_32bit(msb);
        *data.add(1) = *data;
        (*data.add(1)).cc.index = ((*event).data.control.param | 0x20) as u8;
        (*data.add(1)).cc.data = upscale_7_to_32bit(lsb);
        return 2;
    }

    (*data).cc.data = upscale_7_to_32bit(lsb);
    1
}

/* convert RPN/NRPN event to MIDI 2.0 UMP */
unsafe fn rpn_ev_to_ump_midi2(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi2_msg,
    status: u8,
) -> i32 {
    (*data).rpn.status = status;
    (*data).rpn.channel = (*event).data.control.channel;
    (*data).rpn.bank = (((*event).data.control.param >> 7) & 0x7f) as u8;
    (*data).rpn.index = ((*event).data.control.param & 0x7f) as u8;
    (*data).rpn.data = upscale_14_to_32bit(((*event).data.control.value & 0x3fff) as u16);
    1
}

/* convert system / RT message to UMP */
unsafe fn system_ev_to_ump_midi2(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi2_msg,
    status: u8,
) -> i32 {
    system_ev_to_ump_midi1(event, dest_port, data as *mut snd_ump_midi1_msg, status)
}

/* convert system / RT message with 1 parameter to UMP */
unsafe fn system_1p_ev_to_ump_midi2(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi2_msg,
    status: u8,
) -> i32 {
    system_1p_ev_to_ump_midi1(event, dest_port, data as *mut snd_ump_midi1_msg, status)
}

/* convert system / RT message with two parameters to UMP */
unsafe fn system_2p_ev_to_ump_midi2(
    event: *const snd_seq_event,
    dest_port: *mut snd_seq_client_port,
    data: *mut snd_ump_midi2_msg,
    status: u8,
) -> i32 {
    system_2p_ev_to_ump_midi1(event, dest_port, data as *mut snd_ump_midi1_msg, status)
}

struct seq_ev_to_ump {
    seq_type: i32,
    status: u8,
    midi1_encode: unsafe fn(*const snd_seq_event, *mut snd_seq_client_port, *mut snd_ump_midi1_msg, u8) -> i32,
    midi2_encode: unsafe fn(*const snd_seq_event, *mut snd_seq_client_port, *mut snd_ump_midi2_msg, u8) -> i32,
}

static seq_ev_ump_encoders: [seq_ev_to_ump; 20] = [
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_NOTEON, status: UMP_MSG_STATUS_NOTE_ON, midi1_encode: note_ev_to_ump_midi1, midi2_encode: note_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_NOTEOFF, status: UMP_MSG_STATUS_NOTE_OFF, midi1_encode: note_ev_to_ump_midi1, midi2_encode: note_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_KEYPRESS, status: UMP_MSG_STATUS_POLY_PRESSURE, midi1_encode: note_ev_to_ump_midi1, midi2_encode: paf_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_CONTROLLER, status: UMP_MSG_STATUS_CC, midi1_encode: cc_ev_to_ump_midi1, midi2_encode: cc_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_PGMCHANGE, status: UMP_MSG_STATUS_PROGRAM, midi1_encode: ctrl_ev_to_ump_midi1, midi2_encode: pgm_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_CHANPRESS, status: UMP_MSG_STATUS_CHANNEL_PRESSURE, midi1_encode: ctrl_ev_to_ump_midi1, midi2_encode: ctrl_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_PITCHBEND, status: UMP_MSG_STATUS_PITCH_BEND, midi1_encode: pitchbend_ev_to_ump_midi1, midi2_encode: pitchbend_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_CONTROL14, status: 0, midi1_encode: ctrl14_ev_to_ump_midi1, midi2_encode: ctrl14_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_NONREGPARAM, status: UMP_MSG_STATUS_NRPN, midi1_encode: rpn_ev_to_ump_midi1, midi2_encode: rpn_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_REGPARAM, status: UMP_MSG_STATUS_RPN, midi1_encode: rpn_ev_to_ump_midi1, midi2_encode: rpn_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_QFRAME, status: UMP_SYSTEM_STATUS_MIDI_TIME_CODE, midi1_encode: system_1p_ev_to_ump_midi1, midi2_encode: system_1p_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_SONGPOS, status: UMP_SYSTEM_STATUS_SONG_POSITION, midi1_encode: system_2p_ev_to_ump_midi1, midi2_encode: system_2p_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_SONGSEL, status: UMP_SYSTEM_STATUS_SONG_SELECT, midi1_encode: system_1p_ev_to_ump_midi1, midi2_encode: system_1p_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_TUNE_REQUEST, status: UMP_SYSTEM_STATUS_TUNE_REQUEST, midi1_encode: system_ev_to_ump_midi1, midi2_encode: system_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_CLOCK, status: UMP_SYSTEM_STATUS_TIMING_CLOCK, midi1_encode: system_ev_to_ump_midi1, midi2_encode: system_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_START, status: UMP_SYSTEM_STATUS_START, midi1_encode: system_ev_to_ump_midi1, midi2_encode: system_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_CONTINUE, status: UMP_SYSTEM_STATUS_CONTINUE, midi1_encode: system_ev_to_ump_midi1, midi2_encode: system_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_STOP, status: UMP_SYSTEM_STATUS_STOP, midi1_encode: system_ev_to_ump_midi1, midi2_encode: system_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_SENSING, status: UMP_SYSTEM_STATUS_ACTIVE_SENSING, midi1_encode: system_ev_to_ump_midi1, midi2_encode: system_ev_to_ump_midi2 },
    seq_ev_to_ump { seq_type: SNDRV_SEQ_EVENT_RESET, status: UMP_SYSTEM_STATUS_RESET, midi1_encode: system_ev_to_ump_midi1, midi2_encode: system_ev_to_ump_midi2 },
];

fn find_ump_encoder(type_: i32) -> *const seq_ev_to_ump {
    let mut i: usize = 0;

    while i < seq_ev_ump_encoders.len() {
        if seq_ev_ump_encoders[i].seq_type == type_ {
            return &seq_ev_ump_encoders[i];
        }
        i += 1;
    }

    core::ptr::null()
}

unsafe fn setup_ump_event(dest: *mut snd_seq_ump_event, src: *const snd_seq_event) {
    core::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, core::mem::size_of::<snd_seq_event>());
    (*dest).type_ = 0;
    (*dest).flags |= SNDRV_SEQ_EVENT_UMP;
    (*dest).flags &= !SNDRV_SEQ_EVENT_LENGTH_MASK;
    core::ptr::write_bytes((*dest).ump.as_mut_ptr(), 0, (*dest).ump.len());
}

/* Convert ALSA seq event to UMP MIDI 1.0 and deliver it */
unsafe fn cvt_to_ump_midi1(
    dest: *mut snd_seq_client,
    dest_port: *mut snd_seq_client_port,
    event: *mut snd_seq_event,
    atomic: i32,
    hop: i32,
) -> i32 {
    let encoder: *const seq_ev_to_ump;
    let mut ev_cvt: snd_seq_ump_event = core::mem::zeroed();
    let mut data: [snd_ump_midi1_msg; 4] = core::mem::zeroed();
    let mut i: i32;
    let n: i32;
    let mut err: i32;

    encoder = find_ump_encoder((*event).type_);
    if encoder.is_null() {
        return __snd_seq_deliver_single_event(dest, dest_port, event, atomic, hop);
    }

    data[0].raw = make_raw_ump(dest_port, UMP_MSG_TYPE_MIDI1_CHANNEL_VOICE);
    n = ((*encoder).midi1_encode)(event, dest_port, data.as_mut_ptr(), (*encoder).status);
    if n == 0 {
        return 0;
    }

    setup_ump_event(&mut ev_cvt, event);
    i = 0;
    while i < n {
        ev_cvt.ump[0] = data[i as usize].raw;
        err = __snd_seq_deliver_single_event(dest, dest_port, &mut ev_cvt as *mut _ as *mut snd_seq_event, atomic, hop);
        if err < 0 {
            return err;
        }
        i += 1;
    }

    0
}

/* Convert ALSA seq event to UMP MIDI 2.0 and deliver it */
unsafe fn cvt_to_ump_midi2(
    dest: *mut snd_seq_client,
    dest_port: *mut snd_seq_client_port,
    event: *mut snd_seq_event,
    atomic: i32,
    hop: i32,
) -> i32 {
    let encoder: *const seq_ev_to_ump;
    let mut ev_cvt: snd_seq_ump_event = core::mem::zeroed();
    let mut data: [snd_ump_midi2_msg; 2] = core::mem::zeroed();
    let mut i: i32;
    let n: i32;
    let mut err: i32;

    encoder = find_ump_encoder((*event).type_);
    if encoder.is_null() {
        return __snd_seq_deliver_single_event(dest, dest_port, event, atomic, hop);
    }

    data[0].raw[0] = make_raw_ump(dest_port, UMP_MSG_TYPE_MIDI2_CHANNEL_VOICE);
    data[0].raw[1] = 0;
    n = ((*encoder).midi2_encode)(event, dest_port, data.as_mut_ptr(), (*encoder).status);
    if n == 0 {
        return 0;
    }

    setup_ump_event(&mut ev_cvt, event);
    i = 0;
    while i < n {
        core::ptr::copy_nonoverlapping(
            &data[i as usize] as *const _ as *const u8,
            ev_cvt.ump.as_mut_ptr() as *mut u8,
            core::mem::size_of::<snd_ump_midi2_msg>(),
        );
        err = __snd_seq_deliver_single_event(dest, dest_port, &mut ev_cvt as *mut _ as *mut snd_seq_event, atomic, hop);
        if err < 0 {
            return err;
        }
        i += 1;
    }

    0
}

/* Fill up a sysex7 UMP from the byte stream */
unsafe fn fill_sysex7_ump(dest_port: *mut snd_seq_client_port, val: *mut u32, status: u8, buf: *mut u8, len: i32) {
    core::ptr::write_bytes(val as *mut u8, 0, 8);
    core::ptr::copy_nonoverlapping(buf, (val as *mut u8).add(2), len as usize);
    /* C conditional: #ifdef __LITTLE_ENDIAN */
    #[cfg(target_endian = "little")]
    {
        swab32_array(val, 2);
    }
    *val.add(0) |= ump_compose(UMP_MSG_TYPE_DATA, get_ump_group(dest_port), status, len as u8);
}

/* Convert sysex var event to UMP sysex7 packets and deliver them */
unsafe fn cvt_sysex_to_ump(
    dest: *mut snd_seq_client,
    dest_port: *mut snd_seq_client_port,
    event: *mut snd_seq_event,
    atomic: i32,
    hop: i32,
) -> i32 {
    let mut ev_cvt: snd_seq_ump_event = core::mem::zeroed();
    let mut status: u8;
    let mut buf: [u8; 8] = [0; 8];
    let mut xbuf: *mut u8;
    let mut offset: i32 = 0;
    let mut len: i32;
    let mut err: i32;
    let mut finished: bool = false;

    if !snd_seq_ev_is_variable(event) {
        return 0;
    }

    setup_ump_event(&mut ev_cvt, event);
    while !finished {
        len = snd_seq_expand_var_event_at(event, buf.len(), buf.as_mut_ptr(), offset);
        if len <= 0 {
            break;
        }
        if WARN_ON(len > buf.len() as i32) {
            break;
        }

        xbuf = buf.as_mut_ptr();
        status = UMP_SYSEX_STATUS_CONTINUE;
        /* truncate the sysex start-marker */
        if *xbuf == UMP_MIDI1_MSG_SYSEX_START {
            status = UMP_SYSEX_STATUS_START;
            len -= 1;
            offset += 1;
            xbuf = xbuf.add(1);
        }

        /* if the last of this packet or the 1st byte of the next packet
         * is the end-marker, finish the transfer with this packet
         */
        if len > 0 && len < 8 && *xbuf.add((len - 1) as usize) == UMP_MIDI1_MSG_SYSEX_END {
            if status == UMP_SYSEX_STATUS_START {
                status = UMP_SYSEX_STATUS_SINGLE;
            } else {
                status = UMP_SYSEX_STATUS_END;
            }
            len -= 1;
            finished = true;
        }

        len = core::cmp::min(len, 6);
        fill_sysex7_ump(dest_port, ev_cvt.ump.as_mut_ptr(), status, xbuf, len);
        err = __snd_seq_deliver_single_event(dest, dest_port, &mut ev_cvt as *mut _ as *mut snd_seq_event, atomic, hop);
        if err < 0 {
            return err;
        }
        offset += len;
    }
    0
}

/* Convert to UMP packet and deliver */
pub unsafe fn snd_seq_deliver_to_ump(
    source: *mut snd_seq_client,
    dest: *mut snd_seq_client,
    dest_port: *mut snd_seq_client_port,
    event: *mut snd_seq_event,
    atomic: i32,
    hop: i32,
) -> i32 {
    if ((*dest).group_filter & (1_u32 << (*dest_port).ump_group)) != 0 {
        return 0; /* group filtered - skip the event */
    }
    if (*event).type_ == SNDRV_SEQ_EVENT_SYSEX {
        cvt_sysex_to_ump(dest, dest_port, event, atomic, hop)
    } else if snd_seq_client_is_midi2(dest) && (*dest_port).is_midi1 == 0 {
        cvt_to_ump_midi2(dest, dest_port, event, atomic, hop)
    } else {
        cvt_to_ump_midi1(dest, dest_port, event, atomic, hop)
    }
}

/* return the UMP group-port number of the event;
 * return -1 if groupless or non-UMP event
 */
pub unsafe fn snd_seq_ump_group_port(event: *const snd_seq_event) -> i32 {
    let ump_ev = event as *const snd_seq_ump_event;
    let typ: u8;

    if !snd_seq_ev_is_ump(event) {
        return -1;
    }
    typ = ump_message_type((*ump_ev).ump[0]);
    if ump_is_groupless_msg(typ) {
        return -1;
    }
    /* group-port number starts from 1 */
    (ump_message_group((*ump_ev).ump[0]) + 1) as i32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
