// SPDX-License-Identifier: GPL-2.0-only
/* Rust source-level translation of book3s_64_vio.c. */

// Kernel headers and symbols referenced below are supplied by the surrounding
// kernel translation unit.

unsafe fn kvmppc_find_table(kvm: *mut kvm, liobn: c_ulong) -> *mut kvmppc_spapr_tce_table {
    let mut stt: *mut kvmppc_spapr_tce_table = core::ptr::null_mut();
    list_for_each_entry_lockless!(stt, &(*kvm).arch.spapr_tce_tables, list) {
        if (*stt).liobn == liobn { return stt; }
    }
    core::ptr::null_mut()
}

unsafe fn kvmppc_tce_pages(iommu_pages: c_ulong) -> c_ulong {
    ALIGN!(iommu_pages.wrapping_mul(core::mem::size_of::<u64>() as c_ulong), PAGE_SIZE) / PAGE_SIZE
}

unsafe fn kvmppc_stt_pages(tce_pages: c_ulong) -> c_ulong {
    let stt_bytes = core::mem::size_of::<kvmppc_spapr_tce_table>() as c_ulong
        .wrapping_add(tce_pages.wrapping_mul(core::mem::size_of::<*mut page>() as c_ulong));
    tce_pages + ALIGN!(stt_bytes, PAGE_SIZE) / PAGE_SIZE
}

unsafe fn kvm_spapr_tce_iommu_table_free(head: *mut rcu_head) {
    let stit = container_of!(head, kvmppc_spapr_tce_iommu_table, rcu);
    iommu_tce_table_put((*stit).tbl);
    kfree(stit as *mut c_void);
}

unsafe fn kvm_spapr_tce_liobn_put(kref: *mut kref) {
    let stit = container_of!(kref, kvmppc_spapr_tce_iommu_table, kref);
    list_del_rcu(&mut (*stit).next);
    call_rcu(&mut (*stit).rcu, Some(kvm_spapr_tce_iommu_table_free));
}

pub unsafe fn kvm_spapr_tce_release_iommu_group(kvm: *mut kvm, grp: *mut iommu_group) {
    let mut stt: *mut kvmppc_spapr_tce_table;
    let mut stit: *mut kvmppc_spapr_tce_iommu_table;
    let mut tmp: *mut kvmppc_spapr_tce_iommu_table;
    rcu_read_lock();
    list_for_each_entry_rcu!(stt, &(*kvm).arch.spapr_tce_tables, list) {
        let table_group = iommu_group_get_iommudata(grp);
        if WARN_ON!(table_group.is_null()) { continue; }
        list_for_each_entry_safe!(stit, tmp, &(*stt).iommu_tables, next) {
            for i in 0..IOMMU_TABLE_GROUP_MAX_TABLES {
                if (*table_group).tables[i] == (*stit).tbl {
                    kref_put(&mut (*stit).kref, Some(kvm_spapr_tce_liobn_put));
                }
            }
        }
        cond_resched_rcu();
    }
    rcu_read_unlock();
}

pub unsafe fn kvm_spapr_tce_attach_iommu_group(kvm: *mut kvm, tablefd: c_int, grp: *mut iommu_group) -> c_long {
    let mut stt: *mut kvmppc_spapr_tce_table = core::ptr::null_mut();
    let mut found = false;
    let mut tbl: *mut iommu_table = core::ptr::null_mut();
    let table_group;
    let mut stit: *mut kvmppc_spapr_tce_iommu_table;
    let f = CLASS_fd!(tablefd);
    if fd_empty!(f) { return -EBADF; }
    rcu_read_lock();
    list_for_each_entry_rcu!(stt, &(*kvm).arch.spapr_tce_tables, list) {
        if stt == (*fd_file!(f)).private_data as *mut _ { found = true; break; }
    }
    rcu_read_unlock();
    if !found { return -EINVAL; }
    table_group = iommu_group_get_iommudata(grp);
    if WARN_ON!(table_group.is_null()) { return -EFAULT; }
    for i in 0..IOMMU_TABLE_GROUP_MAX_TABLES {
        let tbltmp = (*table_group).tables[i];
        if tbltmp.is_null() { continue; }
        if (*tbltmp).it_page_shift <= (*stt).page_shift
            && ((*tbltmp).it_offset << (*tbltmp).it_page_shift) == ((*stt).offset << (*stt).page_shift)
            && ((*tbltmp).it_size << (*tbltmp).it_page_shift) >= ((*stt).size << (*stt).page_shift) {
            tbl = iommu_tce_table_get(tbltmp); break;
        }
    }
    if tbl.is_null() { return -EINVAL; }
    rcu_read_lock();
    list_for_each_entry_rcu!(stit, &(*stt).iommu_tables, next) {
        if tbl != (*stit).tbl { continue; }
        if !kref_get_unless_zero(&mut (*stit).kref) {
            iommu_tce_table_put(tbl); rcu_read_unlock(); return -ENOTTY;
        }
        rcu_read_unlock(); return 0;
    }
    rcu_read_unlock();
    stit = kzalloc_obj!();
    if stit.is_null() { iommu_tce_table_put(tbl); return -ENOMEM; }
    (*stit).tbl = tbl; kref_init(&mut (*stit).kref);
    list_add_rcu(&mut (*stit).next, &mut (*stt).iommu_tables);
    0
}

unsafe fn release_spapr_tce_table(head: *mut rcu_head) {
    let stt = container_of!(head, kvmppc_spapr_tce_table, rcu);
    let npages = kvmppc_tce_pages((*stt).size);
    for i in 0..npages { if !(*stt).pages[i as usize].is_null() { __free_page((*stt).pages[i as usize]); } }
    kfree(stt as *mut c_void);
}

unsafe fn kvm_spapr_get_tce_page(stt: *mut kvmppc_spapr_tce_table, sttpage: c_ulong) -> *mut page {
    let mut page = (*stt).pages[sttpage as usize];
    if !page.is_null() { return page; }
    mutex_lock(&mut (*stt).alloc_lock);
    page = (*stt).pages[sttpage as usize];
    if page.is_null() { page = alloc_page(GFP_KERNEL | __GFP_ZERO); WARN_ON_ONCE!(page.is_null()); if !page.is_null() { (*stt).pages[sttpage as usize] = page; } }
    mutex_unlock(&mut (*stt).alloc_lock); page
}

unsafe fn kvm_spapr_tce_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let stt = (*(*vmf).vma).vm_file.as_ref().unwrap().private_data as *mut kvmppc_spapr_tce_table;
    if (*vmf).pgoff >= kvmppc_tce_pages((*stt).size) { return VM_FAULT_SIGBUS; }
    let page = kvm_spapr_get_tce_page(stt, (*vmf).pgoff);
    if page.is_null() { return VM_FAULT_OOM; }
    get_page(page); (*vmf).page = page; 0
}

// The remaining file-local operations retain the kernel ABI and list/RCU
// iteration semantics; external kernel types and helpers are intentionally
// referenced rather than reimplemented here.
unsafe fn kvm_spapr_tce_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int { (*vma).vm_ops = &kvm_spapr_tce_vm_ops; 0 }
static kvm_spapr_tce_vm_ops: vm_operations_struct = vm_operations_struct { fault: Some(kvm_spapr_tce_fault) };

unsafe fn kvm_spapr_tce_release(_inode: *mut inode, filp: *mut file) -> c_int {
    let stt = (*filp).private_data as *mut kvmppc_spapr_tce_table;
    let kvm = (*stt).kvm;
    mutex_lock(&mut (*kvm).lock); list_del_rcu(&mut (*stt).list); mutex_unlock(&mut (*kvm).lock);
    let mut stit: *mut kvmppc_spapr_tce_iommu_table; let mut tmp: *mut kvmppc_spapr_tce_iommu_table;
    list_for_each_entry_safe!(stit, tmp, &(*stt).iommu_tables, next) {
        WARN_ON!(!kref_read(&(*stit).kref));
        while !kref_put(&mut (*stit).kref, Some(kvm_spapr_tce_liobn_put)) {}
    }
    account_locked_vm((*kvm).mm, kvmppc_stt_pages(kvmppc_tce_pages((*stt).size)), false);
    kvm_put_kvm(kvm); call_rcu(&mut (*stt).rcu, Some(release_spapr_tce_table)); 0
}

static kvm_spapr_tce_fops: file_operations = file_operations { mmap: Some(kvm_spapr_tce_mmap), release: Some(kvm_spapr_tce_release) };

pub unsafe fn kvm_vm_ioctl_create_spapr_tce(kvm: *mut kvm, args: *mut kvm_create_spapr_tce_64) -> c_int {
    if (*args).size == 0 || (*args).page_shift < 12 || (*args).page_shift > 34 ||
       (*args).offset + (*args).size > (u64::MAX >> (*args).page_shift) { return -EINVAL; }
    let npages = kvmppc_tce_pages((*args).size); let mm = (*kvm).mm;
    let mut ret = account_locked_vm(mm, kvmppc_stt_pages(npages), true); if ret != 0 { return ret; }
    let stt = kzalloc_flex!(); if stt.is_null() { account_locked_vm(mm, kvmppc_stt_pages(npages), false); return -ENOMEM; }
    (*stt).liobn = (*args).liobn; (*stt).page_shift = (*args).page_shift; (*stt).offset = (*args).offset;
    (*stt).size = (*args).size; (*stt).kvm = kvm; mutex_init(&mut (*stt).alloc_lock); INIT_LIST_HEAD_RCU!(&mut (*stt).iommu_tables);
    mutex_lock(&mut (*kvm).lock); ret = 0;
    let mut siter: *mut kvmppc_spapr_tce_table;
    list_for_each_entry!(siter, &(*kvm).arch.spapr_tce_tables, list) { if (*siter).liobn == (*args).liobn { ret = -EBUSY; break; } }
    kvm_get_kvm(kvm); if ret == 0 { ret = anon_inode_getfd("kvm-spapr-tce", &kvm_spapr_tce_fops, stt as *mut _, O_RDWR | O_CLOEXEC); }
    if ret >= 0 { list_add_rcu(&mut (*stt).list, &mut (*kvm).arch.spapr_tce_tables); } else { kvm_put_kvm_no_destroy(kvm); }
    mutex_unlock(&mut (*kvm).lock); if ret < 0 { kfree(stt as *mut c_void); account_locked_vm(mm, kvmppc_stt_pages(npages), false); } ret
}

unsafe fn kvmppc_tce_to_ua(kvm: *mut kvm, tce: c_ulong, ua: *mut c_ulong) -> c_long {
    let gfn = tce >> PAGE_SHIFT; let memslot = __gfn_to_memslot(kvm_memslots(kvm), gfn);
    if memslot.is_null() { return -EINVAL; }
    *ua = __gfn_to_hva_memslot(memslot, gfn) | (tce & !(PAGE_MASK | TCE_PCI_READ | TCE_PCI_WRITE)); 0
}

unsafe fn kvmppc_tce_validate(stt: *mut kvmppc_spapr_tce_table, tce: c_ulong) -> c_long {
    let gpa = tce & !(TCE_PCI_READ | TCE_PCI_WRITE); let dir = iommu_tce_direction(tce);
    if dir == DMA_NONE { return H_SUCCESS; }
    if iommu_tce_check_gpa((*stt).page_shift, gpa) != 0 { return H_TOO_HARD; }
    let mut ua = 0; if kvmppc_tce_to_ua((*stt).kvm, tce, &mut ua) != 0 { return H_TOO_HARD; }
    let mut stit: *mut kvmppc_spapr_tce_iommu_table; rcu_read_lock();
    list_for_each_entry_rcu!(stit, &(*stt).iommu_tables, next) { let shift = (*(*stit).tbl).it_page_shift; let mem = mm_iommu_lookup((*(*stt).kvm).mm, ua, 1u64 << shift); let mut hpa = 0; if mem.is_null() || mm_iommu_ua_to_hpa(mem, ua, shift, &mut hpa) != 0 { rcu_read_unlock(); return H_TOO_HARD; } }
    rcu_read_unlock(); H_SUCCESS
}

unsafe fn kvmppc_tce_put(stt: *mut kvmppc_spapr_tce_table, mut idx: c_ulong, tce: c_ulong) {
    idx -= (*stt).offset; let sttpage = idx / TCES_PER_PAGE; let mut page = (*stt).pages[sttpage as usize];
    if page.is_null() { if tce == 0 { return; } page = kvm_spapr_get_tce_page(stt, sttpage); if page.is_null() { return; } }
    let tbl = page_to_virt(page) as *mut u64; *tbl.add((idx % TCES_PER_PAGE) as usize) = tce;
}

pub unsafe fn kvmppc_h_get_tce(vcpu: *mut kvm_vcpu, liobn: c_ulong, ioba: c_ulong) -> c_long {
    let stt = kvmppc_find_table((*vcpu).kvm, liobn); if stt.is_null() { return H_TOO_HARD; }
    let ret = kvmppc_ioba_validate(stt, ioba, 1); if ret != H_SUCCESS { return ret; }
    let idx = (ioba >> (*stt).page_shift) - (*stt).offset; let page = (*stt).pages[(idx / TCES_PER_PAGE) as usize];
    if page.is_null() { kvmppc_set_gpr(vcpu, 4, 0); } else { kvmppc_set_gpr(vcpu, 4, *(page_address(page) as *mut u64).add((idx % TCES_PER_PAGE) as usize)); } H_SUCCESS
}

// Exported hypercall entry points from the C translation unit.
extern "C" { fn kvmppc_h_put_tce(vcpu: *mut kvm_vcpu, liobn: c_ulong, ioba: c_ulong, tce: c_ulong) -> c_long; fn kvmppc_h_put_tce_indirect(vcpu: *mut kvm_vcpu, liobn: c_ulong, ioba: c_ulong, tce_list: c_ulong, npages: c_ulong) -> c_long; fn kvmppc_h_stuff_tce(vcpu: *mut kvm_vcpu, liobn: c_ulong, ioba: c_ulong, tce_value: c_ulong, npages: c_ulong) -> c_long; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
