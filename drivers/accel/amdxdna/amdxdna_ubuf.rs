// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/Rust bindings are intentionally
// referenced here rather than reimplemented in this translation.

#[repr(C)]
struct amdxdna_ubuf_priv {
    pages: *mut *mut page,
    nr_pages: u64,
    mm: *mut mm_struct,
}

unsafe extern "C" {
    static amdxdna_ubuf_dmabuf_ops: dma_buf_ops;
}

unsafe extern "C" fn amdxdna_ubuf_map(
    attach: *mut dma_buf_attachment,
    direction: dma_data_direction,
) -> *mut sg_table {
    let ubuf = (*(*attach).dmabuf).priv_ as *mut amdxdna_ubuf_priv;
    let sg = kzalloc_obj::<sg_table>();
    let mut ret: i32;

    if sg.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    ret = sg_alloc_table_from_pages(
        sg,
        (*ubuf).pages,
        (*ubuf).nr_pages,
        0,
        (*ubuf).nr_pages.wrapping_shl(PAGE_SHIFT),
        GFP_KERNEL,
    );
    if ret != 0 {
        kfree(sg as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }

    ret = dma_map_sgtable((*attach).dev, sg, direction, 0);
    if ret != 0 {
        sg_free_table(sg);
        kfree(sg as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }

    sg
}

unsafe extern "C" fn amdxdna_ubuf_unmap(
    attach: *mut dma_buf_attachment,
    sg: *mut sg_table,
    direction: dma_data_direction,
) {
    dma_unmap_sgtable((*attach).dev, sg, direction, 0);
    sg_free_table(sg);
    kfree(sg as *mut core::ffi::c_void);
}

unsafe extern "C" fn amdxdna_ubuf_release(dbuf: *mut dma_buf) {
    let ubuf = (*dbuf).priv_ as *mut amdxdna_ubuf_priv;

    unpin_user_pages((*ubuf).pages, (*ubuf).nr_pages);
    kvfree((*ubuf).pages as *mut core::ffi::c_void);
    atomic64_sub((*ubuf).nr_pages, &mut (*(*ubuf).mm).pinned_vm);
    mmdrop((*ubuf).mm);
    kfree(ubuf as *mut core::ffi::c_void);
}

static AMDXDNA_UBUF_DMABUF_OPS: dma_buf_ops = dma_buf_ops {
    map_dma_buf: Some(amdxdna_ubuf_map),
    unmap_dma_buf: Some(amdxdna_ubuf_unmap),
    release: Some(amdxdna_ubuf_release),
};

unsafe extern "C" fn amdxdna_get_ubuf(
    dev: *mut drm_device,
    num_entries: u32,
    va_entries: *mut core::ffi::c_void,
) -> *mut dma_buf {
    let xdna = to_xdna_dev(dev);
    let mut lock_limit: usize;
    let mut new_pinned: usize;
    let mut va_ent: *mut amdxdna_drm_va_entry;
    let mut ubuf: *mut amdxdna_ubuf_priv;
    let mut npages: u32;
    let mut start: u32 = 0;
    let mut dbuf: *mut dma_buf;
    let mut ret: i32;
    let mut exp_info: dma_buf_export_info = core::mem::zeroed();

    if !can_do_mlock() {
        return ERR_PTR(-EPERM);
    }

    ubuf = kzalloc_obj::<amdxdna_ubuf_priv>();
    if ubuf.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*ubuf).mm = (*current).mm;
    mmgrab((*ubuf).mm);

    va_ent = kvzalloc_objs::<amdxdna_drm_va_entry>(num_entries);
    if va_ent.is_null() {
        ret = -ENOMEM;
        mmdrop((*ubuf).mm);
        kfree(ubuf as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }

    if copy_from_user(
        va_ent as *mut core::ffi::c_void,
        va_entries,
        core::mem::size_of::<amdxdna_drm_va_entry>() * num_entries as usize,
    ) != 0 {
        XDNA_DBG(xdna, "Access va entries failed");
        ret = -EINVAL;
        kvfree(va_ent as *mut core::ffi::c_void);
        mmdrop((*ubuf).mm);
        kfree(ubuf as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }

    exp_info.size = 0;
    for i in 0..num_entries {
        let ent = &*va_ent.add(i as usize);
        if !IS_ALIGNED(ent.vaddr, PAGE_SIZE) || !IS_ALIGNED(ent.len, PAGE_SIZE) {
            XDNA_ERR(xdna, "Invalid address or len %llx, %llx", ent.vaddr, ent.len);
            ret = -EINVAL;
            kvfree(va_ent as *mut core::ffi::c_void);
            mmdrop((*ubuf).mm);
            kfree(ubuf as *mut core::ffi::c_void);
            return ERR_PTR(ret);
        }
        let (size, overflow) = exp_info.size.overflowing_add(ent.len);
        exp_info.size = size;
        if overflow {
            ret = -EINVAL;
            kvfree(va_ent as *mut core::ffi::c_void);
            mmdrop((*ubuf).mm);
            kfree(ubuf as *mut core::ffi::c_void);
            return ERR_PTR(ret);
        }
    }

    (*ubuf).nr_pages = exp_info.size >> PAGE_SHIFT;
    lock_limit = rlimit(RLIMIT_MEMLOCK) >> PAGE_SHIFT;
    new_pinned = atomic64_add_return((*ubuf).nr_pages, &mut (*(*ubuf).mm).pinned_vm);
    if new_pinned > lock_limit && !capable(CAP_IPC_LOCK) {
        XDNA_DBG(xdna, "New pin %ld, limit %ld, cap %d", new_pinned, lock_limit, capable(CAP_IPC_LOCK));
        ret = -ENOMEM;
        atomic64_sub((*ubuf).nr_pages, &mut (*(*ubuf).mm).pinned_vm);
        kvfree(va_ent as *mut core::ffi::c_void);
        mmdrop((*ubuf).mm);
        kfree(ubuf as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }

    (*ubuf).pages = kvmalloc_objs::<*mut page>((*ubuf).nr_pages);
    if (*ubuf).pages.is_null() {
        ret = -ENOMEM;
        atomic64_sub((*ubuf).nr_pages, &mut (*(*ubuf).mm).pinned_vm);
        kvfree(va_ent as *mut core::ffi::c_void);
        mmdrop((*ubuf).mm);
        kfree(ubuf as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }

    for i in 0..num_entries {
        let ent = &*va_ent.add(i as usize);
        npages = ent.len >> PAGE_SHIFT;
        ret = pin_user_pages_fast(ent.vaddr, npages, FOLL_WRITE | FOLL_LONGTERM, (*ubuf).pages.add(start as usize));
        if ret >= 0 {
            start = start.wrapping_add(ret as u32);
            if ret as u32 != npages {
                XDNA_ERR(xdna, "Partially pinned pages %d/%u", ret, npages);
                ret = -ENOMEM;
                if start != 0 { unpin_user_pages((*ubuf).pages, start as u64); }
                kvfree((*ubuf).pages as *mut core::ffi::c_void);
                atomic64_sub((*ubuf).nr_pages, &mut (*(*ubuf).mm).pinned_vm);
                kvfree(va_ent as *mut core::ffi::c_void);
                mmdrop((*ubuf).mm);
                kfree(ubuf as *mut core::ffi::c_void);
                return ERR_PTR(ret);
            }
        } else {
            XDNA_ERR(xdna, "Failed to pin pages ret %d", ret);
            if start != 0 { unpin_user_pages((*ubuf).pages, start as u64); }
            kvfree((*ubuf).pages as *mut core::ffi::c_void);
            atomic64_sub((*ubuf).nr_pages, &mut (*(*ubuf).mm).pinned_vm);
            kvfree(va_ent as *mut core::ffi::c_void);
            mmdrop((*ubuf).mm);
            kfree(ubuf as *mut core::ffi::c_void);
            return ERR_PTR(ret);
        }
    }

    exp_info.ops = &AMDXDNA_UBUF_DMABUF_OPS;
    exp_info.priv_ = ubuf as *mut core::ffi::c_void;
    exp_info.flags = O_RDWR | O_CLOEXEC;
    dbuf = dma_buf_export(&exp_info);
    if IS_ERR(dbuf) {
        ret = PTR_ERR(dbuf);
        if start != 0 { unpin_user_pages((*ubuf).pages, start as u64); }
        kvfree((*ubuf).pages as *mut core::ffi::c_void);
        atomic64_sub((*ubuf).nr_pages, &mut (*(*ubuf).mm).pinned_vm);
        kvfree(va_ent as *mut core::ffi::c_void);
        mmdrop((*ubuf).mm);
        kfree(ubuf as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }
    kvfree(va_ent as *mut core::ffi::c_void);
    dbuf
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
