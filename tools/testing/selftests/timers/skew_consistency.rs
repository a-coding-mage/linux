/* ADJ_FREQ Skew consistency test
 *		by: john stultz (johnstul@us.ibm.com)
 *		(C) Copyright IBM 2012
 *		Licensed under the GPLv2
 *
 *  NOTE: This is a meta-test which cranks the ADJ_FREQ knob back
 *  and forth and watches for consistency problems. Thus this test requires
 *  that the inconsistency-check tests be present in the same directory it
 *  is run from.
 *
 *  To build:
 *	$ gcc skew_consistency.c -o skew_consistency -lrt
 *
 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 2 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 */

use libc::{adjtimex, fork, pid_t, printf, system, timex, usleep, waitpid, ADJ_FREQUENCY, WNOHANG};

unsafe extern "C" {
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

fn main() {
    unsafe {
        let mut tx: timex = std::mem::zeroed();
        let mut ret: libc::c_int;
        let mut ppm: libc::c_int;
        let pid: pid_t;

        printf(b"Running Asynchronous Frequency Changing Tests...\n\0".as_ptr() as *const libc::c_char);

        pid = fork();
        if pid == 0 {
            std::process::exit(system(
                b"./inconsistency-check -t 60\0".as_ptr() as *const libc::c_char,
            ));
        }

        ppm = 500;
        ret = 0;

        while pid != waitpid(pid, &mut ret, WNOHANG) {
            ppm = -ppm;
            tx.modes = ADJ_FREQUENCY;
            tx.freq = ppm << 16;
            adjtimex(&mut tx);
            usleep(500000);
        }

        /* Set things back */
        tx.modes = ADJ_FREQUENCY;
        tx.offset = 0;
        adjtimex(&mut tx);

        if ret != 0 {
            printf(b"[FAILED]\n\0".as_ptr() as *const libc::c_char);
            ksft_exit_fail();
        }
        printf(b"[OK]\n\0".as_ptr() as *const libc::c_char);
        ksft_exit_pass();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
