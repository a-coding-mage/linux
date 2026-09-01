// SPDX-License-Identifier: LGPL-2.1

// Original dependencies: "namespaces.h", <unistd.h>, <sys/syscall.h>

extern "C" {
    fn syscall(num: libc::c_long, ...) -> libc::c_long;
}

pub unsafe extern "C" fn setns(fd: libc::c_int, nstype: libc::c_int) -> libc::c_int {
    syscall(libc::SYS_setns as libc::c_long, fd, nstype) as libc::c_int
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
