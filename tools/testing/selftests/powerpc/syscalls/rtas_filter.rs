// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright 2005-2020 IBM Corporation.
 *
 * Includes code from librtas (https://github.com/ibm-power-utilities/librtas/)
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type uint32_t = u32;
type uint64_t = u64;
type __be32 = uint32_t;

const RTAS_IO_ASSERT: c_int = -1098; /* Unexpected I/O Error */
const RTAS_UNKNOWN_OP: c_int = -1099; /* No Firmware Implementation of Function */
const BLOCK_SIZE: usize = 4096;
const PAGE_SIZE: usize = 4096;
const MAX_PAGES: usize = 64;
const PATH_MAX: usize = 4096;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const __NR_rtas: c_long = 255;

static OFDT_RTAS_PATH: &[u8] = b"/proc/device-tree/rtas\0";

#[repr(C)]
struct rtas_args {
    token: __be32,
    nargs: __be32,
    nret: __be32,
    args: [__be32; 16],
    rets: *mut __be32, /* Pointer to return values in args[]. */
}

#[repr(C)]
struct region {
    addr: uint64_t,
    size: uint32_t,
    next: *mut region,
}

unsafe extern "C" {
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn read_file_alloc(path: *const c_char, buf: *mut *mut c_char, len: *mut size_t) -> c_int;
    fn free(ptr: *mut c_void);
    fn perror(s: *const c_char);
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

unsafe extern "C" {
    static mut errno: c_int;
}

#[inline]
fn cpu_to_be32(x: u32) -> u32 {
    if cfg!(target_endian = "little") {
        x.swap_bytes()
    } else {
        x
    }
}

#[inline]
fn be32_to_cpu(x: u32) -> u32 {
    if cfg!(target_endian = "little") {
        x.swap_bytes()
    } else {
        x
    }
}

unsafe fn get_property(
    prop_path: *const c_char,
    prop_name: *const c_char,
    prop_val: *mut *mut c_char,
    prop_len: *mut size_t,
) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];

    let len = unsafe {
        snprintf(
            path.as_mut_ptr(),
            mem::size_of_val(&path),
            b"%s/%s\0".as_ptr() as *const c_char,
            prop_path,
            prop_name,
        )
    };
    if len < 0 || len as usize >= mem::size_of_val(&path) {
        return -ENOMEM;
    }

    unsafe { read_file_alloc(path.as_ptr(), prop_val, prop_len) }
}

unsafe extern "C" fn rtas_token(call_name: *const c_char) -> c_int {
    let mut prop_buf: *mut c_char = ptr::null_mut();
    let mut len: size_t = 0;
    let mut rc: c_int;

    rc = unsafe {
        get_property(
            OFDT_RTAS_PATH.as_ptr() as *const c_char,
            call_name,
            &mut prop_buf,
            &mut len,
        )
    };
    if rc < 0 {
        rc = RTAS_UNKNOWN_OP;
    } else {
        rc = be32_to_cpu(unsafe { *(prop_buf as *mut c_int) } as u32) as c_int;
    }

    unsafe { free(prop_buf as *mut c_void) };
    rc
}

unsafe fn read_kregion_bounds(kregion: *mut region) -> c_int {
    let mut buf: *mut c_char = ptr::null_mut();
    let err: c_int;

    err = unsafe {
        read_file_alloc(
            b"/proc/ppc64/rtas/rmo_buffer\0".as_ptr() as *const c_char,
            &mut buf,
            ptr::null_mut(),
        )
    };
    if err != 0 {
        unsafe { perror(b"Could not open rmo_buffer file\0".as_ptr() as *const c_char) };
        return RTAS_IO_ASSERT;
    }

    unsafe {
        sscanf(
            buf,
            b"%llx %x\0".as_ptr() as *const c_char,
            &mut (*kregion).addr,
            &mut (*kregion).size,
        )
    };
    unsafe { free(buf as *mut c_void) };

    if unsafe { !((*kregion).size != 0 && (*kregion).addr != 0)
        || ((*kregion).size as usize > (PAGE_SIZE * MAX_PAGES)) }
    {
        unsafe { printf(b"Unexpected kregion bounds\n\0".as_ptr() as *const c_char) };
        return RTAS_IO_ASSERT;
    }

    0
}

unsafe fn rtas_call_impl(
    name: *const c_char,
    nargs: c_int,
    nrets: c_int,
    call_args: &[c_ulong],
    ret_args: &[*mut __be32],
) -> c_int {
    let mut args: rtas_args = unsafe { mem::zeroed() };
    let mut rets = [ptr::null_mut::<__be32>(); 16];
    let mut i: c_int;
    let mut rc: c_int;
    let token: c_int;

    token = unsafe { rtas_token(name) };
    if token == RTAS_UNKNOWN_OP {
        // We don't care if the call doesn't exist
        unsafe { printf(b"call '%s' not available, skipping...\0".as_ptr() as *const c_char, name) };
        return RTAS_UNKNOWN_OP;
    }

    args.token = cpu_to_be32(token as u32);
    args.nargs = cpu_to_be32(nargs as u32);
    args.nret = cpu_to_be32(nrets as u32);

    i = 0;
    while i < nargs {
        args.args[i as usize] = call_args[i as usize] as __be32;
        i += 1;
    }

    i = 0;
    while i < nrets {
        rets[i as usize] = ret_args[i as usize];
        i += 1;
    }

    rc = unsafe { syscall(__NR_rtas, &mut args as *mut rtas_args) as c_int };
    if rc != 0 {
        rc = unsafe { -errno };
        return rc;
    }

    if nrets != 0 {
        unsafe {
            *(rets[0]) = be32_to_cpu(args.args[nargs as usize]);
        }

        i = 1;
        while i < nrets {
            unsafe {
                *(rets[i as usize]) = args.args[(nargs + i) as usize];
            }
            i += 1;
        }
    }

    rc
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

unsafe extern "C" fn test() -> c_int {
    let mut rmo_region = region {
        addr: 0,
        size: 0,
        next: ptr::null_mut(),
    };
    let mut rmo_start: uint32_t;
    let mut rmo_end: uint32_t;
    let mut rets: [__be32; 1] = [0; 1];
    let mut rc: c_int;

    // Test a legitimate harmless call
    // Expected: call succeeds
    unsafe { printf(b"Test a permitted call, no parameters... \0".as_ptr() as *const c_char) };
    rc = unsafe {
        rtas_call_impl(
            b"get-time-of-day\0".as_ptr() as *const c_char,
            0,
            1,
            &[],
            &[rets.as_mut_ptr()],
        )
    };
    unsafe { printf(b"rc: %d\n\0".as_ptr() as *const c_char, rc) };
    FAIL_IF!(rc != 0 && rc != RTAS_UNKNOWN_OP);

    // Test a prohibited call
    // Expected: call returns -EINVAL
    unsafe { printf(b"Test a prohibited call... \0".as_ptr() as *const c_char) };
    rc = unsafe {
        rtas_call_impl(
            b"nvram-fetch\0".as_ptr() as *const c_char,
            0,
            1,
            &[],
            &[rets.as_mut_ptr()],
        )
    };
    unsafe { printf(b"rc: %d\n\0".as_ptr() as *const c_char, rc) };
    FAIL_IF!(rc != -EINVAL && rc != RTAS_UNKNOWN_OP);

    // Get RMO
    rc = unsafe { read_kregion_bounds(&mut rmo_region) };
    if rc != 0 {
        unsafe {
            printf(
                b"Couldn't read RMO region bounds, skipping remaining cases\n\0".as_ptr()
                    as *const c_char,
            )
        };
        return 0;
    }
    rmo_start = rmo_region.addr as uint32_t;
    rmo_end = rmo_start.wrapping_add(rmo_region.size).wrapping_sub(1);
    unsafe {
        printf(
            b"RMO range: %08x - %08x\n\0".as_ptr() as *const c_char,
            rmo_start,
            rmo_end,
        )
    };

    // Test a permitted call, user-supplied size, buffer inside RMO
    // Expected: call succeeds
    unsafe {
        printf(
            b"Test a permitted call, user-supplied size, buffer inside RMO... \0".as_ptr()
                as *const c_char,
        )
    };
    rc = unsafe {
        rtas_call_impl(
            b"ibm,get-system-parameter\0".as_ptr() as *const c_char,
            3,
            1,
            &[
                0,
                cpu_to_be32(rmo_start) as c_ulong,
                cpu_to_be32(rmo_end.wrapping_sub(rmo_start).wrapping_add(1)) as c_ulong,
            ],
            &[rets.as_mut_ptr()],
        )
    };
    unsafe { printf(b"rc: %d\n\0".as_ptr() as *const c_char, rc) };
    FAIL_IF!(rc != 0 && rc != RTAS_UNKNOWN_OP);

    // Test a permitted call, user-supplied size, buffer start outside RMO
    // Expected: call returns -EINVAL
    unsafe {
        printf(
            b"Test a permitted call, user-supplied size, buffer start outside RMO... \0".as_ptr()
                as *const c_char,
        )
    };
    rc = unsafe {
        rtas_call_impl(
            b"ibm,get-system-parameter\0".as_ptr() as *const c_char,
            3,
            1,
            &[
                0,
                cpu_to_be32(rmo_end.wrapping_add(1)) as c_ulong,
                cpu_to_be32(4000) as c_ulong,
            ],
            &[rets.as_mut_ptr()],
        )
    };
    unsafe { printf(b"rc: %d\n\0".as_ptr() as *const c_char, rc) };
    FAIL_IF!(rc != -EINVAL && rc != RTAS_UNKNOWN_OP);

    // Test a permitted call, user-supplied size, buffer end outside RMO
    // Expected: call returns -EINVAL
    unsafe {
        printf(
            b"Test a permitted call, user-supplied size, buffer end outside RMO... \0".as_ptr()
                as *const c_char,
        )
    };
    rc = unsafe {
        rtas_call_impl(
            b"ibm,get-system-parameter\0".as_ptr() as *const c_char,
            3,
            1,
            &[
                0,
                cpu_to_be32(rmo_start) as c_ulong,
                cpu_to_be32(rmo_end.wrapping_sub(rmo_start).wrapping_add(2)) as c_ulong,
            ],
            &[rets.as_mut_ptr()],
        )
    };
    unsafe { printf(b"rc: %d\n\0".as_ptr() as *const c_char, rc) };
    FAIL_IF!(rc != -EINVAL && rc != RTAS_UNKNOWN_OP);

    // Test a permitted call, fixed size, buffer end outside RMO
    // Expected: call returns -EINVAL
    unsafe {
        printf(
            b"Test a permitted call, fixed size, buffer end outside RMO... \0".as_ptr()
                as *const c_char,
        )
    };
    rc = unsafe {
        rtas_call_impl(
            b"ibm,configure-connector\0".as_ptr() as *const c_char,
            2,
            1,
            &[cpu_to_be32(rmo_end.wrapping_sub(4000)) as c_ulong, 0],
            &[rets.as_mut_ptr()],
        )
    };
    unsafe { printf(b"rc: %d\n\0".as_ptr() as *const c_char, rc) };
    FAIL_IF!(rc != -EINVAL && rc != RTAS_UNKNOWN_OP);

    0
}

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe { test_harness(test, b"rtas_filter\0".as_ptr() as *const c_char) }
}

fn main() {
    let rc = unsafe { main_impl(0, ptr::null_mut()) };
    std::process::exit(rc);
}
