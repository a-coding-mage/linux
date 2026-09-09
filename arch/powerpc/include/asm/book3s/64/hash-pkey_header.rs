/* SPDX-License-Identifier: GPL-2.0 */

/*  We use key 3 for KERNEL */
pub const HASH_DEFAULT_KERNEL_KEY: u64 = HPTE_R_KEY_BIT0 | HPTE_R_KEY_BIT1;

#[inline]
pub fn hash__vmflag_to_pte_pkey_bits(vm_flags: u64) -> u64 {
    ((if (vm_flags & VM_PKEY_BIT0) != 0 { H_PTE_PKEY_BIT0 } else { 0x0_u64 }) |
        (if (vm_flags & VM_PKEY_BIT1) != 0 { H_PTE_PKEY_BIT1 } else { 0x0_u64 }) |
        (if (vm_flags & VM_PKEY_BIT2) != 0 { H_PTE_PKEY_BIT2 } else { 0x0_u64 }) |
        (if (vm_flags & VM_PKEY_BIT3) != 0 { H_PTE_PKEY_BIT3 } else { 0x0_u64 }) |
        (if (vm_flags & VM_PKEY_BIT4) != 0 { H_PTE_PKEY_BIT4 } else { 0x0_u64 }))
}

#[inline]
pub fn pte_to_hpte_pkey_bits(pteflags: u64, flags: usize) -> usize {
    let pte_pkey: usize =
        ((if (pteflags & H_PTE_PKEY_BIT4) != 0 { HPTE_R_KEY_BIT4 } else { 0x0_usize }) |
            (if (pteflags & H_PTE_PKEY_BIT3) != 0 { HPTE_R_KEY_BIT3 } else { 0x0_usize }) |
            (if (pteflags & H_PTE_PKEY_BIT2) != 0 { HPTE_R_KEY_BIT2 } else { 0x0_usize }) |
            (if (pteflags & H_PTE_PKEY_BIT1) != 0 { HPTE_R_KEY_BIT1 } else { 0x0_usize }) |
            (if (pteflags & H_PTE_PKEY_BIT0) != 0 { HPTE_R_KEY_BIT0 } else { 0x0_usize }));

    if mmu_has_feature(MMU_FTR_KUAP) || mmu_has_feature(MMU_FTR_BOOK3S_KUEP) {
        if (pte_pkey == 0) && ((flags & HPTE_USE_KERNEL_KEY) != 0) {
            return HASH_DEFAULT_KERNEL_KEY as usize;
        }
    }

    pte_pkey
}

#[inline]
pub fn hash__pte_to_pkey_bits(pteflags: u64) -> u16 {
    ((if (pteflags & H_PTE_PKEY_BIT4) != 0 { 0x10_u16 } else { 0x0_u16 }) |
        (if (pteflags & H_PTE_PKEY_BIT3) != 0 { 0x8_u16 } else { 0x0_u16 }) |
        (if (pteflags & H_PTE_PKEY_BIT2) != 0 { 0x4_u16 } else { 0x0_u16 }) |
        (if (pteflags & H_PTE_PKEY_BIT1) != 0 { 0x2_u16 } else { 0x0_u16 }) |
        (if (pteflags & H_PTE_PKEY_BIT0) != 0 { 0x1_u16 } else { 0x0_u16 }))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
