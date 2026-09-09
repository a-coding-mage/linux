// SPDX-License-Identifier: GPL-2.0
/*
 * In-kernel MOUNT protocol client
 *
 * Copyright (C) 1997, Olaf Kirch <okir@monad.swb.de>
 */

// Kernel header dependencies are supplied by the surrounding translation unit.

const MNTPATHLEN: usize = 1024;
const ENCODE_DIRPATH_SZ: usize = 1 + XDR_QUADLEN(MNTPATHLEN);
const MNT_STATUS_SZ: usize = 1;
const MNT_FHANDLE_SZ: usize = XDR_QUADLEN(NFS2_FHSIZE);
const MNT_FHANDLEV3_SZ: usize = XDR_QUADLEN(NFS3_FHSIZE);
const MNT_AUTHFLAV3_SZ: usize = 1 + NFS_MAX_SECFLAVORS;
const MNT_ENC_DIRPATH_SZ: usize = ENCODE_DIRPATH_SZ;
const MNT_DEC_MOUNTRES_SZ: usize = MNT_STATUS_SZ + MNT_FHANDLE_SZ;
const MNT_DEC_MOUNTRES3_SZ: usize = MNT_STATUS_SZ + MNT_FHANDLEV3_SZ + MNT_AUTHFLAV3_SZ;

const MOUNTPROC_NULL: usize = 0;
const MOUNTPROC_MNT: usize = 1;
const MOUNTPROC_DUMP: usize = 2;
const MOUNTPROC_UMNT: usize = 3;
const MOUNTPROC_UMNTALL: usize = 4;
const MOUNTPROC_EXPORT: usize = 5;
const MOUNTPROC3_NULL: usize = 0;
const MOUNTPROC3_MNT: usize = 1;
const MOUNTPROC3_DUMP: usize = 2;
const MOUNTPROC3_UMNT: usize = 3;
const MOUNTPROC3_UMNTALL: usize = 4;
const MOUNTPROC3_EXPORT: usize = 5;

#[repr(u32)]
enum Mountstat { MntOk = 0, MntEperm = 1, MntEnoent = 2, MntEacces = 13, MntEinval = 22 }

#[repr(C)]
struct MntErr { status: u32, errno: i32 }

static mut MNT_ERRTBL: [MntErr; 5] = [
    MntErr { status: Mountstat::MntOk as u32, errno: 0 },
    MntErr { status: Mountstat::MntEperm as u32, errno: -EPERM },
    MntErr { status: Mountstat::MntEnoent as u32, errno: -ENOENT },
    MntErr { status: Mountstat::MntEacces as u32, errno: -EACCES },
    MntErr { status: Mountstat::MntEinval as u32, errno: -EINVAL },
];

#[repr(u32)]
enum Mountstat3 {
    Mnt3Ok = 0, Mnt3ErrPerm = 1, Mnt3ErrNoent = 2, Mnt3ErrIo = 5,
    Mnt3ErrAcces = 13, Mnt3ErrNotdir = 20, Mnt3ErrInval = 22,
    Mnt3ErrNametoolong = 63, Mnt3ErrNotsupp = 10004, Mnt3ErrServerfault = 10006,
}

static mut MNT3_ERRTBL: [MntErr; 10] = [
    MntErr { status: Mountstat3::Mnt3Ok as u32, errno: 0 },
    MntErr { status: Mountstat3::Mnt3ErrPerm as u32, errno: -EPERM },
    MntErr { status: Mountstat3::Mnt3ErrNoent as u32, errno: -ENOENT },
    MntErr { status: Mountstat3::Mnt3ErrIo as u32, errno: -EIO },
    MntErr { status: Mountstat3::Mnt3ErrAcces as u32, errno: -EACCES },
    MntErr { status: Mountstat3::Mnt3ErrNotdir as u32, errno: -ENOTDIR },
    MntErr { status: Mountstat3::Mnt3ErrInval as u32, errno: -EINVAL },
    MntErr { status: Mountstat3::Mnt3ErrNametoolong as u32, errno: -ENAMETOOLONG },
    MntErr { status: Mountstat3::Mnt3ErrNotsupp as u32, errno: -ENOTSUPP },
    MntErr { status: Mountstat3::Mnt3ErrServerfault as u32, errno: -EREMOTEIO },
];

#[repr(C)]
struct Mountres {
    errno: i32,
    fh: *mut nfs_fh,
    auth_count: *mut c_uint,
    auth_flavors: *mut rpc_authflavor_t,
}

pub unsafe fn nfs_mount(info: *mut nfs_mount_request, timeo: i32, retrans: i32) -> i32 {
    let mut mnt_timeout: rpc_timeout = core::mem::zeroed();
    let mut result = Mountres { errno: 0, fh: (*info).fh, auth_count: (*info).auth_flav_len, auth_flavors: (*info).auth_flavs };
    let mut msg: rpc_message = core::mem::zeroed();
    msg.rpc_argp = (*info).dirpath as *const c_void;
    msg.rpc_resp = &mut result as *mut _ as *mut c_void;
    let mut args: rpc_create_args = core::mem::zeroed();
    args.net = (*info).net; args.protocol = (*info).protocol;
    args.address = (*info).sap as *mut sockaddr; args.addrsize = (*info).salen;
    args.timeout = &mut mnt_timeout; args.servername = (*info).hostname;
    args.program = &mnt_program; args.version = (*info).version;
    args.authflavor = RPC_AUTH_UNIX; args.cred = current_cred();
    dprintk!("NFS: sending MNT request for %s:%s\n", if (*info).hostname.is_null() { "server" } else { (*info).hostname }, (*info).dirpath);
    if strlen((*info).dirpath) > MNTPATHLEN { return -ENAMETOOLONG; }
    if (*info).noresvport { args.flags |= RPC_CLNT_CREATE_NONPRIVPORT; }
    nfs_init_timeout_values(&mut mnt_timeout, (*info).protocol, timeo, retrans);
    let mnt_clnt = rpc_create(&args);
    if IS_ERR(mnt_clnt) { let status = PTR_ERR(mnt_clnt); dprintk!("NFS: failed to create MNT RPC client, status=%d\n", status); return status; }
    if (*info).version == NFS_MNT3_VERSION { msg.rpc_proc = (*mnt_clnt).cl_procinfo.add(MOUNTPROC3_MNT); } else { msg.rpc_proc = (*mnt_clnt).cl_procinfo.add(MOUNTPROC_MNT); }
    let mut status = rpc_call_sync(mnt_clnt, &mut msg, RPC_TASK_SOFT | RPC_TASK_TIMEOUT);
    rpc_shutdown_client(mnt_clnt);
    if status < 0 { dprintk!("NFS: MNT request failed, status=%d\n", status); return status; }
    if result.errno != 0 { dprintk!("NFS: MNT server returned result %d\n", result.errno); return result.errno; }
    dprintk!("NFS: MNT request succeeded\n"); status = 0;
    if (*info).version != NFS_MNT3_VERSION || *(*info).auth_flav_len == 0 {
        dprintk!("NFS: Faking up auth_flavs list\n"); *(*info).auth_flavs = RPC_AUTH_NULL; *(*info).auth_flav_len = 1;
    }
    status
}

unsafe fn encode_mntdirpath(xdr: *mut xdr_stream, pathname: *const c_char) { let pathname_len = strlen(pathname); let p = xdr_reserve_space(xdr, 4 + pathname_len); xdr_encode_opaque(p, pathname, pathname_len); }
unsafe fn mnt_xdr_enc_dirpath(_req: *mut rpc_rqst, xdr: *mut xdr_stream, dirpath: *const c_void) { encode_mntdirpath(xdr, dirpath as *const c_char); }

unsafe fn decode_status(xdr: *mut xdr_stream, res: *mut Mountres) -> i32 {
    let p = xdr_inline_decode(xdr, 4); if p.is_null() { return -EIO; } let status = be32_to_cpup(p);
    for e in MNT_ERRTBL.iter() { if e.status == status { (*res).errno = e.errno; return 0; } }
    dprintk!("NFS: unrecognized MNT status code: %u\n", status); (*res).errno = -EACCES; 0
}
unsafe fn decode_fhandle(xdr: *mut xdr_stream, res: *mut Mountres) -> i32 { let p = xdr_inline_decode(xdr, NFS2_FHSIZE); if p.is_null() { return -EIO; } (*(*res).fh).size = NFS2_FHSIZE; memcpy((*(*res).fh).data.as_mut_ptr(), p, NFS2_FHSIZE); 0 }
unsafe fn mnt_xdr_dec_mountres(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *mut c_void) -> i32 { let res = data as *mut Mountres; let status = decode_status(xdr, res); if status != 0 || (*res).errno != 0 { status } else { decode_fhandle(xdr, res) } }

unsafe fn decode_fhs_status(xdr: *mut xdr_stream, res: *mut Mountres) -> i32 { let p = xdr_inline_decode(xdr, 4); if p.is_null() { return -EIO; } let status = be32_to_cpup(p); for e in MNT3_ERRTBL.iter() { if e.status == status { (*res).errno = e.errno; return 0; } } dprintk!("NFS: unrecognized MNT3 status code: %u\n", status); (*res).errno = -EACCES; 0 }
unsafe fn decode_fhandle3(xdr: *mut xdr_stream, res: *mut Mountres) -> i32 { let p = xdr_inline_decode(xdr, 4); if p.is_null() { return -EIO; } let size = be32_to_cpup(p); if size > NFS3_FHSIZE || size == 0 { return -EIO; } let p = xdr_inline_decode(xdr, size); if p.is_null() { return -EIO; } (*(*res).fh).size = size; memcpy((*(*res).fh).data.as_mut_ptr(), p, size); 0 }
unsafe fn decode_auth_flavors(xdr: *mut xdr_stream, res: *mut Mountres) -> i32 { if *(*res).auth_count == 0 { return 0; } let p = xdr_inline_decode(xdr, 4); if p.is_null() { return -EIO; } let mut entries = be32_to_cpup(p); dprintk!("NFS: received %u auth flavors\n", entries); if entries > NFS_MAX_SECFLAVORS { entries = NFS_MAX_SECFLAVORS; } let mut p = xdr_inline_decode(xdr, 4 * entries); if p.is_null() { return -EIO; } if entries > *(*res).auth_count { entries = *(*res).auth_count; } for i in 0..entries { *(*res).auth_flavors.add(i as usize) = be32_to_cpup(p); p = p.add(1); dprintk!("NFS:   auth flavor[%u]: %d\n", i, *(*res).auth_flavors.add(i as usize)); } *(*res).auth_count = entries; 0 }
unsafe fn mnt_xdr_dec_mountres3(_req: *mut rpc_rqst, xdr: *mut xdr_stream, data: *mut c_void) -> i32 { let res = data as *mut Mountres; let status = decode_fhs_status(xdr, res); if status != 0 || (*res).errno != 0 { return status; } let status = decode_fhandle3(xdr, res); if status != 0 { (*res).errno = -EBADHANDLE; return 0; } decode_auth_flavors(xdr, res) }

static MNT_PROCEDURES: [rpc_procinfo; 4] = unsafe { core::mem::zeroed() };
static MNT3_PROCEDURES: [rpc_procinfo; 4] = unsafe { core::mem::zeroed() };
static mut MNT_COUNTS: [c_uint; 4] = [0; 4];
static mut MNT3_COUNTS: [c_uint; 4] = [0; 4];
static MNT_VERSION1: rpc_version = unsafe { core::mem::zeroed() };
static MNT_VERSION3: rpc_version = unsafe { core::mem::zeroed() };
static MNT_VERSION: [*const rpc_version; 4] = [core::ptr::null(), &MNT_VERSION1, core::ptr::null(), &MNT_VERSION3];
static mut MNT_STATS: rpc_stat = unsafe { core::mem::zeroed() };
static MNT_PROGRAM: rpc_program = unsafe { core::mem::zeroed() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
