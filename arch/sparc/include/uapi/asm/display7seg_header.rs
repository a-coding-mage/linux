/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *
 * display7seg - Driver interface for the 7-segment display
 * present on Sun Microsystems CP1400 and CP1500
 *
 * Copyright (c) 2000 Eric Brower <ebrower@usa.net>
 *
 */

// ioctl encoding macros are supplied by the surrounding UAPI environment.
pub const D7S_IOC: u8 = b'p';

pub const D7SIOCRD: _ = _IOR!(D7S_IOC, 0x45, i32); // Read device state
pub const D7SIOCWR: _ = _IOW!(D7S_IOC, 0x46, i32); // Write device state
pub const D7SIOCTM: _ = _IO!(D7S_IOC, 0x47); // Translate mode (FLIP)

/*
 * ioctl flag definitions
 *
 * POINT     - Toggle decimal point (0=absent 1=present)
 * ALARM     - Toggle alarm LED      (0=green  1=red)
 * FLIP      - Toggle inverted mode (0=normal 1=flipped)
 * bits 0-4  - Character displayed   (see definitions below)
 *
 * Display segments are defined as follows,
 * subject to D7S_FLIP register state:
 *
 *    a
 *   ---
 * f|   |b
 *   -g-
 * e|   |c
 *   ---
 *    d
 */

pub const D7S_POINT: u32 = 1u32 << 7; // Decimal point
pub const D7S_ALARM: u32 = 1u32 << 6; // Alarm LED
pub const D7S_FLIP: u32 = 1u32 << 5; // Flip display

pub const D7S_0: u32 = 0x00; // Numerals 0-9
pub const D7S_1: u32 = 0x01;
pub const D7S_2: u32 = 0x02;
pub const D7S_3: u32 = 0x03;
pub const D7S_4: u32 = 0x04;
pub const D7S_5: u32 = 0x05;
pub const D7S_6: u32 = 0x06;
pub const D7S_7: u32 = 0x07;
pub const D7S_8: u32 = 0x08;
pub const D7S_9: u32 = 0x09;
pub const D7S_A: u32 = 0x0A; // Letters A-F, H, L, P
pub const D7S_B: u32 = 0x0B;
pub const D7S_C: u32 = 0x0C;
pub const D7S_D: u32 = 0x0D;
pub const D7S_E: u32 = 0x0E;
pub const D7S_F: u32 = 0x0F;
pub const D7S_H: u32 = 0x10;
pub const D7S_E2: u32 = 0x11;
pub const D7S_L: u32 = 0x12;
pub const D7S_P: u32 = 0x13;
pub const D7S_SEGA: u32 = 0x14; // Individual segments
pub const D7S_SEGB: u32 = 0x15;
pub const D7S_SEGC: u32 = 0x16;
pub const D7S_SEGD: u32 = 0x17;
pub const D7S_SEGE: u32 = 0x18;
pub const D7S_SEGF: u32 = 0x19;
pub const D7S_SEGG: u32 = 0x1A;
pub const D7S_SEGABFG: u32 = 0x1B; // Segment groupings
pub const D7S_SEGCDEG: u32 = 0x1C;
pub const D7S_SEGBCEF: u32 = 0x1D;
pub const D7S_SEGADG: u32 = 0x1E;
pub const D7S_BLANK: u32 = 0x1F; // Clear all segments

pub const D7S_MIN_VAL: u32 = 0x0;
pub const D7S_MAX_VAL: u32 = 0x1F;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
