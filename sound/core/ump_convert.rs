// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Helpers for UMP <-> MIDI 1.0 byte stream conversion
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_int, c_uint, c_uchar};

type u8 = c_uchar;
type u16 = core::ffi::c_ushort;
type u32 = c_uint;

/* Constants and helper functions are supplied by the translated UMP headers. */

/*
 * Minimal C layouts used by this implementation source. The full definitions
 * are supplied by the translated headers in the complete repository.
 */
#[repr(C)]
pub struct ump_cvt_to_ump_bank {
    pub rpn_set: u8,
    pub nrpn_set: u8,
    pub cc_rpn_msb: u8,
    pub cc_rpn_lsb: u8,
    pub cc_nrpn_msb: u8,
    pub cc_nrpn_lsb: u8,
    pub cc_data_msb: u8,
    pub cc_data_lsb: u8,
    pub cc_data_msb_set: u8,
    pub cc_data_lsb_set: u8,
    pub bank_set: u8,
    pub cc_bank_msb: u8,
    pub cc_bank_lsb: u8,
}

#[repr(C)]
pub struct ump_cvt_to_ump {
    pub buf: [u8; 8],
    pub len: c_int,
    pub cmd_bytes: u8,
    pub in_sysex: c_int,
    pub bank: [ump_cvt_to_ump_bank; 16],
    pub ump: *mut u32,
    pub ump_bytes: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_note {
    pub status: u8,
    pub channel: u8,
    pub note: u8,
    pub velocity: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_paf {
    pub status: u8,
    pub channel: u8,
    pub note: u8,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_cc {
    pub status: u8,
    pub channel: u8,
    pub index: u8,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_caf {
    pub status: u8,
    pub channel: u8,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_pg {
    pub status: u8,
    pub channel: u8,
    pub bank_valid: u8,
    pub program: u8,
    pub bank_msb: u8,
    pub bank_lsb: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_pb {
    pub status: u8,
    pub channel: u8,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_midi2_rpn {
    pub status: u8,
    pub channel: u8,
    pub bank: u8,
    pub index: u8,
    pub data: u32,
}

#[repr(C)]
pub union snd_ump_midi2_msg {
    pub note: snd_ump_midi2_note,
    pub paf: snd_ump_midi2_paf,
    pub cc: snd_ump_midi2_cc,
    pub caf: snd_ump_midi2_caf,
    pub pg: snd_ump_midi2_pg,
    pub pb: snd_ump_midi2_pb,
    pub rpn: snd_ump_midi2_rpn,
}

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
    let val: u16;
    let repeat: u16;

    val = (src as u16) << 9;
    if src <= 0x40 {
        return val;
    }
    repeat = (src & 0x3f) as u16;
    val | (repeat << 3) | (repeat >> 3)
}

fn upscale_7_to_32bit(src: u8) -> u32 {
    let val: u32;
    let repeat: u32;

    val = (src as u32) << 25;
    if src <= 0x40 {
        return val;
    }
    repeat = (src & 0x3f) as u32;
    val | (repeat << 19) | (repeat << 13) | (repeat << 7) | (repeat << 1) | (repeat >> 5)
}

fn upscale_14_to_32bit(src: u16) -> u32 {
    let val: u32;
    let repeat: u32;

    val = (src as u32) << 18;
    if src <= 0x2000 {
        return val;
    }
    repeat = (src & 0x1fff) as u32;
    val | (repeat << 5) | (repeat >> 8)
}

/*
 * UMP -> MIDI 1 byte stream conversion
 */
/* convert a UMP System message to MIDI 1.0 byte stream */
unsafe fn cvt_ump_system_to_legacy(data: u32, buf: *mut c_uchar) -> c_int {
    *buf.add(0) = ump_message_status_channel(data);
    match ump_message_status_code(data) {
        UMP_SYSTEM_STATUS_MIDI_TIME_CODE | UMP_SYSTEM_STATUS_SONG_SELECT => {
            *buf.add(1) = ((data >> 8) & 0x7f) as c_uchar;
            2
        }
        UMP_SYSTEM_STATUS_SONG_POSITION => {
            *buf.add(1) = ((data >> 8) & 0x7f) as c_uchar;
            *buf.add(2) = (data & 0x7f) as c_uchar;
            3
        }
        _ => 1,
    }
}

/* convert a UMP MIDI 1.0 Channel Voice message to MIDI 1.0 byte stream */
unsafe fn cvt_ump_midi1_to_legacy(data: u32, buf: *mut c_uchar) -> c_int {
    *buf.add(0) = ump_message_status_channel(data);
    *buf.add(1) = ((data >> 8) & 0xff) as c_uchar;
    match ump_message_status_code(data) {
        UMP_MSG_STATUS_PROGRAM | UMP_MSG_STATUS_CHANNEL_PRESSURE => 2,
        _ => {
            *buf.add(2) = (data & 0xff) as c_uchar;
            3
        }
    }
}

/* convert a UMP MIDI 2.0 Channel Voice message to MIDI 1.0 byte stream */
unsafe fn cvt_ump_midi2_to_legacy(midi2: *const snd_ump_midi2_msg, buf: *mut c_uchar) -> c_int {
    let status: c_uchar = (*midi2).note.status;
    let channel: c_uchar = (*midi2).note.channel;
    let mut v: u16;

    *buf.add(0) = (status << 4) | channel;
    match status as c_uint {
        UMP_MSG_STATUS_NOTE_OFF | UMP_MSG_STATUS_NOTE_ON => {
            *buf.add(1) = (*midi2).note.note;
            *buf.add(2) = downscale_16_to_7bit((*midi2).note.velocity);
            if status as c_uint == UMP_MSG_STATUS_NOTE_ON && *buf.add(2) == 0 {
                *buf.add(2) = 1;
            }
            3
        }
        UMP_MSG_STATUS_POLY_PRESSURE => {
            *buf.add(1) = (*midi2).paf.note;
            *buf.add(2) = downscale_32_to_7bit((*midi2).paf.data);
            3
        }
        UMP_MSG_STATUS_CC => {
            *buf.add(1) = (*midi2).cc.index;
            *buf.add(2) = downscale_32_to_7bit((*midi2).cc.data);
            3
        }
        UMP_MSG_STATUS_CHANNEL_PRESSURE => {
            *buf.add(1) = downscale_32_to_7bit((*midi2).caf.data);
            2
        }
        UMP_MSG_STATUS_PROGRAM => {
            if (*midi2).pg.bank_valid != 0 {
                *buf.add(0) = channel | ((UMP_MSG_STATUS_CC as u8) << 4);
                *buf.add(1) = UMP_CC_BANK_SELECT as u8;
                *buf.add(2) = (*midi2).pg.bank_msb;
                *buf.add(3) = channel | ((UMP_MSG_STATUS_CC as u8) << 4);
                *buf.add(4) = UMP_CC_BANK_SELECT_LSB as u8;
                *buf.add(5) = (*midi2).pg.bank_lsb;
                *buf.add(6) = channel | ((UMP_MSG_STATUS_PROGRAM as u8) << 4);
                *buf.add(7) = (*midi2).pg.program;
                return 8;
            }
            *buf.add(1) = (*midi2).pg.program;
            2
        }
        UMP_MSG_STATUS_PITCH_BEND => {
            v = downscale_32_to_14bit((*midi2).pb.data);
            *buf.add(1) = (v & 0x7f) as u8;
            *buf.add(2) = (v >> 7) as u8;
            3
        }
        UMP_MSG_STATUS_RPN | UMP_MSG_STATUS_NRPN => {
            *buf.add(0) = channel | ((UMP_MSG_STATUS_CC as u8) << 4);
            *buf.add(1) = if status as c_uint == UMP_MSG_STATUS_RPN { UMP_CC_RPN_MSB } else { UMP_CC_NRPN_MSB } as u8;
            *buf.add(2) = (*midi2).rpn.bank;
            *buf.add(3) = *buf.add(0);
            *buf.add(4) = if status as c_uint == UMP_MSG_STATUS_RPN { UMP_CC_RPN_LSB } else { UMP_CC_NRPN_LSB } as u8;
            *buf.add(5) = (*midi2).rpn.index;
            *buf.add(6) = *buf.add(0);
            *buf.add(7) = UMP_CC_DATA as u8;
            v = downscale_32_to_14bit((*midi2).rpn.data);
            *buf.add(8) = (v >> 7) as u8;
            *buf.add(9) = *buf.add(0);
            *buf.add(10) = UMP_CC_DATA_LSB as u8;
            *buf.add(11) = (v & 0x7f) as u8;
            12
        }
        _ => 0,
    }
}

/* convert a UMP 7-bit SysEx message to MIDI 1.0 byte stream */
unsafe fn cvt_ump_sysex7_to_legacy(mut data: *const u32, buf: *mut c_uchar) -> c_int {
    let mut status: c_uchar;
    let mut bytes: c_uchar;
    let mut size: c_int;
    let mut offset: c_int;

    status = ump_sysex_message_status(*data);
    if status as c_uint > UMP_SYSEX_STATUS_END {
        return 0; // unsupported, skip
    }
    bytes = ump_sysex_message_length(*data);
    if bytes > 6 {
        return 0; // skip
    }

    size = 0;
    if status as c_uint == UMP_SYSEX_STATUS_SINGLE || status as c_uint == UMP_SYSEX_STATUS_START {
        *buf.add(0) = UMP_MIDI1_MSG_SYSEX_START as u8;
        size = 1;
    }

    offset = 8;
    while bytes != 0 {
        *buf.add(size as usize) = ((*data >> offset) & 0x7f) as u8;
        if offset == 0 {
            offset = 24;
            data = data.add(1);
        } else {
            offset -= 8;
        }
        bytes -= 1;
        size += 1;
    }

    if status as c_uint == UMP_SYSEX_STATUS_SINGLE || status as c_uint == UMP_SYSEX_STATUS_END {
        *buf.add(size as usize) = UMP_MIDI1_MSG_SYSEX_END as u8;
        size += 1;
    }

    size
}

/**
 * snd_ump_convert_from_ump - convert from UMP to legacy MIDI
 * @data: UMP packet
 * @buf: buffer to store legacy MIDI data
 * @group_ret: pointer to store the target group
 *
 * Convert from a UMP packet @data to MIDI 1.0 bytes at @buf.
 * The target group is stored at @group_ret.
 *
 * The function returns the number of bytes of MIDI 1.0 stream.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ump_convert_from_ump(
    data: *const u32,
    buf: *mut c_uchar,
    group_ret: *mut c_uchar,
) -> c_int {
    *group_ret = ump_message_group(*data);

    match ump_message_type(*data) {
        UMP_MSG_TYPE_SYSTEM => cvt_ump_system_to_legacy(*data, buf),
        UMP_MSG_TYPE_MIDI1_CHANNEL_VOICE => cvt_ump_midi1_to_legacy(*data, buf),
        UMP_MSG_TYPE_MIDI2_CHANNEL_VOICE => {
            cvt_ump_midi2_to_legacy(data as *const snd_ump_midi2_msg, buf)
        }
        UMP_MSG_TYPE_DATA => cvt_ump_sysex7_to_legacy(data, buf),
        _ => 0,
    }
}

/*
 * MIDI 1 byte stream -> UMP conversion
 */
/* convert MIDI 1.0 SysEx to a UMP packet */
unsafe fn cvt_legacy_sysex_to_ump(
    cvt: *mut ump_cvt_to_ump,
    group: c_uchar,
    mut data: *mut u32,
    finish: bool,
) -> c_int {
    let status: c_uchar;
    let start: bool = (*cvt).in_sysex == 1;
    let mut i: c_int;
    let mut offset: c_int;

    if start && finish {
        status = UMP_SYSEX_STATUS_SINGLE as u8;
    } else if start {
        status = UMP_SYSEX_STATUS_START as u8;
    } else if finish {
        status = UMP_SYSEX_STATUS_END as u8;
    } else {
        status = UMP_SYSEX_STATUS_CONTINUE as u8;
    }
    *data = ump_compose(UMP_MSG_TYPE_DATA, group, status, (*cvt).len as u8);
    *data.add(1) = 0;
    offset = 8;
    i = 0;
    while i < (*cvt).len {
        *data |= ((*cvt).buf[i as usize] as u32) << offset;
        if offset == 0 {
            offset = 24;
            data = data.add(1);
        } else {
            offset -= 8;
        }
        i += 1;
    }
    (*cvt).len = 0;
    if finish {
        (*cvt).in_sysex = 0;
    } else {
        (*cvt).in_sysex += 1;
    }
    8
}

/* convert to a UMP System message */
unsafe fn cvt_legacy_system_to_ump(
    cvt: *mut ump_cvt_to_ump,
    group: c_uchar,
    data: *mut u32,
) -> c_int {
    *data.add(0) = ump_compose(UMP_MSG_TYPE_SYSTEM, group, 0, (*cvt).buf[0]);
    if (*cvt).cmd_bytes > 1 {
        *data.add(0) |= ((*cvt).buf[1] as u32) << 8;
    }
    if (*cvt).cmd_bytes > 2 {
        *data.add(0) |= (*cvt).buf[2] as u32;
    }
    4
}

unsafe fn reset_rpn(cc: *mut ump_cvt_to_ump_bank) {
    (*cc).rpn_set = 0;
    (*cc).nrpn_set = 0;
    (*cc).cc_rpn_msb = 0;
    (*cc).cc_rpn_lsb = 0;
    (*cc).cc_data_msb = 0;
    (*cc).cc_data_lsb = 0;
    (*cc).cc_data_msb_set = 0;
    (*cc).cc_data_lsb_set = 0;
}

unsafe fn fill_rpn(cc: *mut ump_cvt_to_ump_bank, midi2: *mut snd_ump_midi2_msg, flush: bool) -> c_int {
    if !((*cc).cc_data_lsb_set != 0 || (*cc).cc_data_msb_set != 0) {
        return 0; // skip
    }
    /* when not flushing, wait for complete data set */
    if !flush && (!((*cc).cc_data_lsb_set != 0) || !((*cc).cc_data_msb_set != 0)) {
        return 0; // skip
    }

    if (*cc).rpn_set != 0 {
        (*midi2).rpn.status = UMP_MSG_STATUS_RPN as u8;
        (*midi2).rpn.bank = (*cc).cc_rpn_msb;
        (*midi2).rpn.index = (*cc).cc_rpn_lsb;
    } else if (*cc).nrpn_set != 0 {
        (*midi2).rpn.status = UMP_MSG_STATUS_NRPN as u8;
        (*midi2).rpn.bank = (*cc).cc_nrpn_msb;
        (*midi2).rpn.index = (*cc).cc_nrpn_lsb;
    } else {
        return 0; // skip
    }

    (*midi2).rpn.data = upscale_14_to_32bit((((*cc).cc_data_msb as u16) << 7) | (*cc).cc_data_lsb as u16);

    reset_rpn(cc);
    1
}

/* convert to a MIDI 1.0 Channel Voice message */
unsafe fn cvt_legacy_cmd_to_ump(
    cvt: *mut ump_cvt_to_ump,
    group: c_uchar,
    protocol: c_uint,
    data: *mut u32,
    bytes: c_uchar,
) -> c_int {
    let buf: *const c_uchar = (*cvt).buf.as_ptr();
    let cc: *mut ump_cvt_to_ump_bank;
    let midi2: *mut snd_ump_midi2_msg = data as *mut snd_ump_midi2_msg;
    let mut status: c_uchar;
    let channel: c_uchar;
    let mut ret: c_int;

    /* BUILD_BUG_ON(sizeof(union snd_ump_midi1_msg) != 4); */
    /* BUILD_BUG_ON(sizeof(union snd_ump_midi2_msg) != 8); */

    /* for MIDI 1.0 UMP, it's easy, just pack it into UMP */
    if protocol & SNDRV_UMP_EP_INFO_PROTO_MIDI1 != 0 {
        *data.add(0) = ump_compose(UMP_MSG_TYPE_MIDI1_CHANNEL_VOICE, group, 0, *buf.add(0));
        *data.add(0) |= (*buf.add(1) as u32) << 8;
        if bytes > 2 {
            *data.add(0) |= *buf.add(2) as u32;
        }
        return 4;
    }

    status = *buf >> 4;
    channel = *buf & 0x0f;
    cc = (*cvt).bank.as_mut_ptr().add(channel as usize);

    /* special handling: treat note-on with 0 velocity as note-off */
    if status as c_uint == UMP_MSG_STATUS_NOTE_ON && *buf.add(2) == 0 {
        status = UMP_MSG_STATUS_NOTE_OFF as u8;
    }

    /* initialize the packet */
    *data.add(0) = ump_compose(UMP_MSG_TYPE_MIDI2_CHANNEL_VOICE, group, status, channel);
    *data.add(1) = 0;

    match status as c_uint {
        UMP_MSG_STATUS_NOTE_ON | UMP_MSG_STATUS_NOTE_OFF => {
            (*midi2).note.note = *buf.add(1);
            (*midi2).note.velocity = upscale_7_to_16bit(*buf.add(2));
        }
        UMP_MSG_STATUS_POLY_PRESSURE => {
            (*midi2).paf.note = *buf.add(1);
            (*midi2).paf.data = upscale_7_to_32bit(*buf.add(2));
        }
        UMP_MSG_STATUS_CC => {
            match *buf.add(1) as c_uint {
                UMP_CC_RPN_MSB => {
                    ret = fill_rpn(cc, midi2, true);
                    (*cc).rpn_set = 1;
                    (*cc).cc_rpn_msb = *buf.add(2);
                    if (*cc).cc_rpn_msb == 0x7f && (*cc).cc_rpn_lsb == 0x7f {
                        reset_rpn(cc);
                    }
                    return ret;
                }
                UMP_CC_RPN_LSB => {
                    ret = fill_rpn(cc, midi2, true);
                    (*cc).rpn_set = 1;
                    (*cc).cc_rpn_lsb = *buf.add(2);
                    if (*cc).cc_rpn_msb == 0x7f && (*cc).cc_rpn_lsb == 0x7f {
                        reset_rpn(cc);
                    }
                    return ret;
                }
                UMP_CC_NRPN_MSB => {
                    ret = fill_rpn(cc, midi2, true);
                    (*cc).nrpn_set = 1;
                    (*cc).cc_nrpn_msb = *buf.add(2);
                    return ret;
                }
                UMP_CC_NRPN_LSB => {
                    ret = fill_rpn(cc, midi2, true);
                    (*cc).nrpn_set = 1;
                    (*cc).cc_nrpn_lsb = *buf.add(2);
                    return ret;
                }
                UMP_CC_DATA => {
                    (*cc).cc_data_msb_set = 1;
                    (*cc).cc_data_msb = *buf.add(2);
                    return fill_rpn(cc, midi2, false);
                }
                UMP_CC_BANK_SELECT => {
                    (*cc).bank_set = 1;
                    (*cc).cc_bank_msb = *buf.add(2);
                    return 0; // skip
                }
                UMP_CC_BANK_SELECT_LSB => {
                    (*cc).bank_set = 1;
                    (*cc).cc_bank_lsb = *buf.add(2);
                    return 0; // skip
                }
                UMP_CC_DATA_LSB => {
                    (*cc).cc_data_lsb_set = 1;
                    (*cc).cc_data_lsb = *buf.add(2);
                    return fill_rpn(cc, midi2, false);
                }
                _ => {
                    (*midi2).cc.index = *buf.add(1);
                    (*midi2).cc.data = upscale_7_to_32bit(*buf.add(2));
                }
            }
        }
        UMP_MSG_STATUS_PROGRAM => {
            (*midi2).pg.program = *buf.add(1);
            if (*cc).bank_set != 0 {
                (*midi2).pg.bank_valid = 1;
                (*midi2).pg.bank_msb = (*cc).cc_bank_msb;
                (*midi2).pg.bank_lsb = (*cc).cc_bank_lsb;
                (*cc).bank_set = 0;
            }
        }
        UMP_MSG_STATUS_CHANNEL_PRESSURE => {
            (*midi2).caf.data = upscale_7_to_32bit(*buf.add(1));
        }
        UMP_MSG_STATUS_PITCH_BEND => {
            (*midi2).pb.data = upscale_14_to_32bit((*buf.add(1) as u16) | ((*buf.add(2) as u16) << 7));
        }
        _ => return 0,
    }

    8
}

unsafe fn do_convert_to_ump(
    cvt: *mut ump_cvt_to_ump,
    group: c_uchar,
    protocol: c_uint,
    c: c_uchar,
    data: *mut u32,
) -> c_int {
    /* bytes for 0x80-0xf0 */
    static cmd_bytes: [c_uchar; 8] = [3, 3, 3, 3, 2, 2, 3, 0];
    /* bytes for 0xf0-0xff */
    static system_bytes: [c_uchar; 16] = [0, 2, 3, 2, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1];
    let mut bytes: c_uchar;

    if c as c_uint == UMP_MIDI1_MSG_SYSEX_START {
        (*cvt).in_sysex = 1;
        (*cvt).len = 0;
        return 0;
    }
    if c as c_uint == UMP_MIDI1_MSG_SYSEX_END {
        if (*cvt).in_sysex == 0 {
            return 0; /* skip */
        }
        return cvt_legacy_sysex_to_ump(cvt, group, data, true);
    }

    if (c as c_uint & 0xf0) == UMP_MIDI1_MSG_REALTIME {
        bytes = system_bytes[(c & 0x0f) as usize];
        if bytes == 0 {
            return 0; /* skip */
        }
        if bytes == 1 {
            *data.add(0) = ump_compose(UMP_MSG_TYPE_SYSTEM, group, 0, c);
            return 4;
        }
        (*cvt).buf[0] = c;
        (*cvt).len = 1;
        (*cvt).cmd_bytes = bytes;
        (*cvt).in_sysex = 0; /* abort SysEx */
        return 0;
    }

    if c & 0x80 != 0 {
        bytes = cmd_bytes[((c >> 4) & 7) as usize];
        (*cvt).buf[0] = c;
        (*cvt).len = 1;
        (*cvt).cmd_bytes = bytes;
        (*cvt).in_sysex = 0; /* abort SysEx */
        return 0;
    }

    if (*cvt).in_sysex != 0 {
        (*cvt).buf[(*cvt).len as usize] = c;
        (*cvt).len += 1;
        if (*cvt).len == 6 {
            return cvt_legacy_sysex_to_ump(cvt, group, data, false);
        }
        return 0;
    }

    if (*cvt).len == 0 {
        return 0;
    }

    (*cvt).buf[(*cvt).len as usize] = c;
    (*cvt).len += 1;
    if (*cvt).len < (*cvt).cmd_bytes as c_int {
        return 0;
    }
    (*cvt).len = 1;
    if ((*cvt).buf[0] as c_uint & 0xf0) == UMP_MIDI1_MSG_REALTIME {
        return cvt_legacy_system_to_ump(cvt, group, data);
    }
    cvt_legacy_cmd_to_ump(cvt, group, protocol, data, (*cvt).cmd_bytes)
}

/**
 * snd_ump_convert_to_ump - convert legacy MIDI byte to UMP packet
 * @cvt: converter context
 * @group: target UMP group
 * @protocol: target UMP protocol
 * @c: MIDI 1.0 byte data
 *
 * Feed a MIDI 1.0 byte @c and convert to a UMP packet if completed.
 * The result is stored in the buffer in @cvt.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_ump_convert_to_ump(
    cvt: *mut ump_cvt_to_ump,
    group: c_uchar,
    protocol: c_uint,
    c: c_uchar,
) {
    (*cvt).ump_bytes = do_convert_to_ump(cvt, group, protocol, c, (*cvt).ump);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
