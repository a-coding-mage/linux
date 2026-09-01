// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const SIZE: c_ulong = 256;
const ITERATIONS: c_int = 10000;

const LARGE_SIZE: c_ulong = 5 * 1024;
const LARGE_ITERATIONS: c_int = 1000;
const LARGE_MAX_OFFSET: c_ulong = 32;
const LARGE_SIZE_START: c_ulong = 4096;

/* This is big enough to fit LARGE_SIZE and works on 4K & 64K kernels */
const MAP_SIZE: c_ulong = 64 * 1024;

const MAX_OFFSET_DIFF_S1_S2: c_ulong = 48;

static mut vmx_count: c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn enter_vmx_ops() -> c_int {
    vmx_count += 1;
    1
}

#[no_mangle]
pub unsafe extern "C" fn exit_vmx_ops() {
    vmx_count -= 1;
}

unsafe extern "C" {
    fn test_memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn random() -> c_long;
    fn srandom(seed: u32);
    fn time(tloc: *mut c_long) -> c_long;
    fn printf(format: *const c_char, ...) -> c_int;
    fn abort() -> !;

    fn have_hwcap2(feature: c_ulong) -> bool;
    fn test_harness_set_timeout(timeout: c_ulong);
    fn test_harness(testcases: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

const PPC_FEATURE2_ARCH_2_07: c_ulong = 0x8000_0000;

unsafe fn fail_if(condition: bool) {
    if condition {
        abort();
    }
}

unsafe fn skip_if(condition: bool) {
    if condition {
        std::process::exit(0);
    }
}

/* test all offsets and lengths */
unsafe fn test_one(
    s1: *mut c_char,
    s2: *mut c_char,
    max_offset: c_ulong,
    size_start: c_ulong,
    max_size: c_ulong,
) {
    let mut offset: c_ulong = 0;

    while offset < max_offset {
        let mut size: c_ulong = size_start;

        while size < max_size - offset {
            let x: c_int;
            let y: c_int;
            let mut i: c_ulong;

            y = memcmp(
                s1.add(offset as usize) as *const c_void,
                s2.add(offset as usize) as *const c_void,
                size as usize,
            );
            x = test_memcmp(
                s1.add(offset as usize) as *const c_void,
                s2.add(offset as usize) as *const c_void,
                size as usize,
            );

            if ((x ^ y) < 0) && /* Trick to compare sign */
                ((x | y) != 0)
            {
                /* check for zero */
                printf(
                    b"memcmp returned %d, should have returned %d (offset %ld size %ld)\n\0"
                        .as_ptr() as *const c_char,
                    x,
                    y,
                    offset,
                    size,
                );

                i = offset;
                while i < offset + size {
                    printf(
                        b"%02x \0".as_ptr() as *const c_char,
                        *s1.add(i as usize) as c_int,
                    );
                    i += 1;
                }
                printf(b"\n\0".as_ptr() as *const c_char);

                i = offset;
                while i < offset + size {
                    printf(
                        b"%02x \0".as_ptr() as *const c_char,
                        *s2.add(i as usize) as c_int,
                    );
                    i += 1;
                }
                printf(b"\n\0".as_ptr() as *const c_char);
                abort();
            }

            if vmx_count != 0 {
                printf(
                    b"vmx enter/exit not paired.(offset:%ld size:%ld s1:%p s2:%p vc:%d\n\0"
                        .as_ptr() as *const c_char,
                    offset,
                    size,
                    s1,
                    s2,
                    vmx_count,
                );
                printf(b"\n\0".as_ptr() as *const c_char);
                abort();
            }

            size += 1;
        }

        offset += 1;
    }
}

unsafe fn testcase(islarge: bool) -> c_int {
    let mut i: c_ulong;
    let comp_size: c_ulong;
    let alloc_size: c_ulong;
    let p: *mut c_char;
    let s1: *mut c_char;
    let s2: *mut c_char;
    let iterations: c_int;

    comp_size = if islarge { LARGE_SIZE } else { SIZE };
    alloc_size = comp_size + MAX_OFFSET_DIFF_S1_S2;
    iterations = if islarge {
        LARGE_ITERATIONS
    } else {
        ITERATIONS
    };

    p = mmap(
        core::ptr::null_mut(),
        (4 * MAP_SIZE) as usize,
        PROT_READ | PROT_WRITE,
        MAP_ANONYMOUS | MAP_PRIVATE,
        -1,
        0,
    ) as *mut c_char;
    fail_if(p == MAP_FAILED as *mut c_char);

    /* Put s1/s2 at the end of a page */
    s1 = p.add((MAP_SIZE - alloc_size) as usize);
    s2 = p.add((3 * MAP_SIZE - alloc_size) as usize);

    /* And unmap the subsequent page to force a fault if we overread */
    munmap(p.add(MAP_SIZE as usize) as *mut c_void, MAP_SIZE as usize);
    munmap(
        p.add((3 * MAP_SIZE) as usize) as *mut c_void,
        MAP_SIZE as usize,
    );

    srandom(time(core::ptr::null_mut()) as u32);

    i = 0;
    while i < iterations as c_ulong {
        let mut j: c_ulong;
        let change: c_ulong;
        let mut rand_s1: *mut c_char = s1;
        let mut rand_s2: *mut c_char = s2;

        j = 0;
        while j < alloc_size {
            *s1.add(j as usize) = random() as c_char;
            j += 1;
        }

        rand_s1 = rand_s1.add((random() as c_ulong % MAX_OFFSET_DIFF_S1_S2) as usize);
        rand_s2 = rand_s2.add((random() as c_ulong % MAX_OFFSET_DIFF_S1_S2) as usize);
        memcpy(
            rand_s2 as *mut c_void,
            rand_s1 as *const c_void,
            comp_size as usize,
        );

        /* change one byte */
        change = random() as c_ulong % comp_size;
        *rand_s2.add(change as usize) = (random() & 0xff) as c_char;

        if islarge {
            test_one(
                rand_s1,
                rand_s2,
                LARGE_MAX_OFFSET,
                LARGE_SIZE_START,
                comp_size,
            );
        } else {
            test_one(rand_s1, rand_s2, SIZE, 0, comp_size);
        }

        i += 1;
    }

    srandom(time(core::ptr::null_mut()) as u32);

    i = 0;
    while i < iterations as c_ulong {
        let mut j: c_ulong;
        let mut change: c_ulong;
        let mut rand_s1: *mut c_char = s1;
        let mut rand_s2: *mut c_char = s2;

        j = 0;
        while j < alloc_size {
            *s1.add(j as usize) = random() as c_char;
            j += 1;
        }

        rand_s1 = rand_s1.add((random() as c_ulong % MAX_OFFSET_DIFF_S1_S2) as usize);
        rand_s2 = rand_s2.add((random() as c_ulong % MAX_OFFSET_DIFF_S1_S2) as usize);
        memcpy(
            rand_s2 as *mut c_void,
            rand_s1 as *const c_void,
            comp_size as usize,
        );

        /* change multiple bytes, 1/8 of total */
        j = 0;
        while j < comp_size / 8 {
            change = random() as c_ulong % comp_size;
            *s2.add(change as usize) = (random() & 0xff) as c_char;
            j += 1;
        }

        if islarge {
            test_one(
                rand_s1,
                rand_s2,
                LARGE_MAX_OFFSET,
                LARGE_SIZE_START,
                comp_size,
            );
        } else {
            test_one(rand_s1, rand_s2, SIZE, 0, comp_size);
        }

        i += 1;
    }

    0
}

unsafe extern "C" fn testcases() -> c_int {
    // Original C condition: #ifdef __powerpc64__
    #[cfg(target_arch = "powerpc64")]
    {
        // vcmpequd used in memcmp_64.S is v2.07
        skip_if(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));
    }

    testcase(false);
    testcase(true);
    0
}

fn main() {
    unsafe {
        test_harness_set_timeout(300);
        std::process::exit(test_harness(
            testcases,
            b"memcmp\0".as_ptr() as *const c_char,
        ));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
