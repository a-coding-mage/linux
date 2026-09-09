// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel/Xen Rust environment:
// linux/pfn.h, asm/xen/page.h, asm/xen/hypercall.h, xen/interface/memory.h,
// and xen-ops.h.

pub unsafe fn arbitrary_virt_to_mfn(vaddr: *mut core::ffi::c_void) -> libc::c_ulong {
    let maddr: xmaddr_t = arbitrary_virt_to_machine(vaddr);

    pfn_down(maddr.maddr)
}

pub unsafe fn arbitrary_virt_to_machine(vaddr: *mut core::ffi::c_void) -> xmaddr_t {
    let address: libc::c_ulong = vaddr as libc::c_ulong;
    let mut level: libc::c_uint;
    let pte: *mut pte_t;
    let offset: libc::c_uint;

    /*
     * if the PFN is in the linear mapped vaddr range, we can just use
     * the (quick) virt_to_machine() p2m lookup
     */
    if virt_addr_valid(vaddr) {
        return virt_to_machine(vaddr);
    }

    /* otherwise we have to do a (slower) full page-table walk */

    pte = lookup_address(address, &mut level);
    if pte.is_null() {
        panic!("BUG_ON(pte == NULL)");
    }
    offset = address & !PAGE_MASK;
    XMADDR(((pte_mfn(*pte) as phys_addr_t) << PAGE_SHIFT) + offset as phys_addr_t)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xen_unmap_domain_gfn_range(
    vma: *mut vm_area_struct,
    nr: libc::c_int,
    pages: *mut *mut page,
) -> libc::c_int {
    if !xen_pv_domain() {
        return xen_xlate_unmap_gfn_range(vma, nr, pages);
    }

    if pages.is_null() {
        return 0;
    }

    -EINVAL
}

// EXPORT_SYMBOL_GPL(arbitrary_virt_to_machine);
// EXPORT_SYMBOL_GPL(xen_unmap_domain_gfn_range);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
