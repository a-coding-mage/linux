// SPDX-License-Identifier: GPL-2.0
unsafe fn anon_vma_alloc() -> *mut anon_vma {
    let av = rust_rmap_kmem_cache_alloc(ANON_VMA_CACHEP, RUST_RMAP_GFP_KERNEL as gfp_t)
        .cast::<anon_vma>();
    if !av.is_null() {
        rust_rmap_atomic_set(addr_of_mut!((*av).refcount), 1);
        (*av).num_children = 0;
        (*av).num_active_vmas = 0;
        (*av).parent = av;
        (*av).root = av;
    }
    av
}

unsafe fn anon_vma_free(av: *mut anon_vma) {
    rust_rmap_diag_118(av);
    // The dec-and-test rust_rmap_barrier pairs with folio_lock_anon_vma_read's acquire.
    rust_rmap_might_sleep();
    if rust_rmap_rwsem_is_locked(addr_of_mut!((*(*av).root).rwsem)) {
        rust_rmap_anon_vma_lock_write(av);
        rust_rmap_anon_vma_unlock_write(av);
    }
    rust_rmap_kmem_cache_free(ANON_VMA_CACHEP, av.cast());
}

unsafe fn anon_vma_chain_alloc(gfp: gfp_t) -> *mut anon_vma_chain {
    rust_rmap_kmem_cache_alloc(ANON_VMA_CHAIN_CACHEP, gfp).cast()
}
unsafe fn anon_vma_chain_free(avc: *mut anon_vma_chain) {
    rust_rmap_kmem_cache_free(ANON_VMA_CHAIN_CACHEP, avc.cast());
}
unsafe fn anon_vma_chain_assign(
    vma: *mut vm_area_struct,
    avc: *mut anon_vma_chain,
    av: *mut anon_vma,
) {
    (*avc).vma = vma;
    (*avc).anon_vma = av;
    rust_rmap_list_add(
        addr_of_mut!((*avc).same_vma),
        addr_of_mut!((*vma).anon_vma_chain),
    );
}

#[no_mangle]
pub unsafe extern "C" fn __anon_vma_prepare(vma: *mut vm_area_struct) -> c_int {
    let mm = (*vma).vm_mm;
    rust_rmap_mmap_assert_locked(mm);
    rust_rmap_might_sleep();
    let mut avc = anon_vma_chain_alloc(RUST_RMAP_GFP_KERNEL as gfp_t);
    if avc.is_null() {
        return -(ENOMEM as c_int);
    }
    let mut av = rust_rmap_find_mergeable_anon_vma(vma);
    let mut allocated = null_mut();
    if av.is_null() {
        av = anon_vma_alloc();
        if av.is_null() {
            anon_vma_chain_free(avc);
            return -(ENOMEM as c_int);
        }
        (*av).num_children = (*av).num_children.wrapping_add(1);
        allocated = av;
    }
    rust_rmap_anon_vma_lock_write(av);
    rust_rmap_spin_lock(rust_rmap_mm_page_table_lock(mm));
    if (*vma).anon_vma.is_null() {
        (*vma).anon_vma = av;
        anon_vma_chain_assign(vma, avc, av);
        rust_rmap_anon_rmap_tree_insert(avc, av);
        (*av).num_active_vmas = (*av).num_active_vmas.wrapping_add(1);
        allocated = null_mut();
        avc = null_mut();
    }
    rust_rmap_spin_unlock(rust_rmap_mm_page_table_lock(mm));
    rust_rmap_anon_vma_unlock_write(av);
    if !allocated.is_null() {
        rust_rmap_put_anon_vma(allocated);
    }
    if !avc.is_null() {
        anon_vma_chain_free(avc);
    }
    0
}

unsafe fn check_anon_vma_clone(
    dst: *mut vm_area_struct,
    src: *mut vm_area_struct,
    operation: vma_operation,
) {
    rust_rmap_mmap_assert_write_locked((*src).vm_mm);
    rust_rmap_diag_clone_1(dst, src, operation);
    rust_rmap_diag_clone_2(dst, src, operation);
    rust_rmap_diag_clone_3(dst, src, operation);
    rust_rmap_diag_clone_4(dst, src, operation);
    rust_rmap_diag_clone_5(dst, src, operation);
    rust_rmap_diag_clone_6(dst, src, operation);
    rust_rmap_diag_clone_7(dst, src, operation);
    #[cfg(CONFIG_PER_VMA_LOCK)]
    rust_rmap_diag_clone_attached(dst, operation);
}
unsafe fn maybe_reuse_anon_vma(dst: *mut vm_area_struct, av: *mut anon_vma) {
    if !(*dst).anon_vma.is_null() || (*av).num_active_vmas > 0 || (*av).num_children > 1 {
        return;
    }
    (*dst).anon_vma = av;
    (*av).num_active_vmas = (*av).num_active_vmas.wrapping_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn anon_vma_clone(
    dst: *mut vm_area_struct,
    src: *mut vm_area_struct,
    operation: vma_operation,
) -> c_int {
    let active = (*src).anon_vma;
    check_anon_vma_clone(dst, src, operation);
    if active.is_null() {
        return 0;
    }
    let head = addr_of_mut!((*src).anon_vma_chain);
    let mut node = (*head).next;
    while node != head {
        let pavc = rust_rmap_avc_from_same_vma(node);
        let avc = anon_vma_chain_alloc(RUST_RMAP_GFP_KERNEL as gfp_t);
        if avc.is_null() {
            cleanup_partial_anon_vmas(dst);
            return -(ENOMEM as c_int);
        }
        anon_vma_chain_assign(dst, avc, (*pavc).anon_vma);
        node = (*node).next;
    }
    rust_rmap_anon_vma_lock_write(active);
    let head = addr_of_mut!((*dst).anon_vma_chain);
    let mut node = (*head).prev;
    while node != head {
        let avc = rust_rmap_avc_from_same_vma(node);
        let av = (*avc).anon_vma;
        rust_rmap_anon_rmap_tree_insert(avc, av);
        if operation == VMA_OP_FORK {
            maybe_reuse_anon_vma(dst, av);
        }
        node = (*node).prev;
    }
    if operation != VMA_OP_FORK {
        (*(*dst).anon_vma).num_active_vmas = (*(*dst).anon_vma).num_active_vmas.wrapping_add(1);
    }
    rust_rmap_anon_vma_unlock_write(active);
    0
}

#[no_mangle]
pub unsafe extern "C" fn anon_vma_fork(
    vma: *mut vm_area_struct,
    pvma: *mut vm_area_struct,
) -> c_int {
    if (*pvma).anon_vma.is_null() {
        return 0;
    }
    (*vma).anon_vma = null_mut();
    let av = anon_vma_alloc();
    if av.is_null() {
        return -(ENOMEM as c_int);
    }
    let avc = anon_vma_chain_alloc(RUST_RMAP_GFP_KERNEL as gfp_t);
    if avc.is_null() {
        rust_rmap_put_anon_vma(av);
        return -(ENOMEM as c_int);
    }
    let rc = anon_vma_clone(vma, pvma, VMA_OP_FORK);
    if rc != 0 || !(*vma).anon_vma.is_null() {
        rust_rmap_put_anon_vma(av);
        anon_vma_chain_free(avc);
        return rc;
    }
    (*av).num_active_vmas = 1;
    (*av).root = (*(*pvma).anon_vma).root;
    (*av).parent = (*pvma).anon_vma;
    rust_rmap_get_anon_vma((*av).root);
    (*vma).anon_vma = av;
    anon_vma_chain_assign(vma, avc, av);
    rust_rmap_anon_vma_lock_write(av);
    rust_rmap_anon_rmap_tree_insert(avc, av);
    (*(*av).parent).num_children = (*(*av).parent).num_children.wrapping_add(1);
    rust_rmap_anon_vma_unlock_write(av);
    0
}
unsafe fn cleanup_partial_anon_vmas(vma: *mut vm_area_struct) {
    let head = addr_of_mut!((*vma).anon_vma_chain);
    let mut node = (*head).next;
    while node != head {
        let next = (*node).next;
        let avc = rust_rmap_avc_from_same_vma(node);
        rust_rmap_list_del(node);
        anon_vma_chain_free(avc);
        node = next;
    }
    (*vma).anon_vma = null_mut();
}
#[no_mangle]
pub unsafe extern "C" fn unlink_anon_vmas(vma: *mut vm_area_struct) {
    let active = (*vma).anon_vma;
    rust_rmap_mmap_assert_locked((*vma).vm_mm);
    if active.is_null() {
        rust_rmap_diag_unlink_empty(vma);
        return;
    }
    rust_rmap_anon_vma_lock_write(active);
    let head = addr_of_mut!((*vma).anon_vma_chain);
    let mut node = (*head).next;
    while node != head {
        let next = (*node).next;
        let avc = rust_rmap_avc_from_same_vma(node);
        let av = (*avc).anon_vma;
        rust_rmap_anon_rmap_tree_remove(avc, av);
        if rust_rmap_rb_empty_root(addr_of!((*av).rb_root.rb_root)) {
            (*(*av).parent).num_children = (*(*av).parent).num_children.wrapping_sub(1);
        } else {
            rust_rmap_list_del(node);
            anon_vma_chain_free(avc);
        }
        node = next;
    }
    (*active).num_active_vmas = (*active).num_active_vmas.wrapping_sub(1);
    (*vma).anon_vma = null_mut();
    rust_rmap_anon_vma_unlock_write(active);
    let mut node = (*head).next;
    while node != head {
        let next = (*node).next;
        let avc = rust_rmap_avc_from_same_vma(node);
        let av = (*avc).anon_vma;
        rust_rmap_diag_unlink_counts_1(av);
        rust_rmap_diag_unlink_counts_2(av);
        rust_rmap_put_anon_vma(av);
        rust_rmap_list_del(node);
        anon_vma_chain_free(avc);
        node = next;
    }
}
unsafe extern "C" fn anon_vma_ctor(data: *mut c_void) {
    let av = data.cast::<anon_vma>();
    rust_rmap_init_rwsem(addr_of_mut!((*av).rwsem));
    rust_rmap_atomic_set(addr_of_mut!((*av).refcount), 0);
    // RB_ROOT_CACHED's members are pointers and are all NULL in the native ABI.
    (*av).rb_root = rust_rmap_rb_root_cached_empty();
}
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RUST_RMAP_INIT_COLD, cold)]
pub unsafe extern "C" fn anon_vma_init() {
    ANON_VMA_CACHEP = rust_rmap_kmem_cache_create(
        c"anon_vma".as_ptr(),
        size_of::<anon_vma>() as c_uint,
        0,
        (RUST_RMAP_SLAB_TYPESAFE_BY_RCU | RUST_RMAP_SLAB_PANIC | RUST_RMAP_SLAB_ACCOUNT)
            as slab_flags_t,
        Some(anon_vma_ctor),
    );
    ANON_VMA_CHAIN_CACHEP = rust_rmap_anon_chain_cache_create();
}
#[no_mangle]
pub unsafe extern "C" fn folio_get_anon_vma(folio: *const folio) -> *mut anon_vma {
    rust_rmap_diag_get_anon_locked(folio);
    rust_rmap_rcu_read_lock();
    let mapping = rust_rmap_folio_mapping_read_once(folio) as c_ulong;
    if mapping & RUST_RMAP_FOLIO_MAPPING_FLAGS as c_ulong != RUST_RMAP_FOLIO_MAPPING_ANON as c_ulong
        || !rust_rmap_folio_mapped(folio)
    {
        rust_rmap_rcu_read_unlock();
        return null_mut();
    }
    let av = mapping.wrapping_sub(RUST_RMAP_FOLIO_MAPPING_ANON as c_ulong) as *mut anon_vma;
    if !rust_rmap_atomic_inc_not_zero(addr_of_mut!((*av).refcount)) {
        rust_rmap_rcu_read_unlock();
        return null_mut();
    }
    if !rust_rmap_folio_mapped(folio) {
        rust_rmap_rcu_read_unlock();
        rust_rmap_put_anon_vma(av);
        return null_mut();
    }
    rust_rmap_rcu_read_unlock();
    av
}
#[no_mangle]
pub unsafe extern "C" fn folio_lock_anon_vma_read(
    folio: *const folio,
    rwc: *mut rmap_walk_control,
) -> *mut anon_vma {
    rust_rmap_diag_lock_anon_locked(folio);
    rust_rmap_rcu_read_lock();
    let mapping = rust_rmap_folio_mapping_read_once(folio) as c_ulong;
    if mapping & RUST_RMAP_FOLIO_MAPPING_FLAGS as c_ulong != RUST_RMAP_FOLIO_MAPPING_ANON as c_ulong
        || !rust_rmap_folio_mapped(folio)
    {
        rust_rmap_rcu_read_unlock();
        return null_mut();
    }
    let av = mapping.wrapping_sub(RUST_RMAP_FOLIO_MAPPING_ANON as c_ulong) as *mut anon_vma;
    let root = rust_rmap_anon_root_read_once(av);
    if rust_rmap_down_read_trylock(addr_of_mut!((*root).rwsem)) != 0 {
        if !rust_rmap_folio_mapped(folio) {
            rust_rmap_up_read(addr_of_mut!((*root).rwsem));
            rust_rmap_rcu_read_unlock();
            return null_mut();
        }
        rust_rmap_rcu_read_unlock();
        return av;
    }
    if !rwc.is_null() && (*rwc).try_lock {
        (*rwc).contended = true;
        rust_rmap_rcu_read_unlock();
        return null_mut();
    }
    if !rust_rmap_atomic_inc_not_zero(addr_of_mut!((*av).refcount)) {
        rust_rmap_rcu_read_unlock();
        return null_mut();
    }
    if !rust_rmap_folio_mapped(folio) {
        rust_rmap_rcu_read_unlock();
        rust_rmap_put_anon_vma(av);
        return null_mut();
    }
    rust_rmap_rcu_read_unlock();
    rust_rmap_anon_vma_lock_read(av);
    if rust_rmap_atomic_dec_and_test(addr_of_mut!((*av).refcount)) {
        rust_rmap_anon_vma_unlock_read(av);
        __put_anon_vma(av);
        return null_mut();
    }
    av
}
#[no_mangle]
pub unsafe extern "C" fn __put_anon_vma(av: *mut anon_vma) {
    let root = (*av).root;
    anon_vma_free(av);
    if root != av && rust_rmap_atomic_dec_and_test(addr_of_mut!((*root).refcount)) {
        anon_vma_free(root);
    }
}
