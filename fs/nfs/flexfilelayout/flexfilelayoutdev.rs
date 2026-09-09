// SPDX-License-Identifier: GPL-2.0
/*
 * Device operations for the pnfs nfs4 file layout driver.
 *
 * Copyright (c) 2014, Primary Data, Inc. All rights reserved.
 *
 * Tao Peng <bergwolf@primarydata.com>
 */

// External Linux/NFS dependencies are supplied by the surrounding translation.

const NFSDBG_FACILITY: u32 = NFSDBG_PNFS_LD;

static mut dataserver_timeo: c_uint = NFS_DEF_TCP_TIMEO;
static mut dataserver_retrans: c_uint = 0;

extern "C" {
    fn ff_layout_has_available_ds(lseg: *mut pnfs_layout_segment) -> bool;
}

pub unsafe fn nfs4_ff_layout_put_deviceid(mirror_ds: *mut nfs4_ff_layout_ds) {
    if !IS_ERR_OR_NULL(mirror_ds) {
        nfs4_put_deviceid_node(&mut (*mirror_ds).id_node);
    }
}

pub unsafe fn nfs4_ff_layout_free_deviceid(mirror_ds: *mut nfs4_ff_layout_ds) {
    nfs4_print_deviceid(&(*mirror_ds).id_node.deviceid);
    nfs4_pnfs_ds_put((*mirror_ds).ds);
    kfree((*mirror_ds).ds_versions);
    kfree_rcu(mirror_ds, id_node.rcu);
}

/* Decode opaque device data and construct new_ds using it */
pub unsafe fn nfs4_ff_alloc_deviceid_node(
    server: *mut nfs_server,
    pdev: *mut pnfs_device,
    gfp_flags: gfp_t,
) -> *mut nfs4_ff_layout_ds {
    let mut stream: xdr_stream = core::mem::zeroed();
    let mut buf: xdr_buf = core::mem::zeroed();
    let mut scratch: *mut folio;
    let mut dsaddrs: list_head = core::mem::zeroed();
    let mut da: *mut nfs4_pnfs_ds_addr;
    let mut new_ds: *mut nfs4_ff_layout_ds = core::ptr::null_mut();
    let mut ds_versions: *mut nfs4_ff_ds_version = core::ptr::null_mut();
    let net: *mut net = (*(*server).nfs_client).cl_net;
    let mut mp_count: u32;
    let mut version_count: u32;
    let mut p: *mut __be32;
    let mut ret: c_int = -ENOMEM;

    scratch = folio_alloc(gfp_flags, 0);
    if scratch.is_null() { return core::ptr::null_mut(); }

    new_ds = kzalloc_obj::<nfs4_ff_layout_ds>(gfp_flags);
    if new_ds.is_null() { folio_put(scratch); return core::ptr::null_mut(); }

    nfs4_init_deviceid_node(&mut (*new_ds).id_node, server, &(*pdev).dev_id);
    INIT_LIST_HEAD(&mut dsaddrs);
    xdr_init_decode_pages(&mut stream, &mut buf, (*pdev).pages, (*pdev).pglen);
    xdr_set_scratch_folio(&mut stream, scratch);

    p = xdr_inline_decode(&mut stream, 4);
    if p.is_null() { goto_out_err_drain_dsaddrs(new_ds, ds_versions, scratch, &mut dsaddrs, ret); }
    mp_count = be32_to_cpup(p);
    dprintk("%s: multipath ds count %d\n", __func__, mp_count);

    for _i in 0..mp_count {
        da = nfs4_decode_mp_ds_addr(net, &mut stream, gfp_flags);
        if !da.is_null() { list_add_tail(&mut (*da).da_node, &mut dsaddrs); }
    }
    if list_empty(&dsaddrs) {
        dprintk("%s: no suitable DS addresses found\n", __func__);
        ret = -ENOMEDIUM;
        goto_out_err_drain_dsaddrs(new_ds, ds_versions, scratch, &mut dsaddrs, ret);
    }

    p = xdr_inline_decode(&mut stream, 4);
    if p.is_null() { goto_out_err_drain_dsaddrs(new_ds, ds_versions, scratch, &mut dsaddrs, ret); }
    version_count = be32_to_cpup(p);
    if version_count == 0 {
        ret = -EINVAL;
        goto_out_err_drain_dsaddrs(new_ds, ds_versions, scratch, &mut dsaddrs, ret);
    }
    dprintk("%s: version count %d\n", __func__, version_count);
    ds_versions = kzalloc_objs::<nfs4_ff_ds_version>(version_count, gfp_flags);
    if ds_versions.is_null() { goto_out_err_drain_dsaddrs(new_ds, ds_versions, scratch, &mut dsaddrs, ret); }

    for i in 0..version_count {
        p = xdr_inline_decode(&mut stream, 20);
        if p.is_null() { goto_out_err_drain_dsaddrs(new_ds, ds_versions, scratch, &mut dsaddrs, ret); }
        (*ds_versions.add(i as usize)).version = be32_to_cpup(p); p = p.add(1);
        (*ds_versions.add(i as usize)).minor_version = be32_to_cpup(p); p = p.add(1);
        (*ds_versions.add(i as usize)).rsize = nfs_io_size(be32_to_cpup(p), (*(*server).nfs_client).cl_proto); p = p.add(1);
        (*ds_versions.add(i as usize)).wsize = nfs_io_size(be32_to_cpup(p), (*(*server).nfs_client).cl_proto); p = p.add(1);
        (*ds_versions.add(i as usize)).tightly_coupled = be32_to_cpup(p);
        if (*ds_versions.add(i as usize)).rsize > NFS_MAX_FILE_IO_SIZE { (*ds_versions.add(i as usize)).rsize = NFS_MAX_FILE_IO_SIZE; }
        if (*ds_versions.add(i as usize)).wsize > NFS_MAX_FILE_IO_SIZE { (*ds_versions.add(i as usize)).wsize = NFS_MAX_FILE_IO_SIZE; }
        if !((*ds_versions.add(i as usize)).version == 3 && (*ds_versions.add(i as usize)).minor_version == 0 ||
             (*ds_versions.add(i as usize)).version == 4 && (*ds_versions.add(i as usize)).minor_version < 3) {
            dprintk("%s: [%d] unsupported ds version %d-%d\n", __func__, i, (*ds_versions.add(i as usize)).version, (*ds_versions.add(i as usize)).minor_version);
            ret = -EPROTONOSUPPORT;
            goto_out_err_drain_dsaddrs(new_ds, ds_versions, scratch, &mut dsaddrs, ret);
        }
        dprintk("%s: [%d] vers %u minor_ver %u rsize %u wsize %u coupled %d\n", __func__, i,
            (*ds_versions.add(i as usize)).version, (*ds_versions.add(i as usize)).minor_version,
            (*ds_versions.add(i as usize)).rsize, (*ds_versions.add(i as usize)).wsize,
            (*ds_versions.add(i as usize)).tightly_coupled);
    }
    (*new_ds).ds_versions = ds_versions;
    (*new_ds).ds_versions_cnt = version_count;
    (*new_ds).ds = nfs4_pnfs_ds_add(net, &mut dsaddrs, (*ds_versions).version, gfp_flags);
    if (*new_ds).ds.is_null() { goto_out_err_drain_dsaddrs(new_ds, ds_versions, scratch, &mut dsaddrs, ret); }
    while !list_empty(&dsaddrs) {
        da = list_first_entry(&mut dsaddrs, nfs4_pnfs_ds_addr, da_node);
        list_del_init(&mut (*da).da_node); kfree((*da).da_remotestr); kfree(da);
    }
    folio_put(scratch); new_ds
}

unsafe fn extend_ds_error(err: *mut nfs4_ff_layout_ds_err, offset: u64, length: u64) {
    let end = max_t::<u64>(pnfs_end_offset((*err).offset, (*err).length), pnfs_end_offset(offset, length));
    (*err).offset = min_t::<u64>((*err).offset, offset);
    (*err).length = end - (*err).offset;
}

unsafe fn ff_ds_error_match(e1: *const nfs4_ff_layout_ds_err, e2: *const nfs4_ff_layout_ds_err) -> c_int {
    if (*e1).opnum != (*e2).opnum { return if (*e1).opnum < (*e2).opnum { -1 } else { 1 }; }
    if (*e1).status != (*e2).status { return if (*e1).status < (*e2).status { -1 } else { 1 }; }
    let mut ret = memcmp((*e1).stateid.data.as_ptr() as *const c_void, (*e2).stateid.data.as_ptr() as *const c_void, core::mem::size_of_val(&(*e1).stateid.data));
    if ret != 0 { return ret; }
    ret = memcmp(&(*e1).deviceid as *const _ as *const c_void, &(*e2).deviceid as *const _ as *const c_void, core::mem::size_of_val(&(*e1).deviceid));
    if ret != 0 { return ret; }
    if pnfs_end_offset((*e1).offset, (*e1).length) < (*e2).offset { return -1; }
    if (*e1).offset > pnfs_end_offset((*e2).offset, (*e2).length) { return 1; }
    0
}

unsafe fn ff_layout_add_ds_error_locked(flo: *mut nfs4_flexfile_layout, dserr: *mut nfs4_ff_layout_ds_err) {
    let mut head = &mut (*flo).error_list as *mut list_head;
    let mut err: *mut nfs4_ff_layout_ds_err;
    let mut tmp: *mut nfs4_ff_layout_ds_err;
    list_for_each_entry_safe!(err, tmp, &mut (*flo).error_list, list, {
        let m = ff_ds_error_match(err, dserr);
        if m < 0 { continue; }
        if m > 0 { head = &mut (*err).list; break; }
        extend_ds_error(dserr, (*err).offset, (*err).length);
        list_replace(&mut (*err).list, &mut (*dserr).list); kfree(err); return;
    });
    list_add_tail(&mut (*dserr).list, head);
}

pub unsafe fn ff_layout_track_ds_error(flo: *mut nfs4_flexfile_layout, mirror: *mut nfs4_ff_layout_mirror, dss_id: u32, offset: u64, length: u64, status: c_int, opnum: enum_nfs_opnum4, gfp_flags: gfp_t) -> c_int {
    if status == 0 { return 0; }
    if IS_ERR_OR_NULL((*mirror).dss[dss_id as usize].mirror_ds) { return -EINVAL; }
    let dserr = kmalloc_obj::<nfs4_ff_layout_ds_err>(gfp_flags);
    if dserr.is_null() { return -ENOMEM; }
    INIT_LIST_HEAD(&mut (*dserr).list); (*dserr).offset = offset; (*dserr).length = length; (*dserr).status = status; (*dserr).opnum = opnum;
    nfs4_stateid_copy(&mut (*dserr).stateid, &(*mirror).dss[dss_id as usize].stateid);
    memcpy(&mut (*dserr).deviceid as *mut _ as *mut c_void, &(*(*mirror).dss[dss_id as usize].mirror_ds).id_node.deviceid as *const _ as *const c_void, NFS4_DEVICEID4_SIZE);
    spin_lock(&mut (*(*flo).generic_hdr.plh_inode).i_lock); ff_layout_add_ds_error_locked(flo, dserr); spin_unlock(&mut (*(*flo).generic_hdr.plh_inode).i_lock); 0
}

pub unsafe fn nfs4_ff_layout_select_ds_fh(mirror: *mut nfs4_ff_layout_mirror, dss_id: u32) -> *mut nfs_fh { &mut (*mirror).dss[dss_id as usize].fh_versions[0] }
pub unsafe fn nfs4_ff_layout_select_ds_stateid(mirror: *const nfs4_ff_layout_mirror, dss_id: u32, stateid: *mut nfs4_stateid) { if nfs4_ff_layout_ds_version(mirror, dss_id) == 4 { nfs4_stateid_copy(stateid, &(*mirror).dss[dss_id as usize].stateid); } }

pub unsafe fn ff_layout_free_ds_ioerr(head: *mut list_head) { while !list_empty(head) { let err = list_first_entry(head, nfs4_ff_layout_ds_err, list); list_del(&mut (*err).list); kfree(err); } }

pub unsafe fn ff_layout_fetch_ds_ioerr(lo: *mut pnfs_layout_hdr, range: *const pnfs_layout_range, head: *mut list_head, maxnum: c_uint) -> c_uint { let ret = do_layout_fetch_ds_ioerr(lo, range, head, maxnum); if ret == maxnum { let mut discard: list_head = core::mem::zeroed(); INIT_LIST_HEAD(&mut discard); do_layout_fetch_ds_ioerr(lo, range, &mut discard, u32::MAX); ff_layout_free_ds_ioerr(&mut discard); } ret }

unsafe fn ff_read_layout_has_available_ds(lseg: *mut pnfs_layout_segment) -> bool { for idx in 0..FF_LAYOUT_MIRROR_COUNT(lseg) { let mirror = FF_LAYOUT_COMP(lseg, idx); if mirror.is_null() { continue; } for dss_id in 0..(*mirror).dss_count { let ds = (*mirror).dss[dss_id as usize].mirror_ds; if ds.is_null() || !IS_ERR(ds) && !nfs4_test_deviceid_unavailable(&mut (*ds).id_node) { return true; } } } false }
unsafe fn ff_rw_layout_has_available_ds(lseg: *mut pnfs_layout_segment) -> bool { for idx in 0..FF_LAYOUT_MIRROR_COUNT(lseg) { let mirror = FF_LAYOUT_COMP(lseg, idx); if mirror.is_null() { return false; } for dss_id in 0..(*mirror).dss_count { let ds = (*mirror).dss[dss_id as usize].mirror_ds; if IS_ERR(ds) || !ds.is_null() && nfs4_test_deviceid_unavailable(&mut (*ds).id_node) { return false; } } } FF_LAYOUT_MIRROR_COUNT(lseg) != 0 }
unsafe fn ff_layout_has_available_ds_local(lseg: *mut pnfs_layout_segment) -> bool { if (*lseg).pls_range.iomode == IOMODE_READ { ff_read_layout_has_available_ds(lseg) } else { ff_rw_layout_has_available_ds(lseg) } }
pub unsafe fn ff_layout_avoid_mds_available_ds(lseg: *mut pnfs_layout_segment) -> bool { ff_layout_no_fallback_to_mds(lseg) || ff_layout_has_available_ds_local(lseg) }
pub unsafe fn ff_layout_avoid_read_on_rw(lseg: *mut pnfs_layout_segment) -> bool { (*lseg).pls_range.iomode == IOMODE_RW && ff_layout_no_read_on_rw(lseg) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
