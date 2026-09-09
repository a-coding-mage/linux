/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct kexec_sha_region {
    pub start: usize,
    pub len: usize,
}

/*
 * These forward declarations serve two purposes:
 *
 * 1) Make sparse happy when checking arch/purgatory
 * 2) Document that these are required to be global so the symbol
 *    lookup in kexec works
 */
extern "C" {
    pub static mut purgatory_sha_regions: [kexec_sha_region; KEXEC_SEGMENT_MAX];
    pub static mut purgatory_sha256_digest: [u8; SHA256_DIGEST_SIZE];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
