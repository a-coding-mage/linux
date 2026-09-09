/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the corresponding vdso datapage definitions.

unsafe extern "C" {
    pub fn __s390_vdso_getcpu(
        cpu: *mut ::core::ffi::c_uint,
        node: *mut ::core::ffi::c_uint,
        unused: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;

    pub fn __s390_vdso_gettimeofday(
        tv: *mut __kernel_old_timeval,
        tz: *mut timezone,
    ) -> ::core::ffi::c_int;

    pub fn __s390_vdso_clock_gettime(
        clock: clockid_t,
        ts: *mut __kernel_timespec,
    ) -> ::core::ffi::c_int;

    pub fn __s390_vdso_clock_getres(
        clock: clockid_t,
        ts: *mut __kernel_timespec,
    ) -> ::core::ffi::c_int;

    pub fn __kernel_getrandom(
        buffer: *mut ::core::ffi::c_void,
        len: usize,
        flags: ::core::ffi::c_uint,
        opaque_state: *mut ::core::ffi::c_void,
        opaque_len: usize,
    ) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
