// SPDX-License-Identifier: GPL-2.0
// Dependency declarations and constants are supplied by linux/termios_internal.h.

extern "C" {
    fn copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, n: usize) -> usize;
    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
}

pub unsafe fn user_termio_to_kernel_termios(
    termios: *mut ktermios,
    termio: *mut termio,
) -> i32 {
    let mut v: termio = core::mem::zeroed();
    let canon: bool;

    if copy_from_user(
        &mut v as *mut termio as *mut core::ffi::c_void,
        termio as *const core::ffi::c_void,
        core::mem::size_of::<termio>(),
    ) != 0 {
        return -EFAULT;
    }

    (*termios).c_iflag = (0xffff0000u32 & (*termios).c_iflag) | v.c_iflag;
    (*termios).c_oflag = (0xffff0000u32 & (*termios).c_oflag) | v.c_oflag;
    (*termios).c_cflag = (0xffff0000u32 & (*termios).c_cflag) | v.c_cflag;
    (*termios).c_lflag = (0xffff0000u32 & (*termios).c_lflag) | v.c_lflag;
    (*termios).c_line = (0xffff0000u32 & (*termios).c_lflag) | v.c_line;

    canon = (v.c_lflag & ICANON) != 0;
    (*termios).c_cc[VINTR] = v.c_cc[_VINTR];
    (*termios).c_cc[VQUIT] = v.c_cc[_VQUIT];
    (*termios).c_cc[VERASE] = v.c_cc[_VERASE];
    (*termios).c_cc[VKILL] = v.c_cc[_VKILL];
    (*termios).c_cc[VEOL2] = v.c_cc[_VEOL2];
    (*termios).c_cc[VSWTC] = v.c_cc[_VSWTC];
    (*termios).c_cc[if canon { VEOF } else { VMIN }] = v.c_cc[_VEOF];
    (*termios).c_cc[if canon { VEOL } else { VTIME }] = v.c_cc[_VEOL];

    0
}

pub unsafe fn kernel_termios_to_user_termio(
    termio: *mut termio,
    termios: *mut ktermios,
) -> i32 {
    let mut v: termio = core::mem::zeroed();
    let canon: bool;

    memset(
        &mut v as *mut termio as *mut core::ffi::c_void,
        0,
        core::mem::size_of::<termio>(),
    );
    v.c_iflag = (*termios).c_iflag;
    v.c_oflag = (*termios).c_oflag;
    v.c_cflag = (*termios).c_cflag;
    v.c_lflag = (*termios).c_lflag;
    v.c_line = (*termios).c_line;

    canon = (v.c_lflag & ICANON) != 0;
    v.c_cc[_VINTR] = (*termios).c_cc[VINTR];
    v.c_cc[_VQUIT] = (*termios).c_cc[VQUIT];
    v.c_cc[_VERASE] = (*termios).c_cc[VERASE];
    v.c_cc[_VKILL] = (*termios).c_cc[VKILL];
    v.c_cc[_VEOF] = (*termios).c_cc[if canon { VEOF } else { VMIN }];
    v.c_cc[_VEOL] = (*termios).c_cc[if canon { VEOL } else { VTIME }];
    v.c_cc[_VEOL2] = (*termios).c_cc[VEOL2];
    v.c_cc[_VSWTC] = (*termios).c_cc[VSWTC];

    copy_to_user(
        termio as *mut core::ffi::c_void,
        &v as *const termio as *const core::ffi::c_void,
        core::mem::size_of::<termio>(),
    ) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
