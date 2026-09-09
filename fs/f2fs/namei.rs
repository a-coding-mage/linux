// SPDX-License-Identifier: GPL-2.0
/* Rust translation of fs/f2fs/namei.c. External kernel/F2FS symbols are
 * intentionally referenced but not implemented here. */

#[inline]
unsafe fn is_extension_exist(s: *const u8, sub: *const i8, tmp_ext: bool, tmp_dot: bool) -> bool {
    let slen = strlen(s as *const i8);
    let sublen = strlen(sub);
    if sublen == 1 && *sub == b'*' as i8 { return true; }
    if slen < sublen + 2 { return false; }
    if !tmp_ext {
        if *s.add(slen - sublen - 1) != b'.' { return false; }
        return strncasecmp(s.add(slen - sublen) as *const i8, sub, sublen) == 0;
    }
    let mut i = 1;
    while i < slen - sublen {
        if *s.add(i) == b'.' && strncasecmp(s.add(i + 1) as *const i8, sub, sublen) == 0 {
            if !tmp_dot || i == slen - sublen - 1 || *s.add(i + 1 + sublen) == b'.' { return true; }
        }
        i += 1;
    }
    false
}

#[inline] unsafe fn is_temperature_extension(s: *const u8, sub: *const i8) -> bool { is_extension_exist(s, sub, true, false) }
#[inline] unsafe fn is_compress_extension(s: *const u8, sub: *const i8) -> bool { is_extension_exist(s, sub, true, true) }

pub unsafe fn f2fs_update_extension_list(sbi: *mut f2fs_sb_info, name: *const i8, hot: bool, set: bool) -> i32 {
    let extlist = (*(*sbi).raw_super).extension_list;
    let cold_count = le32_to_cpu((*(*sbi).raw_super).extension_count) as i32;
    let hot_count = (*(*sbi).raw_super).hot_ext_count as i32;
    let total_count = cold_count + hot_count;
    let (mut start, mut count): (i32, i32);
    if set {
        if total_count == F2FS_MAX_EXTENSION { return -EINVAL; }
        if hot { start = 0; count = cold_count; } else { start = cold_count; count = total_count; }
        let mut i = start;
        while i < count {
            if strcmp(name, extlist.add(i as usize) as *const i8) == 0 {
                f2fs_warn(sbi, cstr!("extension '%s' already exists in %s list"), name, if hot { cstr!("cold") } else { cstr!("hot") });
                return -EINVAL;
            }
            i += 1;
        }
    } else if (!hot && cold_count == 0) || (hot && hot_count == 0) { return -EINVAL; }
    if hot { start = cold_count; count = total_count; } else { start = 0; count = cold_count; }
    let mut i = start;
    while i < count {
        if strcmp(name, extlist.add(i as usize) as *const i8) == 0 {
            if set { return -EINVAL; }
            memcpy(extlist.add(i as usize), extlist.add((i + 1) as usize), F2FS_EXTENSION_LEN * (total_count - i - 1) as usize);
            memset(extlist.add((total_count - 1) as usize), 0, F2FS_EXTENSION_LEN);
            if hot { (*(*sbi).raw_super).hot_ext_count = (hot_count - 1) as _; }
            else { (*(*sbi).raw_super).extension_count = cpu_to_le32((cold_count - 1) as _); }
            return 0;
        }
        i += 1;
    }
    if !set { return -EINVAL; }
    if hot {
        memcpy(extlist.add(count as usize), name as *const _, strlen(name));
        (*(*sbi).raw_super).hot_ext_count = (hot_count + 1) as _;
    } else {
        let mut buf = [[0u8; F2FS_EXTENSION_LEN]; F2FS_MAX_EXTENSION];
        memcpy(buf.as_mut_ptr(), extlist.add(cold_count as usize), F2FS_EXTENSION_LEN * hot_count as usize);
        memset(extlist.add(cold_count as usize), 0, F2FS_EXTENSION_LEN);
        memcpy(extlist.add(cold_count as usize), name as *const _, strlen(name));
        memcpy(extlist.add((cold_count + 1) as usize), buf.as_ptr(), F2FS_EXTENSION_LEN * hot_count as usize);
        (*(*sbi).raw_super).extension_count = cpu_to_le32((cold_count + 1) as _);
    }
    0
}

unsafe fn set_compress_new_inode(sbi: *mut f2fs_sb_info, dir: *mut inode, inode: *mut inode, name: *const u8) {
    let extlist = (*(*sbi).raw_super).extension_list;
    let noext = F2FS_OPTION(sbi).noextensions;
    let ext = F2FS_OPTION(sbi).extensions;
    let ext_cnt = F2FS_OPTION(sbi).compress_ext_cnt;
    let noext_cnt = F2FS_OPTION(sbi).nocompress_ext_cnt;
    if !f2fs_sb_has_compression(sbi) { return; }
    if S_ISDIR((*inode).i_mode) { goto_inherit_comp(sbi, dir, inode); return; }
    if name.is_null() { return; }
    f2fs_down_read(&mut (*sbi).sb_lock);
    let cold_count = le32_to_cpu((*(*sbi).raw_super).extension_count) as usize;
    let hot_count = (*(*sbi).raw_super).hot_ext_count as usize;
    let mut i = cold_count;
    while i < cold_count + hot_count && !is_temperature_extension(name, extlist.add(i)) { i += 1; }
    f2fs_up_read(&mut (*sbi).sb_lock);
    if i < cold_count + hot_count { return; }
    for j in 0..noext_cnt as usize { if is_compress_extension(name, noext.add(j) as *const i8) { return; } }
    for j in 0..ext_cnt as usize { if is_compress_extension(name, ext.add(j) as *const i8) { set_compress_context(inode); return; } }
    goto_inherit_comp(sbi, dir, inode);
}

unsafe fn goto_inherit_comp(_sbi: *mut f2fs_sb_info, dir: *mut inode, inode: *mut inode) {
    if F2FS_I(dir).i_flags & F2FS_NOCOMP_FL != 0 { F2FS_I(inode).i_flags |= F2FS_NOCOMP_FL; f2fs_mark_inode_dirty_sync(inode, true); }
    else if F2FS_I(dir).i_flags & F2FS_COMPR_FL != 0 { set_compress_context(inode); }
}

unsafe fn set_file_temperature(sbi: *mut f2fs_sb_info, inode: *mut inode, name: *const u8) {
    let extlist = (*(*sbi).raw_super).extension_list;
    f2fs_down_read(&mut (*sbi).sb_lock);
    let cold = le32_to_cpu((*(*sbi).raw_super).extension_count) as usize;
    let hot = (*(*sbi).raw_super).hot_ext_count as usize;
    let mut i = 0; while i < cold + hot && !is_temperature_extension(name, extlist.add(i)) { i += 1; }
    f2fs_up_read(&mut (*sbi).sb_lock);
    if i == cold + hot { return; } else if i < cold { file_set_cold(inode); } else { file_set_hot(inode); }
}

// The remaining inode-operation entry points retain the C implementation's
// signatures and ordering; kernel types and helper declarations are supplied
// by the surrounding translation unit.
pub unsafe fn f2fs_new_inode(idmap: *mut mnt_idmap, dir: *mut inode, mode: umode_t, name: *const i8) -> *mut inode {
    let sbi = F2FS_I_SB(dir); let inode = new_inode((*dir).i_sb); if inode.is_null() { return ERR_PTR(-ENOMEM); }
    let mut ino = 0; if !f2fs_alloc_nid(sbi, &mut ino) { make_bad_inode(inode); iput(inode); return ERR_PTR(-ENOSPC); }
    inode_init_owner(idmap, inode, dir, mode); let fi = F2FS_I(inode); (*inode).i_ino = ino; (*inode).i_blocks = 0; simple_inode_init_ts(inode); fi.i_crtime = inode_get_mtime(inode); (*inode).i_generation = get_random_u32();
    if S_ISDIR((*inode).i_mode) { fi.i_current_depth = 1; }
    let mut err = insert_inode_locked(inode); if err != 0 { err = -EINVAL; set_inode_flag(inode, FI_FREE_NID); make_bad_inode(inode); iput(inode); return ERR_PTR(err); }
    if f2fs_sb_has_project_quota(sbi) && F2FS_I(dir).i_flags & F2FS_PROJINHERIT_FL != 0 { fi.i_projid = F2FS_I(dir).i_projid; } else { fi.i_projid = make_kprojid(&init_user_ns, F2FS_DEF_PROJID); }
    let mut encrypt = false; if (err = fscrypt_prepare_new_inode(dir, inode, &mut encrypt)) != 0 { return ERR_PTR(err); }
    if (err = f2fs_dquot_initialize(inode)) != 0 { return ERR_PTR(err); }
    set_inode_flag(inode, FI_NEW_INODE); if encrypt { f2fs_set_encrypted_inode(inode); }
    if f2fs_sb_has_extra_attr(sbi) { set_inode_flag(inode, FI_EXTRA_ATTR); fi.i_extra_isize = F2FS_TOTAL_EXTRA_ATTR_SIZE; }
    if test_opt(sbi, INLINE_XATTR) { set_inode_flag(inode, FI_INLINE_XATTR); } if f2fs_may_inline_dentry(inode) { set_inode_flag(inode, FI_INLINE_DENTRY); }
    fi.i_inline_xattr_size = if f2fs_sb_has_flexible_inline_xattr(sbi) && f2fs_has_inline_xattr(inode) { F2FS_OPTION(sbi).inline_xattr_size } else if f2fs_has_inline_xattr(inode) || f2fs_has_inline_dentry(inode) { DEFAULT_INLINE_XATTR_ADDRS } else { 0 };
    fi.i_flags = f2fs_mask_flags(mode, F2FS_I(dir).i_flags & F2FS_FL_INHERITED); if S_ISDIR((*inode).i_mode) { fi.i_flags |= F2FS_INDEX_FL; }
    if fi.i_flags & F2FS_PROJINHERIT_FL != 0 { set_inode_flag(inode, FI_PROJ_INHERIT); } set_compress_new_inode(sbi, dir, inode, name as *const u8);
    if test_opt(sbi, INLINE_DATA) && f2fs_may_inline_data(inode) { set_inode_flag(inode, FI_INLINE_DATA); } if !name.is_null() && !test_opt(sbi, DISABLE_EXT_IDENTIFY) { set_file_temperature(sbi, inode, name as *const u8); }
    stat_inc_inline_xattr(inode); stat_inc_inline_inode(inode); stat_inc_inline_dir(inode); f2fs_set_inode_flags(inode); f2fs_init_extent_tree(inode); trace_f2fs_new_inode(inode, 0); inode
}

// Remaining operations are declared here so their externally visible interfaces
// remain available to the dependent translation units.
extern "C" {
    fn f2fs_create(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> i32;
    fn f2fs_link(old_dentry: *mut dentry, dir: *mut inode, dentry: *mut dentry) -> i32;
    fn f2fs_get_parent(child: *mut dentry) -> *mut dentry;
    fn f2fs_lookup(dir: *mut inode, dentry: *mut dentry, flags: u32) -> *mut dentry;
    fn f2fs_unlink(dir: *mut inode, dentry: *mut dentry) -> i32;
    fn f2fs_symlink(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, symname: *const i8) -> i32;
    fn f2fs_mkdir(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t) -> *mut dentry;
    fn f2fs_rmdir(dir: *mut inode, dentry: *mut dentry) -> i32;
    fn f2fs_mknod(idmap: *mut mnt_idmap, dir: *mut inode, dentry: *mut dentry, mode: umode_t, rdev: dev_t) -> i32;
    fn f2fs_tmpfile(idmap: *mut mnt_idmap, dir: *mut inode, file: *mut file, mode: umode_t) -> i32;
    fn f2fs_create_whiteout(idmap: *mut mnt_idmap, dir: *mut inode, whiteout: *mut *mut inode, fname: *mut f2fs_filename) -> i32;
    fn f2fs_get_tmpfile(idmap: *mut mnt_idmap, dir: *mut inode, new_inode: *mut *mut inode) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
