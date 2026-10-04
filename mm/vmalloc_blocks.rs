// SPDX-License-Identifier: GPL-2.0-only
// vmalloc.c:2580-3123. Per-CPU block ownership and dirty-range accounting.
const VMAP_BBMAP_BITS: ULong = RVM_VMAP_BBMAP_BITS as ULong;
const VMAP_BLOCK_SIZE: ULong = RVM_VMAP_BLOCK_SIZE as ULong;
const VMAP_MAX_ALLOC: u32 = RVM_VMAP_MAX_ALLOC as u32;
const VMAP_PURGE_THRESHOLD: ULong = RVM_VMAP_PURGE_THRESHOLD as ULong;
const VMAP_RAM: ULong = 0x1;
const VMAP_BLOCK: ULong = 0x2;
const VMAP_FLAGS_MASK: ULong = 0x3;
unsafe fn vb_from_free_list(n: *mut list_head) -> *mut vmap_block {
    n.wrapping_byte_sub(offset_of!(vmap_block, free_list))
        .cast()
}
unsafe fn vb_from_purge(n: *mut list_head) -> *mut vmap_block {
    n.wrapping_byte_sub(offset_of!(vmap_block, purge)).cast()
}
unsafe fn addr_to_vb_xa(addr: ULong) -> *mut xarray {
    let mut index = ((addr / VMAP_BLOCK_SIZE) % nr_cpu_ids as ULong) as i32;
    if !cpu_possible(index as u32) {
        index = cpumask_next(index, possible_mask());
    }
    &raw mut (*rust_vmalloc_vbq(index)).vmap_blocks
}
unsafe fn addr_to_vb_idx(addr: ULong) -> ULong {
    addr.wrapping_sub(vmalloc_start() & !(VMAP_BLOCK_SIZE - 1)) / VMAP_BLOCK_SIZE
}
unsafe fn vmap_block_vaddr(start: ULong, pages_off: ULong) -> *mut Void {
    let addr = start.wrapping_add(pages_off << PAGE_SHIFT);
    bug(addr_to_vb_idx(addr) != addr_to_vb_idx(start));
    addr as *mut Void
}
unsafe fn new_vmap_block(order: u32, gfp_mask: gfp_t) -> *mut Void {
    let node = numa_node_id();
    let vb = kmalloc_node(size_of::<vmap_block>(), gfp_mask, node).cast::<vmap_block>();
    if vb.is_null() {
        return err_ptr(-(ENOMEM as i32));
    }
    let va = alloc_vmap_area(
        VMAP_BLOCK_SIZE,
        VMAP_BLOCK_SIZE,
        vmalloc_start(),
        vmalloc_end(),
        node,
        gfp_mask,
        VMAP_RAM | VMAP_BLOCK,
        null_mut(),
    );
    if err_value(va as ULong) {
        kfree(vb.cast());
        return va.cast();
    }
    let vaddr = vmap_block_vaddr((*va).va_start, 0);
    rust_vmalloc_vb_lock_init(vb);
    (*vb).va = va;
    bug(VMAP_BBMAP_BITS <= 1 << order);
    bitmap_zero((&raw mut (*vb).used_map).cast(), VMAP_BBMAP_BITS as u32);
    (*vb).free = VMAP_BBMAP_BITS - (1 << order);
    (*vb).dirty = 0;
    (*vb).dirty_min = VMAP_BBMAP_BITS;
    (*vb).dirty_max = 0;
    bitmap_set((&raw mut (*vb).used_map).cast(), 0, 1 << order);
    init_list_head(&raw mut (*vb).free_list);
    (*vb).cpu = raw_smp_processor_id();
    let xa = addr_to_vb_xa((*va).va_start);
    let idx = addr_to_vb_idx((*va).va_start);
    let err = xa_insert(xa, idx, vb.cast(), gfp_mask);
    if err != 0 {
        kfree(vb.cast());
        free_vmap_area(va);
        return err_ptr(err);
    }
    let vbq = rust_vmalloc_vbq((*vb).cpu as i32);
    spin_lock(&raw mut (*vbq).lock);
    list_add_tail_rcu(&raw mut (*vb).free_list, &raw mut (*vbq).free);
    spin_unlock(&raw mut (*vbq).lock);
    vaddr
}
unsafe fn free_vmap_block(vb: *mut vmap_block) {
    let xa = addr_to_vb_xa((*(*vb).va).va_start);
    let tmp = xa_erase(xa, addr_to_vb_idx((*(*vb).va).va_start));
    bug(tmp != vb.cast());
    let vn = addr_to_node((*(*vb).va).va_start);
    spin_lock(&raw mut (*vn).busy.lock);
    unlink_va((*vb).va, &raw mut (*vn).busy.root);
    spin_unlock(&raw mut (*vn).busy.lock);
    free_vmap_area_noflush((*vb).va);
    rust_vmalloc_free_vb_rcu(vb);
}
unsafe fn purge_fragmented_block(vb: *mut vmap_block, purge: *mut list_head, force: bool) -> bool {
    let vbq = rust_vmalloc_vbq((*vb).cpu as i32);
    if (*vb).free.wrapping_add((*vb).dirty) != VMAP_BBMAP_BITS || (*vb).dirty == VMAP_BBMAP_BITS {
        return false;
    }
    if !(force || (*vb).free < VMAP_PURGE_THRESHOLD) {
        return false;
    }
    write_ulong(&raw mut (*vb).free, 0);
    write_ulong(&raw mut (*vb).dirty, VMAP_BBMAP_BITS);
    (*vb).dirty_min = 0;
    (*vb).dirty_max = VMAP_BBMAP_BITS;
    spin_lock(&raw mut (*vbq).lock);
    list_del_rcu(&raw mut (*vb).free_list);
    spin_unlock(&raw mut (*vbq).lock);
    list_add_tail(&raw mut (*vb).purge, purge);
    true
}
unsafe fn free_purged_blocks(purge: *mut list_head) {
    let mut cursor = (*purge).next;
    while cursor != purge {
        let next = (*cursor).next;
        let vb = vb_from_purge(cursor);
        list_del(&raw mut (*vb).purge);
        free_vmap_block(vb);
        cursor = next;
    }
}
unsafe fn purge_fragmented_blocks(cpu: i32) {
    let mut purge: list_head = zeroed();
    init_list_head(&mut purge);
    let vbq = rust_vmalloc_vbq(cpu);
    let head = &raw mut (*vbq).free;
    rcu_read_lock();
    let mut cursor = list_next_rcu(head);
    while cursor != head {
        let vb = vb_from_free_list(cursor);
        let free = read_ulong(&raw const (*vb).free);
        let dirty = read_ulong(&raw const (*vb).dirty);
        if free.wrapping_add(dirty) == VMAP_BBMAP_BITS && dirty != VMAP_BBMAP_BITS {
            spin_lock(&raw mut (*vb).lock);
            purge_fragmented_block(vb, &mut purge, true);
            spin_unlock(&raw mut (*vb).lock);
        }
        cursor = list_next_rcu(cursor);
    }
    rcu_read_unlock();
    free_purged_blocks(&mut purge);
}
unsafe fn purge_fragmented_blocks_allcpus() {
    let mut cpu = cpumask_next(-1, possible_mask());
    while (cpu as u32) < nr_cpu_ids {
        purge_fragmented_blocks(cpu);
        cpu = cpumask_next(cpu, possible_mask());
    }
}
unsafe fn vb_alloc(size: ULong, gfp_mask: gfp_t) -> *mut Void {
    bug(size & (PAGE_SIZE - 1) != 0);
    bug(size > PAGE_SIZE * VMAP_MAX_ALLOC as ULong);
    if warn(size == 0) {
        return err_ptr(-(EINVAL as i32));
    }
    let order = get_order(size);
    let mut vaddr = null_mut();
    rcu_read_lock();
    let vbq = rust_vmalloc_raw_vbq();
    let head = &raw mut (*vbq).free;
    let mut cursor = list_next_rcu(head);
    while cursor != head {
        let vb = vb_from_free_list(cursor);
        if read_ulong(&raw const (*vb).free) >= 1 << order {
            spin_lock(&raw mut (*vb).lock);
            if (*vb).free >= 1 << order {
                let offset = VMAP_BBMAP_BITS - (*vb).free;
                vaddr = vmap_block_vaddr((*(*vb).va).va_start, offset);
                write_ulong(&raw mut (*vb).free, (*vb).free.wrapping_sub(1 << order));
                bitmap_set((&raw mut (*vb).used_map).cast(), offset as u32, 1 << order);
                if (*vb).free == 0 {
                    spin_lock(&raw mut (*vbq).lock);
                    list_del_rcu(&raw mut (*vb).free_list);
                    spin_unlock(&raw mut (*vbq).lock);
                }
                spin_unlock(&raw mut (*vb).lock);
                break;
            }
            spin_unlock(&raw mut (*vb).lock);
        }
        cursor = list_next_rcu(cursor);
    }
    rcu_read_unlock();
    if vaddr.is_null() {
        vaddr = new_vmap_block(order, gfp_mask);
    }
    vaddr
}
unsafe fn vb_free(addr: ULong, size: ULong) {
    bug(size & (PAGE_SIZE - 1) != 0);
    bug(size > PAGE_SIZE * VMAP_MAX_ALLOC as ULong);
    flush_cache_vunmap(addr, addr.wrapping_add(size));
    let order = get_order(size);
    let offset = (addr & (VMAP_BLOCK_SIZE - 1)) >> PAGE_SHIFT;
    let xa = addr_to_vb_xa(addr);
    let vb = xa_load(xa, addr_to_vb_idx(addr)).cast::<vmap_block>();
    spin_lock(&raw mut (*vb).lock);
    bitmap_clear((&raw mut (*vb).used_map).cast(), offset as u32, 1 << order);
    spin_unlock(&raw mut (*vb).lock);
    vunmap_range_noflush(addr, addr.wrapping_add(size));
    if debug_pagealloc_enabled_static() {
        flush_tlb_kernel_range(addr, addr.wrapping_add(size));
    }
    spin_lock(&raw mut (*vb).lock);
    (*vb).dirty_min = min((*vb).dirty_min, offset);
    (*vb).dirty_max = max((*vb).dirty_max, offset.wrapping_add(1 << order));
    write_ulong(&raw mut (*vb).dirty, (*vb).dirty.wrapping_add(1 << order));
    if (*vb).dirty == VMAP_BBMAP_BITS {
        bug((*vb).free != 0);
        spin_unlock(&raw mut (*vb).lock);
        free_vmap_block(vb);
    } else {
        spin_unlock(&raw mut (*vb).lock);
    }
}
unsafe fn _vm_unmap_aliases(mut start: ULong, mut end: ULong, mut flush: i32) {
    let mut purge: list_head = zeroed();
    init_list_head(&mut purge);
    if !vmap_initialized {
        return;
    }
    rust_vmalloc_purge_lock();
    let mut cpu = cpumask_next(-1, possible_mask());
    while (cpu as u32) < nr_cpu_ids {
        let vbq = rust_vmalloc_vbq(cpu);
        let xa = &raw mut (*vbq).vmap_blocks;
        let mut idx = 0;
        rcu_read_lock();
        let mut vb = xa_find(xa, &mut idx, ULong::MAX, XA_PRESENT).cast::<vmap_block>();
        while !vb.is_null() {
            spin_lock(&raw mut (*vb).lock);
            if !purge_fragmented_block(vb, &mut purge, false)
                && (*vb).dirty_max != 0
                && (*vb).dirty != VMAP_BBMAP_BITS
            {
                let va_start = (*(*vb).va).va_start;
                let s = va_start.wrapping_add((*vb).dirty_min << PAGE_SHIFT);
                let e = va_start.wrapping_add((*vb).dirty_max << PAGE_SHIFT);
                start = min(s, start);
                end = max(e, end);
                (*vb).dirty_min = VMAP_BBMAP_BITS;
                (*vb).dirty_max = 0;
                flush = 1;
            }
            spin_unlock(&raw mut (*vb).lock);
            vb = xa_find_after(xa, &mut idx, ULong::MAX, XA_PRESENT).cast();
        }
        rcu_read_unlock();
        cpu = cpumask_next(cpu, possible_mask());
    }
    free_purged_blocks(&mut purge);
    if !__purge_vmap_area_lazy(start, end, false) && flush != 0 {
        flush_tlb_kernel_range(start, end);
    }
    rust_vmalloc_purge_unlock();
}
#[no_mangle]
pub unsafe extern "C" fn vm_unmap_aliases() {
    _vm_unmap_aliases(ULong::MAX, 0, 0);
}
#[no_mangle]
pub unsafe extern "C" fn vm_unmap_ram(mem: *const Void, count: u32) {
    let size = (count as ULong) << PAGE_SHIFT;
    let addr = kasan_reset_tag(mem) as ULong;
    might_sleep();
    bug(addr == 0);
    bug(addr < vmalloc_start());
    bug(addr > vmalloc_end());
    bug(!is_aligned(addr, PAGE_SIZE));
    kasan_poison_vmalloc(mem, size);
    if count <= VMAP_MAX_ALLOC {
        debug_check_no_locks_freed(mem, size);
        vb_free(addr, size);
        return;
    }
    let va = find_unlink_vmap_area(addr);
    if warn_unmap_ram(va.is_null()) {
        return;
    }
    debug_check_no_locks_freed((*va).va_start as *const Void, va_size(va));
    free_unmap_vmap_area(va);
}
#[no_mangle]
pub unsafe extern "C" fn vm_map_ram(pages: *mut *mut page, count: u32, node: i32) -> *mut Void {
    let size = (count as ULong) << PAGE_SHIFT;
    let mut mem;
    if count <= VMAP_MAX_ALLOC {
        mem = vb_alloc(size, GFP_KERNEL);
        if err_value(mem as ULong) {
            return null_mut();
        }
    } else {
        let va = alloc_vmap_area(
            size,
            PAGE_SIZE,
            vmalloc_start(),
            vmalloc_end(),
            node,
            GFP_KERNEL,
            VMAP_RAM,
            null_mut(),
        );
        if err_value(va as ULong) {
            return null_mut();
        }
        mem = (*va).va_start as *mut Void;
    }
    let addr = mem as ULong;
    if vmap_pages_range(
        addr,
        addr.wrapping_add(size),
        page_kernel(),
        pages,
        PAGE_SHIFT,
    ) < 0
    {
        vm_unmap_ram(mem, count);
        return null_mut();
    }
    mem = kasan_unpoison_vmalloc(mem, size, KASAN_VMALLOC_PROT_NORMAL);
    mem
}
