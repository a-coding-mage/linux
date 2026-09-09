/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2004 Microtronix Datacom Ltd.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

pub const TRAP_ID_SYSCALL: i32 = 0;

/* `struct pt_regs` is supplied by the surrounding translation unit. */
extern "C" {
    pub fn _exception(
        signo: i32,
        regs: *mut crate::pt_regs,
        code: i32,
        addr: u32,
    );
    pub fn do_page_fault(
        regs: *mut crate::pt_regs,
        cause: u32,
        address: u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
