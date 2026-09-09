// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2014 - 2022 Intel Corporation */

// Translated from qat_bl.c. Kernel types, constants, macros, and functions
// referenced below are supplied by the surrounding Linux/Rust bindings.

pub unsafe fn qat_bl_free_bufl(
    accel_dev: *mut adf_accel_dev,
    buf: *mut qat_request_buffs,
) {
    let dev = &mut GET_DEV(accel_dev);
    let bl = (*buf).bl;
    let blout = (*buf).blout;
    let blp = (*buf).blp;
    let blpout = (*buf).bloutp;
    let sz = (*buf).sz;
    let sz_out = (*buf).sz_out;
    let bl_dma_dir: i32;
    let mut i: i32;

    bl_dma_dir = if blp != blpout { DMA_TO_DEVICE } else { DMA_BIDIRECTIONAL };

    i = 0;
    while i < (*bl).num_bufs {
        dma_unmap_single(dev, (*bl).buffers[i as usize].addr,
                         (*bl).buffers[i as usize].len, bl_dma_dir);
        i += 1;
    }

    dma_unmap_single(dev, blp, sz, DMA_TO_DEVICE);

    if !(*buf).sgl_src_valid {
        kfree(bl as *mut core::ffi::c_void);
    }

    if blp != blpout {
        i = 0;
        while i < (*blout).num_mapped_bufs {
            dma_unmap_single(dev, (*blout).buffers[i as usize].addr,
                             (*blout).buffers[i as usize].len,
                             DMA_BIDIRECTIONAL);
            i += 1;
        }
        dma_unmap_single(dev, blpout, sz_out, DMA_TO_DEVICE);

        if !(*buf).sgl_dst_valid {
            kfree(blout as *mut core::ffi::c_void);
        }
    }
}

unsafe fn __qat_bl_sgl_to_bufl(
    accel_dev: *mut adf_accel_dev,
    sgl: *mut scatterlist,
    sglout: *mut scatterlist,
    buf: *mut qat_request_buffs,
    extra_dst_buff: dma_addr_t,
    sz_extra_dst_buff: usize,
    sskip: u32,
    dskip: u32,
    flags: gfp_t,
) -> i32 {
    let dev = &mut GET_DEV(accel_dev);
    let mut i: i32;
    let mut sg_nctr: i32 = 0;
    let mut n = sg_nents(sgl);
    let mut bufl: *mut qat_alg_buf_list;
    let mut buflout: *mut qat_alg_buf_list = core::ptr::null_mut();
    let mut blp: dma_addr_t = DMA_MAPPING_ERROR;
    let mut bloutp: dma_addr_t = DMA_MAPPING_ERROR;
    let sg: *mut scatterlist;
    let mut sz_out: usize;
    let sz = struct_size_buf_list(n);
    let node = dev_to_node(&mut GET_DEV(accel_dev));
    let mut left: u32;
    let bufl_dma_dir: i32;

    if unlikely(n == 0) { return -EINVAL; }

    (*buf).sgl_src_valid = false;
    (*buf).sgl_dst_valid = false;

    if n > QAT_MAX_BUFF_DESC {
        bufl = kzalloc_node(sz, flags, node) as *mut qat_alg_buf_list;
        if unlikely(bufl.is_null()) { return -ENOMEM; }
    } else {
        bufl = container_of_sgl_src(&mut (*buf).sgl_src.sgl_hdr);
        core::ptr::write_bytes(bufl, 0, 1);
        (*buf).sgl_src_valid = true;
    }

    bufl_dma_dir = if sgl != sglout { DMA_TO_DEVICE } else { DMA_BIDIRECTIONAL };

    i = 0;
    while i < n {
        (*bufl).buffers[i as usize].addr = DMA_MAPPING_ERROR;
        i += 1;
    }

    left = sskip;
    for_each_sg(sgl, sg, n, i) {
        let y = sg_nctr;
        if (*sg).length == 0 { continue; }
        if left >= (*sg).length { left -= (*sg).length; continue; }
        (*bufl).buffers[y as usize].addr = dma_map_single(
            dev, sg_virt(sg).add(left as usize), (*sg).length - left, bufl_dma_dir);
        (*bufl).buffers[y as usize].len = (*sg).length;
        if unlikely(dma_mapping_error(dev, (*bufl).buffers[y as usize].addr)) { goto_err_in!(); }
        sg_nctr += 1;
        if left != 0 { (*bufl).buffers[y as usize].len -= left; left = 0; }
    }
    (*bufl).num_bufs = sg_nctr;
    blp = dma_map_single(dev, bufl, sz, DMA_TO_DEVICE);
    if unlikely(dma_mapping_error(dev, blp)) { goto_err_in!(); }
    (*buf).bl = bufl;
    (*buf).blp = blp;
    (*buf).sz = sz;

    if sgl != sglout {
        let extra_buff = if extra_dst_buff != 0 { 1 } else { 0 };
        let n_sglout = sg_nents(sglout);
        n = n_sglout + extra_buff;
        sz_out = struct_size_buf_list(n);
        left = dskip;
        sg_nctr = 0;
        if n > QAT_MAX_BUFF_DESC {
            buflout = kzalloc_node(sz_out, flags, node) as *mut qat_alg_buf_list;
            if unlikely(buflout.is_null()) { goto_err_in!(); }
        } else {
            buflout = container_of_sgl_dst(&mut (*buf).sgl_dst.sgl_hdr);
            core::ptr::write_bytes(buflout, 0, 1);
            (*buf).sgl_dst_valid = true;
        }
        let buffers = (*buflout).buffers;
        i = 0;
        while i < n { buffers[i as usize].addr = DMA_MAPPING_ERROR; i += 1; }
        for_each_sg(sglout, sg, n_sglout, i) {
            let y = sg_nctr;
            if (*sg).length == 0 { continue; }
            if left >= (*sg).length { left -= (*sg).length; continue; }
            buffers[y as usize].addr = dma_map_single(dev, sg_virt(sg).add(left as usize), (*sg).length - left, DMA_BIDIRECTIONAL);
            if unlikely(dma_mapping_error(dev, buffers[y as usize].addr)) { goto_err_out!(); }
            buffers[y as usize].len = (*sg).length;
            sg_nctr += 1;
            if left != 0 { buffers[y as usize].len -= left; left = 0; }
        }
        if extra_buff != 0 { buffers[sg_nctr as usize].addr = extra_dst_buff; buffers[sg_nctr as usize].len = sz_extra_dst_buff; }
        (*buflout).num_bufs = sg_nctr + extra_buff;
        (*buflout).num_mapped_bufs = sg_nctr;
        bloutp = dma_map_single(dev, buflout, sz_out, DMA_TO_DEVICE);
        if unlikely(dma_mapping_error(dev, bloutp)) { goto_err_out!(); }
        (*buf).blout = buflout; (*buf).bloutp = bloutp; (*buf).sz_out = sz_out;
    } else { (*buf).bloutp = (*buf).blp; (*buf).sz_out = 0; }
    return 0;

    // The labels below preserve the C cleanup branches; binding-specific
    // implementations may lower these to ordinary block control flow.
    goto_err_out!();
    goto_err_in!();
}

pub unsafe fn qat_bl_sgl_to_bufl(
    accel_dev: *mut adf_accel_dev, sgl: *mut scatterlist, sglout: *mut scatterlist,
    buf: *mut qat_request_buffs, params: *mut qat_sgl_to_bufl_params, flags: gfp_t,
) -> i32 {
    let mut extra_dst_buff: dma_addr_t = 0;
    let mut sz_extra_dst_buff: usize = 0;
    let mut sskip: u32 = 0;
    let mut dskip: u32 = 0;
    if !params.is_null() {
        extra_dst_buff = (*params).extra_dst_buff;
        sz_extra_dst_buff = (*params).sz_extra_dst_buff;
        sskip = (*params).sskip;
        dskip = (*params).dskip;
    }
    __qat_bl_sgl_to_bufl(accel_dev, sgl, sglout, buf, extra_dst_buff,
                         sz_extra_dst_buff, sskip, dskip, flags)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
