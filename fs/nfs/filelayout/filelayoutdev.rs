/*
 *  Device operations for the pnfs nfs4 file layout driver.
 *
 *  Copyright (c) 2002
 *  The Regents of the University of Michigan
 *  All Rights Reserved
 *
 *  Dean Hildebrand <dhildebz@umich.edu>
 *  Garth Goodson   <Garth.Goodson@netapp.com>
 *
 *  Permission is granted to use, copy, create derivative works, and
 *  redistribute this software and such derivative works for any purpose,
 *  so long as the name of the University of Michigan is not used in any
 *  advertising or publicity pertaining to the use or distribution
 *  of this software without specific, written prior authorization.
 *
 *  This software is provided as is, without representation or warranty
 *  of any kind either express or implied.
 */

// C dependencies supplied by the surrounding kernel translation unit.
// #include <linux/nfs_fs.h>
// #include <linux/vmalloc.h>
// #include <linux/module.h>
// #include "../internal.h"
// #include "../nfs4session.h"
// #include "filelayout.h"
// #include "../nfs4trace.h"

// #define NFSDBG_FACILITY NFSDBG_PNFS_LD

static mut dataserver_timeo: c_uint = NFS4_DEF_DS_TIMEO;
static mut dataserver_retrans: c_uint = NFS4_DEF_DS_RETRANS;

pub unsafe fn nfs4_fl_free_deviceid(dsaddr: *mut nfs4_file_layout_dsaddr) {
    let mut ds: *mut nfs4_pnfs_ds;
    let mut i: c_int;

    nfs4_print_deviceid(&(*dsaddr).id_node.deviceid);

    i = 0;
    while i < (*dsaddr).ds_num as c_int {
        ds = *(*dsaddr).ds_list.as_mut_ptr().add(i as usize);
        if !ds.is_null() {
            nfs4_pnfs_ds_put(ds);
        }
        i += 1;
    }
    kfree((*dsaddr).stripe_indices as *mut c_void);
    kfree_rcu(dsaddr, id_node.rcu);
}

/* Decode opaque device data and return the result */
pub unsafe fn nfs4_fl_alloc_deviceid_node(
    server: *mut nfs_server,
    pdev: *mut pnfs_device,
    gfp_flags: gfp_t,
) -> *mut nfs4_file_layout_dsaddr {
    let mut i: c_int;
    let mut cnt: u32;
    let mut num: u32;
    let mut indexp: *mut u8;
    let mut p: *mut __be32;
    let mut stripe_indices: *mut u8;
    let mut max_stripe_index: u8;
    let mut dsaddr: *mut nfs4_file_layout_dsaddr = core::ptr::null_mut();
    let mut stream: xdr_stream = core::mem::zeroed();
    let mut buf: xdr_buf = core::mem::zeroed();
    let mut scratch: *mut folio;
    let mut dsaddrs: list_head = core::mem::zeroed();
    let mut da: *mut nfs4_pnfs_ds_addr;
    let net: *mut net = (*(*server).nfs_client).cl_net;

    scratch = folio_alloc(gfp_flags, 0);
    if scratch.is_null() { return core::ptr::null_mut(); }

    xdr_init_decode_pages(&mut stream, &mut buf, (*pdev).pages, (*pdev).pglen);
    xdr_set_scratch_folio(&mut stream, scratch);

    p = xdr_inline_decode(&mut stream, 4);
    if p.is_null() { folio_put(scratch); return core::ptr::null_mut(); }

    cnt = be32_to_cpup(p);
    dprintk!("%s stripe count  %d\n", __func__, cnt);
    if cnt > NFS4_PNFS_MAX_STRIPE_CNT {
        printk!(KERN_WARNING "NFS: %s: stripe count %d greater than supported maximum %d\n", __func__, cnt, NFS4_PNFS_MAX_STRIPE_CNT);
        folio_put(scratch); return core::ptr::null_mut();
    }

    stripe_indices = kcalloc(cnt as usize, core::mem::size_of::<u8>(), gfp_flags) as *mut u8;
    if stripe_indices.is_null() { folio_put(scratch); return core::ptr::null_mut(); }

    p = xdr_inline_decode(&mut stream, cnt << 2);
    if p.is_null() { kfree(stripe_indices as *mut c_void); folio_put(scratch); return core::ptr::null_mut(); }

    indexp = stripe_indices;
    max_stripe_index = 0;
    i = 0;
    while i < cnt as c_int {
        *indexp = be32_to_cpup(p) as u8;
        p = p.add(1);
        max_stripe_index = core::cmp::max(max_stripe_index, *indexp);
        indexp = indexp.add(1);
        i += 1;
    }

    p = xdr_inline_decode(&mut stream, 4);
    if p.is_null() { kfree(stripe_indices as *mut c_void); folio_put(scratch); return core::ptr::null_mut(); }
    num = be32_to_cpup(p);
    dprintk!("%s ds_num %u\n", __func__, num);
    if num > NFS4_PNFS_MAX_MULTI_CNT || max_stripe_index >= num as u8 {
        printk!(KERN_WARNING "NFS: %s: invalid multipath/stripe count\n", __func__);
        kfree(stripe_indices as *mut c_void); folio_put(scratch); return core::ptr::null_mut();
    }

    dsaddr = kzalloc_flex!(nfs4_file_layout_dsaddr, ds_list, num, gfp_flags);
    if dsaddr.is_null() { kfree(stripe_indices as *mut c_void); folio_put(scratch); return core::ptr::null_mut(); }
    (*dsaddr).stripe_count = cnt;
    (*dsaddr).stripe_indices = stripe_indices;
    nfs4_init_deviceid_node(&mut (*dsaddr).id_node, server, &(*pdev).dev_id);
    INIT_LIST_HEAD(&mut dsaddrs);

    i = 0;
    while i < (*dsaddr).ds_num as c_int {
        let mut j: c_int;
        let mut mp_count: u32;
        p = xdr_inline_decode(&mut stream, 4);
        if p.is_null() { nfs4_fl_free_deviceid(dsaddr); folio_put(scratch); return core::ptr::null_mut(); }
        mp_count = be32_to_cpup(p);
        j = 0;
        while j < mp_count as c_int {
            da = nfs4_decode_mp_ds_addr(net, &mut stream, gfp_flags);
            if !da.is_null() { list_add_tail(&mut (*da).da_node, &mut dsaddrs); }
            j += 1;
        }
        if list_empty(&dsaddrs) { nfs4_fl_free_deviceid(dsaddr); folio_put(scratch); return core::ptr::null_mut(); }
        (*dsaddr).ds_list[i as usize] = nfs4_pnfs_ds_add(net, &mut dsaddrs, 4, gfp_flags);
        if (*dsaddr).ds_list[i as usize].is_null() { nfs4_fl_free_deviceid(dsaddr); folio_put(scratch); return core::ptr::null_mut(); }
        trace_fl_getdevinfo!(server, &(*pdev).dev_id, (*(*dsaddr).ds_list[i as usize]).ds_remotestr);
        while !list_empty(&dsaddrs) {
            da = list_first_entry!(dsaddrs, nfs4_pnfs_ds_addr, da_node);
            list_del_init(&mut (*da).da_node);
            kfree((*da).da_remotestr as *mut c_void);
            kfree(da as *mut c_void);
        }
        i += 1;
    }
    folio_put(scratch);
    dsaddr
}

pub unsafe fn nfs4_fl_put_deviceid(dsaddr: *mut nfs4_file_layout_dsaddr) {
    nfs4_put_deviceid_node(&mut (*dsaddr).id_node);
}

/* Want res = (offset - layout->pattern_offset) / layout->stripe_unit */
pub unsafe fn nfs4_fl_calc_j_index(lseg: *mut pnfs_layout_segment, offset: loff_t) -> u32 {
    let flseg = FILELAYOUT_LSEG!(lseg);
    let mut tmp = offset.wrapping_sub((*flseg).pattern_offset) as u64;
    tmp = tmp.wrapping_div((*flseg).stripe_unit);
    tmp = tmp.wrapping_add((*flseg).first_stripe_index as u64);
    (tmp % (*(*flseg).dsaddr).stripe_count as u64) as u32
}

pub unsafe fn nfs4_fl_calc_ds_index(lseg: *mut pnfs_layout_segment, j: u32) -> u8 {
    let flseg = FILELAYOUT_LSEG!(lseg);
    (*flseg).dsaddr.as_ref().unwrap().stripe_indices.add(j as usize).read()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
