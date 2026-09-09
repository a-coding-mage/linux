/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies: linux/kernel.h, linux/types.h, linux/spinlock.h,
// linux/pfn.h, linux/mm.h, linux/device.h, asm/extable.h, asm/page.h,
// xen/xen.h, xen/interface/xen.h, xen/interface/grant_table.h

#[repr(C)]
pub struct xmaddr {
    pub maddr: phys_addr_t,
}
pub type xmaddr_t = xmaddr;

#[repr(C)]
pub struct xpaddr {
    pub paddr: phys_addr_t,
}
pub type xpaddr_t = xpaddr;

// #ifdef CONFIG_X86_64
pub const XEN_PHYSICAL_MASK: u64 = __sme_clr((1u64 << 52) - 1);
// #else: XEN_PHYSICAL_MASK = __PHYSICAL_MASK

pub const XEN_PTE_MFN_MASK: pteval_t = ((PAGE_MASK as i64 as pteval_t) & XEN_PHYSICAL_MASK as pteval_t);

#[inline]
pub const fn XMADDR(x: phys_addr_t) -> xmaddr_t { xmaddr { maddr: x } }
#[inline]
pub const fn XPADDR(x: phys_addr_t) -> xpaddr_t { xpaddr { paddr: x } }

pub const INVALID_P2M_ENTRY: usize = !0;
pub const FOREIGN_FRAME_BIT: usize = 1usize << (BITS_PER_LONG - 1);
pub const IDENTITY_FRAME_BIT: usize = 1usize << (BITS_PER_LONG - 2);
#[inline]
pub const fn FOREIGN_FRAME(m: usize) -> usize { m | FOREIGN_FRAME_BIT }
#[inline]
pub const fn IDENTITY_FRAME(m: usize) -> usize { m | IDENTITY_FRAME_BIT }
pub const P2M_PER_PAGE: usize = PAGE_SIZE / core::mem::size_of::<usize>();

extern "C" {
    pub static mut machine_to_phys_mapping: *mut usize;
    pub static mut machine_to_phys_nr: usize;
    pub static mut xen_p2m_addr: *mut usize;
    pub static mut xen_p2m_size: usize;
    pub static mut xen_max_p2m_pfn: usize;

    pub fn xen_alloc_p2m_entry(pfn: usize) -> core::ffi::c_int;
    pub fn get_phys_to_machine(pfn: usize) -> usize;
    pub fn set_phys_to_machine(pfn: usize, mfn: usize) -> bool;
    pub fn __set_phys_to_machine(pfn: usize, mfn: usize) -> bool;
    pub fn set_phys_range_identity(pfn_s: usize, pfn_e: usize) -> usize;
    pub fn arbitrary_virt_to_machine(address: *mut core::ffi::c_void) -> xmaddr_t;
    pub fn arbitrary_virt_to_mfn(vaddr: *mut core::ffi::c_void) -> usize;
    pub fn make_lowmem_page_readonly(vaddr: *mut core::ffi::c_void);
    pub fn make_lowmem_page_readwrite(vaddr: *mut core::ffi::c_void);
}

// CONFIG_XEN_PV selects the external implementations; otherwise these are the C inline stubs.
#[inline]
pub unsafe fn set_foreign_p2m_mapping(_map_ops: *mut gnttab_map_grant_ref, _kmap_ops: *mut gnttab_map_grant_ref, _pages: *mut *mut page, _count: u32) -> i32 { 0 }
#[inline]
pub unsafe fn clear_foreign_p2m_mapping(_unmap_ops: *mut gnttab_unmap_grant_ref, _kunmap_ops: *mut gnttab_unmap_grant_ref, _pages: *mut *mut page, _count: u32) -> i32 { 0 }

#[inline]
pub unsafe fn xen_safe_write_ulong(addr: *mut usize, val: usize) -> i32 {
    // The C implementation uses fault-catching architecture assembly and an exception table.
    core::ptr::write_volatile(addr, val);
    0
}

#[inline]
pub unsafe fn xen_safe_read_ulong(addr: *const usize, val: *mut usize) -> i32 {
    core::ptr::write(val, core::ptr::read_volatile(addr));
    0
}

#[inline]
pub unsafe fn __pfn_to_mfn(pfn: usize) -> usize {
    // CONFIG_XEN_PV branch
    if pfn < xen_p2m_size {
        *xen_p2m_addr.add(pfn)
    } else if pfn < xen_max_p2m_pfn {
        get_phys_to_machine(pfn)
    } else {
        IDENTITY_FRAME(pfn)
    }
}

#[inline]
pub unsafe fn pfn_to_mfn(pfn: usize) -> usize {
    if !xen_pv_domain() { return pfn; }
    let mut mfn = __pfn_to_mfn(pfn);
    if mfn != INVALID_P2M_ENTRY { mfn &= !(FOREIGN_FRAME_BIT | IDENTITY_FRAME_BIT); }
    mfn
}

#[inline]
pub unsafe fn phys_to_machine_mapping_valid(pfn: usize) -> i32 {
    if !xen_pv_domain() { 1 } else { (__pfn_to_mfn(pfn) != INVALID_P2M_ENTRY) as i32 }
}

#[inline]
pub unsafe fn mfn_to_pfn_no_overrides(mfn: usize) -> usize {
    if mfn >= machine_to_phys_nr { return !0; }
    let mut pfn = !0usize;
    if xen_safe_read_ulong(machine_to_phys_mapping.add(mfn), &mut pfn) < 0 { return !0; }
    pfn
}

#[inline]
pub unsafe fn mfn_to_pfn(mfn: usize) -> usize {
    if !xen_pv_domain() { return mfn; }
    let mut pfn = mfn_to_pfn_no_overrides(mfn);
    if __pfn_to_mfn(pfn) != mfn { pfn = !0; }
    if pfn == !0 && __pfn_to_mfn(mfn) == IDENTITY_FRAME(mfn) { pfn = mfn; }
    pfn
}

#[inline]
pub unsafe fn phys_to_machine(phys: xpaddr_t) -> xmaddr_t {
    let offset = phys.paddr & !PAGE_MASK;
    XMADDR(PFN_PHYS(pfn_to_mfn(PFN_DOWN(phys.paddr))) | offset)
}
#[inline]
pub unsafe fn machine_to_phys(machine: xmaddr_t) -> xpaddr_t {
    let offset = machine.maddr & !PAGE_MASK;
    XPADDR(PFN_PHYS(mfn_to_pfn(PFN_DOWN(machine.maddr))) | offset)
}

#[inline]
pub unsafe fn pfn_to_gfn(pfn: usize) -> usize { if !xen_pv_domain() { pfn } else { pfn_to_mfn(pfn) } }
#[inline]
pub unsafe fn gfn_to_pfn(gfn: usize) -> usize { if !xen_pv_domain() { gfn } else { mfn_to_pfn(gfn) } }
#[inline]
pub unsafe fn pfn_to_bfn(pfn: usize) -> usize { pfn_to_gfn(pfn) }
#[inline]
pub unsafe fn bfn_to_pfn(bfn: usize) -> usize { gfn_to_pfn(bfn) }

#[inline]
pub unsafe fn bfn_to_local_pfn(mfn: usize) -> usize {
    if !xen_pv_domain() { return mfn; }
    let pfn = mfn_to_pfn(mfn);
    if __pfn_to_mfn(pfn) != mfn { return !0; }
    pfn
}

#[inline]
pub unsafe fn virt_to_pfn(v: *const core::ffi::c_void) -> usize { PFN_DOWN(__pa(v)) }
#[inline]
pub unsafe fn virt_to_mfn(v: *const core::ffi::c_void) -> usize { pfn_to_mfn(virt_to_pfn(v)) }
#[inline]
pub unsafe fn mfn_to_virt(m: usize) -> *mut core::ffi::c_void { __va(mfn_to_pfn(m) << PAGE_SHIFT) }
#[inline]
pub unsafe fn virt_to_gfn(v: *const core::ffi::c_void) -> usize { pfn_to_gfn(virt_to_pfn(v)) }
#[inline]
pub unsafe fn gfn_to_virt(g: usize) -> *mut core::ffi::c_void { __va(gfn_to_pfn(g) << PAGE_SHIFT) }

#[inline]
pub const fn pte_mfn(pte: pte_t) -> usize { ((pte.pte & XEN_PTE_MFN_MASK) >> PAGE_SHIFT) as usize }
#[inline]
pub fn mfn_pte(page_nr: usize, pgprot: pgprot_t) -> pte_t { pte_t { pte: ((page_nr as phys_addr_t) << PAGE_SHIFT) | massage_pgprot(pgprot) } }
#[inline]
pub const fn pte_val_ma(pte: pte_t) -> pteval_t { pte.pte }
#[inline]
pub const fn __pte_ma(x: pteval_t) -> pte_t { pte_t { pte: x } }

// pmd_val_ma, pud_val_ma, __pmd_ma, and p4d_val_ma are field-access macros in C;
// their direct Rust equivalents depend on the imported page-table representations.

#[inline]
pub const fn xen_arch_need_swiotlb(_dev: *mut device, _phys: phys_addr_t, _dev_addr: dma_addr_t) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
