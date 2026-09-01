// SPDX-License-Identifier: GPL-2.0-only
/*
 * oxfw_command.c - a part of driver for OXFW970/971 based devices
 *
 * Copyright (c) 2014 Takashi Sakamoto
 */

// Translated from C implementation source. Dependencies from "oxfw.h" are
// declared here as external symbols/types and are expected to be supplied by
// the surrounding driver translation.

pub type u8 = core::ffi::c_uchar;

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

pub type avc_general_plug_dir = core::ffi::c_uint;

pub const AVC_GENERAL_PLUG_DIR_IN: avc_general_plug_dir = 0;
pub const CIP_SFC_COUNT: core::ffi::c_uint = 8;

pub const GFP_KERNEL: core::ffi::c_uint = 0;
pub const ENOMEM: core::ffi::c_int = 12;
pub const EIO: core::ffi::c_int = 5;
pub const ENXIO: core::ffi::c_int = 6;
pub const EINVAL: core::ffi::c_int = 22;
pub const EAGAIN: core::ffi::c_int = 11;

#[inline]
const fn BIT(nr: core::ffi::c_uint) -> core::ffi::c_uint {
    1u32 << nr
}

unsafe extern "C" {
    static amdtp_rate_table: [core::ffi::c_uint; CIP_SFC_COUNT as usize];

    fn kmalloc(
        size: usize,
        flags: core::ffi::c_uint,
    ) -> *mut core::ffi::c_void;
    fn kzalloc(
        size: usize,
        flags: core::ffi::c_uint,
    ) -> *mut core::ffi::c_void;
    fn kfree(ptr: *const core::ffi::c_void);
    fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
    fn memmove(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: usize,
    ) -> *mut core::ffi::c_void;
    fn fcp_avc_transaction(
        unit: *mut fw_unit,
        command_frame: *mut u8,
        command_frame_size: core::ffi::c_uint,
        response_frame: *mut u8,
        response_frame_size: core::ffi::c_uint,
        response_match_bytes: core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avc_stream_set_format(
    unit: *mut fw_unit,
    dir: avc_general_plug_dir,
    pid: core::ffi::c_uint,
    format: *mut u8,
    len: core::ffi::c_uint,
) -> core::ffi::c_int {
    let buf: *mut u8;
    let mut err: core::ffi::c_int;

    buf = unsafe { kmalloc((len + 10) as usize, GFP_KERNEL) as *mut u8 };
    if buf.is_null() {
        return -ENOMEM;
    }

    unsafe {
        *buf.add(0) = 0x00; /* CONTROL */
        *buf.add(1) = 0xff; /* UNIT */
        *buf.add(2) = 0xbf; /* EXTENDED STREAM FORMAT INFORMATION */
        *buf.add(3) = 0xc0; /* SINGLE subfunction */
        *buf.add(4) = dir as u8; /* Plug Direction */
        *buf.add(5) = 0x00; /* UNIT */
        *buf.add(6) = 0x00; /* PCR (Isochronous Plug) */
        *buf.add(7) = (0xff & pid) as u8; /* Plug ID */
        *buf.add(8) = 0xff; /* Padding */
        *buf.add(9) = 0xff; /* Support status in response */
        memcpy(buf.add(10) as *mut core::ffi::c_void, format as *const core::ffi::c_void, len as usize);
    }

    /* do transaction and check buf[1-8] are the same against command */
    err = unsafe {
        fcp_avc_transaction(
            unit,
            buf,
            len + 10,
            buf,
            len + 10,
            BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) | BIT(6) | BIT(7) | BIT(8),
        )
    };
    if err < 0 {
    } else if err < (len + 10) as core::ffi::c_int {
        err = -EIO;
    } else if unsafe { *buf.add(0) } == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENXIO;
    } else if unsafe { *buf.add(0) } == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    } else {
        err = 0;
    }

    unsafe { kfree(buf as *const core::ffi::c_void) };

    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avc_stream_get_format(
    unit: *mut fw_unit,
    dir: avc_general_plug_dir,
    pid: core::ffi::c_uint,
    buf: *mut u8,
    len: *mut core::ffi::c_uint,
    eid: core::ffi::c_uint,
) -> core::ffi::c_int {
    let subfunc: core::ffi::c_uint;
    let mut err: core::ffi::c_int;

    if eid == 0xff {
        subfunc = 0xc0; /* SINGLE */
    } else {
        subfunc = 0xc1; /* LIST */
    }

    unsafe {
        *buf.add(0) = 0x01; /* STATUS */
        *buf.add(1) = 0xff; /* UNIT */
        *buf.add(2) = 0xbf; /* EXTENDED STREAM FORMAT INFORMATION */
        *buf.add(3) = subfunc as u8; /* SINGLE or LIST */
        *buf.add(4) = dir as u8; /* Plug Direction */
        *buf.add(5) = 0x00; /* Unit */
        *buf.add(6) = 0x00; /* PCR (Isochronous Plug) */
        *buf.add(7) = (0xff & pid) as u8; /* Plug ID */
        *buf.add(8) = 0xff; /* Padding */
        *buf.add(9) = 0xff; /* support status in response */
        *buf.add(10) = (0xff & eid) as u8; /* entry ID for LIST subfunction */
        *buf.add(11) = 0xff; /* padding */
    }

    /* do transaction and check buf[1-7] are the same against command */
    err = unsafe {
        fcp_avc_transaction(
            unit,
            buf,
            12,
            buf,
            *len,
            BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) | BIT(6) | BIT(7),
        )
    };
    if err < 0 {
    } else if err < 12 {
        err = -EIO;
    } else if unsafe { *buf.add(0) } == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENXIO;
    } else if unsafe { *buf.add(0) } == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    } else if unsafe { *buf.add(0) } == 0x0b {
        /* IN TRANSITION */
        err = -EAGAIN;
    } else if (subfunc == 0xc1) && (unsafe { *buf.add(10) } != eid as u8) {
        /* LIST subfunction has entry ID */
        err = -EIO;
    }
    if err < 0 {
        return err;
    }

    /* keep just stream format information */
    if subfunc == 0xc0 {
        unsafe {
            memmove(
                buf as *mut core::ffi::c_void,
                buf.add(10) as *const core::ffi::c_void,
                (err - 10) as usize,
            );
            *len = (err - 10) as core::ffi::c_uint;
        }
    } else {
        unsafe {
            memmove(
                buf as *mut core::ffi::c_void,
                buf.add(11) as *const core::ffi::c_void,
                (err - 11) as usize,
            );
            *len = (err - 11) as core::ffi::c_uint;
        }
    }

    err = 0;
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avc_general_inquiry_sig_fmt(
    unit: *mut fw_unit,
    rate: core::ffi::c_uint,
    dir: avc_general_plug_dir,
    pid: core::ffi::c_ushort,
) -> core::ffi::c_int {
    let mut sfc: core::ffi::c_uint;
    let buf: *mut u8;
    let mut err: core::ffi::c_int;

    sfc = 0;
    while sfc < CIP_SFC_COUNT {
        if unsafe { amdtp_rate_table[sfc as usize] } == rate {
            break;
        }
        sfc += 1;
    }
    if sfc == CIP_SFC_COUNT {
        return -EINVAL;
    }

    buf = unsafe { kzalloc(8, GFP_KERNEL) as *mut u8 };
    if buf.is_null() {
        return -ENOMEM;
    }

    unsafe {
        *buf.add(0) = 0x02; /* SPECIFIC INQUIRY */
        *buf.add(1) = 0xff; /* UNIT */
        if dir == AVC_GENERAL_PLUG_DIR_IN {
            *buf.add(2) = 0x19; /* INPUT PLUG SIGNAL FORMAT */
        } else {
            *buf.add(2) = 0x18; /* OUTPUT PLUG SIGNAL FORMAT */
        }
        *buf.add(3) = (0xff & pid) as u8; /* plug id */
        *buf.add(4) = 0x90; /* EOH_1, Form_1, FMT. AM824 */
        *buf.add(5) = (0x07 & sfc) as u8; /* FDF-hi. AM824, frequency */
        *buf.add(6) = 0xff; /* FDF-mid. AM824, SYT hi (not used) */
        *buf.add(7) = 0xff; /* FDF-low. AM824, SYT lo (not used) */
    }

    /* do transaction and check buf[1-5] are the same against command */
    err = unsafe {
        fcp_avc_transaction(
            unit,
            buf,
            8,
            buf,
            8,
            BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5),
        )
    };
    if err < 0 {
    } else if err < 8 {
        err = -EIO;
    } else if unsafe { *buf.add(0) } == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENXIO;
    }
    if err < 0 {
        unsafe { kfree(buf as *const core::ffi::c_void) };
        return err;
    }

    err = 0;
    unsafe { kfree(buf as *const core::ffi::c_void) };
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
