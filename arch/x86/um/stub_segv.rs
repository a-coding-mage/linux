/*
 * Copyright (C) 2004 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Licensed under the GPL
 */

// The declarations below are supplied by the corresponding sysdep headers.

#[repr(C)]
pub struct faultinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mcontext_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_mcontext: mcontext_t,
}

extern "C" {
    fn get_stub_data() -> *mut faultinfo;
    fn trap_myself();
}

// GET_FAULTINFO_FROM_MC is a sysdep-provided macro that copies the fault
// information from the machine context into the supplied faultinfo object.
macro_rules! GET_FAULTINFO_FROM_MC {
    ($f:expr, $mc:expr) => {
        unsafe {
            $crate::GET_FAULTINFO_FROM_MC($f, $mc);
        }
    };
}

#[link_section = ".__syscall_stub"]
pub unsafe extern "C" fn stub_segv_handler(
    sig: ::std::os::raw::c_int,
    info: *mut siginfo_t,
    p: *mut ::std::ffi::c_void,
) {
    let f: *mut faultinfo = get_stub_data();
    let uc: *mut ucontext_t = p as *mut ucontext_t;

    GET_FAULTINFO_FROM_MC!(&mut *f, &(*uc).uc_mcontext);
    trap_myself();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
