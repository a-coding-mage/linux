/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Licensed under the GPL
 */

// Translation of the C header __SYS_SIGCONTEXT_X86_H.
// The declarations below refer to types supplied by the surrounding crate.

extern "C" {
    pub fn get_regs_from_mc(
        regs: *mut crate::uml_pt_regs,
        mc: *mut crate::mcontext_t,
    );
    pub fn get_mc_from_regs(
        regs: *mut crate::uml_pt_regs,
        mc: *mut crate::mcontext_t,
        single_stepping: ::core::ffi::c_int,
    );

    pub fn get_stub_state(
        regs: *mut crate::uml_pt_regs,
        data: *mut crate::stub_data,
        fp_size_out: *mut ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
    pub fn set_stub_state(
        regs: *mut crate::uml_pt_regs,
        data: *mut crate::stub_data,
        single_stepping: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// On i386, mcontext_t exposes cr2 directly.  On other x86 targets, cr2 is
// stored in the gregs array at REG_CR2.  The register constants are supplied
// by the surrounding platform bindings.
#[cfg(target_arch = "x86")]
#[macro_export]
macro_rules! GET_FAULTINFO_FROM_MC {
    ($fi:expr, $mc:expr) => {{
        unsafe {
            ($fi).cr2 = (*$mc).cr2;
            ($fi).error_code = (*$mc).gregs[REG_ERR];
            ($fi).trap_no = (*$mc).gregs[REG_TRAPNO];
        }
    }};
}

#[cfg(not(target_arch = "x86"))]
#[macro_export]
macro_rules! GET_FAULTINFO_FROM_MC {
    ($fi:expr, $mc:expr) => {{
        unsafe {
            ($fi).cr2 = (*$mc).gregs[REG_CR2];
            ($fi).error_code = (*$mc).gregs[REG_ERR];
            ($fi).trap_no = (*$mc).gregs[REG_TRAPNO];
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
