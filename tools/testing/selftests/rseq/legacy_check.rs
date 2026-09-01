// SPDX-License-Identifier: GPL-2.0
//
// Source dependencies preserved from C includes:
// errno.h, signal.h, stdint.h, unistd.h, "rseq.h",
// and "../kselftest_harness.h".

use core::ptr;

#[repr(C)]
pub struct rseq_abi {
    pub cpu_id_start: i32,
}

unsafe extern "C" {
    fn rseq_get_abi() -> *mut rseq_abi;
    fn __rseq_register_current_thread(param_1: bool, param_2: bool) -> i32;
    fn signal(
        signum: i32,
        handler: Option<unsafe extern "C" fn(i32)>,
    ) -> Option<unsafe extern "C" fn(i32)>;
    fn raise(sig: i32) -> i32;
    fn sleep(seconds: u32) -> u32;
}

// Constants supplied by the C system headers in the original source.
const ENOSYS: i32 = libc::ENOSYS;
const EBUSY: i32 = libc::EBUSY;
const SIGUSR1: i32 = libc::SIGUSR1;
const SIG_ERR: Option<unsafe extern "C" fn(i32)> =
    unsafe { core::mem::transmute::<isize, Option<unsafe extern "C" fn(i32)>>(-1) };

// FIXTURE(legacy)
// {
// };
pub struct legacy {}

static mut cpu_id_in_sigfn: i32 = -1;

unsafe extern "C" fn sigfn(_sig: i32) {
    let rs: *mut rseq_abi = unsafe { rseq_get_abi() };

    unsafe {
        cpu_id_in_sigfn = (*rs).cpu_id_start;
    }
}

// FIXTURE_SETUP(legacy)
pub unsafe fn legacy_setup() {
    let res: i32 = unsafe { __rseq_register_current_thread(true, true) };

    match res {
        x if x == -ENOSYS => {
            // SKIP(return, "RSEQ not enabled\n");
            return;
        }
        x if x == -EBUSY => {
            // SKIP(return, "GLIBC owns RSEQ. Disable GLIBC RSEQ registration\n");
            return;
        }
        _ => {
            assert_eq!(res, 0);
        }
    }

    assert_ne!(unsafe { signal(SIGUSR1, Some(sigfn)) }, SIG_ERR);
}

// FIXTURE_TEARDOWN(legacy)
pub unsafe fn legacy_teardown() {}

// TEST_F(legacy, legacy_test)
pub unsafe fn legacy_legacy_test() {
    let rs: *mut rseq_abi = unsafe { rseq_get_abi() };

    assert_ne!(rs, ptr::null_mut());

    /* Overwrite rs::cpu_id_start */
    unsafe {
        (*rs).cpu_id_start = -1;
    }
    unsafe {
        sleep(1);
    }
    assert_ne!(unsafe { (*rs).cpu_id_start }, -1);

    unsafe {
        (*rs).cpu_id_start = -1;
    }
    assert_eq!(unsafe { raise(SIGUSR1) }, 0);
    assert_ne!(unsafe { (*rs).cpu_id_start }, -1);
    assert_ne!(unsafe { cpu_id_in_sigfn }, -1);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
