/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* lirc.h - linux infrared remote control header file */

pub const PULSE_BIT: u32 = 0x01000000;
pub const PULSE_MASK: u32 = 0x00FFFFFF;

pub const LIRC_MODE2_SPACE: u32 = 0x00000000;
pub const LIRC_MODE2_PULSE: u32 = 0x01000000;
pub const LIRC_MODE2_FREQUENCY: u32 = 0x02000000;
pub const LIRC_MODE2_TIMEOUT: u32 = 0x03000000;
pub const LIRC_MODE2_OVERFLOW: u32 = 0x04000000;
pub const LIRC_VALUE_MASK: u32 = 0x00FFFFFF;
pub const LIRC_MODE2_MASK: u32 = 0xFF000000;

#[inline] pub const fn LIRC_SPACE(val: u32) -> u32 { (val & LIRC_VALUE_MASK) | LIRC_MODE2_SPACE }
#[inline] pub const fn LIRC_PULSE(val: u32) -> u32 { (val & LIRC_VALUE_MASK) | LIRC_MODE2_PULSE }
#[inline] pub const fn LIRC_FREQUENCY(val: u32) -> u32 { (val & LIRC_VALUE_MASK) | LIRC_MODE2_FREQUENCY }
#[inline] pub const fn LIRC_TIMEOUT(val: u32) -> u32 { (val & LIRC_VALUE_MASK) | LIRC_MODE2_TIMEOUT }
#[inline] pub const fn LIRC_OVERFLOW(val: u32) -> u32 { (val & LIRC_VALUE_MASK) | LIRC_MODE2_OVERFLOW }
#[inline] pub const fn LIRC_VALUE(val: u32) -> u32 { val & LIRC_VALUE_MASK }
#[inline] pub const fn LIRC_MODE2(val: u32) -> u32 { val & LIRC_MODE2_MASK }
#[inline] pub const fn LIRC_IS_SPACE(val: u32) -> bool { LIRC_MODE2(val) == LIRC_MODE2_SPACE }
#[inline] pub const fn LIRC_IS_PULSE(val: u32) -> bool { LIRC_MODE2(val) == LIRC_MODE2_PULSE }
#[inline] pub const fn LIRC_IS_FREQUENCY(val: u32) -> bool { LIRC_MODE2(val) == LIRC_MODE2_FREQUENCY }
#[inline] pub const fn LIRC_IS_TIMEOUT(val: u32) -> bool { LIRC_MODE2(val) == LIRC_MODE2_TIMEOUT }
#[inline] pub const fn LIRC_IS_OVERFLOW(val: u32) -> bool { LIRC_MODE2(val) == LIRC_MODE2_OVERFLOW }

pub type lirc_t = i32;

#[inline] pub const fn LIRC_MODE2SEND(x: u32) -> u32 { x }
#[inline] pub const fn LIRC_SEND2MODE(x: u32) -> u32 { x }
#[inline] pub const fn LIRC_MODE2REC(x: u32) -> u32 { x << 16 }
#[inline] pub const fn LIRC_REC2MODE(x: u32) -> u32 { x >> 16 }

pub const LIRC_MODE_RAW: u32 = 0x00000001;
pub const LIRC_MODE_PULSE: u32 = 0x00000002;
pub const LIRC_MODE_MODE2: u32 = 0x00000004;
pub const LIRC_MODE_SCANCODE: u32 = 0x00000008;
pub const LIRC_MODE_LIRCCODE: u32 = 0x00000010;
pub const LIRC_CAN_SEND_RAW: u32 = LIRC_MODE_RAW;
pub const LIRC_CAN_SEND_PULSE: u32 = LIRC_MODE_PULSE;
pub const LIRC_CAN_SEND_MODE2: u32 = LIRC_MODE_MODE2;
pub const LIRC_CAN_SEND_LIRCCODE: u32 = LIRC_MODE_LIRCCODE;
pub const LIRC_CAN_SEND_MASK: u32 = 0x0000003f;
pub const LIRC_CAN_SET_SEND_CARRIER: u32 = 0x00000100;
pub const LIRC_CAN_SET_SEND_DUTY_CYCLE: u32 = 0x00000200;
pub const LIRC_CAN_SET_TRANSMITTER_MASK: u32 = 0x00000400;
pub const LIRC_CAN_REC_RAW: u32 = LIRC_MODE_RAW << 16;
pub const LIRC_CAN_REC_PULSE: u32 = LIRC_MODE_PULSE << 16;
pub const LIRC_CAN_REC_MODE2: u32 = LIRC_MODE_MODE2 << 16;
pub const LIRC_CAN_REC_SCANCODE: u32 = LIRC_MODE_SCANCODE << 16;
pub const LIRC_CAN_REC_LIRCCODE: u32 = LIRC_MODE_LIRCCODE << 16;
pub const LIRC_CAN_REC_MASK: u32 = LIRC_CAN_SEND_MASK << 16;
pub const LIRC_CAN_SET_REC_CARRIER: u32 = LIRC_CAN_SET_SEND_CARRIER << 16;
pub const LIRC_CAN_SET_REC_CARRIER_RANGE: u32 = 0x80000000;
pub const LIRC_CAN_GET_REC_RESOLUTION: u32 = 0x20000000;
pub const LIRC_CAN_SET_REC_TIMEOUT: u32 = 0x10000000;
pub const LIRC_CAN_MEASURE_CARRIER: u32 = 0x02000000;
pub const LIRC_CAN_USE_WIDEBAND_RECEIVER: u32 = 0x04000000;
#[inline] pub const fn LIRC_CAN_SEND(x: u32) -> u32 { x & LIRC_CAN_SEND_MASK }
#[inline] pub const fn LIRC_CAN_REC(x: u32) -> u32 { x & LIRC_CAN_REC_MASK }
pub const LIRC_CAN_SET_REC_FILTER: u32 = 0;
pub const LIRC_CAN_NOTIFY_DECODE: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lirc_scancode { pub timestamp: u64, pub flags: u16, pub rc_proto: u16, pub keycode: u32, pub scancode: u64 }
pub const LIRC_SCANCODE_FLAG_TOGGLE: u16 = 1;
pub const LIRC_SCANCODE_FLAG_REPEAT: u16 = 2;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rc_proto {
    RC_PROTO_UNKNOWN = 0, RC_PROTO_OTHER = 1, RC_PROTO_RC5 = 2, RC_PROTO_RC5X_20 = 3,
    RC_PROTO_RC5_SZ = 4, RC_PROTO_JVC = 5, RC_PROTO_SONY12 = 6, RC_PROTO_SONY15 = 7,
    RC_PROTO_SONY20 = 8, RC_PROTO_NEC = 9, RC_PROTO_NECX = 10, RC_PROTO_NEC32 = 11,
    RC_PROTO_SANYO = 12, RC_PROTO_MCIR2_KBD = 13, RC_PROTO_MCIR2_MSE = 14,
    RC_PROTO_RC6_0 = 15, RC_PROTO_RC6_6A_20 = 16, RC_PROTO_RC6_6A_24 = 17,
    RC_PROTO_RC6_6A_32 = 18, RC_PROTO_RC6_MCE = 19, RC_PROTO_SHARP = 20,
    RC_PROTO_XMP = 21, RC_PROTO_CEC = 22, RC_PROTO_IMON = 23, RC_PROTO_RCMM12 = 24,
    RC_PROTO_RCMM24 = 25, RC_PROTO_RCMM32 = 26, RC_PROTO_XBOX_DVD = 27,
    RC_PROTO_MAX = 27,
}

/* ioctl encodings use the platform's Linux _IOR/_IOW definitions. */
// The ioctl constants below are represented by their Linux command numbers.
pub const LIRC_GET_FEATURES: u32 = 0x80046900;
pub const LIRC_GET_SEND_MODE: u32 = 0x80046901;
pub const LIRC_GET_REC_MODE: u32 = 0x80046902;
pub const LIRC_GET_REC_RESOLUTION: u32 = 0x80046907;
pub const LIRC_GET_MIN_TIMEOUT: u32 = 0x80046908;
pub const LIRC_GET_MAX_TIMEOUT: u32 = 0x80046909;
pub const LIRC_GET_LENGTH: u32 = 0x8004690f;
pub const LIRC_SET_SEND_MODE: u32 = 0x40046911;
pub const LIRC_SET_REC_MODE: u32 = 0x40046912;
pub const LIRC_SET_SEND_CARRIER: u32 = 0x40046913;
pub const LIRC_SET_REC_CARRIER: u32 = 0x40046914;
pub const LIRC_SET_SEND_DUTY_CYCLE: u32 = 0x40046915;
pub const LIRC_SET_TRANSMITTER_MASK: u32 = 0x40046917;
pub const LIRC_SET_REC_TIMEOUT: u32 = 0x40046918;
pub const LIRC_SET_REC_TIMEOUT_REPORTS: u32 = 0x40046919;
pub const LIRC_SET_MEASURE_CARRIER_MODE: u32 = 0x4004691d;
pub const LIRC_SET_REC_CARRIER_RANGE: u32 = 0x4004691f;
pub const LIRC_SET_WIDEBAND_RECEIVER: u32 = 0x40046923;
pub const LIRC_GET_REC_TIMEOUT: u32 = 0x80046924;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
