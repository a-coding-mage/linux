// SPDX-License-Identifier: GPL-2.0-only
// Original unit lines 42-1235: fill and protection operations.
struct mfill_state {
    ctx: *mut userfaultfd_ctx,
    src_start: c_ulong,
    dst_start: c_ulong,
    len: c_ulong,
    flags: uffd_flags_t,
    vma: *mut vm_area_struct,
    src_addr: c_ulong,
    dst_addr: c_ulong,
    pmd: *mut pmd_t,
}
struct mfill_retry_state {
    ops: *const vm_uffd_ops,
    file: *mut file,
    flags: vma_flags_t,
    pgoff: pgoff_t,
}
unsafe extern "C" fn anon_can_userfault(vma: *mut vm_area_struct, flags: vm_flags_t) -> bool {
    flags & RUST_UFFD_VM_UFFD_MINOR == 0
}
unsafe extern "C" fn anon_alloc_folio(vma: *mut vm_area_struct, addr: c_ulong) -> *mut folio {
    let f = vma_alloc_folio(RUST_UFFD_GFP_HIGHUSER_MOVABLE as gfp_t, 0, vma, addr);
    if f.is_null() {
        return f;
    }
    if mem_cgroup_charge(f, (*vma).vm_mm, RUST_UFFD_GFP_KERNEL as gfp_t) != 0 {
        folio_put(f);
        return null_mut();
    }
    f
}
static anon_uffd_ops: vm_uffd_ops = vm_uffd_ops {
    can_userfault: Some(anon_can_userfault),
    alloc_folio: Some(anon_alloc_folio),
    get_folio_noalloc: None,
    filemap_add: None,
    filemap_remove: None,
};
unsafe fn vma_uffd_ops(v: *mut vm_area_struct) -> *const vm_uffd_ops {
    if vma_is_anonymous(v) {
        &anon_uffd_ops
    } else {
        (*(*v).vm_ops).uffd_ops
    }
}
unsafe fn validate_dst_vma(v: *mut vm_area_struct, end: c_ulong) -> bool {
    end <= vend(v) && !vctx(v).is_null()
}
unsafe fn find_vma_and_prepare_anon(mm: *mut mm_struct, addr: c_ulong) -> *mut vm_area_struct {
    mmap_assert_locked(mm);
    let v = vma_lookup(mm, addr);
    if v.is_null() {
        return err_ptr(ENOENT);
    }
    if vflags(v) & RUST_UFFD_VM_SHARED == 0 && anon_vma_prepare(v) != 0 {
        return err_ptr(ENOMEM);
    }
    v
}
#[cfg(CONFIG_PER_VMA_LOCK)]
unsafe fn uffd_lock_vma(mm: *mut mm_struct, address: c_ulong) -> *mut vm_area_struct {
    let mut v = lock_vma_under_rcu(mm, address);
    if !v.is_null() {
        if vflags(v) & RUST_UFFD_VM_SHARED == 0 && (*v).anon_vma.is_null() {
            vma_end_read(v);
        } else {
            return v;
        }
    }
    mmap_read_lock(mm);
    v = find_vma_and_prepare_anon(mm, address);
    if !is_err(v) && !vma_start_read_locked(v) {
        v = err_ptr(EAGAIN);
    }
    mmap_read_unlock(mm);
    v
}
unsafe fn uffd_mfill_lock(mm: *mut mm_struct, start: c_ulong, len: c_ulong) -> *mut vm_area_struct {
    #[cfg(CONFIG_PER_VMA_LOCK)]
    {
        let v = uffd_lock_vma(mm, start);
        if is_err(v) || validate_dst_vma(v, start.wrapping_add(len)) {
            return v;
        }
        vma_end_read(v);
        err_ptr(ENOENT)
    }
    #[cfg(not(CONFIG_PER_VMA_LOCK))]
    {
        mmap_read_lock(mm);
        let v = find_vma_and_prepare_anon(mm, start);
        if !is_err(v) && validate_dst_vma(v, start.wrapping_add(len)) {
            return v;
        }
        mmap_read_unlock(mm);
        if is_err(v) {
            v
        } else {
            err_ptr(ENOENT)
        }
    }
}
unsafe fn uffd_mfill_unlock(v: *mut vm_area_struct) {
    #[cfg(CONFIG_PER_VMA_LOCK)]
    vma_end_read(v);
    #[cfg(not(CONFIG_PER_VMA_LOCK))]
    mmap_read_unlock((*v).vm_mm);
}
unsafe fn mfill_put_vma(s: &mut mfill_state) {
    if s.vma.is_null() {
        return;
    }
    up_read(addr_of_mut!((*s.ctx).map_changing_lock));
    uffd_mfill_unlock(s.vma);
    s.vma = null_mut();
}
unsafe fn mfill_get_vma(s: &mut mfill_state) -> c_int {
    let v = uffd_mfill_lock((*s.ctx).mm, s.dst_start, s.len);
    if is_err(v) {
        return ptr_err(v);
    }
    down_read(addr_of_mut!((*s.ctx).map_changing_lock));
    s.vma = v;
    let ret = if atomic_read(addr_of!((*s.ctx).mmap_changing)) != 0 {
        error(EAGAIN)
    } else if warn_anon_shared(vma_is_anonymous(v) && vflags(v) & RUST_UFFD_VM_SHARED != 0)
        || s.flags & RUST_UFFD_MFILL_ATOMIC_WP != 0 && vflags(v) & RUST_UFFD_VM_UFFD_WP == 0
    {
        error(EINVAL)
    } else if is_vm_hugetlb_page(v) {
        0
    } else {
        let ops = vma_uffd_ops(v);
        if ops.is_null()
            || mode_is(s.flags, MFILL_ATOMIC_CONTINUE) && (*ops).get_folio_noalloc.is_none()
        {
            error(EINVAL)
        } else {
            0
        }
    };
    if ret != 0 {
        mfill_put_vma(s);
    }
    ret
}
unsafe fn mm_alloc_pmd(mm: *mut mm_struct, a: c_ulong) -> *mut pmd_t {
    let pgd = pgd_offset(mm, a);
    let p4d = p4d_alloc(mm, pgd, a);
    if p4d.is_null() {
        return null_mut();
    }
    let pud = pud_alloc(mm, p4d, a);
    if pud.is_null() {
        return null_mut();
    }
    pmd_alloc(mm, pud, a)
}
unsafe fn mfill_establish_pmd(s: &mut mfill_state) -> c_int {
    let pmd = mm_alloc_pmd((*s.ctx).mm, s.dst_addr);
    if pmd.is_null() {
        return error(ENOMEM);
    }
    if pmd_none(pmdp_get_lockless(pmd)) && __pte_alloc((*s.ctx).mm, pmd) != 0 {
        return error(ENOMEM);
    }
    let val = pmdp_get_lockless(pmd);
    if !pmd_present(val) || pmd_leaf(val) {
        return error(EEXIST);
    }
    if pmd_bad(val) {
        return error(EFAULT);
    }
    s.pmd = pmd;
    0
}
unsafe fn mfill_file_over_size(v: *mut vm_area_struct, a: c_ulong) -> bool {
    if (*v).vm_file.is_null() {
        return false;
    }
    let sz = i_size_read((*(*v).vm_file).f_inode) as u64;
    linear_page_index(v, a)
        >= sz
            .wrapping_add(PAGE_SIZE as u64 - 1)
            .wrapping_div(PAGE_SIZE as u64) as pgoff_t
}
unsafe fn mfill_atomic_install_pte(
    pmd: *mut pmd_t,
    v: *mut vm_area_struct,
    a: c_ulong,
    p: *mut page,
    f: uffd_flags_t,
) -> c_int {
    let mm = (*v).vm_mm;
    let folio = page_folio(p);
    let cached = !folio_mapping(folio).is_null();
    let mut writable = vflags(v) & RUST_UFFD_VM_WRITE != 0;
    let mut entry = pte_mkdirty(mk_pte(p, (*v).vm_page_prot));
    if cached && vflags(v) & RUST_UFFD_VM_SHARED == 0 {
        writable = false;
    }
    if writable {
        entry = pte_mkwrite(entry, v);
    }
    if f & RUST_UFFD_MFILL_ATOMIC_WP != 0 {
        entry = pte_mkuffd(entry);
    }
    let mut ptl = null_mut();
    let pte = pte_offset_map_lock(mm, pmd, a, &mut ptl);
    if pte.is_null() {
        return error(EAGAIN);
    }
    let old = ptep_get(pte);
    let ret = if mfill_file_over_size(v, a) {
        error(EFAULT)
    } else if !pte_none(old) && !pte_is_uffd_marker(old) {
        error(EEXIST)
    } else {
        if cached {
            folio_add_file_rmap_pte(folio, p, v);
        } else {
            folio_add_new_anon_rmap(folio, v, a, RUST_UFFD_RMAP_EXCLUSIVE as rmap_t);
            folio_add_lru_vma(folio, v);
        }
        inc_mm_counter(mm, mm_counter(folio));
        set_pte_at(mm, a, pte, entry);
        if cached {
            folio_unlock(folio);
        }
        update_mmu_cache(v, a, pte);
        0
    };
    pte_unmap_unlock(pte, ptl);
    ret
}
unsafe fn mfill_copy_folio_locked(f: *mut folio, a: c_ulong) -> c_int {
    let k = kmap_local_folio(f, 0);
    pagefault_disable();
    let left = copy_from_user(k, a as *const c_void, PAGE_SIZE);
    pagefault_enable();
    kunmap_local(k);
    if left != 0 {
        return error(EFAULT);
    }
    flush_dcache_folio(f);
    0
}
unsafe fn mfill_retry_state_save(s: &mut mfill_retry_state, v: *mut vm_area_struct) {
    s.flags = retry_vma_flags(v);
    s.ops = vma_uffd_ops(v);
    s.pgoff = vma_start_pgoff(v);
    if !(*v).vm_file.is_null() {
        s.file = get_file((*v).vm_file);
    }
}
unsafe fn mfill_retry_state_changed(s: &mfill_retry_state, v: *mut vm_area_struct) -> bool {
    let f = retry_vma_flags(v);
    if !vma_flags_same_pair(&s.flags, &f) || s.ops != vma_uffd_ops(v) {
        return true;
    }
    if s.file.is_null() {
        return !vma_is_anonymous(v);
    }
    (*v).vm_file.is_null()
        || (*(*v).vm_file).f_inode != (*s.file).f_inode
        || s.file != (*v).vm_file
        || vma_start_pgoff(v) != s.pgoff
}
unsafe fn mfill_retry_state_put(s: &mut mfill_retry_state) {
    if !s.file.is_null() {
        fput(s.file);
        s.file = null_mut();
    }
}
unsafe fn mfill_copy_folio_retry(s: &mut mfill_state, f: *mut folio) -> c_int {
    let mut saved: mfill_retry_state = zeroed();
    mfill_retry_state_save(&mut saved, s.vma);
    mfill_put_vma(s);
    let k = kmap_local_folio(f, 0);
    let left = copy_from_user(k, s.src_addr as *const c_void, PAGE_SIZE);
    kunmap_local(k);
    let ret = if left != 0 {
        error(EFAULT)
    } else {
        flush_dcache_folio(f);
        let ret = mfill_get_vma(s);
        if ret != 0 {
            ret
        } else if mfill_retry_state_changed(&saved, s.vma) {
            error(EAGAIN)
        } else {
            mfill_establish_pmd(s)
        }
    };
    mfill_retry_state_put(&mut saved);
    ret
}
unsafe fn __mfill_atomic_pte(s: &mut mfill_state, ops: *const vm_uffd_ops) -> c_int {
    if ops.is_null() {
        warn_unsupported_copy();
        return error(EOPNOTSUPP);
    }
    let f = (*ops).alloc_folio.unwrap()(s.vma, s.dst_addr);
    if f.is_null() {
        return error(ENOMEM);
    }
    let mut ret = 0;
    if mode_is(s.flags, MFILL_ATOMIC_COPY) {
        ret = mfill_copy_folio_locked(f, s.src_addr);
        if ret != 0 {
            ret = mfill_copy_folio_retry(s, f);
        }
    } else if mode_is(s.flags, MFILL_ATOMIC_ZEROPAGE) {
        clear_user_highpage(folio_page(f), s.dst_addr);
    } else {
        warn_unknown_folio_fill(s.flags);
    }
    if ret != 0 {
        folio_put(f);
        return ret;
    }
    __folio_mark_uptodate(f);
    if let Some(add) = (*ops).filemap_add {
        ret = add(f, s.vma, s.dst_addr);
        if ret != 0 {
            folio_put(f);
            return ret;
        }
    }
    ret = mfill_atomic_install_pte(s.pmd, s.vma, s.dst_addr, folio_page(f), s.flags);
    if ret != 0 {
        if let Some(remove) = (*ops).filemap_remove {
            remove(f, s.vma);
        }
        folio_put(f);
    }
    ret
}
unsafe fn mfill_atomic_pte_copy(s: &mut mfill_state) -> c_int {
    let ops = if vflags(s.vma) & RUST_UFFD_VM_SHARED == 0 {
        &anon_uffd_ops
    } else {
        vma_uffd_ops(s.vma)
    };
    __mfill_atomic_pte(s, ops)
}
unsafe fn mfill_atomic_pte_zeroed_folio(s: &mut mfill_state) -> c_int {
    __mfill_atomic_pte(s, vma_uffd_ops(s.vma))
}
unsafe fn mfill_atomic_pte_zeropage(s: &mut mfill_state) -> c_int {
    let v = s.vma;
    if mm_forbids_zeropage((*v).vm_mm) || vflags(v) & RUST_UFFD_VM_SHARED != 0 {
        return mfill_atomic_pte_zeroed_folio(s);
    }
    let entry = pte_mkspecial(pfn_pte(zero_pfn(s.dst_addr), (*v).vm_page_prot));
    mfill_empty_pte(s, entry)
}
// Shared Rust implementation of the identical locked empty-PTE installation
// sequences in mfill_atomic_pte_zeropage and mfill_atomic_pte_poison.
unsafe fn mfill_empty_pte(s: &mut mfill_state, entry: pte_t) -> c_int {
    let v = s.vma;
    let mm = (*v).vm_mm;
    let mut ptl = null_mut();
    let pte = pte_offset_map_lock(mm, s.pmd, s.dst_addr, &mut ptl);
    if pte.is_null() {
        return error(EAGAIN);
    }
    let ret = if mfill_file_over_size(v, s.dst_addr) {
        error(EFAULT)
    } else if !pte_none(ptep_get(pte)) {
        error(EEXIST)
    } else {
        set_pte_at(mm, s.dst_addr, pte, entry);
        update_mmu_cache(v, s.dst_addr, pte);
        0
    };
    pte_unmap_unlock(pte, ptl);
    ret
}
unsafe fn mfill_atomic_pte_continue(s: &mut mfill_state) -> c_int {
    let v = s.vma;
    let ops = vma_uffd_ops(v);
    if ops.is_null() {
        warn_unsupported_continue();
        return error(EOPNOTSUPP);
    }
    let off = linear_page_index(v, s.dst_addr);
    let f = (*ops).get_folio_noalloc.unwrap()(file_inode((*v).vm_file), off);
    if f.is_null() || is_err(f) {
        return error(EFAULT);
    }
    let p = folio_file_page(f, off);
    let ret = if PageHWPoison(p) {
        error(EIO)
    } else {
        mfill_atomic_install_pte(s.pmd, v, s.dst_addr, p, s.flags)
    };
    if ret != 0 {
        folio_unlock(f);
        folio_put(f);
    }
    ret
}
unsafe fn mfill_atomic_pte_poison(s: &mut mfill_state) -> c_int {
    mfill_empty_pte(s, make_pte_marker(RUST_UFFD_PTE_MARKER_POISONED))
}
#[cfg(CONFIG_HUGETLB_PAGE)]
unsafe fn mfill_atomic_hugetlb(
    ctx: *mut userfaultfd_ctx,
    mut v: *mut vm_area_struct,
    dst_start: c_ulong,
    src_start: c_ulong,
    len: c_ulong,
    flags: uffd_flags_t,
) -> c_long {
    let mm = (*v).vm_mm;
    let mut f = null_mut();
    let mut src = src_start;
    let mut dst = dst_start;
    let mut copied = 0;
    let size = vma_kernel_pagesize(v);
    let mut map_locked = true;
    let mut v_locked = true;
    let mut ret = error(EINVAL) as c_long;
    if !mode_is(flags, MFILL_ATOMIC_ZEROPAGE)
        && dst_start & (size - 1) == 0
        && len & (size - 1) == 0
    {
        'retry: loop {
            if v.is_null() {
                v = uffd_mfill_lock(mm, dst_start, len);
                if is_err(v) {
                    ret = ptr_err(v) as c_long;
                    break;
                }
                v_locked = true;
                if !is_vm_hugetlb_page(v) {
                    ret = error(ENOENT) as c_long;
                    break;
                }
                if size != vma_kernel_pagesize(v) {
                    ret = error(EINVAL) as c_long;
                    break;
                }
                down_read(addr_of_mut!((*ctx).map_changing_lock));
                map_locked = true;
                if atomic_read(addr_of!((*ctx).mmap_changing)) != 0 {
                    ret = error(EAGAIN) as c_long;
                    break;
                }
            }
            while src < src_start.wrapping_add(len) {
                vm_warn!(warn_001, dst >= dst_start.wrapping_add(len));
                let idx = hugetlb_linear_page_index(v, dst);
                let mapping = (*(*v).vm_file).f_mapping;
                let hash = hugetlb_fault_mutex_hash(mapping, idx);
                let mutex = hugetlb_fault_mutex(hash);
                mutex_lock(mutex);
                hugetlb_vma_lock_read(v);
                let pte = huge_pte_alloc(mm, v, dst, size);
                ret = error(ENOMEM) as c_long;
                if !pte.is_null() {
                    let old = huge_ptep_get(mm, dst, pte);
                    if !mode_is(flags, MFILL_ATOMIC_CONTINUE)
                        && !huge_pte_none(old)
                        && !pte_is_uffd_marker(old)
                    {
                        ret = error(EEXIST) as c_long;
                    } else {
                        ret = hugetlb_mfill_atomic_pte(pte, v, dst, src, flags, &mut f) as c_long;
                    }
                }
                hugetlb_vma_unlock_read(v);
                mutex_unlock(mutex);
                if pte.is_null() {
                    break;
                }
                cond_resched();
                if ret == error(ENOENT) as c_long {
                    up_read(addr_of_mut!((*ctx).map_changing_lock));
                    map_locked = false;
                    uffd_mfill_unlock(v);
                    v_locked = false;
                    vm_warn!(warn_002, f.is_null());
                    ret = copy_folio_from_user(f, src as *const c_void, true) as c_long;
                    if ret != 0 {
                        ret = error(EFAULT) as c_long;
                        break 'retry;
                    }
                    v = null_mut();
                    continue 'retry;
                }
                vm_warn!(warn_003, !f.is_null());
                if ret == 0 {
                    dst = dst.wrapping_add(size);
                    src = src.wrapping_add(size);
                    copied += size as c_long;
                    if fatal_signal_pending(current()) {
                        ret = error(EINTR) as c_long;
                    }
                }
                if ret != 0 {
                    break;
                }
            }
            break;
        }
    }
    if map_locked {
        up_read(addr_of_mut!((*ctx).map_changing_lock));
    }
    if v_locked {
        uffd_mfill_unlock(v);
    }
    if !f.is_null() {
        folio_put(f);
    }
    vm_warn!(warn_004, copied < 0);
    vm_warn!(warn_005, ret > 0);
    vm_warn!(warn_006, copied == 0 && ret == 0);
    if copied != 0 {
        copied
    } else {
        ret
    }
}
unsafe fn mfill_atomic_pte(s: &mut mfill_state) -> c_long {
    (if mode_is(s.flags, MFILL_ATOMIC_CONTINUE) {
        mfill_atomic_pte_continue(s)
    } else if mode_is(s.flags, MFILL_ATOMIC_POISON) {
        mfill_atomic_pte_poison(s)
    } else if mode_is(s.flags, MFILL_ATOMIC_COPY) {
        mfill_atomic_pte_copy(s)
    } else if mode_is(s.flags, MFILL_ATOMIC_ZEROPAGE) {
        mfill_atomic_pte_zeropage(s)
    } else {
        warn_unknown_pte_fill(s.flags);
        error(EOPNOTSUPP)
    }) as c_long
}
unsafe fn mfill_atomic(
    ctx: *mut userfaultfd_ctx,
    dst: c_ulong,
    src: c_ulong,
    len: c_ulong,
    flags: uffd_flags_t,
) -> c_long {
    let mut s = mfill_state {
        ctx,
        dst_start: dst,
        src_start: src,
        len,
        flags,
        vma: null_mut(),
        src_addr: src,
        dst_addr: dst,
        pmd: null_mut(),
    };
    vm_warn!(warn_007, dst & !PAGE_MASK != 0);
    vm_warn!(warn_008, len & !PAGE_MASK != 0);
    vm_warn!(warn_009, src.wrapping_add(len) <= src);
    vm_warn!(warn_010, dst.wrapping_add(len) <= dst);
    let mut ret = mfill_get_vma(&mut s) as c_long;
    if ret != 0 {
        return ret;
    }
    #[cfg(CONFIG_HUGETLB_PAGE)]
    if is_vm_hugetlb_page(s.vma) {
        return mfill_atomic_hugetlb(ctx, s.vma, dst, src, len, flags);
    }
    let mut copied = 0;
    while s.src_addr < src.wrapping_add(len) {
        vm_warn!(warn_011, s.dst_addr >= dst.wrapping_add(len));
        ret = mfill_establish_pmd(&mut s) as c_long;
        if ret != 0 {
            break;
        }
        ret = mfill_atomic_pte(&mut s);
        cond_resched();
        if ret == 0 {
            s.dst_addr = s.dst_addr.wrapping_add(PAGE_SIZE);
            s.src_addr = s.src_addr.wrapping_add(PAGE_SIZE);
            copied += PAGE_SIZE as c_long;
            if fatal_signal_pending(current()) {
                ret = error(EINTR) as c_long;
            }
        }
        if ret != 0 {
            break;
        }
    }
    mfill_put_vma(&mut s);
    vm_warn!(warn_012, copied < 0);
    vm_warn!(warn_013, ret > 0);
    vm_warn!(warn_014, copied == 0 && ret == 0);
    if copied != 0 {
        copied
    } else {
        ret
    }
}
unsafe fn mfill_atomic_copy(
    c: *mut userfaultfd_ctx,
    d: c_ulong,
    s: c_ulong,
    l: c_ulong,
    f: uffd_flags_t,
) -> c_long {
    mfill_atomic(c, d, s, l, set_mode(f, MFILL_ATOMIC_COPY))
}
unsafe fn mfill_atomic_zeropage(c: *mut userfaultfd_ctx, s: c_ulong, l: c_ulong) -> c_long {
    mfill_atomic(c, s, 0, l, set_mode(0, MFILL_ATOMIC_ZEROPAGE))
}
unsafe fn mfill_atomic_continue(
    c: *mut userfaultfd_ctx,
    s: c_ulong,
    l: c_ulong,
    f: uffd_flags_t,
) -> c_long {
    smp_wmb();
    mfill_atomic(c, s, 0, l, set_mode(f, MFILL_ATOMIC_CONTINUE))
}
unsafe fn mfill_atomic_poison(
    c: *mut userfaultfd_ctx,
    s: c_ulong,
    l: c_ulong,
    f: uffd_flags_t,
) -> c_long {
    mfill_atomic(c, s, 0, l, set_mode(f, MFILL_ATOMIC_POISON))
}
#[no_mangle]
pub unsafe extern "C" fn uffd_wp_range(
    v: *mut vm_area_struct,
    start: c_ulong,
    len: c_ulong,
    enable: bool,
) -> c_long {
    warn_range(start < vstart(v) || start.wrapping_add(len) > vend(v));
    let mut flags = if enable {
        RUST_UFFD_MM_CP_UFFD_WP
    } else {
        RUST_UFFD_MM_CP_UFFD_WP_RESOLVE
    };
    if !enable && vma_wants_manual_pte_write_upgrade(v) {
        flags |= RUST_UFFD_MM_CP_TRY_CHANGE_WRITABLE;
    }
    let mut tlb = zeroed();
    tlb_gather_mmu(&mut tlb, (*v).vm_mm);
    let ret = change_protection(&mut tlb, v, start, start.wrapping_add(len), flags);
    tlb_finish_mmu(&mut tlb);
    ret
}
unsafe fn mwriteprotect_range(
    c: *mut userfaultfd_ctx,
    start: c_ulong,
    len: c_ulong,
    enable: bool,
) -> c_int {
    let mm = (*c).mm;
    let end = start.wrapping_add(len);
    let mut i = iterator(mm, start);
    vm_warn!(warn_015, start & !PAGE_MASK != 0);
    vm_warn!(warn_016, len & !PAGE_MASK != 0);
    vm_warn!(warn_017, end <= start);
    mmap_read_lock(mm);
    down_read(addr_of_mut!((*c).map_changing_lock));
    let mut ret = error(EAGAIN) as c_long;
    if atomic_read(addr_of!((*c).mmap_changing)) == 0 {
        ret = error(ENOENT) as c_long;
        loop {
            let v = vma_find(&mut i, end);
            if v.is_null() {
                break;
            }
            if !userfaultfd_wp(v) {
                ret = error(ENOENT) as c_long;
                break;
            }
            if is_vm_hugetlb_page(v) {
                ret = error(EINVAL) as c_long;
                let mask = vma_kernel_pagesize(v) - 1;
                if (start | len) & mask != 0 {
                    break;
                }
            }
            let a = core::cmp::max(vstart(v), start);
            let b = core::cmp::min(vend(v), end);
            ret = uffd_wp_range(v, a, b - a, enable);
            if ret < 0 {
                break;
            }
            ret = 0;
        }
    }
    up_read(addr_of_mut!((*c).map_changing_lock));
    mmap_read_unlock(mm);
    ret as c_int
}
#[no_mangle]
pub unsafe extern "C" fn mrwprotect_range(
    c: *mut userfaultfd_ctx,
    start: c_ulong,
    len: c_ulong,
    enable: bool,
) -> c_int {
    let mm = (*c).mm;
    let end = start.wrapping_add(len);
    let mut i = iterator(mm, start);
    vm_warn!(warn_018, start & !PAGE_MASK != 0);
    vm_warn!(warn_019, len & !PAGE_MASK != 0);
    vm_warn!(warn_020, end <= start);
    mmap_read_lock(mm);
    down_read(addr_of_mut!((*c).map_changing_lock));
    let ret = (|| {
        if atomic_read(addr_of!((*c).mmap_changing)) != 0 {
            return error(EAGAIN);
        }
        let flags = if enable {
            RUST_UFFD_MM_CP_UFFD_RWP
        } else {
            RUST_UFFD_MM_CP_UFFD_RWP_RESOLVE
        };
        let mut found = false;
        loop {
            let v = vma_find(&mut i, end);
            if v.is_null() {
                break;
            }
            if !userfaultfd_rwp(v) {
                return error(ENOENT);
            }
            if is_vm_hugetlb_page(v) && (start | len) & (vma_kernel_pagesize(v) - 1) != 0 {
                return error(EINVAL);
            }
            found = true;
        }
        if !found {
            return error(ENOENT);
        }
        vma_iter_set(&mut i, start);
        let mut tlb = zeroed();
        tlb_gather_mmu(&mut tlb, mm);
        loop {
            let v = vma_find(&mut i, end);
            if v.is_null() {
                break;
            }
            let mut f = flags;
            if !enable && vma_wants_manual_pte_write_upgrade(v) {
                f |= RUST_UFFD_MM_CP_TRY_CHANGE_WRITABLE;
            }
            change_protection(
                &mut tlb,
                v,
                core::cmp::max(vstart(v), start),
                core::cmp::min(vend(v), end),
                f,
            );
        }
        tlb_finish_mmu(&mut tlb);
        0
    })();
    up_read(addr_of_mut!((*c).map_changing_lock));
    mmap_read_unlock(mm);
    ret
}
