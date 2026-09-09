/* SPDX-License-Identifier: GPL-2.0+ */

// Translated from the C header.  Dependency supplied by the surrounding tree:
// <asm/book3s/64/hash-pkey.h>

#[inline]
fn vmflag_to_pte_pkey_bits(vm_flags: vm_flags_t) -> u64 {
    if !mmu_has_feature(MMU_FTR_PKEY) {
        return 0x0u64;
    }

    if radix_enabled() {
        BUG!();
    }
    hash__vmflag_to_pte_pkey_bits(vm_flags)
}

#[inline]
fn pte_to_pkey_bits(pteflags: u64) -> u16 {
    if radix_enabled() {
        BUG!();
    }
    hash__pte_to_pkey_bits(pteflags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
