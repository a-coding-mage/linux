// SPDX-License-Identifier: GPL-2.0-only
/* Some common code for MSG_ZEROCOPY logic
 *
 * Copyright (C) 2023 SberDevices.
 *
 * Author: Arseniy Krasnov <avkrasnov@salutedevices.com>
 */

/* Dependencies in the original C source:
 * stdio.h, stdlib.h, sys/types.h, sys/socket.h, linux/errqueue.h,
 * and "msg_zerocopy_common.h".
 */

use libc::{
    c_char, c_int, cmsghdr, fprintf, msghdr, recvmsg, sock_extended_err, stderr, CMSG_DATA,
    CMSG_FIRSTHDR, EXIT_FAILURE, MSG_ERRQUEUE, SO_EE_CODE_ZEROCOPY_COPIED,
    SO_EE_ORIGIN_ZEROCOPY, SOL_VSOCK, VSOCK_RECVERR,
};

#[no_mangle]
pub unsafe extern "C" fn vsock_recv_completion(fd: c_int, zerocopied: *const bool) {
    let mut serr: *mut sock_extended_err;
    let mut msg: msghdr = unsafe { std::mem::zeroed() };
    let mut cmsg_data: [c_char; 128] = [0; 128];
    let mut cm: *mut cmsghdr;
    let res: isize;

    msg.msg_control = cmsg_data.as_mut_ptr() as *mut libc::c_void;
    msg.msg_controllen = std::mem::size_of_val(&cmsg_data) as _;

    res = unsafe { recvmsg(fd, &mut msg, MSG_ERRQUEUE) };
    if res != 0 {
        unsafe {
            fprintf(
                stderr,
                b"failed to read error queue: %zi\n\0".as_ptr() as *const c_char,
                res,
            );
            libc::exit(EXIT_FAILURE);
        }
    }

    cm = unsafe { CMSG_FIRSTHDR(&msg) };
    if cm.is_null() {
        unsafe {
            fprintf(stderr, b"cmsg: no cmsg\n\0".as_ptr() as *const c_char);
            libc::exit(EXIT_FAILURE);
        }
    }

    if unsafe { (*cm).cmsg_level } != SOL_VSOCK {
        unsafe {
            fprintf(
                stderr,
                b"cmsg: unexpected 'cmsg_level'\n\0".as_ptr() as *const c_char,
            );
            libc::exit(EXIT_FAILURE);
        }
    }

    if unsafe { (*cm).cmsg_type } != VSOCK_RECVERR {
        unsafe {
            fprintf(
                stderr,
                b"cmsg: unexpected 'cmsg_type'\n\0".as_ptr() as *const c_char,
            );
            libc::exit(EXIT_FAILURE);
        }
    }

    serr = unsafe { CMSG_DATA(cm) as *mut sock_extended_err };
    if unsafe { (*serr).ee_origin } != SO_EE_ORIGIN_ZEROCOPY {
        unsafe {
            fprintf(
                stderr,
                b"serr: wrong origin: %u\n\0".as_ptr() as *const c_char,
                (*serr).ee_origin as libc::c_uint,
            );
            libc::exit(EXIT_FAILURE);
        }
    }

    if unsafe { (*serr).ee_errno } != 0 {
        unsafe {
            fprintf(
                stderr,
                b"serr: wrong error code: %u\n\0".as_ptr() as *const c_char,
                (*serr).ee_errno as libc::c_uint,
            );
            libc::exit(EXIT_FAILURE);
        }
    }

    /* This flag is used for tests, to check that transmission was
     * performed as expected: zerocopy or fallback to copy. If NULL
     * - don't care.
     */
    if zerocopied.is_null() {
        return;
    }

    if unsafe { *zerocopied } && (unsafe { (*serr).ee_code } & SO_EE_CODE_ZEROCOPY_COPIED) != 0 {
        unsafe {
            fprintf(
                stderr,
                b"serr: was copy instead of zerocopy\n\0".as_ptr() as *const c_char,
            );
            libc::exit(EXIT_FAILURE);
        }
    }

    if !unsafe { *zerocopied } && (unsafe { (*serr).ee_code } & SO_EE_CODE_ZEROCOPY_COPIED) == 0 {
        unsafe {
            fprintf(
                stderr,
                b"serr: was zerocopy instead of copy\n\0".as_ptr() as *const c_char,
            );
            libc::exit(EXIT_FAILURE);
        }
    }
}
