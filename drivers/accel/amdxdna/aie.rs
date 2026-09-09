// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Translated from aie.c. Declarations supplied by the included kernel headers
// remain external dependencies of this translation unit.

pub unsafe fn aie_dump_mgmt_chann_debug(aie: *mut aie_device) {
    let xdna: *mut amdxdna_dev = (*aie).xdna;

    XDNA_DBG(xdna, "i2x tail    0x%x", (*aie).mgmt_i2x.mb_tail_ptr_reg);
    XDNA_DBG(xdna, "i2x head    0x%x", (*aie).mgmt_i2x.mb_head_ptr_reg);
    XDNA_DBG(xdna, "i2x ringbuf 0x%x", (*aie).mgmt_i2x.rb_start_addr);
    XDNA_DBG(xdna, "i2x rsize   0x%x", (*aie).mgmt_i2x.rb_size);
    XDNA_DBG(xdna, "x2i tail    0x%x", (*aie).mgmt_x2i.mb_tail_ptr_reg);
    XDNA_DBG(xdna, "x2i head    0x%x", (*aie).mgmt_x2i.mb_head_ptr_reg);
    XDNA_DBG(xdna, "x2i ringbuf 0x%x", (*aie).mgmt_x2i.rb_start_addr);
    XDNA_DBG(xdna, "x2i rsize   0x%x", (*aie).mgmt_x2i.rb_size);
    XDNA_DBG(xdna, "x2i chann index 0x%x", (*aie).mgmt_chan_idx);
    XDNA_DBG(xdna, "mailbox protocol major 0x%x", (*aie).mgmt_prot_major);
    XDNA_DBG(xdna, "mailbox protocol minor 0x%x", (*aie).mgmt_prot_minor);
}

pub unsafe fn aie_destroy_chann(aie: *mut aie_device, chann: *mut *mut mailbox_channel) {
    let xdna: *mut amdxdna_dev = (*aie).xdna;

    drm_WARN_ON(&mut (*xdna).ddev, !mutex_is_locked(&(*xdna).dev_lock));

    if (*chann).is_null() {
        return;
    }

    xdna_mailbox_stop_channel(*chann);
    xdna_mailbox_free_channel(*chann);
    *chann = core::ptr::null_mut();
}

pub unsafe fn aie_send_mgmt_msg_wait(
    aie: *mut aie_device,
    msg: *mut xdna_mailbox_msg,
) -> i32 {
    let xdna: *mut amdxdna_dev = (*aie).xdna;
    let hdl: *mut xdna_notify = (*msg).handle;
    let mut ret: i32;

    drm_WARN_ON(&mut (*xdna).ddev, !mutex_is_locked(&(*xdna).dev_lock));

    if (*aie).mgmt_chann.is_null() {
        return -ENODEV;
    }

    ret = xdna_send_msg_wait(xdna, (*aie).mgmt_chann, msg);
    if ret == -ETIME {
        aie_destroy_chann(aie, &mut (*aie).mgmt_chann);
    }

    if ret == 0 && *(*hdl).status != 0 {
        XDNA_ERR(xdna, "command opcode 0x%x failed, status 0x%x", (*msg).opcode, *(*hdl).data);
        ret = -EINVAL;
    }

    ret
}

pub unsafe fn aie_check_protocol(aie: *mut aie_device, fw_major: u32, fw_minor: u32) -> i32 {
    let mut feature: *const amdxdna_fw_feature_tbl = (*(*aie).xdna).dev_info.fw_feature_tbl;
    let mut found = false;

    while (*feature).major != 0 {
        if (*feature).major == fw_major
            && fw_minor >= (*feature).min_minor
            && ((*feature).max_minor == 0 || fw_minor <= (*feature).max_minor)
        {
            (*aie).feature_mask |= (*feature).features;

            /* firmware version matches one of the driver support entry */
            found = true;
        }
        feature = feature.add(1);
    }

    if found { 0 } else { -EOPNOTSUPP }
}

unsafe fn amdxdna_update_vbnv(
    xdna: *mut amdxdna_dev,
    tbl: *const amdxdna_rev_vbnv,
    rev: u32,
) {
    let mut i: isize = 0;

    while (*tbl.offset(i)).vbnv != 0 {
        if (*tbl.offset(i)).revision == rev {
            (*xdna).vbnv = (*tbl.offset(i)).vbnv;
            break;
        }
        i += 1;
    }
}

pub unsafe fn amdxdna_vbnv_init(xdna: *mut amdxdna_dev) {
    let info: *const amdxdna_dev_info = (*xdna).dev_info;
    let mut rev: u32 = 0;

    (*xdna).vbnv = (*info).default_vbnv;

    if (*info).ops.get_dev_revision.is_none() || (*info).rev_vbnv_tbl.is_null() {
        return;
    }

    if ((*info).ops.get_dev_revision.unwrap())(xdna, &mut rev) != 0 {
        return;
    }

    amdxdna_update_vbnv(xdna, (*info).rev_vbnv_tbl, rev);
}

pub unsafe fn amdxdna_get_metadata(
    aie: *mut aie_device,
    _client: *mut amdxdna_client,
    args: *mut amdxdna_drm_get_info,
) -> i32 {
    let buf_sz: u32 = core::cmp::min((*args).buffer_size, core::mem::size_of_val(&(*aie).metadata) as u32);

    if copy_to_user(u64_to_user_ptr((*args).buffer), &(*aie).metadata, buf_sz) != 0 {
        -EFAULT
    } else {
        0
    }
}

pub unsafe fn amdxdna_alloc_msg_buffer(
    xdna: *mut amdxdna_dev,
    size: *mut u32,
    dma_addr: *mut dma_addr_t,
) -> *mut core::ffi::c_void {
    *size = core::cmp::max(*size, SZ_8K);
    let order: i32 = get_order(*size);
    if order > MAX_PAGE_ORDER {
        return ERR_PTR(-EINVAL);
    }
    *size = PAGE_SIZE << order;

    if amdxdna_iova_on(xdna) {
        return amdxdna_iommu_alloc(xdna, *size, dma_addr);
    }

    let vaddr = dma_alloc_noncoherent((*xdna).ddev.dev, *size, dma_addr, DMA_FROM_DEVICE, GFP_KERNEL);
    if vaddr.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    vaddr
}

pub unsafe fn amdxdna_free_msg_buffer(
    xdna: *mut amdxdna_dev,
    size: usize,
    cpu_addr: *mut core::ffi::c_void,
    dma_addr: dma_addr_t,
) {
    if amdxdna_iova_on(xdna) {
        amdxdna_iommu_free(xdna, size, cpu_addr, dma_addr);
        return;
    }

    dma_free_noncoherent((*xdna).ddev.dev, size, cpu_addr, dma_addr, DMA_FROM_DEVICE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
