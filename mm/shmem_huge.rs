// SPDX-License-Identifier: GPL-2.0-only
// C:605-879,1818-1923. Tunable storage has native read-mostly placement.
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
unsafe fn shmem_get_orders_within_size(
    inode: *mut inode,
    mut orders: c_ulong,
    index: Pgoff,
    end: loff_t,
) -> c_uint {
    let mut order = highest_order(orders);
    while orders != 0 {
        let aligned = round_up(index.wrapping_add(1), 1 << order);
        let page_count = vm_acct(core::cmp::max(end, i_size_read(inode)));
        let fits = if size_of::<c_ulong>() == size_of::<loff_t>() {
            page_count as u64 >= aligned as u64
        } else {
            page_count >= aligned as loff_t
        };
        if fits {
            return orders as c_uint;
        }
        order = next_order(&mut orders, order);
    }
    0
}
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
unsafe fn shmem_huge_global_enabled(
    inode: *mut inode,
    index: Pgoff,
    end: loff_t,
    force: bool,
    _vma: *mut vm_area_struct,
    flags: vm_flags_t,
) -> c_uint {
    let pmd = if RUST_SHMEM_HPAGE_PMD_ORDER > RUST_SHMEM_MAX_PAGECACHE_ORDER {
        0
    } else {
        1 << RUST_SHMEM_HPAGE_PMD_ORDER
    };
    if !rust_shmem_isreg((*inode).i_mode)
        || rust_shmem_data_shmem_huge == RUST_SHMEM_SHMEM_HUGE_DENY
    {
        return 0;
    }
    if force || rust_shmem_data_shmem_huge == RUST_SHMEM_SHMEM_HUGE_FORCE {
        return pmd;
    }
    match (*SHMEM_SB((*inode).i_sb)).huge as c_int {
        RUST_SHMEM_SHMEM_HUGE_ALWAYS => RUST_SHMEM_THP_ORDERS_ALL_FILE_DEFAULT as c_uint,
        RUST_SHMEM_SHMEM_HUGE_WITHIN_SIZE => {
            let orders = shmem_get_orders_within_size(
                inode,
                RUST_SHMEM_THP_ORDERS_ALL_FILE_DEFAULT as c_ulong,
                index,
                end,
            );
            if orders != 0 {
                orders
            } else if flags & RUST_SHMEM_VM_HUGEPAGE as vm_flags_t != 0 {
                RUST_SHMEM_THP_ORDERS_ALL_FILE_DEFAULT as c_uint
            } else {
                0
            }
        }
        RUST_SHMEM_SHMEM_HUGE_ADVISE if flags & RUST_SHMEM_VM_HUGEPAGE as vm_flags_t != 0 => {
            RUST_SHMEM_THP_ORDERS_ALL_FILE_DEFAULT as c_uint
        }
        _ => 0,
    }
}
#[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
unsafe fn shmem_huge_global_enabled(
    _inode: *mut inode,
    _index: Pgoff,
    _end: loff_t,
    _force: bool,
    _vma: *mut vm_area_struct,
    _flags: vm_flags_t,
) -> c_uint {
    0
}
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
unsafe fn shmem_parse_huge(s: *const c_char) -> c_int {
    if s.is_null() {
        return -(RUST_SHMEM_EINVAL as c_int);
    }
    let mut huge = -(RUST_SHMEM_EINVAL as c_int);
    for (text, value) in [
        (c"never", RUST_SHMEM_SHMEM_HUGE_NEVER),
        (c"always", RUST_SHMEM_SHMEM_HUGE_ALWAYS),
        (c"within_size", RUST_SHMEM_SHMEM_HUGE_WITHIN_SIZE),
        (c"advise", RUST_SHMEM_SHMEM_HUGE_ADVISE),
        (c"deny", RUST_SHMEM_SHMEM_HUGE_DENY),
        (c"force", RUST_SHMEM_SHMEM_HUGE_FORCE),
    ] {
        if strcmp(s, text.as_char_ptr()) == 0 {
            huge = value;
            break;
        }
    }
    if huge == -(RUST_SHMEM_EINVAL as c_int) {
        return huge;
    }
    if !rust_shmem_has_transparent_hugepage()
        && huge != RUST_SHMEM_SHMEM_HUGE_NEVER
        && huge != RUST_SHMEM_SHMEM_HUGE_DENY
    {
        return -(RUST_SHMEM_EINVAL as c_int);
    }
    if huge == RUST_SHMEM_SHMEM_HUGE_FORCE
        && rust_shmem_data_huge_shmem_orders_inherit != 1 << RUST_SHMEM_HPAGE_PMD_ORDER
    {
        return -(RUST_SHMEM_EINVAL as c_int);
    }
    huge
}
#[cfg(all(CONFIG_TRANSPARENT_HUGEPAGE, any(CONFIG_SYSFS, CONFIG_TMPFS)))]
unsafe fn shmem_format_huge(huge: c_int) -> *const c_char {
    match huge {
        RUST_SHMEM_SHMEM_HUGE_NEVER => c"never".as_char_ptr(),
        RUST_SHMEM_SHMEM_HUGE_ALWAYS => c"always".as_char_ptr(),
        RUST_SHMEM_SHMEM_HUGE_WITHIN_SIZE => c"within_size".as_char_ptr(),
        RUST_SHMEM_SHMEM_HUGE_ADVISE => c"advise".as_char_ptr(),
        RUST_SHMEM_SHMEM_HUGE_DENY => c"deny".as_char_ptr(),
        RUST_SHMEM_SHMEM_HUGE_FORCE => c"force".as_char_ptr(),
        _ => {
            vm_bug!(true);
            c"bad_val".as_char_ptr()
        }
    }
}
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
unsafe fn shmem_unused_huge_shrink(
    sb: *mut shmem_sb_info,
    sc: *mut shrink_control,
    limit: c_ulong,
) -> c_ulong {
    let mut local = MaybeUninit::<list_head>::uninit();
    let list = local.as_mut_ptr();
    INIT_LIST_HEAD(list);
    let head = addr_of_mut!((*sb).shrinklist);
    let mut batch = if sc.is_null() { 128 } else { (*sc).nr_to_scan };
    let mut split: c_ulong = 0;
    let mut freed: c_ulong = 0;
    if rust_shmem_list_empty(head) {
        return RUST_SHMEM_SHRINK_STOP as c_ulong;
    }
    spin_lock(addr_of_mut!((*sb).shrinklist_lock));
    let mut pos = (*head).next;
    while pos != head {
        let next = (*pos).next;
        let info = rust_shmem_shrinklist_inode(pos);
        if igrab(addr_of_mut!((*info).vfs_inode)).is_null() {
            list_del_init(pos);
        } else {
            list_move(pos, list);
        }
        (*sb).shrinklist_len = (*sb).shrinklist_len.wrapping_sub(1);
        batch = batch.wrapping_sub(1);
        if batch == 0 {
            break;
        }
        pos = next;
    }
    spin_unlock(addr_of_mut!((*sb).shrinklist_lock));
    pos = (*list).next;
    while pos != list {
        let next_node = (*pos).next;
        let info = rust_shmem_shrinklist_inode(pos);
        let inode = addr_of_mut!((*info).vfs_inode);
        let mut move_back = limit != 0 && freed >= limit;
        if !move_back {
            let size = i_size_read(inode);
            let f = filemap_get_entry(
                (*inode).i_mapping,
                (size as u64 / RUST_SHMEM_PAGE_SIZE as u64) as Pgoff,
            );
            if !f.is_null() && !xa_is_value(f.cast()) {
                if folio_test_large(f) {
                    let next = folio_next_index(f);
                    let end = shmem_fallocend(
                        inode,
                        (size as u64)
                            .wrapping_add(RUST_SHMEM_PAGE_SIZE as u64 - 1)
                            .wrapping_shr(RUST_SHMEM_PAGE_SHIFT as u32)
                            as Pgoff,
                    );
                    if end > (*rust_shmem_folio_index_ptr(f)) && end < next {
                        if !folio_trylock(f) {
                            move_back = true;
                        } else {
                            let ret = rust_shmem_split_folio(f);
                            folio_unlock(f);
                            if ret != 0 {
                                move_back = true;
                            } else {
                                freed = freed.wrapping_add(next.wrapping_sub(end));
                                split = split.wrapping_add(1);
                            }
                        }
                    }
                }
                folio_put(f);
            }
        }
        if move_back {
            spin_lock(addr_of_mut!((*sb).shrinklist_lock));
            list_move(pos, head);
            (*sb).shrinklist_len = (*sb).shrinklist_len.wrapping_add(1);
            spin_unlock(addr_of_mut!((*sb).shrinklist_lock));
        } else {
            list_del_init(pos);
        }
        iput(inode);
        pos = next_node;
    }
    split
}
#[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
unsafe fn shmem_unused_huge_shrink(
    _sb: *mut shmem_sb_info,
    _sc: *mut shrink_control,
    _limit: c_ulong,
) -> c_ulong {
    0
}
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[export_name = "rust_shmem_owner_shmem_unused_huge_scan"]
unsafe extern "C" fn shmem_unused_huge_scan(
    sb: *mut super_block,
    sc: *mut shrink_control,
) -> c_long {
    let info = SHMEM_SB(sb);
    if rust_shmem_read_shrinklist_len(info) == 0 {
        return RUST_SHMEM_SHRINK_STOP as c_long;
    }
    shmem_unused_huge_shrink(info, sc, 0) as c_long
}
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[export_name = "rust_shmem_owner_shmem_unused_huge_count"]
unsafe extern "C" fn shmem_unused_huge_count(
    sb: *mut super_block,
    sc: *mut shrink_control,
) -> c_long {
    if !mem_cgroup_shrink_is_root(sc) {
        return 0;
    }
    rust_shmem_read_shrinklist_len(SHMEM_SB(sb)) as c_long
}
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[no_mangle]
pub unsafe extern "C" fn shmem_hpage_pmd_enabled() -> bool {
    if rust_shmem_data_shmem_huge == RUST_SHMEM_SHMEM_HUGE_DENY {
        return false;
    }
    let bit = RUST_SHMEM_HPAGE_PMD_ORDER as c_ulong;
    rust_shmem_test_bit(bit, addr_of!(rust_shmem_data_huge_shmem_orders_always))
        || rust_shmem_test_bit(bit, addr_of!(rust_shmem_data_huge_shmem_orders_madvise))
        || rust_shmem_test_bit(bit, addr_of!(rust_shmem_data_huge_shmem_orders_within_size))
        || (rust_shmem_test_bit(bit, addr_of!(rust_shmem_data_huge_shmem_orders_inherit))
            && rust_shmem_data_shmem_huge != RUST_SHMEM_SHMEM_HUGE_NEVER)
}
// Exact !THP header alternative from include/linux/shmem_fs.h:134-140.
// This private body has no exported owner symbol in the disabled alternative.
#[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
unsafe extern "C" fn shmem_allowable_huge_orders(
    _inode: *mut inode,
    _vma: *mut vm_area_struct,
    _index: Pgoff,
    _write_end: loff_t,
    _shmem_huge_force: bool,
) -> c_ulong {
    0
}
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
#[no_mangle]
pub unsafe extern "C" fn shmem_allowable_huge_orders(
    inode: *mut inode,
    vma: *mut vm_area_struct,
    index: Pgoff,
    end: loff_t,
    force: bool,
) -> c_ulong {
    let mut mask = rust_shmem_read_ulong(addr_of!(rust_shmem_data_huge_shmem_orders_always));
    let within = rust_shmem_read_ulong(addr_of!(rust_shmem_data_huge_shmem_orders_within_size));
    let flags = if vma.is_null() {
        0
    } else {
        rust_shmem_vma_vm_flags(vma)
    };
    if thp_disabled_by_hw() || (!vma.is_null() && vma_thp_disabled(vma, flags, force)) {
        return 0;
    }
    let global = shmem_huge_global_enabled(inode, index, end, force, vma, flags);
    if vma.is_null() || !vma_is_anon_shmem(vma) {
        return global as c_ulong;
    }
    if rust_shmem_data_shmem_huge == RUST_SHMEM_SHMEM_HUGE_DENY {
        return 0;
    }
    if rust_shmem_data_shmem_huge == RUST_SHMEM_SHMEM_HUGE_FORCE {
        return rust_shmem_read_ulong(addr_of!(rust_shmem_data_huge_shmem_orders_inherit));
    }
    mask |= shmem_get_orders_within_size(inode, within, index, 0) as c_ulong;
    if flags & RUST_SHMEM_VM_HUGEPAGE as vm_flags_t != 0 {
        mask |= rust_shmem_read_ulong(addr_of!(rust_shmem_data_huge_shmem_orders_madvise));
    }
    if global != 0 {
        mask |= rust_shmem_read_ulong(addr_of!(rust_shmem_data_huge_shmem_orders_inherit));
    }
    RUST_SHMEM_THP_ORDERS_ALL_FILE_DEFAULT as c_ulong & mask
}
#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
unsafe fn shmem_suitable_orders(
    _inode: *mut inode,
    vmf: *mut vm_fault,
    mapping: *mut address_space,
    index: Pgoff,
    mut orders: c_ulong,
) -> c_ulong {
    if !vmf.is_null() {
        orders =
            thp_vma_suitable_orders(rust_shmem_vmf_vma(vmf), rust_shmem_vmf_address(vmf), orders);
        if orders == 0 {
            return 0;
        }
    }
    let mut order = highest_order(orders);
    while orders != 0 {
        let pages = 1 << order;
        let mut aligned = round_down(index, pages);
        if xa_find(
            addr_of_mut!((*mapping).i_pages),
            &mut aligned,
            aligned.wrapping_add(pages).wrapping_sub(1),
            RUST_SHMEM_XA_PRESENT,
        )
        .is_null()
        {
            break;
        }
        order = next_order(&mut orders, order);
    }
    orders
}
#[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
unsafe fn shmem_suitable_orders(
    _inode: *mut inode,
    _vmf: *mut vm_fault,
    _mapping: *mut address_space,
    _index: Pgoff,
    _orders: c_ulong,
) -> c_ulong {
    0
}
