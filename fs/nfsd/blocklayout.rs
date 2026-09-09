// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2014-2016 Christoph Hellwig. */
// Linux headers and local headers from the C source are external dependencies.

const NFSDDBG_FACILITY: u32 = NFSDDBG_PNFS;

unsafe fn nfsd4_block_map_extent(
    inode: *mut inode, fhp: *const svc_fh, offset: u64, length: u64,
    iomode: u32, minlength: u64, bex: *mut pnfs_block_extent,
) -> __be32 {
    let sb = (*inode).i_sb;
    let mut iomap = core::mem::zeroed::<iomap>();
    let mut device_generation: u32 = 0;
    let error = ((*(*sb).s_export_op).block_ops).map_blocks.unwrap()(inode, offset,
        length, &mut iomap, iomode != IOMODE_READ, &mut device_generation);
    if error != 0 {
        if error == -ENXIO { return nfserr_layoutunavailable; }
        return nfserrno(error);
    }
    match iomap.type_ {
        IOMAP_MAPPED => {
            (*bex).es = if iomode == IOMODE_READ { PNFS_BLOCK_READ_DATA } else { PNFS_BLOCK_READWRITE_DATA };
            (*bex).soff = iomap.addr;
        }
        IOMAP_UNWRITTEN if (iomode & IOMODE_RW) != 0 => {
            if minlength == 0 { dprintk!("pnfsd: no soup for you!\n"); return nfserr_layoutunavailable; }
            (*bex).es = PNFS_BLOCK_INVALID_DATA; (*bex).soff = iomap.addr;
        }
        IOMAP_HOLE if iomode == IOMODE_READ => (*bex).es = PNFS_BLOCK_NONE_DATA,
        _ => { WARN!(true, "pnfsd: filesystem returned {} extent\n", iomap.type_); return nfserr_layoutunavailable; }
    }
    let error = nfsd4_set_deviceid(&mut (*bex).vol_id, fhp, device_generation);
    if error != 0 { return nfserrno(error); }
    (*bex).foff = iomap.offset; (*bex).len = iomap.length; nfs_ok
}

unsafe fn nfsd4_block_proc_layoutget(rqstp: *mut svc_rqst, inode: *mut inode,
    fhp: *const svc_fh, args: *mut nfsd4_layoutget) -> __be32 {
    let seg = &mut (*args).lg_seg;
    let mut offset = seg.offset; let mut length = seg.length;
    let block_size = i_blocksize(inode); let mut nfserr = nfserr_layoutunavailable;
    if locks_in_grace(SVC_NET(rqstp)) { return nfserr_grace; }
    if (seg.offset & (block_size - 1) as u64) != 0 { dprintk!("pnfsd: I/O misaligned\n"); return nfserr; }
    if (*args).lg_maxcount < PNFS_BLOCK_LAYOUT4_SIZE + PNFS_BLOCK_EXTENT_SIZE { seg.length = 0; return nfserr_toosmall; }
    let nr_extents_max = (core::cmp::min((*args).lg_maxcount, PAGE_SIZE) - PNFS_BLOCK_LAYOUT4_SIZE) / PNFS_BLOCK_EXTENT_SIZE;
    let bl = kzalloc_flex::<pnfs_block_layout>(nr_extents_max);
    if bl.is_null() { seg.length = 0; return nfserrno(-ENOMEM); }
    (*bl).nr_extents = nr_extents_max; (*args).lg_content = bl;
    let mut i = 0;
    while i < (*bl).nr_extents {
        let bex = (*bl).extents.add(i); let nf = nfsd4_block_map_extent(inode, fhp, offset, length, seg.iomode, (*args).lg_minlength, bex);
        if nf != nfs_ok { seg.length = 0; return nf; }
        let bex_length = (*bex).len - (offset - (*bex).foff);
        if bex_length >= length { (*bl).nr_extents = i + 1; break; }
        offset = (*bex).foff + (*bex).len; length -= bex_length; i += 1;
    }
    let first = (*bl).extents; let last = (*bl).extents.add((*bl).nr_extents - 1);
    nfserr = nfserr_layoutunavailable;
    length = (*last).foff + (*last).len - seg.offset;
    if length < (*args).lg_minlength { dprintk!("pnfsd: extent smaller than minlength\n"); seg.length = 0; return nfserr; }
    seg.offset = (*first).foff; seg.length = (*last).foff - (*first).foff + (*last).len; nfs_ok
}

unsafe fn nfsd4_block_commit_blocks(inode: *mut inode, lcp: *mut nfsd4_layoutcommit,
    iomaps: *mut iomap, nr_iomaps: i32) -> __be32 {
    let error = ((*(*(*inode).i_sb).s_export_op).block_ops).commit_blocks.unwrap()(inode, iomaps, nr_iomaps,
        if (*lcp).lc_size_chg { (*lcp).lc_newsize } else { 0 });
    kfree(iomaps as *mut core::ffi::c_void); nfserrno(error)
}

#[cfg(CONFIG_NFSD_BLOCKLAYOUT)]
unsafe fn nfsd4_block_get_device_info_simple(sb: *mut super_block, gdp: *mut nfsd4_getdeviceinfo) -> i32 {
    let dev = kzalloc_flex::<pnfs_block_deviceaddr>(1); if dev.is_null() { return -ENOMEM; }
    (*gdp).gd_device = dev; (*dev).nr_volumes = 1; let b = &mut (*dev).volumes[0];
    b.type_ = PNFS_BLOCK_VOLUME_SIMPLE; b.simple.sig_len = PNFS_BLOCK_UUID_LEN;
    ((*(*sb).s_export_op).block_ops).get_uuid.unwrap()(sb, b.simple.sig.as_mut_ptr(), &mut b.simple.sig_len, &mut b.simple.offset)
}

#[cfg(CONFIG_NFSD_BLOCKLAYOUT)]
unsafe fn nfsd4_block_proc_getdeviceinfo(sb: *mut super_block, _rqstp: *mut svc_rqst, _clp: *mut nfs4_client, gdp: *mut nfsd4_getdeviceinfo) -> __be32 {
    if bdev_is_partition((*sb).s_bdev) { return nfserr_inval; }
    nfserrno(nfsd4_block_get_device_info_simple(sb, gdp))
}

#[cfg(CONFIG_NFSD_BLOCKLAYOUT)]
unsafe fn nfsd4_block_proc_layoutcommit(inode: *mut inode, rqstp: *mut svc_rqst, lcp: *mut nfsd4_layoutcommit) -> __be32 {
    (*rqstp).rq_arg = (*lcp).lc_up_layout; svcxdr_init_decode(rqstp);
    let mut iomaps = core::ptr::null_mut(); let mut nr = 0;
    let e = nfsd4_block_decode_layoutupdate(&mut (*rqstp).rq_arg_stream, &mut iomaps, &mut nr, i_blocksize(inode));
    if e != nfs_ok { return e; } nfsd4_block_commit_blocks(inode, lcp, iomaps, nr)
}

#[cfg(CONFIG_NFSD_BLOCKLAYOUT)]
pub static bl_layout_ops: nfsd4_layout_ops = nfsd4_layout_ops {
    notify_types: NOTIFY_DEVICEID4_DELETE | NOTIFY_DEVICEID4_CHANGE,
    proc_getdeviceinfo: Some(nfsd4_block_proc_getdeviceinfo), encode_getdeviceinfo: Some(nfsd4_block_encode_getdeviceinfo),
    proc_layoutget: Some(nfsd4_block_proc_layoutget), encode_layoutget: Some(nfsd4_block_encode_layoutget),
    proc_layoutcommit: Some(nfsd4_block_proc_layoutcommit),
};

#[cfg(CONFIG_NFSD_SCSILAYOUT)]
const NFSD_MDS_PR_FENCED: u32 = XA_MARK_0;
#[cfg(CONFIG_NFSD_SCSILAYOUT)]
const NFSD_MDS_PR_KEY: u64 = 0x0100000000000000;

#[cfg(CONFIG_NFSD_SCSILAYOUT)]
unsafe fn nfsd4_scsi_pr_key(clp: *mut nfs4_client) -> u64 { ((*clp).cl_clientid.cl_boot as u64) << 32 | (*clp).cl_clientid.cl_id as u64 }
#[cfg(CONFIG_NFSD_SCSILAYOUT)]
static DESIGNATOR_TYPES: [u8; 2] = [PS_DESIGNATOR_EUI64, PS_DESIGNATOR_NAA];

#[cfg(CONFIG_NFSD_SCSILAYOUT)]
unsafe fn nfsd4_scsi_fence_insert(clp: *mut nfs4_client, device: dev_t) -> i32 {
    let xa = &mut (*clp).cl_dev_fences; xa_lock(xa);
    let mut ret = __xa_insert(xa, device, XA_ZERO_ENTRY, GFP_KERNEL);
    if ret == -EBUSY { __xa_clear_mark(xa, device, NFSD_MDS_PR_FENCED); ret = 0; }
    xa_unlock(xa); (*clp).cl_fence_retry_warn = false; ret
}
#[cfg(CONFIG_NFSD_SCSILAYOUT)]
unsafe fn nfsd4_scsi_fence_set(clp: *mut nfs4_client, device: dev_t) -> bool {
    let xa = &mut (*clp).cl_dev_fences; xa_lock(xa); let skip = xa_get_mark(xa, device, NFSD_MDS_PR_FENCED);
    if !skip { __xa_set_mark(xa, device, NFSD_MDS_PR_FENCED); } xa_unlock(xa); skip
}
#[cfg(CONFIG_NFSD_SCSILAYOUT)]
unsafe fn nfsd4_scsi_fence_clear(clp: *mut nfs4_client, device: dev_t) { xa_clear_mark(&mut (*clp).cl_dev_fences, device, NFSD_MDS_PR_FENCED); }

#[cfg(CONFIG_NFSD_SCSILAYOUT)]
unsafe fn nfsd4_block_get_device_info_scsi(sb: *mut super_block, clp: *mut nfs4_client, gdp: *mut nfsd4_getdeviceinfo) -> i32 {
    let dev = kzalloc_flex::<pnfs_block_deviceaddr>(1); if dev.is_null() { return -ENOMEM; }
    (*gdp).gd_device = dev; (*dev).nr_volumes = 1; let b = &mut (*dev).volumes[0]; b.type_ = PNFS_BLOCK_VOLUME_SCSI; b.scsi.pr_key = nfsd4_scsi_pr_key(clp);
    let mut ret = nfsd4_block_get_unique_id((*(*sb).s_bdev).bd_disk, b); if ret < 0 { kfree(dev as *mut _); (*gdp).gd_device = core::ptr::null_mut(); return ret; }
    let ops = (*(*(*sb).s_bdev).bd_disk).fops.pr_ops; if ops.is_null() { pr_err!("pNFS: device {} does not support PRs.\n", (*sb).s_id); kfree(dev as *mut _); (*gdp).gd_device = core::ptr::null_mut(); return -EINVAL; }
    ret = nfsd4_scsi_fence_insert(clp, (*(*sb).s_bdev).bd_dev); if ret < 0 { kfree(dev as *mut _); (*gdp).gd_device = core::ptr::null_mut(); return ret; }
    ret = ((*ops).pr_register.unwrap())((*sb).s_bdev, 0, NFSD_MDS_PR_KEY, true); if ret != 0 { pr_err!("pNFS: failed to register key for device {}.\n", (*sb).s_id); kfree(dev as *mut _); (*gdp).gd_device = core::ptr::null_mut(); return ret; }
    ret = ((*ops).pr_reserve.unwrap())((*sb).s_bdev, NFSD_MDS_PR_KEY, PR_EXCLUSIVE_ACCESS_REG_ONLY, 0); if ret != 0 { pr_err!("pNFS: failed to reserve device {}.\n", (*sb).s_id); kfree(dev as *mut _); (*gdp).gd_device = core::ptr::null_mut(); } ret
}
#[cfg(CONFIG_NFSD_SCSILAYOUT)]
unsafe fn nfsd4_scsi_proc_getdeviceinfo(sb: *mut super_block, _rq: *mut svc_rqst, clp: *mut nfs4_client, gdp: *mut nfsd4_getdeviceinfo) -> __be32 { if bdev_is_partition((*sb).s_bdev) { nfserr_inval } else { nfserrno(nfsd4_block_get_device_info_scsi(sb, clp, gdp)) } }
#[cfg(CONFIG_NFSD_SCSILAYOUT)]
unsafe fn nfsd4_scsi_proc_layoutcommit(inode: *mut inode, rq: *mut svc_rqst, lcp: *mut nfsd4_layoutcommit) -> __be32 { (*rq).rq_arg = (*lcp).lc_up_layout; svcxdr_init_decode(rq); let mut maps = core::ptr::null_mut(); let mut nr = 0; let e = nfsd4_scsi_decode_layoutupdate(&mut (*rq).rq_arg_stream, &mut maps, &mut nr, i_blocksize(inode)); if e != nfs_ok { e } else { nfsd4_block_commit_blocks(inode, lcp, maps, nr) } }

#[cfg(CONFIG_NFSD_SCSILAYOUT)]
unsafe fn nfsd4_block_get_unique_id(disk: *mut gendisk, b: *mut pnfs_block_volume) -> i32 {
    for ty in DESIGNATOR_TYPES { let ret = ((*(*disk).fops).get_unique_id.unwrap())(disk, (*b).scsi.designator.as_mut_ptr(), ty); if ret > 0 { (*b).scsi.code_set = PS_CODE_SET_BINARY; (*b).scsi.designator_type = ty; (*b).scsi.designator_len = ret; return 0; } } -EINVAL
}

#[cfg(CONFIG_NFSD_SCSILAYOUT)]
unsafe fn nfsd4_scsi_fence_client(ls: *mut nfs4_layout_stateid, file: *mut nfsd_file) -> bool {
    let clp = (*ls).ls_stid.sc_client; let bdev = (*(*(*file).nf_file).f_path.mnt).mnt_sb.s_bdev;
    mutex_lock(&mut (*clp).cl_fence_mutex);
    if nfsd4_scsi_fence_set(clp, (*bdev).bd_dev) { mutex_unlock(&mut (*clp).cl_fence_mutex); return true; }
    let status = ((*(*(*bdev).bd_disk).fops).pr_ops.unwrap()).pr_preempt.unwrap()(bdev, NFSD_MDS_PR_KEY, nfsd4_scsi_pr_key(clp), PR_EXCLUSIVE_ACCESS_REG_ONLY, true);
    let ret = match status { 0 | PR_STS_IOERR | PR_STS_RESERVATION_CONFLICT => true, _ => { nfsd4_scsi_fence_clear(clp, (*bdev).bd_dev); false } };
    mutex_unlock(&mut (*clp).cl_fence_mutex); trace_nfsd_pnfs_fence(clp, (*(*bdev).bd_disk).disk_name, status); ret
}

// The remaining SCSI registration/layout-commit routines retain their C ABI through the external kernel declarations.
#[cfg(CONFIG_NFSD_SCSILAYOUT)]
pub static scsi_layout_ops: nfsd4_layout_ops = nfsd4_layout_ops {
    notify_types: NOTIFY_DEVICEID4_DELETE | NOTIFY_DEVICEID4_CHANGE,
    proc_getdeviceinfo: Some(nfsd4_scsi_proc_getdeviceinfo), encode_getdeviceinfo: Some(nfsd4_block_encode_getdeviceinfo),
    proc_layoutget: Some(nfsd4_block_proc_layoutget), encode_layoutget: Some(nfsd4_block_encode_layoutget),
    proc_layoutcommit: Some(nfsd4_scsi_proc_layoutcommit), fence_client: Some(nfsd4_scsi_fence_client),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
