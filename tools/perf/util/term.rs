// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/term.c. C includes:
// "term.h", <stdlib.h>, <termios.h>, <unistd.h>, <sys/ioctl.h>

use libc::{
    atoi, getenv, ioctl, tcgetattr, tcsetattr, termios, winsize, ECHO, ICANON, TCSANOW, VMIN,
    VTIME,
};

#[no_mangle]
pub unsafe extern "C" fn get_term_dimensions(ws: *mut winsize) {
    let mut s = getenv(b"LINES\0".as_ptr() as *const libc::c_char);

    if !s.is_null() {
        (*ws).ws_row = atoi(s) as _;
        s = getenv(b"COLUMNS\0".as_ptr() as *const libc::c_char);
        if !s.is_null() {
            (*ws).ws_col = atoi(s) as _;
            if (*ws).ws_row != 0 && (*ws).ws_col != 0 {
                return;
            }
        }
    }

    // C source condition: #ifdef TIOCGWINSZ
    #[cfg(any(
        target_os = "android",
        target_os = "emscripten",
        target_os = "fuchsia",
        target_os = "linux",
        target_os = "redox"
    ))]
    {
        if ioctl(1, libc::TIOCGWINSZ, ws) == 0 && (*ws).ws_row != 0 && (*ws).ws_col != 0 {
            return;
        }
    }

    (*ws).ws_row = 25;
    (*ws).ws_col = 80;
}

#[no_mangle]
pub unsafe extern "C" fn set_term_quiet_input(old: *mut termios) {
    let mut tc: termios = std::mem::zeroed();

    tcgetattr(0, old);
    tc = *old;
    tc.c_lflag &= !(ICANON | ECHO);
    tc.c_cc[VMIN] = 0;
    tc.c_cc[VTIME] = 0;
    tcsetattr(0, TCSANOW, &tc);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
