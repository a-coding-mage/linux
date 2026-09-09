// SPDX-License-Identifier: GPL-2.0+
/* Regular and Ethertype DSA tagging. */

// Kernel headers and tag.h provide the external types, constants, macros, and
// functions referenced below.

const DSA_NAME: &str = "dsa";
const EDSA_NAME: &str = "edsa";
const DSA_HLEN: usize = 4;

#[repr(i32)]
enum DsaCmd { ToCpu = 0, FromCpu = 1, ToSniffer = 2, Forward = 3 }

#[repr(i32)]
enum DsaCode {
    MgmtTrap = 0, Frame2Reg = 1, IgmpMldTrap = 2, PolicyTrap = 3,
    ArpMirror = 4, PolicyMirror = 5, Reserved6 = 6, Reserved7 = 7,
}

unsafe fn dsa_xmit_ll(skb: *mut sk_buff, dev: *mut net_device, extra: u8) -> *mut sk_buff {
    let dp = dsa_user_to_port(dev);
    let mut br_dev: *mut net_device;
    let mut tag_dev: u8;
    let mut tag_port: u8;
    let cmd: DsaCmd;
    let dsa_header: *mut u8;

    if (*skb).offload_fwd_mark {
        let bridge_num = dsa_port_bridge_num_get(dp);
        let dst = (*(*dp).ds).dst;
        cmd = DsaCmd::Forward;
        tag_dev = (*dst).last_switch.wrapping_add(bridge_num);
        tag_port = 0;
    } else {
        cmd = DsaCmd::FromCpu;
        tag_dev = (*(*dp).ds).index;
        tag_port = (*dp).index;
    }

    br_dev = dsa_port_bridge_dev_get(dp);
    if (*skb).protocol == htons(ETH_P_8021Q) && (br_dev.is_null() || br_vlan_enabled(br_dev)) {
        if extra != 0 {
            skb_push(skb, extra as usize);
            dsa_alloc_etype_header(skb, extra as usize);
        }
        dsa_header = dsa_etype_header_pos_tx(skb).add(extra as usize);
        *dsa_header = ((cmd as u8) << 6) | 0x20 | tag_dev;
        *dsa_header.add(1) = tag_port << 3;
        if *dsa_header.add(2) & 0x10 != 0 {
            *dsa_header.add(1) |= 0x01;
            *dsa_header.add(2) &= !0x10;
        }
    } else {
        let vid: u16 = if !br_dev.is_null() { MV88E6XXX_VID_BRIDGED } else { MV88E6XXX_VID_STANDALONE };
        skb_push(skb, DSA_HLEN + extra as usize);
        dsa_alloc_etype_header(skb, DSA_HLEN + extra as usize);
        dsa_header = dsa_etype_header_pos_tx(skb).add(extra as usize);
        *dsa_header = (cmd as u8) << 6 | tag_dev;
        *dsa_header.add(1) = tag_port << 3;
        *dsa_header.add(2) = (vid >> 8) as u8;
        *dsa_header.add(3) = vid as u8;
    }
    skb
}

unsafe fn dsa_rcv_ll(skb: *mut sk_buff, dev: *mut net_device, extra: u8) -> *mut sk_buff {
    let mut trap = false;
    let mut trunk = false;
    let dsa_header = dsa_etype_header_pos_rx(skb);
    let cmd = *dsa_header >> 6;
    match cmd {
        3 => trunk = *dsa_header.add(1) & 4 != 0,
        0 => {
            let code = (*dsa_header.add(1) & 0x6) | ((*dsa_header.add(2) >> 4) & 1);
            match code {
                1 => { kfree_skb(skb); return core::ptr::null_mut(); }
                4 | 5 => {}
                0 | 2 | 3 => trap = true,
                _ => { kfree_skb(skb); return core::ptr::null_mut(); }
            }
        }
        _ => { kfree_skb(skb); return core::ptr::null_mut(); }
    }

    let source_device = *dsa_header & 0x1f;
    let source_port = (*dsa_header.add(1) >> 3) & 0x1f;
    if trunk {
        let cpu_dp = (*dev).dsa_ptr;
        let lag = dsa_lag_by_id((*cpu_dp).dst, source_port as i32 + 1);
        (*skb).dev = if lag.is_null() { core::ptr::null_mut() } else { (*lag).dev };
    } else {
        (*skb).dev = dsa_conduit_find_user(dev, source_device, source_port);
    }
    if (*skb).dev.is_null() { kfree_skb(skb); return core::ptr::null_mut(); }
    if trunk { (*skb).offload_fwd_mark = true; } else if !trap { dsa_default_offload_fwd_mark(skb); }

    if *dsa_header & 0x20 != 0 {
        let mut new_header = [0u8; 4];
        new_header[0] = (ETH_P_8021Q >> 8) as u8;
        new_header[1] = ETH_P_8021Q as u8;
        new_header[2] = *dsa_header.add(2) & !0x10;
        new_header[3] = *dsa_header.add(3);
        if *dsa_header.add(1) & 1 != 0 { new_header[2] |= 0x10; }
        if (*skb).ip_summed == CHECKSUM_COMPLETE {
            let mut c = (*skb).csum;
            c = csum_add(c, csum_partial(new_header.as_ptr().add(2), 2, 0));
            c = csum_sub(c, csum_partial(dsa_header.add(2), 2, 0));
            (*skb).csum = c;
        }
        core::ptr::copy_nonoverlapping(new_header.as_ptr(), dsa_header, DSA_HLEN);
        if extra != 0 { dsa_strip_etype_header(skb, extra as usize); }
    } else {
        skb_pull_rcsum(skb, DSA_HLEN);
        dsa_strip_etype_header(skb, DSA_HLEN + extra as usize);
    }
    skb
}

unsafe fn dsa_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff { dsa_xmit_ll(skb, dev, 0) }
unsafe fn dsa_rcv(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    if !pskb_may_pull(skb, DSA_HLEN) { kfree_skb(skb); return core::ptr::null_mut(); }
    dsa_rcv_ll(skb, dev, 0)
}

// CONFIG_NET_DSA_TAG_DSA conditionally registers dsa_netdev_ops.

const EDSA_HLEN: usize = 8;
unsafe fn edsa_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let skb = dsa_xmit_ll(skb, dev, (EDSA_HLEN - DSA_HLEN) as u8);
    if skb.is_null() { return core::ptr::null_mut(); }
    let h = dsa_etype_header_pos_tx(skb);
    *h = (ETH_P_EDSA >> 8) as u8; *h.add(1) = ETH_P_EDSA as u8; *h.add(2) = 0; *h.add(3) = 0;
    skb
}
unsafe fn edsa_rcv(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    if !pskb_may_pull(skb, EDSA_HLEN) { kfree_skb(skb); return core::ptr::null_mut(); }
    skb_pull_rcsum(skb, EDSA_HLEN - DSA_HLEN);
    dsa_rcv_ll(skb, dev, (EDSA_HLEN - DSA_HLEN) as u8)
}

// CONFIG_NET_DSA_TAG_EDSA conditionally registers edsa_netdev_ops.
// The module registration and aliases are supplied by the kernel tag-driver API.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
