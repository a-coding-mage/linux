// SPDX-License-Identifier: GPL-2.0
// External Linux/NFS declarations are supplied by the surrounding translation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct inode { pub i_mode: u32, pub i_acl: *mut posix_acl, pub i_default_acl: *mut posix_acl }
#[repr(C)] pub struct posix_acl { pub a_count: u32 }
#[repr(C)] pub struct page;
#[repr(C)] pub struct nfs_server { pub client_acl: *mut rpc_clnt, pub caps: u64 }
#[repr(C)] pub struct rpc_clnt { pub cl_procinfo: *mut rpc_procinfo }
#[repr(C)] pub struct rpc_procinfo;
#[repr(C)] pub struct nfs_fattr;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct mnt_idmap;
#[repr(C)] pub struct rpc_message { pub rpc_argp: *mut c_void, pub rpc_resp: *mut c_void, pub rpc_proc: *mut rpc_procinfo }
#[repr(C)] pub struct nfs3_getaclargs { pub fh: *mut c_void, pub pages: *mut *mut page, pub mask: u32 }
#[repr(C)] pub struct nfs3_getaclres { pub fattr: *mut nfs_fattr, pub mask: u32, pub acl_access: *mut posix_acl, pub acl_default: *mut posix_acl }
#[repr(C)] pub struct nfs3_setaclargs { pub inode: *mut inode, pub mask: u32, pub acl_access: *mut posix_acl, pub pages: *mut *mut page, pub acl_default: *mut posix_acl, pub len: usize, pub npages: u32 }

extern "C" {
    static mut current: *mut c_void;
    static ACL_NOT_CACHED: *mut posix_acl;
    fn uncached_acl_sentinel(task: *mut c_void) -> *mut posix_acl;
    fn posix_acl_dup(acl: *mut posix_acl);
    fn posix_acl_release(acl: *mut posix_acl);
    fn posix_acl_equiv_mode(acl: *mut posix_acl, mode: *mut c_void) -> c_int;
    fn nfs_alloc_fattr() -> *mut nfs_fattr;
    fn nfs_free_fattr(fattr: *mut nfs_fattr);
    fn nfs_revalidate_inode(inode: *mut inode, flags: u64) -> c_int;
    fn nfs_server_capable(inode: *mut inode, cap: u32) -> bool;
    fn nfs_refresh_inode(inode: *mut inode, fattr: *mut nfs_fattr) -> c_int;
    fn rpc_call_sync(client: *mut rpc_clnt, msg: *mut rpc_message, flags: c_int) -> c_int;
    fn __free_page(page: *mut page);
    fn alloc_page(gfp: u32) -> *mut page;
    fn forget_cached_acl(inode: *mut inode, ty: c_int);
    fn nfs_access_zap_cache(inode: *mut inode);
    fn nfs_zap_acl_cache(inode: *mut inode);
    fn nfsacl_size(acl: *mut posix_acl, dfacl: *mut posix_acl) -> usize;
    fn get_inode_acl(inode: *mut inode, ty: c_int) -> *mut posix_acl;
    fn posix_acl_from_mode(mode: u32, gfp: u32) -> *mut posix_acl;
    fn d_inode(dentry: *mut dentry) -> *mut inode;
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
}

const ECHILD: c_int = 10; const EOPNOTSUPP: c_int = 95; const ENOMEM: c_int = 12;
const EIO: c_int = 5; const ENOSPC: c_int = 28; const ERANGE: c_int = 34;
const NFSACL_MAXPAGES: usize = 4; const NFS_ACL_MAX_ENTRIES: u32 = 1024;
const NFS_ACL_INLINE_BUFSIZE: usize = 256; const PAGE_SHIFT: usize = 12;
const NFS_CAP_ACLS: u32 = 1; const NFS_INO_INVALID_CHANGE: u64 = 1;
const NFS_ACLCNT: u32 = 1; const NFS_ACL: u32 = 2; const NFS_DFACLCNT: u32 = 4;
const NFS_DFACL: u32 = 8; const ACL_TYPE_ACCESS: c_int = 0; const ACL_TYPE_DEFAULT: c_int = 1;
const ACLPROC3_GETACL: usize = 1; const ACLPROC3_SETACL: usize = 2; const GFP_KERNEL: u32 = 0;

unsafe fn nfs3_prepare_get_acl(p: *mut *mut posix_acl) { cmpxchg(p, ACL_NOT_CACHED, uncached_acl_sentinel(current)); }
unsafe fn nfs3_complete_get_acl(p: *mut *mut posix_acl, acl: *mut posix_acl) {
    let sentinel = uncached_acl_sentinel(current); posix_acl_dup(acl);
    if cmpxchg(p, sentinel, acl) != sentinel { posix_acl_release(acl); }
}
unsafe fn nfs3_abort_get_acl(p: *mut *mut posix_acl) { cmpxchg(p, uncached_acl_sentinel(current), ACL_NOT_CACHED); }

unsafe fn cmpxchg(p: *mut *mut posix_acl, old: *mut posix_acl, new: *mut posix_acl) -> *mut posix_acl {
    let actual = *p; if actual == old { *p = new; } actual
}

pub unsafe fn nfs3_get_acl(inode: *mut inode, ty: c_int, rcu: bool) -> *mut posix_acl {
    if rcu { return (-ECHILD) as isize as *mut posix_acl; }
    if !nfs_server_capable(inode, NFS_CAP_ACLS) { return (-EOPNOTSUPP) as isize as *mut posix_acl; }
    let status = nfs_revalidate_inode(inode, NFS_INO_INVALID_CHANGE); if status < 0 { return status as isize as *mut posix_acl; }
    let mut pages = [core::ptr::null_mut(); NFSACL_MAXPAGES];
    let mut args = nfs3_getaclargs { fh: core::ptr::null_mut(), pages: pages.as_mut_ptr(), mask: 0 };
    if ty == ACL_TYPE_ACCESS { args.mask |= NFS_ACLCNT | NFS_ACL; }
    if ((*inode).i_mode & 0o170000) == 0o040000 { args.mask |= NFS_DFACLCNT | NFS_DFACL; }
    if args.mask == 0 { return core::ptr::null_mut(); }
    let mut res = nfs3_getaclres { fattr: nfs_alloc_fattr(), mask: 0, acl_access: core::ptr::null_mut(), acl_default: core::ptr::null_mut() };
    if res.fattr.is_null() { return (-ENOMEM) as isize as *mut posix_acl; }
    if args.mask & NFS_ACL != 0 { nfs3_prepare_get_acl(&mut (*inode).i_acl); }
    if args.mask & NFS_DFACL != 0 { nfs3_prepare_get_acl(&mut (*inode).i_default_acl); }
    let mut msg = rpc_message { rpc_argp: &mut args as *mut _ as *mut c_void, rpc_resp: &mut res as *mut _ as *mut c_void, rpc_proc: core::ptr::null_mut() };
    let status = rpc_call_sync((*NFS_SERVER(inode)).client_acl, &mut msg, 0);
    for p in pages.iter().take_while(|p| !p.is_null()) { __free_page(*p); }
    if status != 0 { nfs3_abort_get_acl(&mut (*inode).i_acl); nfs3_abort_get_acl(&mut (*inode).i_default_acl); posix_acl_release(res.acl_access); posix_acl_release(res.acl_default); nfs_free_fattr(res.fattr); return status as isize as *mut posix_acl; }
    if args.mask & res.mask != args.mask { nfs_free_fattr(res.fattr); return (-EIO) as isize as *mut posix_acl; }
    nfs_free_fattr(res.fattr); if ty == ACL_TYPE_ACCESS { posix_acl_release(res.acl_default); res.acl_access } else { posix_acl_release(res.acl_access); res.acl_default }
}

extern "C" { fn NFS_SERVER(inode: *mut inode) -> *mut nfs_server; }

pub unsafe fn nfs3_proc_setacls(inode: *mut inode, acl: *mut posix_acl, dfacl: *mut posix_acl) -> c_int { let ret = __nfs3_proc_setacls(inode, acl, dfacl); if ret == -EOPNOTSUPP { 0 } else { ret } }
unsafe fn __nfs3_proc_setacls(inode: *mut inode, acl: *mut posix_acl, dfacl: *mut posix_acl) -> c_int {
    if acl.is_null() && (((*inode).i_mode & 0o170000) != 0o040000 || dfacl.is_null()) { return 0; }
    if !nfs_server_capable(inode, NFS_CAP_ACLS) { return -EOPNOTSUPP; }
    if (!acl.is_null() && (*acl).a_count > NFS_ACL_MAX_ENTRIES) || (!dfacl.is_null() && (*dfacl).a_count > NFS_ACL_MAX_ENTRIES) { return -ENOSPC; }
    let mut pages = [core::ptr::null_mut(); NFSACL_MAXPAGES];
    let mut args = nfs3_setaclargs { inode, mask: NFS_ACL, acl_access: acl, pages: pages.as_mut_ptr(), acl_default: core::ptr::null_mut(), len: 0, npages: 0 };
    if ((*inode).i_mode & 0o170000) == 0o040000 { args.mask |= NFS_DFACL; args.acl_default = dfacl; args.len = nfsacl_size(acl, dfacl); } else { args.len = nfsacl_size(acl, core::ptr::null_mut()); }
    if args.len > NFS_ACL_INLINE_BUFSIZE { let npages = 1 + ((args.len - 1) >> PAGE_SHIFT); while (args.npages as usize) < npages { let p = alloc_page(GFP_KERNEL); if p.is_null() { while args.npages != 0 { args.npages -= 1; __free_page(*args.pages.add(args.npages as usize)); } return -ENOMEM; } *args.pages.add(args.npages as usize) = p; args.npages += 1; } }
    let mut fattr = nfs_alloc_fattr(); if fattr.is_null() { while args.npages != 0 { args.npages -= 1; __free_page(*args.pages.add(args.npages as usize)); } return -ENOMEM; }
    let mut msg = rpc_message { rpc_argp: &mut args as *mut _ as *mut c_void, rpc_resp: &mut fattr as *mut _ as *mut c_void, rpc_proc: core::ptr::null_mut() };
    let mut status = rpc_call_sync((*NFS_SERVER(inode)).client_acl, &mut msg, 0); nfs_access_zap_cache(inode); nfs_zap_acl_cache(inode);
    if status == 0 { status = nfs_refresh_inode(inode, fattr); } else if status == -ENOTSUPP { status = -EOPNOTSUPP; }
    nfs_free_fattr(fattr); while args.npages != 0 { args.npages -= 1; __free_page(*args.pages.add(args.npages as usize)); } status
}

pub unsafe fn nfs3_set_acl(_idmap: *mut mnt_idmap, dentry: *mut dentry, acl: *mut posix_acl, ty: c_int) -> c_int {
    let inode = d_inode(dentry); let orig = acl; let mut dfacl = core::ptr::null_mut(); let mut acl = acl;
    if ((*inode).i_mode & 0o170000) == 0o040000 { if ty == ACL_TYPE_ACCESS { dfacl = get_inode_acl(inode, ACL_TYPE_DEFAULT); } else if ty == ACL_TYPE_DEFAULT { dfacl = acl; acl = get_inode_acl(inode, ACL_TYPE_ACCESS); } }
    if acl.is_null() { acl = posix_acl_from_mode((*inode).i_mode, GFP_KERNEL); }
    let status = __nfs3_proc_setacls(inode, acl, dfacl); if acl != orig { posix_acl_release(acl); } if dfacl != orig { posix_acl_release(dfacl); } status
}

pub unsafe fn nfs3_listxattr(dentry: *mut dentry, data: *mut c_char, size: usize) -> isize {
    let inode = d_inode(dentry); let mut result = 0isize;
    for (ty, name) in [(ACL_TYPE_ACCESS, b"system.posix_acl_access\0"), (ACL_TYPE_DEFAULT, b"system.posix_acl_default\0")] {
        let acl = get_inode_acl(inode, ty); if acl.is_null() { continue; } posix_acl_release(acl); result += strlen(name.as_ptr() as *const c_char) as isize + 1;
        if size != 0 { if result as usize > size { return (-ERANGE) as isize; } strcpy(data.offset(result - strlen(name.as_ptr() as *const c_char) as isize - 1), name.as_ptr() as *const c_char); }
    } result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
