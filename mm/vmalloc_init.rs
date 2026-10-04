// SPDX-License-Identifier: GPL-2.0-only
// vmalloc.c:5241-5589. Reporting, import of early reservations and initialization.
#[cfg(CONFIG_PRINTK)]
#[no_mangle]
pub unsafe extern "C" fn vmalloc_dump_obj(object: *mut Void) -> bool {
    let addr = align_up(object as ULong, PAGE_SIZE);
    let vn = addr_to_node(addr);
    if !spin_trylock(&raw mut (*vn).busy.lock) {
        return false;
    }
    let va = __find_vmap_area(addr, &raw mut (*vn).busy.root);
    if va.is_null() || (*va).__bindgen_anon_1.vm.is_null() {
        spin_unlock(&raw mut (*vn).busy.lock);
        return false;
    }
    let vm = (*va).__bindgen_anon_1.vm;
    let addr = (*vm).addr as ULong;
    let caller = (*vm).caller;
    let nr = (*vm).nr_pages;
    spin_unlock(&raw mut (*vn).busy.lock);
    rust_vmalloc_dump_region(nr, addr, caller);
    true
}
#[cfg(CONFIG_PROC_FS)]
unsafe fn show_numa_info(m: *mut seq_file, v: *mut vm_struct, counters: *mut u32) {
    let step = 1u32 << vm_area_page_order(v);
    if counters.is_null() {
        return;
    }
    memset(
        counters.cast(),
        0,
        node_id_count() as usize * size_of::<u32>(),
    );
    let mut i = 0;
    while i < (*v).nr_pages {
        let counter = counters.add(page_to_nid(*(*v).pages.add(i as usize)) as usize);
        *counter = (*counter).wrapping_add(step);
        i += step as ULong;
    }
    let mut nr = first_high_memory_node();
    while nr < RVM_MAX_NUMNODES as u32 {
        if *counters.add(nr as usize) != 0 {
            seq_printf(
                m,
                c" N%u=%u".as_ptr().cast(),
                nr,
                *counters.add(nr as usize),
            );
        }
        nr = next_high_memory_node(nr as i32);
    }
}
#[cfg(CONFIG_PROC_FS)]
unsafe fn show_purge_info(m: *mut seq_file) {
    for i in 0..nr_vmap_nodes {
        let vn = vmap_nodes.add(i as usize);
        let head = &raw mut (*vn).lazy.head;
        spin_lock(&raw mut (*vn).lazy.lock);
        let mut cursor = (*head).next;
        while cursor != head {
            let va = va_from_list(cursor);
            seq_printf(
                m,
                c"0x%pK-0x%pK %7ld unpurged vm_area\n".as_ptr().cast(),
                (*va).va_start as *mut Void,
                (*va).va_end as *mut Void,
                va_size(va),
            );
            cursor = (*cursor).next;
        }
        spin_unlock(&raw mut (*vn).lazy.lock);
    }
}
#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn vmalloc_info_show(m: *mut seq_file, _: *mut Void) -> i32 {
    let counters = if cfg!(CONFIG_NUMA) {
        kmalloc_array(node_id_count() as usize, size_of::<u32>(), GFP_KERNEL).cast::<u32>()
    } else {
        null_mut()
    };
    for i in 0..nr_vmap_nodes {
        let vn = vmap_nodes.add(i as usize);
        let head = &raw mut (*vn).busy.head;
        spin_lock(&raw mut (*vn).busy.lock);
        let mut cursor = (*head).next;
        while cursor != head {
            let va = va_from_list(cursor);
            let v = (*va).__bindgen_anon_1.vm;
            cursor = (*cursor).next;
            if v.is_null() {
                if (*va).flags & VMAP_RAM != 0 {
                    seq_printf(
                        m,
                        c"0x%pK-0x%pK %7ld vm_map_ram\n".as_ptr().cast(),
                        (*va).va_start as *mut Void,
                        (*va).va_end as *mut Void,
                        va_size(va),
                    );
                }
                continue;
            }
            if (*v).flags & VM_UNINITIALIZED as ULong != 0 {
                continue;
            }
            smp_rmb();
            seq_printf(
                m,
                c"0x%pK-0x%pK %7ld".as_ptr().cast(),
                (*v).addr,
                (*v).addr.wrapping_byte_add((*v).size as usize),
                (*v).size,
            );
            if !(*v).caller.is_null() {
                seq_printf(m, c" %pS".as_ptr().cast(), (*v).caller);
            }
            if (*v).nr_pages != 0 {
                seq_printf(m, c" pages=%lu".as_ptr().cast(), (*v).nr_pages);
            }
            if (*v).phys_addr != 0 {
                seq_printf(m, c" phys=%pa".as_ptr().cast(), &raw const (*v).phys_addr);
            }
            if (*v).flags & VM_IOREMAP as ULong != 0 {
                seq_puts(m, c" ioremap".as_ptr().cast());
            }
            if (*v).flags & VM_SPARSE as ULong != 0 {
                seq_puts(m, c" sparse".as_ptr().cast());
            }
            if (*v).flags & VM_ALLOC as ULong != 0 {
                seq_puts(m, c" vmalloc".as_ptr().cast());
            }
            if (*v).flags & VM_MAP as ULong != 0 {
                seq_puts(m, c" vmap".as_ptr().cast());
            }
            if (*v).flags & VM_USERMAP as ULong != 0 {
                seq_puts(m, c" user".as_ptr().cast());
            }
            if (*v).flags & VM_DMA_COHERENT as ULong != 0 {
                seq_puts(m, c" dma-coherent".as_ptr().cast());
            }
            if is_vmalloc_addr((*v).pages.cast()) {
                seq_puts(m, c" vpages".as_ptr().cast());
            }
            if cfg!(CONFIG_NUMA) {
                show_numa_info(m, v, counters);
            }
            seq_putc(m, b'\n' as Char);
        }
        spin_unlock(&raw mut (*vn).busy.lock);
    }
    show_purge_info(m);
    if cfg!(CONFIG_NUMA) {
        kfree(counters.cast());
    }
    0
}
#[cfg(CONFIG_PROC_FS)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_vmalloc_proc_vmalloc_init() -> i32 {
    proc_create_single(
        c"vmallocinfo".as_ptr().cast(),
        0o400,
        null_mut(),
        Some(vmalloc_info_show),
    );
    0
}
#[link_section = ".init.text"]
unsafe fn vmap_init_free_space() {
    let mut start: ULong = 1;
    let end = ULong::MAX;
    let mut busy = vmlist;
    while !busy.is_null() {
        if ((*busy).addr as ULong).wrapping_sub(start) > 0 {
            let free = kmem_cache_zalloc(vmap_area_cachep, GFP_NOWAIT).cast::<vmap_area>();
            if !warn_init_gap(free.is_null()) {
                (*free).va_start = start;
                (*free).va_end = (*busy).addr as ULong;
                insert_vmap_area_augment(
                    free,
                    null_mut(),
                    &raw mut free_vmap_area_root,
                    &raw mut free_vmap_area_list,
                );
            }
        }
        start = ((*busy).addr as ULong).wrapping_add((*busy).size);
        busy = (*busy).__bindgen_anon_1.next;
    }
    if end.wrapping_sub(start) > 0 {
        let free = kmem_cache_zalloc(vmap_area_cachep, GFP_NOWAIT).cast::<vmap_area>();
        if !warn_init_end(free.is_null()) {
            (*free).va_start = start;
            (*free).va_end = end;
            insert_vmap_area_augment(
                free,
                null_mut(),
                &raw mut free_vmap_area_root,
                &raw mut free_vmap_area_list,
            );
        }
    }
}
unsafe fn vmap_init_nodes() {
    #[cfg(CONFIG_64BIT)]
    {
        let n = num_possible_cpus().clamp(1, 128);
        if n > 1 {
            let vn =
                kmalloc_array(n as usize, size_of::<vmap_node>(), GFP_NOWAIT).cast::<vmap_node>();
            if !vn.is_null() {
                vmap_zone_size = (16 * PAGE_SIZE) as u32;
                nr_vmap_nodes = n;
                vmap_nodes = vn;
            } else {
                rust_vmalloc_warn_nodes();
            }
        }
    }
    for i in 0..nr_vmap_nodes {
        let vn = vmap_nodes.add(i as usize);
        (*vn).busy.root = rb_root {
            rb_node: null_mut(),
        };
        init_list_head(&raw mut (*vn).busy.head);
        rust_vmalloc_busy_lock_init(vn);
        (*vn).lazy.root = rb_root {
            rb_node: null_mut(),
        };
        init_list_head(&raw mut (*vn).lazy.head);
        rust_vmalloc_lazy_lock_init(vn);
        for j in 0..MAX_VA_SIZE_PAGES {
            let vp = (&raw mut (*vn).pool).cast::<vmap_pool>().add(j);
            init_list_head(&raw mut (*vp).head);
            write_ulong(&raw mut (*vp).len, 0);
        }
        rust_vmalloc_pool_lock_init(vn);
    }
}
unsafe extern "C" fn vmap_node_shrink_count(_: *mut shrinker, _: *mut shrink_control) -> ULong {
    let mut count: ULong = 0;
    for i in 0..nr_vmap_nodes {
        let vn = vmap_nodes.add(i as usize);
        for j in 0..MAX_VA_SIZE_PAGES {
            count = count.wrapping_add(read_ulong(&raw const (*vn).pool[j].len));
        }
    }
    if count != 0 {
        count
    } else {
        SHRINK_EMPTY as ULong
    }
}
unsafe extern "C" fn vmap_node_shrink_scan(_: *mut shrinker, _: *mut shrink_control) -> ULong {
    rust_vmalloc_purge_lock();
    for i in 0..nr_vmap_nodes {
        decay_va_pool_node(vmap_nodes.add(i as usize), true);
    }
    rust_vmalloc_purge_unlock();
    SHRINK_STOP as ULong
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn vmalloc_init() {
    vmap_area_cachep = rust_vmalloc_create_area_cache();
    let mut i = cpumask_next(-1, possible_mask());
    while (i as u32) < nr_cpu_ids {
        let vbq = rust_vmalloc_vbq(i);
        rust_vmalloc_vbq_lock_init(vbq);
        init_list_head(&raw mut (*vbq).free);
        let p = rust_vmalloc_deferred_cpu(i);
        init_llist_head(&raw mut (*p).list);
        rust_vmalloc_init_deferred_work(p);
        xa_init(&raw mut (*vbq).vmap_blocks);
        i = cpumask_next(i, possible_mask());
    }
    vmap_init_nodes();
    let mut tmp = vmlist;
    while !tmp.is_null() {
        let va = kmem_cache_zalloc(vmap_area_cachep, GFP_NOWAIT).cast::<vmap_area>();
        if !warn_init_import(va.is_null()) {
            (*va).va_start = (*tmp).addr as ULong;
            (*va).va_end = (*va).va_start.wrapping_add((*tmp).size);
            (*va).__bindgen_anon_1.vm = tmp;
            let vn = addr_to_node((*va).va_start);
            insert_vmap_area(va, &raw mut (*vn).busy.root, &raw mut (*vn).busy.head);
        }
        tmp = (*tmp).__bindgen_anon_1.next;
    }
    vmap_init_free_space();
    vmap_initialized = true;
    let shrink = shrinker_alloc(0, c"vmap-node".as_ptr().cast());
    if shrink.is_null() {
        rust_vmalloc_warn_shrinker();
        return;
    }
    (*shrink).count_objects = Some(vmap_node_shrink_count);
    (*shrink).scan_objects = Some(vmap_node_shrink_scan);
    shrinker_register(shrink);
}
