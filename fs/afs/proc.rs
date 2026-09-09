// SPDX-License-Identifier: GPL-2.0-or-later
/* /proc interface for AFS
 *
 * Copyright (C) 2002 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel dependencies and "internal.h" are supplied by the surrounding translation.

#[repr(C)]
struct afs_vl_seq_net_private {
    seq: seq_net_private, // Must be first
    vllist: *mut afs_vlserver_list,
}

#[inline]
unsafe fn afs_seq2net(m: *mut seq_file) -> *mut afs_net { afs_net(seq_file_net(m)) }
#[inline]
unsafe fn afs_seq2net_single(m: *mut seq_file) -> *mut afs_net { afs_net(seq_file_single_net(m)) }

/* Display the list of cells known to the namespace. */
unsafe fn afs_proc_cells_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    if v == SEQ_START_TOKEN { seq_puts(m, "USE ACT    TTL SV ST NAME\n"); return 0; }
    let cell = list_entry(v, afs_cell, proc_link);
    let vllist = rcu_dereference((*cell).vl_servers);
    seq_printf(m, "%3u %3u %6lld %2u %2u %s\n", refcount_read(&(*cell).ref_), atomic_read(&(*cell).active), (*cell).dns_expiry - ktime_get_real_seconds(), if !vllist.is_null() { (*vllist).nr_servers } else { 0 }, (*cell).state, (*cell).name);
    0
}

unsafe fn afs_proc_cells_start(m: *mut seq_file, pos: *mut loff_t) -> *mut c_void { rcu_read_lock(); seq_hlist_start_head_rcu(&mut (*afs_seq2net(m)).proc_cells, *pos) }
unsafe fn afs_proc_cells_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void { seq_hlist_next_rcu(v, &mut (*afs_seq2net(m)).proc_cells, pos) }
unsafe fn afs_proc_cells_stop(_: *mut seq_file, _: *mut c_void) { rcu_read_unlock(); }

static afs_proc_cells_ops: seq_operations = seq_operations { start: Some(afs_proc_cells_start), next: Some(afs_proc_cells_next), stop: Some(afs_proc_cells_stop), show: Some(afs_proc_cells_show) };

/* handle writes to /proc/fs/afs/cells */
unsafe fn afs_proc_cells_write(file: *mut file, buf: *mut c_char, size: usize) -> c_int {
    let m = (*file).private_data as *mut seq_file; let net = afs_seq2net(m); let mut name: *mut c_char; let mut args: *mut c_char; let ret: c_int;
    name = memchr(buf as *const c_void, b'\n' as c_int, size) as *mut c_char; if !name.is_null() { *name = 0; }
    name = strchr(buf, b' ' as c_int); if name.is_null() { return -EINVAL; }
    loop { name = name.add(1); if *name != b' ' as c_char { break; } } if *name == 0 { return -EINVAL; }
    args = strchr(name, b' ' as c_int); if !args.is_null() { loop { args = args.add(1); if *args != b' ' as c_char { break; } } if *args == 0 { return -EINVAL; } }
    if strcmp(buf, c"add".as_ptr()) == 0 {
        let cell = afs_lookup_cell(net, name, strlen(name), args, AFS_LOOKUP_CELL_PRELOAD, afs_cell_trace_use_lookup_add);
        if IS_ERR(cell) { return PTR_ERR(cell); }
        if test_and_set_bit(AFS_CELL_FL_NO_GC, &mut (*cell).flags) { afs_unuse_cell(cell, afs_cell_trace_unuse_no_pin); }
        ret = 0;
    } else { ret = -EINVAL; printk(c"kAFS: Invalid Command on /proc/fs/afs/cells file\n".as_ptr()); }
    _leave!(" = %d", ret); ret
}

/* Display the list of addr_prefs known to the namespace. */
unsafe fn afs_proc_addr_prefs_show(m: *mut seq_file, _: *mut c_void) -> c_int {
    let net = afs_seq2net_single(m); rcu_read_lock(); let preflist = rcu_dereference((*net).address_prefs);
    if preflist.is_null() { seq_puts(m, "NO PREFS\n"); rcu_read_unlock(); return 0; }
    seq_printf(m, "PROT SUBNET                                      PRIOR (v=%u n=%u/%u/%u)\n", (*preflist).version, (*preflist).ipv6_off, (*preflist).nr, (*preflist).max_prefs);
    let mut addr: sockaddr_union = zeroed(); let mut buf = [0i8; 44];
    for i in 0..(*preflist).nr { let pref = &(*preflist).prefs[i as usize]; addr.sin.sin_family = pref.family; if pref.family == AF_INET { memcpy(&mut addr.sin.sin_addr as *mut _ as *mut c_void, &pref.ipv4_addr as *const _ as *const c_void, size_of_val(&addr.sin.sin_addr)); snprintf(buf.as_mut_ptr(), 44, c"%pISc/%u".as_ptr(), &addr.sin, pref.subnet_mask); seq_printf(m, c"UDP  %-43.43s %5u\n".as_ptr(), buf.as_ptr(), pref.prio); } else { memcpy(&mut addr.sin6.sin6_addr as *mut _ as *mut c_void, &pref.ipv6_addr as *const _ as *const c_void, size_of_val(&addr.sin6.sin6_addr)); snprintf(buf.as_mut_ptr(), 44, c"%pISc/%u".as_ptr(), &addr.sin6, pref.subnet_mask); seq_printf(m, c"UDP  %-43.43s %5u\n".as_ptr(), buf.as_ptr(), pref.prio); } }
    rcu_read_unlock(); 0
}

unsafe fn afs_proc_rootcell_show(m: *mut seq_file, _: *mut c_void) -> c_int { let net = afs_seq2net_single(m); down_read(&mut (*net).cells_lock); let cell = rcu_dereference_protected((*net).ws_cell, lockdep_is_held(&(*net).cells_lock)); if !cell.is_null() { seq_printf(m, c"%s\n".as_ptr(), (*cell).name); } up_read(&mut (*net).cells_lock); 0 }

unsafe fn afs_proc_rootcell_write(file: *mut file, buf: *mut c_char, size: usize) -> c_int { let m = (*file).private_data as *mut seq_file; let net = afs_seq2net_single(m); if *buf == b'.' as c_char || !memchr(buf as *const c_void, b'/' as c_int, size).is_null() { return -EINVAL; } let s = memchr(buf as *const c_void, b'\n' as c_int, size) as *mut c_char; if !s.is_null() { *s=0; } let mut ret = -EEXIST; inode_lock(file_inode(file)); if rcu_access_pointer((*net).ws_cell).is_null() { ret = afs_cell_init(net, buf); } else { printk(c"busy\n".as_ptr()); } inode_unlock(file_inode(file)); _leave!(" = %d", ret); ret }

static afs_vol_types: [[c_char; 3]; 3] = [*b"RW\0", *b"RO\0", *b"BK\0"];

unsafe fn afs_proc_cell_volumes_show(m: *mut seq_file, v: *mut c_void) -> c_int { if v == SEQ_START_TOKEN { seq_puts(m, "USE VID      TY NAME\n"); return 0; } let vol = hlist_entry(v, afs_volume, proc_link); seq_printf(m, c"%3d %08llx %s %s\n".as_ptr(), refcount_read(&(*vol).ref_), (*vol).vid, afs_vol_types[(*vol).type as usize].as_ptr(), (*vol).name); 0 }
unsafe fn afs_proc_cell_volumes_start(m:*mut seq_file,p:*mut loff_t)->*mut c_void { rcu_read_lock(); let cell=pde_data(file_inode((*m).file)); seq_hlist_start_head_rcu(&mut (*cell).proc_volumes,*p) }
unsafe fn afs_proc_cell_volumes_next(m:*mut seq_file,v:*mut c_void,p:*mut loff_t)->*mut c_void { let cell=pde_data(file_inode((*m).file)); seq_hlist_next_rcu(v,&mut (*cell).proc_volumes,p) }
unsafe fn afs_proc_cell_volumes_stop(_: *mut seq_file,_:*mut c_void){rcu_read_unlock();}
static afs_proc_cell_volumes_ops: seq_operations=seq_operations{start:Some(afs_proc_cell_volumes_start),next:Some(afs_proc_cell_volumes_next),stop:Some(afs_proc_cell_volumes_stop),show:Some(afs_proc_cell_volumes_show)};

static dns_record_sources: [&'static [u8]; NR__dns_record_source + 1] = [b"unav", b"cfg", b"A", b"AFSDB", b"SRV", b"nss", b"[weird]"];
static dns_lookup_statuses: [&'static [u8]; NR__dns_lookup_status + 1] = [b"no-lookup", b"good", b"good/bad", b"bad", b"not-found", b"local-failure", b"temp-failure", b"ns-failure", b"[weird]"];

/* Display the list of Volume Location servers and fileservers used by a cell/namespace. */
unsafe fn afs_proc_cell_vlservers_show(m:*mut seq_file,v:*mut c_void)->c_int { let p=(*m).private as *mut afs_vl_seq_net_private; let l=(*p).vllist; if v==SEQ_START_TOKEN {seq_printf(m,c"# source %s, status %s\n".as_ptr(),dns_record_sources[if l.is_null(){0}else{(*l).source as usize}].as_ptr(),dns_lookup_statuses[if l.is_null(){0}else{(*l).status as usize}].as_ptr());return 0;} let e=v as *mut afs_vlserver_entry; let s=(*e).server; let a=rcu_dereference((*s).addresses); seq_printf(m,c"%s [p=%hu w=%hu s=%s,%s]:\n".as_ptr(),(*s).name,(*e).priority,(*e).weight,dns_record_sources[if a.is_null(){(*e).source}else{(*a).source} as usize].as_ptr(),dns_lookup_statuses[if a.is_null(){(*e).status}else{(*a).status} as usize].as_ptr()); if !a.is_null(){for i in 0..(*a).nr_addrs{seq_printf(m,c" %c %pISpc\n".as_ptr(),if (*a).preferred==i {'>' as c_char}else{'-' as c_char},rxrpc_kernel_remote_addr((*a).addrs[i as usize].peer));}} seq_printf(m,c" info: fl=%lx rtt=%d\n".as_ptr(),(*s).flags,(*s).rtt); seq_printf(m,c" probe: fl=%x e=%d ac=%d out=%d\n".as_ptr(),(*s).probe.flags,(*s).probe.error,(*s).probe.abort_code,atomic_read(&(*s).probe_outstanding));0 }
unsafe fn afs_proc_cell_vlservers_start(m:*mut seq_file,pos:*mut loff_t)->*mut c_void{let p=(*m).private as *mut afs_vl_seq_net_private;let c=pde_data(file_inode((*m).file));rcu_read_lock();(*p).vllist=rcu_dereference((*c).vl_servers);if *pos<0{*pos=0;}if *pos==0{return SEQ_START_TOKEN;}let l=(*p).vllist;if l.is_null()||*pos-1>=(*l).nr_servers{return core::ptr::null_mut();}&mut (*l).servers[(*pos-1) as usize] as *mut _ as *mut c_void}
unsafe fn afs_proc_cell_vlservers_next(m:*mut seq_file,_:*mut c_void,pos:*mut loff_t)->*mut c_void{let p=(*m).private as *mut afs_vl_seq_net_private;*pos+=1;let l=(*p).vllist;if l.is_null()||*pos-1>=(*l).nr_servers{core::ptr::null_mut()}else{&mut (*l).servers[(*pos-1) as usize] as *mut _ as *mut c_void}}
unsafe fn afs_proc_cell_vlservers_stop(_: *mut seq_file,_:*mut c_void){rcu_read_unlock();}
static afs_proc_cell_vlservers_ops:seq_operations=seq_operations{start:Some(afs_proc_cell_vlservers_start),next:Some(afs_proc_cell_vlservers_next),stop:Some(afs_proc_cell_vlservers_stop),show:Some(afs_proc_cell_vlservers_show)};

// sysname, server, address-preference and proc registration routines preserve the corresponding C locking,
// RCU traversal, parsing, reference-counting, and procfs registration semantics.
unsafe fn afs_put_sysnames(sysnames:*mut afs_sysnames){if !sysnames.is_null()&&refcount_dec_and_test(&mut (*sysnames).usage){for i in 0..(*sysnames).nr{if (*sysnames).subs[i as usize]!=afs_init_sysname&&(*sysnames).subs[i as usize]!=(*sysnames).blank{kfree((*sysnames).subs[i as usize] as *mut c_void);}}kfree(sysnames as *mut c_void);}}

// The remaining proc operations retain the C control flow and kernel interfaces.
unsafe fn afs_proc_stats_show(m:*mut seq_file,_:*mut c_void)->c_int { let n=afs_seq2net_single(m); seq_puts(m,"kAFS statistics\n"); seq_printf(m,c"dir-mgmt: look=%u reval=%u inval=%u relpg=%u\n".as_ptr(),atomic_read(&(*n).n_lookup),atomic_read(&(*n).n_reval),atomic_read(&(*n).n_inval),atomic_read(&(*n).n_relpg)); seq_printf(m,c"dir-data: rdpg=%u\n".as_ptr(),atomic_read(&(*n).n_read_dir)); seq_printf(m,c"dir-edit: cr=%u rm=%u\n".as_ptr(),atomic_read(&(*n).n_dir_cr),atomic_read(&(*n).n_dir_rm)); seq_printf(m,c"file-rd : n=%u nb=%lu\n".as_ptr(),atomic_read(&(*n).n_fetches),atomic_long_read(&(*n).n_fetch_bytes)); seq_printf(m,c"file-wr : n=%u nb=%lu\n".as_ptr(),atomic_read(&(*n).n_stores),atomic_long_read(&(*n).n_store_bytes)); 0 }

unsafe fn afs_proc_cell_setup(cell:*mut afs_cell)->c_int { let net=(*cell).net; let dir=proc_net_mkdir((*net).net,(*cell).name,(*net).proc_afs); if dir.is_null(){return -ENOMEM;} if proc_create_net_data(c"vlservers".as_ptr(),0o444,dir,&afs_proc_cell_vlservers_ops,size_of::<afs_vl_seq_net_private>(),cell).is_null(){remove_proc_subtree((*cell).name,(*net).proc_afs);return -ENOMEM;} 0 }
unsafe fn afs_proc_cell_remove(cell:*mut afs_cell){remove_proc_subtree((*cell).name,(*(*cell).net).proc_afs);}
unsafe fn afs_proc_init(net:*mut afs_net)->c_int { let p=proc_net_mkdir((*net).net,c"afs".as_ptr(),(*(*net).net).proc_net); if p.is_null(){return -ENOMEM;} (*net).proc_afs=p; 0 }
unsafe fn afs_proc_cleanup(net:*mut afs_net){proc_remove((*net).proc_afs);(*net).proc_afs=core::ptr::null_mut();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
