/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* User level driver support for input subsystem. */
/* C header dependencies: <linux/types.h>, <linux/input.h>. */

pub const UINPUT_VERSION: u32 = 5;
pub const UINPUT_MAX_NAME_SIZE: usize = 80;

#[repr(C)]
pub struct uinput_ff_upload {
    pub request_id: u32,
    pub retval: i32,
    pub effect: ff_effect,
    pub old: ff_effect,
}

#[repr(C)]
pub struct uinput_ff_erase {
    pub request_id: u32,
    pub retval: i32,
    pub effect_id: u32,
}

pub const UINPUT_IOCTL_BASE: u8 = b'U';
pub const UI_DEV_CREATE: _ = _IO!(UINPUT_IOCTL_BASE, 1);
pub const UI_DEV_DESTROY: _ = _IO!(UINPUT_IOCTL_BASE, 2);

#[repr(C)]
pub struct uinput_setup {
    pub id: input_id,
    pub name: [core::ffi::c_char; UINPUT_MAX_NAME_SIZE],
    pub ff_effects_max: u32,
}

/**
 * UI_DEV_SETUP - Set device parameters for setup
 *
 * This ioctl sets parameters for the input device to be created. It supersedes
 * the old "struct uinput_user_dev" method, which wrote this data via write().
 */
pub const UI_DEV_SETUP: _ = _IOW!(UINPUT_IOCTL_BASE, 3, uinput_setup);

#[repr(C)]
pub struct uinput_abs_setup {
    pub code: u16, // axis code
    pub absinfo: input_absinfo,
}

/** UI_ABS_SETUP - Set absolute axis information for the device to setup. */
pub const UI_ABS_SETUP: _ = _IOW!(UINPUT_IOCTL_BASE, 4, uinput_abs_setup);

pub const UI_SET_EVBIT: _ = _IOW!(UINPUT_IOCTL_BASE, 100, core::ffi::c_int);
pub const UI_SET_KEYBIT: _ = _IOW!(UINPUT_IOCTL_BASE, 101, core::ffi::c_int);
pub const UI_SET_RELBIT: _ = _IOW!(UINPUT_IOCTL_BASE, 102, core::ffi::c_int);
pub const UI_SET_ABSBIT: _ = _IOW!(UINPUT_IOCTL_BASE, 103, core::ffi::c_int);
pub const UI_SET_MSCBIT: _ = _IOW!(UINPUT_IOCTL_BASE, 104, core::ffi::c_int);
pub const UI_SET_LEDBIT: _ = _IOW!(UINPUT_IOCTL_BASE, 105, core::ffi::c_int);
pub const UI_SET_SNDBIT: _ = _IOW!(UINPUT_IOCTL_BASE, 106, core::ffi::c_int);
pub const UI_SET_FFBIT: _ = _IOW!(UINPUT_IOCTL_BASE, 107, core::ffi::c_int);
pub const UI_SET_PHYS: _ = _IOW!(UINPUT_IOCTL_BASE, 108, *mut core::ffi::c_char);
pub const UI_SET_SWBIT: _ = _IOW!(UINPUT_IOCTL_BASE, 109, core::ffi::c_int);
pub const UI_SET_PROPBIT: _ = _IOW!(UINPUT_IOCTL_BASE, 110, core::ffi::c_int);

pub const UI_BEGIN_FF_UPLOAD: _ = _IOWR!(UINPUT_IOCTL_BASE, 200, uinput_ff_upload);
pub const UI_END_FF_UPLOAD: _ = _IOW!(UINPUT_IOCTL_BASE, 201, uinput_ff_upload);
pub const UI_BEGIN_FF_ERASE: _ = _IOWR!(UINPUT_IOCTL_BASE, 202, uinput_ff_erase);
pub const UI_END_FF_ERASE: _ = _IOW!(UINPUT_IOCTL_BASE, 203, uinput_ff_erase);

/** UI_GET_SYSNAME - get the sysfs name of the created uinput device. */
pub const fn ui_get_sysname(len: u32) -> u32 {
    _IOC!(_IOC_READ, UINPUT_IOCTL_BASE, 44, len)
}

/** UI_GET_VERSION - Return version of uinput protocol. */
pub const UI_GET_VERSION: _ = _IOR!(UINPUT_IOCTL_BASE, 45, u32);

pub const EV_UINPUT: u32 = 0x0101;
pub const UI_FF_UPLOAD: u32 = 1;
pub const UI_FF_ERASE: u32 = 2;

#[repr(C)]
pub struct uinput_user_dev {
    pub name: [core::ffi::c_char; UINPUT_MAX_NAME_SIZE],
    pub id: input_id,
    pub ff_effects_max: u32,
    pub absmax: [i32; ABS_CNT],
    pub absmin: [i32; ABS_CNT],
    pub absfuzz: [i32; ABS_CNT],
    pub absflat: [i32; ABS_CNT],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
