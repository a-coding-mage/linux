/* SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note */
/*
 * ca.h
 *
 * Copyright (C) 2000 Ralph  Metzler <ralph@convergence.de>
 *                  & Marcus Metzler <marcus@convergence.de>
 *                    for convergence integrated media GmbH
 */

/**
 * struct ca_slot_info - CA slot interface types and info.
 *
 * @num: slot number.
 * @type: slot type.
 * @flags: flags applicable to the slot.
 */
#[repr(C)]
pub struct ca_slot_info {
    pub num: ::core::ffi::c_int,
    pub r#type: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_uint,
}

pub const CA_CI: ::core::ffi::c_int = 1;
pub const CA_CI_LINK: ::core::ffi::c_int = 2;
pub const CA_CI_PHYS: ::core::ffi::c_int = 4;
pub const CA_DESCR: ::core::ffi::c_int = 8;
pub const CA_SC: ::core::ffi::c_int = 128;

pub const CA_CI_MODULE_PRESENT: ::core::ffi::c_uint = 1;
pub const CA_CI_MODULE_READY: ::core::ffi::c_uint = 2;

/**
 * struct ca_descr_info - descrambler types and info.
 */
#[repr(C)]
pub struct ca_descr_info {
    pub num: ::core::ffi::c_uint,
    pub r#type: ::core::ffi::c_uint,
}

pub const CA_ECD: ::core::ffi::c_uint = 1;
pub const CA_NDS: ::core::ffi::c_uint = 2;
pub const CA_DSS: ::core::ffi::c_uint = 4;

/**
 * struct ca_caps - CA slot interface capabilities.
 */
#[repr(C)]
pub struct ca_caps {
    pub slot_num: ::core::ffi::c_uint,
    pub slot_type: ::core::ffi::c_uint,
    pub descr_num: ::core::ffi::c_uint,
    pub descr_type: ::core::ffi::c_uint,
}

/**
 * struct ca_msg - a message to/from a CI-CAM
 */
#[repr(C)]
pub struct ca_msg {
    pub index: ::core::ffi::c_uint,
    pub r#type: ::core::ffi::c_uint,
    pub length: ::core::ffi::c_uint,
    pub msg: [::core::ffi::c_uchar; 256],
}

/**
 * struct ca_descr - CA descrambler control words info
 */
#[repr(C)]
pub struct ca_descr {
    pub index: ::core::ffi::c_uint,
    pub parity: ::core::ffi::c_uint,
    pub cw: [::core::ffi::c_uchar; 8],
}

/* These ioctl values use the platform-provided _IO/_IOR/_IOW definitions. */
pub const CA_RESET: _IO_TYPE = _IO(b'o' as _, 128);
pub const CA_GET_CAP: _IO_TYPE = _IOR(b'o' as _, 129, ca_caps);
pub const CA_GET_SLOT_INFO: _IO_TYPE = _IOR(b'o' as _, 130, ca_slot_info);
pub const CA_GET_DESCR_INFO: _IO_TYPE = _IOR(b'o' as _, 131, ca_descr_info);
pub const CA_GET_MSG: _IO_TYPE = _IOR(b'o' as _, 132, ca_msg);
pub const CA_SEND_MSG: _IO_TYPE = _IOW(b'o' as _, 133, ca_msg);
pub const CA_SET_DESCR: _IO_TYPE = _IOW(b'o' as _, 134, ca_descr);

/* This is needed for legacy userspace support. */
pub type ca_slot_info_t = ca_slot_info;
pub type ca_descr_info_t = ca_descr_info;
pub type ca_caps_t = ca_caps;
pub type ca_msg_t = ca_msg;
pub type ca_descr_t = ca_descr;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
