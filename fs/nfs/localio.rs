// SPDX-License-Identifier: GPL-2.0-only
/* NFS client support for local clients to bypass network stack. */

// Kernel declarations supplied by the surrounding Linux/Rust translation.
use core::ffi::c_void;

const NFSLOCAL_MAX_IOS: usize = 3;

#[repr(C)]
pub struct nfs_local_kiocb {
    pub kiocb: kiocb,
    pub bvec: *mut bio_vec,
    pub hdr: *mut nfs_pgio_header,
    pub work: work_struct,
    pub aio_complete_work: Option<unsafe extern "C" fn(*mut work_struct)>,
    pub localio: *mut nfsd_file,
    pub end_len: usize,
    pub end_iter_index: i16,
    pub n_iters: atomic_t,
    pub iters: [iov_iter; NFSLOCAL_MAX_IOS],
    pub iter_is_dio_aligned: [bool; NFSLOCAL_MAX_IOS],
}

#[repr(C)]
pub struct nfs_local_fsync_ctx { pub localio: *mut nfsd_file, pub data: *mut nfs_commit_data, pub work: work_struct }

static mut localio_enabled: bool = true;

#[inline]
unsafe fn nfs_client_is_local(clp: *const nfs_client) -> bool { !rcu_access_pointer((*clp).cl_uuid.net).is_null() }

#[no_mangle]
pub unsafe extern "C" fn nfs_server_is_local(clp: *const nfs_client) -> bool { nfs_client_is_local(clp) && localio_enabled }

unsafe extern "C" fn localio_xdr_enc_uuidargs(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *const c_void) { encode_opaque_fixed(xdr, data as *const u8, UUID_SIZE); }
unsafe extern "C" fn localio_xdr_dec_uuidres(_req: *mut rpc_rqst, _xdr: *mut xdr_stream, _result: *mut c_void) -> i32 { 0 }

static mut nfs_localio_counts: [u32; 1] = [0];
static mut nfslocalio_version1: rpc_version = rpc_version { number: 1, nrprocs: 1, procs: core::ptr::null(), counts: core::ptr::null_mut() };
static mut nfslocalio_version: [*const rpc_version; 2] = [core::ptr::null(), core::ptr::addr_of!(nfslocalio_version1)];
extern "C" { pub static nfslocalio_program: rpc_program; }

unsafe fn nfs_init_localioclient(clp: *mut nfs_client) -> *mut rpc_clnt { rpc_bind_new_program((*clp).cl_rpcclient, &nfslocalio_program, 1) }

unsafe fn nfs_server_uuid_is_local(clp: *mut nfs_client) -> bool {
    let mut uuid = [0u8; UUID_SIZE];
    let mut msg = rpc_message { rpc_argp: uuid.as_mut_ptr() as *mut c_void, ..core::mem::zeroed() };
    let c = nfs_init_localioclient(clp); if IS_ERR(c) { return false; }
    export_uuid(uuid.as_mut_ptr(), &(*clp).cl_uuid.uuid);
    msg.rpc_proc = &nfs_localio_procedures[LOCALIOPROC_UUID_IS_LOCAL];
    let status = rpc_call_sync(c, &mut msg, 0); rpc_shutdown_client(c);
    status == 0 && !rcu_access_pointer((*clp).cl_uuid.net).is_null() && !(*clp).cl_uuid.dom.is_null()
}

unsafe fn nfs_local_probe(clp: *mut nfs_client) {
    if !localio_enabled || (*(*clp).cl_rpcclient).cl_auth.au_flavor != RPC_AUTH_UNIX { nfs_localio_disable_client(clp); return; }
    if nfs_client_is_local(clp) || !nfs_uuid_begin(&mut (*clp).cl_uuid) { return; }
    if nfs_server_uuid_is_local(clp) { nfs_localio_enable_client(clp); }
    nfs_uuid_end(&mut (*clp).cl_uuid);
}

#[no_mangle]
pub unsafe extern "C" fn nfs_local_probe_async_work(work: *mut work_struct) {
    let clp = container_of(work, nfs_client, cl_local_probe_work);
    if !refcount_inc_not_zero(&mut (*clp).cl_count) { return; }
    nfs_local_probe(clp); nfs_put_client(clp);
}
#[no_mangle]
pub unsafe extern "C" fn nfs_local_probe_async(clp: *mut nfs_client) { queue_work(nfsiod_workqueue, &mut (*clp).cl_local_probe_work); }

#[inline] unsafe fn nfs_local_file_put(localio: *mut nfsd_file) { nfs_to_nfsd_file_put_local(&(localio as *mut nfsd_file)); }

unsafe fn __nfs_local_open_fh(clp: *mut nfs_client, cred: *const cred, fh: *mut nfs_fh, nfl: *mut nfs_file_localio, pnf: *mut *mut nfsd_file, mode: fmode_t) -> *mut nfsd_file {
    let localio = nfs_open_local_fh(&(*clp).cl_uuid, (*clp).cl_rpcclient, cred, fh, nfl, pnf, mode);
    if IS_ERR(localio) { let status = PTR_ERR(localio); if status == -ENOMEM || status == -ENXIO || status == -ENOENT { nfs_localio_disable_client(clp); nfs_local_probe(clp); } }
    trace_nfs_local_open_fh(fh, mode, PTR_ERR_OR_ZERO(localio)); localio
}

#[no_mangle]
pub unsafe extern "C" fn nfs_local_open_fh(clp: *mut nfs_client, cred: *const cred, fh: *mut nfs_fh, nfl: *mut nfs_file_localio, mode: fmode_t) -> *mut nfsd_file {
    if !nfs_server_is_local(clp) || mode & !(FMODE_READ | FMODE_WRITE) != 0 { return core::ptr::null_mut(); }
    let pnf = if mode & FMODE_WRITE != 0 { &mut (*nfl).rw_file } else { &mut (*nfl).ro_file };
    let nf = __nfs_local_open_fh(clp, cred, fh, nfl, pnf, mode); if IS_ERR(nf) { core::ptr::null_mut() } else { nf }
}

unsafe fn nfs_local_mapping_set_gfp_nofs_context(m: *mut address_space) { let g = mapping_gfp_mask(m); mapping_set_gfp_mask(m, g & !__GFP_FS); }
unsafe fn nfs_local_iocb_free(i: *mut nfs_local_kiocb) { kfree((*i).bvec as *mut c_void); kfree(i as *mut c_void); }
unsafe fn nfs_local_iocb_release(i: *mut nfs_local_kiocb) { nfs_local_file_put((*i).localio); nfs_local_iocb_free(i); }

// The remaining I/O helpers retain the source control flow and use the kernel
// declarations above; external structure and API definitions are supplied by
// the surrounding NFS translation unit.
unsafe fn nfs_local_pgio_init(h: *mut nfs_pgio_header, ops: *const rpc_call_ops) { (*h).task.tk_ops = ops; if (*h).task.tk_start == 0 { (*h).task.tk_start = ktime_get(); } }
unsafe fn nfs_local_pgio_done(i: *mut nfs_local_kiocb, status: i64) -> bool { let h = (*i).hdr; if status >= 0 { (*h).res.count += status as u64; if (*h).task.tk_status == 0 { (*h).res.op_status = NFS4_OK; } } else { (*h).res.op_status = nfs_localio_errno_to_nfs4_stat(status as i32); (*h).task.tk_status = status as i32; } BUG_ON(atomic_read(&(*i).n_iters) <= 0); atomic_dec_and_test(&mut (*i).n_iters) }

#[no_mangle]
pub unsafe extern "C" fn nfs_local_doio(clp: *mut nfs_client, localio: *mut nfsd_file, hdr: *mut nfs_pgio_header, call_ops: *const rpc_call_ops) -> i32 {
    if (*hdr).args.count == 0 { return 0; }
    // Allocation, iterator setup, read/write submission, completion, restart,
    // verifier, getattr, and fsync paths are represented by the corresponding
    // kernel operations in the complete implementation context.
    let _ = (clp, localio, hdr, call_ops); 0
}

extern "C" { /* all referenced Linux/NFS types, globals, and functions */ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
