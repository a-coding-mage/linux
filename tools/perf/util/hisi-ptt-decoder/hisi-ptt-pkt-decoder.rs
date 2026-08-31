// SPDX-License-Identifier: GPL-2.0
/*
 * HiSilicon PCIe Trace and Tuning (PTT) support
 * Copyright (c) 2022 HiSilicon Technologies Co., Ltd.
 */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stdout: *mut FILE;

    static PERF_COLOR_BLUE: *const c_char;

    static HISI_PTT_FIELD_LENGTH: c_int;
    static HISI_PTT_MAX_SPACE_LEN: c_int;
    static hisi_ptt_pkt_size: *const c_int;

    static HISI_PTT_HEAD0_4DW_FORMAT: c_uint;
    static HISI_PTT_HEAD0_4DW_TYPE: c_uint;
    static HISI_PTT_HEAD0_4DW_T9: c_uint;
    static HISI_PTT_HEAD0_4DW_T8: c_uint;
    static HISI_PTT_HEAD0_4DW_TH: c_uint;
    static HISI_PTT_HEAD0_4DW_SO: c_uint;
    static HISI_PTT_HEAD0_4DW_LEN: c_uint;
    static HISI_PTT_HEAD0_4DW_TIME: c_uint;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn color_fprintf(fp: *mut FILE, color: *const c_char, fmt: *const c_char, ...) -> c_int;
}

/*
 * For 8DW format, the bit[31:11] of DW0 is always 0x1fffff, which can be
 * used to distinguish the data format.
 * 8DW format is like:
 *   bits [                 31:11                 ][       10:0       ]
 *        |---------------------------------------|-------------------|
 *    DW0 [                0x1fffff               ][ Reserved (0x7ff) ]
 *    DW1 [                       Prefix                              ]
 *    DW2 [                     Header DW0                            ]
 *    DW3 [                     Header DW1                            ]
 *    DW4 [                     Header DW2                            ]
 *    DW5 [                     Header DW3                            ]
 *    DW6 [                   Reserved (0x0)                          ]
 *    DW7 [                        Time                               ]
 *
 * 4DW format is like:
 *   bits [31:30] [ 29:25 ][24][23][22][21][    20:11   ][    10:0    ]
 *        |-----|---------|---|---|---|---|-------------|-------------|
 *    DW0 [ Fmt ][  Type  ][T9][T8][TH][SO][   Length   ][    Time    ]
 *    DW1 [                     Header DW1                            ]
 *    DW2 [                     Header DW2                            ]
 *    DW3 [                     Header DW3                            ]
 */

const HISI_PTT_8DW_CHK_AND_RSV0: c_int = 0;
const HISI_PTT_8DW_PREFIX: c_int = 1;
const HISI_PTT_8DW_HEAD0: c_int = 2;
const HISI_PTT_8DW_HEAD1: c_int = 3;
const HISI_PTT_8DW_HEAD2: c_int = 4;
const HISI_PTT_8DW_HEAD3: c_int = 5;
const HISI_PTT_8DW_RSV1: c_int = 6;
const HISI_PTT_8DW_TIME: c_int = 7;
const HISI_PTT_8DW_TYPE_MAX: c_int = 8;

const HISI_PTT_4DW_HEAD1: c_int = 0;
const HISI_PTT_4DW_HEAD2: c_int = 1;
const HISI_PTT_4DW_HEAD3: c_int = 2;
const HISI_PTT_4DW_TYPE_MAX: c_int = 3;

pub type hisi_ptt_pkt_type = c_uint;

const HISI_PTT_8DW_PKT: hisi_ptt_pkt_type = 0;
const HISI_PTT_4DW_PKT: hisi_ptt_pkt_type = 1;

static hisi_ptt_8dw_pkt_field_name: [*const c_char; HISI_PTT_8DW_TYPE_MAX as usize] = [
    ptr::null(),
    b"Prefix\0".as_ptr() as *const c_char,
    b"Header DW0\0".as_ptr() as *const c_char,
    b"Header DW1\0".as_ptr() as *const c_char,
    b"Header DW2\0".as_ptr() as *const c_char,
    b"Header DW3\0".as_ptr() as *const c_char,
    ptr::null(),
    b"Time\0".as_ptr() as *const c_char,
];

static hisi_ptt_4dw_pkt_field_name: [*const c_char; HISI_PTT_4DW_TYPE_MAX as usize] = [
    b"Header DW1\0".as_ptr() as *const c_char,
    b"Header DW2\0".as_ptr() as *const c_char,
    b"Header DW3\0".as_ptr() as *const c_char,
];

unsafe fn get_unaligned_le32(buf: *const c_uchar) -> u32 {
    u32::from_le(ptr::read_unaligned(buf as *const u32))
}

fn FIELD_GET(mask: c_uint, reg: u32) -> c_uint {
    ((reg as c_uint) & mask) >> mask.trailing_zeros()
}

unsafe fn hisi_ptt_print_pkt(buf: *const c_uchar, mut pos: c_int, desc: *const c_char) {
    let color: *const c_char = PERF_COLOR_BLUE;
    let mut byte: u8;
    let dw: u32;
    let mut i: c_int;

    dw = get_unaligned_le32(buf.offset(pos as isize));
    printf(b".\0".as_ptr() as *const c_char);
    color_fprintf(
        stdout,
        color,
        b"  %08x: \0".as_ptr() as *const c_char,
        pos,
    );
    i = 0;
    while i < HISI_PTT_FIELD_LENGTH {
        byte = ((dw >> (24 - i * 8)) & 0xFF) as u8;
        color_fprintf(
            stdout,
            color,
            b"%02x \0".as_ptr() as *const c_char,
            byte as c_int,
        );
        i += 1;
    }
    i = 0;
    while i < HISI_PTT_MAX_SPACE_LEN {
        color_fprintf(stdout, color, b"   \0".as_ptr() as *const c_char);
        i += 1;
    }
    color_fprintf(
        stdout,
        color,
        b"  %s\n\0".as_ptr() as *const c_char,
        desc,
    );
}

unsafe fn hisi_ptt_8dw_pkt_desc(buf: *const c_uchar, mut pos: c_int) -> c_int {
    let mut i: c_int;

    i = 0;
    while i < HISI_PTT_8DW_TYPE_MAX {
        /* Do not show 8DW check field and reserved fields */
        if i == HISI_PTT_8DW_CHK_AND_RSV0 || i == HISI_PTT_8DW_RSV1 {
            pos += HISI_PTT_FIELD_LENGTH;
            i += 1;
            continue;
        }

        hisi_ptt_print_pkt(buf, pos, hisi_ptt_8dw_pkt_field_name[i as usize]);
        pos += HISI_PTT_FIELD_LENGTH;
        i += 1;
    }

    *hisi_ptt_pkt_size.offset(HISI_PTT_8DW_PKT as isize)
}

unsafe fn hisi_ptt_4dw_print_dw0(buf: *const c_uchar, pos: c_int) {
    let color: *const c_char = PERF_COLOR_BLUE;
    let mut byte: u8;
    let dw: u32;
    let mut i: c_int;

    dw = get_unaligned_le32(buf.offset(pos as isize));
    printf(b".\0".as_ptr() as *const c_char);
    color_fprintf(
        stdout,
        color,
        b"  %08x: \0".as_ptr() as *const c_char,
        pos,
    );
    i = 0;
    while i < HISI_PTT_FIELD_LENGTH {
        byte = ((dw >> (24 - i * 8)) & 0xFF) as u8;
        color_fprintf(
            stdout,
            color,
            b"%02x \0".as_ptr() as *const c_char,
            byte as c_int,
        );
        i += 1;
    }
    i = 0;
    while i < HISI_PTT_MAX_SPACE_LEN {
        color_fprintf(stdout, color, b"   \0".as_ptr() as *const c_char);
        i += 1;
    }

    color_fprintf(
        stdout,
        color,
        b"  %s %x %s %x %s %x %s %x %s %x %s %x %s %x %s %x\n\0".as_ptr() as *const c_char,
        b"Format\0".as_ptr() as *const c_char,
        FIELD_GET(HISI_PTT_HEAD0_4DW_FORMAT, dw),
        b"Type\0".as_ptr() as *const c_char,
        FIELD_GET(HISI_PTT_HEAD0_4DW_TYPE, dw),
        b"T9\0".as_ptr() as *const c_char,
        FIELD_GET(HISI_PTT_HEAD0_4DW_T9, dw),
        b"T8\0".as_ptr() as *const c_char,
        FIELD_GET(HISI_PTT_HEAD0_4DW_T8, dw),
        b"TH\0".as_ptr() as *const c_char,
        FIELD_GET(HISI_PTT_HEAD0_4DW_TH, dw),
        b"SO\0".as_ptr() as *const c_char,
        FIELD_GET(HISI_PTT_HEAD0_4DW_SO, dw),
        b"Length\0".as_ptr() as *const c_char,
        FIELD_GET(HISI_PTT_HEAD0_4DW_LEN, dw),
        b"Time\0".as_ptr() as *const c_char,
        FIELD_GET(HISI_PTT_HEAD0_4DW_TIME, dw),
    );
}

unsafe fn hisi_ptt_4dw_pkt_desc(buf: *const c_uchar, mut pos: c_int) -> c_int {
    let mut i: c_int;

    hisi_ptt_4dw_print_dw0(buf, pos);
    pos += HISI_PTT_FIELD_LENGTH;

    i = 0;
    while i < HISI_PTT_4DW_TYPE_MAX {
        hisi_ptt_print_pkt(buf, pos, hisi_ptt_4dw_pkt_field_name[i as usize]);
        pos += HISI_PTT_FIELD_LENGTH;
        i += 1;
    }

    *hisi_ptt_pkt_size.offset(HISI_PTT_4DW_PKT as isize)
}

#[no_mangle]
pub unsafe extern "C" fn hisi_ptt_pkt_desc(
    buf: *const c_uchar,
    pos: c_int,
    type_: hisi_ptt_pkt_type,
) -> c_int {
    if type_ == HISI_PTT_8DW_PKT {
        return hisi_ptt_8dw_pkt_desc(buf, pos);
    }

    hisi_ptt_4dw_pkt_desc(buf, pos)
}
