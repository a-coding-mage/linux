// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2016 Tom Haynes <loghyr@primarydata.com>
 *
 * The following implements a super-simple flex-file server
 * where the NFSv4.1 mds is also the ds. And the storage is
 * the same. I.e., writing to the mds via a NFSv4.1 WRITE
 * goes to the same location as the NFSv3 WRITE.
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

const NFSDDBG_FACILITY: u32 = NFSDDBG_PNFS;

unsafe fn nfsd4_ff_proc_layoutget(
    rqstp: *mut svc_rqst,
    inode: *mut inode,
    fhp: *const svc_fh,
    args: *mut nfsd4_layoutget,
) -> __be32 {
    let seg: *mut nfsd4_layout_seg = &mut (*args).lg_seg;
    let device_generation: u32 = 0;
    let mut error: i32;
    let u: uid_t;

    let mut fl: *mut pnfs_ff_layout;

    /*
     * The super simple flex file server has 1 mirror, 1 data server,
     * and 1 file handle. So instead of 4 allocs, do 1 for now.
     * Zero it out for the stateid - don't want junk in there!
     */
    error = -ENOMEM;
    fl = kzalloc_obj::<pnfs_ff_layout>();
    if fl.is_null() {
        (*seg).length = 0;
        return nfserrno(error);
    }
    (*args).lg_content = fl as *mut _;

    /*
     * Avoid layout commit, try to force the I/O to the DS,
     * and for fun, cause all IOMODE_RW layout segments to
     * effectively be WRITE only.
     */
    (*fl).flags = FF_FLAGS_NO_LAYOUTCOMMIT | FF_FLAGS_NO_IO_THRU_MDS
        | FF_FLAGS_NO_READ_IO;

    /* Do not allow a IOMODE_READ segment to have write pemissions */
    if (*seg).iomode == IOMODE_READ {
        u = from_kuid(&init_user_ns, (*inode).i_uid) + 1;
        (*fl).uid = make_kuid(&init_user_ns, u);
    } else {
        (*fl).uid = (*inode).i_uid;
    }
    (*fl).gid = (*inode).i_gid;

    error = nfsd4_set_deviceid(&mut (*fl).deviceid, fhp, device_generation);
    if error != 0 {
        (*seg).length = 0;
        return nfserrno(error);
    }

    (*fl).fh.size = (*fhp).fh_handle.fh_size;
    memcpy(
        (*fl).fh.data.as_mut_ptr(),
        &(*fhp).fh_handle.fh_raw as *const _,
        (*fl).fh.size,
    );

    /* Give whole file layout segments */
    (*seg).offset = 0;
    (*seg).length = NFS4_MAX_UINT64;

    dprintk!("GET: 0x%llx:0x%llx %d\n", (*seg).offset, (*seg).length, (*seg).iomode);
    0

}
}

unsafe fn nfsd4_ff_proc_getdeviceinfo(
    sb: *mut super_block,
    rqstp: *mut svc_rqst,
    clp: *mut nfs4_client,
    gdp: *mut nfsd4_getdeviceinfo,
) -> __be32 {
    let mut da: *mut pnfs_ff_device_addr;
    let mut port: u16;
    let mut addr = [0i8; INET6_ADDRSTRLEN];

    da = kzalloc_obj::<pnfs_ff_device_addr>();
    if da.is_null() {
        return nfserrno(-ENOMEM);
    }

    (*gdp).gd_device = da as *mut _;
    (*da).version = 3;
    (*da).minor_version = 0;
    (*da).rsize = svc_max_payload(rqstp);
    (*da).wsize = (*da).rsize;

    rpc_ntop(
        &(*rqstp).rq_daddr as *const _ as *const sockaddr,
        addr.as_mut_ptr(),
        INET6_ADDRSTRLEN,
    );
    if (*rqstp).rq_daddr.ss_family == AF_INET {
        let sin = &(*rqstp).rq_daddr as *const _ as *const sockaddr_in;
        port = ntohs((*sin).sin_port);
        snprintf((*da).netaddr.netid.as_mut_ptr(), FF_NETID_LEN + 1, c"tcp");
        (*da).netaddr.netid_len = 3;
    } else {
        let sin6 = &(*rqstp).rq_daddr as *const _ as *const sockaddr_in6;
        port = ntohs((*sin6).sin6_port);
        snprintf((*da).netaddr.netid.as_mut_ptr(), FF_NETID_LEN + 1, c"tcp6");
        (*da).netaddr.netid_len = 4;
    }

    (*da).netaddr.addr_len = snprintf(
        (*da).netaddr.addr.as_mut_ptr(),
        FF_ADDR_LEN + 1,
        c"%s.%d.%d",
        addr.as_ptr(),
        port >> 8,
        port & 0xff,
    );
    (*da).tightly_coupled = false;
    0
}

unsafe fn nfsd4_ff_proc_layoutcommit(
    inode: *mut inode,
    rqstp: *mut svc_rqst,
    lcp: *mut nfsd4_layoutcommit,
) -> __be32 {
    nfs_ok
}

pub static ff_layout_ops: nfsd4_layout_ops = nfsd4_layout_ops {
    notify_types: NOTIFY_DEVICEID4_DELETE | NOTIFY_DEVICEID4_CHANGE,
    disable_recalls: true,
    proc_getdeviceinfo: Some(nfsd4_ff_proc_getdeviceinfo),
    encode_getdeviceinfo: Some(nfsd4_ff_encode_getdeviceinfo),
    proc_layoutget: Some(nfsd4_ff_proc_layoutget),
    encode_layoutget: Some(nfsd4_ff_encode_layoutget),
    proc_layoutcommit: Some(nfsd4_ff_proc_layoutcommit),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
