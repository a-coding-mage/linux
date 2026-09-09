// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2016-20 Intel Corporation. */
// Kernel dependencies supplied by the surrounding Rust translation unit.

#[no_mangle]
pub static mut sgx_epc_sections: [sgx_epc_section; SGX_MAX_EPC_SECTIONS] = [sgx_epc_section::ZERO; SGX_MAX_EPC_SECTIONS];
static mut sgx_nr_epc_sections: i32 = 0;
static mut ksgxd_tsk: *mut task_struct = core::ptr::null_mut();
static mut ksgxd_waitq: wait_queue_head = wait_queue_head::ZERO;
static mut sgx_epc_address_space: xarray = xarray::ZERO;
static mut sgx_active_page_list: list_head = list_head::ZERO;
static mut sgx_reclaimer_lock: spinlock_t = spinlock_t::ZERO;
static mut sgx_nr_free_pages: atomic_long_t = atomic_long_t::ZERO;
static mut sgx_numa_mask: nodemask_t = nodemask_t::ZERO;
static mut sgx_numa_nodes: *mut sgx_numa_node = core::ptr::null_mut();
static mut sgx_dirty_page_list: list_head = list_head::ZERO;

unsafe fn __sgx_sanitize_pages(dirty_page_list: *mut list_head) -> c_ulong {
    let mut left_dirty: c_ulong = 0;
    let mut dirty = list_head::ZERO;
    while !list_empty(dirty_page_list) {
        if kthread_should_stop() { return 0; }
        let page = list_first_entry::<sgx_epc_page>(dirty_page_list);
        if (*page).poison != 0 {
            let section = &mut sgx_epc_sections[(*page).section as usize];
            let node = section.node;
            spin_lock(&mut (*node).lock);
            list_move(&mut (*page).list, &mut (*node).sgx_poison_page_list);
            spin_unlock(&mut (*node).lock);
            continue;
        }
        let ret = __eremove(sgx_get_epc_virt_addr(page));
        if ret == 0 { list_del(&mut (*page).list); sgx_free_epc_page(page); }
        else { list_move_tail(&mut (*page).list, &mut dirty); left_dirty += 1; }
        cond_resched();
    }
    list_splice(&mut dirty, dirty_page_list);
    left_dirty
}

unsafe fn sgx_reclaimer_age(epc_page: *mut sgx_epc_page) -> bool {
    let page = (*epc_page).owner as *mut sgx_encl_page;
    let encl = (*page).encl;
    let mut ret = true;
    let idx = srcu_read_lock(&mut (*encl).srcu);
    let mut encl_mm: *mut sgx_encl_mm = core::ptr::null_mut();
    list_for_each_entry_rcu!(encl_mm, &mut (*encl).mm_list, list, {
        if !mmget_not_zero((*encl_mm).mm) { continue; }
        mmap_read_lock((*encl_mm).mm);
        ret = !sgx_encl_test_and_clear_young((*encl_mm).mm, page);
        mmap_read_unlock((*encl_mm).mm);
        mmput_async((*encl_mm).mm);
        if !ret { break; }
    });
    srcu_read_unlock(&mut (*encl).srcu, idx);
    ret
}

unsafe fn sgx_reclaimer_block(epc_page: *mut sgx_epc_page) {
    let page = (*epc_page).owner as *mut sgx_encl_page;
    let addr = (*page).desc & PAGE_MASK;
    let encl = (*page).encl;
    sgx_zap_enclave_ptes(encl, addr);
    mutex_lock(&mut (*encl).lock);
    let ret = __eblock(sgx_get_epc_virt_addr(epc_page));
    if encls_failed(ret) { ENCLS_WARN(ret, "EBLOCK"); }
    mutex_unlock(&mut (*encl).lock);
}

unsafe fn __sgx_encl_ewb(epc_page: *mut sgx_epc_page, va_slot: *mut c_void, backing: *mut sgx_backing) -> c_int {
    let mut pginfo = sgx_pageinfo { addr: 0, contents: 0, metadata: 0, secs: 0 };
    pginfo.contents = kmap_local_page((*backing).contents) as c_ulong;
    pginfo.metadata = kmap_local_page((*backing).pcmd) as c_ulong + (*backing).pcmd_offset;
    let ret = __ewb(&mut pginfo, sgx_get_epc_virt_addr(epc_page), va_slot);
    set_page_dirty((*backing).pcmd); set_page_dirty((*backing).contents);
    kunmap_local((pginfo.metadata - (*backing).pcmd_offset) as *mut c_void);
    kunmap_local(pginfo.contents as *mut c_void); ret
}

#[no_mangle] pub unsafe extern "C" fn sgx_ipi_cb(_info: *mut c_void) {}

unsafe fn sgx_encl_ewb(epc_page: *mut sgx_epc_page, backing: *mut sgx_backing) {
    let encl_page = (*epc_page).owner as *mut sgx_encl_page;
    let encl = (*encl_page).encl;
    let va_page = list_first_entry::<sgx_va_page>(&mut (*encl).va_pages);
    let va_offset = sgx_alloc_va_slot(va_page);
    let va_slot = (sgx_get_epc_virt_addr((*va_page).epc_page) as usize + va_offset as usize) as *mut c_void;
    if sgx_va_page_full(va_page) { list_move_tail(&mut (*va_page).list, &mut (*encl).va_pages); }
    (*encl_page).desc &= !SGX_ENCL_PAGE_BEING_RECLAIMED;
    let mut ret = __sgx_encl_ewb(epc_page, va_slot, backing);
    if ret == SGX_NOT_TRACKED {
        ret = __etrack(sgx_get_epc_virt_addr((*encl).secs.epc_page));
        if ret != 0 && encls_failed(ret) { ENCLS_WARN(ret, "ETRACK"); }
        ret = __sgx_encl_ewb(epc_page, va_slot, backing);
        if ret == SGX_NOT_TRACKED { on_each_cpu_mask(sgx_encl_cpumask(encl), sgx_ipi_cb, core::ptr::null_mut(), 1); ret = __sgx_encl_ewb(epc_page, va_slot, backing); }
    }
    if ret != 0 { if encls_failed(ret) { ENCLS_WARN(ret, "EWB"); } sgx_free_va_slot(va_page, va_offset); }
    else { (*encl_page).desc |= va_offset; (*encl_page).va_page = va_page; }
}

unsafe fn sgx_reclaimer_write(epc_page: *mut sgx_epc_page, backing: *mut sgx_backing) {
    let encl_page = (*epc_page).owner as *mut sgx_encl_page; let encl = (*encl_page).encl;
    let mut secs_backing = sgx_backing::ZERO;
    mutex_lock(&mut (*encl).lock); sgx_encl_ewb(epc_page, backing); (*encl_page).epc_page = core::ptr::null_mut(); (*encl).secs_child_cnt -= 1; sgx_encl_put_backing(backing);
    if (*encl).secs_child_cnt == 0 && test_bit(SGX_ENCL_INITIALIZED, &(*encl).flags) {
        let ret = sgx_encl_alloc_backing(encl, PFN_DOWN((*encl).size), &mut secs_backing); if ret != 0 { mutex_unlock(&mut (*encl).lock); return; }
        sgx_encl_ewb((*encl).secs.epc_page, &mut secs_backing); sgx_encl_free_epc_page((*encl).secs.epc_page); (*encl).secs.epc_page = core::ptr::null_mut(); sgx_encl_put_backing(&mut secs_backing);
    } mutex_unlock(&mut (*encl).lock);
}

unsafe fn sgx_reclaim_pages() {
    let mut chunk: [*mut sgx_epc_page; SGX_NR_TO_SCAN] = [core::ptr::null_mut(); SGX_NR_TO_SCAN]; let mut backing: [sgx_backing; SGX_NR_TO_SCAN] = [sgx_backing::ZERO; SGX_NR_TO_SCAN]; let mut cnt = 0;
    spin_lock(&mut sgx_reclaimer_lock);
    for _ in 0..SGX_NR_TO_SCAN { if list_empty(&mut sgx_active_page_list) { break; } let epc = list_first_entry::<sgx_epc_page>(&mut sgx_active_page_list); list_del_init(&mut (*epc).list); let page = (*epc).owner as *mut sgx_encl_page; if kref_get_unless_zero(&mut (*(*page).encl).refcount) != 0 { chunk[cnt] = epc; cnt += 1; } else { (*epc).flags &= !SGX_EPC_PAGE_RECLAIMER_TRACKED; } }
    spin_unlock(&mut sgx_reclaimer_lock);
    for i in 0..cnt { let epc = chunk[i]; let page = (*epc).owner as *mut sgx_encl_page; if !sgx_reclaimer_age(epc) { spin_lock(&mut sgx_reclaimer_lock); list_add_tail(&mut (*epc).list, &mut sgx_active_page_list); spin_unlock(&mut sgx_reclaimer_lock); kref_put(&mut (*(*page).encl).refcount, sgx_encl_release); chunk[i] = core::ptr::null_mut(); continue; } let index = PFN_DOWN((*page).desc - (*(*page).encl).base); mutex_lock(&mut (*(*page).encl).lock); if sgx_encl_alloc_backing((*page).encl, index, &mut backing[i]) != 0 { mutex_unlock(&mut (*(*page).encl).lock); continue; } (*page).desc |= SGX_ENCL_PAGE_BEING_RECLAIMED; mutex_unlock(&mut (*(*page).encl).lock); }
    for i in 0..cnt { if !chunk[i].is_null() { sgx_reclaimer_block(chunk[i]); } }
    for i in 0..cnt { if chunk[i].is_null() { continue; } let page = (*chunk[i]).owner as *mut sgx_encl_page; sgx_reclaimer_write(chunk[i], &mut backing[i]); kref_put(&mut (*(*page).encl).refcount, sgx_encl_release); (*chunk[i]).flags &= !SGX_EPC_PAGE_RECLAIMER_TRACKED; sgx_free_epc_page(chunk[i]); }
}

unsafe fn sgx_should_reclaim(watermark: c_ulong) -> bool { atomic_long_read(&sgx_nr_free_pages) < watermark && !list_empty(&mut sgx_active_page_list) }
pub unsafe extern "C" fn sgx_reclaim_direct() { if sgx_should_reclaim(SGX_NR_LOW_PAGES) { sgx_reclaim_pages(); } }

unsafe extern "C" fn ksgxd(_p: *mut c_void) -> c_int {
    set_freezable(); __sgx_sanitize_pages(&mut sgx_dirty_page_list); WARN_ON(__sgx_sanitize_pages(&mut sgx_dirty_page_list));
    while !kthread_should_stop() { if try_to_freeze() { continue; } wait_event_freezable!(&mut ksgxd_waitq, kthread_should_stop() || sgx_should_reclaim(SGX_NR_HIGH_PAGES)); if sgx_should_reclaim(SGX_NR_HIGH_PAGES) { sgx_reclaim_pages(); } cond_resched(); } 0
}

unsafe fn sgx_page_reclaimer_init() -> bool { let tsk = kthread_run(ksgxd, core::ptr::null_mut(), "ksgxd"); if IS_ERR(tsk) { return false; } ksgxd_tsk = tsk; true }
pub unsafe extern "C" fn current_is_ksgxd() -> bool { current == ksgxd_tsk }

unsafe fn __sgx_alloc_epc_page_from_node(nid: c_int) -> *mut sgx_epc_page { let node = &mut *sgx_numa_nodes.add(nid as usize); spin_lock(&mut node.lock); if list_empty(&mut node.free_page_list) { spin_unlock(&mut node.lock); return core::ptr::null_mut(); } let page = list_first_entry::<sgx_epc_page>(&mut node.free_page_list); list_del_init(&mut (*page).list); (*page).flags = 0; spin_unlock(&mut node.lock); atomic_long_dec(&mut sgx_nr_free_pages); page }

pub unsafe extern "C" fn __sgx_alloc_epc_page() -> *mut sgx_epc_page { let current_nid = numa_node_id(); let start = if node_isset(current_nid, &sgx_numa_mask) { current_nid } else { next_node_in(current_nid, &sgx_numa_mask) }; let mut nid = start; loop { let page = __sgx_alloc_epc_page_from_node(nid); if !page.is_null() { return page; } nid = next_node_in(nid, &sgx_numa_mask); if nid == start { break; } } ERR_PTR(-ENOMEM) }

pub unsafe extern "C" fn sgx_mark_page_reclaimable(page: *mut sgx_epc_page) { spin_lock(&mut sgx_reclaimer_lock); (*page).flags |= SGX_EPC_PAGE_RECLAIMER_TRACKED; list_add_tail(&mut (*page).list, &mut sgx_active_page_list); spin_unlock(&mut sgx_reclaimer_lock); }
pub unsafe extern "C" fn sgx_unmark_page_reclaimable(page: *mut sgx_epc_page) -> c_int { spin_lock(&mut sgx_reclaimer_lock); if (*page).flags & SGX_EPC_PAGE_RECLAIMER_TRACKED != 0 { if list_empty(&mut (*page).list) { spin_unlock(&mut sgx_reclaimer_lock); return -EBUSY; } list_del(&mut (*page).list); (*page).flags &= !SGX_EPC_PAGE_RECLAIMER_TRACKED; } spin_unlock(&mut sgx_reclaimer_lock); 0 }

pub unsafe extern "C" fn sgx_alloc_epc_page(owner: *mut c_void, reclaim: bool) -> *mut sgx_epc_page { loop { let page = __sgx_alloc_epc_page(); if !IS_ERR(page) { (*page).owner = owner; if sgx_should_reclaim(SGX_NR_LOW_PAGES) { wake_up(&mut ksgxd_waitq); } return page; } if list_empty(&mut sgx_active_page_list) { return ERR_PTR(-ENOMEM); } if !reclaim { return ERR_PTR(-EBUSY); } if signal_pending(current) { return ERR_PTR(-ERESTARTSYS); } sgx_reclaim_pages(); cond_resched(); } }
pub unsafe extern "C" fn sgx_free_epc_page(page: *mut sgx_epc_page) { let node = (*sgx_epc_sections[(*page).section as usize].node); spin_lock(&mut (*node).lock); (*page).owner = core::ptr::null_mut(); if (*page).poison != 0 { list_add(&mut (*page).list, &mut (*node).sgx_poison_page_list); } else { list_add_tail(&mut (*page).list, &mut (*node).free_page_list); } (*page).flags = SGX_EPC_PAGE_IS_FREE; spin_unlock(&mut (*node).lock); atomic_long_inc(&mut sgx_nr_free_pages); }

unsafe fn __sgx_setup_epc_section(phys_addr: u64, size: u64, index: usize, section: *mut sgx_epc_section) -> bool { let nr_pages = (size >> PAGE_SHIFT) as usize; (*section).virt_addr = memremap(phys_addr, size, MEMREMAP_WB); if (*section).virt_addr.is_null() { return false; } (*section).pages = vmalloc_array(nr_pages, core::mem::size_of::<sgx_epc_page>()); if (*section).pages.is_null() { memunmap((*section).virt_addr); return false; } (*section).phys_addr = phys_addr; xa_store_range(&mut sgx_epc_address_space, phys_addr, phys_addr + size - 1, section, GFP_KERNEL); for i in 0..nr_pages { let p = &mut *(*section).pages.add(i); p.section = index as _; p.flags = 0; p.owner = core::ptr::null_mut(); p.poison = 0; list_add_tail(&mut p.list, &mut sgx_dirty_page_list); } true }
pub unsafe extern "C" fn arch_is_platform_page(paddr: u64) -> bool { !xa_load(&mut sgx_epc_address_space, paddr).is_null() }
unsafe fn sgx_paddr_to_page(paddr: u64) -> *mut sgx_epc_page { let section = xa_load(&mut sgx_epc_address_space, paddr) as *mut sgx_epc_section; if section.is_null() { return core::ptr::null_mut(); } (*section).pages.add(PFN_DOWN(paddr - (*section).phys_addr) as usize) }

pub unsafe extern "C" fn arch_memory_failure(pfn: c_ulong, flags: c_int) -> c_int { let page = sgx_paddr_to_page((pfn as u64) << PAGE_SHIFT); if page.is_null() { return -ENXIO; } if flags & MF_ACTION_REQUIRED != 0 { force_sig(SIGBUS); } let node = (*sgx_epc_sections[(*page).section as usize].node); spin_lock(&mut (*node).lock); if (*page).poison != 0 { spin_unlock(&mut (*node).lock); return 0; } (*page).poison = 1; if (*page).flags & SGX_EPC_PAGE_IS_FREE != 0 { list_move(&mut (*page).list, &mut (*node).sgx_poison_page_list); } else { sgx_unmark_page_reclaimable(page); } spin_unlock(&mut (*node).lock); 0 }

#[inline] unsafe fn __sgx_calc_section_metric(low: u64, high: u64) -> u64 { (low & GENMASK_ULL(31, 12)) + ((high & GENMASK_ULL(19, 0)) << 32) }

static mut sgx_usage_count: c_int = 0; static mut sgx_svn_lock: mutex = mutex::ZERO;
unsafe fn sgx_update_svn() -> c_int { if !cpu_feature_enabled(X86_FEATURE_SGX_EUPDATESVN) { return 0; } WARN(sgx_usage_count != 0, "Elevated usage count when calling EUPDATESVN\n"); let mut ret = 0; for _ in 0..RDRAND_RETRY_LOOPS { ret = __eupdatesvn(); if ret != SGX_INSUFFICIENT_ENTROPY { break; } } match ret { 0 => { pr_info!("SVN updated successfully\n"); 0 }, SGX_NO_UPDATE => 0, SGX_INSUFFICIENT_ENTROPY => -EAGAIN, _ => { ENCLS_WARN(ret, "EUPDATESVN"); -EIO } } }
pub unsafe extern "C" fn sgx_inc_usage_count() -> c_int { mutex_lock(&mut sgx_svn_lock); if sgx_usage_count == 0 { let ret = sgx_update_svn(); if ret != 0 { mutex_unlock(&mut sgx_svn_lock); return ret; } } sgx_usage_count += 1; mutex_unlock(&mut sgx_svn_lock); 0 }
pub unsafe extern "C" fn sgx_dec_usage_count() { mutex_lock(&mut sgx_svn_lock); sgx_usage_count -= 1; mutex_unlock(&mut sgx_svn_lock); }

// CONFIG_NUMA conditional declarations and sysfs attributes are retained as
// external kernel objects; the non-NUMA implementation is an empty function.
#[cfg(CONFIG_NUMA)]
unsafe fn arch_update_sysfs_visibility(nid: c_int) { let node = node_devices[nid as usize]; let ret = sysfs_update_group(&mut (*node).dev.kobj, &arch_node_dev_group); if ret != 0 { pr_err!("sysfs update failed (%d), files may be invisible", ret); } }
#[cfg(not(CONFIG_NUMA))]
unsafe fn arch_update_sysfs_visibility(_nid: c_int) {}

unsafe fn sgx_page_cache_init() -> bool {
    sgx_numa_nodes = kmalloc_objs!(sgx_numa_nodes, num_possible_nodes());
    if sgx_numa_nodes.is_null() { return false; }
    for i in 0..ARRAY_SIZE(sgx_epc_sections) {
        let (mut eax, mut ebx, mut ecx, mut edx) = (0, 0, 0, 0);
        cpuid_count(SGX_CPUID, i as c_int + SGX_CPUID_EPC, &mut eax, &mut ebx, &mut ecx, &mut edx);
        let typ = eax & SGX_CPUID_EPC_MASK;
        if typ == SGX_CPUID_EPC_INVALID { break; }
        if typ != SGX_CPUID_EPC_SECTION { pr_err_once!("Unknown EPC section type: %u\n", typ); break; }
        let pa = __sgx_calc_section_metric(eax as u64, ebx as u64); let size = __sgx_calc_section_metric(ecx as u64, edx as u64);
        pr_info!("EPC section 0x%llx-0x%llx\n", pa, pa + size - 1);
        if !__sgx_setup_epc_section(pa, size, i, &mut sgx_epc_sections[i]) { pr_err!("No free memory for an EPC section\n"); break; }
        let mut nid = numa_map_to_online_node(phys_to_target_node(pa)); if nid == NUMA_NO_NODE { pr_warn!("Unable to map EPC section to online node. Fallback to the NUMA node 0.\n"); nid = 0; }
        if !node_isset(nid, &sgx_numa_mask) { spin_lock_init(&mut (*sgx_numa_nodes.add(nid as usize)).lock); INIT_LIST_HEAD!(&mut (*sgx_numa_nodes.add(nid as usize)).free_page_list); INIT_LIST_HEAD!(&mut (*sgx_numa_nodes.add(nid as usize)).sgx_poison_page_list); node_set(nid, &mut sgx_numa_mask); (*sgx_numa_nodes.add(nid as usize)).size = 0; arch_update_sysfs_visibility(nid); }
        sgx_epc_sections[i].node = sgx_numa_nodes.add(nid as usize); (*sgx_numa_nodes.add(nid as usize)).size += size; sgx_nr_epc_sections += 1;
    }
    if sgx_nr_epc_sections == 0 { pr_err!("There are zero EPC sections.\n"); return false; }
    true
}

pub unsafe extern "C" fn sgx_update_lepubkeyhash(h: *mut u64) { WARN_ON_ONCE(preemptible()); for i in 0..4 { wrmsrq(MSR_IA32_SGXLEPUBKEYHASH0 + i, *h.add(i as usize)); } }
static mut sgx_dev_provision: miscdevice = miscdevice::ZERO;
pub unsafe extern "C" fn sgx_set_attribute(allowed: *mut c_ulong, fd: c_uint) -> c_int { let f = fdget(fd); if f.is_null() || (*f).f_op != &sgx_provision_fops { return -EINVAL; } *allowed |= SGX_ATTR_PROVISIONKEY; 0 }

unsafe fn sgx_init() -> c_int { if !cpu_feature_enabled(X86_FEATURE_SGX) { return -ENODEV; } if !sgx_page_cache_init() { return -ENOMEM; } if !sgx_page_reclaimer_init() { return -ENOMEM; } let ret = misc_register(&mut sgx_dev_provision); if ret != 0 { kthread_stop(ksgxd_tsk); return ret; } let ret = sgx_drv_init(); if sgx_vepc_init() && ret != 0 { misc_deregister(&mut sgx_dev_provision); kthread_stop(ksgxd_tsk); for i in 0..sgx_nr_epc_sections as usize { vfree(sgx_epc_sections[i].pages); memunmap(sgx_epc_sections[i].virt_addr); } } ret }
device_initcall!(sgx_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
