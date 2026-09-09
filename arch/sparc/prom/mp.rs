// SPDX-License-Identifier: GPL-2.0
/*
 * mp.c:  OpenBoot Prom Multiprocessor support routines.  Don't call
 *        these on a UP or else you will halt and catch fire. ;)
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

use core::ffi::{c_char, c_int, c_ulong};

// Declarations supplied by the surrounding kernel/OpenBoot environment.
#[repr(C)]
pub struct linux_prom_registers {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Romvec {
    pub v3_cpustart: Option<unsafe extern "C" fn(c_int, c_int, c_int, *mut c_char) -> c_int>,
}

unsafe extern "C" {
    pub fn restore_current();
    pub fn spin_lock_irqsave(lock: *mut c_ulong, flags: *mut c_ulong);
    pub fn spin_unlock_irqrestore(lock: *mut c_ulong, flags: c_ulong);

    pub static mut prom_lock: c_ulong;
    pub static mut prom_vers: c_int;
    pub static mut romvec: Romvec;
}

pub const PROM_V0: c_int = 0;
pub const PROM_V2: c_int = 2;
pub const PROM_V3: c_int = 3;

/* Start cpu with prom-tree node 'cpunode' using context described
 * by 'ctable_reg' in context 'ctx' at program counter 'pc'.
 *
 * XXX Have to look into what the return values mean. XXX
 */
pub unsafe fn prom_startcpu(
    cpunode: c_int,
    ctable_reg: *mut linux_prom_registers,
    ctx: c_int,
    pc: *mut c_char,
) -> c_int {
    let ret: c_int;
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&raw mut prom_lock, &raw mut flags);
    match prom_vers {
        PROM_V0 | PROM_V2 => {
            ret = -1;
        }
        PROM_V3 => {
            ret = (romvec.v3_cpustart.unwrap())(cpunode, ctable_reg as c_int, ctx, pc);
        }
        _ => {
            ret = -1;
        }
    }
    restore_current();
    spin_unlock_irqrestore(&raw mut prom_lock, flags);

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
