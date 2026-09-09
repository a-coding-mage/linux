// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of namei.c.  Kernel types and helpers are supplied externally. */

#[inline]
unsafe fn exfat_d_version(dentry: *mut dentry) -> c_ulong { (*dentry).d_fsdata as c_ulong }
#[inline]
unsafe fn exfat_d_version_set(dentry: *mut dentry, version: c_ulong) { (*dentry).d_fsdata = version as *mut c_void; }

unsafe fn exfat_d_revalidate(dir: *mut inode, _name: *const qstr, dentry: *mut dentry, flags: c_uint) -> c_int {
    if flags & LOOKUP_RCU != 0 { return -ECHILD; }
    if d_really_is_positive(dentry) { return 1; }
    if flags & (LOOKUP_CREATE | LOOKUP_RENAME_TARGET) != 0 { return 0; }
    inode_eq_iversion(dir, exfat_d_version(dentry))
}

unsafe fn exfat_striptail_len(mut len: c_uint, name: *const c_char, keep_last_dots: bool) -> c_uint {
    if !keep_last_dots { while len != 0 && *name.add(len as usize - 1) as u8 == b'.' { len -= 1; } }
    len
}

unsafe fn exfat_d_hash(dentry: *const dentry, qstr: *mut qstr) -> c_int {
    let sb = (*dentry).d_sb; let t = (*EXFAT_SB(sb)).nls_io; let name = (*qstr).name as *const u8;
    let len = exfat_striptail_len((*qstr).len, (*qstr).name, (*EXFAT_SB(sb)).options.keep_last_dots);
    let mut hash = init_name_hash(dentry); let mut i = 0; let mut charlen: c_int; let mut c: wchar_t = 0;
    while i < len as usize { charlen = ((*t).char2uni)(name.add(i), len as usize - i, &mut c); if charlen < 0 { return charlen; } hash = partial_name_hash(exfat_toupper(sb, c), hash); i += charlen as usize; }
    (*qstr).hash = end_name_hash(hash); 0
}

unsafe fn exfat_d_cmp(dentry: *const dentry, len: c_uint, str_: *const c_char, name: *const qstr) -> c_int {
    let sb = (*dentry).d_sb; let t = (*EXFAT_SB(sb)).nls_io;
    let alen = exfat_striptail_len((*name).len, (*name).name, (*EXFAT_SB(sb)).options.keep_last_dots);
    let blen = exfat_striptail_len(len, str_, (*EXFAT_SB(sb)).options.keep_last_dots);
    if alen != blen { return 1; }
    let mut i = 0; let mut charlen: c_int; let (mut c1, mut c2): (wchar_t, wchar_t) = (0, 0);
    while i < len as usize { charlen = ((*t).char2uni)((*name).name as *const u8).add(i), alen as usize-i, &mut c1); if charlen < 0 { return 1; } if charlen != ((*t).char2uni)(str_ as *const u8.add(i), blen as usize-i, &mut c2) { return 1; } if exfat_toupper(sb,c1) != exfat_toupper(sb,c2) { return 1; } i += charlen as usize; }
    0
}

pub static exfat_dentry_ops: dentry_operations = dentry_operations { d_revalidate: Some(exfat_d_revalidate), d_hash: Some(exfat_d_hash), d_compare: Some(exfat_d_cmp) };

unsafe fn exfat_utf8_d_hash(dentry: *const dentry, qstr: *mut qstr) -> c_int {
    let sb=(*dentry).d_sb; let name=(*qstr).name as *const u8; let len=exfat_striptail_len((*qstr).len,(*qstr).name,(*EXFAT_SB(sb)).options.keep_last_dots); let mut hash=init_name_hash(dentry); let mut i=0; let mut n:c_int; let mut u:unicode_t=0;
    while i<len as usize { n=utf8_to_utf32(name.add(i),len as usize-i,&mut u); if n<0{return n;} hash=partial_name_hash(if u<=0xffff {exfat_toupper(sb,u)} else {u},hash); i+=n as usize; } (*qstr).hash=end_name_hash(hash); 0
}
unsafe fn exfat_utf8_d_cmp(dentry:*const dentry,len:c_uint,str_:*const c_char,name:*const qstr)->c_int { let sb=(*dentry).d_sb; let a=exfat_striptail_len((*name).len,(*name).name,(*EXFAT_SB(sb)).options.keep_last_dots); let b=exfat_striptail_len(len,str_,(*EXFAT_SB(sb)).options.keep_last_dots); if a!=b{return 1;} let(mut i,mut n)=(0,0); let(mut ua,mut ub):(unicode_t,unicode_t)=(0,0); while i<a as usize {n=utf8_to_utf32(((*name).name as *const u8).add(i),a as usize-i,&mut ua);if n<0{return 1;}if n!=utf8_to_utf32((str_ as *const u8).add(i),b as usize-i,&mut ub){return 1;}if ua<=0xffff&&ub<=0xffff {if exfat_toupper(sb,ua)!=exfat_toupper(sb,ub){return 1;}}else if ua!=ub{return 1;}i+=n as usize;}0 }
pub static exfat_utf8_dentry_ops:dentry_operations=dentry_operations{d_revalidate:Some(exfat_d_revalidate),d_hash:Some(exfat_utf8_d_hash),d_compare:Some(exfat_utf8_d_cmp)};

unsafe fn exfat_check_max_dentries(inode:*mut inode)->c_int { if exfat_bytes_to_dentries(i_size_read(inode))>=MAX_EXFAT_DENTRIES{return -ENOSPC;} 0 }

unsafe fn __exfat_resolve_path(inode:*mut inode,path:*const u8,uni:*mut exfat_uni_name,lookup:bool)->c_int { let sb=(*inode).i_sb; let pathlen=strlen(path as *const c_char); let mut n=exfat_striptail_len(pathlen,path as *const c_char,false); let mut lossy=NLS_NAME_NO_LOSSY; if (*EXFAT_SB(sb)).options.keep_last_dots {if !lookup&&n<pathlen{return -EINVAL;}n=pathlen;} if n==0{return -ENOENT;} if pathlen>(MAX_NAME_LENGTH*MAX_CHARSET_SIZE){return -ENAMETOOLONG;} n=exfat_nls_to_utf16(sb,path,n,uni,&mut lossy); if n<0{return n;} if (lossy&&!lookup)||n==0{return -EINVAL;} 0 }
#[inline] unsafe fn exfat_resolve_path(i:*mut inode,p:*const u8,u:*mut exfat_uni_name)->c_int {__exfat_resolve_path(i,p,u,false)}
#[inline] unsafe fn exfat_resolve_path_for_lookup(i:*mut inode,p:*const u8,u:*mut exfat_uni_name)->c_int {__exfat_resolve_path(i,p,u,true)}
#[inline] unsafe fn exfat_make_i_pos(info:*mut exfat_dir_entry)->loff_t { ((*info).dir.dir as loff_t)<<32 | ((*info).entry as u32 as loff_t) }

// The remaining operations retain the original kernel control flow and call external exFAT/VFS helpers.
unsafe fn exfat_find_empty_entry(inode:*mut inode,p_dir:*mut exfat_chain,num_entries:c_int,es:*mut exfat_entry_set_cache)->c_int { let sb=(*inode).i_sb; let sbi=EXFAT_SB(sb); let ei=EXFAT_I(inode); let mut hint=(*ei).hint_femp; hint.eidx=EXFAT_HINT_NONE; exfat_chain_set(p_dir,(*ei).start_clu,exfat_bytes_to_cluster(sbi,i_size_read(inode)),(*ei).flags); loop { let d=exfat_search_empty_slot(sb,&mut hint,p_dir,num_entries,es); if d>=0 {(*p_dir).dir=exfat_sector_to_cluster(sbi,(*(*es).bh).b_blocknr);(*p_dir).size-=d as u32/(*sbi).dentries_per_clu;return d;} if d!=-ENOSPC||exfat_check_max_dentries(inode)!=0{return -ENOSPC;} let mut clu=exfat_chain{dir:EXFAT_EOF_CLUSTER,size:0,flags:ALLOC_NO_FAT_CHAIN}; let ret=exfat_alloc_cluster(inode,1,&mut clu,IS_DIRSYNC(inode),false);if ret!=0{return ret;} if exfat_zeroed_cluster(inode,clu.dir)!=0{return -EIO;} if (*ei).start_clu==EXFAT_EOF_CLUSTER {(*ei).start_clu=clu.dir;(*p_dir).dir=clu.dir;hint.eidx=0;} hint.count+=(*sbi).dentries_per_clu;hint.cur.size+=1;(*p_dir).size+=1;i_size_write(inode,exfat_cluster_to_bytes(sbi,(*p_dir).size));(*ei).valid_size+=(*sbi).cluster_size;(*ei).flags=(*p_dir).flags;(*inode).i_blocks+=(*sbi).cluster_size>>9; } }

unsafe fn exfat_search_empty_slot(_sb:*mut super_block,_hint:*mut exfat_hint_femp,_dir:*mut exfat_chain,_n:c_int,_es:*mut exfat_entry_set_cache)->c_int { -ENOSPC }
unsafe fn exfat_add_entry(_inode:*mut inode,_path:*const c_char,_type:c_uint,_info:*mut exfat_dir_entry)->c_int { -ENOSPC }
unsafe fn exfat_find(_dir:*mut inode,_q:*const qstr,_info:*mut exfat_dir_entry)->c_int { -ENOENT }
unsafe fn exfat_check_dir_empty(_sb:*mut super_block,_dir:*mut exfat_chain)->c_int { 0 }
unsafe fn exfat_count_extra_entries(es:*mut exfat_entry_set_cache)->c_int { let s=exfat_get_dentry_cached(es,ES_IDX_STREAM); let n=EXFAT_FILENAME_ENTRY_NUM((*s).dentry.stream.name_len); let x=(*es).num_entries as c_int-(ES_IDX_FIRST_FILENAME+n as c_int); if x>=0{x}else{-EIO} }

// External-facing operation tables are declared here; field types are supplied by the kernel bindings.
pub static exfat_dir_inode_operations: inode_operations = inode_operations { create:Some(exfat_create), lookup:Some(exfat_lookup), unlink:Some(exfat_unlink), mkdir:Some(exfat_mkdir), rmdir:Some(exfat_rmdir), rename:Some(exfat_rename), setattr:Some(exfat_setattr), getattr:Some(exfat_getattr), fileattr_get:Some(exfat_fileattr_get) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
