// SPDX-License-Identifier: GPL-2.0-or-later
/* Server address list management
 *
 * Copyright (C) 2017 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use crate::*;
use core::ffi::{c_char, c_void};

unsafe fn afs_free_addrlist(rcu: *mut rcu_head) {
    let alist = container_of!(rcu, afs_addr_list, rcu);
    let mut i: u32 = 0;

    while i < (*alist).nr_addrs {
        rxrpc_kernel_put_peer((*alist).addrs[i as usize].peer);
        i += 1;
    }
    trace_afs_alist((*alist).debug_id, refcount_read(&(*alist).usage), afs_alist_trace_free);
    kfree(alist as *mut c_void);
}

/* Release an address list. */
pub unsafe extern "C" fn afs_put_addrlist(
    alist: *mut afs_addr_list,
    reason: afs_alist_trace,
) {
    if alist.is_null() {
        return;
    }
    let debug_id = (*alist).debug_id;
    let mut r: i32 = 0;
    let dead = __refcount_dec_and_test(&mut (*alist).usage, &mut r);
    trace_afs_alist(debug_id, r - 1, reason);
    if dead {
        call_rcu(&mut (*alist).rcu, afs_free_addrlist);
    }
}

pub unsafe extern "C" fn afs_get_addrlist(
    alist: *mut afs_addr_list,
    reason: afs_alist_trace,
) -> *mut afs_addr_list {
    let mut r: i32 = 0;
    if !alist.is_null() {
        __refcount_inc(&mut (*alist).usage, &mut r);
        trace_afs_alist((*alist).debug_id, r + 1, reason);
    }
    alist
}

/* Allocate an address list. */
pub unsafe extern "C" fn afs_alloc_addrlist(mut nr: u32) -> *mut afs_addr_list {
    _enter!("%u", nr);
    if nr > AFS_MAX_ADDRESSES {
        nr = AFS_MAX_ADDRESSES;
    }

    let alist = kzalloc_flex!(afs_addr_list, addrs, nr);
    if alist.is_null() {
        return core::ptr::null_mut();
    }
    refcount_set(&mut (*alist).usage, 1);
    (*alist).max_addrs = nr;
    (*alist).debug_id = atomic_inc_return(&mut AFSLIST_DEBUG_ID);
    trace_afs_alist((*alist).debug_id, 1, afs_alist_trace_alloc);
    alist
}

static mut AFSLIST_DEBUG_ID: atomic_t = atomic_t::new(0);

/* Parse a text string consisting of delimited addresses. */
pub unsafe extern "C" fn afs_parse_text_addrs(
    net: *mut afs_net,
    text: *const c_char,
    len: usize,
    delim: c_char,
    service: u16,
    port: u16,
) -> *mut afs_vlserver_list {
    let mut vllist: *mut afs_vlserver_list;
    let mut alist: *mut afs_addr_list;
    let mut p = text;
    let end = text.add(len);
    let mut problem: *const c_char;
    let mut nr: u32 = 0;
    let mut ret: i32 = -ENOMEM;

    _enter!("%*.*s,%c", len as i32, len as i32, text, delim);
    if len == 0 {
        _leave!(" = -EDESTADDRREQ [empty]");
        return ERR_PTR(-EDESTADDRREQ);
    }

    let mut delimiter = delim;
    if delimiter == b':' as c_char && (memchr(text, b',' as c_int, len).is_some() || memchr(text, b'.' as c_int, len).is_none()) {
        delimiter = b',' as c_char;
    }

    loop {
        if *p == 0 {
            problem = c"nul".as_ptr(); goto_inval!();
        }
        if *p != delimiter {
            nr += 1;
            if *p == b'[' as c_char {
                p = p.add(1);
                if p == end { problem = c"brace1".as_ptr(); goto_inval!(); }
                p = memchr_ptr(p, b']' as c_int, end.offset_from(p) as usize);
                if p.is_null() { problem = c"brace2".as_ptr(); goto_inval!(); }
                p = p.add(1);
                if p >= end { break; }
            }
            p = memchr_ptr(p, delimiter as c_int, end.offset_from(p) as usize);
            if p.is_null() { break; }
            p = p.add(1);
        }
        if p >= end { break; }
    }

    _debug!("%u/%u addresses", nr, AFS_MAX_ADDRESSES);
    vllist = afs_alloc_vlserver_list(1);
    if vllist.is_null() { return ERR_PTR(-ENOMEM); }
    (*vllist).nr_servers = 1;
    (*vllist).servers[0].server = afs_alloc_vlserver(c"<dummy>".as_ptr(), 7, AFS_VL_PORT);
    if (*vllist).servers[0].server.is_null() { goto error_vl; }
    alist = afs_alloc_addrlist(nr);
    if alist.is_null() { goto error; }

    p = text;
    loop {
        let (mut q, mut stop): (*const c_char, *const c_char);
        let mut xport = port;
        let mut x = [0u32; 4];
        let family: i32;
        if *p == delimiter { p = p.add(1); if p >= end { break; } continue; }
        if *p == b'[' as c_char { p = p.add(1); q = memchr_ptr(p, b']' as c_int, end.offset_from(p) as usize); }
        else { q = p; while q < end && *q != b'+' as c_char && *q != delimiter { q = q.add(1); } }
        if in4_pton(p, q.offset_from(p) as i32, x.as_mut_ptr() as *mut u8, -1, &mut stop) { family = AF_INET; }
        else if in6_pton(p, q.offset_from(p) as i32, x.as_mut_ptr() as *mut u8, -1, &mut stop) { family = AF_INET6; }
        else { problem = c"family".as_ptr(); goto_bad_address!(); }
        p = q;
        if stop != p { problem = c"nostop".as_ptr(); goto_bad_address!(); }
        if p < end && *p == b']' as c_char { p = p.add(1); }
        if p < end {
            if *p == b'+' as c_char {
                xport = 0; p = p.add(1);
                if p >= end || !isdigit(*p) { problem = c"port".as_ptr(); goto_bad_address!(); }
                while p < end && isdigit(*p) { xport = xport.wrapping_mul(10).wrapping_add((*p - b'0' as c_char) as u16); if xport > 65535 { problem = c"pval".as_ptr(); goto_bad_address!(); } p = p.add(1); }
            } else if *p == delimiter { p = p.add(1); } else { problem = c"weird".as_ptr(); goto_bad_address!(); }
        }
        ret = if family == AF_INET { afs_merge_fs_addr4(net, alist, x[0], xport) } else { afs_merge_fs_addr6(net, alist, x.as_mut_ptr(), xport) };
        if ret < 0 { goto error; }
        if p >= end { break; }
    }
    rcu_assign_pointer!((*(*vllist).servers[0].server).addresses, alist);
    _leave!(" = [nr %u]", (*alist).nr_addrs);
    return vllist;

    // The labels below preserve the C error paths; project error/logging helpers are external.
    goto_inval!(); goto_bad_address!(); goto error; goto error_vl;
}

pub unsafe extern "C" fn afs_dns_query(cell: *mut afs_cell, expiry: *mut time64_t) -> *mut afs_vlserver_list {
    let mut result: *mut c_char = core::ptr::null_mut();
    _enter!("%s", (*cell).name);
    let ret = dns_query((*cell).net.net, c"afsdb".as_ptr(), (*cell).name, (*cell).name_len, c"srv=1".as_ptr(), &mut result, expiry, true);
    if ret < 0 { _leave!(" = %d [dns]", ret); return ERR_PTR(ret); }
    if *expiry == 0 { *expiry = ktime_get_real_seconds() + 60; }
    let vllist = if ret > 1 && *result == 0 { afs_extract_vlserver_list(cell, result, ret) } else { afs_parse_text_addrs((*cell).net, result, ret as usize, b',' as c_char, VL_SERVICE, AFS_VL_PORT) };
    kfree(result as *mut c_void);
    vllist
}

pub unsafe extern "C" fn afs_merge_fs_addr4(net: *mut afs_net, alist: *mut afs_addr_list, xdr: u32, port: u16) -> i32 { merge_fs_addr!(net, alist, xdr, port, AF_INET) }
pub unsafe extern "C" fn afs_merge_fs_addr6(net: *mut afs_net, alist: *mut afs_addr_list, xdr: *mut u32, port: u16) -> i32 { merge_fs_addr!(net, alist, xdr, port, AF_INET6) }

pub unsafe extern "C" fn afs_set_peer_appdata(server: *mut afs_server, old_alist: *mut afs_addr_list, new_alist: *mut afs_addr_list) {
    let data = server as usize;
    let (mut n, mut o) = (0usize, 0usize);
    if old_alist.is_null() { while n < (*new_alist).nr_addrs as usize { rxrpc_kernel_set_peer_data((*new_alist).addrs[n].peer, data); n += 1; } return; }
    if new_alist.is_null() { while o < (*old_alist).nr_addrs as usize { rxrpc_kernel_set_peer_data((*old_alist).addrs[o].peer, 0); o += 1; } return; }
    while n < (*new_alist).nr_addrs as usize && o < (*old_alist).nr_addrs as usize {
        let pn = (*new_alist).addrs[n].peer; let po = (*old_alist).addrs[o].peer;
        if pn == po { continue; }
        if (pn as usize) < (po as usize) { rxrpc_kernel_set_peer_data(pn, data); n += 1; } else { rxrpc_kernel_set_peer_data(po, 0); o += 1; }
    }
    while n < (*new_alist).nr_addrs as usize { rxrpc_kernel_set_peer_data((*new_alist).addrs[n].peer, data); n += 1; }
    while o < (*old_alist).nr_addrs as usize { rxrpc_kernel_set_peer_data((*old_alist).addrs[o].peer, 0); o += 1; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
