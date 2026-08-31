// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2017 John Sperbeck
 *
 * Test that an access to a mapped but inaccessible area causes a SEGV and
 * reports si_code == SEGV_ACCERR.
 */

// C dependencies:
// stdbool.h, stdio.h, stdlib.h, string.h, unistd.h, signal.h, sys/mman.h,
// assert.h, ucontext.h, and "utils.h".

use libc::{
    c_char, c_int, c_void, sigaction, sigemptyset, siginfo_t, ucontext_t, MAP_ANONYMOUS,
    MAP_FAILED, MAP_PRIVATE, PROT_NONE, SA_SIGINFO, SEGV_ACCERR, SIGSEGV,
};

#[repr(C)]
pub struct pt_regs {
    pub gpr: [libc::c_ulong; 32],
    pub nip: libc::c_ulong,
    pub msr: libc::c_ulong,
    pub orig_gpr3: libc::c_ulong,
    pub ctr: libc::c_ulong,
    pub link: libc::c_ulong,
    pub xer: libc::c_ulong,
    pub ccr: libc::c_ulong,
    pub softe: libc::c_ulong,
    pub trap: libc::c_ulong,
    pub dar: libc::c_ulong,
    pub dsisr: libc::c_ulong,
    pub result: libc::c_ulong,
}

unsafe extern "C" {
    fn getpagesize() -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: libc::size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: libc::off_t,
    ) -> *mut c_void;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
    fn mb();
}

static mut faulted: bool = false;
static mut si_code: c_int = 0;

unsafe extern "C" fn segv_handler(_n: c_int, info: *mut siginfo_t, ctxt_v: *mut c_void) {
    let ctxt: *mut ucontext_t = ctxt_v as *mut ucontext_t;
    let regs: *mut pt_regs = (*ctxt).uc_mcontext.regs as *mut pt_regs;

    faulted = true;
    si_code = (*info).si_code;
    (*regs).nip = (*regs).nip.wrapping_add(4);
}

#[no_mangle]
pub unsafe extern "C" fn test_segv_errors() -> c_int {
    let mut act: sigaction = core::mem::zeroed();
    act.sa_sigaction = segv_handler as usize;
    act.sa_flags = SA_SIGINFO;
    sigemptyset(&mut act.sa_mask);

    let mut c: c_char;
    let mut p: *mut c_char = core::ptr::null_mut();

    p = mmap(
        core::ptr::null_mut(),
        getpagesize() as libc::size_t,
        PROT_NONE,
        MAP_PRIVATE | MAP_ANONYMOUS,
        -1,
        0,
    ) as *mut c_char;
    if p as *mut c_void == MAP_FAILED {
        return 1;
    }

    if libc::sigaction(SIGSEGV, &act, core::ptr::null_mut()) != 0 {
        return 1;
    }

    faulted = false;
    si_code = 0;

    /*
     * We just need a compiler barrier, but mb() works and has the nice
     * property of being easy to spot in the disassembly.
     */
    mb();
    c = core::ptr::read_volatile(p);
    mb();

    if !faulted {
        return 1;
    }
    if si_code != SEGV_ACCERR {
        return 1;
    }

    faulted = false;
    si_code = 0;

    mb();
    core::ptr::write_volatile(p, c);
    mb();

    if !faulted {
        return 1;
    }
    if si_code != SEGV_ACCERR {
        return 1;
    }

    0
}

fn main() {
    let name = b"segv_errors\0";
    unsafe {
        std::process::exit(test_harness(test_segv_errors, name.as_ptr() as *const c_char));
    }
}
