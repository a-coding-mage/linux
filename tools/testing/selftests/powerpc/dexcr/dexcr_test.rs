// SPDX-License-Identifier: GPL-2.0-or-later

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

type pid_t = c_int;

// Dependencies supplied by dexcr.h, utils.h, and system headers in the original C file.
unsafe extern "C" {
    static mut errno: c_int;

    static SPRN_DEXCR_RO: c_ulong;
    static PR_PPC_DEXCR_CTRL_SET: c_int;
    static PR_PPC_DEXCR_CTRL_CLEAR: c_int;
    static PR_PPC_DEXCR_CTRL_SET_ONEXEC: c_int;
    static PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC: c_int;
    static PR_PPC_DEXCR_IBRTPD: c_ulong;
    static PR_PPC_DEXCR_SRAPD: c_ulong;
    static PR_PPC_DEXCR_NPHIE: c_ulong;
    static EINVAL: c_int;

    fn mfspr(spr: c_ulong) -> c_ulong;
    fn pr_which_to_aspect(which: c_ulong) -> c_ulong;
    fn pr_get_dexcr(which: c_ulong) -> c_int;
    fn pr_set_dexcr(which: c_ulong, ctrl: c_int) -> c_int;
    fn dexcr_exists() -> bool;
    fn pr_dexcr_aspect_supported(which: c_ulong) -> bool;
    fn pr_dexcr_aspect_editable(which: c_ulong) -> bool;
    fn parse_ulong(str: *const c_char, len: usize, value: *mut c_ulong, base: c_int) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
    fn await_child_success(pid: pid_t);

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn fork() -> pid_t;
    fn execve(
        pathname: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    fn _exit(status: c_int) -> !;
}

macro_rules! FAIL_IF_MSG {
    ($cond:expr, $msg:expr) => {
        if $cond {
            return 1;
        }
    };
}

macro_rules! FAIL_IF_EXIT_MSG {
    ($cond:expr, $msg:expr) => {
        if $cond {
            _exit(1);
        }
    };
}

macro_rules! SKIP_IF_MSG {
    ($cond:expr, $msg:expr) => {
        if $cond {
            return 4;
        }
    };
}

/*
 * Helper function for testing the behaviour of a newly exec-ed process
 */
unsafe extern "C" fn dexcr_prctl_onexec_test_child(
    which: c_ulong,
    status: *const c_char,
) -> c_int {
    let dexcr: c_ulong = mfspr(SPRN_DEXCR_RO);
    let aspect: c_ulong = pr_which_to_aspect(which);
    let ctrl: c_int = pr_get_dexcr(which);

    if strcmp(status, c"set".as_ptr()) == 0 {
        FAIL_IF_EXIT_MSG!(
            !(ctrl & PR_PPC_DEXCR_CTRL_SET) != 0,
            "setting aspect across exec not applied"
        );

        FAIL_IF_EXIT_MSG!(
            !(ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC) != 0,
            "setting aspect across exec not inherited"
        );

        FAIL_IF_EXIT_MSG!(
            !(aspect & dexcr) != 0,
            "setting aspect across exec did not take effect"
        );
    } else if strcmp(status, c"clear".as_ptr()) == 0 {
        FAIL_IF_EXIT_MSG!(
            !(ctrl & PR_PPC_DEXCR_CTRL_CLEAR) != 0,
            "clearing aspect across exec not applied"
        );

        FAIL_IF_EXIT_MSG!(
            !(ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC) != 0,
            "clearing aspect across exec not inherited"
        );

        FAIL_IF_EXIT_MSG!(
            (aspect & dexcr) != 0,
            "clearing aspect across exec did not take effect"
        );
    } else {
        FAIL_IF_EXIT_MSG!(true, "unknown expected status");
    }

    0
}

/*
 * Test that the given prctl value can be manipulated freely
 */
unsafe extern "C" fn dexcr_prctl_aspect_test(which: c_ulong) -> c_int {
    let aspect: c_ulong = pr_which_to_aspect(which);
    let mut pid: pid_t;
    let mut ctrl: c_int;
    let mut err: c_int;
    let mut errno_save: c_int;

    SKIP_IF_MSG!(!dexcr_exists(), "DEXCR not supported");
    SKIP_IF_MSG!(
        !pr_dexcr_aspect_supported(which),
        "DEXCR aspect not supported"
    );
    SKIP_IF_MSG!(
        !pr_dexcr_aspect_editable(which),
        "DEXCR aspect not editable with prctl"
    );

    /* We reject invalid combinations of arguments */
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR);
    errno_save = errno;
    FAIL_IF_MSG!(
        err != -1,
        "simultaneous set and clear should be rejected"
    );
    FAIL_IF_MSG!(
        errno_save != EINVAL,
        "simultaneous set and clear should be rejected with EINVAL"
    );

    err = pr_set_dexcr(
        which,
        PR_PPC_DEXCR_CTRL_SET_ONEXEC | PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC,
    );
    errno_save = errno;
    FAIL_IF_MSG!(
        err != -1,
        "simultaneous set and clear on exec should be rejected"
    );
    FAIL_IF_MSG!(
        errno_save != EINVAL,
        "simultaneous set and clear on exec should be rejected with EINVAL"
    );

    /* We set the aspect */
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_SET);
    FAIL_IF_MSG!(err != 0, "PR_PPC_DEXCR_CTRL_SET failed");

    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG!(
        !(ctrl & PR_PPC_DEXCR_CTRL_SET) != 0,
        "config value not PR_PPC_DEXCR_CTRL_SET"
    );
    FAIL_IF_MSG!(
        (ctrl & PR_PPC_DEXCR_CTRL_CLEAR) != 0,
        "config value unexpected clear flag"
    );
    FAIL_IF_MSG!(
        !(aspect & mfspr(SPRN_DEXCR_RO)) != 0,
        "setting aspect did not take effect"
    );

    /* We clear the aspect */
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_CLEAR);
    FAIL_IF_MSG!(err != 0, "PR_PPC_DEXCR_CTRL_CLEAR failed");

    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG!(
        !(ctrl & PR_PPC_DEXCR_CTRL_CLEAR) != 0,
        "config value not PR_PPC_DEXCR_CTRL_CLEAR"
    );
    FAIL_IF_MSG!(
        (ctrl & PR_PPC_DEXCR_CTRL_SET) != 0,
        "config value unexpected set flag"
    );
    FAIL_IF_MSG!(
        (aspect & mfspr(SPRN_DEXCR_RO)) != 0,
        "clearing aspect did not take effect"
    );

    /* We make it set on exec (doesn't change our current value) */
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_SET_ONEXEC);
    FAIL_IF_MSG!(err != 0, "PR_PPC_DEXCR_CTRL_SET_ONEXEC failed");

    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG!(
        !(ctrl & PR_PPC_DEXCR_CTRL_CLEAR) != 0,
        "process aspect should still be cleared"
    );
    FAIL_IF_MSG!(
        !(ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC) != 0,
        "config value not PR_PPC_DEXCR_CTRL_SET_ONEXEC"
    );
    FAIL_IF_MSG!(
        (ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC) != 0,
        "config value unexpected clear on exec flag"
    );
    FAIL_IF_MSG!(
        (aspect & mfspr(SPRN_DEXCR_RO)) != 0,
        "scheduling aspect to set on exec should not change it now"
    );

    /* We make it clear on exec (doesn't change our current value) */
    err = pr_set_dexcr(which, PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC);
    FAIL_IF_MSG!(err != 0, "PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC failed");

    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG!(
        !(ctrl & PR_PPC_DEXCR_CTRL_CLEAR) != 0,
        "process aspect config should still be cleared"
    );
    FAIL_IF_MSG!(
        !(ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC) != 0,
        "config value not PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC"
    );
    FAIL_IF_MSG!(
        (ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC) != 0,
        "config value unexpected set on exec flag"
    );
    FAIL_IF_MSG!(
        (aspect & mfspr(SPRN_DEXCR_RO)) != 0,
        "process aspect should still be cleared"
    );

    /* We allow setting the current and on-exec value in a single call */
    err = pr_set_dexcr(
        which,
        PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC,
    );
    FAIL_IF_MSG!(
        err != 0,
        "PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC failed"
    );

    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG!(
        !(ctrl & PR_PPC_DEXCR_CTRL_SET) != 0,
        "config value not PR_PPC_DEXCR_CTRL_SET"
    );
    FAIL_IF_MSG!(
        !(ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC) != 0,
        "config value not PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC"
    );
    FAIL_IF_MSG!(
        !(aspect & mfspr(SPRN_DEXCR_RO)) != 0,
        "process aspect should be set"
    );

    err = pr_set_dexcr(
        which,
        PR_PPC_DEXCR_CTRL_CLEAR | PR_PPC_DEXCR_CTRL_SET_ONEXEC,
    );
    FAIL_IF_MSG!(
        err != 0,
        "PR_PPC_DEXCR_CTRL_CLEAR | PR_PPC_DEXCR_CTRL_SET_ONEXEC failed"
    );

    ctrl = pr_get_dexcr(which);
    FAIL_IF_MSG!(
        !(ctrl & PR_PPC_DEXCR_CTRL_CLEAR) != 0,
        "config value not PR_PPC_DEXCR_CTRL_CLEAR"
    );
    FAIL_IF_MSG!(
        !(ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC) != 0,
        "config value not PR_PPC_DEXCR_CTRL_SET_ONEXEC"
    );
    FAIL_IF_MSG!((aspect & mfspr(SPRN_DEXCR_RO)) != 0, "process aspect should be clear");

    /* Verify the onexec value is applied across exec */
    pid = fork();
    if pid == 0 {
        let mut which_str: [c_char; 32] = [0; 32];
        let mut args: [*mut c_char; 4] = [
            c"dexcr_prctl_onexec_test_child".as_ptr() as *mut c_char,
            which_str.as_mut_ptr(),
            c"set".as_ptr() as *mut c_char,
            core::ptr::null_mut(),
        ];
        let ctrl: c_int = pr_get_dexcr(which);

        sprintf(which_str.as_mut_ptr(), c"%lu".as_ptr(), which);

        FAIL_IF_EXIT_MSG!(
            !(ctrl & PR_PPC_DEXCR_CTRL_SET_ONEXEC) != 0,
            "setting aspect on exec not copied across fork"
        );

        FAIL_IF_EXIT_MSG!(
            (mfspr(SPRN_DEXCR_RO) & aspect) != 0,
            "setting aspect on exec wrongly applied to fork"
        );

        execve(c"/proc/self/exe".as_ptr(), args.as_mut_ptr(), core::ptr::null());
        _exit(errno);
    }
    await_child_success(pid);

    err = pr_set_dexcr(
        which,
        PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC,
    );
    FAIL_IF_MSG!(
        err != 0,
        "PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC failed"
    );

    pid = fork();
    if pid == 0 {
        let mut which_str: [c_char; 32] = [0; 32];
        let mut args: [*mut c_char; 4] = [
            c"dexcr_prctl_onexec_test_child".as_ptr() as *mut c_char,
            which_str.as_mut_ptr(),
            c"clear".as_ptr() as *mut c_char,
            core::ptr::null_mut(),
        ];
        let ctrl: c_int = pr_get_dexcr(which);

        sprintf(which_str.as_mut_ptr(), c"%lu".as_ptr(), which);

        FAIL_IF_EXIT_MSG!(
            !(ctrl & PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC) != 0,
            "clearing aspect on exec not copied across fork"
        );

        FAIL_IF_EXIT_MSG!(
            !(mfspr(SPRN_DEXCR_RO) & aspect) != 0,
            "clearing aspect on exec wrongly applied to fork"
        );

        execve(c"/proc/self/exe".as_ptr(), args.as_mut_ptr(), core::ptr::null());
        _exit(errno);
    }
    await_child_success(pid);

    0
}

unsafe extern "C" fn dexcr_prctl_ibrtpd_test() -> c_int {
    dexcr_prctl_aspect_test(PR_PPC_DEXCR_IBRTPD)
}

unsafe extern "C" fn dexcr_prctl_srapd_test() -> c_int {
    dexcr_prctl_aspect_test(PR_PPC_DEXCR_SRAPD)
}

unsafe extern "C" fn dexcr_prctl_nphie_test() -> c_int {
    dexcr_prctl_aspect_test(PR_PPC_DEXCR_NPHIE)
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut err: c_int = 0;

    /*
     * Some tests require checking what happens across exec, so we may be
     * invoked as the child of a particular test
     */
    if argc > 1 {
        if argc == 3
            && strcmp(
                *argv.offset(0),
                c"dexcr_prctl_onexec_test_child".as_ptr(),
            ) == 0
        {
            let mut which: c_ulong = 0;

            err = parse_ulong(
                *argv.offset(1),
                strlen(*argv.offset(1)),
                &mut which,
                10,
            );
            FAIL_IF_MSG!(err != 0, "failed to parse which value for child");

            return dexcr_prctl_onexec_test_child(which, *argv.offset(2));
        }

        FAIL_IF_MSG!(true, "unknown test case");
    }

    /*
     * Otherwise we are the main test invocation and run the full suite
     */
    err |= test_harness(dexcr_prctl_ibrtpd_test, c"dexcr_prctl_ibrtpd".as_ptr());
    err |= test_harness(dexcr_prctl_srapd_test, c"dexcr_prctl_srapd".as_ptr());
    err |= test_harness(dexcr_prctl_nphie_test, c"dexcr_prctl_nphie".as_ptr());

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
