/*
 * Copyright IBM Corporation, 2015
 * Author Aneesh Kumar K.V <aneesh.kumar@linux.ibm.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2 of the GNU Lesser General Public License
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it would be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 */

// Dependencies are supplied by the surrounding kernel translation.

/*
 * Return true, if the entry has a slot value which
 * the software considers as invalid.
 */
#[inline]
unsafe fn hpte_soft_invalid(hidx: usize) -> bool {
    (hidx & 0xfusize) == 0xfusize
}

/*
 * index from 0 - 15
 */
pub unsafe fn __rpte_sub_valid(rpte: real_pte_t, index: usize) -> bool {
    !hpte_soft_invalid(__rpte_to_hidx(rpte, index))
}

pub unsafe fn __hash_page_4K(
    ea: usize,
    access: usize,
    vsid: usize,
    ptep: *mut pte_t,
    trap: usize,
    flags: usize,
    ssize: i32,
    subpg_prot: i32,
) -> i32 {
    let mut rpte: real_pte_t;
    let mut hpte_group: usize;
    let mut subpg_index: u32;
    let mut rflags: usize;
    let mut pa: usize;
    let mut old_pte: usize;
    let mut new_pte: usize;
    let mut subpg_pte: usize;
    let mut vpn: usize;
    let mut hash: usize;
    let mut slot: usize;
    let mut gslot: usize;
    let shift: usize = mmu_psize_defs[MMU_PAGE_4K].shift;

    loop {
        let pte: pte_t = READ_ONCE(*ptep);
        old_pte = pte_val(pte);
        if unlikely(old_pte & H_PAGE_BUSY) {
            return 0;
        }
        if unlikely(!check_pte_access(access, old_pte)) {
            return 1;
        }
        new_pte = old_pte | H_PAGE_BUSY | _PAGE_ACCESSED | H_PAGE_COMBO;
        if access & _PAGE_WRITE != 0 {
            new_pte |= _PAGE_DIRTY;
        }
        if pte_xchg(ptep, __pte(old_pte), __pte(new_pte)) {
            break;
        }
    }

    subpg_pte = new_pte & !subpg_prot as usize;
    rflags = htab_convert_pte_flags(subpg_pte, flags);
    if cpu_has_feature(CPU_FTR_NOEXECUTE) && !cpu_has_feature(CPU_FTR_COHERENT_ICACHE) {
        rflags = hash_page_do_lazy_icache(rflags, __pte(old_pte), trap);
    }

    subpg_index = ((ea & (PAGE_SIZE - 1)) >> shift) as u32;
    vpn = hpt_vpn(ea, vsid, ssize);
    rpte = __real_pte(__pte(old_pte), ptep, PTRS_PER_PTE);
    if old_pte & H_PAGE_HASHPTE == 0 {
        // htab_insert_hpte
    } else if old_pte & H_PAGE_COMBO == 0 {
        flush_hash_page(vpn, rpte, MMU_PAGE_64K, ssize, flags);
        old_pte &= !H_PAGE_HASHPTE;
        new_pte &= !H_PAGE_HASHPTE;
    } else if __rpte_sub_valid(rpte, subpg_index as usize) {
        gslot = pte_get_hash_gslot(vpn, shift, ssize, rpte, subpg_index as usize);
        let ret = mmu_hash_ops.hpte_updatepp(gslot, rflags, vpn, MMU_PAGE_4K, MMU_PAGE_4K, ssize, flags);
        if ret != -1 {
            *ptep = __pte(new_pte & !H_PAGE_BUSY);
            return 0;
        }
    }

    if old_pte & H_PAGE_COMBO == 0 {
        rpte.hidx = INVALID_RPTE_HIDX;
    }
    if old_pte & H_PAGE_4K_PFN != 0 {
        pa = pte_pfn(__pte(old_pte)) << HW_PAGE_SHIFT;
    } else {
        pa = (pte_pfn(__pte(old_pte)) << PAGE_SHIFT) + ((subpg_index as usize) << shift);
    }
    hash = hpt_hash(vpn, shift, ssize);
    loop {
        hpte_group = (hash & htab_hash_mask) * HPTES_PER_GROUP;
        slot = mmu_hash_ops.hpte_insert(hpte_group, vpn, pa, rflags, 0, MMU_PAGE_4K, MMU_PAGE_4K, ssize);
        if unlikely(slot == usize::MAX) {
            hpte_group = ((!hash) & htab_hash_mask) * HPTES_PER_GROUP;
            slot = mmu_hash_ops.hpte_insert(hpte_group, vpn, pa, rflags, HPTE_V_SECONDARY, MMU_PAGE_4K, MMU_PAGE_4K, ssize);
            let soft_invalid = hpte_soft_invalid(slot);
            if soft_invalid {
                gslot = slot & _PTEIDX_GROUP_IX;
                mmu_hash_ops.hpte_invalidate(hpte_group + gslot, vpn, MMU_PAGE_4K, MMU_PAGE_4K, ssize, 0);
            }
            if slot == usize::MAX || soft_invalid {
                if soft_invalid || (mftb() & 0x1) != 0 {
                    hpte_group = (hash & htab_hash_mask) * HPTES_PER_GROUP;
                }
                mmu_hash_ops.hpte_remove(hpte_group);
                continue;
            }
        }
        break;
    }
    if unlikely(slot == usize::MAX - 1) {
        *ptep = __pte(old_pte);
        hash_failure_debug(ea, access, vsid, trap, ssize, MMU_PAGE_4K, MMU_PAGE_4K, old_pte);
        return -1;
    }
    new_pte |= pte_set_hidx(ptep, rpte, subpg_index as usize, slot, PTRS_PER_PTE);
    new_pte |= H_PAGE_HASHPTE;
    if stress_hpt() {
        hpt_do_stress(ea, hpte_group);
    }
    *ptep = __pte(new_pte & !H_PAGE_BUSY);
    0
}

pub unsafe fn __hash_page_64K(
    ea: usize, access: usize, vsid: usize, ptep: *mut pte_t, trap: usize, flags: usize, ssize: i32,
) -> i32 {
    let mut rpte: real_pte_t;
    let mut hpte_group: usize;
    let mut rflags: usize;
    let mut pa: usize;
    let mut old_pte: usize;
    let mut new_pte: usize;
    let mut vpn: usize;
    let mut hash: usize;
    let mut slot: usize;
    let shift: usize = mmu_psize_defs[MMU_PAGE_64K].shift;

    loop {
        let pte: pte_t = READ_ONCE(*ptep);
        old_pte = pte_val(pte);
        if unlikely(old_pte & H_PAGE_BUSY) { return 0; }
        if unlikely(!check_pte_access(access, old_pte)) { return 1; }
        if !mmu_has_feature(MMU_FTR_CI_LARGE_PAGE) && unlikely(pte_ci(pte)) { return 0; }
        new_pte = old_pte | H_PAGE_BUSY | _PAGE_ACCESSED;
        if access & _PAGE_WRITE != 0 { new_pte |= _PAGE_DIRTY; }
        if pte_xchg(ptep, __pte(old_pte), __pte(new_pte)) { break; }
    }
    rflags = htab_convert_pte_flags(new_pte, flags);
    rpte = __real_pte(__pte(old_pte), ptep, PTRS_PER_PTE);
    if cpu_has_feature(CPU_FTR_NOEXECUTE) && !cpu_has_feature(CPU_FTR_COHERENT_ICACHE) {
        rflags = hash_page_do_lazy_icache(rflags, __pte(old_pte), trap);
    }
    vpn = hpt_vpn(ea, vsid, ssize);
    if unlikely(old_pte & H_PAGE_HASHPTE != 0) {
        let gslot = pte_get_hash_gslot(vpn, shift, ssize, rpte, 0);
        if mmu_hash_ops.hpte_updatepp(gslot, rflags, vpn, MMU_PAGE_64K, MMU_PAGE_64K, ssize, flags) == -1 {
            old_pte &= !_PAGE_HPTEFLAGS;
        }
    }
    if likely(old_pte & H_PAGE_HASHPTE == 0) {
        pa = pte_pfn(__pte(old_pte)) << PAGE_SHIFT;
        hash = hpt_hash(vpn, shift, ssize);
        loop {
            hpte_group = (hash & htab_hash_mask) * HPTES_PER_GROUP;
            slot = mmu_hash_ops.hpte_insert(hpte_group, vpn, pa, rflags, 0, MMU_PAGE_64K, MMU_PAGE_64K, ssize);
            if unlikely(slot == usize::MAX) {
                hpte_group = ((!hash) & htab_hash_mask) * HPTES_PER_GROUP;
                slot = mmu_hash_ops.hpte_insert(hpte_group, vpn, pa, rflags, HPTE_V_SECONDARY, MMU_PAGE_64K, MMU_PAGE_64K, ssize);
                if slot == usize::MAX {
                    if (mftb() & 0x1) != 0 { hpte_group = (hash & htab_hash_mask) * HPTES_PER_GROUP; }
                    mmu_hash_ops.hpte_remove(hpte_group);
                    continue;
                }
            }
            break;
        }
        if unlikely(slot == usize::MAX - 1) {
            *ptep = __pte(old_pte);
            hash_failure_debug(ea, access, vsid, trap, ssize, MMU_PAGE_64K, MMU_PAGE_64K, old_pte);
            return -1;
        }
        new_pte = (new_pte & !_PAGE_HPTEFLAGS) | H_PAGE_HASHPTE;
        new_pte |= pte_set_hidx(ptep, rpte, 0, slot, PTRS_PER_PTE);
        if stress_hpt() { hpt_do_stress(ea, hpte_group); }
    }
    *ptep = __pte(new_pte & !H_PAGE_BUSY);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
