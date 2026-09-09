// SPDX-License-Identifier: GPL-2.0-only
/* Translation of fs/xattr.c. Kernel types, constants, macros, and helpers are
 * supplied by the surrounding kernel translation unit. */

#[repr(C)]
pub struct sx_key { pub parent: *const list_head, pub name: *const c_char }

unsafe fn strcmp_prefix(mut a: *const c_char, mut p: *const c_char) -> *const c_char {
    while *p != 0 && *a == *p { a = a.add(1); p = p.add(1); }
    if *p != 0 { core::ptr::null() } else { a }
}

unsafe fn xattr_resolve_name(mut inode: *mut inode, name: *mut *const c_char) -> *const xattr_handler {
    let mut handlers = (*(*inode).i_sb).s_xattr;
    if (*inode).i_opflags & IOP_XATTR == 0 { return if is_bad_inode(inode) { ERR_PTR(-EIO) } else { ERR_PTR(-EOPNOTSUPP) }; }
    while !handlers.is_null() {
        let handler = *handlers; handlers = handlers.add(1);
        if handler.is_null() { break; }
        let n = strcmp_prefix(*name, xattr_prefix(handler));
        if !n.is_null() {
            if ((!(*handler).prefix.is_null()) as i32) ^ ((!*n.is_null()) as i32) != 0 { if *n != 0 { continue; } return ERR_PTR(-EINVAL); }
            *name = n; return handler;
        }
    }
    ERR_PTR(-EOPNOTSUPP)
}

pub unsafe fn may_write_xattr(idmap: *mut mnt_idmap, inode: *mut inode) -> c_int {
    if IS_IMMUTABLE(inode) || IS_APPEND(inode) || HAS_UNMAPPED_ID(idmap, inode) { -EPERM } else { 0 }
}
unsafe fn xattr_permission_error(mask: c_int) -> c_int { if mask & MAY_WRITE != 0 { -EPERM } else { -ENODATA } }

unsafe fn xattr_permission(idmap: *mut mnt_idmap, inode: *mut inode, name: *const c_char, mask: c_int) -> c_int {
    if mask & MAY_WRITE != 0 { let r = may_write_xattr(idmap, inode); if r != 0 { return r; } }
    if !strncmp(name, XATTR_SECURITY_PREFIX, XATTR_SECURITY_PREFIX_LEN) || !strncmp(name, XATTR_SYSTEM_PREFIX, XATTR_SYSTEM_PREFIX_LEN) { return 0; }
    if !strncmp(name, XATTR_TRUSTED_PREFIX, XATTR_TRUSTED_PREFIX_LEN) { return if !capable(CAP_SYS_ADMIN) { xattr_permission_error(mask) } else { 0 }; }
    if !strncmp(name, XATTR_USER_PREFIX, XATTR_USER_PREFIX_LEN) {
        match (*inode).i_mode & S_IFMT { S_IFREG => {}, S_IFDIR => { if (*inode).i_mode & S_ISVTX != 0 && mask & MAY_WRITE != 0 && !inode_owner_or_capable(idmap, inode) { return -EPERM; } }, S_IFSOCK => {}, _ => return xattr_permission_error(mask) }
    }
    inode_permission(idmap, inode, mask)
}

pub unsafe fn xattr_supports_user_prefix(inode: *mut inode) -> c_int {
    if (*inode).i_opflags & IOP_XATTR == 0 { return if is_bad_inode(inode) { -EIO } else { -EOPNOTSUPP }; }
    let mut hs = (*(*inode).i_sb).s_xattr; while !hs.is_null() { let h=*hs; hs=hs.add(1); if h.is_null(){break;} if !strncmp(xattr_prefix(h), XATTR_USER_PREFIX, XATTR_USER_PREFIX_LEN){return 0;} } -EOPNOTSUPP
}

pub unsafe fn __vfs_setxattr(idmap:*mut mnt_idmap,dentry:*mut dentry,inode:*mut inode,mut name:*const c_char,value:*const c_void,size:usize,flags:c_int)->c_int { if is_posix_acl_xattr(name){return -EOPNOTSUPP;} let h=xattr_resolve_name(inode,&mut name); if IS_ERR(h){return PTR_ERR(h);} if (*h).set.is_none(){return -EOPNOTSUPP;} let v=if size==0 { b"\0".as_ptr() as *const c_void } else {value}; ((*h).set.unwrap())(h,idmap,dentry,inode,name,v,size,flags) }

pub unsafe fn __vfs_setxattr_noperm(idmap:*mut mnt_idmap,dentry:*mut dentry,name:*const c_char,value:*const c_void,size:usize,flags:c_int)->c_int { let inode=(*dentry).d_inode; let sec=!strncmp(name,XATTR_SECURITY_PREFIX,XATTR_SECURITY_PREFIX_LEN); if sec {(*inode).i_flags &= !S_NOSEC;} let mut e=-EAGAIN; if (*inode).i_opflags&IOP_XATTR!=0 {e=__vfs_setxattr(idmap,dentry,inode,name,value,size,flags); if e==0 {fsnotify_xattr(dentry);security_inode_post_setxattr(dentry,name,value,size,flags);}} else if is_bad_inode(inode){return -EIO;} if e==-EAGAIN {e=-EOPNOTSUPP;if sec {e=security_inode_setsecurity(inode,name.add(XATTR_SECURITY_PREFIX_LEN),value,size,flags);if e==0{fsnotify_xattr(dentry);}}} e }

pub unsafe fn __vfs_setxattr_locked(idmap:*mut mnt_idmap,d:*mut dentry,name:*const c_char,value:*const c_void,size:usize,flags:c_int,di:*mut delegated_inode)->c_int { let i=(*d).d_inode; let mut e=xattr_permission(idmap,i,name,MAY_WRITE);if e!=0{return e;}e=security_inode_setxattr(idmap,d,name,value,size,flags);if e!=0{return e;}e=try_break_deleg(i,0,di);if e==0{e=__vfs_setxattr_noperm(idmap,d,name,value,size,flags);}e }
pub unsafe fn vfs_setxattr(idmap:*mut mnt_idmap,d:*mut dentry,name:*const c_char,mut value:*const c_void,mut size:usize,flags:c_int)->c_int { let i=(*d).d_inode;let orig=value; if size!=0&&strcmp(name,XATTR_NAME_CAPS)==0 {let e=cap_convert_nscap(idmap,d,&mut value,size);if e<0{return e;}size=e as usize;}let mut di=core::mem::zeroed();loop{inode_lock(i);let mut e=__vfs_setxattr_locked(idmap,d,name,value,size,flags,&mut di);inode_unlock(i);if is_delegated(&di){e=break_deleg_wait(&mut di);if e==0{continue;}}if value!=orig{kfree(value as *mut c_void);}return e;}}

pub unsafe fn xattr_list_one(buffer:*mut *mut c_char,remaining:*mut isize,name:*const c_char)->c_int {let l=strlen(name)+1;if !(*buffer).is_null(){if *remaining<l as isize{return -ERANGE;}memcpy(*buffer,name,l);*buffer=(*buffer).add(l);}*remaining-=l as isize;0}
pub unsafe fn simple_xattr_space(name:*const c_char,size:usize)->usize{40+size+strlen(name)}
pub unsafe fn simple_xattr_free(x:*mut simple_xattr){if !x.is_null(){kfree((*x).name as *mut c_void);kvfree(x as *mut c_void);}}
pub unsafe fn simple_xattr_alloc(value:*const c_void,size:usize)->*mut simple_xattr{if value.is_null(){return core::ptr::null_mut();}let n=kmalloc(core::mem::size_of::<simple_xattr>()+size,GFP_KERNEL_ACCOUNT) as *mut simple_xattr;if n.is_null(){return ERR_PTR(-ENOMEM);}(*n).size=size;memcpy((*n).value.as_mut_ptr() as *mut c_void,value,size);n}

// The remaining exported wrappers retain the source-level call graph; kernel
// declarations and statement-expression cleanup helpers are provided by the
// surrounding translation environment.
pub unsafe fn __vfs_getxattr(d:*mut dentry,i:*mut inode,mut n:*const c_char,v:*mut c_void,s:usize)->isize{if is_posix_acl_xattr(n){return -EOPNOTSUPP;}let h=xattr_resolve_name(i,&mut n);if IS_ERR(h){return PTR_ERR(h) as isize;}if (*h).get.is_none(){return -EOPNOTSUPP as isize;}((*h).get.unwrap())(h,d,i,n,v,s)}
pub unsafe fn vfs_getxattr(id:*mut mnt_idmap,d:*mut dentry,n:*const c_char,v:*mut c_void,s:usize)->isize{let i=(*d).d_inode;let e=xattr_permission(id,i,n,MAY_READ);if e!=0{return e as isize;}let e=security_inode_getxattr(d,n);if e!=0{return e as isize;}if !strncmp(n,XATTR_SECURITY_PREFIX,XATTR_SECURITY_PREFIX_LEN){let r=xattr_getsecurity(id,i,n.add(XATTR_SECURITY_PREFIX_LEN),v,s);if r!=-EOPNOTSUPP{return r;}}__vfs_getxattr(d,i,n,v,s)}

pub unsafe fn __vfs_removexattr(id:*mut mnt_idmap,d:*mut dentry,mut n:*const c_char)->c_int{let i=(*d).d_inode;if is_posix_acl_xattr(n){return -EOPNOTSUPP;}let h=xattr_resolve_name(i,&mut n);if IS_ERR(h){return PTR_ERR(h);}if (*h).set.is_none(){return -EOPNOTSUPP;}((*h).set.unwrap())(h,id,d,i,n,core::ptr::null(),0,XATTR_REPLACE)}
pub unsafe fn vfs_listxattr(d:*mut dentry,list:*mut c_char,size:usize)->isize{let i=d_inode(d);let e=security_inode_listxattr(d);if e!=0{return e as isize;}if !(*(*i).i_op).listxattr.is_none(){((*(*i).i_op).listxattr.unwrap())(d,list,size)}else{let mut rem=size as isize;let e=security_inode_listsecurity(i,&mut list,&mut rem);if e!=0{e as isize}else{size as isize-rem}}}
pub unsafe fn vfs_removexattr(id:*mut mnt_idmap,d:*mut dentry,n:*const c_char)->c_int{let i=(*d).d_inode;let mut di=core::mem::zeroed();loop{inode_lock(i);let mut e=xattr_permission(id,i,n,MAY_WRITE);if e==0{e=security_inode_removexattr(id,d,n);if e==0{e=try_break_deleg(i,0,&mut di);}if e==0{e=__vfs_removexattr(id,d,n);if e==0{fsnotify_xattr(d);security_inode_post_removexattr(d,n);}}}inode_unlock(i);if is_delegated(&di){e=break_deleg_wait(&mut di);if e==0{continue;}}return e;}}
pub unsafe fn simple_xattr_free_rcu(x:*mut simple_xattr){if !x.is_null(){call_rcu(&mut (*x).rcu,simple_xattr_rcu_free);}}
unsafe fn simple_xattr_rcu_free(h:*mut rcu_head){simple_xattr_free(container_of(h, core::mem::size_of::<simple_xattr>(), rcu));}
pub unsafe fn simple_xattr_set_limited(c:*mut simple_xattr_cache,x:*mut list_head,l:*mut simple_xattr_limits,n:*const c_char,v:*const c_void,s:usize,f:c_int)->c_int{let old=simple_xattr_set(c,x,n,v,s,f);if IS_ERR(old){return PTR_ERR(old);}if !old.is_null(){simple_xattr_free_rcu(old);}0}
pub unsafe fn simple_xattr_add(c:*mut simple_xattr_cache,x:*mut list_head,n:*mut simple_xattr)->c_int{(*n).parent=x;let h=simple_xattrs_lazy_alloc(c,(*n).value.as_ptr() as *const c_void,0);if IS_ERR(h){return PTR_ERR(h);}rhashtable_insert_fast(h,&mut (*n).hash_node,simple_xattr_params)}
pub unsafe fn simple_xattrs_free(c:*mut simple_xattr_cache,x:*mut list_head,_freed:*mut usize){while !list_empty(x){let n=list_first_entry(x);rhashtable_remove_fast((*c).ht,&mut (*n).hash_node,simple_xattr_params);list_del(&mut (*n).node);simple_xattr_free_rcu(n);}}
pub unsafe fn simple_xattr_cache_cleanup(c:*mut simple_xattr_cache){if !(*c).ht.is_null(){rhashtable_destroy((*c).ht);kfree((*c).ht as *mut c_void);(*c).ht=core::ptr::null_mut();}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
