// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Translated from sch_frag.c. Kernel includes and externally supplied symbols
// are intentionally left to the surrounding translation unit.

#[repr(C)]
pub struct sch_frag_data {
    pub dst: ::core::ffi::c_ulong,
    pub cb: qdisc_skb_cb,
    pub inner_protocol: __be16,
    pub vlan_tci: u16,
    pub vlan_proto: __be16,
    pub l2_len: u32,
    pub l2_data: [u8; VLAN_ETH_HLEN as usize],
    pub xmit: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>,
    pub bh_lock: local_lock_t,
}

// DEFINE_PER_CPU(struct sch_frag_data, sch_frag_data_storage) = {
//     .bh_lock = INIT_LOCAL_LOCK(bh_lock),
// };
extern "C" {
    static mut sch_frag_data_storage: sch_frag_data;
}

unsafe fn sch_frag_xmit(
    _net: *mut net,
    _sk: *mut sock,
    skb: *mut sk_buff,
) -> i32 {
    let data = this_cpu_ptr(&raw mut sch_frag_data_storage);

    lockdep_assert_held(&(*data).bh_lock);
    if skb_cow_head(skb, (*data).l2_len) < 0 {
        kfree_skb(skb);
        return -12; // -ENOMEM
    }

    __skb_dst_copy(skb, (*data).dst);
    *qdisc_skb_cb(skb) = (*data).cb;
    (*skb).inner_protocol = (*data).inner_protocol;
    if ((*data).vlan_tci & VLAN_CFI_MASK) != 0 {
        __vlan_hwaccel_put_tag(
            skb,
            (*data).vlan_proto,
            (*data).vlan_tci & !VLAN_CFI_MASK,
        );
    } else {
        __vlan_hwaccel_clear_tag(skb);
    }

    // Reconstruct the MAC header.
    skb_push(skb, (*data).l2_len);
    core::ptr::copy_nonoverlapping(
        (*data).l2_data.as_ptr(),
        (*skb).data,
        (*data).l2_len as usize,
    );
    skb_postpush_rcsum(skb, (*skb).data, (*data).l2_len);
    skb_reset_mac_header(skb);

    ((*data).xmit.expect("xmit callback"))(skb)
}

unsafe fn sch_frag_prepare_frag(
    skb: *mut sk_buff,
    xmit: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>,
) {
    let hlen = skb_network_offset(skb);
    let data = this_cpu_ptr(&raw mut sch_frag_data_storage);

    (*data).dst = (*skb)._skb_refdst;
    (*data).cb = *qdisc_skb_cb(skb);
    (*data).xmit = xmit;
    (*data).inner_protocol = (*skb).inner_protocol;
    if skb_vlan_tag_present(skb) {
        (*data).vlan_tci = skb_vlan_tag_get(skb) | VLAN_CFI_MASK;
    } else {
        (*data).vlan_tci = 0;
    }
    (*data).vlan_proto = (*skb).vlan_proto;
    core::ptr::copy_nonoverlapping(
        (*skb).data,
        (*data).l2_data.as_mut_ptr(),
        hlen as usize,
    );

    core::ptr::write_bytes(IPCB(skb) as *mut u8, 0, core::mem::size_of::<inet_skb_parm>());
    skb_pull(skb, hlen);
}

unsafe extern "C" fn sch_frag_dst_get_mtu(dst: *const dst_entry) -> u32 {
    (*(*dst).dev).mtu
}

#[repr(C)]
static mut sch_frag_dst_ops: dst_ops = dst_ops {
    family: AF_UNSPEC,
    mtu: Some(sch_frag_dst_get_mtu),
};

unsafe fn sch_fragment(
    net: *mut net,
    skb: *mut sk_buff,
    mru: u16,
    xmit: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>,
) -> i32 {
    let mut ret: i32 = -1;

    if skb_network_offset(skb) > VLAN_ETH_HLEN {
        net_warn_ratelimited!("L2 header too long to fragment\n");
        kfree_skb(skb);
        return ret;
    }

    if skb_protocol(skb, true) == htons(ETH_P_IP) {
        let mut sch_frag_rt: rtable = core::mem::zeroed();
        let orig_dst: ::core::ffi::c_ulong;

        local_lock_nested_bh(&raw mut (*this_cpu_ptr(&raw mut sch_frag_data_storage)).bh_lock);
        sch_frag_prepare_frag(skb, xmit);
        dst_init(&mut sch_frag_rt.dst, &raw mut sch_frag_dst_ops, core::ptr::null_mut(), DST_OBSOLETE_NONE, DST_NOCOUNT);
        sch_frag_rt.dst.dev = (*skb).dev;
        orig_dst = (*skb)._skb_refdst;
        skb_dst_set_noref(skb, &mut sch_frag_rt.dst);
        (*IPCB(skb)).frag_max_size = mru;
        ret = ip_do_fragment(net, (*skb).sk, skb, Some(sch_frag_xmit));
        local_unlock_nested_bh(&raw mut (*this_cpu_ptr(&raw mut sch_frag_data_storage)).bh_lock);
        refdst_drop(orig_dst);
    } else if skb_protocol(skb, true) == htons(ETH_P_IPV6) {
        let orig_dst: ::core::ffi::c_ulong;
        let mut sch_frag_rt: rt6_info = core::mem::zeroed();

        local_lock_nested_bh(&raw mut (*this_cpu_ptr(&raw mut sch_frag_data_storage)).bh_lock);
        sch_frag_prepare_frag(skb, xmit);
        dst_init(&mut sch_frag_rt.dst, &raw mut sch_frag_dst_ops, core::ptr::null_mut(), DST_OBSOLETE_NONE, DST_NOCOUNT);
        sch_frag_rt.dst.dev = (*skb).dev;
        orig_dst = (*skb)._skb_refdst;
        skb_dst_set_noref(skb, &mut sch_frag_rt.dst);
        (*IP6CB(skb)).frag_max_size = mru;
        ret = ip6_fragment(net, (*skb).sk, skb, Some(sch_frag_xmit));
        local_unlock_nested_bh(&raw mut (*this_cpu_ptr(&raw mut sch_frag_data_storage)).bh_lock);
        refdst_drop(orig_dst);
    } else {
        net_warn_ratelimited!("Fail frag: unsupported protocol\n");
        kfree_skb(skb);
        return ret;
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn sch_frag_xmit_hook(
    skb: *mut sk_buff,
    xmit: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>,
) -> i32 {
    let mru = (*tc_skb_cb(skb)).mru;
    if mru != 0 && (*skb).len > (mru as u32) + (*(*skb).dev).hard_header_len {
        sch_fragment(dev_net((*skb).dev), skb, mru, xmit)
    } else {
        xmit.expect("xmit callback")(skb)
    }
}

// EXPORT_SYMBOL_GPL(sch_frag_xmit_hook);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
