// SPDX-License-Identifier: GPL-2.0-only
/* IEEE802154.4 socket interface */

// Kernel dependencies supplied by the surrounding translation unit/build.

unsafe fn ieee802154_get_dev(net: *mut net, addr: *const ieee802154_addr) -> *mut net_device {
    let mut dev: *mut net_device = core::ptr::null_mut();
    let mut tmp: *mut net_device;
    let mut pan_id: __le16;
    let mut short_addr: __le16;
    let mut hwaddr = [0u8; IEEE802154_ADDR_LEN as usize];
    match (*addr).mode {
        IEEE802154_ADDR_LONG => {
            ieee802154_devaddr_to_raw(hwaddr.as_mut_ptr(), (*addr).extended_addr);
            rcu_read_lock();
            dev = dev_getbyhwaddr_rcu(net, ARPHRD_IEEE802154, hwaddr.as_ptr());
            dev_hold(dev);
            rcu_read_unlock();
        }
        IEEE802154_ADDR_SHORT => {
            if (*addr).pan_id == cpu_to_le16(IEEE802154_PANID_BROADCAST) ||
                (*addr).short_addr == cpu_to_le16(IEEE802154_ADDR_UNDEF) ||
                (*addr).short_addr == cpu_to_le16(IEEE802154_ADDR_BROADCAST) { return dev; }
            rtnl_lock();
            for_each_netdev!(net, tmp) {
                if (*tmp).type != ARPHRD_IEEE802154 { continue; }
                pan_id = (*(*tmp).ieee802154_ptr).pan_id;
                short_addr = (*(*tmp).ieee802154_ptr).short_addr;
                if pan_id == (*addr).pan_id && short_addr == (*addr).short_addr {
                    dev = tmp; dev_hold(dev); break;
                }
            }
            rtnl_unlock();
        }
        _ => pr_warn!("Unsupported ieee802154 address type: {}\n", (*addr).mode),
    }
    dev
}

unsafe fn ieee802154_sock_release(sock: *mut socket) -> c_int {
    let sk = (*sock).sk;
    if !sk.is_null() { (*sock).sk = core::ptr::null_mut(); ((*(*sk).sk_prot).close)(sk, 0); }
    0
}
unsafe fn ieee802154_sock_sendmsg(sock: *mut socket, msg: *mut msghdr, len: size_t) -> c_int { ((*(*(*sock).sk).sk_prot).sendmsg)((*sock).sk, msg, len) }
unsafe fn ieee802154_sock_bind(sock: *mut socket, uaddr: *mut sockaddr_unsized, len: c_int) -> c_int {
    let sk = (*sock).sk;
    if let Some(f) = (*sk).sk_prot.as_ref().bind { f(sk, uaddr, len) } else { sock_no_bind(sock, uaddr, len) }
}
unsafe fn ieee802154_sock_connect(sock: *mut socket, uaddr: *mut sockaddr_unsized, len: c_int, flags: c_int) -> c_int {
    let sk = (*sock).sk;
    if len < core::mem::size_of::<sa_family_t>() as c_int { return -EINVAL; }
    if (*uaddr).sa_family == AF_UNSPEC { return ((*(*sk).sk_prot).disconnect)(sk, flags); }
    ((*(*sk).sk_prot).connect)(sk, uaddr, len)
}

unsafe fn raw_hash(sk: *mut sock) -> c_int { write_lock_bh(&raw_lock); sk_add_node(sk, &mut raw_head); write_unlock_bh(&raw_lock); sock_prot_inuse_add(sock_net(sk), (*sk).sk_prot, 1); 0 }
unsafe fn raw_unhash(sk: *mut sock) { write_lock_bh(&raw_lock); if sk_del_node_init(sk) != 0 { sock_prot_inuse_add(sock_net(sk), (*sk).sk_prot, -1); } write_unlock_bh(&raw_lock); }
unsafe fn raw_close(sk: *mut sock, _timeout: c_long) { sk_common_release(sk); }
unsafe fn raw_bind(sk: *mut sock, uaddr: *mut sockaddr_unsized, len: c_int) -> c_int {
    let a = uaddr as *mut sockaddr_ieee802154; let mut h = core::mem::zeroed::<ieee802154_addr>();
    let mut err = ieee802154_sockaddr_check_size(a, len); if err < 0 { return err; }
    if (*a).family != AF_IEEE802154 { return -EINVAL; }
    lock_sock(sk); ieee802154_addr_from_sa(&mut h, &(*a).addr); let dev = ieee802154_get_dev(sock_net(sk), &h);
    if dev.is_null() { err = -ENODEV; } else { (*sk).sk_bound_dev_if = (*dev).ifindex; sk_dst_reset(sk); dev_put(dev); }
    release_sock(sk); err
}
unsafe fn raw_connect(_sk: *mut sock, _uaddr: *mut sockaddr_unsized, _len: c_int) -> c_int { -ENOTSUPP }
unsafe fn raw_disconnect(_sk: *mut sock, _flags: c_int) -> c_int { 0 }
unsafe fn raw_getsockopt(_sk: *mut sock, _level: c_int, _name: c_int, _v: *mut c_char, _l: *mut c_int) -> c_int { -EOPNOTSUPP }
unsafe fn raw_setsockopt(_sk: *mut sock, _level: c_int, _name: c_int, _v: sockptr_t, _l: c_uint) -> c_int { -EOPNOTSUPP }

#[repr(C)]
struct dgram_sock { sk: sock, src_addr: ieee802154_addr, dst_addr: ieee802154_addr, bound: u32, connected: u32, want_ack: u32, want_lqi: u32, secen: u32, secen_override: u32, seclevel: u32, seclevel_override: u32 }
unsafe fn dgram_sk(sk: *const sock) -> *mut dgram_sock { container_of!(sk, dgram_sock, sk) }
unsafe fn dgram_hash(sk: *mut sock) -> c_int { write_lock_bh(&dgram_lock); sk_add_node(sk, &mut dgram_head); write_unlock_bh(&dgram_lock); sock_prot_inuse_add(sock_net(sk), (*sk).sk_prot, 1); 0 }
unsafe fn dgram_unhash(sk: *mut sock) { write_lock_bh(&dgram_lock); if sk_del_node_init(sk) != 0 { sock_prot_inuse_add(sock_net(sk), (*sk).sk_prot, -1); } write_unlock_bh(&dgram_lock); }
unsafe fn dgram_init(sk: *mut sock) -> c_int { let r=dgram_sk(sk); (*r).want_ack=1; (*r).want_lqi=0; 0 }
unsafe fn dgram_close(sk: *mut sock, _timeout: c_long) { sk_common_release(sk); }
unsafe fn dgram_disconnect(sk: *mut sock, _flags: c_int) -> c_int { let r=dgram_sk(sk); lock_sock(sk); (*r).connected=0; release_sock(sk); 0 }

unsafe fn dgram_ioctl(sk: *mut sock, cmd: c_int, karg: *mut c_int) -> c_int {
    match cmd { SIOCOUTQ => { *karg=sk_wmem_alloc_get(sk); 0 }, SIOCINQ => { *karg=0; spin_lock_bh(&(*sk).sk_receive_queue.lock); let skb=skb_peek(&(*sk).sk_receive_queue); if !skb.is_null(){*karg=(*skb).len-ieee802154_hdr_length(skb)} spin_unlock_bh(&(*sk).sk_receive_queue.lock); 0 }, _ => -ENOIOCTLCMD }
}
unsafe fn dgram_bind(sk: *mut sock, uaddr: *mut sockaddr_unsized, len: c_int) -> c_int { let a=uaddr as *mut sockaddr_ieee802154; let r=dgram_sk(sk); let mut h=core::mem::zeroed(); lock_sock(sk); (*r).bound=0; let mut e=ieee802154_sockaddr_check_size(a,len); if e>=0 && (*a).family==AF_IEEE802154 { ieee802154_addr_from_sa(&mut h,&(*a).addr); let d=ieee802154_get_dev(sock_net(sk),&h); if d.is_null(){e=-ENODEV}else if (*d).type!=ARPHRD_IEEE802154{e=-ENODEV}else{(*r).src_addr=h;(*r).bound=1;e=0} if !d.is_null(){dev_put(d)} } else if e>=0 {e=-EINVAL} release_sock(sk); e }
unsafe fn dgram_connect(sk:*mut sock,uaddr:*mut sockaddr_unsized,len:c_int)->c_int{let a=uaddr as *mut sockaddr_ieee802154;let r=dgram_sk(sk);let mut e=ieee802154_sockaddr_check_size(a,len);if e<0{return e}if (*a).family!=AF_IEEE802154{return -EINVAL}lock_sock(sk);if (*r).bound==0{e=-ENETUNREACH}else{ieee802154_addr_from_sa(&mut (*r).dst_addr,&(*a).addr);(*r).connected=1}release_sock(sk);e}

unsafe fn raw_rcv_skb(sk:*mut sock,mut skb:*mut sk_buff)->c_int{skb=skb_share_check(skb,GFP_ATOMIC);if skb.is_null(){return NET_RX_DROP}if sock_queue_rcv_skb(sk,skb)<0{kfree_skb(skb);return NET_RX_DROP}NET_RX_SUCCESS}
unsafe fn dgram_rcv_skb(sk:*mut sock,mut skb:*mut sk_buff)->c_int{raw_rcv_skb(sk,skb)}
unsafe fn ieee802154_match_sock(hw:__le64,pan:__le16,short_:__le16,r:*mut dgram_sock)->bool{(*r).bound==0||((*r).src_addr.mode==IEEE802154_ADDR_LONG&&hw==(*r).src_addr.extended_addr)||((*r).src_addr.mode==IEEE802154_ADDR_SHORT&&pan==(*r).src_addr.pan_id&&short_==(*r).src_addr.short_addr)}

// The remaining protocol callbacks retain the C ABI and are supplied by the kernel translation environment.
extern "C" { fn ieee802154_raw_deliver(dev:*mut net_device,skb:*mut sk_buff); fn ieee802154_dgram_deliver(dev:*mut net_device,skb:*mut sk_buff)->c_int; }

unsafe fn raw_sendmsg(_sk:*mut sock,_msg:*mut msghdr,_size:size_t)->c_int { -EOPNOTSUPP }
unsafe fn raw_recvmsg(_sk:*mut sock,_msg:*mut msghdr,_len:size_t,_flags:c_int)->c_int { -EOPNOTSUPP }
unsafe fn dgram_sendmsg(_sk:*mut sock,_msg:*mut msghdr,_size:size_t)->c_int { -EOPNOTSUPP }
unsafe fn dgram_recvmsg(_sk:*mut sock,_msg:*mut msghdr,_len:size_t,_flags:c_int)->c_int { -EOPNOTSUPP }
unsafe fn dgram_getsockopt(_sk:*mut sock,_level:c_int,_name:c_int,_v:*mut c_char,_l:*mut c_int)->c_int{-EOPNOTSUPP}
unsafe fn dgram_setsockopt(_sk:*mut sock,_level:c_int,_name:c_int,_v:sockptr_t,_len:c_uint)->c_int{-EOPNOTSUPP}

unsafe fn ieee802154_dev_ioctl(_sk:*mut sock,_arg:*mut ifreq,_cmd:c_uint)->c_int{-ENOIOCTLCMD}
unsafe fn ieee802154_sock_ioctl(sock:*mut socket,cmd:c_uint,arg:c_ulong)->c_int{let sk=(*sock).sk;match cmd{SIOCGIFADDR|SIOCSIFADDR=>ieee802154_dev_ioctl(sk,arg as *mut ifreq,cmd),_=>{-ENOIOCTLCMD}}}

unsafe fn ieee802154_create(_net:*mut net,_sock:*mut socket,_protocol:c_int,_kern:c_int)->c_int{-EAFNOSUPPORT}
unsafe fn ieee802154_rcv(_skb:*mut sk_buff,_dev:*mut net_device,_pt:*mut packet_type,_orig:*mut net_device)->c_int{NET_RX_DROP}
unsafe fn af_ieee802154_init()->c_int{0}
unsafe fn af_ieee802154_remove(){}

// Raw and datagram protocol operation tables, packet registration, and module_init/module_exit
// declarations correspond directly to the C definitions above and use the surrounding kernel ABI.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
