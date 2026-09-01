// SPDX-License-Identifier: GPL-2.0
// _GNU_SOURCE was needed in C to get the dnotify-related defines from fcntl.h.

use libc::{
    c_int, c_void, fcntl, open, pause, printf, sigaction, sigemptyset, siginfo_t, F_NOTIFY,
    F_SETSIG, O_RDONLY, SA_SIGINFO, SIGRTMIN,
};

static mut event_fd: c_int = 0;

unsafe extern "C" fn handler(_sig: c_int, si: *mut siginfo_t, _data: *mut c_void) {
    unsafe {
        core::ptr::write_volatile(&raw mut event_fd, (*si).si_fd());
    }
}

fn main() {
    unsafe {
        let mut act: sigaction = core::mem::zeroed();
        let mut fd: c_int;

        act.sa_sigaction = handler as usize;
        sigemptyset(&mut act.sa_mask);
        act.sa_flags = SA_SIGINFO;
        sigaction(SIGRTMIN() + 1, &act, core::ptr::null_mut());

        fd = open(c".".as_ptr(), O_RDONLY);
        fcntl(fd, F_SETSIG, SIGRTMIN() + 1);
        fcntl(
            fd,
            F_NOTIFY,
            libc::DN_MODIFY | libc::DN_CREATE | libc::DN_MULTISHOT,
        );
        /* we will now be notified if any of the files
           in "." is modified or new files are created */
        loop {
            pause();
            printf(
                c"Got event on fd=%d\n".as_ptr(),
                core::ptr::read_volatile(&raw const event_fd),
            );
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
