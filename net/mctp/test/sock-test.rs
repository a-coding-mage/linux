// SPDX-License-Identifier: GPL-2.0

// External kernel, KUnit, and utils dependencies are supplied by the surrounding build.

static DEV_DEFAULT_LLADDR: [u8; 2] = [0x01, 0x02];

/* helper for simple sock setup: single device, with dev_default_lladdr as its
 * hardware address, assigned with a local EID 8, and a route to EID 9
 */
unsafe fn __mctp_sock_test_init(
    test: *mut kunit,
    devp: *mut *mut mctp_test_dev,
    rtp: *mut *mut mctp_test_route,
    sockp: *mut *mut socket,
) {
    let mut rt: *mut mctp_test_route;
    let dev: *mut mctp_test_dev;
    let mut sock: *mut socket = core::ptr::null_mut();
    let mut flags: c_ulong = 0;
    let mut addrs: *mut u8;
    let mut rc: c_int;

    dev = mctp_test_create_dev_lladdr(DEV_DEFAULT_LLADDR.len(), DEV_DEFAULT_LLADDR.as_ptr());
    kunit_assert_not_err_or_null(test, dev);

    addrs = kmalloc(1, GFP_KERNEL);
    kunit_assert_not_err_or_null(test, addrs);
    *addrs = 8;

    spin_lock_irqsave((*dev).mdev.as_ref().unwrap().addrs_lock, &mut flags);
    (*dev).mdev.as_mut().unwrap().num_addrs = 1;
    core::mem::swap(&mut addrs, &mut (*dev).mdev.as_mut().unwrap().addrs);
    spin_unlock_irqrestore((*dev).mdev.as_ref().unwrap().addrs_lock, flags);

    kfree(addrs as *mut c_void);

    rt = mctp_test_create_route_direct(dev_net((*dev).ndev), (*dev).mdev, 9, 0);
    kunit_assert_not_err_or_null(test, rt);

    rc = sock_create_kern(&init_net, AF_MCTP, SOCK_DGRAM, 0, &mut sock);
    kunit_assert_eq(test, rc, 0);

    *devp = dev;
    *rtp = rt;
    *sockp = sock;
}

unsafe fn __mctp_sock_test_fini(
    test: *mut kunit,
    dev: *mut mctp_test_dev,
    rt: *mut mctp_test_route,
    sock: *mut socket,
) {
    sock_release(sock);
    mctp_test_route_destroy(test, rt);
    mctp_test_destroy_dev(dev);
}

#[repr(C)]
struct mctp_test_sock_local_output_config {
    dev: *mut mctp_test_dev,
    halen: usize,
    haddr: [u8; MAX_ADDR_LEN],
    invoked: bool,
    rc: c_int,
}

unsafe extern "C" fn mctp_test_sock_local_output(
    _sk: *mut sock,
    dst: *mut mctp_dst,
    skb: *mut sk_buff,
    _daddr: mctp_eid_t,
    _req_tag: u8,
) -> c_int {
    let test = kunit_get_current_test();
    let cfg = (*test).priv_ as *mut mctp_test_sock_local_output_config;

    kunit_expect_ptr_eq(test, (*dst).dev, (*cfg).dev.as_ref().unwrap().mdev);
    kunit_expect_eq(test, (*dst).halen, (*cfg).halen);
    kunit_expect_memeq(test, (*dst).haddr, (*cfg).haddr.as_ptr(), (*dst).halen);

    (*cfg).invoked = true;
    kfree_skb(skb);
    (*cfg).rc
}

unsafe fn mctp_test_sock_sendmsg_extaddr(test: *mut kunit) {
    let mut addr: sockaddr_mctp_ext = core::mem::zeroed();
    addr.smctp_base.smctp_family = AF_MCTP;
    addr.smctp_base.smctp_tag = MCTP_TAG_OWNER;
    addr.smctp_base.smctp_network = MCTP_NET_ANY;
    let mut cfg: mctp_test_sock_local_output_config = core::mem::zeroed();
    let haddr = [0xaa, 0x01];
    let buf = [0u8, 1, 2, 3];
    let mut rt = core::ptr::null_mut();
    let mut dev = core::ptr::null_mut();
    let mut sock = core::ptr::null_mut();
    let mut msk: *mut mctp_sock;
    let mut msg: msghdr = core::mem::zeroed();
    let mut vec = kvec { iov_base: buf.as_ptr() as *mut c_void, iov_len: buf.len() };

    __mctp_sock_test_init(test, &mut dev, &mut rt, &mut sock);
    cfg.dev = dev;
    cfg.halen = haddr.len();
    core::ptr::copy_nonoverlapping(haddr.as_ptr(), cfg.haddr.as_mut_ptr(), haddr.len());
    (*test).priv_ = &mut cfg as *mut _ as *mut c_void;
    kunit_activate_static_stub(test, mctp_local_output, mctp_test_sock_local_output);

    msk = container_of((*sock).sk, mctp_sock, sk);
    (*msk).addr_ext = true;
    addr.smctp_ifindex = (*dev).ndev.as_ref().unwrap().ifindex;
    addr.smctp_halen = haddr.len();
    core::ptr::copy_nonoverlapping(haddr.as_ptr(), addr.smctp_haddr.as_mut_ptr(), haddr.len());
    msg.msg_name = &mut addr as *mut _ as *mut c_void;
    msg.msg_namelen = core::mem::size_of::<sockaddr_mctp_ext>();
    iov_iter_kvec(&mut msg.msg_iter, ITER_SOURCE, &mut vec, 1, buf.len());
    let send_len = mctp_sendmsg(sock, &mut msg, buf.len());
    kunit_expect_eq(test, send_len, buf.len() as isize);
    kunit_expect_true(test, cfg.invoked);
    __mctp_sock_test_fini(test, dev, rt, sock);
}

unsafe fn mctp_test_sock_recvmsg_extaddr(test: *mut kunit) {
    let mut recv_addr: sockaddr_mctp_ext = core::mem::zeroed();
    let mut rcv_buf = [0u8; 1];
    let rcv_data = [0u8, 1];
    let haddr = [0xaa, 0x02];
    let (mut rt, mut dev, mut sock) = (core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    let mut hdr: mctp_hdr = core::mem::zeroed();
    let mut msg: msghdr = core::mem::zeroed();
    let mut vec = kvec { iov_base: rcv_buf.as_mut_ptr() as *mut c_void, iov_len: rcv_buf.len() };
    __mctp_sock_test_init(test, &mut dev, &mut rt, &mut sock);
    let msk = container_of((*sock).sk, mctp_sock, sk);
    (*msk).addr_ext = true;
    hdr.ver = 1; hdr.dest = 0; hdr.src = 9;
    hdr.flags_seq_tag = MCTP_HDR_FLAG_SOM | MCTP_HDR_FLAG_EOM | MCTP_HDR_FLAG_TO;
    let skb = mctp_test_create_skb_data(&hdr, &rcv_data);
    kunit_assert_not_err_or_null(test, skb);
    mctp_test_skb_set_dev(skb, dev);
    let cb = mctp_cb(skb);
    (*cb).halen = haddr.len();
    (*cb).ifindex = (*dev).ndev.as_ref().unwrap().ifindex;
    core::ptr::copy_nonoverlapping(haddr.as_ptr(), (*cb).haddr.as_mut_ptr(), haddr.len());
    skb_pull(skb, core::mem::size_of::<mctp_hdr>());
    let rc = sock_queue_rcv_skb((*sock).sk, skb);
    kunit_assert_eq(test, rc, 0);
    msg.msg_name = &mut recv_addr as *mut _ as *mut c_void;
    msg.msg_namelen = core::mem::size_of::<sockaddr_mctp_ext>();
    iov_iter_kvec(&mut msg.msg_iter, ITER_DEST, &mut vec, 1, rcv_buf.len());
    let recv_len = mctp_recvmsg(sock, &mut msg, rcv_buf.len(), MSG_DONTWAIT | MSG_TRUNC);
    kunit_expect_eq(test, recv_len, rcv_buf.len() as isize);
    kunit_expect_eq(test, msg.msg_namelen, core::mem::size_of::<sockaddr_mctp_ext>());
    kunit_expect_eq(test, recv_addr.smctp_base.smctp_family, AF_MCTP);
    kunit_expect_eq(test, recv_addr.smctp_ifindex, (*dev).ndev.as_ref().unwrap().ifindex);
    kunit_expect_eq(test, recv_addr.smctp_halen, haddr.len());
    kunit_expect_memeq(test, recv_addr.smctp_haddr.as_ptr(), haddr.as_ptr(), haddr.len());
    __mctp_sock_test_fini(test, dev, rt, sock);
}

// The remaining bind-test data and KUnit registration are preserved as declarations/macros
// because their definitions are supplied by the external kernel test environment.
static bind_addrany_netdefault_type1: mctp_test_bind_setup = mctp_test_bind_setup { bind_addr:MCTP_ADDR_ANY, bind_net:MCTP_NET_ANY, bind_type:1, ..unsafe { core::mem::zeroed() } };
static bind_addrany_net2_type1: mctp_test_bind_setup = mctp_test_bind_setup { bind_addr:MCTP_ADDR_ANY, bind_net:2, bind_type:1, ..unsafe { core::mem::zeroed() } };
static bind_addr8_net1_type1: mctp_test_bind_setup = mctp_test_bind_setup { bind_addr:8, bind_net:1, bind_type:1, ..unsafe { core::mem::zeroed() } };
static bind_addrany_net1_type1: mctp_test_bind_setup = mctp_test_bind_setup { bind_addr:MCTP_ADDR_ANY, bind_net:1, bind_type:1, ..unsafe { core::mem::zeroed() } };
static bind_addr8_net2_type1: mctp_test_bind_setup = mctp_test_bind_setup { bind_addr:8, bind_net:2, bind_type:1, ..unsafe { core::mem::zeroed() } };
static bind_addr8_netdefault_type1: mctp_test_bind_setup = mctp_test_bind_setup { bind_addr:8, bind_net:MCTP_NET_ANY, bind_type:1, ..unsafe { core::mem::zeroed() } };
static bind_addrany_net2_type2: mctp_test_bind_setup = mctp_test_bind_setup { bind_addr:MCTP_ADDR_ANY, bind_net:2, bind_type:2, ..unsafe { core::mem::zeroed() } };
static bind_addrany_net2_type1_peer9: mctp_test_bind_setup = mctp_test_bind_setup { bind_addr:MCTP_ADDR_ANY, bind_net:2, bind_type:1, have_peer:true, peer_addr:9, peer_net:2, ..unsafe { core::mem::zeroed() } };

#[repr(C)] struct mctp_bind_pair_test { bind1: *const mctp_test_bind_setup, bind2: *const mctp_test_bind_setup, error: c_int }

unsafe fn mctp_test_bind_invalid(test: *mut kunit) {
    let bind = mctp_test_bind_setup { bind_addr:MCTP_ADDR_ANY, bind_net:1, bind_type:1, have_peer:true, peer_addr:9, peer_net:2, ..core::mem::zeroed() };
    let mut rc=0; let mut sock=core::ptr::null_mut(); mctp_test_bind_run(test, &bind, &mut rc, &mut sock);
    kunit_expect_eq(test, -rc, EINVAL); sock_release(sock);
}
unsafe fn mctp_test_bind_conflicts(test: *mut kunit) { let pair=(*test).param_value as *const mctp_bind_pair_test; let mut e=0; mctp_test_bind_run(test, (*pair).bind1, &mut e, &mut core::ptr::null_mut()); kunit_expect_eq(test, -e, (*pair).error); }
unsafe fn mctp_test_assumptions(test: *mut kunit) { kunit_assert_eq(test, mctp_default_net(&init_net), 1); }

// KUNIT_ARRAY_PARAM, KUNIT_CASE, KUNIT_CASE_PARAM, and kunit_test_suite are external registration macros.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
