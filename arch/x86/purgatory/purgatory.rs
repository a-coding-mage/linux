// SPDX-License-Identifier: GPL-2.0-only
/*
 * purgatory: Runs between two kernels
 *
 * Copyright (C) 2014 Red Hat Inc.
 *
 * Author:
 *       Vivek Goyal <vgoyal@redhat.com>
 */

// C headers:
// linux/bug.h, linux/kernel.h, linux/types.h, crypto/sha2.h,
// asm/purgatory.h, ../boot/compressed/error.h, ../boot/string.h

extern "C" {
    fn sha256_init(ctx: *mut sha256_ctx);
    fn sha256_update(ctx: *mut sha256_ctx, data: *const u8, len: usize);
    fn sha256_final(ctx: *mut sha256_ctx, digest: *mut u8);
    fn memcmp(s1: *const core::ffi::c_void, s2: *const core::ffi::c_void, n: usize) -> i32;
}

// Supplied by the corresponding kernel headers.
#[repr(C)]
pub struct sha256_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kexec_sha_region {
    pub start: usize,
    pub len: usize,
}

pub const SHA256_DIGEST_SIZE: usize = 32;
pub const KEXEC_SEGMENT_MAX: usize = 16;

#[unsafe(link_section = ".kexec-purgatory")]
pub static mut purgatory_sha256_digest: [u8; SHA256_DIGEST_SIZE] =
    [0; SHA256_DIGEST_SIZE];

#[unsafe(link_section = ".kexec-purgatory")]
pub static mut purgatory_sha_regions: [kexec_sha_region; KEXEC_SEGMENT_MAX] =
    [kexec_sha_region { start: 0, len: 0 }; KEXEC_SEGMENT_MAX];

unsafe fn verify_sha256_digest() -> bool {
    let mut sctx: sha256_ctx = sha256_ctx { _private: [] };
    let mut digest = [0u8; SHA256_DIGEST_SIZE];

    sha256_init(&mut sctx);

    let mut ptr = purgatory_sha_regions.as_ptr();
    let end = ptr.add(purgatory_sha_regions.len());

    while ptr < end {
        let region = &*ptr;
        sha256_update(&mut sctx, region.start as *const u8, region.len);
        ptr = ptr.add(1);
    }

    sha256_final(&mut sctx, digest.as_mut_ptr());

    memcmp(
        digest.as_ptr() as *const core::ffi::c_void,
        purgatory_sha256_digest.as_ptr() as *const core::ffi::c_void,
        digest.len(),
    ) == 0
}

pub unsafe fn purgatory() {
    if !verify_sha256_digest() {
        /* loop forever */
        loop {}
    }
}

/*
 * Defined in order to reuse memcpy() and memset() from
 * arch/x86/boot/compressed/string.c
 */
pub unsafe fn warn(_msg: *const core::ffi::c_char) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
