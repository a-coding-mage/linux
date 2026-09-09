// SPDX-License-Identifier: GPL-2.0
/* XDP user-space packet buffer
 * Copyright(c) 2018 Intel Corporation.
 */

// Kernel dependencies and symbols are supplied by the surrounding translation.

static mut UMEM_IDA: Ida = DEFINE_IDA!();

unsafe fn xdp_umem_unpin_pages(umem: *mut xdp_umem) {
    unpin_user_pages_dirty_lock((*umem).pgs, (*umem).npgs, true);

    kvfree((*umem).pgs as *mut core::ffi::c_void);
    (*umem).pgs = core::ptr::null_mut();
}

unsafe fn xdp_umem_unaccount_pages(umem: *mut xdp_umem) {
    if !(*umem).user.is_null() {
        atomic_long_sub((*umem).npgs, &mut (*(*umem).user).locked_vm);
        free_uid((*umem).user);
    }
}

unsafe fn xdp_umem_addr_unmap(umem: *mut xdp_umem) {
    vunmap((*umem).addrs);
    (*umem).addrs = core::ptr::null_mut();
}

unsafe fn xdp_umem_addr_map(
    umem: *mut xdp_umem,
    pages: *mut *mut page,
    nr_pages: u32,
) -> i32 {
    (*umem).addrs = vmap(pages, nr_pages, VM_MAP, PAGE_KERNEL);
    if (*umem).addrs.is_null() {
        return -ENOMEM;
    }
    0
}

unsafe fn xdp_umem_release(umem: *mut xdp_umem) {
    (*umem).zc = false;
    ida_free(&mut UMEM_IDA, (*umem).id);

    xdp_umem_addr_unmap(umem);
    xdp_umem_unpin_pages(umem);

    xdp_umem_unaccount_pages(umem);
    kfree(umem as *mut core::ffi::c_void);
}

unsafe fn xdp_umem_release_deferred(work: *mut work_struct) {
    let umem = container_of!(work, xdp_umem, work);

    xdp_umem_release(umem);
}

pub unsafe fn xdp_get_umem(umem: *mut xdp_umem) {
    refcount_inc(&mut (*umem).users);
}

pub unsafe fn xdp_put_umem(umem: *mut xdp_umem, defer_cleanup: bool) {
    if umem.is_null() {
        return;
    }

    if refcount_dec_and_test(&mut (*umem).users) {
        if defer_cleanup {
            INIT_WORK!(&mut (*umem).work, xdp_umem_release_deferred);
            schedule_work(&mut (*umem).work);
        } else {
            xdp_umem_release(umem);
        }
    }
}

unsafe fn xdp_umem_pin_pages(umem: *mut xdp_umem, address: usize) -> i32 {
    let gup_flags: u32 = FOLL_WRITE;
    let mut npgs: isize;
    let err: i32;

    (*umem).pgs = kvzalloc_objs!((*umem).pgs, (*umem).npgs, GFP_KERNEL | __GFP_NOWARN);
    if (*umem).pgs.is_null() {
        return -ENOMEM;
    }

    mmap_read_lock((*current).mm);
    npgs = pin_user_pages(
        address,
        (*umem).npgs,
        gup_flags | FOLL_LONGTERM,
        &mut *(*umem).pgs,
    );
    mmap_read_unlock((*current).mm);

    if npgs != (*umem).npgs as isize {
        if npgs >= 0 {
            (*umem).npgs = npgs as u32;
            err = -ENOMEM;
            goto_out_pin!(out_pin);
        }
        err = npgs as i32;
        goto_out_pgs!(out_pgs);
    }
    return 0;

out_pin:
    xdp_umem_unpin_pages(umem);
out_pgs:
    kvfree((*umem).pgs as *mut core::ffi::c_void);
    (*umem).pgs = core::ptr::null_mut();
    err
}

unsafe fn xdp_umem_account_pages(umem: *mut xdp_umem) -> i32 {
    let lock_limit: usize;
    let mut new_npgs: usize;
    let mut old_npgs: usize;

    if capable(CAP_IPC_LOCK) {
        return 0;
    }

    lock_limit = rlimit(RLIMIT_MEMLOCK) >> PAGE_SHIFT;
    (*umem).user = get_uid(current_user());

    loop {
        old_npgs = atomic_long_read(&(*(*umem).user).locked_vm);
        new_npgs = old_npgs + (*umem).npgs as usize;
        if new_npgs > lock_limit {
            free_uid((*umem).user);
            (*umem).user = core::ptr::null_mut();
            return -ENOBUFS;
        }
        if atomic_long_cmpxchg(
            &mut (*(*umem).user).locked_vm,
            old_npgs,
            new_npgs,
        ) == old_npgs {
            break;
        }
    }
    0
}

const XDP_UMEM_FLAGS_VALID: u32 =
    XDP_UMEM_UNALIGNED_CHUNK_FLAG | XDP_UMEM_TX_SW_CSUM | XDP_UMEM_TX_METADATA_LEN | 0;

unsafe fn xdp_umem_reg(umem: *mut xdp_umem, mr: *mut xdp_umem_reg) -> i32 {
    let unaligned_chunks = (*mr).flags & XDP_UMEM_UNALIGNED_CHUNK_FLAG != 0;
    let chunk_size = (*mr).chunk_size;
    let headroom = (*mr).headroom;
    let addr = (*mr).addr;
    let size = (*mr).len;
    let mut chunks_rem: u32 = 0;
    let mut npgs_rem: u32 = 0;
    let chunks: u64;
    let npgs: u64;
    let err: i32;

    if chunk_size < XDP_UMEM_MIN_CHUNK_SIZE || chunk_size > PAGE_SIZE {
        /* Strictly speaking we could support this, if:
         * - huge pages, or*
         * - using an IOMMU, or
         * - making sure the memory area is consecutive
         * but for now, we simply say "computer says no".
         */
        return -EINVAL;
    }

    if (*mr).flags & !XDP_UMEM_FLAGS_VALID != 0 {
        return -EINVAL;
    }

    if !unaligned_chunks && !is_power_of_2(chunk_size) {
        return -EINVAL;
    }

    if !PAGE_ALIGNED!(addr) {
        /* Memory area has to be page size aligned. For
         * simplicity, this might change.
         */
        return -EINVAL;
    }

    if addr.wrapping_add(size) < addr {
        return -EINVAL;
    }

    npgs = div_u64_rem(size, PAGE_SIZE, &mut npgs_rem);
    let npgs = if npgs_rem != 0 { npgs + 1 } else { npgs };
    if npgs > U32_MAX as u64 {
        return -EINVAL;
    }

    chunks = div_u64_rem(size, chunk_size, &mut chunks_rem);
    if chunks == 0 || chunks > U32_MAX as u64 {
        return -EINVAL;
    }

    if !unaligned_chunks && chunks_rem != 0 {
        return -EINVAL;
    }

    if headroom > chunk_size - XDP_PACKET_HEADROOM
        - SKB_DATA_ALIGN!(core::mem::size_of::<skb_shared_info>())
        - 128
    {
        return -EINVAL;
    }

    if (*mr).flags & XDP_UMEM_TX_METADATA_LEN != 0 {
        if (*mr).tx_metadata_len >= 256 || (*mr).tx_metadata_len % 8 != 0 {
            return -EINVAL;
        }
        if (*mr).tx_metadata_len < 16 {
            return -EINVAL;
        }
        (*umem).tx_metadata_len = (*mr).tx_metadata_len;
    }

    (*umem).size = size;
    (*umem).headroom = headroom;
    (*umem).chunk_size = chunk_size;
    (*umem).chunks = chunks;
    (*umem).npgs = npgs;
    (*umem).pgs = core::ptr::null_mut();
    (*umem).user = core::ptr::null_mut();
    (*umem).flags = (*mr).flags;

    INIT_LIST_HEAD!(&mut (*umem).xsk_dma_list);
    refcount_set(&mut (*umem).users, 1);

    err = xdp_umem_account_pages(umem);
    if err != 0 {
        return err;
    }

    let err = xdp_umem_pin_pages(umem, addr as usize);
    if err != 0 {
        xdp_umem_unaccount_pages(umem);
        return err;
    }

    let err = xdp_umem_addr_map(umem, (*umem).pgs, (*umem).npgs as u32);
    if err != 0 {
        xdp_umem_unpin_pages(umem);
        xdp_umem_unaccount_pages(umem);
        return err;
    }

    0
}

pub unsafe fn xdp_umem_create(mr: *mut xdp_umem_reg) -> *mut xdp_umem {
    let umem = kzalloc_obj::<xdp_umem>();
    if umem.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let err = ida_alloc(&mut UMEM_IDA, GFP_KERNEL);
    if err < 0 {
        kfree(umem as *mut core::ffi::c_void);
        return ERR_PTR(err);
    }
    (*umem).id = err;

    let err = xdp_umem_reg(umem, mr);
    if err != 0 {
        ida_free(&mut UMEM_IDA, (*umem).id);
        kfree(umem as *mut core::ffi::c_void);
        return ERR_PTR(err);
    }

    umem
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
