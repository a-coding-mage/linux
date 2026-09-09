// SPDX-License-Identifier: GPL-2.0-only

// C dependencies: <net/6lowpan.h>, <net/ndisc.h>,
// <net/ieee802154_netdev.h>, <net/mac802154.h>, and "6lowpan_i.h".

const LOWPAN_FRAG1_HEAD_SIZE: usize = 0x4;
const LOWPAN_FRAGN_HEAD_SIZE: usize = 0x5;

#[repr(C)]
struct lowpan_addr_info {
    daddr: ieee802154_addr,
    saddr: ieee802154_addr,
}

#[inline]
unsafe fn lowpan_skb_priv(skb: *const sk_buff) -> *mut lowpan_addr_info {
    WARN_ON_ONCE(skb_headroom(skb) < core::mem::size_of::<lowpan_addr_info>());
    (skb_data(skb) as *mut u8).sub(core::mem::size_of::<lowpan_addr_info>())
        as *mut lowpan_addr_info
}

/* This callback will be called from AF_PACKET and IPv6 stack, the AF_PACKET
 * sockets gives an 8 byte array for addresses only!
 *
 * TODO I think AF_PACKET DGRAM (sending/receiving) RAW (sending) makes no
 * sense here. We should disable it, the right use-case would be AF_INET6
 * RAW/DGRAM sockets.
 */
unsafe fn lowpan_header_create(
    skb: *mut sk_buff, ldev: *mut net_device, type_: u16,
    daddr: *const core::ffi::c_void, saddr: *const core::ffi::c_void, _len: u32,
) -> i32 {
    let wpan_dev = (*lowpan_802154_dev(ldev)).wdev.ieee802154_ptr;
    let info = lowpan_skb_priv(skb);
    let mut llneigh: *mut lowpan_802154_neigh = core::ptr::null_mut();
    let hdr = ipv6_hdr(skb);
    let mut n: *mut neighbour;

    if daddr.is_null() { return -EINVAL; }
    if type_ != ETH_P_IPV6 { return 0; }

    (*info).saddr.pan_id = (*wpan_dev).pan_id;
    (*info).daddr.pan_id = (*info).saddr.pan_id;

    if !memcmp(daddr, (*ldev).broadcast.as_ptr() as *const _, EUI64_ADDR_LEN) {
        (*info).daddr.short_addr = cpu_to_le16(IEEE802154_ADDR_BROADCAST);
        (*info).daddr.mode = IEEE802154_ADDR_SHORT;
    } else {
        let mut short_addr = cpu_to_le16(IEEE802154_ADDR_SHORT_UNSPEC);
        n = neigh_lookup(&nd_tbl, &(*hdr).daddr, ldev);
        if !n.is_null() {
            llneigh = lowpan_802154_neigh(neighbour_priv(n));
            read_lock_bh(&mut (*n).lock);
            short_addr = (*llneigh).short_addr;
            read_unlock_bh(&mut (*n).lock);
        }
        if !llneigh.is_null() && lowpan_802154_is_valid_src_short_addr(short_addr) {
            (*info).daddr.short_addr = short_addr;
            (*info).daddr.mode = IEEE802154_ADDR_SHORT;
        } else {
            (*info).daddr.mode = IEEE802154_ADDR_LONG;
            ieee802154_be64_to_le64(&mut (*info).daddr.extended_addr, daddr);
        }
        if !n.is_null() { neigh_release(n); }
    }

    if saddr.is_null() {
        if lowpan_802154_is_valid_src_short_addr((*wpan_dev).short_addr) {
            (*info).saddr.mode = IEEE802154_ADDR_SHORT;
            (*info).saddr.short_addr = (*wpan_dev).short_addr;
        } else {
            (*info).saddr.mode = IEEE802154_ADDR_LONG;
            (*info).saddr.extended_addr = (*wpan_dev).extended_addr;
        }
    } else {
        (*info).saddr.mode = IEEE802154_ADDR_LONG;
        ieee802154_be64_to_le64(&mut (*info).saddr.extended_addr, saddr);
    }
    0
}

unsafe fn lowpan_alloc_frag(skb: *mut sk_buff, size: i32,
    master_hdr: *const ieee802154_hdr, frag1: bool) -> *mut sk_buff {
    let wdev = (*lowpan_802154_dev((*skb).dev)).wdev;
    let frag = alloc_skb(wdev.needed_headroom + wdev.needed_tailroom + size, GFP_ATOMIC);
    if likely(!frag.is_null()) {
        (*frag).dev = wdev;
        (*frag).priority = (*skb).priority;
        skb_reserve(frag, wdev.needed_headroom);
        skb_reset_network_header(frag);
        *mac_cb(frag) = *mac_cb(skb);
        if frag1 { skb_put_data(frag, skb_mac_header(skb), (*skb).mac_len); }
        else {
            let rc = wpan_dev_hard_header(frag, wdev, &(*master_hdr).dest, &(*master_hdr).source, size);
            if rc < 0 { kfree_skb(frag); return ERR_PTR(rc); }
        }
        frag
    } else { ERR_PTR(-ENOMEM) }
}

unsafe fn lowpan_xmit_fragment(skb: *mut sk_buff, wpan_hdr: *const ieee802154_hdr,
    frag_hdr: *mut u8, frag_hdrlen: i32, offset: i32, len: i32, frag1: bool) -> i32 {
    raw_dump_inline(__func__, " fragment header", frag_hdr, frag_hdrlen);
    let frag = lowpan_alloc_frag(skb, frag_hdrlen + len, wpan_hdr, frag1);
    if IS_ERR(frag) { return PTR_ERR(frag); }
    skb_put_data(frag, frag_hdr, frag_hdrlen);
    skb_put_data(frag, skb_network_header(skb).add(offset as usize), len);
    raw_dump_table(__func__, " fragment dump", (*frag).data, (*frag).len);
    dev_queue_xmit(frag)
}

unsafe fn lowpan_xmit_fragmented(skb: *mut sk_buff, ldev: *mut net_device,
    wpan_hdr: *const ieee802154_hdr, dgram_size: u16, mut dgram_offset: u16) -> i32 {
    let mut frag_tag = htons((*lowpan_802154_dev(ldev)).fragment_tag);
    (*lowpan_802154_dev(ldev)).fragment_tag += 1;
    let mut frag_hdr = [0u8; 5];
    frag_hdr[0] = LOWPAN_DISPATCH_FRAG1 | ((dgram_size >> 8) as u8 & 0x07);
    frag_hdr[1] = dgram_size as u8;
    memcpy(frag_hdr.as_mut_ptr().add(2), &frag_tag as *const _ as *const _, 2);
    let payload_cap = ieee802154_max_payload(wpan_hdr);
    let mut frag_len = round_down(payload_cap - LOWPAN_FRAG1_HEAD_SIZE as i32 - skb_network_header_len(skb), 8);
    let mut skb_offset = skb_network_header_len(skb);
    let mut skb_unprocessed = (*skb).len - (*skb).mac_len as i32 - skb_offset;
    let mut rc = lowpan_xmit_fragment(skb, wpan_hdr, frag_hdr.as_mut_ptr(), LOWPAN_FRAG1_HEAD_SIZE as i32, 0, frag_len + skb_offset, true);
    if rc != 0 { pr_debug!("%s unable to send FRAG1 packet (tag: %d)", __func__, ntohs(frag_tag)); kfree_skb(skb); return rc; }
    frag_hdr[0] = (frag_hdr[0] & !LOWPAN_DISPATCH_FRAG1) | LOWPAN_DISPATCH_FRAGN;
    let frag_cap = round_down(payload_cap - LOWPAN_FRAGN_HEAD_SIZE as i32, 8);
    loop {
        dgram_offset += frag_len as u16; skb_offset += frag_len; skb_unprocessed -= frag_len;
        frag_len = core::cmp::min(frag_cap, skb_unprocessed); frag_hdr[4] = (dgram_offset >> 3) as u8;
        rc = lowpan_xmit_fragment(skb, wpan_hdr, frag_hdr.as_mut_ptr(), LOWPAN_FRAGN_HEAD_SIZE as i32, skb_offset, frag_len, false);
        if rc != 0 { pr_debug!("%s unable to send a FRAGN packet. (tag: %d, offset: %d)\n", __func__, ntohs(frag_tag), skb_offset); kfree_skb(skb); return rc; }
        if skb_unprocessed <= frag_cap { break; }
    }
    (*ldev).stats.tx_packets += 1; (*ldev).stats.tx_bytes += dgram_size as u64; consume_skb(skb); NET_XMIT_SUCCESS
}

unsafe fn lowpan_header(skb: *mut sk_buff, ldev: *mut net_device, dgram_size: *mut u16, dgram_offset: *mut u16) -> i32 {
    let wpan_dev = (*lowpan_802154_dev(ldev)).wdev.ieee802154_ptr;
    let cb = mac_cb_init(skb); let mut info = core::mem::zeroed::<lowpan_addr_info>();
    memcpy(&mut info as *mut _ as *mut _, lowpan_skb_priv(skb), core::mem::size_of::<lowpan_addr_info>());
    *dgram_size = (*skb).len as u16; lowpan_header_compress(skb, ldev, &mut info.daddr, &mut info.saddr);
    *dgram_offset = (*dgram_size - (*skb).len as u16) + skb_network_header_len(skb) as u16;
    (*cb).type_ = IEEE802154_FC_TYPE_DATA;
    (*cb).ackreq = !(info.daddr.mode == IEEE802154_ADDR_SHORT && ieee802154_is_broadcast_short_addr(info.daddr.short_addr)) && (*wpan_dev).ackreq;
    wpan_dev_hard_header(skb, (*lowpan_802154_dev(ldev)).wdev, &info.daddr, &info.saddr, 0)
}

unsafe fn lowpan_xmit(mut skb: *mut sk_buff, ldev: *mut net_device) -> netdev_tx_t {
    let mut wpan_hdr = core::mem::zeroed::<ieee802154_hdr>(); let mut dgram_size = 0u16; let mut dgram_offset = 0u16;
    pr_debug!("package xmit\n");
    if (*skb).protocol != htons(ETH_P_IPV6) { kfree_skb(skb); return NET_XMIT_DROP; }
    WARN_ON_ONCE((*skb).len > IPV6_MIN_MTU);
    if skb_headroom(skb) < (*ldev).needed_headroom || skb_tailroom(skb) < (*ldev).needed_tailroom {
        let nskb = skb_copy_expand(skb, (*ldev).needed_headroom, (*ldev).needed_tailroom, GFP_ATOMIC);
        if likely(!nskb.is_null()) { consume_skb(skb); skb = nskb; } else { kfree_skb(skb); return NET_XMIT_DROP; }
    } else { skb = skb_unshare(skb, GFP_ATOMIC); if skb.is_null() { return NET_XMIT_DROP; } }
    if lowpan_header(skb, ldev, &mut dgram_size, &mut dgram_offset) < 0 || ieee802154_hdr_peek(skb, &mut wpan_hdr) < 0 { kfree_skb(skb); return NET_XMIT_DROP; }
    let max_single = ieee802154_max_payload(&wpan_hdr);
    if skb_tail_pointer(skb).offset_from(skb_network_header(skb)) <= max_single as isize {
        (*skb).dev = (*lowpan_802154_dev(ldev)).wdev; (*ldev).stats.tx_packets += 1; (*ldev).stats.tx_bytes += dgram_size as u64; return dev_queue_xmit(skb);
    }
    let rc = lowpan_xmit_fragmented(skb, ldev, &wpan_hdr, dgram_size, dgram_offset); if rc < 0 { NET_XMIT_DROP } else { rc }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
