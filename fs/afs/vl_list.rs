// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS vlserver list management.
 *
 * Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel headers and "internal.h" provide the types, constants and
// external functions referenced below.

pub unsafe fn afs_alloc_vlserver(name: *const i8, name_len: usize,
                                 port: u16) -> *mut afs_vlserver {
    static mut DEBUG_IDS: atomic_t = atomic_t { counter: 0 };
    let vlserver = kzalloc_flex::<afs_vlserver>(name_len + 1);
    if !vlserver.is_null() {
        refcount_set(&mut (*vlserver).ref_, 1);
        rwlock_init(&mut (*vlserver).lock);
        init_waitqueue_head(&mut (*vlserver).probe_wq);
        spin_lock_init(&mut (*vlserver).probe_lock);
        (*vlserver).debug_id = atomic_inc_return(&mut DEBUG_IDS);
        (*vlserver).rtt = UINT_MAX;
        (*vlserver).name_len = name_len;
        (*vlserver).service_id = VL_SERVICE;
        (*vlserver).port = port;
        core::ptr::copy_nonoverlapping(name as *const u8,
                                       (*vlserver).name.as_mut_ptr(), name_len);
    }
    vlserver
}

unsafe fn afs_vlserver_rcu(rcu: *mut rcu_head) {
    let vlserver = container_of!(rcu, afs_vlserver, rcu);
    afs_put_addrlist(rcu_access_pointer((*vlserver).addresses),
                     afs_alist_trace_put_vlserver);
    kfree_rcu(vlserver, rcu);
}

pub unsafe fn afs_put_vlserver(net: *mut afs_net, vlserver: *mut afs_vlserver) {
    if !vlserver.is_null() && refcount_dec_and_test(&mut (*vlserver).ref_) {
        call_rcu(&mut (*vlserver).rcu, afs_vlserver_rcu);
    }
}

pub unsafe fn afs_alloc_vlserver_list(nr_servers: u32) -> *mut afs_vlserver_list {
    let vllist = kzalloc_flex::<afs_vlserver_list>(nr_servers as usize);
    if !vllist.is_null() {
        refcount_set(&mut (*vllist).ref_, 1);
        rwlock_init(&mut (*vllist).lock);
    }
    vllist
}

pub unsafe fn afs_put_vlserverlist(net: *mut afs_net, vllist: *mut afs_vlserver_list) {
    if !vllist.is_null() && refcount_dec_and_test(&mut (*vllist).ref_) {
        let mut i = 0;
        while i < (*vllist).nr_servers {
            afs_put_vlserver(net, (*vllist).servers[i as usize].server);
            i += 1;
        }
        kfree_rcu(vllist, rcu);
    }
}

unsafe fn afs_extract_le16(b: &mut *const u8) -> u16 {
    let val = (**b as u16) | ((*b.add(1)) as u16) << 8;
    *b = b.add(2);
    val
}

/* Build a VL server address list from a DNS queried server list. */
unsafe fn afs_extract_vl_addrs(net: *mut afs_net, b_: &mut *const u8,
                               end: *const u8, mut nr_addrs: u8,
                               port: u16) -> *mut afs_addr_list {
    let mut b = *b_;
    let alist = afs_alloc_addrlist(nr_addrs);
    if alist.is_null() { return ERR_PTR(-ENOMEM); }
    if nr_addrs == 0 { return alist; }
    while nr_addrs > 0 && end.offset_from(b) >= nr_addrs as isize {
        let address_type = *b; b = b.add(1);
        let mut x = [0u32; 4];
        let ret = match address_type {
            DNS_ADDRESS_IS_IPV4 => {
                if end.offset_from(b) < 4 { *_leave!(" = -EINVAL [short inet]"); -EINVAL }
                else { core::ptr::copy_nonoverlapping(b, x.as_mut_ptr() as *mut u8, 4); b = b.add(4); afs_merge_fs_addr4(net, alist, x[0], port) }
            }
            DNS_ADDRESS_IS_IPV6 => {
                if end.offset_from(b) < 16 { *_leave!(" = -EINVAL [short inet6]"); -EINVAL }
                else { core::ptr::copy_nonoverlapping(b, x.as_mut_ptr() as *mut u8, 16); b = b.add(16); afs_merge_fs_addr6(net, alist, x.as_ptr(), port) }
            }
            _ => { _leave!(" = -EADDRNOTAVAIL [unknown af %u]", address_type); -EADDRNOTAVAIL }
        };
        if ret < 0 { *b_ = b; afs_put_addrlist(alist, afs_alist_trace_put_parse_error); return ERR_PTR(ret); }
        nr_addrs -= 1;
    }
    if (*alist).nr_ipv4 < (*alist).nr_addrs { (*alist).preferred = (*alist).nr_ipv4; }
    *b_ = b; alist
}

/* Build a VL server list from a DNS queried server list. */
pub unsafe fn afs_extract_vlserver_list(cell: *mut afs_cell, buffer: *const core::ffi::c_void,
                                        buffer_size: usize) -> *mut afs_vlserver_list {
    let hdr = buffer as *const dns_server_list_v1_header;
    let mut b = buffer as *const u8;
    let end = b.add(buffer_size);
    let mut ret = -ENOMEM;
    if end.offset_from(b) < core::mem::size_of::<dns_server_list_v1_header>() as isize ||
       (*hdr).hdr.content != DNS_PAYLOAD_IS_SERVER_LIST || (*hdr).hdr.version != 1 {
        pr_notice!("kAFS: Got DNS record [%u,%u] len %zu\n", (*hdr).hdr.content, (*hdr).hdr.version, end.offset_from(b));
        ret = -EDESTADDRREQ;
        return ERR_PTR(ret);
    }
    let nr_servers = (*hdr).nr_servers as i32;
    let vllist = afs_alloc_vlserver_list(nr_servers as u32);
    if vllist.is_null() { return ERR_PTR(-ENOMEM); }
    (*vllist).source = if (*hdr).source < NR__dns_record_source { (*hdr).source } else { NR__dns_record_source };
    (*vllist).status = if (*hdr).status < NR__dns_lookup_status { (*hdr).status } else { NR__dns_lookup_status };
    read_lock(&mut (*cell).vl_servers_lock);
    let previous = afs_get_vlserverlist(rcu_dereference_protected((*cell).vl_servers, lockdep_is_held(&(*cell).vl_servers_lock)));
    read_unlock(&mut (*cell).vl_servers_lock);
    b = b.add(core::mem::size_of::<dns_server_list_v1_header>());
    while end.offset_from(b) >= core::mem::size_of::<dns_server_list_v1_server>() as isize {
        let mut bs = core::mem::MaybeUninit::<dns_server_list_v1_server>::zeroed().assume_init();
        bs.name_len = afs_extract_le16(&mut b); bs.priority = afs_extract_le16(&mut b); bs.weight = afs_extract_le16(&mut b); bs.port = afs_extract_le16(&mut b);
        bs.source = *b; b=b.add(1); bs.status=*b; b=b.add(1); bs.protocol=*b; b=b.add(1); bs.nr_addrs=*b; b=b.add(1);
        let nlen = core::cmp::min(core::cmp::min(bs.name_len as isize, end.offset_from(b)), 255) as usize;
        _debug!("extract %u %u %u %u %u %u %*.*s", bs.name_len, bs.priority, bs.weight, bs.port, bs.protocol, bs.nr_addrs, bs.name_len, nlen, b);
        if end.offset_from(b) < bs.name_len as isize { break; }
        if bs.protocol == DNS_SERVER_PROTOCOL_UNSPECIFIED { bs.protocol = DNS_SERVER_PROTOCOL_UDP; }
        else if bs.protocol != DNS_SERVER_PROTOCOL_UDP { ret = -EPROTONOSUPPORT; _leave!(" = [proto %u]", bs.protocol); break; }
        if bs.port == 0 { bs.port = AFS_VL_PORT; }
        if bs.source > NR__dns_record_source { bs.source = NR__dns_record_source; }
        if bs.status > NR__dns_lookup_status { bs.status = NR__dns_lookup_status; }
        let mut server = core::ptr::null_mut();
        let mut i = 0; while i < (*previous).nr_servers { let p=(*previous).servers[i as usize].server; if (*p).name_len==bs.name_len && (*p).port==bs.port && strncasecmp(b,(*p).name.as_ptr(),bs.name_len)==0 { server=afs_get_vlserver(p); break; } i+=1; }
        if server.is_null() { server=afs_alloc_vlserver(b as *const i8,bs.name_len as usize,bs.port); if server.is_null() { ret=-ENOMEM; break; } }
        b=b.add(bs.name_len as usize);
        let addrs=afs_extract_vl_addrs((*cell).net,&mut b,end,bs.nr_addrs,bs.port); if IS_ERR(addrs) { ret=PTR_ERR(addrs); afs_put_vlserver((*cell).net,server); break; }
        if (*vllist).nr_servers >= nr_servers as u32 { afs_put_addrlist(addrs,afs_alist_trace_put_parse_empty); afs_put_vlserver((*cell).net,server); continue; }
        (*addrs).source=bs.source; (*addrs).status=bs.status;
        if (*addrs).nr_addrs==0 { afs_put_addrlist(addrs,afs_alist_trace_put_parse_empty); if rcu_access_pointer((*server).addresses).is_null() { afs_put_vlserver((*cell).net,server); continue; } }
        else { write_lock(&mut (*server).lock); let old=rcu_replace_pointer(&mut (*server).addresses,addrs,lockdep_is_held(&(*server).lock)); write_unlock(&mut (*server).lock); afs_put_addrlist(old,afs_alist_trace_put_vlserver_old); }
        let mut j=0; while j<(*vllist).nr_servers { let s=(*vllist).servers[j as usize].server; if (*s).name_len==(*server).name_len && (*s).port==(*server).port && strncasecmp((*s).name.as_ptr(),(*server).name.as_ptr(),(*server).name_len)==0 { afs_put_vlserver((*cell).net,server); server=core::ptr::null_mut(); break; } j+=1; } if server.is_null(){continue;}
        j=0; while j<(*vllist).nr_servers { if bs.priority<(*vllist).servers[j as usize].priority || (bs.priority==(*vllist).servers[j as usize].priority && bs.weight>(*vllist).servers[j as usize].weight){break;} j+=1; }
        if j<(*vllist).nr_servers { core::ptr::copy((*vllist).servers.add(j as usize),(*vllist).servers.add(j as usize+1),((*vllist).nr_servers-j as u32) as usize); }
        clear_bit(AFS_VLSERVER_FL_PROBED,&mut (*server).flags); (*vllist).servers[j as usize].priority=bs.priority; (*vllist).servers[j as usize].weight=bs.weight; (*vllist).servers[j as usize].server=server; (*vllist).nr_servers+=1;
    }
    if b != end { _debug!("parse error %zd", b.offset_from(end)); ret=-EPROTONOSUPPORT; }
    afs_put_vlserverlist((*cell).net,previous); if ret == -ENOMEM || ret == -EPROTONOSUPPORT { afs_put_vlserverlist((*cell).net,vllist); return ERR_PTR(ret); } vllist
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
