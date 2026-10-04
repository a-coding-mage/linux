// SPDX-License-Identifier: GPL-2.0-only
// vmalloc.c:3125-4508. Reservation, lifetimes, allocation and realloc decisions.
#[link_section = ".init.data"]
static mut vmlist: *mut vm_struct = null_mut();
unsafe fn vm_area_page_order(vm: *mut vm_struct) -> u32 {
    #[cfg(CONFIG_HAVE_ARCH_HUGE_VMALLOC)]
    {
        (*vm).page_order
    }
    #[cfg(not(CONFIG_HAVE_ARCH_HUGE_VMALLOC))]
    {
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_vm_area_page_order(vm: *mut vm_struct) -> u32 {
    vm_area_page_order(vm)
}
unsafe fn set_vm_area_page_order(vm: *mut vm_struct, order: u32) {
    #[cfg(CONFIG_HAVE_ARCH_HUGE_VMALLOC)]
    {
        (*vm).page_order = order;
    }
    #[cfg(not(CONFIG_HAVE_ARCH_HUGE_VMALLOC))]
    {
        bug(order != 0);
    }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn vm_area_add_early(vm: *mut vm_struct) {
    bug(vmap_initialized);
    let mut p = &raw mut vmlist;
    while !(*p).is_null() {
        let tmp = *p;
        if (*tmp).addr >= (*vm).addr {
            bug(((*tmp).addr as ULong) < ((*vm).addr as ULong).wrapping_add((*vm).size));
            break;
        } else {
            bug(((*tmp).addr as ULong).wrapping_add((*tmp).size) > (*vm).addr as ULong);
        }
        p = &raw mut (*tmp).__bindgen_anon_1.next;
    }
    (*vm).__bindgen_anon_1.next = *p;
    *p = vm;
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn vm_area_register_early(vm: *mut vm_struct, align: usize) {
    let mut addr = align_up(vmalloc_start(), align as ULong);
    bug(vmap_initialized);
    let mut p = &raw mut vmlist;
    while !(*p).is_null() {
        let cur = *p;
        if ((*cur).addr as ULong).wrapping_sub(addr) >= (*vm).size {
            break;
        }
        addr = align_up(
            ((*cur).addr as ULong).wrapping_add((*cur).size),
            align as ULong,
        );
        p = &raw mut (*cur).__bindgen_anon_1.next;
    }
    bug(addr > vmalloc_end().wrapping_sub((*vm).size));
    (*vm).addr = addr as *mut Void;
    (*vm).__bindgen_anon_1.next = *p;
    *p = vm;
    kasan_populate_early_vm_area_shadow((*vm).addr, (*vm).size);
}
#[no_mangle]
pub unsafe extern "C" fn clear_vm_uninitialized_flag(vm: *mut vm_struct) {
    smp_wmb();
    (*vm).flags &= !(VM_UNINITIALIZED as ULong);
}
#[no_mangle]
pub unsafe extern "C" fn __get_vm_area_node(
    mut size: ULong,
    mut align: ULong,
    shift: ULong,
    flags: ULong,
    start: ULong,
    end: ULong,
    node: i32,
    gfp_mask: gfp_t,
    caller: *const Void,
) -> *mut vm_struct {
    let requested_size = size;
    bug(in_nmi() || in_hardirq());
    size = align_up(size, 1 << shift);
    if size == 0 {
        return null_mut();
    }
    if flags & VM_IOREMAP as ULong != 0 {
        align =
            1 << get_count_order_long(size).clamp(PAGE_SHIFT as i32, RVM_IOREMAP_MAX_ORDER as i32);
    }
    let area =
        kzalloc_node(size_of::<vm_struct>(), gfp_mask & GFP_RECLAIM_MASK, node).cast::<vm_struct>();
    if area.is_null() {
        return null_mut();
    }
    if flags & VM_NO_GUARD as ULong == 0 {
        size = size.wrapping_add(PAGE_SIZE);
    }
    (*area).flags = flags;
    (*area).caller = caller;
    (*area).requested_size = requested_size;
    let va = alloc_vmap_area(size, align, start, end, node, gfp_mask, 0, area);
    if err_value(va as ULong) {
        kfree(area.cast());
        return null_mut();
    }
    if flags & VM_ALLOC as ULong == 0 {
        (*area).addr =
            kasan_unpoison_vmalloc((*area).addr, requested_size, KASAN_VMALLOC_PROT_NORMAL);
    }
    area
}
#[no_mangle]
pub unsafe extern "C" fn __get_vm_area_caller(
    size: ULong,
    flags: ULong,
    start: ULong,
    end: ULong,
    caller: *const Void,
) -> *mut vm_struct {
    __get_vm_area_node(
        size,
        1,
        PAGE_SHIFT as ULong,
        flags,
        start,
        end,
        NUMA_NO_NODE,
        GFP_KERNEL,
        caller,
    )
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_get_vm_area(
    size: ULong,
    flags: ULong,
    caller: *const Void,
) -> *mut vm_struct {
    __get_vm_area_node(
        size,
        1,
        PAGE_SHIFT as ULong,
        flags,
        vmalloc_start(),
        vmalloc_end(),
        NUMA_NO_NODE,
        GFP_KERNEL,
        caller,
    )
}
#[no_mangle]
pub unsafe extern "C" fn get_vm_area_caller(
    size: ULong,
    flags: ULong,
    caller: *const Void,
) -> *mut vm_struct {
    __get_vm_area_node(
        size,
        1,
        PAGE_SHIFT as ULong,
        flags,
        vmalloc_start(),
        vmalloc_end(),
        NUMA_NO_NODE,
        GFP_KERNEL,
        caller,
    )
}
#[no_mangle]
pub unsafe extern "C" fn find_vm_area(addr: *const Void) -> *mut vm_struct {
    let va = find_vmap_area(addr as ULong);
    if va.is_null() {
        null_mut()
    } else {
        (*va).__bindgen_anon_1.vm
    }
}
#[no_mangle]
pub unsafe extern "C" fn remove_vm_area(addr: *const Void) -> *mut vm_struct {
    might_sleep();
    if rust_vmalloc_warn_vfree_align(addr) {
        return null_mut();
    }
    let va = find_unlink_vmap_area(addr as ULong);
    if va.is_null() || (*va).__bindgen_anon_1.vm.is_null() {
        return null_mut();
    }
    let vm = (*va).__bindgen_anon_1.vm;
    debug_check_no_locks_freed((*vm).addr, get_vm_area_size(vm));
    debug_check_no_obj_freed((*vm).addr, get_vm_area_size(vm));
    kasan_free_module_shadow(vm);
    kasan_poison_vmalloc((*vm).addr, get_vm_area_size(vm));
    free_unmap_vmap_area(va);
    vm
}
unsafe fn set_area_direct_map(area: *mut vm_struct, set: unsafe extern "C" fn(*mut page) -> i32) {
    for i in 0..(*area).nr_pages {
        let page = *(*area).pages.add(i as usize);
        if !page_address(page).is_null() {
            set(page);
        }
    }
}
unsafe fn vm_reset_perms(area: *mut vm_struct) {
    let mut start = ULong::MAX;
    let mut end = 0;
    let mut flush = 0;
    let page_order = vm_area_page_order(area);
    let mut i = 0;
    while i < (*area).nr_pages {
        let addr = page_address(*(*area).pages.add(i as usize)) as ULong;
        if addr != 0 {
            start = min(addr, start);
            end = max(addr.wrapping_add(PAGE_SIZE << page_order), end);
            flush = 1;
        }
        i += (1u32 << page_order) as ULong;
    }
    set_area_direct_map(area, set_direct_map_invalid_noflush);
    _vm_unmap_aliases(start, end, flush);
    set_area_direct_map(area, set_direct_map_default_noflush);
}
#[no_mangle]
unsafe extern "C" fn rust_vmalloc_delayed_vfree_work(w: *mut work_struct) {
    let p = w
        .wrapping_byte_sub(offset_of!(vfree_deferred, wq))
        .cast::<vfree_deferred>();
    let mut node = llist_del_all(&raw mut (*p).list);
    while !node.is_null() {
        let next = (*node).next;
        vfree(node.cast());
        node = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn vfree_atomic(addr: *const Void) {
    let p = rust_vmalloc_deferred_raw();
    bug(in_nmi());
    kmemleak_free(addr);
    if !addr.is_null() && llist_add(addr.cast_mut().cast(), &raw mut (*p).list) {
        schedule_work(&raw mut (*p).wq);
    }
}
unsafe fn vm_area_free_pages(vm: *mut vm_struct, start: ULong, end: ULong) {
    if (*vm).flags & VM_MAP_PUT_PAGES as ULong == 0 {
        for i in start..end {
            mod_lruvec_page_state(*(*vm).pages.add(i as usize), NR_VMALLOC, -1);
        }
    }
    free_pages_bulk((*vm).pages.add(start as usize), end.wrapping_sub(start));
    for i in start..end {
        *(*vm).pages.add(i as usize) = null_mut();
    }
}
#[no_mangle]
pub unsafe extern "C" fn vfree(addr: *const Void) {
    if in_interrupt() {
        vfree_atomic(addr);
        return;
    }
    bug(in_nmi());
    kmemleak_free(addr);
    might_sleep();
    if addr.is_null() {
        return;
    }
    let vm = remove_vm_area(addr);
    if vm.is_null() {
        rust_vmalloc_warn_vfree_missing(addr);
        return;
    }
    if (*vm).flags & VM_FLUSH_RESET_PERMS as ULong != 0 {
        vm_reset_perms(vm);
    }
    vm_area_free_pages(vm, 0, (*vm).nr_pages as ULong);
    kvfree((*vm).pages.cast());
    kfree(vm.cast());
}
#[no_mangle]
pub unsafe extern "C" fn vunmap(addr: *const Void) {
    bug(in_interrupt());
    might_sleep();
    if addr.is_null() {
        return;
    }
    let vm = remove_vm_area(addr);
    if vm.is_null() {
        rust_vmalloc_warn_vunmap_missing(addr);
        return;
    }
    kfree(vm.cast());
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_vmap(
    pages: *mut *mut page,
    count: u32,
    mut flags: ULong,
    prot: pgprot_t,
    caller: *const Void,
) -> *mut Void {
    might_sleep();
    if warn_vmap_reset(flags & VM_FLUSH_RESET_PERMS as ULong != 0) {
        return null_mut();
    }
    if warn_vmap_guard(flags & VM_NO_GUARD as ULong != 0) {
        flags &= !(VM_NO_GUARD as ULong);
    }
    if count as ULong > totalram_pages() {
        return null_mut();
    }
    let size = (count as ULong) << PAGE_SHIFT;
    let area = get_vm_area_caller(size, flags, caller);
    if area.is_null() {
        return null_mut();
    }
    let addr = (*area).addr as ULong;
    if vmap_pages_range(
        addr,
        addr.wrapping_add(size),
        pgprot_nx(prot),
        pages,
        PAGE_SHIFT,
    ) < 0
    {
        vunmap((*area).addr);
        return null_mut();
    }
    if flags & VM_MAP_PUT_PAGES as ULong != 0 {
        (*area).pages = pages;
        (*area).nr_pages = count as ULong;
    }
    (*area).addr
}
#[cfg(CONFIG_VMAP_PFN)]
unsafe extern "C" fn vmap_pfn_apply(pte: *mut pte_t, addr: ULong, private: *mut Void) -> i32 {
    let data = private.cast::<vmap_pfn_data>();
    let pfn = *(*data).pfns.add((*data).idx as usize);
    if warn_vmap_pfn(pfn_valid(pfn)) {
        return -(EINVAL as i32);
    }
    set_pte_at(
        &raw mut init_mm,
        addr,
        pte,
        pte_mkspecial(pfn_pte(pfn, (*data).prot)),
    );
    (*data).idx += 1;
    0
}
#[cfg(CONFIG_VMAP_PFN)]
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_vmap_pfn(
    pfns: *mut ULong,
    count: u32,
    prot: pgprot_t,
    caller: *const Void,
) -> *mut Void {
    let mut data = vmap_pfn_data {
        pfns,
        prot: pgprot_nx(prot),
        idx: 0,
    };
    let area = get_vm_area_caller(count as ULong * PAGE_SIZE, VM_IOREMAP as ULong, caller);
    if area.is_null() {
        return null_mut();
    }
    if apply_to_page_range(
        &raw mut init_mm,
        (*area).addr as ULong,
        count as ULong * PAGE_SIZE,
        Some(vmap_pfn_apply),
        (&mut data as *mut vmap_pfn_data).cast(),
    ) != 0
    {
        free_vm_area(area);
        return null_mut();
    }
    flush_cache_vmap(
        (*area).addr as ULong,
        ((*area).addr as ULong).wrapping_add(count as ULong * PAGE_SIZE),
    );
    (*area).addr
}
unsafe fn vmalloc_gfp_adjust(mut flags: gfp_t, large: bool) -> gfp_t {
    flags |= __GFP_NOWARN;
    if large {
        flags &= !(__GFP_NOFAIL as gfp_t);
    }
    flags
}
// Kernel ilog2(0) is -1 at runtime; Rust's ilog2(0) panics. Preserve that edge.
#[inline(always)]
fn native_ilog2(value: ULong) -> i32 {
    (ULong::BITS - value.leading_zeros()) as i32 - 1
}
unsafe fn vm_area_alloc_pages(
    gfp: gfp_t,
    nid: i32,
    order: u32,
    nr_pages: ULong,
    pages: *mut *mut page,
) -> ULong {
    let mut allocated: ULong = 0;
    let mut remaining = nr_pages;
    let mut max_order = MAX_PAGE_ORDER as u32;
    let mut large_order = native_ilog2(remaining) as u32;
    let large_gfp = vmalloc_gfp_adjust(gfp, large_order != 0) & !(__GFP_DIRECT_RECLAIM as gfp_t);
    large_order = min(max_order, large_order);
    while large_order > order && remaining != 0 {
        let page = if nid == NUMA_NO_NODE {
            alloc_pages_noprof(large_gfp, large_order)
        } else {
            alloc_pages_node_noprof(nid, large_gfp, large_order)
        };
        if page.is_null() {
            large_order -= 1;
            max_order = large_order;
            continue;
        }
        mod_lruvec_page_state(page, NR_VMALLOC, 1 << large_order);
        split_page(page, large_order);
        for i in 0..1u32 << large_order {
            *pages.add((allocated + i as ULong) as usize) = page.add(i as usize);
        }
        allocated += (1u32 << large_order) as ULong;
        remaining = nr_pages - allocated;
        large_order = min(max_order, native_ilog2(remaining) as u32);
    }
    if order == 0 {
        while allocated < nr_pages {
            let request = min(100, nr_pages - allocated) as u32;
            let nr;
            #[cfg(CONFIG_NUMA)]
            {
                nr = if nid == NUMA_NO_NODE {
                    alloc_pages_bulk_mempolicy_noprof(
                        gfp,
                        request as ULong,
                        pages.add(allocated as usize),
                    ) as u32
                } else {
                    alloc_pages_bulk_node_noprof(
                        gfp,
                        nid,
                        request as ULong,
                        pages.add(allocated as usize),
                    ) as u32
                };
            }
            #[cfg(not(CONFIG_NUMA))]
            {
                nr = alloc_pages_bulk_node_noprof(
                    gfp,
                    nid,
                    request as ULong,
                    pages.add(allocated as usize),
                ) as u32;
            }
            for i in allocated..allocated + nr as ULong {
                mod_lruvec_page_state(*pages.add(i as usize), NR_VMALLOC, 1);
            }
            allocated += nr as ULong;
            if nr != request {
                break;
            }
        }
    }
    while allocated < nr_pages {
        if gfp & __GFP_NOFAIL as gfp_t == 0 && fatal_signal_pending_current() {
            break;
        }
        let page = if nid == NUMA_NO_NODE {
            alloc_pages_noprof(gfp, order)
        } else {
            alloc_pages_node_noprof(nid, gfp, order)
        };
        if page.is_null() {
            break;
        }
        mod_lruvec_page_state(page, NR_VMALLOC, 1 << order);
        if order != 0 {
            split_page(page, order);
        }
        for i in 0..1u32 << order {
            *pages.add((allocated + i as ULong) as usize) = page.add(i as usize);
        }
        allocated += (1u32 << order) as ULong;
    }
    allocated
}
static mut pending_vm_area_cleanup: llist_head = llist_head { first: null_mut() };
#[no_mangle]
unsafe extern "C" fn rust_vmalloc_cleanup_vm_area_work(_: *mut work_struct) {
    let mut head = llist_del_all(&raw mut pending_vm_area_cleanup);
    while !head.is_null() {
        let next = (*head).next;
        let area = head
            .wrapping_byte_sub(offset_of!(vm_struct, __bindgen_anon_1))
            .cast::<vm_struct>();
        if (*area).pages.is_null() {
            free_vm_area(area);
        } else {
            vfree((*area).addr);
        }
        head = next;
    }
}
unsafe fn defer_vm_area_cleanup(area: *mut vm_struct) {
    if llist_add(
        &raw mut (*area).__bindgen_anon_1.llnode,
        &raw mut pending_vm_area_cleanup,
    ) {
        schedule_work(rust_vmalloc_cleanup_work());
    }
}
#[no_mangle]
pub unsafe extern "C" fn memalloc_apply_gfp_scope(gfp_mask: gfp_t) -> u32 {
    if !gfpflags_allow_blocking(gfp_mask)
        || gfp_mask & (__GFP_RETRY_MAYFAIL | __GFP_NORETRY) as gfp_t != 0
    {
        memalloc_noreclaim_save()
    } else if gfp_mask & (__GFP_FS | __GFP_IO) as gfp_t == __GFP_IO as gfp_t {
        memalloc_nofs_save()
    } else if gfp_mask & (__GFP_FS | __GFP_IO) as gfp_t == 0 {
        memalloc_noio_save()
    } else {
        0
    }
}
#[no_mangle]
pub unsafe extern "C" fn memalloc_restore_scope(flags: u32) {
    if flags != 0 {
        memalloc_flags_restore(flags);
    }
}
unsafe fn __vmalloc_area_node(
    area: *mut vm_struct,
    mut gfp_mask: gfp_t,
    prot: pgprot_t,
    page_shift: u32,
    node: i32,
) -> *mut Void {
    let nested_gfp = (gfp_mask & GFP_RECLAIM_MASK) | __GFP_ZERO as gfp_t;
    let nofail = gfp_mask & __GFP_NOFAIL as gfp_t != 0 && gfpflags_allow_blocking(gfp_mask);
    let addr = (*area).addr as ULong;
    let size = get_vm_area_size(area);
    let nr = size >> PAGE_SHIFT;
    let array_size = nr * size_of::<*mut page>() as ULong;
    if gfp_mask & (GFP_DMA | GFP_DMA32) as gfp_t == 0 {
        gfp_mask |= __GFP_HIGHMEM as gfp_t;
    }
    (*area).pages = if array_size > PAGE_SIZE {
        __vmalloc_node_noprof(array_size, 1, nested_gfp, node, (*area).caller).cast()
    } else {
        kmalloc_node_noprof(array_size as usize, nested_gfp, node).cast()
    };
    if (*area).pages.is_null() {
        rust_vmalloc_warn_array(gfp_mask, nr * PAGE_SIZE, array_size);
        defer_vm_area_cleanup(area);
        return null_mut();
    }
    set_vm_area_page_order(area, page_shift - PAGE_SHIFT);
    let page_order = vm_area_page_order(area);
    (*area).nr_pages = vm_area_alloc_pages(
        vmalloc_gfp_adjust(gfp_mask, page_order != 0),
        node,
        page_order,
        nr,
        (*area).pages,
    ) as _;
    if (*area).nr_pages as ULong != nr {
        if !fatal_signal_pending_current() && page_order == 0 {
            rust_vmalloc_warn_pages(gfp_mask, nr * PAGE_SIZE);
        }
        defer_vm_area_cleanup(area);
        return null_mut();
    }
    let flags = memalloc_apply_gfp_scope(gfp_mask);
    let mut ret;
    loop {
        ret = __vmap_pages_range(
            addr,
            addr.wrapping_add(size),
            prot,
            (*area).pages,
            page_shift,
            nested_gfp,
        );
        if !(nofail && ret < 0) {
            break;
        }
        schedule_timeout_uninterruptible(1);
    }
    memalloc_restore_scope(flags);
    if ret < 0 {
        rust_vmalloc_warn_map(gfp_mask, (*area).nr_pages as ULong * PAGE_SIZE);
        defer_vm_area_cleanup(area);
        return null_mut();
    }
    (*area).addr
}
const GFP_VMALLOC_SUPPORTED: gfp_t = RVM_GFP_VMALLOC_SUPPORTED as gfp_t;
unsafe fn vmalloc_fix_flags(mut flags: gfp_t) -> gfp_t {
    let invalid = flags & !GFP_VMALLOC_SUPPORTED;
    flags &= GFP_VMALLOC_SUPPORTED;
    rust_vmalloc_warn_gfp(invalid, flags);
    flags
}
#[no_mangle]
pub unsafe extern "C" fn __vmalloc_node_range_noprof(
    size: ULong,
    mut align: ULong,
    start: ULong,
    end: ULong,
    mut gfp_mask: gfp_t,
    mut prot: pgprot_t,
    vm_flags: ULong,
    node: i32,
    caller: *const Void,
) -> *mut Void {
    let mut kasan_flags = KASAN_VMALLOC_NONE;
    let original_align = align;
    let mut shift = PAGE_SHIFT;
    let skip_kasan = kasan_hw_tags_enabled() && gfp_mask & __GFP_SKIP_KASAN as gfp_t != 0;
    if warn_vmalloc_zero(size == 0) {
        return null_mut();
    }
    if size >> PAGE_SHIFT > totalram_pages() {
        rust_vmalloc_warn_total(gfp_mask, size);
        return null_mut();
    }
    if vmap_allow_huge && vm_flags & VM_ALLOW_HUGE_VMAP as ULong != 0 {
        shift = if arch_vmap_pmd_supported(prot) && size >= pmd_size() {
            pmd_shift() as u32
        } else {
            arch_vmap_pte_supported_shift(size)
        };
        align = max(original_align, 1 << shift);
    }
    loop {
        let area = __get_vm_area_node(
            size,
            align,
            shift as ULong,
            VM_ALLOC as ULong | VM_UNINITIALIZED as ULong | vm_flags,
            start,
            end,
            node,
            gfp_mask & !(__GFP_SKIP_KASAN as gfp_t),
            caller,
        );
        if area.is_null() {
            let nofail = gfp_mask & __GFP_NOFAIL as gfp_t != 0;
            rust_vmalloc_warn_area(gfp_mask, size, align, nofail);
            if nofail {
                schedule_timeout_uninterruptible(1);
                continue;
            }
        } else {
            if pgprot_val(prot) == pgprot_val(page_kernel()) {
                if kasan_hw_tags_enabled() && !skip_kasan {
                    prot = arch_vmap_pgprot_tagged(prot);
                    gfp_mask |= (__GFP_SKIP_KASAN | __GFP_SKIP_ZERO) as gfp_t;
                }
                kasan_flags |= KASAN_VMALLOC_PROT_NORMAL;
            }
            if !__vmalloc_area_node(area, gfp_mask, prot, shift, node).is_null() {
                kasan_flags |= KASAN_VMALLOC_VM_ALLOC;
                if !want_init_on_free()
                    && want_init_on_alloc(gfp_mask)
                    && gfp_mask & __GFP_SKIP_ZERO as gfp_t != 0
                {
                    kasan_flags |= KASAN_VMALLOC_INIT;
                }
                if !skip_kasan {
                    (*area).addr = kasan_unpoison_vmalloc((*area).addr, size, kasan_flags);
                }
                clear_vm_uninitialized_flag(area);
                if vm_flags & VM_DEFER_KMEMLEAK as ULong == 0 {
                    kmemleak_vmalloc(area, align_up(size, PAGE_SIZE) as usize, gfp_mask);
                }
                return (*area).addr;
            }
        }
        if shift > PAGE_SHIFT {
            shift = PAGE_SHIFT;
            align = original_align;
        } else {
            return null_mut();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn __vmalloc_node_noprof(
    size: ULong,
    align: ULong,
    gfp_mask: gfp_t,
    node: i32,
    caller: *const Void,
) -> *mut Void {
    __vmalloc_node_range_noprof(
        size,
        align,
        vmalloc_start(),
        vmalloc_end(),
        gfp_mask,
        page_kernel(),
        0,
        node,
        caller,
    )
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc___vmalloc_noprof(
    size: ULong,
    mut gfp_mask: gfp_t,
    caller: *const Void,
) -> *mut Void {
    if gfp_mask & !GFP_VMALLOC_SUPPORTED != 0 {
        gfp_mask = vmalloc_fix_flags(gfp_mask);
    }
    __vmalloc_node_noprof(size, 1, gfp_mask, NUMA_NO_NODE, caller)
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_vmalloc_noprof(
    size: ULong,
    caller: *const Void,
) -> *mut Void {
    __vmalloc_node_noprof(size, 1, GFP_KERNEL, NUMA_NO_NODE, caller)
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_vmalloc_huge_node_noprof(
    size: ULong,
    mut gfp_mask: gfp_t,
    node: i32,
    caller: *const Void,
) -> *mut Void {
    if gfp_mask & !GFP_VMALLOC_SUPPORTED != 0 {
        gfp_mask = vmalloc_fix_flags(gfp_mask);
    }
    __vmalloc_node_range_noprof(
        size,
        1,
        vmalloc_start(),
        vmalloc_end(),
        gfp_mask,
        page_kernel(),
        VM_ALLOW_HUGE_VMAP as ULong,
        node,
        caller,
    )
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_vzalloc_noprof(
    size: ULong,
    caller: *const Void,
) -> *mut Void {
    __vmalloc_node_noprof(
        size,
        1,
        GFP_KERNEL | __GFP_ZERO as gfp_t,
        NUMA_NO_NODE,
        caller,
    )
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_vmalloc_user_noprof(
    size: ULong,
    caller: *const Void,
) -> *mut Void {
    __vmalloc_node_range_noprof(
        size,
        RVM_SHMLBA as ULong,
        vmalloc_start(),
        vmalloc_end(),
        GFP_KERNEL | __GFP_ZERO as gfp_t,
        page_kernel(),
        VM_USERMAP as ULong,
        NUMA_NO_NODE,
        caller,
    )
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_vmalloc_node_noprof(
    size: ULong,
    node: i32,
    caller: *const Void,
) -> *mut Void {
    __vmalloc_node_noprof(size, 1, GFP_KERNEL, node, caller)
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_vzalloc_node_noprof(
    size: ULong,
    node: i32,
    caller: *const Void,
) -> *mut Void {
    __vmalloc_node_noprof(size, 1, GFP_KERNEL | __GFP_ZERO as gfp_t, node, caller)
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_vrealloc_node_align_noprof(
    p: *const Void,
    size: usize,
    align: ULong,
    flags: gfp_t,
    nid: i32,
    caller: *const Void,
) -> *mut Void {
    let mut vm: *mut vm_struct = null_mut();
    let mut old_size: usize = 0;
    if size == 0 {
        vfree(p);
        return null_mut();
    }
    let mut reallocate = p.is_null();
    if !p.is_null() {
        vm = find_vm_area(p);
        if vm.is_null() {
            rust_vmalloc_warn_vrealloc_missing(p);
            return null_mut();
        }
        let allocated_size = get_vm_area_size(vm) as usize;
        old_size = (*vm).requested_size as usize;
        if rust_vmalloc_warn_vrealloc_size(allocated_size < old_size, p) {
            return null_mut();
        }
        if rust_vmalloc_warn_vrealloc_align(!is_aligned(p as ULong, align), align) {
            return null_mut();
        }
        reallocate = flags & __GFP_THISNODE as gfp_t != 0
            && nid != NUMA_NO_NODE
            && nid != page_to_nid(vmalloc_to_page(p));
    }
    if !reallocate {
        if size <= old_size {
            let new_nr = align_up(size as ULong, PAGE_SIZE) >> PAGE_SHIFT;
            if want_init_on_free() || want_init_on_alloc(flags) {
                memset(p.cast_mut().byte_add(size), 0, old_size - size);
            }
            if new_nr < (*vm).nr_pages
                && vm_area_page_order(vm) == 0
                && (*vm).flags & (VM_FLUSH_RESET_PERMS | VM_USERMAP) as ULong == 0
                && gfp_has_io_fs(flags)
            {
                let addr = kasan_reset_tag(p) as ULong;
                let old_nr = (*vm).nr_pages;
                let vn = addr_to_node(addr);
                spin_lock(&raw mut (*vn).busy.lock);
                (*vm).nr_pages = new_nr;
                spin_unlock(&raw mut (*vn).busy.lock);
                kmemleak_free_part(
                    addr.wrapping_add(new_nr << PAGE_SHIFT) as *const Void,
                    ((old_nr - new_nr) << PAGE_SHIFT) as usize,
                );
                vunmap_range(
                    addr.wrapping_add(new_nr << PAGE_SHIFT),
                    addr.wrapping_add(old_nr << PAGE_SHIFT),
                );
                vm_area_free_pages(vm, new_nr, old_nr);
            }
            (*vm).requested_size = size as ULong;
            kasan_vrealloc(p, old_size, size);
            return p.cast_mut();
        }
        if size as ULong <= (*vm).nr_pages << PAGE_SHIFT {
            (*vm).requested_size = size as ULong;
            kasan_vrealloc(p, old_size, size);
            return p.cast_mut();
        }
    }
    let n = __vmalloc_node_noprof(size as ULong, align, flags, nid, caller);
    if n.is_null() {
        return null_mut();
    }
    if !p.is_null() {
        memcpy(n, p, min(size, old_size));
        vfree(p);
    }
    n
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_vmalloc_32_noprof(
    size: ULong,
    caller: *const Void,
) -> *mut Void {
    __vmalloc_node_noprof(size, 1, RVM_GFP_VMALLOC32 as gfp_t, NUMA_NO_NODE, caller)
}
#[no_mangle]
pub unsafe extern "C" fn rust_vmalloc_vmalloc_32_user_noprof(
    size: ULong,
    caller: *const Void,
) -> *mut Void {
    __vmalloc_node_range_noprof(
        size,
        RVM_SHMLBA as ULong,
        vmalloc_start(),
        vmalloc_end(),
        RVM_GFP_VMALLOC32 as gfp_t | __GFP_ZERO as gfp_t,
        page_kernel(),
        VM_USERMAP as ULong,
        NUMA_NO_NODE,
        caller,
    )
}
