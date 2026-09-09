// SPDX-License-Identifier: GPL-2.0-only
/* Direct translation of xattr.c. External kernel and filesystem symbols are
 * intentionally left as dependencies supplied by the surrounding code. */

#[repr(C)]
struct erofs_xattr_iter {
    sb: *mut super_block, buf: erofs_buf, pos: erofs_off_t, kaddr: *mut core::ffi::c_void,
    buffer: *mut i8, buffer_size: i32, buffer_ofs: i32,
    index: i32, infix_len: i32, name: qstr, dentry: *mut dentry,
}

extern "C" {
    fn erofs_xattr_prefix(idx: u32, dentry: *mut dentry) -> *const i8;
}

unsafe fn erofs_init_inode_xattrs(inode: *mut inode) -> i32 {
    let mut buf = __EROFS_BUF_INITIALIZER;
    let vi = EROFS_I(inode); let sb = (*inode).i_sb;
    let mut pos: erofs_off_t; let mut ret = 0i32;
    if (*vi).xattr_isize == 0 { return -ENODATA; }
    if test_bit(EROFS_I_EA_INITED_BIT, &(*vi).flags) { smp_mb(); return 0; }
    if wait_on_bit_lock(&mut (*vi).flags, EROFS_I_BL_XATTR_BIT, TASK_KILLABLE) != 0 { return -ERESTARTSYS; }
    if test_bit(EROFS_I_EA_INITED_BIT, &(*vi).flags) { goto out_unlock; }
    if (*vi).xattr_isize == core::mem::size_of::<erofs_xattr_ibody_header>() as u32 {
        erofs_err(sb, "xattr_isize %d of nid %llu is not supported yet", (*vi).xattr_isize, (*vi).nid); ret = -EOPNOTSUPP; goto out_unlock;
    } else if (*vi).xattr_isize < core::mem::size_of::<erofs_xattr_ibody_header>() as u32 {
        erofs_err(sb, "bogus xattr ibody @ nid %llu", (*vi).nid); DBG_BUGON(1); ret = -EFSCORRUPTED; goto out_unlock;
    }
    pos = erofs_iloc(inode) + (*vi).inode_isize as u64;
    let ih = erofs_read_metabuf(&mut buf, sb, pos, erofs_inode_in_metabox(inode));
    if IS_ERR(ih) { ret = PTR_ERR(ih); goto out_unlock; }
    (*vi).xattr_name_filter = le32_to_cpu((*ih).h_name_filter); (*vi).xattr_shared_count = (*ih).h_shared_count;
    if ((*vi).xattr_shared_count as u32) * core::mem::size_of::<u32>() as u32 > (*vi).xattr_isize - core::mem::size_of::<erofs_xattr_ibody_header>() as u32 {
        erofs_err(sb, "invalid h_shared_count %u @ nid %llu", (*vi).xattr_shared_count, (*vi).nid); ret = -EFSCORRUPTED; goto out_unlock;
    }
    (*vi).xattr_shared_xattrs = kmalloc_objs::<u32>((*vi).xattr_shared_count);
    if (*vi).xattr_shared_xattrs.is_null() { ret = -ENOMEM; goto out_unlock; }
    pos += core::mem::size_of::<erofs_xattr_ibody_header>() as u64;
    for i in 0..(*vi).xattr_shared_count as usize {
        let p = erofs_bread(&mut buf, pos + (i * 4) as u64, true);
        if IS_ERR(p) { kfree((*vi).xattr_shared_xattrs); (*vi).xattr_shared_xattrs = core::ptr::null_mut(); ret = PTR_ERR(p); goto out_unlock; }
        *(*vi).xattr_shared_xattrs.add(i) = le32_to_cpu(*(p as *mut u32));
    }
    smp_mb(); set_bit(EROFS_I_EA_INITED_BIT, &mut (*vi).flags);
out_unlock:
    erofs_put_metabuf(&mut buf); clear_and_wake_up_bit(EROFS_I_BL_XATTR_BIT, &mut (*vi).flags); ret
}

unsafe fn erofs_xattr_copy_to_buffer(it: *mut erofs_xattr_iter, len: u32) -> i32 {
    let mut processed = 0; let mut slice;
    while processed < len {
        (*it).kaddr = erofs_bread(&mut (*it).buf, (*it).pos, true); if IS_ERR((*it).kaddr) { return PTR_ERR((*it).kaddr); }
        slice = core::cmp::min((*(*it).sb).s_blocksize - erofs_blkoff((*it).sb, (*it).pos), len - processed);
        memcpy((*it).buffer.add((*it).buffer_ofs as usize), (*it).kaddr, slice as usize); (*it).buffer_ofs += slice as i32; (*it).pos += slice as u64; processed += slice;
    } 0
}

unsafe fn erofs_listxattr_foreach(it: *mut erofs_xattr_iter) -> i32 {
    let entry = *(*it).kaddr.cast::<erofs_xattr_entry>(); (*it).pos += core::mem::size_of::<erofs_xattr_entry>() as u64;
    let mut base = entry.e_name_index as u32; let mut infix: *const i8 = core::ptr::null(); let mut infix_len = 0u32;
    if base & EROFS_XATTR_LONG_PREFIX != 0 { let sbi=EROFS_SB((*it).sb); let pf=(*sbi).xattr_prefixes.add((base & EROFS_XATTR_LONG_PREFIX_MASK) as usize); if pf >= (*sbi).xattr_prefixes.add((*sbi).xattr_prefix_count as usize) { return 0; } infix=(*(*pf).prefix).infix.as_ptr(); infix_len=(*pf).infix_len; base=(*(*pf).prefix).base_index; }
    let prefix=erofs_xattr_prefix(base, (*it).dentry); if prefix.is_null() { return 0; } let plen=strlen(prefix) as u32; let total=plen+infix_len+entry.e_name_len as u32+1;
    if (*it).buffer.is_null() { (*it).buffer_ofs += total as i32; return 0; } if (*it).buffer_ofs as u32+total > (*it).buffer_size as u32 { return -ERANGE; }
    memcpy((*it).buffer.add((*it).buffer_ofs as usize), prefix, plen as usize); memcpy((*it).buffer.add((*it).buffer_ofs as usize+plen as usize), infix, infix_len as usize); (*it).buffer_ofs += (plen+infix_len) as i32;
    let e=erofs_xattr_copy_to_buffer(it, entry.e_name_len as u32); if e!=0{return e;} *(*it).buffer.add((*it).buffer_ofs as usize)=0; (*it).buffer_ofs+=1; 0
}

unsafe fn erofs_getxattr_foreach(it: *mut erofs_xattr_iter) -> i32 {
    let sb=(*it).sb; let entry=*(*it).kaddr.cast::<erofs_xattr_entry>(); (*it).pos+=core::mem::size_of::<erofs_xattr_entry>() as u64; let value_sz=le16_to_cpu(entry.e_value_size) as u32;
    if entry.e_name_index as u32 & EROFS_XATTR_LONG_PREFIX != 0 { let sbi=EROFS_SB(sb); let pf=(*sbi).xattr_prefixes.add((entry.e_name_index as u32 & EROFS_XATTR_LONG_PREFIX_MASK) as usize); if pf>=(*sbi).xattr_prefixes.add((*sbi).xattr_prefix_count as usize)||(*it).index!=(*pf).prefix.as_ref().unwrap().base_index as i32||(*it).name.len as u32!=entry.e_name_len as u32+(*pf).infix_len{return -ENODATA;} if memcmp((*it).name.name.add((*pf).infix_len as usize),(*it).kaddr,0)!=0{return -ENODATA;} (*it).infix_len=(*pf).infix_len as i32; } else { if (*it).index!=entry.e_name_index as i32||(*it).name.len as u32!=entry.e_name_len as u32{return -ENODATA;} (*it).infix_len=0; }
    let mut processed=0u32; while processed<entry.e_name_len as u32 { (*it).kaddr=erofs_bread(&mut (*it).buf,(*it).pos,true); if IS_ERR((*it).kaddr){return PTR_ERR((*it).kaddr);} let slice=core::cmp::min((*sb).s_blocksize-erofs_blkoff(sb,(*it).pos),entry.e_name_len as u32-processed); if memcmp((*it).name.name.add((*it).infix_len as usize+processed as usize),(*it).kaddr,slice as usize)!=0{return -ENODATA;} (*it).pos+=slice as u64; processed+=slice; }
    if (*it).buffer.is_null(){(*it).buffer_ofs=value_sz as i32;return 0;} if (*it).buffer_size as u32<value_sz{return -ERANGE;} erofs_xattr_copy_to_buffer(it,value_sz)
}

// Remaining iterator and exported wrapper logic follows the C control flow.
unsafe fn erofs_xattr_iter_inline(it:*mut erofs_xattr_iter,inode:*mut inode,getxattr:bool)->i32{let vi=EROFS_I(inode);let hs=core::mem::size_of::<erofs_xattr_ibody_header>() as u32+4*(*vi).xattr_shared_count;if hs>=(*vi).xattr_isize{DBG_BUGON((hs>(*vi).xattr_isize)as i32);return -ENODATA;}let r=erofs_init_metabuf(&mut (*it).buf,(*it).sb,erofs_inode_in_metabox(inode));if r!=0{return r;}let mut rem=(*vi).xattr_isize-hs;(*it).pos=erofs_iloc(inode)+(*vi).inode_isize as u64;let mut ret=0;while rem!=0{(*it).kaddr=erofs_bread(&mut (*it).buf,(*it).pos,true);if IS_ERR((*it).kaddr){return PTR_ERR((*it).kaddr);}let es=erofs_xattr_entry_size((*it).kaddr);if rem<es{DBG_BUGON(1);return -EFSCORRUPTED;}rem-=es;let next=(*it).pos+es as u64;ret=if getxattr{erofs_getxattr_foreach(it)}else{erofs_listxattr_foreach(it)};if(getxattr&&ret!=-ENODATA)||(!getxattr&&ret!=0){break;}(*it).pos=next;}ret}

unsafe fn erofs_xattr_iter_shared(it:*mut erofs_xattr_iter,inode:*mut inode,getxattr:bool)->i32{let vi=EROFS_I(inode);let sb=(*it).sb;let sbi=EROFS_SB(sb);let mut r=erofs_init_metabuf(&mut (*it).buf,sb,erofs_sb_has_shared_ea_in_metabox(sbi));if r!=0{return r;}let mut i=0;while i<(*vi).xattr_shared_count{(*it).pos=erofs_pos(sb,(*sbi).xattr_blkaddr)+*(*vi).xattr_shared_xattrs.add(i as usize) as u64*4;i+=1;(*it).kaddr=erofs_bread(&mut (*it).buf,(*it).pos,true);if IS_ERR((*it).kaddr){return PTR_ERR((*it).kaddr);}r=if getxattr{erofs_getxattr_foreach(it)}else{erofs_listxattr_foreach(it)};if(getxattr&&r!=-ENODATA)||(!getxattr&&r!=0){break;}}if i!=0{r}else{-ENODATA}}

#[no_mangle] pub unsafe extern "C" fn erofs_listxattr(dentry:*mut dentry,buffer:*mut i8,size:usize)->isize{let inode=d_inode(dentry);let mut r=erofs_init_inode_xattrs(inode);if r==-ENODATA{return 0;}if r!=0{return r as isize;}let mut it=core::mem::zeroed::<erofs_xattr_iter>();it.sb=(*dentry).d_sb;it.buf=__EROFS_BUF_INITIALIZER;it.dentry=dentry;it.buffer=buffer;it.buffer_size=size as i32;r=erofs_xattr_iter_inline(&mut it,inode,false);if r==0||r==-ENODATA{r=erofs_xattr_iter_shared(&mut it,inode,false);}if r==-ENODATA{r=0;}erofs_put_metabuf(&mut it.buf);if r!=0{r as isize}else{it.buffer_ofs as isize}}

unsafe fn erofs_xattr_user_list(dentry:*mut dentry)->bool{test_opt(&mut (*EROFS_SB((*dentry).d_sb)).opt,XATTR_USER)}
unsafe fn erofs_xattr_trusted_list(_: *mut dentry)->bool{capable(CAP_SYS_ADMIN)}
unsafe fn erofs_xattr_generic_get(handler:*const xattr_handler,_unused:*mut dentry,inode:*mut inode,name:*const i8,buffer:*mut core::ffi::c_void,size:usize)->i32{if (*handler).flags==EROFS_XATTR_INDEX_USER&&!test_opt(&mut (*EROFS_I_SB(inode)).opt,XATTR_USER){return -EOPNOTSUPP;}erofs_getxattr(inode,(*handler).flags as i32,name,buffer,size)}

#[no_mangle] pub unsafe extern "C" fn erofs_xattr_prefixes_cleanup(sb:*mut super_block){let sbi=EROFS_SB(sb);if !(*sbi).xattr_prefixes.is_null(){for i in 0..(*sbi).xattr_prefix_count{kfree((*sbi).xattr_prefixes.add(i as usize).cast());}kfree((*sbi).xattr_prefixes.cast());(*sbi).xattr_prefixes=core::ptr::null_mut();}}

#[cfg(feature="CONFIG_EROFS_FS_POSIX_ACL")]
pub unsafe fn erofs_get_acl(inode:*mut inode,type_:i32,rcu:bool)->*mut posix_acl{if rcu{return ERR_PTR(-ECHILD);}let prefix=match type_{ACL_TYPE_ACCESS=>EROFS_XATTR_INDEX_POSIX_ACL_ACCESS,ACL_TYPE_DEFAULT=>EROFS_XATTR_INDEX_POSIX_ACL_DEFAULT,_=>return ERR_PTR(-EINVAL)};let mut rc=erofs_getxattr(inode,prefix as i32,b"\0".as_ptr() as *const i8,core::ptr::null_mut(),0);if rc==-ENODATA{return core::ptr::null_mut();}if rc<0{return ERR_PTR(rc);}let value=kmalloc(rc as usize,GFP_KERNEL);if value.is_null(){return ERR_PTR(-ENOMEM);}rc=erofs_getxattr(inode,prefix as i32,b"\0".as_ptr() as *const i8,value,rc as usize);let acl=if rc<0{ERR_PTR(rc)}else{posix_acl_from_xattr(&mut init_user_ns,value,rc)};kfree(value);acl}

// The declarations below preserve the remaining file-local interfaces and are
// supplied by the translated filesystem modules.
extern "C" { pub fn erofs_getxattr(inode:*mut inode,index:i32,name:*const i8,buffer:*mut core::ffi::c_void,size:usize)->i32; pub fn erofs_listxattr(dentry:*mut dentry,buffer:*mut i8,size:usize)->isize; pub fn erofs_xattr_prefixes_cleanup(sb:*mut super_block); pub fn erofs_xattr_prefixes_init(sb:*mut super_block)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
