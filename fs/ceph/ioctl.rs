// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel/Ceph translation.

/* ioctls */

/* get and set the file layout */
unsafe fn ceph_ioctl_get_layout(file: *mut file, arg: *mut core::ffi::c_void) -> c_long {
    let ci = ceph_inode(file_inode(file));
    let mut l: ceph_ioctl_layout = core::mem::zeroed();
    let err = ceph_do_getattr(file_inode(file), CEPH_STAT_CAP_LAYOUT, false);
    if err == 0 {
        l.stripe_unit = (*ci).i_layout.stripe_unit;
        l.stripe_count = (*ci).i_layout.stripe_count;
        l.object_size = (*ci).i_layout.object_size;
        l.data_pool = (*ci).i_layout.pool_id;
        l.preferred_osd = -1;
        if copy_to_user(arg, &l as *const _ as *const core::ffi::c_void, core::mem::size_of::<ceph_ioctl_layout>()) != 0 {
            return -EFAULT;
        }
    }
    err
}

unsafe fn __validate_layout(mdsc: *mut ceph_mds_client, l: *mut ceph_ioctl_layout) -> c_long {
    if ((*l).object_size & !PAGE_MASK) != 0 || ((*l).stripe_unit & !PAGE_MASK) != 0 ||
        ((*l).stripe_unit as u32 != 0 && ((*l).object_size as u32 % (*l).stripe_unit as u32) != 0) {
        return -EINVAL;
    }
    mutex_lock(&mut (*mdsc).mutex);
    let mut err = -EINVAL;
    for i in 0..(*mdsc).mdsmap.m_num_data_pg_pools {
        if (*mdsc).mdsmap.m_data_pg_pools[i as usize] == (*l).data_pool {
            err = 0;
            break;
        }
    }
    mutex_unlock(&mut (*mdsc).mutex);
    err
}

unsafe fn ceph_ioctl_set_layout(file: *mut file, arg: *mut core::ffi::c_void) -> c_long {
    let inode = file_inode(file);
    let mdsc = (*ceph_sb_to_fs_client((*inode).i_sb)).mdsc;
    let ci = ceph_inode(file_inode(file));
    let mut l: ceph_ioctl_layout = core::mem::zeroed();
    let mut nl: ceph_ioctl_layout = core::mem::zeroed();
    if !inode_owner_or_capable(file_mnt_idmap(file), inode) { return -EACCES; }
    if copy_from_user(&mut l as *mut _ as *mut core::ffi::c_void, arg, core::mem::size_of::<ceph_ioctl_layout>()) != 0 { return -EFAULT; }
    let err = ceph_do_getattr(inode, CEPH_STAT_CAP_LAYOUT, false);
    if err != 0 { return err; }
    nl.stripe_count = if l.stripe_count != 0 { l.stripe_count } else { (*ci).i_layout.stripe_count };
    nl.stripe_unit = if l.stripe_unit != 0 { l.stripe_unit } else { (*ci).i_layout.stripe_unit };
    nl.object_size = if l.object_size != 0 { l.object_size } else { (*ci).i_layout.object_size };
    nl.data_pool = if l.data_pool != 0 { l.data_pool } else { (*ci).i_layout.pool_id };
    nl.preferred_osd = -1;
    let err = __validate_layout(mdsc, &mut nl);
    if err != 0 { return err; }
    let req = ceph_mdsc_create_request(mdsc, CEPH_MDS_OP_SETLAYOUT, USE_AUTH_MDS);
    if IS_ERR(req) { return PTR_ERR(req); }
    (*req).r_inode = inode; ihold(inode); (*req).r_num_caps = 1;
    (*req).r_inode_drop = CEPH_CAP_FILE_SHARED | CEPH_CAP_FILE_EXCL;
    (*req).r_args.setlayout.layout.fl_stripe_unit = cpu_to_le32(l.stripe_unit);
    (*req).r_args.setlayout.layout.fl_stripe_count = cpu_to_le32(l.stripe_count);
    (*req).r_args.setlayout.layout.fl_object_size = cpu_to_le32(l.object_size);
    (*req).r_args.setlayout.layout.fl_pg_pool = cpu_to_le32(l.data_pool);
    let err = ceph_mdsc_do_request(mdsc, core::ptr::null_mut(), req);
    ceph_mdsc_put_request(req); err
}

unsafe fn ceph_ioctl_set_layout_policy(file: *mut file, arg: *mut core::ffi::c_void) -> c_long {
    let inode = file_inode(file);
    let mdsc = (*ceph_sb_to_fs_client((*inode).i_sb)).mdsc;
    let mut l: ceph_ioctl_layout = core::mem::zeroed();
    if !inode_owner_or_capable(file_mnt_idmap(file), inode) { return -EACCES; }
    if copy_from_user(&mut l as *mut _ as *mut core::ffi::c_void, arg, core::mem::size_of::<ceph_ioctl_layout>()) != 0 { return -EFAULT; }
    let err = __validate_layout(mdsc, &mut l); if err != 0 { return err; }
    let req = ceph_mdsc_create_request(mdsc, CEPH_MDS_OP_SETDIRLAYOUT, USE_AUTH_MDS);
    if IS_ERR(req) { return PTR_ERR(req); }
    (*req).r_inode = inode; ihold(inode); (*req).r_num_caps = 1;
    (*req).r_args.setlayout.layout.fl_stripe_unit = cpu_to_le32(l.stripe_unit);
    (*req).r_args.setlayout.layout.fl_stripe_count = cpu_to_le32(l.stripe_count);
    (*req).r_args.setlayout.layout.fl_object_size = cpu_to_le32(l.object_size);
    (*req).r_args.setlayout.layout.fl_pg_pool = cpu_to_le32(l.data_pool);
    let err = ceph_mdsc_do_request(mdsc, inode, req); ceph_mdsc_put_request(req); err
}

unsafe fn ceph_ioctl_get_dataloc(file: *mut file, arg: *mut core::ffi::c_void) -> c_long {
    let inode = file_inode(file); let ci = ceph_inode(inode);
    let osdc = &mut (*(*ceph_sb_to_fs_client((*inode).i_sb)).client).osdc;
    let mut dl: ceph_ioctl_dataloc = core::mem::zeroed();
    let mut oid: ceph_object_id = core::mem::zeroed(); let mut oloc: ceph_object_locator = core::mem::zeroed();
    let mut pgid: ceph_pg = core::mem::zeroed(); let mut xlen: u32 = 0; let mut tmp: u64;
    if copy_from_user(&mut dl as *mut _ as *mut core::ffi::c_void, arg, core::mem::size_of::<ceph_ioctl_dataloc>()) != 0 { return -EFAULT; }
    down_read(&mut osdc.lock);
    ceph_calc_file_object_mapping(&(*ci).i_layout, dl.file_offset, 1, &mut dl.object_no, &mut dl.object_offset, &mut xlen);
    dl.file_offset -= dl.object_offset; dl.object_size = (*ci).i_layout.object_size; dl.block_size = (*ci).i_layout.stripe_unit;
    tmp = dl.object_offset; dl.block_offset = do_div(&mut tmp, dl.block_size);
    snprintf(dl.object_name.as_mut_ptr(), core::mem::size_of_val(&dl.object_name), c"%llx.%08llx".as_ptr(), ceph_ino(inode), dl.object_no);
    oloc.pool = (*ci).i_layout.pool_id; oloc.pool_ns = ceph_try_get_string((*ci).i_layout.pool_ns);
    ceph_oid_printf(&mut oid, c"%s".as_ptr(), dl.object_name.as_ptr());
    let r = ceph_object_locator_to_pg((*osdc).osdmap, &oid, &oloc, &mut pgid); ceph_oloc_destroy(&mut oloc);
    if r < 0 { up_read(&mut osdc.lock); return r; }
    dl.osd = ceph_pg_to_acting_primary((*osdc).osdmap, &pgid);
    if dl.osd >= 0 { let a = ceph_osd_addr((*osdc).osdmap, dl.osd); if !a.is_null() { memcpy(&mut dl.osd_addr as *mut _ as *mut _, &(*a).in_addr as *const _ as *const _, core::mem::size_of_val(&dl.osd_addr)); } }
    else { memset(&mut dl.osd_addr as *mut _ as *mut _, 0, core::mem::size_of_val(&dl.osd_addr)); }
    up_read(&mut osdc.lock);
    if copy_to_user(arg, &dl as *const _ as *const core::ffi::c_void, core::mem::size_of::<ceph_ioctl_dataloc>()) != 0 { return -EFAULT; } 0
}

unsafe fn ceph_ioctl_lazyio(file: *mut file) -> c_long {
    let fi = (*file).private_data as *mut ceph_file_info; let inode = file_inode(file); let ci = ceph_inode(inode);
    let mdsc = ceph_inode_to_fs_client(inode).mdsc; let cl = (*mdsc).fsc.client; let mut already = false;
    spin_lock(&mut (*ci).i_ceph_lock);
    if ((*fi).fmode & CEPH_FILE_MODE_LAZY) == 0 { (*fi).fmode |= CEPH_FILE_MODE_LAZY; (*ci).i_nr_by_mode[ffs(CEPH_FILE_MODE_LAZY) as usize] += 1; __ceph_touch_fmode(ci, mdsc, (*fi).fmode); } else { already = true; }
    spin_unlock(&mut (*ci).i_ceph_lock);
    if already { doutc(cl, c"file %p %p %llx.%llx already lazy\n".as_ptr(), file, inode, ceph_vinop(inode)); } else { doutc(cl, c"file %p %p %llx.%llx marked lazy\n".as_ptr(), file, inode, ceph_vinop(inode)); ceph_check_caps(ci, 0); } 0
}

unsafe fn ceph_ioctl_syncio(file: *mut file) -> c_long { (*(file as *mut file)).private_data.cast::<ceph_file_info>().as_mut().unwrap().flags |= CEPH_F_SYNC; 0 }

unsafe fn vet_mds_for_fscrypt(file: *mut file) -> c_int {
    let mdsc = ceph_sb_to_mdsc((*file_inode(file)).i_sb); let mut ret = -EOPNOTSUPP; mutex_lock(&mut (*mdsc).mutex);
    for i in 0..(*mdsc).max_sessions { let s = (*mdsc).sessions[i as usize]; if s.is_null() { continue; } if test_bit(CEPHFS_FEATURE_ALTERNATE_NAME, &(*s).s_features) != 0 { ret = 0; } break; }
    mutex_unlock(&mut (*mdsc).mutex); ret
}

unsafe fn ceph_set_encryption_policy(file: *mut file, arg: c_ulong) -> c_long {
    let inode = file_inode(file); let ci = ceph_inode(inode); if (*ci).i_layout.stripe_count > 1 { return -EINVAL; }
    let ret = vet_mds_for_fscrypt(file); if ret != 0 { return ret as c_long; }
    let mut got = 0; let ret = ceph_get_caps(file, CEPH_CAP_FILE_SHARED, 0, -1, &mut got); if ret != 0 { return ret as c_long; }
    let ret = fscrypt_ioctl_set_policy(file, arg as *const core::ffi::c_void); if got != 0 { ceph_put_cap_refs(ci, got); } ret
}

unsafe fn ceph_ioctl_cmd_name(cmd: c_uint) -> *const c_char {
    match cmd { CEPH_IOC_GET_LAYOUT => c"get_layout", CEPH_IOC_SET_LAYOUT => c"set_layout", CEPH_IOC_SET_LAYOUT_POLICY => c"set_layout_policy", CEPH_IOC_GET_DATALOC => c"get_dataloc", CEPH_IOC_LAZYIO => c"lazyio", CEPH_IOC_SYNCIO => c"syncio", FS_IOC_SET_ENCRYPTION_POLICY => c"set_encryption_policy", FS_IOC_GET_ENCRYPTION_POLICY => c"get_encryption_policy", FS_IOC_GET_ENCRYPTION_POLICY_EX => c"get_encryption_policy_ex", FS_IOC_ADD_ENCRYPTION_KEY => c"add_encryption_key", FS_IOC_REMOVE_ENCRYPTION_KEY => c"remove_encryption_key", FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS => c"remove_encryption_key_all_users", FS_IOC_GET_ENCRYPTION_KEY_STATUS => c"get_encryption_key_status", FS_IOC_GET_ENCRYPTION_NONCE => c"get_encryption_nonce", _ => c"unknown" }.as_ptr()
}

pub unsafe fn ceph_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let inode = file_inode(file); let fsc = ceph_inode_to_fs_client(inode);
    doutc((*fsc).client, c"file %p %p %llx.%llx cmd %s arg %lu\n".as_ptr(), file, inode, ceph_vinop(inode), ceph_ioctl_cmd_name(cmd), arg);
    match cmd {
        CEPH_IOC_GET_LAYOUT => ceph_ioctl_get_layout(file, arg as *mut _), CEPH_IOC_SET_LAYOUT => ceph_ioctl_set_layout(file, arg as *mut _), CEPH_IOC_SET_LAYOUT_POLICY => ceph_ioctl_set_layout_policy(file, arg as *mut _), CEPH_IOC_GET_DATALOC => ceph_ioctl_get_dataloc(file, arg as *mut _), CEPH_IOC_LAZYIO => ceph_ioctl_lazyio(file), CEPH_IOC_SYNCIO => ceph_ioctl_syncio(file),
        FS_IOC_SET_ENCRYPTION_POLICY => ceph_set_encryption_policy(file, arg),
        FS_IOC_GET_ENCRYPTION_POLICY => { let r=vet_mds_for_fscrypt(file); if r!=0 { return r as c_long; } fscrypt_ioctl_get_policy(file,arg as *mut _) }, FS_IOC_GET_ENCRYPTION_POLICY_EX => { let r=vet_mds_for_fscrypt(file); if r!=0{return r as c_long;} fscrypt_ioctl_get_policy_ex(file,arg as *mut _) }, FS_IOC_ADD_ENCRYPTION_KEY => { let r=vet_mds_for_fscrypt(file); if r!=0{return r as c_long;} fscrypt_ioctl_add_key(file,arg as *mut _) }, FS_IOC_REMOVE_ENCRYPTION_KEY => fscrypt_ioctl_remove_key(file,arg as *mut _), FS_IOC_REMOVE_ENCRYPTION_KEY_ALL_USERS => fscrypt_ioctl_remove_key_all_users(file,arg as *mut _), FS_IOC_GET_ENCRYPTION_KEY_STATUS => fscrypt_ioctl_get_key_status(file,arg as *mut _), FS_IOC_GET_ENCRYPTION_NONCE => { let r=vet_mds_for_fscrypt(file); if r!=0{return r as c_long;} fscrypt_ioctl_get_nonce(file,arg as *mut _) }, _ => -ENOTTY
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
