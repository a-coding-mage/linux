/* SPDX-License-Identifier: GPL-2.0-only */

/* Dependencies supplied by the surrounding kernel translation unit. */

/*
 * Bits 0-1 are used to encode the memory ownership state of each page from the
 * point of view of a pKVM "component" (host, hyp, guest, ... see enum
 * pkvm_component_id):
 *   00: The page is owned and exclusively accessible by the component;
 *   01: The page is owned and accessible by the component, but is also
 *       accessible by another component;
 *   10: The page is accessible but not owned by the component;
 * The storage of this state depends on the component: either in the
 * hyp_vmemmap for the host and hyp states or in PTE software bits for guests.
 */
#[repr(u64)]
pub enum pkvm_page_state {
    PKVM_PAGE_OWNED = 0,
    PKVM_PAGE_SHARED_OWNED = BIT(0),
    PKVM_PAGE_SHARED_BORROWED = BIT(1),

    /*
     * 'Meta-states' are not stored directly in PTE SW bits for guest
     * states, but inferred from the context (e.g. invalid PTE entries).
     * For the host and hyp, meta-states are stored directly in the
     * struct hyp_page.
     */
    PKVM_NOPAGE = BIT(0) | BIT(1),

    /*
     * 'Meta-states' which aren't encoded directly in the PTE's SW bits (or
     * the hyp_vmemmap entry for the host)
     */
    PKVM_POISON = BIT(2),
}

pub const PKVM_PAGE_STATE_VMEMMAP_MASK: u64 = BIT(0) | BIT(1);
pub const PKVM_PAGE_STATE_PROT_MASK: kvm_pgtable_prot =
    KVM_PGTABLE_PROT_SW0 | KVM_PGTABLE_PROT_SW1;

pub unsafe fn pkvm_mkstate(
    mut prot: kvm_pgtable_prot,
    state: pkvm_page_state,
) -> kvm_pgtable_prot {
    prot &= !PKVM_PAGE_STATE_PROT_MASK;
    prot |= FIELD_PREP(PKVM_PAGE_STATE_PROT_MASK, state);
    prot
}

pub unsafe fn pkvm_getstate(prot: kvm_pgtable_prot) -> pkvm_page_state {
    FIELD_GET(PKVM_PAGE_STATE_PROT_MASK, prot)
}

#[repr(C)]
pub struct hyp_page {
    pub refcount: u16,
    pub order: u8,

    /* Host state. Guarded by the host stage-2 lock. */
    pub __host_state: u8,

    /*
     * Complement of the hyp state. Guarded by the hyp stage-1 lock. We use
     * the complement so that the initial 0 in __hyp_state_comp (due to the
     * entire vmemmap starting off zeroed) encodes PKVM_NOPAGE.
     */
    pub __hyp_state_comp: u8,

    pub host_share_guest_count: u32,
}

pub static mut __hyp_vmemmap: u64 = 0;

pub unsafe fn hyp_phys_to_virt(phys: phys_addr_t) -> *mut core::ffi::c_void {
    __hyp_va((phys as phys_addr_t).wrapping_sub(hyp_physvirt_offset))
}

pub unsafe fn hyp_virt_to_phys(addr: *mut core::ffi::c_void) -> phys_addr_t {
    __hyp_pa(addr)
}

pub unsafe fn hyp_phys_to_pfn(phys: phys_addr_t) -> phys_addr_t {
    phys >> PAGE_SHIFT
}

pub unsafe fn hyp_pfn_to_phys(pfn: phys_addr_t) -> phys_addr_t {
    pfn << PAGE_SHIFT
}

pub unsafe fn hyp_phys_to_page(phys: phys_addr_t) -> *mut hyp_page {
    BUILD_BUG_ON(core::mem::size_of::<hyp_page>() != core::mem::size_of::<u64>());
    (__hyp_vmemmap as *mut hyp_page).add(hyp_phys_to_pfn(phys) as usize)
}

pub unsafe fn hyp_virt_to_page(virt: *mut core::ffi::c_void) -> *mut hyp_page {
    hyp_phys_to_page(__hyp_pa(virt))
}

pub unsafe fn hyp_virt_to_pfn(virt: *mut core::ffi::c_void) -> phys_addr_t {
    hyp_phys_to_pfn(__hyp_pa(virt))
}

pub unsafe fn hyp_page_to_pfn(page: *mut hyp_page) -> phys_addr_t {
    page.offset_from(__hyp_vmemmap as *mut hyp_page) as phys_addr_t
}

pub unsafe fn hyp_page_to_phys(page: *mut hyp_page) -> phys_addr_t {
    hyp_pfn_to_phys(hyp_page_to_pfn(page))
}

pub unsafe fn hyp_page_to_virt(page: *mut hyp_page) -> *mut core::ffi::c_void {
    __hyp_va(hyp_page_to_phys(page))
}

pub unsafe fn hyp_page_to_pool(page: *mut hyp_page) -> *mut core::ffi::c_void {
    (*page).pool
}

pub unsafe fn get_host_state(p: *mut hyp_page) -> pkvm_page_state {
    (*p).__host_state as pkvm_page_state
}

pub unsafe fn set_host_state(p: *mut hyp_page, state: pkvm_page_state) {
    (*p).__host_state = state as u8;
}

pub unsafe fn get_hyp_state(p: *mut hyp_page) -> pkvm_page_state {
    ((*p).__hyp_state_comp as u64 ^ PKVM_PAGE_STATE_VMEMMAP_MASK) as pkvm_page_state
}

pub unsafe fn set_hyp_state(p: *mut hyp_page, state: pkvm_page_state) {
    (*p).__hyp_state_comp = (state as u64 ^ PKVM_PAGE_STATE_VMEMMAP_MASK) as u8;
}

/*
 * Refcounting for 'struct hyp_page'.
 * hyp_pool::lock must be held if atomic access to the refcount is required.
 */
pub unsafe fn hyp_page_count(addr: *mut core::ffi::c_void) -> i32 {
    (*hyp_virt_to_page(addr)).refcount as i32
}

pub unsafe fn hyp_page_ref_inc(p: *mut hyp_page) {
    BUG_ON((*p).refcount == USHRT_MAX);
    (*p).refcount += 1;
}

pub unsafe fn hyp_page_ref_dec(p: *mut hyp_page) {
    BUG_ON((*p).refcount == 0);
    (*p).refcount -= 1;
}

pub unsafe fn hyp_page_ref_dec_and_test(p: *mut hyp_page) -> bool {
    hyp_page_ref_dec(p);
    (*p).refcount == 0
}

pub unsafe fn hyp_set_page_refcounted(p: *mut hyp_page) {
    BUG_ON((*p).refcount != 0);
    (*p).refcount = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
