// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of linux/fs/adfs/super.c. Kernel dependencies are
 * supplied by the surrounding translation unit. */

const ADFS_SB_FLAGS: u32 = SB_NOATIME;
const ADFS_DEFAULT_OWNER_MASK: u32 = S_IRWXU;
const ADFS_DEFAULT_OTHER_MASK: u32 = S_IRWXG | S_IRWXO;

pub unsafe fn __adfs_error(sb: *mut super_block, function: *const c_char,
                           fmt: *const c_char, mut args: ...) {
    let mut vaf: va_format = core::mem::zeroed();
    let mut ap: va_list = args;
    vaf.fmt = fmt;
    vaf.va = &mut ap;
    printk(KERN_CRIT.as_ptr(), (*sb).s_id.as_ptr(),
           if !function.is_null() { ": " } else { "" },
           if !function.is_null() { function } else { "" }, &vaf);
}

pub unsafe fn adfs_msg(sb: *mut super_block, pfx: *const c_char,
                       fmt: *const c_char, mut args: ...) {
    let mut vaf: va_format = core::mem::zeroed();
    let mut ap: va_list = args;
    vaf.fmt = fmt;
    vaf.va = &mut ap;
    printk("%sADFS-fs (%s): %pV\n", pfx, (*sb).s_id.as_ptr(), &vaf);
}

unsafe fn adfs_checkdiscrecord(dr: *mut adfs_discrecord) -> c_int {
    let max_idlen: u32 = if (*dr).format_version != 0 { 19 } else { 16 };
    if (*dr).log2secsize != 8 && (*dr).log2secsize != 9 && (*dr).log2secsize != 10 { return 1; }
    if (*dr).idlen < (*dr).log2secsize + 3 { return 1; }
    if (le32_to_cpu((*dr).disc_size_high) >> (*dr).log2secsize) != 0 { return 1; }
    if (*dr).idlen > max_idlen { return 1; }
    for i in 0..core::mem::size_of_val(&(*dr).unused52) {
        if (*dr).unused52[i] != 0 { return 1; }
    }
    0
}

unsafe fn adfs_put_super(sb: *mut super_block) {
    let asb = ADFS_SB(sb);
    adfs_free_map(sb);
    kfree_rcu(asb, rcu);
}

unsafe fn adfs_show_options(seq: *mut seq_file, root: *mut dentry) -> c_int {
    let asb = ADFS_SB((*root).d_sb);
    if !uid_eq((*asb).s_uid, GLOBAL_ROOT_UID) { seq_printf(seq, ",uid=%u", from_kuid_munged(&init_user_ns, (*asb).s_uid)); }
    if !gid_eq((*asb).s_gid, GLOBAL_ROOT_GID) { seq_printf(seq, ",gid=%u", from_kgid_munged(&init_user_ns, (*asb).s_gid)); }
    if (*asb).s_owner_mask != ADFS_DEFAULT_OWNER_MASK { seq_printf(seq, ",ownmask=%o", (*asb).s_owner_mask); }
    if (*asb).s_other_mask != ADFS_DEFAULT_OTHER_MASK { seq_printf(seq, ",othmask=%o", (*asb).s_other_mask); }
    if (*asb).s_ftsuffix != 0 { seq_printf(seq, ",ftsuffix=%u", (*asb).s_ftsuffix); }
    0
}

enum { Opt_uid, Opt_gid, Opt_ownmask, Opt_othmask, Opt_ftsuffix }
static adfs_param_spec: [fs_parameter_spec; 6] = [
    fsparam_uid!("uid", Opt_uid), fsparam_gid!("gid", Opt_gid),
    fsparam_u32oct!("ownmask", Opt_ownmask), fsparam_u32oct!("othmask", Opt_othmask),
    fsparam_u32!("ftsuffix", Opt_ftsuffix), fsparam_empty!(),
];

unsafe fn adfs_parse_param(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let asb = (*fc).s_fs_info as *mut adfs_sb_info;
    let mut result: fs_parse_result = core::mem::zeroed();
    let opt = fs_parse(fc, adfs_param_spec.as_ptr(), param, &mut result);
    if opt < 0 { return opt; }
    match opt { Opt_uid => (*asb).s_uid = result.uid, Opt_gid => (*asb).s_gid = result.gid,
        Opt_ownmask => (*asb).s_owner_mask = result.uint_32, Opt_othmask => (*asb).s_other_mask = result.uint_32,
        Opt_ftsuffix => (*asb).s_ftsuffix = result.uint_32, _ => return -EINVAL }
    0
}

unsafe fn adfs_reconfigure(fc: *mut fs_context) -> c_int {
    let new_asb = (*fc).s_fs_info as *mut adfs_sb_info;
    let asb = ADFS_SB((*(*fc).root).d_sb);
    sync_filesystem((*fc).root.d_sb); (*fc).sb_flags |= ADFS_SB_FLAGS;
    *asb = *new_asb; 0
}

unsafe fn adfs_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> c_int {
    let sb = (*dentry).d_sb; let sbi = ADFS_SB(sb);
    let id = huge_encode_dev((*sb).s_bdev.bd_dev); adfs_map_statfs(sb, buf);
    (*buf).f_type = ADFS_SUPER_MAGIC; (*buf).f_namelen = (*sbi).s_namelen;
    (*buf).f_bsize = (*sb).s_blocksize;
    (*buf).f_ffree = ((*buf).f_bfree * (*buf).f_files) as c_long / (*buf).f_blocks as c_long;
    (*buf).f_fsid = u64_to_fsid(id); 0
}

static mut adfs_inode_cachep: *mut kmem_cache = core::ptr::null_mut();
unsafe fn adfs_alloc_inode(sb: *mut super_block) -> *mut inode { let ei = alloc_inode_sb(sb, adfs_inode_cachep, GFP_KERNEL); if ei.is_null() { core::ptr::null_mut() } else { &mut (*ei).vfs_inode } }
unsafe fn adfs_free_inode(inode: *mut inode) { kmem_cache_free(adfs_inode_cachep, ADFS_I(inode)); }
unsafe fn adfs_drop_inode(inode: *mut inode) -> c_int { (!IS_ENABLED!(CONFIG_ADFS_FS_RW) || IS_RDONLY(inode)) as c_int }
unsafe fn init_once(foo: *mut c_void) { inode_init_once(&mut (*(foo as *mut adfs_inode_info)).vfs_inode); }

unsafe fn init_inodecache() -> c_int {
    adfs_inode_cachep = kmem_cache_create("adfs_inode_cache", core::mem::size_of::<adfs_inode_info>(), 0, SLAB_RECLAIM_ACCOUNT | SLAB_ACCOUNT, init_once);
    if adfs_inode_cachep.is_null() { -ENOMEM } else { 0 }
}
unsafe fn destroy_inodecache() { rcu_barrier(); kmem_cache_destroy(adfs_inode_cachep); }

static adfs_sops: super_operations = super_operations { alloc_inode: adfs_alloc_inode, free_inode: adfs_free_inode, drop_inode: adfs_drop_inode, write_inode: adfs_write_inode, put_super: adfs_put_super, statfs: adfs_statfs, show_options: adfs_show_options };

unsafe fn adfs_probe(sb: *mut super_block, offset: c_uint, silent: c_int,
    validate: unsafe extern "C" fn(*mut super_block, *mut buffer_head, *mut *mut adfs_discrecord) -> c_int) -> c_int {
    let asb = ADFS_SB(sb); let mut blocksize = BLOCK_SIZE; let mut dr: *mut adfs_discrecord = core::ptr::null_mut();
    for tr in 0..2 { if (*sb).s_blocksize != blocksize && sb_set_blocksize(sb, blocksize) == 0 { if silent == 0 { adfs_msg(sb, KERN_ERR, "error: unsupported blocksize"); } return -EINVAL; }
        let bh = sb_bread(sb, offset >> (*sb).s_blocksize_bits); if bh.is_null() { adfs_msg(sb, KERN_ERR, "error: unable to read block %u, try %d", offset >> (*sb).s_blocksize_bits, tr); return -EIO; }
        let ret = validate(sb, bh, &mut dr); if ret != 0 { brelse(bh); return ret; }
        blocksize = 1 << (*dr).log2secsize; if (*sb).s_blocksize == blocksize { (*asb).s_map = adfs_read_map(sb, dr); brelse(bh); return PTR_ERR_OR_ZERO((*asb).s_map); } brelse(bh); }
    -EIO
}

unsafe fn adfs_validate_bblk(sb: *mut super_block, bh: *mut buffer_head, drp: *mut *mut adfs_discrecord) -> c_int {
    let data = (*bh).b_data.add(ADFS_DISCRECORD % (*sb).s_blocksize as usize); if adfs_checkbblk(data) != 0 { return -EILSEQ; }
    let dr = data.add(ADFS_DR_OFFSET) as *mut adfs_discrecord; if adfs_checkdiscrecord(dr) != 0 || (((*dr).nzones as u16) | ((*dr).nzones_high as u16) << 8) == 0 { return -EILSEQ; } *drp = dr; 0
}
unsafe fn adfs_validate_dr0(_sb: *mut super_block, bh: *mut buffer_head, drp: *mut *mut adfs_discrecord) -> c_int { let dr = (*bh).b_data.add(4) as *mut adfs_discrecord; if adfs_checkdiscrecord(dr) != 0 || (*dr).nzones_high != 0 || (*dr).nzones != 1 { return -EILSEQ; } *drp = dr; 0 }

// Remaining filesystem registration and fill-super logic retain the kernel callback topology.
// External kernel structures and helpers are intentionally referenced, not reimplemented.
unsafe fn adfs_get_tree(fc: *mut fs_context) -> c_int { get_tree_bdev(fc, adfs_fill_super) }
unsafe fn adfs_free_fc(fc: *mut fs_context) { kfree((*fc).s_fs_info); }
unsafe fn adfs_init_fs_context(fc: *mut fs_context) -> c_int { let asb = kzalloc_obj::<adfs_sb_info>(); if asb.is_null() { return -ENOMEM; } (*fc).s_fs_info = asb as *mut c_void; 0 }

unsafe fn adfs_fill_super(sb: *mut super_block, fc: *mut fs_context) -> c_int {
    let asb = (*sb).s_fs_info as *mut adfs_sb_info; let mut dr: *mut adfs_discrecord = core::ptr::null_mut();
    let mut root_obj: object_info = core::mem::zeroed(); let mut ret = -EINVAL;
    let silent = ((*fc).sb_flags & SB_SILENT) != 0; (*sb).s_flags |= ADFS_SB_FLAGS;
    (*sb).s_magic = ADFS_SUPER_MAGIC; (*sb).s_time_gran = 10000000;
    ret = adfs_probe(sb, ADFS_DISCRECORD, 1, adfs_validate_bblk);
    if ret == -EILSEQ { ret = adfs_probe(sb, 0, silent as c_int, adfs_validate_dr0); }
    if ret == -EILSEQ { if !silent { adfs_msg(sb, KERN_ERR, "error: can't find an ADFS filesystem on dev %s.", (*sb).s_id.as_ptr()); } ret = -EINVAL; }
    if ret != 0 { (*sb).s_fs_info = core::ptr::null_mut(); kfree(asb); return ret; }
    (*sb).s_op = &adfs_sops; dr = adfs_map_discrecord((*asb).s_map);
    root_obj.parent_id = le32_to_cpu((*dr).root); root_obj.indaddr = root_obj.parent_id; root_obj.name_len = 0;
    root_obj.loadaddr = 0xfff0003f; root_obj.execaddr = 0xec22c000; root_obj.size = ADFS_NEWDIR_SIZE;
    root_obj.attr = ADFS_NDA_DIRECTORY | ADFS_NDA_OWNER_READ | ADFS_NDA_OWNER_WRITE | ADFS_NDA_PUBLIC_READ;
    if (*dr).format_version != 0 { root_obj.size = le32_to_cpu((*dr).root_size); (*asb).s_dir = &adfs_fplus_dir_ops; (*asb).s_namelen = ADFS_FPLUS_NAME_LEN; }
    else { (*asb).s_dir = &adfs_f_dir_ops; (*asb).s_namelen = ADFS_F_NAME_LEN; }
    if (*asb).s_ftsuffix != 0 { (*asb).s_namelen += 4; }
    set_default_d_op(sb, &adfs_dentry_operations); let root = adfs_iget(sb, &root_obj); (*sb).s_root = d_make_root(root);
    if (*sb).s_root.is_null() { adfs_free_map(sb); adfs_error(sb, "get root inode failed\n"); (*sb).s_fs_info = core::ptr::null_mut(); kfree(asb); return -EIO; } 0
}

static adfs_context_ops: fs_context_operations = fs_context_operations { parse_param: adfs_parse_param, get_tree: adfs_get_tree, reconfigure: adfs_reconfigure, free: adfs_free_fc };
static mut adfs_fs_type: file_system_type = file_system_type { owner: THIS_MODULE, name: "adfs", kill_sb: kill_block_super, fs_flags: FS_REQUIRES_DEV, init_fs_context: adfs_init_fs_context, parameters: adfs_param_spec.as_ptr() };
unsafe fn init_adfs_fs() -> c_int { let err = init_inodecache(); if err != 0 { return err; } let err = register_filesystem(&mut adfs_fs_type); if err != 0 { destroy_inodecache(); } err }
unsafe fn exit_adfs_fs() { unregister_filesystem(&mut adfs_fs_type); destroy_inodecache(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
