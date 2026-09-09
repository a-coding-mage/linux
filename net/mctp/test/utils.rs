// SPDX-License-Identifier: GPL-2.0

// External Linux/MCTP declarations are supplied by the surrounding translation.

unsafe fn mctp_test_dev_tx(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx_t {
    let dev = netdev_priv(ndev);

    skb_queue_tail(&mut (*dev).pkts, skb);

    NETDEV_TX_OK
}

static mut MCTP_TEST_NETDEV_OPS: net_device_ops = net_device_ops {
    ndo_start_xmit: Some(mctp_test_dev_tx),
};

unsafe fn mctp_test_dev_setup(ndev: *mut net_device) {
    (*ndev).type_ = ARPHRD_MCTP;
    (*ndev).mtu = MCTP_DEV_TEST_MTU;
    (*ndev).hard_header_len = 0;
    (*ndev).tx_queue_len = 0;
    (*ndev).flags = IFF_NOARP;
    (*ndev).netdev_ops = &raw mut MCTP_TEST_NETDEV_OPS;
    (*ndev).needs_free_netdev = true;
}

unsafe fn __mctp_test_create_dev(
    lladdr_len: u16,
    lladdr: *const u8,
) -> *mut mctp_test_dev {
    let dev: *mut mctp_test_dev;
    let ndev: *mut net_device;
    let rc: i32;

    if WARN_ON(lladdr_len > MAX_ADDR_LEN) {
        return core::ptr::null_mut();
    }

    ndev = alloc_netdev(
        core::mem::size_of::<mctp_test_dev>(),
        b"mctptest%d\0".as_ptr() as *const i8,
        NET_NAME_ENUM,
        Some(mctp_test_dev_setup),
    );
    if ndev.is_null() {
        return core::ptr::null_mut();
    }

    dev = netdev_priv(ndev);
    (*dev).ndev = ndev;
    (*ndev).addr_len = lladdr_len;
    dev_addr_set(ndev, lladdr);
    skb_queue_head_init(&mut (*dev).pkts);

    rc = register_netdev(ndev);
    if rc != 0 {
        free_netdev(ndev);
        return core::ptr::null_mut();
    }

    rcu_read_lock();
    (*dev).mdev = __mctp_dev_get(ndev);
    (*(*dev).mdev).net = mctp_default_net(dev_net(ndev));
    rcu_read_unlock();

    // bring the device up; we want to be able to TX immediately
    rtnl_lock();
    dev_open(ndev, core::ptr::null_mut());
    rtnl_unlock();

    dev
}

pub unsafe fn mctp_test_create_dev() -> *mut mctp_test_dev {
    __mctp_test_create_dev(0, core::ptr::null())
}

pub unsafe fn mctp_test_create_dev_with_addr(addr: mctp_eid_t) -> *mut mctp_test_dev {
    let dev = __mctp_test_create_dev(0, core::ptr::null());
    if dev.is_null() {
        return core::ptr::null_mut();
    }

    (*(*dev).mdev).addrs = kmalloc_objs::<u8>(1, GFP_KERNEL);
    if (*(*dev).mdev).addrs.is_null() {
        mctp_test_destroy_dev(dev);
        return core::ptr::null_mut();
    }

    (*(*dev).mdev).num_addrs = 1;
    (*(*dev).mdev).addrs[0] = addr;

    dev
}

pub unsafe fn mctp_test_create_dev_lladdr(
    lladdr_len: u16,
    lladdr: *const u8,
) -> *mut mctp_test_dev {
    __mctp_test_create_dev(lladdr_len, lladdr)
}

pub unsafe fn mctp_test_destroy_dev(dev: *mut mctp_test_dev) {
    skb_queue_purge(&mut (*dev).pkts);
    mctp_dev_put((*dev).mdev);
    unregister_netdev((*dev).ndev);
}

unsafe fn mctp_test_dst_output(dst: *mut mctp_dst, skb: *mut sk_buff) -> i32 {
    (*skb).dev = (*(*dst).dev).dev;
    dev_direct_xmit(skb, 0);

    0
}

// local version of mctp_route_alloc()
unsafe fn mctp_route_test_alloc() -> *mut mctp_test_route {
    let rt = kzalloc_obj::<mctp_test_route>();
    if rt.is_null() {
        return core::ptr::null_mut();
    }

    INIT_LIST_HEAD(&mut (*rt).rt.list);
    refcount_set(&mut (*rt).rt.refs, 1);
    (*rt).rt.output = Some(mctp_test_dst_output);

    rt
}

pub unsafe fn mctp_test_create_route_direct(
    net: *mut net,
    dev: *mut mctp_dev,
    eid: mctp_eid_t,
    mtu: u32,
) -> *mut mctp_test_route {
    let rt = mctp_route_test_alloc();
    if rt.is_null() {
        return core::ptr::null_mut();
    }

    (*rt).rt.min = eid;
    (*rt).rt.max = eid;
    (*rt).rt.mtu = mtu;
    (*rt).rt.type_ = RTN_UNSPEC;
    (*rt).rt.dst_type = MCTP_ROUTE_DIRECT;
    if !dev.is_null() {
        mctp_dev_hold(dev);
    }
    (*rt).rt.dev = dev;

    list_add_rcu(&mut (*rt).rt.list, &mut (*net).mctp.routes);

    rt
}

pub unsafe fn mctp_test_create_route_gw(
    net: *mut net,
    netid: u32,
    eid: mctp_eid_t,
    gw: mctp_eid_t,
    mtu: u32,
) -> *mut mctp_test_route {
    let rt = mctp_route_test_alloc();
    if rt.is_null() {
        return core::ptr::null_mut();
    }

    (*rt).rt.min = eid;
    (*rt).rt.max = eid;
    (*rt).rt.mtu = mtu;
    (*rt).rt.type_ = RTN_UNSPEC;
    (*rt).rt.dst_type = MCTP_ROUTE_GATEWAY;
    (*rt).rt.gateway.eid = gw;
    (*rt).rt.gateway.net = netid;

    list_add_rcu(&mut (*rt).rt.list, &mut (*net).mctp.routes);

    rt
}

// Convenience function for our test dst; release with mctp_dst_release()
pub unsafe fn mctp_test_dst_setup(
    test: *mut kunit,
    dst: *mut mctp_dst,
    dev: *mut mctp_test_dev,
    mtu: u32,
) {
    let mut flags: ulong = 0;

    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, dev);

    core::ptr::write_bytes(dst, 0, 1);

    (*dst).dev = (*dev).mdev;
    __mctp_dev_get((*dst).dev.dev);
    (*dst).mtu = mtu;
    (*dst).output = Some(mctp_test_dst_output);
    (*dst).saddr = MCTP_ADDR_NULL;
    spin_lock_irqsave(&mut (*dev).mdev.addrs_lock, &mut flags);
    if (*dev).mdev.num_addrs != 0 {
        (*dst).saddr = (*dev).mdev.addrs[0];
    }
    spin_unlock_irqrestore(&mut (*dev).mdev.addrs_lock, flags);
}

pub unsafe fn mctp_test_route_destroy(test: *mut kunit, rt: *mut mctp_test_route) {
    let refs: u32;

    rtnl_lock();
    list_del_rcu(&mut (*rt).rt.list);
    rtnl_unlock();

    if (*rt).rt.dst_type == MCTP_ROUTE_DIRECT && !(*rt).rt.dev.is_null() {
        mctp_dev_put((*rt).rt.dev);
    }

    refs = refcount_read(&(*rt).rt.refs);
    KUNIT_ASSERT_EQ_MSG(test, refs, 1, b"route ref imbalance\0".as_ptr() as *const i8);

    kfree_rcu(&mut (*rt).rt, rcu);
}

pub unsafe fn mctp_test_skb_set_dev(skb: *mut sk_buff, dev: *mut mctp_test_dev) {
    let cb = mctp_cb(skb);

    (*cb).net = READ_ONCE((*(*dev).mdev).net);
    (*skb).dev = (*dev).ndev;
}

pub unsafe fn mctp_test_create_skb(
    hdr: *const mctp_hdr,
    data_len: u32,
) -> *mut sk_buff {
    let hdr_len = core::mem::size_of::<mctp_hdr>();
    let skb: *mut sk_buff;
    let mut buf: *mut u8;

    skb = alloc_skb(hdr_len + data_len as usize, GFP_KERNEL);
    if skb.is_null() {
        return core::ptr::null_mut();
    }

    __mctp_cb(skb);
    memcpy(skb_put(skb, hdr_len), hdr, hdr_len);

    buf = skb_put(skb, data_len as usize);
    for i in 0..data_len {
        *buf.add(i as usize) = (i & 0xff) as u8;
    }

    skb
}

pub unsafe fn __mctp_test_create_skb_data(
    hdr: *const mctp_hdr,
    data: *const core::ffi::c_void,
    data_len: usize,
) -> *mut sk_buff {
    let hdr_len = core::mem::size_of::<mctp_hdr>();
    let skb = alloc_skb(hdr_len + data_len, GFP_KERNEL);
    if skb.is_null() {
        return core::ptr::null_mut();
    }

    __mctp_cb(skb);
    memcpy(skb_put(skb, hdr_len), hdr, hdr_len);
    memcpy(skb_put(skb, data_len), data, data_len);

    skb
}

pub unsafe fn mctp_test_bind_run(
    test: *mut kunit,
    setup: *const mctp_test_bind_setup,
    ret_bind_errno: *mut i32,
    sock: *mut *mut socket,
) {
    let mut addr: sockaddr_mctp = core::mem::zeroed();
    let rc: i32;

    *ret_bind_errno = -EIO;

    rc = sock_create_kern(&init_net, AF_MCTP, SOCK_DGRAM, 0, sock);
    KUNIT_ASSERT_EQ(test, rc, 0);

    // connect() if requested
    if (*setup).have_peer {
        addr = core::mem::zeroed();
        addr.smctp_family = AF_MCTP;
        addr.smctp_network = (*setup).peer_net;
        addr.smctp_addr.s_addr = (*setup).peer_addr;
        // connect() type must match bind() type
        addr.smctp_type = (*setup).bind_type;
        rc = kernel_connect(
            *sock,
            &mut addr as *mut sockaddr_mctp as *mut sockaddr_unsized,
            core::mem::size_of::<sockaddr_mctp>() as u32,
            0,
        );
        KUNIT_EXPECT_EQ(test, rc, 0);
    }

    // bind()
    addr = core::mem::zeroed();
    addr.smctp_family = AF_MCTP;
    addr.smctp_network = (*setup).bind_net;
    addr.smctp_addr.s_addr = (*setup).bind_addr;
    addr.smctp_type = (*setup).bind_type;

    *ret_bind_errno = kernel_bind(
        *sock,
        &mut addr as *mut sockaddr_mctp as *mut sockaddr_unsized,
        core::mem::size_of::<sockaddr_mctp>() as u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
