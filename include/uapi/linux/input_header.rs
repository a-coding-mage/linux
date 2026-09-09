/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (c) 1999-2002 Vojtech Pavlik
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 */
// Translated from the Linux UAPI input header.  Required Linux types and ioctl
// encoding macros are supplied by the surrounding UAPI translation.

#[repr(C)]
pub struct input_event {
    #[cfg(all(not(target_pointer_width = "32"), not(feature = "kernel")))]
    pub time: libc::timeval,
    #[cfg(any(target_pointer_width = "32", feature = "kernel"))]
    pub __sec: __kernel_ulong_t,
    #[cfg(any(target_pointer_width = "32", feature = "kernel"))]
    pub __usec: __kernel_ulong_t,
    pub type_: __u16,
    pub code: __u16,
    pub value: __s32,
}

pub const EV_VERSION: u32 = 0x010001;

#[repr(C)]
pub struct input_id { pub bustype: __u16, pub vendor: __u16, pub product: __u16, pub version: __u16 }

#[repr(C)]
pub struct input_absinfo { pub value: __s32, pub minimum: __s32, pub maximum: __s32, pub fuzz: __s32, pub flat: __s32, pub resolution: __s32 }

#[repr(C)]
pub struct input_keymap_entry {
    pub flags: __u8, pub len: __u8, pub index: __u16, pub keycode: __u32,
    pub scancode: [__u8; 32],
}
pub const INPUT_KEYMAP_BY_INDEX: u32 = 1 << 0;

#[repr(C)]
pub struct input_mask { pub type_: __u32, pub codes_size: __u32, pub codes_ptr: __u64 }

pub const EVIOCGVERSION: _ = _IOR!('E', 0x01, i32);
pub const EVIOCGID: _ = _IOR!('E', 0x02, input_id);
pub const EVIOCGREP: _ = _IOR!('E', 0x03, [u32; 2]);
pub const EVIOCSREP: _ = _IOW!('E', 0x03, [u32; 2]);
pub const EVIOCGKEYCODE: _ = _IOR!('E', 0x04, [u32; 2]);
pub const EVIOCGKEYCODE_V2: _ = _IOR!('E', 0x04, input_keymap_entry);
pub const EVIOCSKEYCODE: _ = _IOW!('E', 0x04, [u32; 2]);
pub const EVIOCSKEYCODE_V2: _ = _IOW!('E', 0x04, input_keymap_entry);
#[macro_export] macro_rules! EVIOCGNAME { ($len:expr) => { _IOC!(_IOC_READ, 'E', 0x06, $len) }; }
#[macro_export] macro_rules! EVIOCGPHYS { ($len:expr) => { _IOC!(_IOC_READ, 'E', 0x07, $len) }; }
#[macro_export] macro_rules! EVIOCGUNIQ { ($len:expr) => { _IOC!(_IOC_READ, 'E', 0x08, $len) }; }
#[macro_export] macro_rules! EVIOCGPROP { ($len:expr) => { _IOC!(_IOC_READ, 'E', 0x09, $len) }; }
#[macro_export] macro_rules! EVIOCGMTSLOTS { ($len:expr) => { _IOC!(_IOC_READ, 'E', 0x0a, $len) }; }
#[macro_export] macro_rules! EVIOCGKEY { ($len:expr) => { _IOC!(_IOC_READ, 'E', 0x18, $len) }; }
#[macro_export] macro_rules! EVIOCGLED { ($len:expr) => { _IOC!(_IOC_READ, 'E', 0x19, $len) }; }
#[macro_export] macro_rules! EVIOCGSND { ($len:expr) => { _IOC!(_IOC_READ, 'E', 0x1a, $len) }; }
#[macro_export] macro_rules! EVIOCGSW { ($len:expr) => { _IOC!(_IOC_READ, 'E', 0x1b, $len) }; }
#[macro_export] macro_rules! EVIOCGBIT { ($ev:expr, $len:expr) => { _IOC!(_IOC_READ, 'E', 0x20 + ($ev), $len) }; }
#[macro_export] macro_rules! EVIOCGABS { ($abs:expr) => { _IOR!('E', 0x40 + ($abs), input_absinfo) }; }
#[macro_export] macro_rules! EVIOCSABS { ($abs:expr) => { _IOW!('E', 0xc0 + ($abs), input_absinfo) }; }
pub const EVIOCSFF: _ = _IOW!('E', 0x80, ff_effect);
pub const EVIOCRMFF: _ = _IOW!('E', 0x81, i32);
pub const EVIOCGEFFECTS: _ = _IOR!('E', 0x84, i32);
pub const EVIOCGRAB: _ = _IOW!('E', 0x90, i32);
pub const EVIOCREVOKE: _ = _IOW!('E', 0x91, i32);
pub const EVIOCGMASK: _ = _IOR!('E', 0x92, input_mask);
pub const EVIOCSMASK: _ = _IOW!('E', 0x93, input_mask);
pub const EVIOCSCLOCKID: _ = _IOW!('E', 0xa0, i32);

pub const ID_BUS: u32 = 0; pub const ID_VENDOR: u32 = 1; pub const ID_PRODUCT: u32 = 2; pub const ID_VERSION: u32 = 3;
pub const BUS_PCI: u32 = 0x01; pub const BUS_ISAPNP: u32 = 0x02; pub const BUS_USB: u32 = 0x03; pub const BUS_HIL: u32 = 0x04; pub const BUS_BLUETOOTH: u32 = 0x05; pub const BUS_VIRTUAL: u32 = 0x06;
pub const BUS_ISA: u32 = 0x10; pub const BUS_I8042: u32 = 0x11; pub const BUS_XTKBD: u32 = 0x12; pub const BUS_RS232: u32 = 0x13; pub const BUS_GAMEPORT: u32 = 0x14; pub const BUS_PARPORT: u32 = 0x15; pub const BUS_AMIGA: u32 = 0x16; pub const BUS_ADB: u32 = 0x17; pub const BUS_I2C: u32 = 0x18; pub const BUS_HOST: u32 = 0x19; pub const BUS_GSC: u32 = 0x1a; pub const BUS_ATARI: u32 = 0x1b; pub const BUS_SPI: u32 = 0x1c; pub const BUS_RMI: u32 = 0x1d; pub const BUS_CEC: u32 = 0x1e; pub const BUS_INTEL_ISHTP: u32 = 0x1f; pub const BUS_AMD_SFH: u32 = 0x20; pub const BUS_SDW: u32 = 0x21;

pub const MT_TOOL_FINGER: u32 = 0x00; pub const MT_TOOL_PEN: u32 = 0x01; pub const MT_TOOL_PALM: u32 = 0x02; pub const MT_TOOL_DIAL: u32 = 0x0a; pub const MT_TOOL_MAX: u32 = 0x0f;
pub const FF_STATUS_STOPPED: u32 = 0x00; pub const FF_STATUS_PLAYING: u32 = 0x01; pub const FF_STATUS_MAX: u32 = 0x01;

#[repr(C)] pub struct ff_replay { pub length: __u16, pub delay: __u16 }
#[repr(C)] pub struct ff_trigger { pub button: __u16, pub interval: __u16 }
#[repr(C)] pub struct ff_envelope { pub attack_length: __u16, pub attack_level: __u16, pub fade_length: __u16, pub fade_level: __u16 }
#[repr(C)] pub struct ff_constant_effect { pub level: __s16, pub envelope: ff_envelope }
#[repr(C)] pub struct ff_ramp_effect { pub start_level: __s16, pub end_level: __s16, pub envelope: ff_envelope }
#[repr(C)] pub struct ff_condition_effect { pub right_saturation: __u16, pub left_saturation: __u16, pub right_coeff: __s16, pub left_coeff: __s16, pub deadband: __u16, pub center: __s16 }
#[repr(C)] pub struct ff_periodic_effect { pub waveform: __u16, pub period: __u16, pub magnitude: __s16, pub offset: __s16, pub phase: __u16, pub envelope: ff_envelope, pub custom_len: __u32, pub custom_data: *mut __s16 }
#[repr(C)] pub struct ff_rumble_effect { pub strong_magnitude: __u16, pub weak_magnitude: __u16 }
#[repr(C)] pub struct ff_haptic_effect { pub hid_usage: __u16, pub vendor_id: __u16, pub vendor_waveform_page: __u8, pub intensity: __u16, pub repeat_count: __u16, pub retrigger_period: __u16 }
#[repr(C)] pub union ff_effect_u { pub constant: ff_constant_effect, pub ramp: ff_ramp_effect, pub periodic: ff_periodic_effect, pub condition: [ff_condition_effect; 2], pub rumble: ff_rumble_effect, pub haptic: ff_haptic_effect }
#[repr(C)] pub struct ff_effect { pub type_: __u16, pub id: __s16, pub direction: __u16, pub trigger: ff_trigger, pub replay: ff_replay, pub u: ff_effect_u }

pub const FF_HAPTIC: u32 = 0x4f; pub const FF_RUMBLE: u32 = 0x50; pub const FF_PERIODIC: u32 = 0x51; pub const FF_CONSTANT: u32 = 0x52; pub const FF_SPRING: u32 = 0x53; pub const FF_FRICTION: u32 = 0x54; pub const FF_DAMPER: u32 = 0x55; pub const FF_INERTIA: u32 = 0x56; pub const FF_RAMP: u32 = 0x57;
pub const FF_EFFECT_MIN: u32 = FF_HAPTIC; pub const FF_EFFECT_MAX: u32 = FF_RAMP;
pub const FF_SQUARE: u32 = 0x58; pub const FF_TRIANGLE: u32 = 0x59; pub const FF_SINE: u32 = 0x5a; pub const FF_SAW_UP: u32 = 0x5b; pub const FF_SAW_DOWN: u32 = 0x5c; pub const FF_CUSTOM: u32 = 0x5d;
pub const FF_WAVEFORM_MIN: u32 = FF_SQUARE; pub const FF_WAVEFORM_MAX: u32 = FF_CUSTOM;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
