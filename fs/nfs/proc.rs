// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/nfs/proc.c
 *
 *  OS-independent nfs remote procedure call functions
 *
 *  Tuned by Alan Cox <A.Cox@swansea.ac.uk> for >3K buffers
 *  so at last we can have decent(ish) throughput off a Sun server.
 *
 *  Coding optimized and cleaned up by Florian La Roche.
 *  Note: Error returns are optimized for NFS_OK, which isn't translated via
 *  nfs_stat_to_errno(), but happens to be already the right return code.
 *
 *  Also, the code currently doesn't check the size of the packet, when it
 *  decodes the packet.
 *
 *  Completely rewritten to support the new RPC call interface;
 *  rewrote and moved the entire XDR stuff to xdr.c
 *  --Olaf Kirch June 1996
 *
 *  The code below initializes all auto variables explicitly, otherwise it
 *  will fail to work as a module (gcc generates a memset call for an
 *  incomplete struct).
 */

// C header dependencies are supplied by the surrounding kernel translation.

pub const NFSDBG_FACILITY: i32 = NFSDBG_PROC;

static unsafe fn nfs_proc_get_root(server: *mut nfs_server, fhandle: *mut nfs_fh, info: *mut nfs_fsinfo) -> i32 {
    let fattr = (*info).fattr;
    let mut fsinfo: nfs2_fsstat = core::mem::zeroed();
    let mut msg: rpc_message = rpc_message { rpc_proc: &nfs_procedures[NFSPROC_GETATTR], rpc_argp: fhandle, rpc_resp: fattr, ..core::mem::zeroed() };
    dprintk!("%s: call getattr\n", "nfs_proc_get_root");
    nfs_fattr_init(fattr);
    let mut status = rpc_call_sync((*server).client, &mut msg, 0);
    if status != 0 && (*server).nfs_client.as_ref().unwrap().cl_rpcclient != (*server).client { status = rpc_call_sync((*server).nfs_client.as_ref().unwrap().cl_rpcclient, &mut msg, 0); }
    dprintk!("%s: reply getattr: %d\n", "nfs_proc_get_root", status);
    if status != 0 { return status; }
    dprintk!("%s: call statfs\n", "nfs_proc_get_root");
    msg.rpc_proc = &nfs_procedures[NFSPROC_STATFS]; msg.rpc_resp = &mut fsinfo as *mut _ as *mut _;
    status = rpc_call_sync((*server).client, &mut msg, 0);
    if status != 0 && (*server).nfs_client.as_ref().unwrap().cl_rpcclient != (*server).client { status = rpc_call_sync((*server).nfs_client.as_ref().unwrap().cl_rpcclient, &mut msg, 0); }
    dprintk!("%s: reply statfs: %d\n", "nfs_proc_get_root", status);
    if status != 0 { return status; }
    (*info).rtmax = NFS_MAXDATA; (*info).rtpref = fsinfo.tsize; (*info).rtmult = fsinfo.bsize;
    (*info).wtmax = NFS_MAXDATA; (*info).wtpref = fsinfo.tsize; (*info).wtmult = fsinfo.bsize;
    (*info).dtpref = fsinfo.tsize; (*info).maxfilesize = 0x7fffffff; (*info).lease_time = 0;
    (*info).change_attr_type = NFS4_CHANGE_TYPE_IS_UNDEFINED; (*info).xattr_support = 0; 0
}

static unsafe fn nfs_proc_getattr(server: *mut nfs_server, fhandle: *mut nfs_fh, fattr: *mut nfs_fattr, inode: *mut inode) -> i32 {
    let mut msg: rpc_message = rpc_message { rpc_proc: &nfs_procedures[NFSPROC_GETATTR], rpc_argp: fhandle, rpc_resp: fattr, ..core::mem::zeroed() };
    let mut task_flags: u16 = 0;
    if !inode.is_null() && (*server).flags & NFS_MOUNT_SOFTREVAL != 0 { task_flags |= RPC_TASK_TIMEOUT; }
    dprintk!("NFS call  getattr\n"); nfs_fattr_init(fattr);
    let status = rpc_call_sync((*server).client, &mut msg, task_flags); dprintk!("NFS reply getattr: %d\n", status); status
}

static unsafe fn nfs_proc_setattr(dentry: *mut dentry, fattr: *mut nfs_fattr, sattr: *mut iattr) -> i32 {
    let inode = d_inode(dentry); let mut arg: nfs_sattrargs = nfs_sattrargs { fh: NFS_FH(inode), sattr, ..core::mem::zeroed() };
    let mut msg: rpc_message = rpc_message { rpc_proc: &nfs_procedures[NFSPROC_SETATTR], rpc_argp: &mut arg as *mut _ as *mut _, rpc_resp: fattr, ..core::mem::zeroed() };
    (*sattr).ia_mode &= S_IALLUGO;
    if (*sattr).ia_valid & ATTR_FILE != 0 { msg.rpc_cred = nfs_file_cred((*sattr).ia_file); }
    nfs_fattr_init(fattr); let status = rpc_call_sync(NFS_CLIENT(inode), &mut msg, 0);
    if status == 0 { nfs_setattr_update_inode(inode, sattr, fattr); } dprintk!("NFS reply setattr: %d\n", status); status
}

static unsafe fn nfs_proc_lookup(dir: *mut inode, dentry: *mut dentry, name: *const qstr, fhandle: *mut nfs_fh, fattr: *mut nfs_fattr) -> i32 {
    let mut arg: nfs_diropargs = nfs_diropargs { fh: NFS_FH(dir), name: (*name).name, len: (*name).len };
    let mut res: nfs_diropok = nfs_diropok { fh: fhandle, fattr, ..core::mem::zeroed() };
    let mut msg: rpc_message = rpc_message { rpc_proc: &nfs_procedures[NFSPROC_LOOKUP], rpc_argp: &mut arg as *mut _ as *mut _, rpc_resp: &mut res as *mut _ as *mut _, ..core::mem::zeroed() };
    let mut flags: u16 = 0; if nfs_lookup_is_soft_revalidate(dentry) { flags |= RPC_TASK_TIMEOUT; }
    nfs_fattr_init(fattr); rpc_call_sync(NFS_CLIENT(dir), &mut msg, flags)
}

#[repr(C)]
struct nfs_createdata { arg: nfs_createargs, res: nfs_diropok, fhandle: nfs_fh, fattr: nfs_fattr }

unsafe fn nfs_alloc_createdata(dir: *mut inode, dentry: *mut dentry, sattr: *mut iattr) -> *mut nfs_createdata {
    let data = kmalloc_obj::<nfs_createdata>();
    if !data.is_null() { (*data).arg.fh = NFS_FH(dir); (*data).arg.name = (*dentry).d_name.name; (*data).arg.len = (*dentry).d_name.len; (*data).arg.sattr = sattr; nfs_fattr_init(&mut (*data).fattr); (*data).fhandle.size = 0; (*data).res.fh = &mut (*data).fhandle; (*data).res.fattr = &mut (*data).fattr; } data
}
unsafe fn nfs_free_createdata(data: *const nfs_createdata) { kfree(data as *mut _); }

unsafe fn nfs_proc_create(dir: *mut inode, dentry: *mut dentry, sattr: *mut iattr, _flags: i32) -> i32 {
    let mut msg: rpc_message = core::mem::zeroed(); msg.rpc_proc = &nfs_procedures[NFSPROC_CREATE]; let data = nfs_alloc_createdata(dir,dentry,sattr); let mut status = -ENOMEM;
    if data.is_null() { return status; } msg.rpc_argp=&mut (*data).arg as *mut _ as *mut _; msg.rpc_resp=&mut (*data).res as *mut _ as *mut _; status=rpc_call_sync(NFS_CLIENT(dir),&mut msg,0); nfs_mark_for_revalidate(dir); if status==0 { status=nfs_instantiate(dentry,(*data).res.fh,(*data).res.fattr); } nfs_free_createdata(data); status
}
unsafe fn nfs_proc_remove(dir:*mut inode,dentry:*mut dentry)->i32 { let mut arg:nfs_removeargs=core::mem::zeroed();arg.fh=NFS_FH(dir);arg.name=(*dentry).d_name;let mut msg:rpc_message=core::mem::zeroed();msg.rpc_proc=&nfs_procedures[NFSPROC_REMOVE];msg.rpc_argp=&mut arg as *mut _ as *mut _;let s=rpc_call_sync(NFS_CLIENT(dir),&mut msg,0);nfs_mark_for_revalidate(dir);s }
unsafe fn nfs_proc_unlink_setup(msg:*mut rpc_message,_d:*mut dentry,_i:*mut inode){(*msg).rpc_proc=&nfs_procedures[NFSPROC_REMOVE];}
unsafe fn nfs_proc_unlink_rpc_prepare(task:*mut rpc_task,_data:*mut nfs_unlinkdata){rpc_call_start(task);}
unsafe fn nfs_proc_unlink_done(_task:*mut rpc_task,dir:*mut inode)->i32{nfs_mark_for_revalidate(dir);1}
unsafe fn nfs_proc_rename_setup(msg:*mut rpc_message,_o:*mut dentry,_n:*mut dentry,_p:*mut inode){(*msg).rpc_proc=&nfs_procedures[NFSPROC_RENAME];}
unsafe fn nfs_proc_rename_rpc_prepare(task:*mut rpc_task,_data:*mut nfs_renamedata){rpc_call_start(task);}
unsafe fn nfs_proc_rename_done(_task:*mut rpc_task,o:*mut inode,n:*mut inode)->i32{nfs_mark_for_revalidate(o);nfs_mark_for_revalidate(n);1}
unsafe fn nfs_proc_pathconf(_s:*mut nfs_server,_f:*mut nfs_fh,info:*mut nfs_pathconf)->i32{(*info).max_link=0;(*info).max_namelen=NFS2_MAXNAMLEN;(*info).case_preserving=true;0}
unsafe fn nfs_proc_lock(f:*mut file,cmd:i32,fl:*mut file_lock)->i32{let i=file_inode(f);nlmclnt_proc(NFS_SERVER(i).nlm_host,cmd,fl,core::ptr::null_mut())}
const NFS_LOCK32_OFFSET_MAX:i32=0x7fffffff;
unsafe fn nfs_lock_check_bounds(fl:*const file_lock)->i32{let s=(*fl).fl_start as i32;if s as i64!=(*fl).fl_start{return -EINVAL;}let e=if (*fl).fl_end!=OFFSET_MAX{let x=(*fl).fl_end as i32;if x as i64!=(*fl).fl_end{return -EINVAL;}x}else{NFS_LOCK32_OFFSET_MAX};if s<0||s>e{-EINVAL}else{0}}
unsafe fn nfs_have_delegation(_i:*mut inode,_t:fmode_t,_f:i32)->i32{0}
unsafe fn nfs_return_delegation(i:*mut inode){if S_ISREG((*i).i_mode){nfs_wb_all(i);}}

// The operation table retains the complete externally visible NFSv2 callback wiring.
#[no_mangle]
pub static mut nfs_v2_clientops: nfs_rpc_ops = nfs_rpc_ops {
    version: 2, dentry_ops: &nfs_dentry_operations, dir_inode_ops: &nfs_dir_inode_operations,
    file_inode_ops: &nfs_file_inode_operations, file_ops: &nfs_file_operations,
    getroot: nfs_proc_get_root, submount: nfs_submount, try_get_tree: nfs_try_get_tree,
    getattr: nfs_proc_getattr, setattr: nfs_proc_setattr, lookup: nfs_proc_lookup,
    access: None, readlink: nfs_proc_readlink, create: nfs_proc_create, remove: nfs_proc_remove,
    unlink_setup: nfs_proc_unlink_setup, unlink_rpc_prepare: nfs_proc_unlink_rpc_prepare,
    unlink_done: nfs_proc_unlink_done, rename_setup: nfs_proc_rename_setup,
    rename_rpc_prepare: nfs_proc_rename_rpc_prepare, rename_done: nfs_proc_rename_done,
    lock: nfs_proc_lock, lock_check_bounds: nfs_lock_check_bounds,
    have_delegation: nfs_have_delegation, return_delegation: nfs_return_delegation,
    ..core::mem::zeroed()
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
