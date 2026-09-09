/* SPDX-License-Identifier: GPL-2.0 */

// The C header's __KERNEL__ conditional includes linux/types.h and defines
// kernel_ulong_t. Rust's usize is the platform-sized unsigned integer used
// here as the direct equivalent of C's unsigned long.
pub type kernel_ulong_t = usize;

/* Input */
pub const INPUT_DEVICE_ID_EV_MAX: usize = 0x1f;
pub const INPUT_DEVICE_ID_KEY_MIN_INTERESTING: usize = 0x71;
pub const INPUT_DEVICE_ID_KEY_MAX: usize = 0x2ff;
pub const INPUT_DEVICE_ID_REL_MAX: usize = 0x0f;
pub const INPUT_DEVICE_ID_ABS_MAX: usize = 0x3f;
pub const INPUT_DEVICE_ID_MSC_MAX: usize = 0x07;
pub const INPUT_DEVICE_ID_LED_MAX: usize = 0x0f;
pub const INPUT_DEVICE_ID_SND_MAX: usize = 0x07;
pub const INPUT_DEVICE_ID_FF_MAX: usize = 0x7f;
pub const INPUT_DEVICE_ID_SW_MAX: usize = 0x11;
pub const INPUT_DEVICE_ID_PROP_MAX: usize = 0x1f;

pub const INPUT_DEVICE_ID_MATCH_BUS: usize = 1;
pub const INPUT_DEVICE_ID_MATCH_VENDOR: usize = 2;
pub const INPUT_DEVICE_ID_MATCH_PRODUCT: usize = 4;
pub const INPUT_DEVICE_ID_MATCH_VERSION: usize = 8;

pub const INPUT_DEVICE_ID_MATCH_EVBIT: usize = 0x0010;
pub const INPUT_DEVICE_ID_MATCH_KEYBIT: usize = 0x0020;
pub const INPUT_DEVICE_ID_MATCH_RELBIT: usize = 0x0040;
pub const INPUT_DEVICE_ID_MATCH_ABSBIT: usize = 0x0080;
pub const INPUT_DEVICE_ID_MATCH_MSCIT: usize = 0x0100;
pub const INPUT_DEVICE_ID_MATCH_LEDBIT: usize = 0x0200;
pub const INPUT_DEVICE_ID_MATCH_SNDBIT: usize = 0x0400;
pub const INPUT_DEVICE_ID_MATCH_FFBIT: usize = 0x0800;
pub const INPUT_DEVICE_ID_MATCH_SWBIT: usize = 0x1000;
pub const INPUT_DEVICE_ID_MATCH_PROPBIT: usize = 0x2000;

// BITS_PER_LONG is represented by the number of bits in kernel_ulong_t.
#[repr(C)]
pub struct input_device_id {
    pub flags: kernel_ulong_t,

    pub bustype: u16,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,

    pub evbit: [kernel_ulong_t; INPUT_DEVICE_ID_EV_MAX / usize::BITS as usize + 1],
    pub keybit: [kernel_ulong_t; INPUT_DEVICE_ID_KEY_MAX / usize::BITS as usize + 1],
    pub relbit: [kernel_ulong_t; INPUT_DEVICE_ID_REL_MAX / usize::BITS as usize + 1],
    pub absbit: [kernel_ulong_t; INPUT_DEVICE_ID_ABS_MAX / usize::BITS as usize + 1],
    pub mscbit: [kernel_ulong_t; INPUT_DEVICE_ID_MSC_MAX / usize::BITS as usize + 1],
    pub ledbit: [kernel_ulong_t; INPUT_DEVICE_ID_LED_MAX / usize::BITS as usize + 1],
    pub sndbit: [kernel_ulong_t; INPUT_DEVICE_ID_SND_MAX / usize::BITS as usize + 1],
    pub ffbit: [kernel_ulong_t; INPUT_DEVICE_ID_FF_MAX / usize::BITS as usize + 1],
    pub swbit: [kernel_ulong_t; INPUT_DEVICE_ID_SW_MAX / usize::BITS as usize + 1],
    pub propbit: [kernel_ulong_t; INPUT_DEVICE_ID_PROP_MAX / usize::BITS as usize + 1],

    pub driver_info: kernel_ulong_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
