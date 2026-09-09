// SPDX-License-Identifier: GPL-2.0

// Kernel headers and externally supplied symbols are intentionally not redefined here.

pub unsafe fn tso_build_hdr(
    skb: *const sk_buff,
    mut hdr: *mut core::ffi::c_char,
    tso: *mut tso_t,
    size: i32,
    is_last: bool,
) {
    let hdr_len = skb_transport_offset(skb) + (*tso).tlen;
    let mac_hdr_len = skb_network_offset(skb);

    core::ptr::copy_nonoverlapping((*skb).data, hdr, hdr_len as usize);
    if !(*tso).ipv6 {
        let iph = hdr.add(mac_hdr_len as usize) as *mut iphdr;
        (*iph).id = htons((*tso).ip_id);
        (*iph).tot_len = htons(size + hdr_len - mac_hdr_len);
        (*tso).ip_id += 1;
    } else {
        let iph = hdr.add(mac_hdr_len as usize) as *mut ipv6hdr;
        (*iph).payload_len = htons(size + (*tso).tlen);
    }
    hdr = hdr.add(skb_transport_offset(skb) as usize);
    if (*tso).tlen != core::mem::size_of::<udphdr>() as i32 {
        let tcph = hdr as *mut tcphdr;
        put_unaligned_be32((*tso).tcp_seq, &mut (*tcph).seq);
        if !is_last {
            // Clear all special flags for not last packet
            (*tcph).psh = 0;
            (*tcph).fin = 0;
            (*tcph).rst = 0;
        }
    } else {
        let uh = hdr as *mut udphdr;
        // size is after segmentation.
        udp_set_len_short(uh, core::mem::size_of::<udphdr>() as i32 + size);
    }
}

pub unsafe fn tso_build_data(skb: *const sk_buff, tso: *mut tso_t, size: i32) {
    (*tso).tcp_seq = (*tso).tcp_seq.wrapping_add(size as u32); // not worth avoiding this operation for UDP
    (*tso).size -= size;
    (*tso).data = (*tso).data.add(size as usize);

    if (*tso).size == 0 && (*tso).next_frag_idx < skb_shinfo(skb).nr_frags {
        let frag = &mut skb_shinfo(skb).frags[(*tso).next_frag_idx as usize];
        // Move to next segment
        (*tso).size = skb_frag_size(frag);
        (*tso).data = skb_frag_address(frag);
        (*tso).next_frag_idx += 1;
    }
}

pub unsafe fn tso_start(skb: *mut sk_buff, tso: *mut tso_t) -> i32 {
    let tlen = if skb_is_gso_tcp(skb) { tcp_hdrlen(skb) } else { core::mem::size_of::<udphdr>() as i32 };
    let hdr_len = skb_transport_offset(skb) + tlen;

    (*tso).tlen = tlen;
    (*tso).ip_id = ntohs((*ip_hdr(skb)).id);
    (*tso).tcp_seq = if tlen != core::mem::size_of::<udphdr>() as i32 { ntohl((*tcp_hdr(skb)).seq) } else { 0 };
    (*tso).next_frag_idx = 0;
    (*tso).ipv6 = vlan_get_protocol(skb) == htons(ETH_P_IPV6);
    (*tso).size = skb_headlen(skb) - hdr_len;
    (*tso).data = (*skb).data.add(hdr_len as usize);
    if (*tso).size == 0 && (*tso).next_frag_idx < skb_shinfo(skb).nr_frags {
        let frag = &mut skb_shinfo(skb).frags[0];
        // Move to next segment
        (*tso).size = skb_frag_size(frag);
        (*tso).data = skb_frag_address(frag);
        (*tso).next_frag_idx += 1;
    }
    hdr_len
}

unsafe fn tso_dma_iova_try(
    dev: *mut device, map: *mut tso_dma_map, phys: phys_addr_t,
    linear_len: usize, total_len: usize, offset: *mut usize,
) -> i32 {
    if !dma_iova_try_alloc(dev, &mut (*map).iova_state, phys, total_len) { return 1; }
    let skb = (*map).skb;
    let nr_frags = skb_shinfo(skb).nr_frags;
    if linear_len != 0 {
        if dma_iova_link(dev, &mut (*map).iova_state, phys, *offset, linear_len, DMA_TO_DEVICE, 0) { goto_iova_fail(dev, map, offset); return 1; }
        (*map).linear_len = linear_len;
        *offset += linear_len;
    }
    for i in 0..nr_frags {
        let frag = &mut skb_shinfo(skb).frags[i as usize];
        let frag_len = skb_frag_size(frag) as usize;
        if dma_iova_link(dev, &mut (*map).iova_state, skb_frag_phys(frag), *offset, frag_len, DMA_TO_DEVICE, 0) {
            (*map).nr_frags = i;
            goto_iova_fail(dev, map, offset); return 1;
        }
        (*map).frags[i as usize].len = frag_len;
        *offset += frag_len;
        (*map).nr_frags = i + 1;
    }
    if dma_iova_sync(dev, &mut (*map).iova_state, 0, total_len) { goto_iova_fail(dev, map, offset); return 1; }
    0
}

unsafe fn goto_iova_fail(dev: *mut device, map: *mut tso_dma_map, offset: *mut usize) {
    dma_iova_destroy(dev, &mut (*map).iova_state, *offset, DMA_TO_DEVICE, 0);
    core::ptr::write_bytes(&mut (*map).iova_state as *mut _, 0, 1);
    (*map).frag_idx = -1;
    (*map).offset = 0;
    (*map).linear_len = 0;
    (*map).nr_frags = 0;
}

pub unsafe fn tso_dma_map_init(map: *mut tso_dma_map, dev: *mut device, skb: *const sk_buff, hdr_len: u32) -> i32 {
    let linear_len = (skb_headlen(skb) - hdr_len as i32) as usize;
    let nr_frags = skb_shinfo(skb).nr_frags;
    let total_len = ((*skb).len - hdr_len) as usize;
    let mut offset = 0usize;
    (*map).dev = dev; (*map).skb = skb; (*map).hdr_len = hdr_len; (*map).frag_idx = -1;
    (*map).offset = 0; (*map).iova_offset = 0; (*map).total_len = total_len; (*map).linear_len = 0; (*map).nr_frags = 0;
    core::ptr::write_bytes(&mut (*map).iova_state as *mut _, 0, 1);
    if total_len == 0 { return 0; }
    let phys = if linear_len != 0 { virt_to_phys((*skb).data.add(hdr_len as usize)) } else { skb_frag_phys(&skb_shinfo(skb).frags[0]) };
    if tso_dma_iova_try(dev, map, phys, linear_len, total_len, &mut offset) != 0 {
        if linear_len != 0 {
            (*map).linear_dma = dma_map_phys(dev, phys, linear_len, DMA_TO_DEVICE, 0);
            if dma_mapping_error(dev, (*map).linear_dma) { return -12; }
            (*map).linear_len = linear_len;
        }
        for i in 0..nr_frags {
            let frag = &mut skb_shinfo(skb).frags[i as usize];
            let frag_len = skb_frag_size(frag) as usize;
            (*map).frags[i as usize].len = frag_len;
            (*map).frags[i as usize].dma = dma_map_phys(dev, skb_frag_phys(frag), frag_len, DMA_TO_DEVICE, 0);
            if dma_mapping_error(dev, (*map).frags[i as usize].dma) { tso_dma_map_cleanup(map); return -12; }
            (*map).nr_frags = i + 1;
        }
    }
    if linear_len == 0 && nr_frags > 0 { (*map).frag_idx = 0; }
    0
}

pub unsafe fn tso_dma_map_cleanup(map: *mut tso_dma_map) {
    if dma_use_iova(&(*map).iova_state) {
        dma_iova_destroy((*map).dev, &mut (*map).iova_state, (*map).total_len, DMA_TO_DEVICE, 0);
        core::ptr::write_bytes(&mut (*map).iova_state as *mut _, 0, 1);
    } else {
        if (*map).linear_len != 0 { dma_unmap_phys((*map).dev, (*map).linear_dma, (*map).linear_len, DMA_TO_DEVICE, 0); }
        for i in 0..(*map).nr_frags { dma_unmap_phys((*map).dev, (*map).frags[i as usize].dma, (*map).frags[i as usize].len, DMA_TO_DEVICE, 0); }
    }
    (*map).linear_len = 0; (*map).nr_frags = 0;
}

pub unsafe fn tso_dma_map_count(map: *mut tso_dma_map, mut len: u32) -> u32 {
    let mut offset = (*map).offset; let mut idx = (*map).frag_idx; let mut count = 0;
    if len == 0 { return 0; }
    if dma_use_iova(&(*map).iova_state) { return 1; }
    while len > 0 {
        let region_len = if idx == -1 { (*map).linear_len } else { (*map).frags[idx as usize].len };
        let chunk = core::cmp::min(len as usize, region_len - offset);
        len -= chunk as u32; count += 1; offset = 0; idx += 1;
    }
    count
}

pub unsafe fn tso_dma_map_next(map: *mut tso_dma_map, addr: *mut dma_addr_t, chunk_len: *mut u32, mapping_len: *mut u32, seg_remaining: u32) -> bool {
    if seg_remaining == 0 { return false; }
    if dma_use_iova(&(*map).iova_state) {
        *addr = (*map).iova_state.addr + (*map).iova_offset; *chunk_len = seg_remaining; *mapping_len = 0; (*map).iova_offset += seg_remaining as usize; return true;
    }
    let (region_len, base) = if (*map).frag_idx == -1 { ((*map).linear_len, (*map).linear_dma) } else { let f = &(*map).frags[(*map).frag_idx as usize]; (f.len, f.dma) };
    let chunk = core::cmp::min(seg_remaining as usize, region_len - (*map).offset);
    *addr = base + (*map).offset; *mapping_len = if (*map).offset == 0 { region_len as u32 } else { 0 }; *chunk_len = chunk as u32;
    (*map).offset += chunk;
    if (*map).offset >= region_len { (*map).frag_idx += 1; (*map).offset = 0; }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
