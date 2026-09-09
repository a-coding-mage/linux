// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2022 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Dependency declarations and kernel-provided symbols are supplied externally.

pub unsafe fn hl_mmap_mem_buf_get(mmg: *mut hl_mem_mgr, handle: u64) -> *mut hl_mmap_mem_buf {
    let buf: *mut hl_mmap_mem_buf;

    spin_lock(&mut (*mmg).lock);
    buf = idr_find(&mut (*mmg).handles, lower_32_bits(handle >> PAGE_SHIFT));
    if buf.is_null() {
        spin_unlock(&mut (*mmg).lock);
        dev_dbg((*mmg).dev, "Buff get failed, no match to handle %#llx\n", handle);
        return core::ptr::null_mut();
    }
    kref_get(&mut (*buf).refcount);
    spin_unlock(&mut (*mmg).lock);
    buf
}

unsafe fn hl_mmap_mem_buf_destroy(buf: *mut hl_mmap_mem_buf) {
    if !(*(*buf).behavior).release.is_none() {
        ((*(*buf).behavior).release.unwrap())(buf);
    }

    kfree(buf);
}

unsafe fn hl_mmap_mem_buf_release(kref: *mut kref) {
    let buf: *mut hl_mmap_mem_buf = container_of!(kref, hl_mmap_mem_buf, refcount);

    spin_lock(&mut (*(*buf).mmg).lock);
    idr_remove(&mut (*(*buf).mmg).handles,
               lower_32_bits((*buf).handle >> PAGE_SHIFT));
    spin_unlock(&mut (*(*buf).mmg).lock);

    hl_mmap_mem_buf_destroy(buf);
}

unsafe fn hl_mmap_mem_buf_remove_idr_locked(kref: *mut kref) {
    let buf: *mut hl_mmap_mem_buf = container_of!(kref, hl_mmap_mem_buf, refcount);

    idr_remove(&mut (*(*buf).mmg).handles,
               lower_32_bits((*buf).handle >> PAGE_SHIFT));
}

pub unsafe fn hl_mmap_mem_buf_put(buf: *mut hl_mmap_mem_buf) -> i32 {
    kref_put(&mut (*buf).refcount, hl_mmap_mem_buf_release)
}

pub unsafe fn hl_mmap_mem_buf_put_handle(mmg: *mut hl_mem_mgr, handle: u64) -> i32 {
    let buf: *mut hl_mmap_mem_buf;

    spin_lock(&mut (*mmg).lock);
    buf = idr_find(&mut (*mmg).handles, lower_32_bits(handle >> PAGE_SHIFT));
    if buf.is_null() {
        spin_unlock(&mut (*mmg).lock);
        dev_dbg((*mmg).dev, "Buff put failed, no match to handle %#llx\n", handle);
        return -EINVAL;
    }

    if kref_put(&mut (*buf).refcount, hl_mmap_mem_buf_remove_idr_locked) != 0 {
        spin_unlock(&mut (*mmg).lock);
        hl_mmap_mem_buf_destroy(buf);
        return 1;
    }

    spin_unlock(&mut (*mmg).lock);
    0
}

pub unsafe fn hl_mmap_mem_buf_alloc(
    mmg: *mut hl_mem_mgr,
    behavior: *mut hl_mmap_mem_buf_behavior,
    gfp: gfp_t,
    args: *mut core::ffi::c_void,
) -> *mut hl_mmap_mem_buf {
    let buf = kzalloc_obj!(hl_mmap_mem_buf, gfp);
    let mut rc: i32;
    if buf.is_null() {
        return core::ptr::null_mut();
    }

    spin_lock(&mut (*mmg).lock);
    rc = idr_alloc(&mut (*mmg).handles, buf, 1, 0, GFP_ATOMIC);
    spin_unlock(&mut (*mmg).lock);
    if rc < 0 {
        dev_err((*mmg).dev,
                "{}: Failed to allocate IDR for a new buffer, rc={}\n",
                (*behavior).topic, rc);
        goto_free_buf!(buf);
    }

    (*buf).mmg = mmg;
    (*buf).behavior = behavior;
    (*buf).handle = (((rc as u64) | (*behavior).mem_id) << PAGE_SHIFT);
    kref_init(&mut (*buf).refcount);

    rc = ((*behavior).alloc.unwrap())(buf, gfp, args);
    if rc != 0 {
        dev_err((*mmg).dev, "{}: Failure in buffer alloc callback {}\n", (*behavior).topic, rc);
        spin_lock(&mut (*mmg).lock);
        idr_remove(&mut (*mmg).handles, lower_32_bits((*buf).handle >> PAGE_SHIFT));
        spin_unlock(&mut (*mmg).lock);
        kfree(buf);
        return core::ptr::null_mut();
    }

    buf
}

unsafe fn hl_mmap_mem_buf_vm_close(vma: *mut vm_area_struct) {
    let buf = (*vma).vm_private_data as *mut hl_mmap_mem_buf;
    let new_mmap_size = (*buf).real_mapped_size - ((*vma).vm_end - (*vma).vm_start);

    if new_mmap_size > 0 {
        (*buf).real_mapped_size = new_mmap_size;
        return;
    }

    atomic_set(&mut (*buf).mmap, 0);
    hl_mmap_mem_buf_put(buf);
    (*vma).vm_private_data = core::ptr::null_mut();
}

static hl_mmap_mem_buf_vm_ops: vm_operations_struct = vm_operations_struct {
    close: Some(hl_mmap_mem_buf_vm_close),
};

pub unsafe fn hl_mem_mgr_mmap(
    mmg: *mut hl_mem_mgr,
    vma: *mut vm_area_struct,
    args: *mut core::ffi::c_void,
) -> i32 {
    let handle = (*vma).vm_pgoff << PAGE_SHIFT;
    (*vma).vm_pgoff = 0;

    let buf = hl_mmap_mem_buf_get(mmg, handle);
    if buf.is_null() {
        dev_err((*mmg).dev, "Memory mmap failed, no match to handle %#llx\n", handle);
        return -EINVAL;
    }

    let user_mem_size = (*vma).vm_end - (*vma).vm_start;
    if user_mem_size != ALIGN!((*buf).mappable_size, PAGE_SIZE) {
        dev_err((*mmg).dev, "{}: Memory mmap failed, mmap VM size 0x{:x} != 0x{:x} allocated physical mem size\n",
                (*(*buf).behavior).topic, user_mem_size, (*buf).mappable_size);
        hl_mmap_mem_buf_put(buf);
        return -EINVAL;
    }

    if !access_ok((*vma).vm_start as *mut core::ffi::c_void, user_mem_size) {
        dev_err((*mmg).dev, "{}: User pointer is invalid - 0x{:x}\n", (*(*buf).behavior).topic, (*vma).vm_start);
        hl_mmap_mem_buf_put(buf);
        return -EINVAL;
    }

    if atomic_cmpxchg(&mut (*buf).mmap, 0, 1) != 0 {
        dev_err((*mmg).dev, "{}, Memory mmap failed, already mapped to user\n", (*(*buf).behavior).topic);
        hl_mmap_mem_buf_put(buf);
        return -EINVAL;
    }

    (*vma).vm_ops = &hl_mmap_mem_buf_vm_ops;
    (*vma).vm_private_data = buf as *mut core::ffi::c_void;

    let rc = ((*(*buf).behavior).mmap.unwrap())(buf, vma, args);
    if rc != 0 {
        atomic_set(&mut (*buf).mmap, 0);
        hl_mmap_mem_buf_put(buf);
        return rc;
    }

    (*buf).real_mapped_size = (*buf).mappable_size;
    (*vma).vm_pgoff = handle >> PAGE_SHIFT;
    0
}

pub unsafe fn hl_mem_mgr_init(dev: *mut device, mmg: *mut hl_mem_mgr) {
    (*mmg).dev = dev;
    spin_lock_init(&mut (*mmg).lock);
    idr_init(&mut (*mmg).handles);
}

unsafe fn hl_mem_mgr_fini_stats_reset(stats: *mut hl_mem_mgr_fini_stats) {
    if !stats.is_null() { memset(stats, 0, core::mem::size_of::<hl_mem_mgr_fini_stats>()); }
}

unsafe fn hl_mem_mgr_fini_stats_inc(mem_id: u64, stats: *mut hl_mem_mgr_fini_stats) {
    if stats.is_null() { return; }
    match mem_id {
        HL_MMAP_TYPE_CB => (*stats).n_busy_cb += 1,
        HL_MMAP_TYPE_TS_BUFF => (*stats).n_busy_ts += 1,
        _ => (*stats).n_busy_other += 1,
    }
}

pub unsafe fn hl_mem_mgr_fini(mmg: *mut hl_mem_mgr, stats: *mut hl_mem_mgr_fini_stats) {
    hl_mem_mgr_fini_stats_reset(stats);
    let idp = &mut (*mmg).handles;
    idr_for_each_entry!(idp, buf, id, {
        let topic = (*(*buf).behavior).topic;
        let mem_id = (*(*buf).behavior).mem_id;
        if hl_mmap_mem_buf_put(buf) != 1 {
            dev_err((*mmg).dev, "{}: Buff handle {} for CTX is still alive\n", topic, id);
            hl_mem_mgr_fini_stats_inc(mem_id, stats);
        }
    });
}

pub unsafe fn hl_mem_mgr_idr_destroy(mmg: *mut hl_mem_mgr) {
    if !idr_is_empty(&mut (*mmg).handles) {
        dev_crit((*mmg).dev, "memory manager IDR is destroyed while it is not empty!\n");
    }
    idr_destroy(&mut (*mmg).handles);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
