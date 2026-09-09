// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of hmm.c. Kernel-provided types and functions are
 * intentionally referenced as external dependencies. */

#[repr(C)]
pub struct hmm_vma_walk {
    pub range: *mut hmm_range,
    pub locked: *mut bool,
    pub last: c_ulong,
    pub end: c_ulong,
    pub required_fault: c_uint,
}

// Kernel types and constants are supplied by the surrounding kernel bindings.
type c_ulong = usize;
type c_uint = u32;
type c_int = i32;
type c_bool = bool;
#[allow(non_camel_case_types)] type hmm_range = crate::hmm_range;
#[allow(non_camel_case_types)] type mm_walk = crate::mm_walk;
#[allow(non_camel_case_types)] type mm_struct = crate::mm_struct;
#[allow(non_camel_case_types)] type vm_area_struct = crate::vm_area_struct;
#[allow(non_camel_case_types)] type device = crate::device;
#[allow(non_camel_case_types)] type hmm_dma_map = crate::hmm_dma_map;
#[allow(non_camel_case_types)] type pci_p2pdma_map_state = crate::pci_p2pdma_map_state;
#[allow(non_camel_case_types)] type dma_iova_state = crate::dma_iova_state;
#[allow(non_camel_case_types)] type pmd_t = crate::pmd_t;
#[allow(non_camel_case_types)] type pte_t = crate::pte_t;
#[allow(non_camel_case_types)] type pud_t = crate::pud_t;
#[allow(non_camel_case_types)] type spinlock_t = crate::spinlock_t;
#[allow(non_camel_case_types)] type page = crate::page;
type dma_addr_t = usize;
type phys_addr_t = usize;

const HMM_FAULT_PENDING: c_int = -11;
const HMM_FAULT_UNLOCKED: c_int = -37;
const HMM_NEED_FAULT: c_uint = 1 << 0;
const HMM_NEED_WRITE_FAULT: c_uint = 1 << 1;
const HMM_NEED_ALL_BITS: c_uint = HMM_NEED_FAULT | HMM_NEED_WRITE_FAULT;
const HMM_PFN_INOUT_FLAGS: c_ulong = HMM_PFN_DMA_MAPPED | HMM_PFN_P2PDMA | HMM_PFN_P2PDMA_BUS;

unsafe fn hmm_pfns_fill(mut addr: c_ulong, end: c_ulong, range: *mut hmm_range, cpu_flags: c_ulong) -> c_int {
    let mut i = (addr - (*range).start) >> PAGE_SHIFT;
    while addr < end {
        *(*range).hmm_pfns.add(i) &= HMM_PFN_INOUT_FLAGS;
        *(*range).hmm_pfns.add(i) |= cpu_flags;
        addr += PAGE_SIZE; i += 1;
    }
    0
}

unsafe fn hmm_record_fault(addr: c_ulong, end: c_ulong, required_fault: c_uint, walk: *mut mm_walk) -> c_int {
    let w = (*walk).private as *mut hmm_vma_walk;
    (*w).last = addr; (*w).end = end; (*w).required_fault = required_fault;
    HMM_FAULT_PENDING
}

unsafe fn hmm_pte_need_fault(w: *const hmm_vma_walk, mut pfn_req_flags: c_ulong, cpu_flags: c_ulong) -> c_uint {
    let range = (*w).range;
    pfn_req_flags &= (*range).pfn_flags_mask;
    pfn_req_flags |= (*range).default_flags;
    if pfn_req_flags & HMM_PFN_REQ_FAULT == 0 { return 0; }
    if pfn_req_flags & HMM_PFN_REQ_WRITE != 0 && cpu_flags & HMM_PFN_WRITE == 0 {
        return HMM_NEED_ALL_BITS;
    }
    if cpu_flags & HMM_PFN_VALID == 0 { return HMM_NEED_FAULT; }
    0
}

unsafe fn hmm_range_need_fault(w: *const hmm_vma_walk, pfns: *const c_ulong, npages: c_ulong, cpu_flags: c_ulong) -> c_uint {
    let range = (*w).range;
    if ((*range).default_flags | (*range).pfn_flags_mask) & HMM_PFN_REQ_FAULT == 0 { return 0; }
    let mut required = 0;
    for i in 0..npages {
        required |= hmm_pte_need_fault(w, *pfns.add(i), cpu_flags);
        if required == HMM_NEED_ALL_BITS { break; }
    }
    required
}

// The remaining callbacks retain the C walker's exact sequencing and delegate
// kernel page-table operations to the corresponding external bindings.
pub unsafe fn hmm_range_fault(range: *mut hmm_range) -> c_int {
    hmm_range_fault_locked(range, core::ptr::null_mut())
}

unsafe fn hmm_range_fault_locked(range: *mut hmm_range, locked: *mut bool) -> c_int {
    let mut walk = hmm_vma_walk { range, locked, last: (*range).start, end: 0, required_fault: 0 };
    let mm = (*(*range).notifier).mm;
    mmap_assert_locked(mm);
    loop {
        if mmu_interval_check_retry((*range).notifier, (*range).notifier_seq) { return -16; }
        let mut ret = walk_page_range(mm, walk.last, (*range).end, &hmm_walk_ops, &mut walk);
        if ret == HMM_FAULT_PENDING {
            ret = hmm_do_fault(mm, &mut walk);
            if ret == HMM_FAULT_UNLOCKED {
                if fatal_signal_pending(current()) { return -4; }
                return -16;
            }
        }
        if ret != -16 { return ret; }
    }
}

unsafe fn hmm_do_fault(mm: *mut mm_struct, w: *mut hmm_vma_walk) -> c_int {
    let mut addr = (*w).last;
    let end = (*w).end;
    let mut flags = FAULT_FLAG_REMOTE;
    if !(*w).locked.is_null() { flags |= FAULT_FLAG_ALLOW_RETRY | FAULT_FLAG_KILLABLE; }
    let vma = vma_lookup(mm, addr);
    if vma.is_null() { return -14; }
    if (*w).required_fault & HMM_NEED_WRITE_FAULT != 0 {
        if (*vma).vm_flags & VM_WRITE == 0 { return -1; }
        flags |= FAULT_FLAG_WRITE;
    }
    while addr < end {
        let ret = handle_mm_fault(vma, addr, flags, core::ptr::null_mut());
        if ret & (VM_FAULT_COMPLETED | VM_FAULT_RETRY) != 0 {
            if !(*w).locked.is_null() { *(*w).locked = false; }
            return HMM_FAULT_UNLOCKED;
        }
        if ret & VM_FAULT_ERROR != 0 { return vm_fault_to_errno(ret, 0); }
        addr += PAGE_SIZE;
    }
    -16
}

pub unsafe fn hmm_range_fault_unlocked_timeout(range: *mut hmm_range, timeout: c_ulong) -> c_int {
    let mm = (*(*range).notifier).mm;
    let mut deadline = 0; let mut locked = false;
    loop {
        if timeout != 0 && !locked { deadline = jiffies() + timeout; }
        (*range).notifier_seq = mmu_interval_read_begin((*range).notifier);
        let ret = mmap_read_lock_killable(mm); if ret != 0 { return ret; }
        if check_stable_address_space(mm) { mmap_read_unlock(mm); return -14; }
        if timeout != 0 && time_after(jiffies(), deadline) { mmap_read_unlock(mm); return -16; }
        locked = true;
        let ret = hmm_range_fault_locked(range, &mut locked);
        if locked { mmap_read_unlock(mm); }
        if ret != -16 { return ret; }
    }
}

extern "C" {
    fn mmap_assert_locked(mm: *mut mm_struct); fn mmu_interval_check_retry(n: *mut crate::mmu_interval_notifier, s: u64) -> bool;
    fn walk_page_range(mm: *mut mm_struct, start: c_ulong, end: c_ulong, ops: *const crate::mm_walk_ops, private: *mut hmm_vma_walk) -> c_int;
    fn vma_lookup(mm: *mut mm_struct, addr: c_ulong) -> *mut vm_area_struct;
    fn handle_mm_fault(vma: *mut vm_area_struct, addr: c_ulong, flags: c_uint, regs: *mut core::ffi::c_void) -> c_uint;
    fn vm_fault_to_errno(ret: c_uint, page: c_uint) -> c_int; fn fatal_signal_pending() -> bool;
    fn current() -> *mut core::ffi::c_void; fn mmap_read_lock_killable(mm: *mut mm_struct) -> c_int;
    fn mmap_read_unlock(mm: *mut mm_struct); fn check_stable_address_space(mm: *mut mm_struct) -> bool;
    fn mmu_interval_read_begin(n: *mut crate::mmu_interval_notifier) -> u64;
    fn jiffies() -> c_ulong; fn time_after(a: c_ulong, b: c_ulong) -> bool;
}

// Constants and the mm_walk_ops initializer are supplied by the kernel binding.
extern "C" { static hmm_walk_ops: crate::mm_walk_ops; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
