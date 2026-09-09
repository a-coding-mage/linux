// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GRE over IPv4 demultiplexer driver
 *
 * Authors: Dmitry Kozlov (xeb@mail.ru)
 */

// Kernel dependencies are supplied by the surrounding translation unit.

static mut GRE_PROTO: [*const gre_protocol; GREPROTO_MAX as usize] = [core::ptr::null(); GREPROTO_MAX as usize];

pub unsafe fn gre_add_protocol(proto: *const gre_protocol, version: u8) -> i32 {
    if version as u32 >= GREPROTO_MAX {
        return -EINVAL;
    }

    (cmpxchg(
        &mut GRE_PROTO[version as usize] as *mut *const gre_protocol,
        core::ptr::null(),
        proto,
    ) == core::ptr::null())
        .then_some(0)
        .unwrap_or(-EBUSY)
}

pub unsafe fn gre_del_protocol(proto: *const gre_protocol, version: u8) -> i32 {
    if version as u32 >= GREPROTO_MAX {
        return -EINVAL;
    }

    let ret = (cmpxchg(
        &mut GRE_PROTO[version as usize] as *mut *const gre_protocol,
        proto,
        core::ptr::null(),
    ) == proto)
        .then_some(0)
        .unwrap_or(-EBUSY);

    if ret != 0 {
        return ret;
    }

    synchronize_rcu();
    0
}

/* Fills in tpi and returns header length to be pulled.
 * Note that caller must use pskb_may_pull() before pulling GRE header.
 */
pub unsafe fn gre_parse_header(
    skb: *mut sk_buff,
    tpi: *mut tnl_ptk_info,
    csum_err: *mut bool,
    proto: __be16,
    nhs: i32,
) -> i32 {
    let mut greh: *const gre_base_hdr;
    let mut options: *mut __be32;
    let mut hdr_len: i32;

    if !pskb_may_pull(skb, nhs + core::mem::size_of::<gre_base_hdr>() as i32) {
        return -EINVAL;
    }

    greh = ((*skb).data.add(nhs as usize)) as *const gre_base_hdr;
    if (*greh).flags & (GRE_VERSION | GRE_ROUTING) != 0 {
        return -EINVAL;
    }

    gre_flags_to_tnl_flags(&mut (*tpi).flags, (*greh).flags);
    hdr_len = gre_calc_hlen((*tpi).flags);

    if !pskb_may_pull(skb, nhs + hdr_len) {
        return -EINVAL;
    }

    greh = ((*skb).data.add(nhs as usize)) as *const gre_base_hdr;
    (*tpi).proto = (*greh).protocol;
    options = greh.add(1) as *mut __be32;

    if (*greh).flags & GRE_CSUM != 0 {
        if !skb_checksum_simple_validate(skb) {
            skb_checksum_try_convert(skb, IPPROTO_GRE, null_compute_pseudo);
        } else if !csum_err.is_null() {
            *csum_err = true;
            return -EINVAL;
        }
        options = options.add(1);
    }

    if (*greh).flags & GRE_KEY != 0 {
        (*tpi).key = *options;
        options = options.add(1);
    } else {
        (*tpi).key = 0;
    }
    if (*greh).flags & GRE_SEQ != 0 {
        (*tpi).seq = *options;
        options = options.add(1);
    } else {
        (*tpi).seq = 0;
    }

    /* WCCP version 1 and 2 protocol decoding. */
    if (*greh).flags == 0 && (*tpi).proto == htons(ETH_P_WCCP) {
        let mut val = 0u8;
        let ptr = skb_header_pointer(
            skb,
            nhs + hdr_len,
            core::mem::size_of::<u8>() as i32,
            &mut val as *mut u8 as *mut core::ffi::c_void,
        );
        if ptr.is_null() {
            return -EINVAL;
        }
        (*tpi).proto = proto;
        if val & 0xF0 != 0x40 {
            hdr_len += 4;
        }
    }
    (*tpi).hdr_len = hdr_len;

    if ((*greh).protocol == htons(ETH_P_ERSPAN) && hdr_len != 4)
        || (*greh).protocol == htons(ETH_P_ERSPAN2)
    {
        let ershdr = core::ptr::null_mut::<erspan_base_hdr>();
        if !pskb_may_pull(skb, nhs + hdr_len + core::mem::size_of::<erspan_base_hdr>() as i32) {
            return -EINVAL;
        }
        let ershdr = ((*skb).data.add((nhs + hdr_len) as usize)) as *mut erspan_base_hdr;
        (*tpi).key = cpu_to_be32(get_session_id(ershdr));
    }

    hdr_len
}

unsafe fn gre_rcv(skb: *mut sk_buff) -> i32 {
    let proto: *const gre_protocol;
    let ver: u8;
    let ret: i32;

    if !pskb_may_pull(skb, 12) { goto_drop(skb); return NET_RX_DROP; }
    ver = (*skb).data.add(1).read() & 0x7f;
    if ver as u32 >= GREPROTO_MAX { goto_drop(skb); return NET_RX_DROP; }
    rcu_read_lock();
    proto = rcu_dereference(GRE_PROTO[ver as usize]);
    if proto.is_null() || (*proto).handler.is_none() { rcu_read_unlock(); dev_core_stats_rx_nohandler_inc((*skb).dev); kfree_skb(skb); return NET_RX_DROP; }
    ret = ((*proto).handler.unwrap())(skb);
    rcu_read_unlock();
    ret
}

unsafe fn goto_drop(skb: *mut sk_buff) { dev_core_stats_rx_dropped_inc((*skb).dev); kfree_skb(skb); }

unsafe fn gre_err(skb: *mut sk_buff, info: u32) -> i32 {
    let iph = (*skb).data as *const iphdr;
    let ver = (*skb).data.add(((*iph).ihl as usize) << 2).add(1).read() & 0x7f;
    if ver as u32 >= GREPROTO_MAX { return -EINVAL; }
    rcu_read_lock();
    let proto = rcu_dereference(GRE_PROTO[ver as usize]);
    let err = if !proto.is_null() && (*proto).err_handler.is_some() { ((*proto).err_handler.unwrap())(skb, info); 0 } else { -EPROTONOSUPPORT };
    rcu_read_unlock();
    err
}

static net_gre_protocol: net_protocol = net_protocol { handler: Some(gre_rcv), err_handler: Some(gre_err) };

unsafe fn gre_init() -> i32 {
    pr_info!("GRE over IPv4 demultiplexer driver\n");
    if inet_add_protocol(&net_gre_protocol, IPPROTO_GRE) < 0 { pr_err!("can't add protocol\n"); return -EAGAIN; }
    0
}

unsafe fn gre_exit() { inet_del_protocol(&net_gre_protocol, IPPROTO_GRE); }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
