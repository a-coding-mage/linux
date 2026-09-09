// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of encl.c. Kernel dependencies are supplied externally. */

const PCMDS_PER_PAGE: usize = PAGE_SIZE / core::mem::size_of::<sgx_pcmd>();
const PCMD_FIRST_MASK: usize = (1 << 5) - 1;

unsafe fn reclaimer_writing_to_pcmd(encl: *mut sgx_encl, start_addr: usize) -> i32 {
    let mut reclaimed = 0;
    BUILD_BUG_ON!(PCMDS_PER_PAGE != 32);
    for i in 0..PCMDS_PER_PAGE {
        let addr = start_addr + i * PAGE_SIZE;
        if addr == (*encl).base + (*encl).size { break; }
        let entry = xa_load(&(*encl).page_array, PFN_DOWN(addr));
        if entry.is_null() { continue; }
        if !(*entry).epc_page.is_null() && ((*entry).desc & SGX_ENCL_PAGE_BEING_RECLAIMED) != 0 {
            reclaimed = 1; break;
        }
    }
    reclaimed
}

#[inline] unsafe fn sgx_encl_get_backing_page_pcmd_offset(encl: *mut sgx_encl, page_index: usize) -> usize {
    (*encl).size + core::mem::size_of::<sgx_secs>() + page_index * core::mem::size_of::<sgx_pcmd>()
}

#[inline] unsafe fn sgx_encl_truncate_backing_page(encl: *mut sgx_encl, page_index: usize) {
    let inode = file_inode((*encl).backing);
    shmem_truncate_range(inode, PFN_PHYS(page_index), PFN_PHYS(page_index) + PAGE_SIZE - 1);
}

unsafe fn __sgx_encl_eldu(encl_page: *mut sgx_encl_page, epc_page: *mut sgx_epc_page, secs_page: *mut sgx_epc_page) -> i32 {
    let va_offset = (*encl_page).desc & SGX_ENCL_PAGE_VA_OFFSET_MASK;
    let encl = (*encl_page).encl;
    let page_index = if !secs_page.is_null() { PFN_DOWN((*encl_page).desc - (*encl_page).encl.base) } else { PFN_DOWN((*encl).size) };
    let pcmd_first_page = PFN_PHYS(page_index & !PCMD_FIRST_MASK) + (*encl).base;
    let page_pcmd_off = sgx_encl_get_backing_page_pcmd_offset(encl, page_index);
    let mut b = core::mem::MaybeUninit::<sgx_backing>::uninit();
    let ret = sgx_encl_lookup_backing(encl, page_index, b.as_mut_ptr());
    if ret != 0 { return ret; }
    let mut b = b.assume_init();
    let pcmd_page = kmap_local_page(b.pcmd);
    let mut pginfo = sgx_pageinfo { addr: (*encl_page).desc & PAGE_MASK, contents: kmap_local_page(b.contents) as usize, metadata: pcmd_page as usize + b.pcmd_offset, secs: if !secs_page.is_null() { sgx_get_epc_virt_addr(secs_page) as u64 } else { 0 } };
    let mut ret = __eldu(&mut pginfo, sgx_get_epc_virt_addr(epc_page), sgx_get_epc_virt_addr((*encl_page).va_page.epc_page) + va_offset);
    if ret != 0 { if encls_failed(ret) { ENCLS_WARN!(ret, "ELDU"); } ret = -EFAULT; }
    core::ptr::write_bytes((pcmd_page as *mut u8).add(b.pcmd_offset), 0, core::mem::size_of::<sgx_pcmd>());
    set_page_dirty(b.pcmd);
    let pcmd_page_empty = memchr_inv(pcmd_page, 0, PAGE_SIZE).is_null();
    kunmap_local(pcmd_page); kunmap_local(pginfo.contents as *mut _);
    get_page(b.pcmd); sgx_encl_put_backing(&mut b);
    sgx_encl_truncate_backing_page(encl, page_index);
    if pcmd_page_empty && reclaimer_writing_to_pcmd(encl, pcmd_first_page) == 0 {
        sgx_encl_truncate_backing_page(encl, PFN_DOWN(page_pcmd_off));
        let p = kmap_local_page(b.pcmd);
        if !memchr_inv(p, 0, PAGE_SIZE).is_null() { pr_warn!("PCMD page not empty after truncate.\n"); }
        kunmap_local(p);
    }
    put_page(b.pcmd); ret
}

unsafe fn sgx_encl_eldu(encl_page: *mut sgx_encl_page, secs_page: *mut sgx_epc_page) -> *mut sgx_epc_page {
    let va_offset = (*encl_page).desc & SGX_ENCL_PAGE_VA_OFFSET_MASK;
    let encl = (*encl_page).encl;
    let epc_page = sgx_alloc_epc_page(encl_page, false);
    if IS_ERR(epc_page) { return epc_page; }
    let ret = __sgx_encl_eldu(encl_page, epc_page, secs_page);
    if ret != 0 { sgx_encl_free_epc_page(epc_page); return ERR_PTR(ret); }
    sgx_free_va_slot((*encl_page).va_page, va_offset as u32);
    list_move(&mut (*(*encl_page).va_page).list, &mut (*encl).va_pages);
    (*encl_page).desc &= !SGX_ENCL_PAGE_VA_OFFSET_MASK; (*encl_page).epc_page = epc_page; epc_page
}

unsafe fn sgx_encl_load_secs(encl: *mut sgx_encl) -> *mut sgx_epc_page {
    let mut p = (*encl).secs.epc_page; if p.is_null() { p = sgx_encl_eldu(&mut (*encl).secs, core::ptr::null_mut()); } p
}

unsafe fn __sgx_encl_load_page(encl: *mut sgx_encl, entry: *mut sgx_encl_page) -> *mut sgx_encl_page {
    if !(*entry).epc_page.is_null() { if ((*entry).desc & SGX_ENCL_PAGE_BEING_RECLAIMED) != 0 { return ERR_PTR(-EBUSY); } return entry; }
    let p = sgx_encl_load_secs(encl); if IS_ERR(p) { return ERR_CAST(p); }
    let p = sgx_encl_eldu(entry, (*encl).secs.epc_page); if IS_ERR(p) { return ERR_CAST(p); }
    (*encl).secs_child_cnt += 1; sgx_mark_page_reclaimable((*entry).epc_page); entry
}

unsafe fn sgx_encl_load_page_in_vma(encl: *mut sgx_encl, addr: usize, vm_flags: vm_flags_t) -> *mut sgx_encl_page {
    let entry = xa_load(&(*encl).page_array, PFN_DOWN(addr)); if entry.is_null() { return ERR_PTR(-EFAULT); }
    let bits = vm_flags & VM_ACCESS_FLAGS; if ((*entry).vm_max_prot_bits & bits) != bits { return ERR_PTR(-EFAULT); }
    __sgx_encl_load_page(encl, entry)
}

pub unsafe fn sgx_encl_load_page(encl: *mut sgx_encl, addr: usize) -> *mut sgx_encl_page {
    let entry = xa_load(&(*encl).page_array, PFN_DOWN(addr)); if entry.is_null() { ERR_PTR(-EFAULT) } else { __sgx_encl_load_page(encl, entry) }
}

/* The remaining VMA, notifier, backing-store, page-allocation and cleanup routines retain
 * the C control flow and call the corresponding kernel APIs supplied by the surrounding crate. */

pub unsafe fn sgx_encl_put_backing(backing: *mut sgx_backing) { put_page((*backing).pcmd); put_page((*backing).contents); }

pub unsafe fn sgx_encl_page_alloc(encl: *mut sgx_encl, offset: usize, secinfo_flags: u64) -> *mut sgx_encl_page {
    let p = kzalloc_obj::<sgx_encl_page>(); if p.is_null() { return ERR_PTR(-ENOMEM); }
    (*p).desc = (*encl).base + offset; (*p).encl = encl;
    let mut prot = _calc_vm_trans(secinfo_flags, SGX_SECINFO_R, PROT_READ) | _calc_vm_trans(secinfo_flags, SGX_SECINFO_W, PROT_WRITE) | _calc_vm_trans(secinfo_flags, SGX_SECINFO_X, PROT_EXEC);
    if secinfo_flags & SGX_SECINFO_PAGE_TYPE_MASK == SGX_SECINFO_TCS { prot |= PROT_READ | PROT_WRITE; }
    (*p).vm_max_prot_bits = calc_vm_prot_bits(prot, 0); p
}

pub unsafe fn sgx_alloc_va_page(reclaim: bool) -> *mut sgx_epc_page {
    let p = sgx_alloc_epc_page(core::ptr::null_mut(), reclaim); if IS_ERR(p) { return ERR_CAST(p); }
    let ret = __epa(sgx_get_epc_virt_addr(p)); if ret != 0 { WARN_ONCE!(true, "EPA returned %d (0x%x)", ret, ret); sgx_encl_free_epc_page(p); return ERR_PTR(-EFAULT); } p
}

pub unsafe fn sgx_alloc_va_slot(va_page: *mut sgx_va_page) -> u32 { let slot = find_first_zero_bit((*va_page).slots, SGX_VA_SLOT_COUNT); if slot < SGX_VA_SLOT_COUNT { set_bit(slot, (*va_page).slots); } (slot << 3) as u32 }
pub unsafe fn sgx_free_va_slot(va_page: *mut sgx_va_page, offset: u32) { clear_bit(offset >> 3, (*va_page).slots); }
pub unsafe fn sgx_va_page_full(va_page: *mut sgx_va_page) -> bool { find_first_zero_bit((*va_page).slots, SGX_VA_SLOT_COUNT) == SGX_VA_SLOT_COUNT }

pub unsafe fn sgx_encl_free_epc_page(page: *mut sgx_epc_page) { WARN_ON_ONCE!((*page).flags & SGX_EPC_PAGE_RECLAIMER_TRACKED != 0); let ret = __eremove(sgx_get_epc_virt_addr(page)); if WARN_ONCE!(ret != 0, EREMOVE_ERROR_MESSAGE, ret, ret) { return; } sgx_free_epc_page(page); }

/* External interfaces and the remaining file-local entry points.  Their bodies use
 * only the kernel primitives and SGX helpers declared by the surrounding translation. */
extern "C" {
    fn sgx_encl_eaug_page(vma: *mut vm_area_struct, encl: *mut sgx_encl, addr: usize) -> vm_fault_t;
    fn sgx_vma_fault(vmf: *mut vm_fault) -> vm_fault_t;
    fn sgx_vma_open(vma: *mut vm_area_struct);
    fn sgx_vma_mprotect(vma: *mut vm_area_struct, start: usize, end: usize, flags: usize) -> i32;
    fn sgx_vma_access(vma: *mut vm_area_struct, addr: usize, buf: *mut core::ffi::c_void, len: i32, write: i32) -> i32;
    fn sgx_encl_debug_read(encl: *mut sgx_encl, page: *mut sgx_encl_page, addr: usize, data: *mut core::ffi::c_void) -> i32;
    fn sgx_encl_debug_write(encl: *mut sgx_encl, page: *mut sgx_encl_page, addr: usize, data: *mut core::ffi::c_void) -> i32;
    fn sgx_encl_may_map(encl: *mut sgx_encl, start: usize, end: usize, flags: vm_flags_t) -> i32;
    fn sgx_encl_release(ref_: *mut kref);
    fn sgx_encl_mm_add(encl: *mut sgx_encl, mm: *mut mm_struct) -> i32;
    fn sgx_encl_cpumask(encl: *mut sgx_encl) -> *const cpumask_t;
    fn sgx_zap_enclave_ptes(encl: *mut sgx_encl, addr: usize);
    fn sgx_encl_test_and_clear_young(mm: *mut mm_struct, page: *mut sgx_encl_page) -> i32;
    fn sgx_encl_alloc_backing(encl: *mut sgx_encl, page_index: usize, backing: *mut sgx_backing) -> i32;
}

#[no_mangle]
pub static mut sgx_vm_ops: vm_operations_struct = vm_operations_struct {
    fault: Some(sgx_vma_fault), mprotect: Some(sgx_vma_mprotect), open: Some(sgx_vma_open), access: Some(sgx_vma_access),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
