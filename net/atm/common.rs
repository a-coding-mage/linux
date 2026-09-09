// SPDX-License-Identifier: GPL-2.0-only
/* net/atm/common.c - ATM sockets (common part for PVC and SVC) */
/* Written 1995-2000 by Werner Almesberger, EPFL LRC/ICA */

// Linux kernel headers and local headers from the C translation unit are
// intentionally supplied by the surrounding kernel translation.

pub static mut vcc_hash: [hlist_head; VCC_HTABLE_SIZE] = [hlist_head::default(); VCC_HTABLE_SIZE];
pub static mut vcc_sklist_lock: rwlock_t = rwlock_t::new();
static mut atm_dev_notify_chain: atomic_notifier_head = atomic_notifier_head::new();

unsafe fn __vcc_insert_socket(sk: *mut sock) {
    let vcc = atm_sk(sk);
    let head = &mut vcc_hash[((*vcc).vci as usize) & (VCC_HTABLE_SIZE - 1)];
    (*sk).sk_hash = (*vcc).vci & (VCC_HTABLE_SIZE - 1) as i32;
    sk_add_node(sk, head);
}

pub unsafe fn vcc_insert_socket(sk: *mut sock) {
    write_lock_irq(&mut vcc_sklist_lock);
    __vcc_insert_socket(sk);
    write_unlock_irq(&mut vcc_sklist_lock);
}

unsafe fn vcc_remove_socket(sk: *mut sock) {
    write_lock_irq(&mut vcc_sklist_lock);
    sk_del_node_init(sk);
    write_unlock_irq(&mut vcc_sklist_lock);
}

unsafe fn vcc_tx_ready(vcc: *mut atm_vcc, size: c_uint) -> bool {
    let sk = sk_atm(vcc);
    if sk_wmem_alloc_get(sk) != 0 && !atm_may_send(vcc, size) {
        pr_debug!("Sorry: wmem_alloc = {}, size = {}, sndbuf = {}\n", sk_wmem_alloc_get(sk), size, (*sk).sk_sndbuf);
        return false;
    }
    true
}

unsafe extern "C" fn vcc_sock_destruct(sk: *mut sock) {
    if atomic_read(&(*sk).sk_rmem_alloc) != 0 {
        printk!(KERN_DEBUG "{}: rmem leakage ({} bytes) detected.\n", "vcc_sock_destruct", atomic_read(&(*sk).sk_rmem_alloc));
    }
    if refcount_read(&(*sk).sk_wmem_alloc) != 0 {
        printk!(KERN_DEBUG "{}: wmem leakage ({} bytes) detected.\n", "vcc_sock_destruct", refcount_read(&(*sk).sk_wmem_alloc));
    }
}

unsafe extern "C" fn vcc_def_wakeup(sk: *mut sock) {
    rcu_read_lock();
    let wq = rcu_dereference((*sk).sk_wq);
    if skwq_has_sleeper(wq) { wake_up(&mut (*wq).wait); }
    rcu_read_unlock();
}

unsafe fn vcc_writable(sk: *mut sock) -> bool {
    let vcc = atm_sk(sk);
    ((*vcc).qos.txtp.max_sdu + refcount_read(&(*sk).sk_wmem_alloc)) <= (*sk).sk_sndbuf
}

unsafe extern "C" fn vcc_write_space(sk: *mut sock) {
    rcu_read_lock();
    if vcc_writable(sk) {
        let wq = rcu_dereference((*sk).sk_wq);
        if skwq_has_sleeper(wq) { wake_up_interruptible(&mut (*wq).wait); }
        sk_wake_async_rcu(sk, SOCK_WAKE_SPACE, POLL_OUT);
    }
    rcu_read_unlock();
}

unsafe extern "C" fn vcc_release_cb(sk: *mut sock) {
    let vcc = atm_sk(sk);
    if let Some(cb) = (*vcc).release_cb { cb(vcc); }
}

static mut vcc_proto: proto = proto {
    name: b"VCC\0".as_ptr() as *const c_char,
    owner: THIS_MODULE,
    obj_size: core::mem::size_of::<atm_vcc>(),
    release_cb: Some(vcc_release_cb),
};

pub unsafe fn vcc_create(net: *mut net, sock: *mut socket, _protocol: c_int, family: c_int, kern: c_int) -> c_int {
    (*sock).sk = core::ptr::null_mut();
    if (*sock).type_ == SOCK_STREAM { return -EINVAL; }
    let sk = sk_alloc(net, family, GFP_KERNEL, &mut vcc_proto, kern);
    if sk.is_null() { return -ENOMEM; }
    sock_init_data(sock, sk);
    (*sk).sk_state_change = Some(vcc_def_wakeup);
    (*sk).sk_write_space = Some(vcc_write_space);
    let vcc = atm_sk(sk);
    (*vcc).dev = core::ptr::null_mut();
    (*vcc).qos.txtp.max_sdu = 1 << 16;
    refcount_set(&mut (*sk).sk_wmem_alloc, SK_WMEM_ALLOC_BIAS);
    atomic_set(&mut (*sk).sk_rmem_alloc, 0);
    (*vcc).push = None; (*vcc).pop = None; (*vcc).owner = core::ptr::null_mut(); (*vcc).release_cb = None;
    (*vcc).vpi = 0; (*vcc).vci = 0; (*vcc).atm_options = 0; (*vcc).aal_options = 0;
    (*sk).sk_destruct = Some(vcc_sock_destruct);
    0
}

unsafe fn vcc_destroy_socket(sk: *mut sock) {
    let vcc = atm_sk(sk);
    set_bit(ATM_VF_CLOSE, &mut (*vcc).flags); clear_bit(ATM_VF_READY, &mut (*vcc).flags);
    if !(*vcc).dev.is_null() && (*(*vcc).dev).ops.close.is_some() { ((*(*vcc).dev).ops.close.unwrap())(vcc); }
    if let Some(push) = (*vcc).push { push(vcc, core::ptr::null_mut()); }
    module_put((*vcc).owner);
    while let Some(skb) = skb_dequeue(&mut (*sk).sk_receive_queue) { atm_return(vcc, (*skb).truesize); kfree_skb(skb); }
    if !(*vcc).dev.is_null() && (*(*vcc).dev).ops.owner != core::ptr::null_mut() { module_put((*(*vcc).dev).ops.owner); atm_dev_put((*vcc).dev); }
    vcc_remove_socket(sk);
}

pub unsafe fn vcc_release(sock: *mut socket) -> c_int {
    let sk = (*sock).sk;
    if !sk.is_null() { lock_sock(sk); vcc_destroy_socket(sk); release_sock(sk); sock_put(sk); }
    0
}

pub unsafe fn vcc_release_async(vcc: *mut atm_vcc, reply: c_int) {
    let sk = sk_atm(vcc); set_bit(ATM_VF_CLOSE, &mut (*vcc).flags); (*sk).sk_shutdown |= RCV_SHUTDOWN; (*sk).sk_err = -reply; ((*sk).sk_state_change.unwrap())(sk);
}

pub unsafe fn vcc_process_recv_queue(vcc: *mut atm_vcc) {
    let mut queue = sk_buff_head::default(); __skb_queue_head_init(&mut queue);
    let rq = &mut (*sk_atm(vcc)).sk_receive_queue; let flags: c_ulong = 0;
    spin_lock_irqsave(&mut rq.lock, &flags); skb_queue_splice_init(rq, &mut queue); spin_unlock_irqrestore(&mut rq.lock, flags);
    let mut skb = queue.head; while !skb.is_null() { let next = (*skb).next; __skb_unlink(skb, &mut queue); ((*vcc).push.unwrap())(vcc, skb); skb = next; }
}

pub unsafe fn atm_dev_signal_change(dev: *mut atm_dev, signal: c_char) {
    WARN_ON(signal < ATM_PHY_SIG_LOST || signal > ATM_PHY_SIG_FOUND);
    if (*dev).signal == signal { return; }
    (*dev).signal = signal; atomic_notifier_call_chain(&mut atm_dev_notify_chain, signal as c_ulong, dev as *mut c_void);
}

pub unsafe fn atm_dev_release_vccs(dev: *mut atm_dev) {
    write_lock_irq(&mut vcc_sklist_lock);
    for i in 0..VCC_HTABLE_SIZE { let head = &mut vcc_hash[i]; let mut s = head.first; while !s.is_null() { let next = (*s).next; let vcc = atm_sk(s); if (*vcc).dev == dev { vcc_release_async(vcc, -EPIPE); sk_del_node_init(s); } s = next; } }
    write_unlock_irq(&mut vcc_sklist_lock);
}

unsafe fn adjust_tp(tp: *mut atm_trafprm, aal: c_uchar) -> c_int {
    if (*tp).traffic_class == 0 { return 0; }
    let max_sdu = match aal { ATM_AAL0 => ATM_CELL_SIZE - 1, _ => ATM_MAX_AAL5_PDU };
    if (*tp).max_sdu == 0 { (*tp).max_sdu = max_sdu; } else if (*tp).max_sdu > max_sdu { return -EINVAL; }
    if (*tp).max_cdv == 0 { (*tp).max_cdv = ATM_MAX_CDV; } 0
}

unsafe fn check_ci(vcc: *const atm_vcc, vpi: c_short, vci: c_int) -> c_int {
    let head = &vcc_hash[(vci as usize) & (VCC_HTABLE_SIZE - 1)]; let mut s = head.first;
    while !s.is_null() { let walk = atm_sk(s); if (*walk).dev == (*vcc).dev && test_bit(ATM_VF_ADDR, &(*walk).flags) && (*walk).vpi == vpi && (*walk).vci == vci && (((*walk).qos.txtp.traffic_class != ATM_NONE && (*vcc).qos.txtp.traffic_class != ATM_NONE) || ((*walk).qos.rxtp.traffic_class != ATM_NONE && (*vcc).qos.rxtp.traffic_class != ATM_NONE)) { return -EADDRINUSE; } s = (*s).next; } 0
}

unsafe fn find_ci(vcc: *const atm_vcc, vpi: *mut c_short, vci: *mut c_int) -> c_int {
    static mut p: c_short = 0; static mut c: c_int = 0;
    if *vpi != ATM_VPI_ANY && *vci != ATM_VCI_ANY { return check_ci(vcc, *vpi, *vci); }
    if *vpi != ATM_VPI_ANY { p = *vpi; } else if p >= 1 << (*(*vcc).dev).ci_range.vpi_bits { p = 0; }
    if *vci != ATM_VCI_ANY { c = *vci; } else if c < ATM_NOT_RSV_VCI || c >= 1 << (*(*vcc).dev).ci_range.vci_bits { c = ATM_NOT_RSV_VCI; }
    let old_p = p; let old_c = c;
    loop { if check_ci(vcc, p, c) == 0 { *vpi = p; *vci = c; return 0; } if *vci == ATM_VCI_ANY { c += 1; if c >= 1 << (*(*vcc).dev).ci_range.vci_bits { c = ATM_NOT_RSV_VCI; } } if (c == ATM_NOT_RSV_VCI || *vci != ATM_VCI_ANY) && *vpi == ATM_VPI_ANY { p += 1; if p >= 1 << (*(*vcc).dev).ci_range.vpi_bits { p = 0; } } if old_p == p && old_c == c { break; } } -EADDRINUSE
}

// The remaining exported socket operations retain the C implementation's
// signatures and sequencing; kernel helpers and structures are external.
pub unsafe fn __vcc_connect(vcc: *mut atm_vcc, dev: *mut atm_dev, vpi: c_short, vci: c_int) -> c_int { let sk = sk_atm(vcc); if (vpi != ATM_VPI_UNSPEC && vpi != ATM_VPI_ANY && vpi >> (*dev).ci_range.vpi_bits != 0) || (vci != ATM_VCI_UNSPEC && vci != ATM_VCI_ANY && vci >> (*dev).ci_range.vci_bits != 0) { return -EINVAL; } if vci > 0 && vci < ATM_NOT_RSV_VCI && !capable(CAP_NET_BIND_SERVICE) { return -EPERM; } if !try_module_get((*dev).ops.owner) { return -ENODEV; } (*vcc).dev = dev; write_lock_irq(&mut vcc_sklist_lock); if test_bit(ATM_DF_REMOVED, &(*dev).flags) || find_ci(vcc, &mut (vpi as c_short), &mut (vci as c_int)) != 0 { write_unlock_irq(&mut vcc_sklist_lock); module_put((*dev).ops.owner); (*vcc).dev = core::ptr::null_mut(); return -EADDRINUSE; } (*vcc).vpi=vpi; (*vcc).vci=vci; __vcc_insert_socket(sk); write_unlock_irq(&mut vcc_sklist_lock); let mut error = match (*vcc).qos.aal { ATM_AAL0 => { (*vcc).stats=&mut (*dev).stats.aal0; atm_init_aal0(vcc) }, ATM_NO_AAL | ATM_AAL5 => { (*vcc).qos.aal=ATM_AAL5; (*vcc).stats=&mut (*dev).stats.aal5; atm_init_aal5(vcc) }, _ => -EPROTOTYPE }; if error==0 { error=adjust_tp(&mut (*vcc).qos.txtp, (*vcc).qos.aal); } if error==0 { error=adjust_tp(&mut (*vcc).qos.rxtp, (*vcc).qos.aal); } if error!=0 { vcc_remove_socket(sk); module_put((*dev).ops.owner); (*vcc).dev=core::ptr::null_mut(); return error; } if let Some(open)=(*dev).ops.open { error=open(vcc); } if error!=0 { vcc_remove_socket(sk); module_put((*dev).ops.owner); (*vcc).dev=core::ptr::null_mut(); } error }

pub unsafe fn vcc_connect(sock: *mut socket, itf: c_int, vpi: c_short, vci: c_int) -> c_int { let vcc=ATM_SD(sock); if (*sock).state==SS_CONNECTED{return -EISCONN;} if (*sock).state!=SS_UNCONNECTED || (vpi==0&&vci==0){return -EINVAL;} if vpi!=ATM_VPI_UNSPEC&&vci!=ATM_VCI_UNSPEC{clear_bit(ATM_VF_PARTIAL,&mut (*vcc).flags);} else if test_bit(ATM_VF_PARTIAL,&(*vcc).flags){return -EINVAL;} if !test_bit(ATM_VF_HASQOS,&(*vcc).flags){return -EBADFD;} if (*vcc).qos.txtp.traffic_class==ATM_ANYCLASS||(*vcc).qos.rxtp.traffic_class==ATM_ANYCLASS{return -EINVAL;} let dev=if itf!=ATM_ITF_ANY{try_then_request_module(atm_dev_lookup(itf),itf)}else{atm_first_device()}; if dev.is_null(){return -ENODEV;} let e=__vcc_connect(vcc,dev,vpi,vci); if e!=0{atm_dev_put(dev);return e;} if vpi==ATM_VPI_UNSPEC||vci==ATM_VCI_UNSPEC{set_bit(ATM_VF_PARTIAL,&mut (*vcc).flags);} if test_bit(ATM_VF_READY,&(*vcc).flags){(*sock).state=SS_CONNECTED;} 0 }

pub unsafe fn vcc_recvmsg(sock:*mut socket,msg:*mut msghdr,size:usize,flags:c_int)->isize{if (*sock).state!=SS_CONNECTED{return -ENOTCONN as isize;} if flags&!(MSG_DONTWAIT|MSG_PEEK)!=0{return -EOPNOTSUPP as isize;} let vcc=ATM_SD(sock);if test_bit(ATM_VF_CLOSE,&(*vcc).flags)||!test_bit(ATM_VF_READY,&(*vcc).flags){return 0;}let mut e=-EINVAL;let skb=skb_recv_datagram((*sock).sk,flags,&mut e);if skb.is_null(){return e as isize;}let n=(*skb).len.min(size);if n<(*skb).len{(*msg).msg_flags|=MSG_TRUNC;}if skb_copy_datagram_msg(skb,0,msg,n)!=0{return -EFAULT as isize;}sock_recv_cmsgs(msg,(*sock).sk,skb);if flags&MSG_PEEK==0{atm_return(vcc,(*skb).truesize);}skb_free_datagram((*sock).sk,skb);n as isize}
pub unsafe fn vcc_sendmsg(sock:*mut socket,m:*mut msghdr,size:usize)->isize{let sk=(*sock).sk;lock_sock(sk);if (*sock).state!=SS_CONNECTED{release_sock(sk);return -ENOTCONN as isize;}let vcc=ATM_SD(sock);if test_bit(ATM_VF_CLOSE,&(*vcc).flags)||!test_bit(ATM_VF_READY,&(*vcc).flags){release_sock(sk);return -EPIPE as isize;}if size==0{release_sock(sk);return 0;}if size>(*vcc).qos.txtp.max_sdu as usize{release_sock(sk);return -EMSGSIZE as isize;}let eff=(size+3)&!3;while !vcc_tx_ready(vcc,eff as c_uint){if (*m).msg_flags&MSG_DONTWAIT!=0{release_sock(sk);return -EAGAIN as isize;}schedule();}let skb=alloc_skb(eff,GFP_KERNEL);if skb.is_null(){release_sock(sk);return -ENOMEM as isize;}atm_account_tx(vcc,skb);if !copy_from_iter_full(skb_put(skb,size),size,&mut (*m).msg_iter){atm_return_tx(vcc,skb);kfree_skb(skb);release_sock(sk);return -EFAULT as isize;}if eff!=size{memset((*skb).data.add(size),0,eff-size);}let e=(*(*vcc).dev).ops.send.unwrap()(vcc,skb);release_sock(sk);if e!=0{e as isize}else{size as isize}}
pub unsafe fn vcc_poll(file:*mut file,sock:*mut socket,wait:*mut poll_table)->__poll_t{sock_poll_wait(file,sock,wait);let sk=(*sock).sk;let vcc=ATM_SD(sock);let mut mask=if (*sk).sk_err!=0{EPOLLERR}else{0};if test_bit(ATM_VF_CLOSE,&(*vcc).flags){mask|=EPOLLHUP;}if !skb_queue_empty_lockless(&(*sk).sk_receive_queue){mask|=EPOLLIN|EPOLLRDNORM;}if (*vcc).qos.txtp.traffic_class!=ATM_NONE&&vcc_writable(sk){mask|=EPOLLOUT|EPOLLWRNORM|EPOLLWRBAND;}mask}

unsafe fn check_tp(tp:*const atm_trafprm)->c_int{if (*tp).traffic_class>ATM_ANYCLASS{return -EINVAL;}if (*tp).traffic_class==0||(*tp).traffic_class==ATM_ANYCLASS{return 0;}if (*tp).traffic_class!=ATM_UBR&&(*tp).min_pcr==0&&(*tp).pcr==0&&(*tp).max_pcr==0{return -EINVAL;}if (*tp).min_pcr==ATM_MAX_PCR{return -EINVAL;}if (*tp).min_pcr!=0&&(*tp).max_pcr!=0&&(*tp).max_pcr!=ATM_MAX_PCR&&(*tp).min_pcr>(*tp).max_pcr{return -EINVAL;}0}
unsafe fn check_qos(q:*const atm_qos)->c_int{if (*q).txtp.traffic_class==0&&(*q).rxtp.traffic_class==0{return -EINVAL;}if (*q).txtp.traffic_class!=(*q).rxtp.traffic_class&&(*q).txtp.traffic_class!=0&&(*q).rxtp.traffic_class!=0&&(*q).txtp.traffic_class!=ATM_ANYCLASS&&(*q).rxtp.traffic_class!=ATM_ANYCLASS{return -EINVAL;}let e=check_tp(&(*q).txtp);if e!=0{e}else{check_tp(&(*q).rxtp)}}

pub unsafe fn vcc_setsockopt(sock:*mut socket,_level:c_int,optname:c_int,optval:sockptr_t,optlen:c_uint)->c_int{let vcc=ATM_SD(sock);match optname{SO_ATMQOS=>{let mut q=core::mem::zeroed::<atm_qos>();if copy_safe_from_sockptr(&mut q as*mut _ as*mut c_void,core::mem::size_of::<atm_qos>(),optval,optlen)!=0{return -EFAULT;}let e=check_qos(&q);if e!=0{return e;}if (*sock).state!=SS_UNCONNECTED{return -EOPNOTSUPP;}(*vcc).qos=q;set_bit(ATM_VF_HASQOS,&mut (*vcc).flags);0},SO_SETCLP=>{let mut v=0;let e=copy_safe_from_sockptr(&mut v as*mut _ as*mut c_void,core::mem::size_of::<c_int>(),optval,optlen);if e!=0{return e;}if v!=0{(*vcc).atm_options|=ATM_ATMOPT_CLP;}else{(*vcc).atm_options&=!ATM_ATMOPT_CLP;}0},_=>-EINVAL}}
pub unsafe fn vcc_getsockopt(sock:*mut socket,_level:c_int,optname:c_int,opt:*mut sockopt_t)->c_int{let vcc=ATM_SD(sock);match optname{SO_ATMQOS=>{if !test_bit(ATM_VF_HASQOS,&(*vcc).flags){-EINVAL}else{copy_to_iter(&(*vcc).qos as*const _ as*const c_void,core::mem::size_of::<atm_qos>(),&mut (*opt).iter_out) as c_int}},SO_SETCLP=>{let v=if (*vcc).atm_options&ATM_ATMOPT_CLP!=0{1}else{0};copy_to_iter(&v as*const _ as*const c_void,core::mem::size_of::<c_int>(),&mut (*opt).iter_out) as c_int},SO_ATMPVC=>-ENOTCONN,_=>-EINVAL}}
pub unsafe fn register_atmdevice_notifier(nb:*mut notifier_block)->c_int{atomic_notifier_chain_register(&mut atm_dev_notify_chain,nb)}
pub unsafe fn unregister_atmdevice_notifier(nb:*mut notifier_block){atomic_notifier_chain_unregister(&mut atm_dev_notify_chain,nb);}

extern "C" { fn atmpvc_init() -> c_int; fn atmpvc_exit(); fn atm_proc_init() -> c_int; fn atm_proc_exit(); fn atm_sysfs_init() -> c_int; fn atm_sysfs_exit(); fn proto_register(p: *mut proto, m: c_int) -> c_int; fn proto_unregister(p: *mut proto); }

unsafe extern "C" fn atm_init() -> c_int { let mut e=proto_register(&mut vcc_proto,0); if e<0{return e;} e=atmpvc_init(); if e<0{proto_unregister(&mut vcc_proto);return e;} e=atm_proc_init(); if e<0{atmpvc_exit();proto_unregister(&mut vcc_proto);return e;} e=atm_sysfs_init(); if e<0{atm_proc_exit();atmpvc_exit();proto_unregister(&mut vcc_proto);return e;} e }
unsafe extern "C" fn atm_exit() { atm_proc_exit(); atm_sysfs_exit(); atmpvc_exit(); proto_unregister(&mut vcc_proto); }

// subsys_initcall(atm_init); module_exit(atm_exit);
// MODULE_DESCRIPTION("Asynchronous Transfer Mode (ATM) networking core");
// MODULE_LICENSE("GPL"); MODULE_ALIAS_NETPROTO(PF_ATMPVC);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
