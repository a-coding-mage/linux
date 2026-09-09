// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux kernel and ila.h are intentionally external.

pub unsafe fn ila_init_saved_csum(p: *mut ila_params) {
    if !(*p).locator_match.v64 {
        return;
    }

    (*p).csum_diff = compute_csum_diff8(
        &(*p).locator as *const _ as *const __be32,
        &(*p).locator_match as *const _ as *const __be32,
    );
}

unsafe fn get_csum_diff_iaddr(iaddr: *mut ila_addr, p: *mut ila_params) -> __wsum {
    if (*p).locator_match.v64 {
        (*p).csum_diff
    } else {
        compute_csum_diff8(
            &(*p).locator as *const _ as *const __be32,
            &(*iaddr).loc as *const _ as *const __be32,
        )
    }
}

unsafe fn get_csum_diff(ip6h: *mut ipv6hdr, p: *mut ila_params) -> __wsum {
    get_csum_diff_iaddr(ila_a2i(&mut (*ip6h).daddr), p)
}

unsafe fn ila_csum_do_neutral_fmt(iaddr: *mut ila_addr, p: *mut ila_params) {
    let adjust = &mut (*iaddr).ident.v16[3] as *mut _ as *mut __sum16;
    let mut diff: __wsum;
    let fval: __wsum;

    diff = get_csum_diff_iaddr(iaddr, p);

    fval = if ila_csum_neutral_set((*iaddr).ident) {
        CSUM_NEUTRAL_FLAG
    } else {
        !CSUM_NEUTRAL_FLAG
    } as __wsum;

    diff = csum_add(diff, fval);

    *adjust = !csum_fold(csum_add(diff, csum_unfold(*adjust)));

    /* Flip the csum-neutral bit. Either we are doing a SIR->ILA
     * translation with ILA_CSUM_NEUTRAL_MAP as the csum_method
     * and the C-bit is not set, or we are doing an ILA-SIR
     * tranlsation and the C-bit is set.
     */
    (*iaddr).ident.csum_neutral ^= 1;
}

unsafe fn ila_csum_do_neutral_nofmt(iaddr: *mut ila_addr, p: *mut ila_params) {
    let adjust = &mut (*iaddr).ident.v16[3] as *mut _ as *mut __sum16;
    let diff: __wsum;

    diff = get_csum_diff_iaddr(iaddr, p);

    *adjust = !csum_fold(csum_add(diff, csum_unfold(*adjust)));
}

unsafe fn ila_csum_adjust_transport(skb: *mut sk_buff, p: *mut ila_params) {
    let nhoff: usize = core::mem::size_of::<ipv6hdr>();
    let mut ip6h: *mut ipv6hdr = ipv6_hdr(skb);
    let mut diff: __wsum;

    match (*ip6h).nexthdr {
        NEXTHDR_TCP => {
            if pskb_may_pull(skb, nhoff + core::mem::size_of::<tcphdr>()) {
                let th = (skb_network_header(skb).add(nhoff)) as *mut tcphdr;

                ip6h = ipv6_hdr(skb);
                diff = get_csum_diff(ip6h, p);
                inet_proto_csum_replace_by_diff(&mut (*th).check, skb, diff, true, true);
            }
        }
        NEXTHDR_UDP => {
            if pskb_may_pull(skb, nhoff + core::mem::size_of::<udphdr>()) {
                let uh = (skb_network_header(skb).add(nhoff)) as *mut udphdr;

                if (*uh).check != 0 || (*skb).ip_summed == CHECKSUM_PARTIAL {
                    ip6h = ipv6_hdr(skb);
                    diff = get_csum_diff(ip6h, p);
                    inet_proto_csum_replace_by_diff(&mut (*uh).check, skb, diff, true, true);
                    if (*uh).check == 0 {
                        (*uh).check = CSUM_MANGLED_0;
                    }
                }
            }
        }
        NEXTHDR_ICMP => {
            if pskb_may_pull(skb, nhoff + core::mem::size_of::<icmp6hdr>()) {
                let ih = (skb_network_header(skb).add(nhoff)) as *mut icmp6hdr;

                ip6h = ipv6_hdr(skb);
                diff = get_csum_diff(ip6h, p);
                inet_proto_csum_replace_by_diff(&mut (*ih).icmp6_cksum, skb, diff, true, true);
            }
        }
        _ => {}
    }
}

pub unsafe fn ila_update_ipv6_locator(skb: *mut sk_buff, p: *mut ila_params, sir2ila: bool) {
    let mut ip6h: *mut ipv6hdr = ipv6_hdr(skb);
    let mut iaddr: *mut ila_addr = ila_a2i(&mut (*ip6h).daddr);

    'csum: loop {
    match (*p).csum_mode {
        ILA_CSUM_ADJUST_TRANSPORT => {
            ila_csum_adjust_transport(skb, p);
            /*
             * ila_csum_adjust_transport() calls pskb_may_pull(), which can
             * reallocate the skb head and leave ip6h (and the iaddr derived
             * from it) dangling; reload both before the write below.  The
             * other csum modes do not pull, so their cached pointers stay
             * valid.
             */
            ip6h = ipv6_hdr(skb);
            iaddr = ila_a2i(&mut (*ip6h).daddr);
        }
        ILA_CSUM_NEUTRAL_MAP => {
            if sir2ila {
                if WARN_ON(ila_csum_neutral_set((*iaddr).ident)) {
                    /* Checksum flag should never be
                     * set in a formatted SIR address.
                     */
                    break 'csum;
                }
            } else if !ila_csum_neutral_set((*iaddr).ident) {
                /* ILA to SIR translation and C-bit isn't
                 * set so we're good.
                 */
                break 'csum;
            }
            ila_csum_do_neutral_fmt(iaddr, p);
        }
        ILA_CSUM_NEUTRAL_MAP_AUTO => {
            ila_csum_do_neutral_nofmt(iaddr, p);
        }
        ILA_CSUM_NO_ACTION => {}
        _ => {}
    }
    break 'csum;
    }

    /* Now change destination address */
    (*iaddr).loc = (*p).locator;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
