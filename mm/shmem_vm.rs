// SPDX-License-Identifier: GPL-2.0-only
#[export_name = "rust_shmem_owner_synchronous_wake_function"]
unsafe extern "C" fn synchronous_wake_function(
    wait: *mut wait_queue_entry_t,
    mode: c_uint,
    sync: c_int,
    key: *mut c_void,
) -> c_int {
    let ret = default_wake_function(wait, mode, sync, key);
    list_del_init(addr_of_mut!((*wait).entry));
    ret
}
unsafe fn shmem_falloc_wait(vmf: *mut vm_fault, inode: *mut inode) -> vm_fault_t {
    let mut pin = null_mut();
    let mut ret = 0;
    spin_lock(addr_of_mut!((*inode).i_lock));
    let fa = (*inode).i_private as *mut shmem_falloc;
    if !fa.is_null()
        && !(*fa).waitq.is_null()
        && rust_shmem_vmf_pgoff(vmf) >= (*fa).start
        && rust_shmem_vmf_pgoff(vmf) < (*fa).next
    {
        let mut wait = MaybeUninit::<wait_queue_entry_t>::uninit();
        rust_shmem_init_wait(wait.as_mut_ptr(), Some(synchronous_wake_function));
        ret = VM_FAULT_NOPAGE as vm_fault_t;
        pin = maybe_unlock_mmap_for_io(vmf, null_mut());
        let queue = (*fa).waitq;
        prepare_to_wait(
            queue,
            wait.as_mut_ptr(),
            RUST_SHMEM_TASK_UNINTERRUPTIBLE as c_int,
        );
        spin_unlock(addr_of_mut!((*inode).i_lock));
        schedule();
        spin_lock(addr_of_mut!((*inode).i_lock));
        finish_wait(queue, wait.as_mut_ptr());
    }
    spin_unlock(addr_of_mut!((*inode).i_lock));
    if !pin.is_null() {
        fput(pin);
        ret = VM_FAULT_RETRY as vm_fault_t;
    }
    ret
}
#[export_name = "rust_shmem_owner_shmem_fault"]
unsafe extern "C" fn shmem_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let inode = file_inode((*rust_shmem_vmf_vma(vmf)).vm_file);
    let mut ret = 0;
    let mut f = null_mut();
    if !rust_shmem_read_private(inode).is_null() {
        ret = shmem_falloc_wait(vmf, inode);
        if ret != 0 {
            return ret;
        }
    }
    rust_shmem_warn_fault_page(!(*vmf).page.is_null());
    let err = shmem_get_folio_gfp(
        inode,
        rust_shmem_vmf_pgoff(vmf),
        0,
        &mut f,
        SGP_CACHE,
        mapping_gfp_mask((*inode).i_mapping),
        vmf,
        &mut ret,
    );
    if err != 0 {
        return vmf_error(err);
    }
    if !f.is_null() {
        (*vmf).page = folio_file_page(f, rust_shmem_vmf_pgoff(vmf));
        ret |= VM_FAULT_LOCKED as vm_fault_t;
    }
    ret
}
#[no_mangle]
pub unsafe extern "C" fn shmem_get_unmapped_area(
    file: *mut file,
    uaddr: c_ulong,
    len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
) -> c_ulong {
    let task_size = rust_shmem_task_size();
    if len > task_size {
        return (-(RUST_SHMEM_ENOMEM as c_long)) as c_ulong;
    }
    let addr = mm_get_unmapped_area(file, uaddr, len, pgoff, flags);
    #[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
    {
        return addr;
    }
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    {
        if rust_shmem_IS_ERR_VALUE(addr)
            || addr & !(RUST_SHMEM_PAGE_MASK as c_ulong) != 0
            || addr > task_size - len
            || rust_shmem_data_shmem_huge == RUST_SHMEM_SHMEM_HUGE_DENY
            || flags & RUST_SHMEM_MAP_FIXED as c_ulong != 0
            || uaddr == addr
        {
            return addr;
        }
        let mut hsize = RUST_SHMEM_HPAGE_PMD_SIZE as c_ulong;
        if rust_shmem_data_shmem_huge != RUST_SHMEM_SHMEM_HUGE_FORCE {
            let sb;
            let mut order = 0;
            if !file.is_null() {
                vm_bug!((*file).f_op != addr_of!(rust_shmem_data_shmem_file_operations));
                sb = (*file_inode(file)).i_sb;
            } else {
                if IS_ERR(rust_shmem_data_shm_mnt.cast()) {
                    return addr;
                }
                sb = (*rust_shmem_data_shm_mnt).mnt_sb;
                let mut orders =
                    rust_shmem_read_ulong(addr_of!(rust_shmem_data_huge_shmem_orders_always))
                        | rust_shmem_read_ulong(addr_of!(
                            rust_shmem_data_huge_shmem_orders_within_size
                        ))
                        | rust_shmem_read_ulong(addr_of!(
                            rust_shmem_data_huge_shmem_orders_madvise
                        ));
                if (*SHMEM_SB(sb)).huge as c_int != RUST_SHMEM_SHMEM_HUGE_NEVER {
                    orders |=
                        rust_shmem_read_ulong(addr_of!(rust_shmem_data_huge_shmem_orders_inherit));
                }
                if orders != 0 {
                    order = highest_order(orders);
                    hsize = (RUST_SHMEM_PAGE_SIZE as c_ulong) << order;
                }
            }
            if (*SHMEM_SB(sb)).huge as c_int == RUST_SHMEM_SHMEM_HUGE_NEVER && order == 0 {
                return addr;
            }
        }
        if len < hsize {
            return addr;
        }
        let offset = (pgoff << RUST_SHMEM_PAGE_SHIFT) & (hsize - 1);
        if offset != 0 && offset.wrapping_add(len) < hsize.wrapping_mul(2) {
            return addr;
        }
        if addr & (hsize - 1) == offset {
            return addr;
        }
        let inflated_len = len
            .wrapping_add(hsize)
            .wrapping_sub(RUST_SHMEM_PAGE_SIZE as c_ulong);
        if inflated_len > task_size || inflated_len < len {
            return addr;
        }
        let mut inflated = mm_get_unmapped_area(null_mut(), uaddr, inflated_len, 0, flags);
        if rust_shmem_IS_ERR_VALUE(inflated) || inflated & !(RUST_SHMEM_PAGE_MASK as c_ulong) != 0 {
            return addr;
        }
        let off = inflated & (hsize - 1);
        inflated = inflated.wrapping_add(offset.wrapping_sub(off));
        if off > offset {
            inflated = inflated.wrapping_add(hsize);
        }
        if inflated > task_size - len {
            addr
        } else {
            inflated
        }
    }
}
#[cfg(CONFIG_NUMA)]
#[export_name = "rust_shmem_owner_shmem_set_policy"]
unsafe extern "C" fn shmem_set_policy(vma: *mut vm_area_struct, p: *mut mempolicy) -> c_int {
    mpol_set_shared_policy(
        addr_of_mut!((*SHMEM_I(file_inode((*vma).vm_file))).policy),
        vma,
        p,
    )
}
#[cfg(CONFIG_NUMA)]
#[export_name = "rust_shmem_owner_shmem_get_policy"]
unsafe extern "C" fn shmem_get_policy(
    vma: *mut vm_area_struct,
    addr: c_ulong,
    ilx: *mut Pgoff,
) -> *mut mempolicy {
    let inode = file_inode((*vma).vm_file);
    *ilx = (*inode).i_ino as Pgoff;
    mpol_shared_policy_lookup(
        addr_of_mut!((*SHMEM_I(inode)).policy),
        linear_page_index(vma, addr),
    )
}
unsafe fn shmem_get_pgoff_policy(
    info: *mut shmem_inode_info,
    index: Pgoff,
    order: c_uint,
    ilx: *mut Pgoff,
) -> *mut mempolicy {
    #[cfg(CONFIG_NUMA)]
    {
        *ilx = ((*info).vfs_inode.i_ino as Pgoff).wrapping_add(index >> order);
        let p = mpol_shared_policy_lookup(addr_of_mut!((*info).policy), index);
        if p.is_null() {
            rust_shmem_current_policy()
        } else {
            p
        }
    }
    #[cfg(not(CONFIG_NUMA))]
    {
        let _ = (info, index, order);
        *ilx = 0;
        null_mut()
    }
}
#[no_mangle]
pub unsafe extern "C" fn shmem_lock(file: *mut file, lock: c_int, ucounts: *mut ucounts) -> c_int {
    let inode = file_inode(file);
    let info = SHMEM_I(inode);
    if lock != 0 && (*info).flags & RUST_SHMEM_SHMEM_F_LOCKED as c_ulong == 0 {
        if !rust_shmem_user_shm_lock((*inode).i_size, ucounts) {
            return -(RUST_SHMEM_ENOMEM as c_int);
        }
        (*info).flags |= RUST_SHMEM_SHMEM_F_LOCKED as c_ulong;
        mapping_set_unevictable((*file).f_mapping);
    }
    if lock == 0 && (*info).flags & RUST_SHMEM_SHMEM_F_LOCKED as c_ulong != 0 && !ucounts.is_null()
    {
        user_shm_unlock((*inode).i_size as usize, ucounts);
        (*info).flags &= !(RUST_SHMEM_SHMEM_F_LOCKED as c_ulong);
        mapping_clear_unevictable((*file).f_mapping);
    }
    0
}
#[export_name = "rust_shmem_owner_shmem_mmap_prepare"]
unsafe extern "C" fn shmem_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    let file = (*desc).file;
    let inode = file_inode(file);
    file_accessed(file);
    (*desc).vm_ops = if rust_shmem_inode_nlink(inode) != 0 {
        addr_of!(rust_shmem_data_shmem_vm_ops)
    } else {
        addr_of!(rust_shmem_data_shmem_anon_vm_ops)
    };
    0
}
#[export_name = "rust_shmem_owner_shmem_file_open"]
unsafe extern "C" fn shmem_file_open(inode: *mut inode, file: *mut file) -> c_int {
    (*file).f_mode |= RUST_SHMEM_FMODE_CAN_ODIRECT as fmode_t;
    generic_file_open(inode, file)
}
#[cfg(CONFIG_USERFAULTFD)]
#[export_name = "rust_shmem_owner_shmem_mfill_folio_alloc"]
unsafe extern "C" fn shmem_mfill_folio_alloc(
    vma: *mut vm_area_struct,
    addr: c_ulong,
) -> *mut folio {
    let inode = file_inode((*vma).vm_file);
    let index = linear_page_index(vma, addr);
    if index as u64
        >= (i_size_read(inode) as u64).wrapping_add(RUST_SHMEM_PAGE_SIZE as u64 - 1)
            / RUST_SHMEM_PAGE_SIZE as u64
    {
        return null_mut();
    }
    let f = shmem_alloc_folio(
        mapping_gfp_mask((*inode).i_mapping),
        0,
        SHMEM_I(inode),
        index,
    );
    if f.is_null() {
        return f;
    }
    if mem_cgroup_charge(f, (*vma).vm_mm, RUST_SHMEM_GFP_KERNEL as gfp_t) != 0 {
        folio_put(f);
        return null_mut();
    }
    f
}
#[cfg(CONFIG_USERFAULTFD)]
#[export_name = "rust_shmem_owner_shmem_mfill_filemap_add"]
unsafe extern "C" fn shmem_mfill_filemap_add(
    f: *mut folio,
    vma: *mut vm_area_struct,
    addr: c_ulong,
) -> c_int {
    let inode = file_inode((*vma).vm_file);
    let mapping = (*inode).i_mapping;
    __folio_set_locked(f);
    __folio_set_swapbacked(f);
    let mut err = shmem_add_to_page_cache(
        f,
        mapping,
        linear_page_index(vma, addr),
        null_mut(),
        mapping_gfp_mask(mapping),
    );
    if err == 0 {
        if shmem_inode_acct_blocks(inode, 1) != 0 {
            err = -(RUST_SHMEM_ENOMEM as c_int);
            filemap_remove_folio(f);
        } else {
            folio_add_lru(f);
            shmem_recalc_inode(inode, 1, 0);
            return 0;
        }
    }
    folio_unlock(f);
    err
}
#[cfg(CONFIG_USERFAULTFD)]
#[export_name = "rust_shmem_owner_shmem_mfill_filemap_remove"]
unsafe extern "C" fn shmem_mfill_filemap_remove(f: *mut folio, vma: *mut vm_area_struct) {
    filemap_remove_folio(f);
    shmem_recalc_inode(file_inode((*vma).vm_file), 0, 0);
    folio_unlock(f);
}
#[cfg(CONFIG_USERFAULTFD)]
#[export_name = "rust_shmem_owner_shmem_get_folio_noalloc"]
unsafe extern "C" fn shmem_get_folio_noalloc(inode: *mut inode, index: Pgoff) -> *mut folio {
    let mut f = null_mut();
    let err = shmem_get_folio(inode, index, 0, &mut f, SGP_NOALLOC);
    if err != 0 {
        err_ptr(err)
    } else {
        f
    }
}
#[cfg(CONFIG_USERFAULTFD)]
#[export_name = "rust_shmem_owner_shmem_can_userfault"]
unsafe extern "C" fn shmem_can_userfault(_vma: *mut vm_area_struct, _flags: vm_flags_t) -> bool {
    true
}
