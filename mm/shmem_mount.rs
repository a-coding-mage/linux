// SPDX-License-Identifier: GPL-2.0-only
macro_rules! invalfc {
    ($fc:expr, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        logfc(rust_shmem_fc_log($fc), rust_shmem_fc_prefix($fc), b'e' as c_char, $fmt $(, $arg)*);
        -(RUST_SHMEM_EINVAL as c_int)
    }};
}
#[export_name = "rust_shmem_owner_shmem_get_parent"]
unsafe extern "C" fn shmem_get_parent(_child: *mut dentry) -> *mut dentry {
    err_ptr(-(RUST_SHMEM_ESTALE as c_int))
}
#[export_name = "rust_shmem_owner_shmem_match"]
unsafe extern "C" fn shmem_match(inode: *mut inode, vfh: *mut c_void) -> c_int {
    let fh = vfh.cast::<u32>();
    let ino = ((*fh.add(2) as u64) << 32) | *fh.add(1) as u64;
    ((*inode).i_ino as u64 == ino && *fh == (*inode).i_generation) as c_int
}
unsafe fn shmem_find_alias(inode: *mut inode) -> *mut dentry {
    let alias = d_find_alias(inode);
    if alias.is_null() {
        d_find_any_alias(inode)
    } else {
        alias
    }
}
#[export_name = "rust_shmem_owner_shmem_fh_to_dentry"]
unsafe extern "C" fn shmem_fh_to_dentry(
    sb: *mut super_block,
    fid: *mut fid,
    len: c_int,
    _ty: c_int,
) -> *mut dentry {
    if len < 3 {
        return null_mut();
    }
    let raw = rust_shmem_fid_raw(fid);
    let ino = ((*raw.add(2) as u64) << 32) | *raw.add(1) as u64;
    let inode = ilookup5(
        sb,
        ino.wrapping_add(*raw as u64) as c_ulong,
        Some(shmem_match),
        raw.cast(),
    );
    if inode.is_null() {
        null_mut()
    } else {
        let alias = shmem_find_alias(inode);
        iput(inode);
        alias
    }
}
#[export_name = "rust_shmem_owner_shmem_encode_fh"]
unsafe extern "C" fn shmem_encode_fh(
    inode: *mut inode,
    fh: *mut u32,
    len: *mut c_int,
    _parent: *mut inode,
) -> c_int {
    if *len < 3 {
        *len = 3;
        return FILEID_INVALID as c_int;
    }
    if rust_shmem_inode_unhashed(inode) {
        spin_lock(addr_of_mut!(rust_shmem_encode_lock));
        if rust_shmem_inode_unhashed(inode) {
            __insert_inode_hash(
                inode,
                ((*inode).i_ino as c_ulong).wrapping_add((*inode).i_generation as c_ulong),
            );
        }
        spin_unlock(addr_of_mut!(rust_shmem_encode_lock));
    }
    *fh = (*inode).i_generation;
    *fh.add(1) = (*inode).i_ino as u32;
    *fh.add(2) = ((*inode).i_ino as u64 >> 32) as u32;
    *len = 3;
    1
}
unsafe fn shmem_parse_opt_casefold(
    fc: *mut fs_context,
    param: *mut fs_parameter,
    latest: bool,
) -> c_int {
    #[cfg(CONFIG_UNICODE)]
    {
        let ctx = (*fc).fs_private as *mut shmem_options;
        let mut version = RUST_SHMEM_UTF8_LATEST as c_int;
        let text = rust_shmem_param_string(param);
        if !latest {
            if strncmp(text, c"utf8-".as_char_ptr(), 5) != 0 {
                return invalfc!(
                    fc,
                    c"Only UTF-8 encodings are supported in the format: utf8-<version number>"
                        .as_char_ptr()
                );
            }
            version = utf8_parse_version(text.add(5));
            if version < 0 {
                return invalfc!(fc, c"Invalid UTF-8 version: %s".as_char_ptr(), text.add(5));
            }
        }
        let encoding = utf8_load(version as _);
        if IS_ERR(encoding.cast()) {
            return invalfc!(
                fc,
                c"Failed loading UTF-8 version: utf8-%u.%u.%u\n".as_char_ptr(),
                rust_shmem_unicode_major(version),
                rust_shmem_unicode_minor(version),
                rust_shmem_unicode_rev(version)
            );
        }
        rust_shmem_encoding_info(version);
        (*ctx).encoding = encoding;
        0
    }
    #[cfg(not(CONFIG_UNICODE))]
    {
        let _ = (param, latest);
        invalfc!(
            fc,
            c"tmpfs: Kernel not built with CONFIG_UNICODE\n".as_char_ptr()
        )
    }
}
#[export_name = "rust_shmem_owner_shmem_parse_one"]
unsafe extern "C" fn shmem_parse_one(fc: *mut fs_context, param: *mut fs_parameter) -> c_int {
    let ctx = (*fc).fs_private as *mut shmem_options;
    let mut result = MaybeUninit::<fs_parse_result>::uninit();
    let opt = fs_parse(fc, shmem_fs_parameters.as_ptr(), param, result.as_mut_ptr());
    if opt < 0 {
        return opt;
    }
    let result = result.as_ptr();
    let mut rest = null_mut();
    let mut unsupported = false;
    'value: {
        match opt as shmem_param {
            Opt_size => {
                let mut size = memparse(rust_shmem_param_string(param), &mut rest);
                if *rest == b'%' as c_char {
                    size =
                        (size << RUST_SHMEM_PAGE_SHIFT).wrapping_mul(totalram_pages() as u64) / 100;
                    rest = rest.add(1);
                }
                if *rest != 0 {
                    break 'value;
                }
                (*ctx).blocks = size.wrapping_add(RUST_SHMEM_PAGE_SIZE as u64 - 1)
                    / RUST_SHMEM_PAGE_SIZE as u64;
                (*ctx).seen |= RUST_SHMEM_SHMEM_SEEN_BLOCKS;
            }
            Opt_nr_blocks => {
                (*ctx).blocks = memparse(rust_shmem_param_string(param), &mut rest);
                if *rest != 0 || (*ctx).blocks > c_long::MAX as u64 {
                    break 'value;
                }
                (*ctx).seen |= RUST_SHMEM_SHMEM_SEEN_BLOCKS;
            }
            Opt_nr_inodes => {
                (*ctx).inodes = memparse(rust_shmem_param_string(param), &mut rest);
                if *rest != 0
                    || (*ctx).inodes > (c_ulong::MAX / RUST_SHMEM_BOGO_INODE_SIZE as c_ulong) as u64
                {
                    break 'value;
                }
                (*ctx).seen |= RUST_SHMEM_SHMEM_SEEN_INODES;
            }
            Opt_mode => {
                (*ctx).mode = (rust_shmem_result_u32(result) & 0o7777) as umode_t;
            }
            Opt_uid => {
                let uid = rust_shmem_result_uid(result);
                if !kuid_has_mapping((*fc).user_ns, uid) {
                    break 'value;
                }
                (*ctx).uid = uid;
            }
            Opt_gid => {
                let gid = rust_shmem_result_gid(result);
                if !kgid_has_mapping((*fc).user_ns, gid) {
                    break 'value;
                }
                (*ctx).gid = gid;
            }
            Opt_huge => {
                (*ctx).huge = rust_shmem_result_u32(result) as c_int;
                if (*ctx).huge != RUST_SHMEM_SHMEM_HUGE_NEVER
                    && !(cfg!(CONFIG_TRANSPARENT_HUGEPAGE) && rust_shmem_has_transparent_hugepage())
                {
                    unsupported = true;
                    break 'value;
                }
                (*ctx).seen |= RUST_SHMEM_SHMEM_SEEN_HUGE;
            }
            Opt_mpol => {
                #[cfg(CONFIG_NUMA)]
                {
                    mpol_put((*ctx).mpol);
                    (*ctx).mpol = null_mut();
                    if mpol_parse_str(rust_shmem_param_string(param), addr_of_mut!((*ctx).mpol))
                        != 0
                    {
                        break 'value;
                    }
                }
                #[cfg(not(CONFIG_NUMA))]
                {
                    unsupported = true;
                    break 'value;
                }
            }
            Opt_inode32 => {
                (*ctx).full_inums = false;
                (*ctx).seen |= RUST_SHMEM_SHMEM_SEEN_INUMS;
            }
            Opt_inode64 => {
                if size_of::<ino_t>() < 8 {
                    return invalfc!(
                        fc,
                        c"Cannot use inode64 with <64bit inums in kernel\n".as_char_ptr()
                    );
                }
                (*ctx).full_inums = true;
                (*ctx).seen |= RUST_SHMEM_SHMEM_SEEN_INUMS;
            }
            Opt_noswap => {
                if (*fc).user_ns != addr_of_mut!(init_user_ns)
                    || !capable(RUST_SHMEM_CAP_SYS_ADMIN as c_int)
                {
                    return invalfc!(
                        fc,
                        c"Turning off swap in unprivileged tmpfs mounts unsupported".as_char_ptr()
                    );
                }
                (*ctx).noswap = true;
            }
            Opt_quota | Opt_usrquota | Opt_grpquota => {
                if (*fc).user_ns != addr_of_mut!(init_user_ns) {
                    return invalfc!(
                        fc,
                        c"Quotas in unprivileged tmpfs mounts are unsupported".as_char_ptr()
                    );
                }
                (*ctx).seen |= RUST_SHMEM_SHMEM_SEEN_QUOTA;
                (*ctx).quota_types |= match opt as shmem_param {
                    Opt_usrquota => RUST_SHMEM_QTYPE_MASK_USR as u16,
                    Opt_grpquota => RUST_SHMEM_QTYPE_MASK_GRP as u16,
                    _ => (RUST_SHMEM_QTYPE_MASK_USR | RUST_SHMEM_QTYPE_MASK_GRP) as u16,
                };
            }
            Opt_usrquota_block_hardlimit
            | Opt_grpquota_block_hardlimit
            | Opt_usrquota_inode_hardlimit
            | Opt_grpquota_inode_hardlimit => {
                let size = memparse(rust_shmem_param_string(param), &mut rest);
                if *rest != 0 || size == 0 {
                    break 'value;
                }
                let limit = if opt as shmem_param == Opt_usrquota_block_hardlimit
                    || opt as shmem_param == Opt_grpquota_block_hardlimit
                {
                    RUST_SHMEM_SHMEM_QUOTA_MAX_SPC_LIMIT as u64
                } else {
                    RUST_SHMEM_SHMEM_QUOTA_MAX_INO_LIMIT as u64
                };
                if size > limit {
                    let msg = match opt as shmem_param {
                        Opt_usrquota_block_hardlimit => c"User quota block hardlimit too large.",
                        Opt_grpquota_block_hardlimit => c"Group quota block hardlimit too large.",
                        Opt_usrquota_inode_hardlimit => c"User quota inode hardlimit too large.",
                        _ => c"Group quota inode hardlimit too large.",
                    };
                    return invalfc!(fc, msg.as_char_ptr());
                }
                match opt as shmem_param {
                    Opt_usrquota_block_hardlimit => (*ctx).qlimits.usrquota_bhardlimit = size as _,
                    Opt_grpquota_block_hardlimit => (*ctx).qlimits.grpquota_bhardlimit = size as _,
                    Opt_usrquota_inode_hardlimit => (*ctx).qlimits.usrquota_ihardlimit = size as _,
                    _ => (*ctx).qlimits.grpquota_ihardlimit = size as _,
                }
            }
            Opt_casefold_version => return shmem_parse_opt_casefold(fc, param, false),
            Opt_casefold => return shmem_parse_opt_casefold(fc, param, true),
            Opt_strict_encoding => {
                #[cfg(CONFIG_UNICODE)]
                {
                    (*ctx).strict_encoding = true;
                }
                #[cfg(not(CONFIG_UNICODE))]
                {
                    return invalfc!(
                        fc,
                        c"tmpfs: Kernel not built with CONFIG_UNICODE\n".as_char_ptr()
                    );
                }
            }
            _ => {}
        }
        return 0;
    }
    if unsupported {
        invalfc!(
            fc,
            c"Unsupported parameter '%s'".as_char_ptr(),
            (*param).key
        )
    } else {
        invalfc!(fc, c"Bad value for '%s'".as_char_ptr(), (*param).key)
    }
}
#[export_name = "rust_shmem_owner_shmem_next_opt"]
unsafe extern "C" fn shmem_next_opt(s: *mut *mut c_char) -> *mut c_char {
    let begin = *s;
    if begin.is_null() {
        return null_mut();
    }
    loop {
        let p = strchr(*s, b',' as c_int);
        if p.is_null() {
            break;
        }
        *s = p.add(1);
        if !rust_shmem_isdigit(*p.add(1) as c_int) {
            *p = 0;
            return begin;
        }
    }
    *s = null_mut();
    begin
}
#[export_name = "rust_shmem_owner_shmem_parse_monolithic"]
unsafe extern "C" fn shmem_parse_monolithic(fc: *mut fs_context, data: *mut c_void) -> c_int {
    vfs_parse_monolithic_sep(fc, data, Some(shmem_next_opt))
}
#[export_name = "rust_shmem_owner_shmem_reconfigure"]
unsafe extern "C" fn shmem_reconfigure(fc: *mut fs_context) -> c_int {
    let ctx = (*fc).fs_private as *mut shmem_options;
    let sb = SHMEM_SB((*(*fc).root).d_sb);
    let mut old_policy = null_mut();
    let mut message = null();
    rust_shmem_raw_spin_lock(addr_of_mut!((*sb).stat_lock));
    let used = (*sb)
        .max_inodes
        .wrapping_mul(RUST_SHMEM_BOGO_INODE_SIZE as c_ulong)
        .wrapping_sub((*sb).free_ispace);
    'validate: {
        if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_BLOCKS != 0 && (*ctx).blocks != 0 {
            if (*sb).max_blocks == 0 {
                message = c"Cannot retroactively limit size".as_char_ptr();
                break 'validate;
            }
            if percpu_counter_compare(addr_of_mut!((*sb).used_blocks), (*ctx).blocks as _) > 0 {
                message = c"Too small a size for current use".as_char_ptr();
                break 'validate;
            }
        }
        if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_INODES != 0 && (*ctx).inodes != 0 {
            if (*sb).max_inodes == 0 {
                message = c"Cannot retroactively limit inodes".as_char_ptr();
                break 'validate;
            }
            if (*ctx)
                .inodes
                .wrapping_mul(RUST_SHMEM_BOGO_INODE_SIZE as u64)
                < used as u64
            {
                message = c"Too few inodes for current use".as_char_ptr();
                break 'validate;
            }
        }
        if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_INUMS != 0
            && !(*ctx).full_inums
            && (*sb).next_ino > c_uint::MAX as ino_t
        {
            message = c"Current inum too high to switch to 32-bit inums".as_char_ptr();
            break 'validate;
        }
        if (*ctx).noswap && !(*sb).noswap {
            message = c"Cannot disable swap on remount".as_char_ptr();
            break 'validate;
        }
        if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_QUOTA != 0
            && !rust_shmem_sb_any_quota_loaded((*(*fc).root).d_sb)
        {
            message = c"Cannot enable quota on remount".as_char_ptr();
            break 'validate;
        }
        #[cfg(CONFIG_TMPFS_QUOTA)]
        {
            let a = addr_of!((*ctx).qlimits);
            let b = addr_of!((*sb).qlimits);
            if ((*a).usrquota_bhardlimit != 0
                && (*a).usrquota_bhardlimit != (*b).usrquota_bhardlimit)
                || ((*a).usrquota_ihardlimit != 0
                    && (*a).usrquota_ihardlimit != (*b).usrquota_ihardlimit)
                || ((*a).grpquota_bhardlimit != 0
                    && (*a).grpquota_bhardlimit != (*b).grpquota_bhardlimit)
                || ((*a).grpquota_ihardlimit != 0
                    && (*a).grpquota_ihardlimit != (*b).grpquota_ihardlimit)
            {
                message = c"Cannot change global quota limit on remount".as_char_ptr();
                break 'validate;
            }
        }
        if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_HUGE != 0 {
            (*sb).huge = (*ctx).huge as u8;
        }
        if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_INUMS != 0 {
            (*sb).full_inums = (*ctx).full_inums;
        }
        if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_BLOCKS != 0 {
            (*sb).max_blocks = (*ctx).blocks as c_ulong;
        }
        if (*ctx).seen & RUST_SHMEM_SHMEM_SEEN_INODES != 0 {
            (*sb).max_inodes = (*ctx).inodes as c_ulong;
            (*sb).free_ispace = (*ctx)
                .inodes
                .wrapping_mul(RUST_SHMEM_BOGO_INODE_SIZE as u64)
                .wrapping_sub(used as u64) as c_ulong;
        }
        if !(*ctx).mpol.is_null() {
            old_policy = (*sb).mpol;
            (*sb).mpol = (*ctx).mpol;
            (*ctx).mpol = null_mut();
        }
        if (*ctx).noswap {
            (*sb).noswap = true;
        }
    }
    rust_shmem_raw_spin_unlock(addr_of_mut!((*sb).stat_lock));
    if !message.is_null() {
        return invalfc!(fc, c"%s".as_char_ptr(), message);
    }
    mpol_put(old_policy);
    0
}
#[export_name = "rust_shmem_owner_shmem_show_options"]
unsafe extern "C" fn shmem_show_options(seq: *mut seq_file, root: *mut dentry) -> c_int {
    let sb = SHMEM_SB((*root).d_sb);
    if (*sb).max_blocks != shmem_default_max_blocks() {
        seq_printf(
            seq,
            c",size=%luk".as_char_ptr(),
            rust_shmem_blocks_k((*sb).max_blocks),
        );
    }
    if (*sb).max_inodes != shmem_default_max_inodes() {
        seq_printf(seq, c",nr_inodes=%lu".as_char_ptr(), (*sb).max_inodes);
    }
    if (*sb).mode != (0o777 | RUST_SHMEM_S_ISVTX as umode_t) {
        seq_printf(seq, c",mode=%03ho".as_char_ptr(), (*sb).mode as c_int);
    }
    if !uid_eq((*sb).uid, rust_shmem_root_uid()) {
        seq_printf(
            seq,
            c",uid=%u".as_char_ptr(),
            from_kuid_munged(addr_of_mut!(init_user_ns), (*sb).uid),
        );
    }
    if !gid_eq((*sb).gid, rust_shmem_root_gid()) {
        seq_printf(
            seq,
            c",gid=%u".as_char_ptr(),
            from_kgid_munged(addr_of_mut!(init_user_ns), (*sb).gid),
        );
    }
    if cfg!(CONFIG_TMPFS_INODE64) || (*sb).full_inums {
        seq_printf(
            seq,
            c",inode%d".as_char_ptr(),
            if (*sb).full_inums {
                64 as c_int
            } else {
                32 as c_int
            },
        );
    }
    #[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
    if (*sb).huge != 0 {
        seq_printf(
            seq,
            c",huge=%s".as_char_ptr(),
            shmem_format_huge((*sb).huge as c_int),
        );
    }
    let p = shmem_get_sbmpol(sb);
    shmem_show_mpol(seq, p);
    mpol_put(p);
    if (*sb).noswap {
        seq_printf(seq, c",noswap".as_char_ptr());
    }
    #[cfg(CONFIG_TMPFS_QUOTA)]
    {
        if rust_shmem_sb_has_quota_active((*root).d_sb, RUST_SHMEM_USRQUOTA as c_int) {
            seq_printf(seq, c",usrquota".as_char_ptr());
        }
        if rust_shmem_sb_has_quota_active((*root).d_sb, RUST_SHMEM_GRPQUOTA as c_int) {
            seq_printf(seq, c",grpquota".as_char_ptr());
        }
        for (fmt, value) in [
            (
                c",usrquota_block_hardlimit=%lld",
                (*sb).qlimits.usrquota_bhardlimit,
            ),
            (
                c",grpquota_block_hardlimit=%lld",
                (*sb).qlimits.grpquota_bhardlimit,
            ),
            (
                c",usrquota_inode_hardlimit=%lld",
                (*sb).qlimits.usrquota_ihardlimit,
            ),
            (
                c",grpquota_inode_hardlimit=%lld",
                (*sb).qlimits.grpquota_ihardlimit,
            ),
        ] {
            if value != 0 {
                seq_printf(seq, fmt.as_char_ptr(), value);
            }
        }
    }
    0
}
