// SPDX-License-Identifier: GPL-2.0
// C dependencies: errno.h, fcntl.h, limits.h, stdio.h, stdlib.h, unistd.h,
// sys/types.h, sys/stat.h

use libc::{
    atol, c_char, c_int, close, fstat, fprintf, off_t, open, perror, size_t, splice, stat, ssize_t,
    stderr, O_RDONLY, SPLICE_F_MOVE, STDOUT_FILENO,
};

const INT_MAX: off_t = c_int::MAX as off_t;

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let fd: c_int;
    let size: size_t;
    let spliced: ssize_t;

    if argc < 2 {
        fprintf(
            stderr,
            b"Usage: %s INPUT [BYTES]\n\0".as_ptr() as *const c_char,
            *argv.offset(0),
        );
        return libc::EXIT_FAILURE;
    }

    fd = open(*argv.offset(1), O_RDONLY);
    if fd < 0 {
        perror(*argv.offset(1));
        return libc::EXIT_FAILURE;
    }

    if argc == 3 {
        size = atol(*argv.offset(2)) as size_t;
    } else {
        let mut statbuf: stat = std::mem::zeroed();

        if fstat(fd, &mut statbuf) < 0 {
            perror(*argv.offset(1));
            return libc::EXIT_FAILURE;
        }

        if statbuf.st_size > INT_MAX {
            fprintf(
                stderr,
                b"%s: Too big\n\0".as_ptr() as *const c_char,
                *argv.offset(1),
            );
            return libc::EXIT_FAILURE;
        }

        size = statbuf.st_size as size_t;
    }

    /* splice(2) file to stdout. */
    spliced = splice(
        fd,
        std::ptr::null_mut(),
        STDOUT_FILENO,
        std::ptr::null_mut(),
        size,
        SPLICE_F_MOVE,
    );
    if spliced < 0 {
        perror(b"splice\0".as_ptr() as *const c_char);
        return libc::EXIT_FAILURE;
    }

    close(fd);
    libc::EXIT_SUCCESS
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
