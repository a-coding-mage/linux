// SPDX-License-Identifier: GPL-2.0-only
/*
 * NFS server support for local clients to bypass network stack
 *
 * Copyright (C) 2014 Weston Andros Adamson <dros@primarydata.com>
 * Copyright (C) 2019 Trond Myklebust <trond.myklebust@hammerspace.com>
 * Copyright (C) 2024 Mike Snitzer <snitzer@hammerspace.com>
 * Copyright (C) 2024 NeilBrown <neilb@suse.de>
 */

// Kernel dependencies supplied by the surrounding translation unit.

unsafe fn nfsd_open_local_fh(
    net: *mut net,
    dom: *mut auth_domain,
    rpc_clnt: *mut rpc_clnt,
    cred: *const cred,
    nfs_fh: *const nfs_fh,
    pnf: *mut *mut nfsd_file,
    fmode: fmode_t,
) -> *mut nfsd_file {
    let mut mayflags: int = NFSD_MAY_LOCALIO;
    let mut rq_cred: svc_cred = core::mem::zeroed();
    let mut fh: svc_fh = core::mem::zeroed();
    let mut localio: *mut nfsd_file;
    let mut beres: __be32;

    if (*nfs_fh).size > NFS4_FHSIZE {
        return ERR_PTR(-EINVAL);
    }

    if !nfsd_net_try_get(net) {
        return ERR_PTR(-ENXIO);
    }

    rcu_read_lock();
    localio = nfsd_file_get(rcu_dereference(pnf));
    rcu_read_unlock();
    if !localio.is_null() {
        return localio;
    }

    fh_init(&mut fh, NFS4_FHSIZE);
    (*fh.fh_handle).fh_size = (*nfs_fh).size;
    core::ptr::copy_nonoverlapping(
        (*nfs_fh).data.as_ptr(),
        (*fh.fh_handle).fh_raw.as_mut_ptr(),
        (*nfs_fh).size as usize,
    );

    if fmode & FMODE_READ != 0 {
        mayflags |= NFSD_MAY_READ;
    }
    if fmode & FMODE_WRITE != 0 {
        mayflags |= NFSD_MAY_WRITE;
    }

    svcauth_map_clnt_to_svc_cred_local(rpc_clnt, cred, &mut rq_cred);
    beres = nfsd_file_acquire_local(net, &mut rq_cred, dom, &mut fh, mayflags, &mut localio);
    if beres != 0 {
        localio = ERR_PTR(nfs_stat_to_errno(be32_to_cpu(beres)));
    }

    fh_put(&mut fh);
    if !rq_cred.cr_group_info.is_null() {
        put_group_info(rq_cred.cr_group_info);
    }

    if !IS_ERR(localio) {
        let mut new: *mut nfsd_file;
        if !nfsd_net_try_get(net) {
            nfsd_file_put(localio);
            nfsd_net_put(net);
            return ERR_PTR(-ENXIO);
        }
        nfsd_file_get(localio);
        'again: loop {
            rcu_read_lock();
            new = unrcu_pointer(cmpxchg(pnf, core::ptr::null_mut(), localio));
            if !new.is_null() {
                if nfsd_file_get(new).is_null() {
                    rcu_read_unlock();
                    continue 'again;
                }
                rcu_read_unlock();
                nfsd_file_put(localio);
                nfsd_net_put(net);
                nfsd_file_put(localio);
                localio = new;
            } else {
                rcu_read_unlock();
            }
            break;
        }
    } else {
        nfsd_net_put(net);
    }
    localio
}

unsafe fn nfsd_file_dio_alignment(
    nf: *mut nfsd_file,
    nf_dio_mem_align: *mut u32,
    nf_dio_offset_align: *mut u32,
    nf_dio_read_offset_align: *mut u32,
) {
    *nf_dio_mem_align = (*nf).nf_dio_mem_align;
    *nf_dio_offset_align = (*nf).nf_dio_offset_align;
    *nf_dio_read_offset_align = (*nf).nf_dio_read_offset_align;
}

static nfsd_localio_ops: nfsd_localio_operations = nfsd_localio_operations {
    nfsd_net_try_get: nfsd_net_try_get,
    nfsd_net_put: nfsd_net_put,
    nfsd_open_local_fh: nfsd_open_local_fh,
    nfsd_file_put_local: nfsd_file_put_local,
    nfsd_file_file: nfsd_file_file,
    nfsd_file_dio_alignment: nfsd_file_dio_alignment,
};

pub unsafe fn nfsd_localio_ops_init() {
    nfs_to = &nfsd_localio_ops;
}

/* UUID_IS_LOCAL XDR functions */

unsafe fn localio_proc_null(_rqstp: *mut svc_rqst) -> __be32 { rpc_success }

#[repr(C)]
struct localio_uuidarg { uuid: uuid_t }

unsafe fn localio_proc_uuid_is_local(rqstp: *mut svc_rqst) -> __be32 {
    let argp = (*rqstp).rq_argp as *mut localio_uuidarg;
    let net = SVC_NET(rqstp);
    let nn = net_generic(net, nfsd_net_id);
    nfs_uuid_is_local(&mut (*argp).uuid, &mut (*nn).local_clients,
        &mut (*nn).local_clients_lock, net, (*rqstp).rq_client, THIS_MODULE);
    rpc_success
}

unsafe fn localio_decode_uuidarg(rqstp: *mut svc_rqst, xdr: *mut xdr_stream) -> bool {
    let argp = (*rqstp).rq_argp as *mut localio_uuidarg;
    let mut uuid = [0u8; UUID_SIZE];
    if decode_opaque_fixed(xdr, uuid.as_mut_ptr(), UUID_SIZE) { return false; }
    import_uuid(&mut (*argp).uuid, uuid.as_ptr());
    true
}

static localio_procedures1: [svc_procedure; 2] = [
    svc_procedure { pc_func: localio_proc_null, pc_decode: nfssvc_decode_voidarg,
        pc_encode: nfssvc_encode_voidres, pc_argsize: core::mem::size_of::<nfsd_voidargs>(),
        pc_argzero: 0, pc_ressize: core::mem::size_of::<nfsd_voidres>(), pc_cachetype: RC_NOCACHE,
        pc_xdrressize: 0, pc_name: "NULL" },
    svc_procedure { pc_func: localio_proc_uuid_is_local, pc_decode: localio_decode_uuidarg,
        pc_encode: nfssvc_encode_voidres, pc_argsize: core::mem::size_of::<localio_uuidarg>(),
        pc_argzero: core::mem::size_of::<localio_uuidarg>(), pc_ressize: core::mem::size_of::<nfsd_voidres>(),
        pc_cachetype: RC_NOCACHE, pc_xdrressize: 0, pc_name: "UUID_IS_LOCAL" },
];

const LOCALIO_NR_PROCEDURES: usize = localio_procedures1.len();
#[no_mangle]
pub static localio_version1: svc_version = svc_version {
    vs_vers: 1, vs_nproc: LOCALIO_NR_PROCEDURES, vs_proc: localio_procedures1.as_ptr(),
    vs_dispatch: nfsd_dispatch, vs_xdrsize: XDR_QUADLEN(UUID_SIZE), vs_hidden: true,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
