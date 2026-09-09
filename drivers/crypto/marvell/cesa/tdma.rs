// SPDX-License-Identifier: GPL-2.0-only
/*
 * Provide TDMA helper functions used by cipher and hash algorithm
 * implementations.
 *
 * Author: Boris Brezillon <boris.brezillon@free-electrons.com>
 * Author: Arnaud Ebalard <arno@natisbad.org>
 *
 * This work is based on an initial version written by
 * Sebastian Andrzej Siewior < sebastian at breakpoint dot cc >
 */

// Dependencies supplied by the surrounding CESA implementation are intentionally external.

pub unsafe fn mv_cesa_req_dma_iter_next_transfer(
    iter: *mut mv_cesa_dma_iter,
    sgiter: *mut mv_cesa_sg_dma_iter,
    len: c_uint,
) -> bool {
    if (*sgiter).sg.is_null() { return false; }
    (*sgiter).op_offset += len;
    (*sgiter).offset += len;
    if (*sgiter).offset == sg_dma_len((*sgiter).sg) {
        if sg_is_last((*sgiter).sg) { return false; }
        (*sgiter).offset = 0;
        (*sgiter).sg = sg_next((*sgiter).sg);
    }
    if (*sgiter).op_offset == (*iter).op_len { return false; }
    true
}

pub unsafe fn mv_cesa_dma_step(dreq: *mut mv_cesa_req) {
    let engine = (*dreq).engine;
    spin_lock_bh(&mut (*engine).lock);
    if (*engine).chain_sw.first == (*dreq).chain.first {
        (*engine).chain_sw.first = core::ptr::null_mut();
        (*engine).chain_sw.last = core::ptr::null_mut();
    }
    (*engine).chain_hw.first = (*dreq).chain.first;
    (*engine).chain_hw.last = (*dreq).chain.last;
    spin_unlock_bh(&mut (*engine).lock);
    writel_relaxed(0, (*engine).regs.add(CESA_SA_CFG as usize));
    mv_cesa_set_int_mask(engine, CESA_SA_INT_ACC0_IDMA_DONE);
    writel_relaxed(CESA_TDMA_DST_BURST_128B | CESA_TDMA_SRC_BURST_128B |
                   CESA_TDMA_NO_BYTE_SWAP | CESA_TDMA_EN,
                   (*engine).regs.add(CESA_TDMA_CONTROL as usize));
    writel_relaxed(CESA_SA_CFG_ACT_CH0_IDMA | CESA_SA_CFG_MULTI_PKT |
                   CESA_SA_CFG_CH0_W_IDMA | CESA_SA_CFG_PARA_DIS,
                   (*engine).regs.add(CESA_SA_CFG as usize));
    writel_relaxed((*(*dreq).chain.first).cur_dma,
                   (*engine).regs.add(CESA_TDMA_NEXT_ADDR as usize));
    WARN_ON(readl((*engine).regs.add(CESA_SA_CMD as usize)) & CESA_SA_CMD_EN_CESA_SA_ACCL0);
    writel(CESA_SA_CMD_EN_CESA_SA_ACCL0, (*engine).regs.add(CESA_SA_CMD as usize));
}

pub unsafe fn mv_cesa_dma_cleanup(dreq: *mut mv_cesa_req) {
    let mut tdma = (*dreq).chain.first;
    while !tdma.is_null() {
        let old_tdma = tdma;
        let ty = (*tdma).flags & CESA_TDMA_TYPE_MSK;
        if ty == CESA_TDMA_OP {
            dma_pool_free((*cesa_dev).dma.op_pool, (*tdma).op, le32_to_cpu((*tdma).src));
        }
        tdma = (*tdma).next;
        dma_pool_free((*cesa_dev).dma.tdma_desc_pool, old_tdma, (*old_tdma).cur_dma);
    }
    (*dreq).chain.first = core::ptr::null_mut();
    (*dreq).chain.last = core::ptr::null_mut();
}

pub unsafe fn mv_cesa_dma_prepare(dreq: *mut mv_cesa_req, engine: *mut mv_cesa_engine) {
    let mut tdma = (*dreq).chain.first;
    while !tdma.is_null() {
        if (*tdma).flags & CESA_TDMA_DST_IN_SRAM != 0 { (*tdma).dst = cpu_to_le32((*tdma).dst_dma + (*engine).sram_dma); }
        if (*tdma).flags & CESA_TDMA_SRC_IN_SRAM != 0 { (*tdma).src = cpu_to_le32((*tdma).src_dma + (*engine).sram_dma); }
        if (*tdma).flags & CESA_TDMA_TYPE_MSK == CESA_TDMA_OP { mv_cesa_adjust_op(engine, (*tdma).op); }
        tdma = (*tdma).next;
    }
}

pub unsafe fn mv_cesa_tdma_chain(engine: *mut mv_cesa_engine, dreq: *mut mv_cesa_req) {
    let mut last = (*engine).chain_sw.last;
    if last.is_null() || (*(*dreq).chain.first).flags & CESA_TDMA_SET_STATE != 0 {
        (*engine).chain_sw.first = (*dreq).chain.first;
    } else {
        (*last).next = (*dreq).chain.first;
        (*last).next_dma = cpu_to_le32((*(*dreq).chain.first).cur_dma);
    }
    last = (*dreq).chain.last;
    (*engine).chain_sw.last = last;
    if (*last).flags & CESA_TDMA_BREAK_CHAIN != 0 {
        (*engine).chain_sw.first = core::ptr::null_mut();
        (*engine).chain_sw.last = core::ptr::null_mut();
    }
}

pub unsafe fn mv_cesa_dma_add_desc(chain: *mut mv_cesa_tdma_chain, flags: gfp_t) -> *mut mv_cesa_tdma_desc {
    let mut dma_handle: dma_addr_t = 0;
    let new_tdma = dma_pool_zalloc((*cesa_dev).dma.tdma_desc_pool, flags, &mut dma_handle);
    if new_tdma.is_null() { return ERR_PTR(-ENOMEM); }
    (*new_tdma).cur_dma = dma_handle;
    if !(*chain).last.is_null() {
        (*(*chain).last).next_dma = cpu_to_le32(dma_handle);
        (*(*chain).last).next = new_tdma;
    } else { (*chain).first = new_tdma; }
    (*chain).last = new_tdma;
    new_tdma
}

pub unsafe fn mv_cesa_dma_add_data_transfer(chain: *mut mv_cesa_tdma_chain, dst: dma_addr_t, src: dma_addr_t, size: u32, mut flags: u32, gfp_flags: gfp_t) -> i32 {
    let tdma = mv_cesa_dma_add_desc(chain, gfp_flags);
    if IS_ERR(tdma) { return PTR_ERR(tdma); }
    (*tdma).byte_cnt = cpu_to_le32(size | BIT(31));
    (*tdma).src_dma = src; (*tdma).dst_dma = dst;
    flags &= CESA_TDMA_DST_IN_SRAM | CESA_TDMA_SRC_IN_SRAM;
    (*tdma).flags = flags | CESA_TDMA_DATA;
    0
}

// The remaining helpers retain the same operations and rely on declarations from cesa.h.
pub unsafe fn mv_cesa_dma_add_dummy_launch(chain: *mut mv_cesa_tdma_chain, flags: gfp_t) -> i32 { PTR_ERR_OR_ZERO(mv_cesa_dma_add_desc(chain, flags)) }
pub unsafe fn mv_cesa_dma_add_dummy_end(chain: *mut mv_cesa_tdma_chain, flags: gfp_t) -> i32 { let t=mv_cesa_dma_add_desc(chain,flags); if IS_ERR(t){return PTR_ERR(t)}; (*t).byte_cnt=cpu_to_le32(BIT(31)); 0 }

pub unsafe fn mv_cesa_dma_add_result_op(chain: *mut mv_cesa_tdma_chain, src: dma_addr_t, size: u32, mut flags: u32, gfp_flags: gfp_t) -> i32 {
    let tdma = mv_cesa_dma_add_desc(chain, gfp_flags); if IS_ERR(tdma) { return PTR_ERR(tdma); }
    let mut op_desc = (*chain).first;
    while !op_desc.is_null() && (*op_desc).flags & CESA_TDMA_TYPE_MSK != CESA_TDMA_OP { op_desc = (*op_desc).next; }
    if op_desc.is_null() { return -EIO; }
    (*tdma).byte_cnt=cpu_to_le32(size|BIT(31)); (*tdma).src_dma=src; (*tdma).dst_dma=(*op_desc).src_dma; (*tdma).op=(*op_desc).op;
    flags &= CESA_TDMA_DST_IN_SRAM|CESA_TDMA_SRC_IN_SRAM; (*tdma).flags=flags|CESA_TDMA_RESULT; 0
}

pub unsafe fn mv_cesa_dma_add_op(chain: *mut mv_cesa_tdma_chain, templ: *const mv_cesa_op_ctx, skip_ctx: bool, flags: gfp_t) -> *mut mv_cesa_op_ctx {
    let tdma=mv_cesa_dma_add_desc(chain,flags); if IS_ERR(tdma){return ERR_CAST(tdma)};
    let mut dh=0; let op=dma_pool_alloc((*cesa_dev).dma.op_pool,flags,&mut dh); if op.is_null(){return ERR_PTR(-ENOMEM)};
    core::ptr::write(op, core::ptr::read(templ)); let size=if skip_ctx{core::mem::size_of::<(*op).desc>()}else{core::mem::size_of::<mv_cesa_op_ctx>()};
    let t=(*chain).last; (*t).op=op; (*t).byte_cnt=cpu_to_le32(size as u32|BIT(31)); (*t).src=cpu_to_le32(dh); (*t).dst_dma=CESA_SA_CFG_SRAM_OFFSET; (*t).flags=CESA_TDMA_DST_IN_SRAM|CESA_TDMA_OP; op
}

pub unsafe fn mv_cesa_dma_add_op_transfers(chain:*mut mv_cesa_tdma_chain, dma_iter:*mut mv_cesa_dma_iter, sgiter:*mut mv_cesa_sg_dma_iter, gfp:gfp_t)->i32{
 let flags=if (*sgiter).dir==DMA_TO_DEVICE{CESA_TDMA_DST_IN_SRAM}else{CESA_TDMA_SRC_IN_SRAM}; let mut len;
 loop{len=mv_cesa_req_dma_iter_transfer_len(dma_iter,sgiter); let (dst,src)=if (*sgiter).dir==DMA_TO_DEVICE{(CESA_SA_DATA_SRAM_OFFSET+(*sgiter).op_offset,sg_dma_address((*sgiter).sg)+(*sgiter).offset)}else{(sg_dma_address((*sgiter).sg)+(*sgiter).offset,CESA_SA_DATA_SRAM_OFFSET+(*sgiter).op_offset)}; let r=mv_cesa_dma_add_data_transfer(chain,dst,src,len,flags,gfp); if r!=0{return r} if !mv_cesa_req_dma_iter_next_transfer(dma_iter,sgiter,len){break}} 0
}

pub unsafe fn mv_cesa_sg_copy(engine:*mut mv_cesa_engine, sgl:*mut scatterlist, nents:c_uint, sram_off:c_uint, buflen:usize, skip:off_t, to_sram:bool)->usize{
 let mut flags=SG_MITER_ATOMIC; let mut miter=core::mem::zeroed::<sg_mapping_iter>(); if to_sram{flags|=SG_MITER_FROM_SG}else{flags|=SG_MITER_TO_SG}; sg_miter_start(&mut miter,sgl,nents,flags); if !sg_miter_skip(&mut miter,skip){return 0}; let mut offset=0;
 while offset<buflen && sg_miter_next(&mut miter){let len=core::cmp::min(miter.length,buflen-offset); if to_sram{if !(*engine).pool.is_null(){memcpy((*engine).sram_pool.add(sram_off as usize+offset),miter.addr,len)}else{memcpy_toio((*engine).sram.add(sram_off as usize+offset),miter.addr,len)}}else if !(*engine).pool.is_null(){memcpy(miter.addr,(*engine).sram_pool.add(sram_off as usize+offset),len)}else{memcpy_fromio(miter.addr,(*engine).sram.add(sram_off as usize+offset),len)} offset+=len} sg_miter_stop(&mut miter); offset
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
