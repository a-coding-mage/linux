// SPDX-License-Identifier: GPL-2.0
// Included only under CONFIG_NUMA_MIGRATION. User access stays at native typed
// uaccess leaves; all validation, batching, accounting and lifetime live here.
unsafe fn store_status(status: *mut c_int, mut start: c_int, value: c_int, mut nr: c_int) -> c_int {
    loop {
        let previous = nr;
        nr = nr.wrapping_sub(1);
        if previous <= 0 {
            return 0;
        }
        if put_user_int(value, status.wrapping_offset(start as isize)) != 0 {
            return E_FAULT;
        }
        start = start.wrapping_add(1);
    }
}

unsafe fn do_move_pages_to_node(pagelist: *mut list_head, node: c_int) -> c_int {
    let mut mtc: migration_target_control = zeroed();
    mtc.nid = node;
    mtc.gfp_mask = (RUST_MIGRATE_GFP_HIGHUSER_MOVABLE | RUST_MIGRATE___GFP_THISNODE) as _;
    mtc.reason = MR_SYSCALL;
    let err = migrate_pages(
        pagelist,
        Some(alloc_migration_target),
        None,
        addr_of_mut!(mtc) as c_ulong,
        MIGRATE_SYNC,
        MR_SYSCALL,
        null_mut(),
    );
    if err != 0 {
        putback_movable_pages(pagelist);
    }
    err
}

unsafe fn __add_folio_for_migration(
    folio: *mut folio,
    node: c_int,
    pagelist: *mut list_head,
    migrate_all: bool,
) -> c_int {
    if is_zero_folio(folio) || is_huge_zero_folio(folio) {
        return E_FAULT;
    }
    if folio_is_zone_device(folio) {
        return E_NOENT;
    }
    if folio_nid(folio) == node {
        return 0;
    }
    if folio_maybe_mapped_shared(folio) && !migrate_all {
        return E_ACCES;
    }
    if folio_test_hugetlb(folio) {
        if folio_isolate_hugetlb(folio, pagelist) {
            return 1;
        }
    } else if folio_isolate_lru(folio) {
        list_add_tail(folio_lru(folio), pagelist);
        node_stat_mod_folio(folio, isolated_stat(folio), folio_nr_pages(folio) as c_long);
        return 1;
    }
    E_BUSY
}

unsafe fn add_folio_for_migration(
    mm: *mut mm_struct,
    p: *const c_void,
    node: c_int,
    pagelist: *mut list_head,
    migrate_all: bool,
) -> c_int {
    let mut err = E_FAULT;
    mmap_read_lock(mm);
    let addr = untagged_addr_remote(mm, p as c_ulong);
    let vma = vma_lookup(mm, addr);
    if !vma.is_null() && vma_migratable(vma) {
        let mut fw: folio_walk = zeroed();
        let folio = folio_walk_start(&mut fw, vma, addr, RUST_MIGRATE_FW_ZEROPAGE as _);
        if !folio.is_null() {
            err = __add_folio_for_migration(folio, node, pagelist, migrate_all);
            folio_walk_end(&mut fw, vma);
        } else {
            err = E_NOENT;
        }
    }
    mmap_read_unlock(mm);
    err
}

unsafe fn move_pages_and_store_status(
    node: c_int,
    pagelist: *mut list_head,
    status: *mut c_int,
    start: c_int,
    i: c_int,
    nr_pages: c_ulong,
) -> c_int {
    if list_empty(pagelist) {
        return 0;
    }
    let mut err = do_move_pages_to_node(pagelist, node);
    if err != 0 {
        if err > 0 {
            err = (err as c_ulong).wrapping_add(nr_pages.wrapping_sub(i as c_ulong)) as c_int;
        }
        return err;
    }
    store_status(status, start, node, i.wrapping_sub(start))
}

unsafe fn do_pages_move(
    mm: *mut mm_struct,
    task_nodes: nodemask_t,
    nr_pages: c_ulong,
    pages: *const *const c_void,
    nodes: *const c_int,
    status: *mut c_int,
    flags: c_int,
) -> c_int {
    let compat_pages = pages as *const compat_uptr_t;
    let mut current_node = RUST_MIGRATE_NUMA_NO_NODE as c_int;
    let mut pagelist: list_head = zeroed();
    init_list_head(&mut pagelist);
    let mut start: c_int = 0;
    let mut i: c_int = 0;
    let mut err: c_int = 0;
    lru_cache_disable();
    let flush = 'scan: {
        while (i as c_ulong) < nr_pages {
            let mut p: *const c_void = null();
            let mut node: c_int = 0;
            err = E_FAULT;
            if in_compat_syscall() {
                let mut cp: compat_uptr_t = 0;
                if get_user_compat(compat_pages.wrapping_offset(i as isize), &mut cp) != 0 {
                    break 'scan true;
                }
                p = compat_ptr(cp);
            } else if get_user_pointer(pages.wrapping_offset(i as isize), &mut p) != 0 {
                break 'scan true;
            }
            if get_user_int(nodes.wrapping_offset(i as isize), &mut node) != 0 {
                break 'scan true;
            }
            err = E_NODEV;
            if node < 0 || node >= RUST_MIGRATE_MAX_NUMNODES as c_int || !node_state(node, N_MEMORY)
            {
                break 'scan true;
            }
            err = E_ACCES;
            if !node_isset(node, &task_nodes) {
                break 'scan true;
            }
            if current_node == RUST_MIGRATE_NUMA_NO_NODE as c_int {
                current_node = node;
                start = i;
            } else if node != current_node {
                err = move_pages_and_store_status(
                    current_node,
                    &mut pagelist,
                    status,
                    start,
                    i,
                    nr_pages,
                );
                if err != 0 {
                    break 'scan false;
                }
                start = i;
                current_node = node;
            }
            err = add_folio_for_migration(
                mm,
                p,
                current_node,
                &mut pagelist,
                flags & RUST_MIGRATE_MPOL_MF_MOVE_ALL as c_int != 0,
            );
            if err > 0 {
                i = i.wrapping_add(1);
                continue;
            }
            err = store_status(status, i, if err != 0 { err } else { current_node }, 1);
            if err != 0 {
                break 'scan true;
            }
            err = move_pages_and_store_status(
                current_node,
                &mut pagelist,
                status,
                start,
                i,
                nr_pages,
            );
            if err != 0 {
                if err > 0 {
                    err = err.wrapping_sub(1);
                }
                break 'scan false;
            }
            current_node = RUST_MIGRATE_NUMA_NO_NODE as c_int;
            i = i.wrapping_add(1);
        }
        true
    };
    if flush {
        let err1 =
            move_pages_and_store_status(current_node, &mut pagelist, status, start, i, nr_pages);
        if err >= 0 {
            err = err1;
        }
    }
    lru_cache_enable();
    err
}

unsafe fn do_pages_stat_array(
    mm: *mut mm_struct,
    nr_pages: c_ulong,
    mut pages: *const *const c_void,
    mut status: *mut c_int,
) {
    mmap_read_lock(mm);
    let mut i: c_ulong = 0;
    while i < nr_pages {
        let addr = *pages as c_ulong;
        let vma = vma_lookup(mm, addr);
        let mut err = E_FAULT;
        if !vma.is_null() {
            let mut fw: folio_walk = zeroed();
            let folio = folio_walk_start(&mut fw, vma, addr, RUST_MIGRATE_FW_ZEROPAGE as _);
            if !folio.is_null() {
                err = if is_zero_folio(folio) || is_huge_zero_folio(folio) {
                    E_FAULT
                } else if folio_is_zone_device(folio) {
                    E_NOENT
                } else {
                    folio_nid(folio)
                };
                folio_walk_end(&mut fw, vma);
            } else {
                err = E_NOENT;
            }
        }
        *status = err;
        pages = pages.add(1);
        status = status.add(1);
        i = i.wrapping_add(1);
    }
    mmap_read_unlock(mm);
}

unsafe fn get_compat_pages_array(
    chunk_pages: *mut *const c_void,
    pages: *const *const c_void,
    chunk_offset: c_ulong,
    chunk_nr: c_ulong,
) -> c_int {
    let pages32 = pages as *const compat_uptr_t;
    let mut i: c_int = 0;
    while (i as c_ulong) < chunk_nr {
        let mut p: compat_uptr_t = 0;
        let index = chunk_offset.wrapping_add(i as c_ulong);
        if get_user_compat(pages32.wrapping_add(index as usize), &mut p) != 0 {
            return E_FAULT;
        }
        *chunk_pages.add(i as usize) = compat_ptr(p);
        i = i.wrapping_add(1);
    }
    0
}

unsafe fn do_pages_stat(
    mm: *mut mm_struct,
    mut nr_pages: c_ulong,
    pages: *const *const c_void,
    status: *mut c_int,
) -> c_int {
    const DO_PAGES_STAT_CHUNK_NR: usize = 16;
    let mut chunk_pages = [null(); DO_PAGES_STAT_CHUNK_NR];
    let mut chunk_status = [0; DO_PAGES_STAT_CHUNK_NR];
    let mut chunk_offset: c_ulong = 0;
    while nr_pages != 0 {
        let chunk_nr = core::cmp::min(nr_pages, DO_PAGES_STAT_CHUNK_NR as c_ulong);
        if in_compat_syscall() {
            if get_compat_pages_array(chunk_pages.as_mut_ptr(), pages, chunk_offset, chunk_nr) != 0
            {
                break;
            }
        } else if copy_from_user(
            chunk_pages.as_mut_ptr().cast(),
            pages.wrapping_add(chunk_offset as usize).cast(),
            chunk_nr.wrapping_mul(size_of::<*const c_void>() as c_ulong),
        ) != 0
        {
            break;
        }
        do_pages_stat_array(
            mm,
            chunk_nr,
            chunk_pages.as_ptr(),
            chunk_status.as_mut_ptr(),
        );
        if copy_to_user(
            status.wrapping_add(chunk_offset as usize).cast(),
            chunk_status.as_ptr().cast(),
            chunk_nr.wrapping_mul(size_of::<c_int>() as c_ulong),
        ) != 0
        {
            break;
        }
        chunk_offset = chunk_offset.wrapping_add(chunk_nr);
        nr_pages = nr_pages.wrapping_sub(chunk_nr);
    }
    if nr_pages != 0 {
        E_FAULT
    } else {
        0
    }
}

unsafe fn find_mm_struct(pid: pid_t, mem_nodes: *mut nodemask_t) -> *mut mm_struct {
    if pid == 0 {
        let current = current_task();
        mmget(task_mm(current));
        *mem_nodes = cpuset_mems_allowed(current);
        return task_mm(current);
    }
    let task = find_get_task_by_vpid(pid);
    if task.is_null() {
        return err_ptr(E_SRCH as c_long).cast();
    }
    let mm = if down_read_killable(task_exec_update_lock(task)) != 0 {
        err_ptr(E_INTR as c_long).cast()
    } else {
        let result = if !ptrace_may_access(task, RUST_MIGRATE_PTRACE_MODE_READ_REALCREDS as _) {
            err_ptr(E_PERM as c_long).cast()
        } else {
            let security_mm: *mut mm_struct =
                err_ptr(security_task_movememory(task) as c_long).cast();
            if is_err(security_mm.cast()) {
                security_mm
            } else {
                *mem_nodes = cpuset_mems_allowed(task);
                get_task_mm(task)
            }
        };
        up_read(task_exec_update_lock(task));
        result
    };
    put_task_struct(task);
    if mm.is_null() {
        err_ptr(E_INVAL as c_long).cast()
    } else {
        mm
    }
}

// C SYSCALL_DEFINE6 is retained solely as the native architecture/syscall-table
// metadata leaf; its only target is this Rust-owned original kernel body.
#[no_mangle]
pub unsafe extern "C" fn rust_migrate_kernel_move_pages(
    pid: pid_t,
    nr_pages: c_ulong,
    pages: *const *const c_void,
    nodes: *const c_int,
    status: *mut c_int,
    flags: c_int,
) -> c_int {
    if flags & !((RUST_MIGRATE_MPOL_MF_MOVE | RUST_MIGRATE_MPOL_MF_MOVE_ALL) as c_int) != 0 {
        return E_INVAL;
    }
    if flags & RUST_MIGRATE_MPOL_MF_MOVE_ALL as c_int != 0
        && !capable(RUST_MIGRATE_CAP_SYS_NICE as _)
    {
        return E_PERM;
    }
    let mut task_nodes: nodemask_t = zeroed();
    let mm = find_mm_struct(pid, &mut task_nodes);
    if is_err(mm.cast()) {
        return ptr_err(mm.cast()) as c_int;
    }
    let err = if !nodes.is_null() {
        do_pages_move(mm, task_nodes, nr_pages, pages, nodes, status, flags)
    } else {
        do_pages_stat(mm, nr_pages, pages, status)
    };
    mmput(mm);
    err
}
