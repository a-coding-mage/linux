// SPDX-License-Identifier: GPL-2.0
// C dependencies from the original file:
// <malloc.h>, <string.h>, <stdlib.h>, <stdbool.h>, and "utils.h".

use core::ffi::{c_char, c_int, c_ulong, c_void};

const MAX_LEN: c_ulong = 8192;
const MAX_OFFSET: c_ulong = 16;
const MIN_REDZONE: c_ulong = 128;
const BUFLEN: c_ulong = MAX_LEN + MAX_OFFSET + 2 * MIN_REDZONE;
const POISON: c_int = 0xa5;

#[cfg(VMX_TEST)]
const VMX_COPY_THRESHOLD: c_ulong = 3328;

const PPC_FEATURE_HAS_ALTIVEC: c_ulong = 0x1000_0000;

unsafe extern "C" {
    fn COPY_LOOP(to: *mut c_void, from: *const c_void, size: c_ulong) -> c_ulong;

    fn memalign(alignment: usize, size: usize) -> *mut c_void;
    fn malloc(size: usize) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn abort() -> !;
    fn exit(status: c_int) -> !;

    static mut stderr: *mut c_void;

    fn test_harness(test_function: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    #[cfg(VMX_TEST)]
    fn have_hwcap(feature: c_ulong) -> bool;
}

unsafe extern "C" fn do_one(
    src: *mut c_char,
    dst: *mut c_char,
    src_off: c_ulong,
    dst_off: c_ulong,
    len: c_ulong,
    redzone: *mut c_void,
    fill: *mut c_void,
) {
    let srcp: *mut c_char = src.add((MIN_REDZONE + src_off) as usize);
    let dstp: *mut c_char = dst.add((MIN_REDZONE + dst_off) as usize);
    let ret: c_ulong;
    let mut i: c_ulong;

    memset(src as *mut c_void, POISON, BUFLEN as usize);
    memset(dst as *mut c_void, POISON, BUFLEN as usize);
    memcpy(srcp as *mut c_void, fill as *const c_void, len as usize);

    ret = COPY_LOOP(dstp as *mut c_void, srcp as *const c_void, len);
    if ret != 0 && ret != dstp as c_ulong {
        printf(
            c"(%p,%p,%ld) returned %ld\n".as_ptr(),
            dstp,
            srcp,
            len,
            ret,
        );
        abort();
    }

    if memcmp(dstp as *const c_void, srcp as *const c_void, len as usize) != 0 {
        printf(c"(%p,%p,%ld) miscompare\n".as_ptr(), dstp, srcp, len);
        printf(c"src: ".as_ptr());
        i = 0;
        while i < len {
            printf(c"%02x ".as_ptr(), *srcp.add(i as usize) as c_int);
            i += 1;
        }
        printf(c"\ndst: ".as_ptr());
        i = 0;
        while i < len {
            printf(c"%02x ".as_ptr(), *dstp.add(i as usize) as c_int);
            i += 1;
        }
        printf(c"\n".as_ptr());
        abort();
    }

    if memcmp(
        dst as *const c_void,
        redzone as *const c_void,
        dstp.offset_from(dst) as usize,
    ) != 0
    {
        printf(
            c"(%p,%p,%ld) redzone before corrupted\n".as_ptr(),
            dstp,
            srcp,
            len,
        );
        abort();
    }

    if memcmp(
        dstp.add(len as usize) as *const c_void,
        redzone as *const c_void,
        dst.add(BUFLEN as usize).offset_from(dstp.add(len as usize)) as usize,
    ) != 0
    {
        printf(
            c"(%p,%p,%ld) redzone after corrupted\n".as_ptr(),
            dstp,
            srcp,
            len,
        );
        abort();
    }
}

unsafe extern "C" fn test_copy_loop() -> c_int {
    let src: *mut c_char;
    let dst: *mut c_char;
    let redzone: *mut c_char;
    let fill: *mut c_char;
    let mut len: c_ulong;
    let mut src_off: c_ulong;
    let mut dst_off: c_ulong;
    let mut i: c_ulong;

    src = memalign(BUFLEN as usize, BUFLEN as usize) as *mut c_char;
    dst = memalign(BUFLEN as usize, BUFLEN as usize) as *mut c_char;
    redzone = malloc(BUFLEN as usize) as *mut c_char;
    fill = malloc(BUFLEN as usize) as *mut c_char;

    if src.is_null() || dst.is_null() || redzone.is_null() || fill.is_null() {
        fprintf(stderr, c"malloc failed\n".as_ptr());
        exit(1);
    }

    memset(redzone as *mut c_void, POISON, BUFLEN as usize);

    /* Fill with sequential bytes */
    i = 0;
    while i < BUFLEN {
        *fill.add(i as usize) = (i & 0xff) as c_char;
        i += 1;
    }

    #[cfg(VMX_TEST)]
    {
        /* Force sizes above kernel VMX threshold (3328) */
        len = VMX_COPY_THRESHOLD + 1;
    }
    #[cfg(not(VMX_TEST))]
    {
        len = 1;
    }

    while len < MAX_LEN {
        src_off = 0;
        while src_off < MAX_OFFSET {
            dst_off = 0;
            while dst_off < MAX_OFFSET {
                do_one(
                    src,
                    dst,
                    src_off,
                    dst_off,
                    len,
                    redzone as *mut c_void,
                    fill as *mut c_void,
                );
                dst_off += 1;
            }
            src_off += 1;
        }
        len += 1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    #[cfg(VMX_TEST)]
    {
        /* Skip if Altivec not present */
        if !have_hwcap(PPC_FEATURE_HAS_ALTIVEC) {
            printf(c"ALTIVEC not supported\n".as_ptr());
            return 4;
        }
    }

    test_harness(test_copy_loop, c"COPY_LOOP".as_ptr())
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
