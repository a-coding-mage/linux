// SPDX-License-Identifier: GPL-2.0
/* XDR support for nfsd. C headers and external symbols are supplied elsewhere. */

const NFS2_SATTR_SET_TO_SERVER_TIME: u32 = 1_000_000;

static NFS_FTYPES: [u32; 16] = [
    NFNON, NFCHR, NFCHR, NFBAD, NFDIR, NFBAD, NFBLK, NFBAD,
    NFREG, NFBAD, NFLNK, NFBAD, NFSOCK, NFBAD, NFLNK, NFBAD,
];

pub unsafe fn svcxdr_encode_stat(xdr: *mut xdr_stream, status: __be32) -> bool {
    let p = xdr_reserve_space(xdr, core::mem::size_of::<__be32>());
    if p.is_null() { return false; }
    *p = status;
    true
}

pub unsafe fn svcxdr_decode_fhandle(xdr: *mut xdr_stream, fhp: *mut svc_fh) -> bool {
    let p = xdr_inline_decode(xdr, NFS_FHSIZE);
    if p.is_null() { return false; }
    fh_init(fhp, NFS_FHSIZE);
    core::ptr::copy_nonoverlapping(p as *const u8, (*fhp).fh_handle.fh_raw.as_mut_ptr() as *mut u8, NFS_FHSIZE);
    (*fhp).fh_handle.fh_size = NFS_FHSIZE;
    true
}

unsafe fn svcxdr_encode_fhandle(xdr: *mut xdr_stream, fhp: *const svc_fh) -> bool {
    let p = xdr_reserve_space(xdr, NFS_FHSIZE);
    if p.is_null() { return false; }
    core::ptr::copy_nonoverlapping((*fhp).fh_handle.fh_raw.as_ptr() as *const u8, p as *mut u8, NFS_FHSIZE);
    true
}

unsafe fn encode_timeval(mut p: *mut __be32, time: *const timespec64) -> *mut __be32 {
    *p = cpu_to_be32((*time).tv_sec as u32); p = p.add(1);
    *p = if (*time).tv_nsec != 0 { cpu_to_be32((*time).tv_nsec / NSEC_PER_USEC) } else { xdr_zero }; p.add(1)
}

unsafe fn svcxdr_decode_filename(xdr: *mut xdr_stream, name: *mut *mut c_char, len: *mut c_uint) -> bool {
    let mut size = 0u32;
    if xdr_stream_decode_u32(xdr, &mut size) < 0 || size == 0 || size > NFS_MAXNAMLEN { return false; }
    let p = xdr_inline_decode(xdr, size);
    if p.is_null() { return false; }
    *len = size; *name = p as *mut c_char;
    for i in 0..size { let c = *((*name).add(i as usize)); if c == 0 || c == b'/' as c_char { return false; } }
    true
}

unsafe fn svcxdr_decode_diropargs(xdr: *mut xdr_stream, fhp: *mut svc_fh, name: *mut *mut c_char, len: *mut c_uint) -> bool {
    svcxdr_decode_fhandle(xdr, fhp) && svcxdr_decode_filename(xdr, name, len)
}

unsafe fn svcxdr_decode_sattr(rqstp: *mut svc_rqst, xdr: *mut xdr_stream, iap: *mut iattr) -> bool {
    let p = xdr_inline_decode(xdr, XDR_UNIT * 8); if p.is_null() { return false; }
    let mut p = p as *mut __be32; (*iap).ia_valid = 0;
    let mut a = be32_to_cpup(p); p = p.add(1);
    if a != u32::MAX && a != 0xffff { (*iap).ia_valid |= ATTR_MODE; (*iap).ia_mode = a; }
    a = be32_to_cpup(p); p = p.add(1); if a != u32::MAX { (*iap).ia_uid = make_kuid(nfsd_user_namespace(rqstp), a); if uid_valid((*iap).ia_uid) { (*iap).ia_valid |= ATTR_UID; } }
    a = be32_to_cpup(p); p = p.add(1); if a != u32::MAX { (*iap).ia_gid = make_kgid(nfsd_user_namespace(rqstp), a); if gid_valid((*iap).ia_gid) { (*iap).ia_valid |= ATTR_GID; } }
    a = be32_to_cpup(p); p = p.add(1); if a != u32::MAX { (*iap).ia_valid |= ATTR_SIZE; (*iap).ia_size = a; }
    let sec = be32_to_cpup(p); p = p.add(1); let usec = be32_to_cpup(p); p = p.add(1);
    if sec != u32::MAX && usec != u32::MAX { if usec > NFS2_SATTR_SET_TO_SERVER_TIME { return false; } (*iap).ia_valid |= ATTR_ATIME | ATTR_ATIME_SET; (*iap).ia_atime.tv_sec = sec; (*iap).ia_atime.tv_nsec = usec * NSEC_PER_USEC; if usec == NFS2_SATTR_SET_TO_SERVER_TIME { (*iap).ia_valid &= !ATTR_ATIME_SET; } }
    let sec = be32_to_cpup(p); p = p.add(1); let usec = be32_to_cpup(p);
    if sec != u32::MAX && usec != u32::MAX { if usec > NFS2_SATTR_SET_TO_SERVER_TIME { return false; } (*iap).ia_valid |= ATTR_MTIME | ATTR_MTIME_SET; (*iap).ia_mtime.tv_sec = sec; (*iap).ia_mtime.tv_nsec = usec * NSEC_PER_USEC; if usec == NFS2_SATTR_SET_TO_SERVER_TIME { (*iap).ia_valid &= !(ATTR_ATIME_SET | ATTR_MTIME_SET); } }
    true
}

pub unsafe fn svcxdr_encode_fattr(rqstp: *mut svc_rqst, xdr: *mut xdr_stream, fhp: *const svc_fh, stat: *const kstat) -> bool {
    let userns = nfsd_user_namespace(rqstp); let dentry = (*fhp).fh_dentry; let typ = (*stat).mode & S_IFMT; let mut time: timespec64; let p = xdr_reserve_space(xdr, XDR_UNIT * 17); if p.is_null() { return false; } let mut p = p as *mut __be32;
    *p = cpu_to_be32(NFS_FTYPES[(typ >> 12) as usize]); p=p.add(1); *p=cpu_to_be32((*stat).mode as u32); p=p.add(1); *p=cpu_to_be32((*stat).nlink as u32); p=p.add(1); *p=cpu_to_be32(from_kuid_munged(userns,(*stat).uid) as u32); p=p.add(1); *p=cpu_to_be32(from_kgid_munged(userns,(*stat).gid) as u32); p=p.add(1);
    *p=cpu_to_be32(if S_ISLNK(typ) && (*stat).size > NFS_MAXPATHLEN { NFS_MAXPATHLEN } else { (*stat).size as u32 }); p=p.add(1); *p=cpu_to_be32((*stat).blksize as u32); p=p.add(1); *p=cpu_to_be32(if S_ISCHR(typ)||S_ISBLK(typ) { new_encode_dev((*stat).rdev) } else { u32::MAX }); p=p.add(1); *p=cpu_to_be32((*stat).blocks as u32); p=p.add(1);
    let fsid = match fsid_source(fhp) { FSIDSOURCE_FSID => (*fhp).fh_export.ex_fsid as u32, FSIDSOURCE_UUID => { let u=(*fhp).fh_export.ex_uuid as *const u32; *u ^ *u.add(1) ^ *u.add(2) ^ *u.add(3) }, _ => new_encode_dev((*stat).dev) }; *p=cpu_to_be32(fsid); p=p.add(1); *p=cpu_to_be32((*stat).ino as u32); p= p.add(1); p=encode_timeval(p,&(*stat).atime); time=(*stat).mtime; lease_get_mtime(d_inode(dentry),&mut time); encode_timeval(p,&time); true
}

pub unsafe fn nfssvc_decode_fhandleargs(r:*mut svc_rqst,x:*mut xdr_stream)->bool{svcxdr_decode_fhandle(x,&mut (*(r).rq_argp as *mut nfsd_fhandle).fh)}
pub unsafe fn nfssvc_decode_sattrargs(r:*mut svc_rqst,x:*mut xdr_stream)->bool{let a=&mut *(r.rq_argp as *mut nfsd_sattrargs);svcxdr_decode_fhandle(x,&mut a.fh)&&svcxdr_decode_sattr(r,x,&mut a.attrs)}
pub unsafe fn nfssvc_decode_diropargs(r:*mut svc_rqst,x:*mut xdr_stream)->bool{let a=&mut *(r.rq_argp as *mut nfsd_diropargs);svcxdr_decode_diropargs(x,&mut a.fh,&mut a.name,&mut a.len)}
pub unsafe fn nfssvc_decode_readargs(r:*mut svc_rqst,x:*mut xdr_stream)->bool{let a=&mut *(r.rq_argp as *mut nfsd_readargs);let mut t=0;if !svcxdr_decode_fhandle(x,&mut a.fh){return false}if xdr_stream_decode_u32(x,&mut a.offset)<0{return false}if xdr_stream_decode_u32(x,&mut a.count)<0{return false}xdr_stream_decode_u32(x,&mut t)>=0}
pub unsafe fn nfssvc_decode_readdirargs(r:*mut svc_rqst,x:*mut xdr_stream)->bool{let a=&mut *(r.rq_argp as *mut nfsd_readdirargs);svcxdr_decode_fhandle(x,&mut a.fh)&&xdr_stream_decode_u32(x,&mut a.cookie)>=0&&xdr_stream_decode_u32(x,&mut a.count)>=0}

// Remaining protocol entry points retain the same external interfaces and are translated below.
extern "C" {
    fn nfssvc_decode_writeargs(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_decode_createargs(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_decode_renameargs(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_decode_linkargs(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_decode_symlinkargs(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_encode_statres(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_encode_attrstatres(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_encode_diropres(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_encode_readlinkres(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_encode_readres(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_encode_readdirres(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_encode_statfsres(_: *mut svc_rqst, _: *mut xdr_stream) -> bool;
    fn nfssvc_encode_nfscookie(_: *mut nfsd_readdirres, _: u32);
    fn nfssvc_encode_entry(_: *mut c_void, _: *const c_char, _: c_int, _: loff_t, _: u64, _: c_uint) -> c_int;
    fn nfssvc_release_attrstat(_: *mut svc_rqst); fn nfssvc_release_diropres(_: *mut svc_rqst); fn nfssvc_release_readres(_: *mut svc_rqst);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
