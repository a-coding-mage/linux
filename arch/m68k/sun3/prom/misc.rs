// SPDX-License-Identifier: GPL-2.0
/*
 * misc.c:  Miscellaneous prom functions that don't belong
 *          anywhere else.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.com)
 */

use core::ffi::{c_char, c_int, c_ulong};

/* Types and symbols supplied by the surrounding Sun-3 PROM support. */
#[repr(C)]
pub struct Sun3Romvec {
    pub pv_reboot: unsafe extern "C" fn(*mut c_char),
    pub pv_halt: unsafe extern "C" fn(),
    pub pv_romvers: c_int,
}

extern "C" {
    static mut romvec: *mut Sun3Romvec;
    static mut prom_rev: c_int;
    static mut prom_prev: c_int;

    fn local_irq_save(flags: *mut c_ulong);
    fn local_irq_restore(flags: c_ulong);
    fn get_sfc(old_sfc: *mut c_int);
    fn set_sfc(sfc: c_int);
    fn get_control_byte(address: c_int, value: *mut c_int);
}

const FC_CONTROL: c_int = 3;
const SUN3_IDPROM_BASE: c_int = 0;

/* Reset and reboot the machine with the command 'bcommand'. */
pub unsafe extern "C" fn prom_reboot(bcommand: *mut c_char) {
    let mut flags: c_ulong = 0;
    local_irq_save(&mut flags);
    ((*romvec).pv_reboot)(bcommand);
    local_irq_restore(flags);
}

/* Drop into the prom, with the chance to continue with the 'go'
 * prom command.
 */
pub unsafe extern "C" fn prom_cmdline() {
}

/* Drop into the prom, but completely terminate the program.
 * No chance of continuing.
 */
pub unsafe extern "C" fn prom_halt() {
    let mut flags: c_ulong = 0;
    loop {
        local_irq_save(&mut flags);
        ((*romvec).pv_halt)();
        local_irq_restore(flags);
        // PROM is out to get me -DaveM
    }
}

pub type SfuncT = unsafe extern "C" fn();

/* Get the idprom and stuff it into buffer 'idbuf'.  Returns the
 * format type.  'num_bytes' is the number of bytes that your idbuf
 * has space for.  Returns 0xff on error.
 */
pub unsafe extern "C" fn prom_get_idprom(idbuf: *mut c_char, num_bytes: c_int) -> u8 {
    let mut oldsfc: c_int = 0;
    get_sfc(&mut oldsfc);
    set_sfc(FC_CONTROL);
    let mut i = 0;
    while i < num_bytes {
        /* There is a problem with the GET_CONTROL_BYTE
         * macro; defining the extra variable
         * gets around it.
         */
        let mut c: c_int = 0;
        get_control_byte(SUN3_IDPROM_BASE + i, &mut c);
        *idbuf.offset(i as isize) = c as c_char;
        i += 1;
    }
    set_sfc(oldsfc);
    *idbuf as u8
}

/* Get the major prom version number. */
pub unsafe extern "C" fn prom_version() -> c_int {
    (*romvec).pv_romvers
}

/* Get the prom plugin-revision. */
pub unsafe extern "C" fn prom_getrev() -> c_int {
    prom_rev
}

/* Get the prom firmware print revision. */
pub unsafe extern "C" fn prom_getprev() -> c_int {
    prom_prev
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
