// SPDX-License-Identifier: GPL-2.0
/* Rust translation of linux/fs/nfs/nfs2xdr.c. */

// Kernel dependencies supplied by the surrounding translation unit are intentionally external.

const NFSDBG_FACILITY: u32 = NFSDBG_XDR;
const NFS_pagepad_sz: usize = 1;
const NFS_fhandle_sz: usize = 8;
const NFS_sattr_sz: usize = 8;
const NFS_filename_sz: usize = 1 + (NFS2_MAXNAMLEN >> 2);
const NFS_path_sz: usize = 1 + (NFS2_MAXPATHLEN >> 2);
const NFS_fattr_sz: usize = 17;
const NFS_info_sz: usize = 5;
const NFS_entry_sz: usize = NFS_filename_sz + 3;
const NFS_diropargs_sz: usize = NFS_fhandle_sz + NFS_filename_sz;
const NFS_removeargs_sz: usize = NFS_fhandle_sz + NFS_filename_sz;
const NFS_sattrargs_sz: usize = NFS_fhandle_sz + NFS_sattr_sz;
const NFS_readlinkargs_sz: usize = NFS_fhandle_sz;
const NFS_readargs_sz: usize = NFS_fhandle_sz + 3;
const NFS_writeargs_sz: usize = NFS_fhandle_sz + 4;
const NFS_createargs_sz: usize = NFS_diropargs_sz + NFS_sattr_sz;
const NFS_renameargs_sz: usize = NFS_diropargs_sz + NFS_diropargs_sz;
const NFS_linkargs_sz: usize = NFS_fhandle_sz + NFS_diropargs_sz;
const NFS_symlinkargs_sz: usize = NFS_diropargs_sz + 1 + NFS_sattr_sz;
const NFS_readdirargs_sz: usize = NFS_fhandle_sz + 2;
const NFS_attrstat_sz: usize = 1 + NFS_fattr_sz;
const NFS_diropres_sz: usize = 1 + NFS_fhandle_sz + NFS_fattr_sz;
const NFS_readlinkres_sz: usize = 2 + NFS_pagepad_sz;
const NFS_readres_sz: usize = 1 + NFS_fattr_sz + 1 + NFS_pagepad_sz;
const NFS_writeres_sz: usize = NFS_attrstat_sz;
const NFS_stat_sz: usize = 1;
const NFS_readdirres_sz: usize = 1 + NFS_pagepad_sz;
const NFS_statfsres_sz: usize = 1 + NFS_info_sz;

unsafe fn rpc_userns(clnt: *const rpc_clnt) -> *mut user_namespace {
    if !clnt.is_null() && !(*clnt).cl_cred.is_null() { (*(*clnt).cl_cred).user_ns } else { &raw mut init_user_ns }
}
unsafe fn rpc_rqst_userns(rqstp: *const rpc_rqst) -> *mut user_namespace {
    if !(*rqstp).rq_task.is_null() { rpc_userns((*(*rqstp).rq_task).tk_client) } else { &raw mut init_user_ns }
}

unsafe fn decode_nfsdata(xdr: *mut xdr_stream, result: *mut nfs_pgio_res) -> i32 {
    let p = xdr_inline_decode(xdr, 4); if p.is_null() { return -EIO; }
    let mut count = be32_to_cpup(p); let recvd = xdr_read_pages(xdr, count);
    if count > recvd { dprintk!("NFS: server cheating in read result: count %u > recvd %u\n", count, recvd); count = recvd; }
    (*result).eof = 0; (*result).count = count; count as i32
}
unsafe fn decode_stat(xdr: *mut xdr_stream, status: *mut nfs_stat) -> i32 {
    let p = xdr_inline_decode(xdr, 4); if p.is_null() { return -EIO; }
    if *p != cpu_to_be32(NFS_OK) { *status = be32_to_cpup(p); trace_nfs_xdr_status(xdr, *status as i32); } else { *status = 0; } 0
}
unsafe fn xdr_decode_ftype(mut p: *mut __be32, typ: *mut u32) -> *mut __be32 { *typ = be32_to_cpup(p); if *typ > NF2FIFO { *typ = NFBAD; } p.add(1) }
unsafe fn encode_fhandle(xdr: *mut xdr_stream, fh: *const nfs_fh) { let p=xdr_reserve_space(xdr,NFS2_FHSIZE); memcpy(p,(*fh).data.as_ptr() as *const _,NFS2_FHSIZE); }
unsafe fn decode_fhandle(xdr: *mut xdr_stream, fh: *mut nfs_fh) -> i32 { let p=xdr_inline_decode(xdr,NFS2_FHSIZE); if p.is_null(){return -EIO;} (*fh).size=NFS2_FHSIZE; memcpy((*fh).data.as_mut_ptr() as *mut _,p,NFS2_FHSIZE); 0 }
unsafe fn xdr_encode_time(mut p:*mut __be32,timep:*const timespec64)->*mut __be32 { *p=cpu_to_be32((*timep).tv_sec as u32); p=p.add(1); *p=cpu_to_be32(if (*timep).tv_nsec!=0 {(*timep).tv_nsec/NSEC_PER_USEC} else {0}); p.add(1) }
unsafe fn xdr_encode_current_server_time(mut p:*mut __be32,timep:*const timespec64)->*mut __be32 { *p=cpu_to_be32((*timep).tv_sec as u32); *p.add(1)=cpu_to_be32(1000000); p.add(2) }
unsafe fn xdr_decode_time(mut p:*mut __be32,t:*mut timespec64)->*mut __be32 { (*t).tv_sec=be32_to_cpup(p) as _; (*t).tv_nsec=be32_to_cpup(p.add(1)) as _ * NSEC_PER_USEC; p.add(2) }

unsafe fn decode_fattr(xdr:*mut xdr_stream,fattr:*mut nfs_fattr,userns:*mut user_namespace)->i32 {
    let mut p=xdr_inline_decode(xdr,NFS_fattr_sz<<2); if p.is_null(){return -EIO;} (*fattr).valid|=NFS_ATTR_FATTR_V2; let mut typ=0; p=xdr_decode_ftype(p,&mut typ);
    (*fattr).mode=be32_to_cpup(p); p=p.add(1); (*fattr).nlink=be32_to_cpup(p); p=p.add(1); (*fattr).uid=make_kuid(userns,be32_to_cpup(p)); p=p.add(1); if !uid_valid((*fattr).uid){return -EINVAL;} (*fattr).gid=make_kgid(userns,be32_to_cpup(p)); p=p.add(1); if !gid_valid((*fattr).gid){return -EINVAL;}
    (*fattr).size=be32_to_cpup(p); p=p.add(1); (*fattr).du.nfs2.blocksize=be32_to_cpup(p); p=p.add(1); let rdev=be32_to_cpup(p); p=p.add(1); (*fattr).rdev=new_decode_dev(rdev); if typ==NFCHR as u32 && rdev==NFS2_FIFO_DEV as u32 {(*fattr).mode=((*fattr).mode & !S_IFMT)|S_IFIFO;(*fattr).rdev=0;}
    (*fattr).du.nfs2.blocks=be32_to_cpup(p); p=p.add(1); (*fattr).fsid.major=be32_to_cpup(p); (*fattr).fsid.minor=0; p=p.add(1); (*fattr).fileid=be32_to_cpup(p); p=p.add(1); p=xdr_decode_time(p,&mut (*fattr).atime); p=xdr_decode_time(p,&mut (*fattr).mtime); xdr_decode_time(p,&mut (*fattr).ctime); (*fattr).change_attr=nfs_timespec_to_change_attr(&(*fattr).ctime); 0
}

const NFS2_SATTR_NOT_SET:u32=0xffffffff;
unsafe fn xdr_time_not_set(mut p:*mut __be32)->*mut __be32{*p=cpu_to_be32(NFS2_SATTR_NOT_SET);*p.add(1)=cpu_to_be32(NFS2_SATTR_NOT_SET);p.add(2)}
unsafe fn encode_sattr(xdr:*mut xdr_stream,attr:*const iattr,userns:*mut user_namespace){let mut p=xdr_reserve_space(xdr,NFS_sattr_sz<<2);*p=cpu_to_be32(if (*attr).ia_valid&ATTR_MODE!=0{(*attr).ia_mode}else{NFS2_SATTR_NOT_SET});p=p.add(1);*p=cpu_to_be32(if (*attr).ia_valid&ATTR_UID!=0{from_kuid_munged(userns,(*attr).ia_uid)}else{NFS2_SATTR_NOT_SET});p=p.add(1);*p=cpu_to_be32(if (*attr).ia_valid&ATTR_GID!=0{from_kgid_munged(userns,(*attr).ia_gid)}else{NFS2_SATTR_NOT_SET});p=p.add(1);*p=cpu_to_be32(if (*attr).ia_valid&ATTR_SIZE!=0{(*attr).ia_size as u32}else{NFS2_SATTR_NOT_SET});p=p.add(1);p=if (*attr).ia_valid&ATTR_ATIME_SET!=0{xdr_encode_time(p,&(*attr).ia_atime)}else if (*attr).ia_valid&ATTR_ATIME!=0{xdr_encode_current_server_time(p,&(*attr).ia_atime)}else{xdr_time_not_set(p)};if (*attr).ia_valid&ATTR_MTIME_SET!=0{ xdr_encode_time(p,&(*attr).ia_mtime);}else if (*attr).ia_valid&ATTR_MTIME!=0{xdr_encode_current_server_time(p,&(*attr).ia_mtime);}else{xdr_time_not_set(p);}}

unsafe fn encode_filename(xdr:*mut xdr_stream,name:*const i8,length:u32){WARN_ON_ONCE!(length>NFS2_MAXNAMLEN);let p=xdr_reserve_space(xdr,4+length);xdr_encode_opaque(p,name,length);}
unsafe fn decode_filename_inline(xdr:*mut xdr_stream,name:*mut *const i8,length:*mut u32)->i32{let p=xdr_inline_decode(xdr,4);if p.is_null(){return -EIO;}let count=be32_to_cpup(p);if count>NFS3_MAXNAMLEN{return -ENAMETOOLONG;}let p=xdr_inline_decode(xdr,count);if p.is_null(){return -EIO;}*name=p as *const i8;*length=count;0}
unsafe fn encode_path(xdr:*mut xdr_stream,pages:*mut *mut page,length:u32){let p=xdr_reserve_space(xdr,4);*p=cpu_to_be32(length);xdr_write_pages(xdr,pages,0,length);}
unsafe fn decode_path(xdr:*mut xdr_stream)->i32{let p=xdr_inline_decode(xdr,4);if p.is_null(){return -EIO;}let length=be32_to_cpup(p);if length>=(*(*xdr).buf).page_len||length>NFS_MAXPATHLEN{return -ENAMETOOLONG;}let recvd=xdr_read_pages(xdr,length);if length>recvd{return -EIO;}xdr_terminate_string((*xdr).buf,length);0}

unsafe fn decode_attrstat(xdr:*mut xdr_stream,result:*mut nfs_fattr,op_status:*mut u32,userns:*mut user_namespace)->i32{let mut s=0;let e=decode_stat(xdr,&mut s);if e!=0{return e;}if !op_status.is_null(){*op_status=s as u32;}if s!=NFS_OK{return nfs_stat_to_errno(s);}decode_fattr(xdr,result,userns)}
unsafe fn encode_diropargs(xdr:*mut xdr_stream,fh:*const nfs_fh,name:*const i8,length:u32){encode_fhandle(xdr,fh);encode_filename(xdr,name,length);}
unsafe fn decode_diropok(xdr:*mut xdr_stream,result:*mut nfs_diropok,userns:*mut user_namespace)->i32{let e=decode_fhandle(xdr,(*result).fh);if e!=0{return e;}decode_fattr(xdr,(*result).fattr,userns)}
unsafe fn decode_diropres(xdr:*mut xdr_stream,result:*mut nfs_diropok,userns:*mut user_namespace)->i32{let mut s=0;let e=decode_stat(xdr,&mut s);if e!=0{return e;}if s!=NFS_OK{return nfs_stat_to_errno(s);}decode_diropok(xdr,result,userns)}

// NFSv2 encode/decode entry points.
unsafe fn nfs2_xdr_enc_fhandle(_req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){encode_fhandle(xdr,data as *const nfs_fh);}
unsafe fn nfs2_xdr_enc_sattrargs(req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){let a=&*(data as *const nfs_sattrargs);encode_fhandle(xdr,a.fh);encode_sattr(xdr,a.sattr,rpc_rqst_userns(req));}
unsafe fn nfs2_xdr_enc_diropargs(_req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){let a=&*(data as *const nfs_diropargs);encode_diropargs(xdr,a.fh,a.name,a.len);}
unsafe fn nfs2_xdr_enc_readlinkargs(req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){let a=&*(data as *const nfs_readlinkargs);encode_fhandle(xdr,a.fh);rpc_prepare_reply_pages(req,a.pages,a.pgbase,a.pglen,NFS_readlinkres_sz-NFS_pagepad_sz);}
unsafe fn encode_readargs(xdr:*mut xdr_stream,a:*const nfs_pgio_args){encode_fhandle(xdr,(*a).fh);let p=xdr_reserve_space(xdr,12);*p=cpu_to_be32((*a).offset);*p.add(1)=cpu_to_be32((*a).count);*p.add(2)=cpu_to_be32((*a).count);}
unsafe fn nfs2_xdr_enc_readargs(req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){let a=data as *const nfs_pgio_args;encode_readargs(xdr,a);rpc_prepare_reply_pages(req,(*a).pages,(*a).pgbase,(*a).count,NFS_readres_sz-NFS_pagepad_sz);(*req).rq_rcv_buf.flags|=XDRBUF_READ;}
unsafe fn encode_writeargs(xdr:*mut xdr_stream,a:*const nfs_pgio_args){encode_fhandle(xdr,(*a).fh);let p=xdr_reserve_space(xdr,16);*p=cpu_to_be32((*a).offset);*p.add(1)=cpu_to_be32((*a).offset);*p.add(2)=cpu_to_be32((*a).count);*p.add(3)=cpu_to_be32((*a).count);xdr_write_pages(xdr,(*a).pages,(*a).pgbase,(*a).count);}
unsafe fn nfs2_xdr_enc_writeargs(_req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){encode_writeargs(xdr,data as *const nfs_pgio_args);(*xdr).buf.flags|=XDRBUF_WRITE;}
unsafe fn nfs2_xdr_enc_createargs(req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){let a=&*(data as *const nfs_createargs);encode_diropargs(xdr,a.fh,a.name,a.len);encode_sattr(xdr,a.sattr,rpc_rqst_userns(req));}
unsafe fn nfs2_xdr_enc_removeargs(_req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){let a=&*(data as *const nfs_removeargs);encode_diropargs(xdr,a.fh,a.name.name,a.name.len);}
unsafe fn nfs2_xdr_enc_renameargs(_req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){let a=&*(data as *const nfs_renameargs);encode_diropargs(xdr,a.old_dir,(*a.old_name).name,(*a.old_name).len);encode_diropargs(xdr,a.new_dir,(*a.new_name).name,(*a.new_name).len);}
unsafe fn nfs2_xdr_enc_linkargs(_req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){let a=&*(data as *const nfs_linkargs);encode_fhandle(xdr,a.fromfh);encode_diropargs(xdr,a.tofh,a.toname,a.tolen);}
unsafe fn nfs2_xdr_enc_symlinkargs(req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){let a=&*(data as *const nfs_symlinkargs);encode_diropargs(xdr,a.fromfh,a.fromname,a.fromlen);encode_path(xdr,a.pages,a.pathlen);encode_sattr(xdr,a.sattr,rpc_rqst_userns(req));}
unsafe fn encode_readdirargs(xdr:*mut xdr_stream,a:*const nfs_readdirargs){encode_fhandle(xdr,(*a).fh);let p=xdr_reserve_space(xdr,8);*p=cpu_to_be32((*a).cookie);*p.add(1)=cpu_to_be32((*a).count);}
unsafe fn nfs2_xdr_enc_readdirargs(req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*const c_void){let a=data as *const nfs_readdirargs;encode_readdirargs(xdr,a);rpc_prepare_reply_pages(req,(*a).pages,0,(*a).count,NFS_readdirres_sz-NFS_pagepad_sz);}

unsafe fn nfs2_xdr_dec_stat(_req:*mut rpc_rqst,xdr:*mut xdr_stream,_:*mut c_void)->i32{let mut s=0;let e=decode_stat(xdr,&mut s);if e!=0{e}else if s!=NFS_OK{nfs_stat_to_errno(s)}else{0}}
unsafe fn nfs2_xdr_dec_attrstat(req:*mut rpc_rqst,xdr:*mut xdr_stream,result:*mut c_void)->i32{decode_attrstat(xdr,result as *mut nfs_fattr,std::ptr::null_mut(),rpc_rqst_userns(req))}
unsafe fn nfs2_xdr_dec_diropres(req:*mut rpc_rqst,xdr:*mut xdr_stream,result:*mut c_void)->i32{decode_diropres(xdr,result as *mut nfs_diropok,rpc_rqst_userns(req))}
unsafe fn nfs2_xdr_dec_readlinkres(_req:*mut rpc_rqst,xdr:*mut xdr_stream,_:*mut c_void)->i32{let mut s=0;let e=decode_stat(xdr,&mut s);if e!=0{e}else if s!=NFS_OK{nfs_stat_to_errno(s)}else{decode_path(xdr)}}
unsafe fn nfs2_xdr_dec_readres(req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*mut c_void)->i32{let r=data as *mut nfs_pgio_res;let mut s=0;let e=decode_stat(xdr,&mut s);if e!=0{return e;}(*r).op_status=s;if s!=NFS_OK{return nfs_stat_to_errno(s);}let e=decode_fattr(xdr,(*r).fattr,rpc_rqst_userns(req));if e!=0{e}else{decode_nfsdata(xdr,r)}}
unsafe fn nfs2_xdr_dec_writeres(req:*mut rpc_rqst,xdr:*mut xdr_stream,data:*mut c_void)->i32{let r=data as *mut nfs_pgio_res;(*r).verf.committed=NFS_FILE_SYNC;decode_attrstat(xdr,(*r).fattr,&mut (*r).op_status,rpc_rqst_userns(req))}

pub unsafe fn nfs2_decode_dirent(xdr:*mut xdr_stream,entry:*mut nfs_entry,_plus:bool)->i32{let mut p=xdr_inline_decode(xdr,4);if p.is_null(){return -EAGAIN;}if *p==xdr_zero{p=xdr_inline_decode(xdr,4);if p.is_null()||*p==xdr_zero{return -EAGAIN;}(*entry).eof=1;return -EBADCOOKIE;}p=xdr_inline_decode(xdr,4);if p.is_null(){return -EAGAIN;}(*entry).ino=be32_to_cpup(p);let e=decode_filename_inline(xdr,&mut (*entry).name,&mut (*entry).len);if e!=0{return if e==-ENAMETOOLONG{-ENAMETOOLONG}else{-EAGAIN};}p=xdr_inline_decode(xdr,4);if p.is_null(){return -EAGAIN;}(*entry).cookie=be32_to_cpup(p);(*entry).d_type=DT_UNKNOWN;0}
unsafe fn decode_readdirok(xdr:*mut xdr_stream)->i32{xdr_read_pages(xdr,(*(*xdr).buf).page_len) as i32}
unsafe fn nfs2_xdr_dec_readdirres(_req:*mut rpc_rqst,xdr:*mut xdr_stream,_:*mut c_void)->i32{let mut s=0;let e=decode_stat(xdr,&mut s);if e!=0{e}else if s!=NFS_OK{nfs_stat_to_errno(s)}else{decode_readdirok(xdr)}}
unsafe fn decode_info(xdr:*mut xdr_stream,result:*mut nfs2_fsstat)->i32{let p=xdr_inline_decode(xdr,NFS_info_sz<<2);if p.is_null(){return -EIO;}(*result).tsize=be32_to_cpup(p);(*result).bsize=be32_to_cpup(p.add(1));(*result).blocks=be32_to_cpup(p.add(2));(*result).bfree=be32_to_cpup(p.add(3));(*result).bavail=be32_to_cpup(p.add(4));0}
unsafe fn nfs2_xdr_dec_statfsres(_req:*mut rpc_rqst,xdr:*mut xdr_stream,result:*mut c_void)->i32{let mut s=0;let e=decode_stat(xdr,&mut s);if e!=0{e}else if s!=NFS_OK{nfs_stat_to_errno(s)}else{decode_info(xdr,result as *mut nfs2_fsstat)}}

// The PROC macro expands to the following procedure table entries in C.
pub static mut nfs_procedures: [rpc_procinfo; 15] = [
    rpc_procinfo{p_proc:NFSPROC_GETATTR,p_encode:nfs2_xdr_enc_fhandle,p_decode:nfs2_xdr_dec_attrstat,p_arglen:NFS_fhandle_sz,p_replen:NFS_attrstat_sz,p_timer:1,p_statidx:NFSPROC_GETATTR,p_name:"GETATTR"},
    rpc_procinfo{p_proc:NFSPROC_SETATTR,p_encode:nfs2_xdr_enc_sattrargs,p_decode:nfs2_xdr_dec_attrstat,p_arglen:NFS_sattrargs_sz,p_replen:NFS_attrstat_sz,p_timer:0,p_statidx:NFSPROC_SETATTR,p_name:"SETATTR"},
    rpc_procinfo{p_proc:NFSPROC_LOOKUP,p_encode:nfs2_xdr_enc_diropargs,p_decode:nfs2_xdr_dec_diropres,p_arglen:NFS_diropargs_sz,p_replen:NFS_diropres_sz,p_timer:2,p_statidx:NFSPROC_LOOKUP,p_name:"LOOKUP"},
    rpc_procinfo{p_proc:NFSPROC_READLINK,p_encode:nfs2_xdr_enc_readlinkargs,p_decode:nfs2_xdr_dec_readlinkres,p_arglen:NFS_readlinkargs_sz,p_replen:NFS_readlinkres_sz,p_timer:3,p_statidx:NFSPROC_READLINK,p_name:"READLINK"},
    rpc_procinfo{p_proc:NFSPROC_READ,p_encode:nfs2_xdr_enc_readargs,p_decode:nfs2_xdr_dec_readres,p_arglen:NFS_readargs_sz,p_replen:NFS_readres_sz,p_timer:3,p_statidx:NFSPROC_READ,p_name:"READ"},
    rpc_procinfo{p_proc:NFSPROC_WRITE,p_encode:nfs2_xdr_enc_writeargs,p_decode:nfs2_xdr_dec_writeres,p_arglen:NFS_writeargs_sz,p_replen:NFS_writeres_sz,p_timer:4,p_statidx:NFSPROC_WRITE,p_name:"WRITE"},
    rpc_procinfo{p_proc:NFSPROC_CREATE,p_encode:nfs2_xdr_enc_createargs,p_decode:nfs2_xdr_dec_diropres,p_arglen:NFS_createargs_sz,p_replen:NFS_diropres_sz,p_timer:0,p_statidx:NFSPROC_CREATE,p_name:"CREATE"},
    rpc_procinfo{p_proc:nfs2_proc_remove(),p_encode:nfs2_xdr_enc_removeargs,p_decode:nfs2_xdr_dec_stat,p_arglen:NFS_removeargs_sz,p_replen:NFS_stat_sz,p_timer:0,p_statidx:nfs2_proc_remove(),p_name:"REMOVE"},
    rpc_procinfo{p_proc:nfs2_proc_rename(),p_encode:nfs2_xdr_enc_renameargs,p_decode:nfs2_xdr_dec_stat,p_arglen:NFS_renameargs_sz,p_replen:NFS_stat_sz,p_timer:0,p_statidx:nfs2_proc_rename(),p_name:"RENAME"},
    rpc_procinfo{p_proc:nfs2_proc_link(),p_encode:nfs2_xdr_enc_linkargs,p_decode:nfs2_xdr_dec_stat,p_arglen:NFS_linkargs_sz,p_replen:NFS_stat_sz,p_timer:0,p_statidx:nfs2_proc_link(),p_name:"LINK"},
    rpc_procinfo{p_proc:nfs2_proc_symlink(),p_encode:nfs2_xdr_enc_symlinkargs,p_decode:nfs2_xdr_dec_stat,p_arglen:NFS_symlinkargs_sz,p_replen:NFS_stat_sz,p_timer:0,p_statidx:nfs2_proc_symlink(),p_name:"SYMLINK"},
    rpc_procinfo{p_proc:nfs2_proc_mkdir(),p_encode:nfs2_xdr_enc_createargs,p_decode:nfs2_xdr_dec_diropres,p_arglen:NFS_createargs_sz,p_replen:NFS_diropres_sz,p_timer:0,p_statidx:nfs2_proc_mkdir(),p_name:"MKDIR"},
    rpc_procinfo{p_proc:nfs2_proc_rmdir(),p_encode:nfs2_xdr_enc_diropargs,p_decode:nfs2_xdr_dec_stat,p_arglen:NFS_diropargs_sz,p_replen:NFS_stat_sz,p_timer:0,p_statidx:nfs2_proc_rmdir(),p_name:"RMDIR"},
    rpc_procinfo{p_proc:nfs2_proc_readdir(),p_encode:nfs2_xdr_enc_readdirargs,p_decode:nfs2_xdr_dec_readdirres,p_arglen:NFS_readdirargs_sz,p_replen:NFS_readdirres_sz,p_timer:3,p_statidx:nfs2_proc_readdir(),p_name:"READDIR"},
    rpc_procinfo{p_proc:nfs2_proc_statfs(),p_encode:nfs2_xdr_enc_fhandle,p_decode:nfs2_xdr_dec_statfsres,p_arglen:NFS_fhandle_sz,p_replen:NFS_statfsres_sz,p_timer:0,p_statidx:nfs2_proc_statfs(),p_name:"STATFS"},
];
pub static mut nfs_version2_counts:[u32;15]=[0;15];
pub static mut nfs_version2:rpc_version=rpc_version{number:2,nrprocs:15,procs:nfs_procedures.as_ptr(),counts:nfs_version2_counts.as_mut_ptr()};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
