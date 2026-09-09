// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Spanning tree protocol; BPDU handling
 * Linux ethernet bridge
 *
 * Authors:
 * Lennert Buytenhek <buytenh@gnu.org>
 */

const STP_HZ: c_ulong = 256;
const LLC_RESERVE: usize = core::mem::size_of::<llc_pdu_un>();

macro_rules! goto_err {
    ($skb:expr) => {{
        kfree_skb($skb);
        return;
    }};
}

macro_rules! goto_out {
    ($br:expr, $skb:expr) => {{
        spin_unlock(&mut (*$br).lock);
        kfree_skb($skb);
        return;
    }};
}

unsafe fn br_send_bpdu_finish(
    _net: *mut net,
    _sk: *mut sock,
    skb: *mut sk_buff,
) -> c_int {
    dev_queue_xmit(skb)
}

unsafe fn br_send_bpdu(p: *mut net_bridge_port, data: *const u8, length: c_int) {
    let skb = dev_alloc_skb((length as usize) + LLC_RESERVE);
    if skb.is_null() {
        return;
    }

    (*skb).dev = (*p).dev;
    (*skb).protocol = htons(ETH_P_802_2);
    (*skb).priority = TC_PRIO_CONTROL;

    skb_reserve(skb, LLC_RESERVE);
    __skb_put_data(skb, data, length as usize);

    llc_pdu_header_init(
        skb,
        LLC_PDU_TYPE_U,
        LLC_SAP_BSPAN,
        LLC_SAP_BSPAN,
        LLC_PDU_CMD,
    );
    llc_pdu_init_as_ui_cmd(skb);

    llc_mac_hdr_init(skb, (*(*p).dev).dev_addr.as_ptr(), (*(*p).br).group_addr.as_ptr());

    skb_reset_mac_header(skb);

    NF_HOOK(
        NFPROTO_BRIDGE,
        NF_BR_LOCAL_OUT,
        dev_net((*p).dev),
        core::ptr::null_mut(),
        skb,
        core::ptr::null_mut(),
        (*skb).dev,
        br_send_bpdu_finish,
    );
}

unsafe fn br_set_ticks(dest: *mut u8, j: c_int) {
    let ticks: c_ulong = (STP_HZ * j as c_ulong) / HZ as c_ulong;
    put_unaligned_be16(ticks as u16, dest);
}

unsafe fn br_get_ticks(src: *const u8) -> c_int {
    let ticks: c_ulong = get_unaligned_be16(src) as c_ulong;
    div_round_up(ticks * HZ as c_ulong, STP_HZ) as c_int
}

/* called under bridge lock */
pub unsafe fn br_send_config_bpdu(p: *mut net_bridge_port, bpdu: *mut br_config_bpdu) {
    let mut buf = [0u8; 35];

    if (*(*p).br).stp_enabled != BR_KERNEL_STP {
        return;
    }

    buf[0] = 0;
    buf[1] = 0;
    buf[2] = 0;
    buf[3] = BPDU_TYPE_CONFIG;
    buf[4] = ((*bpdu).topology_change as u8) | (((*bpdu).topology_change_ack as u8) << 7);
    buf[5] = (*bpdu).root.prio[0];
    buf[6] = (*bpdu).root.prio[1];
    buf[7..13].copy_from_slice(&(*bpdu).root.addr);
    buf[13] = ((*bpdu).root_path_cost >> 24) as u8;
    buf[14] = ((*bpdu).root_path_cost >> 16) as u8;
    buf[15] = ((*bpdu).root_path_cost >> 8) as u8;
    buf[16] = (*bpdu).root_path_cost as u8;
    buf[17] = (*bpdu).bridge_id.prio[0];
    buf[18] = (*bpdu).bridge_id.prio[1];
    buf[19..25].copy_from_slice(&(*bpdu).bridge_id.addr);
    buf[25] = ((*bpdu).port_id >> 8) as u8;
    buf[26] = (*bpdu).port_id as u8;

    br_set_ticks(buf.as_mut_ptr().add(27), (*bpdu).message_age);
    br_set_ticks(buf.as_mut_ptr().add(29), (*bpdu).max_age);
    br_set_ticks(buf.as_mut_ptr().add(31), (*bpdu).hello_time);
    br_set_ticks(buf.as_mut_ptr().add(33), (*bpdu).forward_delay);

    br_send_bpdu(p, buf.as_ptr(), 35);
    (*p).stp_xstats.tx_bpdu += 1;
}

/* called under bridge lock */
pub unsafe fn br_send_tcn_bpdu(p: *mut net_bridge_port) {
    let mut buf = [0u8; 4];

    if (*(*p).br).stp_enabled != BR_KERNEL_STP {
        return;
    }

    buf[0] = 0;
    buf[1] = 0;
    buf[2] = 0;
    buf[3] = BPDU_TYPE_TCN;
    br_send_bpdu(p, buf.as_ptr(), 4);
    (*p).stp_xstats.tx_tcn += 1;
}

/*
 * Called from llc.
 *
 * NO locks, but rcu_read_lock
 */
pub unsafe fn br_stp_rcv(
    _proto: *const stp_proto,
    skb: *mut sk_buff,
    dev: *mut net_device,
) {
    let p: *mut net_bridge_port;
    let br: *mut net_bridge;
    let mut buf: *const u8;

    if !pskb_may_pull(skb, 4) {
        goto_err!(skb);
    }

    buf = (*skb).data;
    if *buf != 0 || *buf.add(1) != 0 || *buf.add(2) != 0 {
        goto_err!(skb);
    }

    p = br_port_get_check_rcu(dev);
    if p.is_null() {
        goto_err!(skb);
    }

    br = (*p).br;
    spin_lock(&mut (*br).lock);

    if (*br).stp_enabled != BR_KERNEL_STP {
        goto_out!(br, skb);
    }
    if (*(*br).dev).flags & IFF_UP == 0 {
        goto_out!(br, skb);
    }
    if (*p).state == BR_STATE_DISABLED {
        goto_out!(br, skb);
    }
    if !ether_addr_equal(eth_hdr(skb).h_dest.as_ptr(), (*br).group_addr.as_ptr()) {
        goto_out!(br, skb);
    }

    if test_bit(BR_BPDU_GUARD_BIT, &(*p).flags) {
        br_notice(
            br,
            "BPDU received on blocked port %u(%s)\n",
            (*p).port_no as c_uint,
            (*(*p).dev).name.as_ptr(),
        );
        br_stp_disable_port(p);
        goto_out!(br, skb);
    }

    buf = skb_pull(skb, 3);
    if *buf == BPDU_TYPE_CONFIG {
        let mut bpdu: br_config_bpdu = core::mem::zeroed();
        if !pskb_may_pull(skb, 32) {
            goto_out!(br, skb);
        }
        buf = (*skb).data;
        bpdu.topology_change = if *buf.add(1) & 1 != 0 { 1 } else { 0 };
        bpdu.topology_change_ack = if *buf.add(1) & 0x80 != 0 { 1 } else { 0 };
        bpdu.root.prio[0] = *buf.add(2);
        bpdu.root.prio[1] = *buf.add(3);
        core::ptr::copy_nonoverlapping(buf.add(4), bpdu.root.addr.as_mut_ptr(), 6);
        bpdu.root_path_cost = ((*buf.add(10) as u32) << 24)
            | ((*buf.add(11) as u32) << 16)
            | ((*buf.add(12) as u32) << 8)
            | *buf.add(13) as u32;
        bpdu.bridge_id.prio[0] = *buf.add(14);
        bpdu.bridge_id.prio[1] = *buf.add(15);
        core::ptr::copy_nonoverlapping(buf.add(16), bpdu.bridge_id.addr.as_mut_ptr(), 6);
        bpdu.port_id = ((*buf.add(22) as u16) << 8) | *buf.add(23) as u16;
        bpdu.message_age = br_get_ticks(buf.add(24));
        bpdu.max_age = br_get_ticks(buf.add(26));
        bpdu.hello_time = br_get_ticks(buf.add(28));
        bpdu.forward_delay = br_get_ticks(buf.add(30));
        if bpdu.message_age > bpdu.max_age {
            if net_ratelimit() {
                br_notice(
                    (*p).br,
                    "port %u config from %pM (message_age %ul > max_age %ul)\n",
                    (*p).port_no,
                    eth_hdr(skb).h_source.as_ptr(),
                    bpdu.message_age,
                    bpdu.max_age,
                );
            }
            goto_out!(br, skb);
        }
        br_received_config_bpdu(p, &mut bpdu);
    } else if *buf == BPDU_TYPE_TCN {
        br_received_tcn_bpdu(p);
    }

    spin_unlock(&mut (*br).lock);
    kfree_skb(skb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
