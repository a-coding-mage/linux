// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

use core::ffi::c_int;

// Declarations supplied by ksmbd_ida.h and ../glob.h.
#[repr(C)]
pub struct ida {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn ida_alloc_range(ida: *mut ida, min: u32, max: u32, gfp: usize) -> c_int;
    fn ida_alloc_min(ida: *mut ida, min: u32, gfp: usize) -> c_int;
    fn ida_alloc(ida: *mut ida, gfp: usize) -> c_int;
    fn ida_free(ida: *mut ida, id: c_int);
}

unsafe extern "C" {
    static KSMBD_DEFAULT_GFP: usize;
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_acquire_smb2_tid(ida: *mut ida) -> c_int {
    ida_alloc_range(ida, 1, 0xFFFFFFFE, KSMBD_DEFAULT_GFP)
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_acquire_smb2_uid(ida: *mut ida) -> c_int {
    let mut id: c_int;

    id = ida_alloc_min(ida, 1, KSMBD_DEFAULT_GFP);
    if id == 0xFFFE {
        id = ida_alloc_min(ida, 1, KSMBD_DEFAULT_GFP);
    }

    id
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_acquire_async_msg_id(ida: *mut ida) -> c_int {
    ida_alloc_min(ida, 1, KSMBD_DEFAULT_GFP)
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_acquire_id(ida: *mut ida) -> c_int {
    ida_alloc(ida, KSMBD_DEFAULT_GFP)
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_release_id(ida: *mut ida, id: c_int) {
    ida_free(ida, id);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
