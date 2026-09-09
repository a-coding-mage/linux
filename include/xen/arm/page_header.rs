/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from the C header:
// #include <asm/page.h>
// #include <linux/pfn.h>
// #include <linux/types.h>
// #include <linux/dma-mapping.h>
// #include <linux/pgtable.h>
// #include <xen/xen.h>
// #include <xen/interface/grant_table.h>

pub const INVALID_P2M_ENTRY: usize = usize::MAX;

/* Xen machine address */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xmaddr_t {
    pub maddr: phys_addr_t,
}

/* Xen pseudo-physical address */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xpaddr_t {
    pub paddr: phys_addr_t,
}

#[inline]
pub const fn XMADDR(x: phys_addr_t) -> xmaddr_t {
    xmaddr_t { maddr: x }
}

#[inline]
pub const fn XPADDR(x: phys_addr_t) -> xpaddr_t {
    xpaddr_t { paddr: x }
}

#[inline]
pub const fn phys_to_machine_mapping_valid(_pfn: usize) -> usize {
    1
}

/*
 * The pseudo-physical frame (pfn) used in all the helpers is always based
 * on Xen page granularity (i.e 4KB).
 *
 * A Linux page may be split across multiple non-contiguous Xen page so we
 * have to keep track with frame based on 4KB page granularity.
 *
 * PV drivers should never make a direct usage of those helpers (particularly
 * pfn_to_gfn and gfn_to_pfn).
 */

extern "C" {
    pub fn __pfn_to_mfn(pfn: usize) -> usize;
    pub static mut phys_to_mach: rb_root;

    pub fn set_foreign_p2m_mapping(
        map_ops: *mut gnttab_map_grant_ref,
        kmap_ops: *mut gnttab_map_grant_ref,
        pages: *mut *mut page,
        count: c_uint,
    ) -> c_int;

    pub fn clear_foreign_p2m_mapping(
        unmap_ops: *mut gnttab_unmap_grant_ref,
        kunmap_ops: *mut gnttab_unmap_grant_ref,
        pages: *mut *mut page,
        count: c_uint,
    ) -> c_int;

    pub fn __set_phys_to_machine(pfn: usize, mfn: usize) -> bool;
    pub fn __set_phys_to_machine_multi(pfn: usize, mfn: usize, nr_pages: usize) -> bool;

    pub fn xen_arch_need_swiotlb(
        dev: *mut device,
        phys: phys_addr_t,
        dev_addr: dma_addr_t,
    ) -> bool;

    fn BUG() -> !;
}

/* Pseudo-physical <-> Guest conversion */
#[inline]
pub fn pfn_to_gfn(pfn: usize) -> usize {
    pfn
}

#[inline]
pub fn gfn_to_pfn(gfn: usize) -> usize {
    gfn
}

/* Pseudo-physical <-> BUS conversion */
#[inline]
pub unsafe fn pfn_to_bfn(pfn: usize) -> usize {
    let mut mfn: usize;

    if (*core::ptr::addr_of!(phys_to_mach)).rb_node != core::ptr::null_mut() {
        mfn = __pfn_to_mfn(pfn);
        if mfn != INVALID_P2M_ENTRY {
            return mfn;
        }
    }

    pfn
}

#[inline]
pub const fn bfn_to_pfn(bfn: usize) -> usize {
    bfn
}

#[inline]
pub const fn bfn_to_local_pfn(bfn: usize) -> usize {
    bfn_to_pfn(bfn)
}

/* VIRT <-> GUEST conversion */
#[inline]
pub unsafe fn virt_to_gfn(v: *const core::ffi::c_void) -> usize {
    WARN_ON_ONCE(!virt_addr_valid(v));
    pfn_to_gfn(virt_to_phys(v) >> XEN_PAGE_SHIFT)
}

#[inline]
pub unsafe fn gfn_to_virt(m: usize) -> *mut core::ffi::c_void {
    __va(gfn_to_pfn(m) << XEN_PAGE_SHIFT)
}

#[inline]
pub unsafe fn percpu_to_gfn(v: *mut core::ffi::c_void) -> usize {
    pfn_to_gfn(per_cpu_ptr_to_phys(v) >> XEN_PAGE_SHIFT)
}

/* Only used in PV code. But ARM guests are always HVM. */
#[inline]
pub unsafe fn arbitrary_virt_to_machine(_vaddr: *mut core::ffi::c_void) -> xmaddr_t {
    BUG();
}

#[inline]
pub unsafe fn set_phys_to_machine(pfn: usize, mfn: usize) -> bool {
    __set_phys_to_machine(pfn, mfn)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
