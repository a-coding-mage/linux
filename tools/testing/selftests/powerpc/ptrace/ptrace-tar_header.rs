/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

use core::ffi::{c_char, c_int, c_ulong};

pub const TAR_1: c_ulong = 10;
pub const TAR_2: c_ulong = 20;
pub const TAR_3: c_ulong = 30;
pub const TAR_4: c_ulong = 40;
pub const TAR_5: c_ulong = 50;

pub const DSCR_1: c_ulong = 100;
pub const DSCR_2: c_ulong = 200;
pub const DSCR_3: c_ulong = 300;
pub const DSCR_4: c_ulong = 400;
pub const DSCR_5: c_ulong = 500;

pub const PPR_1: c_ulong = 0x4000000000000; /* or 31,31,31*/
pub const PPR_2: c_ulong = 0x8000000000000; /* or 1,1,1 */
pub const PPR_3: c_ulong = 0xc000000000000; /* or 6,6,6 */
pub const PPR_4: c_ulong = 0x10000000000000; /* or 2,2,2 */

#[no_mangle]
pub static mut user_read: *mut c_char = b"[User Read (Running)]\0".as_ptr() as *mut c_char;
#[no_mangle]
pub static mut user_write: *mut c_char = b"[User Write (Running)]\0".as_ptr() as *mut c_char;
#[no_mangle]
pub static mut ptrace_read_running: *mut c_char =
    b"[Ptrace Read (Running)]\0".as_ptr() as *mut c_char;
#[no_mangle]
pub static mut ptrace_write_running: *mut c_char =
    b"[Ptrace Write (Running)]\0".as_ptr() as *mut c_char;
#[no_mangle]
pub static mut ptrace_read_ckpt: *mut c_char =
    b"[Ptrace Read (Checkpointed)]\0".as_ptr() as *mut c_char;
#[no_mangle]
pub static mut ptrace_write_ckpt: *mut c_char =
    b"[Ptrace Write (Checkpointed)]\0".as_ptr() as *mut c_char;

unsafe extern "C" {
    static TEST_FAIL: c_int;
    static TEST_PASS: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn validate_tar_registers(
    reg: *mut c_ulong,
    tar: c_ulong,
    ppr: c_ulong,
    dscr: c_ulong,
) -> c_int {
    let mut match_: c_int = 1;

    if unsafe { *reg.add(0) } != tar {
        match_ = 0;
    }

    if unsafe { *reg.add(1) } != ppr {
        match_ = 0;
    }

    if unsafe { *reg.add(2) } != dscr {
        match_ = 0;
    }

    if match_ == 0 {
        return unsafe { TEST_FAIL };
    }
    unsafe { TEST_PASS }
}
