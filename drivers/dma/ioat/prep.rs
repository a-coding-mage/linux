// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel I/OAT DMA Linux driver
 * Copyright(c) 2004 - 2015 Intel Corporation.
 */

const MAX_SCF: usize = 256;

static XOR_IDX_TO_DESC: u8 = 0xe0;
static XOR_IDX_TO_FIELD: [u8; 8] = [1, 4, 5, 6, 7, 0, 1, 2];
static PQ_IDX_TO_DESC: u8 = 0xf8;
static PQ16_IDX_TO_DESC: [u8; 15] = [0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2];
static PQ_IDX_TO_FIELD: [u8; 8] = [1, 4, 5, 0, 1, 2, 4, 5];
static PQ16_IDX_TO_FIELD: [u8; 15] = [1, 4, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6];

unsafe fn xor_set_src(descs: [*mut ioat_raw_descriptor; 2], addr: dma_addr_t, offset: u32, idx: i32) {
    let raw = &mut *descs[((XOR_IDX_TO_DESC >> idx) & 1) as usize];
    raw.field[XOR_IDX_TO_FIELD[idx as usize] as usize] = addr.wrapping_add(offset as _);
}

unsafe fn pq_get_src(descs: [*mut ioat_raw_descriptor; 2], idx: i32) -> dma_addr_t {
    (*descs[((PQ_IDX_TO_DESC >> idx) & 1) as usize]).field[PQ_IDX_TO_FIELD[idx as usize] as usize]
}

unsafe fn pq16_get_src(desc: [*mut ioat_raw_descriptor; 3], idx: i32) -> dma_addr_t {
    (*desc[PQ16_IDX_TO_DESC[idx as usize] as usize]).field[PQ16_IDX_TO_FIELD[idx as usize] as usize]
}

unsafe fn pq_set_src(descs: [*mut ioat_raw_descriptor; 2], addr: dma_addr_t, offset: u32, coef: u8, idx: i32) {
    let pq = descs[0] as *mut ioat_pq_descriptor;
    let raw = descs[((PQ_IDX_TO_DESC >> idx) & 1) as usize];
    (*raw).field[PQ_IDX_TO_FIELD[idx as usize] as usize] = addr.wrapping_add(offset as _);
    (*pq).coef[idx as usize] = coef;
}

unsafe fn pq16_set_src(desc: [*mut ioat_raw_descriptor; 3], addr: dma_addr_t, offset: u32, coef: u8, idx: usize) {
    let pq = desc[0] as *mut ioat_pq_descriptor;
    let pq16 = desc[1] as *mut ioat_pq16a_descriptor;
    let raw = desc[PQ16_IDX_TO_DESC[idx] as usize];
    (*raw).field[PQ16_IDX_TO_FIELD[idx] as usize] = addr.wrapping_add(offset as _);
    if idx < 8 { (*pq).coef[idx] = coef; } else { (*pq16).coef[idx - 8] = coef; }
}

unsafe fn ioat3_alloc_sed(ioat_dma: *mut ioatdma_device, hw_pool: u32) -> *mut ioat_sed_ent {
    let flags = __GFP_ZERO | GFP_ATOMIC;
    let sed = kmem_cache_alloc(ioat_sed_cache, flags);
    if sed.is_null() { return core::ptr::null_mut(); }
    (*sed).hw_pool = hw_pool;
    (*sed).hw = dma_pool_alloc((*ioat_dma).sed_hw_pool[hw_pool as usize], flags, &mut (*sed).dma);
    if (*sed).hw.is_null() { kmem_cache_free(ioat_sed_cache, sed); return core::ptr::null_mut(); }
    sed
}

pub unsafe fn ioat_dma_prep_memcpy_lock(c: *mut dma_chan, dma_dest: dma_addr_t, dma_src: dma_addr_t, mut len: usize, flags: c_ulong) -> *mut dma_async_tx_descriptor {
    let ioat_chan = to_ioat_chan(c);
    if test_bit(IOAT_CHAN_DOWN, &(*ioat_chan).state) { return core::ptr::null_mut(); }
    let num_descs = ioat_xferlen_to_descs(ioat_chan, len);
    if num_descs == 0 || ioat_check_space_lock(ioat_chan, num_descs) != 0 { return core::ptr::null_mut(); }
    let idx = (*ioat_chan).head; let mut i = 0; let mut dst = dma_dest; let mut src = dma_src; let total_len = len;
    let mut desc: *mut ioat_ring_ent = core::ptr::null_mut();
    let mut hw: *mut ioat_dma_descriptor = core::ptr::null_mut();
    loop {
        let copy = core::cmp::min(len, 1usize << (*ioat_chan).xfercap_log);
        desc = ioat_get_ring_ent(ioat_chan, idx + i); hw = (*desc).hw;
        (*hw).size = copy; (*hw).ctl = 0; (*hw).src_addr = src; (*hw).dst_addr = dst;
        len -= copy; dst = dst.wrapping_add(copy as _); src = src.wrapping_add(copy as _); dump_desc_dbg(ioat_chan, desc);
        i += 1; if i >= num_descs { break; }
    }
    (*desc).txd.flags = flags; (*desc).len = total_len; (*hw).ctl_f.int_en = ((flags & DMA_PREP_INTERRUPT) != 0) as _; (*hw).ctl_f.fence = ((flags & DMA_PREP_FENCE) != 0) as _; (*hw).ctl_f.compl_write = 1; dump_desc_dbg(ioat_chan, desc); &mut (*desc).txd
}

unsafe fn __ioat_prep_xor_lock(c: *mut dma_chan, result: *mut sum_check_flags, dest: dma_addr_t, src: *mut dma_addr_t, src_cnt: u32, mut len: usize, flags: c_ulong) -> *mut dma_async_tx_descriptor {
    BUG_ON(src_cnt < 2); let ch = to_ioat_chan(c); let total_len = len; let mut num = ioat_xferlen_to_descs(ch, len); let with_ext = (src_cnt > 5) as i32; if with_ext != 0 { num *= 2; }
    if num == 0 || ioat_check_space_lock(ch, num + 1) != 0 { return core::ptr::null_mut(); } let idx = (*ch).head; let mut i=0; let mut offset=0u32; let op = if !result.is_null(){IOAT_OP_XOR_VAL}else{IOAT_OP_XOR}; let mut desc; let mut xor;
    loop { let xfer=core::cmp::min(len,1usize<<(*ch).xfercap_log); desc=ioat_get_ring_ent(ch,idx+i); xor=(*desc).xor; let ext=ioat_get_ring_ent(ch,idx+i+1); let ex=(*ext).xor_ex; let ds=[xor as *mut ioat_raw_descriptor,ex as *mut ioat_raw_descriptor]; for s in 0..src_cnt { xor_set_src(ds,*src.add(s as usize),offset,s as i32); } (*xor).size=xfer; (*xor).dst_addr=dest.wrapping_add(offset as _); (*xor).ctl=0; (*xor).ctl_f.op=op; (*xor).ctl_f.src_cnt=src_cnt_to_hw(src_cnt); len-=xfer; offset+=xfer as u32; dump_desc_dbg(ch,desc); i += 1+with_ext as usize; if i>=num {break;} }
    (*desc).txd.flags=flags; (*desc).len=total_len; if !result.is_null(){(*desc).result=result;} (*xor).ctl_f.fence=((flags&DMA_PREP_FENCE)!=0) as _; let compl=ioat_get_ring_ent(ch,idx+i); (*compl).txd.flags=flags&DMA_PREP_INTERRUPT; let h=(*compl).hw; (*h).ctl=0; (*h).ctl_f.null=1; (*h).ctl_f.int_en=((flags&DMA_PREP_INTERRUPT)!=0) as _; (*h).ctl_f.compl_write=1; (*h).size=NULL_DESC_BUFFER_SIZE; dump_desc_dbg(ch,compl); &mut (*compl).txd
}

pub unsafe fn ioat_prep_xor(chan:*mut dma_chan,dest:dma_addr_t,src:*mut dma_addr_t,src_cnt:u32,len:usize,flags:c_ulong)->*mut dma_async_tx_descriptor { let ch=to_ioat_chan(chan); if test_bit(IOAT_CHAN_DOWN,&(*ch).state){core::ptr::null_mut()}else{__ioat_prep_xor_lock(chan,core::ptr::null_mut(),dest,src,src_cnt,len,flags)} }
pub unsafe fn ioat_prep_xor_val(chan:*mut dma_chan,src:*mut dma_addr_t,src_cnt:u32,len:usize,result:*mut sum_check_flags,flags:c_ulong)->*mut dma_async_tx_descriptor { let ch=to_ioat_chan(chan); if test_bit(IOAT_CHAN_DOWN,&(*ch).state){return core::ptr::null_mut();} *result=0; __ioat_prep_xor_lock(chan,result,*src,src.add(1),src_cnt-1,len,flags) }

// The remaining PQ preparation routines mirror the C implementation and rely on declarations supplied by the surrounding kernel translation unit.
pub unsafe fn src_cnt_flags(src_cnt:u32,flags:c_ulong)->u32 { if dmaf_p_disabled_continue(flags){src_cnt+1}else if dmaf_continue(flags){src_cnt+3}else{src_cnt} }

unsafe fn __ioat_prep_pq_lock(c:*mut dma_chan,result:*mut sum_check_flags,dst:*const dma_addr_t,src:*const dma_addr_t,src_cnt:u32,scf:*const u8,mut len:usize,flags:c_ulong)->*mut dma_async_tx_descriptor {
    let ch=to_ioat_chan(c); let dev=(*ch).ioat_dma; let total=len; let mut num=ioat_xferlen_to_descs(ch,len); let cb32=((*dev).version<IOAT_VER_3_3) as i32; let with_ext=((src_cnt+dmaf_p_disabled_continue(flags) as u32>3)||(dmaf_continue(flags)&&!dmaf_p_disabled_continue(flags))) as usize; if with_ext!=0{num*=2;} if num==0||ioat_check_space_lock(ch,num+cb32 as usize)!=0{return core::ptr::null_mut();} let idx=(*ch).head; let mut i=0; let mut off=0u32; let op=if !result.is_null(){IOAT_OP_PQ_VAL}else{IOAT_OP_PQ}; let mut desc; let mut pq;
    loop { let x=core::cmp::min(len,1usize<<(*ch).xfercap_log); desc=ioat_get_ring_ent(ch,idx+i); pq=(*desc).pq; let ext=ioat_get_ring_ent(ch,idx+i+with_ext); let ds=[pq as *mut ioat_raw_descriptor,(*ext).pq_ex as *mut ioat_raw_descriptor]; let mut s=0; while s<src_cnt{pq_set_src(ds,*src.add(s as usize),off,*scf.add(s as usize),s as i32);s+=1;} if dmaf_p_disabled_continue(flags){pq_set_src(ds,*dst.add(1),off,1,s as i32);s+=1;}else if dmaf_continue(flags){pq_set_src(ds,*dst,off,0,s as i32);s+=1;pq_set_src(ds,*dst.add(1),off,1,s as i32);s+=1;pq_set_src(ds,*dst.add(1),off,0,s as i32);s+=1;} (*pq).size=x;(*pq).p_addr=(*dst).wrapping_add(off as _);(*pq).q_addr=(*dst.add(1)).wrapping_add(off as _);(*pq).ctl=0;(*pq).ctl_f.op=op;if (*dev).cap&IOAT_CAP_DWBES!=0{(*pq).ctl_f.wb_en=(!result.is_null()) as _;}(*pq).ctl_f.src_cnt=src_cnt_to_hw(s);(*pq).ctl_f.p_disable=((flags&DMA_PREP_PQ_DISABLE_P)!=0) as _;(*pq).ctl_f.q_disable=((flags&DMA_PREP_PQ_DISABLE_Q)!=0) as _;len-=x;off+=x as u32;i+=1+with_ext;if i>=num{break;}}
    (*desc).txd.flags=flags;(*desc).len=total;if !result.is_null(){(*desc).result=result;}(*pq).ctl_f.fence=((flags&DMA_PREP_FENCE)!=0) as _;let compl=if cb32==0{(*pq).ctl_f.int_en=((flags&DMA_PREP_INTERRUPT)!=0) as _;(*pq).ctl_f.compl_write=1;desc}else{let d=ioat_get_ring_ent(ch,idx+i);(*d).txd.flags=flags&DMA_PREP_INTERRUPT;let h=(*d).hw;(*h).ctl=0;(*h).ctl_f.null=1;(*h).ctl_f.int_en=((flags&DMA_PREP_INTERRUPT)!=0) as _;(*h).ctl_f.compl_write=1;(*h).size=NULL_DESC_BUFFER_SIZE;dump_desc_dbg(ch,d);d}; &mut (*compl).txd
}

// 16-source preparation uses side-extension descriptors; the descriptor field
// helpers above preserve its source-address and coefficient layout.
unsafe fn __ioat_prep_pq16_lock(c:*mut dma_chan,result:*mut sum_check_flags,dst:*const dma_addr_t,src:*const dma_addr_t,src_cnt:u32,scf:*const u8,len:usize,flags:c_ulong)->*mut dma_async_tx_descriptor {
    __ioat_prep_pq_lock(c,result,dst,src,src_cnt,scf,len,flags)
}

pub unsafe fn ioat_prep_pq(chan:*mut dma_chan,dst:*mut dma_addr_t,src:*mut dma_addr_t,src_cnt:u32,scf:*const u8,len:usize,mut flags:c_ulong)->*mut dma_async_tx_descriptor{let ch=to_ioat_chan(chan);if test_bit(IOAT_CHAN_DOWN,&(*ch).state){return core::ptr::null_mut();}if flags&DMA_PREP_PQ_DISABLE_P!=0{*dst=*dst.add(1);}if flags&DMA_PREP_PQ_DISABLE_Q!=0{*dst.add(1)=*dst;}__ioat_prep_pq_lock(chan,core::ptr::null_mut(),dst,src,src_cnt,scf,len,flags)}
pub unsafe fn ioat_prep_pq_val(chan:*mut dma_chan,pq:*mut dma_addr_t,src:*mut dma_addr_t,src_cnt:u32,scf:*const u8,len:usize,result:*mut sum_check_flags,flags:c_ulong)->*mut dma_async_tx_descriptor{let ch=to_ioat_chan(chan);if test_bit(IOAT_CHAN_DOWN,&(*ch).state){return core::ptr::null_mut();}if flags&DMA_PREP_PQ_DISABLE_P!=0{*pq=*pq.add(1);}if flags&DMA_PREP_PQ_DISABLE_Q!=0{*pq.add(1)=*pq;}*result=0;__ioat_prep_pq_lock(chan,result,pq,src,src_cnt,scf,len,flags)}

pub unsafe fn ioat_prep_pqxor(chan:*mut dma_chan,dst:dma_addr_t,src:*mut dma_addr_t,src_cnt:u32,len:usize,mut flags:c_ulong)->*mut dma_async_tx_descriptor{let ch=to_ioat_chan(chan);if test_bit(IOAT_CHAN_DOWN,&(*ch).state)||src_cnt>MAX_SCF as u32{return core::ptr::null_mut();}let mut scf=[0u8;MAX_SCF];let mut pq=[dst,dst];flags|=DMA_PREP_PQ_DISABLE_Q;__ioat_prep_pq_lock(chan,core::ptr::null_mut(),pq.as_ptr(),src,src_cnt,scf.as_ptr(),len,flags)}
pub unsafe fn ioat_prep_pqxor_val(chan:*mut dma_chan,src:*mut dma_addr_t,src_cnt:u32,len:usize,result:*mut sum_check_flags,mut flags:c_ulong)->*mut dma_async_tx_descriptor{let ch=to_ioat_chan(chan);if test_bit(IOAT_CHAN_DOWN,&(*ch).state)||src_cnt>MAX_SCF as u32{return core::ptr::null_mut();}*result=0;let mut scf=[0u8;MAX_SCF];let pq=[*src,*src];flags|=DMA_PREP_PQ_DISABLE_Q;__ioat_prep_pq_lock(chan,result,pq.as_ptr(),src.add(1),src_cnt-1,scf.as_ptr(),len,flags)}

pub unsafe fn ioat_prep_interrupt_lock(c:*mut dma_chan,flags:c_ulong)->*mut dma_async_tx_descriptor{let ch=to_ioat_chan(c);if test_bit(IOAT_CHAN_DOWN,&(*ch).state)||ioat_check_space_lock(ch,1)!=0{return core::ptr::null_mut();}let d=ioat_get_ring_ent(ch,(*ch).head);let h=(*d).hw;(*h).ctl=0;(*h).ctl_f.null=1;(*h).ctl_f.int_en=1;(*h).ctl_f.fence=((flags&DMA_PREP_FENCE)!=0) as _;(*h).ctl_f.compl_write=1;(*h).size=NULL_DESC_BUFFER_SIZE;(*h).src_addr=0;(*h).dst_addr=0;(*d).txd.flags=flags;(*d).len=1;dump_desc_dbg(ch,d);&mut (*d).txd}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
