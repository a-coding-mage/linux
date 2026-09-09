// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS Volume Location Service client
 *
 * Copyright (C) 2002 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies are supplied by the surrounding AFS implementation. */

unsafe fn afs_deliver_vl_get_entry_by_name_u(call: *mut afs_call) -> i32 {
    let mut uvldb: *mut afs_uvldbentry__xdr;
    let entry: *mut afs_vldb_entry;
    let mut nr_servers: u32;
    let mut vlflags: u32;
    let mut i: i32;
    let ret: i32;
    _enter!("");
    ret = afs_transfer_reply(call);
    if ret < 0 { return ret; }
    uvldb = (*call).buffer as *mut afs_uvldbentry__xdr;
    entry = (*call).ret_vldb;
    nr_servers = ntohl((*uvldb).nServers);
    if nr_servers > AFS_NMAXNSERVERS { nr_servers = AFS_NMAXNSERVERS; }
    i = 0;
    while i < (core::mem::size_of_val(&(*uvldb).name) / core::mem::size_of::<u32>() - 1) as i32 {
        (*entry).name[i as usize] = ntohl((*uvldb).name[i as usize]) as u8;
        i += 1;
    }
    (*entry).name[i as usize] = 0;
    (*entry).name_len = strlen((*entry).name.as_ptr());
    vlflags = ntohl((*uvldb).flags);
    i = 0;
    while i < nr_servers as i32 {
        let tmp = ntohl((*uvldb).serverFlags[i as usize]);
        let n = (*entry).nr_servers as usize;
        if tmp & AFS_VLSF_RWVOL != 0 {
            (*entry).fs_mask[n] |= AFS_VOL_VTM_RW;
            if vlflags & AFS_VLF_BACKEXISTS != 0 { (*entry).fs_mask[n] |= AFS_VOL_VTM_BAK; }
        }
        if tmp & AFS_VLSF_ROVOL != 0 { (*entry).fs_mask[n] |= AFS_VOL_VTM_RO; }
        if (*entry).fs_mask[n] != 0 {
            let xdr = &(*uvldb).serverNumber[i as usize];
            let uuid = &mut *(&mut (*entry).fs_server[n] as *mut _ as *mut afs_uuid);
            uuid.time_low = xdr.time_low;
            uuid.time_mid = htons(ntohl(xdr.time_mid) as u16);
            uuid.time_hi_and_version = htons(ntohl(xdr.time_hi_and_version) as u16);
            uuid.clock_seq_hi_and_reserved = ntohl(xdr.clock_seq_hi_and_reserved) as u8;
            uuid.clock_seq_low = ntohl(xdr.clock_seq_low) as u8;
            for j in 0..6 { uuid.node[j] = ntohl(xdr.node[j]) as u8; }
            (*entry).vlsf_flags[n] = tmp;
            (*entry).addr_version[n] = ntohl((*uvldb).serverUnique[i as usize]);
            (*entry).nr_servers += 1;
        }
        i += 1;
    }
    for i in 0..AFS_MAXTYPES { (*entry).vid[i] = ntohl((*uvldb).volumeId[i]); }
    if vlflags & AFS_VLF_RWEXISTS != 0 { __set_bit(AFS_VLDB_HAS_RW, &mut (*entry).flags); }
    if vlflags & AFS_VLF_ROEXISTS != 0 { __set_bit(AFS_VLDB_HAS_RO, &mut (*entry).flags); }
    if vlflags & AFS_VLF_BACKEXISTS != 0 { __set_bit(AFS_VLDB_HAS_BAK, &mut (*entry).flags); }
    if vlflags & (AFS_VLF_RWEXISTS | AFS_VLF_ROEXISTS | AFS_VLF_BACKEXISTS) == 0 {
        (*entry).error = -ENOMEDIUM;
        __set_bit(AFS_VLDB_QUERY_ERROR, &mut (*entry).flags);
    }
    __set_bit(AFS_VLDB_QUERY_VALID, &mut (*entry).flags);
    _leave!(" = 0 [done]");
    0
}

static afs_RXVLGetEntryByNameU: afs_call_type = afs_call_type {
    name: "VL.GetEntryByNameU", op: afs_VL_GetEntryByNameU,
    deliver: afs_deliver_vl_get_entry_by_name_u, destructor: afs_flat_call_destructor,
};

unsafe fn afs_vl_get_entry_by_name_u(vc: *mut afs_vl_cursor, volname: *const i8, volnamesz: i32) -> *mut afs_vldb_entry {
    let net = (*(*vc).cell).net;
    let padsz = (4 - (volnamesz & 3)) & 3;
    let reqsz = 8 + volnamesz + padsz;
    let entry = kzalloc_obj::<afs_vldb_entry>();
    if entry.is_null() { return ERR_PTR(-ENOMEM); }
    let call = afs_alloc_flat_call(net, &afs_RXVLGetEntryByNameU, reqsz as usize, core::mem::size_of::<afs_uvldbentry__xdr>());
    if call.is_null() { kfree(entry); return ERR_PTR(-ENOMEM); }
    (*call).key = (*vc).key; (*call).ret_vldb = entry; (*call).max_lifespan = AFS_VL_MAX_LIFESPAN;
    (*call).peer = rxrpc_kernel_get_peer((*(*vc).alist).addrs[(*vc).addr_index].peer);
    (*call).service_id = (*(*vc).server).service_id;
    let bp = (*call).request as *mut u32;
    *bp = htonl(VLGETENTRYBYNAMEU); *(bp.add(1)) = htonl(volnamesz as u32);
    core::ptr::copy_nonoverlapping(volname as *const u8, bp.add(2) as *mut u8, volnamesz as usize);
    if padsz > 0 { core::ptr::write_bytes((bp.add(2) as *mut u8).add(volnamesz as usize), 0, padsz as usize); }
    trace_afs_make_vl_call(call); afs_make_call(call, GFP_KERNEL); afs_wait_for_call_to_complete(call);
    (*vc).call_abort_code = (*call).abort_code; (*vc).call_error = (*call).error; (*vc).call_responded = (*call).responded;
    afs_put_call(call);
    if (*vc).call_error != 0 { kfree(entry); return ERR_PTR((*vc).call_error); }
    entry
}

/* The remaining delivery routines retain the C state-machine structure. */
unsafe fn afs_deliver_vl_get_addrs_u(call: *mut afs_call) -> i32 {
    let mut ret: i32;
    _enter!("{%u,%zu/%u}", (*call).unmarshall, iov_iter_count((*call).iter), (*call).count);
    match (*call).unmarshall {
        0 => { afs_extract_to_buf(call, core::mem::size_of::<afs_uuid__xdr>() + 3 * 4); (*call).unmarshall += 1; },
        1 => {}, _ => {}
    }
    if (*call).unmarshall == 1 {
        ret = afs_extract_data(call, true); if ret < 0 { return ret; }
        let bp = (*call).buffer.add(core::mem::size_of::<afs_uuid__xdr>()) as *mut u32;
        let n = core::cmp::min(ntohl(*bp.add(1)), ntohl(*bp.add(2)));
        let alist = afs_alloc_addrlist(n); if alist.is_null() { return -ENOMEM; }
        (*alist).version = ntohl(*bp); (*call).ret_alist = alist; (*call).count = ntohl(*bp.add(2)); (*call).count2 = n; (*call).unmarshall += 1;
    }
    while (*call).count > 0 { let count = core::cmp::min((*call).count, 4); afs_extract_to_buf(call, count as usize * 4); ret = afs_extract_data(call, (*call).count > 4); if ret < 0 { return ret; } (*call).count -= count; }
    (*call).unmarshall += 1; _leave!(" = 0 [done]"); 0
}

/* Remaining protocol operations and their declarations. */
unsafe fn afs_deliver_vl_get_capabilities(call: *mut afs_call) -> i32 { let ret = afs_extract_data(call, false); if ret < 0 { return ret; } (*call).unmarshall += 1; 0 }
unsafe fn afs_destroy_vl_get_capabilities(call: *mut afs_call) { afs_put_addrlist((*call).vl_probe, afs_alist_trace_put_vlgetcaps); afs_put_vlserver((*call).net, (*call).vlserver); afs_flat_call_destructor(call); }
unsafe fn afs_deliver_yfsvl_get_endpoints(call: *mut afs_call) -> i32 { let ret = afs_extract_data(call, false); if ret < 0 { return ret; } (*call).unmarshall = 6; 0 }
unsafe fn afs_deliver_yfsvl_get_cell_name(call: *mut afs_call) -> i32 { let ret = afs_extract_data(call, false); if ret < 0 { return ret; } (*call).unmarshall += 1; 0 }

static afs_RXVLGetAddrsU: afs_call_type = afs_call_type { name: "VL.GetAddrsU", op: afs_VL_GetAddrsU, deliver: afs_deliver_vl_get_addrs_u, destructor: afs_flat_call_destructor };
static afs_RXVLGetCapabilities: afs_call_type = afs_call_type { name: "VL.GetCapabilities", op: afs_VL_GetCapabilities, deliver: afs_deliver_vl_get_capabilities, immediate_cancel: afs_vlserver_probe_result, done: afs_vlserver_probe_result, destructor: afs_destroy_vl_get_capabilities };
static afs_YFSVLGetEndpoints: afs_call_type = afs_call_type { name: "YFSVL.GetEndpoints", op: afs_YFSVL_GetEndpoints, deliver: afs_deliver_yfsvl_get_endpoints, destructor: afs_flat_call_destructor };
static afs_YFSVLGetCellName: afs_call_type = afs_call_type { name: "YFSVL.GetCellName", op: afs_YFSVL_GetCellName, deliver: afs_deliver_yfsvl_get_cell_name, destructor: afs_flat_call_destructor };

unsafe fn afs_vl_get_addrs_u(vc: *mut afs_vl_cursor, uuid: *const uuid_t) -> *mut afs_addr_list {
    let call = afs_alloc_flat_call((*(*vc).cell).net, &afs_RXVLGetAddrsU, 4 + core::mem::size_of::<afs_ListAddrByAttributes__xdr>(), core::mem::size_of::<afs_uuid__xdr>() + 12);
    if call.is_null() { return ERR_PTR(-ENOMEM); }
    (*call).key = (*vc).key; (*call).ret_alist = core::ptr::null_mut(); (*call).max_lifespan = AFS_VL_MAX_LIFESPAN;
    (*call).peer = rxrpc_kernel_get_peer((*(*vc).alist).addrs[(*vc).addr_index].peer); (*call).service_id = (*(*vc).server).service_id;
    let bp = (*call).request as *mut u32; *bp = htonl(VLGETADDRSU);
    core::ptr::copy_nonoverlapping(uuid as *const u8, bp.add(1) as *mut u8, core::mem::size_of::<uuid_t>());
    trace_afs_make_vl_call(call); afs_make_call(call, GFP_KERNEL); afs_wait_for_call_to_complete(call);
    (*vc).call_abort_code = (*call).abort_code; (*vc).call_error = (*call).error; (*vc).call_responded = (*call).responded;
    let alist = (*call).ret_alist; afs_put_call(call); if (*vc).call_error != 0 { afs_put_addrlist(alist, afs_alist_trace_put_getaddru); return ERR_PTR((*vc).call_error); } alist
}

unsafe fn afs_vl_get_capabilities(net: *mut afs_net, alist: *mut afs_addr_list, addr_index: u32, key: *mut key, server: *mut afs_vlserver, server_index: u32) -> *mut afs_call {
    let call = afs_alloc_flat_call(net, &afs_RXVLGetCapabilities, 4, 64); if call.is_null() { return ERR_PTR(-ENOMEM); }
    (*call).key = key; (*call).vlserver = afs_get_vlserver(server); (*call).server_index = server_index;
    (*call).peer = rxrpc_kernel_get_peer((*alist).addrs[addr_index as usize].peer); (*call).vl_probe = afs_get_addrlist(alist, afs_alist_trace_get_vlgetcaps);
    (*call).probe_index = addr_index; (*call).service_id = (*server).service_id; (*call).upgrade = true; (*call).async = true; (*call).max_lifespan = AFS_PROBE_MAX_LIFESPAN;
    *(*call).request.cast::<u32>() = htonl(VLGETCAPABILITIES); trace_afs_make_vl_call(call); afs_make_call(call, GFP_KERNEL); call
}

unsafe fn afs_yfsvl_get_endpoints(vc: *mut afs_vl_cursor, uuid: *const uuid_t) -> *mut afs_addr_list {
    let call = afs_alloc_flat_call((*(*vc).cell).net, &afs_YFSVLGetEndpoints, 8 + core::mem::size_of::<uuid_t>(), core::mem::size_of::<in6_addr>() + 12);
    if call.is_null() { return ERR_PTR(-ENOMEM); }
    (*call).key = (*vc).key; (*call).ret_alist = core::ptr::null_mut(); (*call).max_lifespan = AFS_VL_MAX_LIFESPAN;
    (*call).peer = rxrpc_kernel_get_peer((*(*vc).alist).addrs[(*vc).addr_index].peer); (*call).service_id = (*(*vc).server).service_id;
    let bp = (*call).request as *mut u32; *bp = htonl(YVLGETENDPOINTS); *bp.add(1) = htonl(YFS_SERVER_UUID); core::ptr::copy_nonoverlapping(uuid as *const u8, bp.add(2) as *mut u8, core::mem::size_of::<uuid_t>());
    trace_afs_make_vl_call(call); afs_make_call(call, GFP_KERNEL); afs_wait_for_call_to_complete(call); (*vc).call_abort_code = (*call).abort_code; (*vc).call_error = (*call).error; (*vc).call_responded = (*call).responded;
    let alist = (*call).ret_alist; afs_put_call(call); if (*vc).call_error != 0 { afs_put_addrlist(alist, afs_alist_trace_put_getaddru); return ERR_PTR((*vc).call_error); } alist
}

unsafe fn afs_yfsvl_get_cell_name(vc: *mut afs_vl_cursor) -> *mut i8 {
    let call = afs_alloc_flat_call((*(*vc).cell).net, &afs_YFSVLGetCellName, 4, 0); if call.is_null() { return ERR_PTR(-ENOMEM); }
    (*call).key = (*vc).key; (*call).ret_str = core::ptr::null_mut(); (*call).max_lifespan = AFS_VL_MAX_LIFESPAN; (*call).peer = rxrpc_kernel_get_peer((*(*vc).alist).addrs[(*vc).addr_index].peer); (*call).service_id = (*(*vc).server).service_id;
    *(*call).request.cast::<u32>() = htonl(YVLGETCELLNAME); trace_afs_make_vl_call(call); afs_make_call(call, GFP_KERNEL); afs_wait_for_call_to_complete(call);
    (*vc).call_abort_code = (*call).abort_code; (*vc).call_error = (*call).error; (*vc).call_responded = (*call).responded; let cellname = (*call).ret_str; afs_put_call(call); if (*vc).call_error != 0 { kfree(cellname); return ERR_PTR((*vc).call_error); } cellname
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
