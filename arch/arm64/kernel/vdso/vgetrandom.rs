// SPDX-License-Identifier: GPL-2.0

// Dependency supplied by <uapi/asm-generic/errno.h>.
extern "C" {
    static ENOSYS: i32;
}

// The following symbols are supplied by the surrounding kernel implementation.
extern "C" {
    fn __cvdso_getrandom(
        buffer: *mut core::ffi::c_void,
        len: usize,
        flags: u32,
        opaque_state: *mut core::ffi::c_void,
        opaque_len: usize,
    ) -> isize;

    fn alternative_has_cap_likely(cap: u64) -> bool;
    fn getrandom_syscall(
        buffer: *mut core::ffi::c_void,
        len: usize,
        flags: u32,
    ) -> isize;
}

// Dependency supplied by the arm64 capability definitions.
extern "C" {
    static ARM64_HAS_FPSIMD: u64;
}

#[no_mangle]
pub unsafe extern "C" fn __kernel_getrandom(
    buffer: *mut core::ffi::c_void,
    len: usize,
    flags: u32,
    opaque_state: *mut core::ffi::c_void,
    opaque_len: usize,
) -> isize {
    if alternative_has_cap_likely(ARM64_HAS_FPSIMD) {
        return __cvdso_getrandom(buffer, len, flags, opaque_state, opaque_len);
    }

    if opaque_len == usize::MAX && buffer.is_null() && len == 0 && flags == 0 {
        return -ENOSYS as isize;
    }
    getrandom_syscall(buffer, len, flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
