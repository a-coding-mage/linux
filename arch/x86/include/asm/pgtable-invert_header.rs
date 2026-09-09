/* SPDX-License-Identifier: GPL-2.0 */

/*
 * A clear pte value is special, and doesn't get inverted.
 *
 * Note that even users that only pass a pgprot_t (rather
 * than a full pte) won't trigger the special zero case,
 * because even PAGE_NONE has _PAGE_PROTNONE | _PAGE_ACCESSED
 * set. So the all zero case really is limited to just the
 * cleared page table entry case.
 */
#[inline]
fn __pte_needs_invert(val: u64) -> bool {
    val != 0 && (val & _PAGE_PRESENT) == 0
}

/* Get a mask to xor with the page table entry to get the correct pfn. */
#[inline]
fn protnone_mask(val: u64) -> u64 {
    if __pte_needs_invert(val) { !0u64 } else { 0 }
}

#[inline]
fn flip_protnone_guard(oldval: u64, mut val: u64, mask: u64) -> u64 {
    /*
     * When a PTE transitions from NONE to !NONE or vice-versa
     * invert the PFN part to stop speculation.
     * pte_pfn undoes this when needed.
     */
    if __pte_needs_invert(oldval) != __pte_needs_invert(val) {
        val = (val & !mask) | (!val & mask);
    }
    val
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
