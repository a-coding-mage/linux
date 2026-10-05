// SPDX-License-Identifier: GPL-2.0-only
// C:146-497. Locks and charge rollback remain with their original owners.
#[cfg(CONFIG_TMPFS)]
unsafe extern "C" fn shmem_default_max_blocks() -> c_ulong {
    totalram_pages() / 2
}
#[cfg(CONFIG_TMPFS)]
unsafe extern "C" fn shmem_default_max_inodes() -> c_ulong {
    let nr = totalram_pages();
    core::cmp::min(
        core::cmp::min(nr.wrapping_sub(totalhigh_pages()), nr / 2),
        c_ulong::MAX / RUST_SHMEM_BOGO_INODE_SIZE as c_ulong,
    )
}
unsafe fn shmem_acct_size(flags: c_ulong, size: loff_t) -> c_int {
    if flags & RUST_SHMEM_SHMEM_F_NORESERVE as c_ulong != 0 {
        0
    } else {
        security_vm_enough_memory_mm(rust_shmem_current_mm(), vm_acct(size) as c_long)
    }
}
unsafe fn shmem_unacct_size(flags: c_ulong, size: loff_t) {
    if flags & RUST_SHMEM_SHMEM_F_NORESERVE as c_ulong == 0 {
        vm_unacct_memory(vm_acct(size) as c_long);
    }
}
unsafe fn shmem_reacct_size(flags: c_ulong, oldsize: loff_t, newsize: loff_t) -> c_int {
    if flags & RUST_SHMEM_SHMEM_F_NORESERVE as c_ulong == 0 {
        let old = vm_acct(oldsize);
        let new = vm_acct(newsize);
        if new > old {
            return security_vm_enough_memory_mm(
                rust_shmem_current_mm(),
                new.wrapping_sub(old) as c_long,
            );
        }
        if new < old {
            vm_unacct_memory(old.wrapping_sub(new) as c_long);
        }
    }
    0
}
unsafe fn shmem_acct_blocks(flags: c_ulong, pages: c_long) -> c_int {
    if flags & RUST_SHMEM_SHMEM_F_NORESERVE as c_ulong == 0 {
        0
    } else {
        security_vm_enough_memory_mm(
            rust_shmem_current_mm(),
            pages.wrapping_mul(vm_acct(RUST_SHMEM_PAGE_SIZE as loff_t) as c_long),
        )
    }
}
unsafe fn shmem_unacct_blocks(flags: c_ulong, pages: c_long) {
    if flags & RUST_SHMEM_SHMEM_F_NORESERVE as c_ulong != 0 {
        vm_unacct_memory(pages.wrapping_mul(vm_acct(RUST_SHMEM_PAGE_SIZE as loff_t) as c_long));
    }
}
#[no_mangle]
pub unsafe extern "C" fn shmem_inode_acct_blocks(inode: *mut inode, pages: c_long) -> c_int {
    let info = SHMEM_I(inode);
    let sb = SHMEM_SB((*inode).i_sb);
    let mut err = -(RUST_SHMEM_ENOSPC as c_int);
    if shmem_acct_blocks((*info).flags, pages) != 0 {
        return err;
    }
    rust_shmem_might_sleep();
    if (*sb).max_blocks != 0 {
        if percpu_counter_limited_add(
            addr_of_mut!((*sb).used_blocks),
            (*sb).max_blocks as _,
            pages as _,
        ) {
            err = dquot_alloc_block_nodirty(inode, pages as _);
            if err == 0 {
                return 0;
            }
            percpu_counter_sub(addr_of_mut!((*sb).used_blocks), pages as _);
        }
    } else {
        err = dquot_alloc_block_nodirty(inode, pages as _);
        if err == 0 {
            return 0;
        }
    }
    shmem_unacct_blocks((*info).flags, pages);
    err
}
unsafe fn shmem_inode_unacct_blocks(inode: *mut inode, pages: c_long) {
    let info = SHMEM_I(inode);
    let sb = SHMEM_SB((*inode).i_sb);
    rust_shmem_might_sleep();
    dquot_free_block_nodirty(inode, pages as _);
    if (*sb).max_blocks != 0 {
        percpu_counter_sub(addr_of_mut!((*sb).used_blocks), pages as _);
    }
    shmem_unacct_blocks((*info).flags, pages);
}
#[no_mangle]
pub unsafe extern "C" fn shmem_mapping(mapping: *const address_space) -> bool {
    (*mapping).a_ops == addr_of!(rust_shmem_data_shmem_aops)
}
#[no_mangle]
pub unsafe extern "C" fn vma_is_anon_shmem(vma: *const vm_area_struct) -> bool {
    (*vma).vm_ops == addr_of!(rust_shmem_data_shmem_anon_vm_ops)
}
#[no_mangle]
pub unsafe extern "C" fn vma_is_shmem(vma: *const vm_area_struct) -> bool {
    vma_is_anon_shmem(vma) || (*vma).vm_ops == addr_of!(rust_shmem_data_shmem_vm_ops)
}
#[cfg(CONFIG_TMPFS_QUOTA)]
#[export_name = "rust_shmem_owner_shmem_enable_quotas"]
unsafe extern "C" fn shmem_enable_quotas(sb: *mut super_block, quota_types: u16) -> c_int {
    (*sb_dqopt(sb)).flags |= RUST_SHMEM_DQUOT_QUOTA_SYS_FILE | RUST_SHMEM_DQUOT_NOLIST_DIRTY;
    for ty in 0..RUST_SHMEM_SHMEM_MAXQUOTAS as c_int {
        if quota_types & (1u16 << ty) == 0 {
            continue;
        }
        let err = dquot_load_quota_sb(
            sb,
            ty,
            RUST_SHMEM_QFMT_SHMEM as _,
            (RUST_SHMEM_DQUOT_USAGE_ENABLED | RUST_SHMEM_DQUOT_LIMITS_ENABLED) as _,
        );
        if err != 0 {
            rust_shmem_quota_warning(ty, err);
            for previous in (0..ty).rev() {
                dquot_quota_off(sb, previous);
            }
            return err;
        }
    }
    0
}
#[cfg(CONFIG_TMPFS_QUOTA)]
unsafe fn shmem_disable_quotas(sb: *mut super_block) {
    for ty in 0..RUST_SHMEM_SHMEM_MAXQUOTAS as c_int {
        dquot_quota_off(sb, ty);
    }
}
#[cfg(CONFIG_TMPFS_QUOTA)]
#[export_name = "rust_shmem_owner_shmem_get_dquots"]
unsafe extern "C" fn shmem_get_dquots(inode: *mut inode) -> *mut *mut dquot {
    (*SHMEM_I(inode)).i_dquot.as_mut_ptr()
}
unsafe fn shmem_reserve_inode(sb: *mut super_block, inop: *mut ino_t) -> c_int {
    let info = SHMEM_SB(sb);
    if (*sb).s_flags & RUST_SHMEM_SB_KERNMOUNT as c_ulong == 0 {
        rust_shmem_raw_spin_lock(addr_of_mut!((*info).stat_lock));
        if (*info).max_inodes != 0 {
            if (*info).free_ispace < RUST_SHMEM_BOGO_INODE_SIZE as c_ulong {
                rust_shmem_raw_spin_unlock(addr_of_mut!((*info).stat_lock));
                return -(RUST_SHMEM_ENOSPC as c_int);
            }
            (*info).free_ispace = (*info)
                .free_ispace
                .wrapping_sub(RUST_SHMEM_BOGO_INODE_SIZE as c_ulong);
        }
        if !inop.is_null() {
            let mut ino = (*info).next_ino;
            (*info).next_ino = ino.wrapping_add(1);
            if is_zero_ino(ino) {
                ino = (*info).next_ino;
                (*info).next_ino = ino.wrapping_add(1);
            }
            if !(*info).full_inums && ino > c_uint::MAX as ino_t {
                #[cfg(CONFIG_64BIT)]
                rust_shmem_inode_overflow(sb);
                (*info).next_ino = 2;
                ino = 1;
            }
            *inop = ino;
        }
        rust_shmem_raw_spin_unlock(addr_of_mut!((*info).stat_lock));
    } else if !inop.is_null() {
        let next = rust_shmem_get_cpu_ino((*info).ino_batch);
        let mut ino = *next;
        if ino % RUST_SHMEM_SHMEM_INO_BATCH as ino_t == 0 {
            rust_shmem_raw_spin_lock(addr_of_mut!((*info).stat_lock));
            ino = (*info).next_ino;
            (*info).next_ino = (*info)
                .next_ino
                .wrapping_add(RUST_SHMEM_SHMEM_INO_BATCH as ino_t);
            rust_shmem_raw_spin_unlock(addr_of_mut!((*info).stat_lock));
            if is_zero_ino(ino) {
                ino = ino.wrapping_add(1);
            }
        }
        *inop = ino;
        *next = ino.wrapping_add(1);
        rust_shmem_put_cpu();
    }
    0
}
unsafe fn shmem_free_inode(sb: *mut super_block, freed_ispace: usize) {
    let info = SHMEM_SB(sb);
    if (*info).max_inodes != 0 {
        rust_shmem_raw_spin_lock(addr_of_mut!((*info).stat_lock));
        (*info).free_ispace = (*info)
            .free_ispace
            .wrapping_add(RUST_SHMEM_BOGO_INODE_SIZE as c_ulong)
            .wrapping_add(freed_ispace as c_ulong);
        rust_shmem_raw_spin_unlock(addr_of_mut!((*info).stat_lock));
    }
}
#[no_mangle]
pub unsafe extern "C" fn shmem_recalc_inode(
    inode: *mut inode,
    alloced: c_long,
    swapped: c_long,
) -> bool {
    let info = SHMEM_I(inode);
    let mut first = false;
    spin_lock(addr_of_mut!((*info).lock));
    (*info).alloced = (*info).alloced.wrapping_add(alloced as c_ulong);
    (*info).swapped = (*info).swapped.wrapping_add(swapped as c_ulong);
    let mut freed = (*info)
        .alloced
        .wrapping_sub((*info).swapped)
        .wrapping_sub(rust_shmem_mapping_nrpages((*inode).i_mapping)) as c_long;
    if swapped > 0 {
        first = (*info).swapped == swapped as c_ulong;
        freed = freed.wrapping_add(swapped);
    }
    if freed > 0 {
        (*info).alloced = (*info).alloced.wrapping_sub(freed as c_ulong);
    }
    spin_unlock(addr_of_mut!((*info).lock));
    if freed > 0 {
        shmem_inode_unacct_blocks(inode, freed);
    }
    first
}
#[no_mangle]
pub unsafe extern "C" fn shmem_charge(inode: *mut inode, pages: c_long) -> bool {
    let mapping = (*inode).i_mapping;
    if shmem_inode_acct_blocks(inode, pages) != 0 {
        return false;
    }
    rust_shmem_xa_lock_irq(addr_of_mut!((*mapping).i_pages));
    (*mapping).nrpages = (*mapping).nrpages.wrapping_add(pages as c_ulong);
    rust_shmem_xa_unlock_irq(addr_of_mut!((*mapping).i_pages));
    shmem_recalc_inode(inode, pages, 0);
    true
}
#[no_mangle]
pub unsafe extern "C" fn shmem_uncharge(inode: *mut inode, _pages: c_long) {
    shmem_recalc_inode(inode, 0, 0);
}
