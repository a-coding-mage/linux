/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KFENCE support for LoongArch.
 *
 * Author: Enze Li <lienze@kylinos.cn>
 * Copyright (C) 2022-2023 KylinSoft Corporation.
 */

/* Dependencies: linux/kfence.h, linux/vmalloc.h, asm/pgtable.h, and asm/tlb.h. */

/// Architecture-specific KFENCE pool initialization.
#[inline]
pub unsafe fn arch_kfence_init_pool() -> bool {
    let mut err: i32;
    let mut kaddr: *mut core::ffi::c_char;
    let mut vaddr: *mut core::ffi::c_char;
    let kfence_pool: *mut core::ffi::c_char = __kfence_pool;
    let mut area: *mut vm_struct;

    area = __get_vm_area_caller(
        KFENCE_POOL_SIZE,
        VM_IOREMAP,
        KFENCE_AREA_START,
        KFENCE_AREA_END,
        __builtin_return_address(0),
    );
    if area.is_null() {
        return false;
    }

    __kfence_pool = (*area).addr as *mut core::ffi::c_char;
    err = ioremap_page_range(
        __kfence_pool as usize as u64,
        (__kfence_pool as usize as u64).wrapping_add(KFENCE_POOL_SIZE),
        virt_to_phys(kfence_pool as *mut core::ffi::c_void),
        PAGE_KERNEL,
    );
    if err != 0 {
        free_vm_area(area);
        __kfence_pool = kfence_pool;
        return false;
    }

    kaddr = kfence_pool;
    vaddr = __kfence_pool;
    while (kaddr as usize) < (kfence_pool as usize).wrapping_add(KFENCE_POOL_SIZE as usize) {
        set_page_address(virt_to_page(kaddr as *mut core::ffi::c_void), vaddr);
        kaddr = kaddr.add(PAGE_SIZE as usize);
        vaddr = vaddr.add(PAGE_SIZE as usize);
    }

    true
}

/* Protect the given page and flush TLB. */
#[inline]
pub unsafe fn kfence_protect_page(addr: usize, protect: bool) -> bool {
    let pte: *mut pte_t = virt_to_kpte(addr);

    if WARN_ON(pte.is_null()) || pte_none(ptep_get(pte)) {
        return false;
    }

    if protect {
        set_pte(
            pte,
            __pte(pte_val(ptep_get(pte)) & !(_PAGE_VALID | _PAGE_PRESENT)),
        );
    } else {
        set_pte(
            pte,
            __pte(pte_val(ptep_get(pte)) | (_PAGE_VALID | _PAGE_PRESENT)),
        );
    }

    preempt_disable();
    local_flush_tlb_one(addr);
    preempt_enable();

    true
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
