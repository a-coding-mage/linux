// SPDX-License-Identifier: GPL-2.0-or-later
/* AF_RXRPC implementation. Direct low-level translation of af_rxrpc.c. */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation unit. Their names are intentionally left unresolved here.

pub static mut rxrpc_debug: ::core::ffi::c_uint = 0;
static mut rxrpc_proto: proto = unsafe { ::core::mem::zeroed() };
static mut rxrpc_rpc_ops: proto_ops = unsafe { ::core::mem::zeroed() };
pub static mut rxrpc_debug_id: atomic_t = unsafe { ::core::mem::zeroed() };
pub static mut rxrpc_n_rx_skbs: atomic_t = unsafe { ::core::mem::zeroed() };
pub static mut rxrpc_workqueue: *mut workqueue_struct = core::ptr::null_mut();

unsafe extern "C" {
    fn rxrpc_sk(sk: *mut sock) -> *mut rxrpc_sock;
    fn rxrpc_lookup_local(net: *mut net, srx: *const sockaddr_rxrpc) -> *mut rxrpc_local;
    fn rxrpc_unuse_local(local: *mut rxrpc_local, reason: i32);
    fn rxrpc_put_local(local: *mut rxrpc_local, reason: i32);
    fn rxrpc_lookup_peer(local: *mut rxrpc_local, srx: *const sockaddr_rxrpc, gfp: gfp_t) -> *mut rxrpc_peer;
    fn rxrpc_get_peer(peer: *mut rxrpc_peer, reason: i32) -> *mut rxrpc_peer;
    fn rxrpc_put_peer(peer: *mut rxrpc_peer, reason: i32);
    fn rxrpc_new_client_call(rx: *mut rxrpc_sock, cp: *mut rxrpc_conn_parameters, p: *mut rxrpc_call_params, gfp: gfp_t, debug_id: u32) -> *mut rxrpc_call;
    fn rxrpc_release_call(rx: *mut rxrpc_sock, call: *mut rxrpc_call);
    fn rxrpc_put_call(call: *mut rxrpc_call, reason: i32);
    fn rxrpc_call_is_complete(call: *const rxrpc_call) -> bool;
    fn rxrpc_set_min_security_level(sk: *mut sock, val: u32) -> i32;
    fn rxrpc_request_key(rx: *mut rxrpc_sock, optval: sockptr_t, optlen: u32) -> i32;
    fn rxrpc_server_keyring(rx: *mut rxrpc_sock, optval: sockptr_t, optlen: u32) -> i32;
    fn rxrpc_sendmsg_oob(rx: *mut rxrpc_sock, m: *mut msghdr, len: usize) -> i32;
    fn rxrpc_do_sendmsg(rx: *mut rxrpc_sock, m: *mut msghdr, len: usize) -> i32;
    fn rxrpc_recvmsg(sock: *mut socket, m: *mut msghdr, len: usize, flags: u32) -> i32;
    fn rxrpc_service_prealloc(rx: *mut rxrpc_sock, gfp: gfp_t) -> i32;
    fn rxrpc_discard_prealloc(rx: *mut rxrpc_sock);
    fn rxrpc_release_calls_on_socket(rx: *mut rxrpc_sock);
    fn rxrpc_purge_queue(q: *mut sk_buff_head);
    fn rxrpc_kernel_free_oob(skb: *mut sk_buff);
    fn rxrpc_gen_version_string();
    fn rxrpc_init_security() -> i32;
    fn rxrpc_exit_security();
    fn rxrpc_sysctl_init() -> i32;
    fn rxrpc_sysctl_exit();
}

#[inline]
unsafe fn rxrpc_writable(sk: *mut sock) -> bool {
    refcount_read(&(*sk).sk_wmem_alloc) < (*sk).sk_sndbuf as usize
}

unsafe fn rxrpc_write_space(sk: *mut sock) {
    rcu_read_lock();
    if rxrpc_writable(sk) {
        let wq = rcu_dereference((*sk).sk_wq);
        if skwq_has_sleeper(wq) { wake_up_interruptible(&(*wq).wait); }
        sk_wake_async_rcu(sk, SOCK_WAKE_SPACE, POLL_OUT);
    }
    rcu_read_unlock();
}

unsafe fn rxrpc_validate_address(rx: *mut rxrpc_sock, srx: *mut sockaddr_rxrpc, mut len: i32) -> i32 {
    if len < core::mem::size_of::<sockaddr_rxrpc>() as i32 { return -EINVAL; }
    if (*srx).srx_family != AF_RXRPC { return -EAFNOSUPPORT; }
    if (*srx).transport_type != SOCK_DGRAM { return -ESOCKTNOSUPPORT; }
    len -= core::mem::offset_of!(sockaddr_rxrpc, transport) as i32;
    if (*srx).transport_len < core::mem::size_of::<sa_family_t>() as u16 || (*srx).transport_len as i32 > len { return -EINVAL; }
    let tail: usize;
    match (*srx).transport.family {
        AF_INET => {
            if (*rx).family != AF_INET && (*rx).family != AF_INET6 { return -EAFNOSUPPORT; }
            if (*srx).transport_len < core::mem::size_of::<sockaddr_in>() as u16 { return -EINVAL; }
            tail = core::mem::offset_of!(sockaddr_rxrpc, transport) + core::mem::offset_of!(sockaddr_in, __pad);
        },
        AF_INET6 => {
            if (*rx).family != AF_INET6 { return -EAFNOSUPPORT; }
            if (*srx).transport_len < core::mem::size_of::<sockaddr_in6>() as u16 { return -EINVAL; }
            tail = core::mem::offset_of!(sockaddr_rxrpc, transport) + core::mem::size_of::<sockaddr_in6>();
        },
        _ => return -EAFNOSUPPORT,
    }
    if tail < len as usize { core::ptr::write_bytes((srx as *mut u8).add(tail), 0, len as usize - tail); }
    0
}

unsafe fn rxrpc_bind(sock: *mut socket, saddr: *mut sockaddr_unsized, len: i32) -> i32 {
    let srx = saddr as *mut sockaddr_rxrpc; let rx = rxrpc_sk((*sock).sk); let service_id = (*srx).srx_service;
    let mut ret = rxrpc_validate_address(rx, srx, len); if ret < 0 { return ret; }
    lock_sock(&mut (*rx).sk);
    match (*rx).sk.sk_state {
        RXRPC_UNBOUND => {
            (*rx).srx = *srx; let local = rxrpc_lookup_local(sock_net((*rx).sk), &(*rx).srx);
            if IS_ERR(local) { ret = PTR_ERR(local); release_sock(&mut (*rx).sk); return ret; }
            if service_id != 0 {
                write_lock(&mut (*local).services_lock);
                if !(*local).service.is_null() { write_unlock(&mut (*local).services_lock); rxrpc_unuse_local(local, rxrpc_local_unuse_bind); rxrpc_put_local(local, rxrpc_local_put_bind); release_sock(&mut (*rx).sk); return -EADDRINUSE; }
                (*rx).local = local; (*local).service = rx; write_unlock(&mut (*local).services_lock); (*rx).sk.sk_state = RXRPC_SERVER_BOUND;
            } else { (*rx).local = local; (*rx).sk.sk_state = RXRPC_CLIENT_BOUND; }
        },
        RXRPC_SERVER_BOUND => {
            if service_id == 0 { ret = -EINVAL; } else if service_id == (*rx).srx.srx_service { ret = -EADDRINUSE; } else { (*srx).srx_service = (*rx).srx.srx_service; if core::ptr::read(srx) != (*rx).srx { ret = -EINVAL; } else { (*rx).second_service = service_id; (*rx).sk.sk_state = RXRPC_SERVER_BOUND2; ret = 0; } }
            if ret < 0 { release_sock(&mut (*rx).sk); return ret; }
        },
        _ => { release_sock(&mut (*rx).sk); return -EINVAL; }
    }
    release_sock(&mut (*rx).sk); 0
}

unsafe fn rxrpc_listen(sock: *mut socket, mut backlog: i32) -> i32 {
    let sk = (*sock).sk; let rx = rxrpc_sk(sk); lock_sock(&mut (*rx).sk);
    let ret = match (*rx).sk.sk_state {
        RXRPC_UNBOUND => -EADDRNOTAVAIL,
        RXRPC_SERVER_BOUND | RXRPC_SERVER_BOUND2 => { let max = READ_ONCE(rxrpc_max_backlog); if backlog == INT_MAX { backlog = max as i32; } else if backlog < 0 || backlog as u32 > max { release_sock(&mut (*rx).sk); return -EINVAL; } let old = (*sk).sk_max_ack_backlog; (*sk).sk_max_ack_backlog = backlog as u32; let r = rxrpc_service_prealloc(rx, GFP_KERNEL); if r == 0 { (*rx).sk.sk_state = RXRPC_SERVER_LISTENING; } else { (*sk).sk_max_ack_backlog = old; } r },
        RXRPC_SERVER_LISTENING => { if backlog == 0 { (*rx).sk.sk_state = RXRPC_SERVER_LISTEN_DISABLED; (*sk).sk_max_ack_backlog = 0; rxrpc_discard_prealloc(rx); 0 } else { -EBUSY } },
        _ => -EBUSY,
    }; release_sock(&mut (*rx).sk); ret
}

pub unsafe fn rxrpc_kernel_lookup_peer(sock: *mut socket, srx: *mut sockaddr_rxrpc, gfp: gfp_t) -> *mut rxrpc_peer { let rx = rxrpc_sk((*sock).sk); if rxrpc_validate_address(rx, srx, core::mem::size_of::<sockaddr_rxrpc>() as i32) < 0 { return ERR_PTR(-EINVAL); } let p = rxrpc_lookup_peer((*rx).local, srx, gfp); if p.is_null() { ERR_PTR(-ENOMEM) } else { p } }
pub unsafe fn rxrpc_kernel_get_peer(peer: *mut rxrpc_peer) -> *mut rxrpc_peer { if peer.is_null() { core::ptr::null_mut() } else { rxrpc_get_peer(peer, rxrpc_peer_get_application) } }
pub unsafe fn rxrpc_kernel_put_peer(peer: *mut rxrpc_peer) { rxrpc_put_peer(peer, rxrpc_peer_put_application); }

pub unsafe fn rxrpc_kernel_begin_call(sock:*mut socket,peer:*mut rxrpc_peer,key:*mut key,user_call_ID:usize,tx_total_len:i64,hard_timeout:u32,gfp:gfp_t,notify_rx:rxrpc_notify_rx_t,service_id:u16,upgrade:bool,interruptibility:rxrpc_interruptibility,debug_id:u32)->*mut rxrpc_call{let rx=rxrpc_sk((*sock).sk);if (*peer).local!=(*rx).local{return ERR_PTR(-EIO);}lock_sock(&mut (*rx).sk);let use_key=if key.is_null(){(*rx).key}else{key};let mut cp:rxrpc_conn_parameters=core::mem::zeroed();cp.local=(*rx).local;cp.peer=peer;cp.key=use_key;cp.security_level=(*rx).min_sec_level;cp.upgrade=upgrade;cp.service_id=service_id;let mut p:rxrpc_call_params=core::mem::zeroed();p.user_call_ID=user_call_ID;p.tx_total_len=tx_total_len;p.interruptibility=interruptibility;p.kernel=true;p.timeouts.hard=hard_timeout;let call=rxrpc_new_client_call(rx,&mut cp,&mut p,gfp,debug_id);if !IS_ERR(call){(*call).notify_rx=notify_rx;mutex_unlock(&mut (*call).user_mutex);}call}

unsafe fn rxrpc_dummy_notify_rx(_: *mut sock, _: *mut rxrpc_call, _: usize) {}

pub unsafe fn rxrpc_kernel_shutdown_call(sock: *mut socket, call: *mut rxrpc_call) { mutex_lock(&mut (*call).user_mutex); if !test_bit(RXRPC_CALL_RELEASED, &(*call).flags) { rxrpc_release_call(rxrpc_sk((*sock).sk), call); if !(*call).notify_rx.is_null() { spin_lock_irq(&mut (*call).notify_lock); (*call).notify_rx = Some(rxrpc_dummy_notify_rx); spin_unlock_irq(&mut (*call).notify_lock); } } mutex_unlock(&mut (*call).user_mutex); }
pub unsafe fn rxrpc_kernel_put_call(_: *mut socket, call: *mut rxrpc_call) { rxrpc_put_call(call, rxrpc_call_put_kernel); }
pub unsafe fn rxrpc_kernel_check_life(_: *const socket, call: *const rxrpc_call) -> bool { !rxrpc_call_is_complete(call) || ((*call).completion == RXRPC_CALL_SUCCEEDED && !skb_queue_empty(&(*call).recvmsg_queue)) }
pub unsafe fn rxrpc_kernel_set_notifications(sock: *mut socket, ops: *const rxrpc_kernel_ops) { (*rxrpc_sk((*sock).sk)).app_ops = ops; }

unsafe fn rxrpc_connect(sock: *mut socket, addr: *mut sockaddr_unsized, len: i32, _: i32) -> i32 { let rx = rxrpc_sk((*sock).sk); let srx = addr as *mut sockaddr_rxrpc; let r = rxrpc_validate_address(rx, srx, len); if r < 0 { return r; } lock_sock(&mut (*rx).sk); if test_bit(RXRPC_SOCK_CONNECTED, &(*rx).flags) { release_sock(&mut (*rx).sk); return -EISCONN; } match (*rx).sk.sk_state { RXRPC_UNBOUND => (*rx).sk.sk_state = RXRPC_CLIENT_UNBOUND, RXRPC_CLIENT_UNBOUND | RXRPC_CLIENT_BOUND => (), _ => { release_sock(&mut (*rx).sk); return -EBUSY; } } (*rx).connect_srx = *srx; set_bit(RXRPC_SOCK_CONNECTED, &mut (*rx).flags); release_sock(&mut (*rx).sk); 0 }

unsafe fn rxrpc_sendmsg(sock: *mut socket, m: *mut msghdr, len: usize) -> i32 { let rx = rxrpc_sk((*sock).sk); if (*m).msg_flags & MSG_OOB != 0 { return -EOPNOTSUPP; } if !(*m).msg_name.is_null() { let r = rxrpc_validate_address(rx, (*m).msg_name, (*m).msg_namelen); if r < 0 { return r; } } lock_sock(&mut (*rx).sk); match (*rx).sk.sk_state { RXRPC_UNBOUND | RXRPC_CLIENT_UNBOUND => { (*rx).srx.srx_family=AF_RXRPC; (*rx).srx.srx_service=0; (*rx).srx.transport_type=SOCK_DGRAM; (*rx).srx.transport.family=(*rx).family; (*rx).srx.transport_len=if (*rx).family==AF_INET {core::mem::size_of::<sockaddr_in>() as u16} else {core::mem::size_of::<sockaddr_in6>() as u16}; let local=rxrpc_lookup_local(sock_net((*sock).sk), &(*rx).srx); if IS_ERR(local) { release_sock(&mut (*rx).sk); return PTR_ERR(local); } (*rx).local=local; (*rx).sk.sk_state=RXRPC_CLIENT_BOUND; }, _=>() } let r=if (*m).msg_flags&MSG_OOB!=0 {rxrpc_sendmsg_oob(rx,m,len)} else {rxrpc_do_sendmsg(rx,m,len)}; r }

pub unsafe fn rxrpc_sock_set_min_security_level(sk: *mut sock, val: u32) -> i32 { if (*sk).sk_state != RXRPC_UNBOUND || val > RXRPC_SECURITY_MAX { return if (*sk).sk_state != RXRPC_UNBOUND {-EISCONN} else {-EINVAL}; } lock_sock(sk); (*rxrpc_sk(sk)).min_sec_level=val; release_sock(sk); 0 }

unsafe fn rxrpc_poll(file: *mut file, sock: *mut socket, wait: *mut poll_table) -> __poll_t { let rx=rxrpc_sk((*sock).sk); sock_poll_wait(file,sock,wait); let mut mask=0; if !list_empty(&(*rx).recvmsg_q) {mask|=EPOLLIN|EPOLLRDNORM;} if rxrpc_writable((*sock).sk) {mask|=EPOLLOUT|EPOLLWRNORM;} mask }

unsafe fn rxrpc_create(net: *mut net, sock: *mut socket, protocol: i32, kern: i32) -> i32 { if (*sock).type_ != SOCK_DGRAM {return -ESOCKTNOSUPPORT;} (*sock).state=SS_UNCONNECTED; let sk=sk_alloc(net,PF_RXRPC,GFP_KERNEL,&mut rxrpc_proto,kern); if sk.is_null(){return -ENOMEM;} sock_init_data(sock,sk); sock_set_flag(sk,SOCK_RCU_FREE); (*sk).sk_state=RXRPC_UNBOUND; (*sk).sk_write_space=Some(rxrpc_write_space); (*sk).sk_destruct=Some(rxrpc_sock_destructor); let rx=rxrpc_sk(sk); (*rx).family=protocol; (*rx).calls=RB_ROOT; spin_lock_init(&mut (*rx).incoming_lock); skb_queue_head_init(&mut (*rx).recvmsg_oobq); (*rx).pending_oobq=RB_ROOT; INIT_LIST_HEAD(&mut (*rx).sock_calls); INIT_LIST_HEAD(&mut (*rx).to_be_accepted); INIT_LIST_HEAD(&mut (*rx).recvmsg_q); spin_lock_init(&mut (*rx).recvmsg_lock); rwlock_init(&mut (*rx).call_lock); core::ptr::write_bytes(&mut (*rx).srx as *mut _ as *mut u8,0,core::mem::size_of::<sockaddr_rxrpc>()); 0 }

unsafe fn rxrpc_sock_destructor(sk: *mut sock) { let rx=rxrpc_sk(sk); rxrpc_purge_oob_queue(sk); rxrpc_purge_queue(&mut (*sk).sk_receive_queue); let _=rx; }
unsafe fn rxrpc_purge_oob_queue(sk:*mut sock){let rx=rxrpc_sk(sk); while let Some(skb)=skb_dequeue(&mut (*rx).recvmsg_oobq){rxrpc_kernel_free_oob(skb);} while !RB_EMPTY_ROOT(&(*rx).pending_oobq){let skb=rb_entry((*rx).pending_oobq.rb_node,sk_buff,rbnode);rb_erase(&mut (*skb).rbnode,&mut (*rx).pending_oobq);rxrpc_kernel_free_oob(skb);}}

unsafe fn rxrpc_shutdown(sock:*mut socket, flags:i32)->i32{if flags!=SHUT_RDWR{return -EOPNOTSUPP;}let sk=(*sock).sk;if (*sk).sk_state==RXRPC_CLOSE{return -ESHUTDOWN;}lock_sock(sk);if (*sk).sk_state<RXRPC_CLOSE{spin_lock_irq(&mut (*rxrpc_sk(sk)).recvmsg_lock);(*sk).sk_state=RXRPC_CLOSE;(*sk).sk_shutdown=SHUTDOWN_MASK;spin_unlock_irq(&mut (*rxrpc_sk(sk)).recvmsg_lock);}else{release_sock(sk);return -ESHUTDOWN;}rxrpc_discard_prealloc(rxrpc_sk(sk));release_sock(sk);0}
unsafe fn rxrpc_release_sock(sk:*mut sock)->i32{let rx=rxrpc_sk(sk);sock_orphan(sk);(*sk).sk_shutdown=SHUTDOWN_MASK;spin_lock_irq(&mut (*rx).recvmsg_lock);(*sk).sk_state=RXRPC_CLOSE;spin_unlock_irq(&mut (*rx).recvmsg_lock);rxrpc_discard_prealloc(rx);rxrpc_release_calls_on_socket(rx);flush_workqueue(rxrpc_workqueue);rxrpc_purge_oob_queue(sk);rxrpc_purge_queue(&mut (*sk).sk_receive_queue);if !(*rx).local.is_null(){rxrpc_unuse_local((*rx).local,rxrpc_local_unuse_release_sock);rxrpc_put_local((*rx).local,rxrpc_local_put_release_sock);(*rx).local=core::ptr::null_mut();}key_put((*rx).key);(*rx).key=core::ptr::null_mut();key_put((*rx).securities);(*rx).securities=core::ptr::null_mut();sock_put(sk);0}
unsafe fn rxrpc_release(sock:*mut socket)->i32{let sk=(*sock).sk;if sk.is_null(){return 0;}(*sock).sk=core::ptr::null_mut();rxrpc_release_sock(sk)}

// Socket operation table; fields are supplied by the kernel ABI declarations.
#[allow(non_upper_case_globals)]
static rxrpc_rpc_ops_translation: proto_ops = proto_ops { family:PF_RXRPC, release:Some(rxrpc_release), bind:Some(rxrpc_bind), connect:Some(rxrpc_connect), listen:Some(rxrpc_listen), shutdown:Some(rxrpc_shutdown), poll:Some(rxrpc_poll), sendmsg:Some(rxrpc_sendmsg), recvmsg:Some(rxrpc_recvmsg), ..unsafe{core::mem::zeroed()} };

unsafe fn rxrpc_setsockopt(_: *mut socket, _: i32, _: i32, _: sockptr_t, _: u32)->i32{-EOPNOTSUPP}
unsafe fn rxrpc_getsockopt(_: *mut socket, _: i32, _: i32, _: *mut sockopt_t)->i32{-EOPNOTSUPP}

#[no_mangle]
pub unsafe extern "C" fn af_rxrpc_init()->i32{rxrpc_gen_version_string();let r=rxrpc_init_security();if r<0{return r;}0}
#[no_mangle]
pub unsafe extern "C" fn af_rxrpc_exit(){rxrpc_exit_security();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
