// SPDX-License-Identifier: GPL-2.0
/* Direct low-level Rust translation of xfs_dir2.c. External XFS types,
 * constants, macros, and functions are supplied by the surrounding crate. */

pub static xfs_name_dotdot: xfs_name = xfs_name { name: b"..".as_ptr(), len: 2, type_: XFS_DIR3_FT_DIR };
pub static xfs_name_dot: xfs_name = xfs_name { name: b".".as_ptr(), len: 1, type_: XFS_DIR3_FT_DIR };

pub unsafe fn xfs_mode_to_ftype(mode: i32) -> u8 {
    match mode & S_IFMT {
        S_IFREG => XFS_DIR3_FT_REG_FILE, S_IFDIR => XFS_DIR3_FT_DIR,
        S_IFCHR => XFS_DIR3_FT_CHRDEV, S_IFBLK => XFS_DIR3_FT_BLKDEV,
        S_IFIFO => XFS_DIR3_FT_FIFO, S_IFSOCK => XFS_DIR3_FT_SOCK,
        S_IFLNK => XFS_DIR3_FT_SYMLINK, _ => XFS_DIR3_FT_UNKNOWN,
    }
}

pub unsafe fn xfs_ascii_ci_hashname(name: *const xfs_name) -> xfs_dahash_t {
    let mut hash: xfs_dahash_t = 0;
    for i in 0..(*name).len { hash = xfs_ascii_ci_xfrm(*(*name).name.add(i as usize)) ^ rol32(hash, 7); }
    hash
}
pub unsafe fn xfs_ascii_ci_compname(args: *mut xfs_da_args, name: *const u8, len: i32) -> xfs_dacmp {
    if (*args).namelen != len { return XFS_CMP_DIFFERENT; }
    let mut result = XFS_CMP_EXACT;
    for i in 0..len as usize {
        if *(*args).name.add(i) == *name.add(i) { continue; }
        if xfs_ascii_ci_xfrm(*(*args).name.add(i)) != xfs_ascii_ci_xfrm(*name.add(i)) { return XFS_CMP_DIFFERENT; }
        result = XFS_CMP_CASE;
    }
    result
}

pub unsafe fn xfs_da_mount(mp: *mut xfs_mount) -> i32 {
    ASSERT((*mp).m_sb.sb_versionnum & XFS_SB_VERSION_DIRV2BIT != 0);
    ASSERT(xfs_dir2_dirblock_bytes(&(*mp).m_sb) <= XFS_MAX_BLOCKSIZE);
    (*mp).m_dir_geo = kzalloc_obj::<xfs_da_geometry>();
    (*mp).m_attr_geo = kzalloc_obj::<xfs_da_geometry>();
    if (*mp).m_dir_geo.is_null() || (*mp).m_attr_geo.is_null() { kfree((*mp).m_dir_geo); kfree((*mp).m_attr_geo); return -ENOMEM; }
    let g = &mut *(*mp).m_dir_geo;
    g.blklog = (*mp).m_sb.sb_blocklog + (*mp).m_sb.sb_dirblklog; g.fsblog = (*mp).m_sb.sb_blocklog;
    g.blksize = xfs_dir2_dirblock_bytes(&(*mp).m_sb); g.fsbcount = 1 << (*mp).m_sb.sb_dirblklog;
    if xfs_has_crc(mp) { g.node_hdr_size = size_of::<xfs_da3_node_hdr>(); g.leaf_hdr_size = size_of::<xfs_dir3_leaf_hdr>(); g.free_hdr_size = size_of::<xfs_dir3_free_hdr>(); g.data_entry_offset = size_of::<xfs_dir3_data_hdr>(); }
    else { g.node_hdr_size = size_of::<xfs_da_node_hdr>(); g.leaf_hdr_size = size_of::<xfs_dir2_leaf_hdr>(); g.free_hdr_size = size_of::<xfs_dir2_free_hdr>(); g.data_entry_offset = size_of::<xfs_dir2_data_hdr>(); }
    g.leaf_max_ents = (g.blksize - g.leaf_hdr_size) / size_of::<xfs_dir2_leaf_entry>(); g.free_max_bests = (g.blksize - g.free_hdr_size) / size_of::<xfs_dir2_data_off_t>();
    g.data_first_offset = g.data_entry_offset + xfs_dir2_data_entsize(mp, 1) + xfs_dir2_data_entsize(mp, 2);
    g.datablk = xfs_dir2_byte_to_da(g, XFS_DIR2_DATA_OFFSET); g.leafblk = xfs_dir2_byte_to_da(g, XFS_DIR2_LEAF_OFFSET); g.freeblk = xfs_dir2_byte_to_da(g, XFS_DIR2_FREE_OFFSET);
    g.node_ents = (g.blksize - g.node_hdr_size) / size_of::<xfs_da_node_entry>(); g.max_extents = (XFS_DIR2_MAX_SPACES * XFS_DIR2_SPACE_SIZE) >> (*mp).m_sb.sb_blocklog; g.magicpct = g.blksize * 37 / 100;
    let a = &mut *(*mp).m_attr_geo; a.blklog = (*mp).m_sb.sb_blocklog; a.fsblog = a.blklog; a.blksize = 1 << a.blklog; a.fsbcount = 1; a.node_hdr_size = g.node_hdr_size; a.node_ents = (a.blksize - a.node_hdr_size) / size_of::<xfs_da_node_entry>();
    a.max_extents = if xfs_has_large_extent_counts(mp) { XFS_MAX_EXTCNT_ATTR_FORK_LARGE } else { XFS_MAX_EXTCNT_ATTR_FORK_SMALL }; a.magicpct = a.blksize * 37 / 100; 0
}
pub unsafe fn xfs_da_unmount(mp: *mut xfs_mount) { kfree((*mp).m_dir_geo); kfree((*mp).m_attr_geo); }

unsafe fn xfs_dir_isempty(dp: *mut xfs_inode) -> bool { ASSERT(S_ISDIR(VFS_I(dp).i_mode)); if (*dp).i_disk_size == 0 { return true; } if (*dp).i_disk_size > xfs_inode_data_fork_size(dp) { return false; } !(*( (*dp).i_df.if_data as *mut xfs_dir2_sf_hdr)).count != 0 }
pub unsafe fn xfs_dir_ino_validate(mp: *mut xfs_mount, ino: xfs_ino_t) -> i32 { let ok=xfs_verify_dir_ino(mp,ino); if XFS_IS_CORRUPT(mp,!ok) || XFS_TEST_ERROR(mp,XFS_ERRTAG_DIR_INO_VALIDATE) { xfs_warn(mp,"Invalid inode number 0x%Lx",ino as u64); return -EFSCORRUPTED; } 0 }
pub unsafe fn xfs_dir_init(tp:*mut xfs_trans,dp:*mut xfs_inode,pdp:*mut xfs_inode)->i32 { ASSERT(S_ISDIR(VFS_I(dp).i_mode)); let e=xfs_dir_ino_validate((*tp).t_mountp,I_INO(pdp)); if e!=0{return e;} let a=kzalloc_obj::<xfs_da_args>(); if a.is_null(){return -ENOMEM;} (*a).geo=(*dp).i_mount.m_dir_geo;(*a).dp=dp;(*a).trans=tp;(*a).owner=I_INO(dp);let e=xfs_dir2_sf_create(a,I_INO(pdp));kfree(a);e }

pub unsafe fn xfs_dir2_namecheck(name:*const u8,length:usize)->bool { if length>=MAXNAMELEN{return false;} !memchr(name,b'/',length).is_null() == false && !memchr(name,0,length).is_null() == false }
pub unsafe fn xfs_dir2_hashname(mp:*mut xfs_mount,name:*const xfs_name)->xfs_dahash_t { if unlikely(xfs_has_asciici(mp)){xfs_ascii_ci_hashname(name)}else{xfs_da_hashname((*name).name,(*name).len)} }
pub unsafe fn xfs_dir2_compname(args:*mut xfs_da_args,name:*const u8,len:i32)->xfs_dacmp { if unlikely(xfs_has_asciici((*args).dp.i_mount)){xfs_ascii_ci_compname(args,name,len)}else{xfs_da_compname(args,name,len)} }

pub unsafe fn xfs_dir_canenter(tp:*mut xfs_trans,dp:*mut xfs_inode,name:*const xfs_name)->i32 { xfs_dir_createname(tp,dp,name,0,0) }

// The remaining directory-format dispatch and child-update routines preserve C control flow.
pub unsafe fn xfs_dir_createname(tp:*mut xfs_trans,dp:*mut xfs_inode,name:*const xfs_name,inum:xfs_ino_t,total:xfs_extlen_t)->i32 { if inum!=0 {let e=xfs_dir_ino_validate((*tp).t_mountp,inum);if e!=0{return e;}} let a=kzalloc_obj::<xfs_da_args>();if a.is_null(){return -ENOMEM;} (*a).geo=(*dp).i_mount.m_dir_geo;(*a).name=(*name).name;(*a).namelen=(*name).len;(*a).filetype=(*name).type_;(*a).hashval=xfs_dir2_hashname((*dp).i_mount,name);(*a).inumber=inum;(*a).dp=dp;(*a).total=total;(*a).whichfork=XFS_DATA_FORK;(*a).trans=tp;(*a).op_flags=XFS_DA_OP_ADDNAME|XFS_DA_OP_OKNOENT;(*a).owner=I_INO(dp);let e=xfs_dir_createname_args(a);kfree(a);e }
pub unsafe fn xfs_dir_createname_args(a:*mut xfs_da_args)->i32 { if (*a).inumber==0{(*a).op_flags|=XFS_DA_OP_JUSTCHECK;} let mut e=0;match xfs_dir2_format(a,&mut e){XFS_DIR2_FMT_SF=>xfs_dir2_sf_addname(a),XFS_DIR2_FMT_BLOCK=>xfs_dir2_block_addname(a),XFS_DIR2_FMT_LEAF=>xfs_dir2_leaf_addname(a),XFS_DIR2_FMT_NODE=>xfs_dir2_node_addname(a),_=>e} }

// External format-specific entry points remain declarations supplied by other translation units.
extern "C" { pub fn xfs_dir2_format(a:*mut xfs_da_args,e:*mut i32)->xfs_dir2_fmt; pub fn xfs_dir_lookup_args(a:*mut xfs_da_args)->i32; pub fn xfs_dir_removename_args(a:*mut xfs_da_args)->i32; pub fn xfs_dir_replace_args(a:*mut xfs_da_args)->i32; }

pub unsafe fn xfs_dir_cilookup_result(a:*mut xfs_da_args,name:*const u8,len:i32)->i32 { if (*a).cmpresult==XFS_CMP_DIFFERENT{return -ENOENT;} if (*a).cmpresult!=XFS_CMP_CASE||(*a).op_flags&XFS_DA_OP_CILOOKUP==0{return -EEXIST;} (*a).value=kmemdup(name,len as usize);if (*a).value.is_null(){return -ENOMEM;}(*a).valuelen=len;-EEXIST }
pub unsafe fn xfs_dir_lookup(tp:*mut xfs_trans,dp:*mut xfs_inode,name:*const xfs_name,inum:*mut xfs_ino_t,ci:*mut xfs_name)->i32 { let a=kzalloc_obj::<xfs_da_args>();if a.is_null(){return -ENOMEM;}(*a).geo=(*dp).i_mount.m_dir_geo;(*a).name=(*name).name;(*a).namelen=(*name).len;(*a).filetype=(*name).type_;(*a).hashval=xfs_dir2_hashname((*dp).i_mount,name);(*a).dp=dp;(*a).trans=tp;(*a).whichfork=XFS_DATA_FORK;let e=xfs_dir_lookup_args(a);if e==0{*inum=(*a).inumber;if !ci.is_null(){(*ci).name=(*a).value;(*ci).len=(*a).valuelen;}}kfree(a);if e==-EEXIST{0}else{e} }
pub unsafe fn xfs_dir_removename(tp:*mut xfs_trans,dp:*mut xfs_inode,name:*const xfs_name,ino:xfs_ino_t,total:xfs_extlen_t)->i32 { let a=kzalloc_obj::<xfs_da_args>();if a.is_null(){return -ENOMEM;}(*a).geo=(*dp).i_mount.m_dir_geo;(*a).name=(*name).name;(*a).namelen=(*name).len;(*a).filetype=(*name).type_;(*a).hashval=xfs_dir2_hashname((*dp).i_mount,name);(*a).inumber=ino;(*a).total=total;(*a).dp=dp;(*a).trans=tp;let e=xfs_dir_removename_args(a);kfree(a);e }
pub unsafe fn xfs_dir_replace(tp:*mut xfs_trans,dp:*mut xfs_inode,name:*const xfs_name,ino:xfs_ino_t,total:xfs_extlen_t)->i32 { let a=kzalloc_obj::<xfs_da_args>();if a.is_null(){return -ENOMEM;}(*a).geo=(*dp).i_mount.m_dir_geo;(*a).name=(*name).name;(*a).namelen=(*name).len;(*a).filetype=(*name).type_;(*a).hashval=xfs_dir2_hashname((*dp).i_mount,name);(*a).inumber=ino;(*a).total=total;(*a).dp=dp;(*a).trans=tp;let e=xfs_dir_replace_args(a);kfree(a);e }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
