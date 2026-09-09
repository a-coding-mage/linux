// SPDX-License-Identifier: GPL-2.0
/* Copyright 2020-2021 NXP
 *
 * An implementation of the software-defined tag_8021q.c tagger format, which
 * also preserves full functionality under a vlan_filtering bridge. It does
 * this by using the TCAM engines for:
 * - pushing the RX VLAN as a second, outer tag, on egress towards the CPU port
 * - redirecting towards the correct front port based on TX VLAN and popping
 *   that on egress
 */

// Dependencies supplied by the surrounding kernel/DSA implementation.

const OCELOT_8021Q_NAME: &str = "ocelot-8021q";

#[repr(C)]
struct ocelot_8021q_tagger_private {
    data: ocelot_8021q_tagger_data, /* Must be first */
    xmit_worker: *mut kthread_worker,
}

unsafe fn ocelot_defer_xmit(
    dp: *mut dsa_port,
    skb: *mut sk_buff,
) -> *mut sk_buff {
    let priv_: *mut ocelot_8021q_tagger_private = (*(*dp).ds).tagger_data as *mut _;
    let data: *mut ocelot_8021q_tagger_data = &mut (*priv_).data;
    let xmit_work_fn: Option<unsafe extern "C" fn(*mut kthread_work)> = (*data).xmit_work_fn;
    let xmit_worker: *mut kthread_worker = (*priv_).xmit_worker;

    if xmit_work_fn.is_none() || xmit_worker.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    /* PTP over IP packets need UDP checksumming. We may have inherited
     * NETIF_F_HW_CSUM from the DSA conduit, but these packets are not sent
     * through the DSA conduit, so calculate the checksum here.
     */
    if (*skb).ip_summed == CHECKSUM_PARTIAL && skb_checksum_help(skb) != 0 {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    let xmit_work: *mut felix_deferred_xmit_work = kzalloc_obj::<felix_deferred_xmit_work>();
    if xmit_work.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    /* Calls felix_port_deferred_xmit in felix.c */
    kthread_init_work(&mut (*xmit_work).work, xmit_work_fn);
    (*xmit_work).dp = dp;
    (*xmit_work).skb = skb_get(skb);

    kthread_queue_work(xmit_worker, &mut (*xmit_work).work);

    kfree_skb(skb);
    core::ptr::null_mut()
}

unsafe fn ocelot_xmit(skb: *mut sk_buff, netdev: *mut net_device) -> *mut sk_buff {
    let dp: *mut dsa_port = dsa_user_to_port(netdev);
    let queue_mapping: u16 = skb_get_queue_mapping(skb);
    let pcp: u8 = netdev_txq_to_tc(netdev, queue_mapping);
    let tx_vid: u16 = dsa_tag_8021q_standalone_vid(dp);
    let hdr: *mut ethhdr = eth_hdr(skb);

    if ocelot_ptp_rew_op(skb) || is_link_local_ether_addr((*hdr).h_dest.as_ptr()) {
        return ocelot_defer_xmit(dp, skb);
    }

    dsa_8021q_xmit(
        skb,
        netdev,
        ETH_P_8021Q,
        ((pcp as u16) << VLAN_PRIO_SHIFT) | tx_vid,
    )
}

unsafe fn ocelot_rcv(skb: *mut sk_buff, netdev: *mut net_device) -> *mut sk_buff {
    let mut src_port: i32 = -1;
    let mut switch_id: i32 = -1;

    dsa_8021q_rcv(
        skb,
        &mut src_port,
        &mut switch_id,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );

    (*skb).dev = dsa_conduit_find_user(netdev, switch_id, src_port);
    if (*skb).dev.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }

    dsa_default_offload_fwd_mark(skb);
    skb
}

unsafe fn ocelot_disconnect(ds: *mut dsa_switch) {
    let priv_: *mut ocelot_8021q_tagger_private = (*ds).tagger_data as *mut _;

    kthread_destroy_worker((*priv_).xmit_worker);
    kfree(priv_ as *mut core::ffi::c_void);
    (*ds).tagger_data = core::ptr::null_mut();
}

unsafe fn ocelot_connect(ds: *mut dsa_switch) -> i32 {
    let priv_: *mut ocelot_8021q_tagger_private = kzalloc_obj::<ocelot_8021q_tagger_private>();
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).xmit_worker = kthread_run_worker(0, b"felix_xmit\0".as_ptr() as *const i8);
    if IS_ERR((*priv_).xmit_worker) {
        let err: i32 = PTR_ERR((*priv_).xmit_worker);
        kfree(priv_ as *mut core::ffi::c_void);
        return err;
    }

    (*ds).tagger_data = priv_ as *mut core::ffi::c_void;
    0
}

#[repr(C)]
static ocelot_8021q_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: OCELOT_8021Q_NAME,
    proto: DSA_TAG_PROTO_OCELOT_8021Q,
    xmit: Some(ocelot_xmit),
    rcv: Some(ocelot_rcv),
    connect: Some(ocelot_connect),
    disconnect: Some(ocelot_disconnect),
    needed_headroom: VLAN_HLEN,
    promisc_on_conduit: true,
};

// MODULE_DESCRIPTION("DSA tag driver for Ocelot family of switches, using VLAN");
// MODULE_LICENSE("GPL v2");
// MODULE_ALIAS_DSA_TAG_DRIVER(DSA_TAG_PROTO_OCELOT_8021Q, OCELOT_8021Q_NAME);
// module_dsa_tag_driver(ocelot_8021q_netdev_ops);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
