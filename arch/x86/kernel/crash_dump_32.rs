// SPDX-License-Identifier: GPL-2.0
/*
 *	Memory preserving reboot related code.
 *
 *	Created by: Hariprasad Nellitheertha (hari@in.ibm.com)
 *	Copyright (C) IBM Corporation, 2004. All rights reserved
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/slab.h, linux/errno.h, linux/highmem.h, linux/crash_dump.h,
// and linux/uio.h.

#[inline]
unsafe fn is_crashed_pfn_valid(pfn: usize) -> bool {
    // CONFIG_X86_PAE selects the alternate branch at build time.
    #[cfg(not(CONFIG_X86_PAE))]
    {
        /*
         * non-PAE kdump kernel executed from a PAE one will crop high pte
         * bits and poke unwanted space counting again from address 0, we
         * don't want that. pte must fit into unsigned long. In fact, the
         * test checks high 12 bits for being zero (pfn will be shifted left
         * by PAGE_SHIFT).
         */
        return pte_pfn(pfn_pte(pfn, __pgprot(0))) == pfn;
    }

    #[cfg(CONFIG_X86_PAE)]
    {
        true
    }
}

unsafe fn copy_oldmem_page(
    iter: *mut iov_iter,
    pfn: usize,
    mut csize: usize,
    offset: usize,
) -> isize {
    let mut vaddr: *mut core::ffi::c_void;

    if csize == 0 {
        return 0;
    }

    if !is_crashed_pfn_valid(pfn) {
        return -(EFAULT as isize);
    }

    vaddr = kmap_local_pfn(pfn);
    csize = copy_to_iter(
        (vaddr as *mut u8).add(offset) as *const core::ffi::c_void,
        csize,
        iter,
    );
    kunmap_local(vaddr);

    csize as isize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
