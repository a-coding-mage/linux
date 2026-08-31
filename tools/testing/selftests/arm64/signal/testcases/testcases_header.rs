/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2019 ARM Limited */

/* C header dependencies translated as external Rust dependencies:
 * <stddef.h>, <stdio.h>, <stdbool.h>, <stdint.h>, <stdlib.h>,
 * <ucontext.h>, <signal.h>, and <asm/sigcontext.h>.
 */

use core::ffi::{c_char, c_int};
use core::mem::{size_of, size_of_val};

pub const FPSIMD_CTX: u32 = 1 << 0;
pub const SVE_CTX: u32 = 1 << 1;
pub const ZA_CTX: u32 = 1 << 2;
pub const EXTRA_CTX: u32 = 1 << 3;
pub const ZT_CTX: u32 = 1 << 4;
pub const FPMR_CTX: u32 = 1 << 5;
pub const GCS_CTX: u32 = 1 << 6;

pub const KSFT_BAD_MAGIC: u32 = 0xdeadbeef;

pub const HDR_SZ: usize = size_of::<_aarch64_ctx>();

#[repr(C)]
pub struct fake_sigframe {
    pub info: siginfo_t,
    pub uc: ucontext_t,
}

unsafe extern "C" {
    pub static mut stderr: *mut FILE;

    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn abort() -> !;

    pub fn validate_reserved(uc: *mut ucontext_t, resv_sz: usize, err: *mut *mut c_char) -> bool;

    pub fn get_starting_head(
        shead: *mut _aarch64_ctx,
        need_sz: usize,
        resv_sz: usize,
        offset: *mut usize,
    ) -> *mut _aarch64_ctx;
}

pub unsafe fn GET_UC_RESV_HEAD(uc: *mut ucontext_t) -> *mut _aarch64_ctx {
    unsafe { &mut (*uc).uc_mcontext.__reserved as *mut _ as *mut _aarch64_ctx }
}

pub unsafe fn GET_SF_RESV_HEAD(sf: *mut fake_sigframe) -> *mut _aarch64_ctx {
    unsafe { &mut (*sf).uc.uc_mcontext.__reserved as *mut _ as *mut _aarch64_ctx }
}

pub unsafe fn GET_SF_RESV_SIZE(sf: *const fake_sigframe) -> usize {
    unsafe { size_of_val(&(*sf).uc.uc_mcontext.__reserved) }
}

pub unsafe fn GET_BUF_RESV_HEAD(buf: *mut fake_sigframe) -> *mut _aarch64_ctx {
    unsafe { &mut (*buf).uc.uc_mcontext.__reserved as *mut _ as *mut _aarch64_ctx }
}

pub unsafe fn GET_BUF_RESV_SIZE(buf: *const fake_sigframe) -> usize {
    unsafe {
        size_of_val(&*buf) - size_of_val(&(*buf).uc) + size_of_val(&(*buf).uc.uc_mcontext.__reserved)
    }
}

pub unsafe fn GET_UCP_RESV_SIZE(ucp: *const ucontext_t) -> usize {
    unsafe { size_of_val(&(*ucp).uc_mcontext.__reserved) }
}

pub unsafe fn ASSERT_BAD_CONTEXT(uc: *mut ucontext_t) {
    let mut err: *mut c_char = core::ptr::null_mut();

    if unsafe { !validate_reserved(uc, GET_UCP_RESV_SIZE(uc), &mut err) } {
        if !err.is_null() {
            unsafe {
                fprintf(
                    stderr,
                    c"Using badly built context - ERR: %s\n".as_ptr(),
                    err,
                );
            }
        }
    } else {
        unsafe {
            abort();
        }
    }
}

pub unsafe fn ASSERT_GOOD_CONTEXT(uc: *mut ucontext_t) {
    let mut err: *mut c_char = core::ptr::null_mut();

    if unsafe { !validate_reserved(uc, GET_UCP_RESV_SIZE(uc), &mut err) } {
        if !err.is_null() {
            unsafe {
                fprintf(
                    stderr,
                    c"Detected BAD context - ERR: %s\n".as_ptr(),
                    err,
                );
            }
        }
        unsafe {
            abort();
        }
    } else {
        unsafe {
            fprintf(stderr, c"uc context validated.\n".as_ptr());
        }
    }
}

/*
 * A simple record-walker for __reserved area: it walks through assuming
 * only to find a proper struct __aarch64_ctx header descriptor.
 *
 * Instead it makes no assumptions on the content and ordering of the
 * records, any needed bounds checking must be enforced by the caller
 * if wanted: this way can be used by caller on any maliciously built bad
 * contexts.
 *
 * head->size accounts both for payload and header _aarch64_ctx size !
 */
pub unsafe fn GET_RESV_NEXT_HEAD(h: *mut _aarch64_ctx) -> *mut _aarch64_ctx {
    unsafe { (h as *mut c_char).add((*h).size as usize) as *mut _aarch64_ctx }
}

pub unsafe fn get_header(
    mut head: *mut _aarch64_ctx,
    magic: u32,
    resv_sz: usize,
    offset: *mut usize,
) -> *mut _aarch64_ctx {
    let mut offs: usize = 0;
    let mut found: *mut _aarch64_ctx = core::ptr::null_mut();

    if head.is_null() || resv_sz < HDR_SZ {
        return found;
    }

    while offs <= resv_sz - HDR_SZ
        && unsafe { (*head).magic != magic }
        && unsafe { (*head).magic != 0 }
    {
        unsafe {
            offs = offs.wrapping_add((*head).size as usize);
            head = GET_RESV_NEXT_HEAD(head);
        }
    }
    if unsafe { (*head).magic == magic } {
        found = head;
        if !offset.is_null() {
            unsafe {
                *offset = offs;
            }
        }
    }

    found
}

pub unsafe fn get_terminator(
    head: *mut _aarch64_ctx,
    resv_sz: usize,
    offset: *mut usize,
) -> *mut _aarch64_ctx {
    unsafe { get_header(head, 0, resv_sz, offset) }
}

pub unsafe fn write_terminator_record(tail: *mut _aarch64_ctx) {
    if !tail.is_null() {
        unsafe {
            (*tail).magic = 0;
            (*tail).size = 0;
        }
    }
}
