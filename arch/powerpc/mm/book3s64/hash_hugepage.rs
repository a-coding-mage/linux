/*
 * Copyright IBM Corporation, 2013
 * Author Aneesh Kumar K.V <aneesh.kumar@linux.ibm.com>
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of version 2.1 of the GNU Lesser General Public License
 * as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it would be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 *
 */

/*
 * PPC64 THP Support for hash based MMUs
 */

pub unsafe fn __hash_page_thp(
    ea: ::core::ffi::c_ulong,
    access: ::core::ffi::c_ulong,
    vsid: ::core::ffi::c_ulong,
    pmdp: *mut pmd_t,
    trap: ::core::ffi::c_ulong,
    flags: ::core::ffi::c_ulong,
    ssize: ::core::ffi::c_int,
    psize: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_uint;
    let mut valid: ::core::ffi::c_uint;
    let mut hpte_slot_array: *mut ::core::ffi::c_uchar;
    let mut rflags: ::core::ffi::c_ulong;
    let mut pa: ::core::ffi::c_ulong;
    let mut hidx: ::core::ffi::c_ulong;
    let mut old_pmd: ::core::ffi::c_ulong;
    let mut new_pmd: ::core::ffi::c_ulong;
    let mut ret: ::core::ffi::c_int;
    let lpsize: ::core::ffi::c_int = MMU_PAGE_16M;
    let mut vpn: ::core::ffi::c_ulong;
    let mut hash: ::core::ffi::c_ulong;
    let mut shift: ::core::ffi::c_ulong;
    let mut slot: ::core::ffi::c_ulong;

    /*
     * atomically mark the linux large page PMD busy and dirty
     */
    loop {
        let pmd = READ_ONCE(*pmdp);

        old_pmd = pmd_val(pmd);
        /* If PMD busy, retry the access */
        if unlikely(old_pmd & H_PAGE_BUSY != 0) {
            return 0;
        }
        /* If PMD permissions don't match, take page fault */
        if unlikely(check_pte_access(access, old_pmd) == 0) {
            return 1;
        }
        /*
         * Try to lock the PTE, add ACCESSED and DIRTY if it was
         * a write access
         */
        new_pmd = old_pmd | H_PAGE_BUSY | _PAGE_ACCESSED;
        if access & _PAGE_WRITE != 0 {
            new_pmd |= _PAGE_DIRTY;
        }
        if pmd_xchg(pmdp, __pmd(old_pmd), __pmd(new_pmd)) != 0 {
            break;
        }
    }

    /*
     * Make sure this is thp or devmap entry
     */
    if old_pmd & H_PAGE_THP_HUGE == 0 {
        return 0;
    }

    rflags = htab_convert_pte_flags(new_pmd, flags);

    /*
     * THPs are only supported on platforms that can do mixed page size
     * segments (MPSS) and all such platforms have coherent icache. Hence we
     * don't need to do lazy icache flush (hash_page_do_lazy_icache()) on
     * noexecute fault.
     */

    /*
     * Find the slot index details for this ea, using base page size.
     */
    shift = mmu_psize_defs[psize as usize].shift;
    index = ((ea & !HPAGE_PMD_MASK) >> shift) as ::core::ffi::c_uint;
    BUG_ON(index >= PTE_FRAG_SIZE);

    vpn = hpt_vpn(ea, vsid, ssize);
    hpte_slot_array = get_hpte_slot_array(pmdp);
    if psize == MMU_PAGE_4K {
        /*
         * invalidate the old hpte entry if we have that mapped via 64K
         * base page size. This is because demote_segment won't flush
         * hash page table entries.
         */
        if old_pmd & H_PAGE_HASHPTE != 0 && old_pmd & H_PAGE_COMBO == 0 {
            flush_hash_hugepage(vsid, ea, pmdp, MMU_PAGE_64K, ssize, flags);
            /*
             * With THP, we also clear the slot information with
             * respect to all the 64K hash pte mapping the 16MB
             * page. They are all invalid now. This make sure we
             * don't find the slot valid when we fault with 4k
             * base page size.
             *
             */
            memset(hpte_slot_array, 0, PTE_FRAG_SIZE);
        }
    }

    valid = hpte_valid(hpte_slot_array, index);
    if valid != 0 {
        /* update the hpte bits */
        hash = hpt_hash(vpn, shift, ssize);
        hidx = hpte_hash_index(hpte_slot_array, index);
        if hidx & _PTEIDX_SECONDARY != 0 {
            hash = !hash;
        }
        slot = (hash & htab_hash_mask) * HPTES_PER_GROUP;
        slot += hidx & _PTEIDX_GROUP_IX;

        ret = mmu_hash_ops.hpte_updatepp(slot, rflags, vpn, psize, lpsize, ssize, flags);
        /*
         * We failed to update, try to insert a new entry.
         */
        if ret == -1 {
            /*
             * large pte is marked busy, so we can be sure
             * nobody is looking at hpte_slot_array. hence we can
             * safely update this here.
             */
            valid = 0;
            *hpte_slot_array.add(index as usize) = 0;
        }
    }

    if valid == 0 {
        let mut hpte_group: ::core::ffi::c_ulong;

        hash = hpt_hash(vpn, shift, ssize);
        /* insert new entry */
        pa = pmd_pfn(__pmd(old_pmd)) << PAGE_SHIFT;
        new_pmd |= H_PAGE_HASHPTE;

        loop {
            hpte_group = (hash & htab_hash_mask) * HPTES_PER_GROUP;

            /* Insert into the hash table, primary slot */
            slot = mmu_hash_ops.hpte_insert(hpte_group, vpn, pa, rflags, 0, psize, lpsize, ssize);
            /*
             * Primary is full, try the secondary
             */
            if unlikely(slot == -1) {
                hpte_group = (!hash & htab_hash_mask) * HPTES_PER_GROUP;
                slot = mmu_hash_ops.hpte_insert(hpte_group, vpn, pa, rflags, HPTE_V_SECONDARY, psize, lpsize, ssize);
                if slot == -1 {
                    if mftb() & 0x1 != 0 {
                        hpte_group = (hash & htab_hash_mask) * HPTES_PER_GROUP;
                    }
                    mmu_hash_ops.hpte_remove(hpte_group);
                    continue;
                }
            }
            break;
        }
        /*
         * Hypervisor failure. Restore old pmd and return -1
         * similar to __hash_page_*
         */
        if unlikely(slot == -2) {
            *pmdp = __pmd(old_pmd);
            hash_failure_debug(ea, access, vsid, trap, ssize, psize, lpsize, old_pmd);
            return -1;
        }
        /*
         * large pte is marked busy, so we can be sure
         * nobody is looking at hpte_slot_array. hence we can
         * safely update this here.
         */
        mark_hpte_slot_valid(hpte_slot_array, index, slot);
    }
    /*
     * Mark the pte with H_PAGE_COMBO, if we are trying to hash it with
     * base page size 4k.
     */
    if psize == MMU_PAGE_4K {
        new_pmd |= H_PAGE_COMBO;
    }
    /*
     * The hpte valid is stored in the pgtable whose address is in the
     * second half of the PMD. Order this against clearing of the busy bit in
     * huge pmd.
     */
    smp_wmb();
    *pmdp = __pmd(new_pmd & !H_PAGE_BUSY);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
