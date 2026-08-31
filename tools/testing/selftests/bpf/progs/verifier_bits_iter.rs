// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2024 Yafang Shao <laoar.shao@gmail.com> */

/* From:
 * #include "vmlinux.h"
 * #include <bpf/bpf_helpers.h>
 * #include <bpf/bpf_tracing.h>
 * #include "bpf_misc.h"
 * #include "task_kfunc_common.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;

unsafe extern "C" {
    type bpf_iter_bits;
    type bpf_iter_meta;
    type cgroup;

    #[link_name = "bpf_iter_bits_new"]
    fn bpf_iter_bits_new(
        it: *mut bpf_iter_bits,
        unsafe_ptr__ign: *const u64,
        nr_bits: u32,
    ) -> core::ffi::c_int;
    #[link_name = "bpf_iter_bits_next"]
    fn bpf_iter_bits_next(it: *mut bpf_iter_bits) -> *mut core::ffi::c_int;
    #[link_name = "bpf_iter_bits_destroy"]
    fn bpf_iter_bits_destroy(it: *mut bpf_iter_bits);
}

const EINVAL: core::ffi::c_int = 22;
const EFAULT: core::ffi::c_int = 14;

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [core::ffi::c_char; 4] = *b"GPL\0".cast();

#[unsafe(no_mangle)]
pub static mut bits_array: [u64; 511] = [0; 511];

#[unsafe(link_section = "iter.s/cgroup")]
/* __description("bits iter without destroy") */
/* __failure __msg("Unreleased reference") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn no_destroy(
    meta: *mut bpf_iter_meta,
    cgrp: *mut cgroup,
) -> core::ffi::c_int {
    let mut it: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::uninit();
    let mut data: u64 = 1;

    let _ = meta;
    let _ = cgrp;
    bpf_iter_bits_new(it.as_mut_ptr(), &mut data as *mut u64 as *const u64, 1);
    bpf_iter_bits_next(it.as_mut_ptr());
    return 0;
}

#[unsafe(link_section = "iter/cgroup")]
/* __description("uninitialized iter in ->next()") */
/* __failure __msg("expected an initialized iter_bits as R1") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn next_uninit(
    meta: *mut bpf_iter_meta,
    cgrp: *mut cgroup,
) -> core::ffi::c_int {
    let mut it: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::zeroed();

    let _ = meta;
    let _ = cgrp;
    bpf_iter_bits_next(it.as_mut_ptr());
    return 0;
}

#[unsafe(link_section = "iter/cgroup")]
/* __description("uninitialized iter in ->destroy()") */
/* __failure __msg("expected an initialized iter_bits as R1") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn destroy_uninit(
    meta: *mut bpf_iter_meta,
    cgrp: *mut cgroup,
) -> core::ffi::c_int {
    let mut it: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::zeroed();

    let _ = meta;
    let _ = cgrp;
    bpf_iter_bits_destroy(it.as_mut_ptr());
    return 0;
}

#[unsafe(link_section = "syscall")]
/* __description("null pointer") */
/* __success __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn null_pointer() -> core::ffi::c_int {
    let mut iter: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::uninit();
    let mut nr: core::ffi::c_int = 0;
    let mut bit: *mut core::ffi::c_int;
    let mut err: core::ffi::c_int;

    err = bpf_iter_bits_new(iter.as_mut_ptr(), core::ptr::null(), 1);
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    if err != -EINVAL {
        return 1;
    }

    err = bpf_iter_bits_new(iter.as_mut_ptr(), core::ptr::null(), 1);
    if err == 0 {
        loop {
            bit = bpf_iter_bits_next(iter.as_mut_ptr());
            if bit.is_null() {
                break;
            }
            nr += 1;
        }
    }
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    return nr;
}

#[unsafe(link_section = "syscall")]
/* __description("bits copy") */
/* __success __retval(10) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bits_copy() -> core::ffi::c_int {
    let mut data: u64 = 0xf7310; /* 4 + 3 + 2 + 1 + 0*/
    let mut nr: core::ffi::c_int = 0;
    let mut bit: *mut core::ffi::c_int;
    let mut iter: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::uninit();

    if bpf_iter_bits_new(iter.as_mut_ptr(), &mut data as *mut u64 as *const u64, 1) == 0 {
        loop {
            bit = bpf_iter_bits_next(iter.as_mut_ptr());
            if bit.is_null() {
                break;
            }
            nr += 1;
        }
    }
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    return nr;
}

#[unsafe(link_section = "syscall")]
/* __description("bits memalloc") */
/* __success __retval(64) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bits_memalloc() -> core::ffi::c_int {
    let mut data: [u64; 2] = core::mem::MaybeUninit::zeroed().assume_init();
    let mut nr: core::ffi::c_int = 0;
    let mut bit: *mut core::ffi::c_int;
    let mut iter: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::uninit();

    core::ptr::write_bytes(
        data.as_mut_ptr() as *mut core::ffi::c_void,
        0xf0,
        core::mem::size_of_val(&data),
    ); /* 4 * 16 */
    if bpf_iter_bits_new(iter.as_mut_ptr(), &data[0] as *const u64, data.len() as u32) == 0 {
        loop {
            bit = bpf_iter_bits_next(iter.as_mut_ptr());
            if bit.is_null() {
                break;
            }
            nr += 1;
        }
    }
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    return nr;
}

#[unsafe(link_section = "syscall")]
/* __description("bit index") */
/* __success __retval(8) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bit_index() -> core::ffi::c_int {
    let mut data: u64 = 0x100;
    let mut bit_idx: core::ffi::c_int = 0;
    let mut bit: *mut core::ffi::c_int;
    let mut iter: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::uninit();

    if bpf_iter_bits_new(iter.as_mut_ptr(), &mut data as *mut u64 as *const u64, 1) == 0 {
        loop {
            bit = bpf_iter_bits_next(iter.as_mut_ptr());
            if bit.is_null() {
                break;
            }
            if *bit == 0 {
                continue;
            }
            bit_idx = *bit;
        }
    }
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    return bit_idx;
}

#[unsafe(link_section = "syscall")]
/* __description("bits too big") */
/* __success __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bits_too_big() -> core::ffi::c_int {
    let mut data: [u64; 4] = core::mem::MaybeUninit::zeroed().assume_init();
    let mut nr: core::ffi::c_int = 0;
    let mut bit: *mut core::ffi::c_int;
    let mut iter: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::uninit();

    core::ptr::write_bytes(
        data.as_mut_ptr() as *mut core::ffi::c_void,
        0xff,
        core::mem::size_of_val(&data),
    );
    if bpf_iter_bits_new(iter.as_mut_ptr(), &data[0] as *const u64, 512) == 0 {
        /* Be greater than 511 */
        loop {
            bit = bpf_iter_bits_next(iter.as_mut_ptr());
            if bit.is_null() {
                break;
            }
            nr += 1;
        }
    }
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    return nr;
}

#[unsafe(link_section = "syscall")]
/* __description("fewer words") */
/* __success __retval(1) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fewer_words() -> core::ffi::c_int {
    let mut data: [u64; 2] = [0x1, 0xff];
    let mut nr: core::ffi::c_int = 0;
    let mut bit: *mut core::ffi::c_int;
    let mut iter: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::uninit();

    if bpf_iter_bits_new(iter.as_mut_ptr(), &data[0] as *const u64, 1) == 0 {
        loop {
            bit = bpf_iter_bits_next(iter.as_mut_ptr());
            if bit.is_null() {
                break;
            }
            nr += 1;
        }
    }
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    return nr;
}

#[unsafe(link_section = "syscall")]
/* __description("zero words") */
/* __success __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn zero_words() -> core::ffi::c_int {
    let mut data: [u64; 2] = [0x1, 0xff];
    let mut nr: core::ffi::c_int = 0;
    let mut bit: *mut core::ffi::c_int;
    let mut iter: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::uninit();

    if bpf_iter_bits_new(iter.as_mut_ptr(), &data[0] as *const u64, 0) == 0 {
        loop {
            bit = bpf_iter_bits_next(iter.as_mut_ptr());
            if bit.is_null() {
                break;
            }
            nr += 1;
        }
    }
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    return nr;
}

#[unsafe(link_section = "syscall")]
/* __description("huge words") */
/* __success __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn huge_words() -> core::ffi::c_int {
    let mut data: [u64; 8] = [0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1, 0x1];
    let mut nr: core::ffi::c_int = 0;
    let mut bit: *mut core::ffi::c_int;
    let mut iter: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::uninit();

    if bpf_iter_bits_new(iter.as_mut_ptr(), &data[0] as *const u64, 67108865) == 0 {
        loop {
            bit = bpf_iter_bits_next(iter.as_mut_ptr());
            if bit.is_null() {
                break;
            }
            nr += 1;
        }
    }
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    return nr;
}

#[unsafe(link_section = "syscall")]
/* __description("max words") */
/* __success __retval(4) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn max_words() -> core::ffi::c_int {
    let mut nr: core::ffi::c_int = 0;
    let mut bit: *mut core::ffi::c_int;
    let mut iter: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::uninit();

    bits_array[0] = (1_u64 << 63) | 1_u64;
    bits_array[510] = (1_u64 << 33) | (1_u64 << 32);

    if bpf_iter_bits_new(iter.as_mut_ptr(), bits_array.as_ptr(), 511) == 0 {
        loop {
            bit = bpf_iter_bits_next(iter.as_mut_ptr());
            if bit.is_null() {
                break;
            }
            if core::ptr::read_volatile(&nr) == 0 && *bit != 0 {
                break;
            }
            if core::ptr::read_volatile(&nr) == 2 && *bit != 32672 {
                break;
            }
            core::ptr::write_volatile(&mut nr, core::ptr::read_volatile(&nr) + 1);
        }
    }
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    return core::ptr::read_volatile(&nr);
}

#[unsafe(link_section = "syscall")]
/* __description("bad words") */
/* __success __retval(0) */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bad_words() -> core::ffi::c_int {
    let bad_addr: *mut core::ffi::c_void = (-4095_isize) as *mut core::ffi::c_void;
    let mut iter: core::mem::MaybeUninit<bpf_iter_bits> = core::mem::MaybeUninit::uninit();
    let mut nr: core::ffi::c_int;
    let mut bit: *mut core::ffi::c_int;
    let mut err: core::ffi::c_int;

    err = bpf_iter_bits_new(iter.as_mut_ptr(), bad_addr as *const u64, 1);
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    if err != -EFAULT {
        return 1;
    }

    nr = 0;
    if bpf_iter_bits_new(iter.as_mut_ptr(), bad_addr as *const u64, 1) == 0 {
        loop {
            bit = bpf_iter_bits_next(iter.as_mut_ptr());
            if bit.is_null() {
                break;
            }
            core::ptr::write_volatile(&mut nr, core::ptr::read_volatile(&nr) + 1);
        }
    }
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    if core::ptr::read_volatile(&nr) != 0 {
        return 2;
    }

    err = bpf_iter_bits_new(iter.as_mut_ptr(), bad_addr as *const u64, 4);
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    if err != -EFAULT {
        return 3;
    }

    nr = 0;
    if bpf_iter_bits_new(iter.as_mut_ptr(), bad_addr as *const u64, 4) == 0 {
        loop {
            bit = bpf_iter_bits_next(iter.as_mut_ptr());
            if bit.is_null() {
                break;
            }
            core::ptr::write_volatile(&mut nr, core::ptr::read_volatile(&nr) + 1);
        }
    }
    bpf_iter_bits_destroy(iter.as_mut_ptr());
    if core::ptr::read_volatile(&nr) != 0 {
        return 4;
    }

    return 0;
}
