// SPDX-License-Identifier: GPL-2.0-only
/*
 * IPv6 library code, needed by static components when full IPv6 support is
 * not configured or static.
 */

/* External kernel declarations and constants are supplied by other files. */

/*
 * find out if nexthdr is a well-known extension header or a protocol
 */
pub unsafe fn ipv6_ext_hdr(nexthdr: u8) -> bool {
    /* find out if nexthdr is an extension header or a protocol */
    (nexthdr == NEXTHDR_HOP)
        || (nexthdr == NEXTHDR_ROUTING)
        || (nexthdr == NEXTHDR_FRAGMENT)
        || (nexthdr == NEXTHDR_AUTH)
        || (nexthdr == NEXTHDR_NONE)
        || (nexthdr == NEXTHDR_DEST)
}

/*
 * Skip any extension headers. This is used by the ICMP module.
 *
 * This function parses (probably truncated) exthdr set "hdr".
 * It skips all well-known exthdrs, and returns the pointer offset of the
 * first header with unknown type.
 */
pub unsafe fn ipv6_skip_exthdr(
    skb: *const struct_sk_buff,
    mut start: i32,
    nexthdrp: *mut u8,
    frag_offp: *mut __be16,
) -> i32 {
    let mut nexthdr = *nexthdrp;
    let mut exthdr_cnt = 0;

    *frag_offp = 0;

    while ipv6_ext_hdr(nexthdr) {
        let mut _hdr = core::mem::MaybeUninit::<struct_ipv6_opt_hdr>::uninit();
        let hp: *const struct_ipv6_opt_hdr;
        let hdrlen: i32;

        if nexthdr == NEXTHDR_NONE {
            return -1;
        }
        exthdr_cnt += 1;
        if exthdr_cnt >= IP6_MAX_EXT_HDRS_CNT {
            return -1;
        }
        hp = skb_header_pointer(
            skb,
            start,
            core::mem::size_of::<struct_ipv6_opt_hdr>() as i32,
            _hdr.as_mut_ptr() as *mut core::ffi::c_void,
        ) as *const struct_ipv6_opt_hdr;
        if hp.is_null() {
            return -1;
        }
        if nexthdr == NEXTHDR_FRAGMENT {
            let mut _frag_off: __be16 = 0;
            let fp = skb_header_pointer(
                skb,
                start + core::mem::offset_of!(struct_frag_hdr, frag_off) as i32,
                core::mem::size_of::<__be16>() as i32,
                (&mut _frag_off as *mut __be16).cast(),
            ) as *const __be16;
            if fp.is_null() {
                return -1;
            }
            *frag_offp = *fp;
            if ntohs(*frag_offp) & !0x7 != 0 {
                break;
            }
            hdrlen = 8;
        } else if nexthdr == NEXTHDR_AUTH {
            hdrlen = ipv6_authlen(hp);
        } else {
            hdrlen = ipv6_optlen(hp);
        }

        nexthdr = (*hp).nexthdr;
        start += hdrlen;
    }

    *nexthdrp = nexthdr;
    start
}

pub unsafe fn ipv6_find_tlv(skb: *const struct_sk_buff, mut offset: i32, type_: i32) -> i32 {
    let nh = skb_network_header(skb);
    let packet_len = skb_tail_pointer(skb).offset_from(nh) as i32;
    let hdr = (nh as *mut u8).add(offset as usize) as *mut struct_ipv6_opt_hdr;
    let mut len: i32;

    if offset + 2 > packet_len {
        return -1;
    }
    len = (((*hdr).hdrlen as i32 + 1) << 3);
    if offset + len > packet_len {
        return -1;
    }

    offset += 2;
    len -= 2;
    while len > 0 {
        let opttype = *nh.add(offset as usize);
        let optlen: i32;
        if opttype as i32 == type_ {
            return offset;
        }
        if opttype == IPV6_TLV_PAD1 {
            optlen = 1;
        } else {
            if len < 2 {
                return -1;
            }
            optlen = *nh.add((offset + 1) as usize) as i32 + 2;
            if optlen > len {
                return -1;
            }
        }
        offset += optlen;
        len -= optlen;
    }
    -1
}

pub unsafe fn ipv6_find_hdr(
    skb: *const struct_sk_buff,
    offset: *mut u32,
    target: i32,
    fragoff: *mut u16,
    flags: *mut i32,
) -> i32 {
    let mut start = skb_network_offset(skb) + core::mem::size_of::<struct_ipv6hdr>() as u32;
    let ip6 = ipv6_hdr(skb);
    let mut nexthdr = (*ip6).nexthdr;
    let mut exthdr_cnt = 0;
    let mut found;

    if !fragoff.is_null() {
        *fragoff = 0;
    }

    if *offset != 0 {
        let mut _ip6 = core::mem::MaybeUninit::<struct_ipv6hdr>::uninit();
        let ip6p = skb_header_pointer(
            skb,
            *offset as i32,
            core::mem::size_of::<struct_ipv6hdr>() as i32,
            _ip6.as_mut_ptr().cast(),
        ) as *const struct_ipv6hdr;
        if ip6p.is_null() || (*ip6p).version != 6 {
            return -EBADMSG;
        }
        start = *offset + core::mem::size_of::<struct_ipv6hdr>() as u32;
        nexthdr = (*ip6p).nexthdr;
    }

    loop {
        let mut _hdr = core::mem::MaybeUninit::<struct_ipv6_opt_hdr>::uninit();
        found = nexthdr as i32 == target;
        if !ipv6_ext_hdr(nexthdr) || nexthdr == NEXTHDR_NONE {
            if target < 0 || found {
                break;
            }
            return -ENOENT;
        }
        exthdr_cnt += 1;
        if exthdr_cnt >= IP6_MAX_EXT_HDRS_CNT {
            return -EBADMSG;
        }
        let hp = skb_header_pointer(
            skb,
            start as i32,
            core::mem::size_of::<struct_ipv6_opt_hdr>() as i32,
            _hdr.as_mut_ptr().cast(),
        ) as *const struct_ipv6_opt_hdr;
        if hp.is_null() {
            return -EBADMSG;
        }

        if nexthdr == NEXTHDR_ROUTING {
            let mut _rh = core::mem::MaybeUninit::<struct_ipv6_rt_hdr>::uninit();
            let rh = skb_header_pointer(skb, start as i32, core::mem::size_of::<struct_ipv6_rt_hdr>() as i32, _rh.as_mut_ptr().cast()) as *const struct_ipv6_rt_hdr;
            if rh.is_null() { return -EBADMSG; }
            if !flags.is_null() && (*flags & IP6_FH_F_SKIP_RH) != 0 && (*rh).segments_left == 0 { found = false; }
        }

        let hdrlen: u32;
        if nexthdr == NEXTHDR_FRAGMENT {
            if !flags.is_null() { *flags |= IP6_FH_F_FRAG; }
            let mut _frag_off: u16 = 0;
            let fp = skb_header_pointer(skb, (start + core::mem::offset_of!(struct_frag_hdr, frag_off) as u32) as i32, core::mem::size_of::<u16>() as i32, (&mut _frag_off as *mut u16).cast()) as *const __be16;
            if fp.is_null() { return -EBADMSG; }
            _frag_off = ntohs(*fp) & !0x7;
            if _frag_off != 0 {
                if target < 0 && (!ipv6_ext_hdr((*hp).nexthdr) || (*hp).nexthdr == NEXTHDR_NONE) {
                    if !fragoff.is_null() { *fragoff = _frag_off; }
                    return (*hp).nexthdr as i32;
                }
                if !found { return -ENOENT; }
                if !fragoff.is_null() { *fragoff = _frag_off; }
                break;
            }
            hdrlen = 8;
        } else if nexthdr == NEXTHDR_AUTH {
            if !flags.is_null() && (*flags & IP6_FH_F_AUTH) != 0 && target < 0 { break; }
            hdrlen = ipv6_authlen(hp) as u32;
        } else {
            hdrlen = ipv6_optlen(hp) as u32;
        }
        if !found {
            nexthdr = (*hp).nexthdr;
            start += hdrlen;
        }
        if found { break; }
    }
    *offset = start;
    nexthdr as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
