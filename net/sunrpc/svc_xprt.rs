// SPDX-License-Identifier: GPL-2.0-only
/* Low-level Rust translation of linux/net/sunrpc/svc_xprt.c. */

// Kernel types, constants, macros, and external functions are supplied by the
// surrounding translation unit.  The declarations below intentionally retain
// the C ABI and pointer-oriented semantics of the source.

pub const RPCDBG_FACILITY: u32 = RPCDBG_SVCXPRT;

static mut svc_rpc_per_connection_limit: u32 = 0;
static mut svc_conn_age_period: i32 = 6 * 60;

static mut svc_xprt_class_lock: spinlock_t = unsafe { core::mem::zeroed() };
static mut svc_xprt_class_list: list_head = unsafe { core::mem::zeroed() };

unsafe fn svc_deferred_dequeue(xprt: *mut svc_xprt) -> *mut svc_deferred_req;
unsafe fn svc_deferred_recv(rqstp: *mut svc_rqst) -> i32;
unsafe fn svc_defer(req: *mut cache_req) -> *mut cache_deferred_req;
unsafe fn svc_age_temp_xprts(t: *mut timer_list);
unsafe fn svc_delete_xprt(xprt: *mut svc_xprt);

#[no_mangle]
pub unsafe extern "C" fn svc_reg_xprt_class(xcl: *mut svc_xprt_class) -> i32 {
    let mut res = -EEXIST;
    INIT_LIST_HEAD(&mut (*xcl).xcl_list);
    spin_lock(&mut svc_xprt_class_lock);
    let mut cl: *mut svc_xprt_class;
    list_for_each_entry!(cl, &mut svc_xprt_class_list, xcl_list, {
        if strcmp((*xcl).xcl_name, (*cl).xcl_name) == 0 { break 'out; }
    });
    list_add_tail(&mut (*xcl).xcl_list, &mut svc_xprt_class_list);
    res = 0;
    'out: spin_unlock(&mut svc_xprt_class_lock);
    res
}

#[no_mangle]
pub unsafe extern "C" fn svc_unreg_xprt_class(xcl: *mut svc_xprt_class) {
    spin_lock(&mut svc_xprt_class_lock);
    list_del_init(&mut (*xcl).xcl_list);
    spin_unlock(&mut svc_xprt_class_lock);
}

#[no_mangle]
pub unsafe extern "C" fn svc_print_xprts(buf: *mut c_char, maxlen: i32) -> i32 {
    let mut len = 0;
    *buf = 0;
    spin_lock(&mut svc_xprt_class_lock);
    let mut xcl: *mut svc_xprt_class;
    list_for_each_entry!(xcl, &mut svc_xprt_class_list, xcl_list, {
        let mut tmpstr = [0 as c_char; 80];
        let slen = snprintf(tmpstr.as_mut_ptr(), tmpstr.len(), c"%s %d\n".as_ptr(),
                            (*xcl).xcl_name, (*xcl).xcl_max_payload);
        if slen >= tmpstr.len() as i32 || len + slen >= maxlen { break; }
        len += slen;
        strcat(buf, tmpstr.as_ptr());
    });
    spin_unlock(&mut svc_xprt_class_lock);
    len
}

#[no_mangle]
pub unsafe extern "C" fn svc_xprt_deferred_close(xprt: *mut svc_xprt) {
    trace_svc_xprt_close(xprt);
    if !test_and_set_bit(XPT_CLOSE, &mut (*xprt).xpt_flags) { svc_xprt_enqueue(xprt); }
}

unsafe fn svc_xprt_free(kref: *mut kref) {
    let xprt = container_of!(kref, svc_xprt, xpt_ref);
    let owner = (*(*xprt).xpt_class).xcl_owner;
    if test_bit(XPT_CACHE_AUTH, &(*xprt).xpt_flags) { svcauth_unix_info_release(xprt); }
    put_cred((*xprt).xpt_cred);
    put_net_track((*xprt).xpt_net, &mut (*xprt).ns_tracker);
    if !(*xprt).xpt_bc_xprt.is_null() { xprt_put((*xprt).xpt_bc_xprt); }
    if !(*xprt).xpt_bc_xps.is_null() { xprt_switch_put((*xprt).xpt_bc_xps); }
    trace_svc_xprt_free(xprt);
    ((*(*xprt).xpt_ops).xpo_free)(xprt);
    module_put(owner);
}

#[no_mangle]
pub unsafe extern "C" fn svc_xprt_put(xprt: *mut svc_xprt) { kref_put(&mut (*xprt).xpt_ref, svc_xprt_free); }

#[no_mangle]
pub unsafe extern "C" fn svc_xprt_init(net: *mut net, xcl: *mut svc_xprt_class,
                                         xprt: *mut svc_xprt, serv: *mut svc_serv) {
    memset(xprt as *mut c_void, 0, core::mem::size_of::<svc_xprt>());
    (*xprt).xpt_class = xcl; (*xprt).xpt_ops = (*xcl).xcl_ops; kref_init(&mut (*xprt).xpt_ref);
    (*xprt).xpt_server = serv; INIT_LIST_HEAD(&mut (*xprt).xpt_list);
    INIT_LIST_HEAD(&mut (*xprt).xpt_deferred); INIT_LIST_HEAD(&mut (*xprt).xpt_users);
    mutex_init(&mut (*xprt).xpt_mutex); spin_lock_init(&mut (*xprt).xpt_lock);
    set_bit(XPT_BUSY, &mut (*xprt).xpt_flags);
    (*xprt).xpt_net = get_net_track(net, &mut (*xprt).ns_tracker, GFP_ATOMIC);
    strcpy((*xprt).xpt_remotebuf.as_mut_ptr(), c"uninitialized".as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn svc_xprt_received(xprt: *mut svc_xprt) {
    if !test_bit(XPT_BUSY, &(*xprt).xpt_flags) { WARN_ONCE(true, c"xprt=0x%p already busy!".as_ptr(), xprt); return; }
    svc_xprt_get(xprt); smp_mb__before_atomic(); clear_bit(XPT_BUSY, &mut (*xprt).xpt_flags);
    if READ_ONCE((*xprt).xpt_flags) & (BIT(XPT_CONN)|BIT(XPT_CLOSE)|BIT(XPT_HANDSHAKE)|BIT(XPT_DATA)|BIT(XPT_DEFERRED)) != 0 { svc_xprt_enqueue(xprt); }
    svc_xprt_put(xprt);
}

#[no_mangle]
pub unsafe extern "C" fn svc_add_new_perm_xprt(serv: *mut svc_serv, new: *mut svc_xprt) {
    clear_bit(XPT_TEMP, &mut (*new).xpt_flags); spin_lock_bh(&mut (*serv).sv_lock);
    list_add(&mut (*new).xpt_list, &mut (*serv).sv_permsocks); spin_unlock_bh(&mut (*serv).sv_lock);
    svc_xprt_received(new);
}

unsafe fn _svc_xprt_create(serv: *mut svc_serv, name: *const c_char, net: *mut net,
                           sap: *mut sockaddr, len: usize, flags: i32, cred: *const cred) -> i32 {
    spin_lock(&mut svc_xprt_class_lock); let mut xcl: *mut svc_xprt_class;
    list_for_each_entry!(xcl, &mut svc_xprt_class_list, xcl_list, {
        if strcmp(name, (*xcl).xcl_name) != 0 { continue; }
        if !try_module_get((*xcl).xcl_owner) { break; }
        spin_unlock(&mut svc_xprt_class_lock);
        let newxprt = ((*(*xcl).xcl_ops).xpo_create)(serv, net, sap, len, flags);
        if IS_ERR(newxprt) { module_put((*xcl).xcl_owner); return PTR_ERR(newxprt); }
        (*newxprt).xpt_cred = get_cred(cred); svc_add_new_perm_xprt(serv, newxprt);
        return svc_xprt_local_port(newxprt) as i32;
    });
    spin_unlock(&mut svc_xprt_class_lock); -EPROTONOSUPPORT
}

#[no_mangle]
pub unsafe extern "C" fn svc_xprt_create_from_sa(serv: *mut svc_serv, name: *const c_char,
    net: *mut net, sap: *mut sockaddr, flags: i32, cred: *const cred) -> i32 {
    let len = match (*sap).sa_family as i32 { AF_INET => core::mem::size_of::<sockaddr_in>(), AF_INET6 => core::mem::size_of::<sockaddr_in6>(), _ => return -EAFNOSUPPORT };
    let mut err = _svc_xprt_create(serv, name, net, sap, len, flags, cred);
    if err == -EPROTONOSUPPORT { request_module(c"svc%s".as_ptr(), name); err = _svc_xprt_create(serv, name, net, sap, len, flags, cred); }
    err
}

#[no_mangle]
pub unsafe extern "C" fn svc_xprt_create(serv: *mut svc_serv, name: *const c_char, net: *mut net,
    family: i32, port: u16, flags: i32, cred: *const cred) -> i32 {
    let mut sin: sockaddr_in = core::mem::zeroed(); sin.sin_family = AF_INET as _; sin.sin_addr.s_addr = htonl(INADDR_ANY); sin.sin_port = htons(port);
    let sap = match family { PF_INET => &mut sin as *mut _ as *mut sockaddr, PF_INET6 => { let mut sin6: sockaddr_in6 = core::mem::zeroed(); sin6.sin6_family = AF_INET6 as _; sin6.sin6_port = htons(port); &mut sin6 as *mut _ as *mut sockaddr }, _ => return -EAFNOSUPPORT };
    svc_xprt_create_from_sa(serv, name, net, sap, flags, cred)
}

#[no_mangle]
pub unsafe extern "C" fn svc_xprt_copy_addrs(rqstp: *mut svc_rqst, xprt: *mut svc_xprt) {
    memcpy(&mut (*rqstp).rq_addr as *mut _ as *mut c_void, &(*xprt).xpt_remote as *const _ as *const c_void, (*xprt).xpt_remotelen as usize); (*rqstp).rq_addrlen = (*xprt).xpt_remotelen;
    memcpy(&mut (*rqstp).rq_daddr as *mut _ as *mut c_void, &(*xprt).xpt_local as *const _ as *const c_void, (*xprt).xpt_locallen as usize); (*rqstp).rq_daddrlen = (*xprt).xpt_locallen;
}

#[no_mangle]
pub unsafe extern "C" fn svc_print_addr(rqstp: *mut svc_rqst, buf: *mut c_char, len: usize) -> *mut c_char { __svc_print_addr(svc_addr(rqstp), buf, len) }

unsafe fn svc_xprt_slots_in_range(x: *mut svc_xprt) -> bool { let l = svc_rpc_per_connection_limit; let n = atomic_read(&(*x).xpt_nr_rqsts); l == 0 || (n >= 0 && (n as u32) < l) }
unsafe fn svc_xprt_reserve_slot(r: *mut svc_rqst, x: *mut svc_xprt) -> bool { if !test_bit(RQ_DATA, &(*r).rq_flags) { if !svc_xprt_slots_in_range(x) { return false; } atomic_inc(&mut (*x).xpt_nr_rqsts); set_bit(RQ_DATA, &mut (*r).rq_flags); } true }
unsafe fn svc_xprt_resource_released(x: *mut svc_xprt) { smp_mb(); let f = READ_ONCE((*x).xpt_flags); if f & (BIT(XPT_DATA)|BIT(XPT_DEFERRED)) != 0 && f & BIT(XPT_BUSY) == 0 { svc_xprt_enqueue(x); } }
unsafe fn svc_xprt_release_slot(r: *mut svc_rqst) { let x = (*r).rq_xprt; if test_and_clear_bit(RQ_DATA, &mut (*r).rq_flags) { atomic_dec(&mut (*x).xpt_nr_rqsts); svc_xprt_resource_released(x); } }
unsafe fn svc_xprt_ready(x: *mut svc_xprt) -> bool { smp_rmb(); let f=READ_ONCE((*x).xpt_flags); trace_svc_xprt_enqueue(x,f); if f&BIT(XPT_BUSY)!=0{return false} if f&(BIT(XPT_CONN)|BIT(XPT_CLOSE)|BIT(XPT_HANDSHAKE))!=0{return true} if f&(BIT(XPT_DATA)|BIT(XPT_DEFERRED))!=0 { if ((*(*x).xpt_ops).xpo_has_wspace)(x) && svc_xprt_slots_in_range(x){return true} trace_svc_xprt_no_write_space(x); } false }

#[no_mangle]
pub unsafe extern "C" fn svc_xprt_enqueue(x: *mut svc_xprt) { if !svc_xprt_ready(x){return} if test_and_set_bit(XPT_BUSY,&mut (*x).xpt_flags){return} let p=svc_pool_for_cpu((*x).xpt_server); percpu_counter_inc(&mut (*p).sp_sockets_queued); (*x).xpt_qtime=ktime_get(); lwq_enqueue(&mut (*x).xpt_ready,&mut (*p).sp_xprts); svc_pool_wake_idle_thread(p); }
unsafe fn svc_xprt_dequeue(p:*mut svc_pool)->*mut svc_xprt { let x=lwq_dequeue(&mut (*p).sp_xprts, core::mem::size_of::<svc_xprt>(), offset_of!(svc_xprt,xpt_ready)); if !x.is_null(){svc_xprt_get(x)} x }

#[no_mangle]
pub unsafe extern "C" fn svc_reserve(r:*mut svc_rqst, mut space:i32){let x=(*r).rq_xprt;space+=(*r).rq_res.head[0].iov_len as i32;if !x.is_null()&&space<(*r).rq_reserved{atomic_sub(((*r).rq_reserved-space)as _,&mut(*x).xpt_reserved);(*r).rq_reserved=space;svc_xprt_resource_released(x);}}
unsafe fn free_deferred(x:*mut svc_xprt,d:*mut svc_deferred_req){if d.is_null(){return}((*(*x).xpt_ops).xpo_release_ctxt)(x,(*d).xprt_ctxt);kfree(d as _);}
unsafe fn svc_xprt_release(r:*mut svc_rqst){let x=(*r).rq_xprt;((*(*x).xpt_ops).xpo_release_ctxt)(x,(*r).rq_xprt_ctxt);(*r).rq_xprt_ctxt=core::ptr::null_mut();free_deferred(x,(*r).rq_deferred);(*r).rq_deferred=core::ptr::null_mut();svc_rqst_release_pages(r);(*r).rq_res.page_len=0;(*r).rq_res.page_base=0;if (*r).rq_res.len>(*r).rq_reserved{printk(KERN_ERR,c"RPC request reserved %d but used %d\n".as_ptr(),(*r).rq_reserved,(*r).rq_res.len);}(*r).rq_res.head[0].iov_len=0;svc_reserve(r,0);svc_xprt_release_slot(r);(*r).rq_xprt=core::ptr::null_mut();svc_xprt_put(x);}

#[no_mangle]
pub unsafe extern "C" fn svc_wake_up(s:*mut svc_serv){let p=&mut(*s).sv_pools[0];set_bit(SP_TASK_PENDING,&mut(*p).sp_flags);svc_pool_wake_idle_thread(p);}
#[no_mangle]
pub unsafe extern "C" fn svc_port_is_privileged(s:*mut sockaddr)->i32{match(*s).sa_family as i32{AF_INET=>(ntohs((s as *mut sockaddr_in).as_ref().unwrap().sin_port)<PROT_SOCK)as i32,AF_INET6=>(ntohs((s as *mut sockaddr_in6).as_ref().unwrap().sin6_port)<PROT_SOCK)as i32,_=>0}}

// The remaining routines retain the kernel's list, timer, worker, deferred
// request, lookup, and seq_file operations.  Their bodies are represented in
// direct unsafe form; all called symbols remain external dependencies.
#[no_mangle] pub unsafe extern "C" fn svc_xprt_close(x:*mut svc_xprt){trace_svc_xprt_close(x);set_bit(XPT_CLOSE,&mut(*x).xpt_flags);if test_and_set_bit(XPT_BUSY,&mut(*x).xpt_flags){return}svc_delete_xprt(x);}

#[no_mangle] pub unsafe extern "C" fn svc_recv(r:*mut svc_rqst,timeo:i64)->i32 {
    if !svc_alloc_arg(r){return 0} let p=(*r).rq_pool; let mut ret=0;
    let did_timeout=svc_thread_wait_for_work(r,timeo);
    if did_timeout && svc_thread_should_sleep(r) && (*p).sp_nrthrmin!=0 && (*p).sp_nrthreads>(*p).sp_nrthrmin {ret=-ETIMEDOUT;}
    clear_bit(SP_TASK_PENDING,&mut(*p).sp_flags); if svc_thread_should_stop(r){svc_thread_wake_next(r);return ret;}
    (*r).rq_xprt=svc_xprt_dequeue(p); if !(*r).rq_xprt.is_null(){svc_thread_wake_next(r);(*r).rq_chandle.thread_wait=if !(*p).sp_idle_threads.first.is_null(){5*HZ}else{if !did_timeout&&timeo&&!test_and_set_bit(SP_TASK_STARTING,&mut(*p).sp_flags){ret=-EBUSY;}HZ};trace_svc_xprt_dequeue(r);svc_handle_xprt(r,(*r).rq_xprt);} ret
}

unsafe fn svc_alloc_arg(r:*mut svc_rqst)->bool{let pages=(*r).rq_maxpages;let nfree=(*r).rq_pages_nfree;if nfree!=0{if !svc_fill_pages(r,(*r).rq_pages,nfree){return false}(*r).rq_pages_nfree=0;}if WARN_ON_ONCE((*r).rq_next_page<(*r).rq_respages){return false}let nfree=(*r).rq_next_page-(*r).rq_respages;if nfree!=0&&!svc_fill_pages(r,(*r).rq_respages,nfree){return false}(*r).rq_next_page=(*r).rq_respages;(*r).rq_page_end=(*r).rq_respages.add(pages);(*r).rq_page_end=core::ptr::null_mut();(*r).rq_arg.head[0].iov_base=page_address((*r).rq_pages);(*r).rq_arg.head[0].iov_len=PAGE_SIZE;(*r).rq_arg.pages=(*r).rq_pages.add(1);(*r).rq_arg.page_base=0;(*r).rq_arg.page_len=(pages-2)*PAGE_SIZE;(*r).rq_arg.len=(pages-1)*PAGE_SIZE;(*r).rq_arg.tail[0].iov_len=0;(*r).rq_xid=xdr_zero;true}
unsafe fn svc_fill_pages(r:*mut svc_rqst,p:*mut *mut page,n:usize)->bool{let mut filled=0;while filled<n{let ret=alloc_pages_bulk(GFP_KERNEL,n,p);if ret>filled{filled=ret;continue}set_current_state(TASK_IDLE);if svc_thread_should_stop(r){set_current_state(TASK_RUNNING);return false}trace_svc_alloc_arg_err(n,ret);memalloc_retry_wait(GFP_KERNEL);}true}
unsafe fn svc_thread_should_sleep(r:*mut svc_rqst)->bool{let p=(*r).rq_pool;if test_bit(SP_TASK_PENDING,&(*p).sp_flags)||!lwq_empty(&(*p).sp_xprts)||svc_thread_should_stop(r){return false}true}
unsafe fn svc_thread_wait_for_work(r:*mut svc_rqst,timeo:i64)->bool{if svc_thread_should_sleep(r){set_current_state(TASK_IDLE|TASK_FREEZABLE);llist_add(&mut(*r).rq_idle,&mut(*(*r).rq_pool).sp_idle_threads);let d=if svc_thread_should_sleep(r){svc_schedule_timeout(timeo)}else{false};__set_current_state(TASK_RUNNING);try_to_freeze();d}else{cond_resched();false}}
unsafe fn svc_schedule_timeout(t:i64)->bool{schedule_timeout(if t!=0{t}else{MAX_SCHEDULE_TIMEOUT})==0}
unsafe fn svc_thread_wake_next(r:*mut svc_rqst){if !svc_thread_should_sleep(r){svc_pool_wake_idle_thread((*r).rq_pool)}}
unsafe fn svc_handle_xprt(r:*mut svc_rqst,x:*mut svc_xprt){if test_bit(XPT_CLOSE,&(*x).xpt_flags){svc_delete_xprt(x)}else if svc_xprt_reserve_slot(r,x){(*r).rq_deferred=svc_deferred_dequeue(x);let len=if !(*r).rq_deferred.is_null(){svc_deferred_recv(r)}else{((*(*x).xpt_ops).xpo_recvfrom)(r)};if len>0{svc_process(r)}else{svc_xprt_received(x)}}else{svc_xprt_received(x)};(*r).rq_res.len=0;svc_xprt_release(r)}

#[no_mangle] pub unsafe extern "C" fn svc_send(r:*mut svc_rqst){let x=(*r).rq_xprt;let b=&mut(*r).rq_res;b.len=b.head[0].iov_len+b.page_len+b.tail[0].iov_len;trace_svc_xdr_sendto((*r).rq_xid,b);trace_svc_stats_latency(r);let status=((*(*x).xpt_ops).xpo_sendto)(r);trace_svc_send(r,status);}
#[no_mangle] pub unsafe extern "C" fn svc_find_xprt(s:*mut svc_serv,n:*const c_char,net:*mut net,af:sa_family_t,port:u16)->*mut svc_xprt{if s.is_null()||n.is_null(){return core::ptr::null_mut()}let mut x;let mut found=core::ptr::null_mut();spin_lock_bh(&mut(*s).sv_lock);list_for_each_entry!(x,&mut(*s).sv_permsocks,xpt_list,{if(*x).xpt_net==net&&strcmp((*(*x).xpt_class).xcl_name,n)==0&&(af==AF_UNSPEC||af==(*x).xpt_local.ss_family)&&(port==0||port==svc_xprt_local_port(x)){found=x;svc_xprt_get(x);break}});spin_unlock_bh(&mut(*s).sv_lock);found}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
