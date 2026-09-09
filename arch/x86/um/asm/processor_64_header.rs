/*
 * Copyright 2003 PathScale, Inc.
 *
 * Licensed under the GPL
 */

// C header guard: __UM_PROCESSOR_X86_64_H

#[repr(C)]
pub struct arch_thread {
    pub debugregs: [::std::os::raw::c_ulong; 8],
    pub debugregs_seq: ::std::os::raw::c_int,
    pub faultinfo: faultinfo,
}

// INIT_ARCH_THREAD { .debugregs = { [ 0 ... 7 ] = 0 },
//                    .debugregs_seq = 0, .faultinfo = { 0, 0, 0 } }
#[macro_export]
macro_rules! INIT_ARCH_THREAD {
    () => {
        arch_thread {
            debugregs: [0; 8],
            debugregs_seq: 0,
            faultinfo: faultinfo { 0, 0, 0 },
        }
    };
}

pub const STACKSLOTS_PER_LINE: usize = 4;

#[inline]
pub unsafe fn arch_flush_thread(_thread: *mut arch_thread) {
}

#[inline]
pub unsafe fn arch_copy_thread(_from: *const arch_thread, _to: *mut arch_thread) {
}

#[inline]
pub unsafe fn current_sp() -> *mut ::std::ffi::c_void {
    let mut sp: *mut ::std::ffi::c_void;
    ::std::arch::asm!("movq %rsp, {0}", out(reg) sp);
    sp
}

#[inline]
pub unsafe fn current_bp() -> ::std::os::raw::c_ulong {
    let mut bp: ::std::os::raw::c_ulong;
    ::std::arch::asm!("movq %rbp, {0}", out(reg) bp);
    bp
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
