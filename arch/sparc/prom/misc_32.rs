// SPDX-License-Identifier: GPL-2.0
/*
 * misc.c:  Miscellaneous prom functions that don't belong
 *          anywhere else.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

use core::ffi::{c_char, c_int, c_ulong, c_uchar};

// Dependency declarations supplied by the surrounding kernel translation.
#[repr(C)]
pub union PromFortheval {
    pub v0_eval: Option<unsafe extern "C" fn(usize, *mut c_char)>,
    pub v2_eval: Option<unsafe extern "C" fn(*mut c_char)>,
}

#[repr(C)]
pub struct Romvec {
    pub pv_reboot: Option<unsafe extern "C" fn(*mut c_char)>,
    pub pv_fortheval: PromFortheval,
    pub pv_abort: Option<unsafe extern "C" fn()>,
    pub pv_halt: Option<unsafe extern "C" fn()>,
    pub pv_synchook: *mut Option<unsafe extern "C" fn()>,
    pub pv_romvers: c_int,
}

extern "C" {
    pub fn restore_current();
    pub fn spin_lock_irqsave(lock: *mut c_ulong, flags: *mut c_ulong);
    pub fn spin_unlock_irqrestore(lock: *mut c_ulong, flags: c_ulong);
    pub fn strlen(s: *const c_char) -> usize;
    pub fn prom_getproplen(node: c_int, name: *const c_char) -> c_int;
    pub fn prom_getproperty(node: c_int, name: *const c_char, value: *mut c_char, length: c_int) -> c_int;
    pub fn set_auxio(port: c_int, value: c_int);
    pub static mut romvec: *mut Romvec;
    pub static mut prom_vers: c_int;
    pub static mut prom_root_node: c_int;
    pub static mut prom_rev: c_int;
    pub static mut prom_prev: c_int;
}

pub static mut prom_lock: c_ulong = 0;
pub const PROM_V0: c_int = 0;
pub const AUXIO_LED: c_int = 0;

/* Reset and reboot the machine with the command 'bcommand'. */
pub unsafe extern "C" fn prom_reboot(bcommand: *mut c_char) {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut prom_lock, &mut flags);
    if let Some(reboot) = (*romvec).pv_reboot {
        reboot(bcommand);
    }
    /* Never get here. */
    restore_current();
    spin_unlock_irqrestore(&mut prom_lock, flags);
}

/* Forth evaluate the expression contained in 'fstring'. */
pub unsafe extern "C" fn prom_feval(fstring: *mut c_char) {
    let mut flags: c_ulong = 0;
    if fstring.is_null() || *fstring == 0 {
        return;
    }
    spin_lock_irqsave(&mut prom_lock, &mut flags);
    if prom_vers == PROM_V0 {
        if let Some(eval) = (*romvec).pv_fortheval.v0_eval {
            eval(strlen(fstring), fstring);
        }
    } else if let Some(eval) = (*romvec).pv_fortheval.v2_eval {
        eval(fstring);
    }
    restore_current();
    spin_unlock_irqrestore(&mut prom_lock, flags);
}

/* Drop into the prom, with the chance to continue with the 'go'
 * prom command.
 */
pub unsafe extern "C" fn prom_cmdline() {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut prom_lock, &mut flags);
    if let Some(abort) = (*romvec).pv_abort {
        abort();
    }
    restore_current();
    spin_unlock_irqrestore(&mut prom_lock, flags);
    set_auxio(AUXIO_LED, 0);
}

/* Drop into the prom, but completely terminate the program.
 * No chance of continuing.
 */
pub unsafe extern "C" fn prom_halt() -> ! {
    loop {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut prom_lock, &mut flags);
        if let Some(halt) = (*romvec).pv_halt {
            halt();
        }
        /* Never get here. */
        restore_current();
        spin_unlock_irqrestore(&mut prom_lock, flags);
        // PROM is out to get me -DaveM
    }
}

pub type sfunc_t = unsafe extern "C" fn();

/* Set prom sync handler to call function 'funcp'. */
pub unsafe extern "C" fn prom_setsync(funcp: Option<sfunc_t>) {
    if funcp.is_none() {
        return;
    }
    *(*romvec).pv_synchook = funcp;
}

/* Get the idprom and stuff it into buffer 'idbuf'.  Returns the
 * format type.  'num_bytes' is the number of bytes that your idbuf
 * has space for.  Returns 0xff on error.
 */
pub unsafe extern "C" fn prom_get_idprom(idbuf: *mut c_char, num_bytes: c_int) -> c_uchar {
    let len = prom_getproplen(prom_root_node, b"idprom\0".as_ptr() as *const c_char);
    if len > num_bytes || len == -1 {
        return 0xff;
    }
    if prom_getproperty(prom_root_node, b"idprom\0".as_ptr() as *const c_char, idbuf, num_bytes) == 0 {
        return *idbuf as c_uchar;
    }
    0xff
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
