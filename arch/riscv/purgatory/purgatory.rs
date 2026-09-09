// SPDX-License-Identifier: GPL-2.0-only
/*
 * purgatory: Runs between two kernels
 *
 * Copyright (C) 2022 Huawei Technologies Co, Ltd.
 *
 * Author: Li Zhengyu (lizhengyu3@huawei.com)
 *
 */

// Dependencies supplied by the kernel headers and other translation units.

#[link_section = ".kexec-purgatory"]
pub static mut purgatory_sha256_digest: [u8; SHA256_DIGEST_SIZE] = [0; SHA256_DIGEST_SIZE];

#[link_section = ".kexec-purgatory"]
pub static mut purgatory_sha_regions: [kexec_sha_region; KEXEC_SEGMENT_MAX] =
    unsafe { core::mem::zeroed() };

extern "C" {
    fn sha256_init(sctx: *mut sha256_ctx);
    fn sha256_update(sctx: *mut sha256_ctx, data: *const u8, len: usize);
    fn sha256_final(sctx: *mut sha256_ctx, digest: *mut u8);
    fn memcmp(lhs: *const u8, rhs: *const u8, len: usize) -> i32;
}

unsafe fn verify_sha256_digest() -> bool {
    let mut sctx: sha256_ctx = core::mem::zeroed();
    let mut digest = [0u8; SHA256_DIGEST_SIZE];

    sha256_init(&mut sctx);
    let ptr = purgatory_sha_regions.as_ptr();
    let end = ptr.add(purgatory_sha_regions.len());
    let mut current = ptr;
    while current < end {
        sha256_update(
            &mut sctx,
            (*current).start as *const u8,
            (*current).len,
        );
        current = current.add(1);
    }
    sha256_final(&mut sctx, digest.as_mut_ptr());

    memcmp(
        digest.as_ptr(),
        purgatory_sha256_digest.as_ptr(),
        core::mem::size_of_val(&digest),
    ) == 0
}

pub unsafe fn purgatory() {
    if !verify_sha256_digest() {
        loop {
            /* loop forever */
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
