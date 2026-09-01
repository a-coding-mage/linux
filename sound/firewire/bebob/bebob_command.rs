// SPDX-License-Identifier: GPL-2.0-only
/*
 * bebob_command.c - driver for BeBoB based devices
 *
 * Copyright (c) 2013-2014 Takashi Sakamoto
 */

// Rust translation of dependencies provided by "./bebob.h" and the kernel.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_int, c_uint, c_void};

pub type u8 = core::ffi::c_uchar;

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

pub type avc_bridgeco_plug_type = c_uint;

pub const AVC_BRIDGECO_ADDR_BYTES: usize = 6;

pub const ENOMEM: c_int = 12;
pub const EIO: c_int = 5;
pub const ENOSYS: c_int = 38;
pub const EINVAL: c_int = 22;
pub const EAGAIN: c_int = 11;

unsafe extern "C" {
    static GFP_KERNEL: c_uint;

    fn kzalloc(size: usize, flags: c_uint) -> *mut u8;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn fcp_avc_transaction(
        unit: *mut fw_unit,
        command: *mut u8,
        command_size: c_uint,
        response: *mut u8,
        response_size: c_uint,
        response_match_bytes: c_uint,
    ) -> c_int;
}

#[inline]
const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avc_audio_set_selector(
    unit: *mut fw_unit,
    subunit_id: c_uint,
    fb_id: c_uint,
    num: c_uint,
) -> c_int {
    let buf: *mut u8;
    let mut err: c_int;

    buf = unsafe { kzalloc(12, GFP_KERNEL) };
    if buf.is_null() {
        return -ENOMEM;
    }

    unsafe {
        *buf.add(0) = 0x00; /* AV/C CONTROL */
        *buf.add(1) = 0x08 | (0x07 & subunit_id) as u8; /* AUDIO SUBUNIT ID */
        *buf.add(2) = 0xb8; /* FUNCTION BLOCK  */
        *buf.add(3) = 0x80; /* type is 'selector'*/
        *buf.add(4) = (0xff & fb_id) as u8; /* function block id */
        *buf.add(5) = 0x10; /* control attribute is CURRENT */
        *buf.add(6) = 0x02; /* selector length is 2 */
        *buf.add(7) = (0xff & num) as u8; /* input function block plug number */
        *buf.add(8) = 0x01; /* control selector is SELECTOR_CONTROL */
    }

    err = unsafe {
        fcp_avc_transaction(
            unit,
            buf,
            12,
            buf,
            12,
            BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) | BIT(6) | BIT(7) | BIT(8),
        )
    };
    if err < 0 {
    } else if err < 9 {
        err = -EIO;
    } else if unsafe { *buf.add(0) } == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENOSYS;
    } else if unsafe { *buf.add(0) } == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    } else {
        err = 0;
    }

    unsafe { kfree(buf as *mut c_void) };
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avc_audio_get_selector(
    unit: *mut fw_unit,
    subunit_id: c_uint,
    fb_id: c_uint,
    num: *mut c_uint,
) -> c_int {
    let buf: *mut u8;
    let mut err: c_int;

    buf = unsafe { kzalloc(12, GFP_KERNEL) };
    if buf.is_null() {
        return -ENOMEM;
    }

    unsafe {
        *buf.add(0) = 0x01; /* AV/C STATUS */
        *buf.add(1) = 0x08 | (0x07 & subunit_id) as u8; /* AUDIO SUBUNIT ID */
        *buf.add(2) = 0xb8; /* FUNCTION BLOCK */
        *buf.add(3) = 0x80; /* type is 'selector'*/
        *buf.add(4) = (0xff & fb_id) as u8; /* function block id */
        *buf.add(5) = 0x10; /* control attribute is CURRENT */
        *buf.add(6) = 0x02; /* selector length is 2 */
        *buf.add(7) = 0xff; /* input function block plug number */
        *buf.add(8) = 0x01; /* control selector is SELECTOR_CONTROL */
    }

    err = unsafe {
        fcp_avc_transaction(
            unit,
            buf,
            12,
            buf,
            12,
            BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) | BIT(6) | BIT(8),
        )
    };
    if err < 0 {
    } else if err < 9 {
        err = -EIO;
    } else if unsafe { *buf.add(0) } == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENOSYS;
    } else if unsafe { *buf.add(0) } == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    } else if unsafe { *buf.add(0) } == 0x0b {
        /* IN TRANSITION */
        err = -EAGAIN;
    }
    if err < 0 {
        unsafe { kfree(buf as *mut c_void) };
        return err;
    }

    unsafe {
        *num = *buf.add(7) as c_uint;
    }
    err = 0;
    unsafe { kfree(buf as *mut c_void) };
    err
}

#[inline]
unsafe fn avc_bridgeco_fill_extension_addr(buf: *mut u8, addr: *mut u8) {
    unsafe {
        *buf.add(1) = *addr.add(0);
        memcpy(buf.add(4) as *mut c_void, addr.add(1) as *const c_void, 5);
    }
}

#[inline]
unsafe fn avc_bridgeco_fill_plug_info_extension_command(
    buf: *mut u8,
    addr: *mut u8,
    itype: c_uint,
) {
    unsafe {
        *buf.add(0) = 0x01; /* AV/C STATUS */
        *buf.add(2) = 0x02; /* AV/C GENERAL PLUG INFO */
        *buf.add(3) = 0xc0; /* BridgeCo extension */
        avc_bridgeco_fill_extension_addr(buf, addr);
        *buf.add(9) = itype as u8; /* info type */
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avc_bridgeco_get_plug_type(
    unit: *mut fw_unit,
    addr: *mut u8,
    type_: *mut avc_bridgeco_plug_type,
) -> c_int {
    let buf: *mut u8;
    let mut err: c_int;

    buf = unsafe { kzalloc(12, GFP_KERNEL) };
    if buf.is_null() {
        return -ENOMEM;
    }

    /* Info type is 'plug type'. */
    unsafe { avc_bridgeco_fill_plug_info_extension_command(buf, addr, 0x00) };

    err = unsafe {
        fcp_avc_transaction(
            unit,
            buf,
            12,
            buf,
            12,
            BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) | BIT(6) | BIT(7) | BIT(9),
        )
    };
    if err < 0 {
    } else if err < 11 {
        err = -EIO;
    } else if unsafe { *buf.add(0) } == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENOSYS;
    } else if unsafe { *buf.add(0) } == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    } else if unsafe { *buf.add(0) } == 0x0b {
        /* IN TRANSITION */
        err = -EAGAIN;
    }
    if err < 0 {
        unsafe { kfree(buf as *mut c_void) };
        return err;
    }

    unsafe {
        *type_ = *buf.add(10) as avc_bridgeco_plug_type;
    }
    err = 0;
    unsafe { kfree(buf as *mut c_void) };
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avc_bridgeco_get_plug_ch_count(
    unit: *mut fw_unit,
    addr: *mut u8,
    ch_count: *mut c_uint,
) -> c_int {
    let buf: *mut u8;
    let mut err: c_int;

    buf = unsafe { kzalloc(12, GFP_KERNEL) };
    if buf.is_null() {
        return -ENOMEM;
    }

    // Info type is 'plug type'.
    unsafe { avc_bridgeco_fill_plug_info_extension_command(buf, addr, 0x02) };

    err = unsafe {
        fcp_avc_transaction(
            unit,
            buf,
            12,
            buf,
            12,
            BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) | BIT(6) | BIT(7) | BIT(9),
        )
    };
    if err < 0 {
    } else if err < 11 {
        err = -EIO;
    } else if unsafe { *buf.add(0) } == 0x08 {
        // NOT IMPLEMENTED
        err = -ENOSYS;
    } else if unsafe { *buf.add(0) } == 0x0a {
        // REJECTED
        err = -EINVAL;
    } else if unsafe { *buf.add(0) } == 0x0b {
        // IN TRANSITION
        err = -EAGAIN;
    }
    if err < 0 {
        unsafe { kfree(buf as *mut c_void) };
        return err;
    }

    unsafe {
        *ch_count = *buf.add(10) as c_uint;
    }
    err = 0;
    unsafe { kfree(buf as *mut c_void) };
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avc_bridgeco_get_plug_ch_pos(
    unit: *mut fw_unit,
    addr: *mut u8,
    buf: *mut u8,
    len: c_uint,
) -> c_int {
    let mut err: c_int;

    /* Info type is 'channel position'. */
    unsafe { avc_bridgeco_fill_plug_info_extension_command(buf, addr, 0x03) };

    err = unsafe {
        fcp_avc_transaction(
            unit,
            buf,
            12,
            buf,
            256,
            BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) | BIT(6) | BIT(7) | BIT(9),
        )
    };
    if err < 0 {
    } else if err < 11 {
        err = -EIO;
    } else if unsafe { *buf.add(0) } == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENOSYS;
    } else if unsafe { *buf.add(0) } == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    } else if unsafe { *buf.add(0) } == 0x0b {
        /* IN TRANSITION */
        err = -EAGAIN;
    }
    if err < 0 {
        return err;
    }

    /* Pick up specific data. */
    unsafe {
        memmove(
            buf as *mut c_void,
            buf.add(10) as *const c_void,
            (err - 10) as usize,
        );
    }
    err = 0;
    let _ = len;
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avc_bridgeco_get_plug_section_type(
    unit: *mut fw_unit,
    addr: *mut u8,
    mut id: c_uint,
    type_: *mut u8,
) -> c_int {
    let buf: *mut u8;
    let mut err: c_int;

    /* section info includes charactors but this module don't need it */
    buf = unsafe { kzalloc(12, GFP_KERNEL) };
    if buf.is_null() {
        return -ENOMEM;
    }

    /* Info type is 'section info'. */
    unsafe {
        avc_bridgeco_fill_plug_info_extension_command(buf, addr, 0x07);
        id = id.wrapping_add(1);
        *buf.add(10) = (0xff & id) as u8; /* section id */
    }

    err = unsafe {
        fcp_avc_transaction(
            unit,
            buf,
            12,
            buf,
            12,
            BIT(1)
                | BIT(2)
                | BIT(3)
                | BIT(4)
                | BIT(5)
                | BIT(6)
                | BIT(7)
                | BIT(9)
                | BIT(10),
        )
    };
    if err < 0 {
    } else if err < 12 {
        err = -EIO;
    } else if unsafe { *buf.add(0) } == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENOSYS;
    } else if unsafe { *buf.add(0) } == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    } else if unsafe { *buf.add(0) } == 0x0b {
        /* IN TRANSITION */
        err = -EAGAIN;
    }
    if err < 0 {
        unsafe { kfree(buf as *mut c_void) };
        return err;
    }

    unsafe {
        *type_ = *buf.add(11);
    }
    err = 0;
    unsafe { kfree(buf as *mut c_void) };
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avc_bridgeco_get_plug_input(
    unit: *mut fw_unit,
    addr: *mut u8,
    input: *mut u8,
) -> c_int {
    let mut err: c_int;
    let buf: *mut u8;

    buf = unsafe { kzalloc(18, GFP_KERNEL) };
    if buf.is_null() {
        return -ENOMEM;
    }

    /* Info type is 'plug input'. */
    unsafe { avc_bridgeco_fill_plug_info_extension_command(buf, addr, 0x05) };

    err = unsafe {
        fcp_avc_transaction(
            unit,
            buf,
            16,
            buf,
            16,
            BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) | BIT(6) | BIT(7),
        )
    };
    if err < 0 {
    } else if err < 16 {
        err = -EIO;
    } else if unsafe { *buf.add(0) } == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENOSYS;
    } else if unsafe { *buf.add(0) } == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    } else if unsafe { *buf.add(0) } == 0x0b {
        /* IN TRANSITION */
        err = -EAGAIN;
    }
    if err < 0 {
        unsafe { kfree(buf as *mut c_void) };
        return err;
    }

    unsafe {
        memcpy(input as *mut c_void, buf.add(10) as *const c_void, 5);
    }
    err = 0;
    unsafe { kfree(buf as *mut c_void) };
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avc_bridgeco_get_plug_strm_fmt(
    unit: *mut fw_unit,
    addr: *mut u8,
    buf: *mut u8,
    len: *mut c_uint,
    eid: c_uint,
) -> c_int {
    let mut err: c_int;

    /* check given buffer */
    if buf.is_null() || unsafe { *len < 12 } {
        err = -EINVAL;
        return err;
    }

    unsafe {
        *buf.add(0) = 0x01; /* AV/C STATUS */
        *buf.add(2) = 0x2f; /* AV/C STREAM FORMAT SUPPORT */
        *buf.add(3) = 0xc1; /* Bridgeco extension - List Request */
        avc_bridgeco_fill_extension_addr(buf, addr);
        *buf.add(10) = (0xff & eid) as u8; /* Entry ID */
    }

    err = unsafe {
        fcp_avc_transaction(
            unit,
            buf,
            12,
            buf,
            *len,
            BIT(1) | BIT(2) | BIT(3) | BIT(4) | BIT(5) | BIT(6) | BIT(7) | BIT(10),
        )
    };
    if err < 0 {
    } else if err < 12 {
        err = -EIO;
    } else if unsafe { *buf.add(0) } == 0x08 {
        /* NOT IMPLEMENTED */
        err = -ENOSYS;
    } else if unsafe { *buf.add(0) } == 0x0a {
        /* REJECTED */
        err = -EINVAL;
    } else if unsafe { *buf.add(0) } == 0x0b {
        /* IN TRANSITION */
        err = -EAGAIN;
    } else if unsafe { *buf.add(10) } != eid as u8 {
        err = -EIO;
    }
    if err < 0 {
        return err;
    }

    /* Pick up 'stream format info'. */
    unsafe {
        memmove(
            buf as *mut c_void,
            buf.add(11) as *const c_void,
            (err - 11) as usize,
        );
        *len = (err - 11) as c_uint;
    }
    err = 0;
    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
