// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2020 Oracle. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel/RDMA translation.

pub unsafe fn pcl_free(pcl: *mut svc_rdma_pcl) {
    while !list_empty(unsafe { &(*pcl).cl_chunks }) {
        let chunk = pcl_first_chunk(pcl);
        list_del(unsafe { &mut (*chunk).ch_list });
        kfree(chunk);
    }
}

unsafe fn pcl_alloc_chunk(segcount: u32, position: u32) -> *mut svc_rdma_chunk {
    let chunk = kmalloc_flex(segcount);
    if chunk.is_null() {
        return core::ptr::null_mut();
    }

    (*chunk).ch_position = position;
    (*chunk).ch_length = 0;
    (*chunk).ch_payload_length = 0;
    (*chunk).ch_segcount = 0;
    chunk
}

unsafe fn pcl_lookup_position(
    pcl: *mut svc_rdma_pcl,
    position: u32,
) -> *mut svc_rdma_chunk {
    let mut pos: *mut svc_rdma_chunk = core::ptr::null_mut();
    pcl_for_each_chunk(pos, pcl) {
        if (*pos).ch_position == position {
            return pos;
        }
    }
    core::ptr::null_mut()
}

unsafe fn pcl_insert_position(pcl: *mut svc_rdma_pcl, chunk: *mut svc_rdma_chunk) {
    let mut pos: *mut svc_rdma_chunk = core::ptr::null_mut();
    pcl_for_each_chunk(pos, pcl) {
        if (*pos).ch_position > (*chunk).ch_position {
            break;
        }
    }
    __list_add(
        &mut (*chunk).ch_list,
        (*pos).ch_list.prev,
        &mut (*pos).ch_list,
    );
    (*pcl).cl_count += 1;
}

unsafe fn pcl_set_read_segment(
    rctxt: *const svc_rdma_recv_ctxt,
    chunk: *mut svc_rdma_chunk,
    handle: u32,
    length: u32,
    offset: u64,
) {
    let segment = &mut (*chunk).ch_segments[(*chunk).ch_segcount as usize];
    segment.rs_handle = handle;
    segment.rs_length = length;
    segment.rs_offset = offset;

    trace_svcrdma_decode_rseg(&(*rctxt).rc_cid, chunk, segment);

    (*chunk).ch_length += length;
    (*chunk).ch_segcount += 1;
}

pub unsafe fn pcl_alloc_call(
    rctxt: *mut svc_rdma_recv_ctxt,
    mut p: *mut be32,
) -> bool {
    let pcl = &mut (*rctxt).rc_call_pcl;
    let segcount = pcl.cl_count;

    pcl.cl_count = 0;
    for _ in 0..segcount {
        let mut position = 0u32;
        let mut handle = 0u32;
        let mut length = 0u32;
        let mut offset = 0u64;

        p = p.add(1);
        p = xdr_decode_read_segment(p, &mut position, &mut handle, &mut length, &mut offset);
        if position != 0 {
            continue;
        }

        let chunk;
        if pcl_is_empty(pcl) {
            chunk = pcl_alloc_chunk(segcount, position);
            if chunk.is_null() {
                return false;
            }
            pcl_insert_position(pcl, chunk);
        } else {
            chunk = list_first_entry(&pcl.cl_chunks);
        }

        pcl_set_read_segment(rctxt, chunk, handle, length, offset);
    }

    true
}

pub unsafe fn pcl_alloc_read(
    rctxt: *mut svc_rdma_recv_ctxt,
    mut p: *mut be32,
) -> bool {
    let pcl = &mut (*rctxt).rc_read_pcl;
    let segcount = pcl.cl_count;

    pcl.cl_count = 0;
    for _ in 0..segcount {
        let mut position = 0u32;
        let mut handle = 0u32;
        let mut length = 0u32;
        let mut offset = 0u64;

        p = p.add(1);
        p = xdr_decode_read_segment(p, &mut position, &mut handle, &mut length, &mut offset);
        if position == 0 {
            continue;
        }

        let mut chunk = pcl_lookup_position(pcl, position);
        if chunk.is_null() {
            chunk = pcl_alloc_chunk(segcount, position);
            if chunk.is_null() {
                return false;
            }
            pcl_insert_position(pcl, chunk);
        }

        pcl_set_read_segment(rctxt, chunk, handle, length, offset);
    }

    true
}

pub unsafe fn pcl_alloc_write(
    rctxt: *mut svc_rdma_recv_ctxt,
    pcl: *mut svc_rdma_pcl,
    mut p: *mut be32,
) -> bool {
    for _ in 0..(*pcl).cl_count {
        p = p.add(1);
        let segcount = be32_to_cpup(p);
        p = p.add(1);

        let chunk = pcl_alloc_chunk(segcount, 0);
        if chunk.is_null() {
            return false;
        }

        for j in 0..segcount {
            let segment = &mut (*chunk).ch_segments[j as usize];
            p = xdr_decode_rdma_segment(
                p,
                &mut segment.rs_handle,
                &mut segment.rs_length,
                &mut segment.rs_offset,
            );
            trace_svcrdma_decode_wseg(&(*rctxt).rc_cid, chunk, j);

            (*chunk).ch_length += segment.rs_length;
            (*chunk).ch_segcount += 1;
        }
        list_add_tail(&mut (*chunk).ch_list, &mut (*pcl).cl_chunks);
    }
    true
}

pub unsafe fn pcl_check_read_chunk_positions(
    rctxt: *mut svc_rdma_recv_ctxt,
    inline_len: u32,
) -> bool {
    let max_len = (*rctxt).rc_maxpages << PAGE_SHIFT;
    let bound;

    if !pcl_is_empty(&mut (*rctxt).rc_call_pcl) {
        let chunk = pcl_first_chunk(&mut (*rctxt).rc_call_pcl);
        if (*chunk).ch_length > max_len {
            return false;
        }
        bound = (*chunk).ch_length;
    } else {
        bound = inline_len;
    }

    if pcl_is_empty(&mut (*rctxt).rc_read_pcl) {
        return true;
    }

    let mut total_read = 0u32;
    let mut chunk: *mut svc_rdma_chunk = core::ptr::null_mut();
    pcl_for_each_chunk(chunk, &mut (*rctxt).rc_read_pcl) {
        if (*chunk).ch_position - total_read > bound || (*chunk).ch_length > max_len {
            return false;
        }

        let next = pcl_next_chunk(&mut (*rctxt).rc_read_pcl, chunk);
        if next.is_null() {
            break;
        }
        if (*chunk).ch_position + (*chunk).ch_length > (*next).ch_position {
            return false;
        }
        total_read += (*chunk).ch_length;
    }

    true
}

unsafe fn pcl_process_region(
    xdr: *const xdr_buf,
    offset: u32,
    length: u32,
    actor: unsafe fn(*const xdr_buf, *mut core::ffi::c_void) -> i32,
    data: *mut core::ffi::c_void,
) -> i32 {
    let mut subbuf = core::mem::MaybeUninit::<xdr_buf>::uninit();
    if length == 0 {
        return 0;
    }
    if xdr_buf_subsegment(xdr, subbuf.as_mut_ptr(), offset, length) != 0 {
        return -EMSGSIZE;
    }
    actor(subbuf.as_ptr(), data)
}

pub unsafe fn pcl_process_nonpayloads(
    pcl: *const svc_rdma_pcl,
    xdr: *const xdr_buf,
    actor: unsafe fn(*const xdr_buf, *mut core::ffi::c_void) -> i32,
    data: *mut core::ffi::c_void,
) -> i32 {
    let mut chunk = pcl_first_chunk(pcl as *mut svc_rdma_pcl);
    if chunk.is_null() || (*chunk).ch_payload_length == 0 {
        return actor(xdr, data);
    }

    let mut ret = pcl_process_region(xdr, 0, (*chunk).ch_position, actor, data);
    if ret < 0 {
        return ret;
    }

    loop {
        let next = pcl_next_chunk(pcl as *mut svc_rdma_pcl, chunk);
        if next.is_null() || (*next).ch_payload_length == 0 {
            break;
        }
        let start = pcl_chunk_end_offset(chunk);
        ret = pcl_process_region(xdr, start, (*next).ch_position - start, actor, data);
        if ret < 0 {
            return ret;
        }
        chunk = next;
    }

    let start = pcl_chunk_end_offset(chunk);
    ret = pcl_process_region(xdr, start, (*xdr).len - start, actor, data);
    if ret < 0 {
        return ret;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
