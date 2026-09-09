// SPDX-License-Identifier: GPL-2.0

// C dependencies supplied by the surrounding kernel translation.
// The declarations below intentionally retain the original low-level API.

const ETH_PAD_LEN: u32 = ETH_HLEN + 2 * VLAN_HLEN + ETH_FCS_LEN;

pub unsafe fn xp_add_xsk(pool: *mut xsk_buff_pool, xs: *mut xdp_sock) {
    if !(*xs).tx { return; }
    spin_lock(&mut (*pool).xsk_tx_list_lock);
    list_add_rcu(&mut (*xs).tx_list, &mut (*pool).xsk_tx_list);
    spin_unlock(&mut (*pool).xsk_tx_list_lock);
}
pub unsafe fn xp_del_xsk(pool: *mut xsk_buff_pool, xs: *mut xdp_sock) {
    if !(*xs).tx { return; }
    spin_lock(&mut (*pool).xsk_tx_list_lock);
    list_del_rcu(&mut (*xs).tx_list);
    spin_unlock(&mut (*pool).xsk_tx_list_lock);
}
pub unsafe fn xp_destroy(pool: *mut xsk_buff_pool) {
    if pool.is_null() { return; }
    kvfree((*pool).tx_descs as *mut _); kvfree((*pool).heads as *mut _); kvfree(pool as *mut _);
}
pub unsafe fn xp_alloc_tx_descs(pool: *mut xsk_buff_pool, xs: *mut xdp_sock, max_segs: u32) -> i32 {
    let nentries = core::cmp::max((*xs).tx.nentries, max_segs);
    (*pool).tx_descs = kvzalloc_objs((*pool).tx_descs, nentries);
    if (*pool).tx_descs.is_null() { return -ENOMEM; }
    (*pool).tx_descs_nentries = nentries; 0
}
pub unsafe fn xp_create_and_assign_umem(xs: *mut xdp_sock, umem: *mut xdp_umem, max_segs: u32) -> *mut xsk_buff_pool {
    let unaligned = ((*umem).flags & XDP_UMEM_UNALIGNED_CHUNK_FLAG) != 0;
    let entries = if unaligned { (*umem).chunks } else { 0 };
    let pool = kvzalloc_flex(free_heads, entries);
    if pool.is_null() { return core::ptr::null_mut(); }
    (*pool).heads = kvzalloc_objs((*pool).heads, (*umem).chunks);
    if (*pool).heads.is_null() { xp_destroy(pool); return core::ptr::null_mut(); }
    if !(*xs).tx.is_null() && xp_alloc_tx_descs(pool, xs, max_segs) != 0 { xp_destroy(pool); return core::ptr::null_mut(); }
    (*pool).chunk_mask = !((*umem).chunk_size as u64 - 1);
    (*pool).addrs_cnt = (*umem).size; (*pool).heads_cnt = (*umem).chunks; (*pool).free_heads_cnt = (*umem).chunks;
    (*pool).headroom = (*umem).headroom; (*pool).chunk_size = (*umem).chunk_size;
    (*pool).chunk_shift = ffs((*umem).chunk_size) - 1; (*pool).unaligned = unaligned;
    (*pool).frame_len = (*umem).chunk_size - (*umem).headroom - XDP_PACKET_HEADROOM;
    (*pool).umem = umem; (*pool).addrs = (*umem).addrs; (*pool).tx_metadata_len = (*umem).tx_metadata_len;
    (*pool).tx_sw_csum = (*umem).flags & XDP_UMEM_TX_SW_CSUM;
    spin_lock_init(&mut (*pool).rx_lock); INIT_LIST_HEAD(&mut (*pool).free_list); INIT_LIST_HEAD(&mut (*pool).xskb_list);
    INIT_LIST_HEAD(&mut (*pool).xsk_tx_list); spin_lock_init(&mut (*pool).xsk_tx_list_lock); spin_lock_init(&mut (*pool).cq_prod_lock);
    spin_lock_init(&mut (*(*xs).cq_tmp).cq_cached_prod_lock); refcount_set(&mut (*pool).users, 1);
    (*pool).fq = (*xs).fq_tmp; (*pool).cq = (*xs).cq_tmp;
    for i in 0..(*pool).free_heads_cnt { let xskb = &mut *(*pool).heads.add(i as usize); xskb.pool = pool; xskb.xdp.frame_sz = (*umem).chunk_size - (*umem).headroom; INIT_LIST_HEAD(&mut xskb.list_node); if unaligned { (*pool).free_heads.add(i as usize).write(xskb); } else { xp_init_xskb_addr(xskb, pool, i as u64 * (*pool).chunk_size as u64); } }
    pool
}
pub unsafe fn xp_set_rxq_info(pool: *mut xsk_buff_pool, rxq: *mut xdp_rxq_info) { for i in 0..(*pool).heads_cnt { (*(*pool).heads.add(i as usize)).xdp.rxq = rxq; } }
pub unsafe fn xp_fill_cb(pool: *mut xsk_buff_pool, desc: *mut xsk_cb_desc) { for i in 0..(*pool).heads_cnt { let xskb = &mut *(*pool).heads.add(i as usize); memcpy(xskb.cb.add((*desc).off as usize), (*desc).src, (*desc).bytes as usize); } }
unsafe fn xp_disable_drv_zc(pool: *mut xsk_buff_pool) { ASSERT_RTNL(); if (*pool).umem.zc { let mut bpf = core::mem::zeroed::<netdev_bpf>(); bpf.command = XDP_SETUP_XSK_POOL; bpf.xsk.pool = core::ptr::null_mut(); bpf.xsk.queue_id = (*pool).queue_id; let err = ((*(*pool).netdev).netdev_ops).ndo_bpf((*pool).netdev, &mut bpf); if err != 0 { WARN(1, "Failed to disable zero-copy!\n"); } } }
pub unsafe fn xp_assign_dev(pool: *mut xsk_buff_pool, netdev: *mut net_device, queue_id: u16, flags: u16) -> i32 {
    let needed = (*netdev).mtu + ETH_PAD_LEN; let mut segs = (*netdev).xdp_zc_max_segs; let mbuf = flags & XDP_USE_SG != 0; let force_zc = flags & XDP_ZEROCOPY != 0; let force_copy = flags & XDP_COPY != 0; let mut err = 0;
    ASSERT_RTNL(); if force_zc && force_copy { return -EINVAL; } if (*pool).tx_sw_csum && (*netdev).priv_flags & IFF_TX_SKB_NO_LINEAR != 0 { return -EOPNOTSUPP; } if !xsk_get_pool_from_qid(netdev, queue_id).is_null() { return -EBUSY; }
    (*pool).netdev = netdev; (*pool).queue_id = queue_id; err = xsk_reg_pool_at_qid(netdev, pool, queue_id); if err != 0 { return err; }
    if mbuf { (*pool).umem.flags |= XDP_UMEM_SG_FLAG; } if flags & XDP_USE_NEED_WAKEUP != 0 { (*pool).uses_need_wakeup = true; } (*pool).cached_need_wakeup = XDP_WAKEUP_TX; dev_hold(netdev); if force_copy { return 0; }
    if (*netdev).xdp_features & NETDEV_XDP_ACT_XSK != NETDEV_XDP_ACT_XSK { err = -EOPNOTSUPP; goto err_unreg_pool; }
    if mbuf { if segs == 1 { err = -EOPNOTSUPP; goto err_unreg_pool; } } else { segs = 1; }
    let mut frame_size = __xsk_pool_get_rx_frame_size(pool) - xsk_pool_get_tailroom(mbuf); frame_size = ALIGN_DOWN(frame_size, 128); if needed > frame_size * segs { err = -EINVAL; goto err_unreg_pool; } if dev_get_min_mp_channel_count(netdev) != 0 { err = -EBUSY; goto err_unreg_pool; }
    let mut bpf = core::mem::zeroed::<netdev_bpf>(); bpf.command = XDP_SETUP_XSK_POOL; bpf.xsk.pool = pool; bpf.xsk.queue_id = queue_id; netdev_assert_locked_ops_compat(netdev); err = ((*(*netdev).netdev_ops).ndo_bpf)(netdev, &mut bpf); if err != 0 { goto err_unreg_pool; }
    if (*pool).dma_pages.is_null() { WARN(1, "Driver did not DMA map zero-copy buffers"); err = -EINVAL; goto err_unreg_xsk; } (*pool).umem.zc = true; (*pool).xdp_zc_max_segs = (*netdev).xdp_zc_max_segs; return 0;
err_unreg_xsk: xp_disable_drv_zc(pool); err_unreg_pool: if !force_zc { err = 0; } if err != 0 { xsk_clear_pool_at_qid(netdev, queue_id); dev_put(netdev); } err
}
pub unsafe fn xp_assign_dev_shared(pool: *mut xsk_buff_pool, umem_xs: *mut xdp_sock, dev: *mut net_device, queue_id: u16) -> i32 { let mut flags = if (*(*umem_xs).umem).zc { XDP_ZEROCOPY } else { XDP_COPY }; if (*(*umem_xs).umem).flags & XDP_UMEM_SG_FLAG != 0 { flags |= XDP_USE_SG; } if (*(*umem_xs).pool).uses_need_wakeup { flags |= XDP_USE_NEED_WAKEUP; } xp_assign_dev(pool, dev, queue_id, flags) }
pub unsafe fn xp_clear_dev(pool: *mut xsk_buff_pool) { let netdev = (*pool).netdev; if netdev.is_null() { return; } netdev_lock_ops(netdev); xp_disable_drv_zc(pool); xsk_clear_pool_at_qid(netdev, (*pool).queue_id); (*pool).netdev = core::ptr::null_mut(); netdev_unlock_ops(netdev); dev_put(netdev); }
unsafe fn xp_release_deferred(work: *mut work_struct) { let pool = container_of!(work, xsk_buff_pool, work); rtnl_lock(); xp_clear_dev(pool); rtnl_unlock(); if !(*pool).fq.is_null() { xskq_destroy((*pool).fq); (*pool).fq = core::ptr::null_mut(); } if !(*pool).cq.is_null() { xskq_destroy((*pool).cq); (*pool).cq = core::ptr::null_mut(); } xdp_put_umem((*pool).umem, false); xp_destroy(pool); }
pub unsafe fn xp_get_pool(pool: *mut xsk_buff_pool) { refcount_inc(&mut (*pool).users); }
pub unsafe fn xp_put_pool(pool: *mut xsk_buff_pool) -> bool { if pool.is_null() { return false; } if refcount_dec_and_test(&mut (*pool).users) { INIT_WORK(&mut (*pool).work, xp_release_deferred); schedule_work(&mut (*pool).work); return true; } false }
unsafe fn xp_find_dma_map(pool: *mut xsk_buff_pool) -> *mut xsk_dma_map { let mut dma_map; list_for_each_entry!(dma_map, &mut (*(*pool).umem).xsk_dma_list, list, { if (*dma_map).netdev == (*pool).netdev { return dma_map; } }); core::ptr::null_mut() }
unsafe fn xp_create_dma_map(dev: *mut device, netdev: *mut net_device, nr_pages: u32, umem: *mut xdp_umem) -> *mut xsk_dma_map { let dma_map = kzalloc_obj(); if dma_map.is_null() { return core::ptr::null_mut(); } (*dma_map).dma_pages = kvzalloc_objs((*dma_map).dma_pages, nr_pages); if (*dma_map).dma_pages.is_null() { kfree(dma_map); return core::ptr::null_mut(); } (*dma_map).netdev = netdev; (*dma_map).dev = dev; (*dma_map).dma_pages_cnt = nr_pages; refcount_set(&mut (*dma_map).users, 1); list_add(&mut (*dma_map).list, &mut (*umem).xsk_dma_list); dma_map }
unsafe fn xp_destroy_dma_map(dma_map: *mut xsk_dma_map) { list_del(&mut (*dma_map).list); kvfree((*dma_map).dma_pages as *mut _); kfree(dma_map); }
unsafe fn __xp_dma_unmap(dma_map: *mut xsk_dma_map, attrs: ulong) { for i in 0..(*dma_map).dma_pages_cnt { let dma = &mut *(*dma_map).dma_pages.add(i as usize); if *dma != 0 { *dma &= !XSK_NEXT_PG_CONTIG_MASK; dma_unmap_page_attrs((*dma_map).dev, *dma, PAGE_SIZE, DMA_BIDIRECTIONAL, attrs); *dma = 0; } } xp_destroy_dma_map(dma_map); }
pub unsafe fn xp_dma_unmap(pool: *mut xsk_buff_pool, attrs: ulong) { if (*pool).dma_pages.is_null() { return; } let dma_map = xp_find_dma_map(pool); if dma_map.is_null() { WARN(1, "Could not find dma_map for device"); return; } if refcount_dec_and_test(&mut (*dma_map).users) { __xp_dma_unmap(dma_map, attrs); } kvfree((*pool).dma_pages as *mut _); (*pool).dma_pages = core::ptr::null_mut(); (*pool).dma_pages_cnt = 0; (*pool).dev = core::ptr::null_mut(); }
unsafe fn xp_check_dma_contiguity(dma_map: *mut xsk_dma_map) { for i in 0..(*dma_map).dma_pages_cnt - 1 { if (*dma_map).dma_pages[i as usize] + PAGE_SIZE == (*dma_map).dma_pages[(i+1) as usize] { (*dma_map).dma_pages[i as usize] |= XSK_NEXT_PG_CONTIG_MASK; } else { (*dma_map).dma_pages[i as usize] &= !XSK_NEXT_PG_CONTIG_MASK; } } }
unsafe fn xp_init_dma_info(pool: *mut xsk_buff_pool, dma_map: *mut xsk_dma_map) -> i32 { if !(*pool).unaligned { for i in 0..(*pool).heads_cnt { let xskb = &mut *(*pool).heads.add(i as usize); let orig_addr = xskb.xdp.data_hard_start as u64 - (*pool).addrs as u64 - (*pool).headroom as u64; xp_init_xskb_dma(xskb, pool, (*dma_map).dma_pages, orig_addr); } } (*pool).dma_pages = kvzalloc_objs((*pool).dma_pages, (*dma_map).dma_pages_cnt); if (*pool).dma_pages.is_null() { return -ENOMEM; } (*pool).dev = (*dma_map).dev; (*pool).dma_pages_cnt = (*dma_map).dma_pages_cnt; memcpy((*pool).dma_pages, (*dma_map).dma_pages, (*pool).dma_pages_cnt as usize * core::mem::size_of::<dma_addr_t>()); 0 }
pub unsafe fn xp_dma_map(pool: *mut xsk_buff_pool, dev: *mut device, attrs: ulong, pages: *mut *mut page, nr_pages: u32) -> i32 { let mut map = xp_find_dma_map(pool); if !map.is_null() { let err = xp_init_dma_info(pool, map); if err != 0 { return err; } refcount_inc(&mut (*map).users); return 0; } map = xp_create_dma_map(dev, (*pool).netdev, nr_pages, (*pool).umem); if map.is_null() { return -ENOMEM; } for i in 0..(*map).dma_pages_cnt { let dma = dma_map_page_attrs(dev, *pages.add(i as usize), 0, PAGE_SIZE, DMA_BIDIRECTIONAL, attrs); if dma_mapping_error(dev, dma) { __xp_dma_unmap(map, attrs); return -ENOMEM; } (*map).dma_pages[i as usize] = dma; } if (*pool).unaligned { xp_check_dma_contiguity(map); } let err = xp_init_dma_info(pool, map); if err != 0 { __xp_dma_unmap(map, attrs); return err; } 0 }
unsafe fn xp_addr_crosses_non_contig_pg(pool: *mut xsk_buff_pool, addr: u64) -> bool { xp_desc_crosses_non_contig_pg(pool, addr, (*pool).chunk_size) }
unsafe fn xp_check_unaligned(pool: *mut xsk_buff_pool, addr: *mut u64) -> bool { *addr = xp_unaligned_extract_addr(*addr); *addr < (*pool).addrs_cnt && *addr + (*pool).chunk_size as u64 <= (*pool).addrs_cnt && !xp_addr_crosses_non_contig_pg(pool, *addr) }
unsafe fn xp_check_aligned(pool: *mut xsk_buff_pool, addr: *mut u64) -> bool { *addr = xp_aligned_extract_addr(pool, *addr); *addr < (*pool).addrs_cnt }
unsafe fn xp_get_xskb(pool: *mut xsk_buff_pool, addr: u64) -> *mut xdp_buff_xsk { if (*pool).unaligned { let x = *(*pool).free_heads.add(((*pool).free_heads_cnt - 1) as usize); (*pool).free_heads_cnt -= 1; xp_init_xskb_addr(x, pool, addr); if !(*pool).dma_pages.is_null() { xp_init_xskb_dma(x, pool, (*pool).dma_pages, addr); } x } else { (*pool).heads.add(xp_aligned_extract_idx(pool, addr) as usize) } }
unsafe fn __xp_alloc(pool: *mut xsk_buff_pool) -> *mut xdp_buff_xsk { if (*pool).free_heads_cnt == 0 { return core::ptr::null_mut(); } loop { let mut addr=0; if !xskq_cons_peek_addr_unchecked((*pool).fq, &mut addr) { (*pool).fq.queue_empty_descs += 1; return core::ptr::null_mut(); } if !(if (*pool).unaligned { xp_check_unaligned(pool, &mut addr) } else { xp_check_aligned(pool, &mut addr) }) { (*pool).fq.invalid_descs += 1; xskq_cons_release((*pool).fq); continue; } let x = xp_get_xskb(pool, addr); xskq_cons_release((*pool).fq); return x; } }
pub unsafe fn xp_alloc(pool: *mut xsk_buff_pool) -> *mut xdp_buff { let xskb = if (*pool).free_list_cnt == 0 { __xp_alloc(pool) } else { (*pool).free_list_cnt -= 1; let x = list_first_entry!(&mut (*pool).free_list, xdp_buff_xsk, list_node); list_del_init(&mut (*x).list_node); x }; if xskb.is_null() { return core::ptr::null_mut(); } (*xskb).xdp.data = (*xskb).xdp.data_hard_start.add(XDP_PACKET_HEADROOM as usize); (*xskb).xdp.data_meta = (*xskb).xdp.data; (*xskb).xdp.flags = 0; if !(*pool).dev.is_null() { xp_dma_sync_for_device(pool, (*xskb).dma, (*pool).frame_len); } &mut (*xskb).xdp }
unsafe fn xp_alloc_new_from_fq(pool: *mut xsk_buff_pool, mut xdp: *mut *mut xdp_buff, mut max: u32) -> u32 { if max > (*pool).free_heads_cnt { max = (*pool).free_heads_cnt; } max = xskq_cons_nb_entries((*pool).fq, max); let mut cached = (*pool).fq.cached_cons; let mut n = max; let mut i = max; while i != 0 { let mut addr=0; i -= 1; __xskq_cons_read_addr_unchecked((*pool).fq, cached, &mut addr); cached += 1; if !(if (*pool).unaligned { xp_check_unaligned(pool, &mut addr) } else { xp_check_aligned(pool, &mut addr) }) { (*pool).fq.invalid_descs += 1; n -= 1; continue; } let x = xp_get_xskb(pool, addr); xdp.write(&mut (*x).xdp); xdp=xdp.add(1); } xskq_cons_release_n((*pool).fq, max); n }
unsafe fn xp_alloc_reused(pool: *mut xsk_buff_pool, mut xdp: *mut *mut xdp_buff, mut n: u32) -> u32 { n=core::cmp::min(n,(*pool).free_list_cnt); for _ in 0..n { let x=list_first_entry!(&mut (*pool).free_list,xdp_buff_xsk,list_node); list_del_init(&mut (*x).list_node); xdp.write(&mut (*x).xdp); xdp=xdp.add(1); } (*pool).free_list_cnt-=n; n }
unsafe fn xp_alloc_slow(pool: *mut xsk_buff_pool, mut xdp: *mut *mut xdp_buff, max: u32) -> u32 { for i in 0..max { let b=xp_alloc(pool); if b.is_null(){return i;} xdp.write(b); xdp=xdp.add(1); } max }
pub unsafe fn xp_alloc_batch(pool: *mut xsk_buff_pool, mut xdp: *mut *mut xdp_buff, mut max: u32) -> u32 { if !(*pool).dev.is_null() && dma_dev_need_sync((*pool).dev) { return xp_alloc_slow(pool,xdp,max); } let mut n1=0; if (*pool).free_list_cnt!=0 { n1=xp_alloc_reused(pool,xdp,max); if n1==max{return n1;} max-=n1;xdp=xdp.add(n1 as usize); } let n2=xp_alloc_new_from_fq(pool,xdp,max); if n2==0 {(*pool).fq.queue_empty_descs+=1;} n1+n2 }
pub unsafe fn xp_can_alloc(pool: *mut xsk_buff_pool, count: u32) -> bool { if (*pool).free_list_cnt>=count{return true;} let req=count-(*pool).free_list_cnt; let avail=xskq_cons_nb_entries((*pool).fq,req); if avail==0{(*pool).fq.queue_empty_descs+=1;} avail>=req }
pub unsafe fn xp_free(xskb: *mut xdp_buff_xsk) { if !list_empty(&(*xskb).list_node){return;} (*xskb).pool.free_list_cnt+=1; list_add(&mut (*xskb).list_node,&mut (*(*xskb).pool).free_list); }
unsafe fn __xp_raw_get_addr(pool:*const xsk_buff_pool,addr:u64)->u64{if (*pool).unaligned{xp_unaligned_add_offset_to_addr(addr)}else{addr}}
unsafe fn __xp_raw_get_data(pool:*const xsk_buff_pool,addr:u64)->*mut core::ffi::c_void{(*pool).addrs.add(addr as usize) as *mut _}
pub unsafe fn xp_raw_get_data(pool:*mut xsk_buff_pool,addr:u64)->*mut core::ffi::c_void{__xp_raw_get_data(pool,__xp_raw_get_addr(pool,addr))}
unsafe fn __xp_raw_get_dma(pool:*const xsk_buff_pool,addr:u64)->dma_addr_t{((*pool).dma_pages.add((addr>>PAGE_SHIFT) as usize).read() & !XSK_NEXT_PG_CONTIG_MASK)+(addr & !PAGE_MASK)}
pub unsafe fn xp_raw_get_dma(pool:*mut xsk_buff_pool,addr:u64)->dma_addr_t{__xp_raw_get_dma(pool,__xp_raw_get_addr(pool,addr))}
pub unsafe fn xp_raw_get_ctx(pool:*const xsk_buff_pool,mut addr:u64,options:u32)->xdp_desc_ctx{addr=__xp_raw_get_addr(pool,addr);xdp_desc_ctx{dma:__xp_raw_get_dma(pool,addr),meta:__xsk_buff_get_metadata(pool,__xp_raw_get_data(pool,addr),options)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
