/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * mm definition for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependency: ../nolibc.h */

/* C header guard _NOLIBC_SYS_MMAN_H omitted in Rust. */

/* C dependencies: ../arch.h, ../sys.h */

/* C conditional: this definition is present only when _sys_mmap is not already defined. */
#[cfg(__NR_mmap2)]
pub unsafe fn _sys_mmap(
    addr: *mut core::ffi::c_void,
    length: size_t,
    prot: core::ffi::c_int,
    flags: core::ffi::c_int,
    fd: core::ffi::c_int,
    mut offset: off_t,
) -> *mut core::ffi::c_void {
    let n: core::ffi::c_int;

    n = __NR_mmap2;
    offset >>= 12;

    __nolibc_syscall6(n, addr, length, prot, flags, fd, offset) as usize as *mut core::ffi::c_void
}

/* C conditional: fallback used when __NR_mmap2 is not defined. */
#[cfg(not(__NR_mmap2))]
pub unsafe fn _sys_mmap(
    addr: *mut core::ffi::c_void,
    length: size_t,
    prot: core::ffi::c_int,
    flags: core::ffi::c_int,
    fd: core::ffi::c_int,
    offset: off_t,
) -> *mut core::ffi::c_void {
    let n: core::ffi::c_int;

    n = __NR_mmap;

    __nolibc_syscall6(n, addr, length, prot, flags, fd, offset) as usize as *mut core::ffi::c_void
}

pub unsafe fn mmap(
    addr: *mut core::ffi::c_void,
    length: size_t,
    prot: core::ffi::c_int,
    flags: core::ffi::c_int,
    fd: core::ffi::c_int,
    offset: off_t,
) -> *mut core::ffi::c_void {
    let mut ret: *mut core::ffi::c_void = _sys_mmap(addr, length, prot, flags, fd, offset);

    if (ret as usize) >= ((-4095isize) as usize) {
        SET_ERRNO(-((ret as isize) as core::ffi::c_long));
        ret = MAP_FAILED;
    }
    ret
}

pub unsafe fn _sys_mremap(
    old_address: *mut core::ffi::c_void,
    old_size: size_t,
    new_size: size_t,
    flags: core::ffi::c_int,
    new_address: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    __nolibc_syscall5(
        __NR_mremap,
        old_address,
        old_size,
        new_size,
        flags,
        new_address,
    ) as usize as *mut core::ffi::c_void
}

pub unsafe fn mremap(
    old_address: *mut core::ffi::c_void,
    old_size: size_t,
    new_size: size_t,
    flags: core::ffi::c_int,
    new_address: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut ret: *mut core::ffi::c_void =
        _sys_mremap(old_address, old_size, new_size, flags, new_address);

    if (ret as usize) >= ((-4095isize) as usize) {
        SET_ERRNO(-((ret as isize) as core::ffi::c_long));
        ret = MAP_FAILED;
    }
    ret
}

pub unsafe fn _sys_munmap(addr: *mut core::ffi::c_void, length: size_t) -> core::ffi::c_int {
    __nolibc_syscall2(__NR_munmap, addr, length)
}

pub unsafe fn munmap(addr: *mut core::ffi::c_void, length: size_t) -> core::ffi::c_int {
    __sysret(_sys_munmap(addr, length))
}
