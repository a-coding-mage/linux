// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Authors:
 * (C) 2016 Pengutronix, Alexander Aring <aar@pengutronix.de>
 */

// Dependencies supplied by the surrounding kernel translation.

// #if IS_ENABLED(CONFIG_IEEE802154_6LOWPAN)
// The following items are conditionally compiled when IEEE 802.15.4 6LoWPAN
// support is enabled in the surrounding build.
const NDISC_802154_SHORT_ADDR_LENGTH: u8 = 1;

unsafe fn lowpan_ndisc_parse_802154_options(
    dev: *const net_device,
    nd_opt: *mut nd_opt_hdr,
    ndopts: *mut ndisc_options,
) -> i32 {
    match (*nd_opt).nd_opt_len {
        NDISC_802154_SHORT_ADDR_LENGTH => {
            if !(*ndopts).nd_802154_opt_array[(*nd_opt).nd_opt_type as usize].is_null() {
                net_dbg_ratelimited!(
                    "%s: duplicated short addr ND6 option found: type=%d\n",
                    "lowpan_ndisc_parse_802154_options",
                    (*nd_opt).nd_opt_type
                );
            } else {
                (*ndopts).nd_802154_opt_array[(*nd_opt).nd_opt_type as usize] = nd_opt;
            }
            1
        }
        _ => {
            // all others will be handled by ndisc IPv6 option parsing
            0
        }
    }
}

unsafe fn lowpan_ndisc_parse_options(
    dev: *const net_device,
    nd_opt: *mut nd_opt_hdr,
    ndopts: *mut ndisc_options,
) -> i32 {
    if !lowpan_is_ll(dev, LOWPAN_LLTYPE_IEEE802154) {
        return 0;
    }

    match (*nd_opt).nd_opt_type {
        ND_OPT_SOURCE_LL_ADDR | ND_OPT_TARGET_LL_ADDR => {
            lowpan_ndisc_parse_802154_options(dev, nd_opt, ndopts)
        }
        _ => 0,
    }
}

unsafe fn lowpan_ndisc_802154_update(
    n: *mut neighbour,
    flags: u32,
    icmp6_type: u8,
    ndopts: *const ndisc_options,
) {
    let neigh = lowpan_802154_neigh(neighbour_priv(n));
    let mut lladdr_short: *mut u8 = core::ptr::null_mut();

    match icmp6_type {
        NDISC_ROUTER_SOLICITATION | NDISC_ROUTER_ADVERTISEMENT | NDISC_NEIGHBOUR_SOLICITATION => {
            if !(*ndopts).nd_802154_opts_src_lladdr.is_null() {
                lladdr_short = __ndisc_opt_addr_data(
                    (*ndopts).nd_802154_opts_src_lladdr,
                    IEEE802154_SHORT_ADDR_LEN,
                    0,
                );
                if lladdr_short.is_null() {
                    net_dbg_ratelimited!("NA: invalid short link-layer address length\n");
                    return;
                }
            }
        }
        NDISC_REDIRECT | NDISC_NEIGHBOUR_ADVERTISEMENT => {
            if !(*ndopts).nd_802154_opts_tgt_lladdr.is_null() {
                lladdr_short = __ndisc_opt_addr_data(
                    (*ndopts).nd_802154_opts_tgt_lladdr,
                    IEEE802154_SHORT_ADDR_LEN,
                    0,
                );
                if lladdr_short.is_null() {
                    net_dbg_ratelimited!("NA: invalid short link-layer address length\n");
                    return;
                }
            }
        }
        _ => {}
    }

    write_lock_bh(&mut (*n).lock);
    if !lladdr_short.is_null() {
        ieee802154_be16_to_le16(&mut (*neigh).short_addr, lladdr_short);
        if !lowpan_802154_is_valid_src_short_addr((*neigh).short_addr) {
            (*neigh).short_addr = cpu_to_le16(IEEE802154_ADDR_SHORT_UNSPEC);
        }
    }
    write_unlock_bh(&mut (*n).lock);
}

unsafe fn lowpan_ndisc_update(
    dev: *const net_device,
    n: *mut neighbour,
    flags: u32,
    icmp6_type: u8,
    ndopts: *const ndisc_options,
) {
    if !lowpan_is_ll(dev, LOWPAN_LLTYPE_IEEE802154) {
        return;
    }

    // react on overrides only. TODO check if this is really right.
    if flags & NEIGH_UPDATE_F_OVERRIDE != 0 {
        lowpan_ndisc_802154_update(n, flags, icmp6_type, ndopts);
    }
}

unsafe fn lowpan_ndisc_opt_addr_space(
    dev: *const net_device,
    icmp6_type: u8,
    neigh: *mut neighbour,
    ha_buf: *mut u8,
    ha: *mut *mut u8,
) -> i32 {
    let mut addr_space = 0;

    if !lowpan_is_ll(dev, LOWPAN_LLTYPE_IEEE802154) {
        return 0;
    }

    match icmp6_type {
        NDISC_REDIRECT => {
            let n = lowpan_802154_neigh(neighbour_priv(neigh));
            read_lock_bh(&mut (*neigh).lock);
            if lowpan_802154_is_valid_src_short_addr((*n).short_addr) {
                core::ptr::copy_nonoverlapping(
                    &(*n).short_addr as *const _ as *const u8,
                    ha_buf,
                    IEEE802154_SHORT_ADDR_LEN as usize,
                );
                read_unlock_bh(&mut (*neigh).lock);
                addr_space += __ndisc_opt_addr_space(IEEE802154_SHORT_ADDR_LEN, 0);
                *ha = ha_buf;
            } else {
                read_unlock_bh(&mut (*neigh).lock);
            }
        }
        NDISC_NEIGHBOUR_ADVERTISEMENT | NDISC_NEIGHBOUR_SOLICITATION
        | NDISC_ROUTER_SOLICITATION => {
            let wpan_dev = (*lowpan_802154_dev(dev)).wdev.ieee802154_ptr;
            if lowpan_802154_is_valid_src_short_addr((*wpan_dev).short_addr) {
                addr_space = __ndisc_opt_addr_space(IEEE802154_SHORT_ADDR_LEN, 0);
            }
        }
        _ => {}
    }

    addr_space
}

unsafe fn lowpan_ndisc_fill_addr_option(
    dev: *const net_device,
    skb: *mut sk_buff,
    icmp6_type: u8,
    ha: *const u8,
) {
    if !lowpan_is_ll(dev, LOWPAN_LLTYPE_IEEE802154) {
        return;
    }

    let mut short_addr: __be16 = core::mem::zeroed();
    let opt_type: u8;
    match icmp6_type {
        NDISC_REDIRECT => {
            if !ha.is_null() {
                ieee802154_le16_to_be16(&mut short_addr, ha);
                __ndisc_fill_addr_option(
                    skb,
                    ND_OPT_TARGET_LL_ADDR,
                    &short_addr,
                    IEEE802154_SHORT_ADDR_LEN,
                    0,
                );
            }
            return;
        }
        NDISC_NEIGHBOUR_ADVERTISEMENT => opt_type = ND_OPT_TARGET_LL_ADDR,
        NDISC_ROUTER_SOLICITATION | NDISC_NEIGHBOUR_SOLICITATION => {
            opt_type = ND_OPT_SOURCE_LL_ADDR
        }
        _ => return,
    }

    let wpan_dev = (*lowpan_802154_dev(dev)).wdev.ieee802154_ptr;
    if lowpan_802154_is_valid_src_short_addr((*wpan_dev).short_addr) {
        ieee802154_le16_to_be16(&mut short_addr, &(*wpan_dev).short_addr);
        __ndisc_fill_addr_option(
            skb,
            opt_type,
            &short_addr,
            IEEE802154_SHORT_ADDR_LEN,
            0,
        );
    }
}

unsafe fn lowpan_ndisc_prefix_rcv_add_addr(
    net: *mut net,
    dev: *mut net_device,
    pinfo: *const prefix_info,
    in6_dev: *mut inet6_dev,
    addr: *mut in6_addr,
    addr_type: i32,
    addr_flags: u32,
    sllao: bool,
    tokenized: bool,
    valid_lft: __u32,
    prefered_lft: u32,
    dev_addr_generated: bool,
) {
    // generates short based address for RA PIO's
    if lowpan_is_ll(dev, LOWPAN_LLTYPE_IEEE802154)
        && dev_addr_generated
        && !addrconf_ifid_802154_6lowpan((*addr).s6_addr.as_mut_ptr().add(8), dev)
    {
        let err = addrconf_prefix_rcv_add_addr(
            net, dev, pinfo, in6_dev, addr, addr_type, addr_flags, sllao, tokenized,
            valid_lft, prefered_lft,
        );
        if err != 0 {
            net_dbg_ratelimited!(
                "RA: could not add a short address based address for prefix: %pI6c\n",
                &(*pinfo).prefix
            );
        }
    }
}
// #endif

static lowpan_ndisc_ops: ndisc_ops = ndisc_ops {
    // #if IS_ENABLED(CONFIG_IEEE802154_6LOWPAN)
    parse_options: Some(lowpan_ndisc_parse_options),
    update: Some(lowpan_ndisc_update),
    opt_addr_space: Some(lowpan_ndisc_opt_addr_space),
    fill_addr_option: Some(lowpan_ndisc_fill_addr_option),
    prefix_rcv_add_addr: Some(lowpan_ndisc_prefix_rcv_add_addr),
    // #endif
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
