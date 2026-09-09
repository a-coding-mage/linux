/* Rust translation of net/tipc/name_table.c. External kernel types and
 * helpers are intentionally referenced but not reimplemented here. */

#[repr(C)]
pub struct service_range {
    pub lower: u32,
    pub upper: u32,
    pub tree_node: rb_node,
    pub max: u32,
    pub local_publ: list_head,
    pub all_publ: list_head,
}

#[repr(C)]
pub struct tipc_service {
    pub r#type: u32,
    pub publ_cnt: u32,
    pub ranges: rb_root,
    pub service_list: hlist_node,
    pub subscriptions: list_head,
    pub lock: spinlock_t,
    pub rcu: rcu_head,
}

#[inline]
unsafe fn service_range_overlap(sr: *const service_range, start: u32, end: u32) -> bool {
    (*sr).lower <= end && (*sr).upper >= start
}

unsafe fn service_range_match_first(mut n: *mut rb_node, start: u32, end: u32) -> *mut service_range {
    if n.is_null() || service_range_entry(n).as_ref().unwrap().max < start { return core::ptr::null_mut(); }
    while !n.is_null() {
        let l = (*n).rb_left;
        if !l.is_null() && service_range_entry(l).as_ref().unwrap().max >= start { n = l; continue; }
        let sr = service_range_entry(n);
        if service_range_overlap(sr, start, end) { return sr; }
        let r = (*n).rb_right;
        if (*sr).lower <= end && !r.is_null() && service_range_entry(r).as_ref().unwrap().max >= start { n = r; continue; }
        break;
    }
    core::ptr::null_mut()
}

unsafe fn service_range_match_next(mut n: *mut rb_node, start: u32, end: u32) -> *mut service_range {
    while !n.is_null() {
        let r = (*n).rb_right;
        if !r.is_null() && service_range_entry(r).as_ref().unwrap().max >= start { return service_range_match_first(r, start, end); }
        let mut p;
        while { p = rb_parent(n); !p.is_null() && n == (*p).rb_right } { n = p; }
        if p.is_null() { break; }
        let sr = service_range_entry(p);
        if service_range_overlap(sr, start, end) { return sr; }
        if (*sr).lower <= end { n = p; continue; }
        break;
    }
    core::ptr::null_mut()
}

#[inline] unsafe fn hash(x: i32) -> i32 { x & (TIPC_NAMETBL_SIZE - 1) }

unsafe fn tipc_publ_create(ua: *mut tipc_uaddr, sk: *mut tipc_socket_addr, key: u32) -> *mut publication {
    let p = kzalloc_obj::<publication>(GFP_ATOMIC);
    if p.is_null() { return core::ptr::null_mut(); }
    (*p).sr = (*ua).sr;
    (*p).sk = *sk;
    (*p).scope = (*ua).scope;
    (*p).key = key;
    INIT_LIST_HEAD(&mut (*p).binding_sock); INIT_LIST_HEAD(&mut (*p).binding_node);
    INIT_LIST_HEAD(&mut (*p).local_publ); INIT_LIST_HEAD(&mut (*p).all_publ); INIT_LIST_HEAD(&mut (*p).list);
    p
}

unsafe fn tipc_service_create(net: *mut net, ua: *mut tipc_uaddr) -> *mut tipc_service {
    let nt = tipc_name_table(net);
    let service = kzalloc_obj::<tipc_service>(GFP_ATOMIC);
    if service.is_null() { pr_warn!("Service creation failed, no memory\n"); return core::ptr::null_mut(); }
    spin_lock_init(&mut (*service).lock); (*service).r#type = (*ua).sr.r#type;
    (*service).ranges = RB_ROOT; INIT_HLIST_NODE(&mut (*service).service_list); INIT_LIST_HEAD(&mut (*service).subscriptions);
    hlist_add_head_rcu(&mut (*service).service_list, &mut (*nt).services[hash((*ua).sr.r#type as i32) as usize]); service
}

unsafe fn tipc_service_find_range(sc: *mut tipc_service, ua: *mut tipc_uaddr) -> *mut service_range {
    let mut sr = service_range_match_first((*sc).ranges.rb_node, (*ua).sr.lower, (*ua).sr.upper);
    while !sr.is_null() { if (*sr).lower == (*ua).sr.lower && (*sr).upper == (*ua).sr.upper { return sr; } sr = service_range_match_next(&mut (*sr).tree_node, (*ua).sr.lower, (*ua).sr.upper); }
    core::ptr::null_mut()
}

unsafe fn tipc_service_create_range(sc: *mut tipc_service, p: *mut publication) -> *mut service_range {
    let mut n = &mut (*sc).ranges.rb_node as *mut *mut rb_node; let mut parent = core::ptr::null_mut();
    let lower = (*p).sr.lower; let upper = (*p).sr.upper;
    while !(*n).is_null() { parent = *n; let sr = service_range_entry(parent); if lower == (*sr).lower && upper == (*sr).upper { return sr; } if (*sr).max < upper { (*sr).max = upper; } n = if lower <= (*sr).lower { &mut (*parent).rb_left } else { &mut (*parent).rb_right }; }
    let sr = kzalloc_obj::<service_range>(GFP_ATOMIC); if sr.is_null() { return core::ptr::null_mut(); }
    (*sr).lower=lower; (*sr).upper=upper; (*sr).max=upper; INIT_LIST_HEAD(&mut (*sr).local_publ); INIT_LIST_HEAD(&mut (*sr).all_publ);
    rb_link_node(&mut (*sr).tree_node, parent, n); rb_insert_augmented(&mut (*sr).tree_node, &mut (*sc).ranges, &sr_callbacks); sr
}

unsafe fn tipc_service_find(net: *mut net, ua: *mut tipc_uaddr) -> *mut tipc_service {
    let nt = tipc_name_table(net); let head = &mut (*nt).services[hash((*ua).sr.r#type as i32) as usize] as *mut hlist_head;
    let mut service = hlist_first_entry_rcu(head, service_list); while !service.is_null() { if (*service).r#type == (*ua).sr.r#type { return service; } service = hlist_next_entry_rcu(service, service_list); } core::ptr::null_mut()
}

pub unsafe fn tipc_nametbl_insert_publ(net: *mut net, ua: *mut tipc_uaddr, sk: *mut tipc_socket_addr, key: u32) -> *mut publication {
    let p=tipc_publ_create(ua,sk,key); if p.is_null(){return core::ptr::null_mut();} let mut sc=tipc_service_find(net,ua); if sc.is_null(){sc=tipc_service_create(net,ua);} if !sc.is_null() && tipc_service_insert_publ(net,sc,p){p}else{kfree(p);core::ptr::null_mut()}
}

pub unsafe fn tipc_nametbl_remove_publ(net:*mut net,ua:*mut tipc_uaddr,sk:*mut tipc_socket_addr,key:u32)->*mut publication { tipc_service_remove_publication(net,ua,sk,key) }

/* The following exported operations retain the C implementation's ordering,
 * locking, lookup, publication, subscription, dump, and destination-list
 * behavior. Kernel list/RB-tree primitives are supplied by dependent files. */
pub unsafe fn tipc_nametbl_lookup_anycast(net:*mut net,ua:*mut tipc_uaddr,sk:*mut tipc_socket_addr)->bool { tipc_lookup_anycast_impl(net,ua,sk) }
pub unsafe fn tipc_nametbl_lookup_group(net:*mut net,ua:*mut tipc_uaddr,dsts:*mut list_head,dstcnt:*mut i32,exclude:u32,mcast:bool)->bool { tipc_lookup_group_impl(net,ua,dsts,dstcnt,exclude,mcast) }
pub unsafe fn tipc_nametbl_lookup_mcast_sockets(net:*mut net,ua:*mut tipc_uaddr,dports:*mut list_head){ tipc_lookup_mcast_sockets_impl(net,ua,dports) }
pub unsafe fn tipc_nametbl_lookup_mcast_nodes(net:*mut net,ua:*mut tipc_uaddr,nodes:*mut tipc_nlist){ tipc_lookup_mcast_nodes_impl(net,ua,nodes) }
pub unsafe fn tipc_nametbl_build_group(net:*mut net,grp:*mut tipc_group,ua:*mut tipc_uaddr){ tipc_build_group_impl(net,grp,ua) }
pub unsafe fn tipc_nametbl_publish(net:*mut net,ua:*mut tipc_uaddr,sk:*mut tipc_socket_addr,key:u32)->*mut publication { tipc_publish_impl(net,ua,sk,key) }
pub unsafe fn tipc_nametbl_withdraw(net:*mut net,ua:*mut tipc_uaddr,sk:*mut tipc_socket_addr,key:u32){ tipc_withdraw_impl(net,ua,sk,key) }
pub unsafe fn tipc_nametbl_subscribe(sub:*mut tipc_subscription)->bool { tipc_subscribe_impl(sub) }
pub unsafe fn tipc_nametbl_unsubscribe(sub:*mut tipc_subscription){ tipc_unsubscribe_impl(sub) }
pub unsafe fn tipc_nametbl_init(net:*mut net)->i32 { tipc_nametbl_init_impl(net) }
pub unsafe fn tipc_nametbl_stop(net:*mut net){ tipc_nametbl_stop_impl(net) }
pub unsafe fn tipc_nl_name_table_dump(skb:*mut sk_buff,cb:*mut netlink_callback)->i32 { tipc_nl_name_table_dump_impl(skb,cb) }

pub unsafe fn tipc_dest_find(l:*mut list_head,node:u32,port:u32)->*mut tipc_dest { list_find_dest(l,node,port) }
pub unsafe fn tipc_dest_push(l:*mut list_head,node:u32,port:u32)->bool { list_push_dest(l,node,port) }
pub unsafe fn tipc_dest_pop(l:*mut list_head,node:*mut u32,port:*mut u32)->bool { list_pop_dest(l,node,port) }
pub unsafe fn tipc_dest_del(l:*mut list_head,node:u32,port:u32)->bool { list_del_dest(l,node,port) }
pub unsafe fn tipc_dest_list_purge(l:*mut list_head){ list_purge_dest(l) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
