// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 Linaro Ltd.
 * Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
 */

// Dependencies supplied by the surrounding kernel translation.

pub unsafe fn mhi_ep_ring_addr2offset(ring: *mut mhi_ep_ring, ptr: u64) -> usize {
    (ptr.wrapping_sub((*ring).rbase) as usize) / core::mem::size_of::<mhi_ring_element>()
}

unsafe fn mhi_ep_ring_num_elems(ring: *mut mhi_ep_ring) -> u32 {
    let mut rlen: __le64 = core::mem::zeroed();
    memcpy_fromio(
        &mut rlen as *mut __le64 as *mut core::ffi::c_void,
        &(*(*ring).ring_ctx).generic.rlen as *const _ as *const core::ffi::c_void,
        core::mem::size_of::<u64>(),
    );

    (le64_to_cpu(rlen) / core::mem::size_of::<mhi_ring_element>() as u64) as u32
}

pub unsafe fn mhi_ep_ring_inc_index(ring: *mut mhi_ep_ring) {
    (*ring).rd_offset = ((*ring).rd_offset + 1) % (*ring).ring_size as usize;
}

unsafe fn __mhi_ep_cache_ring(ring: *mut mhi_ep_ring, end: usize) -> i32 {
    let mhi_cntrl = (*ring).mhi_cntrl;
    let dev = &(*(*mhi_cntrl).mhi_dev).dev;
    let mut buf_info: mhi_ep_buf_info = core::mem::zeroed();
    let start: usize;
    let mut ret: i32;

    /* Don't proceed in the case of event ring. This happens during mhi_ep_ring_start(). */
    if (*ring).type_ == RING_TYPE_ER {
        return 0;
    }

    /* No need to cache the ring if write pointer is unmodified */
    if (*ring).wr_offset == end {
        return 0;
    }

    start = (*ring).wr_offset;
    if start < end {
        buf_info.size = (end - start) * core::mem::size_of::<mhi_ring_element>();
        buf_info.host_addr = (*ring).rbase + (start * core::mem::size_of::<mhi_ring_element>()) as u64;
        buf_info.dev_addr = (*ring).ring_cache.add(start);
        ret = ((*mhi_cntrl).read_sync)(mhi_cntrl, &mut buf_info);
        if ret != 0 { return ret; }
    } else {
        buf_info.size = ((*ring).ring_size as usize - start) * core::mem::size_of::<mhi_ring_element>();
        buf_info.host_addr = (*ring).rbase + (start * core::mem::size_of::<mhi_ring_element>()) as u64;
        buf_info.dev_addr = (*ring).ring_cache.add(start);
        ret = ((*mhi_cntrl).read_sync)(mhi_cntrl, &mut buf_info);
        if ret != 0 { return ret; }

        if end != 0 {
            buf_info.host_addr = (*ring).rbase;
            buf_info.dev_addr = (*ring).ring_cache;
            buf_info.size = end * core::mem::size_of::<mhi_ring_element>();
            ret = ((*mhi_cntrl).read_sync)(mhi_cntrl, &mut buf_info);
            if ret != 0 { return ret; }
        }
    }

    dev_dbg(dev, "Cached ring: start %zu end %zu size %zu\n", start, end, buf_info.size);
    0
}

unsafe fn mhi_ep_cache_ring(ring: *mut mhi_ep_ring, wr_ptr: u64) -> i32 {
    let wr_offset = mhi_ep_ring_addr2offset(ring, wr_ptr);
    let ret = __mhi_ep_cache_ring(ring, wr_offset);
    if ret != 0 { return ret; }
    (*ring).wr_offset = wr_offset;
    0
}

pub unsafe fn mhi_ep_update_wr_offset(ring: *mut mhi_ep_ring) -> i32 {
    mhi_ep_cache_ring(ring, mhi_ep_mmio_get_db(ring))
}

/* TODO: Support for adding multiple ring elements to the ring */
pub unsafe fn mhi_ep_ring_add_element(ring: *mut mhi_ep_ring, el: *mut mhi_ring_element) -> i32 {
    let mhi_cntrl = (*ring).mhi_cntrl;
    let dev = &(*(*mhi_cntrl).mhi_dev).dev;
    let mut buf_info: mhi_ep_buf_info = core::mem::zeroed();
    let old_offset = 0usize;
    let num_free_elem: u32;
    let mut rp: __le64;
    let ret = mhi_ep_update_wr_offset(ring);
    if ret != 0 { dev_err(dev, "Error updating write pointer\n"); return ret; }
    if (*ring).rd_offset < (*ring).wr_offset {
        num_free_elem = ((*ring).wr_offset - (*ring).rd_offset - 1) as u32;
    } else {
        num_free_elem = (((*ring).ring_size as usize - (*ring).rd_offset) + (*ring).wr_offset - 1) as u32;
    }
    if num_free_elem == 0 { dev_err(dev, "No space left in the ring\n"); return -ENOSPC; }
    dev_dbg(dev, "Adding an element to ring at offset (%zu)\n", (*ring).rd_offset);
    buf_info.host_addr = (*ring).rbase + (old_offset * core::mem::size_of_val(&*el)) as u64;
    buf_info.dev_addr = el;
    buf_info.size = core::mem::size_of_val(&*el);
    let ret = ((*mhi_cntrl).write_sync)(mhi_cntrl, &mut buf_info);
    if ret != 0 { return ret; }
    mhi_ep_ring_inc_index(ring);
    rp = cpu_to_le64(((*ring).rd_offset * core::mem::size_of_val(&*el)) as u64 + (*ring).rbase);
    memcpy_toio(&mut (*(*ring).ring_ctx).generic.rp as *mut _ as *mut core::ffi::c_void, &rp as *const _ as *const core::ffi::c_void, core::mem::size_of::<u64>());
    ret
}

pub unsafe fn mhi_ep_ring_init(ring: *mut mhi_ep_ring, type_: mhi_ep_ring_type, id: u32) {
    (*ring).type_ = type_;
    if type_ == RING_TYPE_CMD { (*ring).db_offset_h = EP_CRDB_HIGHER; (*ring).db_offset_l = EP_CRDB_LOWER; }
    else if type_ == RING_TYPE_CH { (*ring).db_offset_h = CHDB_HIGHER_n(id); (*ring).db_offset_l = CHDB_LOWER_n(id); (*ring).ch_id = id; }
    else { (*ring).db_offset_h = ERDB_HIGHER_n(id); (*ring).db_offset_l = ERDB_LOWER_n(id); }
}

unsafe fn mhi_ep_raise_irq(work: *mut work_struct) {
    let ring = container_of!(work, mhi_ep_ring, intmodt_work.work);
    let mhi_cntrl = (*ring).mhi_cntrl;
    ((*mhi_cntrl).raise_irq)(mhi_cntrl, (*ring).irq_vector);
    WRITE_ONCE!((*ring).irq_pending, false);
}

pub unsafe fn mhi_ep_ring_start(mhi_cntrl: *mut mhi_ep_cntrl, ring: *mut mhi_ep_ring, ctx: *mut mhi_ep_ring_ctx) -> i32 {
    let dev = &(*(*mhi_cntrl).mhi_dev).dev;
    let mut val: __le64;
    (*ring).mhi_cntrl = mhi_cntrl; (*ring).ring_ctx = ctx; (*ring).ring_size = mhi_ep_ring_num_elems(ring) as usize;
    val = core::mem::zeroed(); memcpy_fromio(&mut val as *mut _ as *mut _, &(*ctx).generic.rbase as *const _ as *const _, core::mem::size_of::<u64>()); (*ring).rbase = le64_to_cpu(val);
    if (*ring).type_ == RING_TYPE_CH { (*ring).er_index = le32_to_cpu((*ctx).ch.erindex); }
    if (*ring).type_ == RING_TYPE_ER { (*ring).irq_vector = le32_to_cpu((*ctx).ev.msivec); (*ring).intmodt = FIELD_GET!(EV_CTX_INTMODT_MASK, le32_to_cpu((*ctx).ev.intmod)); INIT_DELAYED_WORK!(&mut (*ring).intmodt_work, mhi_ep_raise_irq); }
    memcpy_fromio(&mut val as *mut _ as *mut _, &(*ctx).generic.rp as *const _ as *const _, core::mem::size_of::<u64>()); (*ring).rd_offset = mhi_ep_ring_addr2offset(ring, le64_to_cpu(val)); (*ring).wr_offset = (*ring).rd_offset;
    (*ring).ring_cache = kzalloc_objs!(mhi_ring_element, (*ring).ring_size); if (*ring).ring_cache.is_null() { return -ENOMEM; }
    memcpy_fromio(&mut val as *mut _ as *mut _, &(*ctx).generic.wp as *const _ as *const _, core::mem::size_of::<u64>());
    let ret = mhi_ep_cache_ring(ring, le64_to_cpu(val)); if ret != 0 { dev_err(dev, "Failed to cache ring\n"); kfree((*ring).ring_cache); return ret; }
    (*ring).started = true; 0
}

pub unsafe fn mhi_ep_ring_reset(_mhi_cntrl: *mut mhi_ep_cntrl, ring: *mut mhi_ep_ring) {
    if (*ring).type_ == RING_TYPE_ER { cancel_delayed_work_sync!(&mut (*ring).intmodt_work); }
    (*ring).started = false; kfree((*ring).ring_cache); (*ring).ring_cache = core::ptr::null_mut();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
