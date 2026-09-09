// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2008-2013 Jozsef Kadlecsik <kadlec@netfilter.org> */
/* Kernel module implementing an IP set type: the list:set type */

// Linux kernel and ipset headers provide the external types, constants, macros,
// and functions referenced below.

const IPSET_TYPE_REV_MIN: u32 = 0;
const IPSET_TYPE_REV_MAX: u32 = 3;

#[repr(C)]
struct set_elem {
    rcu: rcu_head,
    list: list_head,
    set: *mut ip_set,
    id: ip_set_id_t,
}

#[repr(C)]
struct set_adt_elem { id: ip_set_id_t, refid: ip_set_id_t, before: i32 }

#[repr(C)]
struct list_set {
    size: u32,
    gc: timer_list,
    set: *mut ip_set,
    net: *mut net,
    members: list_head,
}

unsafe fn list_set_ktest(set: *mut ip_set, skb: *const sk_buff, par: *const xt_action_param,
    opt: *mut ip_set_adt_opt, ext: *const ip_set_ext) -> i32 {
    let map = (*set).data as *mut list_set;
    let mext = &mut (*opt).ext as *mut ip_set_ext;
    let flags = (*opt).cmdflags;
    (*opt).cmdflags &= !IPSET_FLAG_MATCH_COUNTERS;
    if (*opt).cmdflags & IPSET_FLAG_SKIP_SUBCOUNTER_UPDATE != 0 { (*opt).cmdflags |= IPSET_FLAG_SKIP_COUNTER_UPDATE; }
    let mut e = (*map).members.next as *mut set_elem;
    while e != &mut (*map).members as *mut list_head as *mut set_elem {
        let ret = ip_set_test((*e).id, skb, par, opt);
        if ret > 0 && ip_set_match_extensions(set, ext, mext, flags, e) { return 1; }
        e = (*e).list.next as *mut set_elem;
    }
    0
}

unsafe fn list_set_kadd(set: *mut ip_set, skb: *const sk_buff, par: *const xt_action_param,
    opt: *mut ip_set_adt_opt, ext: *const ip_set_ext) -> i32 {
    let map = (*set).data as *mut list_set; let mut e = (*map).members.next as *mut set_elem;
    while e != &mut (*map).members as *mut list_head as *mut set_elem {
        if SET_WITH_TIMEOUT(set) && ip_set_timeout_expired(ext_timeout(e, set)) { e = (*e).list.next as *mut set_elem; continue; }
        if ip_set_add((*e).id, skb, par, opt) == 0 { return 0; } e = (*e).list.next as *mut set_elem;
    } 0
}

unsafe fn list_set_kdel(set: *mut ip_set, skb: *const sk_buff, par: *const xt_action_param,
    opt: *mut ip_set_adt_opt, ext: *const ip_set_ext) -> i32 {
    let map = (*set).data as *mut list_set; let mut e = (*map).members.next as *mut set_elem;
    while e != &mut (*map).members as *mut list_head as *mut set_elem {
        if SET_WITH_TIMEOUT(set) && ip_set_timeout_expired(ext_timeout(e, set)) { e = (*e).list.next as *mut set_elem; continue; }
        if ip_set_del((*e).id, skb, par, opt) == 0 { return 0; } e = (*e).list.next as *mut set_elem;
    } 0
}

unsafe fn list_set_kadt(set: *mut ip_set, skb: *const sk_buff, par: *const xt_action_param,
    adt: ipset_adt, opt: *mut ip_set_adt_opt) -> i32 {
    let ext = IP_SET_INIT_KEXT(skb, opt, set); let mut ret = -EINVAL;
    rcu_read_lock();
    match adt { IPSET_TEST => ret = list_set_ktest(set, skb, par, opt, &ext), IPSET_ADD => ret = list_set_kadd(set, skb, par, opt, &ext), IPSET_DEL => ret = list_set_kdel(set, skb, par, opt, &ext), _ => {} }
    rcu_read_unlock(); ret
}

unsafe fn __list_set_del_rcu(rcu: *mut rcu_head) { let e = container_of!(rcu, set_elem, rcu); let set = (*e).set; ip_set_ext_destroy(set, e); kfree(e as *mut _); }
unsafe fn list_set_del(set: *mut ip_set, e: *mut set_elem) { let map=(*set).data as *mut list_set; (*set).elements-=1; list_del_rcu(&mut (*e).list); ip_set_put_byindex((*map).net,(*e).id); call_rcu(&mut (*e).rcu,__list_set_del_rcu); }
unsafe fn list_set_replace(set:*mut ip_set,e:*mut set_elem,old:*mut set_elem){let map=(*set).data as *mut list_set;list_replace_rcu(&mut (*old).list,&mut (*e).list);ip_set_put_byindex((*map).net,(*old).id);call_rcu(&mut (*old).rcu,__list_set_del_rcu);}

unsafe fn set_cleanup_entries(set:*mut ip_set){let map=(*set).data as *mut list_set;let mut e=(*map).members.next as *mut set_elem;while e != &mut (*map).members as *mut list_head as *mut set_elem {let n=(*e).list.next as *mut set_elem;if ip_set_timeout_expired(ext_timeout(e,set)){list_set_del(set,e)} e=n;}}

unsafe fn list_set_utest(set:*mut ip_set,value:*mut c_void,_ext:*const ip_set_ext,_mext:*mut ip_set_ext,_flags:u32)->i32{let map=(*set).data as *mut list_set;let d=value as *mut set_adt_elem;let mut prev:*mut set_elem=core::ptr::null_mut();let mut e=(*map).members.next as *mut set_elem;while e!=&mut (*map).members as *mut list_head as *mut set_elem {if SET_WITH_TIMEOUT(set)&&ip_set_timeout_expired(ext_timeout(e,set)){e=(*e).list.next as *mut set_elem;continue;}if (*e).id!=(*d).id{prev=e;e=(*e).list.next as *mut set_elem;continue;}if (*d).before==0{return 1}if (*d).before>0{let next=list_next_entry(e,list);return (!list_is_last(&(*e).list,&(*map).members)&&(*next).id==(*d).refid) as i32;}return (!prev.is_null()&&(*prev).id==(*d).refid) as i32;}0}

unsafe fn list_set_init_extensions(set:*mut ip_set,ext:*const ip_set_ext,e:*mut set_elem){if SET_WITH_COUNTER(set){ip_set_init_counter(ext_counter(e,set),ext)}if SET_WITH_COMMENT(set){ip_set_init_comment(set,ext_comment(e,set),ext)}if SET_WITH_SKBINFO(set){ip_set_init_skbinfo(ext_skbinfo(e,set),ext)}if SET_WITH_TIMEOUT(set){ip_set_timeout_set(ext_timeout(e,set),(*ext).timeout)}}

// The remaining userspace operations retain the source control flow and call
// the corresponding kernel/ipset primitives supplied by the surrounding build.
unsafe fn list_set_uadd(set:*mut ip_set,value:*mut c_void,ext:*const ip_set_ext,mext:*mut ip_set_ext,flags:u32)->i32{let map=(*set).data as *mut list_set;let d=value as *mut set_adt_elem;let mut e=(*map).members.next as *mut set_elem;let mut n:*mut set_elem=core::ptr::null_mut();let mut prev:*mut set_elem=core::ptr::null_mut();let mut next:*mut set_elem=core::ptr::null_mut();while e!=&mut (*map).members as *mut list_head as *mut set_elem{if SET_WITH_TIMEOUT(set)&&ip_set_timeout_expired(ext_timeout(e,set)){e=(*e).list.next as *mut set_elem;continue;}if (*d).id==(*e).id{n=e}else if (*d).before!=0&&(*e).id==(*d).refid{if (*d).before>0{next=e}else{prev=e}}e=(*e).list.next as *mut set_elem;}if ((*d).before>0&&next.is_null())||((*d).before<0&&prev.is_null()){return -IPSET_ERR_REF_EXIST}if !n.is_null(){if flags&IPSET_FLAG_EXIST==0{return -IPSET_ERR_EXIST}ip_set_ext_destroy(set,n);list_set_init_extensions(set,ext,n);ip_set_put_byindex((*map).net,(*d).id);return 0}let e=kzalloc((*set).dsize,GFP_ATOMIC) as *mut set_elem;if e.is_null(){return -ENOMEM}(*e).id=(*d).id;(*e).set=set;INIT_LIST_HEAD(&mut (*e).list);list_set_init_extensions(set,ext,e);if !next.is_null(){list_add_tail_rcu(&mut (*e).list,&mut (*next).list)}else if !prev.is_null(){list_add_rcu(&mut (*e).list,&mut (*prev).list)}else{list_add_tail_rcu(&mut (*e).list,&mut (*map).members)}(*set).elements+=1;0}

unsafe fn list_set_udel(set:*mut ip_set,value:*mut c_void,_ext:*const ip_set_ext,_mext:*mut ip_set_ext,_flags:u32)->i32{let map=(*set).data as *mut list_set;let d=value as *mut set_adt_elem;let mut prev:*mut set_elem=core::ptr::null_mut();let mut e=(*map).members.next as *mut set_elem;while e!=&mut (*map).members as *mut list_head as *mut set_elem{if SET_WITH_TIMEOUT(set)&&ip_set_timeout_expired(ext_timeout(e,set)){e=(*e).list.next as *mut set_elem;continue;}if (*e).id!=(*d).id{prev=e;e=(*e).list.next as *mut set_elem;continue;}if (*d).before>0{let next=list_next_entry(e,list);if list_is_last(&(*e).list,&(*map).members)||(*next).id!=(*d).refid{return -IPSET_ERR_REF_EXIST}}else if (*d).before<0&&(prev.is_null()||(*prev).id!=(*d).refid){return -IPSET_ERR_REF_EXIST}list_set_del(set,e);return 0}if (*d).before!=0{-IPSET_ERR_REF_EXIST}else{-IPSET_ERR_EXIST}}

unsafe fn list_set_flush(set:*mut ip_set){let map=(*set).data as *mut list_set;let mut e=(*map).members.next as *mut set_elem;while e!=&mut (*map).members as *mut list_head as *mut set_elem{let n=(*e).list.next as *mut set_elem;list_set_del(set,e);e=n}}
unsafe fn list_set_destroy(set:*mut ip_set){let map=(*set).data as *mut list_set;WARN_ON_ONCE(!list_empty(&(*map).members));kfree(map as *mut _);(*set).data=core::ptr::null_mut()}
unsafe fn list_set_memsize(map:*const list_set,dsize:usize)->usize{let mut n=0;let mut e=(*map).members.next as *mut set_elem;while e!=&(*map).members as *const list_head as *mut set_elem{n+=1;e=(*e).list.next as *mut set_elem;}core::mem::size_of::<list_set>()+n*dsize}
unsafe fn list_set_same_set(a:*const ip_set,b:*const ip_set)->bool{let x=(*a).data as *const list_set;let y=(*b).data as *const list_set;(*x).size==(*y).size&&(*a).timeout==(*b).timeout&&(*a).extensions==(*b).extensions}
unsafe fn list_set_cancel_gc(set:*mut ip_set){let map=(*set).data as *mut list_set;if SET_WITH_TIMEOUT(set){timer_shutdown_sync(&mut (*map).gc)}list_set_flush(set)}
unsafe fn list_set_gc_init(set:*mut ip_set,gc:unsafe fn(*mut timer_list)){let map=(*set).data as *mut list_set;timer_setup(&mut (*map).gc,gc,0);mod_timer(&mut (*map).gc,jiffies+IPSET_GC_PERIOD((*set).timeout)*HZ)}
unsafe fn list_set_gc(t:*mut timer_list){let map=timer_container_of!(t,list_set,gc);let set=(*map).set;spin_lock_bh(&mut (*set).lock);set_cleanup_entries(set);spin_unlock_bh(&mut (*set).lock);(*map).gc.expires=jiffies+IPSET_GC_PERIOD((*set).timeout)*HZ;add_timer(&mut (*map).gc)}
unsafe fn list_set_uadt(set:*mut ip_set,tb:*mut *mut nlattr,adt:ipset_adt,lineno:*mut u32,flags:u32,retried:bool)->i32{let map=(*set).data as *mut list_set;let mut e=set_adt_elem{id:IPSET_INVALID_ID,refid:IPSET_INVALID_ID,before:0};let ext=IP_SET_INIT_UEXT(set);if !(*tb.add(IPSET_ATTR_NAME as usize)).is_null(){*lineno=nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize));}if ip_set_get_extensions(set,tb,&ext)!=0{return -IPSET_ERR_PROTOCOL}let mut s: *mut ip_set=core::ptr::null_mut();e.id=ip_set_get_byname((*map).net,*tb.add(IPSET_ATTR_NAME as usize),&mut s);if e.id==IPSET_INVALID_ID{return -IPSET_ERR_NAME}let ret=match adt{IPSET_ADD=>list_set_uadd(set,&mut e as *mut _ as *mut c_void,&ext,&ext,flags),IPSET_DEL=>list_set_udel(set,&mut e as *mut _ as *mut c_void,&ext,&ext,flags),IPSET_TEST=>list_set_utest(set,&mut e as *mut _ as *mut c_void,&ext,&mut (ext),flags),_=>-EINVAL};if adt!=IPSET_ADD||ret!=0{ip_set_put_byindex((*map).net,e.id)}if ip_set_eexist(ret,flags){0}else{ret}}
unsafe fn init_list_set(net:*mut net,set:*mut ip_set,size:u32)->bool{let map=kzalloc_obj!(list_set);if map.is_null(){return false}(*map).size=size;(*map).net=net;(*map).set=set;INIT_LIST_HEAD(&mut (*map).members);(*set).data=map as *mut _;true}
unsafe fn list_set_create(net:*mut net,set:*mut ip_set,tb:*mut *mut nlattr,_flags:u32)->i32{let mut size=IP_SET_LIST_DEFAULT_SIZE;if !init_list_set(net,set,size){return -ENOMEM}if !(*tb.add(IPSET_ATTR_SIZE as usize)).is_null(){size=ip_set_get_h32(*tb.add(IPSET_ATTR_SIZE as usize));if size<IP_SET_LIST_MIN_SIZE{size=IP_SET_LIST_MIN_SIZE}}(*set).variant=&set_variant;(*set).dsize=ip_set_elem_len(set,tb,core::mem::size_of::<set_elem>(),core::mem::align_of::<set_elem>());if !(*tb.add(IPSET_ATTR_TIMEOUT as usize)).is_null(){(*set).timeout=ip_set_timeout_uget(*tb.add(IPSET_ATTR_TIMEOUT as usize));list_set_gc_init(set,list_set_gc)}0}

// Declarations corresponding to the module registration and the remaining
// exported variant/type structures are supplied by the kernel ipset headers.
#[allow(dead_code)] unsafe fn list_set_init() -> i32 { ip_set_type_register(&list_set_type) }
#[allow(dead_code)] unsafe fn list_set_fini() { rcu_barrier(); ip_set_type_unregister(&list_set_type); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
