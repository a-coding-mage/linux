// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 1993 Hamish Macdonald
 *  Copyright (C) 1999 D. Jeff Dionne
 *  Copyright (C) 2001 Georges Menie, Ken Desmet
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

use core::ffi::{c_char, c_int};

// Symbols supplied by the platform/kernel sources.
extern "C" {
    fn getserialnum() -> *mut c_char;
    fn gethwaddr(a: c_int) -> *mut u8;
    fn getbenv(a: *mut c_char) -> *mut c_char;
    fn strscpy(dst: *mut c_char, src: *const c_char, size: usize) -> isize;
}

static mut errno: c_int = 0;

// `__init` is a kernel annotation; retain the function's C ABI and behavior.
#[no_mangle]
pub unsafe extern "C" fn init_ucsimm(command: *mut c_char, size: c_int) {
    let mut p: *mut c_char;

    pr_info!(
        "uCsimm/uCdimm serial string [%s]\n",
        getserialnum()
    );
    p = gethwaddr(0);
    pr_info!("uCsimm/uCdimm hwaddr %pM\n", p);
    p = getbenv(b"APPEND\0".as_ptr() as *mut c_char);
    if !p.is_null() {
        strscpy(p, command, size as usize);
    } else {
        *command = 0;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
