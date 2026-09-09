// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2017, Microsoft Corporation.
 *   Copyright (C) 2018, LG Electronics.
 *   Copyright (c) 2025, Stefan Metzmacher
 */

// Dependency declarations supplied by the surrounding kernel/RDMA code are
// intentionally not reproduced here.

unsafe fn smbdirect_connection_wait_for_rw_credits(
    sc: *mut smbdirect_socket,
    credits: i32,
) -> i32 {
    smbdirect_socket_wait_for_credits(
        sc,
        SMBDIRECT_SOCKET_CONNECTED,
        -ENOTCONN,
        unsafe { &mut (*sc).rw_io.credits.wait_queue },
        unsafe { &mut (*sc).rw_io.credits.count },
        credits,
    )
}

unsafe fn smbdirect_connection_calc_rw_credits(
    sc: *mut smbdirect_socket,
    buf: *const core::ffi::c_void,
    len: usize,
) -> i32 {
    DIV_ROUND_UP(
        smbdirect_get_buf_page_count(buf, len),
        unsafe { (*sc).rw_io.credits.num_pages },
    )
}

unsafe fn smbdirect_connection_rdma_get_sg_list(
    mut buf: *mut core::ffi::c_void,
    mut size: usize,
    mut sg_list: *mut scatterlist,
    nentries: usize,
) -> i32 {
    let high = is_vmalloc_addr(buf);
    let mut page: *mut page;
    let mut offset: usize;
    let mut len: usize;
    let mut i: i32 = 0;

    if size == 0 || nentries < smbdirect_get_buf_page_count(buf, size) {
        return -EINVAL;
    }

    offset = offset_in_page(buf);
    buf = (buf as *mut u8).wrapping_sub(offset) as *mut core::ffi::c_void;
    while size > 0 {
        len = core::cmp::min(PAGE_SIZE - offset, size);
        page = if high { vmalloc_to_page(buf) } else { kmap_to_page(buf) };

        if sg_list.is_null() {
            return -EINVAL;
        }
        sg_set_page(sg_list, page, len, offset);
        sg_list = sg_next(sg_list);

        buf = (buf as *mut u8).wrapping_add(PAGE_SIZE) as *mut core::ffi::c_void;
        size -= len;
        offset = 0;
        i += 1;
    }

    i
}

unsafe fn smbdirect_connection_rw_io_free(
    msg: *mut smbdirect_rw_io,
    dir: dma_data_direction,
) {
    let sc = (*msg).socket;

    rdma_rw_ctx_destroy(
        &mut (*msg).rdma_ctx,
        (*sc).ib.qp,
        (*sc).ib.qp.port,
        (*msg).sgt.sgl,
        (*msg).sgt.nents,
        dir,
    );
    sg_free_table_chained(&mut (*msg).sgt, SG_CHUNK_SIZE);
    kfree(msg);
}

unsafe fn smbdirect_connection_rdma_rw_done(
    _cq: *mut ib_cq,
    wc: *mut ib_wc,
    dir: dma_data_direction,
) {
    let msg = container_of((*wc).wr_cqe, smbdirect_rw_io, cqe);
    let sc = (*msg).socket;

    if (*wc).status != IB_WC_SUCCESS {
        (*msg).error = -EIO;
        pr_err!("read/write error. opcode = %d, status = %s(%d)\n", (*wc).opcode, ib_wc_status_msg((*wc).status), (*wc).status);
        if (*wc).status != IB_WC_WR_FLUSH_ERR {
            smbdirect_socket_schedule_cleanup(sc, (*msg).error);
        }
    }

    complete((*msg).completion);
}

unsafe fn smbdirect_connection_rdma_read_done(cq: *mut ib_cq, wc: *mut ib_wc) {
    smbdirect_connection_rdma_rw_done(cq, wc, DMA_FROM_DEVICE);
}

unsafe fn smbdirect_connection_rdma_write_done(cq: *mut ib_cq, wc: *mut ib_wc) {
    smbdirect_connection_rdma_rw_done(cq, wc, DMA_TO_DEVICE);
}

#[no_mangle]
pub unsafe extern "C" fn smbdirect_connection_rdma_xmit(
    sc: *mut smbdirect_socket,
    mut buf: *mut core::ffi::c_void,
    mut buf_len: usize,
    desc: *mut smbdirect_buffer_descriptor_v1,
    desc_len: usize,
    is_read: bool,
) -> i32 {
    let sp = &(*sc).parameters;
    let direction = if is_read { DMA_FROM_DEVICE } else { DMA_TO_DEVICE };
    let mut msg: *mut smbdirect_rw_io;
    let mut next_msg: *mut smbdirect_rw_io;
    let mut i: usize;
    let mut ret: i32;
    let completion = DECLARE_COMPLETION_ONSTACK!();
    let mut first_wr: *mut ib_send_wr;
    let mut msg_list = LIST_HEAD!();
    let mut desc_buf = buf as *mut u8;
    let mut credits_needed: i32;
    let mut desc_buf_len: usize;
    let mut desc_num: usize = 0;

    if (*sc).status != SMBDIRECT_SOCKET_CONNECTED { return -ENOTCONN; }
    if buf_len > sp.max_read_write_size { return -EINVAL; }

    credits_needed = 0;
    i = 0;
    while i < desc_len / core::mem::size_of::<smbdirect_buffer_descriptor_v1>() {
        if buf_len == 0 { break; }
        desc_buf_len = le32_to_cpu((*desc.add(i)).length) as usize;
        if desc_buf_len == 0 { return -EINVAL; }
        if desc_buf_len > buf_len {
            desc_buf_len = buf_len;
            (*desc.add(i)).length = cpu_to_le32(desc_buf_len as u32);
            buf_len = 0;
        }
        credits_needed += smbdirect_connection_calc_rw_credits(sc, desc_buf as *const _, desc_buf_len);
        desc_buf = desc_buf.add(desc_buf_len);
        buf_len -= desc_buf_len;
        desc_num += 1;
        i += 1;
    }

    smbdirect_log_rdma_rw(sc, SMBDIRECT_LOG_INFO, "RDMA %s, len %zu, needed credits %d\n", str_read_write(is_read), buf_len, credits_needed);
    ret = smbdirect_connection_wait_for_rw_credits(sc, credits_needed);
    if ret < 0 { return ret; }

    desc_buf = buf as *mut u8;
    i = 0;
    while i < desc_num {
        let page_count: usize;
        msg = kzalloc_flex!(smbdirect_rw_io, sg_list, SG_CHUNK_SIZE, (*sc).rw_io.mem.gfp_mask);
        if msg.is_null() { ret = -ENOMEM; goto out; }
        desc_buf_len = le32_to_cpu((*desc.add(i)).length) as usize;
        page_count = smbdirect_get_buf_page_count(desc_buf as *const _, desc_buf_len);
        (*msg).socket = sc;
        (*msg).cqe.done = if is_read { Some(smbdirect_connection_rdma_read_done) } else { Some(smbdirect_connection_rdma_write_done) };
        (*msg).completion = &completion;
        (*msg).sgt.sgl = &mut (*msg).sg_list[0];
        ret = sg_alloc_table_chained(&mut (*msg).sgt, page_count, (*msg).sg_list.as_mut_ptr(), SG_CHUNK_SIZE);
        if ret != 0 { ret = -ENOMEM; goto free_msg; }
        ret = smbdirect_connection_rdma_get_sg_list(desc_buf as *mut _, desc_buf_len, (*msg).sgt.sgl, (*msg).sgt.orig_nents);
        if ret < 0 { goto free_table; }
        ret = rdma_rw_ctx_init(&mut (*msg).rdma_ctx, (*sc).ib.qp, (*sc).ib.qp.port, (*msg).sgt.sgl, page_count, 0, le64_to_cpu((*desc.add(i)).offset), le32_to_cpu((*desc.add(i)).token), direction);
        if ret < 0 { pr_err!("failed to init rdma_rw_ctx: %d\n", ret); goto free_table; }
        list_add_tail(&mut (*msg).list, &mut msg_list);
        desc_buf = desc_buf.add(desc_buf_len);
        i += 1;
    }

    first_wr = core::ptr::null_mut();
    list_for_each_entry_reverse!(msg, &msg_list, list, {
        first_wr = rdma_rw_ctx_wrs(&mut (*msg).rdma_ctx, (*sc).ib.qp, (*sc).ib.qp.port, &mut (*msg).cqe, first_wr);
    });
    ret = ib_post_send((*sc).ib.qp, first_wr, core::ptr::null_mut());
    if ret != 0 { pr_err!("failed to post send wr for RDMA R/W: %d\n", ret); goto out; }
    msg = list_last_entry!(&msg_list, smbdirect_rw_io, list);
    wait_for_completion(&completion);
    ret = (*msg).error;
out:
    list_for_each_entry_safe!(msg, next_msg, &mut msg_list, list, {
        list_del(&mut (*msg).list);
        smbdirect_connection_rw_io_free(msg, direction);
    });
    atomic_add(credits_needed, &mut (*sc).rw_io.credits.count);
    wake_up(&mut (*sc).rw_io.credits.wait_queue);
    return ret;
free_table:
    sg_free_table_chained(&mut (*msg).sgt, SG_CHUNK_SIZE);
free_msg:
    kfree(msg);
    goto out;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
