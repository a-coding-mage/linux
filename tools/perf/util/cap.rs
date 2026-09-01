// SPDX-License-Identifier: GPL-2.0
/*
 * Capability utilities
 */

use core::ffi::{c_char, c_int, c_long, c_uint};

// C includes translated as external dependency intent:
// "cap.h", "debug.h", <errno.h>, <string.h>, <sys/syscall.h>, <unistd.h>

type __u32 = c_uint;
type uid_t = c_uint;

const _LINUX_CAPABILITY_VERSION_1: __u32 = 0x19980330;
const _LINUX_CAPABILITY_VERSION_3: __u32 = 0x20080522;
const _LINUX_CAPABILITY_U32S_3: usize = 2;
const MAX_LINUX_CAPABILITY_U32S: usize = _LINUX_CAPABILITY_U32S_3;

const EINVAL: c_int = 22;

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
const SYS_capget: c_long = 125;
#[cfg(target_arch = "x86")]
const SYS_capget: c_long = 184;
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "x86")))]
const SYS_capget: c_long = 125;

#[repr(C)]
pub struct __user_cap_header_struct {
    pub version: __u32,
    pub pid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __user_cap_data_struct {
    pub effective: __u32,
    pub permitted: __u32,
    pub inheritable: __u32,
}

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn geteuid() -> uid_t;
    fn __errno_location() -> *mut c_int;
    fn pr_debug2(fmt: *const c_char, ...);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cap__capable(cap: c_int) -> bool {
    let mut header = __user_cap_header_struct {
        version: _LINUX_CAPABILITY_VERSION_3,
        pid: 0,
    };
    let mut data = [__user_cap_data_struct {
        effective: 0,
        permitted: 0,
        inheritable: 0,
    }; MAX_LINUX_CAPABILITY_U32S];
    let cap_val: __u32;

    while unsafe {
        syscall(
            SYS_capget,
            &mut header as *mut __user_cap_header_struct,
            &mut data[0] as *mut __user_cap_data_struct,
        )
    } == -1
    {
        /* Retry, first attempt has set the header.version correctly. */
        if unsafe { *__errno_location() } == EINVAL
            && header.version != _LINUX_CAPABILITY_VERSION_3
            && header.version == _LINUX_CAPABILITY_VERSION_1
        {
            continue;
        }

        unsafe {
            pr_debug2(
                b"capget syscall failed (%m) fall back on root check\n\0".as_ptr()
                    as *const c_char,
            );
        }
        return unsafe { geteuid() } == 0;
    }

    /* Extract the relevant capability bit. */
    if cap >= 32 {
        if header.version == _LINUX_CAPABILITY_VERSION_3 {
            cap_val = data[1].effective;
        } else {
            /* Capability beyond 32 is requested but only 32 are supported. */
            return false;
        }
    } else {
        cap_val = data[0].effective;
    }
    (cap_val & (1_u32 << (cap & 0x1f))) != 0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
