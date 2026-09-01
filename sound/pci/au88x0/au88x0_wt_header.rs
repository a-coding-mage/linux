/* SPDX-License-Identifier: GPL-2.0 */
/***************************************************************************
 *           WT register offsets.
 *
 *  Wed Oct 22 13:50:20 2003
 *  Copyright  2003  mjander
 *  mjander@users.sourceforge.org
 ****************************************************************************/

/* WT channels are grouped in banks. Each bank has 0x20 channels. */
/* Bank register address boundary is 0x8000 */

pub const NR_WT_PB: u32 = 0x20;

/* WT bank base register (as dword address). */
pub const fn WT_BAR(x: u32) -> u32 {
    ((x & 0xffe0) << 0x8)
}

pub const fn WT_BANK(x: u32) -> u32 {
    x >> 5
}

/* WT Bank registers */
pub const fn WT_CTRL(bank: u32) -> u32 {
    (((bank & 1) << 0xd) + 0x00) << 2 /* 0x0000 */
}

pub const fn WT_SRAMP(bank: u32) -> u32 {
    (((bank & 1) << 0xd) + 0x01) << 2 /* 0x0004 */
}

pub const fn WT_DSREG(bank: u32) -> u32 {
    (((bank & 1) << 0xd) + 0x02) << 2 /* 0x0008 */
}

pub const fn WT_MRAMP(bank: u32) -> u32 {
    (((bank & 1) << 0xd) + 0x03) << 2 /* 0x000c */
}

pub const fn WT_GMODE(bank: u32) -> u32 {
    (((bank & 1) << 0xd) + 0x04) << 2 /* 0x0010 */
}

pub const fn WT_ARAMP(bank: u32) -> u32 {
    (((bank & 1) << 0xd) + 0x05) << 2 /* 0x0014 */
}

/* WT Voice registers */
pub const fn WT_STEREO(voice: u32) -> u32 {
    (WT_BAR(voice) + 0x20 + ((voice & 0x1f) >> 1)) << 2 /* 0x0080 */
}

pub const fn WT_MUTE(voice: u32) -> u32 {
    (WT_BAR(voice) + 0x40 + (voice & 0x1f)) << 2 /* 0x0100 */
}

pub const fn WT_RUN(voice: u32) -> u32 {
    (WT_BAR(voice) + 0x60 + (voice & 0x1f)) << 2 /* 0x0180 */
}

/* Some kind of parameters. */
/* PARM0, PARM1 : Filter (0xFF000000), SampleRate (0x0000FFFF) */
/* PARM2, PARM3 : Still unknown */
pub const fn WT_PARM(x: u32, y: u32) -> u32 {
    (WT_BAR(x) + 0x80 + ((x & 0x1f) << 2) + y) << 2 /* 0x0200 */
}

pub const fn WT_DELAY(x: u32, y: u32) -> u32 {
    (WT_BAR(x) + 0x100 + ((x & 0x1f) << 2) + y) << 2 /* 0x0400 */
}

/* Numeric indexes used by SetReg() and GetReg() */
/*
 * Original C had this enum disabled with #if 0:
 *
 * enum {
 *     run = 0,     // 0  W 1:run 0:stop
 *     parm0,       // 1  W filter, samplerate
 *     parm1,       // 2  W filter, samplerate
 *     parm2,       // 3  W
 *     parm3,       // 4  RW volume. This value is calculated using floating point ops.
 *     sramp,       // 5  W
 *     mute,        // 6  W 1:mute, 0:unmute
 *     gmode,       // 7  RO Looks like only bit0 is used.
 *     aramp,       // 8  W
 *     mramp,       // 9  W
 *     ctrl,        // a  W
 *     delay,       // b  W All 4 values are written at once with same value.
 *     dsreg,       // c  (R)W
 * } wt_reg;
 */

#[repr(C)]
pub struct wt_voice_t {
    pub parm0: u32, /* this_1E4 */
    pub parm1: u32, /* this_1E8 */
    pub parm2: u32, /* this_1EC */
    pub parm3: u32, /* this_1F0 */
    pub this_1D0: u32,
}

/* End of file */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
