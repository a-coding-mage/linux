// SPDX-License-Identifier: GPL-2.0

// Values supplied by the Linux and UML headers.
#[cfg(not(any()))]
const PTRACE_GET_THREAD_AREA: ::std::os::raw::c_ulong = 25;
#[cfg(not(any()))]
const PTRACE_SET_THREAD_AREA: ::std::os::raw::c_ulong = 26;

// Build-time constants and types supplied by the included headers:
// GDT_ENTRY_TLS_MIN_I386, GDT_ENTRY_TLS_MIN_X86_64, __NR_get_thread_area,
// user_desc_t, EINVAL, and ENOSYS.

extern "C" {
    fn syscall(number: ::std::os::raw::c_long, ...) -> ::std::os::raw::c_long;
    fn ptrace(request: ::std::os::raw::c_ulong, ...) -> ::std::os::raw::c_long;
    fn __errno_location() -> *mut ::std::os::raw::c_int;
}

/* Checks whether host supports TLS, and sets *tls_min according to the value
 * valid on the host.
 * i386 host have it == 6; x86_64 host have it == 12, for i386 emulation. */
pub unsafe fn check_host_supports_tls(
    supports_tls: *mut ::std::os::raw::c_int,
    tls_min: *mut ::std::os::raw::c_int,
) {
    /* Values for x86 and x86_64. */
    let val = [GDT_ENTRY_TLS_MIN_I386, GDT_ENTRY_TLS_MIN_X86_64];

    for &entry in val.iter() {
        let mut info: user_desc_t = ::std::mem::zeroed();
        info.entry_number = entry;

        if syscall(__NR_get_thread_area as ::std::os::raw::c_long, &mut info) == 0 {
            *tls_min = entry;
            *supports_tls = 1;
            return;
        } else {
            let errno = *__errno_location();
            if errno == EINVAL {
                continue;
            } else if errno == ENOSYS {
                *supports_tls = 0;
            }
            return;
        }
    }

    *supports_tls = 0;
}

pub unsafe fn os_set_thread_area(info: *mut user_desc_t, pid: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    let mut ret = ptrace(
        PTRACE_SET_THREAD_AREA,
        pid,
        (*info).entry_number,
        info as ::std::os::raw::c_ulong,
    ) as ::std::os::raw::c_int;
    if ret < 0 {
        ret = -(*__errno_location());
    }
    ret
}

pub unsafe fn os_get_thread_area(info: *mut user_desc_t, pid: ::std::os::raw::c_int) -> ::std::os::raw::c_int {
    let mut ret = ptrace(
        PTRACE_GET_THREAD_AREA,
        pid,
        (*info).entry_number,
        info as ::std::os::raw::c_ulong,
    ) as ::std::os::raw::c_int;
    if ret < 0 {
        ret = -(*__errno_location());
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
