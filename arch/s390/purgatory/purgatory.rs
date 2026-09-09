// SPDX-License-Identifier: GPL-2.0
/*
 * Purgatory code running between two kernels.
 *
 * Copyright IBM Corp. 2018
 *
 * Author(s): Philipp Rudo <prudo@linux.vnet.ibm.com>
 */

// Dependencies supplied by the surrounding kernel build.

pub unsafe fn verify_sha256_digest() -> i32 {
    let mut ptr: *mut kexec_sha_region;
    let end: *mut kexec_sha_region;
    let mut digest: [u8; SHA256_DIGEST_SIZE] = [0; SHA256_DIGEST_SIZE];
    let mut sctx: sha256_ctx = core::mem::zeroed();

    sha256_init(&mut sctx);
    end = purgatory_sha_regions.add(ARRAY_SIZE);

    ptr = purgatory_sha_regions;
    while ptr < end {
        sha256_update(
            &mut sctx,
            (*ptr).start as *const u8,
            (*ptr).len,
        );
        ptr = ptr.add(1);
    }

    sha256_final(&mut sctx, digest.as_mut_ptr());

    if core::slice::from_raw_parts(digest.as_ptr(), digest.len())
        != core::slice::from_raw_parts(purgatory_sha256_digest, digest.len())
    {
        return 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
