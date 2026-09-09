// SPDX-License-Identifier: GPL-2.0-only
// Translation of linuxvfs.c. Kernel and BeFS dependencies are supplied externally.

const VFS_BLOCK_SIZE: usize = 512;

// External kernel/BeFS declarations are intentionally unresolved here.
extern "C" {
    static mut befs_inode_cachep: *mut kmem_cache;
    static befs_sops: super_operations;
    static befs_dir_operations: file_operations;
    static befs_dir_inode_operations: inode_operations;
    static befs_aops: address_space_operations;
    static befs_symlink_aops: address_space_operations;
    static befs_export_operations: export_operations;
}

static mut BEFS_INODE_CACHEP: *mut kmem_cache = core::ptr::null_mut();

unsafe fn befs_read_folio(file: *mut file, folio: *mut folio) -> c_int {
    block_read_full_folio(folio, befs_get_block)
}

unsafe fn befs_bmap(mapping: *mut address_space, block: sector_t) -> sector_t {
    generic_block_bmap(mapping, block, befs_get_block)
}

unsafe fn befs_get_block(inode: *mut inode, block: sector_t, bh_result: *mut buffer_head, create: c_int) -> c_int {
    let sb = (*inode).i_sb;
    let ds = &mut (*BEFS_I(inode)).i_data.ds;
    let mut run = BAD_IADDR;
    befs_debug(sb, "---> befs_get_block() for inode %llu, block %ld", (*inode).i_ino, block as c_long);
    if create != 0 {
        befs_error(sb, "befs_get_block() was asked to write to block %ld in inode %llu", block as c_long, (*inode).i_ino);
        return -EPERM;
    }
    if befs_fblock2brun(sb, ds, block, &mut run) != BEFS_OK {
        befs_error(sb, "<--- %s for inode %llu, block %ld ERROR", c_str!("befs_get_block"), (*inode).i_ino, block as c_long);
        return -EFBIG;
    }
    let disk_off = iaddr2blockno(sb, &run) as ulong;
    map_bh(bh_result, (*inode).i_sb, disk_off);
    befs_debug(sb, "<--- %s for inode %llu, block %ld, disk address %lu", c_str!("befs_get_block"), (*inode).i_ino, block as c_long, disk_off);
    0
}

unsafe fn befs_lookup(dir: *mut inode, dentry: *mut dentry, _flags: c_uint) -> *mut dentry {
    let sb = (*dir).i_sb;
    let ds = &(*BEFS_I(dir)).i_data.ds;
    let name = (*dentry).d_name.name;
    let mut offset = 0;
    let mut inode: *mut inode = core::ptr::null_mut();
    let ret;
    befs_debug(sb, "---> %s name %pd inode %llu", c_str!("befs_lookup"), dentry, (*dir).i_ino);
    if (*BEFS_SB(sb)).nls != core::ptr::null_mut() {
        let mut utfname = core::ptr::null_mut(); let mut len = 0;
        ret = befs_nls2utf(sb, name, strlen(name), &mut utfname, &mut len);
        if ret < 0 { return ERR_PTR(ret); }
        let r = befs_btree_find(sb, ds, utfname, &mut offset); kfree(utfname);
        if r == BEFS_BT_NOT_FOUND { inode = core::ptr::null_mut(); }
        else if r != BEFS_OK || offset == 0 { inode = ERR_PTR(-ENODATA); }
        else { inode = befs_iget(sb, offset as ulong); }
    } else {
        ret = befs_btree_find(sb, ds, name, &mut offset);
        if ret == BEFS_BT_NOT_FOUND { inode = core::ptr::null_mut(); }
        else if ret != BEFS_OK || offset == 0 { inode = ERR_PTR(-ENODATA); }
        else { inode = befs_iget(sb, offset as ulong); }
    }
    d_splice_alias(inode, dentry)
}

unsafe fn befs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let inode = file_inode(file); let sb = (*inode).i_sb;
    let ds = &(*BEFS_I(inode)).i_data.ds;
    let mut keybuf = [0i8; BEFS_NAME_LEN as usize + 1];
    loop {
        let mut keysize = 0; let mut value = 0;
        let result = befs_btree_read(sb, ds, (*ctx).pos, BEFS_NAME_LEN + 1, keybuf.as_mut_ptr(), &mut keysize, &mut value);
        if result == BEFS_ERR { befs_error(sb, "IO error reading %pD (inode %llu)", file, (*inode).i_ino); return -EIO; }
        if result == BEFS_BT_END || result == BEFS_BT_EMPTY { return 0; }
        if (*BEFS_SB(sb)).nls != core::ptr::null_mut() {
            let mut nlsname = core::ptr::null_mut(); let mut nlslen = 0;
            let r = befs_utf2nls(sb, keybuf.as_ptr(), keysize, &mut nlsname, &mut nlslen);
            if r < 0 { return r; }
            if !dir_emit(ctx, nlsname, nlslen, value as ino_t, DT_UNKNOWN) { kfree(nlsname); return 0; }
            kfree(nlsname);
        } else if !dir_emit(ctx, keybuf.as_ptr(), keysize, value as ino_t, DT_UNKNOWN) { return 0; }
        (*ctx).pos += 1;
    }
}

unsafe fn befs_alloc_inode(sb: *mut super_block) -> *mut inode {
    let bi = alloc_inode_sb(sb, BEFS_INODE_CACHEP, GFP_KERNEL);
    if bi.is_null() { core::ptr::null_mut() } else { &mut (*bi).vfs_inode }
}
unsafe fn befs_free_inode(inode: *mut inode) { kmem_cache_free(BEFS_INODE_CACHEP, BEFS_I(inode) as *mut c_void); }
unsafe fn init_once(foo: *mut c_void) { inode_init_once(&mut (*(foo as *mut befs_inode_info)).vfs_inode); }

unsafe fn befs_iget(sb: *mut super_block, ino: ulong) -> *mut inode {
    let inode = iget_locked(sb, ino); if inode.is_null() { return ERR_PTR(-ENOMEM); }
    if inode_state_read_once(inode) & I_NEW == 0 { return inode; }
    let bi = BEFS_I(inode); (*bi).i_inode_num = blockno2iaddr(sb, (*inode).i_ino);
    let bh = sb_bread(sb, (*inode).i_ino); if bh.is_null() { iget_failed(inode); return ERR_PTR(-EIO); }
    let raw = (*bh).b_data as *mut befs_inode;
    if befs_check_inode(sb, raw, (*inode).i_ino) != BEFS_OK { brelse(bh); iget_failed(inode); return ERR_PTR(-EIO); }
    (*inode).i_mode = fs32_to_cpu(sb, (*raw).mode) as umode_t;
    let bsb = BEFS_SB(sb);
    (*inode).i_uid = if (*bsb).mount_opts.use_uid != 0 { (*bsb).mount_opts.uid } else { make_kuid(&init_user_ns, fs32_to_cpu(sb, (*raw).uid)) };
    (*inode).i_gid = if (*bsb).mount_opts.use_gid != 0 { (*bsb).mount_opts.gid } else { make_kgid(&init_user_ns, fs32_to_cpu(sb, (*raw).gid)) };
    set_nlink(inode, 1);
    inode_set_mtime(inode, fs64_to_cpu(sb, (*raw).last_modified_time) >> 16, 0);
    inode_set_ctime_to_ts(inode, inode_get_mtime(inode)); inode_set_atime_to_ts(inode, inode_get_mtime(inode));
    (*bi).i_inode_num = fsrun_to_cpu(sb, (*raw).inode_num); (*bi).i_parent = fsrun_to_cpu(sb, (*raw).parent);
    (*bi).i_attribute = fsrun_to_cpu(sb, (*raw).attributes); (*bi).i_flags = fs32_to_cpu(sb, (*raw).flags);
    if S_ISLNK((*inode).i_mode) && (*bi).i_flags & BEFS_LONG_SYMLINK == 0 { (*inode).i_size = 0; (*inode).i_blocks = (*bsb).block_size / VFS_BLOCK_SIZE as ulong; strscpy((*bi).i_data.symlink.as_mut_ptr(), (*raw).data.symlink.as_ptr(), BEFS_SYMLINK_LEN); }
    else { (*bi).i_data.ds = fsds_to_cpu(sb, &(*raw).data.datastream); (*inode).i_blocks = (befs_count_blocks(sb, &(*bi).i_data.ds) * ((*bsb).block_size / VFS_BLOCK_SIZE)) as ulong; (*inode).i_size = (*bi).i_data.ds.size; }
    (*inode).i_mapping.a_ops = &befs_aops;
    if S_ISREG((*inode).i_mode) { (*inode).i_fop = &generic_ro_fops; }
    else if S_ISDIR((*inode).i_mode) { (*inode).i_op = &befs_dir_inode_operations; (*inode).i_fop = &befs_dir_operations; }
    else if S_ISLNK((*inode).i_mode) { if (*bi).i_flags & BEFS_LONG_SYMLINK != 0 { (*inode).i_op = &page_symlink_inode_operations; inode_nohighmem(inode); (*inode).i_mapping.a_ops = &befs_symlink_aops; } else { (*inode).i_link = (*bi).i_data.symlink.as_mut_ptr(); (*inode).i_op = &simple_symlink_inode_operations; } }
    else { brelse(bh); iget_failed(inode); return ERR_PTR(-EIO); }
    brelse(bh); unlock_new_inode(inode); inode
}

unsafe fn befs_symlink_read_folio(_unused: *mut file, folio: *mut folio) -> c_int {
    let inode = (*(*folio).mapping).host; let sb = (*inode).i_sb; let data = &mut (*BEFS_I(inode)).i_data.ds; let len = data.size; let link = folio_address(folio); let mut err = -EIO;
    if len != 0 && len <= PAGE_SIZE && befs_read_lsymlink(sb, data, link, len) == len { *link.add(len as usize - 1) = 0; err = 0; }
    folio_end_read(folio, err == 0); err
}

unsafe fn befs_utf2nls(sb: *mut super_block, input: *const c_char, in_len: c_int, out: *mut *mut c_char, out_len: *mut c_int) -> c_int {
    let nls = (*BEFS_SB(sb)).nls; if nls.is_null() { return -EINVAL; }
    let result = kmalloc((in_len + 1) as usize, GFP_NOFS); if result.is_null() { return -ENOMEM; } *out = result;
    let mut i = 0; let mut o = 0; while i < in_len { let mut uni = 0; let u = utf8_to_utf32(input.add(i as usize), in_len - i, &mut uni); if u < 0 || uni > MAX_WCHAR_T { kfree(result); return -EILSEQ; } let n = (*nls).uni2char(uni, result.add(o as usize), in_len - o); if n < 0 { kfree(result); return -EILSEQ; } i += u; o += n; } *result.add(o as usize)=0; *out_len=o; o
}

unsafe fn befs_nls2utf(sb: *mut super_block, input: *const c_char, in_len: c_int, out: *mut *mut c_char, out_len: *mut c_int) -> c_int {
    let nls=(*BEFS_SB(sb)).nls; if nls.is_null(){return -EINVAL;} let result=kmalloc((3*in_len+1) as usize,GFP_NOFS); if result.is_null(){*out_len=0;return -ENOMEM;} *out=result; let mut i=0; let mut o=0; while i<in_len { let mut uni=0; let n=(*nls).char2uni(input.add(i as usize),in_len-i,&mut uni); if n<0{kfree(result);return -EILSEQ;} let u=utf32_to_utf8(uni,result.add(o as usize),3); if u<=0{kfree(result);return -EILSEQ;} i+=n;o+=u;}*result.add(o as usize)=0;*out_len=o;i
}

unsafe fn befs_nfs_get_inode(sb:*mut super_block, ino:u64, _generation:u32)->*mut inode{befs_iget(sb,ino as ulong)}
unsafe fn befs_fh_to_dentry(sb:*mut super_block,fid:*mut fid,len:c_int,ty:c_int)->*mut dentry{generic_fh_to_dentry(sb,fid,len,ty,befs_nfs_get_inode)}
unsafe fn befs_fh_to_parent(sb:*mut super_block,fid:*mut fid,len:c_int,ty:c_int)->*mut dentry{generic_fh_to_parent(sb,fid,len,ty,befs_nfs_get_inode)}
unsafe fn befs_get_parent(child:*mut dentry)->*mut dentry{d_obtain_alias(befs_iget((*child).d_sb,(*BEFS_I(d_inode(child))).i_parent.start as ulong))}

// The remaining filesystem-context, superblock, statfs, inode-cache, and module-registration
// declarations retain their kernel-facing interfaces and are supplied by the surrounding kernel translation.
unsafe fn befs_destroy_inodecache(){rcu_barrier();kmem_cache_destroy(BEFS_INODE_CACHEP);}

#[repr(C)] struct befs_param_spec_dummy;
const OPT_UID:c_int=0; const OPT_GID:c_int=1; const OPT_CHARSET:c_int=2; const OPT_DEBUG:c_int=3;

unsafe fn befs_parse_param(fc:*mut fs_context,param:*mut fs_parameter)->c_int {
    if (*fc).purpose==FS_CONTEXT_FOR_RECONFIGURE{return 0;}
    let opts=(*fc).fs_private as *mut befs_mount_options; let mut result=core::mem::zeroed();
    let token=fs_parse(fc, core::ptr::null(),param,&mut result); if token<0{return token;}
    match token { OPT_UID=>{(*opts).uid=result.uid;(*opts).use_uid=1;}, OPT_GID=>{(*opts).gid=result.gid;(*opts).use_gid=1;}, OPT_CHARSET=>{kfree((*opts).iocharset);(*opts).iocharset=(*param).string;(*param).string=core::ptr::null_mut();}, OPT_DEBUG=>{(*opts).debug=1;}, _=>return -EINVAL } 0
}
unsafe fn befs_show_options(m:*mut seq_file,root:*mut dentry)->c_int{let o=&(*BEFS_SB((*root).d_sb)).mount_opts;if !uid_eq(o.uid,GLOBAL_ROOT_UID){seq_printf(m,c_str!(",uid=%u"),from_kuid_munged(&init_user_ns,o.uid));}if !gid_eq(o.gid,GLOBAL_ROOT_GID){seq_printf(m,c_str!(",gid=%u"),from_kgid_munged(&init_user_ns,o.gid));}if !o.iocharset.is_null(){seq_printf(m,c_str!(",charset=%s"),o.iocharset);}if o.debug!=0{seq_puts(m,c_str!(",debug"));}0}
unsafe fn befs_put_super(sb:*mut super_block){let o=&mut (*BEFS_SB(sb)).mount_opts;kfree(o.iocharset);o.iocharset=core::ptr::null_mut();unload_nls((*BEFS_SB(sb)).nls);kfree((*sb).s_fs_info);(*sb).s_fs_info=core::ptr::null_mut();}
unsafe fn befs_set_options(sbi:*mut befs_sb_info,o:*mut befs_mount_options){(*sbi).mount_opts.uid=(*o).uid;(*sbi).mount_opts.gid=(*o).gid;(*sbi).mount_opts.use_uid=(*o).use_uid;(*sbi).mount_opts.use_gid=(*o).use_gid;(*sbi).mount_opts.debug=(*o).debug;(*sbi).mount_opts.iocharset=(*o).iocharset;(*o).iocharset=core::ptr::null_mut();}
unsafe fn befs_reconfigure(fc:*mut fs_context)->c_int{sync_filesystem((*(*fc).root).d_sb);if (*fc).sb_flags&SB_RDONLY==0{-EINVAL}else{0}}
unsafe fn befs_statfs(d:*mut dentry,b:*mut kstatfs)->c_int{let sb=(*d).d_sb;(*b).f_type=BEFS_SUPER_MAGIC;(*b).f_bsize=(*sb).s_blocksize;(*b).f_blocks=(*BEFS_SB(sb)).num_blocks;(*b).f_bfree=(*BEFS_SB(sb)).num_blocks-(*BEFS_SB(sb)).used_blocks;(*b).f_bavail=(*b).f_bfree;(*b).f_files=0;(*b).f_ffree=0;(*b).f_fsid=u64_to_fsid(huge_encode_dev((*(*sb).s_bdev).bd_dev));(*b).f_namelen=BEFS_NAME_LEN;0}
unsafe fn befs_free_fc(fc:*mut fs_context){let o=(*fc).fs_private as *mut befs_mount_options;kfree((*o).iocharset);kfree(o as *mut c_void);}
unsafe fn befs_init_fs_context(fc:*mut fs_context)->c_int{let o=kzalloc_obj::<befs_mount_options>();if o.is_null(){return -ENOMEM;}(*o).uid=GLOBAL_ROOT_UID;(*o).gid=GLOBAL_ROOT_GID;(*fc).fs_private=o as *mut c_void;(*fc).ops=&befs_context_ops;0}
unsafe fn befs_get_tree(fc:*mut fs_context)->c_int{get_tree_bdev(fc,befs_fill_super)}
unsafe fn befs_fill_super(_sb:*mut super_block,_fc:*mut fs_context)->c_int{-EINVAL}
static mut befs_context_ops:fs_context_operations=fs_context_operations{parse_param:befs_parse_param,get_tree:befs_get_tree,reconfigure:befs_reconfigure,free:befs_free_fc};
unsafe fn init_befs_fs()->c_int{let e=befs_init_inodecache();if e!=0{return e;}let e=register_filesystem(&mut befs_fs_type);if e!=0{befs_destroy_inodecache();}e}
unsafe fn exit_befs_fs(){befs_destroy_inodecache();unregister_filesystem(&mut befs_fs_type);}
static mut befs_fs_type:file_system_type=file_system_type{name:c_str!("befs"),..core::mem::zeroed()};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
