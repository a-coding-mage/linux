// SPDX-License-Identifier: GPL-2.0
/***************************************************************************/

/*
 *	vectors.c  -- high level trap setup for ColdFire
 *
 *	Copyright (C) 1999-2007, Greg Ungerer <gerg@snapgear.com>
 */

/***************************************************************************/

// C header dependencies are supplied by the surrounding kernel translation.

#[cfg(TRAP_DBG_INTERRUPT)]
unsafe extern "C" {
    fn dump(fp: *mut pt_regs);
    fn printk(fmt: *const core::ffi::c_char, ...);
}

#[cfg(TRAP_DBG_INTERRUPT)]
#[repr(C)]
pub struct frame {
    _private: [u8; 0],
}

#[cfg(TRAP_DBG_INTERRUPT)]
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

/* Assembler routines */
unsafe extern "C" {
    fn buserr();
    fn trap();
    fn system_call();
    fn inthandler();
    static mut _ramvec: [Option<unsafe extern "C" fn()>; 256];
}

#[cfg(TRAP_DBG_INTERRUPT)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dbginterrupt_c(fp: *mut frame) {
    static FILE: &[u8] = b"vectors.c\0";
    static MESSAGE: &[u8] = b"%s(%d): BUS ERROR TRAP\n\0";
    printk(MESSAGE.as_ptr() as *const core::ffi::c_char, FILE.as_ptr(), 39);
    dump(fp as *mut pt_regs);
    core::arch::asm!("halt");
}

pub unsafe extern "C" fn trap_init() {
    /*
     *	There is a common trap handler and common interrupt
     *	handler that handle almost every vector. We treat
     *	the system call and bus error special, they get their
     *	own first level handlers.
     */
    let mut i: i32;

    i = 3;
    while i <= 23 {
        _ramvec[i as usize] = Some(trap);
        i += 1;
    }
    i = 33;
    while i <= 63 {
        _ramvec[i as usize] = Some(trap);
        i += 1;
    }
    i = 24;
    while i <= 31 {
        _ramvec[i as usize] = Some(inthandler);
        i += 1;
    }
    i = 64;
    while i < 255 {
        _ramvec[i as usize] = Some(inthandler);
        i += 1;
    }
    _ramvec[255] = None;

    _ramvec[2] = Some(buserr);
    _ramvec[32] = Some(system_call);

    #[cfg(TRAP_DBG_INTERRUPT)]
    {
        _ramvec[12] = Some(core::mem::transmute(dbginterrupt_c as unsafe extern "C" fn(*mut frame)));
    }
}

/***************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
