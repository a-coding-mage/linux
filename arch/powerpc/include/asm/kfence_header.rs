/* SPDX-License-Identifier: GPL-2.0 */
/*
 * powerpc KFENCE support.
 *
 * Copyright (C) 2020 CS GROUP France
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/mm.h and asm/pgtable.h

// CONFIG_PPC64_ELF_ABI_V1
#[cfg(feature = "CONFIG_PPC64_ELF_ABI_V1")]
pub const ARCH_FUNC_PREFIX: &str = ".";

unsafe extern "C" {
    pub static mut kfence_early_init: bool;
    pub static mut kfence_disabled: bool;
}

#[inline]
pub unsafe fn disable_kfence() {
    unsafe {
        kfence_disabled = true;
    }
}

#[inline]
pub unsafe fn arch_kfence_init_pool() -> bool {
    unsafe { !kfence_disabled }
}

#[inline]
pub unsafe fn kfence_early_init_enabled() -> bool {
    // IS_ENABLED(CONFIG_KFENCE) is a build-time kernel configuration test.
    cfg!(feature = "CONFIG_KFENCE") && unsafe { kfence_early_init }
}

// CONFIG_PPC64
#[cfg(feature = "CONFIG_PPC64")]
#[inline]
pub unsafe fn kfence_protect_page(addr: ::core::ffi::c_ulong, protect: bool) -> bool {
    let page = unsafe { virt_to_page(addr as *mut ::core::ffi::c_void) };

    unsafe {
        __kernel_map_pages(page, 1, !protect);
    }

    true
}

// !CONFIG_PPC64
#[cfg(not(feature = "CONFIG_PPC64"))]
#[inline]
pub unsafe fn kfence_protect_page(addr: ::core::ffi::c_ulong, protect: bool) -> bool {
    let kpte = unsafe { virt_to_kpte(addr) };

    if protect {
        unsafe {
            pte_update(&raw mut init_mm, addr, kpte, _PAGE_PRESENT, 0, 0);
            flush_tlb_kernel_range(addr, addr.wrapping_add(PAGE_SIZE));
        }
    } else {
        unsafe {
            pte_update(&raw mut init_mm, addr, kpte, 0, _PAGE_PRESENT, 0);
        }
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
