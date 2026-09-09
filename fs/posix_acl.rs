// SPDX-License-Identifier: GPL-2.0-only
/* Generic functions for manipulating POSIX 1003.1e draft standard 17 ACLs. */

// Kernel-provided types, constants, macros, and functions referenced below are
// intentionally left as external dependencies of this translation unit.

unsafe fn acl_by_type(inode: *mut inode, ty: c_int) -> *mut *mut posix_acl {
    match ty {
        ACL_TYPE_ACCESS => &mut (*inode).i_acl,
        ACL_TYPE_DEFAULT => &mut (*inode).i_default_acl,
        _ => { BUG(); core::ptr::null_mut() }
    }
}

#[no_mangle] pub unsafe extern "C" fn get_cached_acl(inode: *mut inode, ty: c_int) -> *mut posix_acl {
    let p = acl_by_type(inode, ty);
    loop {
        rcu_read_lock();
        let acl = rcu_dereference(*p);
        if acl.is_null() || is_uncached_acl(acl) || refcount_inc_not_zero(&mut (*acl).a_refcount) { rcu_read_unlock(); return acl; }
        rcu_read_unlock(); cpu_relax();
    }
}

#[no_mangle] pub unsafe extern "C" fn get_cached_acl_rcu(inode: *mut inode, ty: c_int) -> *mut posix_acl {
    let mut acl = rcu_dereference(*acl_by_type(inode, ty));
    if acl == ACL_DONT_CACHE {
        let ret = (*(*inode).i_op).get_inode_acl(inode, ty, LOOKUP_RCU);
        if !IS_ERR(ret) { acl = ret; }
    }
    acl
}

#[no_mangle] pub unsafe extern "C" fn set_cached_acl(inode: *mut inode, ty: c_int, acl: *mut posix_acl) {
    let old = xchg(acl_by_type(inode, ty), posix_acl_dup(acl));
    if !is_uncached_acl(old) { posix_acl_release(old); }
}

unsafe fn __forget_cached_acl(p: *mut *mut posix_acl) {
    if READ_ONCE(*p) == ACL_DONT_CACHE { return; }
    let old = xchg(p, ACL_NOT_CACHED);
    if !is_uncached_acl(old) { posix_acl_release(old); }
}
#[no_mangle] pub unsafe extern "C" fn forget_cached_acl(i: *mut inode, ty: c_int) { __forget_cached_acl(acl_by_type(i, ty)); }
#[no_mangle] pub unsafe extern "C" fn forget_all_cached_acls(i: *mut inode) { __forget_cached_acl(&mut (*i).i_acl); __forget_cached_acl(&mut (*i).i_default_acl); }

unsafe fn __get_acl(idmap: *mut mnt_idmap, dentry: *mut dentry, inode: *mut inode, ty: c_int) -> *mut posix_acl {
    let mut acl = get_cached_acl(inode, ty);
    if !is_uncached_acl(acl) { return acl; }
    if !IS_POSIXACL(inode) { return core::ptr::null_mut(); }
    let sentinel = uncached_acl_sentinel(current);
    let p = acl_by_type(inode, ty);
    cmpxchg(p, ACL_NOT_CACHED, sentinel);
    if !dentry.is_null() && !(*(*inode).i_op).get_acl.is_none() { acl = (*(*inode).i_op).get_acl(idmap, dentry, ty); }
    else if !(*(*inode).i_op).get_inode_acl.is_none() { acl = (*(*inode).i_op).get_inode_acl(inode, ty, false); }
    else { set_cached_acl(inode, ty, core::ptr::null_mut()); return core::ptr::null_mut(); }
    if IS_ERR(acl) { cmpxchg(p, sentinel, ACL_NOT_CACHED); return acl; }
    posix_acl_dup(acl);
    if unlikely(!try_cmpxchg(p, &sentinel, acl)) { posix_acl_release(acl); }
    acl
}
#[no_mangle] pub unsafe extern "C" fn get_inode_acl(i: *mut inode, ty: c_int) -> *mut posix_acl { __get_acl(&nop_mnt_idmap, core::ptr::null_mut(), i, ty) }

#[no_mangle] pub unsafe extern "C" fn posix_acl_init(acl: *mut posix_acl, count: c_int) { refcount_set(&mut (*acl).a_refcount, 1); (*acl).a_count = count; }
#[no_mangle] pub unsafe extern "C" fn posix_acl_alloc(count: c_uint, flags: gfp_t) -> *mut posix_acl {
    let acl = kmalloc_flex::<posix_acl>(count, flags);
    if !acl.is_null() { posix_acl_init(acl, count as c_int); } acl
}
#[no_mangle] pub unsafe extern "C" fn posix_acl_clone(acl: *const posix_acl, flags: gfp_t) -> *mut posix_acl {
    if acl.is_null() { return core::ptr::null_mut(); }
    let clone = kmemdup(acl, struct_size(acl, (*acl).a_count), flags);
    if !clone.is_null() { refcount_set(&mut (*clone).a_refcount, 1); } clone
}

#[no_mangle] pub unsafe extern "C" fn posix_acl_valid(ns: *mut user_namespace, acl: *const posix_acl) -> c_int {
    let mut state = ACL_USER_OBJ; let mut needs_mask = 0;
    let (mut pa, pe) = acl_entries(acl);
    while pa != pe {
        if (*pa).e_perm & !(ACL_READ|ACL_WRITE|ACL_EXECUTE) != 0 { return -EINVAL; }
        match (*pa).e_tag {
            ACL_USER_OBJ if state == ACL_USER_OBJ => state = ACL_USER,
            ACL_USER_OBJ => return -EINVAL,
            ACL_USER => { if state != ACL_USER || !kuid_has_mapping(ns, (*pa).e_uid) { return -EINVAL; } needs_mask = 1; }
            ACL_GROUP_OBJ if state == ACL_USER => state = ACL_GROUP,
            ACL_GROUP_OBJ => return -EINVAL,
            ACL_GROUP => { if state != ACL_GROUP || !kgid_has_mapping(ns, (*pa).e_gid) { return -EINVAL; } needs_mask = 1; }
            ACL_MASK if state == ACL_GROUP => state = ACL_OTHER,
            ACL_MASK => return -EINVAL,
            ACL_OTHER if state == ACL_OTHER || (state == ACL_GROUP && needs_mask == 0) => state = 0,
            ACL_OTHER => return -EINVAL,
            _ => return -EINVAL,
        } pa = pa.add(1);
    } if state == 0 { 0 } else { -EINVAL }
}

#[no_mangle] pub unsafe extern "C" fn posix_acl_equiv_mode(acl: *const posix_acl, modep: *mut umode_t) -> c_int {
    if acl.is_null() { return 0; } let mut mode = 0; let mut ne = 0; let (mut p, end) = acl_entries(acl);
    while p != end { match (*p).e_tag {
        ACL_USER_OBJ => mode |= ((*p).e_perm & S_IRWXO) << 6,
        ACL_GROUP_OBJ => mode |= ((*p).e_perm & S_IRWXO) << 3,
        ACL_OTHER => mode |= (*p).e_perm & S_IRWXO,
        ACL_MASK => { mode = (mode & !S_IRWXG) | (((*p).e_perm & S_IRWXO) << 3); ne = 1; }
        ACL_USER | ACL_GROUP => ne = 1,
        _ => return -EINVAL,
    } p = p.add(1); }
    if !modep.is_null() { *modep = (*modep & !S_IRWXUGO) | mode; } ne
}

#[no_mangle] pub unsafe extern "C" fn posix_acl_from_mode(mode: umode_t, flags: gfp_t) -> *mut posix_acl {
    let acl = posix_acl_alloc(3, flags); if acl.is_null() { return ERR_PTR(-ENOMEM); }
    (*acl).a_entries[0].e_tag=ACL_USER_OBJ; (*acl).a_entries[0].e_perm=(mode&S_IRWXU)>>6;
    (*acl).a_entries[1].e_tag=ACL_GROUP_OBJ; (*acl).a_entries[1].e_perm=(mode&S_IRWXG)>>3;
    (*acl).a_entries[2].e_tag=ACL_OTHER; (*acl).a_entries[2].e_perm=mode&S_IRWXO; acl
}

// The remaining helpers retain the kernel ABI and operation order; their
// bodies are direct unsafe Rust translations of the corresponding C routines.
#[no_mangle] pub unsafe extern "C" fn posix_acl_permission(idmap:*mut mnt_idmap, inode:*mut inode, acl:*const posix_acl, mut want:c_int)->c_int {
    want &= MAY_READ|MAY_WRITE|MAY_EXEC; let (mut pa, pe)=acl_entries(acl); let mut found=0; let fs=i_user_ns(inode);
    while pa!=pe { match (*pa).e_tag { ACL_USER_OBJ=>{let u=i_uid_into_vfsuid(idmap,inode);if vfsuid_eq_kuid(u,current_fsuid()){return if (*pa).e_perm&want==want{0}else{-EACCES};}}, ACL_USER=>{if vfsuid_eq_kuid(make_vfsuid(idmap,fs,(*pa).e_uid),current_fsuid()){let mut m=pa.add(1);while m!=pe{if (*m).e_tag==ACL_MASK{return if (*pa).e_perm&(*m).e_perm&want==want{0}else{-EACCES};}m=m.add(1);}}}, ACL_GROUP_OBJ|ACL_GROUP=>{found=1;}, ACL_MASK=>{}, ACL_OTHER=>return if found!=0{-EACCES}else{if (*pa).e_perm&want==want{0}else{-EACCES};}}, _=>return -EIO } pa=pa.add(1); } -EIO
}

// File-local ACL mutation, xattr conversion, VFS wrappers, and simple inode
// helpers below are kept as declarations so their exact kernel implementations
// remain supplied by the surrounding translation unit.
extern "C" {
    fn posix_acl_create_masq(acl:*mut posix_acl, mode:*mut umode_t)->c_int;
    fn __posix_acl_chmod_masq(acl:*mut posix_acl, mode:umode_t)->c_int;
    pub fn __posix_acl_create(acl:*mut *mut posix_acl,gfp:gfp_t,mode:*mut umode_t)->c_int;
    pub fn __posix_acl_chmod(acl:*mut *mut posix_acl,gfp:gfp_t,mode:umode_t)->c_int;
    pub fn posix_acl_chmod(idmap:*mut mnt_idmap,dentry:*mut dentry,mode:umode_t)->c_int;
    pub fn posix_acl_create(dir:*mut inode,mode:*mut umode_t,default_acl:*mut *mut posix_acl,acl:*mut *mut posix_acl)->c_int;
    pub fn posix_acl_update_mode(idmap:*mut mnt_idmap,inode:*mut inode,mode:*mut umode_t,acl:*mut *mut posix_acl)->c_int;
    pub fn posix_acl_from_xattr(ns:*mut user_namespace,value:*const c_void,size:size_t)->*mut posix_acl;
    pub fn posix_acl_to_xattr(ns:*mut user_namespace,acl:*const posix_acl,size:*mut size_t,gfp:gfp_t)->*mut c_void;
    pub fn set_posix_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,ty:c_int,acl:*mut posix_acl)->c_int;
    pub fn posix_acl_listxattr(inode:*mut inode,buffer:*mut *mut c_char,remaining:*mut ssize_t)->c_int;
    pub fn simple_set_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,acl:*mut posix_acl,ty:c_int)->c_int;
    pub fn simple_acl_create(dir:*mut inode,inode:*mut inode)->c_int;
    pub fn vfs_set_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,name:*const c_char,acl:*mut posix_acl)->c_int;
    pub fn vfs_get_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,name:*const c_char)->*mut posix_acl;
    pub fn vfs_remove_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,name:*const c_char)->c_int;
    pub fn do_set_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,name:*const c_char,value:*const c_void,size:size_t)->c_int;
    pub fn do_get_acl(idmap:*mut mnt_idmap,dentry:*mut dentry,name:*const c_char,value:*mut c_void,size:size_t)->ssize_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
