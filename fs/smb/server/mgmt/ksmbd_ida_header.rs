/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// The C header includes <linux/slab.h> and <linux/idr.h>; their symbols are
// supplied by the surrounding translation unit.

use core::ffi::c_int;

/*
 * 2.2.1.6.7 TID Generation
 *    The value 0xFFFF MUST NOT be used as a valid TID. All other
 *    possible values for TID, including zero (0x0000), are valid.
 *    The value 0xFFFF is used to specify all TIDs or no TID,
 *    depending upon the context in which it is used.
 */
extern "C" {
    pub fn ksmbd_acquire_smb2_tid(ida: *mut crate::ida) -> c_int;
}

/*
 * 2.2.1.6.8 UID Generation
 *    The value 0xFFFE was declared reserved in the LAN Manager 1.0
 *    documentation, so a value of 0xFFFE SHOULD NOT be used as a
 *    valid UID.<21> All other possible values for a UID, excluding
 *    zero (0x0000), are valid.
 */
extern "C" {
    pub fn ksmbd_acquire_smb2_uid(ida: *mut crate::ida) -> c_int;
    pub fn ksmbd_acquire_async_msg_id(ida: *mut crate::ida) -> c_int;

    pub fn ksmbd_acquire_id(ida: *mut crate::ida) -> c_int;

    pub fn ksmbd_release_id(ida: *mut crate::ida, id: c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
