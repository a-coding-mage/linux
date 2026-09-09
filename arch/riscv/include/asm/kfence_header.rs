/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the RISC-V KFENCE header.
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not defined here.

#[inline]
pub fn arch_kfence_init_pool() -> bool {
    true
}

#[inline]
pub unsafe fn kfence_protect_page(addr: usize, protect: bool) -> bool {
    let pte: *mut pte_t = virt_to_kpte(addr);

    if protect {
        set_pte(
            pte,
            __pte(pte_val(ptep_get(pte)) & !(_PAGE_PRESENT)),
        );
    } else {
        set_pte(
            pte,
            __pte(pte_val(ptep_get(pte)) | _PAGE_PRESENT),
        );
        mark_new_valid_map();
    }

    preempt_disable();
    local_flush_tlb_kernel_range(addr, addr + PAGE_SIZE);
    preempt_enable();

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
