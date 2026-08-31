/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2019 ARM Limited */

/*
 * C dependencies: <signal.h>, <stdbool.h>, <ucontext.h>,
 * and architecture-specific <asm/ptrace.h> and <asm/hwcap.h>.
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_void};

/*
 * Direct Rust equivalent for the get_regval(regname, out) macro.  The register
 * name must be an assembler sysreg token accepted by the target assembler.
 */
macro_rules! get_regval {
    ($regname:tt, $out:expr) => {{
        unsafe {
            asm!(
                concat!("mrs {}, ", stringify!($regname)),
                out(reg) $out,
                options(nostack, preserves_flags)
            );
        }
    }};
}

/*
 * Feature flags used in tdescr.feats_required to specify
 * any feature by the test
 */
pub const FSSBS_BIT: c_int = 0;
pub const FSVE_BIT: c_int = 1;
pub const FSME_BIT: c_int = 2;
pub const FSME_FA64_BIT: c_int = 3;
pub const FSME2_BIT: c_int = 4;
pub const FGCS_BIT: c_int = 5;
pub const FPOE_BIT: c_int = 6;
pub const FMAX_END: c_int = 7;

pub const FEAT_SSBS: libc::c_ulong = 1 as libc::c_ulong << FSSBS_BIT;
pub const FEAT_SVE: libc::c_ulong = 1 as libc::c_ulong << FSVE_BIT;
pub const FEAT_SME: libc::c_ulong = 1 as libc::c_ulong << FSME_BIT;
pub const FEAT_SME_FA64: libc::c_ulong = 1 as libc::c_ulong << FSME_FA64_BIT;
pub const FEAT_SME2: libc::c_ulong = 1 as libc::c_ulong << FSME2_BIT;
pub const FEAT_GCS: libc::c_ulong = 1 as libc::c_ulong << FGCS_BIT;
pub const FEAT_POE: libc::c_ulong = 1 as libc::c_ulong << FPOE_BIT;

/*
 * A descriptor used to describe and configure a test case.
 * Fields with a non-trivial meaning are described inline in the following.
 */
#[repr(C)]
pub struct tdescr {
    /* KEEP THIS FIELD FIRST for easier lookup from assembly */
    pub token: *mut c_void,
    /* when disabled token based sanity checking is skipped in handler */
    pub sanity_disabled: bool,
    /* just a name for the test-case; manadatory field */
    pub name: *mut c_char,
    pub descr: *mut c_char,
    pub feats_required: libc::c_ulong,
    pub feats_incompatible: libc::c_ulong,
    /* bitmask of effectively supported feats: populated at run-time */
    pub feats_supported: libc::c_ulong,
    pub initialized: bool,
    pub minsigstksz: libc::c_uint,
    /* signum used as a test trigger. Zero if no trigger-signal is used */
    pub sig_trig: c_int,
    /*
     * signum considered as a successful test completion.
     * Zero when no signal is expected on success
     */
    pub sig_ok: c_int,
    /*
     * expected si_code for sig_ok, or 0 to not check
     */
    pub sig_ok_code: c_int,
    /* signum expected on unsupported CPU features. */
    pub sig_unsupp: c_int,
    /* a timeout in second for test completion */
    pub timeout: libc::c_uint,
    pub triggered: bool,
    pub pass: bool,
    pub result: libc::c_uint,
    /* optional sa_flags for the installed handler */
    pub sa_flags: c_int,
    pub saved_uc: libc::ucontext_t,
    /* used by get_current_ctx() */
    pub live_sz: libc::size_t,
    pub live_uc: *mut libc::ucontext_t,
    pub live_uc_valid: libc::sig_atomic_t,
    /* optional test private data */
    pub priv_: *mut c_void,

    /* a custom setup: called alternatively to default_setup */
    pub setup: Option<unsafe extern "C" fn(td: *mut tdescr) -> c_int>,
    /* a custom init: called by default test init after test_setup */
    pub init: Option<unsafe extern "C" fn(td: *mut tdescr) -> bool>,
    /* a custom cleanup function called before test exits */
    pub cleanup: Option<unsafe extern "C" fn(td: *mut tdescr)>,
    /* an optional function to be used as a trigger for starting test */
    pub trigger: Option<unsafe extern "C" fn(td: *mut tdescr) -> c_int>,
    /*
     * the actual test-core: invoked differently depending on the
     * presence of the trigger function above; this is mandatory
     */
    pub run: Option<
        unsafe extern "C" fn(
            td: *mut tdescr,
            si: *mut libc::siginfo_t,
            uc: *mut libc::ucontext_t,
        ) -> c_int,
    >,
    /* an optional function for custom results' processing */
    pub check_result: Option<unsafe extern "C" fn(td: *mut tdescr)>,
}

unsafe extern "C" {
    pub static mut tde: tdescr;
}
