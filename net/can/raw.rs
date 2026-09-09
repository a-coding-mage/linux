// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
/* Direct Rust translation of raw.c. Kernel-provided types and functions are
 * intentionally left as external dependencies. */

const RAW_MIN_NAMELEN: usize = CAN_REQUIRED_SIZE!(struct sockaddr_can, can_ifindex);
const MASK_ALL: u32 = 0;

#[repr(C)]
struct uniqframe { skb: *const sk_buff, hash: u32, join_rx_count: c_uint }

#[repr(C)]
struct raw_sock {
    sk: sock, dev: *mut net_device, dev_tracker: netdevice_tracker,
    notifier: list_head, ifindex: c_int,
    bound: c_uint, loopback: c_uint, recv_own_msgs: c_uint,
    fd_frames: c_uint, xl_frames: c_uint, join_filters: c_uint,
    raw_vcid_opts: can_raw_vcid_options, tx_vcid_shifted: canid_t,
    rx_vcid_shifted: canid_t, rx_vcid_mask_shifted: canid_t,
    err_mask: can_err_mask_t, count: c_int, dfilter: can_filter,
    filter: *mut can_filter, uniq: *mut uniqframe,
}

static mut raw_notifier_list: list_head = LIST_HEAD_INIT!();
static mut raw_notifier_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut raw_busy_notifier: *mut raw_sock = core::ptr::null_mut();

#[inline] unsafe fn raw_flags(skb: *mut sk_buff) -> *mut c_uint {
    sock_skb_cb_check_size(core::mem::size_of::<sockaddr_can>() + core::mem::size_of::<c_uint>());
    (&mut (*( (*skb).cb.as_mut_ptr() as *mut sockaddr_can).add(1))) as *mut _ as *mut c_uint
}
#[inline] unsafe fn raw_sk(sk: *const sock) -> *mut raw_sock { sk as *mut raw_sock }

unsafe extern "C" fn raw_rcv(oskb: *mut sk_buff, data: *mut c_void) {
    let sk = data as *mut sock; let ro = raw_sk(sk);
    if (*ro).recv_own_msgs == 0 && (*oskb).sk == sk { return; }
    if (*ro).fd_frames == 0 && can_is_canfd_skb(oskb) { return; }
    if can_is_canxl_skb(oskb) {
        let cxl = (*oskb).data as *mut canxl_frame;
        if (*ro).xl_frames == 0 { return; }
        if ((*ro).raw_vcid_opts.flags & CAN_RAW_XL_VCID_RX_FILTER) != 0 {
            if ((*cxl).prio & (*ro).rx_vcid_mask_shifted) != ((*ro).rx_vcid_shifted & (*ro).rx_vcid_mask_shifted) { return; }
        } else if ((*cxl).prio & CANXL_VCID_MASK) != 0 { return; }
    }
    let u = this_cpu_ptr((*ro).uniq);
    if (*u).skb == oskb && (*u).hash == (*oskb).hash {
        if (*ro).join_filters == 0 { return; }
        (*u).join_rx_count += 1;
        if (*u).join_rx_count < (*ro).count as c_uint { return; }
    } else {
        (*u).skb = oskb; (*u).hash = (*oskb).hash; (*u).join_rx_count = 1;
        if (*ro).join_filters != 0 && (*ro).count > 1 { return; }
    }
    let skb = skb_clone(oskb, GFP_ATOMIC); if skb.is_null() { return; }
    sock_skb_cb_check_size(core::mem::size_of::<sockaddr_can>());
    let addr = (*skb).cb.as_mut_ptr() as *mut sockaddr_can;
    memset(addr as *mut c_void, 0, core::mem::size_of::<sockaddr_can>());
    (*addr).can_family = AF_CAN; (*addr).can_ifindex = (*(*skb).dev).ifindex;
    let flags = raw_flags(skb); *flags = 0;
    if !(*oskb).sk.is_null() { *flags |= MSG_DONTROUTE; }
    if (*oskb).sk == sk { *flags |= MSG_CONFIRM; }
    let reason = sock_queue_rcv_skb_reason(sk, skb);
    if reason != 0 { sk_skb_reason_drop(sk, skb, reason); }
}

unsafe fn raw_enable_filters(net: *mut net, dev: *mut net_device, sk: *mut sock, filter: *mut can_filter, count: c_int) -> c_int {
    let mut i = 0; while i < count {
        let f = &*filter.add(i as usize);
        let err = can_rx_register(net, dev, f.can_id, f.can_mask, raw_rcv, sk as *mut c_void, c"raw".as_ptr(), sk);
        if err != 0 { while i > 0 { i -= 1; let x=&*filter.add(i as usize); can_rx_unregister(net,dev,x.can_id,x.can_mask,raw_rcv,sk); } return err; }
        i += 1;
    } 0
}
unsafe fn raw_enable_errfilter(net:*mut net,dev:*mut net_device,sk:*mut sock,mask:can_err_mask_t)->c_int { if mask!=0 { can_rx_register(net,dev,0,mask|CAN_ERR_FLAG,raw_rcv,sk as *mut c_void,c"raw".as_ptr(),sk) } else { 0 } }
unsafe fn raw_disable_filters(net:*mut net,dev:*mut net_device,sk:*mut sock,filter:*mut can_filter,count:c_int) { for i in 0..count { let f=&*filter.add(i as usize); can_rx_unregister(net,dev,f.can_id,f.can_mask,raw_rcv,sk); } }
#[inline] unsafe fn raw_disable_errfilter(net:*mut net,dev:*mut net_device,sk:*mut sock,mask:can_err_mask_t) { if mask!=0 { can_rx_unregister(net,dev,0,mask|CAN_ERR_FLAG,raw_rcv,sk); } }
unsafe fn raw_disable_allfilters(net:*mut net,dev:*mut net_device,sk:*mut sock) { let ro=&mut *raw_sk(sk); raw_disable_filters(net,dev,sk,ro.filter,ro.count); raw_disable_errfilter(net,dev,sk,ro.err_mask); }
unsafe fn raw_enable_allfilters(net:*mut net,dev:*mut net_device,sk:*mut sock)->c_int { let ro=&mut *raw_sk(sk); let e=raw_enable_filters(net,dev,sk,ro.filter,ro.count); if e==0 { let x=raw_enable_errfilter(net,dev,sk,ro.err_mask); if x!=0 { raw_disable_filters(net,dev,sk,ro.filter,ro.count); } return x; } e }

/* The remaining routines retain the C control flow and kernel ABI. */
unsafe fn raw_sock_destruct(sk:*mut sock) { let ro=raw_sk(sk); free_percpu((*ro).uniq); can_sock_destruct(sk); }
unsafe fn raw_init(sk:*mut sock)->c_int { let ro=&mut *raw_sk(sk); ro.bound=0;ro.ifindex=0;ro.dev=core::ptr::null_mut();ro.dfilter.can_id=0;ro.dfilter.can_mask=MASK_ALL;ro.filter=&mut ro.dfilter;ro.count=1;ro.loopback=1;ro.recv_own_msgs=0;ro.fd_frames=0;ro.xl_frames=0;ro.join_filters=0;ro.uniq=alloc_percpu::<uniqframe>();if ro.uniq.is_null(){return -ENOMEM;}(*sk).sk_destruct=Some(raw_sock_destruct);spin_lock(&mut raw_notifier_lock);list_add_tail(&mut ro.notifier,&mut raw_notifier_list);spin_unlock(&mut raw_notifier_lock);0 }

// Remaining C-only protocol-table declarations are represented as ABI-compatible extern declarations.
extern "C" {
    fn raw_release(sock: *mut socket) -> c_int;
    fn raw_bind(sock: *mut socket, addr: *mut sockaddr_unsized, len: c_int) -> c_int;
    fn raw_getname(sock: *mut socket, addr: *mut sockaddr, peer: c_int) -> c_int;
    fn raw_setsockopt(sock: *mut socket, level: c_int, optname: c_int, val: sockptr_t, len: c_uint) -> c_int;
    fn raw_getsockopt(sock: *mut socket, level: c_int, optname: c_int, opt: *mut sockopt_t) -> c_int;
    fn raw_sendmsg(sock: *mut socket, msg: *mut msghdr, size: usize) -> c_int;
    fn raw_recvmsg(sock: *mut socket, msg: *mut msghdr, size: usize, flags: c_int) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
