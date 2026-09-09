// SPDX-License-Identifier: GPL-2.0-only
/*
 *  ebt_among
 *
 *  Authors:
 *  Grzegorz Borowiak <grzes@gnu.univ.gda.pl>
 *
 *  August, 2003
 */

// Kernel headers and build-time definitions supplied by the surrounding tree.

unsafe fn ebt_mac_wormhash_contains(
    wh: *const ebt_mac_wormhash,
    mac: *const c_char,
    ip: __be32,
) -> bool {
    /* You may be puzzled as to how this code works.
     * Some tricks were used, refer to
     * include/linux/netfilter_bridge/ebt_among.h
     * as there you can find a solution of this mystery.
     */
    let mut cmp: [u32; 2] = [0, 0];
    let key = *((mac as *const u8).add(5)) as usize;

    ether_addr_copy((cmp.as_mut_ptr() as *mut u8).add(2), mac as *const u8);
    let start = (*wh).table[key];
    let limit = (*wh).table[key + 1];
    let mut i = start;
    if ip != 0 {
        while i < limit {
            let p = &*(*wh).pool.add(i as usize);
            if cmp[1] == p.cmp[1] && cmp[0] == p.cmp[0]
                && (p.ip == 0 || p.ip == ip)
            {
                return true;
            }
            i += 1;
        }
    } else {
        while i < limit {
            let p = &*(*wh).pool.add(i as usize);
            if cmp[1] == p.cmp[1] && cmp[0] == p.cmp[0] && p.ip == 0 {
                return true;
            }
            i += 1;
        }
    }
    false
}

unsafe fn ebt_mac_wormhash_check_integrity(wh: *const ebt_mac_wormhash) -> c_int {
    let mut i = 0;
    while i < 256 {
        if (*wh).table[i] > (*wh).table[i + 1] { return -0x100 - i as c_int; }
        if (*wh).table[i] < 0 { return -0x200 - i as c_int; }
        if (*wh).table[i] > (*wh).poolsize { return -0x300 - i as c_int; }
        i += 1;
    }
    if (*wh).table[256] > (*wh).poolsize { return -0xc00; }
    0
}

unsafe fn get_ip_dst(skb: *const sk_buff, addr: *mut __be32) -> c_int {
    if (*eth_hdr(skb)).h_proto == htons(ETH_P_IP) {
        let mut iph = core::mem::MaybeUninit::<iphdr>::uninit();
        let ih = skb_header_pointer(skb, 0, core::mem::size_of::<iphdr>(), iph.as_mut_ptr() as *mut c_void);
        if ih.is_null() { return -1; }
        *addr = (*((ih) as *const iphdr)).daddr;
    } else if (*eth_hdr(skb)).h_proto == htons(ETH_P_ARP) {
        let mut arph = core::mem::MaybeUninit::<arphdr>::uninit();
        let ah = skb_header_pointer(skb, 0, core::mem::size_of::<arphdr>(), arph.as_mut_ptr() as *mut c_void);
        if ah.is_null() || (*((ah) as *const arphdr)).ar_pln != core::mem::size_of::<__be32>() as u8
            || (*((ah) as *const arphdr)).ar_hln != ETH_ALEN { return -1; }
        let mut buf: __be32 = 0;
        let bp = skb_header_pointer(skb, core::mem::size_of::<arphdr>() + 2 * ETH_ALEN + core::mem::size_of::<__be32>(), core::mem::size_of::<__be32>(), &mut buf as *mut _ as *mut c_void);
        if bp.is_null() { return -1; }
        *addr = *(bp as *const __be32);
    }
    0
}

unsafe fn get_ip_src(skb: *const sk_buff, addr: *mut __be32) -> c_int {
    if (*eth_hdr(skb)).h_proto == htons(ETH_P_IP) {
        let mut iph = core::mem::MaybeUninit::<iphdr>::uninit();
        let ih = skb_header_pointer(skb, 0, core::mem::size_of::<iphdr>(), iph.as_mut_ptr() as *mut c_void);
        if ih.is_null() { return -1; }
        *addr = (*(ih as *const iphdr)).saddr;
    } else if (*eth_hdr(skb)).h_proto == htons(ETH_P_ARP) {
        let mut arph = core::mem::MaybeUninit::<arphdr>::uninit();
        let ah = skb_header_pointer(skb, 0, core::mem::size_of::<arphdr>(), arph.as_mut_ptr() as *mut c_void);
        if ah.is_null() || (*(ah as *const arphdr)).ar_pln != core::mem::size_of::<__be32>() as u8
            || (*(ah as *const arphdr)).ar_hln != ETH_ALEN { return -1; }
        let mut buf: __be32 = 0;
        let bp = skb_header_pointer(skb, core::mem::size_of::<arphdr>() + ETH_ALEN, core::mem::size_of::<__be32>(), &mut buf as *mut _ as *mut c_void);
        if bp.is_null() { return -1; }
        *addr = *(bp as *const __be32);
    }
    0
}

unsafe fn ebt_among_mt(skb: *const sk_buff, par: *mut xt_action_param) -> bool {
    let info = (*par).matchinfo as *const ebt_among_info;
    let wh_dst = ebt_among_wh_dst(info);
    let wh_src = ebt_among_wh_src(info);
    let mut dip: __be32 = 0;
    let mut sip: __be32 = 0;
    if !wh_src.is_null() {
        let smac = (*eth_hdr(skb)).h_source.as_ptr() as *const c_char;
        if get_ip_src(skb, &mut sip) != 0 { return false; }
        if ((*info).bitmask & EBT_AMONG_SRC_NEG) == 0 {
            if !ebt_mac_wormhash_contains(wh_src, smac, sip) { return false; }
        } else if ebt_mac_wormhash_contains(wh_src, smac, sip) { return false; }
    }
    if !wh_dst.is_null() {
        let dmac = (*eth_hdr(skb)).h_dest.as_ptr() as *const c_char;
        if get_ip_dst(skb, &mut dip) != 0 { return false; }
        if ((*info).bitmask & EBT_AMONG_DST_NEG) == 0 {
            if !ebt_mac_wormhash_contains(wh_dst, dmac, dip) { return false; }
        } else if ebt_mac_wormhash_contains(wh_dst, dmac, dip) { return false; }
    }
    true
}

unsafe fn poolsize_invalid(w: *const ebt_mac_wormhash) -> bool {
    !w.is_null() && (*w).poolsize >= (INT_MAX / core::mem::size_of::<ebt_mac_wormhash_tuple>() as c_int)
}

unsafe fn wormhash_offset_invalid(mut off: c_int, len: c_uint) -> bool {
    if off == 0 { return false; }
    if off < core::mem::size_of::<ebt_among_info>() as c_int || off % core::mem::align_of::<ebt_mac_wormhash>() as c_int != 0 { return true; }
    off += core::mem::size_of::<ebt_mac_wormhash>() as c_int;
    off as c_uint > len
}

unsafe fn wormhash_sizes_valid(wh: *const ebt_mac_wormhash, mut a: c_int, b: c_int) -> bool {
    if a == 0 { a = core::mem::size_of::<ebt_among_info>() as c_int; }
    ebt_mac_wormhash_size(wh) + a == b
}

unsafe fn ebt_among_mt_check(par: *const xt_mtchk_param) -> c_int {
    let info = (*par).matchinfo as *const ebt_among_info;
    let em = container_of_match((*par).matchinfo);
    let mut expected_length = core::mem::size_of::<ebt_among_info>() as c_int;
    if expected_length as c_uint > (*em).match_size { return -EINVAL; }
    if wormhash_offset_invalid((*info).wh_dst_ofs, (*em).match_size) || wormhash_offset_invalid((*info).wh_src_ofs, (*em).match_size) { return -EINVAL; }
    let wh_dst = ebt_among_wh_dst(info);
    if poolsize_invalid(wh_dst) { return -EINVAL; }
    expected_length += ebt_mac_wormhash_size(wh_dst);
    if expected_length as c_uint > (*em).match_size { return -EINVAL; }
    let wh_src = ebt_among_wh_src(info);
    if poolsize_invalid(wh_src) { return -EINVAL; }
    if (*info).wh_src_ofs < (*info).wh_dst_ofs {
        if !wormhash_sizes_valid(wh_src, (*info).wh_src_ofs, (*info).wh_dst_ofs) { return -EINVAL; }
    } else if !wormhash_sizes_valid(wh_dst, (*info).wh_dst_ofs, (*info).wh_src_ofs) { return -EINVAL; }
    expected_length += ebt_mac_wormhash_size(wh_src);
    if (*em).match_size != EBT_ALIGN(expected_length as usize) as c_uint { return -EINVAL; }
    if !wh_dst.is_null() && ebt_mac_wormhash_check_integrity(wh_dst) != 0 { return -EINVAL; }
    if !wh_src.is_null() && ebt_mac_wormhash_check_integrity(wh_src) != 0 { return -EINVAL; }
    0
}

// Registration, module metadata, and external kernel declarations are provided by the kernel build environment.
extern "C" {
    fn ebt_among_wh_dst(info: *const ebt_among_info) -> *const ebt_mac_wormhash;
    fn ebt_among_wh_src(info: *const ebt_among_info) -> *const ebt_mac_wormhash;
    fn ebt_mac_wormhash_size(wh: *const ebt_mac_wormhash) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
