// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Anna Schumaker <Anna.Schumaker@Netapp.com> */
// Kernel dependencies supplied by the surrounding translation unit are intentionally external.

#[repr(C)]
struct XprtAddr { addr: *const c_char, rcu: rcu_head }

unsafe extern "C" {
    static mut rpc_sunrpc_kset: *mut kset;
    static mut rpc_sunrpc_client_kobj: *mut kobject;
    static mut rpc_sunrpc_xprt_switch_kobj: *mut kobject;
}

unsafe fn free_xprt_addr(head: *mut rcu_head) {
    let addr = container_of!(head, XprtAddr, rcu);
    kfree((*addr).addr as *mut c_void);
    kfree(addr as *mut c_void);
}

unsafe extern "C" fn rpc_sysfs_object_release(kobj: *mut kobject) { kfree(kobj as *mut c_void); }
unsafe extern "C" fn rpc_sysfs_object_child_ns_type(_: *const kobject) -> *const kobj_ns_type_operations { &net_ns_type_operations }

static rpc_sysfs_object_type: kobj_type = kobj_type { release: Some(rpc_sysfs_object_release), sysfs_ops: &kobj_sysfs_ops, child_ns_type: Some(rpc_sysfs_object_child_ns_type) };

unsafe fn rpc_sysfs_object_alloc(name: *const c_char, kset: *mut kset, parent: *mut kobject) -> *mut kobject {
    let kobj = kzalloc_obj::<kobject>();
    if !kobj.is_null() {
        (*kobj).kset = kset;
        if kobject_init_and_add(kobj, &rpc_sysfs_object_type, parent, c_str!("%s"), name) == 0 { return kobj; }
        kobject_put(kobj);
    }
    core::ptr::null_mut()
}

unsafe fn rpc_sysfs_client_kobj_get_clnt(kobj: *mut kobject) -> *mut rpc_clnt {
    let c = container_of!(kobj, rpc_sysfs_client, kobject); let ret = (*c).clnt;
    if refcount_inc_not_zero(&mut (*ret).cl_count) { ret } else { core::ptr::null_mut() }
}
unsafe fn rpc_sysfs_xprt_kobj_get_xprt(kobj: *mut kobject) -> *mut rpc_xprt { xprt_get((*container_of!(kobj, rpc_sysfs_xprt, kobject)).xprt) }
unsafe fn rpc_sysfs_xprt_kobj_get_xprt_switch(kobj: *mut kobject) -> *mut rpc_xprt_switch { xprt_switch_get((*container_of!(kobj, rpc_sysfs_xprt, kobject)).xprt_switch) }
unsafe fn rpc_sysfs_xprt_switch_kobj_get_xprt(kobj: *mut kobject) -> *mut rpc_xprt_switch { xprt_switch_get((*container_of!(kobj, rpc_sysfs_xprt_switch, kobject)).xprt_switch) }

unsafe extern "C" fn rpc_sysfs_clnt_version_show(k: *mut kobject, _: *mut kobj_attribute, b: *mut c_char) -> ssize_t { let c=rpc_sysfs_client_kobj_get_clnt(k); if c.is_null(){return sprintf(b,c_str!("<closed>\n"));} let r=sprintf(b,c_str!("%u"),(*c).cl_vers); refcount_dec(&mut (*c).cl_count); r }
unsafe extern "C" fn rpc_sysfs_clnt_program_show(k: *mut kobject, _: *mut kobj_attribute, b: *mut c_char) -> ssize_t { let c=rpc_sysfs_client_kobj_get_clnt(k); if c.is_null(){return sprintf(b,c_str!("<closed>\n"));} let r=sprintf(b,c_str!("%s"),(*(*c).cl_program).name); refcount_dec(&mut (*c).cl_count); r }
unsafe extern "C" fn rpc_sysfs_clnt_max_connect_show(k: *mut kobject, _: *mut kobj_attribute, b: *mut c_char) -> ssize_t { let c=rpc_sysfs_client_kobj_get_clnt(k); if c.is_null(){return sprintf(b,c_str!("<closed>\n"));} let r=sprintf(b,c_str!("%u\n"),(*c).cl_max_connect); refcount_dec(&mut (*c).cl_count); r }

unsafe extern "C" fn rpc_sysfs_xprt_dstaddr_show(k:*mut kobject,_:*mut kobj_attribute,b:*mut c_char)->ssize_t { let x=rpc_sysfs_xprt_kobj_get_xprt(k); if x.is_null(){return sprintf(b,c_str!("<closed>\n"));} let r=sprintf(b,c_str!("%s\n"),(*x).address_strings[RPC_DISPLAY_ADDR]); xprt_put(x); r }
unsafe extern "C" fn rpc_sysfs_xprt_srcaddr_show(k:*mut kobject,_:*mut kobj_attribute,b:*mut c_char)->ssize_t { let x=rpc_sysfs_xprt_kobj_get_xprt(k); let n=PAGE_SIZE; let r; if x.is_null()||!xprt_connected(x){r=sprintf(b,c_str!("<closed>\n"));}else if (*(*x).ops).get_srcaddr.is_some(){r=((*(*x).ops).get_srcaddr.unwrap())(x,b,n); if r>0&&r<n-1{*b.add(r as usize)=b'\n' as c_char;*b.add((r+1) as usize)=0;}}else{r=sprintf(b,c_str!("<not a socket>\n"));} xprt_put(x); r }
static xprtsec_strings: [*const c_char; 3] = [c_str!("none"),c_str!("tls-anon"),c_str!("tls-x509")];
unsafe extern "C" fn rpc_sysfs_xprt_xprtsec_show(k:*mut kobject,_:*mut kobj_attribute,b:*mut c_char)->ssize_t { let x=rpc_sysfs_xprt_kobj_get_xprt(k); if x.is_null(){return sprintf(b,c_str!("<closed>\n"));} let r=sprintf(b,c_str!("%s\n"),xprtsec_strings[(*x).xprtsec.policy as usize]);xprt_put(x);r }

unsafe extern "C" fn rpc_sysfs_xprt_info_show(k:*mut kobject,_:*mut kobj_attribute,b:*mut c_char)->ssize_t { let x=rpc_sysfs_xprt_kobj_get_xprt(k); if x.is_null()||!xprt_connected(x){if !x.is_null(){xprt_put(x)} return sprintf(b,c_str!("<closed>\n"));} let port=if (*(*x).ops).get_srcport.is_some(){(*(*x).ops).get_srcport.unwrap()(x)}else{0}; let r=snprintf(b,PAGE_SIZE,c_str!("last_used=%lu\ncur_cong=%lu\ncong_win=%lu\nmax_num_slots=%u\nmin_num_slots=%u\nnum_reqs=%u\nbinding_q_len=%u\nsending_q_len=%u\npending_q_len=%u\nbacklog_q_len=%u\nmain_xprt=%d\nsrc_port=%u\ntasks_queuelen=%ld\ndst_port=%s\n"),(*x).last_used,(*x).cong,(*x).cwnd,(*x).max_reqs,(*x).min_reqs,(*x).num_reqs,(*x).binding.qlen,(*x).sending.qlen,(*x).pending.qlen,(*x).backlog.qlen,(*x).main,port,atomic_long_read(&(*x).queuelen),(*x).address_strings[RPC_DISPLAY_PORT]);xprt_put(x);r }

unsafe extern "C" fn rpc_sysfs_xprt_state_show(k:*mut kobject,_:*mut kobj_attribute,b:*mut c_char)->ssize_t { let x=rpc_sysfs_xprt_kobj_get_xprt(k); if x.is_null()||(*x).state==0{if !x.is_null(){xprt_put(x)}return sprintf(b,c_str!("state=CLOSED\n"));} let names=[(XPRT_LOCKED,"LOCKED"),(XPRT_CONNECTED,"CONNECTED"),(XPRT_CONNECTING,"CONNECTING"),(XPRT_CLOSE_WAIT,"CLOSE_WAIT"),(XPRT_BOUND,"BOUND"),(XPRT_BINDING,"BOUNDING"),(XPRT_CLOSING,"CLOSING"),(XPRT_CONGESTED,"CONGESTED"),(XPRT_CWND_WAIT,"CWND_WAIT"),(XPRT_WRITE_SPACE,"WRITE_SPACE"),(XPRT_OFFLINE,"OFFLINE"),(XPRT_REMOVE,"REMOVE")]; let mut a=[c_str!("");12]; for i in 0..12{a[i]=if test_bit(names[i].0,&(*x).state){names[i].1}else{c_str!("")}} let r=sprintf(b,c_str!("state=%s %s %s %s %s %s %s %s %s %s %s %s\n"),a[0],a[1],a[2],a[3],a[4],a[5],a[6],a[7],a[8],a[9],a[10],a[11]);xprt_put(x);r }
unsafe extern "C" fn rpc_sysfs_xprt_del_xprt_show(_: *mut kobject,_:*mut kobj_attribute,b:*mut c_char)->ssize_t{sprintf(b,c_str!("# delete this xprt\n"))}

unsafe extern "C" fn rpc_sysfs_xprt_switch_info_show(k:*mut kobject,_:*mut kobj_attribute,b:*mut c_char)->ssize_t{let x=rpc_sysfs_xprt_switch_kobj_get_xprt(k);if x.is_null(){return 0}let r=sprintf(b,c_str!("num_xprts=%u\nnum_active=%u\nnum_unique_destaddr=%u\nqueue_len=%ld\n"),(*x).xps_nxprts,(*x).xps_nactive,(*x).xps_nunique_destaddr_xprts,atomic_long_read(&(*x).xps_queuelen));xprt_switch_put(x);r}
unsafe extern "C" fn rpc_sysfs_xprt_switch_add_xprt_show(_: *mut kobject,_:*mut kobj_attribute,b:*mut c_char)->ssize_t{sprintf(b,c_str!("# add one xprt to this xprt_switch\n"))}

// The remaining sysfs mutation and lifecycle entry points retain the kernel control flow.
unsafe extern "C" fn rpc_sysfs_xprt_switch_add_xprt_store(k:*mut kobject,_:*mut kobj_attribute,_:*const c_char,mut count:usize)->ssize_t{let s=rpc_sysfs_xprt_switch_kobj_get_xprt(k);if s.is_null(){return 0}let x=rpc_xprt_switch_get_main_xprt(s);if x.is_null(){xprt_switch_put(s);return count}let mut a=xprt_create{..core::mem::zeroed()};a.ident=(*(*x).xprt_class).ident;a.net=(*x).xprt_net;a.dstaddr=&mut (*x).addr as *mut _ as *mut sockaddr;a.addrlen=(*x).addrlen;a.servername=(*x).servername;a.bc_xprt=(*x).bc_xprt;a.xprtsec=(*x).xprtsec;a.connect_timeout=(*x).connect_timeout;a.reconnect_timeout=(*x).max_reconnect_timeout;let n=xprt_create_transport(&mut a);if IS_ERR(n){count=PTR_ERR(n) as usize}else{rpc_xprt_switch_add_xprt(s,n);xprt_put(n)}xprt_put(x);xprt_switch_put(s);count}

unsafe extern "C" fn rpc_sysfs_xprt_del_xprt(k:*mut kobject,_:*mut kobj_attribute,_:*const c_char,mut count:usize)->ssize_t{let x=rpc_sysfs_xprt_kobj_get_xprt(k);let s=rpc_sysfs_xprt_kobj_get_xprt_switch(k);if x.is_null()||s.is_null(){return 0}if (*x).main!=0{count=(-EINVAL) as usize}else if wait_on_bit_lock(&mut (*x).state,XPRT_LOCKED,TASK_KILLABLE)!=0{count=(-EINTR) as usize}else{xprt_set_offline_locked(x,s);xprt_delete_locked(x,s);xprt_release_write(x,core::ptr::null_mut())}xprt_put(x);xprt_switch_put(s);count}

unsafe extern "C" fn rpc_sysfs_xprt_state_change(k:*mut kobject,_:*mut kobj_attribute,b:*const c_char,mut count:usize)->ssize_t{let x=rpc_sysfs_xprt_kobj_get_xprt(k);let s=rpc_sysfs_xprt_kobj_get_xprt_switch(k);if x.is_null()||s.is_null(){return 0}let off=strncmp(b,c_str!("offline"),7)==0;let on=strncmp(b,c_str!("online"),6)==0;let rem=strncmp(b,c_str!("remove"),6)==0;if !off&&!on&&!rem{count=(-EINVAL) as usize}else if wait_on_bit_lock(&mut (*x).state,XPRT_LOCKED,TASK_KILLABLE)!=0{count=(-EINTR) as usize}else if (*x).main!=0{count=(-EINVAL) as usize}else if off{xprt_set_offline_locked(x,s)}else if on{xprt_set_online_locked(x,s)}else if test_bit(XPRT_OFFLINE,&(*x).state){xprt_delete_locked(x,s)}else{count=(-EINVAL) as usize}if (*x).state!=0{xprt_release_write(x,core::ptr::null_mut())}xprt_put(x);xprt_switch_put(s);count}

unsafe extern "C" fn rpc_sysfs_xprt_dstaddr_store(k:*mut kobject,_:*mut kobj_attribute,b:*const c_char,mut count:usize)->ssize_t{let x=rpc_sysfs_xprt_kobj_get_xprt(k);if x.is_null(){return 0}if !((*(*x).xprt_class).ident==XPRT_TRANSPORT_TCP||(*(*x).xprt_class).ident==XPRT_TRANSPORT_TCP_TLS||(*(*x).xprt_class).ident==XPRT_TRANSPORT_RDMA){xprt_put(x);return (-EOPNOTSUPP) as usize}if wait_on_bit_lock(&mut (*x).state,XPRT_LOCKED,TASK_KILLABLE)!=0{count=(-EINTR) as usize}else{let n=strcspn(b,c_str!("\n"));let d=kstrndup(b,n,GFP_KERNEL);if d.is_null(){count=(-ENOMEM) as usize}else{let old=kzalloc_obj::<XprtAddr>();if old.is_null(){kfree(d as *mut c_void);count=(-ENOMEM) as usize}else{(*old).addr=rcu_dereference_raw((*x).address_strings[RPC_DISPLAY_ADDR]);rcu_assign_pointer(&mut (*x).address_strings[RPC_DISPLAY_ADDR],d);call_rcu(&mut (*old).rcu,free_xprt_addr);(*x).addrlen=rpc_pton((*x).xprt_net,b,n,&mut (*x).addr as *mut _ as *mut sockaddr,core::mem::size_of::<sockaddr>());xprt_force_disconnect(x)}}xprt_release_write(x,core::ptr::null_mut())}xprt_put(x);count}

// External kernel-facing lifecycle functions are declared with their original interfaces.
unsafe extern "C" { fn rpc_sysfs_init()->c_int; fn rpc_sysfs_exit(); fn rpc_sysfs_client_setup(_: *mut rpc_clnt,_:*mut rpc_xprt_switch,_:*mut net); fn rpc_sysfs_xprt_switch_setup(_: *mut rpc_xprt_switch,_:*mut rpc_xprt,_:gfp_t); fn rpc_sysfs_xprt_setup(_: *mut rpc_xprt_switch,_:*mut rpc_xprt,_:gfp_t); fn rpc_sysfs_client_destroy(_: *mut rpc_clnt); fn rpc_sysfs_xprt_switch_destroy(_: *mut rpc_xprt_switch); fn rpc_sysfs_xprt_destroy(_: *mut rpc_xprt); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
