// SPDX-License-Identifier: GPL-2.0-only
// vmalloc.c:4894-5240. Congruent, top-down per-CPU virtual area allocation.
#[cfg(CONFIG_SMP)]
unsafe fn node_to_va(n: *mut rb_node) -> *mut vmap_area {
    if n.is_null() {
        null_mut()
    } else {
        va_from_rb(n)
    }
}
#[cfg(CONFIG_SMP)]
unsafe fn pvm_find_va_enclose_addr(addr: ULong) -> *mut vmap_area {
    let mut n = free_vmap_area_root.rb_node;
    let mut va = null_mut();
    while !n.is_null() {
        let tmp = va_from_rb(n);
        if (*tmp).va_start <= addr {
            va = tmp;
            if (*tmp).va_end >= addr {
                break;
            }
            n = (*n).rb_right;
        } else {
            n = (*n).rb_left;
        }
    }
    va
}
#[cfg(CONFIG_SMP)]
unsafe fn pvm_determine_end_from_reverse(va: *mut *mut vmap_area, align: ULong) -> ULong {
    let end = vmalloc_end() & !(align - 1);
    if !(*va).is_null() {
        while &raw mut (**va).list != &raw mut free_vmap_area_list {
            let addr = min((**va).va_end & !(align - 1), end);
            if (**va).va_start < addr {
                return addr;
            }
            *va = va_from_list((**va).list.prev);
        }
    }
    0
}
#[cfg(CONFIG_SMP)]
unsafe fn pcpu_release_unpublished(vas: *mut *mut vmap_area, i: usize) {
    let area = *vas.add(i);
    let start = (*area).va_start;
    let end = (*area).va_end;
    let merged = merge_or_add_vmap_area_augment(
        area,
        &raw mut free_vmap_area_root,
        &raw mut free_vmap_area_list,
    );
    if !merged.is_null() {
        kasan_release_vmalloc(
            start,
            end,
            (*merged).va_start,
            (*merged).va_end,
            (KASAN_VMALLOC_PAGE_RANGE | KASAN_VMALLOC_TLB_FLUSH) as ULong,
        );
    }
    *vas.add(i) = null_mut();
}
#[cfg(CONFIG_SMP)]
#[no_mangle]
pub unsafe extern "C" fn pcpu_get_vm_areas(
    offsets: *const ULong,
    sizes: *const usize,
    nr_vms: i32,
    align: usize,
    gfp: gfp_t,
) -> *mut *mut vm_struct {
    let start_limit = align_up(vmalloc_start(), align as ULong);
    let end_limit = vmalloc_end() & !(align as ULong - 1);
    bug(align as ULong & (PAGE_SIZE - 1) != 0 || !align.is_power_of_two());
    let mut last_area = 0usize;
    for area in 0..nr_vms {
        let area = area as usize;
        let start = *offsets.add(area);
        let end = start.wrapping_add(*sizes.add(area) as ULong);
        bug(!is_aligned(*offsets.add(area), align as ULong));
        bug(!is_aligned(*sizes.add(area) as ULong, align as ULong));
        if start > *offsets.add(last_area) {
            last_area = area;
        }
        for area2 in area + 1..nr_vms as usize {
            let start2 = *offsets.add(area2);
            let end2 = start2.wrapping_add(*sizes.add(area2) as ULong);
            bug(start2 < end && start < end2);
        }
    }
    let last_end = (*offsets.add(last_area)).wrapping_add(*sizes.add(last_area) as ULong);
    if end_limit.wrapping_sub(start_limit) < last_end {
        warn(true);
        return null_mut();
    }
    let vms = kcalloc(nr_vms as usize, size_of::<*mut vm_struct>(), gfp).cast::<*mut vm_struct>();
    let vas = kcalloc(nr_vms as usize, size_of::<*mut vmap_area>(), gfp).cast::<*mut vmap_area>();
    if vas.is_null() || vms.is_null() {
        kfree(vas.cast());
        kfree(vms.cast());
        return null_mut();
    }
    let mut initialized = true;
    for area in 0..nr_vms as usize {
        *vas.add(area) = kmem_cache_zalloc(vmap_area_cachep, gfp).cast();
        *vms.add(area) = kzalloc_node(size_of::<vm_struct>(), gfp, NUMA_NO_NODE).cast();
        if (*vas.add(area)).is_null() || (*vms.add(area)).is_null() {
            initialized = false;
            break;
        }
    }
    let mut purged = false;
    'retry: while initialized {
        spin_lock(rust_vmalloc_free_lock());
        let mut area = last_area;
        let mut term_area = last_area;
        let mut start = *offsets.add(area);
        let mut end = start.wrapping_add(*sizes.add(area) as ULong);
        let mut va = pvm_find_va_enclose_addr(end_limit);
        let mut base = pvm_determine_end_from_reverse(&mut va, align as ULong).wrapping_sub(end);
        let mut found = true;
        loop {
            if base.wrapping_add(last_end) < start_limit.wrapping_add(last_end) || va.is_null() {
                found = false;
                break;
            }
            if base.wrapping_add(end) > (*va).va_end {
                base = pvm_determine_end_from_reverse(&mut va, align as ULong).wrapping_sub(end);
                term_area = area;
                continue;
            }
            if base.wrapping_add(start) < (*va).va_start {
                va = node_to_va(rb_prev(&raw const (*va).rb_node));
                base = pvm_determine_end_from_reverse(&mut va, align as ULong).wrapping_sub(end);
                term_area = area;
                continue;
            }
            area = (area + nr_vms as usize - 1) % nr_vms as usize;
            if area == term_area {
                break;
            }
            start = *offsets.add(area);
            end = start.wrapping_add(*sizes.add(area) as ULong);
            va = pvm_find_va_enclose_addr(base.wrapping_add(end));
        }
        if found {
            area = 0;
            while area < nr_vms as usize {
                start = base.wrapping_add(*offsets.add(area));
                let size = *sizes.add(area) as ULong;
                va = pvm_find_va_enclose_addr(start);
                if warn_pcpu_find(va.is_null()) {
                    break;
                }
                let ret = va_clip(
                    &raw mut free_vmap_area_root,
                    &raw mut free_vmap_area_list,
                    va,
                    start,
                    size,
                );
                if warn_pcpu_clip(ret != 0) {
                    break;
                }
                va = *vas.add(area);
                (*va).va_start = start;
                (*va).va_end = start.wrapping_add(size);
                area += 1;
            }
            if area == nr_vms as usize {
                spin_unlock(rust_vmalloc_free_lock());
                let mut shadow_ok = true;
                for j in 0..nr_vms as usize {
                    if kasan_populate_vmalloc((**vas.add(j)).va_start, *sizes.add(j) as ULong, gfp)
                        != 0
                    {
                        shadow_ok = false;
                        break;
                    }
                }
                if !shadow_ok {
                    spin_lock(rust_vmalloc_free_lock());
                    for j in 0..nr_vms as usize {
                        pcpu_release_unpublished(vas, j);
                        kfree((*vms.add(j)).cast());
                    }
                    spin_unlock(rust_vmalloc_free_lock());
                    kfree(vas.cast());
                    kfree(vms.cast());
                    return null_mut();
                }
                for j in 0..nr_vms as usize {
                    let va = *vas.add(j);
                    let vn = addr_to_node((*va).va_start);
                    spin_lock(&raw mut (*vn).busy.lock);
                    insert_vmap_area(va, &raw mut (*vn).busy.root, &raw mut (*vn).busy.head);
                    setup_vmalloc_vm(
                        *vms.add(j),
                        va,
                        VM_ALLOC as ULong,
                        pcpu_get_vm_areas as *const () as *const Void,
                    );
                    spin_unlock(&raw mut (*vn).busy.lock);
                }
                kasan_unpoison_vmap_areas(vms, nr_vms, KASAN_VMALLOC_PROT_NORMAL);
                kfree(vas.cast());
                return vms;
            }
            while area > 0 {
                area -= 1;
                pcpu_release_unpublished(vas, area);
            }
        }
        spin_unlock(rust_vmalloc_free_lock());
        if !purged {
            reclaim_and_purge_vmap_areas();
            purged = true;
            for j in 0..nr_vms as usize {
                if !(*vas.add(j)).is_null() {
                    continue;
                }
                *vas.add(j) = kmem_cache_zalloc(vmap_area_cachep, gfp).cast();
                if (*vas.add(j)).is_null() {
                    break 'retry;
                }
            }
            continue;
        }
        break;
    }
    for area in 0..nr_vms as usize {
        if !(*vas.add(area)).is_null() {
            kmem_cache_free(vmap_area_cachep, (*vas.add(area)).cast());
        }
        kfree((*vms.add(area)).cast());
    }
    kfree(vas.cast());
    kfree(vms.cast());
    null_mut()
}
#[cfg(CONFIG_SMP)]
#[no_mangle]
pub unsafe extern "C" fn pcpu_free_vm_areas(vms: *mut *mut vm_struct, nr_vms: i32) {
    for i in 0..nr_vms {
        free_vm_area(*vms.add(i as usize));
    }
    kfree(vms.cast());
}
