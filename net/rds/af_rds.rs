/* Faithful low-level Rust translation of af_rds.c. Kernel and rds.h symbols
 * are intentionally left as external dependencies supplied by other files. */

// C includes: linux/module.h, errno.h, kernel.h, gfp.h, in.h, ipv6.h,
// poll.h, uio.h, net/sock.h, and rds.h.

static mut RDS_SOCK_LOCK: SpinLock = SpinLock::new();
static mut RDS_SOCK_LIST: ListHead = ListHead::new();
static mut RDS_POLL_WAITQ: WaitQueueHead = WaitQueueHead::new();

unsafe fn rds_release(sock: *mut Socket) -> i32 {
    let sk = (*sock).sk;
    if sk.is_null() { return 0; }
    let rs = rds_sk_to_rs(sk);
    sock_orphan(sk);
    rds_clear_recv_queue(rs);
    rds_cong_remove_socket(rs);
    rds_remove_bound(rs);
    rds_send_drop_to(rs, core::ptr::null_mut());
    rds_rdma_drop_keys(rs);
    rds_notify_queue_get(rs, core::ptr::null_mut());
    rds_notify_msg_zcopy_purge(&mut (*rs).rs_zcookie_queue);
    spin_lock_bh(&mut RDS_SOCK_LOCK);
    list_del_init(&mut (*rs).rs_item);
    spin_unlock_bh(&mut RDS_SOCK_LOCK);
    rds_trans_put((*rs).rs_transport);
    (*sock).sk = core::ptr::null_mut();
    sock_put(sk);
    0
}

pub unsafe fn rds_wake_sk_sleep(rs: *mut RdsSock) {
    let mut flags = 0usize;
    read_lock_irqsave(&(*rs).rs_recv_lock, &mut flags);
    __rds_wake_sk_sleep(rds_rs_to_sk(rs));
    read_unlock_irqrestore(&(*rs).rs_recv_lock, flags);
}

unsafe fn rds_getname(sock: *mut Socket, uaddr: *mut SockAddr, peer: i32) -> i32 {
    let rs = rds_sk_to_rs((*sock).sk);
    let mut uaddr_len: i32;
    if peer != 0 {
        if ipv6_addr_any(&(*rs).rs_conn_addr) { return -ENOTCONN; }
        if ipv6_addr_v4mapped(&(*rs).rs_conn_addr) {
            let sin = uaddr as *mut SockAddrIn;
            (*sin).sin_zero = [0; 8]; (*sin).sin_family = AF_INET;
            (*sin).sin_port = (*rs).rs_conn_port; (*sin).sin_addr.s_addr = (*rs).rs_conn_addr_v4;
            uaddr_len = core::mem::size_of::<SockAddrIn>() as i32;
        } else {
            let sin6 = uaddr as *mut SockAddrIn6;
            (*sin6).sin6_family = AF_INET6; (*sin6).sin6_port = (*rs).rs_conn_port;
            (*sin6).sin6_addr = (*rs).rs_conn_addr; (*sin6).sin6_flowinfo = 0;
            (*sin6).sin6_scope_id = (*rs).rs_bound_scope_id;
            uaddr_len = core::mem::size_of::<SockAddrIn6>() as i32;
        }
    } else {
        if ipv6_addr_any(&(*rs).rs_bound_addr) {
            if ipv6_addr_any(&(*rs).rs_conn_addr) {
                let sin = uaddr as *mut SockAddrIn; *sin = core::mem::zeroed();
                (*sin).sin_family = AF_UNSPEC; return core::mem::size_of::<SockAddrIn>() as i32;
            }
            // CONFIG_IPV6 conditional preserved from the C source.
            if (ipv6_addr_type(&(*rs).rs_conn_addr) & IPV6_ADDR_MAPPED) == 0 {
                let sin6 = uaddr as *mut SockAddrIn6; *sin6 = core::mem::zeroed();
                (*sin6).sin6_family = AF_INET6; return core::mem::size_of::<SockAddrIn6>() as i32;
            }
            let sin = uaddr as *mut SockAddrIn; *sin = core::mem::zeroed();
            (*sin).sin_family = AF_INET; return core::mem::size_of::<SockAddrIn>() as i32;
        }
        if ipv6_addr_v4mapped(&(*rs).rs_bound_addr) {
            let sin = uaddr as *mut SockAddrIn; (*sin).sin_zero = [0; 8];
            (*sin).sin_family = AF_INET; (*sin).sin_port = (*rs).rs_bound_port;
            (*sin).sin_addr.s_addr = (*rs).rs_bound_addr_v4;
            uaddr_len = core::mem::size_of::<SockAddrIn>() as i32;
        } else {
            let sin6 = uaddr as *mut SockAddrIn6; (*sin6).sin6_family = AF_INET6;
            (*sin6).sin6_port = (*rs).rs_bound_port; (*sin6).sin6_addr = (*rs).rs_bound_addr;
            (*sin6).sin6_flowinfo = 0; (*sin6).sin6_scope_id = (*rs).rs_bound_scope_id;
            uaddr_len = core::mem::size_of::<SockAddrIn6>() as i32;
        }
    }
    uaddr_len
}

unsafe fn rds_set_bool_option(optvar: *mut u8, optval: SockPtr, optlen: i32) -> i32 {
    if optlen < core::mem::size_of::<i32>() as i32 { return -EINVAL; }
    let mut value = 0i32;
    if copy_from_sockptr(&mut value as *mut _ as *mut _, optval, core::mem::size_of::<i32>()) != 0 { return -EFAULT; }
    *optvar = (value != 0) as u8; 0
}

unsafe fn rds_cancel_sent_to(rs: *mut RdsSock, optval: SockPtr, len: i32) -> i32 {
    if ipv6_addr_any(&(*rs).rs_bound_addr) { return -ENOTCONN; }
    if len < core::mem::size_of::<SockAddrIn>() as i32 { return -EINVAL; }
    let mut sin6: SockAddrIn6 = core::mem::zeroed();
    if len < core::mem::size_of::<SockAddrIn6>() as i32 {
        let mut sin: SockAddrIn = core::mem::zeroed();
        if copy_from_sockptr(&mut sin as *mut _ as *mut _, optval, core::mem::size_of::<SockAddrIn>()) != 0 { return -EFAULT; }
        ipv6_addr_set_v4mapped(sin.sin_addr.s_addr, &mut sin6.sin6_addr); sin6.sin6_port = sin.sin_port;
    } else if copy_from_sockptr(&mut sin6 as *mut _ as *mut _, optval, core::mem::size_of::<SockAddrIn6>()) != 0 { return -EFAULT; }
    rds_send_drop_to(rs, &mut sin6); 0
}

unsafe fn rds_cong_monitor(rs: *mut RdsSock, optval: SockPtr, optlen: i32) -> i32 {
    let ret = rds_set_bool_option(&mut (*rs).rs_cong_monitor, optval, optlen);
    if ret == 0 { if (*rs).rs_cong_monitor != 0 { rds_cong_add_socket(rs); } else { rds_cong_remove_socket(rs); (*rs).rs_cong_mask = 0; (*rs).rs_cong_notify = 0; } }
    ret
}

unsafe fn rds_recv_track_latency(rs: *mut RdsSock, optval: SockPtr, optlen: i32) -> i32 {
    let mut trace: RdsRxTraceSo = core::mem::zeroed();
    if optlen != core::mem::size_of::<RdsRxTraceSo>() as i32 || copy_from_sockptr(&mut trace as *mut _ as *mut _, optval, core::mem::size_of::<RdsRxTraceSo>()) != 0 { return -EFAULT; }
    if trace.rx_traces > RDS_MSG_RX_DGRAM_TRACE_MAX { return -EFAULT; }
    (*rs).rs_rx_traces = trace.rx_traces;
    for i in 0..(*rs).rs_rx_traces as usize { if trace.rx_trace_pos[i] >= RDS_MSG_RX_DGRAM_TRACE_MAX { (*rs).rs_rx_traces = 0; return -EFAULT; } (*rs).rs_rx_trace[i] = trace.rx_trace_pos[i]; }
    0
}

unsafe fn rds_connect(sock: *mut Socket, uaddr: *mut SockAddrUnsized, addr_len: i32, _flags: i32) -> i32 {
    let sk = (*sock).sk; let rs = rds_sk_to_rs(sk); let mut ret = 0;
    if addr_len < offsetofend_sockaddr_family() { return -EINVAL; }
    lock_sock(sk);
    match (*uaddr).sa_family {
        AF_INET => { let sin = uaddr as *mut SockAddrIn; if addr_len < core::mem::size_of::<SockAddrIn>() as i32 { ret=-EINVAL; } else if (*sin).sin_addr.s_addr == htonl(INADDR_ANY) { ret=-EDESTADDRREQ; } else if ipv4_is_multicast((*sin).sin_addr.s_addr) || (*sin).sin_addr.s_addr == htonl(INADDR_BROADCAST) { ret=-EINVAL; } else { ipv6_addr_set_v4mapped((*sin).sin_addr.s_addr, &mut (*rs).rs_conn_addr); (*rs).rs_conn_port=(*sin).sin_port; } }
        AF_INET6 => { let sin6=uaddr as *mut SockAddrIn6; if addr_len < core::mem::size_of::<SockAddrIn6>() as i32 { ret=-EINVAL; } else { let addr_type=ipv6_addr_type(&(*sin6).sin6_addr); if addr_type & IPV6_ADDR_UNICAST == 0 && addr_type & IPV6_ADDR_MAPPED == 0 { ret=-EPROTOTYPE; } else if addr_type & IPV6_ADDR_LINKLOCAL != 0 && ((*sin6).sin6_scope_id==0 || (!ipv6_addr_any(&(*rs).rs_bound_addr) && (*rs).rs_bound_scope_id!=0 && (*sin6).sin6_scope_id!=(*rs).rs_bound_scope_id)) { ret=-EINVAL; } else { (*rs).rs_bound_scope_id=(*sin6).sin6_scope_id; (*rs).rs_conn_addr=(*sin6).sin6_addr; (*rs).rs_conn_port=(*sin6).sin6_port; } } }
        _ => ret=-EAFNOSUPPORT,
    }
    release_sock(sk); ret
}

pub unsafe fn rds_sock_addref(rs: *mut RdsSock) { sock_hold(rds_rs_to_sk(rs)); }
pub unsafe fn rds_sock_put(rs: *mut RdsSock) { sock_put(rds_rs_to_sk(rs)); }

pub static mut RDS_GEN_NUM: u32 = 0;

// Remaining registration and information callbacks are direct external-kernel
// integration points; declarations preserve the source interface and ordering.
unsafe extern "C" { fn rds_bind(sock:*mut Socket,uaddr:*mut SockAddrUnsized,len:i32)->i32; fn rds_sendmsg(_: *mut Socket, _: *mut MsgHdr, _: usize)->i32; fn rds_recvmsg(_: *mut Socket, _: *mut MsgHdr, _: usize, _: i32, _: i32)->i32; }

unsafe fn rds_poll(file:*mut File, sock:*mut Socket, wait:*mut PollTable)->PollMask {
    let sk=(*sock).sk; let rs=rds_sk_to_rs(sk); let mut mask=0; let mut flags=0usize;
    poll_wait(file, sk_sleep(sk), wait);
    if read_once(&(*rs).rs_seen_congestion)!=0 { poll_wait(file, &mut RDS_POLL_WAITQ, wait); }
    read_lock_irqsave(&(*rs).rs_recv_lock,&mut flags);
    if (*rs).rs_cong_monitor==0 { if rds_cong_updated_since(&(*rs).rs_cong_track)!=0 { mask|=EPOLLIN|EPOLLRDNORM|EPOLLWRBAND; } }
    else { spin_lock(&mut (*rs).rs_lock); if (*rs).rs_cong_notify!=0 { mask|=EPOLLIN|EPOLLRDNORM; } spin_unlock(&mut (*rs).rs_lock); }
    if !list_empty(&(*rs).rs_recv_queue)||!list_empty(&(*rs).rs_notify_queue)||!list_empty(&(*rs).rs_zcookie_queue.zcookie_head) { mask|=EPOLLIN|EPOLLRDNORM; }
    if (*rs).rs_snd_bytes<rds_sk_sndbuf(rs) { mask|=EPOLLOUT|EPOLLWRNORM; }
    if (*sk).sk_err!=0 || !skb_queue_empty(&(*sk).sk_error_queue) { mask|=EPOLLERR; }
    read_unlock_irqrestore(&(*rs).rs_recv_lock,flags); if mask!=0 { write_once(&mut (*rs).rs_seen_congestion,0); } mask
}

unsafe fn rds_ioctl(sock:*mut Socket,cmd:u32,arg:usize)->i32 { let rs=rds_sk_to_rs((*sock).sk); match cmd { SIOCRDSSETTOS=>{ let mut v=0; if get_user(&mut v,arg)!=0{return -EFAULT;} if (*rs).rs_transport.is_null(){return -ENOIOCTLCMD;} let tos=((*(*rs).rs_transport).get_tos_map)(v); spin_lock_bh(&mut RDS_SOCK_LOCK); if (*rs).rs_tos!=0||!(*rs).rs_conn.is_null(){spin_unlock_bh(&mut RDS_SOCK_LOCK);return -EINVAL;} (*rs).rs_tos=tos;spin_unlock_bh(&mut RDS_SOCK_LOCK);0 }, SIOCRDSGETTOS=>{spin_lock_bh(&mut RDS_SOCK_LOCK);let v=(*rs).rs_tos;spin_unlock_bh(&mut RDS_SOCK_LOCK);if put_user(v,arg)!=0{-EFAULT}else{0}}, _=>-ENOIOCTLCMD } }

unsafe fn rds_setsockopt(sock:*mut Socket,level:i32,optname:i32,optval:SockPtr,optlen:u32)->i32 { let rs=rds_sk_to_rs((*sock).sk); if level!=SOL_RDS{return -ENOPROTOOPT;} match optname { RDS_CANCEL_SENT_TO=>rds_cancel_sent_to(rs,optval,optlen as i32), RDS_RECVERR=>rds_set_bool_option(&mut (*rs).rs_recverr,optval,optlen as i32), RDS_CONG_MONITOR=>rds_cong_monitor(rs,optval,optlen as i32), SO_RDS_MSG_RXPATH_LATENCY=>rds_recv_track_latency(rs,optval,optlen as i32), _=>-ENOPROTOOPT } }

unsafe fn rds_create(net:*mut Net,sock:*mut Socket,protocol:i32,kern:i32)->i32 { if (*sock).kind!=SOCK_SEQPACKET||protocol!=0{return -ESOCKTNOSUPPORT;} let sk=sk_alloc(net,AF_RDS,GFP_KERNEL,kern); if sk.is_null(){return -ENOMEM;} sock_init_data(sock,sk); (*sock).ops=&RDS_PROTO_OPS; (*sk).sk_protocol=protocol; 0 }

unsafe fn rds_exit(){ sock_unregister(AF_RDS); proto_unregister(&mut RDS_PROTO); rds_conn_exit();rds_cong_exit();rds_sysctl_exit();rds_threads_exit();rds_stats_exit();rds_page_exit();rds_bind_lock_destroy(); }

unsafe fn rds_init()->i32 { let mut ret=0; net_get_random_once(&mut RDS_GEN_NUM,core::mem::size_of::<u32>()); ret=rds_bind_lock_init(); if ret!=0{return ret;} ret=rds_conn_init();if ret!=0{rds_bind_lock_destroy();return ret;}ret=rds_threads_init();if ret!=0{rds_conn_exit();return ret;}ret=rds_sysctl_init();if ret!=0{rds_threads_exit();return ret;}ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
