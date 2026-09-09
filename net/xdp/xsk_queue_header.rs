/* SPDX-License-Identifier: GPL-2.0 */
/* XDP user-space ring structure
 * Copyright(c) 2018 Intel Corporation.
 */

/* Dependencies supplied by the surrounding kernel translation are intentionally
 * left external: linux/types.h, linux/if_xdp.h, net/xdp_sock.h,
 * net/xsk_buff_pool.h, and xsk.h. */

#[repr(C)]
pub struct xdp_ring {
    pub producer: u32,
    pub pad1: u32,
    pub consumer: u32,
    pub pad2: u32,
    pub flags: u32,
    pub pad3: u32,
}

#[repr(C)]
pub struct xdp_rxtx_ring {
    pub ptrs: xdp_ring,
    pub desc: [xdp_desc; 0],
}

#[repr(C)]
pub struct xdp_umem_ring {
    pub ptrs: xdp_ring,
    pub desc: [u64; 0],
}

#[repr(C)]
pub struct xsk_queue {
    pub ring_mask: u32,
    pub nentries: u32,
    pub cached_prod: u32,
    pub cached_cons: u32,
    pub ring: *mut xdp_ring,
    pub invalid_descs: u64,
    pub queue_empty_descs: u64,
    pub ring_vmalloc_size: usize,
    pub cq_cached_prod_lock: spinlock_t,
}

#[repr(C)]
pub struct parsed_desc {
    pub mb: u32,
    pub valid: u32,
}

#[repr(C)]
pub struct xsk_tx_batch {
    pub tx_descs: u32,
    pub reclaim_descs: u32,
    pub budget_limited: bool,
}

#[inline]
pub unsafe fn xsk_tx_batch_cq_descs(batch: *const xsk_tx_batch) -> u32 {
    (*batch).tx_descs + (*batch).reclaim_descs
}

/* The following ring operations preserve the original producer/consumer
 * ordering and memory-barrier intent. */

#[inline]
pub unsafe fn __xskq_cons_read_addr_unchecked(q: *mut xsk_queue, cached_cons: u32, addr: *mut u64) {
    let ring = (*q).ring as *mut xdp_umem_ring;
    let idx = cached_cons & (*q).ring_mask;
    *addr = (*ring).desc[idx as usize];
}

#[inline]
pub unsafe fn xskq_cons_read_addr_unchecked(q: *mut xsk_queue, addr: *mut u64) -> bool {
    if (*q).cached_cons != (*q).cached_prod {
        __xskq_cons_read_addr_unchecked(q, (*q).cached_cons, addr);
        true
    } else { false }
}

#[inline]
pub fn xp_unused_options_set(options: u32) -> bool {
    options & !(XDP_PKT_CONTD | XDP_TX_METADATA) != 0
}

#[inline]
pub unsafe fn xp_aligned_validate_desc(pool: *mut xsk_buff_pool, desc: *mut xdp_desc) -> bool {
    let len = (*desc).len as u64;
    if len == 0 { return false; }
    let addr = match ((*desc).addr as u64).checked_sub((*pool).tx_metadata_len as u64) {
        Some(v) => v,
        None => return false,
    };
    let offset = addr & ((*pool).chunk_size as u64 - 1);
    if offset + len + (*pool).tx_metadata_len as u64 > (*pool).chunk_size as u64 { return false; }
    if addr >= (*pool).addrs_cnt as u64 { return false; }
    if xp_unused_options_set((*desc).options) { return false; }
    true
}

#[inline]
pub unsafe fn xp_unaligned_validate_desc(pool: *mut xsk_buff_pool, desc: *mut xdp_desc) -> bool {
    let mut len = (*desc).len as u64;
    if len == 0 { return false; }
    len += (*pool).tx_metadata_len as u64;
    if len > (*pool).chunk_size as u64 { return false; }
    let addr0 = xp_unaligned_add_offset_to_addr((*desc).addr);
    let addr = match addr0.checked_sub((*pool).tx_metadata_len as u64) {
        Some(v) => v,
        None => return false,
    };
    if addr >= (*pool).addrs_cnt as u64 { return false; }
    let end = match addr.checked_add(len) { Some(v) => v, None => return false };
    if end > (*pool).addrs_cnt as u64 { return false; }
    if xp_desc_crosses_non_contig_pg(pool, addr, len) { return false; }
    if xp_unused_options_set((*desc).options) { return false; }
    true
}

#[inline]
pub unsafe fn xp_validate_desc(pool: *mut xsk_buff_pool, desc: *mut xdp_desc) -> bool {
    if (*pool).unaligned { xp_unaligned_validate_desc(pool, desc) } else { xp_aligned_validate_desc(pool, desc) }
}

#[inline] pub unsafe fn xskq_has_descs(q: *mut xsk_queue) -> bool { (*q).cached_cons != (*q).cached_prod }

#[inline]
pub unsafe fn xskq_cons_is_valid_desc(q: *mut xsk_queue, d: *mut xdp_desc, pool: *mut xsk_buff_pool) -> bool {
    if !xp_validate_desc(pool, d) { (*q).invalid_descs += 1; false } else { true }
}

#[inline]
pub unsafe fn xskq_cons_read_desc(q: *mut xsk_queue, desc: *mut xdp_desc, pool: *mut xsk_buff_pool) -> bool {
    if (*q).cached_cons != (*q).cached_prod {
        let ring = (*q).ring as *mut xdp_rxtx_ring;
        let idx = (*q).cached_cons & (*q).ring_mask;
        *desc = (*ring).desc[idx as usize];
        xskq_cons_is_valid_desc(q, desc, pool)
    } else { (*q).queue_empty_descs += 1; false }
}

#[inline] pub unsafe fn xskq_cons_release_n(q: *mut xsk_queue, cnt: u32) { (*q).cached_cons += cnt; }

#[inline]
pub unsafe fn parse_desc(q: *mut xsk_queue, pool: *mut xsk_buff_pool, desc: *mut xdp_desc, parsed: *mut parsed_desc) {
    (*parsed).valid = xskq_cons_is_valid_desc(q, desc, pool) as u32;
    (*parsed).mb = xp_mb_desc(desc);
}

#[inline]
pub unsafe fn xskq_cons_read_desc_batch(xs: *mut xdp_sock, pool: *mut xsk_buff_pool, descs: *mut xdp_desc, max: u32) -> xsk_tx_batch {
    let mut drain = READ_ONCE((*xs).drain_cont);
    let mut cached_cons;
    let mut nb_entries = 0u32;
    let mut batch: xsk_tx_batch = core::mem::zeroed();
    let q = (*xs).tx;
    let mut nr_frags = 0u32;
    cached_cons = (*q).cached_cons;
    while cached_cons != (*q).cached_prod && nb_entries < max {
        let ring = (*q).ring as *mut xdp_rxtx_ring;
        let idx = cached_cons & (*q).ring_mask;
        let mut parsed: parsed_desc = core::mem::zeroed();
        *descs.add(nb_entries as usize) = (*ring).desc[idx as usize];
        cached_cons += 1;
        parse_desc(q, pool, descs.add(nb_entries as usize), &mut parsed);
        if parsed.valid == 0 { drain = true; }
        nr_frags += 1; nb_entries += 1;
        if !parsed.mb {
            if drain { batch.reclaim_descs = nr_frags; WRITE_ONCE((*xs).drain_cont, false); nr_frags = 0; break; }
            batch.tx_descs += nr_frags; nr_frags = 0; continue;
        }
        if nr_frags == (*pool).xdp_zc_max_segs { drain = true; }
    }
    if nr_frags != 0 {
        if drain { batch.reclaim_descs = nr_frags; WRITE_ONCE((*xs).drain_cont, true); }
        else { if nb_entries == max { batch.budget_limited = true; } cached_cons -= nr_frags; }
    }
    xskq_cons_release_n(q, cached_cons - (*q).cached_cons);
    batch
}

#[inline] pub unsafe fn __xskq_cons_release(q: *mut xsk_queue) { smp_store_release(&mut (*(*q).ring).consumer, (*q).cached_cons); }
#[inline] pub unsafe fn __xskq_cons_peek(q: *mut xsk_queue) { (*q).cached_prod = smp_load_acquire(&(*(*q).ring).producer); }
#[inline] pub unsafe fn xskq_cons_get_entries(q: *mut xsk_queue) { __xskq_cons_release(q); __xskq_cons_peek(q); }
#[inline] pub unsafe fn xskq_cons_nb_entries(q: *mut xsk_queue, max: u32) -> u32 { let mut e=(*q).cached_prod-(*q).cached_cons; if e>=max{return max} __xskq_cons_peek(q); e=(*q).cached_prod-(*q).cached_cons; if e>=max{max}else{e} }
#[inline] pub unsafe fn xskq_cons_peek_addr_unchecked(q:*mut xsk_queue,a:*mut u64)->bool { if (*q).cached_prod==(*q).cached_cons{xskq_cons_get_entries(q)} xskq_cons_read_addr_unchecked(q,a) }
#[inline] pub unsafe fn xskq_cons_peek_desc(q:*mut xsk_queue,d:*mut xdp_desc,p:*mut xsk_buff_pool)->bool { if (*q).cached_prod==(*q).cached_cons{xskq_cons_get_entries(q)} xskq_cons_read_desc(q,d,p) }
#[inline] pub unsafe fn xskq_cons_release(q:*mut xsk_queue){(*q).cached_cons+=1}
#[inline] pub unsafe fn xskq_cons_cancel_n(q:*mut xsk_queue,c:u32){(*q).cached_cons-=c}
#[inline] pub unsafe fn xskq_cons_present_entries(q:*mut xsk_queue)->u32{READ_ONCE((*q).ring.as_ref().unwrap().producer)-READ_ONCE((*q).ring.as_ref().unwrap().consumer)}
#[inline] pub unsafe fn xskq_get_prod(q:*mut xsk_queue)->u32{READ_ONCE((*q).ring.as_ref().unwrap().producer)}
#[inline] pub unsafe fn xskq_prod_nb_free(q:*mut xsk_queue,max:u32)->u32{let mut f=(*q).nentries-((*q).cached_prod-(*q).cached_cons);if f>=max{return max}(*q).cached_cons=READ_ONCE((*q).ring.as_ref().unwrap().consumer);f=(*q).nentries-((*q).cached_prod-(*q).cached_cons);if f>=max{max}else{f}}
#[inline] pub unsafe fn xskq_prod_is_full(q:*mut xsk_queue)->bool{xskq_prod_nb_free(q,1)==0}
#[inline] pub unsafe fn xskq_prod_cancel_n(q:*mut xsk_queue,c:u32){(*q).cached_prod-=c}
#[inline] pub unsafe fn xskq_prod_reserve(q:*mut xsk_queue)->i32{if xskq_prod_is_full(q){return -ENOSPC}(*q).cached_prod+=1;0}
#[inline] pub unsafe fn xskq_prod_reserve_addr(q:*mut xsk_queue,addr:u64)->i32{if xskq_prod_is_full(q){return -ENOSPC}let r=(*q).ring as *mut xdp_umem_ring;(*r).desc[((*q).cached_prod&(*q).ring_mask)as usize]=addr;(*q).cached_prod+=1;0}
#[inline] pub unsafe fn xskq_prod_write_addr(q:*mut xsk_queue,idx:u32,addr:u64){let r=(*q).ring as *mut xdp_umem_ring;(*r).desc[(idx&(*q).ring_mask)as usize]=addr}
#[inline] pub unsafe fn xskq_prod_write_addr_batch(q:*mut xsk_queue,descs:*mut xdp_desc,nb:u32){let r=(*q).ring as *mut xdp_umem_ring;let mut p=(*q).cached_prod;for i in 0..nb{(*r).desc[(p&(*q).ring_mask)as usize]=(*descs.add(i as usize)).addr;p+=1}(*q).cached_prod=p}
#[inline] pub unsafe fn __xskq_prod_reserve_desc(q:*mut xsk_queue,addr:u64,len:u32,flags:u32){let r=(*q).ring as *mut xdp_rxtx_ring;let i=((*q).cached_prod&(*q).ring_mask)as usize;(*r).desc[i].addr=addr;(*r).desc[i].len=len;(*r).desc[i].options=flags;(*q).cached_prod+=1}
#[inline] pub unsafe fn xskq_prod_reserve_desc(q:*mut xsk_queue,addr:u64,len:u32,flags:u32)->i32{if xskq_prod_is_full(q){return -ENOBUFS}__xskq_prod_reserve_desc(q,addr,len,flags);0}
#[inline] pub unsafe fn __xskq_prod_submit(q:*mut xsk_queue,idx:u32){smp_store_release(&mut (*(*q).ring).producer,idx)}
#[inline] pub unsafe fn xskq_prod_submit(q:*mut xsk_queue){smp_store_release(&mut (*(*q).ring).producer,(*q).cached_prod)}
#[inline] pub unsafe fn xskq_prod_submit_n(q:*mut xsk_queue,nb:u32){__xskq_prod_submit(q,(*(*q).ring).producer+nb)}
#[inline] pub unsafe fn xskq_prod_is_empty(q:*mut xsk_queue)->bool{READ_ONCE((*q).ring.as_ref().unwrap().consumer)==READ_ONCE((*q).ring.as_ref().unwrap().producer)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
