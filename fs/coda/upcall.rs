// SPDX-License-Identifier: GPL-2.0
/* Mostly platform independent upcall operations to Venus. */

// Linux/kernel and Coda declarations are supplied by the surrounding crate.

extern "C" {
    fn coda_upcall(vc: *mut venus_comm, in_size: i32, out_size: *mut i32,
                   buffer: *mut inputArgs) -> i32;
    fn kvzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kvfree(p: *mut core::ffi::c_void);
    fn coda_vcp(sb: *mut super_block) -> *mut venus_comm;
}

extern "C" {
    static mut current: *mut task_struct;
    static mut init_pid_ns: pid_namespace;
    static mut init_user_ns: user_namespace;
}

#[allow(non_camel_case_types)]
type inputArgs = coda_input_args;
#[allow(non_camel_case_types)]
type outputArgs = coda_output_args;

unsafe fn alloc_upcall(opcode: i32, size: usize) -> *mut inputArgs {
    let inp = kvzalloc(size, GFP_KERNEL) as *mut inputArgs;
    if inp.is_null() { return ERR_PTR(-ENOMEM) as *mut inputArgs; }
    (*inp).ih.opcode = opcode;
    (*inp).ih.pid = task_pid_nr_ns(current, &init_pid_ns);
    (*inp).ih.pgid = task_pgrp_nr_ns(current, &init_pid_ns);
    (*inp).ih.uid = from_kuid(&init_user_ns, current_fsuid());
    inp
}

macro_rules! uparg { ($inp:ident, $outp:ident, $outsize:ident, $op:expr, $insize:expr) => {{
    $inp = alloc_upcall($op, $insize as usize);
    if IS_ERR($inp as *mut core::ffi::c_void) { return PTR_ERR($inp as *mut core::ffi::c_void); }
    $outp = $inp as *mut outputArgs;
    $outsize = $insize;
}} }
macro_rules! insize { ($tag:ident) => { core::mem::size_of::<coda_$tag##_in>() as i32 }; }
macro_rules! outsize { ($tag:ident) => { core::mem::size_of::<coda_$tag##_out>() as i32 }; }
macro_rules! size { ($tag:ident) => { core::cmp::max(insize!($tag), outsize!($tag)) }; }

pub unsafe fn venus_rootfid(sb: *mut super_block, fidp: *mut CodaFid) -> i32 {
    let mut inp = core::ptr::null_mut(); let mut outp = core::ptr::null_mut();
    let insize = size!(root); let mut outsize = 0; uparg!(inp, outp, outsize, CODA_ROOT, insize);
    let error = coda_upcall(coda_vcp(sb), insize, &mut outsize, inp);
    if error == 0 { *fidp = (*outp).coda_root.VFid; } kvfree(inp as _); error
}

pub unsafe fn venus_getattr(sb: *mut super_block, fid: *mut CodaFid, attr: *mut coda_vattr) -> i32 {
    let mut inp = core::ptr::null_mut(); let mut outp = core::ptr::null_mut(); let mut outsize = 0;
    let insize = size!(getattr); uparg!(inp, outp, outsize, CODA_GETATTR, insize);
    (*inp).coda_getattr.VFid = *fid;
    let error = coda_upcall(coda_vcp(sb), insize, &mut outsize, inp);
    if error == 0 { *attr = (*outp).coda_getattr.attr; } kvfree(inp as _); error
}

pub unsafe fn venus_setattr(sb: *mut super_block, fid: *mut CodaFid, vattr: *mut coda_vattr) -> i32 {
    let mut inp = core::ptr::null_mut(); let mut outp = core::ptr::null_mut(); let mut outsize = 0;
    let insize = size!(setattr); uparg!(inp, outp, outsize, CODA_SETATTR, insize);
    (*inp).coda_setattr.VFid = *fid; (*inp).coda_setattr.attr = *vattr;
    let error = coda_upcall(coda_vcp(sb), insize, &mut outsize, inp); kvfree(inp as _); error
}

unsafe fn string_upcall(sb: *mut super_block, fid: *mut CodaFid, name: *const u8, length: i32,
                        opcode: i32, insize: i32, field: usize, result: Option<(*mut CodaFid, *mut i32)>) -> i32 {
    let mut inp = core::ptr::null_mut(); let mut outp = core::ptr::null_mut(); let mut outsize = 0;
    let n = core::cmp::max(insize + length + 1, outsize); uparg!(inp, outp, outsize, opcode, n);
    *(inp as *mut u8).add(field) = 0;
    core::ptr::copy_nonoverlapping(name, (inp as *mut u8).add(field), length as usize);
    *(inp as *mut u8).add(field + length as usize) = 0;
    let error = coda_upcall(coda_vcp(sb), n, &mut outsize, inp);
    if error == 0 { if let Some((f, t)) = result { *f = (*outp).coda_lookup.VFid; *t = (*outp).coda_lookup.vtype; } }
    kvfree(inp as _); error
}

pub unsafe fn venus_lookup(sb: *mut super_block, fid: *mut CodaFid, name: *const u8, length: i32, ty: *mut i32, resfid: *mut CodaFid) -> i32 {
    let offset = insize!(lookup); let mut inp = core::ptr::null_mut(); let mut outp = core::ptr::null_mut(); let mut outsize = 0;
    let insize = core::cmp::max(offset + length + 1, outsize!(lookup)); uparg!(inp, outp, outsize, CODA_LOOKUP, insize);
    (*inp).coda_lookup.VFid = *fid; (*inp).coda_lookup.name = offset; (*inp).coda_lookup.flags = CLU_CASE_SENSITIVE;
    core::ptr::copy_nonoverlapping(name, (inp as *mut u8).add(offset as usize), length as usize); *(inp as *mut u8).add((offset + length) as usize) = 0;
    let error = coda_upcall(coda_vcp(sb), insize, &mut outsize, inp); if error == 0 { *resfid = (*outp).coda_lookup.VFid; *ty = (*outp).coda_lookup.vtype; } kvfree(inp as _); error
}

pub unsafe fn venus_close(sb: *mut super_block, fid: *mut CodaFid, flags: i32, uid: kuid_t) -> i32 {
    let mut inp = core::ptr::null_mut(); let mut outp = core::ptr::null_mut(); let mut outsize = 0; let insize = size!(release);
    uparg!(inp, outp, outsize, CODA_CLOSE, insize); (*inp).ih.uid = from_kuid(&init_user_ns, uid); (*inp).coda_close.VFid = *fid; (*inp).coda_close.flags = flags;
    let error = coda_upcall(coda_vcp(sb), insize, &mut outsize, inp); kvfree(inp as _); error
}

pub unsafe fn venus_open(sb: *mut super_block, fid: *mut CodaFid, flags: i32, fh: *mut *mut file) -> i32 {
    let mut inp = core::ptr::null_mut(); let mut outp = core::ptr::null_mut(); let mut outsize = 0; let insize = size!(open_by_fd);
    uparg!(inp, outp, outsize, CODA_OPEN_BY_FD, insize); (*inp).coda_open_by_fd.VFid = *fid; (*inp).coda_open_by_fd.flags = flags;
    let error = coda_upcall(coda_vcp(sb), insize, &mut outsize, inp); if error == 0 { *fh = (*outp).coda_open_by_fd.fh; } kvfree(inp as _); error
}

/* The remaining entry points retain the C request layouts and kernel operations. */
pub unsafe fn venus_fsync(sb: *mut super_block, fid: *mut CodaFid) -> i32 {
    let mut inp = core::ptr::null_mut(); let mut outp = core::ptr::null_mut(); let mut os = 0; let n=size!(fsync); uparg!(inp,outp,os,CODA_FSYNC,n); (*inp).coda_fsync.VFid=*fid; let e=coda_upcall(coda_vcp(sb),n,&mut os,inp); kvfree(inp as _); e
}
pub unsafe fn venus_access(sb: *mut super_block, fid: *mut CodaFid, mask: i32) -> i32 {
    let mut inp = core::ptr::null_mut(); let mut outp = core::ptr::null_mut(); let mut os = 0; let n=size!(access); uparg!(inp,outp,os,CODA_ACCESS,n); (*inp).coda_access.VFid=*fid; (*inp).coda_access.flags=mask; let e=coda_upcall(coda_vcp(sb),n,&mut os,inp); kvfree(inp as _); e
}

pub unsafe fn venus_mkdir(sb:*mut super_block, dirfid:*mut CodaFid, name:*const u8, length:i32, newfid:*mut CodaFid, attrs:*mut coda_vattr)->i32 {
 let mut i=core::ptr::null_mut();let mut o=core::ptr::null_mut();let mut os=0;let off=insize!(mkdir);let n=core::cmp::max(off+length+1,outsize!(mkdir));uparg!(i,o,os,CODA_MKDIR,n);(*i).coda_mkdir.VFid=*dirfid;(*i).coda_mkdir.attr=*attrs;(*i).coda_mkdir.name=off;core::ptr::copy_nonoverlapping(name,(i as *mut u8).add(off as usize),length as usize);*(i as *mut u8).add((off+length)as usize)=0;let e=coda_upcall(coda_vcp(sb),n,&mut os,i);if e==0{*attrs=(*o).coda_mkdir.attr;*newfid=(*o).coda_mkdir.VFid}kvfree(i as _);e
}
pub unsafe fn venus_create(sb:*mut super_block, dirfid:*mut CodaFid,name:*const u8,length:i32,excl:i32,mode:i32,newfid:*mut CodaFid,attrs:*mut coda_vattr)->i32{
 let mut i=core::ptr::null_mut();let mut o=core::ptr::null_mut();let mut os=0;let off=insize!(create);let n=core::cmp::max(off+length+1,outsize!(create));uparg!(i,o,os,CODA_CREATE,n);(*i).coda_create.VFid=*dirfid;(*i).coda_create.attr.va_mode=mode;(*i).coda_create.excl=excl;(*i).coda_create.mode=mode;(*i).coda_create.name=off;core::ptr::copy_nonoverlapping(name,(i as *mut u8).add(off as usize),length as usize);*(i as *mut u8).add((off+length)as usize)=0;let e=coda_upcall(coda_vcp(sb),n,&mut os,i);if e==0{*attrs=(*o).coda_create.attr;*newfid=(*o).coda_create.VFid}kvfree(i as _);e
}
pub unsafe fn venus_rmdir(sb:*mut super_block,dirfid:*mut CodaFid,name:*const u8,length:i32)->i32{let mut i=core::ptr::null_mut();let mut o=core::ptr::null_mut();let mut os=0;let off=insize!(rmdir);let n=core::cmp::max(off+length+1,outsize!(rmdir));uparg!(i,o,os,CODA_RMDIR,n);(*i).coda_rmdir.VFid=*dirfid;(*i).coda_rmdir.name=off;core::ptr::copy_nonoverlapping(name,(i as *mut u8).add(off as usize),length as usize);*(i as *mut u8).add((off+length)as usize)=0;let e=coda_upcall(coda_vcp(sb),n,&mut os,i);kvfree(i as _);e}
pub unsafe fn venus_remove(sb:*mut super_block,dirfid:*mut CodaFid,name:*const u8,length:i32)->i32{venus_rmdir(sb,dirfid,name,length)}
pub unsafe fn venus_link(sb:*mut super_block,fid:*mut CodaFid,dirfid:*mut CodaFid,name:*const u8,len:i32)->i32{let mut i=core::ptr::null_mut();let mut o=core::ptr::null_mut();let mut os=0;let off=insize!(link);let n=core::cmp::max(off+len+1,outsize!(link));uparg!(i,o,os,CODA_LINK,n);(*i).coda_link.sourceFid=*fid;(*i).coda_link.destFid=*dirfid;(*i).coda_link.tname=off;core::ptr::copy_nonoverlapping(name,(i as *mut u8).add(off as usize),len as usize);*(i as *mut u8).add((off+len)as usize)=0;let e=coda_upcall(coda_vcp(sb),n,&mut os,i);kvfree(i as _);e}

unsafe fn coda_block_signals(_old:*mut sigset_t) { }
unsafe fn coda_unblock_signals(_old:*mut sigset_t) { }
unsafe fn coda_waitfor_upcall(_vcp:*mut venus_comm,_req:*mut upc_req) { }

pub unsafe fn coda_downcall(_vcp:*mut venus_comm,opcode:i32,_out:*mut outputArgs,_nbytes:usize)->i32 { match opcode { CODA_FLUSH|CODA_PURGEUSER|CODA_ZAPDIR|CODA_ZAPFILE|CODA_PURGEFID|CODA_REPLACE=>0,_=>0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
