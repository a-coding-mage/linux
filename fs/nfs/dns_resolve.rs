// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/nfs/dns_resolve.c
 *
 * Copyright (c) 2009 Trond Myklebust <Trond.Myklebust@netapp.com>
 *
 * Resolves DNS hostnames into valid ip addresses
 */

// C headers and local headers are supplied by the surrounding kernel bindings.

#[cfg(feature = "CONFIG_NFS_USE_KERNEL_DNS")]
pub unsafe fn nfs_dns_resolve_name(
    net: *mut net,
    name: *mut ::std::os::raw::c_char,
    namelen: usize,
    ss: *mut sockaddr_storage,
    salen: usize,
) -> isize {
    let sa = ss as *mut sockaddr;
    let mut ip_addr: *mut ::std::os::raw::c_char = core::ptr::null_mut();
    let ip_len = dns_query(net, core::ptr::null(), name, namelen, core::ptr::null_mut(), &mut ip_addr, core::ptr::null_mut(), false);
    let ret = if ip_len > 0 {
        rpc_pton(net, ip_addr, ip_len, sa, salen)
    } else {
        -ESRCH
    };
    kfree(ip_addr as *mut core::ffi::c_void);
    ret
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
const NFS_DNS_HASHBITS: u32 = 4;
#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
const NFS_DNS_HASHTBL_SIZE: u32 = 1 << NFS_DNS_HASHBITS;

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
#[repr(C)]
struct nfs_dns_ent {
    h: cache_head,
    hostname: *mut ::std::os::raw::c_char,
    namelen: usize,
    addr: sockaddr_storage,
    addrlen: usize,
    rcu_head: rcu_head,
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_ent_update(cnew: *mut cache_head, ckey: *mut cache_head) {
    let new = container_of!(cnew, nfs_dns_ent, h);
    let key = container_of!(ckey, nfs_dns_ent, h);
    memcpy(&mut (*new).addr as *mut _ as *mut _, &(*key).addr as *const _ as *const _, (*key).addrlen);
    (*new).addrlen = (*key).addrlen;
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_ent_init(cnew: *mut cache_head, ckey: *mut cache_head) {
    let new = container_of!(cnew, nfs_dns_ent, h);
    let key = container_of!(ckey, nfs_dns_ent, h);
    kfree((*new).hostname as *mut _);
    (*new).hostname = kmemdup_nul((*key).hostname, (*key).namelen, GFP_KERNEL);
    if !(*new).hostname.is_null() {
        (*new).namelen = (*key).namelen;
        nfs_dns_ent_update(cnew, ckey);
    } else {
        (*new).namelen = 0;
        (*new).addrlen = 0;
    }
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_ent_free_rcu(head: *mut rcu_head) {
    let item = container_of!(head, nfs_dns_ent, rcu_head);
    kfree((*item).hostname as *mut _);
    kfree(item as *mut _);
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_ent_put(r: *mut kref) {
    let item = container_of!(r, nfs_dns_ent, h.ref_);
    call_rcu(&mut (*item).rcu_head, nfs_dns_ent_free_rcu);
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_ent_alloc() -> *mut cache_head {
    let item = kmalloc_obj::<nfs_dns_ent>();
    if !item.is_null() {
        (*item).hostname = core::ptr::null_mut();
        (*item).namelen = 0;
        (*item).addrlen = 0;
        return &mut (*item).h;
    }
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_hash(key: *const nfs_dns_ent) -> u32 { hash_str((*key).hostname, NFS_DNS_HASHBITS) }

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_request(_cd: *mut cache_detail, ch: *mut cache_head, bpp: *mut *mut ::std::os::raw::c_char, blen: *mut i32) {
    let key = container_of!(ch, nfs_dns_ent, h);
    qword_add(bpp, blen, (*key).hostname);
    (*bpp).offset(-1).write(b'\n' as _);
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_upcall(cd: *mut cache_detail, ch: *mut cache_head) -> i32 {
    let key = container_of!(ch, nfs_dns_ent, h);
    if test_and_set_bit(CACHE_PENDING, &mut (*ch).flags) != 0 { return 0; }
    if nfs_cache_upcall(cd, (*key).hostname) == 0 { return 0; }
    clear_bit(CACHE_PENDING, &mut (*ch).flags);
    sunrpc_cache_upcall_warn(cd, ch)
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_match(ca: *mut cache_head, cb: *mut cache_head) -> i32 {
    let a = container_of!(ca, nfs_dns_ent, h); let b = container_of!(cb, nfs_dns_ent, h);
    if (*a).namelen == 0 || (*a).namelen != (*b).namelen { return 0; }
    (memcmp((*a).hostname as *const _, (*b).hostname as *const _, (*a).namelen) == 0) as i32
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_lookup(cd: *mut cache_detail, key: *mut nfs_dns_ent) -> *mut nfs_dns_ent {
    let ch = sunrpc_cache_lookup_rcu(cd, &mut (*key).h, nfs_dns_hash(key));
    if ch.is_null() { core::ptr::null_mut() } else { container_of!(ch, nfs_dns_ent, h) }
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_ent_update_ref(cnew: *mut cache_head, ckey: *mut cache_head) { nfs_dns_ent_update(cnew, ckey); }

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_update(cd: *mut cache_detail, new: *mut nfs_dns_ent, key: *mut nfs_dns_ent) -> *mut nfs_dns_ent {
    let ch = sunrpc_cache_update(cd, &mut (*new).h, &mut (*key).h, nfs_dns_hash(key));
    if ch.is_null() { core::ptr::null_mut() } else { container_of!(ch, nfs_dns_ent, h) }
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_parse(cd: *mut cache_detail, buf: *mut ::std::os::raw::c_char, buflen: i32) -> i32 {
    let mut buf1 = [0 as ::std::os::raw::c_char; NFS_DNS_HOSTNAME_MAXLEN as usize + 1];
    let mut key: nfs_dns_ent = core::mem::zeroed();
    if *buf.offset((buflen - 1) as isize) != b'\n' as _ { return -EINVAL; }
    *buf.offset((buflen - 1) as isize) = 0;
    let mut p = buf; let len = qword_get(&mut p, buf1.as_mut_ptr(), buf1.len()); if len <= 0 { return -EINVAL; }
    key.addrlen = rpc_pton((*cd).net, buf1.as_mut_ptr(), len, &mut key.addr as *mut _ as *mut sockaddr, core::mem::size_of::<sockaddr_storage>());
    let len = qword_get(&mut p, buf1.as_mut_ptr(), buf1.len()); if len <= 0 { return -EINVAL; }
    key.hostname = buf1.as_mut_ptr(); key.namelen = len as usize; memset(&mut key.h as *mut _ as *mut _, 0, core::mem::size_of::<cache_head>());
    let mut ttl = 0u32; if get_uint(&mut p, &mut ttl) < 0 || ttl == 0 { return -EINVAL; }
    key.h.expiry_time = ttl as _ + seconds_since_boot();
    let mut item = nfs_dns_lookup(cd, &mut key); if item.is_null() { return -ENOMEM; }
    if key.addrlen == 0 { set_bit(CACHE_NEGATIVE, &mut key.h.flags); }
    item = nfs_dns_update(cd, &mut key, item); if item.is_null() { return -ENOMEM; }
    cache_put(&mut (*item).h, cd); 0
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_show(m: *mut seq_file, _cd: *mut cache_detail, h: *mut cache_head) -> i32 {
    if h.is_null() { seq_puts(m, "# ip address      hostname        ttl\n"); return 0; }
    let item = container_of!(h, nfs_dns_ent, h); let mut ttl = (*item).h.expiry_time - seconds_since_boot(); if ttl < 0 { ttl = 0; }
    if test_bit(CACHE_NEGATIVE, &(*h).flags) == 0 { let mut buf = [0 as ::std::os::raw::c_char; INET6_ADDRSTRLEN as usize + IPV6_SCOPE_ID_LEN as usize + 1]; rpc_ntop(&(*item).addr as *const _ as *mut sockaddr, buf.as_mut_ptr(), buf.len()); seq_printf(m, "%15s ", buf.as_ptr()); } else { seq_puts(m, "<none>          "); }
    seq_printf(m, "%15s %ld\n", (*item).hostname, ttl); 0
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs4_dns_net_init(net: *mut net) -> i32 { nfs_dns_resolver_cache_init(net) }
#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs4_dns_net_exit(net: *mut net) { nfs_dns_resolver_cache_destroy(net); }

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn rpc_pipefs_event(nb: *mut notifier_block, event: u64, ptr: *mut core::ffi::c_void) -> i32 {
    let sb = ptr as *mut super_block; let net = (*sb).s_fs_info; let nn = net_generic(net, nfs_net_id); let cd = (*nn).nfs_dns_resolve; if cd.is_null() { return 0; }
    if try_module_get(THIS_MODULE) == 0 { return 0; }
    let ret = match event { RPC_PIPEFS_MOUNT => nfs_cache_register_sb(sb, cd), RPC_PIPEFS_UMOUNT => { nfs_cache_unregister_sb(sb, cd); 0 }, _ => -ENOTSUPP };
    module_put(THIS_MODULE); ret
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
pub unsafe fn nfs_dns_resolve_name(net: *mut net, name: *mut ::std::os::raw::c_char, namelen: usize, ss: *mut sockaddr_storage, salen: usize) -> isize {
    let mut key: nfs_dns_ent = core::mem::zeroed(); key.hostname = name; key.namelen = namelen;
    let mut item = core::ptr::null_mut(); let nn = net_generic(net, nfs_net_id);
    let mut ret = do_cache_lookup_wait((*nn).nfs_dns_resolve, &mut key, &mut item);
    if ret == 0 {
        if salen >= (*item).addrlen { memcpy(ss as *mut _, &(*item).addr as *const _ as *const _, (*item).addrlen); ret = (*item).addrlen as isize; } else { ret = -EOVERFLOW; }
        cache_put(&mut (*item).h, (*nn).nfs_dns_resolve);
    } else if ret == -ENOENT { ret = -ESRCH; }
    ret
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn do_cache_lookup_wait(cd: *mut cache_detail, key: *mut nfs_dns_ent, item: *mut *mut nfs_dns_ent) -> isize {
    let dreq = nfs_cache_defer_req_alloc(); if dreq.is_null() { return -ENOMEM; }
    let mut ret = do_cache_lookup(cd, key, item, dreq);
    if ret == -EAGAIN { ret = nfs_cache_wait_for_upcall(dreq); if ret == 0 { ret = do_cache_lookup_nowait(cd, key, item); } }
    nfs_cache_defer_req_put(dreq); ret
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn do_cache_lookup(cd: *mut cache_detail, key: *mut nfs_dns_ent, item: *mut *mut nfs_dns_ent, dreq: *mut nfs_cache_defer_req) -> isize {
    *item = nfs_dns_lookup(cd, key); if !(*item).is_null() { let ret = cache_check(cd, &mut (**item).h, &mut (*dreq).req); if ret != 0 { *item = core::ptr::null_mut(); return ret as isize; } return 0; } -ENOMEM
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn do_cache_lookup_nowait(cd: *mut cache_detail, key: *mut nfs_dns_ent, item: *mut *mut nfs_dns_ent) -> isize {
    *item = nfs_dns_lookup(cd, key); if (*item).is_null() { return -ENOMEM; }
    if test_bit(CACHE_VALID, &(*(*item)).h.flags) == 0 || (*(*item)).h.expiry_time < seconds_since_boot() || (*cd).flush_time > (*(*item)).h.last_refresh { cache_put(&mut (*(*item)).h, cd); *item = core::ptr::null_mut(); return -ETIMEDOUT; }
    if test_bit(CACHE_NEGATIVE, &(*(*item)).h.flags) != 0 { cache_put(&mut (*(*item)).h, cd); *item = core::ptr::null_mut(); return -ENOENT; } 0
}

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_resolver_cache_init(net: *mut net) -> i32 { let nn = net_generic(net, nfs_net_id); (*nn).nfs_dns_resolve = cache_create_net(&mut nfs_dns_resolve_template, net); if IS_ERR((*nn).nfs_dns_resolve) { return PTR_ERR((*nn).nfs_dns_resolve); } let err = nfs_cache_register_net(net, (*nn).nfs_dns_resolve); if err != 0 { cache_destroy_net((*nn).nfs_dns_resolve, net); } err }

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
unsafe fn nfs_dns_resolver_cache_destroy(net: *mut net) { let nn = net_generic(net, nfs_net_id); nfs_cache_unregister_net(net, (*nn).nfs_dns_resolve); cache_destroy_net((*nn).nfs_dns_resolve, net); }

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
static mut nfs_dns_resolve_template: cache_detail = cache_detail {
    owner: THIS_MODULE, hash_size: NFS_DNS_HASHTBL_SIZE, name: b"dns_resolve\0".as_ptr() as _,
    cache_put: Some(nfs_dns_ent_put), cache_upcall: Some(nfs_dns_upcall), cache_request: Some(nfs_dns_request),
    cache_parse: Some(nfs_dns_parse), cache_show: Some(nfs_dns_show), match_: Some(nfs_dns_match),
    init: Some(nfs_dns_ent_init), update: Some(nfs_dns_ent_update_ref), alloc: Some(nfs_dns_ent_alloc),
};

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
static mut nfs4_dns_resolver_ops: pernet_operations = pernet_operations { init: Some(nfs4_dns_net_init), exit: Some(nfs4_dns_net_exit) };

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
static mut nfs_dns_resolver_block: notifier_block = notifier_block { notifier_call: Some(rpc_pipefs_event) };

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
pub unsafe fn nfs_dns_resolver_init() -> i32 { let mut err = register_pernet_subsys(&mut nfs4_dns_resolver_ops); if err < 0 { return err; } err = rpc_pipefs_notifier_register(&mut nfs_dns_resolver_block); if err < 0 { unregister_pernet_subsys(&mut nfs4_dns_resolver_ops); } err }

#[cfg(not(feature = "CONFIG_NFS_USE_KERNEL_DNS"))]
pub unsafe fn nfs_dns_resolver_destroy() { rpc_pipefs_notifier_unregister(&mut nfs_dns_resolver_block); unregister_pernet_subsys(&mut nfs4_dns_resolver_ops); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
