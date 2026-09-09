// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of linux/fs/affs/namei.c. */

type ToupperT = unsafe fn(i32) -> i32;

unsafe fn affs_toupper(mut ch: i32) -> i32 {
    if ch >= b'a' as i32 && ch <= b'z' as i32 { ch -= b'a' as i32 - b'A' as i32; }
    ch
}

unsafe fn affs_intl_toupper(mut ch: i32) -> i32 {
    if (ch >= b'a' as i32 && ch <= b'z' as i32) ||
       (ch >= 0xe0 && ch <= 0xfe && ch != 0xf7) { ch -= b'a' as i32 - b'A' as i32; }
    ch
}

unsafe fn affs_get_toupper(sb: *mut super_block) -> ToupperT {
    if affs_test_opt((*AFFS_SB(sb)).s_flags, SF_INTL) { affs_intl_toupper } else { affs_toupper }
}

unsafe fn __affs_hash_dentry(dentry: *const dentry, qstr: *mut qstr, fn_: ToupperT, notruncate: bool) -> i32 {
    let mut name = (*qstr).name as *const u8;
    let retval = affs_check_name((*qstr).name, (*qstr).len, notruncate);
    if retval != 0 { return retval; }
    let mut hash = init_name_hash(dentry);
    let mut len = core::cmp::min((*qstr).len, AFFSNAMEMAX);
    while len > 0 { hash = partial_name_hash(fn_(*name as i32), hash); name = name.add(1); len -= 1; }
    (*qstr).hash = end_name_hash(hash);
    0
}

unsafe fn affs_hash_dentry(dentry: *const dentry, qstr: *mut qstr) -> i32 {
    __affs_hash_dentry(dentry, qstr, affs_toupper, affs_nofilenametruncate(dentry))
}
unsafe fn affs_intl_hash_dentry(dentry: *const dentry, qstr: *mut qstr) -> i32 {
    __affs_hash_dentry(dentry, qstr, affs_intl_toupper, affs_nofilenametruncate(dentry))
}

unsafe fn __affs_compare_dentry(mut len: u32, str_: *const i8, name: *const qstr, fn_: ToupperT, notruncate: bool) -> i32 {
    let mut aname = str_ as *const u8;
    let mut bname = (*name).name as *const u8;
    if affs_check_name((*name).name, (*name).len, notruncate) != 0 { return 1; }
    if len >= AFFSNAMEMAX { if (*name).len < AFFSNAMEMAX { return 1; } len = AFFSNAMEMAX; }
    else if len != (*name).len { return 1; }
    while len > 0 { if fn_(*aname as i32) != fn_(*bname as i32) { return 1; } aname = aname.add(1); bname = bname.add(1); len -= 1; }
    0
}
unsafe fn affs_compare_dentry(d: *const dentry, len: u32, s: *const i8, n: *const qstr) -> i32 { __affs_compare_dentry(len,s,n,affs_toupper,affs_nofilenametruncate(d)) }
unsafe fn affs_intl_compare_dentry(d: *const dentry, len: u32, s: *const i8, n: *const qstr) -> i32 { __affs_compare_dentry(len,s,n,affs_intl_toupper,affs_nofilenametruncate(d)) }

unsafe fn affs_match(d: *mut dentry, mut name2: *const u8, fn_: ToupperT) -> i32 {
    let mut name = (*d).d_name.name as *const u8;
    let mut len = (*d).d_name.len;
    if len >= AFFSNAMEMAX { if *name2 < AFFSNAMEMAX { return 0; } len = AFFSNAMEMAX; }
    else if len != *name2 { return 0; }
    name2 = name2.add(1);
    while len > 0 { if fn_(*name as i32) != fn_(*name2 as i32) { return 0; } name=name.add(1); name2=name2.add(1); len-=1; }
    1
}

pub unsafe fn affs_hash_name(sb: *mut super_block, mut name: *const u8, mut len: u32) -> i32 {
    let fn_ = affs_get_toupper(sb); len = core::cmp::min(len, AFFSNAMEMAX); let mut hash = len;
    while len > 0 { hash = (hash.wrapping_mul(13).wrapping_add(fn_(*name as i32) as u32)) & 0x7ff; name=name.add(1); len-=1; }
    (hash % (*AFFS_SB(sb)).s_hashsize) as i32
}

unsafe fn affs_find_entry(dir: *mut inode, d: *mut dentry) -> *mut buffer_head {
    let sb=(*dir).i_sb; let fn_=affs_get_toupper(sb); let mut bh=affs_bread(sb,(*dir).i_ino); if bh.is_null(){return ERR_PTR(-EIO);}
    let mut key=be32_to_cpu((*AFFS_HEAD(bh)).table[affs_hash_name(sb,(*d).d_name.name,(*d).d_name.len) as usize]);
    loop { affs_brelse(bh); if key==0{return core::ptr::null_mut();} bh=affs_bread(sb,key); if bh.is_null(){return ERR_PTR(-EIO);} if affs_match(d,(*AFFS_TAIL(sb,bh)).name,fn_)!=0{return bh;} key=be32_to_cpu((*AFFS_TAIL(sb,bh)).hash_chain); }
}

pub unsafe fn affs_lookup(dir:*mut inode,d:*mut dentry,_flags:u32)->*mut dentry { let sb=(*dir).i_sb; affs_lock_dir(dir); let bh=affs_find_entry(dir,d); if IS_ERR(bh){affs_unlock_dir(dir);return ERR_CAST(bh);} let mut inode=core::ptr::null_mut(); if !bh.is_null(){let mut ino=(*bh).b_blocknr; (*d).d_fsdata=ino as usize as *mut core::ffi::c_void; if be32_to_cpu((*AFFS_TAIL(sb,bh)).stype)==ST_LINKFILE{ino=be32_to_cpu((*AFFS_TAIL(sb,bh)).original);} affs_brelse(bh); inode=affs_iget(sb,ino);} let r=d_splice_alias(inode,d); if !IS_ERR_OR_NULL(r){(*r).d_fsdata=(*d).d_fsdata;} affs_unlock_dir(dir); r }
pub unsafe fn affs_unlink(_dir:*mut inode,d:*mut dentry)->i32{affs_remove_header(d)}

pub unsafe fn affs_create(_idmap:*mut mnt_idmap,dir:*mut inode,d:*mut dentry,mode:umode_t)->i32{let sb=(*dir).i_sb;let inode=affs_new_inode(dir);if inode.is_null(){return -ENOSPC;}(*inode).i_mode=mode;affs_mode_to_prot(inode);mark_inode_dirty(inode);(*inode).i_op=&affs_file_inode_operations;(*inode).i_fop=&affs_file_operations;(*(*inode).i_mapping).a_ops=if affs_test_opt((*AFFS_SB(sb)).s_flags,SF_OFS){&affs_aops_ofs}else{&affs_aops};let e=affs_add_entry(dir,inode,d,ST_FILE);if e!=0{clear_nlink(inode);iput(inode);}e}
pub unsafe fn affs_mkdir(_idmap:*mut mnt_idmap,dir:*mut inode,d:*mut dentry,mode:umode_t)->*mut dentry{let inode=affs_new_inode(dir);if inode.is_null(){return ERR_PTR(-ENOSPC);}(*inode).i_mode=mode;affs_mode_to_prot(inode);(*inode).i_op=&affs_dir_inode_operations;(*inode).i_fop=&affs_dir_operations;let e=affs_add_entry(dir,inode,d,ST_USERDIR);if e!=0{clear_nlink(inode);mark_inode_dirty(inode);iput(inode);return ERR_PTR(e);}core::ptr::null_mut()}
pub unsafe fn affs_rmdir(_dir:*mut inode,d:*mut dentry)->i32{affs_remove_header(d)}

pub unsafe fn affs_symlink(_idmap:*mut mnt_idmap,dir:*mut inode,d:*mut dentry,symname:*const i8)->i32{let sb=(*dir).i_sb;let maxlen=(*AFFS_SB(sb)).s_hashsize*core::mem::size_of::<u32>() as u32-1;let inode=affs_new_inode(dir);if inode.is_null(){return -ENOSPC;}(*inode).i_op=&affs_symlink_inode_operations;inode_nohighmem(inode);(*inode).i_data.a_ops=&affs_symlink_aops;(*inode).i_mode=S_IFLNK|0o777;affs_mode_to_prot(inode);let mut bh=affs_bread(sb,(*inode).i_ino);if bh.is_null(){clear_nlink(inode);mark_inode_dirty(inode);iput(inode);return -EIO;}let mut i=0;let mut p=(*AFFS_HEAD(bh)).table as *mut i8;let mut lc=b'/';let mut s=symname;if *s==b'/' as i8{while *s==b'/' as i8{s=s.add(1);}let sbi=AFFS_SB(sb);spin_lock(&mut (*sbi).symlink_lock);while (*sbi).s_volume[i]!=0{*p=(*sbi).s_volume[i] as i8;p=p.add(1);i+=1;}spin_unlock(&mut (*sbi).symlink_lock);}while i<maxlen{let c=*s;s=s.add(1);if c==0{break;}if c==b'.' as i8&&lc==b'/'&&*s==b'.' as i8&&*s.add(1)==b'/' as i8{*p=b'/ ' as i8;p=p.add(1);i+=1;s=s.add(2);lc=b'/';}else if c==b'.' as i8&&lc==b'/'&&*s==b'/' as i8{s=s.add(1);lc=b'/';}else{*p=c;p=p.add(1);lc=c;i+=1;}if lc==b'/'{while *s==b'/' as i8{s=s.add(1);}}}*p=0;(*inode).i_size=(i+1) as i64;mark_buffer_dirty(bh);affs_brelse(bh);mark_inode_dirty(inode);let e=affs_add_entry(dir,inode,d,ST_SOFTLINK);if e!=0{clear_nlink(inode);mark_inode_dirty(inode);iput(inode);}e}
pub unsafe fn affs_link(old:*mut dentry,dir:*mut inode,d:*mut dentry)->i32{affs_add_entry(dir,d_inode(old),d,ST_LINKFILE)}

unsafe fn affs_rename(old_dir:*mut inode,old:*mut dentry,new_dir:*mut inode,new:*mut dentry)->i32{let sb=(*old_dir).i_sb;let e=affs_check_name((*new).d_name.name,(*new).d_name.len,affs_nofilenametruncate(old));if e!=0{return e;}if d_really_is_positive(new){let e=affs_remove_header(new);if e!=0{return e;}}let bh=affs_bread(sb,(*d_inode(old)).i_ino);if bh.is_null(){return -EIO;}affs_lock_dir(old_dir);let mut r=affs_remove_hash(old_dir,bh);affs_unlock_dir(old_dir);if r==0{affs_copy_name((*AFFS_TAIL(sb,bh)).name,new);affs_fix_checksum(sb,bh);affs_lock_dir(new_dir);r=affs_insert_hash(new_dir,bh);affs_unlock_dir(new_dir);}mark_buffer_dirty(bh);affs_brelse(bh);r}
unsafe fn affs_xrename(old_dir:*mut inode,old:*mut dentry,new_dir:*mut inode,new:*mut dentry)->i32{let sb=(*old_dir).i_sb;let a=affs_bread(sb,(*d_inode(old)).i_ino);if a.is_null(){return -EIO;}let b=affs_bread(sb,(*d_inode(new)).i_ino);if b.is_null(){affs_brelse(a);return -EIO;}affs_lock_dir(old_dir);let mut r=affs_remove_hash(old_dir,a);affs_unlock_dir(old_dir);if r==0{affs_lock_dir(new_dir);r=affs_remove_hash(new_dir,b);affs_unlock_dir(new_dir);}if r==0{affs_copy_name((*AFFS_TAIL(sb,a)).name,new);affs_fix_checksum(sb,a);affs_lock_dir(new_dir);r=affs_insert_hash(new_dir,a);affs_unlock_dir(new_dir);affs_copy_name((*AFFS_TAIL(sb,b)).name,old);affs_fix_checksum(sb,b);affs_lock_dir(old_dir);r=affs_insert_hash(old_dir,b);affs_unlock_dir(old_dir);}mark_buffer_dirty(a);mark_buffer_dirty(b);affs_brelse(a);affs_brelse(b);r}
pub unsafe fn affs_rename2(_idmap:*mut mnt_idmap,od:*mut inode,old:*mut dentry,nd:*mut inode,new:*mut dentry,flags:u32)->i32{if flags&!(RENAME_NOREPLACE|RENAME_EXCHANGE)!=0{return -EINVAL;}if flags&RENAME_EXCHANGE!=0{affs_xrename(od,old,nd,new)}else{affs_rename(od,old,nd,new)}}

unsafe fn affs_get_parent(child:*mut dentry)->*mut dentry{let bh=affs_bread((*child).d_sb,(*d_inode(child)).i_ino);if bh.is_null(){return ERR_PTR(-EIO);}let p=affs_iget((*child).d_sb,be32_to_cpu((*AFFS_TAIL((*child).d_sb,bh)).parent));brelse(bh);d_obtain_alias(p)}
unsafe fn affs_nfs_get_inode(sb:*mut super_block,ino:u64,_generation:u32)->*mut inode{if !affs_validblock(sb,ino){return ERR_PTR(-ESTALE);}affs_iget(sb,ino)}
unsafe fn affs_fh_to_dentry(sb:*mut super_block,fid:*mut fid,len:i32,typ:i32)->*mut dentry{generic_fh_to_dentry(sb,fid,len,typ,affs_nfs_get_inode)}
unsafe fn affs_fh_to_parent(sb:*mut super_block,fid:*mut fid,len:i32,typ:i32)->*mut dentry{generic_fh_to_parent(sb,fid,len,typ,affs_nfs_get_inode)}

#[repr(C)] pub struct export_operations { pub encode_fh: unsafe fn(), pub fh_to_dentry: unsafe fn(*mut super_block,*mut fid,i32,i32)->*mut dentry, pub fh_to_parent: unsafe fn(*mut super_block,*mut fid,i32,i32)->*mut dentry, pub get_parent: unsafe fn(*mut dentry)->*mut dentry }
#[repr(C)] pub struct dentry_operations { pub d_hash: unsafe fn(*const dentry,*mut qstr)->i32, pub d_compare: unsafe fn(*const dentry,u32,*const i8,*const qstr)->i32 }
extern "C" { static mut generic_encode_ino32_fh: unsafe fn(); }
pub static mut affs_export_ops: export_operations=export_operations{encode_fh:generic_encode_ino32_fh,fh_to_dentry:affs_fh_to_dentry,fh_to_parent:affs_fh_to_parent,get_parent:affs_get_parent};
pub static mut affs_dentry_operations:dentry_operations=dentry_operations{d_hash:affs_hash_dentry,d_compare:affs_compare_dentry};
pub static mut affs_intl_dentry_operations:dentry_operations=dentry_operations{d_hash:affs_intl_hash_dentry,d_compare:affs_intl_compare_dentry};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
