// SPDX-License-Identifier: GPL-2.0+
/*
 * net/dsa/tag_ksz.c - Microchip KSZ Switch tag format handling
 * Copyright (c) 2017 Microchip Technology
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.

const KSZ8463_NAME: &str = "ksz8463";
const KSZ8795_NAME: &str = "ksz8795";
const KSZ9477_NAME: &str = "ksz9477";
const KSZ9893_NAME: &str = "ksz9893";
const LAN937X_NAME: &str = "lan937x";

const KSZ_PTP_TAG_LEN: usize = 4;
const KSZ_EGRESS_TAG_LEN: usize = 1;
const KSZ_INGRESS_TAG_LEN: usize = 1;
const KSZ_HWTS_EN: usize = 0;

#[repr(C)]
struct ksz_tagger_private {
    data: ksz_tagger_data, // Must be first
    state: c_ulong,
    xmit_worker: *mut kthread_worker,
}

unsafe fn ksz_tagger_private(ds: *mut dsa_switch) -> *mut ksz_tagger_private {
    (*ds).tagger_data as *mut ksz_tagger_private
}

unsafe fn ksz_hwtstamp_set_state(ds: *mut dsa_switch, on: bool) {
    let priv_ = ksz_tagger_private(ds);
    if on {
        set_bit(KSZ_HWTS_EN, &mut (*priv_).state);
    } else {
        clear_bit(KSZ_HWTS_EN, &mut (*priv_).state);
    }
}

unsafe fn ksz_disconnect(ds: *mut dsa_switch) {
    let priv_ = (*ds).tagger_data as *mut ksz_tagger_private;
    kthread_destroy_worker((*priv_).xmit_worker);
    kfree(priv_ as *mut c_void);
    (*ds).tagger_data = core::ptr::null_mut();
}

unsafe fn ksz_connect(ds: *mut dsa_switch) -> c_int {
    let priv_ = kzalloc_obj::<ksz_tagger_private>();
    if priv_.is_null() {
        return -ENOMEM;
    }

    let xmit_worker = kthread_run_worker(
        0,
        c_str!("dsa%d:%d_xmit"),
        (*(*ds).dst).index,
        (*ds).index,
    );
    if is_err(xmit_worker as *const c_void) {
        let ret = ptr_err(xmit_worker as *const c_void);
        kfree(priv_ as *mut c_void);
        return ret;
    }

    (*priv_).xmit_worker = xmit_worker;
    // Export functions for switch driver use
    let tagger_data = &mut (*priv_).data;
    (*tagger_data).hwtstamp_set_state = Some(ksz_hwtstamp_set_state);
    (*ds).tagger_data = priv_ as *mut c_void;
    0
}

unsafe fn ksz_common_rcv(
    skb: *mut sk_buff,
    dev: *mut net_device,
    port: c_uint,
    len: c_uint,
) -> *mut sk_buff {
    (*skb).dev = dsa_conduit_find_user(dev, 0, port);
    if (*skb).dev.is_null() {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }
    if pskb_trim_rcsum(skb, (*skb).len - len as usize) != 0 {
        kfree_skb(skb);
        return core::ptr::null_mut();
    }
    dsa_default_offload_fwd_mark(skb);
    skb
}

const KSZ9477_INGRESS_TAG_LEN: usize = 2;
const KSZ9477_PTP_TAG_LEN: usize = 4;
const KSZ9477_PTP_TAG_INDICATION: u8 = 1 << 7;
const KSZ9477_TAIL_TAG_EG_PORT_M: u16 = (1 << 3) - 1;
const KSZ9477_TAIL_TAG_PRIO: u16 = ((1 << 2) - 1) << 7;
const KSZ9477_TAIL_TAG_OVERRIDE: u16 = 1 << 9;
const KSZ9477_TAIL_TAG_LOOKUP: u16 = 1 << 10;

unsafe fn ksz_rcv_timestamp(skb: *mut sk_buff, tag: *mut u8) {
    let tstamp_raw = tag.sub(KSZ_PTP_TAG_LEN);
    let tstamp = ksz_decode_tstamp(get_unaligned_be32(tstamp_raw));
    (*ksz_skb_cb(skb)).tstamp = tstamp;
}

unsafe fn ksz_xmit_timestamp(dp: *mut dsa_port, skb: *mut sk_buff) {
    let priv_ = ksz_tagger_private((*dp).ds);
    if !test_bit(KSZ_HWTS_EN, &(*priv_).state) {
        return;
    }
    if !(*ksz_skb_cb(skb)).update_correction {
        goto_output_tag(skb);
        return;
    }
    let ptp_type = (*ksz_skb_cb(skb)).ptp_type;
    let ptp_hdr = ptp_parse_header(skb, ptp_type);
    if ptp_hdr.is_null() {
        goto_output_tag(skb);
        return;
    }
    let correction = get_unaligned_be64(&(*ptp_hdr).correction) as i64;
    let mut tstamp_raw: u32 = 0;
    if correction < 0 {
        let ts = ns_to_timespec64(((-correction) >> 16) as u64);
        tstamp_raw = (((ts.tv_sec & 3) << 30) | ts.tv_nsec) as u32;
        ptp_header_update_correction(skb, ptp_type, ptp_hdr, 0);
    }
    goto_output_tag_with_value(skb, tstamp_raw);
}

unsafe fn goto_output_tag(skb: *mut sk_buff) {
    goto_output_tag_with_value(skb, 0);
}

unsafe fn goto_output_tag_with_value(skb: *mut sk_buff, tstamp_raw: u32) {
    put_unaligned_be32(tstamp_raw, skb_put(skb, KSZ_PTP_TAG_LEN));
}

unsafe fn ksz_defer_xmit(dp: *mut dsa_port, skb: *mut sk_buff) -> *mut sk_buff {
    let tagger_data = ksz_tagger_data((*dp).ds);
    let priv_ = ksz_tagger_private((*dp).ds);
    let clone = (*ksz_skb_cb(skb)).clone;
    if clone.is_null() { return skb; }
    let xmit_work_fn = (*tagger_data).xmit_work_fn;
    let xmit_worker = (*priv_).xmit_worker;
    if xmit_work_fn.is_none() || xmit_worker.is_null() {
        kfree_skb(skb); return core::ptr::null_mut();
    }
    let xmit_work = kzalloc_obj::<ksz_deferred_xmit_work>();
    if xmit_work.is_null() { kfree_skb(skb); return core::ptr::null_mut(); }
    kthread_init_work(&mut (*xmit_work).work, xmit_work_fn.unwrap());
    (*xmit_work).dp = dp;
    (*xmit_work).skb = skb_get(skb);
    kthread_queue_work(xmit_worker, &mut (*xmit_work).work);
    kfree_skb(skb);
    core::ptr::null_mut()
}

unsafe fn ksz_common_xmit(skb: *mut sk_buff, dev: *mut net_device, do_tstamp: bool, prio: u8, override_mask: u8) -> *mut sk_buff {
    let dp = dsa_user_to_port(dev);
    if (*skb).ip_summed == CHECKSUM_PARTIAL && skb_checksum_help(skb) != 0 { kfree_skb(skb); return core::ptr::null_mut(); }
    if do_tstamp { ksz_xmit_timestamp(dp, skb); }
    let tag = skb_put(skb, KSZ_INGRESS_TAG_LEN);
    let hdr = skb_eth_hdr(skb);
    *tag = dsa_xmit_port_mask(skb, dev) | prio;
    if is_link_local_ether_addr((*hdr).h_dest.as_ptr()) { *tag |= override_mask; }
    ksz_defer_xmit(dp, skb)
}

unsafe fn ksz9477_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let prio = netdev_txq_to_tc(dev, skb_get_queue_mapping(skb));
    let dp = dsa_user_to_port(dev);
    if (*skb).ip_summed == CHECKSUM_PARTIAL && skb_checksum_help(skb) != 0 { kfree_skb(skb); return core::ptr::null_mut(); }
    ksz_xmit_timestamp(dp, skb);
    let tag = skb_put(skb, KSZ9477_INGRESS_TAG_LEN) as *mut u16;
    let hdr = skb_eth_hdr(skb);
    let mut val = dsa_xmit_port_mask(skb, dev) as u16 | field_prep(KSZ9477_TAIL_TAG_PRIO, prio as u16);
    if is_link_local_ether_addr((*hdr).h_dest.as_ptr()) { val |= KSZ9477_TAIL_TAG_OVERRIDE; }
    *tag = cpu_to_be16(val);
    ksz_defer_xmit(dp, skb)
}

unsafe fn ksz9477_rcv(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let mut len = KSZ_EGRESS_TAG_LEN as c_uint;
    if skb_linearize(skb) != 0 { kfree_skb(skb); return core::ptr::null_mut(); }
    let tag = skb_tail_pointer(skb).sub(KSZ_EGRESS_TAG_LEN);
    let port = (*tag as u16 & KSZ9477_TAIL_TAG_EG_PORT_M) as c_uint;
    if (*tag & KSZ9477_PTP_TAG_INDICATION) != 0 { ksz_rcv_timestamp(skb, tag); len += KSZ_PTP_TAG_LEN as c_uint; }
    ksz_common_rcv(skb, dev, port, len)
}

unsafe fn ksz8795_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff { ksz_common_xmit(skb, dev, false, 0, 1 << 6) }
unsafe fn ksz8795_rcv(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    if skb_linearize(skb) != 0 { kfree_skb(skb); return core::ptr::null_mut(); }
    let tag = skb_tail_pointer(skb).sub(KSZ_EGRESS_TAG_LEN);
    ksz_common_rcv(skb, dev, (*tag & 3) as c_uint, KSZ_EGRESS_TAG_LEN as c_uint)
}

const KSZ9893_TAIL_TAG_PRIO: u8 = 7 << 3;
unsafe fn ksz9893_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let prio = netdev_txq_to_tc(dev, skb_get_queue_mapping(skb));
    ksz_common_xmit(skb, dev, true, field_prep(KSZ9893_TAIL_TAG_PRIO as u16, prio as u16) as u8, 1 << 5)
}

const KSZ8463_TAIL_TAG_PRIO: u8 = 7 << 3;
unsafe fn ksz8463_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let prio = netdev_txq_to_tc(dev, skb_get_queue_mapping(skb));
    ksz_common_xmit(skb, dev, false, field_prep(KSZ8463_TAIL_TAG_PRIO as u16, prio as u16) as u8, 0)
}

unsafe fn ksz8463_rcv(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    if skb_linearize(skb) != 0 { kfree_skb(skb); return core::ptr::null_mut(); }
    (*ksz_skb_cb(skb)).tstamp = 0;
    let tag = skb_tail_pointer(skb).sub(KSZ_EGRESS_TAG_LEN);
    let port = (*tag & 7) as c_uint;
    __skb_push(skb, ETH_HLEN); let ptp_class = ptp_classify_raw(skb); __skb_pull(skb, ETH_HLEN);
    if ptp_class != PTP_CLASS_NONE { let ptp_hdr = ptp_parse_header(skb, ptp_class); if !ptp_hdr.is_null() { (*ksz_skb_cb(skb)).tstamp = ksz_decode_tstamp(get_unaligned_be32(&(*ptp_hdr).reserved2)); (*ptp_hdr).reserved2 = 0; } }
    ksz_common_rcv(skb, dev, port, KSZ_EGRESS_TAG_LEN as c_uint)
}

const LAN937X_EGRESS_TAG_LEN: usize = 2;
unsafe fn lan937x_xmit(skb: *mut sk_buff, dev: *mut net_device) -> *mut sk_buff {
    let prio = netdev_txq_to_tc(dev, skb_get_queue_mapping(skb)); let dp = dsa_user_to_port(dev); let hdr = eth_hdr(skb);
    if (*skb).ip_summed == CHECKSUM_PARTIAL && skb_checksum_help(skb) != 0 { kfree_skb(skb); return core::ptr::null_mut(); }
    ksz_xmit_timestamp(dp, skb); let tag = skb_put(skb, LAN937X_EGRESS_TAG_LEN) as *mut u16;
    let mut val = dsa_xmit_port_mask(skb, dev) as u16 | field_prep(7 << 8, prio as u16);
    if is_link_local_ether_addr((*hdr).h_dest.as_ptr()) { val |= 1 << 11; } val |= 1 << 13; put_unaligned_be16(val, tag as *mut u8); ksz_defer_xmit(dp, skb)
}

#[allow(non_upper_case_globals)]
static ksz9477_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: KSZ9477_NAME, proto: DSA_TAG_PROTO_KSZ9477, xmit: Some(ksz9477_xmit), rcv: Some(ksz9477_rcv),
    connect: Some(ksz_connect), disconnect: Some(ksz_disconnect), needed_tailroom: KSZ9477_INGRESS_TAG_LEN + KSZ_PTP_TAG_LEN,
};
#[allow(non_upper_case_globals)]
static ksz8795_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: KSZ8795_NAME, proto: DSA_TAG_PROTO_KSZ8795, xmit: Some(ksz8795_xmit), rcv: Some(ksz8795_rcv),
    connect: None, disconnect: None, needed_tailroom: KSZ_INGRESS_TAG_LEN,
};
#[allow(non_upper_case_globals)]
static ksz9893_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: KSZ9893_NAME, proto: DSA_TAG_PROTO_KSZ9893, xmit: Some(ksz9893_xmit), rcv: Some(ksz9477_rcv),
    connect: Some(ksz_connect), disconnect: Some(ksz_disconnect), needed_tailroom: KSZ_INGRESS_TAG_LEN + KSZ_PTP_TAG_LEN,
};
#[allow(non_upper_case_globals)]
static ksz8463_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: KSZ8463_NAME, proto: DSA_TAG_PROTO_KSZ8463, xmit: Some(ksz8463_xmit), rcv: Some(ksz8463_rcv),
    connect: Some(ksz_connect), disconnect: Some(ksz_disconnect), needed_tailroom: KSZ_INGRESS_TAG_LEN,
};
#[allow(non_upper_case_globals)]
static lan937x_netdev_ops: dsa_device_ops = dsa_device_ops {
    name: LAN937X_NAME, proto: DSA_TAG_PROTO_LAN937X, xmit: Some(lan937x_xmit), rcv: Some(ksz9477_rcv),
    connect: Some(ksz_connect), disconnect: Some(ksz_disconnect), needed_tailroom: LAN937X_EGRESS_TAG_LEN + KSZ_PTP_TAG_LEN,
};

static dsa_tag_driver_array: [*const dsa_tag_driver; 5] = [
    &DSA_TAG_DRIVER_NAME!(ksz8463_netdev_ops), &DSA_TAG_DRIVER_NAME!(ksz8795_netdev_ops),
    &DSA_TAG_DRIVER_NAME!(ksz9477_netdev_ops), &DSA_TAG_DRIVER_NAME!(ksz9893_netdev_ops),
    &DSA_TAG_DRIVER_NAME!(lan937x_netdev_ops),
];

extern_driver_registration!(ksz9477_netdev_ops, KSZ9477_NAME);
extern_driver_registration!(ksz8795_netdev_ops, KSZ8795_NAME);
extern_driver_registration!(ksz9893_netdev_ops, KSZ9893_NAME);
extern_driver_registration!(ksz8463_netdev_ops, KSZ8463_NAME);
extern_driver_registration!(lan937x_netdev_ops, LAN937X_NAME);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
