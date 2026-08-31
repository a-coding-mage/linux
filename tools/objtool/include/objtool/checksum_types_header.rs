/* SPDX-License-Identifier: GPL-2.0 */

#[repr(C)]
pub struct sym_checksum {
    pub addr: u64,
    pub checksum: u64,
}

/*
 * C conditional:
 * #ifdef BUILD_KLP
 *
 * Depends on <xxhash.h> for XXH3_state_t and XXH64_hash_t.
 */
#[cfg(BUILD_KLP)]
#[repr(C)]
pub struct checksum {
    pub state: *mut XXH3_state_t,
    pub checksum: XXH64_hash_t,
}

/*
 * C conditional:
 * #else
 */
#[cfg(not(BUILD_KLP))]
pub struct checksum {}
