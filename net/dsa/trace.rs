// SPDX-License-Identifier: GPL-2.0-or-later
/* Copyright 2022-2023 NXP
 */

use core::ffi::{c_char, c_int, c_void};

// Supplied by the corresponding trace declarations and dependent headers.
unsafe extern "C" {
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
}

unsafe extern "C" {
    static DSA_DB_BUFSIZ: usize;
}

pub unsafe fn dsa_db_print(db: *const dsa_db, buf: *mut c_char) {
    match (*db).type_ {
        DSA_DB_PORT => {
            sprintf(
                buf,
                c"port %s".as_ptr(),
                (*(*db).dp).name.as_ptr(),
            );
        }
        DSA_DB_LAG => {
            sprintf(
                buf,
                c"lag %s id %d".as_ptr(),
                (*(*db).lag.dev).name.as_ptr(),
                (*db).lag.id,
            );
        }
        DSA_DB_BRIDGE => {
            sprintf(
                buf,
                c"bridge %s num %d".as_ptr(),
                (*(*db).bridge.dev).name.as_ptr(),
                (*db).bridge.num,
            );
        }
        _ => {
            sprintf(buf, c"unknown".as_ptr());
        }
    }
}

pub unsafe fn dsa_port_kind(dp: *const dsa_port) -> *const c_char {
    match (*dp).type_ {
        DSA_PORT_TYPE_USER => c"user".as_ptr(),
        DSA_PORT_TYPE_CPU => c"cpu".as_ptr(),
        DSA_PORT_TYPE_DSA => c"dsa".as_ptr(),
        _ => c"unused".as_ptr(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
