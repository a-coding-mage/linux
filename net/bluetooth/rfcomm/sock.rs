// SPDX-License-Identifier: GPL-2.0
/* RFCOMM implementation for Linux Bluetooth stack (BlueZ). */
/* Translated from sock.c; external kernel symbols are supplied by dependencies. */

static mut rfcomm_sock_ops: proto_ops = proto_ops { };
static mut rfcomm_sk_list: bt_sock_list = bt_sock_list {
    lock: __RW_LOCK_UNLOCKED!(rfcomm_sk_list.lock),
};

unsafe fn rfcomm_sock_close(sk: *mut sock);
unsafe fn rfcomm_sock_kill(sk: *mut sock);

unsafe fn rfcomm_sk_data_ready(d: *mut rfcomm_dlc, skb: *mut sk_buff) {
    let sk = (*d).owner;
    if sk.is_null() { return; }
    atomic_add((*skb).len, &mut (*sk).sk_rmem_alloc);
    skb_queue_tail(&mut (*sk).sk_receive_queue, skb);
    ((*sk).sk_data_ready)(sk);
    if atomic_read(&(*sk).sk_rmem_alloc) >= (*sk).sk_rcvbuf { rfcomm_dlc_throttle(d); }
}

unsafe fn rfcomm_sk_state_change(d: *mut rfcomm_dlc, err: i32) {
    let sk = (*d).owner;
    if sk.is_null() { return; }
    let parent;
    BT_DBG!("dlc %p state %ld err %d", d, (*d).state, err);
    lock_sock(sk);
    if err != 0 { (*sk).sk_err = err; }
    (*sk).sk_state = (*d).state;
    parent = (*bt_sk(sk)).parent;
    if !parent.is_null() {
        if (*d).state == BT_CLOSED { sock_set_flag(sk, SOCK_ZAPPED); bt_accept_unlink(sk); }
        ((*parent).sk_data_ready)(parent);
    } else {
        if (*d).state == BT_CONNECTED { rfcomm_session_getaddr((*d).session, &mut (*rfcomm_pi(sk)).src, core::ptr::null_mut()); }
        ((*sk).sk_state_change)(sk);
    }
    release_sock(sk);
    if !parent.is_null() && sock_flag(sk, SOCK_ZAPPED) {
        rfcomm_dlc_unlock(d); rfcomm_sock_kill(sk); rfcomm_dlc_lock(d);
    }
}

unsafe fn __rfcomm_get_listen_sock_by_addr(channel: u8, src: *mut bdaddr_t) -> *mut sock {
    let mut sk: *mut sock = core::ptr::null_mut();
    sk_for_each!(sk, &rfcomm_sk_list.head, {
        if (*rfcomm_pi(sk)).channel != channel { continue; }
        if bacmp(&(*rfcomm_pi(sk)).src, src) != 0 { continue; }
        if (*sk).sk_state == BT_BOUND || (*sk).sk_state == BT_LISTEN { break; }
    });
    if !sk.is_null() { sk } else { core::ptr::null_mut() }
}

unsafe fn rfcomm_get_sock_by_channel(state: i32, channel: u8, src: *mut bdaddr_t) -> *mut sock {
    let mut sk: *mut sock = core::ptr::null_mut();
    let mut sk1: *mut sock = core::ptr::null_mut();
    read_lock(&mut rfcomm_sk_list.lock);
    sk_for_each!(sk, &rfcomm_sk_list.head, {
        if state != 0 && (*sk).sk_state != state { continue; }
        if (*rfcomm_pi(sk)).channel == channel {
            if bacmp(&(*rfcomm_pi(sk)).src, src) == 0 { sock_hold(sk); break; }
            if bacmp(&(*rfcomm_pi(sk)).src, &BDADDR_ANY) == 0 {
                if !sk1.is_null() { sock_put(sk1); }
                sk1 = sk; sock_hold(sk1);
            }
        }
    });
    if !sk.is_null() && !sk1.is_null() { sock_put(sk1); }
    read_unlock(&mut rfcomm_sk_list.lock);
    if !sk.is_null() { sk } else { sk1 }
}

unsafe fn rfcomm_sock_destruct(sk: *mut sock) {
    let d = (*rfcomm_pi(sk)).dlc;
    BT_DBG!("sk %p dlc %p", sk, d);
    skb_queue_purge(&mut (*sk).sk_receive_queue); skb_queue_purge(&mut (*sk).sk_write_queue);
    rfcomm_dlc_lock(d); (*rfcomm_pi(sk)).dlc = core::ptr::null_mut();
    if (*d).owner == sk { (*d).owner = core::ptr::null_mut(); }
    rfcomm_dlc_unlock(d); rfcomm_dlc_put(d);
}

unsafe fn rfcomm_sock_cleanup_listen(parent: *mut sock) {
    let mut sk;
    BT_DBG!("parent %p", parent);
    loop {
        sk = bt_accept_dequeue(parent, core::ptr::null_mut());
        if sk.is_null() { break; }
        rfcomm_sock_close(sk); rfcomm_sock_kill(sk); sock_put(sk);
    }
    (*parent).sk_state = BT_CLOSED; sock_set_flag(parent, SOCK_ZAPPED);
}

unsafe fn rfcomm_sock_kill(sk: *mut sock) {
    if !sock_flag(sk, SOCK_ZAPPED) || !(*sk).sk_socket.is_null() { return; }
    BT_DBG!("sk %p state %d refcnt %d", sk, (*sk).sk_state, refcount_read(&(*sk).sk_refcnt));
    bt_sock_unlink(&mut rfcomm_sk_list, sk); sock_set_flag(sk, SOCK_DEAD); sock_put(sk);
}

unsafe fn __rfcomm_sock_close(sk: *mut sock) {
    let d = (*rfcomm_pi(sk)).dlc;
    BT_DBG!("sk %p state %d socket %p", sk, (*sk).sk_state, (*sk).sk_socket);
    match (*sk).sk_state {
        BT_LISTEN => rfcomm_sock_cleanup_listen(sk),
        BT_CONNECT | BT_CONNECT2 | BT_CONFIG | BT_CONNECTED => { rfcomm_dlc_close(d, 0); sock_set_flag(sk, SOCK_ZAPPED); },
        _ => sock_set_flag(sk, SOCK_ZAPPED),
    }
}
unsafe fn rfcomm_sock_close(sk: *mut sock) { lock_sock(sk); __rfcomm_sock_close(sk); release_sock(sk); }

unsafe fn rfcomm_sock_init(sk: *mut sock, parent: *mut sock) {
    let pi = rfcomm_pi(sk); BT_DBG!("sk %p", sk);
    if !parent.is_null() {
        (*sk).sk_type = (*parent).sk_type;
        (*pi).dlc.defer_setup = test_bit(BT_SK_DEFER_SETUP, &(*bt_sk(parent)).flags);
        (*pi).sec_level = (*rfcomm_pi(parent)).sec_level; (*pi).role_switch = (*rfcomm_pi(parent)).role_switch;
        security_sk_clone(parent, sk);
    } else { (*pi).dlc.defer_setup = 0; (*pi).sec_level = BT_SECURITY_LOW; (*pi).role_switch = 0; }
    (*pi).dlc.sec_level = (*pi).sec_level; (*pi).dlc.role_switch = (*pi).role_switch;
}

static mut rfcomm_proto: proto = proto { name: "RFCOMM", owner: THIS_MODULE, obj_size: core::mem::size_of::<rfcomm_pinfo>() };

unsafe fn rfcomm_sock_alloc(net: *mut net, sock: *mut socket, proto: i32, prio: gfp_t, kern: i32) -> *mut sock {
    let d = rfcomm_dlc_alloc(prio); if d.is_null() { return core::ptr::null_mut(); }
    let sk = bt_sock_alloc(net, sock, &mut rfcomm_proto, proto, prio, kern);
    if sk.is_null() { rfcomm_dlc_free(d); return core::ptr::null_mut(); }
    (*d).data_ready = Some(rfcomm_sk_data_ready); (*d).state_change = Some(rfcomm_sk_state_change);
    (*rfcomm_pi(sk)).dlc = d; (*d).owner = sk; (*sk).sk_destruct = Some(rfcomm_sock_destruct);
    (*sk).sk_sndtimeo = RFCOMM_CONN_TIMEOUT;
    (*sk).sk_sndbuf = RFCOMM_MAX_CREDITS * RFCOMM_DEFAULT_MTU * 10;
    (*sk).sk_rcvbuf = RFCOMM_MAX_CREDITS * RFCOMM_DEFAULT_MTU * 10;
    bt_sock_link(&mut rfcomm_sk_list, sk); BT_DBG!("sk %p", sk); sk
}

unsafe fn rfcomm_sock_create(net: *mut net, socket: *mut socket, protocol: i32, kern: i32) -> i32 {
    BT_DBG!("sock %p", socket); (*socket).state = SS_UNCONNECTED;
    if (*socket).type_ != SOCK_STREAM && (*socket).type_ != SOCK_RAW { return -ESOCKTNOSUPPORT; }
    (*socket).ops = &rfcomm_sock_ops;
    let sk = rfcomm_sock_alloc(net, socket, protocol, GFP_ATOMIC, kern); if sk.is_null() { return -ENOMEM; }
    rfcomm_sock_init(sk, core::ptr::null_mut()); 0
}

unsafe fn rfcomm_sock_bind(sock: *mut socket, addr: *mut sockaddr_unsized, addr_len: i32) -> i32 { let sk=(*sock).sk; if addr.is_null() || addr_len < core::mem::size_of::<u16>() as i32 || (*addr).sa_family != AF_BLUETOOTH { return -EINVAL; } lock_sock(sk); if (*sk).sk_state != BT_OPEN { release_sock(sk); return -EBADFD; } if (*sk).sk_type != SOCK_STREAM { release_sock(sk); return -EINVAL; } write_lock(&mut rfcomm_sk_list.lock); let sa=addr as *mut sockaddr_rc; if (*sa).rc_channel != 0 && !__rfcomm_get_listen_sock_by_addr((*sa).rc_channel, &mut (*sa).rc_bdaddr).is_null() { write_unlock(&mut rfcomm_sk_list.lock); release_sock(sk); return -EADDRINUSE; } bacpy(&mut (*rfcomm_pi(sk)).src, &mut (*sa).rc_bdaddr); (*rfcomm_pi(sk)).channel=(*sa).rc_channel; (*sk).sk_state=BT_BOUND; write_unlock(&mut rfcomm_sk_list.lock); release_sock(sk); 0 }
unsafe fn rfcomm_sock_connect(sock: *mut socket, addr: *mut sockaddr_unsized, alen: i32, flags: i32) -> i32 { let sk=(*sock).sk; let sa=addr as *mut sockaddr_rc; let d=(*rfcomm_pi(sk)).dlc; if alen < core::mem::size_of::<sockaddr_rc>() as i32 || (*addr).sa_family != AF_BLUETOOTH { return -EINVAL; } sock_hold(sk); lock_sock(sk); if (*sk).sk_state != BT_OPEN && (*sk).sk_state != BT_BOUND { release_sock(sk); sock_put(sk); return -EBADFD; } if (*sk).sk_type != SOCK_STREAM { release_sock(sk); sock_put(sk); return -EINVAL; } (*sk).sk_state=BT_CONNECT; bacpy(&mut (*rfcomm_pi(sk)).dst,&mut (*sa).rc_bdaddr); (*rfcomm_pi(sk)).channel=(*sa).rc_channel; release_sock(sk); let mut err=rfcomm_dlc_open(d,&mut (*rfcomm_pi(sk)).src,&mut (*sa).rc_bdaddr,(*sa).rc_channel); lock_sock(sk); if err==0 && !sock_flag(sk,SOCK_ZAPPED) { err=bt_sock_wait_state(sk,BT_CONNECTED,sock_sndtimeo(sk,flags & O_NONBLOCK)); } release_sock(sk); sock_put(sk); err }
unsafe fn rfcomm_sock_listen(sock: *mut socket, backlog: i32) -> i32 { let sk=(*sock).sk; lock_sock(sk); if (*sk).sk_state != BT_BOUND { release_sock(sk); return -EBADFD; } if (*sk).sk_type != SOCK_STREAM { release_sock(sk); return -EINVAL; } if (*rfcomm_pi(sk)).channel==0 { write_lock(&mut rfcomm_sk_list.lock); for c in 1..31 { if __rfcomm_get_listen_sock_by_addr(c,&mut (*rfcomm_pi(sk)).src).is_null() { (*rfcomm_pi(sk)).channel=c; break; } } write_unlock(&mut rfcomm_sk_list.lock); } (*sk).sk_max_ack_backlog=backlog; (*sk).sk_ack_backlog=0; (*sk).sk_state=BT_LISTEN; release_sock(sk); 0 }
unsafe fn rfcomm_sock_accept(sock: *mut socket, newsock: *mut socket, _arg: *mut proto_accept_arg) -> i32 { let sk=(*sock).sk; lock_sock_nested(sk,SINGLE_DEPTH_NESTING); if (*sk).sk_type != SOCK_STREAM { release_sock(sk); return -EINVAL; } let nsk=bt_accept_dequeue(sk,newsock); if nsk.is_null() { release_sock(sk); return -EAGAIN; } sock_put(nsk); (*newsock).state=SS_CONNECTED; release_sock(sk); 0 }
unsafe fn rfcomm_sock_getname(sock: *mut socket, addr: *mut sockaddr, peer: i32) -> i32 { let sk=(*sock).sk; if peer && (*sk).sk_state != BT_CONNECTED && (*sk).sk_state != BT_CONNECT && (*sk).sk_state != BT_CONNECT2 { return -ENOTCONN; } let sa=addr as *mut sockaddr_rc; core::ptr::write_bytes(sa,0,1); (*sa).rc_family=AF_BLUETOOTH; (*sa).rc_channel=(*rfcomm_pi(sk)).channel; if peer { bacpy(&mut (*sa).rc_bdaddr,&mut (*rfcomm_pi(sk)).dst); } else { bacpy(&mut (*sa).rc_bdaddr,&mut (*rfcomm_pi(sk)).src); } core::mem::size_of::<sockaddr_rc>() as i32 }
unsafe fn rfcomm_sock_sendmsg(sock: *mut socket, msg: *mut msghdr, len: usize) -> i32 { let sk=(*sock).sk; let d=(*rfcomm_pi(sk)).dlc; if test_bit(RFCOMM_DEFER_SETUP,&(*d).flags) { return -ENOTCONN; } if (*msg).msg_flags & MSG_OOB != 0 { return -EOPNOTSUPP; } if (*sk).sk_shutdown & SEND_SHUTDOWN != 0 { return -EPIPE; } let skb=bt_skb_sendmmsg(sk,msg,len,(*d).mtu,RFCOMM_SKB_HEAD_RESERVE,RFCOMM_SKB_TAIL_RESERVE); if IS_ERR(skb) { return PTR_ERR(skb); } let n=rfcomm_dlc_send(d,skb); if n<0 { kfree_skb(skb); } n }
unsafe fn rfcomm_sock_recvmsg(sock: *mut socket, msg: *mut msghdr, size: usize, flags: i32) -> i32 { let sk=(*sock).sk; let d=(*rfcomm_pi(sk)).dlc; if test_and_clear_bit(RFCOMM_DEFER_SETUP,&mut (*d).flags) { rfcomm_dlc_accept(d); return 0; } let n=bt_sock_stream_recvmsg(sock,msg,size,flags); lock_sock(sk); if flags & MSG_PEEK == 0 && n>0 { atomic_sub(n,&mut (*sk).sk_rmem_alloc); } if atomic_read(&(*sk).sk_rmem_alloc) <= ((*sk).sk_rcvbuf>>2) { rfcomm_dlc_unthrottle(d); } release_sock(sk); n }
unsafe fn rfcomm_sock_setsockopt(sock: *mut socket, level: i32, optname: i32, optval: sockptr_t, optlen: u32) -> i32 { if level==SOL_RFCOMM { return -ENOPROTOOPT; } if level!=SOL_BLUETOOTH { return -ENOPROTOOPT; } let sk=(*sock).sk; lock_sock(sk); let mut sec=bt_security{level:BT_SECURITY_LOW,key_size:0}; let e=copy_safe_from_sockptr(&mut sec,core::mem::size_of::<bt_security>(),optval,optlen); release_sock(sk); if e!=0 { return e; } if optname==BT_SECURITY { (*rfcomm_pi(sk)).sec_level=sec.level; 0 } else { -ENOPROTOOPT } }
unsafe fn rfcomm_sock_getsockopt(sock: *mut socket, level: i32, optname: i32, sopt: *mut sockopt_t) -> i32 { if level!=SOL_BLUETOOTH { return -ENOPROTOOPT; } let sk=(*sock).sk; if optname==BT_SECURITY { let sec=bt_security{level:(*rfcomm_pi(sk)).sec_level,key_size:0}; if copy_to_iter(&sec,core::mem::size_of::<bt_security>(),&mut (*sopt).iter_out)!=core::mem::size_of::<bt_security>() { return -EFAULT; } 0 } else { -ENOPROTOOPT } }
unsafe fn rfcomm_sock_ioctl(sock: *mut socket, cmd: u32, arg: usize) -> i32 { bt_sock_ioctl(sock,cmd,arg) }
unsafe fn rfcomm_sock_shutdown(sock: *mut socket, _how: i32) -> i32 { let sk=(*sock).sk; if sk.is_null(){return 0;} lock_sock(sk); (*sk).sk_shutdown=SHUTDOWN_MASK; release_sock(sk); __rfcomm_sock_close(sk); 0 }
unsafe fn rfcomm_sock_release(sock: *mut socket) -> i32 { if (*sock).sk.is_null(){return 0;} let e=rfcomm_sock_shutdown(sock,2); sock_orphan((*sock).sk); rfcomm_sock_kill((*sock).sk); e }

unsafe fn rfcomm_connect_ind(s: *mut rfcomm_session, channel: u8, d: *mut *mut rfcomm_dlc) -> i32;
unsafe fn rfcomm_sock_debugfs_show(f: *mut seq_file, p: *mut core::ffi::c_void) -> i32;

static mut rfcomm_sock_debugfs: *mut dentry = core::ptr::null_mut();
static mut rfcomm_sock_family_ops: net_proto_family = net_proto_family { family: PF_BLUETOOTH, owner: THIS_MODULE, create: Some(rfcomm_sock_create) };

unsafe fn rfcomm_init_sockets() -> i32 {
    BUILD_BUG_ON!(core::mem::size_of::<sockaddr_rc>() > core::mem::size_of::<sockaddr>());
    let mut err = proto_register(&mut rfcomm_proto, 0); if err < 0 { return err; }
    err = bt_sock_register(BTPROTO_RFCOMM, &mut rfcomm_sock_family_ops); if err < 0 { BT_ERR!("RFCOMM socket layer registration failed"); proto_unregister(&mut rfcomm_proto); return err; }
    err = bt_procfs_init(&init_net, "rfcomm", &mut rfcomm_sk_list, core::ptr::null_mut()); if err < 0 { BT_ERR!("Failed to create RFCOMM proc file"); bt_sock_unregister(BTPROTO_RFCOMM); proto_unregister(&mut rfcomm_proto); return err; }
    BT_INFO!("RFCOMM socket layer initialized");
    if IS_ERR_OR_NULL(bt_debugfs) { return 0; }
    rfcomm_sock_debugfs = debugfs_create_file!("rfcomm", 0o444, bt_debugfs, core::ptr::null_mut(), &rfcomm_sock_debugfs_fops); 0
}
unsafe fn rfcomm_cleanup_sockets() { bt_procfs_cleanup(&init_net, "rfcomm"); debugfs_remove(rfcomm_sock_debugfs); bt_sock_unregister(BTPROTO_RFCOMM); proto_unregister(&mut rfcomm_proto); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
