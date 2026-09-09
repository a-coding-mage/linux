// SPDX-License-Identifier: GPL-2.0-only
/* QNX6 file system, Linux implementation. Rust translation of inode.c. */

// Kernel types, constants, macros, and functions referenced below are supplied
// by the translated kernel/QNX6 dependencies.

static mut QNX6_SOPS: super_operations = super_operations {
    alloc_inode: Some(qnx6_alloc_inode), free_inode: Some(qnx6_free_inode),
    put_super: Some(qnx6_put_super), statfs: Some(qnx6_statfs),
    show_options: Some(qnx6_show_options),
};

unsafe fn qnx6_show_options(seq: *mut seq_file, root: *mut dentry) -> i32 {
    let sb = (*root).d_sb;
    let sbi = QNX6_SB(sb);
    if ((*sbi).s_mount_opt & QNX6_MOUNT_MMI_FS) != 0 { seq_puts(seq, ",mmi_fs"); }
    0
}

unsafe fn qnx6_reconfigure(fc: *mut fs_context) -> i32 {
    let sb = (*(*fc).root).d_sb;
    sync_filesystem(sb); (*fc).sb_flags |= SB_RDONLY; 0
}

unsafe fn qnx6_get_devblock(sb: *mut super_block, block: __fs32) -> c_uint {
    fs32_to_cpu(QNX6_SB(sb), block) + (*QNX6_SB(sb)).s_blks_off
}

unsafe fn qnx6_get_block(inode: *mut inode, iblock: sector_t,
                         bh: *mut buffer_head, _create: i32) -> i32 {
    pr_debug!("qnx6_get_block inode=[%llu] iblock=[%ld]\n", (*inode).i_ino, iblock as c_ulong);
    let phys = qnx6_block_map(inode, iblock as c_uint);
    if phys != 0 { map_bh(bh, (*inode).i_sb, phys); } 0
}

unsafe fn qnx6_check_blockptr(ptr: __fs32) -> i32 {
    if ptr == !0 as __fs32 { pr_err!("hit unused blockpointer.\n"); 0 } else { 1 }
}
unsafe fn qnx6_read_folio(_file: *mut file, folio: *mut folio) -> i32 { mpage_read_folio(folio, qnx6_get_block) }
unsafe fn qnx6_readahead(rac: *mut readahead_control) { mpage_readahead(rac, qnx6_get_block); }

unsafe fn qnx6_block_map(inode: *mut inode, no: c_uint) -> c_uint {
    let s = (*inode).i_sb; let sbi = QNX6_SB(s); let ei = QNX6_I(inode);
    let mut block = 0; let mut bitdelta = (*sbi).s_ptrbits * (*ei).di_filelevels as i32;
    let mask = (1u32 << (*sbi).s_ptrbits) - 1;
    let mut levelptr = no >> bitdelta; let depth = (*ei).di_filelevels;
    if levelptr > QNX6_NO_DIRECT_POINTERS - 1 { pr_err!("Requested file block number (%u) too big.", no); return 0; }
    block = qnx6_get_devblock(s, (*ei).di_block_ptr[levelptr as usize]);
    for _ in 0..depth {
        let bh = sb_bread(s, block); if bh.is_null() { pr_err!("Error reading block (%u)\n", block); return 0; }
        bitdelta -= (*sbi).s_ptrbits as i32; levelptr = (no >> bitdelta) & mask;
        let ptr = (*( (*bh).b_data as *const __fs32).add(levelptr as usize));
        if qnx6_check_blockptr(ptr) == 0 { return 0; }
        block = qnx6_get_devblock(s, ptr); brelse(bh);
    } block
}

unsafe fn qnx6_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32 {
    let sb = (*dentry).d_sb; let sbi = QNX6_SB(sb); let id = huge_encode_dev((*sb).s_bdev.bd_dev);
    (*buf).f_type=(*sb).s_magic; (*buf).f_bsize=(*sb).s_blocksize;
    (*buf).f_blocks=fs32_to_cpu(sbi, (*sbi).sb.sb_num_blocks); (*buf).f_bfree=fs32_to_cpu(sbi, (*sbi).sb.sb_free_blocks);
    (*buf).f_files=fs32_to_cpu(sbi, (*sbi).sb.sb_num_inodes); (*buf).f_ffree=fs32_to_cpu(sbi, (*sbi).sb.sb_free_inodes);
    (*buf).f_bavail=(*buf).f_bfree; (*buf).f_namelen=QNX6_LONG_NAME_MAX; (*buf).f_fsid=u64_to_fsid(id); 0
}

unsafe fn qnx6_private_inode(s: *mut super_block, p: *mut qnx6_root_node) -> *mut inode {
    let i = new_inode(s); if !i.is_null() { let ei=QNX6_I(i); let sbi=QNX6_SB(s);
        (*i).i_size=fs64_to_cpu(sbi,(*p).size); memcpy((*ei).di_block_ptr.as_mut_ptr() as *mut c_void,(*p).ptr.as_ptr() as *const c_void,core::mem::size_of_val(&(*p).ptr)); (*ei).di_filelevels=(*p).levels; (*i).i_mode=S_IFREG|S_IRUSR; (*(*i).i_mapping).a_ops=&QNX6_AOPS; } i
}

unsafe fn qnx6_iget(sb: *mut super_block, ino: c_uint) -> *mut inode {
    let sbi=QNX6_SB(sb); let inode=iget_locked(sb,ino); if inode.is_null(){return ERR_PTR(-ENOMEM)}; if (inode_state_read_once(inode)&I_NEW)==0{return inode}; let ei=QNX6_I(inode); (*inode).i_mode=0;
    if ino==0 { pr_err!("bad inode number on dev %s: %u is out of range\n",(*sb).s_id,ino); iget_failed(inode); return ERR_PTR(-EIO); }
    let n=(ino-1)>>(PAGE_SHIFT-QNX6_INODE_SIZE_BITS); let folio=read_mapping_folio((*(*sbi).inodes).i_mapping,n as c_ulong,core::ptr::null_mut()); if IS_ERR(folio){iget_failed(inode);return ERR_CAST(folio)};
    let raw=kmap_local_folio(folio,offset_in_folio(folio,(ino-1)<<QNX6_INODE_SIZE_BITS)); (*inode).i_mode=fs16_to_cpu(sbi,(*raw).di_mode); i_uid_write(inode,fs32_to_cpu(sbi,(*raw).di_uid) as uid_t); i_gid_write(inode,fs32_to_cpu(sbi,(*raw).di_gid) as gid_t); (*inode).i_size=fs64_to_cpu(sbi,(*raw).di_size); (*inode).i_blocks=((*inode).i_size+511)>>9; memcpy((*ei).di_block_ptr.as_mut_ptr() as *mut c_void,(*raw).di_block_ptr.as_ptr() as *const c_void,core::mem::size_of_val(&(*raw).di_block_ptr)); (*ei).di_filelevels=(*raw).di_filelevels;
    if S_ISREG((*inode).i_mode){(*inode).i_fop=&generic_ro_fops;(*(*inode).i_mapping).a_ops=&QNX6_AOPS} else if S_ISDIR((*inode).i_mode){(*inode).i_op=&qnx6_dir_inode_operations;(*inode).i_fop=&qnx6_dir_operations;(*(*inode).i_mapping).a_ops=&QNX6_AOPS} else if S_ISLNK((*inode).i_mode){(*inode).i_op=&page_symlink_inode_operations;inode_nohighmem(inode);(*(*inode).i_mapping).a_ops=&QNX6_AOPS} else {init_special_inode(inode,(*inode).i_mode,0)}; folio_release_kmap(folio,raw); unlock_new_inode(inode); inode
}

static mut qnx6_inode_cachep: *mut kmem_cache = core::ptr::null_mut();
unsafe fn qnx6_alloc_inode(sb:*mut super_block)->*mut inode { let ei=alloc_inode_sb(sb,qnx6_inode_cachep,GFP_KERNEL); if ei.is_null(){core::ptr::null_mut()}else{&mut (*ei).vfs_inode} }
unsafe fn qnx6_free_inode(i:*mut inode){kmem_cache_free(qnx6_inode_cachep,QNX6_I(i));}
unsafe fn init_once(foo:*mut c_void){inode_init_once(&mut (*((foo as *mut qnx6_inode_info))).vfs_inode);}
unsafe fn init_inodecache()->i32{qnx6_inode_cachep=kmem_cache_create(cstr!("qnx6_inode_cache"),core::mem::size_of::<qnx6_inode_info>(),0,SLAB_RECLAIM_ACCOUNT|SLAB_ACCOUNT,init_once);if qnx6_inode_cachep.is_null(){-ENOMEM}else{0}}
unsafe fn destroy_inodecache(){rcu_barrier();kmem_cache_destroy(qnx6_inode_cachep);}

// The remaining filesystem-context registration and superblock routines retain
// the C implementation's external kernel calls and lifecycle ordering.
unsafe fn qnx6_put_super(sb:*mut super_block){let qs=QNX6_SB(sb);brelse((*qs).sb_buf);iput((*qs).longfile);iput((*qs).inodes);kfree(qs);(*sb).s_fs_info=core::ptr::null_mut();}
unsafe fn qnx6_get_tree(fc:*mut fs_context)->i32{get_tree_bdev(fc,qnx6_fill_super)}
unsafe fn qnx6_free_fc(fc:*mut fs_context){kfree((*fc).fs_private);}
unsafe fn init_qnx6_fs()->i32{let e=init_inodecache();if e!=0{return e} register_filesystem(&mut qnx6_fs_type)}
unsafe fn exit_qnx6_fs(){unregister_filesystem(&mut qnx6_fs_type);destroy_inodecache();}

// Direct translations of the parser, superblock fill, and filesystem context
// tables. Their field types and helper definitions are provided by qnx6.h.
#[repr(C)] struct qnx6_context { s_mount_opts: c_ulong }
const OPT_MMIFS: i32 = 0;
unsafe fn qnx6_parse_param(fc:*mut fs_context,param:*mut fs_parameter)->i32 {
    let ctx=(*fc).fs_private as *mut qnx6_context; let mut result=core::mem::zeroed::<fs_parse_result>();
    let opt=fs_parse(fc,QNX6_PARAM_SPEC,param,&mut result); if opt<0{return opt};
    match opt { OPT_MMIFS=>{(*ctx).s_mount_opts|=QNX6_MOUNT_MMI_FS;0}, _=>-EINVAL }
}
unsafe fn qnx6_checkroot(s:*mut super_block)->*const c_char {
    let root=d_inode((*s).s_root); let folio=read_mapping_folio((*root).i_mapping,0,core::ptr::null_mut()); if IS_ERR(folio){return cstr!("error reading root directory")}; let p=kmap_local_folio(folio,0) as *mut qnx6_dir_entry; let bad=memcmp((*p).de_fname.as_ptr() as *const c_void,cstr!(".") as *const c_void,2)!=0 || memcmp((*p.add(1)).de_fname.as_ptr() as *const c_void,cstr!("..") as *const c_void,3)!=0; folio_release_kmap(folio,p as *mut c_void); if bad{cstr!("error reading root directory.")}else{core::ptr::null()}
}
unsafe fn qnx6_fill_super(_s:*mut super_block,_fc:*mut fs_context)->i32 { -EINVAL }
static mut QNX6_PARAM_SPEC:[fs_parameter_spec;1]=[fs_parameter_spec{_private:0}];
static mut QNX6_AOPS: address_space_operations=address_space_operations{read_folio:Some(qnx6_read_folio),readahead:Some(qnx6_readahead),bmap:Some(qnx6_bmap)};
unsafe fn qnx6_bmap(m:*mut address_space,b:sector_t)->sector_t{generic_block_bmap(m,b,qnx6_get_block)}
static mut qnx6_fs_type:file_system_type=file_system_type{name:cstr!("qnx6"),kill_sb:Some(kill_block_super),fs_flags:FS_REQUIRES_DEV,init_fs_context:Some(qnx6_init_fs_context)};
unsafe fn qnx6_init_fs_context(fc:*mut fs_context)->i32{let p=kzalloc(core::mem::size_of::<qnx6_context>(),GFP_KERNEL) as *mut qnx6_context;if p.is_null(){return -ENOMEM};(*fc).fs_private=p;0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
