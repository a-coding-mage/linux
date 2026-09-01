// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022-2024 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;

const AT_HWCAP: c_ulong = 16;

#[cfg(target_arch = "aarch64")]
const HWCAP_ASIMD: c_ulong = 1 << 1;

#[cfg(target_arch = "s390x")]
const HWCAP_S390_VXRS: c_ulong = 1 << 11;

unsafe extern "C" {
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> isize;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_test_result_skip(msg: *const c_char, ...);
    fn ksft_test_result_pass(msg: *const c_char, ...);
    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
    fn ksft_exit_pass() -> !;
    fn ksft_finished() -> !;
}

#[cfg(target_arch = "aarch64")]
fn cpu_has_capabilities() -> bool {
    unsafe { (getauxval(AT_HWCAP) & HWCAP_ASIMD) != 0 }
}

#[cfg(target_arch = "s390x")]
fn cpu_has_capabilities() -> bool {
    unsafe { (getauxval(AT_HWCAP) & HWCAP_S390_VXRS) != 0 }
}

#[cfg(not(any(target_arch = "aarch64", target_arch = "s390x")))]
fn cpu_has_capabilities() -> bool {
    true
}

fn rol32(word: u32, shift: c_uint) -> u32 {
    (word << (shift & 31)) | (word >> ((!shift).wrapping_add(1) & 31))
}

unsafe fn put_unaligned_le32(value: u32, dst: *mut u8) {
    ptr::write_unaligned(dst as *mut u32, value.to_le());
}

unsafe fn reference_chacha20_blocks(
    mut dst_bytes: *mut u8,
    key: *const u32,
    counter: *mut u32,
    mut nblocks: size_t,
) {
    let mut s: [u32; 16] = [
        0x61707865u32,
        0x3320646eu32,
        0x79622d32u32,
        0x6b206574u32,
        *key.add(0),
        *key.add(1),
        *key.add(2),
        *key.add(3),
        *key.add(4),
        *key.add(5),
        *key.add(6),
        *key.add(7),
        *counter.add(0),
        *counter.add(1),
        0,
        0,
    ];

    while {
        let old = nblocks;
        nblocks = nblocks.wrapping_sub(1);
        old != 0
    } {
        let mut x: [u32; 16] = [0; 16];
        ptr::copy_nonoverlapping(s.as_ptr(), x.as_mut_ptr(), x.len());
        let mut r: c_uint = 0;
        while r < 20 {
            macro_rules! QR {
                ($a:expr, $b:expr, $c:expr, $d:expr) => {{
                    x[$a] = x[$a].wrapping_add(x[$b]);
                    x[$d] = rol32(x[$d] ^ x[$a], 16);
                    x[$c] = x[$c].wrapping_add(x[$d]);
                    x[$b] = rol32(x[$b] ^ x[$c], 12);
                    x[$a] = x[$a].wrapping_add(x[$b]);
                    x[$d] = rol32(x[$d] ^ x[$a], 8);
                    x[$c] = x[$c].wrapping_add(x[$d]);
                    x[$b] = rol32(x[$b] ^ x[$c], 7);
                }};
            }

            QR!(0, 4, 8, 12);
            QR!(1, 5, 9, 13);
            QR!(2, 6, 10, 14);
            QR!(3, 7, 11, 15);
            QR!(0, 5, 10, 15);
            QR!(1, 6, 11, 12);
            QR!(2, 7, 8, 13);
            QR!(3, 4, 9, 14);
            r += 2;
        }
        let mut i: c_uint = 0;
        while i < 16 {
            put_unaligned_le32(
                x[i as usize].wrapping_add(s[i as usize]),
                dst_bytes,
            );
            dst_bytes = dst_bytes.add(size_of::<u32>());
            i += 1;
        }
        s[12] = s[12].wrapping_add(1);
        if s[12] == 0 {
            s[13] = s[13].wrapping_add(1);
        }
    }
    *counter.add(0) = s[12];
    *counter.add(1) = s[13];
}

// C source declares this as weak so architectures may override it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __arch_chacha20_blocks_nostack(
    _dst_bytes: *mut u8,
    _key: *const u32,
    _counter: *mut u32,
    _nblocks: size_t,
) {
    ksft_test_result_skip(c"Not implemented on architecture\n".as_ptr());
    ksft_finished();
}

const TRIALS: c_uint = 1000;
const BLOCKS: c_uint = 128;
const BLOCK_SIZE: c_uint = 64;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut key: [u32; 8] = [0; 8];
    let mut counter1: [u32; 2] = [0; 2];
    let mut counter2: [u32; 2] = [0; 2];
    let mut output1: [u8; (BLOCK_SIZE * BLOCKS) as usize] = [0; (BLOCK_SIZE * BLOCKS) as usize];
    let mut output2: [u8; (BLOCK_SIZE * BLOCKS) as usize] = [0; (BLOCK_SIZE * BLOCKS) as usize];

    ksft_print_header();
    if !cpu_has_capabilities() {
        ksft_exit_skip(c"Required CPU capabilities missing\n".as_ptr());
    }
    ksft_set_plan(1);

    let mut trial: c_uint = 0;
    while trial < TRIALS {
        if getrandom(
            key.as_mut_ptr() as *mut c_void,
            size_of::<[u32; 8]>(),
            0,
        ) != size_of::<[u32; 8]>() as isize
        {
            ksft_exit_skip(c"getrandom() failed unexpectedly\n".as_ptr());
        }
        memset(
            counter1.as_mut_ptr() as *mut c_void,
            0,
            size_of::<[u32; 2]>(),
        );
        reference_chacha20_blocks(
            output1.as_mut_ptr(),
            key.as_ptr(),
            counter1.as_mut_ptr(),
            BLOCKS as size_t,
        );
        let mut split: c_uint = 0;
        while split < BLOCKS {
            memset(
                output2.as_mut_ptr() as *mut c_void,
                b'X' as c_int,
                size_of::<[u8; (BLOCK_SIZE * BLOCKS) as usize]>(),
            );
            memset(
                counter2.as_mut_ptr() as *mut c_void,
                0,
                size_of::<[u32; 2]>(),
            );
            if split != 0 {
                __arch_chacha20_blocks_nostack(
                    output2.as_mut_ptr(),
                    key.as_ptr(),
                    counter2.as_mut_ptr(),
                    split as size_t,
                );
            }
            __arch_chacha20_blocks_nostack(
                output2.as_mut_ptr().add((split * BLOCK_SIZE) as usize),
                key.as_ptr(),
                counter2.as_mut_ptr(),
                (BLOCKS - split) as size_t,
            );
            if memcmp(
                output1.as_ptr() as *const c_void,
                output2.as_ptr() as *const c_void,
                size_of::<[u8; (BLOCK_SIZE * BLOCKS) as usize]>(),
            ) != 0
            {
                ksft_exit_fail_msg(
                    c"Main loop outputs do not match on trial %u, split %u\n".as_ptr(),
                    trial,
                    split,
                );
            }
            if memcmp(
                counter1.as_ptr() as *const c_void,
                counter2.as_ptr() as *const c_void,
                size_of::<[u32; 2]>(),
            ) != 0
            {
                ksft_exit_fail_msg(
                    c"Main loop counters do not match on trial %u, split %u\n".as_ptr(),
                    trial,
                    split,
                );
            }
            split += 1;
        }
        trial += 1;
    }
    memset(
        counter1.as_mut_ptr() as *mut c_void,
        0,
        size_of::<[u32; 2]>(),
    );
    counter1[0] = (0u32).wrapping_sub(BLOCKS).wrapping_add(2);
    memset(
        counter2.as_mut_ptr() as *mut c_void,
        0,
        size_of::<[u32; 2]>(),
    );
    counter2[0] = (0u32).wrapping_sub(BLOCKS).wrapping_add(2);

    reference_chacha20_blocks(
        output1.as_mut_ptr(),
        key.as_ptr(),
        counter1.as_mut_ptr(),
        BLOCKS as size_t,
    );
    __arch_chacha20_blocks_nostack(
        output2.as_mut_ptr(),
        key.as_ptr(),
        counter2.as_mut_ptr(),
        BLOCKS as size_t,
    );
    if memcmp(
        output1.as_ptr() as *const c_void,
        output2.as_ptr() as *const c_void,
        size_of::<[u8; (BLOCK_SIZE * BLOCKS) as usize]>(),
    ) != 0
    {
        ksft_exit_fail_msg(c"Block limit outputs do not match after first round\n".as_ptr());
    }
    if memcmp(
        counter1.as_ptr() as *const c_void,
        counter2.as_ptr() as *const c_void,
        size_of::<[u32; 2]>(),
    ) != 0
    {
        ksft_exit_fail_msg(c"Block limit counters do not match after first round\n".as_ptr());
    }

    reference_chacha20_blocks(
        output1.as_mut_ptr(),
        key.as_ptr(),
        counter1.as_mut_ptr(),
        BLOCKS as size_t,
    );
    __arch_chacha20_blocks_nostack(
        output2.as_mut_ptr(),
        key.as_ptr(),
        counter2.as_mut_ptr(),
        BLOCKS as size_t,
    );
    if memcmp(
        output1.as_ptr() as *const c_void,
        output2.as_ptr() as *const c_void,
        size_of::<[u8; (BLOCK_SIZE * BLOCKS) as usize]>(),
    ) != 0
    {
        ksft_exit_fail_msg(c"Block limit outputs do not match after second round\n".as_ptr());
    }
    if memcmp(
        counter1.as_ptr() as *const c_void,
        counter2.as_ptr() as *const c_void,
        size_of::<[u32; 2]>(),
    ) != 0
    {
        ksft_exit_fail_msg(c"Block limit counters do not match after second round\n".as_ptr());
    }

    ksft_test_result_pass(c"chacha: PASS\n".as_ptr());
    ksft_exit_pass();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
