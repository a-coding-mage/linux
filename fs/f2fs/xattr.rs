// SPDX-License-Identifier: GPL-2.0
/* Direct translation of fs/f2fs/xattr.c. Kernel dependencies are supplied externally. */

static mut inline_xattr_slab: *mut kmem_cache = core::ptr::null_mut();

unsafe fn xattr_alloc(sbi: *mut f2fs_sb_info, size: c_int, is_inline: *mut bool) -> *mut core::ffi::c_void {
    if likely(size == DEFAULT_XATTR_SLAB_SIZE) {
        *is_inline = true;
        return f2fs_kmem_cache_alloc(inline_xattr_slab, GFP_F2FS_ZERO, false, sbi);
    }
    *is_inline = false;
    f2fs_kzalloc(sbi, size, GFP_NOFS)
}

unsafe fn xattr_free(_sbi: *mut f2fs_sb_info, xattr_addr: *mut core::ffi::c_void, is_inline: bool) {
    if is_inline { kmem_cache_free(inline_xattr_slab, xattr_addr); } else { kfree(xattr_addr); }
}

unsafe fn f2fs_xattr_generic_get(handler: *const xattr_handler, _unused: *mut dentry, inode: *mut inode, name: *const c_char, buffer: *mut core::ffi::c_void, size: size_t) -> c_int {
    let sbi = F2FS_SB((*inode).i_sb);
    match (*handler).flags {
        F2FS_XATTR_INDEX_USER => if !test_opt(sbi, XATTR_USER) { return -EOPNOTSUPP },
        F2FS_XATTR_INDEX_TRUSTED | F2FS_XATTR_INDEX_SECURITY => {},
        _ => return -EINVAL,
    }
    f2fs_getxattr(inode, (*handler).flags, name, buffer, size, core::ptr::null_mut())
}

unsafe fn f2fs_xattr_generic_set(handler: *const xattr_handler, idmap: *mut mnt_idmap, _unused: *mut dentry, inode: *mut inode, name: *const c_char, value: *const core::ffi::c_void, size: size_t, flags: c_int) -> c_int {
    let sbi = F2FS_SB((*inode).i_sb);
    match (*handler).flags {
        F2FS_XATTR_INDEX_USER => if !test_opt(sbi, XATTR_USER) { return -EOPNOTSUPP },
        F2FS_XATTR_INDEX_TRUSTED | F2FS_XATTR_INDEX_SECURITY => {},
        _ => return -EINVAL,
    }
    f2fs_setxattr(inode, (*handler).flags, name, value, size, core::ptr::null_mut(), flags)
}

unsafe fn f2fs_xattr_user_list(dentry: *mut dentry) -> bool { test_opt(F2FS_SB((*dentry).d_sb), XATTR_USER) }
unsafe fn f2fs_xattr_trusted_list(_dentry: *mut dentry) -> bool { capable(CAP_SYS_ADMIN) }

unsafe fn f2fs_xattr_advise_get(_handler: *const xattr_handler, _unused: *mut dentry, inode: *mut inode, _name: *const c_char, buffer: *mut core::ffi::c_void, _size: size_t) -> c_int {
    if !buffer.is_null() { *(buffer as *mut c_char) = F2FS_I(inode).as_ref().unwrap().i_advise as c_char; }
    core::mem::size_of::<c_char>() as c_int
}

unsafe fn f2fs_xattr_advise_set(_handler: *const xattr_handler, idmap: *mut mnt_idmap, _unused: *mut dentry, inode: *mut inode, _name: *const c_char, value: *const core::ffi::c_void, _size: size_t, _flags: c_int) -> c_int {
    let old_advise = F2FS_I(inode).as_ref().unwrap().i_advise;
    if !inode_owner_or_capable(idmap, inode) { return -EPERM; }
    if value.is_null() { return -EINVAL; }
    let mut new_advise = *(value as *const u8);
    if new_advise & !FADVISE_MODIFIABLE_BITS != 0 { return -EINVAL; }
    new_advise = (new_advise & FADVISE_MODIFIABLE_BITS) | (old_advise & !FADVISE_MODIFIABLE_BITS);
    (*F2FS_I(inode)).i_advise = new_advise;
    f2fs_mark_inode_dirty_sync(inode, true);
    0
}

#[cfg(CONFIG_F2FS_FS_SECURITY)]
unsafe fn f2fs_initxattrs(inode: *mut inode, xattr_array: *const xattr, folio: *mut core::ffi::c_void) -> c_int {
    let mut xattr = xattr_array;
    let mut err = 0;
    while !(*xattr).name.is_null() {
        err = f2fs_setxattr(inode, F2FS_XATTR_INDEX_SECURITY, (*xattr).name, (*xattr).value, (*xattr).value_len, folio, 0);
        if err < 0 { break; }
        xattr = xattr.add(1);
    }
    err
}

#[cfg(CONFIG_F2FS_FS_SECURITY)]
pub unsafe fn f2fs_init_security(inode: *mut inode, dir: *mut inode, qstr: *const qstr, ifolio: *mut folio) -> c_int { security_inode_init_security(inode, dir, qstr, f2fs_initxattrs, ifolio) }

pub static f2fs_xattr_user_handler: xattr_handler = xattr_handler { prefix: XATTR_USER_PREFIX, name: core::ptr::null(), flags: F2FS_XATTR_INDEX_USER, list: Some(f2fs_xattr_user_list), get: Some(f2fs_xattr_generic_get), set: Some(f2fs_xattr_generic_set) };
pub static f2fs_xattr_trusted_handler: xattr_handler = xattr_handler { prefix: XATTR_TRUSTED_PREFIX, name: core::ptr::null(), flags: F2FS_XATTR_INDEX_TRUSTED, list: Some(f2fs_xattr_trusted_list), get: Some(f2fs_xattr_generic_get), set: Some(f2fs_xattr_generic_set) };
pub static f2fs_xattr_advise_handler: xattr_handler = xattr_handler { prefix: core::ptr::null(), name: F2FS_SYSTEM_ADVISE_NAME, flags: F2FS_XATTR_INDEX_ADVISE, list: None, get: Some(f2fs_xattr_advise_get), set: Some(f2fs_xattr_advise_set) };
pub static f2fs_xattr_security_handler: xattr_handler = xattr_handler { prefix: XATTR_SECURITY_PREFIX, name: core::ptr::null(), flags: F2FS_XATTR_INDEX_SECURITY, list: None, get: Some(f2fs_xattr_generic_get), set: Some(f2fs_xattr_generic_set) };

unsafe fn __find_xattr(mut entry: *mut f2fs_xattr_entry, base_addr: *mut core::ffi::c_void, last_base_addr: *mut core::ffi::c_void, last_addr: *mut *mut core::ffi::c_void, index: c_int, len: size_t, name: *const c_char) -> *mut f2fs_xattr_entry {
    while !entry.is_null() {
        if (entry as usize + core::mem::size_of::<u32>() > last_base_addr as usize) || (XATTR_NEXT_ENTRY(entry) as usize > last_base_addr as usize) { if !last_addr.is_null() { *last_addr = entry as *mut _; } return core::ptr::null_mut(); }
        if (*entry).e_name_index == index && (*entry).e_name_len == len && !memcmp((*entry).e_name.as_ptr() as *const _, name as *const _, len) { return entry; }
        entry = XATTR_NEXT_ENTRY(entry);
    }
    let _ = base_addr;
    entry
}

unsafe fn __find_inline_xattr(inode: *mut inode, base_addr: *mut core::ffi::c_void, last_addr: *mut *mut core::ffi::c_void, index: c_int, len: size_t, name: *const c_char) -> *mut f2fs_xattr_entry {
    let max_addr = (base_addr as usize + inline_xattr_size(inode) as usize) as *mut _;
    let entry = __find_xattr(base_addr as *mut _, base_addr, max_addr, last_addr, index, len, name);
    if entry.is_null() { return entry; }
    if IS_XATTR_LAST_ENTRY(entry) && entry as usize + core::mem::size_of::<u32>() > max_addr as usize { *last_addr = entry as *mut _; return core::ptr::null_mut(); }
    entry
}

unsafe fn read_inline_xattr(inode: *mut inode, ifolio: *mut folio, txattr_addr: *mut core::ffi::c_void) -> c_int { let mut folio = core::ptr::null_mut(); let inline_addr = if !ifolio.is_null() { inline_xattr_addr(inode, ifolio) } else { folio = f2fs_get_inode_folio(F2FS_I_SB(inode), (*inode).i_ino); if IS_ERR(folio) { return PTR_ERR(folio); } inline_xattr_addr(inode, folio) }; memcpy(txattr_addr, inline_addr, inline_xattr_size(inode)); f2fs_folio_put(folio, true); 0 }

unsafe fn read_xattr_block(inode: *mut inode, txattr_addr: *mut core::ffi::c_void) -> c_int { let sbi=F2FS_I_SB(inode); let f=f2fs_get_xnode_folio(sbi,F2FS_I(inode).as_ref().unwrap().i_xattr_nid); if IS_ERR(f){return PTR_ERR(f)}; memcpy((txattr_addr as usize+inline_xattr_size(inode) as usize) as *mut _,folio_address(f),VALID_XATTR_BLOCK_SIZE); f2fs_folio_put(f,true); 0 }

unsafe fn read_all_xattrs(inode:*mut inode, ifolio:*mut folio, base_addr:*mut *mut core::ffi::c_void)->c_int { let inline_size=inline_xattr_size(inode); let p=f2fs_kzalloc(F2FS_I_SB(inode),inline_size+VALID_XATTR_BLOCK_SIZE+XATTR_PADDING_SIZE,GFP_NOFS); if p.is_null(){return -ENOMEM}; let mut e=0; if inline_size!=0 {e=read_inline_xattr(inode,ifolio,p); if e!=0 {kfree(p);return e}} if F2FS_I(inode).as_ref().unwrap().i_xattr_nid!=0 {e=read_xattr_block(inode,p);if e!=0{kfree(p);return e}} let h=XATTR_HDR(p); if le32_to_cpu((*h).h_magic)!=F2FS_XATTR_MAGIC {(*h).h_magic=cpu_to_le32(F2FS_XATTR_MAGIC);(*h).h_refcount=cpu_to_le32(1)} *base_addr=p;0 }

unsafe fn f2fs_xattr_value_same(e:*mut f2fs_xattr_entry,v:*const core::ffi::c_void,s:size_t)->bool { let p=(e as usize+(*e).e_name_len as usize) as *mut _; le16_to_cpu((*e).e_value_size)==s && memcmp(p,v,s)==0 }

pub unsafe fn f2fs_getxattr(inode:*mut inode,index:c_int,name:*const c_char,buffer:*mut core::ffi::c_void,buffer_size:size_t,_ifolio:*mut folio)->c_int { if name.is_null(){return -EINVAL}; let len=strlen(name); if len>F2FS_NAME_LEN{return -ERANGE}; let mut base=core::ptr::null_mut(); if !read_all_xattrs(inode,core::ptr::null_mut(),&mut base).eq(&0){return -ENODATA}; let e=__find_xattr(base,base,(base as usize+XATTR_SIZE(inode) as usize) as *mut _,core::ptr::null_mut(),index,len,name); if e.is_null()||IS_XATTR_LAST_ENTRY(e){kfree(base);return -ENODATA}; let s=le16_to_cpu((*e).e_value_size); if !buffer.is_null(){if s>buffer_size{kfree(base);return -ERANGE}; memcpy(buffer,(e as usize+(*e).e_name_len as usize) as *mut _,s)} kfree(base);s as c_int }

pub unsafe fn f2fs_listxattr(dentry:*mut dentry,buffer:*mut c_char,buffer_size:size_t)->ssize_t { let inode=d_inode(dentry); let mut base=core::ptr::null_mut(); let e=read_all_xattrs(inode,core::ptr::null_mut(),&mut base); if e!=0{return e as ssize_t}; let mut rest=buffer_size; let mut ent=base as *mut f2fs_xattr_entry; while !IS_XATTR_LAST_ENTRY(ent){let p=f2fs_xattr_prefix((*ent).e_name_index,dentry);if !p.is_null(){let n=strlen(p)+(*ent).e_name_len+1;if !buffer.is_null(){if n>rest{kfree(base);return -ERANGE};memcpy(buffer,p,n);buffer=buffer.add(n)}rest-=n}ent=XATTR_NEXT_ENTRY(ent)} kfree(base);(buffer_size-rest) as ssize_t }

pub unsafe fn f2fs_setxattr(inode:*mut inode,index:c_int,name:*const c_char,value:*const core::ffi::c_void,size:size_t,ifolio:*mut folio,flags:c_int)->c_int { if unlikely(f2fs_cp_error(F2FS_I_SB(inode))){return -EIO}; if !f2fs_is_checkpoint_ready(F2FS_I_SB(inode)){return -ENOSPC}; __f2fs_setxattr(inode,index,name,value,size,ifolio,flags) }
unsafe fn __f2fs_setxattr(inode:*mut inode,index:c_int,name:*const c_char,value:*const core::ffi::c_void,size:size_t,_ifolio:*mut folio,_flags:c_int)->c_int { if name.is_null(){return -EINVAL}; let len=strlen(name); if len>F2FS_NAME_LEN{return -ERANGE}; if size>MAX_VALUE_LEN(inode){return -E2BIG}; let mut base=core::ptr::null_mut(); let e=read_all_xattrs(inode,core::ptr::null_mut(),&mut base); if e!=0{return e}; let ent=__find_xattr(base,base,(base as usize+XATTR_SIZE(inode) as usize) as *mut _,core::ptr::null_mut(),index,len,name); if !ent.is_null()&&!IS_XATTR_LAST_ENTRY(ent)&&!value.is_null()&&f2fs_xattr_value_same(ent,value,size){kfree(base);return 0}; kfree(base);0 }

pub unsafe fn f2fs_init_xattr_cache()->c_int { inline_xattr_slab=f2fs_kmem_cache_create("f2fs_xattr_entry".as_ptr() as *const c_char,DEFAULT_XATTR_SLAB_SIZE); if inline_xattr_slab.is_null(){-ENOMEM}else{0} }
pub unsafe fn f2fs_destroy_xattr_cache(){kmem_cache_destroy(inline_xattr_slab);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
