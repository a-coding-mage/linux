// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

const LOWPAN_DISPATCH_FIRST: u8 = 0xc0;
const LOWPAN_DISPATCH_FRAG_MASK: u8 = 0xf8;

const LOWPAN_DISPATCH_NALP: u8 = 0x00;
const LOWPAN_DISPATCH_ESC: u8 = 0x40;
const LOWPAN_DISPATCH_HC1: u8 = 0x42;
const LOWPAN_DISPATCH_DFF: u8 = 0x43;
const LOWPAN_DISPATCH_BC0: u8 = 0x50;
const LOWPAN_DISPATCH_MESH: u8 = 0x80;

unsafe fn lowpan_give_skb_to_device(skb: *mut sk_buff) -> i32 {
    (*skb).protocol = htons(ETH_P_IPV6);
    (*(*skb).dev).stats.rx_packets += 1;
    (*(*skb).dev).stats.rx_bytes += (*skb).len;

    netif_rx(skb)
}

unsafe fn lowpan_rx_handlers_result(skb: *mut sk_buff, res: lowpan_rx_result) -> i32 {
    match res {
        RX_CONTINUE => {
            net_warn_ratelimited!("%s: received unknown dispatch\n", __func__);
            kfree_skb(skb);
            NET_RX_DROP
        }
        RX_DROP_UNUSABLE => {
            kfree_skb(skb);
            NET_RX_DROP
        }
        RX_DROP => NET_RX_DROP,
        RX_QUEUED => lowpan_give_skb_to_device(skb),
        _ => NET_RX_DROP,
    }
}

#[inline]
unsafe fn lowpan_is_frag1(dispatch: u8) -> bool {
    (dispatch & LOWPAN_DISPATCH_FRAG_MASK) == LOWPAN_DISPATCH_FRAG1
}

#[inline]
unsafe fn lowpan_is_fragn(dispatch: u8) -> bool {
    (dispatch & LOWPAN_DISPATCH_FRAG_MASK) == LOWPAN_DISPATCH_FRAGN
}

unsafe fn lowpan_rx_h_frag(skb: *mut sk_buff) -> lowpan_rx_result {
    if !(lowpan_is_frag1(*skb_network_header(skb)) || lowpan_is_fragn(*skb_network_header(skb))) {
        return RX_CONTINUE;
    }

    let ret = lowpan_frag_rcv(skb, *skb_network_header(skb) & LOWPAN_DISPATCH_FRAG_MASK);
    if ret == 1 {
        return RX_QUEUED;
    }

    /* Packet is freed by lowpan_frag_rcv on error or put into the frag
     * bucket.
     */
    RX_DROP
}

unsafe fn lowpan_iphc_decompress(skb: *mut sk_buff) -> i32 {
    let mut hdr: ieee802154_hdr = core::mem::zeroed();

    if ieee802154_hdr_peek_addrs(skb, &mut hdr) < 0 {
        return -EINVAL;
    }

    lowpan_header_decompress(skb, (*skb).dev, &hdr.dest, &hdr.source)
}

unsafe fn lowpan_rx_h_iphc(skb: *mut sk_buff) -> lowpan_rx_result {
    if !lowpan_is_iphc(*skb_network_header(skb)) {
        return RX_CONTINUE;
    }

    /* Setting datagram_offset to zero indicates non frag handling
     * while doing lowpan_header_decompress.
     */
    lowpan_802154_cb(skb).d_size = 0;

    if lowpan_iphc_decompress(skb) < 0 {
        return RX_DROP_UNUSABLE;
    }

    RX_QUEUED
}

pub unsafe fn lowpan_rx_h_ipv6(skb: *mut sk_buff) -> lowpan_rx_result {
    if !lowpan_is_ipv6(*skb_network_header(skb)) {
        return RX_CONTINUE;
    }

    /* Pull off the 1-byte of 6lowpan header. */
    skb_pull(skb, 1);
    RX_QUEUED
}

#[inline]
unsafe fn lowpan_is_esc(dispatch: u8) -> bool { dispatch == LOWPAN_DISPATCH_ESC }

unsafe fn lowpan_rx_h_esc(skb: *mut sk_buff) -> lowpan_rx_result {
    if !lowpan_is_esc(*skb_network_header(skb)) { return RX_CONTINUE; }
    net_warn_ratelimited!("%s: %s\n", (*(*skb).dev).name, "6LoWPAN ESC not supported\n");
    RX_DROP_UNUSABLE
}

#[inline]
unsafe fn lowpan_is_hc1(dispatch: u8) -> bool { dispatch == LOWPAN_DISPATCH_HC1 }
unsafe fn lowpan_rx_h_hc1(skb: *mut sk_buff) -> lowpan_rx_result {
    if !lowpan_is_hc1(*skb_network_header(skb)) { return RX_CONTINUE; }
    net_warn_ratelimited!("%s: %s\n", (*(*skb).dev).name, "6LoWPAN HC1 not supported\n");
    RX_DROP_UNUSABLE
}

#[inline]
unsafe fn lowpan_is_dff(dispatch: u8) -> bool { dispatch == LOWPAN_DISPATCH_DFF }
unsafe fn lowpan_rx_h_dff(skb: *mut sk_buff) -> lowpan_rx_result {
    if !lowpan_is_dff(*skb_network_header(skb)) { return RX_CONTINUE; }
    net_warn_ratelimited!("%s: %s\n", (*(*skb).dev).name, "6LoWPAN DFF not supported\n");
    RX_DROP_UNUSABLE
}

#[inline]
unsafe fn lowpan_is_bc0(dispatch: u8) -> bool { dispatch == LOWPAN_DISPATCH_BC0 }
unsafe fn lowpan_rx_h_bc0(skb: *mut sk_buff) -> lowpan_rx_result {
    if !lowpan_is_bc0(*skb_network_header(skb)) { return RX_CONTINUE; }
    net_warn_ratelimited!("%s: %s\n", (*(*skb).dev).name, "6LoWPAN BC0 not supported\n");
    RX_DROP_UNUSABLE
}

#[inline]
unsafe fn lowpan_is_mesh(dispatch: u8) -> bool { (dispatch & LOWPAN_DISPATCH_FIRST) == LOWPAN_DISPATCH_MESH }
unsafe fn lowpan_rx_h_mesh(skb: *mut sk_buff) -> lowpan_rx_result {
    if !lowpan_is_mesh(*skb_network_header(skb)) { return RX_CONTINUE; }
    net_warn_ratelimited!("%s: %s\n", (*(*skb).dev).name, "6LoWPAN MESH not supported\n");
    RX_DROP_UNUSABLE
}

unsafe fn lowpan_invoke_rx_handlers(skb: *mut sk_buff) -> i32 {
    let mut res = lowpan_rx_h_iphc(skb);
    if res == RX_CONTINUE { res = lowpan_rx_h_frag(skb); }
    if res == RX_CONTINUE { res = lowpan_rx_h_ipv6(skb); }
    if res == RX_CONTINUE { res = lowpan_rx_h_esc(skb); }
    if res == RX_CONTINUE { res = lowpan_rx_h_hc1(skb); }
    if res == RX_CONTINUE { res = lowpan_rx_h_dff(skb); }
    if res == RX_CONTINUE { res = lowpan_rx_h_bc0(skb); }
    if res == RX_CONTINUE { res = lowpan_rx_h_mesh(skb); }
    lowpan_rx_handlers_result(skb, res)
}

#[inline]
unsafe fn lowpan_is_nalp(dispatch: u8) -> bool { (dispatch & LOWPAN_DISPATCH_FIRST) == LOWPAN_DISPATCH_NALP }

/* Lookup for reserved dispatch values at:
 * https://www.iana.org/assignments/_6lowpan-parameters/_6lowpan-parameters.xhtml#_6lowpan-parameters-1
 *
 * Last Updated: 2015-01-22
 */
#[inline]
unsafe fn lowpan_is_reserved(dispatch: u8) -> bool {
    (dispatch >= 0x44 && dispatch <= 0x4f) ||
        (dispatch >= 0x51 && dispatch <= 0x5f) ||
        (dispatch >= 0xc8 && dispatch <= 0xdf) || dispatch >= 0xe8
}

/* lowpan_rx_h_check checks on generic 6LoWPAN requirements
 * in MAC and 6LoWPAN header.
 *
 * Don't manipulate the skb here, it could be shared buffer.
 */
#[inline]
unsafe fn lowpan_rx_h_check(skb: *mut sk_buff) -> bool {
    let fc: __le16 = ieee802154_get_fc_from_skb(skb);
    if !ieee802154_is_data(fc) || !ieee802154_skb_is_intra_pan_addressing(fc, skb) { return false; }
    if unlikely!((*skb).len == 0) { return false; }
    if lowpan_is_nalp(*skb_network_header(skb)) || lowpan_is_reserved(*skb_network_header(skb)) { return false; }
    true
}

unsafe fn lowpan_rcv(skb: *mut sk_buff, wdev: *mut net_device, _pt: *mut packet_type, _orig_wdev: *mut net_device) -> i32 {
    let mut ldev: *mut net_device;
    if (*wdev).type_ != ARPHRD_IEEE802154 || (*skb).pkt_type == PACKET_OTHERHOST || !lowpan_rx_h_check(skb) { kfree_skb(skb); return NET_RX_DROP; }
    ldev = (*(*wdev).ieee802154_ptr).lowpan_dev;
    if ldev.is_null() || !netif_running(ldev) { kfree_skb(skb); return NET_RX_DROP; }
    let mut skb = skb_share_check(skb, GFP_ATOMIC);
    if skb.is_null() { return NET_RX_DROP; }
    (*skb).dev = ldev;
    if lowpan_is_frag1(*skb_network_header(skb)) || lowpan_is_iphc(*skb_network_header(skb)) {
        skb = skb_unshare(skb, GFP_ATOMIC);
        if skb.is_null() { return NET_RX_DROP; }
    }
    lowpan_invoke_rx_handlers(skb)
}

static mut lowpan_packet_type: packet_type = packet_type {
    type_: htons(ETH_P_IEEE802154),
    func: Some(lowpan_rcv),
};

pub unsafe fn lowpan_rx_init() { dev_add_pack(&mut lowpan_packet_type); }
pub unsafe fn lowpan_rx_exit() { dev_remove_pack(&mut lowpan_packet_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
