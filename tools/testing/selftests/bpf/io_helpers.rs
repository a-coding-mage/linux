// SPDX-License-Identifier: GPL-2.0
// C dependencies: <sys/select.h>, <unistd.h>, <errno.h>

#[no_mangle]
pub unsafe extern "C" fn read_with_timeout(
    fd: libc::c_int,
    buf: *mut libc::c_char,
    count: libc::size_t,
    usec: libc::c_long,
) -> libc::c_int {
    const M: libc::c_long = 1000 * 1000;
    let mut tv = libc::timeval {
        tv_sec: usec / M,
        tv_usec: usec % M,
    };
    let mut fds: libc::fd_set = core::mem::zeroed();
    let mut err: libc::c_int;

    libc::FD_ZERO(&mut fds);
    libc::FD_SET(fd, &mut fds);
    err = libc::select(
        fd + 1,
        &mut fds,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &mut tv,
    );
    if err < 0 {
        return err;
    }
    if libc::FD_ISSET(fd, &mut fds) {
        return libc::read(fd, buf as *mut libc::c_void, count) as libc::c_int;
    }
    -libc::EAGAIN
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
